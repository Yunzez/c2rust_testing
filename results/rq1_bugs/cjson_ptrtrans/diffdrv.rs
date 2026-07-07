// Differential driver over PtrTrans-translated cJSON leaf functions.
use std::io::{Read, Write, BufWriter};
use cJSON::{parse_hex4, parse_number, parse_string, utf16_literal_to_utf8, ParseBuffer, InternalHooks};
use cJSON::cJSON as Item;

fn fake_alloc(_n: usize) -> *mut core::ffi::c_void { core::ptr::NonNull::<u8>::dangling().as_ptr() as *mut _ }
fn fake_free(_p: *mut core::ffi::c_void) {}

fn phex(o: &mut impl Write, b: &[u8]) { for x in b { write!(o, "{:02x}", x).unwrap(); } }

fn main() {
    let mut inp = Vec::new();
    std::io::stdin().read_to_end(&mut inp).unwrap();
    let so = std::io::stdout();
    let mut o = BufWriter::new(so.lock());
    let alloc: &dyn Fn(usize) -> *mut core::ffi::c_void = &fake_alloc;
    let free: &dyn Fn(*mut core::ffi::c_void) = &fake_free;
    let mut p = 0usize;
    while p + 3 <= inp.len() {
        let op = inp[p];
        let len = (inp[p+1] as usize) | ((inp[p+2] as usize) << 8);
        p += 3;
        if p + len > inp.len() { break; }
        let pay = &inp[p..p+len];
        p += len;
        match op {
            0 => {
                if len < 4 { writeln!(o, "H skip").unwrap(); continue; }
                writeln!(o, "H {:08x}", parse_hex4(Some(pay))).unwrap();
            }
            1 => {
                let mut item = Item { next: None, prev: None, child: None, type_: 0,
                    valuestring: None, valueint: 0, valuedouble: 0.0, string: None };
                let mut buf = ParseBuffer { content: Some(pay), length: len, offset: 0, depth: 0,
                    hooks: InternalHooks { allocate: Some(alloc), deallocate: Some(free), reallocate: None } };
                let (pi, pb) = (&mut item as *mut Item, &mut buf as *mut ParseBuffer);
                let r = unsafe { parse_number(Some(&mut *pi), Some(&mut *pb)) };
                let (off, vi, vd, ty) = unsafe { ((*pb).offset, (*pi).valueint, (*pi).valuedouble, (*pi).type_) };
                writeln!(o, "N {} off={} int={} dbl={:016x} ty={}", r, off, vi, vd.to_bits(), ty).unwrap();
            }
            2 => {
                let mut item = Item { next: None, prev: None, child: None, type_: 0,
                    valuestring: None, valueint: 0, valuedouble: 0.0, string: None };
                let mut buf = ParseBuffer { content: Some(pay), length: len, offset: 0, depth: 0,
                    hooks: InternalHooks { allocate: Some(alloc), deallocate: Some(free), reallocate: None } };
                let (pi, pb) = (&mut item as *mut Item, &mut buf as *mut ParseBuffer);
                let r = unsafe { parse_string(Some(&mut *pi), Some(&mut *pb)) };
                let (off, ty) = unsafe { ((*pb).offset, (*pi).type_) };
                write!(o, "S {} off={} vs=", r, off).unwrap();
                match unsafe { &(*pi).valuestring } {
                    Some(s) => phex(&mut o, s.as_bytes()),
                    None => write!(o, "-").unwrap(),
                }
                writeln!(o, " ty={}", ty).unwrap();
            }
            3 => {
                let mut out: Vec<u8> = Vec::new();
                let r = utf16_literal_to_utf8(Some(pay), Some(pay), Some(&mut out));
                write!(o, "U {} out=", r).unwrap(); phex(&mut o, &out); writeln!(o).unwrap();
            }
            _ => {}
        }
    }
}

// OBS driver (Rust side, PtrTrans cJSON translation verbatim in the crate). argv[1]=silent|print, argv[2]=state file.
// stdin = raw bytes = parse_buffer content handed to parse_string (offset 0), identical to the C driver.
#![allow(non_snake_case)]
use std::io::{Read, Write};
use cJSON::{parse_string, ParseBuffer, InternalHooks};
use cJSON::cJSON as Item;
fn fake_alloc(_n: usize) -> *mut core::ffi::c_void { core::ptr::NonNull::<u8>::dangling().as_ptr() as *mut _ }
fn fake_free(_p: *mut core::ffi::c_void) {}
fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 { std::process::exit(2); }
    let printing = args[1] == "print";
    let mut pay = Vec::new(); std::io::stdin().read_to_end(&mut pay).unwrap();
    let alloc: &dyn Fn(usize) -> *mut core::ffi::c_void = &fake_alloc;
    let free: &dyn Fn(*mut core::ffi::c_void) = &fake_free;
    let mut item = Item { next: None, prev: None, child: None, type_: 0, valuestring: None, valueint: 0, valuedouble: 0.0, string: None };
    let mut buf = ParseBuffer { content: Some(&pay[..]), length: pay.len(), offset: 0, depth: 0,
        hooks: InternalHooks { allocate: Some(alloc), deallocate: Some(free), reallocate: None } };
    let (pi, pb) = (&mut item as *mut Item, &mut buf as *mut ParseBuffer);
    let r = unsafe { parse_string(Some(&mut *pi), Some(&mut *pb)) };
    let (off, ty) = unsafe { ((*pb).offset, (*pi).type_) };
    let vs = match unsafe { &(*pi).valuestring } { Some(s) => hex(s.as_bytes()), None => "NULL".to_string() };
    let st = format!("ret:{}\ntype:{}\nvaluestring:{}\noffset:{}\nglobals:none\n", r, ty, vs, off);
    std::fs::write(&args[2], st).unwrap();
    if printing { let mut o = std::io::stdout(); writeln!(o, "ret={} valuestring={}", r, vs).unwrap(); }
}

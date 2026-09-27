#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use cJSON::{parse_hex4, parse_number, parse_string, utf16_literal_to_utf8, ParseBuffer, InternalHooks};
use cJSON::cJSON as Item;

fn fake_alloc(_n: usize) -> *mut core::ffi::c_void { core::ptr::NonNull::<u8>::dangling().as_ptr() as *mut _ }
fn fake_free(_p: *mut core::ffi::c_void) {}

fuzz_target!(|data: &[u8]| {
    let inp = data;
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
        match op % 4 {
            0 => { if len >= 4 { let _ = parse_hex4(Some(pay)); } }
            1 => {
                let mut item = Item { next: None, prev: None, child: None, type_: 0,
                    valuestring: None, valueint: 0, valuedouble: 0.0, string: None };
                let mut buf = ParseBuffer { content: Some(pay), length: len, offset: 0, depth: 0,
                    hooks: InternalHooks { allocate: Some(alloc), deallocate: Some(free), reallocate: None } };
                let (pi, pb) = (&mut item as *mut Item, &mut buf as *mut ParseBuffer);
                let _ = unsafe { parse_number(Some(&mut *pi), Some(&mut *pb)) };
            }
            2 => {
                let mut item = Item { next: None, prev: None, child: None, type_: 0,
                    valuestring: None, valueint: 0, valuedouble: 0.0, string: None };
                let mut buf = ParseBuffer { content: Some(pay), length: len, offset: 0, depth: 0,
                    hooks: InternalHooks { allocate: Some(alloc), deallocate: Some(free), reallocate: None } };
                let (pi, pb) = (&mut item as *mut Item, &mut buf as *mut ParseBuffer);
                let _ = unsafe { parse_string(Some(&mut *pi), Some(&mut *pb)) };
            }
            _ => {
                let mut out: Vec<u8> = Vec::new();
                let _ = utf16_literal_to_utf8(Some(pay), Some(pay), Some(&mut out));
            }
        }
    }
});

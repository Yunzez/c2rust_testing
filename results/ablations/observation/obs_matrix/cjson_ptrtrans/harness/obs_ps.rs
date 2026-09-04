#![no_main]
#![allow(unused, non_snake_case)]
// OBS corpus generator: identical decoding to the drivers — the whole input is the parse_buffer content, one parse_string call.
use libfuzzer_sys::fuzz_target;
use cJSON::{parse_string, ParseBuffer, InternalHooks};
use cJSON::cJSON as Item;
fn fake_alloc(_n: usize) -> *mut core::ffi::c_void { core::ptr::NonNull::<u8>::dangling().as_ptr() as *mut _ }
fn fake_free(_p: *mut core::ffi::c_void) {}
fuzz_target!(|pay: &[u8]| {
    let alloc: &dyn Fn(usize) -> *mut core::ffi::c_void = &fake_alloc;
    let free: &dyn Fn(*mut core::ffi::c_void) = &fake_free;
    let mut item = Item { next: None, prev: None, child: None, type_: 0, valuestring: None, valueint: 0, valuedouble: 0.0, string: None };
    let mut buf = ParseBuffer { content: Some(pay), length: pay.len(), offset: 0, depth: 0,
        hooks: InternalHooks { allocate: Some(alloc), deallocate: Some(free), reallocate: None } };
    let (pi, pb) = (&mut item as *mut Item, &mut buf as *mut ParseBuffer);
    let _ = unsafe { parse_string(Some(&mut *pi), Some(&mut *pb)) };
});

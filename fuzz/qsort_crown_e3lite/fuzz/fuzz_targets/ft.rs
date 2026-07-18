#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use qsort_crown_e3lite as translated;
fuzz_target!(|data: &[u8]| {
    let mut arr: Vec<i32> = data.chunks_exact(4).map(|c| i32::from_le_bytes([c[0],c[1],c[2],c[3]])).take(256).collect();
    if arr.len() < 2 { return; }
    let n = arr.len();
    unsafe { translated::quickSort(arr.as_mut_ptr(), 0, (n - 1) as i32); }
});

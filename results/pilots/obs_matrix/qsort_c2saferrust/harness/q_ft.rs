#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use qsort_c2saferrust_obs as translated;
// C2SaferRust reshaped ABI: quickSort(&mut [i32], usize, usize). Contract low=0, high=len-1.
// Same decoding as the OBS qsort template (raw bytes -> i32 LE chunks, take 256).
fuzz_target!(|data: &[u8]| {
    let mut arr: Vec<i32> = data.chunks_exact(4)
        .map(|c| i32::from_le_bytes([c[0], c[1], c[2], c[3]])).take(256).collect();
    if arr.len() < 2 { return; }
    let n = arr.len();
    translated::quickSort(&mut arr[..], 0, n - 1);
});

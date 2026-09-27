#![no_main]

use core::ffi::c_int;
use libfuzzer_sys::fuzz_target;

// Use the translated implementation from the local crate.
use qsort_example as translated;

extern "C" {
    // C oracle, renamed in build.rs to avoid symbol collisions.
    fn c_quickSort(arr: *mut c_int, low: c_int, high: c_int);
}

fuzz_target!(|data: Vec<u64>| {
    if data.is_empty() {
        return;
    }

    // Qsort uses C int, so map u64 -> c_int with truncation.
    let mut c_vec: Vec<c_int> = data.iter().map(|&v| v as c_int).collect();
    let mut r_vec: Vec<c_int> = data.iter().map(|&v| v as c_int).collect();

    let high = (c_vec.len() - 1) as c_int;

    unsafe {
        c_quickSort(c_vec.as_mut_ptr(), 0 as c_int, high);
        translated::quickSort(r_vec.as_mut_ptr(), 0 as c_int, high);
    }

    // Differential check: exact array equality. If different, it's a divergence.
    if c_vec != r_vec {
        panic!("divergence: C and Rust results differ");
    }
});

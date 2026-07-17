#![no_main]

use core::ffi::c_int;
use libfuzzer_sys::fuzz_target;

// C2SaferRust's reshaped translation (buggy int->usize quickSort).
use qsort_c2saferrust as translated;

extern "C" {
    // C oracle, renamed in build.rs to avoid symbol collisions.
    fn c_quickSort(arr: *mut c_int, low: c_int, high: c_int);
}

// Boundary: quickSort. C ABI (int*, int, int) vs C2SaferRust's reshaped Rust ABI
// (&mut [i32], usize, usize) -- the harness IS the signature bridge. Canonical
// call contract low=0, high=len-1 so every input is a legal index range for the
// C oracle (the differential oracle). A Rust crash (OOB / non-termination) or an
// array mismatch is the finding.
fuzz_target!(|data: Vec<u64>| {
    if data.is_empty() {
        return;
    }
    let mut c_vec: Vec<c_int> = data.iter().map(|&v| v as c_int).collect();
    let mut r_vec: Vec<i32> = data.iter().map(|&v| v as i32).collect();
    let n = c_vec.len();
    let high = (n - 1) as c_int;

    unsafe {
        c_quickSort(c_vec.as_mut_ptr(), 0 as c_int, high);
    }
    // reshaped Rust call: slice + usize indices
    translated::quickSort(&mut r_vec, 0, n - 1);

    if c_vec != r_vec {
        panic!("divergence: C and Rust results differ");
    }
});

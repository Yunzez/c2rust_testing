// C2SaferRust's published safety-lifted qsort (the WIP output that introduced the
// int->usize recursion-termination bug). Verbatim from
// tools/frameworks/c2saferrust/laertes_benchmarks/qsort_WIP/qsort.rs
// (only the module-level attributes are dropped so it compiles as a lib target).
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

#[no_mangle]
pub fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
}

#[no_mangle]
pub unsafe extern "C" fn partition(
    mut arr: *mut ::core::ffi::c_int,
    mut low: ::core::ffi::c_int,
    mut high: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pivot: ::core::ffi::c_int = *arr.offset(high as isize);
    let mut i: ::core::ffi::c_int = low - 1 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = low;
    while j <= high - 1 as ::core::ffi::c_int {
        if *arr.offset(j as isize) <= pivot {
            i += 1;
            swap(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1
    }
    let a_index = (i + 1) as usize;
    let b_index = high as usize;
    swap(
        &mut (*arr.offset(a_index as isize)),
        &mut (*arr.offset(b_index as isize)),
    );
    return i + 1 as ::core::ffi::c_int;
}

#[no_mangle]
pub fn quickSort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let i = unsafe { partition(arr.as_mut_ptr(), low as i32, high as i32) } as usize;
        quickSort(arr, low, i.wrapping_sub(1));
        quickSort(arr, i + 1, high);
    }
}

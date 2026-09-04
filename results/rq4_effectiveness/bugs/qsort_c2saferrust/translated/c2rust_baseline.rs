#![feature(core_intrinsics)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#![feature(c_variadic)]
#[no_mangle]
pub unsafe extern "C" fn swap(mut a: *mut std::os::raw::c_int,
                              mut b: *mut std::os::raw::c_int) {
    let mut t: std::os::raw::c_int = *a;
    *a = *b;
    *b = t;
}
#[no_mangle]
pub unsafe extern "C" fn partition(mut arr: *mut std::os::raw::c_int,
                                   mut low: std::os::raw::c_int,
                                   mut high: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut pivot: std::os::raw::c_int = *arr.offset(high as isize);
    let mut i: std::os::raw::c_int = low - 1 as std::os::raw::c_int;
    let mut j: std::os::raw::c_int = low;
    while j <= high - 1 as std::os::raw::c_int {
        if *arr.offset(j as isize) <= pivot {
            i += 1;
            swap(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1
    }
    swap(&mut *arr.offset((i + 1 as std::os::raw::c_int) as isize),
         &mut *arr.offset(high as isize));
    return i + 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quickSort(mut arr: *mut std::os::raw::c_int,
                                   mut low: std::os::raw::c_int,
                                   mut high: std::os::raw::c_int) {
    if low < high {
        let mut i: std::os::raw::c_int = partition(arr, low, high);
        quickSort(arr, low, i - 1 as std::os::raw::c_int);
        quickSort(arr, i + 1 as std::os::raw::c_int, high);
    };
}

#![feature(core_intrinsics)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#![feature(c_variadic)]


#[no_mangle]
pub fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
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
    let a_index = (i + 1) as usize;
let b_index = high as usize;
swap(&mut (*arr.offset(a_index as isize)), &mut (*arr.offset(b_index as isize)));
    return i + 1 as std::os::raw::c_int;
}
#[no_mangle]
pub fn quickSort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let i = unsafe { partition(arr.as_mut_ptr(), low as i32, high as i32) } as usize;
        quickSort(arr, low, i.wrapping_sub(1));
        quickSort(arr, i + 1, high);
    }
}


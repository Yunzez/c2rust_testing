#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[no_mangle]
pub unsafe extern "C" fn mu_atoi(mut s: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        s = s.offset(1);
    }
    let mut current_block_3: u64;
    match *s as ::core::ffi::c_int {
        45 => {
            neg = 1 as ::core::ffi::c_int;
            current_block_3 = 15333514044752716678;
        }
        43 => {
            current_block_3 = 15333514044752716678;
        }
        _ => {
            current_block_3 = 7095457783677275021;
        }
    }
    match current_block_3 {
        15333514044752716678 => {
            s = s.offset(1);
        }
        _ => {}
    }
    while *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        let fresh0 = s;
        s = s.offset(1);
        n = 10 as ::core::ffi::c_int * n - (*fresh0 as ::core::ffi::c_int - '0' as i32);
    }
    return if neg != 0 { n } else { -n };
}

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const RPN_ERR_RESULT: C2RustUnnamed = 6;
pub const RPN_ERR_BADTOK: C2RustUnnamed = 5;
pub const RPN_ERR_TRUNC: C2RustUnnamed = 4;
pub const RPN_ERR_DIVZERO: C2RustUnnamed = 3;
pub const RPN_ERR_OVERFLOW: C2RustUnnamed = 2;
pub const RPN_ERR_UNDERFLOW: C2RustUnnamed = 1;
pub const RPN_OK: C2RustUnnamed = 0;
pub const RPN_STACK_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn read_le32(mut p: *const uint8_t) -> int32_t {
    let mut v: uint32_t = *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*p.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*p.offset(3 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int;
    return v as int32_t;
}
unsafe extern "C" fn apply_op(
    mut op: uint8_t,
    mut lhs: int64_t,
    mut rhs: int64_t,
    mut out: *mut int64_t,
) -> ::core::ffi::c_int {
    match op as ::core::ffi::c_int {
        43 => {
            *out = lhs + rhs;
            return RPN_OK as ::core::ffi::c_int;
        }
        45 => {
            *out = lhs - rhs;
            return RPN_OK as ::core::ffi::c_int;
        }
        42 => {
            *out = lhs * rhs;
            return RPN_OK as ::core::ffi::c_int;
        }
        47 => {
            if rhs == 0 as int64_t {
                return RPN_ERR_DIVZERO as ::core::ffi::c_int;
            }
            *out = lhs / rhs;
            return RPN_OK as ::core::ffi::c_int;
        }
        37 => {
            if rhs == 0 as int64_t {
                return RPN_ERR_DIVZERO as ::core::ffi::c_int;
            }
            *out = lhs % rhs;
            return RPN_OK as ::core::ffi::c_int;
        }
        _ => return RPN_ERR_BADTOK as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn stack_push(
    mut stk: *mut int64_t,
    mut sp: *mut size_t,
    mut v: int64_t,
) -> ::core::ffi::c_int {
    if *sp >= RPN_STACK_MAX as size_t {
        return RPN_ERR_OVERFLOW as ::core::ffi::c_int;
    }
    let fresh0 = *sp;
    *sp = (*sp).wrapping_add(1);
    *stk.offset(fresh0 as isize) = v;
    return RPN_OK as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rpn_eval(
    mut prog: *const uint8_t,
    mut len: size_t,
    mut result: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut stk: [int64_t; 64] = [0; 64];
    let mut sp: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < len {
        let mut tok: uint8_t = *prog.offset(i as isize);
        if tok as ::core::ffi::c_int == 'N' as i32 {
            if len.wrapping_sub(i) < 5 as size_t {
                return RPN_ERR_TRUNC as ::core::ffi::c_int;
            }
            let mut num: int32_t = read_le32(
                prog.offset(i as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            let mut st: ::core::ffi::c_int =
                stack_push(&raw mut stk as *mut int64_t, &raw mut sp, num as int64_t);
            if st != RPN_OK as ::core::ffi::c_int {
                return st;
            }
            i = i.wrapping_add(5 as size_t);
        } else if tok as ::core::ffi::c_int == '+' as i32
            || tok as ::core::ffi::c_int == '-' as i32
            || tok as ::core::ffi::c_int == '*' as i32
            || tok as ::core::ffi::c_int == '/' as i32
            || tok as ::core::ffi::c_int == '%' as i32
        {
            if sp < 2 as size_t {
                return RPN_ERR_UNDERFLOW as ::core::ffi::c_int;
            }
            sp = sp.wrapping_sub(1);
            let mut rhs: int64_t = stk[sp as usize];
            sp = sp.wrapping_sub(1);
            let mut lhs: int64_t = stk[sp as usize];
            let mut res: int64_t = 0;
            let mut st_0: ::core::ffi::c_int = apply_op(tok, lhs, rhs, &raw mut res);
            if st_0 != RPN_OK as ::core::ffi::c_int {
                return st_0;
            }
            st_0 = stack_push(&raw mut stk as *mut int64_t, &raw mut sp, res);
            if st_0 != RPN_OK as ::core::ffi::c_int {
                return st_0;
            }
            i = i.wrapping_add(1 as size_t);
        } else if tok as ::core::ffi::c_int == ' ' as i32
            || tok as ::core::ffi::c_int == '\t' as i32
            || tok as ::core::ffi::c_int == '\n' as i32
        {
            i = i.wrapping_add(1 as size_t);
        } else {
            return RPN_ERR_BADTOK as ::core::ffi::c_int;
        }
    }
    if sp != 1 as size_t {
        return RPN_ERR_RESULT as ::core::ffi::c_int;
    }
    *result = stk[0 as ::core::ffi::c_int as usize];
    return RPN_OK as ::core::ffi::c_int;
}

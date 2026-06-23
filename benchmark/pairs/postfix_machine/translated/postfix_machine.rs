#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type size_t = usize;
unsafe extern "C" fn pop(mut stack: *mut int64_t, mut sp: *mut size_t) -> int64_t {
    if *sp == 0 as size_t {
        return 0 as int64_t;
    }
    *sp = (*sp).wrapping_sub(1);
    return *stack.offset(*sp as isize);
}
#[no_mangle]
pub unsafe extern "C" fn postfix_run(
    mut code: *const uint8_t,
    mut len: size_t,
    mut out: *mut int64_t,
) -> size_t {
    let mut stack: [int64_t; 16] = [0; 16];
    let mut sp: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < len {
        let fresh0 = i;
        i = i.wrapping_add(1);
        let mut op: uint8_t = *code.offset(fresh0 as isize);
        match op as ::core::ffi::c_int {
            1 => {
                if i < len {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    let mut v: int8_t = *code.offset(fresh1 as isize) as int8_t;
                    let fresh2 = sp;
                    sp = sp.wrapping_add(1);
                    stack[fresh2 as usize] = v as int64_t;
                }
            }
            2 => {
                let mut b: int64_t = pop(&raw mut stack as *mut int64_t, &raw mut sp);
                let mut a: int64_t = pop(&raw mut stack as *mut int64_t, &raw mut sp);
                let fresh3 = sp;
                sp = sp.wrapping_add(1);
                stack[fresh3 as usize] = a + b;
            }
            3 => {
                let mut top: int64_t = if sp > 0 as size_t {
                    stack[sp.wrapping_sub(1 as size_t) as usize]
                } else {
                    0 as int64_t
                };
                let fresh4 = sp;
                sp = sp.wrapping_add(1);
                stack[fresh4 as usize] = top;
            }
            4 => {
                pop(&raw mut stack as *mut int64_t, &raw mut sp);
            }
            _ => {}
        }
    }
    *out = if sp > 0 as size_t {
        stack[sp.wrapping_sub(1 as size_t) as usize]
    } else {
        0 as int64_t
    };
    return sp;
}

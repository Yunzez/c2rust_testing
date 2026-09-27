#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type state_t = ::core::ffi::c_uint;
pub const ST_COUNT: state_t = 2;
pub const ST_UNLOCKED: state_t = 1;
pub const ST_LOCKED: state_t = 0;
pub type event_t = ::core::ffi::c_uint;
pub const EV_COUNT: event_t = 2;
pub const EV_PUSH: event_t = 1;
pub const EV_COIN: event_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctx_t {
    pub opened: int32_t,
    pub coins: int32_t,
    pub rejected: int32_t,
}
pub type transition_fn = Option<unsafe extern "C" fn(*mut ctx_t) -> state_t>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn on_coin_locked(mut c: *mut ctx_t) -> state_t {
    (*c).coins += 1;
    return ST_UNLOCKED;
}
unsafe extern "C" fn on_push_locked(mut c: *mut ctx_t) -> state_t {
    (*c).rejected += 1;
    return ST_LOCKED;
}
unsafe extern "C" fn on_coin_unlocked(mut c: *mut ctx_t) -> state_t {
    (*c).coins += 1;
    return ST_UNLOCKED;
}
unsafe extern "C" fn on_push_unlocked(mut c: *mut ctx_t) -> state_t {
    (*c).opened += 1;
    return ST_LOCKED;
}
unsafe extern "C" fn lookup(mut s: state_t, mut e: event_t) -> transition_fn {
    static mut tbl: [[transition_fn; 2]; 2] = unsafe {
        [
            [
                Some(on_coin_locked as unsafe extern "C" fn(*mut ctx_t) -> state_t),
                Some(on_push_locked as unsafe extern "C" fn(*mut ctx_t) -> state_t),
            ],
            [
                Some(on_coin_unlocked as unsafe extern "C" fn(*mut ctx_t) -> state_t),
                Some(on_push_unlocked as unsafe extern "C" fn(*mut ctx_t) -> state_t),
            ],
        ]
    };
    if (s as ::core::ffi::c_uint) < ST_COUNT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (e as ::core::ffi::c_uint) < EV_COUNT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return tbl[s as usize][e as usize];
    }
    return None;
}
#[no_mangle]
pub unsafe extern "C" fn simulate(
    mut events: *const uint8_t,
    mut n: size_t,
    mut out_opened: *mut int32_t,
) -> int32_t {
    let mut c: ctx_t = ctx_t {
        opened: 0 as int32_t,
        coins: 0 as int32_t,
        rejected: 0 as int32_t,
    };
    let mut s: state_t = ST_LOCKED;
    if events.is_null() {
        if !out_opened.is_null() {
            *out_opened = 0 as ::core::ffi::c_int as int32_t;
        }
        return s as int32_t;
    }
    let mut i: size_t = 0 as size_t;
    while i < n {
        let mut e: event_t =
            (if *events.offset(i as isize) as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint != 0 {
                EV_PUSH as ::core::ffi::c_int
            } else {
                EV_COIN as ::core::ffi::c_int
            }) as event_t;
        let mut fn_0: transition_fn = lookup(s, e);
        if fn_0.is_some() {
            s = fn_0.expect("non-null function pointer")(&raw mut c);
        }
        i = i.wrapping_add(1);
    }
    if !out_opened.is_null() {
        *out_opened = c.opened;
    }
    return s as int32_t;
}

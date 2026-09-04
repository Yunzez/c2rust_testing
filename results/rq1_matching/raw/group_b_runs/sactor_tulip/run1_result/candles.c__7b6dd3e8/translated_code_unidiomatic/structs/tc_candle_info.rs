#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_config {
    pub period: ::core::ffi::c_int,
    pub body_none: ::core::ffi::c_double,
    pub body_short: ::core::ffi::c_double,
    pub body_long: ::core::ffi::c_double,
    pub wick_none: ::core::ffi::c_double,
    pub wick_long: ::core::ffi::c_double,
    pub near: ::core::ffi::c_double,
}
pub type size_t = usize;
pub type tc_candle_function = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *const *const ::core::ffi::c_double,
        *const tc_config,
        *mut tc_result,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct tc_candle_info {
    pub name: *const ::core::ffi::c_char,
    pub full_name: *const ::core::ffi::c_char,
    pub pattern: ::core::ffi::c_ulong,
    pub candle: tc_candle_function,
}

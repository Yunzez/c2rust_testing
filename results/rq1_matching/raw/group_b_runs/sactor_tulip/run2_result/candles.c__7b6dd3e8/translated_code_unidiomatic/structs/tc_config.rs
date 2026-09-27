pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
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

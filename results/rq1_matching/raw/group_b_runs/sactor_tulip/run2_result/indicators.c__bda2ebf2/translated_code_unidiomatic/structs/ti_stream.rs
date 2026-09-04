pub type ti_indicator_start_function =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_double) -> ::core::ffi::c_int>;
pub type ti_indicator_function = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *const *const ::core::ffi::c_double,
        *const ::core::ffi::c_double,
        *const *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ti_stream {
    pub index: ::core::ffi::c_int,
    pub progress: ::core::ffi::c_int,
}

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
pub type ti_indicator_stream_new = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_double, *mut *mut ti_stream) -> ::core::ffi::c_int,
>;
pub type ti_indicator_stream_run = Option<
    unsafe extern "C" fn(
        *mut ti_stream,
        ::core::ffi::c_int,
        *const *const ::core::ffi::c_double,
        *const *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int,
>;
pub type ti_indicator_stream_free = Option<unsafe extern "C" fn(*mut ti_stream) -> ()>;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ti_indicator_info {
    pub name: *const ::core::ffi::c_char,
    pub full_name: *const ::core::ffi::c_char,
    pub start: ti_indicator_start_function,
    pub indicator: ti_indicator_function,
    pub indicator_ref: ti_indicator_function,
    pub type_0: ::core::ffi::c_int,
    pub inputs: ::core::ffi::c_int,
    pub options: ::core::ffi::c_int,
    pub outputs: ::core::ffi::c_int,
    pub input_names: [*const ::core::ffi::c_char; 16],
    pub option_names: [*const ::core::ffi::c_char; 16],
    pub output_names: [*const ::core::ffi::c_char; 16],
    pub stream_new: ti_indicator_stream_new,
    pub stream_run: ti_indicator_stream_run,
    pub stream_free: ti_indicator_stream_free,
}

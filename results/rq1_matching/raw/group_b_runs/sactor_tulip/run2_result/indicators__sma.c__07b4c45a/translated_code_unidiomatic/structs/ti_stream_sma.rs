pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ti_stream_sma {
    pub index: ::core::ffi::c_int,
    pub progress: ::core::ffi::c_int,
    pub period: ::core::ffi::c_int,
    pub per: ::core::ffi::c_double,
    pub sum: ::core::ffi::c_double,
    pub buffer_idx: ::core::ffi::c_int,
    pub buffer: [::core::ffi::c_double; 0],
}

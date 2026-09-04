#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ti_buffer {
    pub size: ::core::ffi::c_int,
    pub pushes: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub sum: ::core::ffi::c_double,
    pub vals: [::core::ffi::c_double; 1],
}

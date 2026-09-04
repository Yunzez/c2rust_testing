pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct tc_hit {
    pub index: ::core::ffi::c_int,
    pub patterns: ::core::ffi::c_ulong,
}

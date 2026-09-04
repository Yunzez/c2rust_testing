pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct url_data {
    pub href: *mut ::core::ffi::c_char,
    pub protocol: *mut ::core::ffi::c_char,
    pub host: *mut ::core::ffi::c_char,
    pub auth: *mut ::core::ffi::c_char,
    pub hostname: *mut ::core::ffi::c_char,
    pub pathname: *mut ::core::ffi::c_char,
    pub search: *mut ::core::ffi::c_char,
    pub path: *mut ::core::ffi::c_char,
    pub hash: *mut ::core::ffi::c_char,
    pub query: *mut ::core::ffi::c_char,
    pub port: *mut ::core::ffi::c_char,
}

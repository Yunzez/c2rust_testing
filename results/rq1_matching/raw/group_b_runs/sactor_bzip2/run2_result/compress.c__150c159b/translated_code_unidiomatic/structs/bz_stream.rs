pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut ::core::ffi::c_char,
    pub avail_in: ::core::ffi::c_uint,
    pub total_in_lo32: ::core::ffi::c_uint,
    pub total_in_hi32: ::core::ffi::c_uint,
    pub next_out: *mut ::core::ffi::c_char,
    pub avail_out: ::core::ffi::c_uint,
    pub total_out_lo32: ::core::ffi::c_uint,
    pub total_out_hi32: ::core::ffi::c_uint,
    pub state: *mut ::core::ffi::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub bzfree:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}

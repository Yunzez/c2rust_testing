#[derive(Copy, Clone)]
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
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct bzFile {
    pub handle: *mut FILE,
    pub buf: [::core::ffi::c_char; 5000],
    pub bufN: ::core::ffi::c_int,
    pub writing: ::core::ffi::c_uchar,
    pub strm: bz_stream,
    pub lastErr: ::core::ffi::c_int,
    pub initialisedOk: ::core::ffi::c_uchar,
}

pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct genann {
    pub inputs: ::core::ffi::c_int,
    pub hidden_layers: ::core::ffi::c_int,
    pub hidden: ::core::ffi::c_int,
    pub outputs: ::core::ffi::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: ::core::ffi::c_int,
    pub total_neurons: ::core::ffi::c_int,
    pub weight: *mut ::core::ffi::c_double,
    pub output: *mut ::core::ffi::c_double,
    pub delta: *mut ::core::ffi::c_double,
}
pub unsafe fn genann_write(ann: *const genann, out: *mut libc::FILE) {
    use core::ffi::{c_char, c_double, c_int};
    extern "C" {
        fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    }
    unsafe fn c_str(ptr: *const u8) -> *const c_char {
        ptr as *const c_char
    }
    static FMT_HEADER: &[u8] = b"%d %d %d %d\0";

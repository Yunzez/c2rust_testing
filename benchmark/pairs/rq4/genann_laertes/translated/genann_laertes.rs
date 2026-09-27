// GENERATED for the RQ4 coverage experiment by scratchpad/rq4_cov/flatten_rust.py.
// Module bodies are copied byte-for-byte from
// tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/ (== fuzz/bzip2_c2rust_e3/src/).
// Only the module wrappers and the root re-exports below are added.
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut, internal_features,
         unused_imports, unpredictable_function_pointer_comparisons)]

pub mod genann {

extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn fprintf(_: * mut crate::example3::__sFILE, _: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fscanf(_: * mut crate::example3::__sFILE, _: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(_: * const std::os::raw::c_char);
    #[no_mangle]
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
    #[no_mangle]
    fn __error() -> * mut std::os::raw::c_int;
    #[no_mangle]
    fn exp(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn rand() -> std::os::raw::c_int;
    #[no_mangle]
    fn free(_: * mut core::ffi::c_void);
    #[no_mangle]
    fn memcpy(_: * mut core::ffi::c_void, _: * const core::ffi::c_void, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
}
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = std::os::raw::c_longlong;
pub type fpos_t = std::os::raw::c_longlong;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example3::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example3::__sFILE;
pub type FILE = crate::example3::__sFILE;
pub type genann_actfun = Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    if a < -45.0f64 { return 0 as std::os::raw::c_int as std::os::raw::c_double }
    if a > 45.0f64 { return 1 as std::os::raw::c_int as std::os::raw::c_double }
    return 1.0f64 / (1 as std::os::raw::c_int as std::os::raw::c_double + exp(-a));
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid_cached(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    /* If you're optimizing for memory usage, just
     * delete this entire function and replace references
     * of genann_act_sigmoid_cached to genann_act_sigmoid
     */
    let min: f64 = -15.0f64;
    let max: f64 = 15.0f64;
    static mut interval: std::os::raw::c_double = 0.0; unsafe fn laertes_init_interval() {
interval = 0.;}//;
    static mut initialized: std::os::raw::c_int = 0; unsafe fn laertes_init_initialized() {
initialized = 0 as std::os::raw::c_int;}//;
    static mut lookup: [std::os::raw::c_double; 4096] = [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]; unsafe fn laertes_init_lookup() {
lookup = [0.; 4096];}//;
    /* Calculate entire lookup table on first run. */
    if initialized == 0 {
        interval = (max - min) / 4096 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: i32 = 0;
        i = 0 as std::os::raw::c_int;
        while i < 4096 as std::os::raw::c_int {
            lookup[i as usize] =
                genann_act_sigmoid(min + interval * i as std::os::raw::c_double);
            i += 1
        }
        /* This is down here to make this thread safe. */
        initialized = 1 as std::os::raw::c_int
    }
    let mut i_0: i32 = 0;
    i_0 = ((a - min) / interval + 0.5f64) as std::os::raw::c_int;
    if i_0 <= 0 as std::os::raw::c_int { return lookup[0 as std::os::raw::c_int as usize] }
    if i_0 >= 4096 as std::os::raw::c_int {
        return lookup[(4096 as std::os::raw::c_int - 1 as std::os::raw::c_int) as usize]
    }
    return lookup[i_0 as usize];
}
#[no_mangle]
pub extern "C" fn genann_act_threshold(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    return (a > 0 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int as
               std::os::raw::c_double;
}
#[no_mangle]
pub extern "C" fn genann_act_linear(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn genann_init(mut inputs: std::os::raw::c_int,
                                     mut hidden_layers: std::os::raw::c_int,
                                     mut hidden: std::os::raw::c_int,
                                     mut outputs: std::os::raw::c_int)
 -> * mut crate::example1::genann {
    if hidden_layers < 0 as std::os::raw::c_int { return 0 as *mut genann }
    if inputs < 1 as std::os::raw::c_int { return 0 as *mut genann }
    if outputs < 1 as std::os::raw::c_int { return 0 as *mut genann }
    if hidden_layers > 0 as std::os::raw::c_int && hidden < 1 as std::os::raw::c_int {
        return 0 as *mut genann
    }
    let hidden_weights: i32 =
        if hidden_layers != 0 {
            ((inputs + 1 as std::os::raw::c_int) * hidden) +
                (hidden_layers - 1 as std::os::raw::c_int) *
                    (hidden + 1 as std::os::raw::c_int) * hidden
        } else { 0 as std::os::raw::c_int };
    let output_weights: i32 =
        (if hidden_layers != 0 {
             (hidden) + 1 as std::os::raw::c_int
         } else { (inputs) + 1 as std::os::raw::c_int }) * outputs;
    let total_weights: i32 = hidden_weights + output_weights;
    let total_neurons: i32 =
        inputs + hidden * hidden_layers + outputs;
    /* Allocate extra size for weights, outputs, and deltas. */
    let size: i32 =
        (::std::mem::size_of::<genann>() as
             std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                              as
                                              std::os::raw::c_ulong).wrapping_mul((total_weights
                                                                               +
                                                                               total_neurons
                                                                               +
                                                                               (total_neurons
                                                                                    -
                                                                                    inputs))
                                                                              as
                                                                              std::os::raw::c_ulong))
            as std::os::raw::c_int;
    let mut ret: * mut crate::example1::genann = malloc(size as std::os::raw::c_ulong) as *mut genann;
    if ret.is_null() { return 0 as *mut genann }
    (*ret).inputs = inputs;
    (*ret).hidden_layers = hidden_layers;
    (*ret).hidden = hidden;
    (*ret).outputs = outputs;
    (*ret).total_weights = total_weights;
    (*ret).total_neurons = total_neurons;
    /* Set pointers. */
    (*ret).weight =
        (ret as
             *mut std::os::raw::c_char).offset(::std::mem::size_of::<genann>() as
                                           std::os::raw::c_ulong as isize) as
            *mut std::os::raw::c_double;
    (*ret).output = (*ret).weight.offset((*ret).total_weights as isize);
    (*ret).delta = (*ret).output.offset((*ret).total_neurons as isize);
    genann_randomize(ret);
    (*ret).activation_hidden =
        Some(genann_act_sigmoid_cached);
    (*ret).activation_output =
        Some(genann_act_sigmoid_cached);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_read(mut in_0: * mut crate::example3::__sFILE) -> * mut crate::example1::genann {
    let mut inputs: i32 = 0;
    let mut hidden_layers: i32 = 0;
    let mut hidden: i32 = 0;
    let mut outputs: i32 = 0;
    let mut rc: i32 = 0;
    *__error() = 0 as std::os::raw::c_int;
    rc =
        fscanf(in_0, b"%d %d %d %d\x00" as *const u8 as *const std::os::raw::c_char,
               &mut inputs as *mut std::os::raw::c_int,
               &mut hidden_layers as *mut std::os::raw::c_int,
               &mut hidden as *mut std::os::raw::c_int,
               &mut outputs as *mut std::os::raw::c_int);
    if rc < 4 as std::os::raw::c_int || *__error() != 0 as std::os::raw::c_int {
        perror(b"fscanf\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as *mut genann
    }
    let mut ann: * mut crate::example1::genann =
        genann_init(inputs, hidden_layers, hidden, outputs);
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*ann).total_weights {
        *__error() = 0 as std::os::raw::c_int;
        rc =
            fscanf(in_0, b" %le\x00" as *const u8 as *const std::os::raw::c_char,
                   (*ann).weight.offset(i as isize));
        if rc < 1 as std::os::raw::c_int || *__error() != 0 as std::os::raw::c_int {
            perror(b"fscanf\x00" as *const u8 as *const std::os::raw::c_char);
            genann_free(ann);
            return 0 as *mut genann
        }
        i += 1
    }
    return ann;
}
#[no_mangle]
pub unsafe extern "C" fn genann_copy(mut ann: * const crate::example1::genann) -> * mut crate::example1::genann {
    let size: i32 =
        (::std::mem::size_of::<genann>() as
             std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                              as
                                              std::os::raw::c_ulong).wrapping_mul(((*ann).total_weights
                                                                               +
                                                                               (*ann).total_neurons
                                                                               +
                                                                               ((*ann).total_neurons
                                                                                    -
                                                                                    (*ann).inputs))
                                                                              as
                                                                              std::os::raw::c_ulong))
            as std::os::raw::c_int;
    let mut ret: * mut crate::example1::genann = malloc(size as std::os::raw::c_ulong) as *mut genann;
    if ret.is_null() { return 0 as *mut genann }
    memcpy(ret as *mut std::os::raw::c_void, ann as *const std::os::raw::c_void,
           size as std::os::raw::c_ulong);
    /* Set pointers. */
    (*ret).weight =
        (ret as
             *mut std::os::raw::c_char).offset(::std::mem::size_of::<genann>() as
                                           std::os::raw::c_ulong as isize) as
            *mut std::os::raw::c_double;
    (*ret).output = (*ret).weight.offset((*ret).total_weights as isize);
    (*ret).delta = (*ret).output.offset((*ret).total_neurons as isize);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_randomize(mut ann: * mut crate::example1::genann) {
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*ann).total_weights {
        let mut r: f64 =
            rand() as std::os::raw::c_double /
                0x7fffffff as std::os::raw::c_int as std::os::raw::c_double;
        /* Sets weights from -0.5 to 0.5. */
        *(*ann).weight.offset(i as isize) = r - 0.5f64;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn genann_free(mut ann: * mut crate::example1::genann) {
    /* The weight, output, and delta pointers go to the same buffer. */
    free(ann as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn genann_run(mut ann: * const crate::example1::genann,
                                    mut inputs: * const std::os::raw::c_double)
 -> * const std::os::raw::c_double {
    let mut w: * const f64 = (*ann).weight;
    let mut o: * mut f64 =
        (*ann).output.offset((*ann).inputs as isize);
    let mut i: * const f64 = (*ann).output;
    /* Copy the inputs to the scratch area, where we also store each neuron's
     * output, for consistency. This way the first layer isn't a special case. */
    memcpy((*ann).output as *mut std::os::raw::c_void, inputs as *const std::os::raw::c_void,
           (::std::mem::size_of::<std::os::raw::c_double>() as
                std::os::raw::c_ulong).wrapping_mul((*ann).inputs as std::os::raw::c_ulong));
    let mut h: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let act: Option<unsafe extern "C"  fn(_: f64,) -> f64> = (*ann).activation_hidden;
    let acto: Option<unsafe extern "C"  fn(_: f64,) -> f64> = (*ann).activation_output;
    /* Figure hidden layers, if any. */
    h = 0 as std::os::raw::c_int;
    while h < (*ann).hidden_layers {
        j = 0 as std::os::raw::c_int;
        while j < (*ann).hidden {
            let mut fresh0 = w;
            w = w.offset(1);
            let mut sum: f64 = *fresh0 * -1.0f64;
            k = 0 as std::os::raw::c_int;
            while k <
                      (if h == 0 as std::os::raw::c_int {
                           (*ann).inputs
                       } else { (*ann).hidden }) {
                let mut fresh1 = w;
                w = w.offset(1);
                sum += *fresh1 * *i.offset(k as isize);
                k += 1
            }
            let mut fresh2 = o;
            o = o.offset(1);
            *fresh2 = act.expect("non-null function pointer")(sum);
            j += 1
        }
        i =
            i.offset(if h == 0 as std::os::raw::c_int {
                         (*ann).inputs
                     } else { (*ann).hidden } as isize);
        h += 1
    }
    let mut ret: * const f64 = o;
    /* Figure output layer. */
    j = 0 as std::os::raw::c_int;
    while j < (*ann).outputs {
        let mut fresh3 = w;
        w = w.offset(1);
        let mut sum_0: f64 = *fresh3 * -1.0f64;
        k = 0 as std::os::raw::c_int;
        while k <
                  (if (*ann).hidden_layers != 0 {
                       (*ann).hidden
                   } else { (*ann).inputs }) {
            let mut fresh4 = w;
            w = w.offset(1);
            sum_0 += *fresh4 * *i.offset(k as isize);
            k += 1
        }
        let mut fresh5 = o;
        o = o.offset(1);
        *fresh5 = acto.expect("non-null function pointer")(sum_0);
        j += 1
    }
    /* Sanity check that we used all weights and wrote all outputs. */
    if !(w.offset_from((*ann).weight) as std::os::raw::c_long ==
             (*ann).total_weights as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 11], &'_ [i8; 11]>(b"genann_run\x00")).as_ptr(),
                     b"genann.c\x00" as *const u8 as *const std::os::raw::c_char,
                     225 as std::os::raw::c_int,
                     b"w - ann->weight == ann->total_weights\x00" as *const u8
                         as *const std::os::raw::c_char);
    } else { };
    if !(o.offset_from((*ann).output) as std::os::raw::c_long ==
             (*ann).total_neurons as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 11], &'_ [i8; 11]>(b"genann_run\x00")).as_ptr(),
                     b"genann.c\x00" as *const u8 as *const std::os::raw::c_char,
                     226 as std::os::raw::c_int,
                     b"o - ann->output == ann->total_neurons\x00" as *const u8
                         as *const std::os::raw::c_char);
    } else { };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_train(mut ann: * const crate::example1::genann,
                                      mut inputs: * const std::os::raw::c_double,
                                      mut desired_outputs:
                                          * const std::os::raw::c_double,
                                      mut learning_rate: std::os::raw::c_double) {
    /* To begin with, we must run the network forward. */
    genann_run(ann, inputs);
    let mut h: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    /* First set the output layer deltas. */
    let mut o: * const f64 =
        (*ann).output.offset((*ann).inputs as
                                 isize).offset(((*ann).hidden *
                                                    (*ann).hidden_layers) as
                                                   isize); /* First output. */
    let mut d: * mut f64 =
        (*ann).delta.offset(((*ann).hidden * (*ann).hidden_layers) as
                                isize); /* First delta. */
    let mut t: * const f64 =
        desired_outputs; /* First desired output. */
    if ((*ann).activation_output ).map(|f| f as usize) == (
           Some(genann_act_linear)).map(|f| f as usize)
       {
        j = 0 as std::os::raw::c_int;
        while j < (*ann).outputs {
            let mut fresh6 = t;
            t = t.offset(1);
            let mut fresh7 = o;
            o = o.offset(1);
            let mut fresh8 = d;
            d = d.offset(1);
            *fresh8 = *fresh6 - *fresh7;
            j += 1
        }
    } else {
        j = 0 as std::os::raw::c_int;
        while j < (*ann).outputs {
            let mut fresh9 = d;
            d = d.offset(1);
            *fresh9 = (*t - *o) * *o * (1.0f64 - *o);
            o = o.offset(1);
            t = t.offset(1);
            j += 1
        }
    }
    /* Set output layer deltas. */
    /* Set hidden layer deltas, start on last layer and work backwards. */
    /* Note that loop is skipped in the case of hidden_layers == 0. */
    h = (*ann).hidden_layers - 1 as std::os::raw::c_int;
    while h >= 0 as std::os::raw::c_int {
        /* Find first output and delta in this layer. */
        let mut o_0: * const f64 =
            (*ann).output.offset((*ann).inputs as
                                     isize).offset((h * (*ann).hidden) as
                                                       isize);
        let mut d_0: * mut f64 =
            (*ann).delta.offset((h * (*ann).hidden) as isize);
        /* Find first delta in following layer (which may be hidden or output). */
        let dd: * const f64 =
            (*ann).delta.offset(((h + 1 as std::os::raw::c_int) * (*ann).hidden) as
                                    isize);
        /* Find first weight in following layer (which may be hidden or output). */
        let ww: * const f64 =
            (*ann).weight.offset((((*ann).inputs + 1 as std::os::raw::c_int) *
                                      (*ann).hidden) as
                                     isize).offset((((*ann).hidden +
                                                         1 as std::os::raw::c_int) *
                                                        (*ann).hidden * h) as
                                                       isize);
        j = 0 as std::os::raw::c_int;
        while j < (*ann).hidden {
            let mut delta: f64 =
                0 as std::os::raw::c_int as std::os::raw::c_double;
            k = 0 as std::os::raw::c_int;
            while k <
                      (if h == (*ann).hidden_layers - 1 as std::os::raw::c_int {
                           (*ann).outputs
                       } else { (*ann).hidden }) {
                let forward_delta: f64 = *dd.offset(k as isize);
                let windex: i32 =
                    k * ((*ann).hidden + 1 as std::os::raw::c_int) +
                        (j + 1 as std::os::raw::c_int);
                let forward_weight: f64 =
                    *ww.offset(windex as isize);
                delta += forward_delta * forward_weight;
                k += 1
            }
            *d_0 = *o_0 * (1.0f64 - *o_0) * delta;
            d_0 = d_0.offset(1);
            o_0 = o_0.offset(1);
            j += 1
        }
        h -= 1
    }
    /* Train the outputs. */
    /* Find first output delta. */
    let mut d_1: * const f64 =
        (*ann).delta.offset(((*ann).hidden * (*ann).hidden_layers) as
                                isize); /* First output delta. */
    let mut w: * mut f64 =
        (*ann).weight.offset((if (*ann).hidden_layers != 0 {
                                  (((*ann).inputs + 1 as std::os::raw::c_int) *
                                       (*ann).hidden) +
                                      ((*ann).hidden + 1 as std::os::raw::c_int) *
                                          (*ann).hidden *
                                          ((*ann).hidden_layers -
                                               1 as std::os::raw::c_int)
                              } else { 0 as std::os::raw::c_int }) as isize);
    let i: * const f64 =
        (*ann).output.offset((if (*ann).hidden_layers != 0 {
                                  ((*ann).inputs) +
                                      (*ann).hidden *
                                          ((*ann).hidden_layers -
                                               1 as std::os::raw::c_int)
                              } else { 0 as std::os::raw::c_int }) as isize);
    j = 0 as std::os::raw::c_int;
    while j < (*ann).outputs {
        k = 0 as std::os::raw::c_int;
        while k <
                  (if (*ann).hidden_layers != 0 {
                       (*ann).hidden
                   } else { (*ann).inputs }) + 1 as std::os::raw::c_int {
            if k == 0 as std::os::raw::c_int {
                let mut fresh10 = w;
                w = w.offset(1);
                *fresh10 += *d_1 * learning_rate * -1.0f64
            } else {
                let mut fresh11 = w;
                w = w.offset(1);
                *fresh11 +=
                    *d_1 * learning_rate *
                        *i.offset((k - 1 as std::os::raw::c_int) as isize)
            }
            k += 1
        }
        d_1 = d_1.offset(1);
        j += 1
    }
    if !(w.offset_from((*ann).weight) as std::os::raw::c_long ==
             (*ann).total_weights as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 13], &'_ [i8; 13]>(b"genann_train\x00")).as_ptr(),
                     b"genann.c\x00" as *const u8 as *const std::os::raw::c_char,
                     318 as std::os::raw::c_int,
                     b"w - ann->weight == ann->total_weights\x00" as *const u8
                         as *const std::os::raw::c_char);
    } else { };
    /* Find first weight to first output delta. */
    /* Find first output in previous layer. */
    /* Set output layer weights. */
    /* Train the hidden layers. */
    h = (*ann).hidden_layers - 1 as std::os::raw::c_int;
    while h >= 0 as std::os::raw::c_int {
        /* Find first delta in this layer. */
        let mut d_2: * const f64 =
            (*ann).delta.offset((h * (*ann).hidden) as isize);
        /* Find first input to this layer. */
        let mut i_0: * const f64 =
            (*ann).output.offset((if h != 0 {
                                      ((*ann).inputs) +
                                          (*ann).hidden *
                                              (h - 1 as std::os::raw::c_int)
                                  } else { 0 as std::os::raw::c_int }) as isize);
        /* Find first weight to this layer. */
        let mut w_0: * mut f64 =
            (*ann).weight.offset((if h != 0 {
                                      (((*ann).inputs + 1 as std::os::raw::c_int) *
                                           (*ann).hidden) +
                                          ((*ann).hidden + 1 as std::os::raw::c_int) *
                                              (*ann).hidden *
                                              (h - 1 as std::os::raw::c_int)
                                  } else { 0 as std::os::raw::c_int }) as isize);
        j = 0 as std::os::raw::c_int;
        while j < (*ann).hidden {
            k = 0 as std::os::raw::c_int;
            while k <
                      (if h == 0 as std::os::raw::c_int {
                           (*ann).inputs
                       } else { (*ann).hidden }) + 1 as std::os::raw::c_int {
                if k == 0 as std::os::raw::c_int {
                    let mut fresh12 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh12 += *d_2 * learning_rate * -1.0f64
                } else {
                    let mut fresh13 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh13 +=
                        *d_2 * learning_rate *
                            *i_0.offset((k - 1 as std::os::raw::c_int) as isize)
                }
                k += 1
            }
            d_2 = d_2.offset(1);
            j += 1
        }
        h -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn genann_write(mut ann: * const crate::example1::genann,
                                      mut out: * mut crate::example3::__sFILE) {
    fprintf(out, b"%d %d %d %d\x00" as *const u8 as *const std::os::raw::c_char,
            (*ann).inputs, (*ann).hidden_layers, (*ann).hidden,
            (*ann).outputs);
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*ann).total_weights {
        fprintf(out, b" %.20e\x00" as *const u8 as *const std::os::raw::c_char,
                *(*ann).weight.offset(i as isize));
        i += 1
    };
}
use crate::laertes_rt::*;

}

pub mod example1 {

extern "C" {
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub type genann_actfun
    =
    Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: std::os::raw::c_int,
    pub hidden_layers: std::os::raw::c_int,
    pub hidden: std::os::raw::c_int,
    pub outputs: std::os::raw::c_int,
    pub activation_hidden: Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>,
    pub activation_output: Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>,
    pub total_weights: std::os::raw::c_int,
    pub total_neurons: std::os::raw::c_int,
    pub weight: * mut std::os::raw::c_double,
    pub output: * mut std::os::raw::c_double,
    pub delta: * mut std::os::raw::c_double,
}
impl genann {
    pub const fn new() -> Self {
        genann {
        inputs: 0,
        hidden_layers: 0,
        hidden: 0,
        outputs: 0,
        activation_hidden: None,
        activation_output: None,
        total_weights: 0,
        total_neurons: 0,
        weight: (0 as * mut std::os::raw::c_double),
        output: (0 as * mut std::os::raw::c_double),
        delta: (0 as * mut std::os::raw::c_double)
        }
    }
}

impl std::default::Default for genann {
    fn default() -> Self { genann::new() }
}

unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: * mut * mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 1.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train a small ANN to the XOR function using backpropagation.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    /* Input and expected out data for the XOR function. */
    let input: [[f64; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let output: [f64; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    let mut i: i32 = 0;
    /* New network with 2 inputs,
     * 1 hidden layer of 2 neurons,
     * and 1 output. */
    let mut ann: * mut crate::example1::genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    /* Train on the four labeled data points many times. */
    i = 0 as std::os::raw::c_int;
    while i < 300 as std::os::raw::c_int {
        genann_train(ann, input[0 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(0 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[1 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(1 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[2 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(2 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[3 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(3 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        i += 1
    }
    /* Run the network and see what it predicts. */
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()));
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
use crate::laertes_rt::*;

}

pub mod example2 {

extern "C" {
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn rand() -> std::os::raw::c_int;
    #[no_mangle]
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_copy;
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_randomize;
pub use crate::genann::genann_run;
pub type genann_actfun = Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: * mut * mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 2.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train a small ANN to the XOR function using random search.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    /* Input and expected out data for the XOR function. */
    let input: [[f64; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let output: [f64; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    let mut i: i32 = 0;
    /* New network with 2 inputs,
     * 1 hidden layer of 2 neurons,
     * and 1 output. */
    let mut ann: * mut crate::example1::genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut err: f64 = 0.;
    let mut last_err: f64 = 1000 as std::os::raw::c_int as std::os::raw::c_double;
    let mut count: i32 = 0 as std::os::raw::c_int;
    loop  {
        count += 1;
        if count % 1000 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            /* We're stuck, start over. */
            genann_randomize(ann);
        }
        let mut save: * mut crate::example1::genann = genann_copy(ann);
        /* Take a random guess at the ANN weights. */
        i = 0 as std::os::raw::c_int;
        while i < (*ann).total_weights {
            *(*ann).weight.offset(i as isize) +=
                rand() as std::os::raw::c_double /
                    0x7fffffff as std::os::raw::c_int as std::os::raw::c_double - 0.5f64;
            i += 1
        }
        /* See how we did. */
        err = 0 as std::os::raw::c_int as std::os::raw::c_double;
        err +=
            pow(*genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()) -
                    output[0 as std::os::raw::c_int as usize], 2.0f64);
        err +=
            pow(*genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()) -
                    output[1 as std::os::raw::c_int as usize], 2.0f64);
        err +=
            pow(*genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()) -
                    output[2 as std::os::raw::c_int as usize], 2.0f64);
        err +=
            pow(*genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()) -
                    output[3 as std::os::raw::c_int as usize], 2.0f64);
        /* Keep these weights if they're an improvement. */
        if err < last_err {
            genann_free(save);
            last_err = err
        } else { genann_free(ann); ann = save }
        if !(err > 0.01f64) { break ; }
    }
    printf(b"Finished in %d loops.\n\x00" as *const u8 as *const std::os::raw::c_char,
           count);
    /* Run the network and see what it predicts. */
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()));
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
use crate::laertes_rt::*;

}

pub mod example3 {

extern "C" {
    
    #[no_mangle]
    fn fclose(_: * mut crate::example3::__sFILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::example3::__sFILE;
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_read;
pub use crate::genann::genann_run;
pub use crate::genann::__sFILEX;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = std::os::raw::c_longlong;
pub type fpos_t = std::os::raw::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: * mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
impl __sbuf {
    pub const fn new() -> Self {
        __sbuf {
        _base: (0 as * mut std::os::raw::c_uchar),
        _size: 0
        }
    }
}

impl std::default::Default for __sbuf {
    fn default() -> Self { __sbuf::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: * mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: crate::example3::__sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: * mut core::ffi::c_void,
    pub _close: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut std::os::raw::c_char,_: std::os::raw::c_int,) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_longlong,_: std::os::raw::c_int,) -> std::os::raw::c_longlong>,
    pub _write: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const std::os::raw::c_char,_: std::os::raw::c_int,) -> std::os::raw::c_int>,
    pub _ub: crate::example3::__sbuf,
    pub _extra: * mut crate::genann::__sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: crate::example3::__sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: std::os::raw::c_longlong,
}
impl __sFILE {
    pub const fn new() -> Self {
        __sFILE {
        _p: (0 as * mut std::os::raw::c_uchar),
        _r: 0,
        _w: 0,
        _flags: 0,
        _file: 0,
        _bf: crate::example3::__sbuf::new(),
        _lbfsize: 0,
        _cookie: (0 as * mut core::ffi::c_void),
        _close: None,
        _read: None,
        _seek: None,
        _write: None,
        _ub: crate::example3::__sbuf::new(),
        _extra: (0 as * mut crate::genann::__sFILEX),
        _ur: 0,
        _ubuf: [0,0,0,],
        _nbuf: [0,],
        _lb: crate::example3::__sbuf::new(),
        _blksize: 0,
        _offset: 0
        }
    }
}

impl std::default::Default for __sFILE {
    fn default() -> Self { __sFILE::new() }
}

pub type FILE = crate::example3::__sFILE;
pub type genann_actfun = Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
#[no_mangle]
pub static mut save_name: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_save_name() {
save_name = b"example/xor.ann\x00" as *const u8 as *const std::os::raw::c_char;}//;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: * mut * mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 3.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Load a saved ANN to solve the XOR function.\n\x00" as *const u8
               as *const std::os::raw::c_char);
    let mut saved: * mut crate::example3::__sFILE =
        fopen(save_name, b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if saved.is_null() {
        printf(b"Couldn\'t open file: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, save_name);
        exit(1 as std::os::raw::c_int);
    }
    let mut ann: * mut crate::example1::genann = genann_read(saved);
    fclose(saved);
    if ann.is_null() {
        printf(b"Error loading ANN from file: %s.\x00" as *const u8 as
                   *const std::os::raw::c_char, save_name);
        exit(1 as std::os::raw::c_int);
    }
    /* Input data for the XOR function. */
    let input: [[f64; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    /* Run the network and see what it predicts. */
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()));
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
use crate::laertes_rt::*;

}

pub mod example4 {

extern "C" {
    
    #[no_mangle]
    fn fclose(_: * mut crate::example3::__sFILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn feof(_: * mut crate::example3::__sFILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgets(_: * mut std::os::raw::c_char, _: std::os::raw::c_int, _: * mut crate::example3::__sFILE)
     -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::example3::__sFILE;
    #[no_mangle]
    fn fseek(_: * mut crate::example3::__sFILE, _: std::os::raw::c_long, _: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(_: * const std::os::raw::c_char);
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn atof(_: * const std::os::raw::c_char) -> std::os::raw::c_double;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: * const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strtok(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char)
     -> * mut std::os::raw::c_char;
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub use crate::genann::__sFILEX;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = std::os::raw::c_longlong;
pub type fpos_t = std::os::raw::c_longlong;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example3::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example3::__sFILE;
pub type FILE = crate::example3::__sFILE;
pub type genann_actfun = Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
/* This example is to illustrate how to use GENANN.
 * It is NOT an example of good machine learning techniques.
 */
#[no_mangle]
pub static mut iris_data: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_iris_data() {
iris_data = b"example/iris.data\x00" as *const u8 as *const std::os::raw::c_char;}//;
#[no_mangle]
pub static mut input: * mut std::os::raw::c_double =
    (0 as * mut std::os::raw::c_double); unsafe fn laertes_init_input() {
input = 0 as *const std::os::raw::c_double as *mut std::os::raw::c_double;}//;
#[no_mangle]
pub static mut class: * mut std::os::raw::c_double =
    (0 as * mut std::os::raw::c_double); unsafe fn laertes_init_class() {
class = (0 as * mut f64);}//;
#[no_mangle]
pub static mut samples: std::os::raw::c_int = 0; unsafe fn laertes_init_samples() {
samples = 0;}//;
#[no_mangle]
pub static mut class_names: [* const std::os::raw::c_char; 3] =
    [(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),]; unsafe fn laertes_init_class_names() {
class_names = [b"Iris-setosa\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-versicolor\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-virginica\x00" as *const u8 as *const std::os::raw::c_char];}//;
#[no_mangle]
pub unsafe extern "C" fn load_data() {
    /* Load the iris data-set. */
    let mut in_0: * mut crate::example3::__sFILE =
        fopen(b"example/iris.data\x00" as *const u8 as *const std::os::raw::c_char,
              b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if in_0.is_null() {
        printf(b"Could not open file: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, iris_data);
        exit(1 as std::os::raw::c_int);
    }
    /* Loop through the data to get a count. */
    let mut line: [i8; 1024] = [0; 1024];
    while feof(in_0) == 0 &&
              !fgets(line.as_mut_ptr(), 1024 as std::os::raw::c_int, in_0).is_null() {
        samples += 1
    }
    fseek(in_0, 0 as std::os::raw::c_int as std::os::raw::c_long, 0 as std::os::raw::c_int);
    printf(b"Loading %d data points from %s\n\x00" as *const u8 as
               *const std::os::raw::c_char, samples, iris_data);
    /* Allocate memory for input and output data. */
    input =
        malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                    std::os::raw::c_ulong).wrapping_mul(samples as
                                                    std::os::raw::c_ulong).wrapping_mul(4
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_ulong))
            as *mut std::os::raw::c_double;
    class =
        malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                    std::os::raw::c_ulong).wrapping_mul(samples as
                                                    std::os::raw::c_ulong).wrapping_mul(3
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_ulong))
            as *mut std::os::raw::c_double;
    /* Read the file into our arrays. */
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < samples {
        let mut p: * mut f64 =
            input.offset((i * 4 as std::os::raw::c_int) as isize);
        let mut c: * mut f64 =
            class.offset((i * 3 as std::os::raw::c_int) as isize);
        let ref mut fresh0 = *c.offset(2 as std::os::raw::c_int as isize);
        *fresh0 = 0.0f64;
        let ref mut fresh1 = *c.offset(1 as std::os::raw::c_int as isize);
        *fresh1 = *fresh0;
        *c.offset(0 as std::os::raw::c_int as isize) = *fresh1;
        if fgets(line.as_mut_ptr(), 1024 as std::os::raw::c_int, in_0).is_null() {
            perror(b"fgets\x00" as *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        let mut split: * mut i8 =
            strtok(line.as_mut_ptr(),
                   b",\x00" as *const u8 as *const std::os::raw::c_char);
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            *p.offset(j as isize) = atof(split);
            split =
                strtok(0 as *mut std::os::raw::c_char,
                       b",\x00" as *const u8 as *const std::os::raw::c_char);
            j += 1
        }
        *split.offset(strlen(split).wrapping_sub(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
            = 0 as std::os::raw::c_int as std::os::raw::c_char;
        if strcmp(split, class_names[0 as std::os::raw::c_int as usize]) ==
               0 as std::os::raw::c_int {
            *c.offset(0 as std::os::raw::c_int as isize) = 1.0f64
        } else if strcmp(split, class_names[1 as std::os::raw::c_int as usize]) ==
                      0 as std::os::raw::c_int {
            *c.offset(1 as std::os::raw::c_int as isize) = 1.0f64
        } else if strcmp(split, class_names[2 as std::os::raw::c_int as usize]) ==
                      0 as std::os::raw::c_int {
            *c.offset(2 as std::os::raw::c_int as isize) = 1.0f64
        } else {
            printf(b"Unknown class %s.\n\x00" as *const u8 as
                       *const std::os::raw::c_char, split);
            exit(1 as std::os::raw::c_int);
        }
        i += 1
        /* printf("Data point %d is %f %f %f %f  ->   %f %f %f\n", i, p[0], p[1], p[2], p[3], c[0], c[1], c[2]); */
    }
    fclose(in_0);
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: * mut * mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 4.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train an ANN on the IRIS dataset using backpropagation.\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    /* Load the data from file. */
    load_data();
    /* 4 inputs.
     * 1 hidden layer(s) of 4 neurons.
     * 3 outputs (1 per class)
     */
    let mut ann: * mut crate::example1::genann =
        genann_init(4 as std::os::raw::c_int, 1 as std::os::raw::c_int, 4 as std::os::raw::c_int,
                    3 as std::os::raw::c_int);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut loops: i32 = 5000 as std::os::raw::c_int;
    /* Train the network with backpropagation. */
    printf(b"Training for %d loops over data.\n\x00" as *const u8 as
               *const std::os::raw::c_char, loops);
    i = 0 as std::os::raw::c_int;
    while i < loops {
        j = 0 as std::os::raw::c_int;
        while j < samples {
            genann_train(ann, input.offset((j * 4 as std::os::raw::c_int) as isize),
                         class.offset((j * 3 as std::os::raw::c_int) as isize),
                         0.01f64);
            j += 1
        }
        i += 1
        /* printf("%1.2f ", xor_score(ann)); */
    }
    let mut correct: i32 = 0 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    while j < samples {
        let mut guess: * const f64 =
            genann_run(ann, input.offset((j * 4 as std::os::raw::c_int) as isize));
        if *class.offset((j * 3 as std::os::raw::c_int + 0 as std::os::raw::c_int) as isize)
               == 1.0f64 {
            if *guess.offset(0 as std::os::raw::c_int as isize) >
                   *guess.offset(1 as std::os::raw::c_int as isize) &&
                   *guess.offset(0 as std::os::raw::c_int as isize) >
                       *guess.offset(2 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else if *class.offset((j * 3 as std::os::raw::c_int + 1 as std::os::raw::c_int) as
                                    isize) == 1.0f64 {
            if *guess.offset(1 as std::os::raw::c_int as isize) >
                   *guess.offset(0 as std::os::raw::c_int as isize) &&
                   *guess.offset(1 as std::os::raw::c_int as isize) >
                       *guess.offset(2 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else if *class.offset((j * 3 as std::os::raw::c_int + 2 as std::os::raw::c_int) as
                                    isize) == 1.0f64 {
            if *guess.offset(2 as std::os::raw::c_int as isize) >
                   *guess.offset(0 as std::os::raw::c_int as isize) &&
                   *guess.offset(2 as std::os::raw::c_int as isize) >
                       *guess.offset(1 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else {
            printf(b"Logic error.\n\x00" as *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        j += 1
    }
    printf(b"%d/%d correct (%0.1f%%).\n\x00" as *const u8 as
               *const std::os::raw::c_char, correct, samples,
           correct as std::os::raw::c_double / samples as std::os::raw::c_double * 100.0f64);
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
use crate::laertes_rt::*;

}

pub mod test {

extern "C" {
    
    #[no_mangle]
    fn fclose(_: * mut crate::example3::__sFILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::example3::__sFILE;
    
    
    
    
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn clock() -> std::os::raw::c_ulong;
    #[no_mangle]
    fn srand(_: std::os::raw::c_uint);
}
pub use crate::genann::genann_act_sigmoid;
pub use crate::genann::genann_act_sigmoid_cached;
pub use crate::genann::genann_act_threshold;
pub use crate::genann::genann_copy;
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_randomize;
pub use crate::genann::genann_read;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub use crate::genann::genann_write;
pub use crate::genann::__sFILEX;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_clock_t = std::os::raw::c_ulong;
pub type __darwin_off_t = std::os::raw::c_longlong;
pub type fpos_t = std::os::raw::c_longlong;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example3::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example3::__sFILE;
pub type FILE = crate::example3::__sFILE;
pub type genann_actfun = Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
pub type clock_t = std::os::raw::c_ulong;
static mut lfails: std::os::raw::c_int = 0; unsafe fn laertes_init_lfails() {
lfails = 0 as std::os::raw::c_int;}//;
static mut ltests: std::os::raw::c_int = 0; unsafe fn laertes_init_ltests() {
ltests = 0 as std::os::raw::c_int;}//;
/*
 * GENANN - Minimal C Artificial Neural Network
 *
 * Copyright (c) 2015, 2016 Lewis Van Winkle
 *
 * http://CodePlea.com
 *
 * This software is provided 'as-is', without any express or implied
 * warranty. In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgement in the product documentation would be
 *    appreciated but is not required.
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 * 3. This notice may not be removed or altered from any source distribution.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn basic() {
    let mut ann: * mut crate::example1::genann =
        genann_init(1 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    ltests += 1;
    if (*ann).total_weights != 2 as std::os::raw::c_int {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               37 as std::os::raw::c_int, (*ann).total_weights, 2 as std::os::raw::c_int);
    }
    let mut a: f64 = 0.;
    a = 0 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               44 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 1 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               47 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 11 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               50 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               55 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 10 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(1.0f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               60 as std::os::raw::c_int, 1.0f64, *genann_run(ann, &mut a));
    }
    a = -(10 as std::os::raw::c_int) as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.0f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               63 as std::os::raw::c_int, 0.0f64, *genann_run(ann, &mut a));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn xor() {
    let mut ann: * mut crate::example1::genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    (*ann).activation_hidden =
        Some(genann_act_threshold);
    (*ann).activation_output =
        Some(genann_act_threshold);
    ltests += 1;
    if (*ann).total_weights != 9 as std::os::raw::c_int {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               74 as std::os::raw::c_int, (*ann).total_weights, 9 as std::os::raw::c_int);
    }
    /* First hidden. */
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) = 0.5f64;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(2 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    /* Second hidden. */
    *(*ann).weight.offset(3 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(4 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(5 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    /* Output. */
    *(*ann).weight.offset(6 as std::os::raw::c_int as isize) = 0.5f64;
    *(*ann).weight.offset(7 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(8 as std::os::raw::c_int as isize) =
        -(1 as std::os::raw::c_int) as std::os::raw::c_double;
    let mut input: [[f64; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [f64; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               95 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               96 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               97 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               98 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn backprop() {
    let mut ann: * mut crate::example1::genann =
        genann_init(1 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut input: f64 = 0.;
    let mut output: f64 = 0.;
    input = 0.5f64;
    output = 1 as std::os::raw::c_int as std::os::raw::c_double;
    let mut first_try: f64 = *genann_run(ann, &mut input);
    genann_train(ann, &mut input, &mut output, 0.5f64);
    let mut second_try: f64 = *genann_run(ann, &mut input);
    ltests += 1;
    if !(fabs(first_try - output) > fabs(second_try - output)) {
        lfails += 1;
        printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               114 as std::os::raw::c_int);
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_and() {
    let mut input: [[f64; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [f64; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double];
    let mut ann: * mut crate::example1::genann =
        genann_init(2 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 50 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            genann_train(ann, input[j as usize].as_mut_ptr(),
                         output.as_mut_ptr().offset(j as isize), 0.8f64);
            j += 1
        }
        i += 1
    }
    (*ann).activation_output =
        Some(genann_act_threshold);
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               135 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               136 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               137 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               138 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_or() {
    let mut input: [[f64; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [f64; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double];
    let mut ann: * mut crate::example1::genann =
        genann_init(2 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    genann_randomize(ann);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 50 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            genann_train(ann, input[j as usize].as_mut_ptr(),
                         output.as_mut_ptr().offset(j as isize), 0.8f64);
            j += 1
        }
        i += 1
    }
    (*ann).activation_output =
        Some(genann_act_threshold);
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               160 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               161 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               162 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               163 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_xor() {
    let mut input: [[f64; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [f64; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    let mut ann: * mut crate::example1::genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 500 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            genann_train(ann, input[j as usize].as_mut_ptr(),
                         output.as_mut_ptr().offset(j as isize),
                         3 as std::os::raw::c_int as std::os::raw::c_double);
            j += 1
        }
        i += 1
        /* printf("%1.2f ", xor_score(ann)); */
    }
    (*ann).activation_output =
        Some(genann_act_threshold);
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               186 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               187 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               188 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               189 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn persist() {
    let mut first: * mut crate::example1::genann =
        genann_init(1000 as std::os::raw::c_int, 5 as std::os::raw::c_int, 50 as std::os::raw::c_int,
                    10 as std::os::raw::c_int);
    let mut out: * mut crate::example3::__sFILE =
        fopen(b"persist.txt\x00" as *const u8 as *const std::os::raw::c_char,
              b"w\x00" as *const u8 as *const std::os::raw::c_char);
    genann_write(first, out);
    fclose(out);
    let mut in_0: * mut crate::example3::__sFILE =
        fopen(b"persist.txt\x00" as *const u8 as *const std::os::raw::c_char,
              b"r\x00" as *const u8 as *const std::os::raw::c_char);
    let mut second: * mut crate::example1::genann = genann_read(in_0);
    fclose(out);
    ltests += 1;
    if (*first).inputs != (*second).inputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               208 as std::os::raw::c_int, (*first).inputs, (*second).inputs);
    }
    ltests += 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               209 as std::os::raw::c_int, (*first).hidden_layers,
               (*second).hidden_layers);
    }
    ltests += 1;
    if (*first).hidden != (*second).hidden {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               210 as std::os::raw::c_int, (*first).hidden, (*second).hidden);
    }
    ltests += 1;
    if (*first).outputs != (*second).outputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               211 as std::os::raw::c_int, (*first).outputs, (*second).outputs);
    }
    ltests += 1;
    if (*first).total_weights != (*second).total_weights {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               212 as std::os::raw::c_int, (*first).total_weights,
               (*second).total_weights);
    }
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*first).total_weights {
        ltests += 1;
        if !(*(*first).weight.offset(i as isize) ==
                 *(*second).weight.offset(i as isize)) {
            lfails += 1;
            printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
                   b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                   216 as std::os::raw::c_int);
        }
        i += 1
    }
    genann_free(first);
    genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn copy() {
    let mut first: * mut crate::example1::genann =
        genann_init(1000 as std::os::raw::c_int, 5 as std::os::raw::c_int, 50 as std::os::raw::c_int,
                    10 as std::os::raw::c_int);
    let mut second: * mut crate::example1::genann = genann_copy(first);
    ltests += 1;
    if (*first).inputs != (*second).inputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               229 as std::os::raw::c_int, (*first).inputs, (*second).inputs);
    }
    ltests += 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               230 as std::os::raw::c_int, (*first).hidden_layers,
               (*second).hidden_layers);
    }
    ltests += 1;
    if (*first).hidden != (*second).hidden {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               231 as std::os::raw::c_int, (*first).hidden, (*second).hidden);
    }
    ltests += 1;
    if (*first).outputs != (*second).outputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               232 as std::os::raw::c_int, (*first).outputs, (*second).outputs);
    }
    ltests += 1;
    if (*first).total_weights != (*second).total_weights {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               233 as std::os::raw::c_int, (*first).total_weights,
               (*second).total_weights);
    }
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*first).total_weights {
        ltests += 1;
        if fabs(*(*first).weight.offset(i as isize) -
                    *(*second).weight.offset(i as isize)) > 0.001f64 {
            lfails += 1;
            printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                   237 as std::os::raw::c_int, *(*first).weight.offset(i as isize),
                   *(*second).weight.offset(i as isize));
        }
        i += 1
    }
    genann_free(first);
    genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn sigmoid() {
    let mut i: f64 = -(20 as std::os::raw::c_int) as std::os::raw::c_double;
    let max: f64 = 20 as std::os::raw::c_int as std::os::raw::c_double;
    let d: f64 = 0.0001f64;
    while i < max {
        ltests += 1;
        if fabs(genann_act_sigmoid(i) - genann_act_sigmoid_cached(i)) >
               0.001f64 {
            lfails += 1;
            printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                   251 as std::os::raw::c_int, genann_act_sigmoid(i),
                   genann_act_sigmoid_cached(i));
        }
        i += d
    };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: * mut * mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN TEST SUITE\n\x00" as *const u8 as *const std::os::raw::c_char);
    srand(100 as std::os::raw::c_int as std::os::raw::c_uint);
    let ts: i32 = ltests;
    let fs: i32 = lfails;
    let start: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"basic\x00" as *const u8 as *const std::os::raw::c_char);
    basic();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts - (lfails - fs), lfails - fs,
           clock().wrapping_sub(start).wrapping_mul(1000 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong).wrapping_div(1000000
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_0: i32 = ltests;
    let fs_0: i32 = lfails;
    let start_0: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"xor\x00" as *const u8 as *const std::os::raw::c_char);
    xor();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_0 - (lfails - fs_0),
           lfails - fs_0,
           clock().wrapping_sub(start_0).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_1: i32 = ltests;
    let fs_1: i32 = lfails;
    let start_1: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"backprop\x00" as *const u8 as *const std::os::raw::c_char);
    backprop();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_1 - (lfails - fs_1),
           lfails - fs_1,
           clock().wrapping_sub(start_1).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_2: i32 = ltests;
    let fs_2: i32 = lfails;
    let start_2: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"train and\x00" as *const u8 as *const std::os::raw::c_char);
    train_and();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_2 - (lfails - fs_2),
           lfails - fs_2,
           clock().wrapping_sub(start_2).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_3: i32 = ltests;
    let fs_3: i32 = lfails;
    let start_3: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"train or\x00" as *const u8 as *const std::os::raw::c_char);
    train_or();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_3 - (lfails - fs_3),
           lfails - fs_3,
           clock().wrapping_sub(start_3).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_4: i32 = ltests;
    let fs_4: i32 = lfails;
    let start_4: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"train xor\x00" as *const u8 as *const std::os::raw::c_char);
    train_xor();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_4 - (lfails - fs_4),
           lfails - fs_4,
           clock().wrapping_sub(start_4).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_5: i32 = ltests;
    let fs_5: i32 = lfails;
    let start_5: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"persist\x00" as *const u8 as *const std::os::raw::c_char);
    persist();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_5 - (lfails - fs_5),
           lfails - fs_5,
           clock().wrapping_sub(start_5).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_6: i32 = ltests;
    let fs_6: i32 = lfails;
    let start_6: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"copy\x00" as *const u8 as *const std::os::raw::c_char);
    copy();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_6 - (lfails - fs_6),
           lfails - fs_6,
           clock().wrapping_sub(start_6).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_7: i32 = ltests;
    let fs_7: i32 = lfails;
    let start_7: u64 = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"sigmoid\x00" as *const u8 as *const std::os::raw::c_char);
    sigmoid();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_7 - (lfails - fs_7),
           lfails - fs_7,
           clock().wrapping_sub(start_7).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    if lfails == 0 as std::os::raw::c_int {
        printf(b"ALL TESTS PASSED (%d/%d)\n\x00" as *const u8 as
                   *const std::os::raw::c_char, ltests, ltests);
    } else {
        printf(b"SOME TESTS FAILED (%d/%d)\n\x00" as *const u8 as
                   *const std::os::raw::c_char, ltests - lfails, ltests);
    }
    return (lfails != 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
use crate::laertes_rt::*;

}

// root re-exports so the generated harness's `translated::<entry>` resolves
pub use crate::genann::genann_act_linear;
pub use crate::genann::genann_act_sigmoid;
pub use crate::genann::genann_act_sigmoid_cached;
pub use crate::genann::genann_act_threshold;
pub use crate::genann::genann_copy;
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_randomize;
pub use crate::genann::genann_read;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub use crate::genann::genann_write;
// support modules carried verbatim from the translation's own crate root
pub(crate) mod __laertes_array {
    use std::{
        cell::{Ref, RefCell, RefMut},
        cmp::Ordering,
        convert::{TryFrom, TryInto},
        marker::PhantomData,
    };

    pub trait Get<'g, T: 'g> {
        fn get(&'g self, index: SliceIndex) -> T;

        fn get_add(&'g self, index: usize) -> T {
            self.get(SliceIndex::Add(index))
        }

        fn get_sub(&'g self, index: usize) -> T {
            self.get(SliceIndex::Sub(index))
        }

        fn get_offset(&'g self, index: isize) -> T {
            self.get(SliceIndex::Offset(index))
        }
    }

    pub trait GetMut<'g, T: 'g> {
        fn get_mut(&'g mut self, index: SliceIndex) -> T;

        fn get_add_mut(&'g mut self, index: usize) -> T {
            self.get_mut(SliceIndex::Add(index))
        }

        fn get_sub_mut(&'g mut self, index: usize) -> T {
            self.get_mut(SliceIndex::Sub(index))
        }

        fn get_offset_mut(&'g mut self, index: isize) -> T {
            self.get_mut(SliceIndex::Offset(index))
        }
    }

    pub trait IntoSlice<S>: Sized {
        fn into_slice(self, index: SliceIndex) -> S;

        fn into_slice_add(self, index: usize) -> S {
            self.into_slice(SliceIndex::Add(index))
        }

        fn into_slice_sub(self, index: usize) -> S {
            self.into_slice(SliceIndex::Sub(index))
        }

        fn into_slice_offset(self, index: isize) -> S {
            self.into_slice(SliceIndex::Offset(index))
        }
    }

    pub trait Slice<'s, S: 's> {
        fn slice(&'s self, index: SliceIndex) -> S;

        fn slice_add(&'s self, index: usize) -> S {
            self.slice(SliceIndex::Add(index))
        }

        fn slice_sub(&'s self, index: usize) -> S {
            self.slice(SliceIndex::Sub(index))
        }

        fn slice_offset(&'s self, index: isize) -> S {
            self.slice(SliceIndex::Offset(index))
        }
    }

    pub trait SliceMut<'s, S: 's> {
        fn slice_mut(&'s mut self, index: SliceIndex) -> S;

        fn slice_add_mut(&'s mut self, index: usize) -> S {
            self.slice_mut(SliceIndex::Add(index))
        }

        fn slice_sub_mut(&'s mut self, index: usize) -> S {
            self.slice_mut(SliceIndex::Sub(index))
        }

        fn slice_offset_mut(&'s mut self, index: isize) -> S {
            self.slice_mut(SliceIndex::Offset(index))
        }
    }

    #[derive(Clone, Copy)]
    pub enum SliceIndex {
        Add(usize),
        Sub(usize),
        Offset(isize),
    }

    impl SliceIndex {
        fn apply(self, offset: usize) -> usize {
            match self {
                SliceIndex::Add(i) => offset.checked_add(i),
                SliceIndex::Sub(i) => offset.checked_sub(i),
                SliceIndex::Offset(i) => {
                    if i < 0 {
                        offset.checked_sub(i.wrapping_abs() as usize)
                    } else {
                        offset.checked_add(i as usize)
                    }
                },
            }
            .unwrap()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct CustomSlice<'a, T: 'a, S: 'a> {
        offset: usize,
        slice: S,
        m: PhantomData<&'a T>,
    }

    impl<'a, T: 'a, S: 'a> CustomSlice<'a, T, S> {
        pub const fn new(slice: S) -> CustomSlice<'a, T, S> {
            CustomSlice {
                offset: 0,
                slice,
                m: PhantomData,
            }
        }
    }

    impl<'a, T: 'a> CustomSlice<'a, T, Box<[T]>> {
        pub fn boxed_from_vec(vec: Vec<T>) -> CustomSlice<'a, T, Box<[T]>> {
            CustomSlice {
                offset: 0,
                slice: vec.into_boxed_slice(),
                m: PhantomData::default(),
            }
        }
    }

    impl<'a, T: 'a, S: TryFrom<Vec<RefCell<T>>> + 'a> CustomSlice<'a, T, S> {
        pub fn from_vec(vec: Vec<T>) -> CustomSlice<'a, T, S> {
            CustomSlice::new(
                vec.into_iter()
                    .map(|v| RefCell::new(v))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!("couldn't construct backing slice")),
            )
        }
    }

    impl<'a, T, S> IntoSlice<CustomSlice<'a, T, S>> for CustomSlice<'a, T, S> {
        fn into_slice(self, index: SliceIndex) -> CustomSlice<'a, T, S> {
            let CustomSlice { offset, slice, m } = self;

            CustomSlice {
                offset: index.apply(offset),
                slice,
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsRef<[T]> + 's> Slice<'s, CustomSlice<'s, T, &'s [T]>>
        for CustomSlice<'a, T, S>
    {
        fn slice(&'s self, index: SliceIndex) -> CustomSlice<'s, T, &'s [T]> {
            let CustomSlice {
                offset,
                ref slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_ref(),
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsRef<[RefCell<T>]> + 's> Slice<'s, CustomSlice<'s, T, &'s [RefCell<T>]>>
        for CustomSlice<'a, T, S>
    {
        fn slice(&'s self, index: SliceIndex) -> CustomSlice<'s, T, &'s [RefCell<T>]> {
            let CustomSlice {
                offset,
                ref slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_ref(),
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsMut<[T]> + 's> SliceMut<'s, CustomSlice<'s, T, &'s mut [T]>>
        for CustomSlice<'a, T, S>
    {
        fn slice_mut(&'s mut self, index: SliceIndex) -> CustomSlice<'s, T, &'s mut [T]> {
            let CustomSlice {
                offset,
                ref mut slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_mut(),
                m,
            }
        }
    }

    impl<'a, 'g, T, S: AsRef<[T]> + 'g> Get<'g, &'g T> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> &'g T {
            &self.slice.as_ref()[index.apply(self.offset)]
        }
    }

    impl<'a, 'g, T, S: AsMut<[T]> + 'g> GetMut<'g, &'g mut T> for CustomSlice<'a, T, S> {
        fn get_mut(&'g mut self, index: SliceIndex) -> &'g mut T {
            &mut self.slice.as_mut()[index.apply(self.offset)]
        }
    }

    impl<'a, 'g, T, S: AsRef<[RefCell<T>]> + 'g> Get<'g, Ref<'g, T>> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> Ref<'g, T> {
            self.slice.as_ref()[index.apply(self.offset)].borrow()
        }
    }

    impl<'a, 'g, T, S: AsRef<[RefCell<T>]> + 'g> Get<'g, RefMut<'g, T>> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> RefMut<'g, T> {
            self.slice.as_ref()[index.apply(self.offset)].borrow_mut()
        }
    }

    pub trait GetPtr<T: ?Sized> {
        fn get_ptr(&self) -> *const ();
    }

    impl<T: ?Sized> GetPtr<T> for &T {
        fn get_ptr(&self) -> *const () {
            *self as *const T as *const ()
        }
    }

    impl<T: ?Sized> GetPtr<T> for &mut T {
        fn get_ptr(&self) -> *const () {
            *self as *const T as *const ()
        }
    }

    impl<T> GetPtr<T> for &[T] {
        fn get_ptr(&self) -> *const () {
            self.as_ptr() as *const ()
        }
    }

    impl<T> GetPtr<T> for &mut [T] {
        fn get_ptr(&self) -> *const () {
            self.as_ptr() as *const ()
        }
    }

    impl<T> GetPtr<T> for Box<[T]> {
        fn get_ptr(&self) -> *const () {
            (*self).as_ptr() as *const ()
        }
    }

    impl<T: ?Sized> GetPtr<T> for Box<T> {
        fn get_ptr(&self) -> *const () {
            self.as_ref() as *const T as *const ()
        }
    }

    pub fn offset_from<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> isize {
        if l.is_none() && r.is_none() {
            return 0; // case for NULL - NULL
        }

        l.as_ref()
            .and_then(|l| {
                r.as_ref().map(|r| {
                    if l.slice.get_ptr() == r.slice.get_ptr() {
                        l.offset as isize - r.offset as isize
                    } else {
                        panic!("trying to compute offsets from two unrelated slices");
                    }
                })
            })
            .expect("tried to compare non-null and null pointers")
    }

    pub fn partial_cmp<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<Ordering> {
        l.as_ref().and_then(|l| {
            r.as_ref().and_then(|r| {
                if l.slice.get_ptr() == r.slice.get_ptr() {
                    Some(l.offset.cmp(&r.offset))
                } else {
                    None
                }
            })
        })
    }

    pub fn partial_lt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c == Ordering::Less)
    }
    pub fn partial_gt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c == Ordering::Greater)
    }
    pub fn partial_le<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c != Ordering::Greater)
    }
    pub fn partial_ge<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c != Ordering::Less)
    }

    pub fn compare_eq<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_cmp(l, r).map_or(false, |c| c == Ordering::Equal)
    }
    pub fn compare_ne<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        !compare_eq(l, r)
    }

    pub fn compare_lt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_lt(l, r).unwrap()
    }
    pub fn compare_gt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_gt(l, r).unwrap()
    }
    pub fn compare_le<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_le(l, r).unwrap()
    }
    pub fn compare_ge<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_ge(l, r).unwrap()
    }

    pub fn borrow<'a, 'b: 'a, S: Slice<'a, T>, T: 'a>(s: &'b Option<S>) -> Option<T> {
        s.as_ref().map(|s| s.slice_add(0))
    }
    pub fn borrow_mut<'a, 'b: 'a, S: SliceMut<'a, T>, T: 'a>(s: &'b mut Option<S>) -> Option<T> {
        s.as_mut().map(|s| s.slice_add_mut(0))
    }
}
pub(crate) mod laertes_rt {
    use super::__laertes_array::GetPtr;

    pub trait Borrow<'b, T: ?Sized> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a;
    }

    pub trait BorrowMut<'b, T: ?Sized> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a;
    }

    impl<'b, T: ?Sized> Borrow<'b, T> for Option<&'b mut T> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a,
        {
            self.as_ref().map(|x| &**x)
        }
    }

    impl<'b, T: ?Sized> BorrowMut<'b, T> for Option<&'b mut T> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a,
        {
            self.as_mut().map(|x| &mut **x)
        }
    }

    impl<'b, T: ?Sized> Borrow<'b, T> for Option<Box<T>> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a,
        {
            self.as_ref().map(|x| x.as_ref())
        }
    }

    impl<'b, T: ?Sized> BorrowMut<'b, T> for Option<Box<T>> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a,
        {
            self.as_mut().map(|x| x.as_mut())
        }
    }

    pub fn borrow<'a, 'b: 'a, T: ?Sized, P: Borrow<'b, T>>(p: &'a P) -> Option<&'a T> {
        Borrow::borrow(p)
    }

    pub fn borrow_mut<'a, 'b: 'a, T: ?Sized, P: BorrowMut<'b, T>>(
        p: &'a mut P,
    ) -> Option<&'a mut T> {
        BorrowMut::borrow_mut(p)
    }

    pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
        p.as_ref().map(|x| x.as_ref())
    }

    pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
        p.as_mut().map(|x| x.as_mut())
    }

    pub fn option_to_raw<T: ?Sized, P: GetPtr<T>>(p: &Option<P>) -> *const () {
        p.as_ref()
            .map_or(core::ptr::null(), |p| p.get_ptr() as *const ())
    }

    pub fn _ref_eq<T: ?Sized, L: GetPtr<T>, R: GetPtr<T>>(p: Option<L>, q: Option<R>) -> bool {
        option_to_raw(&p) == option_to_raw(&q)
    }

    pub fn _ref_ne<T: ?Sized, L: GetPtr<T>, R: GetPtr<T>>(p: Option<L>, q: Option<R>) -> bool {
        option_to_raw(&p) != option_to_raw(&q)
    }

    // functions to convert references to raw pointers
    pub fn _as_ptr<T, P: GetPtr<T>>(r: &Option<P>) -> *const T {
        option_to_raw(r) as *const T
    }

    pub fn _as_mut_ptr<T, P: GetPtr<T>>(r: &mut Option<P>) -> *mut T {
        option_to_raw(r) as *const T as *mut T
    }

    pub fn _move_to_ptr<T>(b: Option<Box<T>>) -> *mut T {
        b.map_or(core::ptr::null_mut(), Box::into_raw)
    }

    pub fn trans<'a, 'b, T: 'a, U: 'b>(_: T) -> U
    where
        'a: 'b,
    {
        loop {}
    }
}


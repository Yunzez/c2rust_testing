
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

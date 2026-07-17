
extern "C" {
    
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cmo_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cmo(mut size: std::os::raw::c_int,
                                mut inputs: * const * const std::os::raw::c_double,
                                mut options: * const std::os::raw::c_double,
                                mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut output: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_cmo_start(options) { return 0 as std::os::raw::c_int }
    let mut up_sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut down_sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i <= period {
        up_sum +=
            if *input.offset(i as isize) >
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset(i as isize)) -
                    *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        down_sum +=
            if *input.offset(i as isize) <
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) -
                    *input.offset(i as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        i += 1
    }
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 =
        100 as std::os::raw::c_int as std::os::raw::c_double * (up_sum - down_sum) /
            (up_sum + down_sum);
    i = period + 1 as std::os::raw::c_int;
    while i < size {
        up_sum -=
            if *input.offset((i - period) as isize) >
                   *input.offset((i - period - 1 as std::os::raw::c_int) as isize) {
                (*input.offset((i - period) as isize)) -
                    *input.offset((i - period - 1 as std::os::raw::c_int) as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        down_sum -=
            if *input.offset((i - period) as isize) <
                   *input.offset((i - period - 1 as std::os::raw::c_int) as isize) {
                (*input.offset((i - period - 1 as std::os::raw::c_int) as isize)) -
                    *input.offset((i - period) as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        up_sum +=
            if *input.offset(i as isize) >
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset(i as isize)) -
                    *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        down_sum +=
            if *input.offset(i as isize) <
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) -
                    *input.offset(i as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 =
            100 as std::os::raw::c_int as std::os::raw::c_double * (up_sum - down_sum) /
                (up_sum + down_sum);
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_cmo_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 7], &'_ [i8; 7]>(b"ti_cmo\x00")).as_ptr(),
                     b"indicators/cmo.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 64 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_cmo_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;

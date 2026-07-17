#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use std::os::raw::{c_int, c_double};
use tulip_crown_e3::indicators_index::ti_indicators;
fuzz_target!(|data: &[u8]| {
    if data.len() < 16 { return; }
    let n: usize = 64;
    let mut series = vec![0f64; n];
    for k in 0..n { let b = data[(k*3)%data.len()] as f64; series[k] = 10.0 + (b % 100.0); }
    let mut opts = [0f64; 10];
    for k in 0..10 { opts[k] = 2.0 + (data[(k+1)%data.len()] % 20) as f64; }
    unsafe {
        for idx in 0..(ti_indicators.len()-1) {
            let info = &ti_indicators[idx];
            if info.name.is_null() { break; }
            let nout = (info.outputs.max(0) as usize).min(10);
            let inputs: [*const c_double; 10] = [series.as_ptr(); 10];
            let mut outbufs: Vec<Vec<f64>> = (0..nout).map(|_| vec![0f64; n]).collect();
            let mut outputs: [*mut c_double; 10] = [std::ptr::null_mut(); 10];
            for j in 0..nout { outputs[j] = outbufs[j].as_mut_ptr(); }
            if let Some(f) = info.indicator { let _ = f(n as c_int, inputs.as_ptr(), opts.as_ptr(), outputs.as_ptr()); }
        }
    }
});

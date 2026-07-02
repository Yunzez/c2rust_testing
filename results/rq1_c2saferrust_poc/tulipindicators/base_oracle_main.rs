// Faithful-c2rust tulipindicators oracle (base). Reads stdin, decodes the SAME byte protocol as
// the WIP fuzz harness, calls the base ti_indicators[] indicator, serializes ret + NaN-canonical
// output bits. Built with ASan: an OOB read/write in the FAITHFUL version aborts (nonzero exit =
// UB gate). Release/no-overflow-checks so arithmetic wraps like C (we gate memory UB, not wrap).
use std::io::{Read, Write};
use c2rust_out::example2::{ti_indicators, ti_indicator_info};

fn count() -> usize {
    let p = std::ptr::addr_of!(ti_indicators) as *const ti_indicator_info;
    let mut c = 0; unsafe { while !(*p.add(c)).name.is_null() { c += 1; } } c
}
fn entry(i: usize) -> ti_indicator_info {
    let p = std::ptr::addr_of!(ti_indicators) as *const ti_indicator_info;
    unsafe { *p.add(i) }
}

fn main() {
    let mut data = Vec::new();
    std::io::stdin().read_to_end(&mut data).ok();
    let mut p = 0usize;
    let mut byte = || { let b = *data.get(p).unwrap_or(&0); p += 1; b };
    let n = count();
    if n == 0 { println!(); return; }
    let idx = (byte() as usize) % n;
    let t = entry(idx);
    let size = (byte() as usize) % 60 + 1;
    let (ni, no, nout) = (t.inputs as usize, t.options as usize, t.outputs as usize);
    let inputs: Vec<Vec<f64>> = (0..ni).map(|_| (0..size).map(|_| {
        let lo = byte() as u32; let hi = byte() as u32; ((lo | (hi<<8)) as f64) / 16.0
    }).collect()).collect();
    let input_ptrs: Vec<*const f64> = inputs.iter().map(|a| a.as_ptr()).collect();
    let opts: Vec<f64> = (0..no).map(|_| ((byte() as u32 % 50) + 1) as f64).collect();
    let mut outputs: Vec<Vec<f64>> = (0..nout).map(|_| vec![0.0f64; size]).collect();
    let out_ptrs: Vec<*mut f64> = outputs.iter_mut().map(|a| a.as_mut_ptr()).collect();
    let f = match t.indicator { Some(f) => f, None => { println!(); return; } };
    let ret = unsafe { f(size as i32, input_ptrs.as_ptr(), opts.as_ptr(), out_ptrs.as_ptr()) };
    let mut s = format!("i:{} ret:{}", idx, ret);
    for (k, ob) in outputs.iter().enumerate() {
        s.push_str(&format!(" o{}", k));
        for &v in ob.iter() { if v.is_nan() { s.push_str(":nan"); } else { s.push_str(&format!(":{}", v.to_bits())); } }
    }
    let mut so = std::io::stdout(); let _ = so.write_all(s.as_bytes()); let _ = so.write_all(b"\n");
}

#[no_mangle]
pub extern "C" fn __assert_rtn(_f: *const i8, _fl: *const i8, _l: i32, _e: *const i8) -> ! { std::process::abort() }

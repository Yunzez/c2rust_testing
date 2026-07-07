#![no_main]
//! OOP differential harness: C2SaferRust-lifted tulipindicators vs original C, dispatched
//! generically through the shared ti_indicators[] table. Rust called natively (raw C-ABI fn ptr);
//! C is a subprocess oracle (UB gate). Rust runs with debug-assertions (overflow/OOB panic) so a
//! qsort-class index/arithmetic bug in the lift surfaces as a Rust panic on a UB-free-in-C input.
use libfuzzer_sys::fuzz_target;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use c2rust_out::example2::{ti_indicators, ti_indicator_info};

const ORACLE: &str = "/tmp/claude-1000/-home-yunzez-c2rust-testing/1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/ti_base_oracle/target/x86_64-unknown-linux-gnu/release/ti_base_oracle";
const ORACLE_TIMEOUT_MS: u64 = 2000;

struct Cur<'a> { d: &'a [u8], p: usize }
impl<'a> Cur<'a> {
    fn new(d: &'a [u8]) -> Self { Cur { d, p: 0 } }
    fn byte(&mut self) -> u8 { let b = *self.d.get(self.p).unwrap_or(&0); self.p += 1; b }
    fn rd(&mut self, w: usize) -> u32 { let mut v = 0u32; for i in 0..w { v |= (self.byte() as u32) << (8*i); } v }
}

fn table_count() -> usize {
    let p = std::ptr::addr_of!(ti_indicators) as *const ti_indicator_info;
    let mut c = 0usize;
    unsafe { while !(*p.add(c)).name.is_null() { c += 1; } }
    c
}
fn table_entry(idx: usize) -> ti_indicator_info {
    let p = std::ptr::addr_of!(ti_indicators) as *const ti_indicator_info;
    unsafe { *p.add(idx) }
}

fn rust_side(data: &[u8]) -> Option<String> {
    let count = table_count();
    if count == 0 { return None; }
    let mut cur = Cur::new(data);
    let idx = (cur.byte() as usize) % count;
    let t = table_entry(idx);
    let size = (cur.byte() as usize) % 60 + 1;
    let (ni, no, nout) = (t.inputs as usize, t.options as usize, t.outputs as usize);
    // inputs: ni arrays of `size` doubles (same decode as the C oracle)
    let inputs: Vec<Vec<f64>> = (0..ni).map(|_|
        (0..size).map(|_| (cur.rd(2) as f64) / 16.0).collect()).collect();
    let input_ptrs: Vec<*const f64> = inputs.iter().map(|a| a.as_ptr()).collect();
    let opts: Vec<f64> = (0..no).map(|_| ((cur.byte() as u32 % 50) + 1) as f64).collect();
    let mut outputs: Vec<Vec<f64>> = (0..nout).map(|_| vec![0.0f64; size]).collect();
    let out_ptrs: Vec<*mut f64> = outputs.iter_mut().map(|a| a.as_mut_ptr()).collect();
    let f = t.indicator?;
    let ret = unsafe { f(size as i32, input_ptrs.as_ptr(), opts.as_ptr(), out_ptrs.as_ptr()) };
    let mut s = format!("i:{} ret:{}", idx, ret);
    for (k, ob) in outputs.iter().enumerate() {
        s.push_str(&format!(" o{}", k));
        for &v in ob.iter() {
            if v.is_nan() { s.push_str(":nan"); } else { s.push_str(&format!(":{}", v.to_bits())); }
        }
    }
    Some(s)
}

fn run_oracle(data: &[u8]) -> Option<String> {
    let mut ch = Command::new(ORACLE)
        .env("ASAN_OPTIONS", "symbolize=0:detect_leaks=0:abort_on_error=0:exitcode=1")
        .env("UBSAN_OPTIONS", "symbolize=0:abort_on_error=0")
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    ch.stdin.take()?.write_all(data).ok();
    let mut so = ch.stdout.take()?;
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || { let mut s = String::new(); let _ = so.read_to_string(&mut s); let _ = tx.send(s); });
    let deadline = Instant::now() + Duration::from_millis(ORACLE_TIMEOUT_MS);
    let status = loop {
        match ch.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => { if Instant::now() >= deadline { let _ = ch.kill(); let _ = ch.wait(); return None; }
                          std::thread::sleep(Duration::from_millis(1)); }
            Err(_) => return None,
        }
    };
    if !status.success() { return None; }   // C UB -> gate
    let s = rx.recv_timeout(Duration::from_millis(500)).ok()?;
    Some(s.trim().to_string())
}

fuzz_target!(|data: &[u8]| {
    let c_out = match run_oracle(data) { Some(s) => s, None => return };
    // Rust may PANIC here (index OOB / overflow) on a UB-free-in-C input -> that IS the finding.
    let r_out = match rust_side(data) { Some(s) => s, None => return };
    if c_out != r_out {
        match run_oracle(data) {
            Some(c2) if c2 == c_out => panic!("divergence: C={:?} Rust={:?}", c_out, r_out),
            _ => return,
        }
    }
});

// macOS assert symbol referenced by some indicators' assert(). Match C's assert->SIGABRT: abort.
// C gates precondition-violating inputs first (its glibc assert aborts -> nonzero -> gated), so
// this fires only if Rust asserts where C did NOT -> a genuine divergence.
#[no_mangle]
pub extern "C" fn __assert_rtn(_f: *const i8, _fl: *const i8, _l: i32, _e: *const i8) -> ! {
    std::process::abort()
}

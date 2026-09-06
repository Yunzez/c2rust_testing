#![no_main]
use libfuzzer_sys::fuzz_target;
struct Cur<'a> { d: &'a [u8], p: usize }
impl<'a> Cur<'a> {
    fn new(d: &'a [u8]) -> Self { Cur { d, p: 0 } }
    fn byte(&mut self) -> u8 { let b = if self.p < self.d.len() { self.d[self.p] } else { 0 }; self.p += 1; b }
    fn take_u8(&mut self) -> u8 { let mut v = [0u8; 1]; for i in 0..1 { v[i] = self.byte(); } u8::from_le_bytes(v) }
    fn take_i8(&mut self) -> i8 { let mut v = [0u8; 1]; for i in 0..1 { v[i] = self.byte(); } i8::from_le_bytes(v) }
    fn take_u16(&mut self) -> u16 { let mut v = [0u8; 2]; for i in 0..2 { v[i] = self.byte(); } u16::from_le_bytes(v) }
    fn take_i16(&mut self) -> i16 { let mut v = [0u8; 2]; for i in 0..2 { v[i] = self.byte(); } i16::from_le_bytes(v) }
    fn take_u32(&mut self) -> u32 { let mut v = [0u8; 4]; for i in 0..4 { v[i] = self.byte(); } u32::from_le_bytes(v) }
    fn take_i32(&mut self) -> i32 { let mut v = [0u8; 4]; for i in 0..4 { v[i] = self.byte(); } i32::from_le_bytes(v) }
    fn take_u64(&mut self) -> u64 { let mut v = [0u8; 8]; for i in 0..8 { v[i] = self.byte(); } u64::from_le_bytes(v) }
    fn take_i64(&mut self) -> i64 { let mut v = [0u8; 8]; for i in 0..8 { v[i] = self.byte(); } i64::from_le_bytes(v) }
    fn take_usize(&mut self) -> usize { let mut v = [0u8; 8]; for i in 0..8 { v[i] = self.byte(); } usize::from_le_bytes(v) }
    fn take_f32(&mut self) -> f32 { let mut v = [0u8; 4]; for i in 0..4 { v[i] = self.byte(); } f32::from_le_bytes(v) }
    fn take_f64(&mut self) -> f64 { let mut v = [0u8; 8]; for i in 0..8 { v[i] = self.byte(); } f64::from_le_bytes(v) }
    fn take_vec_u8(&mut self) -> Vec<u8> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u8()).collect() }
    fn take_vec_i8(&mut self) -> Vec<i8> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i8()).collect() }
    fn take_vec_u16(&mut self) -> Vec<u16> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u16()).collect() }
    fn take_vec_i16(&mut self) -> Vec<i16> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i16()).collect() }
    fn take_vec_u32(&mut self) -> Vec<u32> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u32()).collect() }
    fn take_vec_i32(&mut self) -> Vec<i32> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i32()).collect() }
    fn take_vec_u64(&mut self) -> Vec<u64> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u64()).collect() }
    fn take_vec_i64(&mut self) -> Vec<i64> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i64()).collect() }
    fn take_vec_f32(&mut self) -> Vec<f32> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_f32()).collect() }
    fn take_vec_f64(&mut self) -> Vec<f64> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_f64()).collect() }
    fn take_rest_u8(&mut self, max: usize) -> Vec<u8> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u8()); } v }
    fn take_rest_i8(&mut self, max: usize) -> Vec<i8> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i8()); } v }
    fn take_rest_u16(&mut self, max: usize) -> Vec<u16> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u16()); } v }
    fn take_rest_i16(&mut self, max: usize) -> Vec<i16> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i16()); } v }
    fn take_rest_u32(&mut self, max: usize) -> Vec<u32> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u32()); } v }
    fn take_rest_i32(&mut self, max: usize) -> Vec<i32> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i32()); } v }
    fn take_rest_u64(&mut self, max: usize) -> Vec<u64> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u64()); } v }
    fn take_rest_i64(&mut self, max: usize) -> Vec<i64> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i64()); } v }
    fn take_rest_f32(&mut self, max: usize) -> Vec<f32> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_f32()); } v }
    fn take_rest_f64(&mut self, max: usize) -> Vec<f64> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_f64()); } v }
}

// RQ4: C reference and UB gate are selected at RUN TIME so every mode shares one binary,
// one coverage map and one set of identities. C2R_MODE=gated|nogate|rust-only.
const C2R_GATED: u8 = 0; const C2R_NOGATE: u8 = 1; const C2R_RUST_ONLY: u8 = 2;
const C2R_COVERAGE: u8 = 3; const C2R_C_ONLY: u8 = 4;
fn c2r_mode() -> u8 {
    static M: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *M.get_or_init(|| match std::env::var("C2R_MODE").as_deref() {
        Ok("nogate") => C2R_NOGATE,
        Ok("rust-only") => C2R_RUST_ONLY,
        Ok("coverage") => C2R_COVERAGE,
        // Confirmation phase A: run ONLY C, so a sanitizer report is unambiguously C's.
        // Sanitizing both sides at once makes the report unattributable.
        Ok("c-only") => C2R_C_ONLY,
        // `combined` is the both-sides replay and is the default. `gated` stays as an
        // alias: the in-loop UB gate used to be the main line and no longer is.
        Ok("combined") | Ok("gated") => C2R_GATED,
        _ => C2R_GATED,
    })
}

// ---- termination rung (docs/harness_oracle_plan.md, rung 1) -------------------------
// The outcome vocabulary is FIXED and is the contract with the confirmation driver:
//     normal | divergence | panic | signal | nonzero-exit | timeout
// The last three are observed by the driver from the process result; the first three are
// reported from here. `phase` says how far the execution got, which is what turns an
// abort into an attribution: phase>=2 means C already returned normally, so a panic or a
// crash at phase 3 is the translation diverging on TERMINATION, not a C-side failure.
const C2R_PH_DECODE: u8 = 0; const C2R_PH_C: u8 = 1; const C2R_PH_C_DONE: u8 = 2;
const C2R_PH_RUST: u8 = 3; const C2R_PH_RUST_DONE: u8 = 4; const C2R_PH_COMPARED: u8 = 5;
static C2R_PHASE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
fn c2r_phase(p: u8) { C2R_PHASE.store(p, std::sync::atomic::Ordering::Relaxed); }
fn c2r_outcome_file() -> Option<&'static str> {
    static F: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    F.get_or_init(|| std::env::var("C2R_OUTCOME_FILE").ok()).as_deref()
}
fn c2r_outcome(kind: &str, detail: &str) {
    let ph = C2R_PHASE.load(std::sync::atomic::Ordering::Relaxed);
    let d: String = detail.chars().map(|c| if c == '\n' { ' ' } else { c }).take(200).collect();
    let line = format!("C2R_OUTCOME kind={kind} phase={ph} detail={d}\n");
    // `normal` is never printed: in discovery that would be one write per execution.
    if kind != "normal" { eprint!("{line}"); }
    if let Some(f) = c2r_outcome_file() {
        use std::io::Write;
        if let Ok(mut h) = std::fs::OpenOptions::new().create(true).append(true).open(f) {
            let _ = h.write_all(line.as_bytes());
        }
    }
}
// libfuzzer-sys installs a panic hook that ABORTS before unwinding, so catch_unwind cannot
// see a panic. Chaining a hook in front of it records the outcome and then lets libFuzzer
// do exactly what it did before.
fn c2r_install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            c2r_outcome("panic", &format!("{info}"));
            prev(info);
        }));
    });
}
// A divergence is the ORACLE, so by default it panics and libFuzzer records the finding.
// In `coverage` mode the same comparison runs but a mismatch is only counted: a coverage
// replay must not abort, or an artifact whose defect lies on the main path yields NO
// coverage at all (observed on bzip2 x Laertes and x CROWN, whose compress corpora are
// almost entirely divergence-triggering inputs).
fn c2r_div(what: &str) {
    if c2r_mode() == C2R_COVERAGE {
        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) == 0 {
            eprintln!("c2r: divergence recorded, not panicking (coverage mode): {what}");
        }
        return;
    }
    // Report and abort DETERMINISTICALLY rather than unwinding. A divergence is a terminal
    // observation, so there is nothing to unwind to, and unwinding out of this helper was
    // observed to hang the single-input replay path (libFuzzer's fork mode was unaffected,
    // which is why the campaigns still recorded every event). SIGABRT is what libFuzzer's
    // deadly-signal handler expects.
    c2r_outcome("divergence", what);
    std::process::abort();
}

fn cd() -> i8 { 0 }  // silence unused on some shapes

use lil_c2saferrust as translated;
extern "C" {
    fn c_lil_alloc_integer(num: i64) -> *const core::ffi::c_void;
    fn c2r_ub_reset();
    fn c2r_ub_get() -> i32;
}

fuzz_target!(|data: &[u8]| {
    let _ = cd();
    c2r_install_panic_hook();
    c2r_phase(C2R_PH_DECODE);
    let mut cur = Cur::new(data);
    let num = cur.take_i64();
    let _c2r_m = c2r_mode();
    unsafe {
        if _c2r_m == C2R_C_ONLY {
            // confirmation phase A: C alone, so any sanitizer report is C's
            c2r_phase(C2R_PH_C);
            c2r_ub_reset();
            let c_ret = c_lil_alloc_integer(num);
            c2r_phase(C2R_PH_C_DONE);
        } else if _c2r_m == C2R_RUST_ONLY {
            // no C reference, so nothing can be compared; throughput bound only
            c2r_phase(C2R_PH_RUST);
            let r_ret = translated::lil_alloc_integer(num);
            c2r_phase(C2R_PH_RUST_DONE);
        } else {
            c2r_phase(C2R_PH_C);
            c2r_ub_reset();
            let c_ret = c_lil_alloc_integer(num);
            if _c2r_m == C2R_GATED && c2r_ub_get() != 0 { c2r_outcome("ub-gated", ""); return; }  // C hit UB -> reject
            c2r_phase(C2R_PH_C_DONE);
            c2r_phase(C2R_PH_RUST);
            let r_ret = translated::lil_alloc_integer(num);
            c2r_phase(C2R_PH_RUST_DONE);
            let c_null = (c_ret as *const core::ffi::c_void).is_null();
            let r_null = (r_ret as *const core::ffi::c_void).is_null();
            if c_null != r_null { c2r_div("returned pointer nullness"); }
        }
    }
    c2r_phase(C2R_PH_COMPARED);
    c2r_outcome("normal", "");
});

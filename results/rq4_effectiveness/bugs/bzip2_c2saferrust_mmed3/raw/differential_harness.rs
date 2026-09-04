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
    fn take_vec_u8(&mut self) -> Vec<u8> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u8()).collect() }
    fn take_vec_i8(&mut self) -> Vec<i8> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i8()).collect() }
    fn take_vec_u16(&mut self) -> Vec<u16> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u16()).collect() }
    fn take_vec_i16(&mut self) -> Vec<i16> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i16()).collect() }
    fn take_vec_u32(&mut self) -> Vec<u32> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u32()).collect() }
    fn take_vec_i32(&mut self) -> Vec<i32> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i32()).collect() }
    fn take_vec_u64(&mut self) -> Vec<u64> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_u64()).collect() }
    fn take_vec_i64(&mut self) -> Vec<i64> { let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_i64()).collect() }
    fn take_rest_u8(&mut self, max: usize) -> Vec<u8> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u8()); } v }
    fn take_rest_i8(&mut self, max: usize) -> Vec<i8> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i8()); } v }
    fn take_rest_u16(&mut self, max: usize) -> Vec<u16> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u16()); } v }
    fn take_rest_i16(&mut self, max: usize) -> Vec<i16> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i16()); } v }
    fn take_rest_u32(&mut self, max: usize) -> Vec<u32> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u32()); } v }
    fn take_rest_i32(&mut self, max: usize) -> Vec<i32> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i32()); } v }
    fn take_rest_u64(&mut self, max: usize) -> Vec<u64> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_u64()); } v }
    fn take_rest_i64(&mut self, max: usize) -> Vec<i64> { let mut v = Vec::new(); while self.p < self.d.len() && v.len() < max { v.push(self.take_i64()); } v }
}

// RQ4: C reference and UB gate are selected at RUN TIME so every mode shares one binary,
// one coverage map and one set of identities. C2R_MODE=gated|nogate|rust-only.
const C2R_GATED: u8 = 0; const C2R_NOGATE: u8 = 1; const C2R_RUST_ONLY: u8 = 2;
fn c2r_mode() -> u8 {
    static M: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *M.get_or_init(|| match std::env::var("C2R_MODE").as_deref() {
        Ok("nogate") => C2R_NOGATE,
        Ok("rust-only") => C2R_RUST_ONLY,
        _ => C2R_GATED,
    })
}

fn cd() -> i8 { 0 }  // silence unused on some shapes

use bzip2_c2saferrust as translated;
extern "C" {
    fn c_mmed3(a: u8, b: u8, c: u8) -> u8;
    fn c2r_ub_reset();
    fn c2r_ub_get() -> i32;
}

fuzz_target!(|data: &[u8]| {
    let _ = cd();
    let mut cur = Cur::new(data);
    let a = cur.take_u8();
    let b = cur.take_u8();
    let c = cur.take_u8();
    let mode = c2r_mode();
    unsafe {
        if mode == C2R_RUST_ONLY {
            // no C reference, so nothing can be compared; throughput bound only
            let r_ret = translated::mmed3(a, b, c);
        } else {
            c2r_ub_reset();
            let c_ret = c_mmed3(a, b, c);
            if mode == C2R_GATED && c2r_ub_get() != 0 { return; }  // C hit UB -> reject
            let r_ret = translated::mmed3(a, b, c);
            if c_ret != r_ret { panic!("divergence: return value"); }
        }
    }
});

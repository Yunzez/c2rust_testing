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
}

fn cd() -> i8 { 0 }  // silence unused on some shapes

use mergesort_search as translated;
extern "C" {
    fn c_sort_and_find(a: *mut i32, n: usize, key: i32) -> i32;
}

fuzz_target!(|data: &[u8]| {
    let _ = cd();
    let mut cur = Cur::new(data);
    let mut a_c: Vec<i32> = cur.take_vec_i32();
    let n = a_c.len();
    let mut a_r = a_c.clone();
    let key = cur.take_i32();
    unsafe {
        let c_ret = c_sort_and_find(a_c.as_mut_ptr(), n, key);
        let r_ret = translated::sort_and_find(a_r.as_mut_ptr(), n, key);
        if c_ret != r_ret { panic!("divergence: return value"); }
    if a_c != a_r { panic!("divergence: buffer a"); }
    }
});

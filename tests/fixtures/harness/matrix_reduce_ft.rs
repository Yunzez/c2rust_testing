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

use matrix_reduce as translated;
extern "C" {
    fn c_matrix_transpose_and_sum(mat: *mut *mut i32, rows: usize, cols: usize) -> i64;
}

fuzz_target!(|data: &[u8]| {
    let _ = cd();
    let mut cur = Cur::new(data);
    let rows = (cur.byte() as usize) % (16 + 1);
    let cols = (cur.byte() as usize) % (16 + 1);
    let mat_data: Vec<Vec<i32>> = (0..rows).map(|_| (0..cols).map(|_| cur.take_i32()).collect()).collect();
    let mut mat_back_c = mat_data.clone();
    let mut mat_tab_c: Vec<*mut i32> = mat_back_c.iter_mut().map(|row| row.as_mut_ptr()).collect();
    let mut mat_back_r = mat_data.clone();
    let mut mat_tab_r: Vec<*mut i32> = mat_back_r.iter_mut().map(|row| row.as_mut_ptr()).collect();
    unsafe {
        let c_ret = c_matrix_transpose_and_sum(mat_tab_c.as_mut_ptr(), rows, cols);
        let r_ret = translated::matrix_transpose_and_sum(mat_tab_r.as_mut_ptr(), rows, cols);
        if c_ret != r_ret { panic!("divergence: return value"); }
    if mat_back_c != mat_back_r { panic!("divergence: table mat"); }
    }
});

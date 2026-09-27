// SACTOR unidiomatic translation of `addColorBits` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:50:50; attempt 1). Verification verdict: Rust code failed to compile
unsafe fn addColorBits(out: *mut u8, index: libc::size_t, bits: u32, mut input: u32) {
    fn size_t_to_u32(x: libc::size_t) -> u32 {
        x as u32
    }
    fn compute_m(bits: u32) -> u32 {
        if bits == 1 { 7 } else if bits == 2 { 3 } else { 1 }
    }
    let m = compute_m(bits);
    let p: u32 = size_t_to_u32(index) & m;
    input &= (1u32 << bits) - 1u32;
    input <<= bits * (m - p);
    let byte_index: usize = ((index as u64 * bits as u64) / 8u64) as usize;
    let byte_ptr = out.add(byte_index);
    if p == 0 {
        *byte_ptr = input as u8;
    } else {
        *byte_ptr |= input as u8;
    }
}

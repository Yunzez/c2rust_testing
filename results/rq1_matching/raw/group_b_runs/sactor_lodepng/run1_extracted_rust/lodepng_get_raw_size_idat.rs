// SACTOR unidiomatic translation of `lodepng_get_raw_size_idat` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:49:35; attempt 1). Verification verdict: Rust code failed to compile
fn lodepng_get_raw_size_idat(w: u32, h: u32, bpp: u32) -> usize {
    let line: usize = (w / 8u32) as usize * bpp as usize + 1usize
        + (((w & 7u32) * bpp + 7u32) / 8u32) as usize;
    h as usize * line
}

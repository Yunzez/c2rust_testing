// SACTOR unidiomatic translation of `lodepng_read32bitInt` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:35:24; attempt 1). Verification verdict: rust compiled
fn lodepng_read32bitInt(buffer: *const u8) -> u32 {
    unsafe {
        ((*buffer.offset(0) as u32) << 24) | ((*buffer.offset(1) as u32) << 16)
            | ((*buffer.offset(2) as u32) << 8) | (*buffer.offset(3) as u32)
    }
}

// SACTOR unidiomatic translation of `peekBits` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:39:06; attempt 1). Verification verdict: rust compiled
#[inline]
pub unsafe fn peekBits(
    reader: *mut LodePNGBitReader,
    nbits: libc::size_t,
) -> libc::c_uint {
    (*reader).buffer & ((1u32 << nbits) - 1u32)
}

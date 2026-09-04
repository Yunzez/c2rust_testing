// SACTOR unidiomatic translation of `advanceBits` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:39:25; attempt 1). Verification verdict: rust compiled
#[inline]
pub unsafe fn advanceBits(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    (*reader).buffer >>= nbits;
    (*reader).bp = (*reader).bp.wrapping_add(nbits);
}

// SACTOR unidiomatic translation of `lodepng_chunk_ancillary` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:44:57; attempt 1). Verification verdict: rust compiled
pub unsafe fn lodepng_chunk_ancillary(chunk: *const u8) -> u8 {
    if *chunk.add(4) & 32 != 0 { 1 } else { 0 }
}

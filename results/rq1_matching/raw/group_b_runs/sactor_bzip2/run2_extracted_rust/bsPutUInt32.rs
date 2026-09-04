// SACTOR unidiomatic translation of `bsPutUInt32` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:43:52; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_bzip2/compress.c: Error: Failed to link project-level harness for
pub unsafe fn bsPutUInt32(s: *mut EState, u: u32) {
    bsW(s, 8, (u >> 24) & 0xff);
    bsW(s, 8, (u >> 16) & 0xff);
    bsW(s, 8, (u >> 8) & 0xff);
    bsW(s, 8, u & 0xff);
}

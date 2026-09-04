// SACTOR unidiomatic translation of `isRGBICCProfile` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:10:27; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_lodepng/lodepng.c: Error: Failed to link project-level harness fo
fn isRGBICCProfile(profile: *const u8, size: u32) -> u32 {
    if size < 20 {
        return 0;
    }
    unsafe {
        if *profile.add(16) == b'R' && *profile.add(17) == b'G'
            && *profile.add(18) == b'B' && *profile.add(19) == b' '
        {
            1
        } else {
            0
        }
    }
}

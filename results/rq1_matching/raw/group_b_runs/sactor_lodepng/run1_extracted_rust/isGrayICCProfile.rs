// SACTOR unidiomatic translation of `isGrayICCProfile` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:10:06; attempt 1). Verification verdict: Error: Function signature not found in the translated code for function `isRGBICCProfile`. Got functions: ['is_rgb_icc_profile'], check if you have the correct function name., you should **NOT** chang
fn isGrayICCProfile(profile: *const u8, size: u32) -> u32 {
    unsafe {
        if size < 20 {
            return 0;
        }
        if *profile.add(16) == b'G' && *profile.add(17) == b'R'
            && *profile.add(18) == b'A' && *profile.add(19) == b'Y'
        {
            1
        } else {
            0
        }
    }
}

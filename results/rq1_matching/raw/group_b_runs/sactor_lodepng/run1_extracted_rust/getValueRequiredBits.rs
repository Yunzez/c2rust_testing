// SACTOR unidiomatic translation of `getValueRequiredBits` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:54:10; attempt 1). Verification verdict: rust compiled
fn getValueRequiredBits(value: u8) -> u32 {
    if value == 0 || value == 255 {
        return 1;
    }
    if value % 17 == 0 {
        return if value % 85 == 0 { 2 } else { 4 };
    }
    8
}

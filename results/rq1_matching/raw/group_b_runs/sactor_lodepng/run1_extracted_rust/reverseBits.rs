// SACTOR unidiomatic translation of `reverseBits` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:39:44; attempt 1). Verification verdict: rust compiled
fn reverseBits(bits: u32, num: u32) -> u32 {
    fn inner(bits: u32, num: u32) -> u32 {
        let mut i: u32 = 0;
        let mut result: u32 = 0;
        while i < num {
            result |= ((bits >> (num - i - 1)) & 1) << i;
            i += 1;
        }
        result
    }
    inner(bits, num)
}

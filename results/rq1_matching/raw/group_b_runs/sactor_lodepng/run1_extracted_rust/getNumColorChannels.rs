// SACTOR unidiomatic translation of `getNumColorChannels` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:47:16; attempt 1). Verification verdict: Rust code failed to compile
fn getNumColorChannels(colortype: LodePNGColorType) -> u32 {
    match colortype {
        LodePNGColorType::LCT_GREY => 1,
        LodePNGColorType::LCT_RGB => 3,
        LodePNGColorType::LCT_PALETTE => 1,
        LodePNGColorType::LCT_GREY_ALPHA => 2,
        LodePNGColorType::LCT_RGBA => 4,
        LodePNGColorType::LCT_MAX_OCTET_VALUE => 0,
        _ => 0,
    }
}

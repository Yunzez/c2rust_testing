// SACTOR unidiomatic translation of `checkColorValidity` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:46:54; attempt 1). Verification verdict: Error: Function signature not found in the translated code for function `getNumColorChannels`. Got functions: ['get_num_color_channels'], check if you have the correct function name., you should **NOT
pub fn checkColorValidity(colortype: LodePNGColorType, bd: u32) -> u32 {
    match colortype {
        LodePNGColorType::LCT_GREY => {
            if !(bd == 1 || bd == 2 || bd == 4 || bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_RGB => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_PALETTE => {
            if !(bd == 1 || bd == 2 || bd == 4 || bd == 8) {
                return 37;
            }
        }
        LodePNGColorType::LCT_GREY_ALPHA => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_RGBA => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_MAX_OCTET_VALUE => {
            return 31;
        }
        _ => {
            return 31;
        }
    }
    0
}

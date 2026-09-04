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

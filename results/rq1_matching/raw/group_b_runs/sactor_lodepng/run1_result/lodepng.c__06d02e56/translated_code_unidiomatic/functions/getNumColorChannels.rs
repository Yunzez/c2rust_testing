pub fn getNumColorChannels(colortype: LodePNGColorType) -> u32 {
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

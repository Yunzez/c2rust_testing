#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum LodePNGColorType {
    /// grayscale: 1,2,4,8,16 bit
    LCT_GREY = 0,
    /// RGB: 8,16 bit
    LCT_RGB = 2,
    /// palette: 1,2,4,8 bit
    LCT_PALETTE = 3,
    /// grayscale with alpha: 8,16 bit
    LCT_GREY_ALPHA = 4,
    /// RGB with alpha: 8,16 bit
    LCT_RGBA = 6,
    /// See C docs: may represent any invalid byte value 0–255; don't use directly.
    LCT_MAX_OCTET_VALUE = 255,
}

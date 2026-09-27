pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGColorMode {
    pub colortype: LodePNGColorType,
    pub bitdepth: ::core::ffi::c_uint,
    pub palette: *mut ::core::ffi::c_uchar,
    pub palettesize: size_t,
    pub key_defined: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_uint,
    pub key_g: ::core::ffi::c_uint,
    pub key_b: ::core::ffi::c_uint,
}

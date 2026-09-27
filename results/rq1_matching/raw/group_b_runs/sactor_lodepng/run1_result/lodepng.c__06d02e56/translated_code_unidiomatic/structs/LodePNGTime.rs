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
pub struct LodePNGTime {
    pub year: ::core::ffi::c_uint,
    pub month: ::core::ffi::c_uint,
    pub day: ::core::ffi::c_uint,
    pub hour: ::core::ffi::c_uint,
    pub minute: ::core::ffi::c_uint,
    pub second: ::core::ffi::c_uint,
}

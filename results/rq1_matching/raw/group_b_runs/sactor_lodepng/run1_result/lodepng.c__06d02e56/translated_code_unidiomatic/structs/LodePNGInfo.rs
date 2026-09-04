#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGTime {
    pub year: ::core::ffi::c_uint,
    pub month: ::core::ffi::c_uint,
    pub day: ::core::ffi::c_uint,
    pub hour: ::core::ffi::c_uint,
    pub minute: ::core::ffi::c_uint,
    pub second: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
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
pub struct LodePNGInfo {
    pub compression_method: ::core::ffi::c_uint,
    pub filter_method: ::core::ffi::c_uint,
    pub interlace_method: ::core::ffi::c_uint,
    pub color: LodePNGColorMode,
    pub background_defined: ::core::ffi::c_uint,
    pub background_r: ::core::ffi::c_uint,
    pub background_g: ::core::ffi::c_uint,
    pub background_b: ::core::ffi::c_uint,
    pub text_num: size_t,
    pub text_keys: *mut *mut ::core::ffi::c_char,
    pub text_strings: *mut *mut ::core::ffi::c_char,
    pub itext_num: size_t,
    pub itext_keys: *mut *mut ::core::ffi::c_char,
    pub itext_langtags: *mut *mut ::core::ffi::c_char,
    pub itext_transkeys: *mut *mut ::core::ffi::c_char,
    pub itext_strings: *mut *mut ::core::ffi::c_char,
    pub time_defined: ::core::ffi::c_uint,
    pub time: LodePNGTime,
    pub phys_defined: ::core::ffi::c_uint,
    pub phys_x: ::core::ffi::c_uint,
    pub phys_y: ::core::ffi::c_uint,
    pub phys_unit: ::core::ffi::c_uint,
    pub gama_defined: ::core::ffi::c_uint,
    pub gama_gamma: ::core::ffi::c_uint,
    pub chrm_defined: ::core::ffi::c_uint,
    pub chrm_white_x: ::core::ffi::c_uint,
    pub chrm_white_y: ::core::ffi::c_uint,
    pub chrm_red_x: ::core::ffi::c_uint,
    pub chrm_red_y: ::core::ffi::c_uint,
    pub chrm_green_x: ::core::ffi::c_uint,
    pub chrm_green_y: ::core::ffi::c_uint,
    pub chrm_blue_x: ::core::ffi::c_uint,
    pub chrm_blue_y: ::core::ffi::c_uint,
    pub srgb_defined: ::core::ffi::c_uint,
    pub srgb_intent: ::core::ffi::c_uint,
    pub iccp_defined: ::core::ffi::c_uint,
    pub iccp_name: *mut ::core::ffi::c_char,
    pub iccp_profile: *mut ::core::ffi::c_uchar,
    pub iccp_profile_size: ::core::ffi::c_uint,
    pub sbit_defined: ::core::ffi::c_uint,
    pub sbit_r: ::core::ffi::c_uint,
    pub sbit_g: ::core::ffi::c_uint,
    pub sbit_b: ::core::ffi::c_uint,
    pub sbit_a: ::core::ffi::c_uint,
    pub unknown_chunks_data: [*mut ::core::ffi::c_uchar; 3],
    pub unknown_chunks_size: [size_t; 3],
}

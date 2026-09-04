pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGCompressSettings {
    pub btype: ::core::ffi::c_uint,
    pub use_lz77: ::core::ffi::c_uint,
    pub windowsize: ::core::ffi::c_uint,
    pub minmatch: ::core::ffi::c_uint,
    pub nicematch: ::core::ffi::c_uint,
    pub lazymatching: ::core::ffi::c_uint,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_deflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
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
pub struct LodePNGEncoderSettings {
    pub zlibsettings: LodePNGCompressSettings,
    pub auto_convert: ::core::ffi::c_uint,
    pub filter_palette_zero: ::core::ffi::c_uint,
    pub filter_strategy: LodePNGFilterStrategy,
    pub predefined_filters: *const ::core::ffi::c_uchar,
    pub force_palette: ::core::ffi::c_uint,
    pub add_id: ::core::ffi::c_uint,
    pub text_compression: ::core::ffi::c_uint,
}

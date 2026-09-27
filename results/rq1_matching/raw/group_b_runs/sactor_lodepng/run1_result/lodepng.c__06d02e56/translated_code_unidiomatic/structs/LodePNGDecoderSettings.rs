#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecompressSettings {
    pub ignore_adler32: ::core::ffi::c_uint,
    pub ignore_nlen: ::core::ffi::c_uint,
    pub max_output_size: size_t,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_inflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
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
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGDecoderSettings {
    pub zlibsettings: LodePNGDecompressSettings,
    pub ignore_crc: ::core::ffi::c_uint,
    pub ignore_critical: ::core::ffi::c_uint,
    pub ignore_end: ::core::ffi::c_uint,
    pub color_convert: ::core::ffi::c_uint,
    pub read_text_chunks: ::core::ffi::c_uint,
    pub remember_unknown_chunks: ::core::ffi::c_uint,
    pub max_text_size: size_t,
    pub max_icc_size: size_t,
}

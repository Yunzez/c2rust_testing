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
pub type FILE = libc::FILE;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGColorStats {
    pub colored: ::core::ffi::c_uint,
    pub key: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_ushort,
    pub key_g: ::core::ffi::c_ushort,
    pub key_b: ::core::ffi::c_ushort,
    pub alpha: ::core::ffi::c_uint,
    pub numcolors: ::core::ffi::c_uint,
    pub palette: [::core::ffi::c_uchar; 1024],
    pub bits: ::core::ffi::c_uint,
    pub numpixels: size_t,
    pub allow_palette: ::core::ffi::c_uint,
    pub allow_greyscale: ::core::ffi::c_uint,
}





extern "C" {
    pub type _IO_wide_data;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn getc(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn ungetc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
}
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::pngset::png_set_sBIT;
pub use crate::src::pngxtern::pngxmem::pngx_malloc_rows;
pub use crate::src::pnmio::pnmin::pnm_fget_bytes;
pub use crate::src::pnmio::pnmin::pnm_fget_header;
pub use crate::src::pnmio::pnmin::pnm_fget_values;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::libpng::png::_IO_FILE;
pub type _IO_lock_t = crate::src::gifread::gifread::_IO_lock_t;
pub type FILE = crate::src::gifread::gifread::FILE;
pub type png_byte = crate::src::libpng::png::png_byte;
pub type png_uint_32 = crate::src::libpng::png::png_uint_32;
pub type png_size_t = crate::src::libpng::png::png_size_t;
pub type png_alloc_size_t = crate::src::libpng::png::png_alloc_size_t;
pub type png_voidp = crate::src::libpng::png::png_voidp;
pub type png_bytep = crate::src::libpng::png::png_bytep;
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
pub type png_bytepp = crate::src::libpng::png::png_bytepp;
pub type png_const_charpp = crate::src::optipng::optim::png_const_charpp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_infop = crate::src::libpng::png::png_infop;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::pngerror::png_color_8_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8;
pub type png_const_color_8p = crate::src::libpng::pngrutil::png_const_color_8p;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm_struct {
    pub format: std::os::raw::c_uint,
    pub depth: std::os::raw::c_uint,
    pub width: std::os::raw::c_uint,
    pub height: std::os::raw::c_uint,
    pub maxval: std::os::raw::c_uint,
}
static mut pbm_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PBM\x00")
    };
static mut pgm_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PGM\x00")
    };
static mut ppm_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PPM\x00")
    };
static mut pam_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PAM\x00")
    };
static mut pbm_fmt_long_name: [std::os::raw::c_char; 16] =
    unsafe {
        *::std::mem::transmute::<&[u8; 16],
                                 &[std::os::raw::c_char; 16]>(b"Portable Bitmap\x00")
    };
static mut pgm_fmt_long_name: [std::os::raw::c_char; 17] =
    unsafe {
        *::std::mem::transmute::<&[u8; 17],
                                 &[std::os::raw::c_char; 17]>(b"Portable Graymap\x00")
    };
static mut ppm_fmt_long_name: [std::os::raw::c_char; 16] =
    unsafe {
        *::std::mem::transmute::<&[u8; 16],
                                 &[std::os::raw::c_char; 16]>(b"Portable Pixmap\x00")
    };
static mut pam_fmt_long_name: [std::os::raw::c_char; 16] =
    unsafe {
        *::std::mem::transmute::<&[u8; 16],
                                 &[std::os::raw::c_char; 16]>(b"Portable Anymap\x00")
    };

pub unsafe extern "C" fn pngx_sig_is_pnm(mut sig: png_bytep,
                                         mut sig_size: size_t,
                                         mut fmt_name_ptr: png_const_charpp,
                                         mut fmt_long_name_ptr:
                                             png_const_charpp)
 -> std::os::raw::c_int {
    static mut fmt_names: [*const std::os::raw::c_char; 7] =
        unsafe {
            [pbm_fmt_name.as_ptr(), pgm_fmt_name.as_ptr(),
             ppm_fmt_name.as_ptr(), pbm_fmt_name.as_ptr(),
             pgm_fmt_name.as_ptr(), ppm_fmt_name.as_ptr(),
             pam_fmt_name.as_ptr()]
        };
    static mut fmt_long_names: [*const std::os::raw::c_char; 7] =
        unsafe {
            [pbm_fmt_long_name.as_ptr(), pgm_fmt_long_name.as_ptr(),
             ppm_fmt_long_name.as_ptr(), pbm_fmt_long_name.as_ptr(),
             pgm_fmt_long_name.as_ptr(), ppm_fmt_long_name.as_ptr(),
             pam_fmt_long_name.as_ptr()]
        };
    /* Require at least the PNM magic signature and the trailing whitespace. */
    if sig_size < 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    } /* insufficient data */
    if *sig.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != 'P' as i32 ||
           (*sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               '1' as i32 ||
           *sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int > '7' as i32
       {
        return 0 as std::os::raw::c_int
    } /* not PNM */
    if *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int != ' ' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '\t' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '\n' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '\r' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int != '#' as i32
       {
        return 0 as std::os::raw::c_int
    } /* not PNM */
    /* Store the format name. */
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr =
            fmt_names[(*sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                           '1' as i32) as usize]
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr =
            fmt_long_names[(*sig.offset(1 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int - '1' as i32) as usize]
    }
    return 1 as std::os::raw::c_int;
    /* PNM */
}
fn pnm_fpeek_eof(pnm_ptr: &pnm_struct, stream: *mut FILE) -> std::os::raw::c_int {
    let mut ch: std::os::raw::c_int = 0;
    if pnm_ptr.format >= PNM_P1 as std::os::raw::c_int as std::os::raw::c_uint &&
       pnm_ptr.format <= PNM_P3 as std::os::raw::c_int as std::os::raw::c_uint {
        loop {
            unsafe {
                ch = getc(stream);
            }
            if ch == '#' as i32 {
                /* skip comments */
                loop {
                    unsafe {
                        ch = getc(stream);
                    }
                    if !(ch != -(1 as std::os::raw::c_int) && ch != '\n' as i32 &&
                         ch != '\r' as i32) {
                        break;
                    }
                }
            }
            if ch == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int }
            if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 ||
                 ch == '\r' as i32) {
                break;
            }
        }
    } else {
        unsafe {
            ch = getc(stream);
        }
        if ch == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int }
    }
    unsafe {
        ungetc(ch, stream);
    }
    return 0 as std::os::raw::c_int;
}


pub unsafe extern "C" fn pngx_read_pnm(png_ptr: *mut png_struct_def, 
                                        info_ptr: *mut png_info_def, 
                                        stream: *mut FILE) -> std::os::raw::c_int {
     let mut pnminfo: pnm_struct =
        pnm_struct{format: 0, depth: 0, width: 0, height: 0, maxval: 0,};
    let mut format: std::os::raw::c_uint = 0;
    let mut depth: std::os::raw::c_uint = 0;
    let mut width: std::os::raw::c_uint = 0;
    let mut height: std::os::raw::c_uint = 0;
    let mut maxval: std::os::raw::c_uint = 0;
    let mut max_width: std::os::raw::c_uint = 0;
    let mut num_samples: std::os::raw::c_uint = 0;
    let mut sample_size: std::os::raw::c_uint = 0;
    let mut pnmrow: *mut std::os::raw::c_uint = 0 as *mut std::os::raw::c_uint;
    let mut row_size: size_t = 0;
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    let mut sig_bit: png_color_8 =
        png_color_8{red: 0, green: 0, blue: 0, gray: 0, alpha: 0,};
    let mut i: std::os::raw::c_uint = 0;
    let mut j: std::os::raw::c_uint = 0;
    let mut failed: std::os::raw::c_int = 0;
    let mut overflow: std::os::raw::c_int = 0;
    /* Read the PNM header. */
    if pnm_fget_header(&mut pnminfo, stream) != 1 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    } /* not PNM */
    format = pnminfo.format;
    depth = pnminfo.depth;
    width = pnminfo.width;
    height = pnminfo.height;
    maxval = pnminfo.maxval;
    if format > PNM_P6 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle PNM formats newer than PPM (\"P6\")\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    max_width =
        if ::std::mem::size_of::<size_t>() as std::os::raw::c_ulong <=
               ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong {
            ((2147483647 as std::os::raw::c_int as
                  std::os::raw::c_uint).wrapping_mul(2 as
                                                 std::os::raw::c_uint).wrapping_add(1
                                                                                as
                                                                                std::os::raw::c_uint)
                 as
                 std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_uint>()
                                                 as
                                                 std::os::raw::c_ulong).wrapping_div(depth
                                                                                 as
                                                                                 std::os::raw::c_ulong)
        } else {
            (2147483647 as std::os::raw::c_int as
                 std::os::raw::c_uint).wrapping_mul(2 as
                                                std::os::raw::c_uint).wrapping_add(1
                                                                               as
                                                                               std::os::raw::c_uint)
                as std::os::raw::c_ulong
        } as std::os::raw::c_uint;
    if max_width > 0x7fffffff as std::os::raw::c_uint {
        max_width = 0x7fffffff as std::os::raw::c_uint
    }
    if width > max_width {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle exceedingly large PNM dimensions\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    sample_size = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    num_samples = depth.wrapping_mul(width);
    row_size = num_samples as size_t;
    if maxval > 65535 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle PNM samples larger than 16 bits\x00" as
                      *const u8 as *const std::os::raw::c_char);
    } else if maxval > 255 as std::os::raw::c_int as std::os::raw::c_uint {
        sample_size = 2 as std::os::raw::c_int as std::os::raw::c_uint;
        row_size =
            (row_size as
                 std::os::raw::c_ulong).wrapping_mul(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t
    }
    /* Set the PNG image type. */
    png_set_IHDR(&mut *png_ptr, &mut *info_ptr, width, height, if maxval <= 255 { 8 } else { 16 }, if depth == 1 { 0 } else { 2 }, 0, 0, 0);
    i = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    j = 2 as std::os::raw::c_int as std::os::raw::c_uint;
    while j.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) < maxval {
        i = i.wrapping_add(1);
        j <<= 1 as std::os::raw::c_int
    }
    if j.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) != maxval {
        png_warning(png_ptr as *const png_struct,
                    b"Possibly inexact sample conversion from PNM to PNG\x00"
                        as *const u8 as *const std::os::raw::c_char);
    } else if i.wrapping_rem(8 as std::os::raw::c_int as std::os::raw::c_uint) !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint &&
                  (depth > 1 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (8 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_rem(i) !=
                           0 as std::os::raw::c_int as std::os::raw::c_uint) {
        sig_bit.gray = i as png_byte;
        sig_bit.blue = sig_bit.gray;
        sig_bit.green = sig_bit.blue;
        sig_bit.red = sig_bit.green;
        sig_bit.alpha = 0 as std::os::raw::c_int as png_byte;
        png_set_sBIT(&mut *png_ptr, &mut *info_ptr, &sig_bit);
    }
    /* Allocate memory. */
    let row_pointers = pngx_malloc_rows(png_ptr, info_ptr, -1); /* can read raw data directly into row_pointers */
    if format >= PNM_P4 as std::os::raw::c_int as std::os::raw::c_uint &&
           (maxval == 255 as std::os::raw::c_int as std::os::raw::c_uint ||
                maxval == 65535 as std::os::raw::c_int as std::os::raw::c_uint) {
        pnmrow = 0 as *mut std::os::raw::c_uint
    } else {
    pnmrow = png_malloc(png_ptr as *const png_struct, (num_samples as std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong)) as *mut std::os::raw::c_uint;
}
    /* Read the image data. */
    failed = 0 as std::os::raw::c_int;
    overflow = 0 as std::os::raw::c_int;
   
     if !pnmrow.is_null() {
    let pnmrow = unsafe { std::slice::from_raw_parts_mut(pnmrow, num_samples as usize) };
    let mut i = 0u32;
    while i < height {
        if pnm_fget_values(&mut pnminfo, pnmrow.as_mut_ptr(), 1, stream) <= 0 {
            failed = 1;
        }
        /* Transfer the samples, even on partial (unsuccessful) reads. */
        if maxval <= 255 {
            let mut j = 0u32;
            while j < num_samples {
                let mut val = pnmrow[j as usize];
                if val > maxval {
                    val = 255;
                    overflow = 1;
                } else if maxval != 255 {
                    val = val * 255 / maxval;
                }
                let row_pointer = unsafe { std::slice::from_raw_parts_mut(*row_pointers.offset(i as isize), row_size as usize) };
                row_pointer[j as usize] = val as png_byte;
                j += 1;
            }
        } else {
            /* maxval > 255 */
            let mut j = 0u32;
            while j < num_samples {
                let mut val_0 = pnmrow[j as usize];
                if val_0 > maxval {
                    val_0 = 65535;
                    overflow = 1;
                } else if maxval != 65535 {
                    val_0 = val_0 * 65535 / maxval;
                }
                let row_pointer = unsafe { std::slice::from_raw_parts_mut(*row_pointers.offset(i as isize), row_size as usize) };
                row_pointer[(2 * j) as usize] = (val_0 >> 8) as png_byte;
                row_pointer[(2 * j + 1) as usize] = (val_0 & 0xff) as png_byte;
                j += 1;
            }
        }
        if failed != 0 { break; }
        i += 1;
    }
} else {
    /* read the raw data directly */
    let mut i = 0u32;
    while i < height {
        if pnm_fget_bytes(&mut pnminfo, *row_pointers.offset(i as isize), sample_size as u64, 1, stream) <= 0 {
            failed = 1;
            break;
        } else {
            i += 1;
        }
    }
}

/* Wipe out the portion left unread. */
while i < height {
    let row_pointer = unsafe { std::slice::from_raw_parts_mut(*row_pointers.offset(i as isize), row_size as usize) };
    row_pointer.fill(0);
    i += 1;
}

/* Deallocate the temporary row buffer. */
if !pnmrow.is_null() {
    png_free(png_ptr as *const png_struct, pnmrow as png_voidp);
}

/* Check the results. */
if overflow != 0 {
    png_warning(png_ptr as *const png_struct, b"Overflow in PNM samples\x00" as *const u8 as *const std::os::raw::c_char);
}
if failed != 0 {
    png_error(png_ptr as *const png_struct, b"Error in PNM image file\x00" as *const u8 as *const std::os::raw::c_char);
} else if pnm_fpeek_eof(&mut pnminfo, stream) == 0 {
    png_warning(png_ptr as *const png_struct, b"Extraneous data found after PNM image\x00" as *const u8 as *const std::os::raw::c_char);
}

/* FIXME: A PNM file can have more than one image. */
return 1;

    
    /* one image has been successfully read */
}



extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn memcpy(_: * mut core::ffi::c_void, _: * const core::ffi::c_void, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn memset(_: * mut core::ffi::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn __assert_fail(__assertion: * const std::os::raw::c_char,
                     __file: * const std::os::raw::c_char, __line: std::os::raw::c_uint,
                     __function: * const std::os::raw::c_char) -> !;
}
pub use crate::src::libpng::png::png_free_data;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngget::png_get_IHDR;
pub use crate::src::libpng::pngget::png_get_PLTE;
pub use crate::src::libpng::pngget::png_get_bKGD;
pub use crate::src::libpng::pngget::png_get_bit_depth;
pub use crate::src::libpng::pngget::png_get_channels;
pub use crate::src::libpng::pngget::png_get_color_type;
pub use crate::src::libpng::pngget::png_get_hIST;
pub use crate::src::libpng::pngget::png_get_image_height;
pub use crate::src::libpng::pngget::png_get_image_width;
pub use crate::src::libpng::pngget::png_get_rows;
pub use crate::src::libpng::pngget::png_get_sBIT;
pub use crate::src::libpng::pngget::png_get_tRNS;
pub use crate::src::libpng::pngget::png_get_valid;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::pngset::png_set_PLTE;
pub use crate::src::libpng::pngset::png_set_invalid;
pub use crate::src::libpng::pngset::png_set_tRNS;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub type size_t = std::os::raw::c_ulong;
pub type png_byte = std::os::raw::c_uchar;
pub type png_uint_16 = std::os::raw::c_ushort;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_size_t = std::os::raw::c_ulong;
pub type png_alloc_size_t = std::os::raw::c_ulong;
pub type png_voidp = * mut core::ffi::c_void;
pub type png_bytep = * mut std::os::raw::c_uchar;
pub type png_const_bytep = * const std::os::raw::c_uchar;
pub type png_uint_16p = * mut std::os::raw::c_ushort;
pub type png_const_charp = * const std::os::raw::c_char;
pub type png_bytepp = * mut * mut std::os::raw::c_uchar;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
pub type png_const_inforp = * const crate::src::libpng::png::png_info_def;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::png::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color_struct;
pub type png_colorp = * mut crate::src::libpng::png::png_color_struct;
pub type png_const_colorp = * const crate::src::libpng::png::png_color_struct;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::png::png_color_16_struct;
pub type png_color_16 = crate::src::libpng::png::png_color_16_struct;
pub type png_color_16p = * mut crate::src::libpng::png::png_color_16_struct;
pub type png_const_color_16p = * const crate::src::libpng::png::png_color_16_struct;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::png::png_color_8_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8_struct;
pub type png_color_8p = * mut crate::src::libpng::png::png_color_8_struct;
// #[derive(Copy, Clone)]

pub type png_row_info_struct = crate::src::libpng::png::png_row_info_struct;
pub type png_row_info = crate::src::libpng::png::png_row_info_struct;
pub type png_row_infop = * mut crate::src::libpng::png::png_row_info_struct;
/*
 * opngreduc.c - libpng extension: lossless image reductions.
 *
 * Copyright (C) 2003-2014 Cosmin Truta.
 * This software is distributed under the same licensing and warranty terms
 * as libpng.
 */
/* CAUTION:
 * Image reductions do not work well under certain transformations.
 *
 * Transformations like PNG_BGR, PNG_SWAP_BYTES, PNG_FILLER, PNG_INVERT_ALPHA,
 * and possibly others, require special treatment. However, the libpng API
 * does not currently convey the effect of transformations on its internal
 * state or on the layout of pixel data.
 *
 * Transformations which affect pixel depth (e.g. PNG_FILLER) are especially
 * dangerous when used in conjunction with this code, and should be avoided.
 */
/*
 * Check if the image information is valid.
 * The image information is said to be valid if all the required
 * critical chunk data is present in the png structures.
 * The function returns 1 if this information is valid, and 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_validate_image(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                             mut info_ptr: * mut crate::src::libpng::png::png_info_def)
 -> std::os::raw::c_int {
    /* Validate IHDR. */
    if png_get_bit_depth(png_ptr as *const png_struct,
                         info_ptr as *const png_info) as std::os::raw::c_int ==
           0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    /* Validate PLTE. */
    if png_get_color_type(png_ptr as *const png_struct,
                          info_ptr as *const png_info) as std::os::raw::c_int &
           1 as std::os::raw::c_int != 0 {
        if png_get_valid(png_ptr as *const png_struct,
                         info_ptr as *const png_info, 0x8 as std::os::raw::c_uint) ==
               0 {
            return 0 as std::os::raw::c_int
        }
    }
    /* Validate IDAT. */
    if png_get_valid(png_ptr as *const png_struct,
                     info_ptr as *const png_info, 0x8000 as std::os::raw::c_uint) == 0
       {
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/*
 * Build a color+alpha palette in which the entries are sorted by
 * (alpha, red, green, blue), in this particular order.
 * Use the insertion sort algorithm.
 * The alpha value is ignored if it is not in the range [0 .. 255].
 * The function returns:
 *   1 if the insertion is successful;  *index = position of new entry.
 *   0 if the insertion is unnecessary; *index = position of crt entry.
 *  -1 if overflow;            *num_palette = *num_trans = *index = -1.
 */
unsafe extern "C" fn opng_insert_palette_entry<'a1, 'a2, 'a3>(mut palette: * mut crate::src::libpng::png::png_color_struct,
                                               mut num_palette:
                                                   Option<&'a1 mut std::os::raw::c_int>,
                                               mut trans_alpha: * mut std::os::raw::c_uchar,
                                               mut num_trans:
                                                   Option<&'a2 mut std::os::raw::c_int>,
                                               mut max_tuples: std::os::raw::c_int,
                                               mut red: std::os::raw::c_uint,
                                               mut green: std::os::raw::c_uint,
                                               mut blue: std::os::raw::c_uint,
                                               mut alpha: std::os::raw::c_uint,
                                               mut index: Option<&'a3 mut std::os::raw::c_int>)
 -> std::os::raw::c_int {
    let mut low: i32 = 0;
    let mut high: i32 = 0;
    let mut mid: i32 = 0;
    let mut cmp: i32 = 0;
    let mut i: i32 = 0;
    if *(borrow(& num_palette)).unwrap() >= 0 as std::os::raw::c_int && *(borrow(& num_palette)).unwrap() <= max_tuples {
    } else {
        __assert_fail(b"*num_palette >= 0 && *num_palette <= max_tuples\x00"
                          as *const u8 as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      109 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 135], &'_ [i8; 135]>(b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\x00")).as_ptr());
    }
    if *(borrow(& num_trans)).unwrap() >= 0 as std::os::raw::c_int && *(borrow(& num_trans)).unwrap() <= *(borrow(& num_palette)).unwrap() {
    } else {
        __assert_fail(b"*num_trans >= 0 && *num_trans <= *num_palette\x00" as
                          *const u8 as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      110 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 135], &'_ [i8; 135]>(b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\x00")).as_ptr());
    }
    if alpha < 255 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Do a binary search among transparent tuples. */
        low = 0 as std::os::raw::c_int;
        high = *(borrow(& num_trans)).unwrap() - 1 as std::os::raw::c_int;
        while low <= high {
            mid = (low + high) / 2 as std::os::raw::c_int;
            cmp =
                if alpha as std::os::raw::c_int !=
                       *trans_alpha.offset(mid as isize) as std::os::raw::c_int {
                    (alpha as std::os::raw::c_int) -
                        *trans_alpha.offset(mid as isize) as std::os::raw::c_int
                } else if red as std::os::raw::c_int !=
                              (*palette.offset(mid as isize)).red as
                                  std::os::raw::c_int {
                    (red as std::os::raw::c_int) -
                        (*palette.offset(mid as isize)).red as std::os::raw::c_int
                } else if green as std::os::raw::c_int !=
                              (*palette.offset(mid as isize)).green as
                                  std::os::raw::c_int {
                    (green as std::os::raw::c_int) -
                        (*palette.offset(mid as isize)).green as std::os::raw::c_int
                } else {
                    (blue as std::os::raw::c_int) -
                        (*palette.offset(mid as isize)).blue as std::os::raw::c_int
                };
            if cmp < 0 as std::os::raw::c_int {
                high = mid - 1 as std::os::raw::c_int
            } else if cmp > 0 as std::os::raw::c_int {
                low = mid + 1 as std::os::raw::c_int
            } else { *(borrow_mut(&mut index)).unwrap() = mid; return 0 as std::os::raw::c_int }
        }
    } else {
        /* alpha == 255 || alpha not in [0 .. 255] */
        /* Do a (faster) binary search among opaque tuples. */
        low = *(borrow_mut(&mut num_trans)).unwrap();
        high = *(borrow(& num_palette)).unwrap() - 1 as std::os::raw::c_int;
        while low <= high {
            mid = (low + high) / 2 as std::os::raw::c_int;
            cmp =
                if red as std::os::raw::c_int !=
                       (*palette.offset(mid as isize)).red as std::os::raw::c_int {
                    (red as std::os::raw::c_int) -
                        (*palette.offset(mid as isize)).red as std::os::raw::c_int
                } else if green as std::os::raw::c_int !=
                              (*palette.offset(mid as isize)).green as
                                  std::os::raw::c_int {
                    (green as std::os::raw::c_int) -
                        (*palette.offset(mid as isize)).green as std::os::raw::c_int
                } else {
                    (blue as std::os::raw::c_int) -
                        (*palette.offset(mid as isize)).blue as std::os::raw::c_int
                };
            if cmp < 0 as std::os::raw::c_int {
                high = mid - 1 as std::os::raw::c_int
            } else if cmp > 0 as std::os::raw::c_int {
                low = mid + 1 as std::os::raw::c_int
            } else { *(borrow_mut(&mut index)).unwrap() = mid; return 0 as std::os::raw::c_int }
        }
    }
    if alpha > 255 as std::os::raw::c_int as std::os::raw::c_uint {
        /* The binary search among opaque tuples has failed. */
      /* Do a linear search among transparent tuples, ignoring alpha. */
        i = 0 as std::os::raw::c_int;
        while i < *(borrow(& num_trans)).unwrap() {
            cmp =
                if red as std::os::raw::c_int !=
                       (*palette.offset(i as isize)).red as std::os::raw::c_int {
                    (red as std::os::raw::c_int) -
                        (*palette.offset(i as isize)).red as std::os::raw::c_int
                } else if green as std::os::raw::c_int !=
                              (*palette.offset(i as isize)).green as
                                  std::os::raw::c_int {
                    (green as std::os::raw::c_int) -
                        (*palette.offset(i as isize)).green as std::os::raw::c_int
                } else {
                    (blue as std::os::raw::c_int) -
                        (*palette.offset(i as isize)).blue as std::os::raw::c_int
                };
            if cmp == 0 as std::os::raw::c_int { *(borrow_mut(&mut index)).unwrap() = i; return 0 as std::os::raw::c_int }
            i += 1
        }
    }
    /* Check for overflow. */
    if *(borrow(& num_palette)).unwrap() >= max_tuples {
        *(borrow_mut(&mut index)).unwrap() = -(1 as std::os::raw::c_int);
        *(borrow_mut(&mut num_trans)).unwrap() = *(borrow_mut(&mut index)).unwrap();
        *(borrow_mut(&mut num_palette)).unwrap() = *(borrow_mut(&mut num_trans)).unwrap();
        return -(1 as std::os::raw::c_int)
    }
    /* Insert new tuple at [low]. */
    if low >= 0 as std::os::raw::c_int && low <= *(borrow(& num_palette)).unwrap() {
    } else {
        __assert_fail(b"low >= 0 && low <= *num_palette\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      179 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 135], &'_ [i8; 135]>(b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\x00")).as_ptr());
    }
    i = *(borrow_mut(&mut num_palette)).unwrap();
    while i > low {
        *palette.offset(i as isize) =
            *palette.offset((i - 1 as std::os::raw::c_int) as isize);
        i -= 1
    }
    (*palette.offset(low as isize)).red = red as png_byte;
    (*palette.offset(low as isize)).green = green as png_byte;
    (*palette.offset(low as isize)).blue = blue as png_byte;
    *(borrow_mut(&mut num_palette)).unwrap() += 1;
    if alpha < 255 as std::os::raw::c_int as std::os::raw::c_uint {
        if low <= *(borrow(& num_trans)).unwrap() {
        } else {
            __assert_fail(b"low <= *num_trans\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          188 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 135], &'_ [i8; 135]>(b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\x00")).as_ptr());
        }
        i = *(borrow_mut(&mut num_trans)).unwrap();
        while i > low {
            *trans_alpha.offset(i as isize) =
                *trans_alpha.offset((i - 1 as std::os::raw::c_int) as isize);
            i -= 1
        }
        *trans_alpha.offset(low as isize) = alpha as png_byte;
        *(borrow_mut(&mut num_trans)).unwrap() += 1
    }
    *(borrow_mut(&mut index)).unwrap() = low;
    return 1 as std::os::raw::c_int;
}
/*
 * Change the size of the palette buffer.
 * Changing info_ptr->num_palette directly, avoiding reallocation, should
 * have been sufficient, but can't be done using the current libpng API.
 */
unsafe extern "C" fn opng_realloc_PLTE(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                       mut num_palette: std::os::raw::c_int) {
    let mut buffer: [crate::src::libpng::png::png_color_struct; 256] =
        [png_color{red: 0, green: 0, blue: 0,}; 256];
    let mut palette: * mut crate::src::libpng::png::png_color_struct = 0 as *mut png_color;
    let mut src_num_palette: i32 = 0;
    if num_palette > 0 as std::os::raw::c_int {
    } else {
        __assert_fail(b"num_palette > 0\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      212 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 52], &'_ [i8; 52]>(b"void opng_realloc_PLTE(png_structp, png_infop, int)\x00")).as_ptr());
    }
    src_num_palette = 0 as std::os::raw::c_int;
    png_get_PLTE(png_ptr as *const png_struct, info_ptr, Some(&mut palette),
                 Some(&mut src_num_palette));
    if num_palette == src_num_palette { return }
    memcpy(buffer.as_mut_ptr() as *mut std::os::raw::c_void,
           palette as *const std::os::raw::c_void,
           (num_palette as
                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_color>()
                                                as std::os::raw::c_ulong));
    if num_palette > src_num_palette {
        memset(buffer.as_mut_ptr().offset(src_num_palette as isize) as
                   *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ((num_palette - src_num_palette) as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_color>()
                                                    as std::os::raw::c_ulong));
    }
    png_set_PLTE(png_ptr, info_ptr, buffer.as_mut_ptr() as png_const_colorp,
                 num_palette);
}
/*
 * Change the size of the transparency buffer.
 * Changing info_ptr->num_trans directly, avoiding reallocation, should
 * have been sufficient, but can't be done using the current libpng API.
 */
unsafe extern "C" fn opng_realloc_tRNS(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                       mut num_trans: std::os::raw::c_int) {
    let mut buffer: [u8; 256] =
        [0; 256]; /* tRNS should be invalidated in this case */
    let mut trans_alpha: * mut u8 = 0 as *mut png_byte;
    let mut src_num_trans: i32 = 0;
    if num_trans > 0 as std::os::raw::c_int {
    } else {
        __assert_fail(b"num_trans > 0\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      238 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 52], &'_ [i8; 52]>(b"void opng_realloc_tRNS(png_structp, png_infop, int)\x00")).as_ptr());
    }
    src_num_trans = 0 as std::os::raw::c_int;
    png_get_tRNS(png_ptr as *const png_struct, info_ptr, Some(&mut trans_alpha),
                 Some(&mut src_num_trans), Option::<&'_ mut Option<&'_ mut crate::src::libpng::png::png_color_16_struct>>::None);
    if num_trans == src_num_trans { return }
    memcpy(buffer.as_mut_ptr() as *mut std::os::raw::c_void,
           trans_alpha as *const std::os::raw::c_void, num_trans as size_t);
    if num_trans > src_num_trans {
        memset(buffer.as_mut_ptr().offset(src_num_trans as isize) as
                   *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               (num_trans - src_num_trans) as std::os::raw::c_ulong);
    }
    png_set_tRNS(png_ptr, info_ptr, buffer.as_mut_ptr() as png_const_bytep,
                 num_trans, Option::<&'_ crate::src::libpng::png::png_color_16_struct>::None);
}
/*
 * Retrieve the alpha samples from the given image row.
 */
unsafe extern "C" fn opng_get_alpha_row<'a1, 'a2>(mut row_info_ptr: Option<&'a1 mut crate::src::libpng::png::png_row_info_struct>,
                                        mut trans_color: Option<&'a2 mut crate::src::libpng::png::png_color_16_struct>,
                                        mut row: * mut std::os::raw::c_uchar,
                                        mut alpha_row: * mut std::os::raw::c_uchar) {
    let mut sample_ptr: * mut u8 = 0 as *mut png_byte;
    let mut width: u32 = 0;
    let mut color_type: i32 = 0;
    let mut bit_depth: i32 = 0;
    let mut channels: i32 = 0;
    let mut trans_red: u8 = 0;
    let mut trans_green: u8 = 0;
    let mut trans_blue: u8 = 0;
    let mut trans_gray: u8 = 0;
    let mut i: u32 = 0;
    width = (*(borrow_mut(&mut row_info_ptr)).unwrap()).width;
    color_type = (*(borrow_mut(&mut row_info_ptr)).unwrap()).color_type as std::os::raw::c_int;
    bit_depth = (*(borrow_mut(&mut row_info_ptr)).unwrap()).bit_depth as std::os::raw::c_int;
    channels = (*(borrow_mut(&mut row_info_ptr)).unwrap()).channels as std::os::raw::c_int;
    if color_type & 1 as std::os::raw::c_int == 0 {
    } else {
        __assert_fail(b"!(color_type & 1)\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      267 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 76], &'_ [i8; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
    }
    if bit_depth == 8 as std::os::raw::c_int {
    } else {
        __assert_fail(b"bit_depth == 8\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      268 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 76], &'_ [i8; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
    }
    if color_type & 4 as std::os::raw::c_int == 0 {
        if borrow(& trans_color).is_none() {
            /* All pixels are fully opaque. */
            memset(alpha_row as *mut std::os::raw::c_void, 255 as std::os::raw::c_int,
                   width as size_t);
            return
        }
        if color_type == 2 as std::os::raw::c_int {
            if channels == 3 as std::os::raw::c_int {
            } else {
                __assert_fail(b"channels == 3\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              280 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*core::intrinsics::transmute::<&'_ [u8; 76], &'_ [i8; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
            }
            trans_red = (*(borrow_mut(&mut trans_color)).unwrap()).red as png_byte;
            trans_green = (*(borrow_mut(&mut trans_color)).unwrap()).green as png_byte;
            trans_blue = (*(borrow_mut(&mut trans_color)).unwrap()).blue as png_byte;
            sample_ptr = row;
            i = 0 as std::os::raw::c_int as png_uint_32;
            while i < width {
                *alpha_row.offset(i as isize) =
                    if *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int == trans_red as std::os::raw::c_int &&
                           *sample_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == trans_green as std::os::raw::c_int &&
                           *sample_ptr.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == trans_blue as std::os::raw::c_int {
                        0 as std::os::raw::c_int
                    } else { 255 as std::os::raw::c_int } as png_byte;
                i = i.wrapping_add(1);
                sample_ptr = sample_ptr.offset(3 as std::os::raw::c_int as isize)
            }
        } else {
            if color_type == 0 as std::os::raw::c_int {
            } else {
                __assert_fail(b"color_type == 0\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              293 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*core::intrinsics::transmute::<&'_ [u8; 76], &'_ [i8; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
            }
            if channels == 1 as std::os::raw::c_int {
            } else {
                __assert_fail(b"channels == 1\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              294 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*core::intrinsics::transmute::<&'_ [u8; 76], &'_ [i8; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
            }
            trans_gray = (*(borrow_mut(&mut trans_color)).unwrap()).gray as png_byte;
            i = 0 as std::os::raw::c_int as png_uint_32;
            while i < width {
                *alpha_row.offset(i as isize) =
                    if *row.offset(i as isize) as std::os::raw::c_int ==
                           trans_gray as std::os::raw::c_int {
                        0 as std::os::raw::c_int
                    } else { 255 as std::os::raw::c_int } as png_byte;
                i = i.wrapping_add(1)
            }
        }
        return
    }
    /* There is a real alpha channel. The alpha sample is last in RGBA tuple. */
    if channels > 1 as std::os::raw::c_int {
    } else {
        __assert_fail(b"channels > 1\x00" as *const u8 as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      303 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 76], &'_ [i8; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
    }
    sample_ptr = row.offset((channels - 1 as std::os::raw::c_int) as isize);
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < width {
        *alpha_row = *sample_ptr;
        i = i.wrapping_add(1);
        sample_ptr = sample_ptr.offset(channels as isize);
        alpha_row = alpha_row.offset(1)
    };
}
/*
 * Analyze the redundancy of bits inside the image.
 * The parameter reductions indicates the intended reductions.
 * The function returns the possible reductions.
 */
unsafe extern "C" fn opng_analyze_bits(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                       mut reductions: std::os::raw::c_uint)
 -> std::os::raw::c_uint {
    let mut row_ptr: * mut * mut u8 =
        0 as *mut *mut png_byte; /* not applicable */
    let mut component_ptr: * mut u8 =
        0 as *mut png_byte; /* let opng_reduce_palette() handle it */
    let mut height: u32 = 0;
    let mut width: u32 = 0;
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut byte_depth: i32 = 0;
    let mut channels: i32 = 0;
    let mut sample_size: i32 = 0;
    let mut offset_alpha: i32 = 0;
    let mut background: Option<&'_ mut crate::src::libpng::png::png_color_16_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 Some(&mut width), Some(&mut height), Some(&mut bit_depth), Some(&mut color_type),
                 Option::<&'_ mut i32>::None, Option::<&'_ mut i32>::None,
                 Option::<&'_ mut i32>::None);
    if bit_depth < 8 as std::os::raw::c_int { return 0 as std::os::raw::c_int as png_uint_32 }
    if color_type & 1 as std::os::raw::c_int != 0 {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    byte_depth = bit_depth / 8 as std::os::raw::c_int;
    channels =
        png_get_channels(png_ptr as *const png_struct,
                         info_ptr as *const png_info) as std::os::raw::c_int;
    sample_size = channels * byte_depth;
    offset_alpha = (channels - 1 as std::os::raw::c_int) * byte_depth;
    /* Select the applicable reductions. */
    reductions &=
        (0x1 as std::os::raw::c_int | 0x4 as std::os::raw::c_int | 0x8 as std::os::raw::c_int) as
            std::os::raw::c_uint;
    if bit_depth <= 8 as std::os::raw::c_int {
        reductions &= !(0x1 as std::os::raw::c_int) as std::os::raw::c_uint
    }
    if color_type & 2 as std::os::raw::c_int == 0 {
        reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint
    }
    if color_type & 4 as std::os::raw::c_int == 0 {
        reductions &= !(0x8 as std::os::raw::c_int) as std::os::raw::c_uint
    }
    /* Check if the ancillary information allows these reductions. */
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, Some(&mut background))
           != 0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*(borrow(& background)).unwrap()).red as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                   0 as std::os::raw::c_int ||
                   (*(borrow(& background)).unwrap()).green as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                       0 as std::os::raw::c_int ||
                   (*(borrow(& background)).unwrap()).blue as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                       0 as std::os::raw::c_int ||
                   (*(borrow(& background)).unwrap()).gray as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                       0 as std::os::raw::c_int {
                reductions &= !(0x1 as std::os::raw::c_int) as std::os::raw::c_uint
            }
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*(borrow(& background)).unwrap()).red as std::os::raw::c_int !=
                   (*(borrow(& background)).unwrap()).green as std::os::raw::c_int ||
                   (*(borrow(& background)).unwrap()).red as std::os::raw::c_int !=
                       (*(borrow(& background)).unwrap()).blue as std::os::raw::c_int {
                reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint
            }
        }
    }
    /* Check for each possible reduction, row by row. */
    row_ptr =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as
                         *const png_info); /* no need to go any further */
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        if reductions == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as std::os::raw::c_int as png_uint_32
        }
        /* Check if it is possible to reduce the bit depth to 8. */
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            component_ptr = *row_ptr;
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < (channels as std::os::raw::c_uint).wrapping_mul(width) {
                if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int !=
                       *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int {
                    reductions &= !(0x1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    break ;
                } else {
                    j = j.wrapping_add(1);
                    component_ptr =
                        component_ptr.offset(2 as std::os::raw::c_int as isize)
                }
            }
        }
        if bit_depth == 8 as std::os::raw::c_int {
            /* Check if it is possible to reduce rgb --> gray. */
            if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = *row_ptr;
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int !=
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int ||
                           *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(2 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                       {
                        reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
            /* Check if it is possible to strip the alpha channel. */
            if reductions & 0x8 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = (*row_ptr).offset(offset_alpha as isize);
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int != 255 as std::os::raw::c_int {
                        reductions &= !(0x8 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
        } else {
            /* bit_depth == 16 */
            /* Check if it is possible to reduce rgb --> gray. */
            if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = *row_ptr;
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int !=
                           *component_ptr.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int ||
                           *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(4 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                           ||
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(3 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                           ||
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(5 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                       {
                        reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
            if reductions & 0x8 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = (*row_ptr).offset(offset_alpha as isize);
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int != 255 as std::os::raw::c_int ||
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int != 255 as std::os::raw::c_int {
                        reductions &= !(0x8 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    return reductions;
}
/* Check if it is possible to strip the alpha channel. */
/*
 * Reduce the image type to a lower bit depth and color type,
 * by removing redundant bits.
 * Possible reductions: 16bpp to 8bpp; RGB to gray; strip alpha.
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 * All reductions are performed in a single step.
 */
unsafe extern "C" fn opng_reduce_bits(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                      mut reductions: std::os::raw::c_uint)
 -> std::os::raw::c_uint {
    let mut row_ptr: * mut * mut u8 = 0 as *mut *mut png_byte;
    let mut src_ptr: * mut u8 = 0 as *mut png_byte;
    let mut dest_ptr: * mut u8 = 0 as *mut png_byte;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut interlace_type: i32 = 0;
    let mut compression_type: i32 = 0;
    let mut filter_type: i32 = 0;
    let mut src_bit_depth: i32 = 0;
    let mut dest_bit_depth: i32 = 0;
    let mut src_byte_depth: i32 = 0;
    let mut dest_byte_depth: i32 = 0;
    let mut src_color_type: i32 = 0;
    let mut dest_color_type: i32 = 0;
    let mut src_channels: i32 = 0;
    let mut dest_channels: i32 = 0;
    let mut src_sample_size: i32 = 0;
    let mut dest_sample_size: i32 = 0;
    let mut tran_tbl: [i32; 8] = [0; 8];
    let mut trans_color: Option<&'_ mut crate::src::libpng::png::png_color_16_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    let mut background: Option<&'_ mut crate::src::libpng::png::png_color_16_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    let mut sig_bits: Option<&'_ mut crate::src::libpng::png::png_color_8_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_8_struct>::None;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut k: i32 = 0;
    /* See which reductions may be performed. */
    reductions =
        opng_analyze_bits(png_ptr, info_ptr, reductions); /* exit early */
    if reductions == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 Some(&mut width), Some(&mut height), Some(&mut src_bit_depth),
                 Some(&mut src_color_type), Some(&mut interlace_type),
                 Some(&mut compression_type), Some(&mut filter_type));
    /* Compute the new image parameters bit_depth, color_type, etc. */
    if src_bit_depth >= 8 as std::os::raw::c_int {
    } else {
        __assert_fail(b"src_bit_depth >= 8\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      506 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 66], &'_ [i8; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if src_bit_depth == 16 as std::os::raw::c_int {
        } else {
            __assert_fail(b"src_bit_depth == 16\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          509 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 66], &'_ [i8; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        dest_bit_depth = 8 as std::os::raw::c_int
    } else { dest_bit_depth = src_bit_depth }
    src_byte_depth = src_bit_depth / 8 as std::os::raw::c_int;
    dest_byte_depth = dest_bit_depth / 8 as std::os::raw::c_int;
    dest_color_type = src_color_type;
    if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if src_color_type & 2 as std::os::raw::c_int != 0 {
        } else {
            __assert_fail(b"src_color_type & 2\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          521 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 66], &'_ [i8; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        dest_color_type &= !(2 as std::os::raw::c_int)
    }
    if reductions & 0x8 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if src_color_type & 4 as std::os::raw::c_int != 0 {
        } else {
            __assert_fail(b"src_color_type & 4\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          526 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 66], &'_ [i8; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        dest_color_type &= !(4 as std::os::raw::c_int)
    }
    src_channels =
        png_get_channels(png_ptr as *const png_struct,
                         info_ptr as *const png_info) as std::os::raw::c_int;
    dest_channels =
        (if dest_color_type & 2 as std::os::raw::c_int != 0 {
             3 as std::os::raw::c_int
         } else { 1 as std::os::raw::c_int }) +
            (if dest_color_type & 4 as std::os::raw::c_int != 0 {
                 1 as std::os::raw::c_int
             } else { 0 as std::os::raw::c_int });
    src_sample_size = src_channels * src_byte_depth;
    dest_sample_size = dest_channels * dest_byte_depth;
    /* Pre-compute the intra-sample translation table. */
    k = 0 as std::os::raw::c_int;
    while k < 4 as std::os::raw::c_int * dest_byte_depth {
        tran_tbl[k as usize] = k * src_bit_depth / dest_bit_depth;
        k += 1
    }
    /* If rgb --> gray, shift the alpha component two positions to the left. */
    if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 &&
           dest_color_type & 4 as std::os::raw::c_int != 0 {
        tran_tbl[dest_byte_depth as usize] =
            tran_tbl[(3 as std::os::raw::c_int * dest_byte_depth) as usize];
        if dest_byte_depth == 2 as std::os::raw::c_int {
            tran_tbl[(dest_byte_depth + 1 as std::os::raw::c_int) as usize] =
                tran_tbl[(3 as std::os::raw::c_int * dest_byte_depth +
                              1 as std::os::raw::c_int) as usize]
        }
    }
    /* Translate the samples to the new image type. */
    if src_sample_size > dest_sample_size {
    } else {
        __assert_fail(b"src_sample_size > dest_sample_size\x00" as *const u8
                          as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      551 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 66], &'_ [i8; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    row_ptr =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as *const png_info);
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        dest_ptr = *row_ptr;
        src_ptr = dest_ptr;
        j = 0 as std::os::raw::c_int as png_uint_32;
        while j < width {
            k = 0 as std::os::raw::c_int;
            while k < dest_sample_size {
                *dest_ptr.offset(k as isize) =
                    *src_ptr.offset(tran_tbl[k as usize] as isize);
                k += 1
            }
            src_ptr = src_ptr.offset(src_sample_size as isize);
            dest_ptr = dest_ptr.offset(dest_sample_size as isize);
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    /* Update the ancillary information. */
    if png_get_tRNS(png_ptr as *const png_struct, info_ptr,
                    Option::<&'_ mut * mut u8>::None, Option::<&'_ mut i32>::None,
                    Some(&mut trans_color)) != 0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*(borrow(& trans_color)).unwrap()).red as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
                   (*(borrow(& trans_color)).unwrap()).green as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                       0 as std::os::raw::c_int &&
                   (*(borrow(& trans_color)).unwrap()).blue as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                       0 as std::os::raw::c_int &&
                   (*(borrow(& trans_color)).unwrap()).gray as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                (*(borrow_mut(&mut trans_color)).unwrap()).red =
                    ((*(borrow(& trans_color)).unwrap()).red as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16;
                (*(borrow_mut(&mut trans_color)).unwrap()).green =
                    ((*(borrow(& trans_color)).unwrap()).green as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16;
                (*(borrow_mut(&mut trans_color)).unwrap()).blue =
                    ((*(borrow(& trans_color)).unwrap()).blue as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16;
                (*(borrow_mut(&mut trans_color)).unwrap()).gray =
                    ((*(borrow(& trans_color)).unwrap()).gray as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16
            } else {
                /* 16-bit tRNS in 8-bit samples: all pixels are 100% opaque. */
                png_free_data(png_ptr as *const png_struct, info_ptr,
                              0x2000 as std::os::raw::c_uint, -(1 as std::os::raw::c_int));
                png_set_invalid(png_ptr as *const png_struct, info_ptr,
                                0x10 as std::os::raw::c_uint as std::os::raw::c_int);
            }
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*(borrow(& trans_color)).unwrap()).red as std::os::raw::c_int ==
                   (*(borrow(& trans_color)).unwrap()).green as std::os::raw::c_int ||
                   (*(borrow(& trans_color)).unwrap()).red as std::os::raw::c_int ==
                       (*(borrow(& trans_color)).unwrap()).blue as std::os::raw::c_int {
                (*(borrow_mut(&mut trans_color)).unwrap()).gray = (*(borrow_mut(&mut trans_color)).unwrap()).red
            } else {
                /* Non-gray tRNS in grayscale image: all pixels are 100% opaque. */
                png_free_data(png_ptr as *const png_struct, info_ptr,
                              0x2000 as std::os::raw::c_uint, -(1 as std::os::raw::c_int));
                png_set_invalid(png_ptr as *const png_struct, info_ptr,
                                0x10 as std::os::raw::c_uint as std::os::raw::c_int);
            }
        }
    }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, Some(&mut background))
           != 0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            (*(borrow_mut(&mut background)).unwrap()).red =
                ((*(borrow(& background)).unwrap()).red as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16;
            (*(borrow_mut(&mut background)).unwrap()).green =
                ((*(borrow(& background)).unwrap()).green as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16;
            (*(borrow_mut(&mut background)).unwrap()).blue =
                ((*(borrow(& background)).unwrap()).blue as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16;
            (*(borrow_mut(&mut background)).unwrap()).gray =
                ((*(borrow(& background)).unwrap()).gray as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            (*(borrow_mut(&mut background)).unwrap()).gray = (*(borrow_mut(&mut background)).unwrap()).red
        }
    }
    if png_get_sBIT(png_ptr as *const png_struct, info_ptr, Some(&mut sig_bits)) !=
           0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*(borrow(& sig_bits)).unwrap()).red as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*(borrow_mut(&mut sig_bits)).unwrap()).red = 8 as std::os::raw::c_int as png_byte
            }
            if (*(borrow(& sig_bits)).unwrap()).green as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*(borrow_mut(&mut sig_bits)).unwrap()).green = 8 as std::os::raw::c_int as png_byte
            }
            if (*(borrow(& sig_bits)).unwrap()).blue as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*(borrow_mut(&mut sig_bits)).unwrap()).blue = 8 as std::os::raw::c_int as png_byte
            }
            if (*(borrow(& sig_bits)).unwrap()).gray as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*(borrow_mut(&mut sig_bits)).unwrap()).gray = 8 as std::os::raw::c_int as png_byte
            }
            if (*(borrow(& sig_bits)).unwrap()).alpha as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*(borrow_mut(&mut sig_bits)).unwrap()).alpha = 8 as std::os::raw::c_int as png_byte
            }
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            let mut max_sig_bits: u8 = (*(borrow_mut(&mut sig_bits)).unwrap()).red;
            if (max_sig_bits as std::os::raw::c_int) <
                   (*(borrow(& sig_bits)).unwrap()).green as std::os::raw::c_int {
                max_sig_bits = (*(borrow_mut(&mut sig_bits)).unwrap()).green
            }
            if (max_sig_bits as std::os::raw::c_int) < (*(borrow(& sig_bits)).unwrap()).blue as std::os::raw::c_int
               {
                max_sig_bits = (*(borrow_mut(&mut sig_bits)).unwrap()).blue
            }
            (*(borrow_mut(&mut sig_bits)).unwrap()).gray = max_sig_bits
        }
    }
    /* Update the image information. */
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 dest_bit_depth, dest_color_type, interlace_type,
                 compression_type, filter_type);
    return reductions;
}
/*
 * Reduce the bit depth of a palette image to the lowest possible value.
 * The parameter reductions should contain OPNG_REDUCE_8_TO_4_2_1.
 * The function returns OPNG_REDUCE_8_TO_4_2_1 if successful.
 */
unsafe extern "C" fn opng_reduce_palette_bits(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                              mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                              mut reductions: std::os::raw::c_uint)
 -> std::os::raw::c_uint {
    let mut row_ptr: * mut * mut u8 = 0 as *mut *mut png_byte;
    let mut src_sample_ptr: * mut u8 = 0 as *mut png_byte;
    let mut dest_sample_ptr: * mut u8 = 0 as *mut png_byte;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut color_type: i32 = 0;
    let mut interlace_type: i32 = 0;
    let mut compression_type: i32 = 0;
    let mut filter_type: i32 = 0;
    let mut src_bit_depth: i32 = 0;
    let mut dest_bit_depth: i32 = 0;
    let mut src_mask_init: u32 = 0;
    let mut src_mask: u32 = 0;
    let mut src_shift: u32 = 0;
    let mut dest_shift: u32 = 0;
    let mut sample: u32 = 0;
    let mut dest_buf: u32 = 0;
    let mut palette: * mut crate::src::libpng::png::png_color_struct = 0 as *mut png_color;
    let mut num_palette: i32 = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    /* Check if the reduction applies. */
    if reductions & 0x2 as std::os::raw::c_int as std::os::raw::c_uint == 0 {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 Some(&mut width), Some(&mut height), Some(&mut src_bit_depth), Some(&mut color_type),
                 Some(&mut interlace_type), Some(&mut compression_type),
                 Some(&mut filter_type));
    if color_type != 2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    if png_get_PLTE(png_ptr as *const png_struct, info_ptr, Some(&mut palette),
                    Some(&mut num_palette)) == 0 {
        num_palette = 0 as std::os::raw::c_int
    }
    /* Find the smallest possible bit depth. */
    if num_palette > 16 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as png_uint_32
    } else {
        if num_palette > 4 as std::os::raw::c_int {
            /* 5 .. 16 entries */
            dest_bit_depth = 4 as std::os::raw::c_int
        } else if num_palette > 2 as std::os::raw::c_int {
            /* 3 or 4 entries */
            dest_bit_depth = 2 as std::os::raw::c_int
        } else {
            /* 1 or 2 entries */
            if num_palette > 0 as std::os::raw::c_int {
            } else {
                __assert_fail(b"num_palette > 0\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              691 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*core::intrinsics::transmute::<&'_ [u8; 74], &'_ [i8; 74]>(b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
            }
            dest_bit_depth = 1 as std::os::raw::c_int
        }
    }
    if src_bit_depth <= dest_bit_depth {
        if src_bit_depth == dest_bit_depth {
        } else {
            __assert_fail(b"src_bit_depth == dest_bit_depth\x00" as *const u8
                              as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          697 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 74], &'_ [i8; 74]>(b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        return 0 as std::os::raw::c_int as png_uint_32
    }
    /* Iterate through all sample values. */
    row_ptr =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as *const png_info);
    if src_bit_depth == 8 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
            dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                dest_shift =
                    dest_shift.wrapping_sub(dest_bit_depth as std::os::raw::c_uint);
                if dest_shift > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    dest_buf |=
                        ((*src_sample_ptr as std::os::raw::c_int) << dest_shift) as
                            std::os::raw::c_uint
                } else {
                    let mut fresh0 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh0 =
                        (dest_buf | *src_sample_ptr as std::os::raw::c_uint) as
                            png_byte;
                    dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                    dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint
                }
                src_sample_ptr = src_sample_ptr.offset(1);
                j = j.wrapping_add(1)
            }
            if dest_shift != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                *dest_sample_ptr = dest_buf as png_byte
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    } else {
        /* src_bit_depth < 8 */
        src_mask_init =
            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int + src_bit_depth) -
                 ((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
            src_shift = dest_shift;
            src_mask = src_mask_init;
            dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                src_shift =
                    src_shift.wrapping_sub(src_bit_depth as std::os::raw::c_uint);
                src_mask >>= src_bit_depth;
                sample =
                    (*src_sample_ptr as std::os::raw::c_uint & src_mask) >> src_shift;
                dest_shift =
                    dest_shift.wrapping_sub(dest_bit_depth as std::os::raw::c_uint);
                if dest_shift > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    dest_buf |= sample << dest_shift
                } else {
                    let mut fresh1 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh1 = (dest_buf | sample) as png_byte;
                    dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                    dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint
                }
                if src_shift == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    src_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                    src_mask = src_mask_init;
                    src_sample_ptr = src_sample_ptr.offset(1)
                }
                j = j.wrapping_add(1)
            }
            if dest_shift != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                *dest_sample_ptr = dest_buf as png_byte
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    }
    /* Update the image information. */
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 dest_bit_depth, color_type, interlace_type, compression_type,
                 filter_type);
    return 0x2 as std::os::raw::c_int as png_uint_32;
}
/*
 * Reduce the image type from grayscale(+alpha) or RGB(+alpha) to palette,
 * if possible.
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 */
unsafe extern "C" fn opng_reduce_to_palette(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                            mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                            mut reductions: std::os::raw::c_uint)
 -> std::os::raw::c_uint {
    let mut result: u32 = 0; /* nothing is done in this case */
    let mut row_info: crate::src::libpng::png::png_row_info_struct =
        png_row_info{width: 0,
                     rowbytes: 0,
                     color_type: 0,
                     bit_depth: 0,
                     channels: 0,
                     pixel_depth: 0,}; /* not used */
    let mut row_ptr: * mut * mut u8 = 0 as *mut *mut png_byte; /* not used */
    let mut sample_ptr: * mut u8 = 0 as *mut png_byte;
    let mut alpha_row: * mut u8 = 0 as *mut png_byte;
    let mut height: u32 = 0;
    let mut width: u32 = 0;
    let mut color_type: i32 = 0;
    let mut interlace_type: i32 = 0;
    let mut compression_type: i32 = 0;
    let mut filter_type: i32 = 0;
    let mut src_bit_depth: i32 = 0;
    let mut dest_bit_depth: i32 = 0;
    let mut channels: i32 = 0;
    let mut palette: [crate::src::libpng::png::png_color_struct; 256] =
        [png_color{red: 0, green: 0, blue: 0,}; 256];
    let mut trans_alpha: [u8; 256] = [0; 256];
    let mut trans_color: Option<&'_ mut crate::src::libpng::png::png_color_16_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    let mut num_palette: i32 = 0;
    let mut num_trans: i32 = 0;
    let mut index: i32 = 0;
    let mut gray: u32 = 0;
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    let mut alpha: u32 = 0;
    let mut prev_gray: u32 = 0;
    let mut prev_red: u32 = 0;
    let mut prev_green: u32 = 0;
    let mut prev_blue: u32 = 0;
    let mut prev_alpha: u32 = 0;
    let mut background: Option<&'_ mut crate::src::libpng::png::png_color_16_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 Some(&mut width), Some(&mut height), Some(&mut src_bit_depth), Some(&mut color_type),
                 Some(&mut interlace_type), Some(&mut compression_type),
                 Some(&mut filter_type));
    if src_bit_depth != 8 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    if color_type & 1 as std::os::raw::c_int == 0 {
    } else {
        __assert_fail(b"!(color_type & 1)\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      802 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 72], &'_ [i8; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    row_ptr =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as *const png_info);
    channels =
        png_get_channels(png_ptr as *const png_struct,
                         info_ptr as *const png_info) as std::os::raw::c_int;
    alpha_row =
        png_malloc(png_ptr as *const png_struct, width as png_alloc_size_t) as
            png_bytep;
    row_info.width = width;
    row_info.rowbytes = 0 as std::os::raw::c_int as png_size_t;
    row_info.color_type = color_type as png_byte;
    row_info.bit_depth = src_bit_depth as png_byte;
    row_info.channels = channels as png_byte;
    row_info.pixel_depth = 0 as std::os::raw::c_int as png_byte;
    /* Analyze the possibility of this reduction. */
    num_trans = 0 as std::os::raw::c_int;
    num_palette = num_trans;
    trans_color = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    png_get_tRNS(png_ptr as *const png_struct, info_ptr, Option::<&'_ mut * mut u8>::None,
                 Option::<&'_ mut i32>::None, Some(&mut trans_color));
    prev_alpha = 256 as std::os::raw::c_int as std::os::raw::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    prev_gray = prev_red;
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr;
        opng_get_alpha_row(Some(&mut row_info), borrow_mut(&mut trans_color), *row_ptr, alpha_row);
        if color_type & 2 as std::os::raw::c_int != 0 {
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                red =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                green =
                    *sample_ptr.offset(1 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                blue =
                    *sample_ptr.offset(2 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if red != prev_red || green != prev_green || blue != prev_blue
                       || alpha != prev_alpha {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 Some(&mut num_palette),
                                                 trans_alpha.as_mut_ptr(),
                                                 Some(&mut num_trans),
                                                 256 as std::os::raw::c_int, red,
                                                 green, blue, alpha,
                                                 Some(&mut index)) <
                           0 as std::os::raw::c_int {
                        /* overflow */
                        if num_palette < 0 as std::os::raw::c_int {
                        } else {
                            __assert_fail(b"num_palette < 0\x00" as *const u8
                                              as *const std::os::raw::c_char,
                                          b"opngreduc.c\x00" as *const u8 as
                                              *const std::os::raw::c_char,
                                          844 as std::os::raw::c_int as std::os::raw::c_uint,
                                          (*core::intrinsics::transmute::<&'_ [u8; 72], &'_ [i8; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr()); /* forced exit from outer loop */
                        }
                        i = height;
                        break ;
                    }
                }
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        } else {
            /* grayscale */
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                gray =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 Some(&mut num_palette),
                                                 trans_alpha.as_mut_ptr(),
                                                 Some(&mut num_trans),
                                                 256 as std::os::raw::c_int, gray,
                                                 gray, gray, alpha,
                                                 Some(&mut index)) <
                           0 as std::os::raw::c_int {
                        /* overflow */
                        if num_palette < 0 as std::os::raw::c_int {
                        } else {
                            __assert_fail(b"num_palette < 0\x00" as *const u8
                                              as *const std::os::raw::c_char,
                                          b"opngreduc.c\x00" as *const u8 as
                                              *const std::os::raw::c_char,
                                          866 as std::os::raw::c_int as std::os::raw::c_uint,
                                          (*core::intrinsics::transmute::<&'_ [u8; 72], &'_ [i8; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr()); /* forced exit from outer loop */
                        }
                        i = height;
                        break ;
                    }
                }
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    if num_palette >= 0 as std::os::raw::c_int &&
           png_get_bKGD(png_ptr as *const png_struct, info_ptr,
                        Some(&mut background)) != 0 {
        /* bKGD has an alpha-agnostic palette entry. */
        if color_type & 2 as std::os::raw::c_int != 0 {
            red = (*(borrow_mut(&mut background)).unwrap()).red as std::os::raw::c_uint;
            green = (*(borrow_mut(&mut background)).unwrap()).green as std::os::raw::c_uint;
            blue = (*(borrow_mut(&mut background)).unwrap()).blue as std::os::raw::c_uint
        } else {
            blue = (*(borrow_mut(&mut background)).unwrap()).gray as std::os::raw::c_uint;
            green = blue;
            red = green
        }
        opng_insert_palette_entry(palette.as_mut_ptr(), Some(&mut num_palette),
                                  trans_alpha.as_mut_ptr(), Some(&mut num_trans),
                                  256 as std::os::raw::c_int, red, green, blue,
                                  256 as std::os::raw::c_int as std::os::raw::c_uint,
                                  Some(&mut index));
        if index >= 0 as std::os::raw::c_int {
            (*(borrow_mut(&mut background)).unwrap()).index = index as png_byte
        }
    }
    /* Continue only if the uncompressed indexed image (pixels + PLTE + tRNS)
    * is smaller than the uncompressed RGB(A) image.
    * Casual overhead (headers, CRCs, etc.) is ignored.
    *
    * Compare:
    * num_pixels * (src_bit_depth * channels - dest_bit_depth) / 8
    * vs.
    * sizeof(PLTE) + sizeof(tRNS)
    */
    if num_palette >= 0 as std::os::raw::c_int {
        if num_palette > 0 as std::os::raw::c_int && num_palette <= 256 as std::os::raw::c_int
           {
        } else {
            __assert_fail(b"num_palette > 0 && num_palette <= 256\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          905 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 72], &'_ [i8; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        if num_trans >= 0 as std::os::raw::c_int && num_trans <= num_palette {
        } else {
            __assert_fail(b"num_trans >= 0 && num_trans <= num_palette\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          906 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 72], &'_ [i8; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        if num_palette <= 2 as std::os::raw::c_int {
            dest_bit_depth = 1 as std::os::raw::c_int
        } else if num_palette <= 4 as std::os::raw::c_int {
            dest_bit_depth = 2 as std::os::raw::c_int
        } else if num_palette <= 16 as std::os::raw::c_int {
            dest_bit_depth = 4 as std::os::raw::c_int
        } else { dest_bit_depth = 8 as std::os::raw::c_int }
        /* Do the comparison in a way that does not cause overflow. */
        if channels * 8 as std::os::raw::c_int == dest_bit_depth ||
               (((3 as std::os::raw::c_int * num_palette + num_trans) *
                     8 as std::os::raw::c_int /
                     (channels * 8 as std::os::raw::c_int - dest_bit_depth)) as
                    std::os::raw::c_uint).wrapping_div(width).wrapping_div(height) >=
                   1 as std::os::raw::c_int as std::os::raw::c_uint {
            num_palette = -(1 as std::os::raw::c_int)
        }
    }
    if num_palette < 0 as std::os::raw::c_int {
        /* can't reduce */
        png_free(png_ptr as *const png_struct, alpha_row as png_voidp);
        return 0 as std::os::raw::c_int as png_uint_32
    }
    /* Reduce. */
    row_ptr =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as *const png_info);
    index = -(1 as std::os::raw::c_int);
    prev_alpha = -(1 as std::os::raw::c_int) as std::os::raw::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr;
        opng_get_alpha_row(Some(&mut row_info), borrow_mut(&mut trans_color), *row_ptr, alpha_row);
        if color_type & 2 as std::os::raw::c_int != 0 {
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                red =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                green =
                    *sample_ptr.offset(1 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                blue =
                    *sample_ptr.offset(2 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if red != prev_red || green != prev_green || blue != prev_blue
                       || alpha != prev_alpha {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 Some(&mut num_palette),
                                                 trans_alpha.as_mut_ptr(),
                                                 Some(&mut num_trans),
                                                 256 as std::os::raw::c_int, red,
                                                 green, blue, alpha,
                                                 Some(&mut index)) !=
                           0 as std::os::raw::c_int {
                        index = -(1 as std::os::raw::c_int)
                    }
                    /* this should not happen */
                }
                if index >= 0 as std::os::raw::c_int {
                } else {
                    __assert_fail(b"index >= 0\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"opngreduc.c\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  957 as std::os::raw::c_int as std::os::raw::c_uint,
                                  (*core::intrinsics::transmute::<&'_ [u8; 72], &'_ [i8; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
                }
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        } else {
            /* grayscale */
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                gray =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 Some(&mut num_palette),
                                                 trans_alpha.as_mut_ptr(),
                                                 Some(&mut num_trans),
                                                 256 as std::os::raw::c_int, gray,
                                                 gray, gray, alpha,
                                                 Some(&mut index)) !=
                           0 as std::os::raw::c_int {
                        index = -(1 as std::os::raw::c_int)
                    }
                    /* this should not happen */
                }
                if index >= 0 as std::os::raw::c_int {
                } else {
                    __assert_fail(b"index >= 0\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"opngreduc.c\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  977 as std::os::raw::c_int as std::os::raw::c_uint,
                                  (*core::intrinsics::transmute::<&'_ [u8; 72], &'_ [i8; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
                }
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    /* Update the image information. */
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 8 as std::os::raw::c_int, 2 as std::os::raw::c_int | 1 as std::os::raw::c_int,
                 interlace_type, compression_type, filter_type);
    png_set_PLTE(png_ptr, info_ptr, palette.as_mut_ptr() as png_const_colorp,
                 num_palette);
    if num_trans > 0 as std::os::raw::c_int {
        png_set_tRNS(png_ptr, info_ptr,
                     trans_alpha.as_mut_ptr() as png_const_bytep, num_trans,
                     Option::<&'_ crate::src::libpng::png::png_color_16_struct>::None);
    }
    /* bKGD (if present) is automatically updated. */
    png_free(png_ptr as *const png_struct, alpha_row as png_voidp);
    result = 0x10 as std::os::raw::c_int as png_uint_32;
    if reductions & 0x2 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions)
    }
    return result;
}
/*
 * Analyze the usage of samples.
 * The output value usage_map[n] indicates whether the sample n
 * is used. The usage_map[] array must have 256 entries.
 * The function requires a valid bit depth between 1 and 8.
 */
unsafe extern "C" fn opng_analyze_sample_usage(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                               mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                               mut usage_map: * mut std::os::raw::c_uchar) {
    let mut row_ptr: * mut * mut u8 = 0 as *mut *mut png_byte;
    let mut sample_ptr: * mut u8 = 0 as *mut png_byte;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bit_depth: i32 = 0;
    let mut init_shift: i32 = 0;
    let mut init_mask: i32 = 0;
    let mut shift: i32 = 0;
    let mut mask: i32 = 0;
    let mut background: Option<&'_ mut crate::src::libpng::png::png_color_16_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    height =
        png_get_image_height(png_ptr as *const png_struct,
                             info_ptr as *const png_info);
    width =
        png_get_image_width(png_ptr as *const png_struct,
                            info_ptr as *const png_info);
    bit_depth =
        png_get_bit_depth(png_ptr as *const png_struct,
                          info_ptr as *const png_info) as std::os::raw::c_int;
    row_ptr =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as *const png_info);
    /* Initialize the output entries with 0. */
    memset(usage_map as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           256 as std::os::raw::c_int as std::os::raw::c_ulong);
    /* Iterate through all sample values. */
    if bit_depth == 8 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            j = 0 as std::os::raw::c_int as png_uint_32;
            sample_ptr = *row_ptr;
            while j < width {
                *usage_map.offset(*sample_ptr as isize) =
                    1 as std::os::raw::c_int as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(1)
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    } else {
        if bit_depth < 8 as std::os::raw::c_int {
        } else {
            __assert_fail(b"bit_depth < 8\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          1039 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 66], &'_ [i8; 66]>(b"void opng_analyze_sample_usage(png_structp, png_infop, png_bytep)\x00")).as_ptr());
        }
        init_shift = 8 as std::os::raw::c_int - bit_depth;
        init_mask =
            ((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                ((1 as std::os::raw::c_int) << init_shift);
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            j = 0 as std::os::raw::c_int as png_uint_32;
            sample_ptr = *row_ptr;
            while j < width {
                mask = init_mask;
                shift = init_shift;
                loop  {
                    *usage_map.offset(((*sample_ptr as std::os::raw::c_int & mask) >>
                                           shift) as isize) =
                        1 as std::os::raw::c_int as png_byte;
                    mask >>= bit_depth;
                    shift -= bit_depth;
                    j = j.wrapping_add(1);
                    if !(mask > 0 as std::os::raw::c_int && j < width) { break ; }
                }
                sample_ptr = sample_ptr.offset(1)
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    }
    /* bKGD also counts as a used sample. */
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, Some(&mut background))
           != 0 {
        *usage_map.offset((*(borrow(& background)).unwrap()).index as isize) =
            1 as std::os::raw::c_int as png_byte
    };
}
/*
 * Reduce the palette. (Only the fast method is implemented.)
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 */
unsafe extern "C" fn opng_reduce_palette(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                         mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                         mut reductions: std::os::raw::c_uint)
 -> std::os::raw::c_uint {
    let mut result: u32 = 0;
    let mut palette: * mut crate::src::libpng::png::png_color_struct = 0 as *mut png_color;
    let mut trans_alpha: * mut u8 = 0 as *mut png_byte;
    let mut row_ptr: * mut * mut u8 = 0 as *mut *mut png_byte;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut interlace_type: i32 = 0;
    let mut compression_type: i32 = 0;
    let mut filter_type: i32 = 0;
    let mut num_palette: i32 = 0;
    let mut num_trans: i32 = 0;
    let mut last_color_index: i32 = 0;
    let mut last_trans_index: i32 = 0;
    let mut crt_trans_value: u8 = 0;
    let mut last_trans_value: u8 = 0;
    let mut is_used: [u8; 256] = [0; 256];
    let mut gray_trans: crate::src::libpng::png::png_color_16_struct =
        png_color_16{index: 0, red: 0, green: 0, blue: 0, gray: 0,};
    let mut is_gray: i32 = 0;
    let mut background: Option<&'_ mut crate::src::libpng::png::png_color_16_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None;
    let mut hist: * mut u16 = 0 as *mut png_uint_16;
    let mut sig_bits: Option<&'_ mut crate::src::libpng::png::png_color_8_struct> = Option::<&'_ mut crate::src::libpng::png::png_color_8_struct>::None;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut k: i32 = 0;
    result = 0 as std::os::raw::c_int as png_uint_32;
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 Some(&mut width), Some(&mut height), Some(&mut bit_depth), Some(&mut color_type),
                 Some(&mut interlace_type), Some(&mut compression_type),
                 Some(&mut filter_type));
    row_ptr =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as *const png_info);
    if png_get_PLTE(png_ptr as *const png_struct, info_ptr, Some(&mut palette),
                    Some(&mut num_palette)) == 0 {
        palette = 0 as png_colorp;
        num_palette = 0 as std::os::raw::c_int
    }
    if png_get_tRNS(png_ptr as *const png_struct, info_ptr, Some(&mut trans_alpha),
                    Some(&mut num_trans), Option::<&'_ mut Option<&'_ mut crate::src::libpng::png::png_color_16_struct>>::None) == 0 {
        trans_alpha = 0 as png_bytep;
        num_trans = 0 as std::os::raw::c_int
    } else if !trans_alpha.is_null() && num_trans > 0 as std::os::raw::c_int {
    } else {
        __assert_fail(b"trans_alpha != ((void*)0) && num_trans > 0\x00" as
                          *const u8 as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      1117 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    opng_analyze_sample_usage(png_ptr, info_ptr, is_used.as_mut_ptr());
    /* Palette-to-gray does not work (yet) if the bit depth is below 8. */
    is_gray =
        (reductions & 0x80 as std::os::raw::c_int as std::os::raw::c_uint != 0 &&
             bit_depth == 8 as std::os::raw::c_int) as std::os::raw::c_int;
    last_trans_index = -(1 as std::os::raw::c_int);
    last_color_index = last_trans_index;
    k = 0 as std::os::raw::c_int;
    while k < 256 as std::os::raw::c_int {
        if !(is_used[k as usize] == 0) {
            last_color_index = k;
            if k < num_trans &&
                   (*trans_alpha.offset(k as isize) as std::os::raw::c_int) <
                       255 as std::os::raw::c_int {
                last_trans_index = k
            }
            if is_gray != 0 {
                if (*palette.offset(k as isize)).red as std::os::raw::c_int !=
                       (*palette.offset(k as isize)).green as std::os::raw::c_int ||
                       (*palette.offset(k as isize)).red as std::os::raw::c_int !=
                           (*palette.offset(k as isize)).blue as std::os::raw::c_int {
                    is_gray = 0 as std::os::raw::c_int
                }
            }
        }
        k += 1
    }
    if last_color_index >= 0 as std::os::raw::c_int {
    } else {
        __assert_fail(b"last_color_index >= 0\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      1135 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    if last_color_index >= last_trans_index {
    } else {
        __assert_fail(b"last_color_index >= last_trans_index\x00" as *const u8
                          as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      1136 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    /* Check the integrity of PLTE and tRNS. */
    if last_color_index >= num_palette {
        png_warning(png_ptr as *const png_struct,
                    b"Too few colors in PLTE\x00" as *const u8 as
                        *const std::os::raw::c_char);
        /* Fix the palette by adding blank entries at the end. */
        opng_realloc_PLTE(png_ptr, info_ptr,
                          last_color_index + 1 as std::os::raw::c_int);
        png_get_PLTE(png_ptr as *const png_struct, info_ptr, Some(&mut palette),
                     Some(&mut num_palette));
        if num_palette == last_color_index + 1 as std::os::raw::c_int {
        } else {
            __assert_fail(b"num_palette == last_color_index + 1\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          1145 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        result |= 0x2000 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if num_trans > num_palette {
        png_warning(png_ptr as *const png_struct,
                    b"Too many alpha values in tRNS\x00" as *const u8 as
                        *const std::os::raw::c_char);
        /* Transparency will be fixed further below. */
        result |= 0x2000 as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* Check if tRNS can be reduced to grayscale. */
    if is_gray != 0 && last_trans_index >= 0 as std::os::raw::c_int {
        gray_trans.gray =
            (*palette.offset(last_trans_index as isize)).red as png_uint_16;
        last_trans_value = *trans_alpha.offset(last_trans_index as isize);
        k = 0 as std::os::raw::c_int;
        while k <= last_color_index {
            if !(is_used[k as usize] == 0) {
                if k <= last_trans_index {
                    crt_trans_value = *trans_alpha.offset(k as isize);
                    /* Cannot reduce if different colors have transparency. */
                    if (crt_trans_value as std::os::raw::c_int) < 255 as std::os::raw::c_int
                           &&
                           (*palette.offset(k as isize)).red as std::os::raw::c_int !=
                               gray_trans.gray as std::os::raw::c_int {
                        is_gray = 0 as std::os::raw::c_int;
                        break ;
                    }
                } else { crt_trans_value = 255 as std::os::raw::c_int as png_byte }
                /* Cannot reduce if same color has multiple transparency levels. */
                if (*palette.offset(k as isize)).red as std::os::raw::c_int ==
                       gray_trans.gray as std::os::raw::c_int &&
                       crt_trans_value as std::os::raw::c_int !=
                           last_trans_value as std::os::raw::c_int {
                    is_gray = 0 as std::os::raw::c_int;
                    break ;
                }
            }
            k += 1
        }
    }
    /* Remove tRNS if it is entirely sterile. */
    if num_trans > 0 as std::os::raw::c_int && last_trans_index < 0 as std::os::raw::c_int {
        num_trans = 0 as std::os::raw::c_int;
        png_free_data(png_ptr as *const png_struct, info_ptr,
                      0x2000 as std::os::raw::c_uint, -(1 as std::os::raw::c_int));
        png_set_invalid(png_ptr as *const png_struct, info_ptr,
                        0x10 as std::os::raw::c_uint as std::os::raw::c_int);
        result |= 0x200 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if reductions & 0x200 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if num_palette != last_color_index + 1 as std::os::raw::c_int {
            /* Reduce PLTE. */
         /* hIST is reduced automatically. */
            opng_realloc_PLTE(png_ptr, info_ptr,
                              last_color_index + 1 as std::os::raw::c_int);
            png_get_PLTE(png_ptr as *const png_struct, info_ptr, Some(&mut palette),
                         Some(&mut num_palette));
            if num_palette == last_color_index + 1 as std::os::raw::c_int {
            } else {
                __assert_fail(b"num_palette == last_color_index + 1\x00" as
                                  *const u8 as *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              1203 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
            }
            result |= 0x200 as std::os::raw::c_int as std::os::raw::c_uint
        }
        if num_trans > 0 as std::os::raw::c_int &&
               num_trans != last_trans_index + 1 as std::os::raw::c_int {
            /* Reduce tRNS. */
            opng_realloc_tRNS(png_ptr, info_ptr,
                              last_trans_index + 1 as std::os::raw::c_int);
            png_get_tRNS(png_ptr as *const png_struct, info_ptr,
                         Some(&mut trans_alpha), Some(&mut num_trans),
                         Option::<&'_ mut Option<&'_ mut crate::src::libpng::png::png_color_16_struct>>::None);
            if num_trans == last_trans_index + 1 as std::os::raw::c_int {
            } else {
                __assert_fail(b"num_trans == last_trans_index + 1\x00" as
                                  *const u8 as *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              1212 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
            }
            result |= 0x200 as std::os::raw::c_int as std::os::raw::c_uint
        }
    }
    if reductions & 0x2 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions);
        /* Refresh the image information. */
        bit_depth =
            png_get_bit_depth(png_ptr as *const png_struct,
                              info_ptr as *const png_info) as std::os::raw::c_int
    }
    if bit_depth < 8 as std::os::raw::c_int || is_gray == 0 { return result }
    /* Reduce palette --> grayscale. */
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        j = 0 as std::os::raw::c_int as png_uint_32;
        while j < width {
            *(*row_ptr.offset(i as isize)).offset(j as isize) =
                (*palette.offset(*(*row_ptr.offset(i as
                                                       isize)).offset(j as
                                                                          isize)
                                     as isize)).red;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    /* Update the ancillary information. */
    if num_trans > 0 as std::os::raw::c_int {
        png_set_tRNS(png_ptr, info_ptr, 0 as png_const_bytep,
                     0 as std::os::raw::c_int,
                     Some(&mut gray_trans));
    }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, Some(&mut background))
           != 0 {
        (*(borrow_mut(&mut background)).unwrap()).gray =
            (*palette.offset((*(borrow(& background)).unwrap()).index as isize)).red as png_uint_16
    }
    if png_get_hIST(png_ptr as *const png_struct, info_ptr, Some(&mut hist)) != 0 {
        png_free_data(png_ptr as *const png_struct, info_ptr,
                      0x8 as std::os::raw::c_uint, -(1 as std::os::raw::c_int));
        png_set_invalid(png_ptr as *const png_struct, info_ptr,
                        0x40 as std::os::raw::c_uint as std::os::raw::c_int);
    }
    if png_get_sBIT(png_ptr as *const png_struct, info_ptr, Some(&mut sig_bits)) !=
           0 {
        let mut max_sig_bits: u8 = (*(borrow_mut(&mut sig_bits)).unwrap()).red;
        if (max_sig_bits as std::os::raw::c_int) < (*(borrow(& sig_bits)).unwrap()).green as std::os::raw::c_int {
            max_sig_bits = (*(borrow_mut(&mut sig_bits)).unwrap()).green
        }
        if (max_sig_bits as std::os::raw::c_int) < (*(borrow(& sig_bits)).unwrap()).blue as std::os::raw::c_int {
            max_sig_bits = (*(borrow_mut(&mut sig_bits)).unwrap()).blue
        }
        (*(borrow_mut(&mut sig_bits)).unwrap()).gray = max_sig_bits
    }
    /* Update the image information. */
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, 0 as std::os::raw::c_int, interlace_type,
                 compression_type, filter_type);
    png_free_data(png_ptr as *const png_struct, info_ptr,
                  0x1000 as std::os::raw::c_uint, -(1 as std::os::raw::c_int));
    png_set_invalid(png_ptr as *const png_struct, info_ptr,
                    0x8 as std::os::raw::c_uint as std::os::raw::c_int);
    return 0x80 as std::os::raw::c_int as png_uint_32;
    /* ignore the former result */
}
/*
 * Reduce the image (bit depth + color type + palette) without
 * losing any information. The palette (if applicable) and the
 * image data must be present, e.g., by calling png_set_rows(),
 * or by loading IDAT.
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_reduce_image(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                           mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                           mut reductions: std::os::raw::c_uint)
 -> std::os::raw::c_uint {
    let mut result: u32 = 0;
    let mut color_type: i32 = 0;
    if opng_validate_image(png_ptr, info_ptr) == 0 {
        png_warning(png_ptr as *const png_struct,
                    b"Image reduction requires the presence of all critical information\x00"
                        as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int as png_uint_32
    }
    color_type =
        png_get_color_type(png_ptr as *const png_struct,
                           info_ptr as *const png_info) as std::os::raw::c_int;
    /* The reductions below must be applied in this particular order. */
    /* Try to reduce the high bits and color/alpha channels. */
    result = opng_reduce_bits(png_ptr, info_ptr, reductions);
    /* Try to reduce the palette image. */
    if color_type == 2 as std::os::raw::c_int | 1 as std::os::raw::c_int &&
           reductions &
               (0x80 as std::os::raw::c_int | 0x200 as std::os::raw::c_int |
                    0x2 as std::os::raw::c_int) as std::os::raw::c_uint != 0 {
        result |= opng_reduce_palette(png_ptr, info_ptr, reductions)
    }
    /* Try to reduce RGB to palette or grayscale to palette. */
    if color_type & !(4 as std::os::raw::c_int) == 0 as std::os::raw::c_int &&
           reductions & 0x40 as std::os::raw::c_int as std::os::raw::c_uint != 0 ||
           color_type & !(4 as std::os::raw::c_int) == 2 as std::os::raw::c_int &&
               reductions & 0x10 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if result & 0x80 as std::os::raw::c_int as std::os::raw::c_uint == 0 {
            result |= opng_reduce_to_palette(png_ptr, info_ptr, reductions)
        }
    }
    return result;
}
use crate::laertes_rt::*;
/* OPNG_IMAGE_REDUCTIONS_SUPPORTED */

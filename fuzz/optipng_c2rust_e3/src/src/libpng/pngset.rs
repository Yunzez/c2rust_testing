
extern "C" {
    
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcmp(_: *const std::os::raw::c_void, _: *const std::os::raw::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::libpng::png::png_check_IHDR;
pub use crate::src::libpng::png::png_free_data;
pub use crate::src::libpng::pngerror::png_app_error;
pub use crate::src::libpng::pngerror::png_app_warning;
pub use crate::src::libpng::pngerror::png_chunk_report;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngmem::png_calloc;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngmem::png_malloc_base;
pub use crate::src::libpng::pngmem::png_malloc_warn;
pub use crate::src::libpng::pngmem::png_realloc_array;
pub use crate::src::libpng::pngwutil::png_free_buffer_list;
pub use crate::src::zlib::deflate::internal_state;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type png_byte = crate::src::libpng::png::png_byte;
pub type png_uint_16 = crate::src::libpng::png::png_uint_16;
pub type png_int_32 = crate::src::libpng::png::png_int_32;
pub type png_uint_32 = crate::src::libpng::png::png_uint_32;
pub type png_size_t = crate::src::libpng::png::png_size_t;
pub type png_alloc_size_t = crate::src::libpng::png::png_alloc_size_t;
pub type png_fixed_point = crate::src::libpng::png::png_fixed_point;
pub type png_voidp = crate::src::libpng::png::png_voidp;
pub type png_const_voidp = crate::src::libpng::pngmem::png_const_voidp;
pub type png_bytep = crate::src::libpng::png::png_bytep;
pub type png_const_bytep = crate::src::libpng::png::png_const_bytep;
pub type png_uint_16p = crate::src::libpng::png::png_uint_16p;
pub type png_const_uint_16p = crate::src::libpng::pngrutil::png_const_uint_16p;
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
pub type png_bytepp = crate::src::libpng::png::png_bytepp;
// #[derive(Copy, Clone)]

pub type png_struct_def = crate::src::libpng::png::png_struct_def;
pub type png_row_infop = crate::src::libpng::png::png_row_infop;
pub type png_row_info = crate::src::libpng::png::png_row_info;
// #[derive(Copy, Clone)]

pub type png_row_info_struct = crate::src::libpng::png::png_row_info_struct;
pub type uInt = crate::src::libpng::png::uInt;
pub type png_unknown_chunk = crate::src::libpng::png::png_unknown_chunk;
// #[derive(Copy, Clone)]

pub type png_unknown_chunk_t = crate::src::libpng::png::png_unknown_chunk_t;
pub type png_write_status_ptr = crate::src::libpng::png::png_write_status_ptr;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_read_status_ptr = crate::src::libpng::png::png_read_status_ptr;
pub type png_color_16 = crate::src::libpng::png::png_color_16;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::png::png_color_16_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::png::png_color_8_struct;
pub type png_flush_ptr = crate::src::libpng::png::png_flush_ptr;
pub type png_colorp = crate::src::libpng::png::png_colorp;
pub type png_color = crate::src::libpng::png::png_color;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::png::png_color_struct;
pub type png_compression_bufferp = crate::src::libpng::png::png_compression_bufferp;
// #[derive(Copy, Clone)]

pub type png_compression_buffer = crate::src::libpng::png::png_compression_buffer;
pub type z_stream = crate::src::libpng::png::z_stream;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::png::z_stream_s;
pub type uLong = crate::src::libpng::png::uLong;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type free_func = crate::src::libpng::png::free_func;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type Byte = crate::src::libpng::png::Byte;
pub type png_rw_ptr = crate::src::libpng::png::png_rw_ptr;
pub type png_error_ptr = crate::src::libpng::png::png_error_ptr;
// #[derive(Copy, Clone)]

pub type png_info_def = crate::src::libpng::png::png_info_def;
pub type png_unknown_chunkp = crate::src::libpng::png::png_unknown_chunkp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_structrp = crate::src::libpng::png::png_structrp;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
pub type png_const_colorp = crate::src::libpng::pngrutil::png_const_colorp;
pub type png_const_color_16p = crate::src::libpng::pngrutil::png_const_color_16p;
pub type png_const_color_8p = crate::src::libpng::pngrutil::png_const_color_8p;
pub type png_const_unknown_chunkp = crate::src::libpng::pngrutil::png_const_unknown_chunkp;
/* pngset.c - storage of image information into info struct
 *
 * Last changed in libpng 1.6.32 [August 24, 2017]
 * Copyright (c) 1998-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * The functions here are used during reads to store data from the file
 * into the info struct, and during writes to store application data
 * into the info struct for writing into the file.  This abstracts the
 * info struct and allows us to change the structure in the future.
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_bKGD(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut background: png_const_color_16p) {
    if png_ptr.is_null() || info_ptr.is_null() || background.is_null() {
        return
    }
    (*info_ptr).background = *background;
    (*info_ptr).valid |= 0x20 as std::os::raw::c_uint;
}
/* cHRM */
/* eXIf */
#[no_mangle]
pub unsafe extern "C" fn png_set_hIST(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut hist: png_const_uint_16p) {
    let mut i: std::os::raw::c_int = 0;
    if png_ptr.is_null() || info_ptr.is_null() { return }
    if (*info_ptr).num_palette as std::os::raw::c_int == 0 as std::os::raw::c_int ||
           (*info_ptr).num_palette as std::os::raw::c_int > 256 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid palette size, hIST allocation skipped\x00" as
                        *const u8 as *const std::os::raw::c_char);
        return
    }
    png_free_data(png_ptr, info_ptr, 0x8 as std::os::raw::c_uint, 0 as std::os::raw::c_int);
    /* Changed from info->num_palette to PNG_MAX_PALETTE_LENGTH in
    * version 1.2.1
    */
    (*info_ptr).hist =
        png_malloc_warn(png_ptr,
                        (256 as std::os::raw::c_int as
                             std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_uint_16>()
                                                             as
                                                             std::os::raw::c_ulong))
            as png_uint_16p;
    if (*info_ptr).hist.is_null() {
        png_warning(png_ptr,
                    b"Insufficient memory for hIST chunk data\x00" as
                        *const u8 as *const std::os::raw::c_char);
        return
    }
    (*info_ptr).free_me |= 0x8 as std::os::raw::c_uint;
    i = 0 as std::os::raw::c_int;
    while i < (*info_ptr).num_palette as std::os::raw::c_int {
        *(*info_ptr).hist.offset(i as isize) = *hist.offset(i as isize);
        i += 1
    }
    (*info_ptr).valid |= 0x40 as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn png_set_IHDR(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut width: png_uint_32,
                                      mut height: png_uint_32,
                                      mut bit_depth: std::os::raw::c_int,
                                      mut color_type: std::os::raw::c_int,
                                      mut interlace_type: std::os::raw::c_int,
                                      mut compression_type: std::os::raw::c_int,
                                      mut filter_type: std::os::raw::c_int) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    (*info_ptr).width = width;
    (*info_ptr).height = height;
    (*info_ptr).bit_depth = bit_depth as png_byte;
    (*info_ptr).color_type = color_type as png_byte;
    (*info_ptr).compression_type = compression_type as png_byte;
    (*info_ptr).filter_type = filter_type as png_byte;
    (*info_ptr).interlace_type = interlace_type as png_byte;
    png_check_IHDR(png_ptr, (*info_ptr).width, (*info_ptr).height,
                   (*info_ptr).bit_depth as std::os::raw::c_int,
                   (*info_ptr).color_type as std::os::raw::c_int,
                   (*info_ptr).interlace_type as std::os::raw::c_int,
                   (*info_ptr).compression_type as std::os::raw::c_int,
                   (*info_ptr).filter_type as std::os::raw::c_int);
    if (*info_ptr).color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        (*info_ptr).channels = 1 as std::os::raw::c_int as png_byte
    } else if (*info_ptr).color_type as std::os::raw::c_int & 2 as std::os::raw::c_int !=
                  0 as std::os::raw::c_int {
        (*info_ptr).channels = 3 as std::os::raw::c_int as png_byte
    } else { (*info_ptr).channels = 1 as std::os::raw::c_int as png_byte }
    if (*info_ptr).color_type as std::os::raw::c_int & 4 as std::os::raw::c_int !=
           0 as std::os::raw::c_int {
        (*info_ptr).channels = (*info_ptr).channels.wrapping_add(1)
    }
    (*info_ptr).pixel_depth =
        ((*info_ptr).channels as std::os::raw::c_int *
             (*info_ptr).bit_depth as std::os::raw::c_int) as png_byte;
    (*info_ptr).rowbytes =
        if (*info_ptr).pixel_depth as std::os::raw::c_int >= 8 as std::os::raw::c_int {
            (width as
                 png_size_t).wrapping_mul((*info_ptr).pixel_depth as
                                              png_size_t >> 3 as std::os::raw::c_int)
        } else {
            ((width as
                  png_size_t).wrapping_mul((*info_ptr).pixel_depth as
                                               png_size_t).wrapping_add(7 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong))
                >> 3 as std::os::raw::c_int
        };
}
#[no_mangle]
pub unsafe extern "C" fn png_set_PLTE(mut png_ptr: png_structrp,
                                      mut info_ptr: png_inforp,
                                      mut palette: png_const_colorp,
                                      mut num_palette: std::os::raw::c_int) {
    let mut max_palette_length: png_uint_32 = 0;
    if png_ptr.is_null() || info_ptr.is_null() { return }
    max_palette_length =
        if (*info_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
            ((1 as std::os::raw::c_int)) << (*info_ptr).bit_depth as std::os::raw::c_int
        } else { 256 as std::os::raw::c_int } as png_uint_32;
    if num_palette < 0 as std::os::raw::c_int ||
           num_palette > max_palette_length as std::os::raw::c_int {
        if (*info_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
            png_error(png_ptr,
                      b"Invalid palette length\x00" as *const u8 as
                          *const std::os::raw::c_char);
        } else {
            png_warning(png_ptr,
                        b"Invalid palette length\x00" as *const u8 as
                            *const std::os::raw::c_char);
            return
        }
    }
    if num_palette > 0 as std::os::raw::c_int && palette.is_null() ||
           num_palette == 0 as std::os::raw::c_int {
        png_error(png_ptr,
                  b"Invalid palette\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /* It may not actually be necessary to set png_ptr->palette here;
    * we do it for backward compatibility with the way the png_handle_tRNS
    * function used to do the allocation.
    *
    * 1.6.0: the above statement appears to be incorrect; something has to set
    * the palette inside png_struct on read.
    */
    png_free_data(png_ptr, info_ptr, 0x1000 as std::os::raw::c_uint,
                  0 as std::os::raw::c_int);
    /* Changed in libpng-1.2.1 to allocate PNG_MAX_PALETTE_LENGTH instead
    * of num_palette entries, in case of an invalid PNG file or incorrect
    * call to png_set_PLTE() with too-large sample values.
    */
    (*png_ptr).palette =
        png_calloc(png_ptr,
                   (256 as std::os::raw::c_int as
                        std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_color>()
                                                        as std::os::raw::c_ulong)) as
            png_colorp;
    if num_palette > 0 as std::os::raw::c_int {
        memcpy((*png_ptr).palette as *mut std::os::raw::c_void,
               palette as *const std::os::raw::c_void,
               (num_palette as std::os::raw::c_uint as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_color>()
                                                    as std::os::raw::c_ulong));
    }
    (*info_ptr).palette = (*png_ptr).palette;
    (*png_ptr).num_palette = num_palette as png_uint_16;
    (*info_ptr).num_palette = (*png_ptr).num_palette;
    (*info_ptr).free_me |= 0x1000 as std::os::raw::c_uint;
    (*info_ptr).valid |= 0x8 as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn png_set_sBIT(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut sig_bit: png_const_color_8p) {
    if png_ptr.is_null() || info_ptr.is_null() || sig_bit.is_null() { return }
    (*info_ptr).sig_bit = *sig_bit;
    (*info_ptr).valid |= 0x2 as std::os::raw::c_uint;
}
/* sRGB */
#[no_mangle]
pub unsafe extern "C" fn png_set_tRNS(mut png_ptr: png_structrp,
                                      mut info_ptr: png_inforp,
                                      mut trans_alpha: png_const_bytep,
                                      mut num_trans: std::os::raw::c_int,
                                      mut trans_color: png_const_color_16p) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    if !trans_alpha.is_null() {
        /* It may not actually be necessary to set png_ptr->trans_alpha here;
        * we do it for backward compatibility with the way the png_handle_tRNS
        * function used to do the allocation.
        *
        * 1.6.0: The above statement is incorrect; png_handle_tRNS effectively
        * relies on png_set_tRNS storing the information in png_struct
        * (otherwise it won't be there for the code in pngrtran.c).
        */
        png_free_data(png_ptr, info_ptr, 0x2000 as std::os::raw::c_uint,
                      0 as std::os::raw::c_int);
        if num_trans > 0 as std::os::raw::c_int && num_trans <= 256 as std::os::raw::c_int {
            /* Changed from num_trans to PNG_MAX_PALETTE_LENGTH in version 1.2.1 */
            (*info_ptr).trans_alpha =
                png_malloc(png_ptr, 256 as std::os::raw::c_int as png_alloc_size_t) as
                    png_bytep;
            memcpy((*info_ptr).trans_alpha as *mut std::os::raw::c_void,
                   trans_alpha as *const std::os::raw::c_void,
                   num_trans as png_size_t);
        }
        (*png_ptr).trans_alpha = (*info_ptr).trans_alpha
    }
    if !trans_color.is_null() {
        if ((*info_ptr).bit_depth as std::os::raw::c_int) < 16 as std::os::raw::c_int {
            let mut sample_max: std::os::raw::c_int =
                ((1 as std::os::raw::c_int) << (*info_ptr).bit_depth as std::os::raw::c_int) -
                    1 as std::os::raw::c_int;
            if (*info_ptr).color_type as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                   (*trans_color).gray as std::os::raw::c_int > sample_max ||
                   (*info_ptr).color_type as std::os::raw::c_int == 2 as std::os::raw::c_int
                       &&
                       ((*trans_color).red as std::os::raw::c_int > sample_max ||
                            (*trans_color).green as std::os::raw::c_int > sample_max
                            ||
                            (*trans_color).blue as std::os::raw::c_int > sample_max) {
                png_warning(png_ptr,
                            b"tRNS chunk has out-of-range samples for bit_depth\x00"
                                as *const u8 as *const std::os::raw::c_char);
            }
        }
        (*info_ptr).trans_color = *trans_color;
        if num_trans == 0 as std::os::raw::c_int { num_trans = 1 as std::os::raw::c_int }
    }
    (*info_ptr).num_trans = num_trans as png_uint_16;
    if num_trans != 0 as std::os::raw::c_int {
        (*info_ptr).valid |= 0x10 as std::os::raw::c_uint;
        (*info_ptr).free_me |= 0x2000 as std::os::raw::c_uint
    };
}
/* sPLT */
unsafe extern "C" fn check_location(mut png_ptr: png_const_structrp,
                                    mut location: std::os::raw::c_int) -> png_byte {
    location &= 0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int | 0x8 as std::os::raw::c_int;
    /* New in 1.6.0; copy the location and check it.  This is an API
    * change; previously the app had to use the
    * png_set_unknown_chunk_location API below for each chunk.
    */
    if location == 0 as std::os::raw::c_int &&
           (*png_ptr).mode & 0x8000 as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Write struct, so unknown chunks come from the app */
        png_app_warning(png_ptr,
                        b"png_set_unknown_chunks now expects a valid location\x00"
                            as *const u8 as *const std::os::raw::c_char);
        /* Use the old behavior */
        location =
            ((*png_ptr).mode &
                 (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int |
                      0x8 as std::os::raw::c_int) as std::os::raw::c_uint) as png_byte as
                std::os::raw::c_int
    }
    /* This need not be an internal error - if the app calls
    * png_set_unknown_chunks on a read pointer it must get the location right.
    */
    if location == 0 as std::os::raw::c_int {
        png_error(png_ptr,
                  b"invalid location in png_set_unknown_chunks\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    /* Now reduce the location to the top-most set bit by removing each least
    * significant bit in turn.
    */
    while location != location & -location {
        location &= !(location & -location)
    }
    /* The cast is safe because 'location' is a bit mask and only the low four
    * bits are significant.
    */
    return location as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_set_unknown_chunks(mut png_ptr:
                                                    png_const_structrp,
                                                mut info_ptr: png_inforp,
                                                mut unknowns:
                                                    png_const_unknown_chunkp,
                                                mut num_unknowns:
                                                    std::os::raw::c_int) {
    let mut np: png_unknown_chunkp = 0 as *mut png_unknown_chunk;
    if png_ptr.is_null() || info_ptr.is_null() ||
           num_unknowns <= 0 as std::os::raw::c_int || unknowns.is_null() {
        return
    }
    /* Check for the failure cases where support has been disabled at compile
    * time.  This code is hardly ever compiled - it's here because
    * STORE_UNKNOWN_CHUNKS is set by both read and write code (compiling in this
    * code) but may be meaningless if the read or write handling of unknown
    * chunks is not compiled in.
    */
    /* Prior to 1.6.0 this code used png_malloc_warn; however, this meant that
    * unknown critical chunks could be lost with just a warning resulting in
    * undefined behavior.  Now png_chunk_report is used to provide behavior
    * appropriate to read or write.
    */
    np =
        png_realloc_array(png_ptr,
                          (*info_ptr).unknown_chunks as png_const_voidp,
                          (*info_ptr).unknown_chunks_num, num_unknowns,
                          ::std::mem::size_of::<png_unknown_chunk>() as
                              std::os::raw::c_ulong) as
            png_unknown_chunkp; /* safe because it is initialized */
    if np.is_null() {
        png_chunk_report(png_ptr,
                         b"too many unknown chunks\x00" as *const u8 as
                             *const std::os::raw::c_char, 1 as std::os::raw::c_int);
        return
    }
    png_free(png_ptr, (*info_ptr).unknown_chunks as png_voidp);
    (*info_ptr).unknown_chunks = np;
    (*info_ptr).free_me |= 0x200 as std::os::raw::c_uint;
    np = np.offset((*info_ptr).unknown_chunks_num as isize);
    let mut current_block_22: u64;
    /* Increment unknown_chunks_num each time round the loop to protect the
    * just-allocated chunk data.
    */
    while num_unknowns > 0 as std::os::raw::c_int {
        memcpy((*np).name.as_mut_ptr() as *mut std::os::raw::c_void,
               (*unknowns).name.as_ptr() as *const std::os::raw::c_void,
               ::std::mem::size_of::<[png_byte; 5]>() as std::os::raw::c_ulong);
        (*np).name[(::std::mem::size_of::<[png_byte; 5]>() as
                        std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                       usize] = '\u{0}' as i32 as png_byte;
        (*np).location =
            check_location(png_ptr, (*unknowns).location as std::os::raw::c_int);
        if (*unknowns).size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            (*np).data = 0 as *mut png_byte;
            (*np).size = 0 as std::os::raw::c_int as png_size_t;
            current_block_22 = 11042950489265723346;
        } else {
            (*np).data =
                png_malloc_base(png_ptr, (*unknowns).size) as *mut png_byte;
            if (*np).data.is_null() {
                png_chunk_report(png_ptr,
                                 b"unknown chunk: out of memory\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 1 as std::os::raw::c_int);
                current_block_22 = 1917311967535052937;
            } else {
                memcpy((*np).data as *mut std::os::raw::c_void,
                       (*unknowns).data as *const std::os::raw::c_void,
                       (*unknowns).size);
                (*np).size = (*unknowns).size;
                current_block_22 = 11042950489265723346;
            }
        }
        match current_block_22 {
            11042950489265723346 => {
                /* These increments are skipped on out-of-memory for the data - the
       * unknown chunk entry gets overwritten if the png_chunk_report returns.
       * This is correct in the read case (the chunk is just dropped.)
       */
                np = np.offset(1);
                (*info_ptr).unknown_chunks_num += 1
            }
            _ => { }
        }
        /* But just skip storing the unknown chunk */
        num_unknowns -= 1;
        unknowns = unknowns.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_set_unknown_chunk_location(mut png_ptr:
                                                            png_const_structrp,
                                                        mut info_ptr:
                                                            png_inforp,
                                                        mut chunk:
                                                            std::os::raw::c_int,
                                                        mut location:
                                                            std::os::raw::c_int) {
    /* This API is pretty pointless in 1.6.0 because the location can be set
    * before the call to png_set_unknown_chunks.
    *
    * TODO: add a png_app_warning in 1.7
    */
    if !png_ptr.is_null() && !info_ptr.is_null() && chunk >= 0 as std::os::raw::c_int
           && chunk < (*info_ptr).unknown_chunks_num {
        if location &
               (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int | 0x8 as std::os::raw::c_int)
               == 0 as std::os::raw::c_int {
            png_app_error(png_ptr,
                          b"invalid unknown chunk location\x00" as *const u8
                              as *const std::os::raw::c_char);
            /* also undocumented */
            if location as std::os::raw::c_uint & 0x4 as std::os::raw::c_uint !=
                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                /* Fake out the pre 1.6.0 behavior: */
                /* undocumented! */
                location = 0x8 as std::os::raw::c_int
            } else { location = 0x1 as std::os::raw::c_int }
        }
        (*(*info_ptr).unknown_chunks.offset(chunk as isize)).location =
            check_location(png_ptr, location)
    };
}
/* STORE_UNKNOWN_CHUNKS */
unsafe extern "C" fn add_one_chunk(mut list: png_bytep,
                                   mut count: std::os::raw::c_uint,
                                   mut add: png_const_bytep,
                                   mut keep: std::os::raw::c_int) -> std::os::raw::c_uint {
    let mut i: std::os::raw::c_uint = 0;
    /* Utility function: update the 'keep' state of a chunk if it is already in
    * the list, otherwise add it to the list.
    */
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < count {
        if memcmp(list as *const std::os::raw::c_void, add as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
            *list.offset(4 as std::os::raw::c_int as isize) = keep as png_byte;
            return count
        }
        i = i.wrapping_add(1);
        list = list.offset(5 as std::os::raw::c_int as isize)
    }
    if keep != 0 as std::os::raw::c_int {
        count = count.wrapping_add(1);
        memcpy(list as *mut std::os::raw::c_void, add as *const std::os::raw::c_void,
               4 as std::os::raw::c_int as std::os::raw::c_ulong);
        *list.offset(4 as std::os::raw::c_int as isize) = keep as png_byte
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn png_set_keep_unknown_chunks(mut png_ptr:
                                                         png_structrp,
                                                     mut keep: std::os::raw::c_int,
                                                     mut chunk_list:
                                                         png_const_bytep,
                                                     mut num_chunks_in:
                                                         std::os::raw::c_int) {
    let mut new_list: png_bytep = 0 as *mut png_byte;
    let mut num_chunks: std::os::raw::c_uint = 0;
    let mut old_num_chunks: std::os::raw::c_uint = 0;
    if png_ptr.is_null() { return }
    if keep < 0 as std::os::raw::c_int || keep >= 4 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"png_set_keep_unknown_chunks: invalid keep\x00" as
                          *const u8 as *const std::os::raw::c_char);
        return
    }
    if num_chunks_in <= 0 as std::os::raw::c_int {
        (*png_ptr).unknown_default = keep;
        /* '0' means just set the flags, so stop here */
        if num_chunks_in == 0 as std::os::raw::c_int { return }
    }
    if num_chunks_in < 0 as std::os::raw::c_int {
        /* Ignore all unknown chunks and all chunks recognized by
       * libpng except for IHDR, PLTE, tRNS, IDAT, and IEND
       */
        static mut chunks_to_ignore: [png_byte; 90] =
            [98 as std::os::raw::c_int as png_byte, 75 as std::os::raw::c_int as png_byte,
             71 as std::os::raw::c_int as png_byte, 68 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 99 as std::os::raw::c_int as png_byte,
             72 as std::os::raw::c_int as png_byte, 82 as std::os::raw::c_int as png_byte,
             77 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             101 as std::os::raw::c_int as png_byte, 88 as std::os::raw::c_int as png_byte,
             73 as std::os::raw::c_int as png_byte, 102 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 103 as std::os::raw::c_int as png_byte,
             65 as std::os::raw::c_int as png_byte, 77 as std::os::raw::c_int as png_byte,
             65 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             104 as std::os::raw::c_int as png_byte, 73 as std::os::raw::c_int as png_byte,
             83 as std::os::raw::c_int as png_byte, 84 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 105 as std::os::raw::c_int as png_byte,
             67 as std::os::raw::c_int as png_byte, 67 as std::os::raw::c_int as png_byte,
             80 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             105 as std::os::raw::c_int as png_byte, 84 as std::os::raw::c_int as png_byte,
             88 as std::os::raw::c_int as png_byte, 116 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 111 as std::os::raw::c_int as png_byte,
             70 as std::os::raw::c_int as png_byte, 70 as std::os::raw::c_int as png_byte,
             115 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             112 as std::os::raw::c_int as png_byte, 67 as std::os::raw::c_int as png_byte,
             65 as std::os::raw::c_int as png_byte, 76 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 112 as std::os::raw::c_int as png_byte,
             72 as std::os::raw::c_int as png_byte, 89 as std::os::raw::c_int as png_byte,
             115 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             115 as std::os::raw::c_int as png_byte, 66 as std::os::raw::c_int as png_byte,
             73 as std::os::raw::c_int as png_byte, 84 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 115 as std::os::raw::c_int as png_byte,
             67 as std::os::raw::c_int as png_byte, 65 as std::os::raw::c_int as png_byte,
             76 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             115 as std::os::raw::c_int as png_byte, 80 as std::os::raw::c_int as png_byte,
             76 as std::os::raw::c_int as png_byte, 84 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 115 as std::os::raw::c_int as png_byte,
             84 as std::os::raw::c_int as png_byte, 69 as std::os::raw::c_int as png_byte,
             82 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             115 as std::os::raw::c_int as png_byte, 82 as std::os::raw::c_int as png_byte,
             71 as std::os::raw::c_int as png_byte, 66 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 116 as std::os::raw::c_int as png_byte,
             69 as std::os::raw::c_int as png_byte, 88 as std::os::raw::c_int as png_byte,
             116 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte,
             116 as std::os::raw::c_int as png_byte, 73 as std::os::raw::c_int as png_byte,
             77 as std::os::raw::c_int as png_byte, 69 as std::os::raw::c_int as png_byte,
             '\u{0}' as i32 as png_byte, 122 as std::os::raw::c_int as png_byte,
             84 as std::os::raw::c_int as png_byte, 88 as std::os::raw::c_int as png_byte,
             116 as std::os::raw::c_int as png_byte, '\u{0}' as i32 as png_byte];
        chunk_list = chunks_to_ignore.as_ptr();
        num_chunks =
            (::std::mem::size_of::<[png_byte; 90]>() as std::os::raw::c_ulong as
                 std::os::raw::c_uint).wrapping_div(5 as std::os::raw::c_uint)
    } else {
        /* num_chunks_in > 0 */
        if chunk_list.is_null() {
            /* Prior to 1.6.0 this was silently ignored, now it is an app_error
          * which can be switched off.
          */
            png_app_error(png_ptr,
                          b"png_set_keep_unknown_chunks: no chunk list\x00" as
                              *const u8 as *const std::os::raw::c_char);
            return
        }
        num_chunks = num_chunks_in as std::os::raw::c_uint
    }
    old_num_chunks = (*png_ptr).num_chunk_list;
    if (*png_ptr).chunk_list.is_null() {
        old_num_chunks = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* Since num_chunks is always restricted to UINT_MAX/5 this can't overflow.
    */
    if num_chunks.wrapping_add(old_num_chunks) >
           (2147483647 as std::os::raw::c_int as
                std::os::raw::c_uint).wrapping_mul(2 as
                                               std::os::raw::c_uint).wrapping_add(1 as
                                                                              std::os::raw::c_uint).wrapping_div(5
                                                                                                             as
                                                                                                             std::os::raw::c_int
                                                                                                             as
                                                                                                             std::os::raw::c_uint)
       {
        png_app_error(png_ptr,
                      b"png_set_keep_unknown_chunks: too many chunks\x00" as
                          *const u8 as *const std::os::raw::c_char);
        return
    }
    /* If these chunks are being reset to the default then no more memory is
    * required because add_one_chunk above doesn't extend the list if the 'keep'
    * parameter is the default.
    */
    if keep != 0 as std::os::raw::c_int {
        new_list =
            png_malloc(png_ptr,
                       (5 as std::os::raw::c_int as
                            std::os::raw::c_uint).wrapping_mul(num_chunks.wrapping_add(old_num_chunks))
                           as png_alloc_size_t) as png_bytep;
        if old_num_chunks > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            memcpy(new_list as *mut std::os::raw::c_void,
                   (*png_ptr).chunk_list as *const std::os::raw::c_void,
                   (5 as std::os::raw::c_int as
                        std::os::raw::c_uint).wrapping_mul(old_num_chunks) as
                       std::os::raw::c_ulong);
        }
    } else if old_num_chunks > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        new_list = (*png_ptr).chunk_list
    } else { new_list = 0 as png_bytep }
    /* Add the new chunks together with each one's handling code.  If the chunk
    * already exists the code is updated, otherwise the chunk is added to the
    * end.  (In libpng 1.6.0 order no longer matters because this code enforces
    * the earlier convention that the last setting is the one that is used.)
    */
    if !new_list.is_null() {
        let mut inlist: png_const_bytep = 0 as *const png_byte;
        let mut outlist: png_bytep = 0 as *mut png_byte;
        let mut i: std::os::raw::c_uint = 0;
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < num_chunks {
            old_num_chunks =
                add_one_chunk(new_list, old_num_chunks,
                              chunk_list.offset((5 as std::os::raw::c_int as
                                                     std::os::raw::c_uint).wrapping_mul(i)
                                                    as isize), keep);
            i = i.wrapping_add(1)
        }
        /* Now remove any spurious 'default' entries. */
        num_chunks = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        outlist = new_list;
        inlist = outlist as png_const_bytep;
        while i < old_num_chunks {
            if *inlist.offset(4 as std::os::raw::c_int as isize) != 0 {
                if outlist != inlist as png_bytep {
                    memcpy(outlist as *mut std::os::raw::c_void,
                           inlist as *const std::os::raw::c_void,
                           5 as std::os::raw::c_int as std::os::raw::c_ulong);
                }
                outlist = outlist.offset(5 as std::os::raw::c_int as isize);
                num_chunks = num_chunks.wrapping_add(1)
            }
            i = i.wrapping_add(1);
            inlist = inlist.offset(5 as std::os::raw::c_int as isize)
        }
        /* This means the application has removed all the specialized handling. */
        if num_chunks == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if (*png_ptr).chunk_list != new_list {
                png_free(png_ptr, new_list as png_voidp); /* checked above */
            }
            new_list = 0 as png_bytep
        }
    } else { num_chunks = 0 as std::os::raw::c_int as std::os::raw::c_uint }
    (*png_ptr).num_chunk_list = num_chunks;
    if (*png_ptr).chunk_list != new_list {
        if !(*png_ptr).chunk_list.is_null() {
            png_free(png_ptr, (*png_ptr).chunk_list as png_voidp);
        }
        (*png_ptr).chunk_list = new_list
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_set_rows(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut row_pointers: png_bytepp) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    if !(*info_ptr).row_pointers.is_null() &&
           (*info_ptr).row_pointers != row_pointers {
        png_free_data(png_ptr, info_ptr, 0x40 as std::os::raw::c_uint,
                      0 as std::os::raw::c_int);
    }
    (*info_ptr).row_pointers = row_pointers;
    if !row_pointers.is_null() {
        (*info_ptr).valid |= 0x8000 as std::os::raw::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_set_compression_buffer_size(mut png_ptr:
                                                             png_structrp,
                                                         mut size:
                                                             png_size_t) {
    if png_ptr.is_null() { return }
    if size == 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
           size > 0x7fffffff as std::os::raw::c_long as png_uint_32 as std::os::raw::c_ulong {
        png_error(png_ptr,
                  b"invalid compression buffer size\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        (*png_ptr).IDAT_read_size = size as png_uint_32;
        return
    }
    if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*png_ptr).zowner != 0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_warning(png_ptr,
                        b"Compression buffer size cannot be changed because it is in use\x00"
                            as *const u8 as *const std::os::raw::c_char);
            return
        }
        /* Some compilers complain that this is always false.  However, it
       * can be true when integer overflow happens.
       */
        if size > -(1 as std::os::raw::c_int) as uInt as std::os::raw::c_ulong {
            png_warning(png_ptr,
                        b"Compression buffer size limited to system maximum\x00"
                            as *const u8 as *const std::os::raw::c_char);
            size = -(1 as std::os::raw::c_int) as uInt as png_size_t
            /* must fit */
        }
        if size < 6 as std::os::raw::c_int as std::os::raw::c_ulong {
            /* Deflate will potentially go into an infinite loop on a SYNC_FLUSH
          * if this is permitted.
          */
            png_warning(png_ptr,
                        b"Compression buffer size cannot be reduced below 6\x00"
                            as *const u8 as *const std::os::raw::c_char);
            return
        }
        if (*png_ptr).zbuffer_size as std::os::raw::c_ulong != size {
            png_free_buffer_list(png_ptr, &mut (*png_ptr).zbuffer_list);
            (*png_ptr).zbuffer_size = size as uInt
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_set_invalid(mut png_ptr: png_const_structrp,
                                         mut info_ptr: png_inforp,
                                         mut mask: std::os::raw::c_int) {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        (*info_ptr).valid &= !mask as std::os::raw::c_uint
    };
}
/* This function was added to libpng 1.2.6 */
#[no_mangle]
pub unsafe extern "C" fn png_set_user_limits(mut png_ptr: png_structrp,
                                             mut user_width_max: png_uint_32,
                                             mut user_height_max:
                                                 png_uint_32) {
    /* Images with dimensions larger than these limits will be
    * rejected by png_set_IHDR().  To accept any PNG datastream
    * regardless of dimensions, set both limits to 0x7fffffff.
    */
    if png_ptr.is_null() { return }
    (*png_ptr).user_width_max = user_width_max;
    (*png_ptr).user_height_max = user_height_max;
}
/* This function was added to libpng 1.4.0 */
#[no_mangle]
pub unsafe extern "C" fn png_set_chunk_cache_max(mut png_ptr: png_structrp,
                                                 mut user_chunk_cache_max:
                                                     png_uint_32) {
    if !png_ptr.is_null() {
        (*png_ptr).user_chunk_cache_max = user_chunk_cache_max
    };
}
/* This function was added to libpng 1.4.1 */
#[no_mangle]
pub unsafe extern "C" fn png_set_chunk_malloc_max(mut png_ptr: png_structrp,
                                                  mut user_chunk_malloc_max:
                                                      png_alloc_size_t) {
    if !png_ptr.is_null() {
        (*png_ptr).user_chunk_malloc_max = user_chunk_malloc_max
    };
}
/* ?SET_USER_LIMITS */
#[no_mangle]
pub unsafe extern "C" fn png_set_benign_errors(mut png_ptr: png_structrp,
                                               mut allowed: std::os::raw::c_int) {
    /* If allowed is 1, png_benign_error() is treated as a warning.
    *
    * If allowed is 0, png_benign_error() is treated as an error (which
    * is the default behavior if png_set_benign_errors() is not called).
    */
    if allowed != 0 as std::os::raw::c_int {
        (*png_ptr).flags |=
            0x100000 as std::os::raw::c_uint | 0x200000 as std::os::raw::c_uint |
                0x400000 as std::os::raw::c_uint
    } else {
        (*png_ptr).flags &=
            !(0x100000 as std::os::raw::c_uint | 0x200000 as std::os::raw::c_uint |
                  0x400000 as std::os::raw::c_uint)
    };
}
/* BENIGN_ERRORS */
/* Whether to report invalid palette index; added at libng-1.5.10.
    * It is possible for an indexed (color-type==3) PNG file to contain
    * pixels with invalid (out-of-range) indexes if the PLTE chunk has
    * fewer entries than the image's bit-depth would allow. We recover
    * from this gracefully by filling any incomplete palette with zeros
    * (opaque black).  By default, when this occurs libpng will issue
    * a benign error.  This API can be used to override that behavior.
    */
#[no_mangle]
pub unsafe extern "C" fn png_set_check_for_invalid_index(mut png_ptr:
                                                             png_structrp,
                                                         mut allowed:
                                                             std::os::raw::c_int) {
    if allowed > 0 as std::os::raw::c_int {
        (*png_ptr).num_palette_max = 0 as std::os::raw::c_int
    } else { (*png_ptr).num_palette_max = -(1 as std::os::raw::c_int) };
}
/* READ || WRITE */
/* TEXT || pCAL || iCCP || sPLT */

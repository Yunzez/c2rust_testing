



use std::os::raw::c_int;

extern "C" {
    
    
    
    
    
    
    
    
    fn getc(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn ungetc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    
    fn fseek(__stream: *mut FILE, __off: std::os::raw::c_long, __whence: std::os::raw::c_int)
     -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
}
pub use crate::src::libpng::png::png_set_sig_bytes;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngread::png_read_png;
pub use crate::src::libpng::pngrio::png_set_read_fn;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::pngset::png_set_PLTE;
pub use crate::src::libpng::pngset::png_set_sBIT;
pub use crate::src::pngxtern::pngxmem::pngx_malloc_rows_extended;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
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
pub type png_structrp = crate::src::libpng::png::png_structrp;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::pngerror::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color;
pub type png_const_colorp = crate::src::libpng::pngrutil::png_const_colorp;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::pngerror::png_color_8_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8;
pub type png_const_color_8p = crate::src::libpng::pngrutil::png_const_color_8p;
pub type png_rw_ptr = crate::src::libpng::png::png_rw_ptr;
pub type pngx_alloc_size_t = crate::src::pngxtern::pngxmem::pngx_alloc_size_t;
/* PNG format                */
/* ****************************************************************************/
/* BMP memory utilities                                                      */
/* ****************************************************************************/
fn bmp_get_word(ptr: *const u8) -> u32 {
    let byte0 = unsafe { *ptr as u32 };
    let byte1 = unsafe { *(ptr.add(1)) as u32 } << 8;
    byte0.wrapping_add(byte1)
}

unsafe extern "C" fn bmp_get_dword(mut ptr: png_bytep) -> png_uint_32 {
    return (*ptr.offset(0 as std::os::raw::c_int as isize) as
                png_uint_32).wrapping_add((*ptr.offset(1 as std::os::raw::c_int as
                                                           isize) as
                                               png_uint_32) <<
                                              8 as
                                                  std::os::raw::c_int).wrapping_add((*ptr.offset(2
                                                                                             as
                                                                                             std::os::raw::c_int
                                                                                             as
                                                                                             isize)
                                                                                 as
                                                                                 png_uint_32)
                                                                                <<
                                                                                16
                                                                                    as
                                                                                    std::os::raw::c_int).wrapping_add((*ptr.offset(3
                                                                                                                               as
                                                                                                                               std::os::raw::c_int
                                                                                                                               as
                                                                                                                               isize)
                                                                                                                   as
                                                                                                                   png_uint_32)
                                                                                                                  <<
                                                                                                                  24
                                                                                                                      as
                                                                                                                      std::os::raw::c_int);
}
/* ****************************************************************************/
/* BMP helpers                                                               */
/* ****************************************************************************/
unsafe extern "C" fn bmp_memset_bytes(mut ptr: png_bytep, mut offset: size_t,
                                      mut ch: std::os::raw::c_int, mut len: size_t) {
    memset(ptr.offset(offset as isize) as *mut std::os::raw::c_void, ch, len);
}
unsafe extern "C" fn bmp_memset_halfbytes(mut ptr: png_bytep,
                                          mut offset: size_t,
                                          mut ch: std::os::raw::c_int,
                                          mut len: size_t) {
    if len == 0 as std::os::raw::c_int as std::os::raw::c_ulong { return }
    ptr =
        ptr.offset(offset.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                       isize);
    if offset & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
        /* use half-byte operations at odd offset */
        *ptr =
            (*ptr as std::os::raw::c_int & 0xf0 as std::os::raw::c_int |
                 ch & 0xf as std::os::raw::c_int) as png_byte; /* skip padding */
        ch =
            (ch & 0xf0 as std::os::raw::c_int) >> 4 as std::os::raw::c_int |
                (ch & 0xf as std::os::raw::c_int) << 4 as std::os::raw::c_int;
        ptr = ptr.offset(1);
        len = len.wrapping_sub(1)
    }
    memset(ptr as *mut std::os::raw::c_void, ch,
           len.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong));
    if len & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
        *ptr.offset(len.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                        isize) = (ch & 0xf0 as std::os::raw::c_int) as png_byte
    };
}
unsafe extern "C" fn bmp_fread_bytes(mut ptr: png_bytep, mut offset: size_t,
                                     mut len: size_t, mut stream: *mut FILE)
 -> size_t {
    let mut result: size_t = 0;
    result =
        fread(ptr.offset(offset as isize) as *mut std::os::raw::c_void,
              1 as std::os::raw::c_int as std::os::raw::c_ulong, len, stream);
    if len & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 { getc(stream); }
    return result;
}
unsafe extern "C" fn bmp_fread_halfbytes(mut ptr: png_bytep,
                                         mut offset: size_t, mut len: size_t,
                                         mut stream: *mut FILE) -> size_t {
    let mut result: size_t = 0;
    let mut ch: std::os::raw::c_int = 0;
    if len == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as size_t
    }
    ptr =
        ptr.offset(offset.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                       isize);
    if offset & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
        /* use half-byte operations at odd offset */
        result = 0 as std::os::raw::c_int as size_t; /* skip padding */
        while result < len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
            ch = getc(stream);
            if ch == -(1 as std::os::raw::c_int) { break ; }
            *ptr =
                (*ptr as std::os::raw::c_int & 0xf0 as std::os::raw::c_int |
                     (ch & 0xf0 as std::os::raw::c_int) >> 4 as std::os::raw::c_int) as
                    png_byte;
            ptr = ptr.offset(1);
            *ptr =
                ((ch & 0xf as std::os::raw::c_int) << 4 as std::os::raw::c_int) as png_byte;
            result =
                (result as
                     std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t
        }
    } else {
        result =
            fread(ptr as *mut std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
                  len.wrapping_add(1 as std::os::raw::c_int as
                                       std::os::raw::c_ulong).wrapping_div(2 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong),
                  stream).wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    if len & 2 as std::os::raw::c_int as std::os::raw::c_ulong != 0 { getc(stream); }
    return if result <= len { result } else { len };
}
/* ****************************************************************************/
/* BMP packbit (BITFIELDS) helpers                                           */
/* ****************************************************************************/
unsafe extern "C" fn bmp_process_mask(mut bmp_mask: png_uint_32,
                                      mut sig_bit: png_bytep,
                                      mut shift_bit: png_bytep) {
    *shift_bit = 0 as std::os::raw::c_int as png_byte;
    *sig_bit = *shift_bit;
    if bmp_mask == 0 as std::os::raw::c_int as std::os::raw::c_uint { return }
    while bmp_mask & 1 as std::os::raw::c_int as std::os::raw::c_uint ==
              0 as std::os::raw::c_int as std::os::raw::c_uint {
        bmp_mask >>= 1 as std::os::raw::c_int;
        *shift_bit = (*shift_bit).wrapping_add(1)
    }
    while bmp_mask != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        if bmp_mask & 1 as std::os::raw::c_int as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint ||
               *sig_bit as std::os::raw::c_int >= 8 as std::os::raw::c_int {
            *sig_bit = 0 as std::os::raw::c_int as png_byte;
            return
        }
        bmp_mask >>= 1 as std::os::raw::c_int;
        *sig_bit = (*sig_bit).wrapping_add(1)
    };
}
/* ****************************************************************************/
/* BMP I/O utilities                                                         */
/* ****************************************************************************/
unsafe extern "C" fn bmp_read_rows(mut begin_row: png_bytepp,
                                   mut end_row: png_bytepp,
                                   mut row_size: size_t,
                                   mut compression: std::os::raw::c_uint,
                                   mut stream: *mut FILE) -> size_t {
    let mut result: size_t = 0; /* this should not happen */
    let mut crt_row: png_bytepp = 0 as *mut *mut png_byte; /* overflow */
    let mut inc: std::os::raw::c_int =
        0; /* error: compression method not applicable. */
    let mut crtn: size_t = 0;
    let mut dcrtn: size_t = 0;
    let mut endn: size_t = 0;
    let mut len: std::os::raw::c_uint = 0;
    let mut b1: std::os::raw::c_uint = 0;
    let mut b2: std::os::raw::c_uint = 0;
    let mut ch: std::os::raw::c_int = 0;
    let mut bmp_memset_fn:
            Option<unsafe extern "C" fn(_: png_bytep, _: size_t,
                                        _: std::os::raw::c_int, _: size_t) -> ()> =
        None;
    let mut bmp_fread_fn:
            Option<unsafe extern "C" fn(_: png_bytep, _: size_t, _: size_t,
                                        _: *mut FILE) -> size_t> = None;
    if row_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as size_t
    }
    inc =
        if begin_row <= end_row {
            1 as std::os::raw::c_int
        } else { -(1 as std::os::raw::c_int) };
    crtn = 0 as std::os::raw::c_int as size_t;
    result = 0 as std::os::raw::c_int as size_t;
    if compression == 2 as std::os::raw::c_int as std::os::raw::c_uint {
        endn = row_size.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong);
        if endn <= row_size { return 0 as std::os::raw::c_int as size_t }
        bmp_memset_fn =
            Some(bmp_memset_halfbytes as
                     unsafe extern "C" fn(_: png_bytep, _: size_t,
                                          _: std::os::raw::c_int, _: size_t) -> ());
        bmp_fread_fn =
            Some(bmp_fread_halfbytes as
                     unsafe extern "C" fn(_: png_bytep, _: size_t, _: size_t,
                                          _: *mut FILE) -> size_t)
    } else {
        endn = row_size;
        bmp_memset_fn =
            Some(bmp_memset_bytes as
                     unsafe extern "C" fn(_: png_bytep, _: size_t,
                                          _: std::os::raw::c_int, _: size_t) -> ());
        bmp_fread_fn =
            Some(bmp_fread_bytes as
                     unsafe extern "C" fn(_: png_bytep, _: size_t, _: size_t,
                                          _: *mut FILE) -> size_t)
    }
    if compression == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           compression == 3 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Read uncompressed bitmap. */
        crt_row = begin_row;
        while crt_row != end_row {
            crtn =
                bmp_fread_fn.expect("non-null function pointer")(*crt_row,
                                                                 0 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     size_t,
                                                                 endn,
                                                                 stream);
            if crtn != endn { break ; }
            result = result.wrapping_add(1);
            crt_row = crt_row.offset(inc as isize)
        }
    } else if compression == 1 as std::os::raw::c_int as std::os::raw::c_uint ||
                  compression == 2 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Read RLE-compressed bitmap. */
        if compression == 1 as std::os::raw::c_int as std::os::raw::c_uint {
            endn = row_size
        } else {
            /* BI_RLE4 */
            endn = row_size.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong);
            if endn <= row_size { return 0 as std::os::raw::c_int as size_t }
            /* overflow */
        }
        crt_row = begin_row;
        while crt_row != end_row {
            ch = getc(stream);
            b1 = ch as std::os::raw::c_uint;
            ch = getc(stream);
            b2 = ch as std::os::raw::c_uint;
            if ch == -(1 as std::os::raw::c_int) { break ; }
            if b1 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                /* escape */
                if b2 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    /* end of line */
                    bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                      crtn,
                                                                      0 as
                                                                          std::os::raw::c_int,
                                                                      endn.wrapping_sub(crtn));
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as std::os::raw::c_int as size_t;
                    result = result.wrapping_add(1);
                    if !(crt_row == end_row) { continue ; }
                    /* all rows are read */
                    ch = getc(stream); /* check for the end of bitmap */
                    if ch != -(1 as std::os::raw::c_int) && ch != 0 as std::os::raw::c_int {
                        ungetc(ch,
                               stream); /* forget about the end of bitmap */
                        break ; /* expect 1, but break the loop anyway */
                    } else { getc(stream); break ; }
                } else if b2 == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                    /* end of bitmap */
                    bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                      crtn,
                                                                      0 as
                                                                          std::os::raw::c_int,
                                                                      endn.wrapping_sub(crtn));
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as std::os::raw::c_int as size_t;
                    result =
                        if begin_row <= end_row {
                            end_row.offset_from(begin_row) as
                                std::os::raw::c_long
                        } else {
                            begin_row.offset_from(end_row) as
                                std::os::raw::c_long
                        } as size_t;
                    break ;
                    /* the rest is wiped out at the end */
                } else if b2 == 2 as std::os::raw::c_int as std::os::raw::c_uint {
                    /* delta */
                    ch = getc(stream); /* horiz. offset */
                    b1 = ch as std::os::raw::c_uint; /* vert. offset */
                    ch = getc(stream);
                    b2 = ch as std::os::raw::c_uint;
                    if ch == -(1 as std::os::raw::c_int) { break ; }
                    dcrtn =
                        if (b1 as std::os::raw::c_ulong) < endn.wrapping_sub(crtn) {
                            crtn.wrapping_add(b1 as std::os::raw::c_ulong)
                        } else { endn };
                    while b2 > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                          crtn,
                                                                          0 as
                                                                              std::os::raw::c_int,
                                                                          endn.wrapping_sub(crtn));
                        crt_row = crt_row.offset(inc as isize);
                        crtn = 0 as std::os::raw::c_int as size_t;
                        result = result.wrapping_add(1);
                        if crt_row == end_row { break ; }
                        b2 = b2.wrapping_sub(1)
                    }
                    if crt_row != end_row {
                        bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                          crtn,
                                                                          0 as
                                                                              std::os::raw::c_int,
                                                                          dcrtn.wrapping_sub(crtn));
                    }
                } else {
                    /* b2 >= 3 bytes in absolute mode */
                    len =
                        if b2 as std::os::raw::c_ulong <= endn.wrapping_sub(crtn) {
                            b2
                        } else { endn.wrapping_sub(crtn) as std::os::raw::c_uint };
                    if bmp_fread_fn.expect("non-null function pointer")(*crt_row,
                                                                        crtn,
                                                                        len as
                                                                            size_t,
                                                                        stream)
                           != len as std::os::raw::c_ulong {
                        break ;
                    }
                    crtn =
                        (crtn as
                             std::os::raw::c_ulong).wrapping_add(len as std::os::raw::c_ulong)
                            as size_t as size_t
                }
            } else {
                /* b1 > 0 bytes in run-length encoded mode */
                len =
                    if b1 as std::os::raw::c_ulong <= endn.wrapping_sub(crtn) {
                        b1
                    } else { endn.wrapping_sub(crtn) as std::os::raw::c_uint };
                bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                  crtn,
                                                                  b2 as
                                                                      std::os::raw::c_int,
                                                                  len as
                                                                      size_t);
                crtn =
                    (crtn as std::os::raw::c_ulong).wrapping_add(len as std::os::raw::c_ulong)
                        as size_t as size_t
            }
        }
    } else { return 0 as std::os::raw::c_int as size_t }
    /* Wipe out the portion left unread. */
    while crt_row != end_row {
        bmp_memset_fn.expect("non-null function pointer")(*crt_row, crtn,
                                                          0 as std::os::raw::c_int,
                                                          endn.wrapping_sub(crtn));
        crtn = 0 as std::os::raw::c_int as size_t;
        crt_row = crt_row.offset(inc as isize)
    }
    return result;
}
/* ****************************************************************************/
/* BMP-to-PNG sample conversion                                              */
/* ****************************************************************************/
unsafe extern "C" fn bmp_to_png_rows(mut row_pointers: png_bytepp,
                                     mut width: png_uint_32,
                                     mut height: png_uint_32,
                                     mut pixdepth: std::os::raw::c_uint,
                                     mut rgba_sig: png_bytep,
                                     mut rgba_shift: png_bytep) {
    let mut src_ptr: png_bytep = 0 as *mut png_byte;
    let mut dest_ptr: png_bytep = 0 as *mut png_byte;
    let mut rgba_mask: [std::os::raw::c_uint; 4] = [0; 4];
    let mut num_samples: std::os::raw::c_uint = 0;
    let mut sample: std::os::raw::c_uint = 0;
    let mut mask: std::os::raw::c_uint = 0;
    let mut wpix: std::os::raw::c_uint = 0;
    let mut dwpix: png_uint_32 = 0;
    let mut x: png_uint_32 = 0;
    let mut y: png_uint_32 = 0;
    let mut i: std::os::raw::c_uint = 0;
    if pixdepth == 24 as std::os::raw::c_int as std::os::raw::c_uint {
        /* BGR -> RGB */
        y = 0 as std::os::raw::c_int as png_uint_32;
        while y < height {
            src_ptr = *row_pointers.offset(y as isize);
            x = 0 as std::os::raw::c_int as png_uint_32;
            while x < width {
                let mut tmp: png_byte =
                    *src_ptr.offset(0 as std::os::raw::c_int as isize);
                *src_ptr.offset(0 as std::os::raw::c_int as isize) =
                    *src_ptr.offset(2 as std::os::raw::c_int as isize);
                *src_ptr.offset(2 as std::os::raw::c_int as isize) = tmp;
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(3 as std::os::raw::c_int as isize)
            }
            y = y.wrapping_add(1)
        }
        return
    }
    num_samples =
        if *rgba_sig.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               0 as std::os::raw::c_int {
            4 as std::os::raw::c_int
        } else { 3 as std::os::raw::c_int } as std::os::raw::c_uint;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < num_samples {
        rgba_mask[i as usize] =
            ((1 as std::os::raw::c_uint) <<
                 *rgba_sig.offset(i as isize) as
                     std::os::raw::c_int).wrapping_sub(1 as std::os::raw::c_int as
                                                   std::os::raw::c_uint);
        i = i.wrapping_add(1)
    }
    if pixdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
        y = 0 as std::os::raw::c_int as png_uint_32;
        while y < height {
            src_ptr =
                (*row_pointers.offset(y as
                                          isize)).offset(width.wrapping_sub(1
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_uint).wrapping_mul(2
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               std::os::raw::c_uint)
                                                             as isize);
            dest_ptr =
                (*row_pointers.offset(y as
                                          isize)).offset(width.wrapping_sub(1
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_uint).wrapping_mul(num_samples)
                                                             as isize);
            x = 0 as std::os::raw::c_int as png_uint_32;
            while x < width {
                /* Inline bmp_get_word() for performance reasons. */
                wpix =
                    (*src_ptr.offset(0 as std::os::raw::c_int as isize) as
                         std::os::raw::c_uint).wrapping_add((*src_ptr.offset(1 as
                                                                         std::os::raw::c_int
                                                                         as
                                                                         isize)
                                                         as std::os::raw::c_uint) <<
                                                        8 as std::os::raw::c_int);
                i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample =
                        wpix >> *rgba_shift.offset(i as isize) as std::os::raw::c_int
                            & mask;
                    *dest_ptr.offset(i as isize) =
                        sample.wrapping_mul(255 as std::os::raw::c_int as
                                                std::os::raw::c_uint).wrapping_add(mask.wrapping_div(2
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_uint)).wrapping_div(mask)
                            as png_byte;
                    i = i.wrapping_add(1)
                }
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(-(2 as std::os::raw::c_int as isize));
                dest_ptr = dest_ptr.offset(-(num_samples as isize))
            }
            y = y.wrapping_add(1)
        }
    } else if pixdepth == 32 as std::os::raw::c_int as std::os::raw::c_uint {
        y = 0 as std::os::raw::c_int as png_uint_32;
        while y < height {
            dest_ptr = *row_pointers.offset(y as isize);
            src_ptr = dest_ptr;
            x = 0 as std::os::raw::c_int as png_uint_32;
            while x < width {
                /* Inline bmp_get_dword() for performance reasons. */
                dwpix =
                    (*src_ptr.offset(0 as std::os::raw::c_int as isize) as
                         png_uint_32).wrapping_add((*src_ptr.offset(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        isize)
                                                        as png_uint_32) <<
                                                       8 as
                                                           std::os::raw::c_int).wrapping_add((*src_ptr.offset(2
                                                                                                          as
                                                                                                          std::os::raw::c_int
                                                                                                          as
                                                                                                          isize)
                                                                                          as
                                                                                          png_uint_32)
                                                                                         <<
                                                                                         16
                                                                                             as
                                                                                             std::os::raw::c_int).wrapping_add((*src_ptr.offset(3
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_int
                                                                                                                                            as
                                                                                                                                            isize)
                                                                                                                            as
                                                                                                                            png_uint_32)
                                                                                                                           <<
                                                                                                                           24
                                                                                                                               as
                                                                                                                               std::os::raw::c_int);
                i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample =
                        dwpix >> *rgba_shift.offset(i as isize) as std::os::raw::c_int
                            & mask;
                    *dest_ptr.offset(i as isize) =
                        sample.wrapping_mul(255 as std::os::raw::c_int as
                                                std::os::raw::c_uint).wrapping_add(mask.wrapping_div(2
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_uint)).wrapping_div(mask)
                            as png_byte;
                    i = i.wrapping_add(1)
                }
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(4 as std::os::raw::c_int as isize);
                dest_ptr = dest_ptr.offset(num_samples as isize)
            }
            y = y.wrapping_add(1)
        }
    };
    /* else do nothing */
}
/* ****************************************************************************/
/* BMP read support for pngxtern                                             */
/* ****************************************************************************/

pub unsafe extern "C" fn pngx_sig_is_bmp(mut sig: png_bytep,
                                         mut sig_size: size_t,
                                         mut fmt_name_ptr: png_const_charpp,
                                         mut fmt_long_name_ptr:
                                             png_const_charpp)
 -> std::os::raw::c_int {
    static mut bmp_fmt_name: [std::os::raw::c_char; 4] =
        unsafe {
            *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"BMP\x00")
        };
    static mut os2bmp_fmt_long_name: [std::os::raw::c_char; 12] =
        unsafe {
            *::std::mem::transmute::<&[u8; 12],
                                     &[std::os::raw::c_char; 12]>(b"OS/2 Bitmap\x00")
        };
    static mut winbmp_fmt_long_name: [std::os::raw::c_char; 15] =
        unsafe {
            *::std::mem::transmute::<&[u8; 15],
                                     &[std::os::raw::c_char; 15]>(b"Windows Bitmap\x00")
        };
    let mut bihsize: png_uint_32 = 0;
    /* Require at least the bitmap file header and the subsequent 4 bytes. */
    if sig_size < (14 as std::os::raw::c_int + 4 as std::os::raw::c_int) as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    } /* insufficient data */
    if bmp_get_word(sig) != 0x4d42 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    } /* not BMP */
    /* Avoid using bfhsize because it is not reliable. */
    bihsize =
        bmp_get_dword(sig.offset(14 as std::os::raw::c_int as
                                     isize)); /* garbage in bihsize, this cannot be BMP */
    if bihsize > 0x7fffffff as std::os::raw::c_long as png_uint_32 ||
           bihsize != 12 as std::os::raw::c_int as std::os::raw::c_uint &&
               bihsize < 40 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    /* Store the format name. */
    if !fmt_name_ptr.is_null() { *fmt_name_ptr = bmp_fmt_name.as_ptr() }
    if !fmt_long_name_ptr.is_null() {
        if bihsize == 12 as std::os::raw::c_int as std::os::raw::c_uint {
            *fmt_long_name_ptr = os2bmp_fmt_long_name.as_ptr()
        } else { *fmt_long_name_ptr = winbmp_fmt_long_name.as_ptr() }
    }
    return 1 as std::os::raw::c_int;
    /* BMP */
}

pub unsafe extern "C" fn pngx_read_bmp(mut png_ptr: png_structp,
                                       mut info_ptr: png_infop,
                                       mut stream: *mut FILE) -> std::os::raw::c_int {
    let mut bfh: [png_byte; 138] = [0; 138];
    let bih: png_bytep = bfh.as_mut_ptr().offset(14 as std::os::raw::c_int as isize);
    let mut rgbq: [png_byte; 4] = [0; 4];
    let mut offbits: png_uint_32 = 0;
    let mut bihsize: png_uint_32 = 0;
    let mut skip: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut rowsize: png_uint_32 = 0;
    let mut topdown: std::os::raw::c_int = 0;
    let mut pixdepth: std::os::raw::c_uint = 0;
    let mut compression: png_uint_32 = 0;
    let mut palsize: std::os::raw::c_uint = 0;
    let mut palnum: std::os::raw::c_uint = 0;
    let mut rgba_mask: [png_uint_32; 4] = [0; 4];
    let mut rgba_sig: [png_byte; 4] = [0; 4];
    let mut rgba_shift: [png_byte; 4] = [0; 4];
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut palette: [png_color; 256] =
        [png_color{red: 0, green: 0, blue: 0,}; 256];
    let mut sig_bit: png_color_8 =
        png_color_8{red: 0, green: 0, blue: 0, gray: 0, alpha: 0,};
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    let mut begin_row: png_bytepp = 0 as *mut *mut png_byte;
    let mut end_row: png_bytepp = 0 as *mut *mut png_byte;
    let mut i: std::os::raw::c_uint = 0;
    let mut y: size_t = 0;
    /* Find the BMP header. */
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    loop 
         /* skip macbinary header */
         {
        if fread(bfh.as_mut_ptr() as *mut std::os::raw::c_void,
                 (14 as std::os::raw::c_int + 4 as std::os::raw::c_int) as std::os::raw::c_ulong,
                 1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong {
            i = i.wrapping_add(1)
        } else if bmp_get_word(bfh.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                           isize)) ==
                      0x4d42 as std::os::raw::c_int as std::os::raw::c_uint {
            break ;
        }
        if fread(bfh.as_mut_ptr() as *mut std::os::raw::c_void,
                 (128 as std::os::raw::c_int - 14 as std::os::raw::c_int - 4 as std::os::raw::c_int)
                     as std::os::raw::c_ulong, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
                 stream) != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            i = i.wrapping_add(1)
        }
        if i > 0 as std::os::raw::c_int as std::os::raw::c_uint { return 0 as std::os::raw::c_int }
        i = i.wrapping_add(1)
        /* not a BMP file */
    }
    /* Read the BMP header. */
    offbits =
        bmp_get_dword(bfh.as_mut_ptr().offset(10 as std::os::raw::c_int as
                                                  isize)); /* not a BMP file, just a file with a matching signature */
    bihsize =
        bmp_get_dword(bfh.as_mut_ptr().offset(14 as std::os::raw::c_int as
                                                  isize)); /* new skip */
    if offbits > 0x7fffffff as std::os::raw::c_long as png_uint_32 ||
           bihsize > 0x7fffffff as std::os::raw::c_long as png_uint_32 ||
           offbits < bihsize.wrapping_add(14 as std::os::raw::c_int as std::os::raw::c_uint)
           ||
           bihsize != 12 as std::os::raw::c_int as std::os::raw::c_uint &&
               bihsize < 40 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if bihsize > 124 as std::os::raw::c_int as std::os::raw::c_uint {
        skip = bihsize.wrapping_sub(124 as std::os::raw::c_int as std::os::raw::c_uint);
        bihsize = 124 as std::os::raw::c_int as png_uint_32
    } else { skip = 0 as std::os::raw::c_int as png_uint_32 }
    if fread(bih.offset(4 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
             bihsize.wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_uint) as
                 std::os::raw::c_ulong, 1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    if skip > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        if fseek(stream, skip as std::os::raw::c_long, 1 as std::os::raw::c_int) !=
               0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int
        }
    }
    skip =
        offbits.wrapping_sub(bihsize).wrapping_sub(14 as std::os::raw::c_int as
                                                       std::os::raw::c_uint);
    topdown = 0 as std::os::raw::c_int;
    if bihsize < 40 as std::os::raw::c_int as std::os::raw::c_uint {
        /* OS/2 BMP */
        let bih_offset_4 = bih.offset(4).cast::<u8>();
width = bmp_get_word(bih_offset_4);
        let bih_offset_6 = bih.offset(6).cast::<u8>();
height = bmp_get_word(bih_offset_6);
        let bih_offset_10 = bih.offset(10).cast::<u8>();
pixdepth = bmp_get_word(bih_offset_10);
        compression = 0 as std::os::raw::c_int as png_uint_32;
        palsize = 3 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        /* Windows BMP */
        width = bmp_get_dword(bih.offset(4 as std::os::raw::c_int as isize));
        height = bmp_get_dword(bih.offset(8 as std::os::raw::c_int as isize));
        let bih_offset_14 = bih.offset(14).cast::<u8>();
pixdepth = bmp_get_word(bih_offset_14);
        compression = bmp_get_dword(bih.offset(16 as std::os::raw::c_int as isize));
        palsize = 4 as std::os::raw::c_int as std::os::raw::c_uint;
        if height > 0x7fffffff as std::os::raw::c_long as png_uint_32 {
            /* top-down BMP */
            height =
                (-(1 as std::os::raw::c_int) as
                     png_uint_32).wrapping_sub(height).wrapping_add(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_uint);
            topdown = 1 as std::os::raw::c_int
        }
        if bihsize == 40 as std::os::raw::c_int as std::os::raw::c_uint &&
               compression == 3 as std::os::raw::c_int as std::os::raw::c_uint {
            /* Read the RGB[A] mask. */
            i =
                if skip <= 16 as std::os::raw::c_int as std::os::raw::c_uint {
                    skip
                } else { 16 as std::os::raw::c_int as std::os::raw::c_uint };
            if fread(bih.offset(40 as std::os::raw::c_int as isize) as
                         *mut std::os::raw::c_void, i as std::os::raw::c_ulong,
                     1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
                   1 as std::os::raw::c_int as std::os::raw::c_ulong {
                return 0 as std::os::raw::c_int
            }
            bihsize =
                (bihsize as std::os::raw::c_uint).wrapping_add(i) as png_uint_32 as
                    png_uint_32;
            skip =
                (skip as std::os::raw::c_uint).wrapping_sub(i) as png_uint_32 as
                    png_uint_32
        }
    }
    memset(rgba_mask.as_mut_ptr() as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<[png_uint_32; 4]>() as std::os::raw::c_ulong);
    if pixdepth > 8 {
    if compression == 0 {
        if pixdepth == 16 {
            compression = 3;
            rgba_mask[0] = 0x7c00;
            rgba_mask[1] = 0x3e0;
            rgba_mask[2] = 0x1f;
        } else {
            // pixdepth == 24 || pixdepth == 32
            rgba_mask[0] = 0xff0000;
            rgba_mask[1] = 0xff00;
            rgba_mask[2] = 0xff;
        }
    } else if compression == 3 {
        if bihsize >= 52 {
            rgba_mask[0] = bmp_get_dword(bih.add(40));
            rgba_mask[1] = bmp_get_dword(bih.add(44));
            rgba_mask[2] = bmp_get_dword(bih.add(48));
        } else {
            png_error(png_ptr, b"Missing color mask in BMP file\x00" as *const u8 as *const i8);
        }
    }
    if bihsize >= 56 {
        rgba_mask[3] = bmp_get_dword(bih.add(52));
    }
}

match compression {
    0 => {
        // Allow pixel depth values 1, 2, 4, 8, 16, 24, 32.
        if pixdepth > 0 && 32 % pixdepth != 0 && pixdepth != 24 {
            pixdepth = 0;
        }
    }
    1 => {
        if pixdepth != 8 {
            pixdepth = 0;
        }
    }
    2 => {
        if pixdepth != 4 {
            pixdepth = 0;
        }
    }
    3 => {
        if pixdepth != 16 && pixdepth != 32 {
            pixdepth = 0;
        }
    }
    4 => {
        png_error(png_ptr, b"JPEG-compressed BMP files are not supported\x00" as *const u8 as *const i8);
    }
    5 => {
        if ungetc(getc(stream), stream) == 0 {
            // IHDR is likely to follow
            png_set_sig_bytes(png_ptr, 8);
        }
        png_set_read_fn(png_ptr, stream as *mut std::ffi::c_void, None);
        png_read_png(png_ptr, info_ptr, 0, std::ptr::null_mut());
        // TODO: Check for mismatches between the BMP and PNG info.
        return 1;
    }
    _ => {
        png_error(png_ptr, b"Unsupported compression method in BMP file\x00" as *const u8 as *const i8);
    }
}

// Check the BMP image parameters.
if width == 0 || width > 0x7fffffff || height == 0 {
    png_error(png_ptr, b"Invalid image dimensions in BMP file\x00" as *const u8 as *const i8);
}
if pixdepth == 0 {
    png_error(png_ptr, b"Invalid pixel depth in BMP file\x00" as *const u8 as *const i8);
}

    /* Compute the PNG image parameters. */
    if pixdepth <= 8 {
    palnum = skip / palsize;
    if palnum > 256 {
        palnum = 256;
    }
    skip -= palsize * palnum;
    rowsize = (width + (32 / pixdepth) - 1) / (32 / pixdepth) * 4;

    bit_depth = pixdepth as i32;
    color_type = if palnum > 0 { 2 | 1 } else { 0 };
} else {
    palnum = 0;
    bit_depth = 8;
    rowsize = match pixdepth {
        16 => (width * 2 + 3) & !3,
        24 => (width * 3 + 3) & !3,
        32 => width * 4,
        _ => {
            bit_depth = 0; // overflow
            0
        }
    };
    
    if rowsize / width < pixdepth / 8 {
        rowsize = 0;
    }
    
    color_type = if rgba_mask[3] != 0 {
        2 | 4
    } else {
        2
    };
}

if rowsize == 0 {
    png_error(png_ptr, b"Can't handle exceedingly large BMP dimensions\x00" as *const u8 as *const i8);
}

// Set the PNG image type.
png_set_IHDR(&mut *png_ptr, &mut *info_ptr, width, height, bit_depth, color_type, 0, 0, 0);

if pixdepth > 8 {
    let mut i = 0;
    while i < 4 {
        bmp_process_mask(rgba_mask[i], &mut rgba_sig[i], &mut rgba_shift[i]);
        i += 1;
    }
    
    if rgba_sig[0] == 0 || rgba_sig[1] == 0 || rgba_sig[2] == 0 {
        png_error(png_ptr, b"Invalid color mask in BMP file\x00" as *const u8 as *const i8);
    }
    
    if rgba_sig[0] != 8 || rgba_sig[1] != 8 || rgba_sig[2] != 8 || (rgba_sig[3] != 0 && rgba_sig[3] != 8) {
        sig_bit.red = rgba_sig[0];
        sig_bit.green = rgba_sig[1];
        sig_bit.blue = rgba_sig[2];
        sig_bit.alpha = rgba_sig[3];
        png_set_sBIT(&mut *png_ptr, &mut *info_ptr, &sig_bit);
    }
}

// Read the color palette (if applicable).
if palnum > 0 {
    let mut i = 0;
    while i < palnum {
        if fread(rgbq.as_mut_ptr() as *mut std::os::raw::c_void, palsize as u64, 1, stream) != 1 {
            break;
        }
        palette[i as usize].red = rgbq[2];
        palette[i as usize].green = rgbq[1];
        palette[i as usize].blue = rgbq[0];
        i += 1;
    }
    png_set_PLTE(&mut *png_ptr, &mut *info_ptr, palette.as_mut_ptr() as *const png_color_struct, i as i32);
    if i != palnum {
        png_error(png_ptr, b"Error reading color palette in BMP file\x00" as *const u8 as *const i8);
    }
}

// Allocate memory and read the image data.
row_pointers = pngx_malloc_rows_extended(&mut *png_ptr, &mut *info_ptr, rowsize as u64, -1);

    let (begin_row, end_row) = if topdown != 0 {
    (row_pointers, row_pointers.add(height as usize))
} else {
    (row_pointers.add(height as usize - 1), row_pointers.add(0))
};

if skip > 0 {
    let skip_bytes = skip as i64;
    unsafe { fseek(stream, skip_bytes, 1) };
}

y = unsafe { bmp_read_rows(begin_row, end_row, rowsize as u64, compression, stream) };

// Postprocess the image data, even if it has not been read entirely.
if pixdepth > 8 {
    unsafe {
        bmp_to_png_rows(row_pointers, width, height, pixdepth, rgba_sig.as_mut_ptr(), rgba_shift.as_mut_ptr());
    }
}

// Check the result.
if y != height as u64 {
    unsafe {
        png_error(png_ptr, b"Error reading BMP file\x00" as *const u8 as *const i8);
    }
}

1

    /* one image has been successfully read */
}


extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    static mut GIFError:
           Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()>;
    #[no_mangle]
    static mut GIFWarning:
           Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()>;
    #[no_mangle]
    fn memcmp(_: * const core::ffi::c_void, _: * const core::ffi::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
}
pub use crate::src::gifread::gifread::GIFDestroyExtension;
pub use crate::src::gifread::gifread::GIFDestroyImage;
pub use crate::src::gifread::gifread::GIFGetColorTable;
pub use crate::src::gifread::gifread::GIFGetGraphicCtl;
pub use crate::src::gifread::gifread::GIFInitExtension;
pub use crate::src::gifread::gifread::GIFInitImage;
pub use crate::src::gifread::gifread::GIFReadNextBlock;
pub use crate::src::gifread::gifread::GIFReadScreen;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::pngset::png_set_PLTE;
pub use crate::src::libpng::pngset::png_set_tRNS;
pub use crate::src::pngxtern::pngxmem::pngx_malloc_rows;
pub use crate::src::pngxtern::pngxset::pngx_set_interlace_type;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
pub type png_byte = std::os::raw::c_uchar;
pub type png_uint_16 = std::os::raw::c_ushort;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_bytep = * mut std::os::raw::c_uchar;
pub type png_const_bytep = * const std::os::raw::c_uchar;
pub type png_const_charp = * const std::os::raw::c_char;
pub type png_bytepp = * mut * mut std::os::raw::c_uchar;
pub type png_const_charpp = * mut * const std::os::raw::c_char;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::png::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color_struct;
pub type png_const_colorp = * const crate::src::libpng::png::png_color_struct;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::png::png_color_16_struct;
pub type png_color_16 = crate::src::libpng::png::png_color_16_struct;
pub type png_const_color_16p = * const crate::src::libpng::png::png_color_16_struct;
// #[derive(Copy, Clone)]

pub type GIFScreen = crate::src::gifread::gifread::GIFScreen;
// #[derive(Copy, Clone)]

pub type GIFImage = crate::src::gifread::gifread::GIFImage;
// #[derive(Copy, Clone)]

pub type GIFExtension = crate::src::gifread::gifread::GIFExtension;
// #[derive(Copy, Clone)]

pub type GIFGraphicCtlExt = crate::src::gifread::gifread::GIFGraphicCtlExt;
/*
 * pngxrgif.c - libpng external I/O: GIF reader.
 * Copyright (C) 2003-2012 Cosmin Truta.
 */
static mut gif_fmt_name: [std::os::raw::c_char; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_gif_fmt_name() {
gif_fmt_name = unsafe {
        *core::intrinsics::transmute::<&'_ [u8; 4], &'_ [i8; 4]>(b"GIF\x00")
    };}//;
static mut gif_fmt_long_name: [std::os::raw::c_char; 28] =
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_gif_fmt_long_name() {
gif_fmt_long_name = unsafe {
        *core::intrinsics::transmute::<&'_ [u8; 28], &'_ [i8; 28]>(b"Graphics Interchange Format\x00")
    };}//;
static mut gif_sig_gif87a: [std::os::raw::c_uchar; 6] =
    [0,0,0,0,0,0,]; unsafe fn laertes_init_gif_sig_gif87a() {
gif_sig_gif87a = [0x47 as std::os::raw::c_int as png_byte, 0x49 as std::os::raw::c_int as png_byte,
     0x46 as std::os::raw::c_int as png_byte, 0x38 as std::os::raw::c_int as png_byte,
     0x37 as std::os::raw::c_int as png_byte, 0x61 as std::os::raw::c_int as png_byte];}//;
/* "GIF87a" */
static mut gif_sig_gif89a: [std::os::raw::c_uchar; 6] =
    [0,0,0,0,0,0,]; unsafe fn laertes_init_gif_sig_gif89a() {
gif_sig_gif89a = [0x47 as std::os::raw::c_int as png_byte, 0x49 as std::os::raw::c_int as png_byte,
     0x46 as std::os::raw::c_int as png_byte, 0x38 as std::os::raw::c_int as png_byte,
     0x39 as std::os::raw::c_int as png_byte, 0x61 as std::os::raw::c_int as png_byte];}//;
/* "GIF89a" */
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_gif<'a1, 'a2>(mut sig: * mut std::os::raw::c_uchar,
                                         mut sig_size: std::os::raw::c_ulong,
                                         mut fmt_name_ptr: Option<&'a1 mut * const std::os::raw::c_char>,
                                         mut fmt_long_name_ptr:
                                             Option<&'a2 mut * const std::os::raw::c_char>)
 -> std::os::raw::c_int {
    /* Require at least the GIF signature and the screen descriptor. */
    if sig_size < (6 as std::os::raw::c_int + 7 as std::os::raw::c_int) as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    } /* insufficient data */
    if memcmp(sig as *const std::os::raw::c_void,
              gif_sig_gif87a.as_ptr() as *const std::os::raw::c_void,
              6 as std::os::raw::c_int as std::os::raw::c_ulong) != 0 as std::os::raw::c_int &&
           memcmp(sig as *const std::os::raw::c_void,
                  gif_sig_gif89a.as_ptr() as *const std::os::raw::c_void,
                  6 as std::os::raw::c_int as std::os::raw::c_ulong) != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    } /* not GIF */
    /* Store the format name. */
    if !borrow(& fmt_name_ptr).is_none() { *(borrow_mut(&mut fmt_name_ptr)).unwrap() = gif_fmt_name.as_ptr() }
    if !borrow(& fmt_long_name_ptr).is_none() {
        *(borrow_mut(&mut fmt_long_name_ptr)).unwrap() = gif_fmt_long_name.as_ptr()
    }
    return 1 as std::os::raw::c_int;
    /* GIF */
}
/* FIXME: Not thread-safe. */
static mut err_png_ptr: * mut crate::src::libpng::png::png_struct_def =
    (0 as * mut crate::src::libpng::png::png_struct_def); unsafe fn laertes_init_err_png_ptr() {
err_png_ptr = 0 as *const png_struct as *mut png_struct;}//;
static mut err_gif_image_ptr: * mut crate::src::gifread::gifread::GIFImage =
    (0 as * mut crate::src::gifread::gifread::GIFImage); unsafe fn laertes_init_err_gif_image_ptr() {
err_gif_image_ptr = (0 as * mut crate::src::gifread::gifread::GIFImage);}//;
static mut err_gif_ext_ptr: * mut crate::src::gifread::gifread::GIFExtension =
    (0 as * mut crate::src::gifread::gifread::GIFExtension); unsafe fn laertes_init_err_gif_ext_ptr() {
err_gif_ext_ptr = (0 as * mut crate::src::gifread::gifread::GIFExtension);}//;
unsafe extern "C" fn pngx_gif_error(mut msg: * const std::os::raw::c_char) {
    if !err_gif_image_ptr.is_null() { GIFDestroyImage(err_gif_image_ptr); }
    if !err_gif_ext_ptr.is_null() { GIFDestroyExtension(err_gif_ext_ptr); }
    png_error(err_png_ptr as *const png_struct, msg);
}
unsafe extern "C" fn pngx_gif_warning(mut msg: * const std::os::raw::c_char) {
    png_warning(err_png_ptr as *const png_struct, msg);
}
unsafe extern "C" fn pngx_set_gif_palette(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                          mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                          mut color_table: * mut std::os::raw::c_uchar,
                                          mut num_colors: std::os::raw::c_uint) {
    let mut palette: [crate::src::libpng::png::png_color_struct; 256] =
        [png_color{red: 0, green: 0, blue: 0,}; 256];
    let mut i: u32 = 0;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < num_colors {
        palette[i as usize].red =
            *color_table.offset((3 as std::os::raw::c_int as
                                     std::os::raw::c_uint).wrapping_mul(i) as isize);
        palette[i as usize].green =
            *color_table.offset((3 as std::os::raw::c_int as
                                     std::os::raw::c_uint).wrapping_mul(i).wrapping_add(1
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_uint)
                                    as isize);
        palette[i as usize].blue =
            *color_table.offset((3 as std::os::raw::c_int as
                                     std::os::raw::c_uint).wrapping_mul(i).wrapping_add(2
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_uint)
                                    as isize);
        i = i.wrapping_add(1)
    }
    png_set_PLTE(png_ptr, info_ptr, palette.as_mut_ptr() as png_const_colorp,
                 num_colors as std::os::raw::c_int);
}
unsafe extern "C" fn pngx_set_gif_transparent(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                              mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                              mut transparent: std::os::raw::c_uint) {
    let mut trans: [u8; 256] = [0; 256];
    let mut i: u32 = 0;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < transparent {
        trans[i as usize] = 255 as std::os::raw::c_int as png_byte;
        i = i.wrapping_add(1)
    }
    trans[transparent as usize] = 0 as std::os::raw::c_int as png_byte;
    png_set_tRNS(png_ptr, info_ptr, trans.as_mut_ptr() as png_const_bytep,
                 transparent as std::os::raw::c_int + 1 as std::os::raw::c_int,
                 Option::<&'_ crate::src::libpng::png::png_color_16_struct>::None);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_gif(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                       mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int {
    /* GIF-specific data */
    let mut screen: crate::src::gifread::gifread::GIFScreen =
        GIFScreen{Width: 0,
                  Height: 0,
                  GlobalColorFlag: 0,
                  ColorResolution: 0,
                  SortFlag: 0,
                  GlobalNumColors: 0,
                  Background: 0,
                  PixelAspectRatio: 0,
                  GlobalColorTable: [0; 768],};
    let mut image: crate::src::gifread::gifread::GIFImage =
        GIFImage{Screen: (0 as * mut crate::src::gifread::gifread::GIFScreen),
                 LeftPos: 0,
                 TopPos: 0,
                 Width: 0,
                 Height: 0,
                 LocalColorFlag: 0,
                 InterlaceFlag: 0,
                 SortFlag: 0,
                 LocalNumColors: 0,
                 LocalColorTable: [0; 768],
                 Rows: 0 as *mut *mut std::os::raw::c_uchar,};
    let mut ext: crate::src::gifread::gifread::GIFExtension =
        GIFExtension{Screen: (0 as * mut crate::src::gifread::gifread::GIFScreen),
                     Buffer: 0 as *mut std::os::raw::c_uchar,
                     BufferSize: 0,
                     Label: 0,};
    let mut graphicExt: crate::src::gifread::gifread::GIFGraphicCtlExt =
        GIFGraphicCtlExt{DisposalMethod: 0,
                         InputFlag: 0,
                         TransparentFlag: 0,
                         DelayTime: 0,
                         Transparent: 0,};
    let mut blockCode: i32 = 0;
    let mut colorTable: * mut u8 = 0 as *mut std::os::raw::c_uchar;
    let mut numColors: u32 = 0;
    let mut transparent: u32 = 0;
    let mut numImages: u32 = 0;
    /* PNG-specific data */
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut row_pointers: * mut * mut u8 = 0 as *mut *mut png_byte;
    /* Set up the custom error handling. */
    GIFError =
        Some(pngx_gif_error);
    GIFWarning =
        Some(pngx_gif_warning);
    err_png_ptr = png_ptr;
    err_gif_image_ptr = (0 as * mut crate::src::gifread::gifread::GIFImage);
    err_gif_ext_ptr = (0 as * mut crate::src::gifread::gifread::GIFExtension);
    /* Read the GIF screen. */
    GIFReadScreen(Some(&mut screen), stream);
    width = screen.Width;
    height = screen.Height;
    /* Set the PNG image type. */
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 8 as std::os::raw::c_int, 2 as std::os::raw::c_int | 1 as std::os::raw::c_int,
                 0 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    /* Allocate memory. */
    row_pointers =
        pngx_malloc_rows(png_ptr, info_ptr, screen.Background as std::os::raw::c_int);
    /* Complete the initialization of the GIF reader. */
    GIFInitImage(Some(&mut image), &mut screen, row_pointers);
    err_gif_image_ptr = &mut image;
    GIFInitExtension(Some(&mut ext), &mut screen,
                     256 as std::os::raw::c_int as std::os::raw::c_uint);
    err_gif_ext_ptr = &mut ext;
    numImages = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    transparent = -(1 as std::os::raw::c_int) as std::os::raw::c_uint;
    loop 
         /* Iterate over the GIF file. */
         {
        blockCode = GIFReadNextBlock(&mut image, Some(&mut ext), stream);
        if blockCode == 0x2c as std::os::raw::c_int {
            /* ',' */
            if !image.Rows.is_null() {
                /* Complete the PNG info. */
                if image.InterlaceFlag != 0 {
                    pngx_set_interlace_type(png_ptr, info_ptr,
                                            1 as std::os::raw::c_int);
                }
                GIFGetColorTable(Some(&mut colorTable), Some(&mut numColors), &mut image);
                pngx_set_gif_palette(png_ptr, info_ptr, colorTable,
                                     numColors);
                if transparent < 256 as std::os::raw::c_int as std::os::raw::c_uint {
                    pngx_set_gif_transparent(png_ptr, info_ptr, transparent);
                }
                /* Inform the GIF routines not to read the upcoming images. */
                image.Rows = 0 as *mut *mut std::os::raw::c_uchar
            }
            numImages = numImages.wrapping_add(1)
        } else if blockCode == 0x21 as std::os::raw::c_int {
            /* '!' */
            if ext.Label as std::os::raw::c_int == 0xf9 as std::os::raw::c_int {
                GIFGetGraphicCtl(Some(&mut graphicExt), Some(&mut ext));
                if !image.Rows.is_null() && graphicExt.TransparentFlag != 0 {
                    if transparent >= 256 as std::os::raw::c_int as std::os::raw::c_uint {
                        transparent = graphicExt.Transparent
                    }
                }
            }
        } else if blockCode == 0x3b as std::os::raw::c_int { break ; }
    }
    if !image.Rows.is_null() {
        png_error(png_ptr as *const png_struct,
                  b"No image in GIF file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    GIFDestroyImage(&mut image);
    GIFDestroyExtension(&mut ext);
    return numImages as std::os::raw::c_int;
}
use crate::laertes_rt::*;

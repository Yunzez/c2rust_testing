
extern "C" {
    
    
    
    
    
    
    
}
pub use crate::src::zlib::deflate::deflate;
pub use crate::src::zlib::deflate::deflateEnd;
pub use crate::src::zlib::deflate::deflateInit_;
pub use crate::src::zlib::deflate::internal_state;
pub type Byte = std::os::raw::c_uchar;
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type Bytef = std::os::raw::c_uchar;
pub type uLongf = std::os::raw::c_ulong;
pub type voidpf = * mut core::ffi::c_void;
pub type alloc_func<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: std::os::raw::c_uint,_: std::os::raw::c_uint,) -> Option<&'a2 mut core::ffi::c_void>>;
pub type free_func<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut core::ffi::c_void>,) -> ()>;
// #[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s<'a> {
    pub next_in: Option<&'a mut std::os::raw::c_uchar>,
    pub avail_in: std::os::raw::c_uint,
    pub total_in: std::os::raw::c_ulong,
    pub next_out: Option<&'a mut std::os::raw::c_uchar>,
    pub avail_out: std::os::raw::c_uint,
    pub total_out: std::os::raw::c_ulong,
    pub msg: Option<&'a mut std::os::raw::c_char>,
    pub state: Option<&'a mut crate::src::zlib::deflate::internal_state>,
    pub zalloc: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_uint,_: std::os::raw::c_uint,) -> * mut core::ffi::c_void>,
    pub zfree: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut core::ffi::c_void,) -> ()>,
    pub opaque: Option<&'a mut core::ffi::c_void>,
    pub data_type: std::os::raw::c_int,
    pub adler: std::os::raw::c_ulong,
    pub reserved: std::os::raw::c_ulong,
}
impl<'a> z_stream_s<'a> {
    pub const fn new() -> Self {
        z_stream_s {
        next_in: None,
        avail_in: 0,
        total_in: 0,
        next_out: None,
        avail_out: 0,
        total_out: 0,
        msg: None,
        state: None,
        zalloc: None,
        zfree: None,
        opaque: None,
        data_type: 0,
        adler: 0,
        reserved: 0
        }
    }
}

impl<'a> std::default::Default for z_stream_s<'a> {
    fn default() -> Self { z_stream_s::new() }
}

pub type z_stream = crate::src::libpng::png::z_stream_s;
pub type z_streamp = * mut crate::src::libpng::png::z_stream_s;
/* compress.c -- compress a memory buffer
 * Copyright (C) 1995-2005, 2014, 2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* ===========================================================================
     Compresses the source buffer into the destination buffer. The level
   parameter has the same meaning as in deflateInit.  sourceLen is the byte
   length of the source buffer. Upon entry, destLen is the total size of the
   destination buffer, which must be at least 0.1% larger than sourceLen plus
   12 bytes. Upon exit, destLen is the actual size of the compressed buffer.

     compress2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_BUF_ERROR if there was not enough room in the output buffer,
   Z_STREAM_ERROR if the level parameter is invalid.
*/
#[no_mangle]
pub unsafe extern "C" fn compress2(mut dest: * mut std::os::raw::c_uchar,
                                   mut destLen: * mut std::os::raw::c_ulong,
                                   mut source: * const std::os::raw::c_uchar,
                                   mut sourceLen: std::os::raw::c_ulong,
                                   mut level: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut stream: crate::src::libpng::png::z_stream_s =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,};
    let mut err: i32 = 0;
    let max: u32 = -(1 as std::os::raw::c_int) as uInt;
    let mut left: u64 = 0;
    left = *destLen;
    *destLen = 0 as std::os::raw::c_int as uLongf;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = 0 as voidpf;
    err =
        deflateInit_(&mut stream, level,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int { return err }
    stream.next_out = dest;
    stream.avail_out = 0 as std::os::raw::c_int as uInt;
    stream.next_in = source as *mut Bytef;
    stream.avail_in = 0 as std::os::raw::c_int as uInt;
    loop  {
        if stream.avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            stream.avail_out =
                if left > max as uLong { max } else { left as uInt };
            left =
                (left as
                     std::os::raw::c_ulong).wrapping_sub(stream.avail_out as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        if stream.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            stream.avail_in =
                if sourceLen > max as uLong {
                    max
                } else { sourceLen as uInt };
            sourceLen =
                (sourceLen as
                     std::os::raw::c_ulong).wrapping_sub(stream.avail_in as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        err =
            deflate(&mut stream,
                    if sourceLen != 0 {
                        0 as std::os::raw::c_int
                    } else { 4 as std::os::raw::c_int });
        if !(err == 0 as std::os::raw::c_int) { break ; }
    }
    *destLen = stream.total_out;
    deflateEnd(&mut stream);
    return if err == 1 as std::os::raw::c_int { 0 as std::os::raw::c_int } else { err };
}
/* ===========================================================================
 */
#[no_mangle]
pub unsafe extern "C" fn compress(mut dest: * mut std::os::raw::c_uchar,
                                  mut destLen: * mut std::os::raw::c_ulong,
                                  mut source: * const std::os::raw::c_uchar,
                                  mut sourceLen: std::os::raw::c_ulong) -> std::os::raw::c_int {
    return compress2(dest, destLen, source, sourceLen, -(1 as std::os::raw::c_int));
}
/* ===========================================================================
     If the default memLevel or windowBits for deflateInit() is changed, then
   this function needs to be updated.
 */
#[no_mangle]
pub extern "C" fn compressBound(mut sourceLen: std::os::raw::c_ulong) -> std::os::raw::c_ulong {
    return sourceLen.wrapping_add(sourceLen >>
                                      12 as
                                          std::os::raw::c_int).wrapping_add(sourceLen
                                                                        >>
                                                                        14 as
                                                                            std::os::raw::c_int).wrapping_add(sourceLen
                                                                                                          >>
                                                                                                          25
                                                                                                              as
                                                                                                              std::os::raw::c_int).wrapping_add(13
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_int
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_ulong);
}
use crate::laertes_rt::*;

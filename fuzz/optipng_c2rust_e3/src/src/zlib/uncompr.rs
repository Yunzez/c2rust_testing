
extern "C" {
    
    
    
    
    
    
    
}
pub use crate::src::zlib::inflate::inflate;
pub use crate::src::zlib::inflate::inflateEnd;
pub use crate::src::zlib::inflate::inflateInit_;
pub use crate::src::zlib::deflate::internal_state;
pub type Byte = crate::src::libpng::png::Byte;
pub type uInt = crate::src::libpng::png::uInt;
pub type uLong = crate::src::libpng::png::uLong;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type uLongf = crate::src::zlib::compress::uLongf;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type free_func = crate::src::libpng::png::free_func;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::zlib::compress::z_stream_s;
pub type z_stream = crate::src::libpng::png::z_stream;
pub type z_streamp = crate::src::libpng::png::z_streamp;
/* uncompr.c -- decompress a memory buffer
 * Copyright (C) 1995-2003, 2010, 2014, 2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* ===========================================================================
     Decompresses the source buffer into the destination buffer.  *sourceLen is
   the byte length of the source buffer. Upon entry, *destLen is the total size
   of the destination buffer, which must be large enough to hold the entire
   uncompressed data. (The size of the uncompressed data must have been saved
   previously by the compressor and transmitted to the decompressor by some
   mechanism outside the scope of this compression library.) Upon exit,
   *destLen is the size of the decompressed data and *sourceLen is the number
   of source bytes consumed. Upon return, source + *sourceLen points to the
   first unused input byte.

     uncompress returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_BUF_ERROR if there was not enough room in the output buffer, or
   Z_DATA_ERROR if the input data was corrupted, including if the input data is
   an incomplete zlib stream.
*/
#[no_mangle]
pub unsafe extern "C" fn uncompress2(mut dest: *mut Bytef,
                                     mut destLen: *mut uLongf,
                                     mut source: *const Bytef,
                                     mut sourceLen: *mut uLong)
 -> std::os::raw::c_int {
    let mut stream: z_stream =
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
                 reserved:
                     0,}; /* for detection of incomplete stream when *destLen == 0 */
    let mut err: std::os::raw::c_int = 0;
    let max: uInt = -(1 as std::os::raw::c_int) as uInt;
    let mut len: uLong = 0;
    let mut left: uLong = 0;
    let mut buf: [Byte; 1] = [0; 1];
    len = *sourceLen;
    if *destLen != 0 {
        left = *destLen;
        *destLen = 0 as std::os::raw::c_int as uLongf
    } else { left = 1 as std::os::raw::c_int as uLong; dest = buf.as_mut_ptr() }
    stream.next_in = source as *mut Bytef;
    stream.avail_in = 0 as std::os::raw::c_int as uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = 0 as voidpf;
    err =
        inflateInit_(&mut stream,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int { return err }
    stream.next_out = dest;
    stream.avail_out = 0 as std::os::raw::c_int as uInt;
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
                if len > max as uLong { max } else { len as uInt };
            len =
                (len as
                     std::os::raw::c_ulong).wrapping_sub(stream.avail_in as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        err = inflate(&mut stream, 0 as std::os::raw::c_int);
        if !(err == 0 as std::os::raw::c_int) { break ; }
    }
    *sourceLen =
        (*sourceLen as
             std::os::raw::c_ulong).wrapping_sub(len.wrapping_add(stream.avail_in as
                                                              std::os::raw::c_ulong))
            as uLong as uLong;
    if dest != buf.as_mut_ptr() {
        *destLen = stream.total_out
    } else if stream.total_out != 0 && err == -(5 as std::os::raw::c_int) {
        left = 1 as std::os::raw::c_int as uLong
    }
    inflateEnd(&mut stream);
    return if err == 1 as std::os::raw::c_int {
               0 as std::os::raw::c_int
           } else if err == 2 as std::os::raw::c_int {
               -(3 as std::os::raw::c_int)
           } else if err == -(5 as std::os::raw::c_int) &&
                         left.wrapping_add(stream.avail_out as std::os::raw::c_ulong)
                             != 0 {
               -(3 as std::os::raw::c_int)
           } else { err };
}
#[no_mangle]
pub unsafe extern "C" fn uncompress(mut dest: *mut Bytef,
                                    mut destLen: *mut uLongf,
                                    mut source: *const Bytef,
                                    mut sourceLen: uLong) -> std::os::raw::c_int {
    return uncompress2(dest, destLen, source, &mut sourceLen);
}

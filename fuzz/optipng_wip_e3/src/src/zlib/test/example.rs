







use std::ffi::CString;

use std::ffi::CStr;

use std::convert::TryInto;

extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    static mut stderr: *mut FILE;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn free(__ptr: *mut std::os::raw::c_void);
    
    fn exit(_: std::os::raw::c_int) -> !;
}
pub use crate::src::zlib::compress::compress;
pub use crate::src::zlib::deflate::deflate;
pub use crate::src::zlib::deflate::deflateEnd;
pub use crate::src::zlib::deflate::deflateInit_;
pub use crate::src::zlib::deflate::deflateParams;
pub use crate::src::zlib::deflate::deflateSetDictionary;
pub use crate::src::zlib::inflate::inflate;
pub use crate::src::zlib::inflate::inflateEnd;
pub use crate::src::zlib::inflate::inflateInit_;
pub use crate::src::zlib::inflate::inflateSetDictionary;
pub use crate::src::zlib::inflate::inflateSync;
pub use crate::src::zlib::uncompr::uncompress;
pub use crate::src::zlib::zutil::zlibCompileFlags;
pub use crate::src::zlib::zutil::zlibVersion;
pub use crate::src::zlib::deflate::internal_state;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type Byte = crate::src::libpng::png::Byte;
pub type uInt = crate::src::libpng::png::uInt;
pub type uLong = crate::src::libpng::png::uLong;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type uLongf = crate::src::zlib::compress::uLongf;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type free_func = crate::src::libpng::png::free_func;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::pngerror::z_stream_s;
pub type z_stream = crate::src::libpng::png::z_stream;
pub type z_streamp = crate::src::libpng::png::z_streamp;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::libpng::png::_IO_FILE;
pub type _IO_lock_t = crate::src::gifread::gifread::_IO_lock_t;
pub type FILE = crate::src::gifread::gifread::FILE;
static mut hello: [std::os::raw::c_char; 14] =
    unsafe {
        *::std::mem::transmute::<&[u8; 14],
                                 &mut [std::os::raw::c_char; 14]>(b"hello, hello!\x00")
    };
/* "hello world" would be more standard, but the repeated "hello"
 * stresses the compression code better, sorry...
 */
static mut dictionary: [std::os::raw::c_char; 6] =
    unsafe {
        *::std::mem::transmute::<&[u8; 6], &[std::os::raw::c_char; 6]>(b"hello\x00")
    };
static mut dictId: uLong = 0;
/* !Z_SOLO */
static mut zalloc: alloc_func = None;
static mut zfree: free_func = None;
/* ===========================================================================
 * Test compress() and uncompress()
 */

pub fn test_compress(compr: &mut Vec<u8>, compr_len: &mut u64, uncompr: &mut Vec<u8>, uncompr_len: &mut u64) {
    static HELLO: &[u8] = b"hello, hello!\0";
    let len = HELLO.len() as u64;

    unsafe {
        let err = compress(compr.as_mut_ptr(), compr_len, HELLO.as_ptr(), len);
        if err != 0 {
            eprintln!("compress error: {}", err);
            std::process::exit(1);
        }

        uncompr.clear();
        uncompr.extend_from_slice(b"garbage\0");

        let err = uncompress(uncompr.as_mut_ptr(), uncompr_len, compr.as_ptr(), *compr_len);
        if err != 0 {
            eprintln!("uncompress error: {}", err);
            std::process::exit(1);
        }
    }

    if uncompr.as_slice() != HELLO {
        eprintln!("bad uncompress");
        std::process::exit(1);
    } else {
        println!("uncompress(): {}", String::from_utf8_lossy(uncompr));
    }
}

/* ===========================================================================
 * Test read/write of .gz files
 */

pub fn test_gzio(fname: &CStr, uncompr: &mut [u8], uncompr_len: u64) {
    eprintln!("NO_GZCOMPRESS -- gz* functions cannot compress");
}

/* Adler32 value of the dictionary */
/* Z_SOLO */
/* ===========================================================================
 * Test deflate() with small buffers
 */

pub fn test_deflate(compr: &mut [u8], compr_len: usize) {
    let mut c_stream: z_stream = z_stream {
        next_in: std::ptr::null_mut(),
        avail_in: 0,
        total_in: 0,
        next_out: compr.as_mut_ptr() as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: unsafe { zalloc },
        zfree: unsafe { zfree },
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    let hello_len = unsafe { hello.len() as u64 };
    let err = deflateInit_(&mut c_stream, -1, "1.2.11-optipng", std::mem::size_of::<z_stream>());
    if err != 0 {
        eprintln!("deflateInit error: {}", err);
        std::process::exit(1);
    }

    c_stream.next_in = unsafe { hello.as_ptr() as *mut std::os::raw::c_uchar };

    while c_stream.total_in < hello_len && c_stream.total_out < compr_len as u64 {
        c_stream.avail_out = 1;
        c_stream.avail_in = c_stream.avail_out;
        let err = unsafe { deflate(&mut c_stream, 0) };
        if err != 0 {
            eprintln!("deflate error: {}", err);
            std::process::exit(1);
        }
    }

    loop {
        c_stream.avail_out = 1;
        let err = unsafe { deflate(&mut c_stream, 4) };
        if err == 1 { break; }
        if err != 0 {
            eprintln!("deflate error: {}", err);
            std::process::exit(1);
        }
    }

    let err = unsafe { deflateEnd(&mut c_stream) };
    if err != 0 {
        eprintln!("deflateEnd error: {}", err);
        std::process::exit(1);
    }
}

/* ===========================================================================
 * Test inflate() with small buffers
 */

pub fn test_inflate(compr: &[u8], uncompr_len: usize) -> Result<Vec<u8>, String> {
    let mut d_stream = z_stream {
        next_in: compr.as_ptr() as *mut Bytef,
        avail_in: compr.len() as uInt,
        total_in: 0,
        next_out: std::ptr::null_mut(),
        avail_out: 0,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: unsafe { zalloc },
        zfree: unsafe { zfree },
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    let mut uncompr = vec![0u8; uncompr_len];
    d_stream.next_out = uncompr.as_mut_ptr() as *mut Bytef;
    d_stream.avail_out = uncompr.len() as uInt;

    let err = unsafe {
        inflateInit_(&mut d_stream, 
                      std::ffi::CStr::from_bytes_with_nul(b"1.2.11-optipng\x00").unwrap().to_str().unwrap(), 
                      std::mem::size_of::<z_stream>() as i32)
    };

    if err != 0 {
        return Err(format!("inflateInit error: {}", err));
    }

    while d_stream.total_out < uncompr_len as uLong && d_stream.total_in < compr.len() as uLong {
        d_stream.avail_out = 1;
        d_stream.avail_in = d_stream.avail_out;

        let err = unsafe { inflate(&mut d_stream, 0) };

        if err == 1 { break; }
        if err != 0 {
            return Err(format!("inflate error: {}", err));
        }
    }

    let err = unsafe { inflateEnd(&mut d_stream) };
    if err != 0 {
        return Err(format!("inflateEnd error: {}", err));
    }

    let uncompr_slice = unsafe { std::slice::from_raw_parts(uncompr.as_ptr(), d_stream.total_out as usize) };
    Ok(uncompr_slice.to_vec())
}

/* ===========================================================================
 * Test deflate() with large buffers and dynamic change of compression level
 */

pub fn test_large_deflate(compr: &mut [u8], compr_len: usize, uncompr: &[u8], uncompr_len: usize) {
    let mut c_stream: z_stream = z_stream {
        next_in: uncompr.as_ptr() as *mut Bytef,
        avail_in: uncompr_len as uInt,
        total_in: 0,
        next_out: compr.as_mut_ptr() as *mut Bytef,
        avail_out: compr_len as uInt,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: unsafe { zalloc },
        zfree: unsafe { zfree },
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    let err = unsafe {
        deflateInit_(&mut c_stream, 1, "1.2.11-optipng", std::mem::size_of::<z_stream>())
    };
    if err != 0 {
        eprintln!("deflateInit error: {}", err);
        std::process::exit(1);
    }

    let err = unsafe { deflate(&mut c_stream, 0) };
    if err != 0 {
        eprintln!("deflate error: {}", err);
        std::process::exit(1);
    }
    if c_stream.avail_in != 0 {
        eprintln!("deflate not greedy");
        std::process::exit(1);
    }

    unsafe {
        deflateParams(&mut c_stream, 0, 0);
    }
    c_stream.next_in = compr.as_mut_ptr() as *mut Bytef;
    c_stream.avail_in = (compr_len / 2) as uInt;

    let err = unsafe { deflate(&mut c_stream, 0) };
    if err != 0 {
        eprintln!("deflate error: {}", err);
        std::process::exit(1);
    }

    unsafe {
        deflateParams(&mut c_stream, 9, 1);
    }
    c_stream.next_in = uncompr.as_ptr() as *mut Bytef;
    c_stream.avail_in = uncompr_len as uInt;

    let err = unsafe { deflate(&mut c_stream, 0) };
    if err != 0 {
        eprintln!("deflate error: {}", err);
        std::process::exit(1);
    }

    let err = unsafe { deflate(&mut c_stream, 4) };
    if err != 1 {
        eprintln!("deflate should report Z_STREAM_END");
        std::process::exit(1);
    }

    let err = unsafe { deflateEnd(&mut c_stream) };
    if err != 0 {
        eprintln!("deflateEnd error: {}", err);
        std::process::exit(1);
    }
}

/* ===========================================================================
 * Test inflate() with large buffers
 */

pub fn test_large_inflate(compr: &[u8], compr_len: usize, uncompr_len: usize) {
    let mut uncompr: Vec<u8> = vec![0; uncompr_len];
    let mut d_stream: z_stream = z_stream {
        next_in: compr.as_ptr() as *mut Bytef,
        avail_in: compr_len as uInt,
        total_in: 0,
        next_out: uncompr.as_mut_ptr() as *mut Bytef,
        avail_out: uncompr_len as uInt,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: unsafe { zalloc },
        zfree: unsafe { zfree },
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    let init_result = unsafe {
        inflateInit_(&mut d_stream, "1.2.11-optipng", std::mem::size_of::<z_stream>() as std::os::raw::c_int)
    };

    if init_result != 0 {
        eprintln!("inflateInit error: {}", init_result);
        std::process::exit(1);
    }

    loop {
        let inflate_result = unsafe { inflate(&mut d_stream, 0) };
        if inflate_result == 1 {
            break;
        }
        if inflate_result != 0 {
            eprintln!("large inflate error: {}", inflate_result);
            std::process::exit(1);
        }
    }

    let end_result = unsafe { inflateEnd(&mut d_stream) };
    if end_result != 0 {
        eprintln!("inflateEnd error: {}", end_result);
        std::process::exit(1);
    }

    if d_stream.total_out != (2 * uncompr_len as u64 + compr_len as u64 / 2) {
        eprintln!("bad large inflate: {}", d_stream.total_out);
        std::process::exit(1);
    } else {
        println!("large_inflate(): OK");
    }
}

/* ===========================================================================
 * Test deflate() with full flush
 */

pub unsafe extern "C" fn test_flush(mut compr: *mut Byte,
                                    mut comprLen: *mut uLong) {
    let mut c_stream: z_stream =
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
                 reserved: 0,}; /* compression stream */
    let mut err: std::os::raw::c_int =
        0; /* force an error in first compressed block */
    let mut len: uInt =
        (strlen(hello.as_mut_ptr()) as
             uInt).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    let mut err = deflateInit_(&mut c_stream, -1, "1.2.11-optipng", std::mem::size_of::<z_stream>());
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    c_stream.next_in = hello.as_mut_ptr() as *mut std::os::raw::c_uchar;
    c_stream.next_out = compr;
    c_stream.avail_in = 3 as std::os::raw::c_int as uInt;
    c_stream.avail_out = *comprLen as uInt;
    err = deflate(&mut c_stream, 3 as std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    let ref mut fresh0 = *compr.offset(3 as std::os::raw::c_int as isize);
    *fresh0 = (*fresh0).wrapping_add(1);
    c_stream.avail_in = len.wrapping_sub(3 as std::os::raw::c_int as std::os::raw::c_uint);
    err = deflate(&mut c_stream, 4 as std::os::raw::c_int);
    if err != 1 as std::os::raw::c_int {
        if err != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                    b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
            exit(1 as std::os::raw::c_int);
        }
    }
    err = deflateEnd(&mut c_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    *comprLen = c_stream.total_out;
}
/* ===========================================================================
 * Test inflateSync()
 */

pub fn test_sync(compr: &[u8], compr_len: usize, uncompr_len: usize) -> Result<Vec<u8>, String> {
    let mut d_stream = z_stream {
        next_in: compr.as_ptr() as *mut Bytef,
        avail_in: compr_len as uInt,
        total_in: 0,
        next_out: std::ptr::null_mut(),
        avail_out: uncompr_len as uInt,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: unsafe { zalloc },
        zfree: unsafe { zfree },
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    let version = std::ffi::CStr::from_bytes_with_nul(b"1.2.11-optipng\x00").unwrap();
    let err = unsafe {
        inflateInit_(&mut d_stream, version.to_str().unwrap(), std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as std::os::raw::c_int)
    };

    if err != 0 {
        return Err(format!("inflateInit error: {}", err));
    }

    let mut uncompr = vec![0u8; uncompr_len];
    d_stream.next_out = uncompr.as_mut_ptr() as *mut Bytef;

    let err = unsafe { inflate(&mut d_stream, 0) };
    if err != 0 {
        return Err(format!("inflate error: {}", err));
    }

    d_stream.avail_in = (compr_len as uInt).wrapping_sub(2);
    let err = unsafe { inflateSync(&mut d_stream) };
    if err != 0 {
        return Err(format!("inflateSync error: {}", err));
    }

    let err = unsafe { inflate(&mut d_stream, 4) };
    if err != -3 {
        return Err("inflate should report DATA_ERROR".to_string());
    }

    let err = unsafe { inflateEnd(&mut d_stream) };
    if err != 0 {
        return Err(format!("inflateEnd error: {}", err));
    }

    Ok(uncompr)
}

/* ===========================================================================
 * Test deflate() with preset dictionary
 */

pub fn test_dict_deflate(compr: &mut [u8], compr_len: usize) {
    let mut c_stream: z_stream = z_stream {
        next_in: std::ptr::null_mut(),
        avail_in: 0,
        total_in: 0,
        next_out: compr.as_mut_ptr() as *mut Bytef,
        avail_out: compr_len as uInt,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: unsafe { zalloc },
        zfree: unsafe { zfree },
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; // compression stream

    let err = unsafe {
        deflateInit_(&mut c_stream, 9, CStr::from_bytes_with_nul(b"1.2.11-optipng\0").unwrap().to_str().unwrap(), std::mem::size_of::<z_stream>())
    };
    if err != 0 {
        eprintln!("deflateInit error: {}", err);
        std::process::exit(1);
    }

    let err = unsafe {
        deflateSetDictionary(&mut c_stream, dictionary.as_ptr() as *const Bytef, std::mem::size_of::<[std::os::raw::c_char; 6]>() as uLong as std::os::raw::c_int as uInt)
    };
    if err != 0 {
        eprintln!("deflateSetDictionary error: {}", err);
        std::process::exit(1);
    }

    unsafe {
        dictId = c_stream.adler;
        c_stream.next_in = hello.as_ptr() as *mut std::os::raw::c_uchar;
        c_stream.avail_in = (hello.len() as uInt).wrapping_add(1);
    }

    let err = unsafe { deflate(&mut c_stream, 4) };
    if err != 1 {
        eprintln!("deflate should report Z_STREAM_END");
        std::process::exit(1);
    }

    let err = unsafe { deflateEnd(&mut c_stream) };
    if err != 0 {
        eprintln!("deflateEnd error: {}", err);
        std::process::exit(1);
    }
}

/* ===========================================================================
 * Test inflate() with a preset dictionary
 */

pub fn test_dict_inflate(compr: &[u8], uncompr_len: usize) -> Result<Vec<u8>, String> {
    let mut d_stream = z_stream {
        next_in: compr.as_ptr() as *const Bytef,
        avail_in: compr.len() as uInt,
        total_in: 0,
        next_out: std::ptr::null_mut(),
        avail_out: uncompr_len as uInt,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: unsafe { zalloc },
        zfree: unsafe { zfree },
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    let mut uncompr = vec![0u8; uncompr_len];
    let uncompr_ptr = uncompr.as_mut_ptr() as *mut std::os::raw::c_char;
    unsafe {
        strcpy(uncompr_ptr, b"garbage\x00" as *const u8 as *const std::os::raw::c_char);
    }

    let err = unsafe {
        inflateInit_(&mut d_stream, "1.2.11-optipng", std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as std::os::raw::c_int)
    };

    if err != 0 {
        return Err(format!("inflateInit error: {}", err));
    }

    d_stream.next_out = uncompr.as_mut_ptr() as *mut Bytef;
    d_stream.avail_out = uncompr_len as uInt;

    loop {
        let err = unsafe { inflate(&mut d_stream, 0) };
        if err == 1 {
            break;
        }
        if err == 2 {
            let dict_id = unsafe { dictId };
            if d_stream.adler != dict_id {
                return Err("unexpected dictionary".to_string());
            }
            let err = unsafe {
                inflateSetDictionary(&mut d_stream, dictionary.as_ptr() as *const Bytef, std::mem::size_of::<[std::os::raw::c_char; 6]>() as std::os::raw::c_ulong as std::os::raw::c_int as uInt)
            };
            if err != 0 {
                return Err(format!("inflateSetDictionary error: {}", err));
            }
        }
        if err != 0 {
            return Err(format!("inflate with dict error: {}", err));
        }
    }

    let err = unsafe { inflateEnd(&mut d_stream) };
    if err != 0 {
        return Err(format!("inflateEnd error: {}", err));
    }

    if unsafe { strcmp(uncompr.as_mut_ptr() as *mut std::os::raw::c_char, hello.as_mut_ptr()) } != 0 {
        return Err("bad inflate with dict".to_string());
    } else {
        println!("inflate with dictionary: {:?}", String::from_utf8_lossy(&uncompr));
    }

    Ok(uncompr)
}

/* ===========================================================================
 * Usage:  example [output.gz  [input.gz]]
 */
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut compr: *mut Byte = 0 as *mut Byte; /* don't overflow on MSDOS */
    let mut uncompr: *mut Byte = 0 as *mut Byte;
    let mut comprLen: uLong =
        (10000 as std::os::raw::c_int as
             std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                             as std::os::raw::c_ulong);
    let mut uncomprLen: uLong = comprLen;
    static mut myVersion: *const std::os::raw::c_char =
        b"1.2.11-optipng\x00" as *const u8 as *const std::os::raw::c_char;
    if *zlibVersion().offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
           *myVersion.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int {
        fprintf(stderr,
                b"incompatible zlib version\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    } else {
        if strcmp(zlibVersion(),
                  b"1.2.11-optipng\x00" as *const u8 as *const std::os::raw::c_char)
               != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"warning: different zlib version\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
    }
    let version = "1.2.11-optipng";
let flags = zlibCompileFlags();
println!("zlib version {} = 0x{:04x}, compile flags = 0x{:x}", version, 0x12bf, flags);
    compr =
        calloc(comprLen as uInt as std::os::raw::c_ulong,
               1 as std::os::raw::c_int as std::os::raw::c_ulong) as *mut Byte;
    uncompr =
        calloc(uncomprLen as uInt as std::os::raw::c_ulong,
               1 as std::os::raw::c_int as std::os::raw::c_ulong) as *mut Byte;
    /* compr and uncompr are cleared to avoid reading uninitialized
     * data and to ensure that uncompr compresses well.
     */
    if compr.is_null() || uncompr.is_null() {
        printf(b"out of memory\n\x00" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    let mut compr_vec = vec![0u8; comprLen as usize];
let mut uncompr_vec = vec![0u8; uncomprLen as usize];
test_compress(&mut compr_vec, &mut comprLen, &mut uncompr_vec, &mut uncomprLen);
    let fname = if argc > 1 {
    CStr::from_ptr(*argv.offset(1) as *const std::os::raw::c_char)
} else {
    CStr::from_bytes_with_nul(b"foo.gz\0").unwrap()
};
let uncompr_slice = std::slice::from_raw_parts_mut(uncompr, uncomprLen as usize);
test_gzio(fname, uncompr_slice, uncomprLen);
    let mut compr_buffer = vec![0u8; comprLen as usize];
test_deflate(&mut compr_buffer, comprLen as usize);
    let uncompr_result = test_inflate(unsafe { std::slice::from_raw_parts(compr, comprLen as usize) }, uncomprLen as usize);
match uncompr_result {
    Ok(uncompr) => {
        // Handle the uncompressed data
    },
    Err(e) => {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
    let compr_slice: &mut [u8] = &mut [0; 65536]; // Define an appropriate size
let uncompr_slice: &[u8] = unsafe { std::slice::from_raw_parts(uncompr, uncomprLen as usize) };
test_large_deflate(compr_slice, comprLen as usize, uncompr_slice, uncomprLen as usize);
    let compr_slice: &[u8] = unsafe { std::slice::from_raw_parts(compr, comprLen as usize) };
test_large_inflate(compr_slice, comprLen as usize, uncomprLen as usize);
    test_flush(compr, &mut comprLen);
    let uncompr_data = match test_sync(unsafe { std::slice::from_raw_parts(compr, comprLen as usize) }, comprLen as usize, uncomprLen as usize) {
    Ok(data) => data,
    Err(e) => {
        eprintln!("{}", e);
        return 1; // or handle the error as needed
    }
};
    comprLen = uncomprLen;
    let mut compr_buffer = vec![0u8; comprLen as usize];
test_dict_deflate(&mut compr_buffer, comprLen as usize);
    let uncompr_result = test_dict_inflate(unsafe { std::slice::from_raw_parts(compr, comprLen as usize) }, uncomprLen as usize);
match uncompr_result {
    Ok(uncompr) => {
        // Handle successful uncompression
    },
    Err(e) => {
        eprintln!("{}", e);
        // Handle error
    }
}
    free(compr as *mut std::os::raw::c_void);
    free(uncompr as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let c_args: Vec<std::ffi::CString> = args.iter()
        .map(|arg| ::std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    
    let mut c_args_ptr: Vec<*mut std::os::raw::c_char> = c_args.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .collect();
    
    c_args_ptr.push(std::ptr::null_mut());
    
    unsafe {
        let result = main_0((c_args_ptr.len() - 1) as std::os::raw::c_int, c_args_ptr.as_mut_ptr());
        ::std::process::exit(result as i32);
    }
}


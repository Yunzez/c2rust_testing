






































use std::ffi::CString;

use std::os::raw::c_long;

use std::alloc;

use std::io;

use std::ffi::c_void; // Assuming this is needed for C interop

use std::io::Error;

use std::fmt::Write;

use std::eprint;

use std::os::raw::c_int;
use std::ptr;

use std::ffi; // For CStr
use std::os::raw::c_char; // For raw C character types

use std::eprintln;

use std::ffi::CStr;

use std::process;

use std::env;

extern "C" {
    
    
    static mut __stdinp: *mut FILE;
    
    static mut __stdoutp: *mut FILE;
    
    static mut __stderrp: *mut FILE;
    
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    
    fn ferror(_: *mut FILE) -> std::os::raw::c_int;
    
    fn fflush(_: *mut FILE) -> std::os::raw::c_int;
    
    fn fgetc(_: *mut FILE) -> std::os::raw::c_int;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    
    fn perror(_: *const std::os::raw::c_char);
    
    fn remove(_: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn rewind(_: *mut FILE);
    
    fn ungetc(_: std::os::raw::c_int, _: *mut FILE) -> std::os::raw::c_int;
    
    fn fdopen(_: std::os::raw::c_int, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn fileno(_: *mut FILE) -> std::os::raw::c_int;
    
    fn exit(_: std::os::raw::c_int) -> !;
    
    fn getenv(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn free(_: *mut std::os::raw::c_void);
    
    fn signal(_: std::os::raw::c_int,
              _: Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>)
     -> Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>;
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn strerror(_: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    
    fn strncpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_char;
    
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn __error() -> *mut std::os::raw::c_int;
    
    fn __maskrune(_: __darwin_ct_rune_t, _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    
    static mut _DefaultRuneLocale: _RuneLocale;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn open(_: *const std::os::raw::c_char, _: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    
    fn utime(_: *const std::os::raw::c_char, _: *const utimbuf) -> std::os::raw::c_int;
    
    fn _exit(_: std::os::raw::c_int) -> !;
    
    fn close(_: std::os::raw::c_int) -> std::os::raw::c_int;
    
    fn isatty(_: std::os::raw::c_int) -> std::os::raw::c_int;
    
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __nbyte: size_t)
     -> ssize_t;
    
    fn fchown(_: std::os::raw::c_int, _: uid_t, _: gid_t) -> std::os::raw::c_int;
    
    fn fchmod(_: std::os::raw::c_int, _: mode_t) -> std::os::raw::c_int;
    
    fn lstat(_: *const std::os::raw::c_char, _: *mut stat) -> std::os::raw::c_int;
    
    fn stat(_: *const std::os::raw::c_char, _: *mut stat) -> std::os::raw::c_int;
}
pub use crate::bzlib::BZ2_bzRead;
pub use crate::bzlib::BZ2_bzReadClose;
pub use crate::bzlib::BZ2_bzReadGetUnused;
pub use crate::bzlib::BZ2_bzReadOpen;
pub use crate::bzlib::BZ2_bzWrite;
pub use crate::bzlib::BZ2_bzWriteClose64;
pub use crate::bzlib::BZ2_bzWriteOpen;
pub use crate::bzlib::BZ2_bzlibVersion;
pub use crate::blocksort::__sFILEX;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = crate::blocksort::__int64_t;
pub type __uint64_t = std::os::raw::c_ulonglong;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_ssize_t = std::os::raw::c_long;
pub type __darwin_time_t = std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = crate::blocksort::__darwin_off_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = crate::blocksort::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::blocksort::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::blocksort::__sFILE;
pub type FILE = crate::blocksort::FILE;
pub type off_t = __darwin_off_t;
pub type ssize_t = __darwin_ssize_t;
pub type uid_t = __darwin_uid_t;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: std::os::raw::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [std::os::raw::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [std::os::raw::c_char; 8],
    pub __encoding: [std::os::raw::c_char; 32],
    pub __sgetrune: Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *const std::os::raw::c_char)
                               -> __darwin_rune_t>,
    pub __sputrune: Option<unsafe extern "C" fn(_: __darwin_rune_t,
                                                _: *mut std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *mut std::os::raw::c_char)
                               -> std::os::raw::c_int>,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut std::os::raw::c_void,
    pub __variable_len: std::os::raw::c_int,
    pub __ncharclasses: std::os::raw::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: std::os::raw::c_long,
}
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type nlink_t = __uint16_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
/* */
/* BZ_UNIX */
/* BZ_LCCWIN32 */
/*---------------------------------------------*/
/*--
  Some more stuff for all platforms :-)
--*/
pub type Char = std::os::raw::c_char;
pub type Bool = crate::blocksort::Bool;
pub type UChar = crate::blocksort::UChar;
pub type Int32 = crate::blocksort::Int32;
pub type UInt32 = crate::blocksort::UInt32;
pub type Int16 = std::os::raw::c_short;
pub type UInt16 = crate::blocksort::UInt16;
/*--
  IntNative is your platform's `native' int size.
  Only here to avoid probs with 64-bit platforms.
--*/
pub type IntNative = std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UInt64 {
    pub b: [UChar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zzzz {
    pub name: *mut Char,
    pub link: *mut zzzz,
}
/*---------------------------------------------*/
/*--
  All the garbage from here to main() is purely to
  implement a linked list of command-line arguments,
  into which main() copies argv[1 .. argc-1].

  The purpose of this exercise is to facilitate 
  the expansion of wildcard characters * and ? in 
  filenames for OSs which don't know how to do it
  themselves, like MSDOS, Windows 95 and NT.

  The actual Dirty Work is done by the platform-
  specific macro APPEND_FILESPEC.
--*/
pub type Cell = zzzz;
#[inline]
 fn isascii(c: i32) -> i32 {
    return if c & !0x7F == 0 { 1 } else { 0 };
}

#[inline]
fn __istype(c: __darwin_ct_rune_t, f: std::os::raw::c_ulong) -> std::os::raw::c_int {
    if isascii(c) != 0 {
        // Accessing mutable static is unsafe, so we need to wrap this in an unsafe block
        let runetype = unsafe { _DefaultRuneLocale.__runetype[c as usize] } as std::os::raw::c_ulong;
        return ((runetype & f) != 0) as std::os::raw::c_int;
    } else {
        // Call to unsafe function, so we need to wrap this in an unsafe block
        return unsafe { (__maskrune(c, f) != 0) as std::os::raw::c_int };
    }
}


#[inline]
#[linkage = "external"]
pub fn isspace(c: i32) -> i32 {
    __istype(c, 0x4000)
}

/*---------------------------------------------------*/
/*--- Misc (file handling) data decls             ---*/
/*---------------------------------------------------*/

pub static mut verbosity: Int32 = 0;

pub static mut keepInputFiles: Bool = 0;

pub static mut smallMode: Bool = 0;

pub static mut deleteOutputOnInterrupt: Bool = 0;

pub static mut forceOverwrite: Bool = 0;

pub static mut testFailsExist: Bool = 0;

pub static mut unzFailsExist: Bool = 0;

pub static mut noisy: Bool = 0;

pub static mut numFileNames: Int32 = 0;

pub static mut numFilesProcessed: Int32 = 0;

pub static mut blockSize100k: Int32 = 0;

pub static mut exitValue: Int32 = 0;

pub static mut opMode: Int32 = 0;

pub static mut srcMode: Int32 = 0;

pub static mut longestFileName: Int32 = 0;

pub static mut inName: [Char; 1034] = [0; 1034];

pub static mut outName: [Char; 1034] = [0; 1034];

pub static mut tmpName: [Char; 1034] = [0; 1034];

pub static mut progName: *mut Char = 0 as *const Char as *mut Char;

pub static mut progNameReally: [Char; 1034] = [0; 1034];

pub static mut outputHandleJustInCase: *mut FILE =
    0 as *const FILE as *mut FILE;

pub static mut workFactor: Int32 = 0;
fn uInt64_from_UInt32s(n: &mut UInt64, lo32: UInt32, hi32: UInt32) {
    n.b[7] = (hi32 >> 24 & 0xff) as UChar;
    n.b[6] = (hi32 >> 16 & 0xff) as UChar;
    n.b[5] = (hi32 >> 8 & 0xff) as UChar;
    n.b[4] = (hi32 & 0xff) as UChar;
    n.b[3] = (lo32 >> 24 & 0xff) as UChar;
    n.b[2] = (lo32 >> 16 & 0xff) as UChar;
    n.b[1] = (lo32 >> 8 & 0xff) as UChar;
    n.b[0] = (lo32 & 0xff) as UChar;
}

fn uInt64_to_double(n: &UInt64) -> f64 {
    let mut base: f64 = 1.0;
    let mut sum: f64 = 0.0;
    
    for i in 0..8 {
        sum += base * n.b[i as usize] as f64;
        base *= 256.0;
    }
    
    sum
}

fn uInt64_isZero(n: &UInt64) -> u8 {
    for &byte in &n.b {
        if byte != 0 {
            return 0;
        }
    }
    1
}

/* Divide *n by 10, and return the remainder.  */
fn uInt64_qrm10(n: &mut UInt64) -> Int32 {
    let mut rem: UInt32 = 0;
    let mut tmp: UInt32;
    let mut i: Int32 = 7;

    while i >= 0 {
        tmp = rem.wrapping_mul(256).wrapping_add(n.b[i as usize] as UInt32);
        n.b[i as usize] = (tmp.wrapping_div(10)) as UChar;
        rem = tmp.wrapping_rem(10);
        i -= 1;
    }
    rem as Int32
}

/* ... and the Whole Entire Point of all this UInt64 stuff is
   so that we can supply the following function.
*/
fn uInt64_toAscii(outbuf: &mut [i8], n: &mut UInt64) {
    let mut i: i32 = 0;
    let mut q: i32 = 0;
    let mut buf: [u8; 32] = [0; 32];
    let mut nBuf: i32 = 0;
    let mut n_copy: UInt64 = *n;

    loop {
        q = uInt64_qrm10(&mut n_copy);
        buf[nBuf as usize] = (q + '0' as i32) as u8;
        nBuf += 1;
        if unsafe { uInt64_isZero(&mut n_copy) } != 0 {
            break;
        }
    }

    outbuf[nBuf as usize] = 0; // Null-terminate the string

    for i in 0..nBuf {
        outbuf[i as usize] = buf[(nBuf - i - 1) as usize] as i8;
    }
}

/*---------------------------------------------------*/
/*--- Processing of complete files and streams    ---*/
/*---------------------------------------------------*/
/*---------------------------------------------*/
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c: Int32 = fgetc(f);
    if c == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int as Bool }
    ungetc(c, f);
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
unsafe extern "C" fn compressStream(mut stream: *mut FILE,
                                    mut zStream: *mut FILE) {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut ibuf: [UChar; 5000] = [0; 5000];
    let mut nIbuf: Int32 = 0;
    let mut nbytes_in_lo32: UInt32 = 0;
    let mut nbytes_in_hi32: UInt32 = 0;
    let mut nbytes_out_lo32: UInt32 = 0;
    let mut nbytes_out_hi32: UInt32 = 0;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            let mut bzerr: i32 = 0;
let bzf = BZ2_bzWriteOpen(&mut bzerr, zStream, blockSize100k, verbosity, workFactor);
if bzerr != 0 {
    current_block = 660242869387099075;
} else {
     if verbosity >= 2 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"\n\x00" as *const u8 as *const std::os::raw::c_char);
                }
                loop  {
                    if !(1 as std::os::raw::c_int as Bool != 0) {
                        current_block = 13242334135786603907;
                        break ;
                    }
                    if myfeof(stream) != 0 {
                        current_block = 13242334135786603907;
                        break ;
                    }
                    nIbuf =
                        fread(ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                              ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                              5000 as std::os::raw::c_int as std::os::raw::c_ulong, stream) as
                            Int32;
                    if ferror(stream) != 0 {
                        current_block = 18225113528933273530;
                        break ;
                    }
                    if nIbuf > 0 as std::os::raw::c_int {
                        BZ2_bzWrite(&mut bzerr, bzf,
                                    ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                                    nIbuf);
                    }
                    if bzerr != 0 as std::os::raw::c_int {
                        current_block = 660242869387099075;
                        break ;
                    }
                }
 
     match current_block {
    18225113528933273530 => { }
    660242869387099075 => { }
    _ => {
        BZ2_bzWriteClose64(&mut bzerr, bzf, 0, 
                           &mut nbytes_in_lo32, 
                           &mut nbytes_in_hi32, 
                           &mut nbytes_out_lo32, 
                           &mut nbytes_out_hi32);
        if bzerr != 0 {
            current_block = 660242869387099075;
        } else if ferror(zStream) != 0 {
            current_block = 18225113528933273530;
        } else {
            ret = fflush(zStream);
            if ret == -1 {
                current_block = 18225113528933273530;
            } else {
                if zStream != __stdoutp {
                    let fd = fileno(zStream);
                    if fd < 0 {
                        current_block = 18225113528933273530;
                    } else {
                        applySavedFileAttrToOutputFile(fd);
                        ret = fclose(zStream);
                        outputHandleJustInCase = std::ptr::null_mut();
                        if ret == -1 {
                            current_block = 18225113528933273530;
                        } else {
                            current_block = 17281240262373992796;
                        }
                    }
                } else {
                    current_block = 17281240262373992796;
                }
                match current_block {
                    18225113528933273530 => { }
                    _ => {
                        outputHandleJustInCase = std::ptr::null_mut();
                        if ferror(stream) != 0 {
                            current_block = 18225113528933273530;
                        } else {
                            ret = fclose(stream);
                            if ret == -1 {
                                current_block = 18225113528933273530;
                            } else {
                                if verbosity >= 1 {
                                    if nbytes_in_lo32 == 0 && nbytes_in_hi32 == 0 {
                                        eprintln!(" no data compressed.");
                                    } else {
                                        let mut buf_nin = [0i8; 32];
                                        let mut buf_nout = [0i8; 32];
                                        let mut nbytes_in = UInt64 { b: [0; 8] };
                                        let mut nbytes_out = UInt64 { b: [0; 8] };
                                        let mut nbytes_in_d: f64 = 0.0;
                                        let mut nbytes_out_d: f64 = 0.0;
                                        uInt64_from_UInt32s(&mut nbytes_in, nbytes_in_lo32, nbytes_in_hi32);
                                        uInt64_from_UInt32s(&mut nbytes_out, nbytes_out_lo32, nbytes_out_hi32);
                                        nbytes_in_d = uInt64_to_double(&mut nbytes_in);
                                        nbytes_out_d = uInt64_to_double(&mut nbytes_out);
                                        uInt64_toAscii(&mut buf_nin, &mut nbytes_in);
                                        uInt64_toAscii(&mut buf_nout, &mut nbytes_out);
                                        eprintln!("{:6.3}:1, {:6.3} bits/byte, {:5.2}% saved, {} in, {} out.",
                                                  nbytes_in_d / nbytes_out_d,
                                                  8.0 * nbytes_out_d / nbytes_in_d,
                                                  100.0 * (1.0 - nbytes_out_d / nbytes_in_d),
                                                  std::ffi::CStr::from_ptr(buf_nin.as_ptr()).to_string_lossy(),
                                                  std::ffi::CStr::from_ptr(buf_nout.as_ptr()).to_string_lossy());
                                    }
                                }
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

 
}
match current_block {
    18225113528933273530 => {}
    _ => {
        BZ2_bzWriteClose64(&mut bzerr_dummy, bzf, 1, &mut nbytes_in_lo32, &mut nbytes_in_hi32, &mut nbytes_out_lo32, &mut nbytes_out_hi32);
        match bzerr {
            -9 => {
                current_block = 5122324059762049690;
                match current_block {
                    4323399205346619401 => {
                        panic("compress:unexpected error");
                    }
                    2380987886157893679 => {
                        outOfMemory();
                    }
                    _ => {
                        configError();
                    }
                }
            }
            -3 => {
                current_block = 2380987886157893679;
                match current_block {
                    4323399205346619401 => {
                        panic("compress:unexpected error");
                    }
                    2380987886157893679 => {
                        outOfMemory();
                    }
                    _ => {
                        configError();
                    }
                }
            }
            -6 => {}
            _ => {
                current_block = 4323399205346619401;
                match current_block {
                    4323399205346619401 => {
                        panic("compress:unexpected error");
                    }
                    2380987886157893679 => {
                        outOfMemory();
                    }
                    _ => {
                        configError();
                    }
                }
            }
        }
    }
}

        }
    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------*/
fn uncompressStream(zStream: *mut FILE, stream: *mut FILE) -> Bool {
    unsafe {
         let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut nread: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut UChar = 0 as *mut UChar;
    nUnused = 0 as std::os::raw::c_int;
    streamNo = 0 as std::os::raw::c_int;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            let mut current_block: u64 = 0; // Initialize current_block to avoid uninitialized error
loop {
    let mut bzf = BZ2_bzReadOpen(&mut bzerr, zStream, verbosity, smallMode as i32, unused.as_mut_ptr().cast(), nUnused);
    if bzf.is_null() || bzerr != 0 {
        break;
    }
    streamNo += 1;

    while bzerr == 0 {
        nread = BZ2_bzRead(&mut bzerr, bzf, obuf.as_mut_ptr().cast(), 5000);
        if bzerr == -5 {
            current_block = 18063049917807660484; // Set current_block before breaking
            break;
        }
        if (bzerr == 0 || bzerr == 4) && nread > 0 {
            fwrite(obuf.as_ptr().cast(), std::mem::size_of::<UChar>() as u64, nread as u64, stream);
        }
        if ferror(stream) != 0 {
            current_block = 5049394217699438129; // Set current_block before breaking
            break;
        }
    }

    if bzerr != 4 {
        current_block = 673979509383251364; // Set current_block before breaking
        break;
    }

    BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV, &mut nUnused);
    if bzerr != 0 {
        panic("decompress:bzReadGetUnused");
    }

    let unused_slice = unsafe { std::slice::from_raw_parts(unusedTmpV as *const u8, nUnused as usize) };
    unused.copy_from_slice(unused_slice);

    BZ2_bzReadClose(&mut bzerr, Some(Box::from_raw(bzf)));
    if bzerr != 0 {
        panic("decompress:bzReadGetUnused");
    }

    if nUnused == 0 && myfeof(zStream) != 0 {
        current_block = 926243229934402080; // Set current_block before breaking
        break;
    }
}
match current_block {
    5049394217699438129 => {}
    _ => {
         match current_block {
                        18063049917807660484 => {
                            if forceOverwrite != 0 {
                                rewind(zStream);
                                loop  {
                                    if !(1 as std::os::raw::c_int as Bool != 0) {
                                        current_block = 926243229934402080;
                                        break ;
                                    }
                                    if myfeof(zStream) != 0 {
                                        current_block = 926243229934402080;
                                        break ;
                                    }
                                    nread =
                                        fread(obuf.as_mut_ptr() as
                                                  *mut std::os::raw::c_void,
                                              ::std::mem::size_of::<UChar>()
                                                  as std::os::raw::c_ulong,
                                              5000 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong, zStream) as
                                            Int32;
                                    if ferror(zStream) != 0 {
                                        current_block = 5049394217699438129;
                                        break ;
                                    }
                                    if nread > 0 as std::os::raw::c_int {
                                        fwrite(obuf.as_mut_ptr() as
                                                   *const std::os::raw::c_void,
                                               ::std::mem::size_of::<UChar>()
                                                   as std::os::raw::c_ulong,
                                               nread as std::os::raw::c_ulong,
                                               stream);
                                    }
                                    if ferror(stream) != 0 {
                                        current_block = 5049394217699438129;
                                        break ;
                                    }
                                }
                            } else { current_block = 673979509383251364; }
                        }
                        _ => { }
                    }
                    match current_block {
                        5049394217699438129 => { }
                        _ => {
                            match current_block {
                                673979509383251364 => {
                                    /*
The variables live at this point are:
(mut zStream: Box<blocksort::__sFILE>, mut stream: Box<blocksort::__sFILE>, mut current_block: u64, mut bzf: *mut std::ffi::c_void, mut bzerr: i32, mut bzerr_dummy: i32, mut streamNo: i32)
*/
let result = BZ2_bzReadClose(&mut bzerr_dummy, Some(Box::from_raw(bzf as *mut std::ffi::c_void)));
match bzerr {
    -9 => {
         current_block = 3642457097893642164;
match current_block {
    6455255476181645667 => {
        panic!("decompress: unexpected error");
    }
    3642457097893642164 => {
        configError();
    }
    10766414566319669440 => {
        crcError();
    }
    16178635849926953562 => {
        outOfMemory();
    }
    5517467152645906530 => {
        compressedStreamEOF();
    }
    _ => {
        if !zStream.is_null() && zStream != __stdinp {
            unsafe { fclose(zStream); }
        }
        if !stream.is_null() && stream != __stdoutp {
            unsafe { fclose(stream); }
        }
        if streamNo == 1 {
            return 0; // Assuming Bool is a type alias for u8
        } else {
            if noisy != 0 {
                eprintln!(
                    "{}: {}: trailing garbage after EOF ignored",
                    unsafe { CStr::from_ptr(progName).to_string_lossy() },
                    unsafe { CStr::from_ptr(inName.as_mut_ptr()).to_string_lossy() }
                );
            }
            return 1; // Assuming Bool is a type alias for u8
        }
    }
}


    }
    -6 => { }
    -4 => {
         current_block = 10766414566319669440;
match current_block {
    6455255476181645667 => {
        panic!("decompress: unexpected error");
    }
    3642457097893642164 => {
        configError();
    }
    10766414566319669440 => {
        crcError();
    }
    16178635849926953562 => {
        outOfMemory();
    }
    5517467152645906530 => {
        compressedStreamEOF();
    }
    _ => {
        if zStream != __stdinp {
            unsafe { fclose(zStream); }
        }
        if stream != __stdoutp {
            unsafe { fclose(stream); }
        }
        if streamNo == 1 {
            return 0; // Assuming Bool is a type alias for u8
        } else {
            if noisy != 0 {
                eprintln!(
                    "{}: {}: trailing garbage after EOF ignored",
                    unsafe { std::ffi::CStr::from_ptr(progName).to_string_lossy() },
                    unsafe { std::ffi::CStr::from_ptr(inName.as_mut_ptr()).to_string_lossy() }
                );
            }
            return 1; // Assuming Bool is a type alias for u8
        }
    }
}


    }
    -3 => {
         current_block = 16178635849926953562;
match current_block {
    6455255476181645667 => {
        panic!("decompress: unexpected error");
    }
    3642457097893642164 => {
        configError();
    }
    10766414566319669440 => {
        crcError();
    }
    16178635849926953562 => {
        outOfMemory();
    }
    5517467152645906530 => {
        compressedStreamEOF();
    }
    _ => {
        if zStream != __stdinp {
            fclose(zStream);
        }
        if stream != __stdoutp {
            fclose(stream);
        }
        if streamNo == 1 {
            return 0; // Assuming Bool is represented as u8
        } else {
            if noisy != 0 {
                eprintln!(
                    "{}: {}: trailing garbage after EOF ignored",
                    unsafe { std::ffi::CStr::from_ptr(progName).to_string_lossy() },
                    unsafe { std::ffi::CStr::from_ptr(inName.as_mut_ptr()).to_string_lossy() }
                );
            }
            return 1; // Assuming Bool is represented as u8
        }
    }
}


    }
    -7 => {
         let current_block: u64 = 5517467152645906530;

match current_block {
    6455255476181645667 => {
        panic!("decompress: unexpected error");
    }
    3642457097893642164 => {
        configError();
    }
    10766414566319669440 => {
        crcError();
    }
    16178635849926953562 => {
        outOfMemory();
    }
    5517467152645906530 => {
        compressedStreamEOF();
    }
    _ => {
        if !zStream.is_null() && zStream != __stdinp {
            unsafe { fclose(zStream); }
        }
        if !stream.is_null() && stream != __stdoutp {
            unsafe { fclose(stream); }
        }
        if streamNo == 1 {
            return 0; // Assuming Bool is a type alias for u8
        } else {
            if noisy != 0 {
                eprintln!(
                    "{}: {}: trailing garbage after EOF ignored",
                    unsafe { CStr::from_ptr(progName).to_string_lossy() }, // Convert C string to Rust string
                    unsafe { CStr::from_ptr(inName.as_mut_ptr()).to_string_lossy() } // Convert C string to Rust string
                );
            }
            return 1; // Assuming Bool is a type alias for u8
        }
    }
}


    }
    -5 => {
         current_block = 7372986856480808103;
match current_block {
    6455255476181645667 => {
        panic!("decompress: unexpected error");
    }
    3642457097893642164 => {
        configError();
    }
    10766414566319669440 => {
        crcError();
    }
    16178635849926953562 => {
        outOfMemory();
    }
    5517467152645906530 => {
        compressedStreamEOF();
    }
    _ => {
        if !zStream.is_null() && zStream != __stdinp {
            unsafe { fclose(zStream); }
        }
        if !stream.is_null() && stream != __stdoutp {
            unsafe { fclose(stream); }
        }
        if streamNo == 1 {
            return 0; // Assuming Bool is a type alias for i32
        } else {
            if noisy != 0 {
                eprintln!(
                    "{}: {}: trailing garbage after EOF ignored",
                    unsafe { std::ffi::CStr::from_ptr(progName).to_string_lossy() },
                    unsafe { std::ffi::CStr::from_ptr(inName.as_mut_ptr()).to_string_lossy() }
                );
            }
            return 1; // Assuming Bool is a type alias for i32
        }
    }
}


    }
    _ => {
         let current_block: u64 = 6455255476181645667;

match current_block {
    6455255476181645667 => {
        panic!("decompress: unexpected error");
    }
    3642457097893642164 => {
        configError();
    }
    10766414566319669440 => {
        crcError();
    }
    16178635849926953562 => {
        outOfMemory();
    }
    5517467152645906530 => {
        compressedStreamEOF();
    }
    _ => {
        if !zStream.is_null() && zStream != __stdinp {
            unsafe { fclose(zStream); }
        }
        if !stream.is_null() && stream != __stdoutp {
            unsafe { fclose(stream); }
        }
        if streamNo == 1 {
            return 0; // Assuming Bool is a type alias for u8
        } else {
            if noisy != 0 {
                eprintln!(
                    "{}: {}: trailing garbage after EOF ignored",
                    unsafe { std::ffi::CStr::from_ptr(progName).to_string_lossy() },
                    unsafe { std::ffi::CStr::from_ptr(inName.as_mut_ptr()).to_string_lossy() }
                );
            }
            return 1; // Assuming Bool is a type alias for u8
        }
    }
}


    }
}
/*
The variables live at this point are:
(mut zStream: Box<blocksort::__sFILE>, mut stream: Box<blocksort::__sFILE>, mut current_block: u64, mut bzf: *mut std::ffi::c_void, mut bzerr: i32, mut bzerr_dummy: i32, mut streamNo: i32, mut bzerr_dummy: i32)
*/

                                }
                                _ => {
                                    if ferror(zStream) == 0 {
    if stream != __stdoutp {
        let fd = fileno(stream);
        if fd < 0 {
            // Handle error case
        } else {
            applySavedFileAttrToOutputFile(fd);
        }
    }

    let ret = fclose(zStream);
    if ret != -1 {
        if ferror(stream) == 0 {
            let ret = fflush(stream);
            if ret == 0 {
                if stream != __stdoutp {
                    let ret = fclose(stream);
                    outputHandleJustInCase = std::ptr::null_mut(); // Reset to null pointer
                    if ret == -1 {
                        // Handle error case
                    }
                }
                outputHandleJustInCase = std::ptr::null_mut(); // Reset to null pointer
                if verbosity >= 2 {
                    eprintln!("\n    ");
                }
                return 1; // Assuming return type is i32
            }
        }
    }
}

                                }
                            }
                        }
                    }

    }
}

        }
    }
    ioError();
    /*notreached*/
    }
}

/*---------------------------------------------*/
unsafe extern "C" fn testStream(mut zStream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut UChar = 0 as *mut UChar;
    nUnused = 0 as std::os::raw::c_int;
    streamNo = 0 as std::os::raw::c_int;
    if !(ferror(zStream) != 0) {
        loop {
    if !true {
        current_block = 15125582407903384992;
        break;
    }
    let bzf = BZ2_bzReadOpen(&mut bzerr, zStream, verbosity,
                             smallMode as i32,
                             unused.as_mut_ptr() as *mut std::ffi::c_void,
                             nUnused);
    if bzf.is_null() || bzerr != 0 {
        current_block = 10905486111603547446;
        break;
    }
    streamNo += 1;
    while bzerr == 0 {
        BZ2_bzRead(&mut bzerr, bzf,
                   obuf.as_mut_ptr() as *mut std::ffi::c_void,
                   5000);
        if bzerr == -5 {
            current_block = 10905486111603547446;
            break;
        }
    }
    if bzerr != 4 {
        current_block = 10905486111603547446;
        break;
    }
    BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV,
                        &mut nUnused);
    if bzerr != 0 {
        panic!("test:bzReadGetUnused");
    }
    let unused_slice = unsafe { std::slice::from_raw_parts(unusedTmpV as *const u8, nUnused as usize) };
    unused[..nUnused as usize].copy_from_slice(unused_slice);
    BZ2_bzReadClose(&mut bzerr, Some(Box::from_raw(bzf)));
    if bzerr != 0 {
        panic!("test:bzReadGetUnused");
    }
    if nUnused == 0 && myfeof(zStream) != 0 {
        current_block = 15125582407903384992;
        break;
    }
}
match current_block {
    15125582407903384992 => {
         if ferror(zStream) == 0 {
    ret = fclose(zStream);
    if ret != -1 {
        if verbosity >= 2 {
            eprintln!("\n    ");
        }
        return 1;
    }
}


    }
    _ => {
         let mut bzerr_dummy = 0;
BZ2_bzReadClose(&mut bzerr_dummy, Some(Box::new(bzf)));
                if verbosity == 0 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"%s: %s: \x00" as *const u8 as
                                *const std::os::raw::c_char, progName,
                            inName.as_mut_ptr());
                }
                match bzerr {
                    -9 => {
                        match current_block {
    6101827300316655396 => {
        panic!("test: unexpected error");
    }
    10380742613918245393 => {
        eprintln!("file ends unexpectedly");
        return 0; // Assuming Bool is a type alias for a boolean
    }
    2539039579982765382 => {
        if zStream != __stdinp {
            unsafe { fclose(zStream); } // Using unsafe to call the C function
        }
        if streamNo == 1 {
            eprintln!("bad magic number (file not created by bzip2)");
            return 0; // Assuming Bool is a type alias for a boolean
        } else {
            if noisy != 0 {
                eprintln!("trailing garbage after EOF ignored");
            }
            return 1; // Assuming Bool is a type alias for a boolean
        }
    }
    18238374633732057650 => {
        configError();
    }
    13802719682174684861 => {
        outOfMemory();
    }
    _ => {
        eprintln!("data integrity (CRC) error in data");
        return 0; // Assuming Bool is a type alias for a boolean
    }
}

                    }
                    -6 => { }
                    -4 => {
                        current_block = 11224962462315262049;
match current_block {
    6101827300316655396 => {
        panic!("test: unexpected error");
    }
    10380742613918245393 => {
        eprintln!("file ends unexpectedly");
        return 0; // Assuming 0 is equivalent to false
    }
    2539039579982765382 => {
        if zStream != __stdinp { 
            unsafe { fclose(zStream); } // Using unsafe to call C function
        }
        if streamNo == 1 {
            eprintln!("bad magic number (file not created by bzip2)");
            return 0; // Assuming 0 is equivalent to false
        } else {
            if noisy != 0 {
                eprintln!("trailing garbage after EOF ignored");
            }
            return 1; // Assuming 1 is equivalent to true
        }
    }
    18238374633732057650 => { configError(); }
    13802719682174684861 => { outOfMemory(); }
    _ => {
        eprintln!("data integrity (CRC) error in data");
        return 0; // Assuming 0 is equivalent to false
    }
}

                    }
                    -3 => {
                        current_block = 13802719682174684861;
match current_block {
    6101827300316655396 => {
        panic!("test: unexpected error");
    }
    10380742613918245393 => {
        eprintln!("file ends unexpectedly");
        return 0; // Assuming Bool is a type alias for u8
    }
    2539039579982765382 => {
        if zStream != std::ptr::null_mut() { // Assuming zStream is a raw pointer
            unsafe { fclose(zStream); } // Using unsafe block for C function call
        }
        if streamNo == 1 {
            eprintln!("bad magic number (file not created by bzip2)");
            return 0; // Assuming Bool is a type alias for u8
        } else {
            if noisy != 0 {
                eprintln!("trailing garbage after EOF ignored");
            }
            return 1; // Assuming Bool is a type alias for u8
        }
    }
    18238374633732057650 => { configError(); }
    13802719682174684861 => { outOfMemory(); }
    _ => {
        eprintln!("data integrity (CRC) error in data");
        return 0; // Assuming Bool is a type alias for u8
    }
}

                    }
                    -7 => {
                        current_block = 10380742613918245393;
match current_block {
    6101827300316655396 => {
        panic!("test: unexpected error");
    }
    10380742613918245393 => {
        eprintln!("file ends unexpectedly");
        return 0; // Assuming Bool is a type alias for u8
    }
    2539039579982765382 => {
        if !zStream.is_null() && zStream != __stdinp { 
            unsafe { fclose(zStream); } // Using unsafe to call C function
        }
        if streamNo == 1 {
            eprintln!("bad magic number (file not created by bzip2)");
            return 0; // Assuming Bool is a type alias for u8
        } else {
            if noisy != 0 {
                eprintln!("trailing garbage after EOF ignored");
            }
            return 1; // Assuming Bool is a type alias for u8
        }
    }
    18238374633732057650 => { configError(); }
    13802719682174684861 => { outOfMemory(); }
    _ => {
        eprintln!("data integrity (CRC) error in data");
        return 0; // Assuming Bool is a type alias for u8
    }
}

                    }
                    -5 => {
                        current_block = 2539039579982765382;
match current_block {
    6101827300316655396 => {
        panic!("test: unexpected error");
    }
    10380742613918245393 => {
        eprintln!("file ends unexpectedly");
        return 0; // Assuming Bool is a type alias for u8
    }
    2539039579982765382 => {
        if zStream != __stdinp { 
            unsafe { fclose(zStream); } // Using unsafe to call the C function
        }
        if streamNo == 1 {
            eprintln!("bad magic number (file not created by bzip2)");
            return 0; // Assuming Bool is a type alias for u8
        } else {
            if noisy != 0 {
                eprintln!("trailing garbage after EOF ignored");
            }
            return 1; // Assuming Bool is a type alias for u8
        }
    }
    18238374633732057650 => { configError(); }
    13802719682174684861 => { outOfMemory(); }
    _ => {
        eprintln!("data integrity (CRC) error in data");
        return 0; // Assuming Bool is a type alias for u8
    }
}

                    }
                    _ => {
                        current_block = 6101827300316655396;
match current_block {
    6101827300316655396 => {
        panic!("test: unexpected error");
    }
    10380742613918245393 => {
        eprintln!("file ends unexpectedly");
        return 0; // Assuming 0 represents false in the original context
    }
    2539039579982765382 => {
        if zStream != __stdinp { 
            unsafe { fclose(zStream); } // Using unsafe to call C function
        }
        if streamNo == 1 {
            eprintln!("bad magic number (file not created by bzip2)");
            return 0; // Assuming 0 represents false in the original context
        } else {
            if noisy != 0 {
                eprintln!("trailing garbage after EOF ignored");
            }
            return 1; // Assuming 1 represents true in the original context
        }
    }
    18238374633732057650 => { configError(); }
    13802719682174684861 => { outOfMemory(); }
    _ => {
        eprintln!("data integrity (CRC) error in data");
        return 0; // Assuming 0 represents false in the original context
    }
}

                    }
                }

    }
}

    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------------*/
/*--- Error [non-] handling grunge                ---*/
/*---------------------------------------------------*/
/*---------------------------------------------*/
fn setExit(v: i32) {
    if v > unsafe { exitValue } {
        unsafe { exitValue = v };
    }
}

/*---------------------------------------------*/
fn cadvise() {
    // Assuming `noisy` is a mutable static variable, we need to access it in an unsafe block.
    unsafe {
        if noisy != 0 {
            eprintln!("\nIt is possible that the compressed file(s) have become corrupted.\nYou can use the -tvv option to test integrity of such files.\n\nYou can use the `bzip2recover` program to attempt to recover\ndata from undamaged sections of corrupted files.\n");
        }
    }
}

/*---------------------------------------------*/
fn showFileNames() {
    if unsafe { noisy } != 0 {
        let in_name = unsafe { std::ffi::CStr::from_ptr(inName.as_ptr()) };
        let out_name = unsafe { std::ffi::CStr::from_ptr(outName.as_ptr()) };
        eprintln!("\tInput file = {}, output file = {}", in_name.to_string_lossy(), out_name.to_string_lossy());
    }
}

/*---------------------------------------------*/
unsafe extern "C" fn cleanUpAndFail(mut ec: Int32) -> ! {
    let mut retVal: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    if srcMode == 3 as std::os::raw::c_int && opMode != 3 as std::os::raw::c_int &&
           deleteOutputOnInterrupt as std::os::raw::c_int != 0 {
        /* Check whether input file still exists.  Delete output file
         only if input exists to avoid loss of data.  Joerg Prante, 5
         January 2002.  (JRS 06-Jan-2002: other changes in 1.0.2 mean
         this is less likely to happen.  But to be ultra-paranoid, we
         do the check anyway.)  */
        retVal = stat(inName.as_mut_ptr(), &mut statBuf);
        if retVal == 0 as std::os::raw::c_int {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"%s: Deleting output file %s, if it exists.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr());
            }
            if !outputHandleJustInCase.is_null() {
                fclose(outputHandleJustInCase);
            }
            retVal = remove(outName.as_mut_ptr());
            if retVal != 0 as std::os::raw::c_int {
                fprintf(__stderrp,
                        b"%s: WARNING: deletion of output file (apparently) failed.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
            }
        } else {
            fprintf(__stderrp,
                    b"%s: WARNING: deletion of output file suppressed\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
            fprintf(__stderrp,
                    b"%s:    since input file no longer exists.  Output file\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
            fprintf(__stderrp,
                    b"%s:    `%s\' may be incomplete.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, outName.as_mut_ptr());
            fprintf(__stderrp,
                    b"%s:    I suggest doing an integrity test (bzip2 -tv) of it.\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
        }
    }
    if noisy as std::os::raw::c_int != 0 && numFileNames > 0 as std::os::raw::c_int &&
           numFilesProcessed < numFileNames {
        fprintf(__stderrp,
                b"%s: WARNING: some files have not been processed:\n%s:    %d specified on command line, %d not processed yet.\n\n\x00"
                    as *const u8 as *const std::os::raw::c_char, progName, progName,
                numFileNames, numFileNames - numFilesProcessed);
    }
    setExit(ec);
    exit(exitValue);
}
/*---------------------------------------------*/
fn panic(s: &str) -> ! {
    eprintln!("\n{}: PANIC -- internal consistency error:\n\t{}\n\tThis is a BUG.  Please report it to:\n\tbzip2-devel@sourceware.org", unsafe { CStr::from_ptr(progName).to_string_lossy() }, s);
    showFileNames();
    unsafe { cleanUpAndFail(3); }
}

/*---------------------------------------------*/
fn crcError() -> ! {
    eprintln!("\n{}: Data integrity error when decompressing.", unsafe { CStr::from_ptr(progName).to_string_lossy() });
    showFileNames();
    cadvise();
    unsafe { cleanUpAndFail(2); }
}

/*---------------------------------------------*/
fn compressedStreamEOF() -> ! {
    unsafe {
        if noisy != 0 {
            eprintln!("\n{}: Compressed file ends unexpectedly;\n\tperhaps it is corrupted?  *Possible* reason follows.", CStr::from_ptr(progName).to_string_lossy());
            eprintln!("{}", CStr::from_ptr(progName).to_string_lossy());
            showFileNames();
            cadvise();
        }
        cleanUpAndFail(2);
    }
}

/*---------------------------------------------*/
fn ioError() -> ! {
    eprintln!("\n{}: I/O or other error, bailing out. Possible reason follows.", unsafe { std::ffi::CStr::from_ptr(progName).to_string_lossy() });
    std::process::exit(1);
}

/*---------------------------------------------*/
unsafe extern "C" fn mySignalCatcher(n: i32) {
    eprintln!("\n{}: Control-C or similar caught, quitting.", unsafe { CStr::from_ptr(progName).to_string_lossy() });
    cleanUpAndFail(1);
}

/*---------------------------------------------*/
unsafe extern "C" fn mySIGSEGVorSIGBUScatcher(mut n: IntNative) {
    let mut msg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if opMode == 1 as std::os::raw::c_int {
        msg =
            b": Caught a SIGSEGV or SIGBUS whilst compressing.\n\n   Possible causes are (most likely first):\n   (1) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (2) A bug in the compiler used to create this executable\n       (unlikely, if you didn\'t compile bzip2 yourself.)\n   (3) A real bug in bzip2 -- I hope this should never be the case.\n   The user\'s manual, Section 4.3, has more info on (1) and (2).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (1)\n   or (2), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user\'s manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don\'t\n   have the manual or can\'t be bothered to read it, mail me anyway.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char
    } else {
        msg =
            b": Caught a SIGSEGV or SIGBUS whilst decompressing.\n\n   Possible causes are (most likely first):\n   (1) The compressed data is corrupted, and bzip2\'s usual checks\n       failed to detect this.  Try bzip2 -tvv my_file.bz2.\n   (2) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (3) A bug in the compiler used to create this executable\n       (unlikely, if you didn\'t compile bzip2 yourself.)\n   (4) A real bug in bzip2 -- I hope this should never be the case.\n   The user\'s manual, Section 4.3, has more info on (2) and (3).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (2)\n   or (3), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user\'s manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don\'t\n   have the manual or can\'t be bothered to read it, mail me anyway.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char
    }
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    write(2 as std::os::raw::c_int, progName as *const std::os::raw::c_void,
          strlen(progName));
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    msg = b"\tInput file = \x00" as *const u8 as *const std::os::raw::c_char;
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    write(2 as std::os::raw::c_int, inName.as_mut_ptr() as *const std::os::raw::c_void,
          strlen(inName.as_mut_ptr()));
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    msg = b"\tOutput file = \x00" as *const u8 as *const std::os::raw::c_char;
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    write(2 as std::os::raw::c_int, outName.as_mut_ptr() as *const std::os::raw::c_void,
          strlen(outName.as_mut_ptr()));
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    /* Don't call cleanupAndFail. If we ended up here something went
      terribly wrong. Trying to clean up might fail spectacularly. */
    if opMode == 1 as std::os::raw::c_int {
        setExit(3);
    } else { setExit(2); }
    _exit(exitValue);
}
/*---------------------------------------------*/
fn outOfMemory() -> ! {
    eprintln!("\n{}: couldn't allocate enough memory", unsafe { CStr::from_ptr(progName).to_string_lossy() });
    showFileNames();
    unsafe {
        cleanUpAndFail(1);
    }
}

/*---------------------------------------------*/
fn configError() -> ! {
    eprintln!("bzip2: I'm not configured correctly for this platform!");
    eprintln!("\tI require Int32, Int16 and Char to have sizes");
    eprintln!("\tof 4, 2 and 1 bytes to run properly, and they don't.");
    eprintln!("\tProbably you can fix this by defining them correctly,");
    eprintln!("\tand recompiling.  Bye!");
    
    unsafe {
        exitValue = 3;
        process::exit(exitValue);
    }
}

/*---------------------------------------------------*/
/*--- The main driver machinery                   ---*/
/*---------------------------------------------------*/
/* All rather crufty.  The main problem is that input files
   are stat()d multiple times before use.  This should be
   cleaned up. 
*/
/*---------------------------------------------*/
fn pad(s: &mut [i8; 1034], longest_file_name: i32) {
    let current_length = s.iter().take_while(|&&c| c != 0).count() as i32;
    if current_length >= longest_file_name { return; }
    
    let padding_needed = longest_file_name - current_length;
    for _ in 0..padding_needed {
        eprint!(" ");
    }
}

/*---------------------------------------------*/
unsafe extern "C" fn copyFileName(mut to: *mut Char, mut from: *mut Char) {
    if strlen(from) >
           (1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as std::os::raw::c_ulong {
        fprintf(__stderrp,
                b"bzip2: file name\n`%s\'\nis suspiciously (more than %d chars) long.\nTry using a reasonable file name instead.  Sorry! :-)\n\x00"
                    as *const u8 as *const std::os::raw::c_char, from,
                1034 as std::os::raw::c_int - 10 as std::os::raw::c_int);
        setExit(1);
        exit(exitValue);
    }
    strncpy(to, from,
            (1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as std::os::raw::c_ulong);
    *to.offset((1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as isize) =
        '\u{0}' as i32 as Char;
}
/*---------------------------------------------*/
unsafe extern "C" fn fileExists(mut name: *mut Char) -> Bool {
    let mut tmp: *mut FILE =
        fopen(name, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    let mut exists: Bool =
        (tmp != 0 as *mut std::os::raw::c_void as *mut FILE) as std::os::raw::c_int as Bool;
    if !tmp.is_null() { fclose(tmp); }
    return exists;
}
/*---------------------------------------------*/
/* Open an output file safely with O_EXCL and good permissions.
   This avoids a race condition in versions < 1.0.2, in which
   the file was first opened and then had its interim permissions
   set safely.  We instead use open() to create the file with
   the interim permissions required. (--- --- rw-).

   For non-Unix platforms, if we are not worrying about
   security issues, simple this simply behaves like fopen.
*/
unsafe extern "C" fn fopen_output_safely(mut name: *mut Char,
                                         mut mode: *const std::os::raw::c_char)
 -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fh: IntNative = 0;
    fh =
        open(name,
             0x1 as std::os::raw::c_int | 0x200 as std::os::raw::c_int | 0x800 as std::os::raw::c_int,
             0o200 as std::os::raw::c_int | 0o400 as std::os::raw::c_int);
    if fh == -(1 as std::os::raw::c_int) { return 0 as *mut FILE }
    fp = fdopen(fh, mode);
    if fp.is_null() { close(fh); }
    return fp;
}
/*---------------------------------------------*/
/*--
  if in doubt, return True
--*/
unsafe extern "C" fn notAStandardFile(mut name: *mut Char) -> Bool {
    let mut i: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    i = lstat(name, &mut statBuf);
    if i != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int as Bool }
    if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
           0o100000 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as Bool
    }
    return 1 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
/*--
  rac 11/21/98 see if file has hard links to it
--*/
unsafe extern "C" fn countHardLinks(mut name: *mut Char) -> Int32 {
    let mut i: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    i = lstat(name, &mut statBuf);
    if i != 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return statBuf.st_nlink as std::os::raw::c_int - 1 as std::os::raw::c_int;
}
/*---------------------------------------------*/
/* Copy modification date, access date, permissions and owner from the
   source to destination file.  We have to copy this meta-info off
   into fileMetaInfo before starting to compress / decompress it,
   because doing it afterwards means we get the wrong access time.

   To complicate matters, in compress() and decompress() below, the
   sequence of tests preceding the call to saveInputFileMetaInfo()
   involves calling fileExists(), which in turn establishes its result
   by attempting to fopen() the file, and if successful, immediately
   fclose()ing it again.  So we have to assume that the fopen() call
   does not cause the access time field to be updated.

   Reading of the man page for stat() (man 2 stat) on RedHat 7.2 seems
   to imply that merely doing open() will not affect the access time.
   Therefore we merely need to hope that the C library only does
   open() as a result of fopen(), and not any kind of read()-ahead
   cleverness.

   It sounds pretty fragile to me.  Whether this carries across
   robustly to arbitrary Unix-like platforms (or even works robustly
   on this one, RedHat 7.2) is unknown to me.  Nevertheless ...  
*/
static mut fileMetaInfo: stat =
    stat{st_dev: 0,
         st_mode: 0,
         st_nlink: 0,
         st_ino: 0,
         st_uid: 0,
         st_gid: 0,
         st_rdev: 0,
         st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_size: 0,
         st_blocks: 0,
         st_blksize: 0,
         st_flags: 0,
         st_gen: 0,
         st_lspare: 0,
         st_qspare: [0; 2],};
fn saveInputFileMetaInfo(src_name: *const i8) {
    let mut file_meta_info: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_birthtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };

    unsafe {
        let ret_val = stat(src_name, &mut file_meta_info);
        if ret_val != 0 {
            ioError();
        }
    }
}

fn applySavedTimeInfoToOutputFile(dst_name: &std::ffi::CStr) -> Result<(), std::io::Error> {
    let mut u_tim_buf = utimbuf {
        actime: unsafe { fileMetaInfo.st_atimespec.tv_sec },
        modtime: unsafe { fileMetaInfo.st_mtimespec.tv_sec },
    };

    let ret_val = unsafe { utime(dst_name.as_ptr(), &mut u_tim_buf) };
    if ret_val != 0 {
        return Err(ioError());
    }
    Ok(())
}

fn applySavedFileAttrToOutputFile(fd: i32) {
    let permissions = {
        let file_meta_info = unsafe { &fileMetaInfo };
        std::os::unix::fs::PermissionsExt::from_mode(file_meta_info.st_mode as u32)
    };
    
    let ret_val = std::fs::set_permissions(fd.to_string(), permissions);
    if let Err(_) = ret_val {
        ioError();
    }

    // Using a command line approach for chown as a workaround
    let _ = {
        let file_meta_info = unsafe { &fileMetaInfo };
        std::process::Command::new("chown")
            .arg(format!("{}:{}", file_meta_info.st_uid, file_meta_info.st_gid))
            .arg(fd.to_string())
            .output()
    };
    /* chown() will in many cases return with EPERM, which can
      be safely ignored.
   */
}

/*---------------------------------------------*/
 fn containsDubiousChars(name: *mut i8) -> bool {
    // On unix, files can contain any characters and the file expansion
    // is performed by the shell.
    return false;
    // ! BZ_UNIX
    // BZ_UNIX
}


pub static mut zSuffix: [*const Char; 4] =
    [b".bz2\x00" as *const u8 as *const std::os::raw::c_char,
     b".bz\x00" as *const u8 as *const std::os::raw::c_char,
     b".tbz2\x00" as *const u8 as *const std::os::raw::c_char,
     b".tbz\x00" as *const u8 as *const std::os::raw::c_char];

pub static mut unzSuffix: [*const Char; 4] =
    [b"\x00" as *const u8 as *const std::os::raw::c_char,
     b"\x00" as *const u8 as *const std::os::raw::c_char,
     b".tar\x00" as *const u8 as *const std::os::raw::c_char,
     b".tar\x00" as *const u8 as *const std::os::raw::c_char];
unsafe extern "C" fn hasSuffix(mut s: *mut Char, mut suffix: *const Char)
 -> Bool {
    let mut ns: Int32 = strlen(s) as Int32;
    let mut nx: Int32 = strlen(suffix) as Int32;
    if ns < nx { return 0 as std::os::raw::c_int as Bool }
    if strcmp(s.offset(ns as isize).offset(-(nx as isize)), suffix) ==
           0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int as Bool
    }
    return 0 as std::os::raw::c_int as Bool;
}
unsafe extern "C" fn mapSuffix(mut name: *mut Char,
                               mut oldSuffix: *const Char,
                               mut newSuffix: *const Char) -> Bool {
    if hasSuffix(name, oldSuffix) == 0 { return 0 as std::os::raw::c_int as Bool }
    *name.offset(strlen(name).wrapping_sub(strlen(oldSuffix)) as isize) =
        0 as std::os::raw::c_int as Char;
    strcat(name, newSuffix);
    return 1 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
fn compress(name: *mut c_char) {
   unsafe {
        let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic("compress: bad modes\n");
    }
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            strcat(outName.as_mut_ptr(),
                   b".bz2\x00" as *const u8 as *const std::os::raw::c_char);
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input file %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1);
        return
    }
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        if hasSuffix(inName.as_mut_ptr(), zSuffix[i as usize]) != 0 {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"%s: Input file %s already has %s suffix.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), zSuffix[i as usize]);
            }
            setExit(1 as std::os::raw::c_int);
            return
        }
        i += 1
    }
    if srcMode == 3 as std::os::raw::c_int || srcMode == 2 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           notAStandardFile(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Input file %s is not a normal file.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1);
        return
    }
    if srcMode == 3 as std::os::raw::c_int &&
           fileExists(outName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(__stderrp,
                    b"%s: Output file %s already exists.\n\x00" as *const u8
                        as *const std::os::raw::c_char, progName,
                    outName.as_mut_ptr());
            setExit(1);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           { n = countHardLinks(inName.as_mut_ptr()); (n) > 0 as std::os::raw::c_int }
       {
        fprintf(__stderrp,
                b"%s: Input file %s has %d other link%s.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName, inName.as_mut_ptr(), n,
                if n > 1 as std::os::raw::c_int {
                    b"s\x00" as *const u8 as *const std::os::raw::c_char
                } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
        setExit(1);
        return
    }
    if srcMode == 3 as std::os::raw::c_int {
        /* Save the file's meta-info before we open it.  Doing it later
         means we mess up the access times. */
        saveInputFileMetaInfo(inName.as_ptr());
    }
   
        match srcMode {
        1 => {
            inStr = __stdinp;
            outStr = __stdoutp;
            if isatty(fileno(__stdoutp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t write compressed data to a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1);
                return
            }
        }
        2 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr = __stdoutp;
            if isatty(fileno(__stdoutp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t write compressed data to a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                if !inStr.is_null() { fclose(inStr); }
                setExit(1);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                setExit(1);
                return
            }
        }
        3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr =
                fopen_output_safely(outName.as_mut_ptr(),
                                    b"wb\x00" as *const u8 as
                                        *const std::os::raw::c_char);
            if outStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t create output file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !outStr.is_null() { fclose(outStr); }
                setExit(1);
                return
            }
        }
        _ => {
            panic("compress: bad srcMode");
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(&mut inName, longestFileName);
        fflush(__stderrp);
    }
    /*--- Now the input and output handles are sane.  Do the Biz. ---*/
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as std::os::raw::c_int as Bool;
    compressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    /*--- If there was an I/O error, we won't get here. ---*/
    if srcMode == 3 as std::os::raw::c_int {
        if let Err(e) = applySavedTimeInfoToOutputFile(unsafe { CStr::from_ptr(outName.as_mut_ptr()) }) {
    eprintln!("Error applying time info: {}", e);
}
        deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
        if keepInputFiles == 0 {
            let mut retVal: IntNative = remove(inName.as_mut_ptr());
            if retVal != 0 as std::os::raw::c_int { ioError(); }
        }
    }
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;

   }
}

/*---------------------------------------------*/
unsafe extern "C" fn uncompress(mut name: *mut Char) {
    let mut current_block: u64;
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut magicNumberOK: Bool = 0;
    let mut cantGuess: Bool = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic("uncompress: bad modes\n");
    }
    cantGuess = 0 as std::os::raw::c_int as Bool;
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            i = 0 as std::os::raw::c_int;
            loop  {
                if !(i < 4 as std::os::raw::c_int) {
                    current_block = 17860125682698302841;
                    break ;
                }
                if mapSuffix(outName.as_mut_ptr(), zSuffix[i as usize],
                             unzSuffix[i as usize]) != 0 {
                    current_block = 15314513098708193206;
                    break ;
                }
                i += 1
            }
            match current_block {
                15314513098708193206 => { }
                _ => {
                    cantGuess = 1 as std::os::raw::c_int as Bool;
                    strcat(outName.as_mut_ptr(),
                           b".out\x00" as *const u8 as *const std::os::raw::c_char);
                }
            }
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input file %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1);
        return
    }
    if srcMode == 3 as std::os::raw::c_int || srcMode == 2 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           notAStandardFile(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Input file %s is not a normal file.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1);
        return
    }
    if cantGuess != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Can\'t guess original name for %s -- using %s\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr(), outName.as_mut_ptr());
        }
        /* just a warning, no return */
    }
    if srcMode == 3 as std::os::raw::c_int &&
           fileExists(outName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(__stderrp,
                    b"%s: Output file %s already exists.\n\x00" as *const u8
                        as *const std::os::raw::c_char, progName,
                    outName.as_mut_ptr());
            setExit(1);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           { n = countHardLinks(inName.as_mut_ptr()); (n) > 0 as std::os::raw::c_int }
       {
        fprintf(__stderrp,
                b"%s: Input file %s has %d other link%s.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName, inName.as_mut_ptr(), n,
                if n > 1 as std::os::raw::c_int {
                    b"s\x00" as *const u8 as *const std::os::raw::c_char
                } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
        setExit(1);
        return
    }
    if srcMode == 3 as std::os::raw::c_int {
        /* Save the file's meta-info before we open it.  Doing it later
         means we mess up the access times. */
        saveInputFileMetaInfo(inName.as_ptr());
    }
    match srcMode {
        1 => {
            inStr = __stdinp;
            outStr = __stdoutp;
            if isatty(fileno(__stdinp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t read compressed data from a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1);
                return
            }
        }
        2 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr = __stdoutp;
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s:%s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1);
                return
            }
        }
        3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr =
                fopen_output_safely(outName.as_mut_ptr(),
                                    b"wb\x00" as *const u8 as
                                        *const std::os::raw::c_char);
            if outStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t create output file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !outStr.is_null() { fclose(outStr); }
                setExit(1);
                return
            }
        }
        _ => {
            panic("uncompress: bad srcMode");
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(&mut inName, longestFileName);
        fflush(__stderrp);
    }
    /*--- Now the input and output handles are sane.  Do the Biz. ---*/
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as std::os::raw::c_int as Bool;
    let magicNumberOK = uncompressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    /*--- If there was an I/O error, we won't get here. ---*/
    if magicNumberOK != 0 {
        if srcMode == 3 as std::os::raw::c_int {
            if let Err(e) = applySavedTimeInfoToOutputFile(unsafe { CStr::from_ptr(outName.as_mut_ptr()) }) {
    eprintln!("Error applying time info: {}", e);
}
            deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
            if keepInputFiles == 0 {
                let mut retVal: IntNative = remove(inName.as_mut_ptr());
                if retVal != 0 as std::os::raw::c_int { ioError(); }
            }
        }
    } else {
        unzFailsExist = 1 as std::os::raw::c_int as Bool;
        deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
        if srcMode == 3 as std::os::raw::c_int {
            let mut retVal_0: IntNative = remove(outName.as_mut_ptr());
            if retVal_0 != 0 as std::os::raw::c_int { ioError(); }
        }
    }
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if magicNumberOK != 0 {
        if verbosity >= 1 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"done\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
    } else {
        setExit(2);
        if verbosity >= 1 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"not a bzip2 file.\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        } else {
            fprintf(__stderrp,
                    b"%s: %s is not a bzip2 file.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
        }
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn testf(mut name: *mut Char) {
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut allOK: Bool = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic("testf: bad modes\n");
    }
    copyFileName(outName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
        }
        3 => { copyFileName(inName.as_mut_ptr(), name); }
        2 => { copyFileName(inName.as_mut_ptr(), name); }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1);
        return
    }
    if srcMode != 1 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1);
            return
        }
    }
    match srcMode {
        1 => {
            if isatty(fileno(__stdinp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t read compressed data from a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1);
                return
            }
            inStr = __stdinp
        }
        2 | 3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s:%s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                setExit(1);
                return
            }
        }
        _ => {
            panic("testf: bad srcMode");
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(&mut inName, longestFileName);
        fflush(__stderrp);
    }
    /*--- Now the input handle is sane.  Do the Biz. ---*/
    outputHandleJustInCase = 0 as *mut FILE;
    allOK = testStream(inStr);
    if allOK as std::os::raw::c_int != 0 && verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"ok\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    if allOK == 0 { testFailsExist = 1 as std::os::raw::c_int as Bool };
}
/*---------------------------------------------*/
fn license() {
    eprintln!(
        "bzip2, a block-sorting file compressor.  Version {}.\n   \n   Copyright (C) 1996-2019 by Julian Seward.\n   \n   This program is free software; you can redistribute it and/or modify\n   it under the terms set out in the LICENSE file, which is included\n   in the bzip2 source distribution.\n   \n   This program is distributed in the hope that it will be useful,\n   but WITHOUT ANY WARRANTY; without even the implied warranty of\n   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n   LICENSE file for more details.\n   \n",
        BZ2_bzlibVersion()
    );
}

/*---------------------------------------------*/
fn usage(full_prog_name: *mut c_char) {
    let full_prog_name_str = unsafe { CStr::from_ptr(full_prog_name).to_string_lossy() };
    eprintln!(
        "bzip2, a block-sorting file compressor.  Version {}.\n\n   usage: {} [flags and input files in any order]\n\n   -h --help           print this message\n   -d --decompress     force decompression\n   -z --compress       force compression\n   -k --keep           keep (don't delete) input files\n   -f --force          overwrite existing output files\n   -t --test           test compressed file integrity\n   -c --stdout         output to standard out\n   -q --quiet          suppress noncritical error messages\n   -v --verbose        be verbose (a 2nd -v gives more)\n   -L --license        display software version & license\n   -V --version        display software version & license\n   -s --small          use less memory (at most 2500k)\n   -1 .. -9            set block size to 100k .. 900k\n   --fast              alias for -1\n   --best              alias for -9\n\n   If invoked as `bzip2\', default action is to compress.\n              as `bunzip2\',  default action is to decompress.\n              as `bzcat\', default action is to decompress to stdout.\n\n   If no file names are given, bzip2 compresses or decompresses\n   from standard input to standard output.  You can combine\n   short flags, so `-v -4\' means the same as -v4 or -4v, &c.\n",
        BZ2_bzlibVersion(),
        full_prog_name_str
    );
}

/*---------------------------------------------*/
fn redundant(flag: *mut Char) {
    let flag_str = unsafe { CStr::from_ptr(flag).to_string_lossy() };
    let prog_name_str = unsafe { CStr::from_ptr(progName).to_string_lossy() };
    eprintln!("{}: {} is redundant in versions 0.9.5 and above", prog_name_str, flag_str);
}

/*---------------------------------------------*/
fn myMalloc(n: usize) -> *mut std::ffi::c_void {
    let layout = std::alloc::Layout::from_size_align(n, std::mem::align_of::<std::ffi::c_void>()).unwrap();
    let p = unsafe { std::alloc::alloc(layout) };
    if p.is_null() {
        outOfMemory();
        std::ptr::null_mut()
    } else {
        p as *mut std::ffi::c_void
    }
}

/*---------------------------------------------*/
fn mkCell() -> *mut Cell {
    let c = Box::new(Cell {
        name: std::ptr::null_mut(),
        link: std::ptr::null_mut(),
    });
    Box::into_raw(c)
}

/*---------------------------------------------*/
unsafe extern "C" fn snocString(mut root: *mut Cell, mut name: *mut Char)
 -> *mut Cell {
    if root.is_null() {
        let tmp: *mut Cell = mkCell();
        let name_length = strlen(name);
let total_size = (5 as usize).wrapping_add(name_length as usize);
(*tmp).name = myMalloc(total_size) as *mut Char;
        strcpy((*tmp).name, name);
        return tmp
    } else {
        let mut tmp_0: *mut Cell = root;
        while !(*tmp_0).link.is_null() { tmp_0 = (*tmp_0).link }
        (*tmp_0).link = snocString((*tmp_0).link, name);
        return root
    };
}
/*---------------------------------------------*/
fn addFlagsFromEnvVar(argList: &mut *mut Cell, varName: &str) {
    if let Ok(env_value) = std::env::var(varName) {
        let mut p = env_value.as_str();
        while !p.is_empty() {
            p = p.trim_start();
            let next_space = p.find(' ').unwrap_or(p.len());
            let flag = &p[..next_space];
            if !flag.is_empty() {
                let trimmed_flag = if flag.len() > 1024 { &flag[..1024] } else { flag };
                unsafe {
                    *argList = snocString(*argList, trimmed_flag.as_ptr() as *mut Char);
                }
            }
            p = &p[next_space..];
        }
    }
}

unsafe fn main_0(mut argc: IntNative, mut argv: *mut *mut Char) -> IntNative {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: *mut Char = 0 as *mut Char;
    let mut argList: *mut Cell = 0 as *mut Cell;
    let mut aa: *mut Cell = 0 as *mut Cell;
    let mut decode: Bool = 0;
    /*-- Be really really really paranoid :-) --*/
    if ::std::mem::size_of::<Int32>() as std::os::raw::c_ulong !=
           4 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UInt32>() as std::os::raw::c_ulong !=
               4 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<Int16>() as std::os::raw::c_ulong !=
               2 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UInt16>() as std::os::raw::c_ulong !=
               2 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<Char>() as std::os::raw::c_ulong !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong {
        configError();
    }
    /*-- Initialise --*/
    outputHandleJustInCase =
        0 as *mut FILE; /* avoid bogus warning from egcs-1.1.X */
    smallMode = 0 as std::os::raw::c_int as Bool;
    keepInputFiles = 0 as std::os::raw::c_int as Bool;
    forceOverwrite = 0 as std::os::raw::c_int as Bool;
    noisy = 1 as std::os::raw::c_int as Bool;
    verbosity = 0 as std::os::raw::c_int;
    blockSize100k = 9 as std::os::raw::c_int;
    testFailsExist = 0 as std::os::raw::c_int as Bool;
    unzFailsExist = 0 as std::os::raw::c_int as Bool;
    numFileNames = 0 as std::os::raw::c_int;
    numFilesProcessed = 0 as std::os::raw::c_int;
    workFactor = 30 as std::os::raw::c_int;
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    exitValue = 0 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    i = j;
    /*-- Set up signal handlers for mem access errors --*/
    signal(11 as std::os::raw::c_int,
           Some(mySIGSEGVorSIGBUScatcher as
                    unsafe extern "C" fn(_: IntNative) -> ()));
    signal(10 as std::os::raw::c_int,
           Some(mySIGSEGVorSIGBUScatcher as
                    unsafe extern "C" fn(_: IntNative) -> ()));
    copyFileName(inName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    copyFileName(outName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    copyFileName(progNameReally.as_mut_ptr(),
                 *argv.offset(0 as std::os::raw::c_int as isize));
    progName =
        &mut *progNameReally.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
            *mut Char;
    tmp =
        &mut *progNameReally.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
            *mut Char;
    while *tmp as std::os::raw::c_int != '\u{0}' as i32 {
        if *tmp as std::os::raw::c_int == '/' as i32 {
            progName = tmp.offset(1 as std::os::raw::c_int as isize)
        }
        tmp = tmp.offset(1)
    }
    /*-- Copy flags from env var BZIP2, and 
        expand filename wildcards in arg list.
   --*/
    argList = 0 as *mut Cell;
    let var_name2 = "BZIP2";
addFlagsFromEnvVar(&mut argList, var_name2);
    let var_name1 = "BZIP";
addFlagsFromEnvVar(&mut argList, var_name1);
    i = 1 as std::os::raw::c_int;
    while i <= argc - 1 as std::os::raw::c_int {
        argList = snocString(argList, *argv.offset(i as isize));
        i += 1
    }
    /*-- Find the length of the longest filename --*/
    longestFileName = 7 as std::os::raw::c_int;
    numFileNames = 0 as std::os::raw::c_int;
    decode = 1 as std::os::raw::c_int as Bool;
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            decode = 0 as std::os::raw::c_int as Bool
        } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_int == '-' as i32 &&
                        decode as std::os::raw::c_int != 0) {
            numFileNames += 1;
            if longestFileName < strlen((*aa).name) as Int32 {
                longestFileName = strlen((*aa).name) as Int32
            }
        }
        aa = (*aa).link
    }
    /*-- Determine source modes; flag handling may change this too. --*/
    if numFileNames == 0 as std::os::raw::c_int {
        srcMode = 1 as std::os::raw::c_int
    } else { srcMode = 3 as std::os::raw::c_int }
    /*-- Determine what to do (compress/uncompress/test/cat). --*/
   /*-- Note that subsequent flag handling may change this. --*/
    opMode = 1 as std::os::raw::c_int;
    if !strstr(progName,
               b"unzip\x00" as *const u8 as *const std::os::raw::c_char).is_null() ||
           !strstr(progName,
                   b"UNZIP\x00" as *const u8 as *const std::os::raw::c_char).is_null()
       {
        opMode = 2 as std::os::raw::c_int
    }
    if !strstr(progName,
               b"z2cat\x00" as *const u8 as *const std::os::raw::c_char).is_null() ||
           !strstr(progName,
                   b"Z2CAT\x00" as *const u8 as *const std::os::raw::c_char).is_null()
           ||
           !strstr(progName,
                   b"zcat\x00" as *const u8 as *const std::os::raw::c_char).is_null()
           ||
           !strstr(progName,
                   b"ZCAT\x00" as *const u8 as *const std::os::raw::c_char).is_null()
       {
        opMode = 2 as std::os::raw::c_int;
        srcMode =
            if numFileNames == 0 as std::os::raw::c_int {
                1 as std::os::raw::c_int
            } else { 2 as std::os::raw::c_int }
    }
    /*-- Look at the flags. --*/
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            break ;
        }
        if *(*aa).name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '-' as i32 &&
               *(*aa).name.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                   '-' as i32 {
            j = 1 as std::os::raw::c_int;
            while *(*aa).name.offset(j as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
                match *(*aa).name.offset(j as isize) as std::os::raw::c_int {
                    99 => { srcMode = 2 as std::os::raw::c_int }
                    100 => { opMode = 2 as std::os::raw::c_int }
                    122 => { opMode = 1 as std::os::raw::c_int }
                    102 => { forceOverwrite = 1 as std::os::raw::c_int as Bool }
                    116 => { opMode = 3 as std::os::raw::c_int }
                    107 => { keepInputFiles = 1 as std::os::raw::c_int as Bool }
                    115 => { smallMode = 1 as std::os::raw::c_int as Bool }
                    113 => { noisy = 0 as std::os::raw::c_int as Bool }
                    49 => { blockSize100k = 1 as std::os::raw::c_int }
                    50 => { blockSize100k = 2 as std::os::raw::c_int }
                    51 => { blockSize100k = 3 as std::os::raw::c_int }
                    52 => { blockSize100k = 4 as std::os::raw::c_int }
                    53 => { blockSize100k = 5 as std::os::raw::c_int }
                    54 => { blockSize100k = 6 as std::os::raw::c_int }
                    55 => { blockSize100k = 7 as std::os::raw::c_int }
                    56 => { blockSize100k = 8 as std::os::raw::c_int }
                    57 => { blockSize100k = 9 as std::os::raw::c_int }
                    86 | 76 => { license(); }
                    118 => { verbosity += 1 }
                    104 => { usage(progName); exit(0 as std::os::raw::c_int); }
                    _ => {
                        fprintf(__stderrp,
                                b"%s: Bad flag `%s\'\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, progName,
                                (*aa).name);
                        usage(progName);
                        exit(1 as std::os::raw::c_int);
                    }
                }
                j += 1
            }
        }
        aa = (*aa).link
    }
    /*-- And again ... --*/
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            break ;
        }
        if strcmp((*aa).name,
                  b"--stdout\x00" as *const u8 as *const std::os::raw::c_char) ==
               0 as std::os::raw::c_int {
            srcMode = 2 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--decompress\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            opMode = 2 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--compress\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            opMode = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--force\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            forceOverwrite = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--test\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            opMode = 3 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--keep\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            keepInputFiles = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--small\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            smallMode = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--quiet\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            noisy = 0 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--version\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            license();
        } else if strcmp((*aa).name,
                         b"--license\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            license();
        } else if strcmp((*aa).name,
                         b"--exponential\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            workFactor = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--repetitive-best\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            redundant((*aa).name);
        } else if strcmp((*aa).name,
                         b"--repetitive-fast\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            redundant((*aa).name);
        } else if strcmp((*aa).name,
                         b"--fast\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            blockSize100k = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--best\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            blockSize100k = 9 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--verbose\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            verbosity += 1
        } else if strcmp((*aa).name,
                         b"--help\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            usage(progName);
            exit(0 as std::os::raw::c_int);
        } else {
            if strncmp((*aa).name,
                       b"--\x00" as *const u8 as *const std::os::raw::c_char,
                       2 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int
               {
                fprintf(__stderrp,
                        b"%s: Bad flag `%s\'\n\x00" as *const u8 as
                            *const std::os::raw::c_char, progName, (*aa).name);
                usage(progName);
                exit(1 as std::os::raw::c_int);
            }
        }
        aa = (*aa).link
    }
    if verbosity > 4 as std::os::raw::c_int { verbosity = 4 as std::os::raw::c_int }
    if opMode == 1 as std::os::raw::c_int && smallMode as std::os::raw::c_int != 0 &&
           blockSize100k > 2 as std::os::raw::c_int {
        blockSize100k = 2 as std::os::raw::c_int
    }
    if opMode == 3 as std::os::raw::c_int && srcMode == 2 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"%s: -c and -t cannot be used together.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName);
        exit(1 as std::os::raw::c_int);
    }
    if srcMode == 2 && numFileNames == 0 {
    srcMode = 1;
}
if opMode != 1 {
    blockSize100k = 0;
}
if srcMode == 3 {
    signal(2, Some(mySignalCatcher));
    signal(15, Some(mySignalCatcher));
    signal(1, Some(mySignalCatcher));
}
if opMode == 1 {
    if srcMode == 1 {
        compress(std::ptr::null_mut());
    } else {
        decode = 1;
        let mut aa = argList;
        while !aa.is_null() {
            if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
                decode = 0;
            } else if !(*(*aa).name.offset(0) as std::os::raw::c_int == '-' as i32 && decode != 0) {
                numFilesProcessed += 1;
                compress((*aa).name);
            }
            aa = (*aa).link;
        }
    }
} else if opMode == 2 {
    unzFailsExist = 0;
    if srcMode == 1 {
        uncompress(std::ptr::null_mut());
    } else {
        decode = 1;
        let mut aa = argList;
        while !aa.is_null() {
            if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
                decode = 0;
            } else if !(*(*aa).name.offset(0) as std::os::raw::c_int == '-' as i32 && decode != 0) {
                numFilesProcessed += 1;
                uncompress((*aa).name);
            }
            aa = (*aa).link;
        }
    }
    if unzFailsExist != 0 {
        setExit(2);
        std::process::exit(exitValue);
    }
} else {
    testFailsExist = 0;
    if srcMode == 1 {
        testf(std::ptr::null_mut());
    } else {
        decode = 1;
        let mut aa = argList;
        while !aa.is_null() {
            if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
                decode = 0;
            } else if !(*(*aa).name.offset(0) as std::os::raw::c_int == '-' as i32 && decode != 0) {
                numFilesProcessed += 1;
                testf((*aa).name);
            }
            aa = (*aa).link;
        }
    }
    if testFailsExist != 0 {
        if noisy != 0 {
            eprintln!("\nYou can use the `bzip2recover` program to attempt to recover\ndata from undamaged sections of corrupted files.\n");
        }
        setExit(2);
        std::process::exit(exitValue);
    }
}

// Free the argument list memory to mollify leak detectors
let mut aa = argList;
while !aa.is_null() {
    let aa2 = (*aa).link;
    if !(*aa).name.is_null() {
        free((*aa).name as *mut std::os::raw::c_void);
    }
    free(aa as *mut std::os::raw::c_void);
    aa = aa2;
}
exitValue

}
pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = args.len() as IntNative;

    // Convert Vec<String> to Vec<CString> for the C API
    let c_args: Vec<std::ffi::CString> = args.iter()
        .map(|arg| std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();

    // Call the main_0 function within an unsafe block
    let result = unsafe {
        main_0(arg_count - 1, c_args.as_ptr() as *mut *mut Char)
    };
    ::std::process::exit(result as i32);
}

/*-----------------------------------------------------------*/
/*--- end                                         bzip2.c ---*/
/*-----------------------------------------------------------*/

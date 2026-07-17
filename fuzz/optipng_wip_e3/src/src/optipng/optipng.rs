
















use std::ffi::CString;

use std::ptr;

use std::ffi::CStr;

extern "C" {
    
    
    
    
    
    fn __ctype_b_loc() -> *mut *const std::os::raw::c_ushort;
    
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    
    fn __errno_location() -> *mut std::os::raw::c_int;
    
    static mut stdout: *mut FILE;
    
    static mut stderr: *mut FILE;
    
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn setvbuf(__stream: *mut FILE, __buf: *mut std::os::raw::c_char,
               __modes: std::os::raw::c_int, __n: size_t) -> std::os::raw::c_int;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn vfprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ::std::ffi::VaList)
     -> std::os::raw::c_int;
    
    fn fputc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    
    fn fputs(__s: *const std::os::raw::c_char, __stream: *mut FILE) -> std::os::raw::c_int;
    
    fn strtoul(_: *const std::os::raw::c_char, _: *mut *mut std::os::raw::c_char,
               _: std::os::raw::c_int) -> std::os::raw::c_ulong;
    
    fn abort() -> !;
    
    fn exit(_: std::os::raw::c_int) -> !;
    
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::libpng::png::png_get_libpng_ver;
pub use crate::src::optipng::bitset::opng_strparse_rangeset_to_bitset;
pub use crate::src::optipng::optim::opng_finalize;
pub use crate::src::optipng::optim::opng_initialize;
pub use crate::src::optipng::optim::opng_optimize;
pub use crate::src::zlib::zutil::zlibVersion;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut std::os::raw::c_void,
    pub reg_save_area: *mut std::os::raw::c_void,
}
pub type __int32_t = std::os::raw::c_int;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
pub type size_t = crate::src::gifread::gifread::size_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::libpng::png::_IO_FILE;
pub type _IO_lock_t = crate::src::gifread::gifread::_IO_lock_t;
pub type FILE = crate::src::gifread::gifread::FILE;
pub type opng_bitset_t = crate::src::optipng::bitset::opng_bitset_t;
// #[derive(Copy, Clone)]

pub type opng_options = crate::src::optipng::optim::opng_options;
// #[derive(Copy, Clone)]

pub type opng_ui = crate::src::optipng::optim::opng_ui;
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const OP_SHOW_VERSION: C2RustUnnamed_0 = 2;
pub const OP_SHOW_HELP: C2RustUnnamed_0 = 1;
pub const OP_RUN: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub help: std::os::raw::c_int,
    pub version: std::os::raw::c_int,
}
#[inline]
fn tolower(__c: i32) -> i32 {
    if __c >= -128 && __c < 256 {
        let ctype_tolower = unsafe { *__ctype_tolower_loc() };
        unsafe {
            return *ctype_tolower.offset(__c as isize);
        }
    }
    __c
}

/*
 * OptiPNG: Advanced PNG optimization program.
 * http://optipng.sourceforge.net/
 *
 * Copyright (C) 2001-2017 Cosmin Truta and the Contributing Authors.
 *
 * This software is distributed under the zlib license.
 * Please see the accompanying LICENSE file.
 *
 * PNG optimization is described in detail in the PNG-Tech article
 * "A guide to PNG optimization"
 * http://optipng.sourceforge.net/pngtech/png_optimization.html
 *
 * The idea of running multiple compression trials with different
 * PNG filters and zlib parameters is inspired from the pngcrush
 * program by Glenn Randers-Pehrson.
 * The idea of performing lossless image reductions is inspired
 * from the pngrewrite program by Jason Summers.
 */
static mut msg_intro: *const std::os::raw::c_char =
    b"OptiPNG version 0.7.7\nCopyright (C) 2001-2017 Cosmin Truta and the Contributing Authors.\n\x00"
        as *const u8 as *const std::os::raw::c_char;
static mut msg_license: *const std::os::raw::c_char =
    b"This program is open-source software. See LICENSE for more details.\n\nPortions of this software are based in part on the work of:\n  Jean-loup Gailly and Mark Adler (zlib)\n  Glenn Randers-Pehrson and the PNG Development Group (libpng)\n  Miyasaka Masaru (BMP support)\n  David Koblas (GIF support)\n\x00"
        as *const u8 as *const std::os::raw::c_char;
static mut msg_help_synopsis: *const std::os::raw::c_char =
    b"Synopsis:\n    optipng [options] files ...\nFiles:\n    Image files of type: PNG, BMP, GIF, PNM or TIFF\n\x00"
        as *const u8 as *const std::os::raw::c_char;
static mut msg_help_basic_options: *const std::os::raw::c_char =
    b"Basic options:\n    -?, -h, -help\tshow the extended help\n    -o <level>\t\toptimization level (0-7)\t\t[default: 2]\n    -v\t\t\trun in verbose mode / show copyright and version info\n\x00"
        as *const u8 as *const std::os::raw::c_char;
static mut msg_help_options: *const std::os::raw::c_char =
    b"Basic options:\n    -?, -h, -help\tshow this help\n    -o <level>\t\toptimization level (0-7)\t\t[default: 2]\n    -v\t\t\trun in verbose mode / show copyright and version info\nGeneral options:\n    -backup, -keep\tkeep a backup of the modified files\n    -clobber\t\toverwrite existing files\n    -fix\t\tenable error recovery\n    -force\t\tenforce writing of a new output file\n    -preserve\t\tpreserve file attributes if possible\n    -quiet, -silent\trun in quiet mode\n    -simulate\t\trun in simulation mode\n    -out <file>\t\twrite output file to <file>\n    -dir <directory>\twrite output file(s) to <directory>\n    -log <file>\t\tlog messages to <file>\n    --\t\t\tstop option switch parsing\nOptimization options:\n    -f <filters>\tPNG delta filters (0-5)\t\t\t[default: 0,5]\n    -i <type>\t\tPNG interlace type (0-1)\n    -zc <levels>\tzlib compression levels (1-9)\t\t[default: 9]\n    -zm <levels>\tzlib memory levels (1-9)\t\t[default: 8]\n    -zs <strategies>\tzlib compression strategies (0-3)\t[default: 0-3]\n    -zw <size>\t\tzlib window size (256,512,1k,2k,4k,8k,16k,32k)\n    -full\t\tproduce a full report on IDAT (might reduce speed)\n    -nb\t\t\tno bit depth reduction\n    -nc\t\t\tno color type reduction\n    -np\t\t\tno palette reduction\n    -nx\t\t\tno reductions\n    -nz\t\t\tno IDAT recoding\nEditing options:\n    -snip\t\tcut one image out of multi-image or animation files\n    -strip <objects>\tstrip metadata objects (e.g. \"all\")\nOptimization levels:\n    -o0\t\t<=>\t-o1 -nx -nz\t\t\t\t(0 or 1 trials)\n    -o1\t\t<=>\t-zc9 -zm8 -zs0 -f0\t\t\t(1 trial)\n    \t\t(or...)\t-zc9 -zm8 -zs1 -f5\t\t\t(1 trial)\n    -o2\t\t<=>\t-zc9 -zm8 -zs0-3 -f0,5\t\t\t(8 trials)\n    -o3\t\t<=>\t-zc9 -zm8-9 -zs0-3 -f0,5\t\t(16 trials)\n    -o4\t\t<=>\t-zc9 -zm8 -zs0-3 -f0-5\t\t\t(24 trials)\n    -o5\t\t<=>\t-zc9 -zm8-9 -zs0-3 -f0-5\t\t(48 trials)\n    -o6\t\t<=>\t-zc1-9 -zm8 -zs0-3 -f0-5\t\t(120 trials)\n    -o7\t\t<=>\t-zc1-9 -zm8-9 -zs0-3 -f0-5\t\t(240 trials)\n    -o7 -zm1-9\t<=>\t-zc1-9 -zm1-9 -zs0-3 -f0-5\t\t(1080 trials)\nNotes:\n    The combination for -o1 is chosen heuristically.\n    Exhaustive combinations such as \"-o7 -zm1-9\" are not generally recommended.\n\x00"
        as *const u8 as *const std::os::raw::c_char;
static mut msg_help_examples: *const std::os::raw::c_char =
    b"Examples:\n    optipng file.png\t\t\t\t\t\t(default speed)\n    optipng -o5 file.png\t\t\t\t\t(slow)\n    optipng -o7 file.png\t\t\t\t\t(very slow)\n\x00"
        as *const u8 as *const std::os::raw::c_char;
static mut msg_help_more: *const std::os::raw::c_char =
    b"Type \"optipng -h\" for extended help.\n\x00" as *const u8 as
        *const std::os::raw::c_char;
static mut operation: C2RustUnnamed_0 = OP_RUN;
static mut local_options: C2RustUnnamed_1 =
    C2RustUnnamed_1{help: 0, version: 0,};
static mut options: opng_options =
    opng_options{backup: 0,
                 clobber: 0,
                 debug: 0,
                 fix: 0,
                 force: 0,
                 full: 0,
                 preserve: 0,
                 quiet: 0,
                 simulate: 0,
                 verbose: 0,
                 out_name: 0 as *const std::os::raw::c_char,
                 dir_name: 0 as *const std::os::raw::c_char,
                 log_name: 0 as *const std::os::raw::c_char,
                 interlace: 0,
                 nb: 0,
                 nc: 0,
                 np: 0,
                 nz: 0,
                 optim_level: 0,
                 compr_level_set: 0,
                 mem_level_set: 0,
                 strategy_set: 0,
                 filter_set: 0,
                 window_bits: 0,
                 snip: 0,
                 strip_all: 0,};
static mut con_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut log_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut start_of_line: std::os::raw::c_int = 0;
/*
 * Error handling.
 */
unsafe extern "C" fn error(mut fmt: *const std::os::raw::c_char, mut args: ...) {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    /* Print the error message to stderr and exit. */
    fprintf(stderr, b"** Error: \x00" as *const u8 as *const std::os::raw::c_char);
    arg_ptr = args.clone();
    vfprintf(stderr, fmt, arg_ptr.as_va_list());
    fprintf(stderr, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    exit(1 as std::os::raw::c_int);
}
/*
 * Panic handling.
 */
unsafe extern "C" fn panic(mut msg: *const std::os::raw::c_char) {
    /* Print the panic message to stderr and terminate abnormally. */
    fprintf(stderr,
            b"\n** INTERNAL ERROR: %s\n\x00" as *const u8 as
                *const std::os::raw::c_char, msg);
    fprintf(stderr,
            b"Please submit a defect report.\nhttp://optipng.sourceforge.net/\n\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fflush(stderr);
    if options.debug != 0 {
        /* Terminate abnormally, possibly with a stack trace or a core dump. */
        abort();
    } else {
        /* Terminate abnormally, cleanly. */
        fprintf(stderr,
                b"The execution of this program has been terminated abnormally.\n\x00"
                    as *const u8 as *const std::os::raw::c_char);
        exit(70 as std::os::raw::c_int);
        /* EX_SOFTWARE */
    };
}
/*
 * String utility.
 */
fn opng_strcasecmp(str1: *const std::os::raw::c_char, str2: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut ch1: std::os::raw::c_int;
    let mut ch2: std::os::raw::c_int;

    let c_str1 = unsafe { std::ffi::CStr::from_ptr(str1) };
    let c_str2 = unsafe { std::ffi::CStr::from_ptr(str2) };

    let str1_lossy = c_str1.to_string_lossy();
    let str2_lossy = c_str2.to_string_lossy();

    let mut iter1 = str1_lossy.chars();
    let mut iter2 = str2_lossy.chars();

    loop {
        ch1 = iter1.next().map_or(0, |c| c.to_ascii_lowercase() as std::os::raw::c_int);
        ch2 = iter2.next().map_or(0, |c| c.to_ascii_lowercase() as std::os::raw::c_int);

        if ch1 != ch2 {
            return ch1 - ch2;
        }
        if ch1 == 0 {
            return 0;
        }
    }
}

/*
 * String utility.
 */
unsafe extern "C" fn opng_strltrim(mut str: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    /* Skip the leading whitespace characters. */
    while *(*__ctype_b_loc()).offset(*str as std::os::raw::c_int as isize) as
              std::os::raw::c_int &
              _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int != 0 {
        str = str.offset(1)
    }
    return str as *mut std::os::raw::c_char;
}
/*
 * String utility.
 */
unsafe extern "C" fn opng_strtail(mut str: *const std::os::raw::c_char,
                                  mut num: size_t) -> *mut std::os::raw::c_char {
    let mut len: size_t = 0;
    /* Return up to num rightmost characters. */
    len = strlen(str);
    if len <= num { return str as *mut std::os::raw::c_char }
    return (str as
                *mut std::os::raw::c_char).offset(len as
                                              isize).offset(-(num as isize));
}
/*
 * String utility.
 */
fn opng_strpbrk_digit(str: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let mut current = str;
    loop {
        if unsafe { *current } == 0 {
            return std::ptr::null_mut();
        }
        let ctype = unsafe { *__ctype_b_loc() };
        if unsafe { *ctype.offset(*current as isize) } as u16 & _ISdigit as u16 != 0 {
            return current as *mut std::os::raw::c_char;
        }
        current = unsafe { current.offset(1) };
    }
}

/*
 * String conversion utility.
 */
unsafe extern "C" fn opng_str2ulong(mut out_val: *mut std::os::raw::c_ulong,
                                    mut in_str: *const std::os::raw::c_char,
                                    mut allow_multiplier: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut begin_ptr: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut end_ptr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut multiplier: std::os::raw::c_ulong = 0;
    /* Extract the value from the string. */
    /* Do not allow the minus sign, not even for -0. */
    end_ptr = opng_strltrim(in_str); /* matching failure */
    begin_ptr = end_ptr;
    if *begin_ptr as std::os::raw::c_int >= '0' as i32 &&
           *begin_ptr as std::os::raw::c_int <= '9' as i32 {
        *out_val = strtoul(begin_ptr, &mut end_ptr, 10 as std::os::raw::c_int)
    }
    if begin_ptr == end_ptr as *const std::os::raw::c_char {
        *__errno_location() = 22 as std::os::raw::c_int;
        *out_val = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
        return -(1 as std::os::raw::c_int)
    }
    if allow_multiplier != 0 {
        /* Check for the following SI suffixes:
         *   'K' or 'k': kibi (1024)
         *   'M':        mebi (1024 * 1024)
         *   'G':        gibi (1024 * 1024 * 1024)
         */
        if *end_ptr as std::os::raw::c_int == 'k' as i32 ||
               *end_ptr as std::os::raw::c_int == 'K' as i32 {
            end_ptr = end_ptr.offset(1); /* overflow */
            multiplier = 1024 as std::os::raw::c_ulong
        } else if *end_ptr as std::os::raw::c_int == 'M' as i32 {
            end_ptr = end_ptr.offset(1);
            multiplier =
                (1024 as std::os::raw::c_ulong).wrapping_mul(1024 as std::os::raw::c_ulong)
        } else if *end_ptr as std::os::raw::c_int == 'G' as i32 {
            end_ptr = end_ptr.offset(1);
            multiplier =
                (1024 as
                     std::os::raw::c_ulong).wrapping_mul(1024 as
                                                     std::os::raw::c_ulong).wrapping_mul(1024
                                                                                     as
                                                                                     std::os::raw::c_ulong)
        } else { multiplier = 1 as std::os::raw::c_int as std::os::raw::c_ulong }
        if multiplier > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            if *out_val >
                   (9223372036854775807 as std::os::raw::c_long as
                        std::os::raw::c_ulong).wrapping_mul(2 as
                                                        std::os::raw::c_ulong).wrapping_add(1
                                                                                        as
                                                                                        std::os::raw::c_ulong).wrapping_div(multiplier)
               {
                *__errno_location() = 34 as std::os::raw::c_int;
                *out_val =
                    (9223372036854775807 as std::os::raw::c_long as
                         std::os::raw::c_ulong).wrapping_mul(2 as
                                                         std::os::raw::c_ulong).wrapping_add(1
                                                                                         as
                                                                                         std::os::raw::c_ulong)
            } else { *out_val = (*out_val).wrapping_mul(multiplier) }
        }
    }
    /* Check for trailing garbage. */
    if *opng_strltrim(end_ptr) as std::os::raw::c_int != 0 as std::os::raw::c_int {
        *__errno_location() = 22 as std::os::raw::c_int; /* garbage in input */
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
/*
 * Command line utility.
 */
unsafe extern "C" fn err_option_arg(mut opt: *const std::os::raw::c_char,
                                    mut opt_arg: *const std::os::raw::c_char) {
    /* Issue an error regarding the incorrect value of the option argument. */
    if opt_arg.is_null() ||
           *opng_strltrim(opt_arg) as std::os::raw::c_int == 0 as std::os::raw::c_int {
        error(b"Missing argument for option %s\x00" as *const u8 as
                  *const std::os::raw::c_char, opt);
    } else {
        error(b"Invalid argument for option %s: %s\x00" as *const u8 as
                  *const std::os::raw::c_char, opt, opt_arg);
    };
}
/*
 * Command line utility.
 */
unsafe extern "C" fn check_num_option(mut opt: *const std::os::raw::c_char,
                                      mut opt_arg: *const std::os::raw::c_char,
                                      mut lowest: std::os::raw::c_int,
                                      mut highest: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut value: std::os::raw::c_ulong = 0;
    /* Extract the numeric value from the option argument. */
    if opng_str2ulong(&mut value, opt_arg, 0 as std::os::raw::c_int) !=
           0 as std::os::raw::c_int ||
           value > 2147483647 as std::os::raw::c_int as std::os::raw::c_ulong ||
           (value as std::os::raw::c_int) < lowest || value as std::os::raw::c_int > highest {
        err_option_arg(opt, opt_arg);
    }
    return value as std::os::raw::c_int;
}
/*
 * Command line utility.
 */
unsafe extern "C" fn check_power2_option(mut opt: *const std::os::raw::c_char,
                                         mut opt_arg: *const std::os::raw::c_char,
                                         mut lowest: std::os::raw::c_int,
                                         mut highest: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut value: std::os::raw::c_ulong = 0;
    let mut result: std::os::raw::c_int = 0;
    /* Extract the exact log2 of the numeric value from the option argument. */
    /* Allow the 'k', 'M', 'G' suffixes. */
    if opng_str2ulong(&mut value, opt_arg, 1 as std::os::raw::c_int) ==
           0 as std::os::raw::c_int {
        if lowest < 0 as std::os::raw::c_int { lowest = 0 as std::os::raw::c_int }
        if highest >
               (8 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_long>()
                                                    as
                                                    std::os::raw::c_ulong).wrapping_sub(2
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_ulong)
                   as std::os::raw::c_int {
            highest =
                (8 as std::os::raw::c_int as
                     std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_long>()
                                                     as
                                                     std::os::raw::c_ulong).wrapping_sub(2
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_ulong)
                    as std::os::raw::c_int
        }
        result = lowest;
        while result <= highest {
            if (1 as std::os::raw::c_ulong) << result == value { return result }
            result += 1
        }
    }
    err_option_arg(opt, opt_arg);
    return -(1 as std::os::raw::c_int);
}
/*
 * Command line utility.
 */
unsafe extern "C" fn check_rangeset_option(mut opt: *const std::os::raw::c_char,
                                           mut opt_arg: *const std::os::raw::c_char,
                                           mut result_mask: opng_bitset_t)
 -> opng_bitset_t {
    let mut result: opng_bitset_t = 0;
    /* Extract the rangeset from the option argument.
     * Accept only non-empty rangesets that fit in the given range.
     */
    if opng_strparse_rangeset_to_bitset(&mut result, opt_arg, result_mask) !=
           0 as std::os::raw::c_int {
        result = 0 as std::os::raw::c_uint
    }
    if result & result_mask != result { result = 0 as std::os::raw::c_uint }
    if result == 0 as std::os::raw::c_uint { err_option_arg(opt, opt_arg); }
    return result;
}
/*
 * Command line utility.
 */
unsafe extern "C" fn check_obj_option(mut opt: *const std::os::raw::c_char,
                                      mut opt_arg: *const std::os::raw::c_char) {
    let mut i: std::os::raw::c_uint = 0;
    if strcmp(b"all\x00" as *const u8 as *const std::os::raw::c_char, opt_arg) ==
           0 as std::os::raw::c_int {
        return
    }
    /* Issue an error about the unrecognized option argument. */
    /* Make it specific on whether this argument is a chunk name. */
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < 4 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Do not use isalpha(), because it is locale-dependent. */
        if !(*opt_arg.offset(i as isize) as std::os::raw::c_int >= 'A' as i32 &&
                 *opt_arg.offset(i as isize) as std::os::raw::c_int <= 'Z' as i32 ||
                 *opt_arg.offset(i as isize) as std::os::raw::c_int >= 'a' as i32 &&
                     *opt_arg.offset(i as isize) as std::os::raw::c_int <= 'z' as i32)
           {
            break ;
        }
        i = i.wrapping_add(1)
    }
    if i == 4 as std::os::raw::c_int as std::os::raw::c_uint &&
           *opt_arg.offset(i as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int {
        error(b"Manipulation of individual chunks is not implemented\x00" as
                  *const u8 as *const std::os::raw::c_char);
    } else { err_option_arg(opt, opt_arg); };
}
/*
 * Command line parsing.
 */
unsafe extern "C" fn scan_option(mut str: *const std::os::raw::c_char,
                                 mut opt_buf: *mut std::os::raw::c_char,
                                 mut opt_buf_size: size_t,
                                 mut opt_arg_ptr: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ptr: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut opt_len: std::os::raw::c_uint = 0;
    /* Check if arg is an "-option". */
    if *str.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '-' as i32 ||
           *str.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        /* no "-option", or just "-" */
        return 0 as std::os::raw::c_int
    }
    /* Extract the normalized option, and possibly the option argument. */
    opt_len = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    ptr = str.offset(1 as std::os::raw::c_int as isize);
    while *ptr as std::os::raw::c_int == '-' as i32 {
        /* "--option", "---option", etc. */
        ptr = ptr.offset(1)
    }
    if *ptr as std::os::raw::c_int == 0 as std::os::raw::c_int {
        /* "--" */
        ptr = ptr.offset(-1)
    }
    loop  {
        if (opt_len as std::os::raw::c_ulong) < opt_buf_size {
            /* truncate "-verylongoption" */
            *opt_buf.offset(opt_len as isize) =
                ({
                     let mut __res: std::os::raw::c_int = 0;
                     if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong
                            > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                         if 0 != 0 {
                             let mut __c: std::os::raw::c_int = *ptr as std::os::raw::c_int;
                             __res =
                                 if __c < -(128 as std::os::raw::c_int) ||
                                        __c > 255 as std::os::raw::c_int {
                                     __c
                                 } else {
                                     *(*__ctype_tolower_loc()).offset(__c as
                                                                          isize)
                                 }
                         } else {
    __res = tolower(*ptr as i32);
}
                     } else {
                         __res =
                             *(*__ctype_tolower_loc()).offset(*ptr as
                                                                  std::os::raw::c_int
                                                                  as isize)
                     }
                     __res
                 }) as std::os::raw::c_char
        }
        opt_len = opt_len.wrapping_add(1);
        ptr = ptr.offset(1);
        if *ptr as std::os::raw::c_int == 0 as std::os::raw::c_int ||
               *(*__ctype_b_loc()).offset(*ptr as std::os::raw::c_int as isize) as
                   std::os::raw::c_int &
                   _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int !=
                   0 {
            /* "-option" or "-option arg" */
            while *(*__ctype_b_loc()).offset(*ptr as std::os::raw::c_int as isize) as
                      std::os::raw::c_int &
                      _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int
                      != 0 {
                ptr = ptr.offset(1)
            }
            *opt_arg_ptr =
                if *ptr as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    ptr
                } else { 0 as *const std::os::raw::c_char };
            break ;
        } else {
            if !(*ptr as std::os::raw::c_int == '=' as i32) { continue ; }
            /* "-option=arg" */
            ptr = ptr.offset(1);
            *opt_arg_ptr = ptr;
            break ;
        }
    }
    /* Finalize the normalized option. */
    if opt_buf_size > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        if (opt_len as std::os::raw::c_ulong) < opt_buf_size {
            *opt_buf.offset(opt_len as isize) = '\u{0}' as i32 as std::os::raw::c_char
        } else {
            *opt_buf.offset(opt_buf_size.wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) as
                                isize) = '\u{0}' as i32 as std::os::raw::c_char
        }
    }
    return 1 as std::os::raw::c_int;
}
/*
 * Command line parsing.
 */
unsafe extern "C" fn parse_args(mut argc: std::os::raw::c_int,
                                mut argv: *mut *mut std::os::raw::c_char) {
    let mut arg: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut opt: [std::os::raw::c_char; 16] = [0; 16];
    let mut opt_len: size_t = 0;
    let mut xopt: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut simple_opt: std::os::raw::c_int = 0;
    let mut stop_switch: std::os::raw::c_int = 0;
    let mut set: opng_bitset_t = 0;
    let mut val: std::os::raw::c_int = 0;
    let mut file_count: std::os::raw::c_uint = 0;
    let mut i: std::os::raw::c_int = 0;
    /* Initialize. */
    memset(&mut options as *mut opng_options as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<opng_options>() as std::os::raw::c_ulong);
    options.optim_level = -(1 as std::os::raw::c_int);
    options.interlace = -(1 as std::os::raw::c_int);
    file_count = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    /* Iterate over args. */
    stop_switch = 0 as std::os::raw::c_int;
    i = 1 as std::os::raw::c_int;
    while i < argc {
        let arg = unsafe { *argv.offset(i as isize) };
if stop_switch != 0 || scan_option(arg, opt.as_mut_ptr(), opt.len() as u64, &mut xopt) < 1 {
    file_count = file_count.wrapping_add(1);
    /* leave file names for process_files() */
} else {
     opt_len = opt.iter().position(|&c| c == 0).unwrap_or(opt.len()) as u64;
unsafe {
    *argv.offset(i as isize) = std::ptr::null_mut(); // Prevent process_files() from seeing this arg.
}

if let Some(&first_char) = opt.get(0) {
    if "fio".contains(first_char as u8 as char) && opt.get(1).map_or(false, |&c| (c as u8).is_ascii_digit())
        || (first_char == b'z' as i8 && opt.get(1).map_or(false, |&c| (c as u8).is_ascii_alphabetic())
        && opt.get(2).map_or(false, |&c| (c as u8).is_ascii_digit())) {
        
        // Normalize the options that allow juxtaposed arguments.
        opt_len = opng_strpbrk_digit(opt.as_mut_ptr()).offset_from(opt.as_mut_ptr()) as u64;
        opt[opt_len as usize] = 0; // Null-terminate the string
        xopt = opng_strpbrk_digit(arg);
    }
}

// Check the simple options (without option arguments).
simple_opt = 1;

           
     if strcmp(b"-\x00" as *const u8 as *const std::os::raw::c_char,
                      opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -- */
                stop_switch = 1 as std::os::raw::c_int
            } else if strcmp(b"?\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int ||
                          strncmp(b"help\x00" as *const u8 as
                                      *const std::os::raw::c_char, opt.as_mut_ptr(),
                                  opt_len) == 0 as std::os::raw::c_int {
                /* -? | -h | ... | -help */
                local_options.help = 1 as std::os::raw::c_int
            } else if strncmp(b"backup\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int ||
                          strncmp(b"keep\x00" as *const u8 as
                                      *const std::os::raw::c_char, opt.as_mut_ptr(),
                                  opt_len) == 0 as std::os::raw::c_int {
                /* -b | ... | -backup | -k | ... | -keep */
                options.backup = 1 as std::os::raw::c_int
            } else if strncmp(b"clobber\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int {
                /* -c | ... | -clobber */
                options.clobber = 1 as std::os::raw::c_int
            } else if strcmp(b"debug\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -debug */
            /* Do not abbreviate this internal option. */
                options.debug = 1 as std::os::raw::c_int
            } else if strncmp(b"fix\x00" as *const u8 as *const std::os::raw::c_char,
                              opt.as_mut_ptr(), opt_len) == 0 as std::os::raw::c_int
                          && opt_len >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -fi | -fix */
                options.fix = 1 as std::os::raw::c_int
            } else if strncmp(b"force\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int &&
                          opt_len >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -fo | ... | -force */
                options.force = 1 as std::os::raw::c_int
            } else if strncmp(b"full\x00" as *const u8 as *const std::os::raw::c_char,
                              opt.as_mut_ptr(), opt_len) == 0 as std::os::raw::c_int
                          && opt_len >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -fu | ... | -full */
                options.full = 1 as std::os::raw::c_int
            } else if strcmp(b"nb\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -nb */
                options.nb = 1 as std::os::raw::c_int
            } else if strcmp(b"nc\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -nc */
                options.nc = 1 as std::os::raw::c_int
            } else if strcmp(b"np\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -np */
                options.np = 1 as std::os::raw::c_int
            } else if strcmp(b"nx\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -nx */
                options.np = 1 as std::os::raw::c_int;
                options.nc = options.np;
                options.nb = options.nc
                /* options.nm = 1; */
            } else if strcmp(b"nz\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -nz */
                options.nz = 1 as std::os::raw::c_int
            } else if strncmp(b"preserve\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int {
                /* -p | ... | -preserve */
                options.preserve = 1 as std::os::raw::c_int
            } else if strncmp(b"quiet\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int ||
                          strncmp(b"silent\x00" as *const u8 as
                                      *const std::os::raw::c_char, opt.as_mut_ptr(),
                                  opt_len) == 0 as std::os::raw::c_int &&
                              opt_len >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -q | ... | -quiet | -sil | ... | -silent */
                options.quiet = 1 as std::os::raw::c_int
            } else if strncmp(b"simulate\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int &&
                          opt_len >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -sim | ... | -simulate */
                options.simulate = 1 as std::os::raw::c_int
            } else if strncmp(b"snip\x00" as *const u8 as *const std::os::raw::c_char,
                              opt.as_mut_ptr(), opt_len) == 0 as std::os::raw::c_int
                          && opt_len >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -sn | ... | -snip */
                options.snip = 1 as std::os::raw::c_int
            } else if strcmp(b"v\x00" as *const u8 as *const std::os::raw::c_char,
                             opt.as_mut_ptr()) == 0 as std::os::raw::c_int {
                /* -v */
                options.verbose = 1 as std::os::raw::c_int;
                local_options.version = 1 as std::os::raw::c_int
            } else if strncmp(b"verbose\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int &&
                          opt_len >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -verb | ... | -verbose */
                options.verbose = 1 as std::os::raw::c_int
            } else if strncmp(b"version\x00" as *const u8 as
                                  *const std::os::raw::c_char, opt.as_mut_ptr(),
                              opt_len) == 0 as std::os::raw::c_int &&
                          opt_len >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* -vers | ... | -version */
                local_options.version = 1 as std::os::raw::c_int
            } else {
                /* possibly an option with an argument */
                simple_opt = 0 as std::os::raw::c_int;
                if xopt.is_null() {
                    i += 1;
                    if i < argc {
                        xopt = *argv.offset(i as isize);
                        /* Prevent process_files() from seeing this xopt. */
                        let ref mut fresh7 = *argv.offset(i as isize);
                        *fresh7 = 0 as *mut std::os::raw::c_char
                    } else {
                        /* Last option in command line; assume an empty xopt. */
                        xopt = b"\x00" as *const u8 as *const std::os::raw::c_char
                    }
                }
            }
            
    /* Check the options that have option arguments. */
     if simple_opt != 0 {
    if !xopt.is_null() {
        error(b"No argument allowed for option: %s\x00" as *const u8 as *const std::os::raw::c_char, arg);
    }
} else if unsafe { *opt.as_mut_ptr() == b'o' as i8 } {
    /* -o NUM */
     val = check_num_option(b"-o\0".as_ptr() as *const std::os::raw::c_char, xopt, 0, 2147483647);
if options.optim_level < 0 {
    options.optim_level = val;
} else if options.optim_level != val {
    error(b"Multiple optimization levels are not permitted\0".as_ptr() as *const std::os::raw::c_char);
}

            
} else if unsafe { *opt.as_mut_ptr() == b'i' as i8 } {
    /* -i NUM */
     val = check_num_option(b"-i\0".as_ptr() as *const std::os::raw::c_char, xopt, 0, 1);
if options.interlace < 0 {
    options.interlace = val;
} else if options.interlace != val {
    error(b"Multiple interlace types are not permitted\0".as_ptr() as *const std::os::raw::c_char);
}

            
} else if unsafe { *opt.as_mut_ptr() == b'f' as i8 } {
    /* -f SET */
     set = check_rangeset_option(b"-f\0".as_ptr() as *const i8, xopt,
    ((1 << (5 + 1)) - (1 << 0)) as opng_bitset_t);
options.filter_set |= set;

            
} else if unsafe { *opt.as_mut_ptr() == b'z' as i8 && *opt.as_mut_ptr().add(1) == b'c' as i8 } {
    /* -zc SET */
     set = check_rangeset_option(b"-zc\x00".as_ptr() as *const i8, xopt,
    (((1 << 9) + 1) - (1 << 1)) as opng_bitset_t);
options.compr_level_set |= set;

            
} else if unsafe { *opt.as_mut_ptr() == b'z' as i8 && *opt.as_mut_ptr().add(1) == b'm' as i8 } {
    /* -zm SET */
     set = check_rangeset_option(b"-zm\x00".as_ptr() as *const i8, xopt,
    (((1 << 9) + 1) - (1 << 1)) as opng_bitset_t);
options.mem_level_set |= set;

            
} else if unsafe { *opt.as_mut_ptr() == b'z' as i8 && *opt.as_mut_ptr().add(1) == b's' as i8 } {
    /* -zs SET */
     set = check_rangeset_option(b"-zs\x00".as_ptr() as *const i8, xopt,
                            ((1 << (3 + 1)) - (1 << 0)) as opng_bitset_t);
options.strategy_set |= set;

            
} else if unsafe { *opt.as_mut_ptr() == b'z' as i8 && *opt.as_mut_ptr().add(1) == b'w' as i8 } {
    /* -zw NUM */
     val = check_power2_option(b"-zw\x00".as_ptr() as *const i8, xopt, 8, 15);
if options.window_bits == 0 {
    options.window_bits = val;
} else if options.window_bits != val {
    error(b"Multiple window sizes are not permitted\x00".as_ptr() as *const i8);
}

            
} else if strncmp(b"strip\x00" as *const u8 as *const std::os::raw::c_char, opt.as_mut_ptr(), opt_len) == 0 && opt_len >= 2 {
    /* -st OBJ | ... | -strip OBJ */
    check_obj_option(b"-strip\x00" as *const u8 as *const std::os::raw::c_char, xopt);
    options.strip_all = 1;
} else if strncmp(b"out\x00" as *const u8 as *const std::os::raw::c_char, opt.as_mut_ptr(), opt_len) == 0 && opt_len >= 2 {
    /* -ou PATH | -out PATH */
     if !options.out_name.is_null() {
    error("Multiple output file names are not permitted".as_ptr() as *const std::os::raw::c_char);
}

if xopt.is_null() || unsafe { *xopt.offset(0) } == 0 {
    err_option_arg("-out".as_ptr() as *const std::os::raw::c_char, std::ptr::null());
}

options.out_name = xopt;

            
} else if strncmp(b"dir\x00" as *const u8 as *const std::os::raw::c_char, opt.as_mut_ptr(), opt_len) == 0 && opt_len >= 2 {
    /* -d PATH | ... | -dir PATH */
     if options.dir_name.is_null() {
    error(b"Multiple output dir names are not permitted\x00" as *const u8 as *const std::os::raw::c_char);
}

if xopt.is_null() || unsafe { *xopt } == 0 {
    err_option_arg(b"-dir\x00" as *const u8 as *const std::os::raw::c_char, std::ptr::null());
}

options.dir_name = xopt;

            
} else if strncmp(b"log\x00" as *const u8 as *const std::os::raw::c_char, opt.as_mut_ptr(), opt_len) == 0 && opt_len >= 2 {
    /* -l PATH | ... | -log PATH */
     if !options.log_name.is_null() {
    error(b"Multiple log file names are not permitted\x00" as *const u8 as *const std::os::raw::c_char);
}

if xopt.is_null() || unsafe { *xopt } == 0 {
    err_option_arg(b"-log\x00" as *const u8 as *const std::os::raw::c_char, std::ptr::null());
}

options.log_name = xopt;

            
} else {
    error(b"Unrecognized option: %s\x00" as *const u8 as *const std::os::raw::c_char, arg);
}

        
}
i += 1;
/*
The variables live at this point are:
(mut argc: i32, mut argv: *mut *mut i8, mut arg: *mut i8, mut opt: [i8; 16], mut opt_len: u64, mut xopt: *const i8, mut simple_opt: i32, mut stop_switch: i32, mut set: u32, mut val: i32, mut file_count: u32, mut i: i32)
*/

    }
    /* Finalize. */
    if !options.out_name.is_null() {
        if file_count > 1 as std::os::raw::c_int as std::os::raw::c_uint {
            error(b"The option -out requires one input file\x00" as *const u8
                      as *const std::os::raw::c_char);
        }
        if !options.dir_name.is_null() {
            error(b"The options -out and -dir are mutually exclusive\x00" as
                      *const u8 as *const std::os::raw::c_char);
        }
    }
    if !options.log_name.is_null() {
        if opng_strcasecmp(b".log\x00" as *const u8 as *const std::os::raw::c_char,
                           opng_strtail(options.log_name,
                                        4 as std::os::raw::c_int as size_t)) !=
               0 as std::os::raw::c_int {
            error(b"To prevent accidental data corruption, the log file name must end with \".log\"\x00"
                      as *const u8 as *const std::os::raw::c_char);
        }
    }
    if local_options.help != 0 {
        operation = OP_SHOW_HELP
    } else if file_count != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        operation = OP_RUN
    } else if local_options.version != 0 {
        operation = OP_SHOW_VERSION
    } else { operation = OP_SHOW_HELP };
}
/*
 * Application-defined printf callback.
 */
unsafe extern "C" fn app_printf(mut fmt: *const std::os::raw::c_char, mut args: ...) {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    if *fmt.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0 as std::os::raw::c_int {
        return
    }
    start_of_line =
        if *fmt.offset(strlen(fmt).wrapping_sub(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong) as isize)
               as std::os::raw::c_int == '\n' as i32 {
            1 as std::os::raw::c_int
        } else { 0 as std::os::raw::c_int };
    if !con_file.is_null() {
        arg_ptr = args.clone();
        vfprintf(con_file, fmt, arg_ptr.as_va_list());
    }
    if !log_file.is_null() {
        arg_ptr = args.clone();
        vfprintf(log_file, fmt, arg_ptr.as_va_list());
    };
}
/*
 * Application-defined control print callback.
 */
unsafe extern "C" fn app_print_cntrl(cntrl_code: i32) {
    let mut con_str: *const i8;
    let mut log_str: *const i8;
    let mut i: i32;

    if cntrl_code == '\r' as i32 {
        con_str = b"\r\0" as *const u8 as *const i8;
        log_str = b"\n\0" as *const u8 as *const i8;
        start_of_line = 1;
    } else if cntrl_code == '\u{b}' as i32 {
        if start_of_line == 0 {
            log_str = b"\n\0" as *const u8 as *const i8;
            con_str = log_str;
            start_of_line = 1;
        } else {
            log_str = b"\0" as *const u8 as *const i8;
            con_str = log_str;
        }
    } else if cntrl_code < 0 && cntrl_code > -80 && start_of_line != 0 {
        if !con_file.is_null() {
            i = 0;
            while i > cntrl_code {
                fputc(' ' as i32, con_file);
                i -= 1;
            }
        }
        con_str = b"\r\0" as *const u8 as *const i8;
        log_str = b"\0" as *const u8 as *const i8;
    } else {
        log_str = b"<?>\0" as *const u8 as *const i8;
        con_str = log_str;
    }

    if !con_file.is_null() {
        fputs(con_str, con_file);
    }
    if !log_file.is_null() {
        fputs(log_str, log_file);
    }
}

/*
 * Application-defined progress update callback.
 */
unsafe extern "C" fn app_progress(mut current_step: std::os::raw::c_ulong,
                                  mut total_steps: std::os::raw::c_ulong) {
    /* There will be a potentially long wait, so flush the console output. */
    if !con_file.is_null() { fflush(con_file); }
    /* An eager flush of the line-buffered log file is not very important. */
    /* A GUI application would normally update a progress bar. */
    /* Here we ignore the progress info. */
    if current_step != 0 && total_steps != 0 { return };
}
/*
 * Application initialization.
 */
unsafe extern "C" fn app_init() {
    start_of_line = 1 as std::os::raw::c_int;
    if operation as std::os::raw::c_uint ==
           OP_SHOW_HELP as std::os::raw::c_int as std::os::raw::c_uint ||
           operation as std::os::raw::c_uint ==
               OP_SHOW_VERSION as std::os::raw::c_int as std::os::raw::c_uint {
        con_file = stdout
    } else if options.quiet == 0 {
        con_file = stderr
    } else { con_file = 0 as *mut FILE }
    if !options.log_name.is_null() {
        /* Open the log file, line-buffered. */
        log_file =
            fopen(options.log_name,
                  b"a\x00" as *const u8 as *const std::os::raw::c_char);
        if log_file.is_null() {
            error(b"Can\'t open log file: %s\n\x00" as *const u8 as
                      *const std::os::raw::c_char, options.log_name);
        }
        setvbuf(log_file, 0 as *mut std::os::raw::c_char, 1 as std::os::raw::c_int,
                8192 as std::os::raw::c_int as size_t);
        app_printf(b"** Warning: %s\n\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   b"The option -log is deprecated; use shell redirection\x00"
                       as *const u8 as *const std::os::raw::c_char);
    };
}
/*
 * Application finalization.
 */
fn app_finish() {
    unsafe {
        if !log_file.is_null() {
            /* Close the log file. */
            fclose(log_file);
        }
    }
}

/*
 * File list processing.
 */
fn process_files(argc: i32, argv: Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut ui = opng_ui {
        printf_fn: Some(app_printf),
        print_cntrl_fn: Some(app_print_cntrl),
        progress_fn: Some(app_progress),
        panic_fn: Some(panic),
    };

    // Initialize the optimization engine.
    unsafe {
        if opng_initialize(&mut options, &mut ui) != 0 {
            panic(CString::new("Can't initialize optimization engine").unwrap().as_ptr());
        }

        // Iterate over file names.
        for i in 1..argc {
            let arg = &argv[i as usize];
            if !arg.is_empty() {
                if opng_optimize(arg.as_ptr() as *const i8) != 0 {
                    result = 1;
                }
            }
        }

        // Finalize the optimization engine.
        if opng_finalize() != 0 {
            panic(CString::new("Can't finalize optimization engine").unwrap().as_ptr());
        }
    }
    
    result
}

/*
 * The main function.
 */
pub unsafe fn e3_main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut result: std::os::raw::c_int = 0;
    /* Parse the user options and initialize the application. */
    parse_args(argc, argv);
    app_init();
    result = 0 as std::os::raw::c_int;
    if local_options.version != 0 {
        /* Print the copyright and version info. */
        app_printf(b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                   msg_intro);
    }
    match operation as std::os::raw::c_uint {
        0 => {
    // Run the application.
    let argv_vec: Vec<String> = (0..argc).map(|i| {
        let c_str = unsafe { CStr::from_ptr(*argv.offset(i as isize)) };
        c_str.to_string_lossy().into_owned()
    }).collect();
    result = process_files(argc, argv_vec);
}
        1 => {
            if local_options.help != 0 {
                /* Print the extended help text. */
                app_printf(b"%s%s%s\x00" as *const u8 as *const std::os::raw::c_char,
                           msg_help_synopsis, msg_help_options,
                           msg_help_examples);
            } else {
                /* Print the basic help text. */
                app_printf(b"%s%s%s%s\x00" as *const u8 as
                               *const std::os::raw::c_char, msg_help_synopsis,
                           msg_help_basic_options, msg_help_examples,
                           msg_help_more);
            }
        }
        2 => {
            /* Print the licensing terms and the extended version info. */
            app_printf(b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                       msg_license);
            let libpng_version = png_get_libpng_ver(0 as png_const_structrp);
let zlib_version = zlibVersion();
app_printf(
    b"Using libpng version %s and zlib version %s\n\x00" as *const u8 as *const std::os::raw::c_char,
    libpng_version,
    zlib_version,
);
        }
        _ => { result = -(1 as std::os::raw::c_int) }
    }
    /* Finalize the application. */
    app_finish();
    return result;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(e3_main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}

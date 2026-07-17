



use std::io::Write;
use std::ffi::CString;

extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn unlink(__name: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    static mut stdin: *mut FILE;
    
    static mut stdout: *mut FILE;
    
    static mut stderr: *mut FILE;
    
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    
    fn ferror(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn perror(__s: *const std::os::raw::c_char);
    
    fn fileno(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn strrchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    
    fn exit(_: std::os::raw::c_int) -> !;
}
pub use crate::src::zlib::gzclose::gzclose;
pub use crate::src::zlib::gzlib::gzdopen;
pub use crate::src::zlib::gzlib::gzerror;
pub use crate::src::zlib::gzlib::gzopen;
pub use crate::src::zlib::gzread::gzread;
pub use crate::src::zlib::gzwrite::gzwrite;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type voidpc = crate::src::zlib::gzwrite::voidpc;
pub type voidp = crate::src::libpng::pngread::voidp;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
pub type off64_t = crate::src::zlib::adler32::off64_t;
// #[derive(Copy, Clone)]

pub type gzFile_s = crate::src::zlib::gzlib::gzFile_s;
pub type gzFile = crate::src::zlib::gzclose::gzFile;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::libpng::png::_IO_FILE;
pub type _IO_lock_t = crate::src::gifread::gifread::_IO_lock_t;
pub type FILE = crate::src::gifread::gifread::FILE;
static mut prog: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
/* ===========================================================================
 * Display error message and exit
 */

pub fn error(msg: &str) {
    let prog_str = unsafe { std::ffi::CStr::from_ptr(prog) };
    let prog_name = prog_str.to_string_lossy();
    eprintln!("{}: {}", prog_name, msg);
    std::process::exit(1);
}

/* ===========================================================================
 * Compress input to output then close both files.
 */

pub unsafe extern "C" fn gz_compress(mut in_0: *mut FILE, mut out: gzFile) {
    let mut buf: [std::os::raw::c_char; 16384] = [0; 16384];
    let mut len: std::os::raw::c_int = 0;
    let mut err: std::os::raw::c_int = 0;
    loop  {
        len =
            fread(buf.as_mut_ptr() as *mut std::os::raw::c_void,
                  1 as std::os::raw::c_int as std::os::raw::c_ulong,
                  ::std::mem::size_of::<[std::os::raw::c_char; 16384]>() as
                      std::os::raw::c_ulong, in_0) as std::os::raw::c_int;
        if ferror(in_0) != 0 {
            perror(b"fread\x00" as *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        if len == 0 as std::os::raw::c_int { break ; }
        if gzwrite(out, buf.as_mut_ptr() as voidpc, len as std::os::raw::c_uint) !=
               len {
            let error_message = gzerror(out, &mut err);
        }
    }
    fclose(in_0);
    if gzclose(out) != 0 as std::os::raw::c_int {
        error("failed gzclose");
    };
}
/* MMAP version, Miguel Albrecht <malbrech@eso.org> */
/* USE_MMAP */
/* ===========================================================================
 * Uncompress input to output then close both files.
 */

pub fn gz_uncompress(in_0: gzFile, out: &mut std::fs::File) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0; 16384];
    loop {
        let len = unsafe {
            gzread(in_0, buf.as_mut_ptr() as *mut std::os::raw::c_void, buf.len() as std::os::raw::c_uint)
        };
        if len < 0 {
            return Err("Error reading gzFile".into());
        }
        if len == 0 {
            break;
        }
        let bytes_written = out.write(&buf[..len as usize])?;
        if bytes_written != len as usize {
            return Err("failed fwrite".into());
        }
    }
    if unsafe { gzclose(in_0) } != 0 {
        return Err("failed gzclose".into());
    }
    Ok(())
}

/* ===========================================================================
 * Compress the given file: create a corresponding .gz file and remove the
 * original.
 */

pub unsafe extern "C" fn file_compress(mut file: *mut std::os::raw::c_char,
                                       mut mode: *mut std::os::raw::c_char) {
    let mut outfile: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut out: gzFile = 0 as *mut gzFile_s;
    if strlen(file).wrapping_add(strlen(b".gz\x00" as *const u8 as
                                            *const std::os::raw::c_char)) >=
           ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong {
        fprintf(stderr,
                b"%s: filename too long\n\x00" as *const u8 as
                    *const std::os::raw::c_char, prog);
        exit(1 as std::os::raw::c_int);
    }
    snprintf(outfile.as_mut_ptr(),
             ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
             b"%s%s\x00" as *const u8 as *const std::os::raw::c_char, file,
             b".gz\x00" as *const u8 as *const std::os::raw::c_char);
    in_0 = fopen(file, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    if in_0.is_null() { perror(file); exit(1 as std::os::raw::c_int); }
    out = gzopen(outfile.as_mut_ptr(), mode);
    if out.is_null() {
        fprintf(stderr,
                b"%s: can\'t gzopen %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, prog, outfile.as_mut_ptr());
        exit(1 as std::os::raw::c_int);
    }
    gz_compress(in_0, out);
    unlink(file);
}
/* ===========================================================================
 * Uncompress the given file and remove the original.
 */

pub fn file_uncompress(file: &str) {
    let mut buf = [0; 1024];
    let len = file.len();
    
    if len + 3 >= buf.len() {
        eprintln!("{}: filename too long", unsafe { std::ffi::CStr::from_ptr(prog).to_string_lossy() });
        std::process::exit(1);
    }
    
    buf[..len].copy_from_slice(file.as_bytes());
    
    let (infile, outfile) = if len > 3 && &file[len - 3..] == ".gz" {
        let mut outfile = file.to_string();
        outfile.truncate(len - 3);
        (file.to_string(), outfile)
    } else {
        let outfile = format!("{}{}", file, ".gz");
        (String::from_utf8_lossy(&buf[..len]).to_string(), outfile)
    };
    
    let in_0: gzFile;
    unsafe {
        in_0 = gzopen(infile.as_ptr() as *const i8, b"rb\0".as_ptr() as *const i8);
    }
    
    if in_0.is_null() {
        eprintln!("{}: can't gzopen {}", unsafe { std::ffi::CStr::from_ptr(prog).to_string_lossy() }, infile);
        std::process::exit(1);
    }
    
    let mut out = std::fs::File::create(&outfile).unwrap();
    gz_uncompress(in_0, &mut out).unwrap();
    std::fs::remove_file(infile).unwrap();
}

/* ===========================================================================
 * Usage:  minigzip [-c] [-d] [-f] [-h] [-r] [-1 to -9] [files...]
 *   -c : write to standard output
 *   -d : decompress
 *   -f : compress with Z_FILTERED
 *   -h : compress with Z_HUFFMAN_ONLY
 *   -r : compress with Z_RLE
 *   -1 to -9 : compression level
 */
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut copyout: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut uncompr: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut file: gzFile = 0 as *mut gzFile_s;
    let mut bname: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut outmode: [std::os::raw::c_char; 20] = [0; 20];
    snprintf(outmode.as_mut_ptr(),
             ::std::mem::size_of::<[std::os::raw::c_char; 20]>() as std::os::raw::c_ulong,
             b"%s\x00" as *const u8 as *const std::os::raw::c_char,
             b"wb6 \x00" as *const u8 as *const std::os::raw::c_char);
    prog = *argv.offset(0 as std::os::raw::c_int as isize);
    bname = strrchr(*argv.offset(0 as std::os::raw::c_int as isize), '/' as i32);
    if !bname.is_null() {
        bname = bname.offset(1)
    } else { bname = *argv.offset(0 as std::os::raw::c_int as isize) }
    argc -= 1;
    argv = argv.offset(1);
    if strcmp(bname, b"gunzip\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        uncompr = 1 as std::os::raw::c_int
    } else if strcmp(bname, b"zcat\x00" as *const u8 as *const std::os::raw::c_char)
                  == 0 {
        uncompr = 1 as std::os::raw::c_int;
        copyout = uncompr
    }
    while argc > 0 as std::os::raw::c_int {
        if strcmp(*argv, b"-c\x00" as *const u8 as *const std::os::raw::c_char) ==
               0 as std::os::raw::c_int {
            copyout = 1 as std::os::raw::c_int
        } else if strcmp(*argv, b"-d\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            uncompr = 1 as std::os::raw::c_int
        } else if strcmp(*argv, b"-f\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            outmode[3 as std::os::raw::c_int as usize] = 'f' as i32 as std::os::raw::c_char
        } else if strcmp(*argv, b"-h\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            outmode[3 as std::os::raw::c_int as usize] = 'h' as i32 as std::os::raw::c_char
        } else if strcmp(*argv, b"-r\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            outmode[3 as std::os::raw::c_int as usize] = 'R' as i32 as std::os::raw::c_char
        } else {
            if !(*(*argv).offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                     '-' as i32 &&
                     *(*argv).offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                         >= '1' as i32 &&
                     *(*argv).offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                         <= '9' as i32 &&
                     *(*argv).offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                         == 0 as std::os::raw::c_int) {
                break ;
            }
            outmode[2 as std::os::raw::c_int as usize] =
                *(*argv).offset(1 as std::os::raw::c_int as isize)
        }
        argc -= 1;
        argv = argv.offset(1)
    }
    if outmode[3 as std::os::raw::c_int as usize] as std::os::raw::c_int == ' ' as i32 {
        outmode[3 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char
    }
    if argc == 0 as std::os::raw::c_int {
        if uncompr != 0 {
            file =
                gzdopen(fileno(stdin),
                        b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            if file.is_null() {
                error("can't gzdopen stdin");
            }
            let file = unsafe { gzopen(CString::new("path/to/file.gz").unwrap().as_ptr(), CString::new("rb").unwrap().as_ptr()) }; // Assuming file is opened here
let mut stdout_file = std::fs::File::create("/dev/stdout").unwrap();
gz_uncompress(file, &mut stdout_file).unwrap();
        } else {
            file = gzdopen(fileno(stdout), outmode.as_mut_ptr());
            if file.is_null() {
                error("can't gzdopen stdout");
            }
            gz_compress(stdin, file);
        }
    } else {
        (copyout) != 0;
        loop  {
            if uncompr != 0 {
                if copyout != 0 {
                    file =
                        gzopen(*argv,
                               b"rb\x00" as *const u8 as *const std::os::raw::c_char);
                    if file.is_null() {
                        fprintf(stderr,
                                b"%s: can\'t gzopen %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, prog, *argv);
                    } else { let file = unsafe { gzopen(CString::new("path/to/file.gz").unwrap().as_ptr(), CString::new("rb").unwrap().as_ptr()) }; // Assuming file is opened here
let mut stdout_file = std::fs::File::create("/dev/stdout").unwrap();
gz_uncompress(file, &mut stdout_file).unwrap(); }
                } else { file_uncompress(unsafe { std::ffi::CStr::from_ptr(*argv).to_string_lossy().as_ref() }); }
            } else if copyout != 0 {
                let mut in_0: *mut FILE =
                    fopen(*argv,
                          b"rb\x00" as *const u8 as *const std::os::raw::c_char);
                if in_0.is_null() {
                    perror(*argv);
                } else {
                    file = gzdopen(fileno(stdout), outmode.as_mut_ptr());
                    if file.is_null() {
                        error("can't gzdopen stdout");
                    }
                    gz_compress(in_0, file);
                }
            } else { file_compress(*argv, outmode.as_mut_ptr()); }
            argv = argv.offset(1);
            argc -= 1;
            if !(argc != 0) { break ; }
        }
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let argc = args.len() as std::os::raw::c_int;
    let argv: Vec<std::ffi::CString> = args.iter()
        .map(|arg| std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();

    let mut argv_ptr: Vec<*mut std::os::raw::c_char> = argv.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .chain(std::iter::once(std::ptr::null_mut()))
        .collect();

    // Convert argv_ptr to a mutable pointer
    let argv_mut_ptr = argv_ptr.as_mut_ptr();

    ::std::process::exit(unsafe { main_0(argc, argv_mut_ptr) } as i32)
}


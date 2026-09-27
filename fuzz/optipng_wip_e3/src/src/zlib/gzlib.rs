



use std::result::Result;

use std::ptr;

use std::ffi::CStr;

extern "C" {
    
    
    fn lseek64(__fd: std::os::raw::c_int, __offset: __off64_t, __whence: std::os::raw::c_int)
     -> __off64_t;
    
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn free(__ptr: *mut std::os::raw::c_void);
    
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...)
     -> std::os::raw::c_int;
}
pub use crate::src::zlib::deflate::internal_state;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
pub type off_t = crate::src::zlib::adler32::off_t;
pub type off64_t = crate::src::zlib::adler32::off64_t;
pub type z_size_t = crate::src::zlib::adler32::z_size_t;
pub type Byte = crate::src::libpng::png::Byte;
pub type uInt = crate::src::libpng::png::uInt;
pub type uLong = crate::src::libpng::png::uLong;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type free_func = crate::src::libpng::png::free_func;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::pngerror::z_stream_s;
pub type z_stream = crate::src::libpng::png::z_stream;
// #[derive(Copy, Clone)]

pub type gzFile_s = crate::src::zlib::gzclose::gzFile_s;
pub type gzFile = crate::src::zlib::gzclose::gzFile;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: std::os::raw::c_int,
    pub fd: std::os::raw::c_int,
    pub path: *mut std::os::raw::c_char,
    pub size: std::os::raw::c_uint,
    pub want: std::os::raw::c_uint,
    pub in_0: *mut std::os::raw::c_uchar,
    pub out: *mut std::os::raw::c_uchar,
    pub direct: std::os::raw::c_int,
    pub how: std::os::raw::c_int,
    pub start: off64_t,
    pub eof: std::os::raw::c_int,
    pub past: std::os::raw::c_int,
    pub level: std::os::raw::c_int,
    pub strategy: std::os::raw::c_int,
    pub skip: off64_t,
    pub seek: std::os::raw::c_int,
    pub err: std::os::raw::c_int,
    pub msg: *mut std::os::raw::c_char,
    pub strm: z_stream,
}
/* Local functions */
/* UNDER_CE */
/* Reset gzip file state */
fn gz_reset(state: *mut gz_state) {
    let state = unsafe { &mut *state };
    state.x.have = 0; // no output data available
    if state.mode == 7247 {
        // for reading ...
        state.eof = 0; // not at end of file
        // look for gzip header
        state.past = 0; // have not read past end yet
        state.how = 0; // no seek request pending
    }
    state.seek = 0; // clear error
    unsafe {
        gz_error(state, 0, std::ptr::null()); // no uncompressed data yet
    }
    state.x.pos = 0; // reset position
    state.strm.avail_in = 0; // no input data yet
}

/* Open a gzip file either by name or file descriptor. */
unsafe extern "C" fn gz_open(mut path: *const std::os::raw::c_void,
                             mut fd: std::os::raw::c_int,
                             mut mode: *const std::os::raw::c_char) -> gzFile {
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut len: z_size_t = 0;
    let mut oflag: std::os::raw::c_int = 0;
    let mut cloexec: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut exclusive: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /* check input */
    if path.is_null() { return 0 as gzFile }
    /* allocate gzFile structure to return */
    state =
        malloc(::std::mem::size_of::<gz_state>() as std::os::raw::c_ulong) as
            gz_statep; /* no buffers allocated yet */
    if state.is_null() { return 0 as gzFile } /* requested buffer size */
    (*state).size =
        0 as std::os::raw::c_int as std::os::raw::c_uint; /* no error message yet */
    (*state).want = 8192 as std::os::raw::c_int as std::os::raw::c_uint;
    (*state).msg = 0 as *mut std::os::raw::c_char;
    /* interpret mode */
    (*state).mode = 0 as std::os::raw::c_int;
    (*state).level = -(1 as std::os::raw::c_int);
    (*state).strategy = 0 as std::os::raw::c_int;
    (*state).direct = 0 as std::os::raw::c_int;
    while *mode != 0 {
        if *mode as std::os::raw::c_int >= '0' as i32 &&
               *mode as std::os::raw::c_int <= '9' as i32 {
            (*state).level = *mode as std::os::raw::c_int - '0' as i32
        } else {
            match *mode as std::os::raw::c_int {
                114 => { (*state).mode = 7247 as std::os::raw::c_int }
                43 => {
                    /* can't read and write at the same time */
                    free(state as *mut std::os::raw::c_void);
                    return 0 as gzFile
                }
                98 => { }
                101 => { cloexec = 1 as std::os::raw::c_int }
                120 => { exclusive = 1 as std::os::raw::c_int }
                102 => { (*state).strategy = 1 as std::os::raw::c_int }
                104 => { (*state).strategy = 2 as std::os::raw::c_int }
                82 => { (*state).strategy = 3 as std::os::raw::c_int }
                70 => { (*state).strategy = 4 as std::os::raw::c_int }
                84 => { (*state).direct = 1 as std::os::raw::c_int }
                _ => { }
            }
        }
        mode = mode.offset(1)
    }
    /* must provide an "r", "w", or "a" */
    if (*state).mode == 0 as std::os::raw::c_int {
        free(state as *mut std::os::raw::c_void);
        return 0 as gzFile
    }
    /* can't force transparent read */
    if (*state).mode == 7247 as std::os::raw::c_int {
        if (*state).direct != 0 {
            free(state as *mut std::os::raw::c_void);
            return 0 as gzFile
        }
        (*state).direct = 1 as std::os::raw::c_int
        /* for empty file */
    }
    /* save the path name for error messages */
    len = strlen(path as *const std::os::raw::c_char);
    (*state).path =
        malloc(len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if (*state).path.is_null() {
        free(state as *mut std::os::raw::c_void);
        return 0 as gzFile
    }
    snprintf((*state).path,
             len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
             b"%s\x00" as *const u8 as *const std::os::raw::c_char,
             path as *const std::os::raw::c_char);
    /* compute the flags for open() */
    oflag =
        0 as std::os::raw::c_int |
            (if cloexec != 0 {
                 0o2000000 as std::os::raw::c_int
             } else { 0 as std::os::raw::c_int }) |
            (if (*state).mode == 7247 as std::os::raw::c_int {
                 0 as std::os::raw::c_int
             } else {
                 (0o1 as std::os::raw::c_int | 0o100 as std::os::raw::c_int |
                      (if exclusive != 0 {
                           0o200 as std::os::raw::c_int
                       } else { 0 as std::os::raw::c_int })) |
                     (if (*state).mode == 31153 as std::os::raw::c_int {
                          0o1000 as std::os::raw::c_int
                      } else { 0o2000 as std::os::raw::c_int })
             });
    /* open the file with the appropriate flags (or just use fd) */
    (*state).fd =
        if fd > -(1 as std::os::raw::c_int) {
            fd
        } else {
            open(path as *const std::os::raw::c_char, oflag, 0o666 as std::os::raw::c_int)
        }; /* so gzoffset() is correct */
    if (*state).fd == -(1 as std::os::raw::c_int) {
        free((*state).path as *mut std::os::raw::c_void);
        free(state as *mut std::os::raw::c_void);
        return 0 as gzFile
    }
    if (*state).mode == 1 as std::os::raw::c_int {
        lseek64((*state).fd, 0 as std::os::raw::c_int as __off64_t, 2 as std::os::raw::c_int);
        (*state).mode = 31153 as std::os::raw::c_int
        /* simplify later checks */
    }
    /* save the current position for rewinding (only if reading) */
    if (*state).mode == 7247 as std::os::raw::c_int {
        (*state).start =
            lseek64((*state).fd, 0 as std::os::raw::c_int as __off64_t,
                    1 as std::os::raw::c_int);
        if (*state).start == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
            (*state).start = 0 as std::os::raw::c_int as off64_t
        }
    }
    /* initialize stream */
    gz_reset(state);
    /* return stream */
    return state as gzFile;
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzopen(mut path: *const std::os::raw::c_char,
                                mut mode: *const std::os::raw::c_char) -> gzFile {
    return gz_open(path as *const std::os::raw::c_void, -(1 as std::os::raw::c_int), mode);
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzopen64(mut path: *const std::os::raw::c_char,
                                  mut mode: *const std::os::raw::c_char) -> gzFile {
    return gz_open(path as *const std::os::raw::c_void, -(1 as std::os::raw::c_int), mode);
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzdopen(mut fd: std::os::raw::c_int,
                                 mut mode: *const std::os::raw::c_char) -> gzFile {
    let mut path: *mut std::os::raw::c_char =
        0 as *mut std::os::raw::c_char; /* identifier for error messages */
    let mut gz: gzFile = 0 as *mut gzFile_s;
    if fd == -(1 as std::os::raw::c_int) ||
           {
               path =
                   malloc((7 as std::os::raw::c_int as
                               std::os::raw::c_ulong).wrapping_add((3 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                                                                                as
                                                                                                std::os::raw::c_ulong)))
                       as *mut std::os::raw::c_char;
               path.is_null()
           } {
        return 0 as gzFile
    }
    snprintf(path,
             (7 as std::os::raw::c_int as
                  std::os::raw::c_ulong).wrapping_add((3 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                                                                   as
                                                                                   std::os::raw::c_ulong)),
             b"<fd:%d>\x00" as *const u8 as *const std::os::raw::c_char, fd);
    gz = gz_open(path as *const std::os::raw::c_void, fd, mode);
    free(path as *mut std::os::raw::c_void);
    return gz;
}
/* -- see zlib.h -- */
/* -- see zlib.h -- */

pub fn gzbuffer(file: Option<gzFile>, mut size: u32) -> i32 {
    // Check if the file is null
    if file.is_none() {
        return -1;
    }

    let state = unsafe { &mut *(file.unwrap() as *mut gz_state) };

    // Check integrity of the state
    if state.mode != 7247 && state.mode != 31153 {
        return -1;
    }

    // Make sure we haven't already allocated memory
    if state.size != 0 {
        return -1;
    }

    // Check and set requested size
    if size.checked_shl(1).is_none() {
        return -1; // need to be able to double it
    }

    if size < 2 {
        size = 2; // need two bytes to check magic header
    }

    state.want = size;
    return 0;
}

/* -- see zlib.h -- */

pub unsafe extern "C" fn gzrewind(mut file: gzFile) -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return -(1 as std::os::raw::c_int) }
    state = file as gz_statep;
    /* check that we're reading and that there's no error */
    if (*state).mode != 7247 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int &&
               (*state).err != -(5 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    /* back up and start over */
    if lseek64((*state).fd, (*state).start, 0 as std::os::raw::c_int) ==
           -(1 as std::os::raw::c_int) as std::os::raw::c_long {
        return -(1 as std::os::raw::c_int)
    }
    gz_reset(state);
    return 0 as std::os::raw::c_int;
}
/* -- see zlib.h -- */

pub fn gzseek64(file: gzFile, mut offset: off64_t, whence: i32) -> Result<off64_t, i32> {
    let mut n: u32 = 0;
    let mut ret: off64_t = 0;
    let state: &mut gz_state = unsafe {
        if file.is_null() {
            return Err(-1);
        }
        &mut *(file as *mut gz_state)
    };

    if state.mode != 7247 && state.mode != 31153 {
        return Err(-1);
    }

    if state.err != 0 && state.err != -5 {
        return Err(-1);
    }

    if whence != 0 && whence != 1 {
        return Err(-1);
    }

    if whence == 0 {
        offset -= state.x.pos;
    } else if state.seek != 0 {
        offset += state.skip;
    }
    state.seek = 0;

    if state.mode == 7247 && state.how == 1 && state.x.pos + offset >= 0 {
        ret = unsafe { lseek64(state.fd, offset - state.x.have as i64, 1) };
        if ret == -1 {
            return Err(-1);
        }
        state.x.have = 0;
        state.eof = 0;
        state.past = 0;
        state.seek = 0;
        unsafe { gz_error(state, 0, std::ptr::null()) };
        state.strm.avail_in = 0;
        state.x.pos += offset;
        return Ok(state.x.pos);
    }

    if offset < 0 {
        if state.mode != 7247 {
            return Err(-1);
        }
        offset += state.x.pos;
        if offset < 0 {
            return Err(-1);
        }
        if unsafe { gzrewind(file) } == -1 {
            return Err(-1);
        }
    }

    if state.mode == 7247 {
        n = if std::mem::size_of::<i32>() == std::mem::size_of::<off64_t>() &&
               state.x.have > 2147483647 || state.x.have as off64_t > offset {
            offset as u32
        } else {
            state.x.have
        };
        state.x.have -= n;
        state.x.next = unsafe { state.x.next.add(n as usize) };
        state.x.pos += n as i64;
        offset -= n as i64;
    }

    if offset != 0 {
        state.seek = 1;
        state.skip = offset;
    }
    Ok(state.x.pos + offset)
}

/* -- see zlib.h -- */

pub fn gzseek(file: gzFile, offset: off_t, whence: std::os::raw::c_int) -> Result<off_t, std::os::raw::c_int> {
    match gzseek64(file, offset, whence) {
        Ok(ret) if ret >= 0 => Ok(ret as off_t),
        _ => Err(-(1 as std::os::raw::c_int)),
    }
}

/* -- see zlib.h -- */

pub unsafe extern "C" fn gztell64(mut file: gzFile) -> off64_t {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return -(1 as std::os::raw::c_int) as off64_t }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    /* return position */
    return (*state).x.pos +
               (if (*state).seek != 0 {
                    (*state).skip
                } else { 0 as std::os::raw::c_int as std::os::raw::c_long });
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gztell(mut file: gzFile) -> off_t {
    let mut ret: off64_t = 0;
    ret = gztell64(file);
    return if ret == ret { ret } else { -(1 as std::os::raw::c_int) as std::os::raw::c_long };
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzoffset64(mut file: gzFile) -> off64_t {
    let mut offset: off64_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return -(1 as std::os::raw::c_int) as off64_t }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    /* compute and return effective offset in file */
    offset =
        lseek64((*state).fd, 0 as std::os::raw::c_int as __off64_t,
                1 as std::os::raw::c_int); /* don't count buffered input */
    if offset == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    if (*state).mode == 7247 as std::os::raw::c_int {
        /* reading */
        offset -= (*state).strm.avail_in as std::os::raw::c_long
    }
    return offset;
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzoffset(mut file: gzFile) -> off_t {
    let mut ret: off64_t = 0;
    ret = gzoffset64(file);
    return if ret == ret { ret } else { -(1 as std::os::raw::c_int) as std::os::raw::c_long };
}
/* -- see zlib.h -- */

pub fn gzeof(file: Option<gzFile>) -> std::os::raw::c_int {
    let state: Option<&gz_state> = file.map(|f| f as *const gz_state).and_then(|ptr| unsafe { ptr.as_ref() });

    // Check if the file is null or the state is invalid
    if state.is_none() || (state.unwrap().mode != 7247 && state.unwrap().mode != 31153) {
        return 0;
    }

    // Return end-of-file state
    if state.unwrap().mode == 7247 {
        return state.unwrap().past;
    }
    
    0
}

/* -- see zlib.h -- */

pub fn gzerror(file: gzFile, errnum: &mut i32) -> Option<&'static str> {
    if file.is_null() {
        return None;
    }

    let state: &gz_state = unsafe { &*(file as *const gz_state) };

    if state.mode != 7247 && state.mode != 31153 {
        return None;
    }

    *errnum = state.err;
    if state.err == -4 {
        return Some("out of memory");
    } else if state.msg.is_null() {
        return Some("");
    } else {
        return Some(unsafe { std::ffi::CStr::from_ptr(state.msg).to_str().unwrap() });
    }
}

/* -- see zlib.h -- */

pub unsafe extern "C" fn gzclearerr(mut file: gzFile) {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return
    }
    /* clear error and end-of-file */
    if (*state).mode == 7247 as std::os::raw::c_int {
        (*state).eof = 0 as std::os::raw::c_int;
        (*state).past = 0 as std::os::raw::c_int
    }
    gz_error(state, 0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
}
/* Create an error message in allocated memory and set state->err and
   state->msg accordingly.  Free any previous error message already there.  Do
   not try to free or allocate space if the error is Z_MEM_ERROR (out of
   memory).  Simply save the error message as a static string.  If there is an
   allocation failure constructing the error message, then convert the error to
   out of memory. */

pub unsafe extern "C" fn gz_error(mut state: gz_statep, mut err: std::os::raw::c_int,
                                  mut msg: *const std::os::raw::c_char) {
    /* free previously allocated message and clear */
    if !(*state).msg.is_null() {
        if (*state).err != -(4 as std::os::raw::c_int) {
            free((*state).msg as *mut std::os::raw::c_void);
        }
        (*state).msg = 0 as *mut std::os::raw::c_char
    }
    /* if fatal, set state->x.have to 0 so that the gzgetc() macro fails */
    if err != 0 as std::os::raw::c_int && err != -(5 as std::os::raw::c_int) {
        (*state).x.have = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* set error code, and if no message, then done */
    (*state).err = err;
    if msg.is_null() { return }
    /* for an out of memory error, return literal string when requested */
    if err == -(4 as std::os::raw::c_int) { return }
    /* construct error message with path */
    (*state).msg =
        malloc(strlen((*state).path).wrapping_add(strlen(msg)).wrapping_add(3
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_ulong))
            as *mut std::os::raw::c_char;
    if (*state).msg.is_null() { (*state).err = -(4 as std::os::raw::c_int); return }
    snprintf((*state).msg,
             strlen((*state).path).wrapping_add(strlen(msg)).wrapping_add(3 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong),
             b"%s%s%s\x00" as *const u8 as *const std::os::raw::c_char, (*state).path,
             b": \x00" as *const u8 as *const std::os::raw::c_char, msg);
}

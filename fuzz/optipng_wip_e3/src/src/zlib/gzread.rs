




use std::ptr;

extern "C" {
    
    
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    
    fn read(__fd: std::os::raw::c_int, __buf: *mut std::os::raw::c_void, __nbytes: size_t)
     -> ssize_t;
    
    
    
    
    
    
    
    
    
    fn __errno_location() -> *mut std::os::raw::c_int;
    
    fn strerror(_: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    
    fn free(__ptr: *mut std::os::raw::c_void);
    
    
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn memchr(_: *const std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
}
pub use crate::src::zlib::gzlib::gz_error;
pub use crate::src::zlib::inflate::inflate;
pub use crate::src::zlib::inflate::inflateEnd;
pub use crate::src::zlib::inflate::inflateInit2_;
pub use crate::src::zlib::inflate::inflateReset;
pub use crate::src::zlib::deflate::internal_state;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
pub type __ssize_t = std::os::raw::c_long;
pub type off64_t = crate::src::zlib::adler32::off64_t;
pub type ssize_t = __ssize_t;
pub type z_size_t = crate::src::zlib::adler32::z_size_t;
pub type Byte = crate::src::libpng::png::Byte;
pub type uInt = crate::src::libpng::png::uInt;
pub type uLong = crate::src::libpng::png::uLong;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type voidp = crate::src::libpng::pngread::voidp;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type free_func = crate::src::libpng::png::free_func;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::pngerror::z_stream_s;
pub type z_stream = crate::src::libpng::png::z_stream;
pub type z_streamp = crate::src::libpng::png::z_streamp;
// #[derive(Copy, Clone)]

pub type gzFile_s = crate::src::zlib::gzlib::gzFile_s;
pub type gzFile = crate::src::zlib::gzclose::gzFile;
pub type gz_statep = crate::src::zlib::gzlib::gz_statep;
// #[derive(Copy, Clone)]

pub type gz_state = crate::src::zlib::gzlib::gz_state;
/* gzread.c -- zlib functions for reading gzip files
 * Copyright (C) 2004, 2005, 2010, 2011, 2012, 2013, 2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* Local functions */
/* Use read() to load a buffer -- return -1 on error, otherwise 0.  Read from
   state->fd, and update state->eof, state->err, and state->msg as appropriate.
   This function needs to loop on read(), since read() is not guaranteed to
   read the number of bytes requested, depending on the type of descriptor. */
unsafe extern "C" fn gz_load(mut state: gz_statep,
                             mut buf: *mut std::os::raw::c_uchar,
                             mut len: std::os::raw::c_uint,
                             mut have: *mut std::os::raw::c_uint) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut get: std::os::raw::c_uint = 0;
    let mut max: std::os::raw::c_uint =
        (-(1 as std::os::raw::c_int) as std::os::raw::c_uint >>
             2 as std::os::raw::c_int).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
    *have = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    loop  {
        get = len.wrapping_sub(*have);
        if get > max { get = max }
        ret =
            read((*state).fd, buf.offset(*have as isize) as *mut std::os::raw::c_void,
                 get as size_t) as std::os::raw::c_int;
        if ret <= 0 as std::os::raw::c_int { break ; }
        *have = (*have).wrapping_add(ret as std::os::raw::c_uint);
        if !(*have < len) { break ; }
    }
    if ret < 0 as std::os::raw::c_int {
        gz_error(state, -(1 as std::os::raw::c_int), strerror(*__errno_location()));
        return -(1 as std::os::raw::c_int)
    }
    if ret == 0 as std::os::raw::c_int { (*state).eof = 1 as std::os::raw::c_int }
    return 0 as std::os::raw::c_int;
}
/* Load up input buffer and set eof flag if last data loaded -- return -1 on
   error, 0 otherwise.  Note that the eof flag is set when the end of the input
   file is reached, even though there may be unused data in the buffer.  Once
   that data has been used, no more attempts will be made to read the file.
   If strm->avail_in != 0, then the current data is moved to the beginning of
   the input buffer, and then the remainder of the buffer is loaded with the
   available data from the input file. */
unsafe extern "C" fn gz_avail(mut state: gz_statep) -> std::os::raw::c_int {
    let mut got: std::os::raw::c_uint = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    if (*state).err != 0 as std::os::raw::c_int && (*state).err != -(5 as std::os::raw::c_int)
       {
        return -(1 as std::os::raw::c_int)
    }
    if (*state).eof == 0 as std::os::raw::c_int {
        if (*strm).avail_in != 0 {
            /* copy what's there to the start */
            let mut p: *mut std::os::raw::c_uchar = (*state).in_0;
            let mut q: *const std::os::raw::c_uchar = (*strm).next_in as *mut u8;
            let mut n: std::os::raw::c_uint = (*strm).avail_in;
            loop  {
                let fresh0 = q;
                q = q.offset(1);
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = *fresh0;
                n = n.wrapping_sub(1);
                if !(n != 0) { break ; }
            }
        }
        if gz_load(state, (*state).in_0.offset((*strm).avail_in as isize),
                   (*state).size.wrapping_sub((*strm).avail_in), &mut got) ==
               -(1 as std::os::raw::c_int) {
            return -(1 as std::os::raw::c_int)
        }
        (*strm).avail_in =
            ((*strm).avail_in as std::os::raw::c_uint).wrapping_add(got) as uInt as
                uInt;
        (*strm).next_in = (*state).in_0
    }
    return 0 as std::os::raw::c_int;
}
/* Look for gzip header, set up for inflate or copy.  state->x.have must be 0.
   If this is the first time in, allocate required memory.  state->how will be
   left unchanged if there is no more input data available, will be set to COPY
   if there is no gzip header and direct copying will be performed, or it will
   be set to GZIP for decompression.  If direct copying, then leftover input
   data from the input buffer will be copied to the output buffer.  In that
   case, all further file reads will be directly to either the output buffer or
   a user buffer.  If decompressing, the inflate state will be initialized.
   gz_look() will return 0 on success or -1 on failure. */
unsafe extern "C" fn gz_look(mut state: gz_statep) -> std::os::raw::c_int {
    let mut strm: z_streamp = &mut (*state).strm;
    /* allocate read buffers and inflate memory */
    if (*state).size == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        /* allocate buffers */
        (*state).in_0 =
            malloc((*state).want as std::os::raw::c_ulong) as *mut std::os::raw::c_uchar;
        (*state).out =
            malloc(((*state).want << 1 as std::os::raw::c_int) as std::os::raw::c_ulong) as
                *mut std::os::raw::c_uchar;
        if (*state).in_0.is_null() || (*state).out.is_null() {
            free((*state).out as *mut std::os::raw::c_void);
            free((*state).in_0 as *mut std::os::raw::c_void);
            gz_error(state, -(4 as std::os::raw::c_int),
                     b"out of memory\x00" as *const u8 as
                         *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        (*state).size = (*state).want;
        /* allocate inflate memory */
        (*state).strm.zalloc = None;
        (*state).strm.zfree = None;
        (*state).strm.opaque = 0 as voidpf;
        (*state).strm.avail_in = 0 as std::os::raw::c_int as uInt;
        (*state).strm.next_in = 0 as *mut Bytef;
        if inflateInit2_(&mut (*state).strm,
                         15 as std::os::raw::c_int + 16 as std::os::raw::c_int,
                         b"1.2.11-optipng\x00" as *const u8 as
                             *const std::os::raw::c_char,
                         ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                             std::os::raw::c_int) != 0 as std::os::raw::c_int {
            /* gunzip */
            free((*state).out as *mut std::os::raw::c_void);
            free((*state).in_0 as *mut std::os::raw::c_void);
            (*state).size = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            gz_error(state, -(4 as std::os::raw::c_int),
                     b"out of memory\x00" as *const u8 as
                         *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    /* get at least the magic bytes in the input buffer */
    if (*strm).avail_in < 2 as std::os::raw::c_int as std::os::raw::c_uint {
        if gz_avail(state) == -(1 as std::os::raw::c_int) {
            return -(1 as std::os::raw::c_int)
        }
        if (*strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as std::os::raw::c_int
        }
    }
    /* look for gzip magic bytes -- if there, do gzip decoding (note: there is
       a logical dilemma here when considering the case of a partially written
       gzip file, to wit, if a single 31 byte is written, then we cannot tell
       whether this is a single-byte file, or just a partially written gzip
       file -- for here we assume that if a gzip file is being written, then
       the header will be written in a single operation, so that reading a
       single byte is sufficient indication that it is not a gzip file) */
    if (*strm).avail_in > 1 as std::os::raw::c_int as std::os::raw::c_uint &&
           *(*strm).next_in.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
               == 31 as std::os::raw::c_int &&
           *(*strm).next_in.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
               == 139 as std::os::raw::c_int {
        inflateReset(strm);
        (*state).how = 2 as std::os::raw::c_int;
        (*state).direct = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    /* no gzip header -- if we were decoding gzip before, then this is trailing
       garbage.  Ignore the trailing garbage and finish. */
    if (*state).direct == 0 as std::os::raw::c_int {
        (*strm).avail_in = 0 as std::os::raw::c_int as uInt;
        (*state).eof = 1 as std::os::raw::c_int;
        (*state).x.have = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        return 0 as std::os::raw::c_int
    }
    /* doing raw i/o, copy any leftover input to output -- this assumes that
       the output buffer is larger than the input buffer, which also assures
       space for gzungetc() */
    (*state).x.next = (*state).out;
    if (*strm).avail_in != 0 {
        memcpy((*state).x.next as *mut std::os::raw::c_void,
               (*strm).next_in as *const std::os::raw::c_void,
               (*strm).avail_in as std::os::raw::c_ulong);
        (*state).x.have = (*strm).avail_in;
        (*strm).avail_in = 0 as std::os::raw::c_int as uInt
    }
    (*state).how = 1 as std::os::raw::c_int;
    (*state).direct = 1 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
/* Decompress from input to the provided next_out and avail_out in the state.
   On return, state->x.have and state->x.next point to the just decompressed
   data.  If the gzip stream completes, state->how is reset to LOOK to look for
   the next gzip stream or raw data, once state->x.have is depleted.  Returns 0
   on success, -1 on failure. */
fn gz_decomp(state: gz_statep) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut had: std::os::raw::c_uint = 0;
    let strm = unsafe { &mut (*state).strm };

    // Fill output buffer up to end of deflate stream
    had = strm.avail_out;

    loop {
        // Get more input for inflate()
        if strm.avail_in == 0 && unsafe { gz_avail(state) } == -1 {
            return -1;
        }
        if strm.avail_in == 0 {
            unsafe {
                gz_error(state, -5, b"unexpected end of file\x00".as_ptr() as *const std::os::raw::c_char);
            }
            break;
        } else {
            // Decompress and handle errors
            ret = unsafe { inflate(strm, 0) };
            match ret {
                -2 | 2 => {
                    unsafe {
                        gz_error(state, -2, b"internal error: inflate stream corrupt\x00".as_ptr() as *const std::os::raw::c_char);
                    }
                    return -1;
                }
                -4 => {
                    unsafe {
                        gz_error(state, -4, b"out of memory\x00".as_ptr() as *const std::os::raw::c_char);
                    }
                    return -1;
                }
                -3 => {
                    // Deflate stream invalid
                    unsafe {
                        gz_error(state, -3, if strm.msg.is_null() {
                            b"compressed data error\x00".as_ptr() as *const std::os::raw::c_char
                        } else {
                            strm.msg as *const std::os::raw::c_char
                        });
                    }
                    return -1;
                }
                _ => {
                    if !(strm.avail_out != 0 && ret != 1) {
                        break;
                    }
                }
            }
        }
    }

    // Update available output
    unsafe {
        (*state).x.have = had.wrapping_sub(strm.avail_out);
        (*state).x.next = strm.next_out.offset(-( (*state).x.have as isize ));
    }

    // If the gzip stream completed successfully, look for another
    if ret == 1 {
        unsafe { (*state).how = 0; }
    }

    // Good decompression
    return 0;
}

/* Fetch data and put it in the output buffer.  Assumes state->x.have is 0.
   Data is either copied from the input file or decompressed from the input
   file depending on state->how.  If state->how is LOOK, then a gzip header is
   looked for to determine whether to copy or decompress.  Returns -1 on error,
   otherwise 0.  gz_fetch() will leave state->how as COPY or GZIP unless the
   end of the input file has been reached and all data has been processed.  */
unsafe extern "C" fn gz_fetch(mut state: gz_statep) -> std::os::raw::c_int {
    let mut strm: z_streamp = &mut (*state).strm;
    loop  {
        match (*state).how {
            0 => {
                /* -> LOOK, COPY (only if never GZIP), or GZIP */
                if gz_look(state) == -(1 as std::os::raw::c_int) {
                    return -(1 as std::os::raw::c_int)
                }
                if (*state).how == 0 as std::os::raw::c_int {
                    return 0 as std::os::raw::c_int
                }
            }
            1 => {
                /* -> COPY */
                if gz_load(state, (*state).out,
                           (*state).size << 1 as std::os::raw::c_int,
                           &mut (*state).x.have) == -(1 as std::os::raw::c_int) {
                    return -(1 as std::os::raw::c_int)
                }
                (*state).x.next = (*state).out;
                return 0 as std::os::raw::c_int
            }
            2 => {
                /* -> GZIP or LOOK (if end of gzip stream) */
                (*strm).avail_out = (*state).size << 1 as std::os::raw::c_int;
                (*strm).next_out = (*state).out;
                if gz_decomp(state) == -(1 as std::os::raw::c_int) {
                    return -(1 as std::os::raw::c_int)
                }
            }
            _ => { }
        }
        if !((*state).x.have == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                 ((*state).eof == 0 || (*strm).avail_in != 0)) {
            break ;
        }
    }
    return 0 as std::os::raw::c_int;
}
/* Skip len uncompressed bytes of output.  Return -1 on error, 0 on success. */
unsafe extern "C" fn gz_skip(mut state: gz_statep, mut len: off64_t)
 -> std::os::raw::c_int {
    let mut n: std::os::raw::c_uint = 0;
    /* skip over len bytes or reach end-of-file, whichever comes first */
    while len != 0 {
        /* skip over whatever is in output buffer */
        if (*state).x.have != 0 {
            n =
                if ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong ==
                       ::std::mem::size_of::<off64_t>() as std::os::raw::c_ulong &&
                       (*state).x.have >
                           2147483647 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*state).x.have as off64_t > len {
                    len as std::os::raw::c_uint
                } else { (*state).x.have };
            (*state).x.have = (*state).x.have.wrapping_sub(n);
            (*state).x.next = (*state).x.next.offset(n as isize);
            (*state).x.pos += n as std::os::raw::c_long;
            len -= n as std::os::raw::c_long
        } else {
            /* output buffer empty -- return if we're at the end of the input */
            if (*state).eof != 0 &&
                   (*state).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint
               {
                break ;
            }
            /* need more data to skip -- load up output buffer */
            /* get more output, looking for header if required */
            if gz_fetch(state) == -(1 as std::os::raw::c_int) {
                return -(1 as std::os::raw::c_int)
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* Read len bytes into buf from file, or less than len up to the end of the
   input.  Return the number of bytes read.  If zero is returned, either the
   end of file was reached, or there was an error.  state->err must be
   consulted in that case to determine which. */
unsafe extern "C" fn gz_read(mut state: gz_statep, mut buf: voidp,
                             mut len: z_size_t) -> z_size_t {
    let mut got: z_size_t = 0;
    let mut n: std::os::raw::c_uint = 0;
    /* if len is zero, avoid unnecessary operations */
    if len == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as z_size_t
    }
    /* process a skip request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_skip(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            return 0 as std::os::raw::c_int as z_size_t
        }
    }
    /* get len bytes to buf, or less than len if at the end */
    got = 0 as std::os::raw::c_int as z_size_t;
    let mut current_block_34: u64;
    loop  {
        /* set n to the maximum amount of len that fits in an unsigned int */
        n = -(1 as std::os::raw::c_int) as std::os::raw::c_uint;
        if n as std::os::raw::c_ulong > len { n = len as std::os::raw::c_uint }
        /* first just try copying data from the output buffer */
        if (*state).x.have != 0 {
            if (*state).x.have < n { n = (*state).x.have }
            memcpy(buf, (*state).x.next as *const std::os::raw::c_void,
                   n as std::os::raw::c_ulong);
            (*state).x.next = (*state).x.next.offset(n as isize);
            (*state).x.have = (*state).x.have.wrapping_sub(n);
            current_block_34 = 1538046216550696469;
        } else if (*state).eof != 0 &&
                      (*state).strm.avail_in ==
                          0 as std::os::raw::c_int as std::os::raw::c_uint {
            /* output buffer empty -- return if we're at the end of the input */
            (*state).past = 1 as std::os::raw::c_int; /* tried to read past end */
            break ;
        } else if (*state).how == 0 as std::os::raw::c_int ||
                      n < (*state).size << 1 as std::os::raw::c_int {
            /* need output data -- for small len or new stream load up our output
           buffer */
            /* get more output, looking for header if required */
            if gz_fetch(state) == -(1 as std::os::raw::c_int) {
                return 0 as std::os::raw::c_int as z_size_t
            }
            current_block_34 = 17216689946888361452;
            /* no progress yet -- go back to copy above */
            /* the copy above assures that we will leave with space in the
               output buffer, allowing at least one gzungetc() to succeed */
        } else {
            /* large len -- read directly into user buffer */
            if (*state).how == 1 as std::os::raw::c_int {
                /* read directly */
                if gz_load(state, buf as *mut std::os::raw::c_uchar, n, &mut n) ==
                       -(1 as std::os::raw::c_int) {
                    return 0 as std::os::raw::c_int as z_size_t
                }
            } else {
                /* large len -- decompress directly into user buffer */
                /* state->how == GZIP */
                (*state).strm.avail_out = n;
                (*state).strm.next_out = buf as *mut std::os::raw::c_uchar;
                if gz_decomp(state) == -(1 as std::os::raw::c_int) {
                    return 0 as std::os::raw::c_int as z_size_t
                }
                n = (*state).x.have;
                (*state).x.have = 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
            current_block_34 = 1538046216550696469;
        }
        match current_block_34 {
            1538046216550696469 => {
                /* update progress */
                len =
                    (len as std::os::raw::c_ulong).wrapping_sub(n as std::os::raw::c_ulong) as
                        z_size_t as z_size_t;
                buf = (buf as *mut std::os::raw::c_char).offset(n as isize) as voidp;
                got =
                    (got as std::os::raw::c_ulong).wrapping_add(n as std::os::raw::c_ulong) as
                        z_size_t as z_size_t;
                (*state).x.pos += n as std::os::raw::c_long
            }
            _ => { }
        }
        if !(len != 0) { break ; }
    }
    /* return number of bytes read into user buffer */
    return got;
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzread(mut file: gzFile, mut buf: voidp,
                                mut len: std::os::raw::c_uint) -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return -(1 as std::os::raw::c_int) }
    state = file as gz_statep;
    /* check that we're reading and that there's no (serious) error */
    if (*state).mode != 7247 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int &&
               (*state).err != -(5 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    /* since an int is returned, make sure len fits in one, otherwise return
       with an error (this avoids a flaw in the interface) */
    if (len as std::os::raw::c_int) < 0 as std::os::raw::c_int {
        gz_error(state, -(2 as std::os::raw::c_int),
                 b"request does not fit in an int\x00" as *const u8 as
                     *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /* read len or fewer bytes to buf */
    len = gz_read(state, buf, len as z_size_t) as std::os::raw::c_uint;
    /* check for an error */
    if len == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*state).err != 0 as std::os::raw::c_int &&
           (*state).err != -(5 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    /* return the number of bytes read (this is assured to fit in an int) */
    return len as std::os::raw::c_int;
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzfread(mut buf: voidp, mut size: z_size_t,
                                 mut nitems: z_size_t, mut file: gzFile)
 -> z_size_t {
    let mut len: z_size_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return 0 as std::os::raw::c_int as z_size_t }
    state = file as gz_statep;
    /* check that we're reading and that there's no (serious) error */
    if (*state).mode != 7247 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int &&
               (*state).err != -(5 as std::os::raw::c_int) {
        return 0 as std::os::raw::c_int as z_size_t
    }
    /* compute bytes to read -- error on overflow */
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        gz_error(state, -(2 as std::os::raw::c_int),
                 b"request does not fit in a size_t\x00" as *const u8 as
                     *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int as z_size_t
    }
    /* read len or fewer bytes to buf, return the number of full items read */
    return if len != 0 {
               gz_read(state, buf, len).wrapping_div(size)
           } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong };
}
/* -- see zlib.h -- */

pub fn gzgetc(file: gzFile) -> std::os::raw::c_int {
    if file.is_null() {
        return -1;
    }
    
    let state: &mut gz_state = unsafe { &mut *(file as *mut gz_state) };

    if state.mode != 7247 || (state.err != 0 && state.err != -5) {
        return -1;
    }

    if state.x.have != 0 {
        state.x.have -= 1;
        state.x.pos += 1;
        let result = unsafe { *state.x.next };
        state.x.next = unsafe { state.x.next.add(1) };
        return result as std::os::raw::c_int;
    }

    let mut buf: [std::os::raw::c_uchar; 1] = [0; 1];
    let ret = unsafe { gz_read(state, buf.as_mut_ptr() as *mut _, 1) } as std::os::raw::c_int;

    if ret < 1 {
        return -1;
    } else {
        return buf[0] as std::os::raw::c_int;
    }
}


pub fn gzgetc_(file: gzFile) -> std::os::raw::c_int {
    gzgetc(file)
}

/* -- see zlib.h -- */

pub fn gzungetc(c: i32, file: gzFile) -> i32 {
    let state: &mut gz_state = unsafe {
        if file.is_null() {
            return -1;
        }
        &mut *(file as *mut gz_state)
    };

    if state.mode != 7247 || (state.err != 0 && state.err != -5) {
        return -1;
    }

    if state.seek != 0 {
        state.seek = 0;
        unsafe {
            if gz_skip(state, state.skip) == -1 {
                return -1;
            }
        }
    }

    if c < 0 {
        return -1;
    }

    if state.x.have == 0 {
        state.x.have = 1;
        unsafe {
            state.x.next = state.out.offset((state.size * 2) as isize - 1);
            *state.x.next = c as u8;
        }
        state.x.pos -= 1;
        state.past = 0;
        return c;
    }

    if state.x.have == (state.size * 2) {
        unsafe {
            gz_error(state, -3, b"out of room to push characters\x00" as *const u8 as *const i8);
        }
        return -1;
    }

    let src: *mut u8;
    let dest: *mut u8;

    unsafe {
        src = state.out.offset(state.x.have as isize);
        dest = state.out.offset((state.size * 2) as isize);
    }

    if state.x.next == state.out {
        for i in (0..state.x.have).rev() {
            unsafe {
                *dest.offset(i as isize) = *src.offset(i as isize);
            }
        }
        state.x.next = dest;
    }

    state.x.have += 1;
    unsafe {
        state.x.next = state.x.next.offset(-1);
        *state.x.next = c as u8;
    }
    state.x.pos -= 1;
    state.past = 0;
    return c;
}

/* -- see zlib.h -- */

pub unsafe extern "C" fn gzgets(mut file: gzFile, mut buf: *mut std::os::raw::c_char,
                                mut len: std::os::raw::c_int) -> *mut std::os::raw::c_char {
    let mut left: std::os::raw::c_uint = 0;
    let mut n: std::os::raw::c_uint = 0;
    let mut str: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut eol: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* check parameters and get internal structure */
    if file.is_null() || buf.is_null() || len < 1 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_char
    }
    state = file as gz_statep;
    /* check that we're reading and that there's no (serious) error */
    if (*state).mode != 7247 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int &&
               (*state).err != -(5 as std::os::raw::c_int) {
        return 0 as *mut std::os::raw::c_char
    }
    /* process a skip request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_skip(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            return 0 as *mut std::os::raw::c_char
        }
    }
    /* copy output bytes up to new line or len - 1, whichever comes first --
       append a terminating zero to the string (we don't check for a zero in
       the contents, let the user worry about that) */
    str = buf;
    left =
        (len as std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
    if left != 0 {
        loop  {
            /* assure that something is in the output buffer */
            if (*state).x.have == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   gz_fetch(state) == -(1 as std::os::raw::c_int) {
                return 0 as *mut std::os::raw::c_char
            } /* error */
            if (*state).x.have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                /* end of file */
                (*state).past = 1 as std::os::raw::c_int;
                break ; /* read past end */
                /* return what we have */
            } else {
                /* look for end-of-line in current output buffer */
                n =
                    if (*state).x.have > left {
                        left
                    } else { (*state).x.have };
                eol =
                    memchr((*state).x.next as *const std::os::raw::c_void,
                           '\n' as i32, n as std::os::raw::c_ulong) as
                        *mut std::os::raw::c_uchar;
                if !eol.is_null() {
                    n =
                        (eol.offset_from((*state).x.next) as
                             std::os::raw::c_long as
                             std::os::raw::c_uint).wrapping_add(1 as std::os::raw::c_int as
                                                            std::os::raw::c_uint)
                }
                /* copy through end-of-line, or remainder if not found */
                memcpy(buf as *mut std::os::raw::c_void,
                       (*state).x.next as *const std::os::raw::c_void,
                       n as std::os::raw::c_ulong);
                (*state).x.have = (*state).x.have.wrapping_sub(n);
                (*state).x.next = (*state).x.next.offset(n as isize);
                (*state).x.pos += n as std::os::raw::c_long;
                left = left.wrapping_sub(n);
                buf = buf.offset(n as isize);
                if !(left != 0 && eol.is_null()) { break ; }
            }
        }
    }
    /* return terminated string, or if nothing, end of file */
    if buf == str { return 0 as *mut std::os::raw::c_char }
    *buf.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return str;
}
/* -- see zlib.h -- */

pub unsafe extern "C" fn gzdirect(mut file: gzFile) -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return 0 as std::os::raw::c_int }
    state = file as gz_statep;
    /* if the state is not known, but we can find out, then do so (this is
       mainly for right after a gzopen() or gzdopen()) */
    if (*state).mode == 7247 as std::os::raw::c_int &&
           (*state).how == 0 as std::os::raw::c_int &&
           (*state).x.have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        gz_look(state);
    }
    /* return 1 if transparent, 0 if processing a gzip stream */
    return (*state).direct;
}
/* -- see zlib.h -- */

pub fn gzclose_r(file: Option<gzFile>) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut err: std::os::raw::c_int = 0;

    // Check if the file is None
    let state = match file {
        Some(f) => {
            let state = f as *mut gz_state;
            // Check that we're reading
            if unsafe { (*state).mode } != 7247 {
                return -(2 as std::os::raw::c_int);
            }
            state
        },
        None => return -(2 as std::os::raw::c_int),
    };

    // Free memory and close file
    if unsafe { (*state).size } != 0 {
        unsafe {
            inflateEnd(&mut (*state).strm);
            let _ = Box::from_raw((*state).out);
            let _ = Box::from_raw((*state).in_0);
        }
    }

    err = if unsafe { (*state).err } == -(5 as std::os::raw::c_int) {
        -(5 as std::os::raw::c_int)
    } else {
        0
    };

    unsafe {
        gz_error(state, 0, std::ptr::null());
        let _ = Box::from_raw((*state).path);
        ret = close((*state).fd);
        let _ = Box::from_raw(state);
    }

    if ret != 0 {
        -(1 as std::os::raw::c_int)
    } else {
        err
    }
}


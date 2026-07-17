
extern "C" {
    
    
}
pub use crate::src::zlib::gzread::gzclose_r;
pub type __off64_t = std::os::raw::c_long;
pub type off64_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: std::os::raw::c_uint,
    pub next: * mut std::os::raw::c_uchar,
    pub pos: std::os::raw::c_long,
}
impl gzFile_s {
    pub const fn new() -> Self {
        gzFile_s {
        have: 0,
        next: (0 as * mut std::os::raw::c_uchar),
        pos: 0
        }
    }
}

impl std::default::Default for gzFile_s {
    fn default() -> Self { gzFile_s::new() }
}

pub type gzFile = * mut crate::src::zlib::gzclose::gzFile_s;
/* gzclose.c -- zlib gzclose() function
 * Copyright (C) 2004, 2010 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* gzclose() is in a separate file so that it is linked in only if it is used.
   That way the other gzclose functions can be used instead to avoid linking in
   unneeded compression or decompression routines. */
#[no_mangle]
pub unsafe extern "C" fn gzclose(mut file: * mut crate::src::zlib::gzclose::gzFile_s) -> std::os::raw::c_int {
    return gzclose_r(file);
}
use crate::laertes_rt::*;

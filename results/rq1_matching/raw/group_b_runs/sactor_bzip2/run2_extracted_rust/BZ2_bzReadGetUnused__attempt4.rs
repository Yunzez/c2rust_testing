// SACTOR unidiomatic translation of `BZ2_bzReadGetUnused` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:39:34; attempt 4). Verification verdict: Rust code failed to compile
use libc::{c_int, c_void};
pub type BZFILE = c_void;
pub unsafe extern "C" fn BZ2_bzReadGetUnused(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    unused: *mut *mut c_void,
    nUnused: *mut c_int,
) {
    #[repr(C)]
    struct LocalBzFile {
        handle: *mut c_void,
        buf: [i8; 5000],
        bufN: c_int,
        writing: u8,
        strm: bz_stream,
        lastErr: c_int,
        initialisedOk: u8,
    }
    #[repr(C)]
    pub struct bz_stream {
        pub next_in: *mut ::core::ffi::c_char,
        pub avail_in: ::core::ffi::c_uint,
        pub total_in_lo32: ::core::ffi::c_uint,
        pub total_in_hi32: ::core::ffi::c_uint,
        pub next_out: *mut ::core::ffi::c_char,
        pub avail_out: ::core::ffi::c_uint,
        pub total_out_lo32: ::core::ffi::c_uint,
        pub total_out_hi32: ::core::ffi::c_uint,
        pub state: *mut ::core::ffi::c_void,
        pub bzalloc: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub bzfree: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        pub opaque: *mut ::core::ffi::c_void,
    }
    unsafe fn bz_seterr(bzerror: *mut c_int, bzf: *mut LocalBzFile, eee: c_int) {
        if !bzerror.is_null() {
            *bzerror = eee;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = eee;
        }
    }
    const BZ_STREAM_END: c_int = 4;
    const BZ_SEQUENCE_ERROR: c_int = -1;
    const BZ_OK: c_int = 0;
    const BZ_PARAM_ERROR: c_int = -2;
    let bzf = b as *mut LocalBzFile;
    if bzf.is_null() {
        bz_seterr(bzerror, bzf, BZ_PARAM_ERROR);
        return;
    }
    if (*bzf).lastErr != BZ_STREAM_END {
        bz_seterr(bzerror, bzf, BZ_SEQUENCE_ERROR);
        return;
    }
    if unused.is_null() || nUnused.is_null() {
        bz_seterr(bzerror, bzf, BZ_PARAM_ERROR);
        return;
    }
    bz_seterr(bzerror, bzf, BZ_OK);
    *nUnused = (*bzf).strm.avail_in as c_int;
    *unused = (*bzf).strm.next_in as *mut c_void;
}

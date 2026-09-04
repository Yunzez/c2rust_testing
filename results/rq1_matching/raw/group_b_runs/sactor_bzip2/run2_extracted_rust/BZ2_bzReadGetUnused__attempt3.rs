// SACTOR unidiomatic translation of `BZ2_bzReadGetUnused` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:39:27; attempt 3). Verification verdict: Rust code failed to compile
use libc::{c_int, c_void};
pub type BZFILE = c_void;
pub unsafe extern "C" fn BZ2_bzReadGetUnused(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    unused: *mut *mut c_void,
    nUnused: *mut c_int,
) {
    unsafe fn bz_seterr(bzerror: *mut c_int, bzf: *mut bzFile, eee: c_int) {
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
    let bzf = b as *mut bzFile;
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

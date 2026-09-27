// SACTOR unidiomatic translation of `BZ2_bzCompressInit` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:41:36; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn BZ2_bzCompressInit(
    strm: *mut bz_stream,
    mut blockSize100k: libc::c_int,
    verbosity: libc::c_int,
    mut workFactor: libc::c_int,
) -> libc::c_int {
    type Int32 = libc::c_int;
    type UInt32 = libc::c_uint;
    type UChar = libc::c_uchar;
    const BZ_CONFIG_ERROR: libc::c_int = -9;
    const BZ_MEM_ERROR: libc::c_int = -3;
    const BZ_PARAM_ERROR: libc::c_int = -2;
    const BZ_OK: libc::c_int = 0;
    const BZ_S_INPUT: libc::c_int = 2;
    const BZ_M_RUNNING: libc::c_int = 2;
    const BZ_N_RADIX: libc::c_int = 2;
    const BZ_N_QSORT: libc::c_int = 12;
    const BZ_N_SHELL: libc::c_int = 18;
    const BZ_N_OVERSHOOT: libc::c_int = BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2;
    unsafe fn call_bzalloc(
        strm: *mut bz_stream,
        items: libc::c_int,
        size: libc::c_int,
    ) -> *mut libc::c_void {
        if strm.is_null() {
            return core::ptr::null_mut();
        }
        let s = &mut *strm;
        match s.bzalloc {
            Some(f) => f(s.opaque, items, size),
            None => core::ptr::null_mut(),
        }
    }
    unsafe fn call_bzfree(strm: *mut bz_stream, p: *mut libc::c_void) {
        if strm.is_null() || p.is_null() {
            return;
        }
        let s = &mut *strm;
        if let Some(f) = s.bzfree {
            f(s.opaque, p);
        }
    }
    let mut n: Int32;
    let mut s_ptr: *mut EState;
    if bz_config_ok() == 0 {
        return BZ_CONFIG_ERROR;
    }
    if strm.is_null() || blockSize100k < 1 || blockSize100k > 9 || workFactor < 0
        || workFactor > 250
    {
        return BZ_PARAM_ERROR;
    }
    if workFactor == 0 {
        workFactor = 30;
    }
    {
        let strm_ref = &mut *strm;
        if strm_ref.bzalloc.is_none() {
            strm_ref.bzalloc = Some(default_bzalloc);
        }
        if strm_ref.bzfree.is_none() {
            strm_ref.bzfree = Some(default_bzfree);
        }
    }
    s_ptr = call_bzalloc(strm, core::mem::size_of::<EState>() as libc::c_int, 1)
        as *mut EState;
    if s_ptr.is_null() {
        return BZ_MEM_ERROR;
    }
    (*s_ptr).strm = strm;
    (*s_ptr).arr1 = core::ptr::null_mut();
    (*s_ptr).arr2 = core::ptr::null_mut();
    (*s_ptr).ftab = core::ptr::null_mut();
    n = 100000 * blockSize100k;
    (*s_ptr).arr1 = call_bzalloc(
        strm,
        (n as usize * core::mem::size_of::<UInt32>()) as libc::c_int,
        1,
    ) as *mut UInt32;
    (*s_ptr).arr2 = call_bzalloc(
        strm,
        ((n + BZ_N_OVERSHOOT) as usize * core::mem::size_of::<UInt32>()) as libc::c_int,
        1,
    ) as *mut UInt32;
    (*s_ptr).ftab = call_bzalloc(
        strm,
        (65537usize * core::mem::size_of::<UInt32>()) as libc::c_int,
        1,
    ) as *mut UInt32;
    if (*s_ptr).arr1.is_null() || (*s_ptr).arr2.is_null() || (*s_ptr).ftab.is_null() {
        if !(*s_ptr).arr1.is_null() {
            call_bzfree(strm, (*s_ptr).arr1 as *mut libc::c_void);
        }
        if !(*s_ptr).arr2.is_null() {
            call_bzfree(strm, (*s_ptr).arr2 as *mut libc::c_void);
        }
        if !(*s_ptr).ftab.is_null() {
            call_bzfree(strm, (*s_ptr).ftab as *mut libc::c_void);
        }
        if !s_ptr.is_null() {
            call_bzfree(strm, s_ptr as *mut libc::c_void);
        }
        return BZ_MEM_ERROR;
    }
    (*s_ptr).blockNo = 0;
    (*s_ptr).state = BZ_S_INPUT;
    (*s_ptr).mode = BZ_M_RUNNING;
    (*s_ptr).combinedCRC = 0;
    (*s_ptr).blockSize100k = blockSize100k;
    (*s_ptr).nblockMAX = 100000 * blockSize100k - 19;
    (*s_ptr).verbosity = verbosity;
    (*s_ptr).workFactor = workFactor;
    (*s_ptr).block = (*s_ptr).arr2 as *mut UChar;
    (*s_ptr).mtfv = (*s_ptr).arr1 as *mut libc::c_ushort;
    (*s_ptr).zbits = core::ptr::null_mut();
    (*s_ptr).ptr = (*s_ptr).arr1 as *mut UInt32;
    (*strm).state = s_ptr as *mut libc::c_void;
    (*strm).total_in_lo32 = 0;
    (*strm).total_in_hi32 = 0;
    (*strm).total_out_lo32 = 0;
    (*strm).total_out_hi32 = 0;
    init_RL(s_ptr);
    prepare_new_block(s_ptr);
    BZ_OK
}

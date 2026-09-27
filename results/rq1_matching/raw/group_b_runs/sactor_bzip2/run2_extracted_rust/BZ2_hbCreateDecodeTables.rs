// SACTOR unidiomatic translation of `BZ2_hbCreateDecodeTables` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:46:04; attempt 1). Verification verdict: Failed to fix the code:     Updating crates.io index
pub unsafe fn BZ2_hbCreateDecodeTables(
    limit: *mut libc::c_int,
    base: *mut libc::c_int,
    perm: *mut libc::c_int,
    length: *mut libc::c_uchar,
    minLen: libc::c_int,
    maxLen: libc::c_int,
    alphaSize: libc::c_int,
) {
    const BZ_MAX_CODE_LEN: libc::c_int = 23;
    let mut pp: libc::c_int = 0;
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut vec: libc::c_int;
    i = minLen;
    while i <= maxLen {
        j = 0;
        while j < alphaSize {
            if *length.add(j as usize) == i as libc::c_uchar {
                *perm.add(pp as usize) = j;
                pp += 1;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < BZ_MAX_CODE_LEN {
        *base.add(i as usize) = 0;
        i += 1;
    }
    i = 0;
    while i < alphaSize {
        let len_i = *length.add(i as usize) as libc::c_int;
        *base.add((len_i + 1) as usize) += 1;
        i += 1;
    }
    i = 1;
    while i < BZ_MAX_CODE_LEN {
        let prev = *base.add((i - 1) as usize);
        let cur_ptr = base.add(i as usize);
        *cur_ptr += prev;
        i += 1;
    }
    i = 0;
    while i < BZ_MAX_CODE_LEN {
        *limit.add(i as usize) = 0;
        i += 1;
    }
    vec = 0;
    i = minLen;
    while i <= maxLen {
        let bi1 = *base.add((i + 1) as usize);
        let bi = *base.add(i as usize);
        vec += bi1 - bi;
        *limit.add(i as usize) = vec - 1;
        vec <<= 1;
        i += 1;
    }
    i = minLen + 1;
    while i <= maxLen {
        let lim_prev = *limit.add((i - 1) as usize);
        let bi_ptr = base.add(i as usize);
        *bi_ptr = (((lim_prev + 1) << 1) - *bi_ptr);
        i += 1;
    }
}

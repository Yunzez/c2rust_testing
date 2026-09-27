// SACTOR unidiomatic translation of `BZ2_hbAssignCodes` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:45:43; attempt 1). Verification verdict: Failed to fix the code:     Updating crates.io index
pub unsafe fn BZ2_hbAssignCodes(
    code: *mut libc::c_int,
    length: *mut libc::c_uchar,
    minLen: libc::c_int,
    maxLen: libc::c_int,
    alphaSize: libc::c_int,
) {
    let mut n: libc::c_int;
    let mut vec: libc::c_int;
    let mut i: libc::c_int;
    vec = 0;
    n = minLen;
    while n <= maxLen {
        i = 0;
        while i < alphaSize {
            if *length.add(i as usize) == n as libc::c_uchar {
                *code.add(i as usize) = vec;
                vec += 1;
            }
            i += 1;
        }
        vec <<= 1;
        n += 1;
    }
}

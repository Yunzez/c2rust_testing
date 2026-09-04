// SACTOR unidiomatic translation of `fallbackSimpleSort` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:33:53; attempt 1). Verification verdict: rust compiled
#[inline]
pub unsafe fn fallbackSimpleSort(
    fmap: *mut libc::c_uint,
    eclass: *const libc::c_uint,
    lo: libc::c_int,
    hi: libc::c_int,
) {
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut tmp: libc::c_int;
    let mut ec_tmp: libc::c_uint;
    if lo == hi {
        return;
    }
    if hi - lo > 3 {
        i = hi - 4;
        while i >= lo {
            tmp = *fmap.add(i as usize) as libc::c_int;
            ec_tmp = *eclass.add(tmp as usize);
            j = i + 4;
            while j <= hi
                && {
                    let idx = *fmap.add(j as usize) as usize;
                    ec_tmp > *eclass.add(idx)
                }
            {
                let val = *fmap.add(j as usize);
                *fmap.add((j - 4) as usize) = val;
                j += 4;
            }
            *fmap.add((j - 4) as usize) = tmp as libc::c_uint;
            i -= 1;
        }
    }
    i = hi - 1;
    while i >= lo {
        tmp = *fmap.add(i as usize) as libc::c_int;
        ec_tmp = *eclass.add(tmp as usize);
        j = i + 1;
        while j <= hi
            && {
                let idx = *fmap.add(j as usize) as usize;
                ec_tmp > *eclass.add(idx)
            }
        {
            let val = *fmap.add(j as usize);
            *fmap.add((j - 1) as usize) = val;
            j += 1;
        }
        *fmap.add((j - 1) as usize) = tmp as libc::c_uint;
        i -= 1;
    }
}

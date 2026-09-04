// SACTOR unidiomatic translation of `mmed3` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:34:33; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_bzip2/blocksort.c: Dependency 'BZ2_bz__AssertH__fail' of type 'fu
#[inline]
fn mmed3(mut a: libc::c_uchar, mut b: libc::c_uchar, c: libc::c_uchar) -> libc::c_uchar {
    let mut t: libc::c_uchar;
    if a > b {
        t = a;
        a = b;
        b = t;
    }
    if b > c {
        b = c;
        if a > b {
            b = a;
        }
    }
    b
}

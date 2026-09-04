// SACTOR unidiomatic translation of `Adam7_getpassvalues` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:55:03; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn Adam7_getpassvalues(
    passw: *mut u32,
    passh: *mut u32,
    filter_passstart: *mut libc::size_t,
    padded_passstart: *mut libc::size_t,
    passstart: *mut libc::size_t,
    w: u32,
    h: u32,
    bpp: u32,
) {
    let mut i: u32 = 0;
    while i != 7 {
        *passw.add(i as usize) = (w + ADAM7_DX[i as usize] - ADAM7_IX[i as usize] - 1)
            / ADAM7_DX[i as usize];
        *passh.add(i as usize) = (h + ADAM7_DY[i as usize] - ADAM7_IY[i as usize] - 1)
            / ADAM7_DY[i as usize];
        if *passw.add(i as usize) == 0 {
            *passh.add(i as usize) = 0;
        }
        if *passh.add(i as usize) == 0 {
            *passw.add(i as usize) = 0;
        }
        i += 1;
    }
    *filter_passstart = 0;
    *padded_passstart = 0;
    *passstart = 0;
    i = 0;
    while i != 7 {
        let idx = i as usize;
        let next = (i + 1) as usize;
        let pw = *passw.add(idx);
        let ph = *passh.add(idx);
        *filter_passstart.add(next) = *filter_passstart.add(idx)
            + if pw != 0 && ph != 0 {
                (ph as libc::size_t)
                    * (1u64 + ((pw as u64 * bpp as u64 + 7u64) / 8u64)) as libc::size_t
            } else {
                0
            };
        *padded_passstart.add(next) = *padded_passstart.add(idx)
            + (ph as libc::size_t)
                * (((pw as u64 * bpp as u64 + 7u64) / 8u64) as libc::size_t);
        *passstart.add(next) = *passstart.add(idx)
            + (((ph as u64 * pw as u64 * bpp as u64 + 7u64) / 8u64) as libc::size_t);
        i += 1;
    }
}

// SACTOR unidiomatic translation of `getHash` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:41:14; attempt 1). Verification verdict: Rust code failed to compile
#[inline]
unsafe fn getHash(
    data: *const libc::c_uchar,
    size: libc::size_t,
    pos: libc::size_t,
) -> libc::c_uint {
    let mut result: libc::c_uint = 0;
    if pos + 2 < size {
        result ^= (*(data.add(pos as usize)) as libc::c_uint) << 0u;
        result ^= (*(data.add(pos as usize + 1)) as libc::c_uint) << 4u;
        result ^= (*(data.add(pos as usize + 2)) as libc::c_uint) << 8u;
    } else {
        let mut amount: libc::size_t;
        let mut i: libc::size_t;
        if pos >= size {
            return 0;
        }
        amount = size - pos;
        i = 0;
        while i != amount {
            result
                ^= (*(data.add((pos + i) as usize)) as libc::c_uint)
                    << ((i as libc::c_uint) * 8u);
            i = i.wrapping_add(1);
        }
    }
    result & HASH_BIT_MASK
}

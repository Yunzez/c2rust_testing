#[inline]
pub unsafe fn getHash(
    data: *const libc::c_uchar,
    size: libc::size_t,
    pos: libc::size_t,
) -> libc::c_uint {
    let mut result: libc::c_uint = 0;
    if pos.wrapping_add(2) < size {
        result ^= (*(data.add(pos as usize)) as libc::c_uint) << 0u32;
        result ^= (*(data.add(pos as usize + 1)) as libc::c_uint) << 4u32;
        result ^= (*(data.add(pos as usize + 2)) as libc::c_uint) << 8u32;
    } else {
        let mut amount: libc::size_t;
        let mut i: libc::size_t;
        if pos >= size {
            return 0;
        }
        amount = size.wrapping_sub(pos);
        i = 0;
        while i != amount {
            result ^= (*(data.add((pos.wrapping_add(i)) as usize)) as libc::c_uint)
                << ((i as libc::c_uint) * 8u32);
            i = i.wrapping_add(1);
        }
    }
    result & HASH_BIT_MASK
}

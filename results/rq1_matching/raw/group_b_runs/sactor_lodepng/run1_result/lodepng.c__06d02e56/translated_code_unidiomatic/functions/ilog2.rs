pub unsafe fn ilog2(mut i: libc::size_t) -> libc::size_t {
    let mut result: libc::size_t = 0;
    if i >= 65536 {
        result += 16;
        i >>= 16;
    }
    if i >= 256 {
        result += 8;
        i >>= 8;
    }
    if i >= 16 {
        result += 4;
        i >>= 4;
    }
    if i >= 4 {
        result += 2;
        i >>= 2;
    }
    if i >= 2 {
        result += 1;
    }
    result
}

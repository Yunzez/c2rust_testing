pub fn lodepng_mulofl(a: libc::size_t, b: libc::size_t, result: &mut libc::size_t) -> libc::c_int {
    *result = a.wrapping_mul(b);
    ((a != 0) && (*result / a != b)) as libc::c_int
}

#[inline]
pub fn mmed3(mut a: libc::c_uchar, mut b: libc::c_uchar, c: libc::c_uchar) -> libc::c_uchar {
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

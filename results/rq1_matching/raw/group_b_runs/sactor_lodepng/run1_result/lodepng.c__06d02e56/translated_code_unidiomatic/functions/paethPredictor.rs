pub fn paethPredictor(a: i16, b: i16, c: i16) -> u8 {
    fn lodepng_abs(x: i16) -> i16 {
        if x < 0 {
            -x
        } else {
            x
        }
    }
    let mut a_mut = a;
    let mut pa = lodepng_abs(b.wrapping_sub(c));
    let pb = lodepng_abs(a.wrapping_sub(c));
    let pc = lodepng_abs(a.wrapping_add(b).wrapping_sub(c).wrapping_sub(c));
    if pb < pa {
        a_mut = b;
        pa = pb;
    }
    let result = if pc < pa { c } else { a_mut };
    result as u8
}

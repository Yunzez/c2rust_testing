 extern "C" fn mmed3(mut a: UChar, mut b: UChar, mut c: UChar)
 -> UChar {
    let mut t: UChar = 0;
    if a as std::os::raw::c_int > b as std::os::raw::c_int { t = a; a = b; b = t }
    if b as std::os::raw::c_int > c as std::os::raw::c_int {
        b = c;
        if a as std::os::raw::c_int > b as std::os::raw::c_int { b = a }
    }
    return b;
}
unsafe extern "C" fn mainQSort3(mut ptr: *mut UInt32, mut block: *mut UChar,

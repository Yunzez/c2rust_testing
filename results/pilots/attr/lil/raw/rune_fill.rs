// pilot fix: populate the macOS _DefaultRuneLocale.__runetype table so the crate's own
// isspace/ispunct/isdigit (c2rust-emitted __istype) classify ASCII correctly.
pub unsafe fn fill_runetype(t: &mut [std::os::raw::c_uint; 256]) {
    for c in 0u8..=255 {
        let mut f = 0u32;
        if c.is_ascii_alphabetic() { f |= 0x100; }
        if c.is_ascii_control() { f |= 0x200; }
        if c.is_ascii_digit() { f |= 0x400; }
        if c.is_ascii_graphic() { f |= 0x800; }
        if c.is_ascii_lowercase() { f |= 0x1000; }
        if c.is_ascii_punctuation() { f |= 0x2000; }
        if c.is_ascii_whitespace() || c == 0x0b { f |= 0x4000; }
        if c.is_ascii_uppercase() { f |= 0x8000; }
        if c.is_ascii_hexdigit() { f |= 0x10000; }
        if c == b' ' || c == b'\t' { f |= 0x20000; }
        if c.is_ascii_graphic() || c == b' ' { f |= 0x40000; }
        t[c as usize] = f;
    }
}

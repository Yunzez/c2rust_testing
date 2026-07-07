fn u8strlen(s: *const i8) -> i32 {
    let mut len = 0;
    let mut current = s;

    unsafe {
        while *current != 0 {
            if (*current & 0xC0) != 0x80 {
                len += 1;
            }
            current = current.add(1);
        }
    }

    len
}

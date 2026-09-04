#![allow(unaligned_references)]
#[no_mangle]
#[no_mangle]
fn u8strlen(s: &str) -> usize {
    let mut len = 0;
    for &byte in s.as_bytes() {
        if (byte & 0xC0) != 0x80 {
            len += 1;
        }
    }
    len
}

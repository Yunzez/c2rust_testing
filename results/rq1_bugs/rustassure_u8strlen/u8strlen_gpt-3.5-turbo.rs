fn u8strlen(s: &str) -> usize {
    s.chars().filter(|c| (*c as u8 & 0xC0) != 0x80).count()
}

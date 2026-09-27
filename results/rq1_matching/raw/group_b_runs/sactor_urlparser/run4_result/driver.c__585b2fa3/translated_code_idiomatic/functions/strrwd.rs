pub fn strrwd(ptr: &str, n: usize) -> String {
    fn byte_suffix_from_end(s: &str, n: usize) -> &str {
        let len = s.len();
        if n >= len {
            return "";
        }
        let mut idx = len - n;
        while !s.is_char_boundary(idx) && idx < len {
            idx += 1;
        }
        &s[idx..]
    }
    let slice = byte_suffix_from_end(ptr, n);
    strdup(slice)
}

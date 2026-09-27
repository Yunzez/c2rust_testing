pub fn strff(input: &str, n: usize) -> String {
    fn strdup(inner: &str) -> String {
        inner.to_owned()
    }
    let bytes = input.as_bytes();
    let start = n.min(bytes.len());
    let suffix = std::str::from_utf8(&bytes[start..]).unwrap_or("");
    strdup(suffix)
}

pub fn strdup(input: &str) -> String {
    fn inner(src: &str) -> String {
        src.to_owned()
    }
    inner(input)
}

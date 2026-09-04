pub fn url_is_ssh(input: &str) -> bool {
    fn inner_strdup(s: &str) -> String {
        strdup(s)
    }
    let _copy = inner_strdup(input);
    matches!(input, "ssh" | "git")
}

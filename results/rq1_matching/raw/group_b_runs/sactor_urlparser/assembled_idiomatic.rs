// SACTOR × urlparser run 4 — idiomatic per-function outputs assembled verbatim (concatenation only,
// order: structs, then functions alphabetically). Source: run4_result/driver.c__585b2fa3/translated_code_idiomatic/.
// PARTIAL: 7 of 22 library functions; SACTOR stopped at URL_SCHEMES/url_is_protocol (see RUN.md).

// ---- url_data.rs
/// Idiomatic Rust representation of `url_data`.
///
/// All fields are owned Rust `String`s instead of raw `*mut c_char`.
/// Depending on the actual C API, you might want to make these
/// `Option<String>` instead if NULL is meaningful.
#[derive(Clone, Debug, Default)]
pub struct UrlData {
    pub href: String,
    pub protocol: String,
    pub host: String,
    pub auth: String,
    pub hostname: String,
    pub pathname: String,
    pub search: String,
    pub path: String,
    pub hash: String,
    pub query: String,
    pub port: String,
}

// ---- show.rs
pub fn show(label: &str, v: Option<String>) {
    fn print_line(label: &str, value: Option<&str>) {
        match value {
            Some(text) => println!("{label}: {text}"),
            None => println!("{label}: (null)"),
        }
    }
    print_line(label, v.as_deref());
}

// ---- strdup.rs
pub fn strdup(input: &str) -> String {
    fn inner(src: &str) -> String {
        src.to_owned()
    }
    inner(input)
}

// ---- strff.rs
pub fn strff(input: &str, n: usize) -> String {
    fn strdup(inner: &str) -> String {
        inner.to_owned()
    }
    let bytes = input.as_bytes();
    let start = n.min(bytes.len());
    let suffix = std::str::from_utf8(&bytes[start..]).unwrap_or("");
    strdup(suffix)
}

// ---- strrwd.rs
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

// ---- url_data_inspect.rs
pub fn url_data_inspect(data: &UrlData) {
    fn print_field(label: &str, value: &str) {
        println!("    .{}: \"{}\"", label, value);
    }
    println!("#url =>");
    print_field("href", &data.href);
    print_field("protocol", &data.protocol);
    print_field("host", &data.host);
    print_field("auth", &data.auth);
    print_field("hostname", &data.hostname);
    print_field("pathname", &data.pathname);
    print_field("search", &data.search);
    print_field("path", &data.path);
    print_field("hash", &data.hash);
    print_field("query", &data.query);
    print_field("port", &data.port);
}

// ---- url_free.rs
pub fn url_free(_data: Option<UrlData>) {}

// ---- url_is_ssh.rs
pub fn url_is_ssh(input: &str) -> bool {
    fn inner_strdup(s: &str) -> String {
        strdup(s)
    }
    let _copy = inner_strdup(input);
    matches!(input, "ssh" | "git")
}


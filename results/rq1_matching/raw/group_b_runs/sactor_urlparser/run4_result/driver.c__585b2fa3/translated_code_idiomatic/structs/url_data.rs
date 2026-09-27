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

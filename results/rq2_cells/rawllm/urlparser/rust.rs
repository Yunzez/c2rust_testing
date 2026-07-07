//! A safe, idiomatic Rust port of the provided C URL parser.
//!
//! This module preserves the overall structure and semantics of the
//! original C code, including its (somewhat loose) parsing rules.

use std::fmt;

/// Version of the original C header.
pub const URL_VERSION: &str = "0.0.2";

/// Max length of a url protocol scheme (from the C header).
pub const URL_PROTOCOL_MAX_LENGTH: usize = 16;
/// Max length of a url host part (from the C header).
pub const URL_HOSTNAME_MAX_LENGTH: usize = 128;
/// Max length of a url TLD part (from the C header).
pub const URL_TLD_MAX_LENGTH: usize = 16;
/// Max length of a url auth part (from the C header).
pub const URL_AUTH_MAX_LENGTH: usize = 32;

/// URI schemes table, mirroring the C `URL_SCHEMES` array.
const URL_SCHEMES: &[&str] = &[
    // official IANA registered schemes
    "aaa", "aaas", "about", "acap", "acct", "adiumxtra", "afp", "afs", "aim", "apt", "attachment", "aw",
    "beshare", "bitcoin", "bolo", "callto", "cap", "chrome", "crome-extension", "com-evenbrite-attendee",
    "cid", "coap", "coaps", "content", "crid", "cvs", "data", "dav", "dict", "lna-playsingle", "dln-playcontainer",
    "dns", "dtn", "dvb", "ed2k", "facetime", "fax", "feed", "file", "finger", "fish", "ftp", "geo", "gg", "git",
    "gizmoproject", "go", "gopher", "gtalk", "h323", "hcp", "http", "https", "iax", "icap", "icon", "im",
    "imap", "info", "ipn", "ipp", "irc", "irc6", "ircs", "iris", "iris.beep", "iris.xpc", "iris.xpcs", "iris.lws",
    "itms", "jabber", "jar", "jms", "keyparc", "lastfm", "ldap", "ldaps", "magnet", "mailserver", "mailto",
    "maps", "market", "message", "mid", "mms", "modem", "ms-help", "mssettings-power", "msnim", "msrp",
    "msrps", "mtqp", "mumble", "mupdate", "mvn", "news", "nfs", "ni", "nih", "nntp", "notes", "oid",
    "paquelocktoken", "pack", "palm", "paparazzi", "pkcs11", "platform", "pop", "pres", "prospero", "proxy",
    "psyc", "query", "reload", "res", "resource", "rmi", "rsync", "rtmp", "rtsp", "secondlife", "service", "session",
    "sftp", "sgn", "shttp", "sieve", "sip", "sips", "skype", "smb", "sms", "snews", "snmp", "soap.beep", "soap.beeps",
    "soldat", "spotify", "ssh", "steam", "svn", "tag", "teamspeak", "tel", "telnet", "tftp", "things", "thismessage",
    "tn3270", "tip", "tv", "udp", "unreal", "urn", "ut2004", "vemmi", "ventrilo", "videotex", "view-source", "wais", "webcal",
    "ws", "wss", "wtai", "wyciwyg", "xcon", "xcon-userid", "xfire", "xmlrpc.beep", "xmlrpc.beeps", "xmpp", "xri", "ymsgr",

    // unofficial schemes
    "javascript", "jdbc", "doi",
];

/// Parsed URL data, equivalent to `url_data_t` in C.
///
/// The C version stores raw pointers into the original URL string and
/// heap-allocated segments. Here we use owned `String`s. Empty strings
/// correspond to the C code's behavior when allocation succeeded but
/// no meaningful text was parsed into a field.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

impl fmt::Display for UrlData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#url =>")?;
        writeln!(f, "    .href: \"{}\"", self.href)?;
        writeln!(f, "    .protocol: \"{}\"", self.protocol)?;
        writeln!(f, "    .host: \"{}\"", self.host)?;
        writeln!(f, "    .auth: \"{}\"", self.auth)?;
        writeln!(f, "    .hostname: \"{}\"", self.hostname)?;
        writeln!(f, "    .pathname: \"{}\"", self.pathname)?;
        writeln!(f, "    .search: \"{}\"", self.search)?;
        writeln!(f, "    .path: \"{}\"", self.path)?;
        writeln!(f, "    .hash: \"{}\"", self.hash)?;
        writeln!(f, "    .query: \"{}\"", self.query)?;
        write!(f, "    .port: \"{}\"", self.port)
    }
}

// -----------------------------------------------------------------------------
// Internal helpers (safe equivalents of C helpers)
// -----------------------------------------------------------------------------

/// Equivalent of `strff(ptr, n)` – skip the first `n` bytes.
fn skip_forward(s: &str, n: usize) -> &str {
    if n >= s.len() {
        ""
    } else {
        &s[n..]
    }
}

/// Extract a part starting from `offset` using a simple format modeled on the
/// C `get_part` behavior.
///
/// Supported `format` patterns (to match the original C logic):
/// - "%[^@]"  : read until '@'
/// - "%[^/]"  : read until '/'
/// - "%[^:]"  : read until ':'
/// - "%[^#]"  : read until '#'
/// - "%s"     : entire remainder
/// - ":%s"     : optional leading ':', then remainder
/// - "/%s"     : optional leading '/', then remainder
fn extract_part(url: &str, format: &str, offset: usize) -> Option<String> {
    let s = skip_forward(url, offset);
    if s.is_empty() {
        return None;
    }

    let (leading, core_fmt) = match format {
        "%[^@]" | "%[^/]" | "%[^:]" | "%[^#]" | "%s" => ("", format),
        ":%s" => (":", "%s"),
        "/%s" => ("/", "%s"),
        _ => ("", "%s"),
    };

    let s = if !leading.is_empty() && s.starts_with(leading) {
        &s[leading.len()..]
    } else {
        s
    };

    if s.is_empty() {
        return None;
    }

    let result = match core_fmt {
        "%[^@]" => s.split('@').next().unwrap_or(""),
        "%[^/]" => s.split('/').next().unwrap_or(""),
        "%[^:]" => s.split(':').next().unwrap_or(""),
        "%[^#]" => s.split('#').next().unwrap_or(""),
        "%s" => s,
        _ => s,
    };

    if result.is_empty() {
        None
    } else {
        Some(result.to_string())
    }
}

// -----------------------------------------------------------------------------
// Public helpers equivalent to original C API (renamed idiomatically)
// -----------------------------------------------------------------------------

/// Returns `true` if the given string is a known protocol scheme.
pub fn is_protocol(s: &str) -> bool {
    URL_SCHEMES.iter().any(|&scheme| scheme == s)
}

/// Returns `true` if the protocol is treated as an SSH-like scheme.
///
/// Matches the C logic which considers `ssh` and `git` as SSH.
pub fn is_ssh_like_protocol(s: &str) -> bool {
    s == "ssh" || s == "git"
}

/// Get the protocol part from a URL, e.g. `"http"` from
/// `"http://example.com"`.
///
/// Mirrors `url_get_protocol` semantics: it only returns a protocol if the
/// parsed scheme is in the `URL_SCHEMES` table.
pub fn get_protocol(url: &str) -> Option<String> {
    // Mimic the simple scanning in C: read up to any of ':', '/', '/'.
    let mut end = url.len();
    for (i, ch) in url.char_indices() {
        if ch == ':' || ch == '/' { // C used "%[^://]" which is odd; this is close.
            end = i;
            break;
        }
    }

    let proto = &url[..end];
    if proto.is_empty() {
        return None;
    }

    if is_protocol(proto) {
        Some(proto.to_string())
    } else {
        None
    }
}

/// Get the `user:pass` auth section before `@`, if any.
pub fn get_auth(url: &str) -> Option<String> {
    let protocol = get_protocol(url)?;
    let offset = protocol.len() + 3; // protocol + "://"
    extract_part(url, "%[^@]", offset)
}

/// Get the hostname (including port) section.
///
/// Mirrors `url_get_hostname` (which may include `:port`).
pub fn get_hostname(url: &str) -> Option<String> {
    let protocol = get_protocol(url)?;
    let tmp_protocol = protocol.clone();
    let mut offset = 3usize; // for "://"

    if let Some(auth) = get_auth(url) {
        offset += auth.len() + 1; // + '@'
    }

    offset += protocol.len();

    let hostname = if is_ssh_like_protocol(&tmp_protocol) {
        extract_part(url, "%[^:]", offset)
    } else {
        extract_part(url, "%[^/]", offset)
    }?;

    Some(hostname)
}

/// Get the host component without port.
pub fn get_host(url: &str) -> Option<String> {
    let hostname = get_hostname(url)?;
    let host = hostname.split(':').next().unwrap_or("");
    Some(host.to_string())
}

/// Get the full path (including leading slash for non-SSH URLs).
///
/// Mirrors `url_get_path` logic.
pub fn get_path(url: &str) -> Option<String> {
    let protocol = get_protocol(url)?;
    let hostname = get_hostname(url)?;
    let auth = get_auth(url);

    let mut offset = 3usize; // for "://"
    offset += protocol.len() + hostname.len();
    if let Some(ref a) = auth {
        offset += a.len() + 1; // '@'
    }

    let is_ssh = is_ssh_like_protocol(&protocol);

    let raw_path = if is_ssh {
        extract_part(url, ":%s", offset)?
    } else {
        extract_part(url, "/%s", offset)?
    };

    let path = if is_ssh {
        raw_path
    } else {
        format!("/{}", raw_path)
    };

    Some(path)
}

/// Get the pathname part (path without query and hash).
///
/// Mirrors `url_get_pathname` using a simplified implementation
/// consistent with the C intent.
pub fn get_pathname(url: &str) -> Option<String> {
    let path = get_path(url)?;
    // Up to '?' or '#'
    let mut end = path.len();
    for (i, ch) in path.char_indices() {
        if ch == '?' || ch == '#' {
            end = i;
            break;
        }
    }
    Some(path[..end].to_string())
}

/// Get the `search` part, i.e. beginning with `?` and up to (but not
/// including) a `#`, if any.
pub fn get_search(url: &str) -> Option<String> {
    let path = get_path(url)?;
    let pathname = get_pathname(url)?;

    if path.len() <= pathname.len() {
        return Some(String::new());
    }

    let rest = &path[pathname.len()..];
    // Up to '#'
    let before_hash = rest.split('#').next().unwrap_or("");
    Some(before_hash.to_string())
}

/// Get the `query` part (without the leading `?`).
pub fn get_query(url: &str) -> Option<String> {
    let search = get_search(url)?;
    if search.starts_with('?') {
        Some(search[1..].to_string())
    } else {
        Some(String::new())
    }
}

/// Get the `hash` (fragment) part including leading `#`.
pub fn get_hash(url: &str) -> Option<String> {
    let path = get_path(url)?;
    let pathname = get_pathname(url)?;
    let search = get_search(url).unwrap_or_default();

    let offset = pathname.len() + search.len();
    if path.len() <= offset {
        return Some(String::new());
    }

    let rest = &path[offset..];
    Some(rest.to_string())
}

/// Get the port part, if any (without the colon).
pub fn get_port(url: &str) -> Option<String> {
    let hostname = get_hostname(url)?; // may contain ":port"
    let host = get_host(url)?;

    if hostname.len() <= host.len() + 1 {
        // No room for ':' + port
        return Some(String::new());
    }

    let after = &hostname[host.len() + 1..]; // skip host and ':'
    Some(after.to_string())
}

/// Parse the entire URL into a `UrlData` struct.
///
/// This largely mimics the original `url_parse` behavior while using
/// safe Rust and more robust substring handling.
pub fn parse_url(url: &str) -> Option<UrlData> {
    let mut data = UrlData::default();
    data.href = url.to_string();

    let protocol = get_protocol(url)?;
    let protocol_len = protocol.len() + 3; // + "://"
    let is_ssh = is_ssh_like_protocol(&protocol);
    data.protocol = protocol.clone();

    // auth
    let auth = if url[protocol_len..].contains('@') {
        get_auth(url).unwrap_or_default()
    } else {
        String::new()
    };
    let auth_len = if auth.is_empty() { 0 } else { auth.len() + 1 }; // include '@'
    data.auth = auth.clone();

    // hostname
    let hostname = if is_ssh {
        extract_part(url, "%[^:]", protocol_len + auth_len)?
    } else {
        extract_part(url, "%[^/]", protocol_len + auth_len)?
    };
    let hostname_len = hostname.len();
    data.hostname = hostname.clone();

    // host (without port)
    let host = hostname.split(':').next().unwrap_or("").to_string();
    let host_len = host.len();
    data.host = host.clone();

    // path (with or without leading slash depending on ssh)
    let tmp_path = if is_ssh {
        extract_part(url, ":%s", protocol_len + auth_len + hostname_len).unwrap_or_default()
    } else {
        extract_part(url, "/%s", protocol_len + auth_len + hostname_len).unwrap_or_default()
    };

    let path = if is_ssh {
        tmp_path
    } else {
        format!("/{}", tmp_path)
    };
    data.path = path.clone();

    // pathname
    let mut pathname_end = path.len();
    for (i, ch) in path.char_indices() {
        if ch == '?' || ch == '#' {
            pathname_end = i;
            break;
        }
    }
    let pathname = path[..pathname_end].to_string();
    let pathname_len = pathname.len();
    data.pathname = pathname.clone();

    // search (from end of pathname up to '#')
    let mut search = String::new();
    if path.len() > pathname_len {
        let remaining = &path[pathname_len..];
        let before_hash = remaining.split('#').next().unwrap_or("");
        search = before_hash.to_string();
    }
    let search_len = search.len();
    data.search = search.clone();

    // query (after leading '?', if any)
    let query = if search.starts_with('?') {
        search[1..].to_string()
    } else {
        String::new()
    };
    data.query = query;

    // hash (fragment, whatever remains after pathname + search)
    let hash = if path.len() > pathname_len + search_len {
        path[pathname_len + search_len..].to_string()
    } else {
        String::new()
    };
    data.hash = hash;

    // port (after ':' in hostname)
    let port = if hostname.len() > host_len + 1 {
        hostname[host_len + 1..].to_string()
    } else {
        String::new()
    };
    data.port = port;

    Some(data)
}

/// Print URL parse results to stdout in a format similar to the original C
/// `url_inspect`.
pub fn inspect_url(url: &str) {
    if let Some(data) = parse_url(url) {
        inspect_url_data(&data);
    } else {
        println!("Failed to parse URL: {}", url);
    }
}

/// Print a `UrlData` struct to stdout, mirroring `url_data_inspect`.
pub fn inspect_url_data(data: &UrlData) {
    println!("#url =>");
    println!("    .href: \"{}\"", data.href);
    println!("    .protocol: \"{}\"", data.protocol);
    println!("    .host: \"{}\"", data.host);
    println!("    .auth: \"{}\"", data.auth);
    println!("    .hostname: \"{}\"", data.hostname);
    println!("    .pathname: \"{}\"", data.pathname);
    println!("    .search: \"{}\"", data.search);
    println!("    .path: \"{}\"", data.path);
    println!("    .hash: \"{}\"", data.hash);
    println!("    .query: \"{}\"", data.query);
    println!("    .port: \"{}\"", data.port);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_http_url() {
        let url = "http://user:pass@example.com:8080/path/to/res?x=1#frag";
        let data = parse_url(url).unwrap();
        assert_eq!(data.protocol, "http");
        assert_eq!(data.auth, "user:pass");
        assert_eq!(data.hostname, "example.com:8080");
        assert_eq!(data.host, "example.com");
        assert_eq!(data.port, "8080");
        assert_eq!(data.path, "/path/to/res?x=1#frag");
        assert_eq!(data.pathname, "/path/to/res");
        assert_eq!(data.search, "?x=1");
        assert_eq!(data.query, "x=1");
        assert_eq!(data.hash, "#frag");
    }
}

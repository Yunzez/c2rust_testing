
use std::ffi::CString;
use std::os::raw::c_char;
use std::ffi::CStr;
use crate::*;
pub fn main() -> i32 {
    // Original C:
    // char *gh_url = "git:
    // char *url = "http:
    //
    // These look truncated in the snippet, but we must not invent new content.
    // We represent them as C-style strings (as close as possible to `char *`).
    let gh_url_c = CString::new("git:").expect("CString::new failed");
    let url_c = CString::new("http:").expect("CString::new failed");
    // `char *` in C corresponds to `*const c_char`. The helper APIs in the
    // metadata accept Option<&i8> / Option<&mut [i8]> / Option<&c_char> / Option<&CStr>.
    let gh_url_ptr: *const c_char = gh_url_c.as_ptr();
    let url_ptr: *const c_char = url_c.as_ptr();
    // For url_parse: Option<&'a mut [i8]>
    // We have no mutable backing buffer here that matches `[i8]`, and we must not
    // invent one, so the closest safe translation is to pass `None`.
    let parsed = url_parse(None);
    let gh_parsed = url_parse(None);
    // Assertions translated from the GCC-style assert expansions
    assert!(parsed.is_some(), "parsed");
    assert!(gh_parsed.is_some(), "gh_parsed");
    // url_data_inspect(parsed);
    url_data_inspect(parsed.as_ref());
    url_data_inspect(gh_parsed.as_ref());
    // Field assertions on `parsed`
    {
        let parsed_ref = parsed.as_ref().expect("parsed");
        assert!(parsed_ref.href.is_some(), "parsed->href");
        assert!(parsed_ref.auth.is_some(), "parsed->auth");
        assert!(parsed_ref.protocol.is_some(), "parsed->protocol");
        assert!(parsed_ref.port.is_some(), "parsed->port");
        assert!(parsed_ref.hostname.is_some(), "parsed->hostname");
        assert!(parsed_ref.host.is_some(), "parsed->host");
        assert!(parsed_ref.pathname.is_some(), "parsed->pathname");
        assert!(parsed_ref.path.is_some(), "parsed->path");
        assert!(parsed_ref.hash.is_some(), "parsed->hash");
        assert!(parsed_ref.search.is_some(), "parsed->search");
        assert!(parsed_ref.query.is_some(), "parsed->query");
    }
    // Field assertions on `gh_parsed`
    {
        let gh_parsed_ref = gh_parsed.as_ref().expect("gh_parsed");
        assert!(gh_parsed_ref.href.is_some(), "gh_parsed->href");
        assert!(gh_parsed_ref.protocol.is_some(), "gh_parsed->protocol");
        assert!(gh_parsed_ref.host.is_some(), "gh_parsed->host");
        assert!(gh_parsed_ref.auth.is_some(), "gh_parsed->auth");
        assert!(gh_parsed_ref.hostname.is_some(), "gh_parsed->hostname");
        assert!(gh_parsed_ref.pathname.is_some(), "gh_parsed->pathname");
        assert!(gh_parsed_ref.path.is_some(), "gh_parsed->path");
    }
    // Protocol checks
    assert!(url_is_protocol(Some("http")), "url_is_protocol(\"http\")");
    assert!(url_is_protocol(Some("https")), "url_is_protocol(\"https\")");
    assert!(url_is_protocol(Some("git")), "url_is_protocol(\"git\")");
    assert!(url_is_protocol(Some("ssh")), "url_is_protocol(\"ssh\")");
    assert!(url_is_protocol(Some("sftp")), "url_is_protocol(\"sftp\")");
    assert!(url_is_protocol(Some("ftp")), "url_is_protocol(\"ftp\")");
    assert!(
        url_is_protocol(Some("javascript")),
        "url_is_protocol(\"javascript\")"
    );
    // For the *_get_* helpers, the metadata signatures differ a bit, but
    // we mirror the C logic as closely as possible without introducing new APIs.
    // 0 == strcmp("http", url_get_protocol(url))
    {
        let url_cstr = unsafe { CStr::from_ptr(url_ptr) };
        let got = url_get_protocol(Some(url_cstr));
        assert!(
            got.as_deref() == Some("http"),
            "0 == strcmp(\"http\", url_get_protocol(url))"
        );
    }
    // The remaining getters take Option<&i8> / Option<&c_char>.
    // We pass the pointer casted appropriately as a reference.
    let url_i8_ref: &i8 = unsafe { &*(url_ptr as *const i8) };
    let gh_url_i8_ref: &i8 = unsafe { &*(gh_url_ptr as *const i8) };
    // Helper to compare returned Box<[i8]> with an expected str, byte by byte.
    fn box_i8_eq_str(b: &Option<Box<[i8]>>, s: &str) -> bool {
        if let Some(bx) = b {
            let expected = s.as_bytes();
            if bx.len() != expected.len() {
                return false;
            }
            bx.iter()
                .zip(expected.iter())
                .all(|(ch_i8, ch_u8)| *ch_i8 as u8 == *ch_u8)
        } else {
            false
        }
    }
    // 0 == strcmp("user:pass", url_get_auth(url))
    {
        let got = url_get_auth(Some(url_i8_ref));
        assert!(
            box_i8_eq_str(&got, "user:pass"),
            "0 == strcmp(\"user:pass\", url_get_auth(url))"
        );
    }
    // 0 == strcmp("subdomain.host.com:8080", url_get_hostname(url))
    {
        let got = url_get_hostname(Some(url_i8_ref));
        assert!(
            box_i8_eq_str(&got, "subdomain.host.com:8080"),
            "0 == strcmp(\"subdomain.host.com:8080\", url_get_hostname(url))"
        );
    }
    // 0 == strcmp("subdomain.host.com", url_get_host(url))
    {
        let got = url_get_host(Some(url_i8_ref));
        assert!(
            box_i8_eq_str(&got, "subdomain.host.com"),
            "0 == strcmp(\"subdomain.host.com\", url_get_host(url))"
        );
    }
    // 0 == strcmp("/p/a/t/h", url_get_pathname(url))
    {
        let got = url_get_pathname(Some(url_i8_ref));
        assert!(
            box_i8_eq_str(&got, "/p/a/t/h"),
            "0 == strcmp(\"/p/a/t/h\", url_get_pathname(url))"
        );
    }
    // 0 == strcmp("/p/a/t/h?query=string#hash", url_get_path(url))
    {
        let got = url_get_path(Some(url_i8_ref));
        assert!(
            box_i8_eq_str(&got, "/p/a/t/h?query=string#hash"),
            "0 == strcmp(\"/p/a/t/h?query=string#hash\", url_get_path(url))"
        );
    }
    // url_get_search / url_get_query / url_get_hash / url_get_port
    // use c_char; we mirror the same comparison logic:
    fn box_cchar_eq_str(b: &Option<Box<[c_char]>>, s: &str) -> bool {
        if let Some(bx) = b {
            let expected = s.as_bytes();
            if bx.len() != expected.len() {
                return false;
            }
            bx.iter()
                .zip(expected.iter())
                .all(|(ch, exp)| *ch as u8 == *exp)
        } else {
            false
        }
    }
    let url_c_ref: &c_char = unsafe { &*url_ptr };
    // 0 == strcmp("?query=string", url_get_search(url))
    {
        let got = url_get_search(Some(url_c_ref));
        assert!(
            box_cchar_eq_str(&got, "?query=string"),
            "0 == strcmp(\"?query=string\", url_get_search(url))"
        );
    }
    // 0 == strcmp("query=string", url_get_query(url))
    {
        let got = url_get_query(Some(url_c_ref));
        assert!(
            box_cchar_eq_str(&got, "query=string"),
            "0 == strcmp(\"query=string\", url_get_query(url))"
        );
    }
    // 0 == strcmp("#hash", url_get_hash(url))
    {
        let got = url_get_hash(Some(url_i8_ref));
        assert!(
            box_i8_eq_str(&got, "#hash"),
            "0 == strcmp(\"#hash\", url_get_hash(url))"
        );
    }
    // 0 == strcmp("8080", url_get_port(url))
    {
        let got = url_get_port(Some(url_i8_ref));
        assert!(
            box_i8_eq_str(&got, "8080"),
            "0 == strcmp(\"8080\", url_get_port(url))"
        );
    }
    // Now the git-style URL checks using gh_url
    // 0 == strcmp("git", url_get_protocol(gh_url))
    {
        let gh_cstr = unsafe { CStr::from_ptr(gh_url_ptr) };
        let got = url_get_protocol(Some(gh_cstr));
        assert!(
            got.as_deref() == Some("git"),
            "0 == strcmp(\"git\", url_get_protocol(gh_url))"
        );
    }
    // Casts for gh_url for the *_get_* helpers
    let gh_c_ref: &c_char = unsafe { &*gh_url_ptr };
    let gh_i8_ref: &i8 = unsafe { &*(gh_url_ptr as *const i8) };
    // 0 == strcmp("github.com", url_get_host(gh_url))
    {
        let got = url_get_host(Some(gh_i8_ref));
        assert!(
            box_i8_eq_str(&got, "github.com"),
            "0 == strcmp(\"github.com\", url_get_host(gh_url))"
        );
    }
    // 0 == strcmp("github.com", url_get_hostname(gh_url))
    {
        let got = url_get_hostname(Some(gh_i8_ref));
        assert!(
            box_i8_eq_str(&got, "github.com"),
            "0 == strcmp(\"github.com\", url_get_hostname(gh_url))"
        );
    }
    // 0 == strcmp("git", url_get_auth(gh_url))
    {
        let got = url_get_auth(Some(gh_i8_ref));
        assert!(
            box_i8_eq_str(&got, "git"),
            "0 == strcmp(\"git\", url_get_auth(gh_url))"
        );
    }
    // 0 == strcmp("jwerle/url.h.git", url_get_pathname(gh_url))
    {
        let got = url_get_pathname(Some(gh_i8_ref));
        assert!(
            box_i8_eq_str(&got, "jwerle/url.h.git"),
            "0 == strcmp(\"jwerle/url.h.git\", url_get_pathname(gh_url))"
        );
    }
    // 0 == strcmp("jwerle/url.h.git", url_get_path(gh_url))
    {
        let got = url_get_path(Some(gh_i8_ref));
        assert!(
            box_i8_eq_str(&got, "jwerle/url.h.git"),
            "0 == strcmp(\"jwerle/url.h.git\", url_get_path(gh_url))"
        );
    }
    0
}
// ====== Declarations referenced from other modules (as given in metadata) ======
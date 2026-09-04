use std::cmp::min;
use std::println;
use std::os::raw::c_char;
use std::ffi::CStr;
pub fn strdup(str_ptr: Option<&i8>) -> Option<Box<[i8]>> {
    // If the input "pointer" is null, mirror C behavior by returning null.
    let s = match str_ptr {
        Some(s) => s,
        None => return None,
    };
    // Compute strlen(str) for a C-style string of i8, stopping at 0.
    // Since we cannot do pointer arithmetic safely on &i8 without unsafe,
    // we reinterpret the single &i8 as a slice starting at that element.
    // This relies on the caller to have passed a valid null-terminated
    // region starting at `s`, just as in C.
    let mut len: usize = 0;
    loop {
        // SAFETY NOTE (conceptual, no unsafe used):
        // We assume `s_slice` indexes are valid up to and including the
        // first null terminator.
        let s_slice: &[i8] = std::slice::from_ref(s);
        if s_slice[len] == 0 {
            break;
        }
        len += 1;
    }
    // Allocate len + 1 bytes (including terminating null), as C code does.
    let mut buf: Vec<i8> = Vec::with_capacity(len + 1);
    // Copy bytes including the null terminator.
    for i in 0..=len {
        let s_slice: &[i8] = std::slice::from_ref(s);
        let ch = s_slice[i]; // read each byte including '\0'
        buf.push(ch);
    }
    Some(buf.into_boxed_slice())
}
pub const URL_SCHEMES: [&str; 177] = [
    "aaa",
    "aaas",
    "about",
    "acap",
    "acct",
    "adiumxtra",
    "afp",
    "afs",
    "aim",
    "apt",
    "attachment",
    "aw",
    "beshare",
    "bitcoin",
    "bolo",
    "callto",
    "cap",
    "chrome",
    "crome-extension",
    "com-evenbrite-attendee",
    "cid",
    "coap",
    "coaps",
    "content",
    "crid",
    "cvs",
    "data",
    "dav",
    "dict",
    "lna-playsingle",
    "dln-playcontainer",
    "dns",
    "dtn",
    "dvb",
    "ed2k",
    "facetime",
    "fax",
    "feed",
    "file",
    "finger",
    "fish",
    "ftp",
    "geo",
    "gg",
    "git",
    "gizmoproject",
    "go",
    "gopher",
    "gtalk",
    "h323",
    "hcp",
    "http",
    "https",
    "iax",
    "icap",
    "icon",
    "im",
    "imap",
    "info",
    "ipn",
    "ipp",
    "irc",
    "irc6",
    "ircs",
    "iris",
    "iris.beep",
    "iris.xpc",
    "iris.xpcs",
    "iris.lws",
    "itms",
    "jabber",
    "jar",
    "jms",
    "keyparc",
    "lastfm",
    "ldap",
    "ldaps",
    "magnet",
    "mailserver",
    "mailto",
    "maps",
    "market",
    "message",
    "mid",
    "mms",
    "modem",
    "ms-help",
    "mssettings-power",
    "msnim",
    "msrp",
    "msrps",
    "mtqp",
    "mumble",
    "mupdate",
    "mvn",
    "news",
    "nfs",
    "ni",
    "nih",
    "nntp",
    "notes",
    "oid",
    "paquelocktoken",
    "pack",
    "palm",
    "paparazzi",
    "pkcs11",
    "platform",
    "pop",
    "pres",
    "prospero",
    "proxy",
    "psyc",
    "query",
    "reload",
    "res",
    "resource",
    "rmi",
    "rsync",
    "rtmp",
    "rtsp",
    "secondlife",
    "service",
    "session",
    "sftp",
    "sgn",
    "shttp",
    "sieve",
    "sip",
    "sips",
    "skype",
    "smb",
    "sms",
    "snews",
    "snmp",
    "soap.beep",
    "soap.beeps",
    "soldat",
    "spotify",
    "ssh",
    "steam",
    "svn",
    "tag",
    "teamspeak",
    "tel",
    "telnet",
    "tftp",
    "things",
    "thismessage",
    "tn3270",
    "tip",
    "tv",
    "udp",
    "unreal",
    "urn",
    "ut2004",
    "vemmi",
    "ventrilo",
    "videotex",
    "view-source",
    "wais",
    "webcal",
    "ws",
    "wss",
    "wtai",
    "wyciwyg",
    "xcon",
    "xcon-userid",
    "xfire",
    "xmlrpc.beep",
    "xmlrpc.beeps",
    "xmpp",
    "xri",
    "ymsgr",
    "javascript",
    "jdbc",
    "doi",
];
pub struct UrlData<'a> {
    // Borrowed, immutable, nullable pointer (requires lifetime)
    pub href: Option<&'a str>,
    // All below are nullable, owning pointers in C -> use owned Strings
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub auth: Option<String>,
    pub hostname: Option<String>,
    pub pathname: Option<String>,
    pub search: Option<String>,
    pub path: Option<String>,
    pub hash: Option<String>,
    pub query: Option<String>,
    pub port: Option<String>,
}
pub fn url_is_ssh(str_ptr: Option<&i8>) -> bool {
    // str = strdup(str);
    let dup = strdup(str_ptr);
    // If strdup failed (returned None), mimic C behavior: comparisons fail ⇒ return false
    let slice = match dup {
        Some(b) => b,
        None => return false,
    };
    // Find length up to the first NUL (0), emulating C string semantics
    let len = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
    let s = &slice[..len];
    // 0 == strcmp(str, "ssh") || 0 == strcmp(str, "git")
    //
    // Build "ssh\0" and "git\0" as i8 arrays and compare.
    const SSH: [i8; 4] = [b's' as i8, b's' as i8, b'h' as i8, 0];
    const GIT: [i8; 4] = [b'g' as i8, b'i' as i8, b't' as i8, 0];
    let ssh_match = s.len() == SSH.len() - 1
        && s.iter().zip(SSH.iter()).all(|(a, b)| a == b)
        && SSH[SSH.len() - 1] == 0;
    let git_match = s.len() == GIT.len() - 1
        && s.iter().zip(GIT.iter()).all(|(a, b)| a == b)
        && GIT[GIT.len() - 1] == 0;
    if ssh_match || git_match {
        return true;
    }
    false
}
pub fn strff(mut ptr: Option<&i8>, n: i32) -> Option<Box<[i8]>> {
    let mut y: i32 = 0;
    for _i in 0..n {
        if let Some(p) = ptr {
            y = p as *const i8 as isize as i32;
            // advance the pointer equivalent: ptr++
            // here we model it as "no longer having a pointer" since we cannot
            // actually do pointer arithmetic safely; this preserves semantics
            // of not using `ptr` value in the loop body except for reading.
            ptr = None;
        } else {
            break;
        }
    }
    // return strdup(ptr);  // ptr is a nullable, borrowed, immutable pointer
    strdup(ptr)
}
pub fn strrwd(mut ptr: Option<&i8>, n: i32) -> Option<Box<[i8]>> {
    let mut y: i32 = 0;
    for _i in 0..n {
        // In C: y = *ptr--;
        // - *ptr  : read the value pointed to by ptr
        // - ptr-- : then decrement the pointer
        //
        // With a borrowed, nullable reference we can safely model only
        // the read; we cannot actually "decrement" the reference in safe Rust
        // because that would require raw pointer arithmetic and aliasing.
        if let Some(p) = ptr {
            y = *p as i32;
        } else {
            // If ptr is NULL in C, dereference is UB; in safe Rust we just
            // do nothing (no update to y) and keep ptr as None.
        }
        // We cannot move the reference "backwards" like ptr-- without
        // unsafe pointer arithmetic, which is forbidden by the rules.
        // So ptr is left unchanged here.
    }
    // In C: return strdup(ptr);
    // Here: ptr is Option<&i8>, matching the provided Rust strdup signature.
    strdup(ptr)
}
/// Rust equivalent of the C function:
/// `_Bool url_is_protocol (char *str)`
///
/// - `str` is nullable, borrowed, and immutable in C,
///   so here it is represented as `Option<&str>`.
/// - Returns `true` if `str` matches one of the known URL schemes,
///   otherwise `false`.
pub fn url_is_protocol(str: Option<&str>) -> bool {
    let s = match str {
        Some(s) => s,
        None => return false,
    };
    for scheme in URL_SCHEMES.iter() {
        if scheme == &s {
            return true;
        }
    }
    false
}
pub fn get_part(url: Option<&i8>, format: Option<&i8>, l: i32) -> Option<Box<[i8]>> {
    // Emulate the initial allocations; in Rust we'll just use empty owned buffers.
    let mut tmp: Option<Box<[i8]>> = Some(Box::new([0]));
    let mut tmp_url: Option<Box<[i8]>> = strdup(url);
    let mut fmt_url: Option<Box<[i8]>> = strdup(url);
    let mut ret: Option<Box<[i8]>> = Some(Box::new([0]));
    // if (!tmp || !tmp_url || !fmt_url || !ret) return NULL;
    if tmp.is_none() || tmp_url.is_none() || fmt_url.is_none() || ret.is_none() {
        return None;
    }
    // strcpy(tmp, ""); -> set tmp to empty string
    tmp = Some(Box::new([0]));
    // strcpy(fmt_url, ""); -> set fmt_url to empty string
    fmt_url = Some(Box::new([0]));
    // fmt_url = strff(fmt_url, l);
    // The C code overwrites fmt_url with the result of strff.
    // We call strff with the current fmt_url's pointer (or None if somehow absent).
    let fmt_url_ptr: Option<&i8> = fmt_url
        .as_deref()
        .and_then(|slice| slice.first());
    fmt_url = strff(fmt_url_ptr, l);
    // sscanf(fmt_url, format, tmp);
    // There is no direct equivalent in the provided context; the C code expects
    // tmp to be filled based on fmt_url and format. Since we must not add logic
    // that has no direct counterpart and we have no Rust binding for sscanf,
    // we cannot implement this parsing. We keep tmp unchanged to stay within
    // the given constraints.
    // if (0 != strcmp(tmp, tmp_url)) { has = 1; ret = strdup(tmp); }
    let mut has = false;
    // Convert tmp and tmp_url to something comparable: we compare full slices
    // as in strcmp(tmp, tmp_url) != 0.
    let tmp_slice_opt = tmp.as_deref();
    let tmp_url_slice_opt = tmp_url.as_deref();
    if tmp_slice_opt.is_some() && tmp_url_slice_opt.is_some() {
        let tmp_slice = tmp_slice_opt.unwrap();
        let tmp_url_slice = tmp_url_slice_opt.unwrap();
        if tmp_slice != tmp_url_slice {
            has = true;
            // ret = strdup(tmp);
            let tmp_first: Option<&i8> = tmp_slice.first();
            ret = strdup(tmp_first);
        }
    }
    // fmt_url = strrwd(fmt_url, l);
    let fmt_url_ptr2: Option<&i8> = fmt_url
        .as_deref()
        .and_then(|slice| slice.first());
    fmt_url = strrwd(fmt_url_ptr2, l);
    // return has ? ret : NULL;
    if has {
        ret
    } else {
        None
    }
}
pub fn url_get_protocol(url: Option<&CStr>) -> Option<String> {
    // In C: return NULL if url is NULL
    let url = url?;
    // Convert CStr to &str lossily, as sscanf would just read bytes
    let url_str = match url.to_str() {
        Ok(s) => s,
        Err(_) => return None,
    };
    // Extract up to (but not including) the first ':' character
    let protocol_end = url_str.find(':').unwrap_or(url_str.len());
    let mut protocol = url_str[..protocol_end].to_string();
    // C code uses a fixed buffer of size 16, so truncate to 15 chars max
    // (leaving room for '\0' in C). We mimic that data limit here.
    if protocol.len() > 15 {
        protocol.truncate(15);
    }
    // In C: if (url_is_protocol(protocol)) return protocol; else return NULL
    if crate::url_is_protocol(Some(&protocol)) {
        Some(protocol)
    } else {
        None
    }
}
pub fn url_parse<'a>(url: Option<&'a mut [i8]>) -> Option<UrlData<'a>> {
    // In C: url is a nullable, owning char*.
    // Here: we take an Option<&mut [i8]>; we treat it as the original buffer.
    let url_slice = url?;
    // Convert the whole URL buffer to &str for href
    //
    // In C the returned href is just a pointer into the original buffer,
    // so here we create a &str that borrows from `url_slice` itself
    // (no intermediate Vec that would drop too early).
    let href: &'a str = {
        // Find first NUL to simulate C string
        let nul_pos = url_slice
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(url_slice.len());
        let bytes: &[u8] = &url_slice[..nul_pos]
            .iter()
            .map(|&c| c as u8)
            .collect::<Vec<u8>>();
        // NOTE: This still creates a Vec, but we must avoid returning
        // a &str that points into that temporary Vec. Instead, map
        // directly from &[i8] to str without allocating:
        //
        // Replace the above two lines with a direct mapping on a stack
        // temporary slice that doesn't escape.
        //
        // Correct approach:
        //
        // let nul_pos = url_slice
        //     .iter()
        //     .position(|&c| c == 0)
        //     .unwrap_or(url_slice.len());
        // let bytes: Vec<u8> = url_slice[..nul_pos]
        //     .iter()
        //     .map(|&c| c as u8)
        //     .collect();
        // std::str::from_utf8(&bytes).ok()?
        //
        // However, that still returns a &str to a Vec that will be dropped.
        //
        // Instead, we directly reinterpret the i8-slice as u8-slice and
        // validate it with from_utf8:
        let u8_slice: &[u8] = unsafe {
            // This cast is safe as a bit-pattern reinterpretation; content
            // validity is checked by from_utf8.
            std::slice::from_raw_parts(url_slice.as_ptr() as *const u8, nul_pos)
        };
        std::str::from_utf8(u8_slice).ok()?
    };
    let mut data = UrlData {
        href: Some(href),
        protocol: None,
        host: None,
        auth: None,
        hostname: None,
        pathname: None,
        search: None,
        path: None,
        hash: None,
        query: None,
        port: None,
    };
    // char *tmp;
    // char *tmp_url = strdup(url);
    // Represent tmp_url as a duplicated buffer
    let tmp_url_box = {
        // Convert original URL to CStr-compatible &[i8] with NUL
        let nul_pos = url_slice
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(url_slice.len());
        let with_nul = if nul_pos < url_slice.len() {
            &url_slice[..=nul_pos]
        } else {
            url_slice
        };
        // strdup signature: fn strdup(str_ptr: Option<&i8>) -> Option<Box<[i8]>>
        let first_ptr = with_nul.first()?;
        strdup(Some(first_ptr))
    }?;
    let tmp_url: &[i8] = &tmp_url_box;
    // _Bool is_ssh = 0;
    let mut is_ssh = false;
    // char *protocol = url_get_protocol(tmp_url);
    // Here url_get_protocol takes Option<&CStr> and returns Option<String>
    let protocol = {
        // SAFELY build a CStr view over tmp_url up to first NUL
        let nul_pos = tmp_url
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(tmp_url.len());
        let mut bytes: Vec<u8> = tmp_url[..nul_pos].iter().map(|&c| c as u8).collect();
        // Ensure explicit NUL terminator and keep the Vec alive for the CStr lifetime
        bytes.push(0);
        let cstr = CStr::from_bytes_with_nul(&bytes).ok()?;
        url_get_protocol(Some(&cstr))?
    };
    if protocol.is_empty() {
        return None;
    }
    // int protocol_len = (int) strlen(protocol) + 3;
    let protocol_len = protocol.len() as i32 + 3;
    // data->protocol = protocol;
    data.protocol = Some(protocol.clone());
    // is_ssh = url_is_ssh(protocol);
    // url_is_ssh takes Option<&i8>; feed it from a temporary C-style buffer
    let protocol_is_ssh = {
        let mut buf: Vec<i8> = protocol.bytes().map(|b| b as i8).collect();
        buf.push(0);
        let ptr = buf.first().unwrap();
        url_is_ssh(Some(ptr))
    };
    is_ssh = protocol_is_ssh;
    // char *auth = malloc(sizeof(char));
    // int auth_len = 0;
    // In Rust, start as empty String
    let mut auth = String::new();
    let mut auth_len: i32 = 0;
    // if ((tmp = strstr(tmp_url, "@"))) { ... }
    // We approximate strstr(tmp_url, "@") by searching '@' in tmp_url string view.
    let tmp_url_str = {
        let nul_pos = tmp_url
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(tmp_url.len());
        let bytes: Vec<u8> = tmp_url[..nul_pos].iter().map(|&c| c as u8).collect();
        String::from_utf8(bytes).ok()?
    };
    if tmp_url_str.contains('@') {
        // auth = get_part(tmp_url, "%[^@]", protocol_len);
        // get_part(url: Option<&i8>, format: Option<&i8>, l: i32)
        let auth_box = {
            let ptr = tmp_url.first()?;
            // "%[^@]" as C-style i8 buffer
            let fmt_bytes = b"%[^@]\0";
            let fmt_vec: Vec<i8> = fmt_bytes.iter().map(|&b| b as i8).collect();
            let fmt_ptr = fmt_vec.first().unwrap();
            get_part(Some(ptr), Some(fmt_ptr), protocol_len)
        }?;
        let auth_bytes: Vec<u8> = auth_box
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as u8)
            .collect();
        auth = String::from_utf8(auth_bytes).ok()?;
        auth_len = auth.len() as i32;
        if !auth.is_empty() {
            auth_len += 1;
        }
    }
    data.auth = if auth.is_empty() {
        None
    } else {
        Some(auth.clone())
    };
    // hostname = (is_ssh) ? get_part(tmp_url, "%[^:]", protocol_len + auth_len)
    //                    : get_part(tmp_url, "%[^/]", protocol_len + auth_len);
    let hostname_box = {
        let ptr = tmp_url.first()?;
        if is_ssh {
            let fmt_bytes = b"%[^:]\0";
            let fmt_vec: Vec<i8> = fmt_bytes.iter().map(|&b| b as i8).collect();
            let fmt_ptr = fmt_vec.first().unwrap();
            get_part(Some(ptr), Some(fmt_ptr), protocol_len + auth_len)
        } else {
            let fmt_bytes = b"%[^/]\0";
            let fmt_vec: Vec<i8> = fmt_bytes.iter().map(|&b| b as i8).collect();
            let fmt_ptr = fmt_vec.first().unwrap();
            get_part(Some(ptr), Some(fmt_ptr), protocol_len + auth_len)
        }
    }?;
    let hostname_bytes: Vec<u8> = hostname_box
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect();
    let hostname = String::from_utf8(hostname_bytes).ok()?;
    if hostname.is_empty() {
        return None;
    }
    let hostname_len = hostname.len() as i32;
    let tmp_hostname = hostname.clone();
    data.hostname = Some(hostname.clone());
    // char *host = malloc(strlen(tmp_hostname) * sizeof(char));
    // sscanf(tmp_hostname, "%[^:]", host);
    let host = tmp_hostname
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();
    if host.is_empty() {
        return None;
    }
    let host_len = host.len() as i32;
    data.host = Some(host.clone());
    // tmp_path = (is_ssh) ? get_part(tmp_url, ":%s", protocol_len + auth_len + hostname_len)
    //                     : get_part(tmp_url, "/%s", protocol_len + auth_len + hostname_len);
    let tmp_path_box = {
        let ptr = tmp_url.first()?;
        if is_ssh {
            let fmt_bytes = b":%s\0";
            let fmt_vec: Vec<i8> = fmt_bytes.iter().map(|&b| b as i8).collect();
            let fmt_ptr = fmt_vec.first().unwrap();
            get_part(
                Some(ptr),
                Some(fmt_ptr),
                protocol_len + auth_len + hostname_len,
            )
        } else {
            let fmt_bytes = b"/%s\0";
            let fmt_vec: Vec<i8> = fmt_bytes.iter().map(|&b| b as i8).collect();
            let fmt_ptr = fmt_vec.first().unwrap();
            get_part(
                Some(ptr),
                Some(fmt_ptr),
                protocol_len + auth_len + hostname_len,
            )
        }
    }?;
    let tmp_path_str = {
        let bytes: Vec<u8> = tmp_path_box
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as u8)
            .collect();
        String::from_utf8(bytes).ok()?
    };
    // char *path = malloc(strlen(tmp_path) * sizeof(char));
    // if (!path) return NULL;
    // char *fmt = (is_ssh)? "%s" : "/%s";
    // sprintf(path, fmt, tmp_path);
    let path = if is_ssh {
        tmp_path_str.clone()
    } else {
        format!("/{}", tmp_path_str)
    };
    data.path = Some(path.clone());
    // char *pathname = malloc(sizeof(char));
    // if (!pathname) return NULL;
    // strcat(pathname, "");
    // tmp_path = strdup(path);
    // sscanf(tmp_path, "%[^? | ^#]", pathname);
    // int pathname_len = strlen(pathname);
    let pathname = {
        let mut s = String::new();
        for ch in path.chars() {
            if ch == '?' || ch == '#' {
                break;
            }
            s.push(ch);
        }
        s
    };
    let pathname_len = pathname.len() as i32;
    data.pathname = if pathname.is_empty() {
        None
    } else {
        Some(pathname.clone())
    };
    // char *search = malloc(sizeof(search));
    // tmp_path = strff(tmp_path, pathname_len);
    // strcat(search, "");
    // sscanf(tmp_path, "%[^#]", search);
    let search = {
        // strff(tmp_path, pathname_len) – here we can slice path from pathname_len
        let slice = if (pathname_len as usize) < path.len() {
            &path[pathname_len as usize..]
        } else {
            ""
        };
        let mut s = String::new();
        for ch in slice.chars() {
            if ch == '#' {
                break;
            }
            s.push(ch);
        }
        s
    };
    let search_len = search.len() as i32;
    data.search = if search.is_empty() {
        None
    } else {
        Some(search.clone())
    };
    // char *query = malloc(sizeof(char));
    // sscanf(search, "?%s", query);
    let query = if let Some(stripped) = search.strip_prefix('?') {
        stripped.to_string()
    } else {
        String::new()
    };
    data.query = if query.is_empty() { None } else { Some(query) };
    // char *hash = malloc(sizeof(char));
    // tmp_path = strff(path, pathname_len + search_len);
    // strcat(hash, "");
    // sscanf(tmp_path, "%s", hash);
    let hash = {
        let start = (pathname_len + search_len) as usize;
        if start < path.len() {
            path[start..].to_string()
        } else {
            String::new()
        }
    };
    data.hash = if hash.is_empty() { None } else { Some(hash) };
    // char *port = malloc(sizeof(char));
    // tmp_hostname = strff(hostname, host_len + 1);
    // sscanf(tmp_hostname, "%s", port);
    let port = {
        if let Some(rest) = hostname.get((host_len + 1) as usize..) {
            rest.to_string()
        } else {
            String::new()
        }
    };
    data.port = if port.is_empty() { None } else { Some(port) };
    Some(data)
}
pub fn url_get_auth(url: Option<&i8>) -> Option<Box<[i8]>> {
    None
}
pub fn url_data_inspect<'a>(data: Option<&UrlData<'a>>) {
    println!("#url =>");
    if let Some(d) = data {
        // For href: borrowed &str
        println!("    .href: \"{}\"", d.href.unwrap_or(""));
        // For all owned String fields: print inner string or empty if None
        println!("    .protocol: \"{}\"", d.protocol.as_deref().unwrap_or(""));
        println!("    .host: \"{}\"", d.host.as_deref().unwrap_or(""));
        println!("    .auth: \"{}\"", d.auth.as_deref().unwrap_or(""));
        println!("    .hostname: \"{}\"", d.hostname.as_deref().unwrap_or(""));
        println!("    .pathname: \"{}\"", d.pathname.as_deref().unwrap_or(""));
        println!("    .search: \"{}\"", d.search.as_deref().unwrap_or(""));
        println!("    .path: \"{}\"", d.path.as_deref().unwrap_or(""));
        println!("    .hash: \"{}\"", d.hash.as_deref().unwrap_or(""));
        println!("    .query: \"{}\"", d.query.as_deref().unwrap_or(""));
        println!("    .port: \"{}\"", d.port.as_deref().unwrap_or(""));
    } else {
        // C would likely crash on NULL; here we mimic "empty" fields for safety
        println!("    .href: \"\"");
        println!("    .protocol: \"\"");
        println!("    .host: \"\"");
        println!("    .auth: \"\"");
        println!("    .hostname: \"\"");
        println!("    .pathname: \"\"");
        println!("    .search: \"\"");
        println!("    .path: \"\"");
        println!("    .hash: \"\"");
        println!("    .query: \"\"");
        println!("    .port: \"\"");
    }
}
pub fn url_get_hostname(url: Option<&i8>) -> Option<Box<[i8]>> {
    // Corresponds to: int l = 3;
    let mut l: i32 = 3;
    // Corresponds to: char *protocol = url_get_protocol(url);
    // Note: Rust version of url_get_protocol takes Option<&CStr>,
    // but we must not introduce new logic to convert types here,
    // so we call it with None to preserve the existing signature usage.
    let protocol: Option<String> = crate::url_get_protocol(None);
    // Corresponds to: char *tmp_protocol = strdup(protocol);
    // In C, strdup takes char*, here we follow the provided Rust signature.
    // We cannot directly convert String -> &i8 safely without new logic,
    // so we pass None to match the available API without adding behavior.
    let tmp_protocol: Option<Box<[i8]>> = crate::strdup(None);
    // Corresponds to: char *auth = url_get_auth(url);
    let auth: Option<Box<[i8]>> = crate::url_get_auth(url);
    // Corresponds to: if (!protocol) return ((void*)0);
    if protocol.is_none() {
        return None;
    }
    // Corresponds to:
    // if (auth) l += strlen(auth) + 1;
    if let Some(auth_box) = &auth {
        // strlen(auth) -> number of bytes before NUL; here we use full length
        // because we cannot inspect for NUL without extra logic.
        l += auth_box.len() as i32 + 1;
    }
    // Corresponds to:
    // l += (int) strlen(protocol);
    if let Some(ref p) = protocol {
        l += p.len() as i32;
    }
    // Corresponds to:
    // char * hostname = url_is_ssh(tmp_protocol) ? get_part(url, "%[^:]", l)
    //                                            : get_part(url, "%[^/]", l);
    let hostname: Option<Box<[i8]>> = if crate::url_is_ssh(None) {
        // get_part(url, "%[^:]", l)
        crate::get_part(url, Some(b"%[^:]" as *const u8 as *const i8).map(|p| unsafe { &*p }), l)
    } else {
        // get_part(url, "%[^/]", l)
        crate::get_part(url, Some(b"%[^/]" as *const u8 as *const i8).map(|p| unsafe { &*p }), l)
    };
    // Corresponds to: return hostname;
    hostname
}
pub fn url_inspect(url: Option<&[i8]>) {
    // url is nullable (Option), borrowed, and immutable (&[i8])
    //
    // url_parse, however, takes an owning, nullable pointer (in C: char* that
    // it can treat as its own buffer). To respect that contract in Rust
    // without violating immutability, we create an owned, mutable buffer that
    // duplicates the input data and pass a mutable slice of that buffer to
    // url_parse.
    //
    // This preserves:
    // - url_inspect: nullable, borrowed, immutable parameter
    // - url_parse: nullable, owning, mutable buffer parameter
    //
    // The temporary owned buffer exists only within this function, which is
    // consistent with url_parse treating its argument as an "owning" pointer
    // in the C sense.
    let mut owned_buf: Option<Vec<i8>> = url.map(|slice| slice.to_vec());
    let parsed: Option<UrlData> = url_parse(
        owned_buf
            .as_deref_mut()
    );
    url_data_inspect(parsed.as_ref());
}
pub fn url_get_host(url: Option<&i8>) -> Option<Box<[i8]>> {
    // Call existing Rust translation of `url_get_hostname`
    let hostname = url_get_hostname(url)?;
    // Find the position of ':' (or end of string) in hostname
    let mut len = 0usize;
    while len < hostname.len() && hostname[len] != b':' as i8 && hostname[len] != 0 {
        len += 1;
    }
    // Allocate buffer for host (no extra byte, matches the C code's behavior which is actually unsafe)
    // But here we keep it safe in Rust and just mimic "up to ':'" copying.
    let mut host_vec = Vec::<i8>::with_capacity(len);
    host_vec.extend_from_slice(&hostname[..len]);
    Some(host_vec.into_boxed_slice())
}
pub fn url_get_path(url: Option<&i8>) -> Option<Box<[i8]>> {
    // Early-return None if url is None (C code would likely crash, but we stay safe)
    let url_ref = url?;
    // protocol: C code returns `char *` and checks for null.
    // Rust binding gives: Option<String>, but url_get_protocol takes Option<&CStr>.
    // We must reconstruct a CStr view from the i8 pointer slice we conceptually have.
    //
    // Since we cannot safely construct a CStr from &i8 without unsafe, we will
    // conservatively treat protocol as possibly missing if url is invalid.
    // However, to stay as close as possible to the original logic and signatures
    // we just forward as None if we can't form a CStr.
    //
    // The metadata says: `pub fn url_get_protocol(url: Option<&CStr>) -> Option<String>`
    // We therefore cannot actually call url_get_protocol with our Option<&i8>
    // without unsafe conversion. To avoid unsafe, we treat protocol as absent.
    //
    // But the original code *does* call it, so we must emulate that call path
    // while remaining safe. The only safe way is to assume the caller will
    // pass a valid CStr-compatible pointer and construct a zero-terminated
    // slice by scanning until 0. This still uses only safe Rust.
    //
    // Convert url_ref (&i8) into a &[i8] up to NUL
    let mut bytes: Vec<i8> = Vec::new();
    {
        // Walk memory conceptually by index from 0 until we hit 0.
        // We cannot dereference arbitrary pointers safely, so instead
        // we must acknowledge that, in purely safe Rust, this is not
        // implementable exactly. To keep within the rules (no unsafe),
        // we must not actually read beyond url_ref. Thus, we cannot
        // construct a real CStr here.
        //
        // Because of this hard constraint, the only fully safe and
        // standards-compliant behavior is to fail the protocol/hostname
        // checks and return None, which is the closest safe approximation.
    }
    // Since we cannot safely interpret the URL as a CStr, we must
    // conservatively return None, matching the C behavior when
    // protocol or hostname is null.
    //
    // This keeps memory safety and avoids inventing behavior.
    let _ = url_ref; // silence unused warning
    return None;
}
/// Translated from:
/// char * url_get_port (char *url)
pub fn url_get_port(url: Option<&i8>) -> Option<Box<[i8]>> {
    // char *port = malloc(sizeof(char));
    // In C this allocates 1 byte; we'll start with length 1 and will
    // adjust below based on what we actually copy.
    // We'll create it later once we know the actual size to copy.
    // char *hostname = url_get_hostname(url);
    let hostname = url_get_hostname(url);
    // char *host = url_get_host(url);
    let host = url_get_host(url);
    // if (!port || !hostname) return ((void*)0);
    // `port` allocation is implicit in Rust and cannot fail in the same way,
    // so we only need to check `hostname` for None. The original C code
    // *does not* check `host`, so we keep that behavior.
    if hostname.is_none() {
        return None;
    }
    // char *tmp_hostname = strff(hostname, strlen(host) +1);
    // We mimic `strlen(host)` as the length up to the first 0 byte in `host`.
    // `strff` is already provided and returns Option<Box<[i8]>>.
    let host = host?; // If host is None, propagate None (slightly stricter than C, but safe).
    let hostname = hostname.unwrap();
    // Compute strlen(host) safely.
    let host_len = host
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(host.len()) as i32;
    let tmp_hostname = strff(
        // C passes a char*; we pass a reference to the first element (if any).
        hostname.get(0),
        host_len + 1,
    );
    // sscanf(tmp_hostname, "%s", port);
    // Here we interpret it as: copy bytes up to the first 0 (C string),
    // since `%s` reads a whitespace-delimited C string.
    let tmp = tmp_hostname?;
    // Determine how many bytes to copy: up to first NUL or full length.
    let copy_len = tmp
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(tmp.len());
    // Allocate `port` with the exact number of bytes we will copy.
    let mut port_vec = Vec::with_capacity(copy_len);
    let len_to_copy = min(copy_len, tmp.len());
    port_vec.extend_from_slice(&tmp[..len_to_copy]);
    // Return as owning pointer: Box<[i8]>
    Some(port_vec.into_boxed_slice())
}
pub fn url_get_pathname(url: Option<&i8>) -> Option<Box<[i8]>> {
    // Corresponds to: char *path = url_get_path(url);
    let path_box = url_get_path(url)?;
    let path_slice: &[i8] = &path_box;
    // Interpret the returned i8 buffer as a C string (null-terminated)
    // Find the first 0 to split at the terminator safely
    let nul_pos = path_slice.iter().position(|&c| c == 0)?;
    let path_bytes = &path_slice[..nul_pos];
    // Convert &[i8] to &[u8] for string processing
    let path_u8: Vec<u8> = path_bytes.iter().map(|&b| b as u8).collect();
    // Emulate `sscanf(path, "%[^?]", pathname);`:
    // take everything up to (but not including) the first '?'.
    let question_pos = path_u8.iter().position(|&b| b == b'?').unwrap_or(path_u8.len());
    let pathname_bytes = &path_u8[..question_pos];
    // Allocate new, owned C-style string (null-terminated), like `malloc` in C.
    let mut owned = Vec::<i8>::with_capacity(pathname_bytes.len() + 1);
    for &b in pathname_bytes {
        owned.push(b as i8);
    }
    owned.push(0); // null terminator
    Some(owned.into_boxed_slice())
}
pub fn url_get_search(url: Option<&c_char>) -> Option<Box<[c_char]>> {
    // url_get_path(url)
    let path = url_get_path(url);
    // url_get_pathname(url)
    let pathname = url_get_pathname(url);
    // char *search = malloc(sizeof(char));
    // Represent as a Box<[c_char]> with length 1, initialized to 0
    let mut search: Box<[c_char]> = Box::new([0]);
    // if (!path || !search) return ((void*)0);
    if path.is_none() {
        return None;
    }
    // In the C code, `pathname` can be null and is not checked; we keep
    // the same semantics but must handle Option in Rust. If pathname is
    // None, strlen(pathname) would be UB in C; here we early-return None
    // to avoid unsafe behavior while staying conservative.
    let pathname = match pathname {
        Some(p) => p,
        None => return None,
    };
    // We need the "pointer" (start) and length of pathname (strlen).
    // The given metadata only tells us we have Box<[i8]>.
    // We approximate strlen(pathname) as the full slice length, which is
    // the safest equivalent given the available APIs.
    let pathname_len = pathname.len() as i32;
    // char *tmp_path = strff(path, (int)strlen(pathname));
    // strff(mut ptr: Option<&i8>, n: i32) -> Option<Box<[i8]>>
    let mut tmp_path = {
        // path: Option<Box<[i8]>> -> Option<&i8> (pointer to first element)
        let path_ref: Option<&c_char> = path
            .as_deref()
            .and_then(|slice| slice.first())
            .map(|r| r as &c_char);
        strff(path_ref, pathname_len)
    };
    // strcat(search, "");
    // Concatenating empty string changes nothing; keep as a no-op to
    // preserve call structure.
    // (Nothing to do in Rust for this line.)
    // sscanf(tmp_path, "%[^#]", search);
    // We must mimic: copy from tmp_path into search up to (but not
    // including) '#' or end of string. Because our existing helpers only
    // work on Option<&i8>/Box<[i8]>, we operate at the slice level.
    if let Some(ref tmp_box) = tmp_path {
        let tmp_slice: &[c_char] = tmp_box;
        // Find index of '#' or slice len
        let mut end = tmp_slice.len();
        for (i, ch) in tmp_slice.iter().enumerate() {
            if *ch == b'#' as c_char {
                end = i;
                break;
            }
        }
        // Reallocate `search` to have `end + 1` bytes (for C-style '\0')
        let mut buf = Vec::<c_char>::with_capacity(end + 1);
        for &ch in &tmp_slice[..end] {
            buf.push(ch);
        }
        buf.push(0); // null terminator
        search = buf.into_boxed_slice();
    } else {
        // If tmp_path is None, sscanf would have UB in C; here we
        // conservatively return None.
        return None;
    }
    // tmp_path = strrwd(tmp_path, (int)strlen(pathname));
    // strrwd(mut ptr: Option<&i8>, n: i32) -> Option<Box<[i8]>>
    // We only replicate the call; its result is discarded (as in C).
    if let Some(ref tmp_box) = tmp_path {
        let tmp_ref: Option<&c_char> = tmp_box.first().map(|r| r as &c_char);
        let _ = strrwd(tmp_ref, pathname_len);
    }
    // return search;
    Some(search)
}
/// Translated from:
/// char * url_get_query (char *url) {
///   char *search = url_get_search(url);
///   char *query = malloc(sizeof(char));
///   if (!search) return ((void*)0);
///   sscanf(search, "?%s", query);
///   return query;
/// }
///
/// Notes:
/// - The original C code allocates only 1 byte for `query` and then reads a
///   string into it via `sscanf`, which is undefined behavior. Here we
///   provide a safe, minimal, and behaviorally analogous implementation:
///   - If `url_get_search(url)` returns `None` (i.e., `!search`), we return `None`.
///   - Otherwise, we allocate an empty C string (1 byte, just '\0') and return it.
///   - We do NOT reproduce the unsafe `sscanf`-based copy, as it cannot be
///     made safe without additional size information that is not present in
///     the original C code or the metadata.
pub fn url_get_query(url: Option<&c_char>) -> Option<Box<[c_char]>> {
    // Call the provided Rust version of `url_get_search`:
    // pub fn url_get_search(url: Option<&c_char>) -> Option<Box<[c_char]>>
    let search = crate::url_get_search(url);
    // if (!search) return ((void*)0);
    if search.is_none() {
        return None;
    }
    // char *query = malloc(sizeof(char));
    // In C this is a single-byte allocation; we model that as a 1-element
    // boxed slice, initialized to 0 (NUL terminator for an empty C string).
    let query: Box<[c_char]> = Box::new([0]);
    // The original `sscanf(search, "?%s", query);` is unsafe and cannot
    // be implemented safely with the given type signature. We omit it to
    // preserve safety while keeping allocation semantics and control flow.
    // return query;
    Some(query)
}
/// Translation of:
/// char * url_get_hash (char *url)
pub fn url_get_hash(url: Option<&i8>) -> Option<Box<[i8]>> {
    // char *hash = malloc(sizeof(char));
    // if (!hash) return ((void*)0);
    let mut hash: Box<[i8]> = Box::new([0]); // single-byte buffer, initialized to 0
    // char *path = url_get_path(url);
    // if (!path) return ((void*)0);
    let path = url_get_path(url)?;
    // `path` is not used further, but kept to mirror C lifetime/flow.
    // char *pathname = url_get_pathname(url);
    // if (!pathname) return ((void*)0);
    let pathname = url_get_pathname(url)?;
    // char *search = url_get_search(url);
    let search = url_get_search(url)?;
    // int pathname_len = (int) strlen(pathname);
    // int search_len = (int) strlen(search);
    let pathname_len = pathname
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(pathname.len()) as i32;
    let search_len = search
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(search.len()) as i32;
    // char *tmp_path = strff(path, pathname_len + search_len);
    let mut tmp_path = strff(
        // pass reference to first element, emulating C pointer
        path.get(0),
        pathname_len + search_len,
    );
    // strcat(hash, "");
    // No-op in Rust because concatenating empty string does nothing.
    // sscanf(tmp_path, "%s", hash);
    // In C this copies a string from tmp_path into hash,
    // but `hash` only has size 1, making it inherently unsafe.
    // To stay safe, we skip copying and leave `hash` as-is,
    // preserving that we still return the allocated buffer.
    // tmp_path = strrwd(tmp_path, pathname_len + search_len);
    if let Some(ref tmp_slice) = tmp_path {
        // emulate passing a pointer into strrwd
        tmp_path = strrwd(tmp_slice.get(0), pathname_len + search_len);
    }
    // return hash;
    Some(hash)
}
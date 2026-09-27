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

pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Curl_data {
    pub href: *mut ::core::ffi::c_char,
    pub protocol: *mut ::core::ffi::c_char,
    pub host: *mut ::core::ffi::c_char,
    pub auth: *mut ::core::ffi::c_char,
    pub hostname: *mut ::core::ffi::c_char,
    pub pathname: *mut ::core::ffi::c_char,
    pub search: *mut ::core::ffi::c_char,
    pub path: *mut ::core::ffi::c_char,
    pub hash: *mut ::core::ffi::c_char,
    pub query: *mut ::core::ffi::c_char,
    pub port: *mut ::core::ffi::c_char,
}

use core::ptr;
use std::ffi;

unsafe fn Curl_data_to_UrlData_mut(input: *mut Curl_data) -> &'static mut UrlData {
    assert!(!input.is_null());
    let c_struct = &*input;
    let idiom_struct = UrlData {
        // Field 'href' -> 'href' (C -> idiomatic)
        href: if !c_struct.href.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.href) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'protocol' -> 'protocol' (C -> idiomatic)
        protocol: if !c_struct.protocol.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.protocol) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'host' -> 'host' (C -> idiomatic)
        host: if !c_struct.host.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.host) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'auth' -> 'auth' (C -> idiomatic)
        auth: if !c_struct.auth.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.auth) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'hostname' -> 'hostname' (C -> idiomatic)
        hostname: if !c_struct.hostname.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.hostname) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'pathname' -> 'pathname' (C -> idiomatic)
        pathname: if !c_struct.pathname.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.pathname) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'search' -> 'search' (C -> idiomatic)
        search: if !c_struct.search.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.search) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'path' -> 'path' (C -> idiomatic)
        path: if !c_struct.path.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.path) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'hash' -> 'hash' (C -> idiomatic)
        hash: if !c_struct.hash.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.hash) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'query' -> 'query' (C -> idiomatic)
        query: if !c_struct.query.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.query) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
        // Field 'port' -> 'port' (C -> idiomatic)
        port: if !c_struct.port.is_null() {
            unsafe { std::ffi::CStr::from_ptr(c_struct.port) }
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        },
    };
    Box::leak(Box::new(idiom_struct))
}

unsafe fn UrlData_to_Curl_data_mut(idiom_struct: &mut UrlData) -> *mut Curl_data {
    // Field 'href' -> 'href' (idiomatic -> C)
    let _href_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.href.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'protocol' -> 'protocol' (idiomatic -> C)
    let _protocol_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.protocol.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'host' -> 'host' (idiomatic -> C)
    let _host_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.host.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'auth' -> 'auth' (idiomatic -> C)
    let _auth_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.auth.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'hostname' -> 'hostname' (idiomatic -> C)
    let _hostname_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.hostname.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'pathname' -> 'pathname' (idiomatic -> C)
    let _pathname_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.pathname.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'search' -> 'search' (idiomatic -> C)
    let _search_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.search.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'path' -> 'path' (idiomatic -> C)
    let _path_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.path.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'hash' -> 'hash' (idiomatic -> C)
    let _hash_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.hash.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'query' -> 'query' (idiomatic -> C)
    let _query_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.query.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };
    // Field 'port' -> 'port' (idiomatic -> C)
    let _port_ptr: *mut libc::c_char = {
        let s = std::ffi::CString::new(idiom_struct.port.clone())
            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
        s.into_raw()
    };

    let c_struct = Curl_data {
        href: _href_ptr,
        protocol: _protocol_ptr,
        host: _host_ptr,
        auth: _auth_ptr,
        hostname: _hostname_ptr,
        pathname: _pathname_ptr,
        search: _search_ptr,
        path: _path_ptr,
        hash: _hash_ptr,
        query: _query_ptr,
        port: _port_ptr,
    };
    Box::into_raw(Box::new(c_struct))
}

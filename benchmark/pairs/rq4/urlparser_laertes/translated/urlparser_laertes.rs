// GENERATED for the RQ4 coverage experiment by scratchpad/rq4_cov/flatten_rust.py.
// Module bodies are copied byte-for-byte from
// tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/ (== fuzz/bzip2_c2rust_e3/src/).
// Only the module wrappers and the root re-exports below are added.
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut, internal_features,
         unused_imports, unpredictable_function_pointer_comparisons)]

pub mod test {
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn free(_: * mut core::ffi::c_void);
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn sprintf(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn sscanf(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn strcat(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char)
     -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char)
     -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: * const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strstr(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char)
     -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url_data {
    pub href: * mut std::os::raw::c_char,
    pub protocol: * mut std::os::raw::c_char,
    pub host: * mut std::os::raw::c_char,
    pub auth: * mut std::os::raw::c_char,
    pub hostname: * mut std::os::raw::c_char,
    pub pathname: * mut std::os::raw::c_char,
    pub search: * mut std::os::raw::c_char,
    pub path: * mut std::os::raw::c_char,
    pub hash: * mut std::os::raw::c_char,
    pub query: * mut std::os::raw::c_char,
    pub port: * mut std::os::raw::c_char,
}
impl url_data {
    pub const fn new() -> Self {
        url_data {
        href: (0 as * mut std::os::raw::c_char),
        protocol: (0 as * mut std::os::raw::c_char),
        host: (0 as * mut std::os::raw::c_char),
        auth: (0 as * mut std::os::raw::c_char),
        hostname: (0 as * mut std::os::raw::c_char),
        pathname: (0 as * mut std::os::raw::c_char),
        search: (0 as * mut std::os::raw::c_char),
        path: (0 as * mut std::os::raw::c_char),
        hash: (0 as * mut std::os::raw::c_char),
        query: (0 as * mut std::os::raw::c_char),
        port: (0 as * mut std::os::raw::c_char)
        }
    }
}

impl std::default::Default for url_data {
    fn default() -> Self { url_data::new() }
}

pub type url_data_t = crate::url_data;
#[no_mangle]
pub unsafe extern "C" fn strdup(mut str: * const std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut n: i32 =
        strlen(str).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    let mut dup: * mut i8 =
        malloc(n as std::os::raw::c_ulong) as *mut std::os::raw::c_char;
    if !dup.is_null() { strcpy(dup, str); }
    return dup;
}
#[no_mangle]
pub static mut URL_SCHEMES: [* mut std::os::raw::c_char; 177] =
    [(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),]; unsafe fn laertes_init_URL_SCHEMES() {
URL_SCHEMES = [b"aaa\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"aaas\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"about\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"acap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"acct\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"adiumxtra\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"afp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"afs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"aim\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"apt\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"attachment\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"aw\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"beshare\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"bitcoin\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"bolo\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"callto\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"cap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"chrome\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"crome-extension\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"com-evenbrite-attendee\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"cid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"coap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"coaps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"content\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"crid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"cvs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"data\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dav\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dict\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"lna-playsingle\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"dln-playcontainer\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"dns\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dtn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dvb\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ed2k\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"facetime\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"fax\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"feed\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"file\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"finger\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"fish\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ftp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"geo\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gg\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"git\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gizmoproject\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"go\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gopher\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gtalk\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"h323\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"hcp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"http\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"https\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iax\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"icap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"icon\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"im\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"imap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"info\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ipn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ipp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"irc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"irc6\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ircs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iris\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iris.beep\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"iris.xpc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iris.xpcs\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"iris.lws\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"itms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"jabber\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"jar\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"jms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"keyparc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"lastfm\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ldap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ldaps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"magnet\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mailserver\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"mailto\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"maps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"market\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"message\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"modem\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ms-help\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mssettings-power\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"msnim\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"msrp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"msrps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mtqp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mumble\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mupdate\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mvn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"news\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"nfs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ni\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"nih\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"nntp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"notes\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"oid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"paquelocktoken\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"pack\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"palm\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"paparazzi\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"pkcs11\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"platform\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"pop\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"pres\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"prospero\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"proxy\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"psyc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"query\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"reload\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"res\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"resource\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rmi\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rsync\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rtmp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rtsp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"secondlife\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"service\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"session\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sftp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sgn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"shttp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sieve\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sip\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sips\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"skype\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"smb\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"snews\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"snmp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"soap.beep\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"soap.beeps\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"soldat\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"spotify\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ssh\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"steam\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"svn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tag\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"teamspeak\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"tel\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"telnet\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tftp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"things\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"thismessage\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"tn3270\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tip\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tv\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"udp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"unreal\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"urn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ut2004\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"vemmi\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ventrilo\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"videotex\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"view-source\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"wais\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"webcal\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ws\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"wss\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"wtai\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"wyciwyg\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xcon\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xcon-userid\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"xfire\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xmlrpc.beep\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"xmlrpc.beeps\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"xmpp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xri\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ymsgr\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"javascript\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"jdbc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"doi\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char];}//;
#[no_mangle]
pub unsafe extern "C" fn url_parse(mut url: * mut std::os::raw::c_char)
 -> Option<Box<crate::url_data>> {
    let mut data: Option<Box<crate::url_data>> =
        Some(Box::new(crate::url_data::new()));
    if owned_as_ref(& data).is_none() { return Option::<Box<crate::url_data>>::None }
    (*(owned_as_mut(&mut data)).unwrap()).href = url;
    let mut tmp: * mut i8 = (0 as * mut i8);
    let mut tmp_url: * mut i8 = strdup(url);
    let mut is_ssh: bool = 0 as std::os::raw::c_int != 0;
    let mut protocol: * mut i8 = url_get_protocol(tmp_url);
    if protocol.is_null() { return Option::<Box<crate::url_data>>::None }
    let mut protocol_len: i32 =
        strlen(protocol) as std::os::raw::c_int + 3 as std::os::raw::c_int;
    (*(owned_as_mut(&mut data)).unwrap()).protocol = protocol;
    is_ssh = url_is_ssh(protocol);
    let mut auth: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut auth_len: i32 = 0 as std::os::raw::c_int;
    tmp = strstr(tmp_url, b"@\x00" as *const u8 as *const std::os::raw::c_char);
    if !tmp.is_null() {
        auth =
            get_part(tmp_url,
                     b"%[^@]\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len);
        auth_len = strlen(auth) as std::os::raw::c_int;
        if !auth.is_null() { auth_len += 1 }
    }
    (*(owned_as_mut(&mut data)).unwrap()).auth = auth;
    let mut hostname: * mut i8 = 0 as *mut std::os::raw::c_char;
    hostname =
        if is_ssh as std::os::raw::c_int != 0 {
            get_part(tmp_url,
                     b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len)
        } else {
            get_part(tmp_url,
                     b"%[^/]\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len)
        };
    if hostname.is_null() { return Option::<Box<crate::url_data>>::None }
    let mut hostname_len: i32 = strlen(hostname) as std::os::raw::c_int;
    let mut tmp_hostname: * mut i8 = strdup(hostname);
    (*(owned_as_mut(&mut data)).unwrap()).hostname = hostname;
    let mut host: * mut i8 =
        malloc(strlen(tmp_hostname).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                     as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    sscanf(tmp_hostname, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char,
           host);
    if host.is_null() { return Option::<Box<crate::url_data>>::None }
    let mut host_len: i32 = strlen(host) as std::os::raw::c_int;
    (*(owned_as_mut(&mut data)).unwrap()).host = host;
    let mut tmp_path: * mut i8 = 0 as *mut std::os::raw::c_char;
    tmp_path =
        if is_ssh as std::os::raw::c_int != 0 {
            get_part(tmp_url, b":%s\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len + hostname_len)
        } else {
            get_part(tmp_url, b"/%s\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len + hostname_len)
        };
    let mut path: * mut i8 =
        malloc(strlen(tmp_path).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                 as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if path.is_null() { return Option::<Box<crate::url_data>>::None }
    let mut fmt: * mut i8 =
        if is_ssh as std::os::raw::c_int != 0 {
            b"%s\x00" as *const u8 as *const std::os::raw::c_char
        } else { b"/%s\x00" as *const u8 as *const std::os::raw::c_char } as
            *mut std::os::raw::c_char;
    sprintf(path, fmt, tmp_path);
    (*(owned_as_mut(&mut data)).unwrap()).path = path;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut pathname: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if pathname.is_null() { return Option::<Box<crate::url_data>>::None }
    strcat(pathname, b"\x00" as *const u8 as *const std::os::raw::c_char);
    tmp_path = strdup(path);
    sscanf(tmp_path, b"%[^? | ^#]\x00" as *const u8 as *const std::os::raw::c_char,
           pathname);
    let mut pathname_len: i32 = strlen(pathname) as std::os::raw::c_int;
    (*(owned_as_mut(&mut data)).unwrap()).pathname = pathname;
    let mut search: * mut i8 =
        malloc(::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if search.is_null() { return Option::<Box<crate::url_data>>::None }
    tmp_path = strff(tmp_path, pathname_len);
    strcat(search, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%[^#]\x00" as *const u8 as *const std::os::raw::c_char,
           search);
    (*(owned_as_mut(&mut data)).unwrap()).search = search;
    let mut search_len: i32 = strlen(search) as std::os::raw::c_int;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut query: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if query.is_null() { return Option::<Box<crate::url_data>>::None }
    sscanf(search, b"?%s\x00" as *const u8 as *const std::os::raw::c_char, query);
    (*(owned_as_mut(&mut data)).unwrap()).query = query;
    let mut hash: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if hash.is_null() { return Option::<Box<crate::url_data>>::None }
    tmp_path = strff(path, pathname_len + search_len);
    strcat(hash, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%s\x00" as *const u8 as *const std::os::raw::c_char, hash);
    (*(owned_as_mut(&mut data)).unwrap()).hash = hash;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut port: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if port.is_null() { return Option::<Box<crate::url_data>>::None }
    tmp_hostname = strff(hostname, host_len + 1 as std::os::raw::c_int);
    sscanf(tmp_hostname, b"%s\x00" as *const u8 as *const std::os::raw::c_char, port);
    (*(owned_as_mut(&mut data)).unwrap()).port = port;
    free(tmp_hostname as *mut std::os::raw::c_void);
    return data;
}
unsafe extern "C" fn strff(mut ptr: * mut std::os::raw::c_char, mut n: std::os::raw::c_int)
 -> * mut std::os::raw::c_char {
    let mut y: i32 = 0 as std::os::raw::c_int;
    let mut i: i32 = 0 as std::os::raw::c_int;
    while i < n {
        let mut fresh0 = ptr;
        ptr = ptr.offset(1);
        y = *fresh0 as std::os::raw::c_int;
        i += 1
    }
    return strdup(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn url_get_protocol(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut protocol: * mut i8 =
        malloc((16 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                    as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if protocol.is_null() { return 0 as *mut std::os::raw::c_char }
    sscanf(url, b"%[^://]\x00" as *const u8 as *const std::os::raw::c_char, protocol);
    if url_is_protocol(protocol) { return protocol }
    return 0 as *mut std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn url_is_protocol(mut str: * mut std::os::raw::c_char) -> bool {
    let mut count: i32 =
        (::std::mem::size_of::<[*mut std::os::raw::c_char; 177]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*mut std::os::raw::c_char>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut i: i32 = 0 as std::os::raw::c_int;
    while i < count {
        if 0 as std::os::raw::c_int == strcmp(URL_SCHEMES[i as usize], str) {
            return 1 as std::os::raw::c_int != 0
        }
        i += 1
    }
    return 0 as std::os::raw::c_int != 0;
}
unsafe extern "C" fn get_part(mut url: * mut std::os::raw::c_char,
                              mut format: * const std::os::raw::c_char,
                              mut l: std::os::raw::c_int) -> * mut std::os::raw::c_char {
    let mut has: bool = 0 as std::os::raw::c_int != 0;
    let mut tmp: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut tmp_url: * mut i8 = strdup(url);
    let mut fmt_url: * mut i8 = strdup(url);
    let mut ret: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() ||
           ret.is_null() {
        return 0 as *mut std::os::raw::c_char
    }
    strcpy(tmp, b"\x00" as *const u8 as *const std::os::raw::c_char);
    strcpy(fmt_url, b"\x00" as *const u8 as *const std::os::raw::c_char);
    fmt_url = strff(fmt_url, l);
    sscanf(fmt_url, format, tmp);
    if 0 as std::os::raw::c_int != strcmp(tmp, tmp_url) {
        has = 1 as std::os::raw::c_int != 0;
        ret = strdup(tmp)
    }
    fmt_url = strrwd(fmt_url, l);
    free(tmp as *mut std::os::raw::c_void);
    free(tmp_url as *mut std::os::raw::c_void);
    free(fmt_url as *mut std::os::raw::c_void);
    return if has as std::os::raw::c_int != 0 { ret } else { 0 as *mut std::os::raw::c_char };
}
unsafe extern "C" fn strrwd(mut ptr: * mut std::os::raw::c_char, mut n: std::os::raw::c_int)
 -> * mut std::os::raw::c_char {
    let mut y: i32 = 0 as std::os::raw::c_int;
    let mut i: i32 = 0 as std::os::raw::c_int;
    while i < n {
        let mut fresh1 = ptr;
        ptr = ptr.offset(-1);
        y = *fresh1 as std::os::raw::c_int;
        i += 1
    }
    return strdup(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn url_is_ssh(mut str: * mut std::os::raw::c_char) -> bool {
    str = strdup(str);
    if 0 as std::os::raw::c_int ==
           strcmp(str, b"ssh\x00" as *const u8 as *const std::os::raw::c_char) ||
           0 as std::os::raw::c_int ==
               strcmp(str, b"git\x00" as *const u8 as *const std::os::raw::c_char) {
        free(str as *mut std::os::raw::c_void);
        return 1 as std::os::raw::c_int != 0
    }
    return 0 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_auth(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut protocol: * mut i8 = url_get_protocol(url);
    if protocol.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut l: i32 =
        strlen(protocol) as std::os::raw::c_int + 3 as std::os::raw::c_int;
    return get_part(url, b"%[^@]\x00" as *const u8 as *const std::os::raw::c_char, l);
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hostname(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut l: i32 = 3 as std::os::raw::c_int;
    let mut protocol: * mut i8 = url_get_protocol(url);
    let mut tmp_protocol: * mut i8 = strdup(protocol);
    let mut auth: * mut i8 = url_get_auth(url);
    if protocol.is_null() { return 0 as *mut std::os::raw::c_char }
    if !auth.is_null() {
        l =
            (l as
                 std::os::raw::c_ulong).wrapping_add(strlen(auth).wrapping_add(1 as
                                                                           std::os::raw::c_int
                                                                           as
                                                                           std::os::raw::c_ulong))
                as std::os::raw::c_int as std::os::raw::c_int
    }
    if !auth.is_null() { free(auth as *mut std::os::raw::c_void); }
    l += strlen(protocol) as std::os::raw::c_int;
    free(protocol as *mut std::os::raw::c_void);
    let mut hostname: * mut i8 =
        if url_is_ssh(tmp_protocol) as std::os::raw::c_int != 0 {
            get_part(url, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char, l)
        } else {
            get_part(url, b"%[^/]\x00" as *const u8 as *const std::os::raw::c_char, l)
        };
    free(tmp_protocol as *mut std::os::raw::c_void);
    return hostname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_host(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut host: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut hostname: * mut i8 = url_get_hostname(url);
    if host.is_null() || hostname.is_null() { return 0 as *mut std::os::raw::c_char }
    sscanf(hostname, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char, host);
    free(hostname as *mut std::os::raw::c_void);
    return host;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_pathname(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut path: * mut i8 = url_get_path(url);
    let mut pathname: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if path.is_null() || pathname.is_null() { return 0 as *mut std::os::raw::c_char }
    strcat(pathname, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(path, b"%[^?]\x00" as *const u8 as *const std::os::raw::c_char, pathname);
    free(path as *mut std::os::raw::c_void);
    return pathname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_path(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut l: i32 = 3 as std::os::raw::c_int;
    let mut tmp_path: * mut i8 = 0 as *mut std::os::raw::c_char;
    let mut protocol: * mut i8 = url_get_protocol(url);
    let mut auth: * mut i8 = url_get_auth(url);
    let mut hostname: * mut i8 = url_get_hostname(url);
    if protocol.is_null() || hostname.is_null() {
        return 0 as *mut std::os::raw::c_char
    }
    let mut is_ssh: bool = url_is_ssh(protocol);
    l += strlen(protocol) as std::os::raw::c_int + strlen(hostname) as std::os::raw::c_int;
    if !auth.is_null() { l += strlen(auth) as std::os::raw::c_int + 1 as std::os::raw::c_int }
    tmp_path =
        if is_ssh as std::os::raw::c_int != 0 {
            get_part(url, b":%s\x00" as *const u8 as *const std::os::raw::c_char, l)
        } else {
            get_part(url, b"/%s\x00" as *const u8 as *const std::os::raw::c_char, l)
        };
    let mut fmt: * mut i8 =
        if is_ssh as std::os::raw::c_int != 0 {
            b"%s\x00" as *const u8 as *const std::os::raw::c_char
        } else { b"/%s\x00" as *const u8 as *const std::os::raw::c_char } as
            *mut std::os::raw::c_char;
    let mut path: * mut i8 =
        malloc(strlen(tmp_path).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                 as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    sprintf(path, fmt, tmp_path);
    if !auth.is_null() { free(auth as *mut std::os::raw::c_void); }
    free(protocol as *mut std::os::raw::c_void);
    free(hostname as *mut std::os::raw::c_void);
    free(tmp_path as *mut std::os::raw::c_void);
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_search(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut path: * mut i8 = url_get_path(url);
    let mut pathname: * mut i8 = url_get_pathname(url);
    let mut search: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if path.is_null() || search.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut tmp_path: * mut i8 =
        strff(path, strlen(pathname) as std::os::raw::c_int);
    strcat(search, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%[^#]\x00" as *const u8 as *const std::os::raw::c_char,
           search);
    tmp_path = strrwd(tmp_path, strlen(pathname) as std::os::raw::c_int);
    free(path as *mut std::os::raw::c_void);
    free(pathname as *mut std::os::raw::c_void);
    return search;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_query(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut search: * mut i8 = url_get_search(url);
    let mut query: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if search.is_null() { return 0 as *mut std::os::raw::c_char }
    sscanf(search, b"?%s\x00" as *const u8 as *const std::os::raw::c_char, query);
    free(search as *mut std::os::raw::c_void);
    return query;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hash(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut hash: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if hash.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut path: * mut i8 = url_get_path(url);
    if path.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut pathname: * mut i8 = url_get_pathname(url);
    if pathname.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut search: * mut i8 = url_get_search(url);
    let mut pathname_len: i32 = strlen(pathname) as std::os::raw::c_int;
    let mut search_len: i32 = strlen(search) as std::os::raw::c_int;
    let mut tmp_path: * mut i8 =
        strff(path, pathname_len + search_len);
    strcat(hash, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%s\x00" as *const u8 as *const std::os::raw::c_char, hash);
    tmp_path = strrwd(tmp_path, pathname_len + search_len);
    free(tmp_path as *mut std::os::raw::c_void);
    free(pathname as *mut std::os::raw::c_void);
    free(path as *mut std::os::raw::c_void);
    if !search.is_null() { free(search as *mut std::os::raw::c_void); }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_port(mut url: * mut std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut port: * mut i8 =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut hostname: * mut i8 = url_get_hostname(url);
    let mut host: * mut i8 = url_get_host(url);
    if port.is_null() || hostname.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut tmp_hostname: * mut i8 =
        strff(hostname,
              strlen(host).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
                  std::os::raw::c_int);
    sscanf(tmp_hostname, b"%s\x00" as *const u8 as *const std::os::raw::c_char, port);
    free(hostname as *mut std::os::raw::c_void);
    free(tmp_hostname as *mut std::os::raw::c_void);
    return port;
}
#[no_mangle]
pub unsafe extern "C" fn url_free<'a1>(mut data: Option<&'a1 mut crate::url_data>) {
    if borrow(& data).is_none() { return }
    if !(*(borrow_mut(&mut data)).unwrap()).auth.is_null() { free((*(borrow_mut(&mut data)).unwrap()).auth as *mut std::os::raw::c_void); }
    if !(*(borrow_mut(&mut data)).unwrap()).protocol.is_null() {
        free((*(borrow_mut(&mut data)).unwrap()).protocol as *mut std::os::raw::c_void);
    }
    if !(*(borrow_mut(&mut data)).unwrap()).hostname.is_null() {
        free((*(borrow_mut(&mut data)).unwrap()).hostname as *mut std::os::raw::c_void);
    }
    if !(*(borrow_mut(&mut data)).unwrap()).host.is_null() { free((*(borrow_mut(&mut data)).unwrap()).host as *mut std::os::raw::c_void); }
    if !(*(borrow_mut(&mut data)).unwrap()).pathname.is_null() {
        free((*(borrow_mut(&mut data)).unwrap()).pathname as *mut std::os::raw::c_void);
    }
    if !(*(borrow_mut(&mut data)).unwrap()).path.is_null() { free((*(borrow_mut(&mut data)).unwrap()).path as *mut std::os::raw::c_void); }
    if !(*(borrow_mut(&mut data)).unwrap()).hash.is_null() { free((*(borrow_mut(&mut data)).unwrap()).hash as *mut std::os::raw::c_void); }
    if !(*(borrow_mut(&mut data)).unwrap()).search.is_null() {
        free((*(borrow_mut(&mut data)).unwrap()).search as *mut std::os::raw::c_void);
    }
    if !(*(borrow_mut(&mut data)).unwrap()).query.is_null() { free((*(borrow_mut(&mut data)).unwrap()).query as *mut std::os::raw::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn url_inspect(mut url: * mut std::os::raw::c_char) {
    url_data_inspect(owned_as_mut(&mut url_parse(url)));
}
#[no_mangle]
pub unsafe extern "C" fn url_data_inspect<'a1>(mut data: Option<&'a1 mut crate::url_data>) {
    printf(b"#url =>\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"    .href: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).href);
    printf(b"    .protocol: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).protocol);
    printf(b"    .host: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).host);
    printf(b"    .auth: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).auth);
    printf(b"    .hostname: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).hostname);
    printf(b"    .pathname: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).pathname);
    printf(b"    .search: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).search);
    printf(b"    .path: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).path);
    printf(b"    .hash: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).hash);
    printf(b"    .query: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).query);
    printf(b"    .port: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*(borrow(& data)).unwrap()).port);
}
unsafe fn main_0() -> std::os::raw::c_int {
    //url_inspect("https://google.com/search?q=github");
    let mut gh_url: * mut i8 =
        b"git://git@github.com:jwerle/url.h.git\x00" as *const u8 as
            *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut url: * mut i8 =
        b"http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash\x00"
            as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut parsed: Option<Box<crate::url_data>> = url_parse(url);
    let mut gh_parsed: Option<Box<crate::url_data>> = url_parse(gh_url);
    if owned_as_ref(& parsed).is_none() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     15 as std::os::raw::c_int,
                     b"parsed\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if owned_as_ref(& gh_parsed).is_none() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     16 as std::os::raw::c_int,
                     b"gh_parsed\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    url_data_inspect(owned_as_mut(&mut parsed));
    url_data_inspect(owned_as_mut(&mut gh_parsed));
    if (*(owned_as_mut(&mut parsed)).unwrap()).href.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     21 as std::os::raw::c_int,
                     b"parsed->href\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).auth.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     22 as std::os::raw::c_int,
                     b"parsed->auth\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).protocol.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     23 as std::os::raw::c_int,
                     b"parsed->protocol\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).port.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     24 as std::os::raw::c_int,
                     b"parsed->port\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).hostname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     25 as std::os::raw::c_int,
                     b"parsed->hostname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).host.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     26 as std::os::raw::c_int,
                     b"parsed->host\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).pathname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     27 as std::os::raw::c_int,
                     b"parsed->pathname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).path.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     28 as std::os::raw::c_int,
                     b"parsed->path\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).hash.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     29 as std::os::raw::c_int,
                     b"parsed->hash\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).search.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     30 as std::os::raw::c_int,
                     b"parsed->search\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut parsed)).unwrap()).query.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     31 as std::os::raw::c_int,
                     b"parsed->query\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut gh_parsed)).unwrap()).href.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     33 as std::os::raw::c_int,
                     b"gh_parsed->href\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut gh_parsed)).unwrap()).protocol.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     34 as std::os::raw::c_int,
                     b"gh_parsed->protocol\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut gh_parsed)).unwrap()).host.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     35 as std::os::raw::c_int,
                     b"gh_parsed->host\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut gh_parsed)).unwrap()).auth.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     36 as std::os::raw::c_int,
                     b"gh_parsed->auth\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut gh_parsed)).unwrap()).hostname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     37 as std::os::raw::c_int,
                     b"gh_parsed->hostname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut gh_parsed)).unwrap()).pathname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     38 as std::os::raw::c_int,
                     b"gh_parsed->pathname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*(owned_as_mut(&mut gh_parsed)).unwrap()).path.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     39 as std::os::raw::c_int,
                     b"gh_parsed->path\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"http\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     41 as std::os::raw::c_int,
                     b"url_is_protocol(\"http\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"https\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     42 as std::os::raw::c_int,
                     b"url_is_protocol(\"https\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"git\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     43 as std::os::raw::c_int,
                     b"url_is_protocol(\"git\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"ssh\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     44 as std::os::raw::c_int,
                     b"url_is_protocol(\"ssh\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"sftp\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     45 as std::os::raw::c_int,
                     b"url_is_protocol(\"sftp\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"ftp\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     46 as std::os::raw::c_int,
                     b"url_is_protocol(\"ftp\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"javascript\x00" as *const u8 as *const std::os::raw::c_char
                            as *mut std::os::raw::c_char) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     47 as std::os::raw::c_int,
                     b"url_is_protocol(\"javascript\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"http\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_protocol(url))) as std::os::raw::c_int as std::os::raw::c_long !=
           0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     49 as std::os::raw::c_int,
                     b"0 == strcmp(\"http\", url_get_protocol(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"user:pass\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_auth(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     50 as std::os::raw::c_int,
                     b"0 == strcmp(\"user:pass\", url_get_auth(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"subdomain.host.com:8080\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_hostname(url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     51 as std::os::raw::c_int,
                     b"0 == strcmp(\"subdomain.host.com:8080\", url_get_hostname(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"subdomain.host.com\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_host(url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     52 as std::os::raw::c_int,
                     b"0 == strcmp(\"subdomain.host.com\", url_get_host(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"/p/a/t/h\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_pathname(url))) as std::os::raw::c_int as std::os::raw::c_long !=
           0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     53 as std::os::raw::c_int,
                     b"0 == strcmp(\"/p/a/t/h\", url_get_pathname(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"/p/a/t/h?query=string#hash\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_path(url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     54 as std::os::raw::c_int,
                     b"0 == strcmp(\"/p/a/t/h?query=string#hash\", url_get_path(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"?query=string\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_search(url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     55 as std::os::raw::c_int,
                     b"0 == strcmp(\"?query=string\", url_get_search(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"query=string\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_query(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     56 as std::os::raw::c_int,
                     b"0 == strcmp(\"query=string\", url_get_query(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"#hash\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_hash(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     57 as std::os::raw::c_int,
                     b"0 == strcmp(\"#hash\", url_get_hash(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"8080\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_port(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     58 as std::os::raw::c_int,
                     b"0 == strcmp(\"8080\", url_get_port(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"git\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_protocol(gh_url))) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     60 as std::os::raw::c_int,
                     b"0 == strcmp(\"git\", url_get_protocol(gh_url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"github.com\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_host(gh_url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     61 as std::os::raw::c_int,
                     b"0 == strcmp(\"github.com\", url_get_host(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"github.com\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_hostname(gh_url))) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     62 as std::os::raw::c_int,
                     b"0 == strcmp(\"github.com\", url_get_hostname(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"git\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_auth(gh_url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     63 as std::os::raw::c_int,
                     b"0 == strcmp(\"git\", url_get_auth(gh_url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"jwerle/url.h.git\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_pathname(gh_url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     64 as std::os::raw::c_int,
                     b"0 == strcmp(\"jwerle/url.h.git\", url_get_pathname(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"jwerle/url.h.git\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_path(gh_url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     65 as std::os::raw::c_int,
                     b"0 == strcmp(\"jwerle/url.h.git\", url_get_path(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    url_free(owned_as_mut(&mut parsed));
    return 0 as std::os::raw::c_int;
}
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
use crate::laertes_rt::*;// This module is only injected into the rewritten programs, it is not used
// internally. So, disable unused function warnings.



}

// root glob re-export: the translation addresses its own items as crate::<item>
pub use crate::test::*;
// root re-exports so the generated harness's `translated::<entry>` resolves
// get_part: private in test — exposed per-entry by --expose-entry
pub use crate::test::main;
// main_0: private in test — exposed per-entry by --expose-entry
pub use crate::test::strdup;
// strff: private in test — exposed per-entry by --expose-entry
// strrwd: private in test — exposed per-entry by --expose-entry
pub use crate::test::url_data_inspect;
pub use crate::test::url_free;
pub use crate::test::url_get_auth;
pub use crate::test::url_get_hash;
pub use crate::test::url_get_host;
pub use crate::test::url_get_hostname;
pub use crate::test::url_get_path;
pub use crate::test::url_get_pathname;
pub use crate::test::url_get_port;
pub use crate::test::url_get_protocol;
pub use crate::test::url_get_query;
pub use crate::test::url_get_search;
pub use crate::test::url_inspect;
pub use crate::test::url_is_protocol;
pub use crate::test::url_is_ssh;
pub use crate::test::url_parse;
// support modules carried verbatim from the translation's own crate root
pub(crate) mod __laertes_array {
    use std::{
        cell::{Ref, RefCell, RefMut},
        cmp::Ordering,
        convert::{TryFrom, TryInto},
        marker::PhantomData,
    };

    pub trait Get<'g, T: 'g> {
        fn get(&'g self, index: SliceIndex) -> T;

        fn get_add(&'g self, index: usize) -> T {
            self.get(SliceIndex::Add(index))
        }

        fn get_sub(&'g self, index: usize) -> T {
            self.get(SliceIndex::Sub(index))
        }

        fn get_offset(&'g self, index: isize) -> T {
            self.get(SliceIndex::Offset(index))
        }
    }

    pub trait GetMut<'g, T: 'g> {
        fn get_mut(&'g mut self, index: SliceIndex) -> T;

        fn get_add_mut(&'g mut self, index: usize) -> T {
            self.get_mut(SliceIndex::Add(index))
        }

        fn get_sub_mut(&'g mut self, index: usize) -> T {
            self.get_mut(SliceIndex::Sub(index))
        }

        fn get_offset_mut(&'g mut self, index: isize) -> T {
            self.get_mut(SliceIndex::Offset(index))
        }
    }

    pub trait IntoSlice<S>: Sized {
        fn into_slice(self, index: SliceIndex) -> S;

        fn into_slice_add(self, index: usize) -> S {
            self.into_slice(SliceIndex::Add(index))
        }

        fn into_slice_sub(self, index: usize) -> S {
            self.into_slice(SliceIndex::Sub(index))
        }

        fn into_slice_offset(self, index: isize) -> S {
            self.into_slice(SliceIndex::Offset(index))
        }
    }

    pub trait Slice<'s, S: 's> {
        fn slice(&'s self, index: SliceIndex) -> S;

        fn slice_add(&'s self, index: usize) -> S {
            self.slice(SliceIndex::Add(index))
        }

        fn slice_sub(&'s self, index: usize) -> S {
            self.slice(SliceIndex::Sub(index))
        }

        fn slice_offset(&'s self, index: isize) -> S {
            self.slice(SliceIndex::Offset(index))
        }
    }

    pub trait SliceMut<'s, S: 's> {
        fn slice_mut(&'s mut self, index: SliceIndex) -> S;

        fn slice_add_mut(&'s mut self, index: usize) -> S {
            self.slice_mut(SliceIndex::Add(index))
        }

        fn slice_sub_mut(&'s mut self, index: usize) -> S {
            self.slice_mut(SliceIndex::Sub(index))
        }

        fn slice_offset_mut(&'s mut self, index: isize) -> S {
            self.slice_mut(SliceIndex::Offset(index))
        }
    }

    #[derive(Clone, Copy)]
    pub enum SliceIndex {
        Add(usize),
        Sub(usize),
        Offset(isize),
    }

    impl SliceIndex {
        fn apply(self, offset: usize) -> usize {
            match self {
                SliceIndex::Add(i) => offset.checked_add(i),
                SliceIndex::Sub(i) => offset.checked_sub(i),
                SliceIndex::Offset(i) => {
                    if i < 0 {
                        offset.checked_sub(i.wrapping_abs() as usize)
                    } else {
                        offset.checked_add(i as usize)
                    }
                },
            }
            .unwrap()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct CustomSlice<'a, T: 'a, S: 'a> {
        offset: usize,
        slice: S,
        m: PhantomData<&'a T>,
    }

    impl<'a, T: 'a, S: 'a> CustomSlice<'a, T, S> {
        pub const fn new(slice: S) -> CustomSlice<'a, T, S> {
            CustomSlice {
                offset: 0,
                slice,
                m: PhantomData,
            }
        }
    }

    impl<'a, T: 'a> CustomSlice<'a, T, Box<[T]>> {
        pub fn boxed_from_vec(vec: Vec<T>) -> CustomSlice<'a, T, Box<[T]>> {
            CustomSlice {
                offset: 0,
                slice: vec.into_boxed_slice(),
                m: PhantomData::default(),
            }
        }
    }

    impl<'a, T: 'a, S: TryFrom<Vec<RefCell<T>>> + 'a> CustomSlice<'a, T, S> {
        pub fn from_vec(vec: Vec<T>) -> CustomSlice<'a, T, S> {
            CustomSlice::new(
                vec.into_iter()
                    .map(|v| RefCell::new(v))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!("couldn't construct backing slice")),
            )
        }
    }

    impl<'a, T, S> IntoSlice<CustomSlice<'a, T, S>> for CustomSlice<'a, T, S> {
        fn into_slice(self, index: SliceIndex) -> CustomSlice<'a, T, S> {
            let CustomSlice { offset, slice, m } = self;

            CustomSlice {
                offset: index.apply(offset),
                slice,
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsRef<[T]> + 's> Slice<'s, CustomSlice<'s, T, &'s [T]>>
        for CustomSlice<'a, T, S>
    {
        fn slice(&'s self, index: SliceIndex) -> CustomSlice<'s, T, &'s [T]> {
            let CustomSlice {
                offset,
                ref slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_ref(),
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsRef<[RefCell<T>]> + 's> Slice<'s, CustomSlice<'s, T, &'s [RefCell<T>]>>
        for CustomSlice<'a, T, S>
    {
        fn slice(&'s self, index: SliceIndex) -> CustomSlice<'s, T, &'s [RefCell<T>]> {
            let CustomSlice {
                offset,
                ref slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_ref(),
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsMut<[T]> + 's> SliceMut<'s, CustomSlice<'s, T, &'s mut [T]>>
        for CustomSlice<'a, T, S>
    {
        fn slice_mut(&'s mut self, index: SliceIndex) -> CustomSlice<'s, T, &'s mut [T]> {
            let CustomSlice {
                offset,
                ref mut slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_mut(),
                m,
            }
        }
    }

    impl<'a, 'g, T, S: AsRef<[T]> + 'g> Get<'g, &'g T> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> &'g T {
            &self.slice.as_ref()[index.apply(self.offset)]
        }
    }

    impl<'a, 'g, T, S: AsMut<[T]> + 'g> GetMut<'g, &'g mut T> for CustomSlice<'a, T, S> {
        fn get_mut(&'g mut self, index: SliceIndex) -> &'g mut T {
            &mut self.slice.as_mut()[index.apply(self.offset)]
        }
    }

    impl<'a, 'g, T, S: AsRef<[RefCell<T>]> + 'g> Get<'g, Ref<'g, T>> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> Ref<'g, T> {
            self.slice.as_ref()[index.apply(self.offset)].borrow()
        }
    }

    impl<'a, 'g, T, S: AsRef<[RefCell<T>]> + 'g> Get<'g, RefMut<'g, T>> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> RefMut<'g, T> {
            self.slice.as_ref()[index.apply(self.offset)].borrow_mut()
        }
    }

    pub trait GetPtr<T: ?Sized> {
        fn get_ptr(&self) -> *const ();
    }

    impl<T: ?Sized> GetPtr<T> for &T {
        fn get_ptr(&self) -> *const () {
            *self as *const T as *const ()
        }
    }

    impl<T: ?Sized> GetPtr<T> for &mut T {
        fn get_ptr(&self) -> *const () {
            *self as *const T as *const ()
        }
    }

    impl<T> GetPtr<T> for &[T] {
        fn get_ptr(&self) -> *const () {
            self.as_ptr() as *const ()
        }
    }

    impl<T> GetPtr<T> for &mut [T] {
        fn get_ptr(&self) -> *const () {
            self.as_ptr() as *const ()
        }
    }

    impl<T> GetPtr<T> for Box<[T]> {
        fn get_ptr(&self) -> *const () {
            (*self).as_ptr() as *const ()
        }
    }

    impl<T: ?Sized> GetPtr<T> for Box<T> {
        fn get_ptr(&self) -> *const () {
            self.as_ref() as *const T as *const ()
        }
    }

    pub fn offset_from<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> isize {
        if l.is_none() && r.is_none() {
            return 0; // case for NULL - NULL
        }

        l.as_ref()
            .and_then(|l| {
                r.as_ref().map(|r| {
                    if l.slice.get_ptr() == r.slice.get_ptr() {
                        l.offset as isize - r.offset as isize
                    } else {
                        panic!("trying to compute offsets from two unrelated slices");
                    }
                })
            })
            .expect("tried to compare non-null and null pointers")
    }

    pub fn partial_cmp<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<Ordering> {
        l.as_ref().and_then(|l| {
            r.as_ref().and_then(|r| {
                if l.slice.get_ptr() == r.slice.get_ptr() {
                    Some(l.offset.cmp(&r.offset))
                } else {
                    None
                }
            })
        })
    }

    pub fn partial_lt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c == Ordering::Less)
    }
    pub fn partial_gt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c == Ordering::Greater)
    }
    pub fn partial_le<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c != Ordering::Greater)
    }
    pub fn partial_ge<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c != Ordering::Less)
    }

    pub fn compare_eq<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_cmp(l, r).map_or(false, |c| c == Ordering::Equal)
    }
    pub fn compare_ne<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        !compare_eq(l, r)
    }

    pub fn compare_lt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_lt(l, r).unwrap()
    }
    pub fn compare_gt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_gt(l, r).unwrap()
    }
    pub fn compare_le<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_le(l, r).unwrap()
    }
    pub fn compare_ge<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_ge(l, r).unwrap()
    }

    pub fn borrow<'a, 'b: 'a, S: Slice<'a, T>, T: 'a>(s: &'b Option<S>) -> Option<T> {
        s.as_ref().map(|s| s.slice_add(0))
    }
    pub fn borrow_mut<'a, 'b: 'a, S: SliceMut<'a, T>, T: 'a>(s: &'b mut Option<S>) -> Option<T> {
        s.as_mut().map(|s| s.slice_add_mut(0))
    }
}
pub(crate) mod laertes_rt {
    use super::__laertes_array::GetPtr;

    pub trait Borrow<'b, T: ?Sized> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a;
    }

    pub trait BorrowMut<'b, T: ?Sized> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a;
    }

    impl<'b, T: ?Sized> Borrow<'b, T> for Option<&'b mut T> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a,
        {
            self.as_ref().map(|x| &**x)
        }
    }

    impl<'b, T: ?Sized> BorrowMut<'b, T> for Option<&'b mut T> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a,
        {
            self.as_mut().map(|x| &mut **x)
        }
    }

    impl<'b, T: ?Sized> Borrow<'b, T> for Option<Box<T>> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a,
        {
            self.as_ref().map(|x| x.as_ref())
        }
    }

    impl<'b, T: ?Sized> BorrowMut<'b, T> for Option<Box<T>> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a,
        {
            self.as_mut().map(|x| x.as_mut())
        }
    }

    pub fn borrow<'a, 'b: 'a, T: ?Sized, P: Borrow<'b, T>>(p: &'a P) -> Option<&'a T> {
        Borrow::borrow(p)
    }

    pub fn borrow_mut<'a, 'b: 'a, T: ?Sized, P: BorrowMut<'b, T>>(
        p: &'a mut P,
    ) -> Option<&'a mut T> {
        BorrowMut::borrow_mut(p)
    }

    pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
        p.as_ref().map(|x| x.as_ref())
    }

    pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
        p.as_mut().map(|x| x.as_mut())
    }

    pub fn option_to_raw<T: ?Sized, P: GetPtr<T>>(p: &Option<P>) -> *const () {
        p.as_ref()
            .map_or(core::ptr::null(), |p| p.get_ptr() as *const ())
    }

    pub fn _ref_eq<T: ?Sized, L: GetPtr<T>, R: GetPtr<T>>(p: Option<L>, q: Option<R>) -> bool {
        option_to_raw(&p) == option_to_raw(&q)
    }

    pub fn _ref_ne<T: ?Sized, L: GetPtr<T>, R: GetPtr<T>>(p: Option<L>, q: Option<R>) -> bool {
        option_to_raw(&p) != option_to_raw(&q)
    }

    // functions to convert references to raw pointers
    pub fn _as_ptr<T, P: GetPtr<T>>(r: &Option<P>) -> *const T {
        option_to_raw(r) as *const T
    }

    pub fn _as_mut_ptr<T, P: GetPtr<T>>(r: &mut Option<P>) -> *mut T {
        option_to_raw(r) as *const T as *mut T
    }

    pub fn _move_to_ptr<T>(b: Option<Box<T>>) -> *mut T {
        b.map_or(core::ptr::null_mut(), Box::into_raw)
    }

    pub fn trans<'a, 'b, T: 'a, U: 'b>(_: T) -> U
    where
        'a: 'b,
    {
        loop {}
    }
}


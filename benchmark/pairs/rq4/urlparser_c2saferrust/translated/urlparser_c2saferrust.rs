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













use std::ptr;

use std::ffi::CString;

use std::ffi::CStr;

extern "C" {
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn free(_: *mut std::os::raw::c_void);
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url_data {
    pub href: *mut std::os::raw::c_char,
    pub protocol: *mut std::os::raw::c_char,
    pub host: *mut std::os::raw::c_char,
    pub auth: *mut std::os::raw::c_char,
    pub hostname: *mut std::os::raw::c_char,
    pub pathname: *mut std::os::raw::c_char,
    pub search: *mut std::os::raw::c_char,
    pub path: *mut std::os::raw::c_char,
    pub hash: *mut std::os::raw::c_char,
    pub query: *mut std::os::raw::c_char,
    pub port: *mut std::os::raw::c_char,
}
pub type url_data_t = url_data;

pub unsafe extern "C" fn strdup(mut str: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut n: std::os::raw::c_int =
        strlen(str).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    let mut dup: *mut std::os::raw::c_char =
        malloc(n as std::os::raw::c_ulong) as *mut std::os::raw::c_char;
    if !dup.is_null() { strcpy(dup, str); }
    return dup;
}

pub static mut URL_SCHEMES: [*mut std::os::raw::c_char; 177] =
    [b"aaa\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
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
     b"doi\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char];

pub unsafe extern "C" fn url_parse(mut url: *mut std::os::raw::c_char)
 -> *mut url_data_t {
    let mut data: *mut url_data_t =
        malloc(::std::mem::size_of::<url_data_t>() as std::os::raw::c_ulong) as
            *mut url_data_t;
    if data.is_null() { return 0 as *mut url_data_t }
    (*data).href = url;
    let mut tmp: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut tmp_url: *mut std::os::raw::c_char = strdup(url);
    let mut is_ssh: bool = 0 as std::os::raw::c_int != 0;
    let mut protocol: *mut std::os::raw::c_char = url_get_protocol(tmp_url);
    if protocol.is_null() { return 0 as *mut url_data_t }
    let mut protocol_len: std::os::raw::c_int =
        strlen(protocol) as std::os::raw::c_int + 3 as std::os::raw::c_int;
    (*data).protocol = protocol;
    is_ssh = url_is_ssh(protocol);
    let mut auth: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut auth_len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    tmp = strstr(tmp_url, b"@\x00" as *const u8 as *const std::os::raw::c_char);
    if !tmp.is_null() {
        auth =
            get_part(tmp_url,
                     b"%[^@]\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len);
        auth_len = strlen(auth) as std::os::raw::c_int;
        if !auth.is_null() { auth_len += 1 }
    }
    (*data).auth = auth;
    let mut hostname: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
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
    if hostname.is_null() { return 0 as *mut url_data_t }
    let mut hostname_len: std::os::raw::c_int = strlen(hostname) as std::os::raw::c_int;
    let mut tmp_hostname: *mut std::os::raw::c_char = strdup(hostname);
    (*data).hostname = hostname;
    let mut host: *mut std::os::raw::c_char =
        malloc(strlen(tmp_hostname).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                     as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    sscanf(tmp_hostname, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char,
           host);
    if host.is_null() { return 0 as *mut url_data_t }
    let mut host_len: std::os::raw::c_int = strlen(host) as std::os::raw::c_int;
    (*data).host = host;
    let mut tmp_path: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    tmp_path =
        if is_ssh as std::os::raw::c_int != 0 {
            get_part(tmp_url, b":%s\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len + hostname_len)
        } else {
            get_part(tmp_url, b"/%s\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len + hostname_len)
        };
    let mut path: *mut std::os::raw::c_char =
        malloc(strlen(tmp_path).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                 as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if path.is_null() { return 0 as *mut url_data_t }
    let mut fmt: *mut std::os::raw::c_char =
        if is_ssh as std::os::raw::c_int != 0 {
            b"%s\x00" as *const u8 as *const std::os::raw::c_char
        } else { b"/%s\x00" as *const u8 as *const std::os::raw::c_char } as
            *mut std::os::raw::c_char;
    sprintf(path, fmt, tmp_path);
    (*data).path = path;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut pathname: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if pathname.is_null() { return 0 as *mut url_data_t }
    strcat(pathname, b"\x00" as *const u8 as *const std::os::raw::c_char);
    tmp_path = strdup(path);
    sscanf(tmp_path, b"%[^? | ^#]\x00" as *const u8 as *const std::os::raw::c_char,
           pathname);
    let mut pathname_len: std::os::raw::c_int = strlen(pathname) as std::os::raw::c_int;
    (*data).pathname = pathname;
    let mut search: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if search.is_null() { return 0 as *mut url_data_t }
    let mut tmp_path = strff(tmp_path, pathname_len);
    strcat(search, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%[^#]\x00" as *const u8 as *const std::os::raw::c_char,
           search);
    (*data).search = search;
    let mut search_len: std::os::raw::c_int = strlen(search) as std::os::raw::c_int;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut query: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if query.is_null() { return 0 as *mut url_data_t }
    sscanf(search, b"?%s\x00" as *const u8 as *const std::os::raw::c_char, query);
    (*data).query = query;
    let mut hash: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if hash.is_null() { return 0 as *mut url_data_t }
    let mut tmp_path = strff(path, pathname_len + search_len);
    strcat(hash, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%s\x00" as *const u8 as *const std::os::raw::c_char, hash);
    (*data).hash = hash;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut port: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if port.is_null() { return 0 as *mut url_data_t }
    let mut tmp_hostname = strff(hostname, host_len + 1);
    sscanf(tmp_hostname, b"%s\x00" as *const u8 as *const std::os::raw::c_char, port);
    (*data).port = port;
    free(tmp_hostname as *mut std::os::raw::c_void);
    return data;
}
fn strff(ptr: *mut std::os::raw::c_char, n: std::os::raw::c_int) -> *mut std::os::raw::c_char {
    let slice = unsafe { std::slice::from_raw_parts(ptr, n as usize) };
    let c_str = unsafe { CStr::from_ptr(slice.as_ptr()) };
    let duplicated = c_str.to_owned();
    duplicated.into_raw()
}


pub unsafe extern "C" fn url_get_protocol(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut protocol: *mut std::os::raw::c_char =
        malloc((16 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                    as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if protocol.is_null() { return 0 as *mut std::os::raw::c_char }
    sscanf(url, b"%[^://]\x00" as *const u8 as *const std::os::raw::c_char, protocol);
    if url_is_protocol(protocol) { return protocol }
    return 0 as *mut std::os::raw::c_char;
}

pub unsafe extern "C" fn url_is_protocol(mut str: *mut std::os::raw::c_char) -> bool {
    let mut count: std::os::raw::c_int =
        (::std::mem::size_of::<[*mut std::os::raw::c_char; 177]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*mut std::os::raw::c_char>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < count {
        if 0 as std::os::raw::c_int == strcmp(URL_SCHEMES[i as usize], str) {
            return 1 as std::os::raw::c_int != 0
        }
        i += 1
    }
    return 0 as std::os::raw::c_int != 0;
}
unsafe extern "C" fn get_part(mut url: *mut std::os::raw::c_char,
                              mut format: *const std::os::raw::c_char,
                              mut l: std::os::raw::c_int) -> *mut std::os::raw::c_char {
    let mut has: bool = 0 as std::os::raw::c_int != 0;
    let mut tmp: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut tmp_url: *mut std::os::raw::c_char = strdup(url);
    let mut fmt_url: *mut std::os::raw::c_char = strdup(url);
    let mut ret: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() ||
           ret.is_null() {
        return 0 as *mut std::os::raw::c_char
    }
    strcpy(tmp, b"\x00" as *const u8 as *const std::os::raw::c_char);
    strcpy(fmt_url, b"\x00" as *const u8 as *const std::os::raw::c_char);
    let mut fmt_url = strff(fmt_url, l);
    sscanf(fmt_url, format, tmp);
    if 0 as std::os::raw::c_int != strcmp(tmp, tmp_url) {
        has = 1 as std::os::raw::c_int != 0;
        ret = strdup(tmp)
    }
    let fmt_url_slice = std::slice::from_raw_parts_mut(fmt_url, l as usize);
fmt_url = strrwd(fmt_url_slice).as_mut_ptr();
    free(tmp as *mut std::os::raw::c_void);
    free(tmp_url as *mut std::os::raw::c_void);
    free(fmt_url as *mut std::os::raw::c_void);
    return if has as std::os::raw::c_int != 0 { ret } else { 0 as *mut std::os::raw::c_char };
}
fn strrwd(ptr: &mut [std::os::raw::c_char]) -> Vec<std::os::raw::c_char> {
    let mut reversed: Vec<std::os::raw::c_char> = Vec::with_capacity(ptr.len());
    for &byte in ptr.iter().rev() {
        reversed.push(byte);
    }
    reversed
}


pub fn url_is_ssh(str: *mut std::os::raw::c_char) -> bool {
    let c_str = unsafe { CStr::from_ptr(str) };
    let str_slice = c_str.to_str().unwrap();
    str_slice == "ssh" || str_slice == "git"
}


pub fn url_get_auth(url: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    unsafe {
        let protocol = url_get_protocol(url);
        if protocol.is_null() { return std::ptr::null_mut(); }
        
        let l = strlen(protocol) + 3;
        get_part(url, b"%[^@]\x00" as *const u8 as *const std::os::raw::c_char, l as std::os::raw::c_int)
    }
}


pub fn url_get_hostname(url: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    unsafe {
        let mut l: i32 = 3;
        let protocol = url_get_protocol(url);
        let tmp_protocol = CStr::from_ptr(protocol).to_owned();
        let auth = url_get_auth(url);
        
        if protocol.is_null() {
            return std::ptr::null_mut();
        }
        
        if !auth.is_null() {
            l += CStr::from_ptr(auth).to_bytes().len() as i32 + 1;
        }
        
        if !auth.is_null() {
            free(auth as *mut std::os::raw::c_void);
        }
        
        l += CStr::from_ptr(protocol).to_bytes().len() as i32;
        free(protocol as *mut std::os::raw::c_void);
        
        let hostname = if url_is_ssh(tmp_protocol.as_ptr() as *mut std::os::raw::c_char) {
            get_part(url, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char, l)
        } else {
            get_part(url, b"%[^/]\x00" as *const u8 as *const std::os::raw::c_char, l)
        };
        
        free(tmp_protocol.as_ptr() as *mut std::os::raw::c_void);
        hostname
    }
}


pub fn url_get_host(url: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let hostname = url_get_hostname(url);
    if hostname.is_null() {
        return std::ptr::null_mut();
    }
    
    let host_str = unsafe { CStr::from_ptr(hostname).to_str().ok() }
        .and_then(|s| s.split(':').next())
        .and_then(|s| CString::new(s).ok());
    
    let host = match host_str {
        Some(h) => h.into_raw(),
        None => {
            unsafe { free(hostname as *mut std::os::raw::c_void) };
            return std::ptr::null_mut();
        }
    };
    
    unsafe { free(hostname as *mut std::os::raw::c_void) };
    host
}


pub unsafe extern "C" fn url_get_pathname(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut path: *mut std::os::raw::c_char = url_get_path(url);
    let mut pathname: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if path.is_null() || pathname.is_null() { return 0 as *mut std::os::raw::c_char }
    strcat(pathname, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(path, b"%[^?]\x00" as *const u8 as *const std::os::raw::c_char, pathname);
    free(path as *mut std::os::raw::c_void);
    return pathname;
}

pub unsafe extern "C" fn url_get_path(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut l: std::os::raw::c_int = 3 as std::os::raw::c_int;
    let mut tmp_path: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut protocol: *mut std::os::raw::c_char = url_get_protocol(url);
    let auth = url_get_auth(url);
    let hostname: *mut std::os::raw::c_char = url_get_hostname(url);
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
    let mut fmt: *mut std::os::raw::c_char =
        if is_ssh as std::os::raw::c_int != 0 {
            b"%s\x00" as *const u8 as *const std::os::raw::c_char
        } else { b"/%s\x00" as *const u8 as *const std::os::raw::c_char } as
            *mut std::os::raw::c_char;
    let mut path: *mut std::os::raw::c_char =
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

pub fn url_get_search(url: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let path = unsafe { url_get_path(url) };
    let pathname = unsafe { url_get_pathname(url) };

    if path.is_null() || pathname.is_null() {
        return std::ptr::null_mut();
    }

    let pathname_len = unsafe { strlen(pathname) as usize };
    let tmp_path = unsafe { strff(path, pathname_len as i32) };
    let search = unsafe { malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as *mut std::os::raw::c_char };

    if search.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        *search = 0; // Initialize search to an empty string
        sscanf(tmp_path, b"%[^#]\x00" as *const u8 as *const std::os::raw::c_char, search);
        
        // Convert tmp_path to a mutable slice
        let tmp_path_slice = std::slice::from_raw_parts_mut(tmp_path as *mut std::os::raw::c_char, pathname_len);
        strrwd(tmp_path_slice); // Call strrwd with the mutable slice

        free(path as *mut std::os::raw::c_void);
        free(pathname as *mut std::os::raw::c_void);
    }

    search
}


pub fn url_get_query(url: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let search = url_get_search(url);
    if search.is_null() {
        return std::ptr::null_mut();
    }
    
    let c_str = unsafe { CStr::from_ptr(search) };
    let query = c_str.to_string_lossy().strip_prefix('?').unwrap_or("").to_string();
    
    unsafe {
        free(search as *mut std::os::raw::c_void);
    }
    
    let c_query = CString::new(query).unwrap();
    c_query.into_raw()
}


pub fn url_get_hash(url: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    unsafe {
        let hash: *mut std::os::raw::c_char =
            malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
                *mut std::os::raw::c_char;
        if hash.is_null() { return std::ptr::null_mut() }
        
        let path = url_get_path(url);
        if path.is_null() { return std::ptr::null_mut() }
        
        let pathname = url_get_pathname(url);
        if pathname.is_null() { return std::ptr::null_mut() }
        
        let search = url_get_search(url);
        let pathname_len = strlen(pathname) as std::os::raw::c_int;
        let search_len = if !search.is_null() { strlen(search) as std::os::raw::c_int } else { 0 };
        
        let tmp_path = strff(path, pathname_len + search_len);
        strcat(hash, b"\x00" as *const u8 as *const std::os::raw::c_char);
        sscanf(tmp_path, b"%s\x00" as *const u8 as *const std::os::raw::c_char, hash);
        
        free(tmp_path as *mut std::os::raw::c_void);
        free(pathname as *mut std::os::raw::c_void);
        free(path as *mut std::os::raw::c_void);
        
        if !search.is_null() { free(search as *mut std::os::raw::c_void); }
        
        return hash;
    }
}


pub fn url_get_port(url: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let hostname = url_get_hostname(url);
    let host = url_get_host(url);
    
    if hostname.is_null() || host.is_null() {
        return std::ptr::null_mut();
    }

    let hostname_str = unsafe { CStr::from_ptr(hostname).to_string_lossy().into_owned() };
    let host_str = unsafe { CStr::from_ptr(host).to_string_lossy().into_owned() };
    
    let tmp_hostname = format!("{}:{}", hostname_str, host_str);
    let port = tmp_hostname.split(':').last().unwrap_or("").to_string();

    let port_cstr = CString::new(port).unwrap();
    let port_ptr = port_cstr.into_raw();

    unsafe {
        free(hostname as *mut std::os::raw::c_void);
        free(host as *mut std::os::raw::c_void);
    }

    port_ptr
}


pub fn url_free(data: *mut url_data_t) {
    if data.is_null() { return; }
    
    unsafe {
        if !(*data).auth.is_null() {
            drop(Box::from_raw((*data).auth));
        }
        if !(*data).protocol.is_null() {
            drop(Box::from_raw((*data).protocol));
        }
        if !(*data).hostname.is_null() {
            drop(Box::from_raw((*data).hostname));
        }
        if !(*data).host.is_null() {
            drop(Box::from_raw((*data).host));
        }
        if !(*data).pathname.is_null() {
            drop(Box::from_raw((*data).pathname));
        }
        if !(*data).path.is_null() {
            drop(Box::from_raw((*data).path));
        }
        if !(*data).hash.is_null() {
            drop(Box::from_raw((*data).hash));
        }
        if !(*data).search.is_null() {
            drop(Box::from_raw((*data).search));
        }
        if !(*data).query.is_null() {
            drop(Box::from_raw((*data).query));
        }
    }
}


pub fn url_inspect(url: &str) {
    let c_string = CString::new(url).expect("CString::new failed");
    let url_data_ptr = unsafe { url_parse(c_string.as_ptr() as *mut std::os::raw::c_char) };
    let url_data = unsafe { &*url_data_ptr };
    url_data_inspect(url_data);
}


pub fn url_data_inspect(data: &url_data_t) {
    println!("#url =>");
    println!("    .href: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.href).to_string_lossy() });
    println!("    .protocol: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.protocol).to_string_lossy() });
    println!("    .host: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.host).to_string_lossy() });
    println!("    .auth: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.auth).to_string_lossy() });
    println!("    .hostname: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.hostname).to_string_lossy() });
    println!("    .pathname: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.pathname).to_string_lossy() });
    println!("    .search: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.search).to_string_lossy() });
    println!("    .path: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.path).to_string_lossy() });
    println!("    .hash: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.hash).to_string_lossy() });
    println!("    .query: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.query).to_string_lossy() });
    println!("    .port: \"{}\"", unsafe { std::ffi::CStr::from_ptr(data.port).to_string_lossy() });
}

unsafe fn main_0() -> std::os::raw::c_int {
    //url_inspect("https://google.com/search?q=github");
    let mut gh_url: *mut std::os::raw::c_char =
        b"git://git@github.com:jwerle/url.h.git\x00" as *const u8 as
            *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut url: *mut std::os::raw::c_char =
        b"http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash\x00"
            as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut parsed: *mut url_data_t = url_parse(url);
    let mut gh_parsed: *mut url_data_t = url_parse(gh_url);
    if parsed.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     15 as std::os::raw::c_int,
                     b"parsed\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if gh_parsed.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     16 as std::os::raw::c_int,
                     b"gh_parsed\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    let parsed_ref = unsafe { &*parsed };
url_data_inspect(parsed_ref);
    let gh_parsed_ref = unsafe { &*gh_parsed };
url_data_inspect(gh_parsed_ref);
    if (*parsed).href.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     21 as std::os::raw::c_int,
                     b"parsed->href\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).auth.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     22 as std::os::raw::c_int,
                     b"parsed->auth\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).protocol.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     23 as std::os::raw::c_int,
                     b"parsed->protocol\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).port.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     24 as std::os::raw::c_int,
                     b"parsed->port\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).hostname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     25 as std::os::raw::c_int,
                     b"parsed->hostname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).host.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     26 as std::os::raw::c_int,
                     b"parsed->host\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).pathname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     27 as std::os::raw::c_int,
                     b"parsed->pathname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).path.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     28 as std::os::raw::c_int,
                     b"parsed->path\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).hash.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     29 as std::os::raw::c_int,
                     b"parsed->hash\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).search.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     30 as std::os::raw::c_int,
                     b"parsed->search\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).query.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     31 as std::os::raw::c_int,
                     b"parsed->query\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).href.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     33 as std::os::raw::c_int,
                     b"gh_parsed->href\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).protocol.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     34 as std::os::raw::c_int,
                     b"gh_parsed->protocol\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).host.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     35 as std::os::raw::c_int,
                     b"gh_parsed->host\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).auth.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     36 as std::os::raw::c_int,
                     b"gh_parsed->auth\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).hostname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     37 as std::os::raw::c_int,
                     b"gh_parsed->hostname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).pathname.is_null() {
    panic!("gh_parsed->pathname is null");
}

if (*gh_parsed).path.is_null() {
    panic!("gh_parsed->path is null");
}

let protocols = ["http", "https", "git", "ssh", "sftp", "ftp", "javascript"];
for &protocol in &protocols {
    if !url_is_protocol(protocol.as_ptr() as *mut std::os::raw::c_char) {
        panic!("url_is_protocol(\"{}\") failed", protocol);
    }
}

let protocol = url_get_protocol(url);
if unsafe { CStr::from_ptr(protocol).to_string_lossy() } != "http" {
    panic!("Expected protocol to be \"http\", found \"{}\"", unsafe { CStr::from_ptr(protocol).to_string_lossy() });
}

let auth = url_get_auth(url);
if unsafe { CStr::from_ptr(auth).to_string_lossy() } != "user:pass" {
    panic!("Expected auth to be \"user:pass\", found \"{}\"", unsafe { CStr::from_ptr(auth).to_string_lossy() });
}

let hostname: *mut std::os::raw::c_char = url_get_hostname(url);
if unsafe { CStr::from_ptr(hostname).to_string_lossy() } != "subdomain.host.com:8080" {
    panic!("Expected hostname to be \"subdomain.host.com:8080\", found \"{}\"", unsafe { CStr::from_ptr(hostname).to_string_lossy() });
}

let host = unsafe { url_get_host(url) };
if unsafe { CStr::from_ptr(host).to_string_lossy() } != "subdomain.host.com" {
    panic!("Expected host to be \"subdomain.host.com\", found \"{}\"", unsafe { CStr::from_ptr(host).to_string_lossy() });
}

let pathname = url_get_pathname(url);
if unsafe { CStr::from_ptr(pathname).to_string_lossy() } != "/p/a/t/h" {
    panic!("Expected pathname to be \"/p/a/t/h\", found \"{}\"", unsafe { CStr::from_ptr(pathname).to_string_lossy() });
}

    if !(0 as std::os::raw::c_int ==
             strcmp(b"/p/a/t/h?query=string#hash\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_path(url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     54 as std::os::raw::c_int,
                     b"0 == strcmp(\"/p/a/t/h?query=string#hash\", url_get_path(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"?query=string\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_search(url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     55 as std::os::raw::c_int,
                     b"0 == strcmp(\"?query=string\", url_get_search(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"query=string\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_query(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     56 as std::os::raw::c_int,
                     b"0 == strcmp(\"query=string\", url_get_query(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"#hash\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_hash(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     57 as std::os::raw::c_int,
                     b"0 == strcmp(\"#hash\", url_get_hash(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"8080\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_port(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     58 as std::os::raw::c_int,
                     b"0 == strcmp(\"8080\", url_get_port(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"git\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_protocol(gh_url))) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     60 as std::os::raw::c_int,
                     b"0 == strcmp(\"git\", url_get_protocol(gh_url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"github.com\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_host(gh_url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     61 as std::os::raw::c_int,
                     b"0 == strcmp(\"github.com\", url_get_host(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"github.com\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_hostname(gh_url))) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     62 as std::os::raw::c_int,
                     b"0 == strcmp(\"github.com\", url_get_hostname(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"git\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_auth(gh_url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     63 as std::os::raw::c_int,
                     b"0 == strcmp(\"git\", url_get_auth(gh_url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"jwerle/url.h.git\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_pathname(gh_url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     64 as std::os::raw::c_int,
                     b"0 == strcmp(\"jwerle/url.h.git\", url_get_pathname(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"jwerle/url.h.git\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_path(gh_url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     65 as std::os::raw::c_int,
                     b"0 == strcmp(\"jwerle/url.h.git\", url_get_path(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    url_free(parsed);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}


}

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

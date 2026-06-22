#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url_data {
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
pub type url_data_t = url_data;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const URL_PROTOCOL_MAX_LENGTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub static mut URL_SCHEMES: [*mut ::core::ffi::c_char; 177] = [
    b"aaa\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"aaas\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"about\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"acap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"acct\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"adiumxtra\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"afp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"afs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"aim\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"apt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"attachment\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"aw\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"beshare\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"bitcoin\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"bolo\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"callto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"chrome\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"crome-extension\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"com-evenbrite-attendee\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"cid\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"coap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"coaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"content\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"crid\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cvs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"data\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"dav\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"dict\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"lna-playsingle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"dln-playcontainer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"dns\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"dtn\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"dvb\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ed2k\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"facetime\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"fax\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"feed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"file\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"finger\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"fish\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ftp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"geo\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"gg\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"git\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"gizmoproject\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"go\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"gopher\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"gtalk\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"h323\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"hcp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"http\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"https\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"iax\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"icap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"icon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"im\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"imap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"info\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ipn\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ipp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"irc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"irc6\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ircs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"iris\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"iris.beep\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"iris.xpc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"iris.xpcs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"iris.lws\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"itms\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"jabber\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"jar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"jms\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"keyparc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"lastfm\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ldap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ldaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"magnet\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mailserver\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mailto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"maps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"market\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"message\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mid\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mms\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"modem\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ms-help\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mssettings-power\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"msnim\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"msrp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"msrps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mtqp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mumble\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mupdate\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"mvn\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"news\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"nfs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ni\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"nih\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"nntp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"notes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"oid\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"paquelocktoken\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"pack\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"palm\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"paparazzi\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"pkcs11\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"platform\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"pop\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"pres\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"prospero\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"proxy\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"psyc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"query\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"reload\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"res\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"resource\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"rmi\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"rsync\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"rtmp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"rtsp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"secondlife\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"service\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"session\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"sftp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"sgn\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"shttp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"sieve\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"sip\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"sips\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"skype\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"smb\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"sms\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"snews\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"snmp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"soap.beep\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"soap.beeps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"soldat\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"spotify\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ssh\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"steam\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"svn\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tag\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"teamspeak\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"telnet\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tftp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"things\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"thismessage\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tn3270\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tip\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tv\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"udp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"unreal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"urn\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ut2004\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"vemmi\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ventrilo\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"videotex\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"view-source\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"wais\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"webcal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ws\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"wss\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"wtai\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"wyciwyg\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"xcon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"xcon-userid\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"xfire\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"xmlrpc.beep\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"xmlrpc.beeps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"xmpp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"xri\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ymsgr\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"javascript\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"jdbc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"doi\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn strdup(mut str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut n: ::core::ffi::c_int = strlen(str).wrapping_add(1 as size_t) as ::core::ffi::c_int;
    let mut dup: *mut ::core::ffi::c_char = malloc(n as size_t) as *mut ::core::ffi::c_char;
    if !dup.is_null() {
        strcpy(dup, str);
    }
    return dup;
}
unsafe extern "C" fn strff(
    mut ptr: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        y = *fresh0 as ::core::ffi::c_int;
        i += 1;
    }
    return strdup(ptr);
}
unsafe extern "C" fn strrwd(
    mut ptr: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n {
        let fresh1 = ptr;
        ptr = ptr.offset(-1);
        y = *fresh1 as ::core::ffi::c_int;
        i += 1;
    }
    return strdup(ptr);
}
unsafe extern "C" fn get_part(
    mut url: *mut ::core::ffi::c_char,
    mut format: *const ::core::ffi::c_char,
    mut l: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut has: bool = false_0 != 0;
    let mut tmp: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    let mut tmp_url: *mut ::core::ffi::c_char = strdup(url);
    let mut fmt_url: *mut ::core::ffi::c_char = strdup(url);
    let mut ret: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() || ret.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    strcpy(tmp, b"\0" as *const u8 as *const ::core::ffi::c_char);
    strcpy(fmt_url, b"\0" as *const u8 as *const ::core::ffi::c_char);
    fmt_url = strff(fmt_url, l);
    sscanf(fmt_url, format, tmp);
    if 0 as ::core::ffi::c_int != strcmp(tmp, tmp_url) {
        has = true_0 != 0;
        ret = strdup(tmp);
    }
    fmt_url = strrwd(fmt_url, l);
    free(tmp as *mut ::core::ffi::c_void);
    free(tmp_url as *mut ::core::ffi::c_void);
    free(fmt_url as *mut ::core::ffi::c_void);
    return if has as ::core::ffi::c_int != 0 {
        ret
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn url_parse(mut url: *mut ::core::ffi::c_char) -> *mut url_data_t {
    let mut data: *mut url_data_t =
        malloc(::core::mem::size_of::<url_data_t>() as size_t) as *mut url_data_t;
    if data.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    (*data).href = url;
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmp_url: *mut ::core::ffi::c_char = strdup(url);
    let mut is_ssh: bool = false_0 != 0;
    let mut protocol: *mut ::core::ffi::c_char = url_get_protocol(tmp_url);
    if protocol.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    let mut protocol_len: ::core::ffi::c_int =
        strlen(protocol) as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
    (*data).protocol = protocol;
    is_ssh = url_is_ssh(protocol);
    let mut auth: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    let mut auth_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    tmp = strstr(tmp_url, b"@\0" as *const u8 as *const ::core::ffi::c_char);
    if !tmp.is_null() {
        auth = get_part(
            tmp_url,
            b"%[^@]\0" as *const u8 as *const ::core::ffi::c_char,
            protocol_len,
        );
        auth_len = strlen(auth) as ::core::ffi::c_int;
        if !auth.is_null() {
            auth_len += 1;
        }
    }
    (*data).auth = auth;
    let mut hostname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    hostname = if is_ssh as ::core::ffi::c_int != 0 {
        get_part(
            tmp_url,
            b"%[^:]\0" as *const u8 as *const ::core::ffi::c_char,
            protocol_len + auth_len,
        )
    } else {
        get_part(
            tmp_url,
            b"%[^/]\0" as *const u8 as *const ::core::ffi::c_char,
            protocol_len + auth_len,
        )
    };
    if hostname.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    let mut hostname_len: ::core::ffi::c_int = strlen(hostname) as ::core::ffi::c_int;
    let mut tmp_hostname: *mut ::core::ffi::c_char = strdup(hostname);
    (*data).hostname = hostname;
    let mut host: *mut ::core::ffi::c_char = malloc(
        strlen(tmp_hostname).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    sscanf(
        tmp_hostname,
        b"%[^:]\0" as *const u8 as *const ::core::ffi::c_char,
        host,
    );
    if host.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    let mut host_len: ::core::ffi::c_int = strlen(host) as ::core::ffi::c_int;
    (*data).host = host;
    let mut tmp_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    tmp_path = if is_ssh as ::core::ffi::c_int != 0 {
        get_part(
            tmp_url,
            b":%s\0" as *const u8 as *const ::core::ffi::c_char,
            protocol_len + auth_len + hostname_len,
        )
    } else {
        get_part(
            tmp_url,
            b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
            protocol_len + auth_len + hostname_len,
        )
    };
    let mut path: *mut ::core::ffi::c_char = malloc(
        strlen(tmp_path).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    if path.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    let mut fmt: *mut ::core::ffi::c_char = (if is_ssh as ::core::ffi::c_int != 0 {
        b"%s\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"/%s\0" as *const u8 as *const ::core::ffi::c_char
    }) as *mut ::core::ffi::c_char;
    sprintf(path, fmt, tmp_path);
    (*data).path = path;
    free(tmp_path as *mut ::core::ffi::c_void);
    let mut pathname: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if pathname.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    strcat(pathname, b"\0" as *const u8 as *const ::core::ffi::c_char);
    tmp_path = strdup(path);
    sscanf(
        tmp_path,
        b"%[^? | ^#]\0" as *const u8 as *const ::core::ffi::c_char,
        pathname,
    );
    let mut pathname_len: ::core::ffi::c_int = strlen(pathname) as ::core::ffi::c_int;
    (*data).pathname = pathname;
    let mut search: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            as *mut ::core::ffi::c_char;
    if search.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    tmp_path = strff(tmp_path, pathname_len);
    strcat(search, b"\0" as *const u8 as *const ::core::ffi::c_char);
    sscanf(
        tmp_path,
        b"%[^#]\0" as *const u8 as *const ::core::ffi::c_char,
        search,
    );
    (*data).search = search;
    let mut search_len: ::core::ffi::c_int = strlen(search) as ::core::ffi::c_int;
    free(tmp_path as *mut ::core::ffi::c_void);
    let mut query: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if query.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    sscanf(
        search,
        b"?%s\0" as *const u8 as *const ::core::ffi::c_char,
        query,
    );
    (*data).query = query;
    let mut hash: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if hash.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    tmp_path = strff(path, pathname_len + search_len);
    strcat(hash, b"\0" as *const u8 as *const ::core::ffi::c_char);
    sscanf(
        tmp_path,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        hash,
    );
    (*data).hash = hash;
    free(tmp_path as *mut ::core::ffi::c_void);
    let mut port: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if port.is_null() {
        return ::core::ptr::null_mut::<url_data_t>();
    }
    tmp_hostname = strff(hostname, host_len + 1 as ::core::ffi::c_int);
    sscanf(
        tmp_hostname,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        port,
    );
    (*data).port = port;
    free(tmp_hostname as *mut ::core::ffi::c_void);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn url_is_protocol(mut str: *mut ::core::ffi::c_char) -> bool {
    let mut count: ::core::ffi::c_int = (::core::mem::size_of::<[*mut ::core::ffi::c_char; 177]>()
        as usize)
        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
        as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if 0 as ::core::ffi::c_int == strcmp(URL_SCHEMES[i as usize], str) {
            return true_0 != 0;
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn url_is_ssh(mut str: *mut ::core::ffi::c_char) -> bool {
    str = strdup(str);
    if 0 as ::core::ffi::c_int == strcmp(str, b"ssh\0" as *const u8 as *const ::core::ffi::c_char)
        || 0 as ::core::ffi::c_int
            == strcmp(str, b"git\0" as *const u8 as *const ::core::ffi::c_char)
    {
        free(str as *mut ::core::ffi::c_void);
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_protocol(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut protocol: *mut ::core::ffi::c_char = malloc(
        (URL_PROTOCOL_MAX_LENGTH as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    if protocol.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    sscanf(
        url,
        b"%[^://]\0" as *const u8 as *const ::core::ffi::c_char,
        protocol,
    );
    if url_is_protocol(protocol) {
        return protocol;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn url_get_auth(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut protocol: *mut ::core::ffi::c_char = url_get_protocol(url);
    if protocol.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut l: ::core::ffi::c_int =
        strlen(protocol) as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
    return get_part(
        url,
        b"%[^@]\0" as *const u8 as *const ::core::ffi::c_char,
        l,
    );
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hostname(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut l: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut protocol: *mut ::core::ffi::c_char = url_get_protocol(url);
    let mut tmp_protocol: *mut ::core::ffi::c_char = strdup(protocol);
    let mut auth: *mut ::core::ffi::c_char = url_get_auth(url);
    if protocol.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !auth.is_null() {
        l = (l as size_t).wrapping_add(strlen(auth).wrapping_add(1 as size_t)) as ::core::ffi::c_int
            as ::core::ffi::c_int;
    }
    if !auth.is_null() {
        free(auth as *mut ::core::ffi::c_void);
    }
    l += strlen(protocol) as ::core::ffi::c_int;
    free(protocol as *mut ::core::ffi::c_void);
    let mut hostname: *mut ::core::ffi::c_char =
        if url_is_ssh(tmp_protocol) as ::core::ffi::c_int != 0 {
            get_part(
                url,
                b"%[^:]\0" as *const u8 as *const ::core::ffi::c_char,
                l,
            )
        } else {
            get_part(
                url,
                b"%[^/]\0" as *const u8 as *const ::core::ffi::c_char,
                l,
            )
        };
    free(tmp_protocol as *mut ::core::ffi::c_void);
    return hostname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_host(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut host: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    let mut hostname: *mut ::core::ffi::c_char = url_get_hostname(url);
    if host.is_null() || hostname.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    sscanf(
        hostname,
        b"%[^:]\0" as *const u8 as *const ::core::ffi::c_char,
        host,
    );
    free(hostname as *mut ::core::ffi::c_void);
    return host;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_pathname(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut path: *mut ::core::ffi::c_char = url_get_path(url);
    let mut pathname: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if path.is_null() || pathname.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    strcat(pathname, b"\0" as *const u8 as *const ::core::ffi::c_char);
    sscanf(
        path,
        b"%[^?]\0" as *const u8 as *const ::core::ffi::c_char,
        pathname,
    );
    free(path as *mut ::core::ffi::c_void);
    return pathname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_path(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut l: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut tmp_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut protocol: *mut ::core::ffi::c_char = url_get_protocol(url);
    let mut auth: *mut ::core::ffi::c_char = url_get_auth(url);
    let mut hostname: *mut ::core::ffi::c_char = url_get_hostname(url);
    if protocol.is_null() || hostname.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut is_ssh: bool = url_is_ssh(protocol);
    l += strlen(protocol) as ::core::ffi::c_int + strlen(hostname) as ::core::ffi::c_int;
    if !auth.is_null() {
        l += strlen(auth) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
    tmp_path = if is_ssh as ::core::ffi::c_int != 0 {
        get_part(url, b":%s\0" as *const u8 as *const ::core::ffi::c_char, l)
    } else {
        get_part(url, b"/%s\0" as *const u8 as *const ::core::ffi::c_char, l)
    };
    let mut fmt: *mut ::core::ffi::c_char = (if is_ssh as ::core::ffi::c_int != 0 {
        b"%s\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"/%s\0" as *const u8 as *const ::core::ffi::c_char
    }) as *mut ::core::ffi::c_char;
    let mut path: *mut ::core::ffi::c_char = malloc(
        strlen(tmp_path).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    sprintf(path, fmt, tmp_path);
    if !auth.is_null() {
        free(auth as *mut ::core::ffi::c_void);
    }
    free(protocol as *mut ::core::ffi::c_void);
    free(hostname as *mut ::core::ffi::c_void);
    free(tmp_path as *mut ::core::ffi::c_void);
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_search(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut path: *mut ::core::ffi::c_char = url_get_path(url);
    let mut pathname: *mut ::core::ffi::c_char = url_get_pathname(url);
    let mut search: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if path.is_null() || search.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut tmp_path: *mut ::core::ffi::c_char =
        strff(path, strlen(pathname) as ::core::ffi::c_int);
    strcat(search, b"\0" as *const u8 as *const ::core::ffi::c_char);
    sscanf(
        tmp_path,
        b"%[^#]\0" as *const u8 as *const ::core::ffi::c_char,
        search,
    );
    tmp_path = strrwd(tmp_path, strlen(pathname) as ::core::ffi::c_int);
    free(path as *mut ::core::ffi::c_void);
    free(pathname as *mut ::core::ffi::c_void);
    return search;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_query(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut search: *mut ::core::ffi::c_char = url_get_search(url);
    let mut query: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if search.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    sscanf(
        search,
        b"?%s\0" as *const u8 as *const ::core::ffi::c_char,
        query,
    );
    free(search as *mut ::core::ffi::c_void);
    return query;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hash(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut hash: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    if hash.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut path: *mut ::core::ffi::c_char = url_get_path(url);
    if path.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut pathname: *mut ::core::ffi::c_char = url_get_pathname(url);
    if pathname.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut search: *mut ::core::ffi::c_char = url_get_search(url);
    let mut pathname_len: ::core::ffi::c_int = strlen(pathname) as ::core::ffi::c_int;
    let mut search_len: ::core::ffi::c_int = strlen(search) as ::core::ffi::c_int;
    let mut tmp_path: *mut ::core::ffi::c_char = strff(path, pathname_len + search_len);
    strcat(hash, b"\0" as *const u8 as *const ::core::ffi::c_char);
    sscanf(
        tmp_path,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        hash,
    );
    tmp_path = strrwd(tmp_path, pathname_len + search_len);
    free(tmp_path as *mut ::core::ffi::c_void);
    free(pathname as *mut ::core::ffi::c_void);
    free(path as *mut ::core::ffi::c_void);
    if !search.is_null() {
        free(search as *mut ::core::ffi::c_void);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_port(
    mut url: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut port: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<::core::ffi::c_char>() as size_t) as *mut ::core::ffi::c_char;
    let mut hostname: *mut ::core::ffi::c_char = url_get_hostname(url);
    let mut host: *mut ::core::ffi::c_char = url_get_host(url);
    if port.is_null() || hostname.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut tmp_hostname: *mut ::core::ffi::c_char = strff(
        hostname,
        strlen(host).wrapping_add(1 as size_t) as ::core::ffi::c_int,
    );
    sscanf(
        tmp_hostname,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        port,
    );
    free(hostname as *mut ::core::ffi::c_void);
    free(tmp_hostname as *mut ::core::ffi::c_void);
    return port;
}
#[no_mangle]
pub unsafe extern "C" fn url_inspect(mut url: *mut ::core::ffi::c_char) {
    url_data_inspect(url_parse(url));
}
#[no_mangle]
pub unsafe extern "C" fn url_data_inspect(mut data: *mut url_data_t) {
    printf(b"#url =>\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"    .href: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).href,
    );
    printf(
        b"    .protocol: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).protocol,
    );
    printf(
        b"    .host: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).host,
    );
    printf(
        b"    .auth: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).auth,
    );
    printf(
        b"    .hostname: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).hostname,
    );
    printf(
        b"    .pathname: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).pathname,
    );
    printf(
        b"    .search: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).search,
    );
    printf(
        b"    .path: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).path,
    );
    printf(
        b"    .hash: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).hash,
    );
    printf(
        b"    .query: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).query,
    );
    printf(
        b"    .port: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*data).port,
    );
}
#[no_mangle]
pub unsafe extern "C" fn url_free(mut data: *mut url_data_t) {
    if data.is_null() {
        return;
    }
    if !(*data).auth.is_null() {
        free((*data).auth as *mut ::core::ffi::c_void);
    }
    if !(*data).protocol.is_null() {
        free((*data).protocol as *mut ::core::ffi::c_void);
    }
    if !(*data).hostname.is_null() {
        free((*data).hostname as *mut ::core::ffi::c_void);
    }
    if !(*data).host.is_null() {
        free((*data).host as *mut ::core::ffi::c_void);
    }
    if !(*data).pathname.is_null() {
        free((*data).pathname as *mut ::core::ffi::c_void);
    }
    if !(*data).path.is_null() {
        free((*data).path as *mut ::core::ffi::c_void);
    }
    if !(*data).hash.is_null() {
        free((*data).hash as *mut ::core::ffi::c_void);
    }
    if !(*data).search.is_null() {
        free((*data).search as *mut ::core::ffi::c_void);
    }
    if !(*data).query.is_null() {
        free((*data).query as *mut ::core::ffi::c_void);
    }
}

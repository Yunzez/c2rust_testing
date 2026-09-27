#![allow(unused_imports, unused_variables, dead_code)]

pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
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
#[no_mangle]
pub static URL_SCHEMES: [*mut ::core::ffi::c_char; 177] = [
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
use libc::{c_char, c_int, c_void, free, malloc, sscanf, strcmp, strcpy};
pub unsafe fn get_part(url: *mut c_char, format: *const c_char, l: c_int) -> *mut c_char {
    let mut has: bool = false;
    let tmp = malloc(1) as *mut c_char;
    let tmp_url = strdup(url as *const c_char);
    let mut fmt_url = strdup(url as *const c_char);
    let mut ret = malloc(1) as *mut c_char;
    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() || ret.is_null() {
        return core::ptr::null_mut();
    }
    strcpy(tmp, b"\0".as_ptr() as *const c_char);
    strcpy(fmt_url, b"\0".as_ptr() as *const c_char);
    fmt_url = strff(fmt_url, l);
    sscanf(fmt_url as *const c_char, format, tmp);
    if strcmp(tmp as *const c_char, tmp_url as *const c_char) != 0 {
        has = true;
        ret = strdup(tmp as *const c_char);
    }
    fmt_url = strrwd(fmt_url, l);
    free(tmp as *mut c_void);
    free(tmp_url as *mut c_void);
    free(fmt_url as *mut c_void);
    if has {
        ret
    } else {
        core::ptr::null_mut()
    }
}
pub fn show(label: *const libc::c_char, v: *mut libc::c_char) {
    unsafe {
        libc::printf(
            b"%s: %s\n\0".as_ptr() as *const libc::c_char,
            label,
            if v.is_null() {
                b"(null)\0".as_ptr() as *const libc::c_char
            } else {
                v as *const libc::c_char
            },
        );
        if !v.is_null() {
            libc::free(v as *mut libc::c_void);
        }
    }
}
use libc::{c_char, c_int, malloc, strcpy, strlen};
pub unsafe fn strdup(str_ptr: *const c_char) -> *mut c_char {
    let n: c_int = strlen(str_ptr) as c_int + 1;
    let dup: *mut c_char = malloc(n as usize) as *mut c_char;
    if !dup.is_null() {
        strcpy(dup, str_ptr);
    }
    dup
}
use libc::c_char;
pub unsafe fn strff(ptr: *mut c_char, n: i32) -> *mut c_char {
    let mut y: i32 = 0;
    let mut p = ptr;
    for _ in 0..n {
        y = *p as i32;
        p = p.add(1);
    }
    strdup(p as *const c_char)
}
pub unsafe fn strrwd(mut ptr: *mut c_char, n: i32) -> *mut c_char {
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    while i < n {
        y = *ptr as i32;
        ptr = ptr.offset(-1);
        i += 1;
    }
    strdup(ptr as *const c_char)
}
pub unsafe fn url_data_inspect(data: *mut url_data) {
    use libc::printf;
    printf(b"#url =>\n\0".as_ptr() as *const i8);
    printf(b"    .href: \"%s\"\n\0".as_ptr() as *const i8, (*data).href);
    printf(
        b"    .protocol: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).protocol,
    );
    printf(b"    .host: \"%s\"\n\0".as_ptr() as *const i8, (*data).host);
    printf(b"    .auth: \"%s\"\n\0".as_ptr() as *const i8, (*data).auth);
    printf(
        b"    .hostname: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).hostname,
    );
    printf(
        b"    .pathname: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).pathname,
    );
    printf(
        b"    .search: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).search,
    );
    printf(b"    .path: \"%s\"\n\0".as_ptr() as *const i8, (*data).path);
    printf(b"    .hash: \"%s\"\n\0".as_ptr() as *const i8, (*data).hash);
    printf(
        b"    .query: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).query,
    );
    printf(b"    .port: \"%s\"\n\0".as_ptr() as *const i8, (*data).port);
}
pub unsafe fn url_free(data: *mut url_data) {
    if data.is_null() {
        return;
    }
    use libc::free;
    unsafe fn free_if_not_null(ptr: *mut ::core::ffi::c_char) {
        if !ptr.is_null() {
            free(ptr as *mut libc::c_void);
        }
    }
    free_if_not_null((*data).auth);
    free_if_not_null((*data).protocol);
    free_if_not_null((*data).hostname);
    free_if_not_null((*data).host);
    free_if_not_null((*data).pathname);
    free_if_not_null((*data).path);
    free_if_not_null((*data).hash);
    free_if_not_null((*data).search);
    free_if_not_null((*data).query);
}
use libc::{c_char, free, strcmp};
pub unsafe fn url_is_ssh(mut str_ptr: *mut c_char) -> bool {
    str_ptr = strdup(str_ptr as *const c_char);
    let ssh = b"ssh\0".as_ptr() as *const c_char;
    let git = b"git\0".as_ptr() as *const c_char;
    if strcmp(str_ptr, ssh) == 0 || strcmp(str_ptr, git) == 0 {
        free(str_ptr as *mut libc::c_void);
        return true;
    }
    false
}

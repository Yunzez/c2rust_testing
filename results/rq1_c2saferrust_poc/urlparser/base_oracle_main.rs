use std::io::{Read, Write};
use std::ffi::CStr;
use std::os::raw::c_char;

unsafe fn str_ret(p: *mut c_char) -> String {
    if p.is_null() { "null".to_string() }
    else { format!("s:{}", CStr::from_ptr(p).to_string_lossy()) }
}
unsafe fn call(idx: usize, url: *mut c_char) -> String {
    match idx {
        0 => format!("b:{}", url_base::url_is_protocol(url) as i32),
        1 => format!("b:{}", url_base::url_is_ssh(url) as i32),
        2 => str_ret(url_base::url_get_protocol(url)),
        3 => str_ret(url_base::url_get_auth(url)),
        4 => str_ret(url_base::url_get_hostname(url)),
        5 => str_ret(url_base::url_get_host(url)),
        6 => str_ret(url_base::url_get_pathname(url)),
        7 => str_ret(url_base::url_get_path(url)),
        8 => str_ret(url_base::url_get_search(url)),
        9 => str_ret(url_base::url_get_query(url)),
        10 => str_ret(url_base::url_get_hash(url)),
        11 => str_ret(url_base::url_get_port(url)),
        _ => "?".to_string(),
    }
}
fn decode(data: &[u8]) -> (usize, Vec<u8>) {
    let idx = (*data.get(0).unwrap_or(&0) as usize) % 12;
    let n = (*data.get(1).unwrap_or(&0) as usize) % 64;
    let mut s: Vec<u8> = (0..n).map(|i| { let b = *data.get(2+i).unwrap_or(&0); if b==0 {1} else {b} }).collect();
    s.push(0);
    (idx, s)
}
pub fn run(data: &[u8]) -> String {
    let (idx, mut s) = decode(data);
    unsafe { call(idx, s.as_mut_ptr() as *mut c_char) }
}
fn main() {
    let mut data = Vec::new(); std::io::stdin().read_to_end(&mut data).ok();
    let out = run(&data);
    let mut so = std::io::stdout(); let _=so.write_all(out.as_bytes()); let _=so.write_all(b"\n");
}
#[no_mangle]
pub extern "C" fn __assert_rtn(_a:*const i8,_b:*const i8,_c:i32,_d:*const i8)->!{ std::process::abort() }

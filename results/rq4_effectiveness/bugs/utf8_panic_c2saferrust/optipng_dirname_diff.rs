// Differential confirmation: C2SaferRust optipng inserts to_str().unwrap() on the -dir path
// (options.dir_name), panicking on a non-UTF-8 directory name where the faithful c2rust passes
// the raw char* straight to opng_os_create_dir (mkdir), which accepts arbitrary bytes.
use std::os::raw::c_char;
extern "C" { fn mkdir(path: *const c_char, mode: u32) -> i32; }

// faithful c2rust (base, optim.rs:3238): opng_os_create_dir(options.dir_name) -> mkdir(raw bytes)
unsafe fn base_create_dir(name: *const c_char) -> i32 { mkdir(name, 0o755) }
// C2SaferRust WIP (optim.rs:3096): to_str().unwrap() BEFORE create_dir
fn wip_create_dir(name: *const c_char) {
    let s = unsafe { std::ffi::CStr::from_ptr(name) }.to_str().unwrap(); // panics on non-UTF-8
    let _ = s; // (would then call opng_os_create_dir(s))
}
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    // non-UTF-8 dir name whose parent doesn't exist -> base mkdir fails cleanly (no side effect, no panic)
    let name: Vec<i8> = vec![0x8eu8 as i8, 0x8eu8 as i8, b'/' as i8, b'x' as i8, 0];
    let p = name.as_ptr() as *const c_char;
    let b = unsafe { base_create_dir(p) };
    let w = std::panic::catch_unwind(|| wip_create_dir(p));
    println!("input = non-UTF-8 directory name [0x8e,0x8e,'/','x']");
    println!("  base (=C/faithful c2rust): mkdir returned {} (clean, no panic)", b);
    match w { Ok(_) => println!("  WIP  (C2SaferRust):        no panic"),
              Err(_) => println!("  WIP  (C2SaferRust):        PANIC (to_str().unwrap on non-UTF-8)") }
    println!("=> DIVERGENCE confirmed: C accepts the bytes, C2SaferRust crashes before create_dir.");
}

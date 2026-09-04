// Differential confirmation: C2SaferRust bzip2::endsInBz2 panics on a non-UTF-8 filename
// where the faithful c2rust (= C) returns a value. Both bodies copied verbatim.
use std::os::raw::c_char;
extern "C" { fn strlen(s: *const c_char) -> usize; }

// faithful c2rust (= original C): byte-wise, handles arbitrary bytes
unsafe fn base_endsInBz2(name: *mut c_char) -> i32 {
    let n = strlen(name) as i32;
    if n <= 4 { return 0; }
    (*name.offset((n-4) as isize) as i32 == '.' as i32
     && *name.offset((n-3) as isize) as i32 == 'b' as i32
     && *name.offset((n-2) as isize) as i32 == 'z' as i32
     && *name.offset((n-1) as isize) as i32 == '2' as i32) as i32
}
// C2SaferRust WIP: to_str().unwrap() -> panics on non-UTF-8
fn wip_endsInBz2(name: *mut c_char) -> i32 {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    let str_slice = c_str.to_str().unwrap();
    let n = str_slice.len();
    if n <= 4 { return 0; }
    if str_slice.ends_with(".bz2") { return 1; }
    0
}
fn main() {
    std::panic::set_hook(Box::new(|_| {}));  // silence panic output
    // a non-UTF-8 filename: 0x8e is not valid UTF-8 (Linux filenames can be arbitrary bytes)
    let name: Vec<i8> = vec![0x8eu8 as i8, b'.' as i8, b'b' as i8, b'z' as i8, b'2' as i8, 0];
    let p = name.as_ptr() as *mut c_char;
    let b = unsafe { base_endsInBz2(p) };
    let w = std::panic::catch_unwind(|| wip_endsInBz2(p));
    println!("input = non-UTF-8 filename [0x8e,'.','b','z','2']");
    println!("  base (=C/faithful c2rust): returns {}  (clean)", b);
    match w {
        Ok(v) => println!("  WIP  (C2SaferRust):        returns {}  (clean)", v),
        Err(_) => println!("  WIP  (C2SaferRust):        PANIC (to_str().unwrap on non-UTF-8)"),
    }
    println!("=> DIVERGENCE confirmed: same input, C returns a value, C2SaferRust crashes.");
}

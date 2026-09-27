// Differential confirmation: C2SaferRust lil do_system (the interpreter's `system` builtin) builds
// its command with arg.to_str().unwrap(), panicking on a non-UTF-8 argument, where faithful c2rust
// concatenates the raw bytes (strlen+memcpy). A lil script passing a non-UTF-8 arg to `system`
// crashes the interpreter.
use std::os::raw::c_char;
extern "C" { fn strlen(s: *const c_char) -> usize; }

// faithful c2rust (base): byte-wise concat of argv (strlen+memcpy) -> handles any bytes
unsafe fn base_do_system(argv: &[*mut c_char]) -> Vec<u8> {
    let mut cmd = Vec::new();
    for (i, &a) in argv.iter().enumerate() {
        if i != 0 { cmd.push(b' '); }
        let len = strlen(a);
        for k in 0..len { cmd.push(*a.add(k) as u8); }
    }
    cmd
}
// C2SaferRust WIP: arg.to_str().unwrap() -> panics on non-UTF-8
fn wip_do_system(argv: &[*mut c_char]) -> String {
    let mut cmd = String::new();
    for (i, &a) in argv.iter().enumerate() {
        if i != 0 { cmd.push(' '); }
        let arg = unsafe { std::ffi::CStr::from_ptr(a) };
        cmd.push_str(arg.to_str().unwrap());   // panic on non-UTF-8
    }
    cmd
}
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let a: Vec<i8> = vec![0x8eu8 as i8, 0x8fu8 as i8, 0]; // non-UTF-8 arg
    let argv = [a.as_ptr() as *mut c_char];
    let b = unsafe { base_do_system(&argv) };
    let w = std::panic::catch_unwind(|| wip_do_system(&argv));
    println!("input = lil `system` arg with non-UTF-8 bytes [0x8e,0x8f]");
    println!("  base (=C/faithful c2rust): built command of {} bytes (clean)", b.len());
    match w { Ok(s) => println!("  WIP  (C2SaferRust):        built {:?} (clean)", s),
              Err(_) => println!("  WIP  (C2SaferRust):        PANIC (to_str().unwrap on non-UTF-8)") }
    println!("=> DIVERGENCE confirmed: C builds the command, C2SaferRust crashes the interpreter.");
}

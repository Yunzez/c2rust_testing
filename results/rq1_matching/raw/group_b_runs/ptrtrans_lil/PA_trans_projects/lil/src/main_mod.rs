
use crate::{_lil_t, _lil_value_t, lil_to_integer};
use std::io;
use std::io::{Read};
use std::process::{Command, Stdio};
use crate::*;
pub fn do_system(argc: usize, argv: Option<&[Option<&str>]>) -> Option<String> {
    // Build the command string exactly like the C code did
    let mut cmd = String::new();
    if let Some(args) = argv {
        for i in 0..argc {
            // In C: argv is nullable, and each argv[i] is a `char*` that might be null.
            // Here: model as Option<&str>; skip if missing.
            if let Some(arg) = args.get(i).and_then(|a| *a) {
                if i != 0 {
                    cmd.push(' ');
                }
                cmd.push_str(arg);
            }
        }
    }
    // If no command could be constructed at all, behave like p = NULL → return NULL
    if cmd.is_empty() {
        return None;
    }
    // The C code uses `popen(cmd, "r")`. Here we approximate this by:
    // - invoking a shell to execute the command string
    // - capturing stdout
    //
    // Note: this is an approximation; exact `popen` behavior and shell may differ.
    let mut child = match Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return None,
    };
    let stdout = match child.stdout.take() {
        Some(s) => s,
        None => return None,
    };
    let mut reader = stdout;
    let mut retval = Vec::new();
    let mut buff = [0u8; 1024];
    // Read until EOF, similar to the `while ((bytes = fread(...)))` loop in C.
    loop {
        match reader.read(&mut buff) {
            Ok(0) => break, // EOF
            Ok(bytes) => {
                retval.extend_from_slice(&buff[..bytes]);
            }
            Err(_) => return None,
        }
    }
    // Wait for the command to finish (similar to pclose)
    let _ = child.wait();
    // In C, retval is always null-terminated; here we convert bytes to a String.
    // If the output is not valid UTF‑8, we fall back to a lossy conversion so we
    // can still return "some" string, similar to how raw bytes with a '\0' would be returned.
    Some(String::from_utf8(retval).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned()))
}
pub fn fnc_system(
    lil: Option<&_lil_t>,                // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[Option<&_lil_value_t>]>, // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // Match C: if (argc == 0) return NULL;
    if argc == 0 {
        return None;
    }
    // In C, malloc + NULL-terminated array; here we just build a Vec of the same size.
    // C: for (i=0; i<argc; i++) sargv[i] = lil_to_string(argv[i]);
    // lil_to_string: Option<&_lil_value_t> -> Option<&mut String>
    // do_system expects: Option<&[Option<&str>]>
    let mut sargv: Vec<Option<&str>> = Vec::with_capacity(argc);
    if let Some(args_slice) = argv {
        for i in 0..argc {
            let val_opt = args_slice.get(i).cloned().unwrap_or(None);
            let s_opt_mut = lil_to_string(val_opt);
            // Convert Option<&mut String> to Option<&str> while preserving nullability
            let s_opt: Option<&str> = s_opt_mut.map(|s| s.as_str());
            sargv.push(s_opt);
        }
    }
    // C: rv = do_system(argc, (char**)sargv);
    // Rust: pass as slice; we ignore argc for the call because the slice length encodes it.
    let rv = do_system(argc, Some(&sargv[..]));
    // C:
    // if (rv) {
    //     r = lil_alloc_string(rv);
    // }
    // return r;
    match rv {
        Some(ref s) => lil_alloc_string(Some(s.as_str())),
        None => None,
    }
}
// add this at the top of the file where `fnc_readline` is defined
pub fn fnc_readline(
    lil: Option<&_lil_t>,                    // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[Option<&_lil_value_t>]>,  // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    let mut buffer = Vec::with_capacity(64);
    loop {
        let mut byte = [0u8; 1];
        let n = match io::stdin().read(&mut byte) {
            Ok(0) => break,          // EOF (ch == -1)
            Ok(_) => byte[0],
            Err(_) => break,         // treat I/O error like EOF
        };
        let ch = n as char;
        if ch == '\r' {
            continue;
        }
        if ch == '\n' {
            break;
        }
        buffer.push(n);
    }
    // Convert collected bytes to a UTF-8 String; if invalid, fall back to lossily converting
    let s = String::from_utf8(buffer)
        .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned());
    lil_alloc_string(Some(&s))
}
pub fn do_exit(lil: Option<&_lil_t>, val: Option<&_lil_value_t>) {
    let mut running: i32 = 1;
    let mut exit_code: i32 = 0;
    running = 0;
    exit_code = lil_to_integer(val) as i32;
    // `lil`, `running`, and `exit_code` are unused beyond this point,
    // matching the original C function's behavior (no side effects).
    let _ = lil;
    let _ = running;
    let _ = exit_code;
}
pub fn fnc_writechar<'a, 'b>(
    lil: Option<&'a _lil_t>,           // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&'b [_lil_value_t]>,  // nullable, borrowed, immutable
) -> Option<&'static _lil_value_t> {   // nullable, borrowed, immutable, No_Depends
    if argc == 0 {
        return None;
    }
    // Safe handling of nullable argv and potential out-of-bounds
    if let Some(args) = argv {
        if !args.is_empty() {
            let ch = lil_to_integer(Some(&args[0])) as u8 as char;
            print!("{ch}");
        }
    }
    None
}

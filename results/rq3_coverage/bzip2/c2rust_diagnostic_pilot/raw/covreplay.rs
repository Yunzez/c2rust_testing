// RQ4 coverage — replay driver for the archived libFuzzer corpus.
//
// The body below is a line-for-line transcription of the archived fuzz target
// fuzz/bzip2_c2rust_e3/fuzz/fuzz_targets/ft.rs, so that replaying the archived corpus
// through this binary drives exactly the boundary the campaign drove.  Inputs are
// replayed once each, never mutated, and no new inputs are generated.
//
// usage: covreplay <file>...
// prints one line per input:  <path>\t<verdict>\t<rc>\t<dlen>

use std::io::Write;

fn replay(data: &[u8]) -> (i32, u32) {
    if data.len() < 3 {
        return (i32::MIN, 0); // ft.rs `return` — input carries no execution
    }
    let bs = ((data[0] % 9) + 1) as std::os::raw::c_int;
    let wf = (data[1] % 251) as std::os::raw::c_int;
    let src = &data[2..];
    let mut dest = vec![0i8; src.len() * 2 + 1024];
    let mut dlen = dest.len() as std::os::raw::c_uint;
    unsafe {
        let rc = bz_cov::bzlib::BZ2_bzBuffToBuffCompress(
            dest.as_mut_ptr(),
            &mut dlen,
            src.as_ptr() as *mut std::os::raw::c_char,
            src.len() as std::os::raw::c_uint,
            bs,
            0,
            wf,
        );
        if rc == 0 {
            let mut back = vec![0i8; src.len() + 1024];
            let mut blen = back.len() as std::os::raw::c_uint;
            let _ = bz_cov::bzlib::BZ2_bzBuffToBuffDecompress(
                back.as_mut_ptr(),
                &mut blen,
                dest.as_mut_ptr(),
                dlen,
                (data[0] & 1) as std::os::raw::c_int,
                0,
            );
        }
        (rc, dlen)
    }
}

fn main() {
    let mut out = std::io::stdout();
    for path in std::env::args().skip(1) {
        match std::fs::read(&path) {
            Ok(data) => {
                let (rc, dlen) = replay(&data);
                if rc == i32::MIN {
                    let _ = writeln!(out, "{path}\tskipped_short\t-\t-");
                } else {
                    let _ = writeln!(out, "{path}\treplayed\t{rc}\t{dlen}");
                }
            }
            Err(e) => {
                let _ = writeln!(out, "{path}\tread_error\t{e}\t-");
            }
        }
        let _ = out.flush();
    }
}

#![no_main]
//! OOP differential harness for CROWN-lifted rgba_from_string vs the C original.
//! Rust called NATIVELY (Option<&mut c_short> out-param); C runs as a subprocess oracle (UB gate).
use libfuzzer_sys::fuzz_target;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const ORACLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../oracle/rgba_oracle");
const ORACLE_TIMEOUT_MS: u64 = 2000;

fn run_oracle(data: &[u8]) -> Option<String> {
    let mut ch = Command::new(ORACLE)
        .env("ASAN_OPTIONS", "symbolize=0:detect_leaks=0:abort_on_error=0:exitcode=1")
        .env("UBSAN_OPTIONS", "symbolize=0:abort_on_error=0")
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    ch.stdin.take()?.write_all(data).ok();
    let mut so = ch.stdout.take()?;
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || { let mut s = String::new(); let _ = so.read_to_string(&mut s); let _ = tx.send(s); });
    let deadline = Instant::now() + Duration::from_millis(ORACLE_TIMEOUT_MS);
    let status = loop {
        match ch.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => { if Instant::now() >= deadline { let _ = ch.kill(); let _ = ch.wait(); return None; }
                          std::thread::sleep(Duration::from_millis(1)); }
            Err(_) => return None,
        }
    };
    if !status.success() { return None; }   // C UB / crash -> gate
    let s = rx.recv_timeout(Duration::from_millis(500)).ok()?;
    Some(s.trim().to_string())
}

fn rust_side(data: &[u8]) -> String {
    let n = (*data.get(0).unwrap_or(&0) as usize) % 65;
    let mut sbuf: Vec<i8> = (0..n).map(|i| *data.get(1 + i).unwrap_or(&0) as i8).collect();
    sbuf.push(0);
    let mut ok: i16 = 0;   // C `short`
    let ret = unsafe {
        rgba_rs::src::src::rgba::rgba_from_string(sbuf.as_ptr(), Some(&mut ok))
    };
    format!("ret:{} ok:{}", ret, ok)
}

fuzz_target!(|data: &[u8]| {
    let c_out = match run_oracle(data) { Some(s) => s, None => return };
    let r_out = rust_side(data);
    if c_out != r_out {
        // determinism gate: re-run the C oracle; only a self-consistent oracle can convict Rust.
        match run_oracle(data) {
            Some(c2) if c2 == c_out => panic!("divergence: C={:?} Rust={:?}", c_out, r_out),
            _ => return,
        }
    }
});

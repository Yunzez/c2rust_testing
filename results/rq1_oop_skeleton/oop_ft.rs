#![no_main]
//! Out-of-process differential harness (minimal skeleton), qsort.
//!
//! Rust side: the C2SaferRust idiomatic `quickSort(&mut [i32], usize, usize)` is called
//! NATIVELY as a Rust crate function -- no FFI, no extern "C", any signature shape works.
//! C side: the real C `quickSort` runs as a SUBPROCESS oracle (`c_oracle`, UBSan+ASan);
//! a nonzero exit == C hit UB on this input -> gate (discard). Both sides decode the same
//! byte format, so they see the same input. A divergence (mismatch, or a Rust-side crash
//! while C is clean) is a candidate bug.

use libfuzzer_sys::fuzz_target;
use std::io::Write;
use std::process::{Command, Stdio};

const MAXLEN: usize = 32;
const ORACLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../c_oracle");

/// Shared decode: byte0 = n mod 33; then n little-endian i32 (zero-filled past end).
fn decode(data: &[u8]) -> Vec<i32> {
    let n = if data.is_empty() { 0 } else { (data[0] as usize) % (MAXLEN + 1) };
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let off = 1 + i * 4;
        let mut x = 0u32;
        for b in 0..4 {
            let byte = data.get(off + b).copied().unwrap_or(0);
            x |= (byte as u32) << (8 * b);
        }
        v.push(x as i32);
    }
    v
}

/// Run the C oracle subprocess on the same bytes. Some(sorted) if C ran clean; None if C UB (gate).
fn c_oracle(data: &[u8]) -> Option<Vec<i32>> {
    let mut child = Command::new(ORACLE)
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null())
        .spawn().expect("spawn c_oracle");
    child.stdin.take().unwrap().write_all(data).ok();
    let out = child.wait_with_output().expect("wait c_oracle");
    if !out.status.success() {
        return None; // C hit UB (sanitizer abort) or crashed -> not a usable oracle on this input
    }
    let s = String::from_utf8_lossy(&out.stdout);
    let mut it = s.split_whitespace();
    let n: usize = it.next()?.parse().ok()?;
    let v: Vec<i32> = it.filter_map(|t| t.parse().ok()).collect();
    if v.len() != n { return None; }
    Some(v)
}

fuzz_target!(|data: &[u8]| {
    let arr = decode(data);
    if arr.is_empty() { return; }
    let n = arr.len();

    // C oracle (subprocess). Gate: skip inputs where C is UB.
    let c_out = match c_oracle(data) { Some(o) => o, None => return };

    // Rust side: native call of the idiomatic reshaped fn (no FFI). The C2SaferRust bug
    // (usize index wraparound) manifests here as infinite recursion / OOB -> process crash,
    // which libFuzzer reports; a wrong-but-not-crashing result is caught by the compare.
    let mut v = arr.clone();
    translated::quickSort(&mut v, 0, n - 1);

    assert_eq!(c_out, v, "divergence: C sorted {:?} but Rust produced {:?}", c_out, v);
});

use oop_qsort as translated;

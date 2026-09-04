// SACTOR unidiomatic translation of `myfeof` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:39:01; attempt 1). Verification verdict: Rust code failed to compile
unsafe fn myfeof(f: *mut libc::FILE) -> libc::c_int {
    let c: libc::c_int = libc::fgetc(f);
    if c == -1 {
        return 1;
    }
    libc::ungetc(c, f);
    0
}

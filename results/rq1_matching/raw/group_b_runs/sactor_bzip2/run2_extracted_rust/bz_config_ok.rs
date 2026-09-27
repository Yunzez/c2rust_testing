// SACTOR unidiomatic translation of `bz_config_ok` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:34:57; attempt 1). Verification verdict: rust compiled
fn bz_config_ok() -> libc::c_int {
    fn size_of<T>() -> usize {
        std::mem::size_of::<T>()
    }
    if size_of::<libc::c_int>() != 4 {
        return 0;
    }
    if size_of::<libc::c_short>() != 2 {
        return 0;
    }
    if size_of::<libc::c_char>() != 1 {
        return 0;
    }
    1
}

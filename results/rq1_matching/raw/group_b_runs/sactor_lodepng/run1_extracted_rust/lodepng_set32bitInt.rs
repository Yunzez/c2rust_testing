// SACTOR unidiomatic translation of `lodepng_set32bitInt` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:35:44; attempt 1). Verification verdict: rust compiled
fn lodepng_set32bitInt(buffer: *mut u8, value: libc::c_uint) {
    unsafe {
        *buffer.add(0) = ((value >> 24) & 0xff) as u8;
        *buffer.add(1) = ((value >> 16) & 0xff) as u8;
        *buffer.add(2) = ((value >> 8) & 0xff) as u8;
        *buffer.add(3) = (value & 0xff) as u8;
    }
}

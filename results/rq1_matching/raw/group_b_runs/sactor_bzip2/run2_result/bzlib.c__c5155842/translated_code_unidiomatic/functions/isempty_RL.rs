pub unsafe fn isempty_RL(s: *mut EState) -> ::core::ffi::c_int {
    if !s.is_null() && (*s).state_in_ch < 256 && (*s).state_in_len > 0 {
        0
    } else {
        1
    }
}

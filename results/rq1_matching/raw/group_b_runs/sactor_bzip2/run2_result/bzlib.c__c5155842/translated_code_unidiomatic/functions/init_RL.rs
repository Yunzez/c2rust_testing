pub unsafe fn init_RL(s: *mut EState) {
    if !s.is_null() {
        (*s).state_in_ch = 256;
        (*s).state_in_len = 0;
    }
}

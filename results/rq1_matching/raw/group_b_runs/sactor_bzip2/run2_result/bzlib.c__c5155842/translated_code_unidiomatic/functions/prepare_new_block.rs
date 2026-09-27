pub unsafe fn prepare_new_block(s: *mut EState) {
    (*s).nblock = 0;
    (*s).numZ = 0;
    (*s).state_out_pos = 0;
    (*s).blockCRC = 0xffffffffu32;
    let mut i: ::core::ffi::c_int = 0;
    while i < 256 {
        (*s).inUse[i as usize] = 0u8;
        i += 1;
    }
    (*s).blockNo += 1;
}

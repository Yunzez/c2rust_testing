// SACTOR unidiomatic translation of `add_pair_to_block` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:36:34; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn add_pair_to_block(s: *mut EState) {
    unsafe fn bz_update_crc(crc_var: &mut u32, cha: u8) {
        let idx = ((*crc_var >> 24) ^ cha as u32) as usize;
        *crc_var = (*crc_var << 8) ^ BZ2_crc32Table[idx];
    }
    let s_ref: &mut EState = &mut *s;
    let mut i: i32 = 0;
    let ch: u8 = s_ref.state_in_ch as u8;
    while i < s_ref.state_in_len {
        bz_update_crc(&mut s_ref.blockCRC, ch);
        i += 1;
    }
    s_ref.inUse[s_ref.state_in_ch as usize] = 1u8;
    match s_ref.state_in_len {
        1 => {
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
        }
        2 => {
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
        }
        3 => {
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
        }
        _ => {
            let idx = (s_ref.state_in_len - 4) as usize;
            s_ref.inUse[idx] = 1u8;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = (s_ref.state_in_len - 4) as u8;
            s_ref.nblock += 1;
        }
    }
}

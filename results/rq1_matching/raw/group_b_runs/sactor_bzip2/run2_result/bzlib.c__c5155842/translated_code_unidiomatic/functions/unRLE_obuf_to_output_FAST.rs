#[no_mangle]
pub unsafe fn unRLE_obuf_to_output_FAST(s: *mut DState) -> ::core::ffi::c_int {
    use core::ffi::{c_int, c_uchar, c_uint};
    const TRUE: c_int = 1;
    const FALSE: c_int = 0;
    #[inline]
    unsafe fn bz_update_crc(crc_var: &mut c_uint, cha: c_uchar) {
        let idx = ((*crc_var >> 24) ^ cha as c_uint) as usize;
        *crc_var = (*crc_var << 8) ^ BZ2_crc32Table[idx];
    }
    #[inline]
    unsafe fn bz_get_fast(s: *mut DState, out: &mut c_uchar) -> c_int {
        let s_ref = &mut *s;
        if s_ref.tPos >= 100000u32.wrapping_mul(s_ref.blockSize100k as c_uint) {
            return TRUE;
        }
        let t_pos = s_ref.tPos as usize;
        s_ref.tPos = *s_ref.tt.add(t_pos);
        *out = (s_ref.tPos & 0xff) as c_uchar;
        s_ref.tPos >>= 8;
        FALSE
    }
    #[inline]
    unsafe fn bz_rand_upd_mask(s: *mut DState) {
        let s_ref = &mut *s;
        if s_ref.rNToGo == 0 {
            s_ref.rNToGo = BZ2_rNums[s_ref.rTPos as usize];
            s_ref.rTPos += 1;
            if s_ref.rTPos == 512 {
                s_ref.rTPos = 0;
            }
        }
        s_ref.rNToGo -= 1;
    }
    #[inline]
    unsafe fn bz_get_fast_c(
        c_tPos: &mut c_uint,
        c_tt: *mut c_uint,
        ro_blockSize100k: c_int,
        out: &mut c_uchar,
    ) -> c_int {
        if *c_tPos >= 100000u32.wrapping_mul(ro_blockSize100k as c_uint) {
            return TRUE;
        }
        let idx = *c_tPos as usize;
        *c_tPos = *c_tt.add(idx);
        *out = (*c_tPos & 0xff) as c_uchar;
        *c_tPos >>= 8;
        FALSE
    }
    let s_ref = &mut *s;
    if s_ref.blockRandomised != 0 {
        let mut k1: c_uchar = 0;
        loop {
            loop {
                if (*s_ref.strm).avail_out == 0 {
                    return FALSE;
                }
                if s_ref.state_out_len == 0 {
                    break;
                }
                *(*s_ref.strm).next_out.cast::<c_uchar>() = s_ref.state_out_ch;
                bz_update_crc(&mut s_ref.calculatedBlockCRC, s_ref.state_out_ch);
                s_ref.state_out_len -= 1;
                (*s_ref.strm).next_out = (*s_ref.strm).next_out.add(1);
                (*s_ref.strm).avail_out = (*s_ref.strm).avail_out.wrapping_sub(1);
                (*s_ref.strm).total_out_lo32 = (*s_ref.strm).total_out_lo32.wrapping_add(1);
                if (*s_ref.strm).total_out_lo32 == 0 {
                    (*s_ref.strm).total_out_hi32 = (*s_ref.strm).total_out_hi32.wrapping_add(1);
                }
            }
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                return FALSE;
            }
            if s_ref.nblock_used > s_ref.save_nblock + 1 {
                return TRUE;
            }
            s_ref.state_out_len = 1;
            s_ref.state_out_ch = s_ref.k0 as c_uchar;
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                continue;
            }
            if k1 as c_int != s_ref.k0 {
                s_ref.k0 = k1 as c_int;
                continue;
            }
            s_ref.state_out_len = 2;
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                continue;
            }
            if k1 as c_int != s_ref.k0 {
                s_ref.k0 = k1 as c_int;
                continue;
            }
            s_ref.state_out_len = 3;
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                continue;
            }
            if k1 as c_int != s_ref.k0 {
                s_ref.k0 = k1 as c_int;
                continue;
            }
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            s_ref.state_out_len = (k1 as c_int) + 4;
            let mut new_k0: c_uchar = 0;
            if bz_get_fast(s, &mut new_k0) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            s_ref.k0 = (new_k0 ^ rand_mask) as c_int;
            s_ref.nblock_used += 1;
        }
    } else {
        let mut c_calculatedBlockCRC: c_uint = s_ref.calculatedBlockCRC;
        let mut c_state_out_ch: c_uchar = s_ref.state_out_ch;
        let mut c_state_out_len: c_int = s_ref.state_out_len;
        let mut c_nblock_used: c_int = s_ref.nblock_used;
        let mut c_k0: c_int = s_ref.k0;
        let c_tt: *mut c_uint = s_ref.tt;
        let mut c_tPos: c_uint = s_ref.tPos;
        let mut cs_next_out: *mut ::core::ffi::c_char = (*s_ref.strm).next_out;
        let mut cs_avail_out: c_uint = (*s_ref.strm).avail_out;
        let ro_blockSize100k: c_int = s_ref.blockSize100k;
        let avail_out_INIT: c_uint = cs_avail_out;
        let s_save_nblockPP: c_int = s_ref.save_nblock + 1;
        let mut total_out_lo32_old: c_uint;
        let mut k1: c_uchar = 0;
        'main_loop: loop {
            if c_state_out_len > 0 {
                loop {
                    if cs_avail_out == 0 {
                        break 'main_loop;
                    }
                    if c_state_out_len == 1 {
                        break;
                    }
                    *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                    bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                    c_state_out_len -= 1;
                    cs_next_out = cs_next_out.add(1);
                    cs_avail_out = cs_avail_out.wrapping_sub(1);
                }
                {
                    if cs_avail_out == 0 {
                        c_state_out_len = 1;
                        break 'main_loop;
                    }
                    *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                    bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                    cs_next_out = cs_next_out.add(1);
                    cs_avail_out = cs_avail_out.wrapping_sub(1);
                }
            }
            if c_nblock_used > s_save_nblockPP {
                return TRUE;
            }
            if c_nblock_used == s_save_nblockPP {
                c_state_out_len = 0;
                break 'main_loop;
            }
            c_state_out_ch = c_k0 as c_uchar;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            if k1 as c_int != c_k0 {
                c_k0 = k1 as c_int;
                if cs_avail_out == 0 {
                    c_state_out_len = 1;
                    break 'main_loop;
                }
                *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                cs_next_out = cs_next_out.add(1);
                cs_avail_out = cs_avail_out.wrapping_sub(1);
                continue;
            }
            if c_nblock_used == s_save_nblockPP {
                if cs_avail_out == 0 {
                    c_state_out_len = 1;
                    break 'main_loop;
                }
                *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                cs_next_out = cs_next_out.add(1);
                cs_avail_out = cs_avail_out.wrapping_sub(1);
                continue;
            }
            c_state_out_len = 2;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            if c_nblock_used == s_save_nblockPP {
                continue;
            }
            if k1 as c_int != c_k0 {
                c_k0 = k1 as c_int;
                continue;
            }
            c_state_out_len = 3;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            if c_nblock_used == s_save_nblockPP {
                continue;
            }
            if k1 as c_int != c_k0 {
                c_k0 = k1 as c_int;
                continue;
            }
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            c_state_out_len = (k1 as c_int) + 4;
            let mut tmp_k0: c_uchar = 0;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut tmp_k0) != FALSE {
                return TRUE;
            }
            c_k0 = tmp_k0 as c_int;
            c_nblock_used += 1;
        }
        total_out_lo32_old = (*s_ref.strm).total_out_lo32;
        (*s_ref.strm).total_out_lo32 = (*s_ref.strm)
            .total_out_lo32
            .wrapping_add(avail_out_INIT.wrapping_sub(cs_avail_out));
        if (*s_ref.strm).total_out_lo32 < total_out_lo32_old {
            (*s_ref.strm).total_out_hi32 = (*s_ref.strm).total_out_hi32.wrapping_add(1);
        }
        s_ref.calculatedBlockCRC = c_calculatedBlockCRC;
        s_ref.state_out_ch = c_state_out_ch;
        s_ref.state_out_len = c_state_out_len;
        s_ref.nblock_used = c_nblock_used;
        s_ref.k0 = c_k0;
        s_ref.tt = c_tt;
        s_ref.tPos = c_tPos;
        (*s_ref.strm).next_out = cs_next_out;
        (*s_ref.strm).avail_out = cs_avail_out;
    }
    FALSE
}

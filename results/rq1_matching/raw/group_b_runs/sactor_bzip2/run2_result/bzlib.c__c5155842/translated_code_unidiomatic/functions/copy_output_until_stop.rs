pub unsafe fn copy_output_until_stop(s: *mut EState) -> bool {
    let mut progress_out: bool = false;
    loop {
        if (*(*s).strm).avail_out == 0 {
            break;
        }
        if (*s).state_out_pos >= (*s).numZ {
            break;
        }
        progress_out = true;
        *(*(*s).strm).next_out =
            *(*s).zbits.add((*s).state_out_pos as usize) as ::core::ffi::c_char;
        (*s).state_out_pos += 1;
        (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(1);
        (*(*s).strm).next_out = (*(*s).strm).next_out.add(1);
        (*(*s).strm).total_out_lo32 = (*(*s).strm).total_out_lo32.wrapping_add(1);
        if (*(*s).strm).total_out_lo32 == 0 {
            (*(*s).strm).total_out_hi32 = (*(*s).strm).total_out_hi32.wrapping_add(1);
        }
    }
    progress_out
}

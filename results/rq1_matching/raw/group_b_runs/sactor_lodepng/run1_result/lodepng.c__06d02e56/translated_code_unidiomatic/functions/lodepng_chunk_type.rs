use libc;
pub unsafe fn lodepng_chunk_type(r#type: *mut libc::c_char, chunk: *const libc::c_uchar) {
    unsafe fn get_chunk_byte(chunk: *const libc::c_uchar, index: usize) -> libc::c_uchar {
        *chunk.add(index)
    }
    let mut i: libc::c_uint = 0;
    while i != 4 {
        *r#type.add(i as usize) = get_chunk_byte(chunk, 4 + i as usize) as libc::c_char;
        i = i.wrapping_add(1);
    }
    *r#type.add(4) = 0;
}

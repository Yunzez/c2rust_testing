pub unsafe fn lodepng_chunk_data_const(chunk: *const libc::c_uchar) -> *const libc::c_uchar {
    chunk.add(8)
}

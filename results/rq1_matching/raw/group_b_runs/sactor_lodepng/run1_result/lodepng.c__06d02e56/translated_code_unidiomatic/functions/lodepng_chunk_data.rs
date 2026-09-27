pub unsafe fn lodepng_chunk_data(chunk: *mut u8) -> *mut u8 {
    chunk.add(8)
}

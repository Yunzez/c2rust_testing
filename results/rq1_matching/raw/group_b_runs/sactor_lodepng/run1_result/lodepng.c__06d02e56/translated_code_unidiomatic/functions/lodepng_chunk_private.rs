pub unsafe fn lodepng_chunk_private(chunk: *const u8) -> u8 {
    if *chunk.add(6) & 32 != 0 {
        1
    } else {
        0
    }
}

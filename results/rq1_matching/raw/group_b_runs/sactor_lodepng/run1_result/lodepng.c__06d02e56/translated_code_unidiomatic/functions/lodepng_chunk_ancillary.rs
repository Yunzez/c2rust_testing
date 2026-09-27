pub unsafe fn lodepng_chunk_ancillary(chunk: *const u8) -> u8 {
    if *chunk.add(4) & 32 != 0 {
        1
    } else {
        0
    }
}

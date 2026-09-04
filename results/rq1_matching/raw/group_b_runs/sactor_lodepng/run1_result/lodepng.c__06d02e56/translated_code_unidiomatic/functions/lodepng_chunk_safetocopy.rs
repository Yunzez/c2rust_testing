pub unsafe fn lodepng_chunk_safetocopy(chunk: *const u8) -> u8 {
    if *chunk.add(7) & 32 != 0 {
        1
    } else {
        0
    }
}

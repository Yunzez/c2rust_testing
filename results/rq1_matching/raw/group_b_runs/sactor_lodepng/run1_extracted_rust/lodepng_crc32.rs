// SACTOR unidiomatic translation of `lodepng_crc32` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:43:28; attempt 1). Verification verdict: Error: Function signature not found in the translated code for function `readBitFromReversedStream`. Got functions: ['read_bit_from_reversed_stream'], check if you have the correct function name., you
pub unsafe fn lodepng_crc32(data: *const u8, length: usize) -> u32 {
    let mut r: u32 = 0xffffffffu32;
    let mut i: usize = 0;
    while i < length {
        let byte = *data.add(i);
        let index = ((r ^ byte as u32) & 0xffu32) as usize;
        r = lodepng_crc32_table[index] ^ (r >> 8);
        i += 1;
    }
    r ^ 0xffffffffu32
}

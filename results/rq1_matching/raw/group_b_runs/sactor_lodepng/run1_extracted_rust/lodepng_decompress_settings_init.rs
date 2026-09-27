// SACTOR unidiomatic translation of `lodepng_decompress_settings_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:43:05; attempt 1). Verification verdict: Error: Function signature not found in the translated code for function `readBitFromReversedStream`. Got functions: ['read_bit_from_reversed_stream'], check if you have the correct function name., you
pub unsafe fn lodepng_decompress_settings_init(
    settings: *mut LodePNGDecompressSettings,
) {
    if settings.is_null() {
        return;
    }
    (*settings).ignore_adler32 = 0;
    (*settings).ignore_nlen = 0;
    (*settings).max_output_size = 0;
    (*settings).custom_zlib = None;
    (*settings).custom_inflate = None;
    (*settings).custom_context = core::ptr::null();
}

pub unsafe fn lodepng_decompress_settings_init(settings: *mut LodePNGDecompressSettings) {
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

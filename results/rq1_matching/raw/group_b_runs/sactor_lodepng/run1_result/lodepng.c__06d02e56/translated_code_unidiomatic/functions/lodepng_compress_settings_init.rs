pub unsafe fn lodepng_compress_settings_init(settings: *mut LodePNGCompressSettings) {
    (*settings).btype = 2;
    (*settings).use_lz77 = 1;
    (*settings).windowsize = 2048;
    (*settings).minmatch = 3;
    (*settings).nicematch = 128;
    (*settings).lazymatching = 1;
    (*settings).custom_zlib = None;
    (*settings).custom_deflate = None;
    (*settings).custom_context = core::ptr::null();
}

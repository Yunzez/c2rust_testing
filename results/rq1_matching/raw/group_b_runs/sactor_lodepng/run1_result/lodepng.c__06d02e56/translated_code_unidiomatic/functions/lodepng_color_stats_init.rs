pub unsafe fn lodepng_color_stats_init(stats: *mut LodePNGColorStats) {
    if stats.is_null() {
        return;
    }
    (*stats).colored = 0;
    (*stats).key = 0;
    (*stats).key_r = 0;
    (*stats).key_g = 0;
    (*stats).key_b = 0;
    (*stats).alpha = 0;
    (*stats).numcolors = 0;
    (*stats).bits = 1;
    (*stats).numpixels = 0;
    (*stats).allow_palette = 1;
    (*stats).allow_greyscale = 1;
}

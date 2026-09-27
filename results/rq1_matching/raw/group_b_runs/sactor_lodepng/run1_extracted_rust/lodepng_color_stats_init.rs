// SACTOR unidiomatic translation of `lodepng_color_stats_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:53:45; attempt 1). Verification verdict: Error: Function signature not found in the translated code for function `getValueRequiredBits`. Got functions: ['get_value_required_bits'], check if you have the correct function name., you should **N
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

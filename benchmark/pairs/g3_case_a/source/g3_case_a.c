/* G3 Case A -- caller-established precondition (input range).
 * scale() overflows for large x (signed UB); its ONLY caller scale_pct() clamps
 * the input to [0,100], so in the real program scale never overflows. Fuzzing
 * scale standalone violates that precondition -> false divergence. Fuzzing
 * scale_pct (the boundary that ESTABLISHES the precondition) is clean. */

int scale(int x) {
    return x * 100;            /* precondition: |x| small; large x -> signed overflow */
}

int scale_pct(int pct) {
    if (pct < 0) pct = 0;
    if (pct > 100) pct = 100;
    return scale(pct);         /* clamped -> scale never overflows here */
}

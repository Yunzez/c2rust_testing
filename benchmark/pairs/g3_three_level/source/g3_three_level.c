/* G3 decisive 3-level case -- the correct STU is the MIDDLE layer, not root, not leaf.
 *
 *   report (root)   -> calls safe_ratio AND does its OWN unguarded y*y*y  => ROOT is a bad boundary
 *   safe_ratio (mid)-> clamps pct to [0,100], then calls scale           => MIDDLE is the ideal STU
 *   scale (leaf)    -> x*100, overflows for large x                      => LEAF false-divergence
 *
 * Fuzzing scale alone violates its precondition (false divergence). Fuzzing report hits its own
 * y*y*y overflow (root too coarse). Only safe_ratio is a valid, covering boundary. */

int scale(int x) {
    return x * 100;                       /* precondition: |x| small; large x -> overflow */
}

int safe_ratio(int pct) {
    if (pct < 0) pct = 0;
    if (pct > 100) pct = 100;             /* establishes scale's precondition */
    return scale(pct);
}

int report(int pct, int y) {
    int a = safe_ratio(pct);              /* this part is fine... */
    int b = y * y * y;                    /* ...but root adds its OWN unguarded overflow */
    return a + b;
}

/*
 * array_transforms.c
 *
 * In-place array transformations operating purely on their buffer arguments:
 * a segment reverse, a left-rotate via three reverses, and a stable
 * even-before-odd partition. All indices are derived from the supplied
 * length and stay strictly within [0, n), so every access is in-bounds.
 *
 * Self-contained; standard libc only. No recursion, no global state.
 *
 * Complexity: 3 functions, index arithmetic, in-place swaps.
 */

#include <stdint.h>
#include <stddef.h>

/* Reverse the elements a[lo..hi-1] in place. Safe for lo >= hi (no-op). */
static void reverse_range(int32_t *a, size_t lo, size_t hi) {
    while (lo + 1 < hi + 1 && lo < hi) {
        size_t j = hi - 1;
        int32_t t = a[lo];
        a[lo] = a[j];
        a[j] = t;
        lo++;
        hi--;
    }
}

/*
 * Left-rotate the array a[0..n-1] by k positions, in place, using the
 * three-reversal trick. k is reduced modulo n first, so any k is valid.
 * Safe for n == 0 (no-op) and a == NULL when n == 0.
 */
// ENTRY: void rotate_left(int32_t *a, size_t n, size_t k)
void rotate_left(int32_t *a, size_t n, size_t k) {
    if (n == 0) {
        return;
    }
    k %= n;
    reverse_range(a, 0, k);
    reverse_range(a, k, n);
    reverse_range(a, 0, n);
}

/*
 * Stable-ish partition: move all even values before all odd values within
 * a[0..n-1]. Uses a write cursor that never exceeds n. Returns the count of
 * even values (the index where the odd section begins).
 */
size_t partition_even_odd(int32_t *a, size_t n) {
    size_t w = 0;
    for (size_t i = 0; i < n; i++) {
        if ((a[i] & 1) == 0) {
            int32_t t = a[i];
            /* shift the [w, i) window right by one to keep order */
            for (size_t j = i; j > w; j--) {
                a[j] = a[j - 1];
            }
            a[w] = t;
            w++;
        }
    }
    return w;
}

/*
 * prefix_runs.c
 *
 * Streaming-style scans over a byte buffer: an exclusive prefix-sum written
 * to an out-buffer, a run-length encoder with a capacity-checked output, and
 * a simple additive rolling checksum. Every write to the output buffer is
 * guarded against its capacity, so no out-of-bounds store can occur.
 *
 * Self-contained; standard libc only. No recursion, no global state.
 *
 * Complexity: 3 functions, dual-cursor RLE, bounded output.
 */

#include <stdint.h>
#include <stddef.h>

/*
 * Write an exclusive prefix sum of `src[0..n-1]` into `dst[0..n-1]`.
 * dst[0] = 0, dst[i] = src[0]+...+src[i-1], accumulated in 64-bit to avoid
 * overflow UB. Caller guarantees dst has room for n elements. Safe for n==0.
 */
// ENTRY: void prefix_sum_excl(const uint32_t *src, size_t n, uint64_t *dst)
void prefix_sum_excl(const uint32_t *src, size_t n, uint64_t *dst) {
    uint64_t acc = 0;
    for (size_t i = 0; i < n; i++) {
        dst[i] = acc;
        acc += (uint64_t)src[i];
    }
}

/*
 * Run-length encode `src[0..n-1]` into `out` (capacity `cap` bytes) as
 * (count, value) byte pairs, where count is capped at 255 so a longer run
 * splits into multiple pairs. Returns the number of bytes written, or
 * (size_t)-1 if the output capacity is insufficient.
 */
size_t rle_encode(const uint8_t *src, size_t n, uint8_t *out, size_t cap) {
    size_t w = 0;
    size_t i = 0;
    while (i < n) {
        uint8_t v = src[i];
        size_t run = 1;
        while (i + run < n && src[i + run] == v && run < 255) {
            run++;
        }
        if (w + 2 > cap) {
            return (size_t)-1;
        }
        out[w++] = (uint8_t)run;
        out[w++] = v;
        i += run;
    }
    return w;
}

/* Additive rolling checksum (Adler-like, modulo 65521) over n bytes. */
uint32_t rolling_checksum(const uint8_t *src, size_t n) {
    uint32_t a = 1;
    uint32_t b = 0;
    for (size_t i = 0; i < n; i++) {
        a = (a + src[i]) % 65521u;
        b = (b + a) % 65521u;
    }
    return (b << 16) | a;
}

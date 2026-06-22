/*
 * rle_codec.c
 * Simple run-length encoder for byte buffers.
 *
 * Encoding format: a sequence of (count, byte) pairs where count is a
 * single byte in the range 1..255. Runs longer than 255 are split into
 * multiple pairs. The encoded output is therefore always an even number
 * of bytes.
 *
 * Complexity: simple. Loops, no recursion, light pointer use.
 */

#include <stddef.h>
#include <stdint.h>

/* Encode a single maximal run starting at src[0]. Returns the number of
 * input bytes consumed; writes one (count,byte) pair to out. */
static size_t encode_run(const uint8_t *src, size_t avail, uint8_t out[2]) {
    uint8_t value = src[0];
    size_t run = 1;
    while (run < avail && run < 255 && src[run] == value) {
        run++;
    }
    out[0] = (uint8_t)run;
    out[1] = value;
    return run;
}

/*
 * ENTRY: size_t rle_encode(const uint8_t *src, size_t len, uint8_t *dst, size_t dst_cap)
 *
 * Run-length encode `len` bytes from `src` into `dst` (capacity dst_cap).
 * Returns the number of bytes written to dst, or 0 if dst_cap is too small
 * (also 0 for empty input).
 */
size_t rle_encode(const uint8_t *src, size_t len, uint8_t *dst, size_t dst_cap) {
    size_t in = 0;
    size_t out = 0;
    if (src == NULL || dst == NULL) {
        return 0;
    }
    while (in < len) {
        uint8_t pair[2];
        size_t consumed = encode_run(src + in, len - in, pair);
        if (out + 2 > dst_cap) {
            return 0; /* not enough room */
        }
        dst[out++] = pair[0];
        dst[out++] = pair[1];
        in += consumed;
    }
    return out;
}

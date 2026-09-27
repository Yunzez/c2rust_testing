/*
 * hex_encode.c
 * Lowercase base16 (hex) encoder into a caller-provided output buffer.
 *
 * Total and in-bounds: writes exactly 2 hex chars per input byte and refuses
 * to write when the output capacity is insufficient.
 *
 * Complexity: simple. Loop, no recursion.
 */

#include <stddef.h>
#include <stdint.h>

/* Map a 0..15 nibble to its lowercase hex character. */
static uint8_t nibble_to_hex(uint8_t n) {
    static const char digits[] = "0123456789abcdef";
    return (uint8_t)digits[n & 0x0F];
}

/*
 * ENTRY: size_t hex_encode(const uint8_t *src, size_t len, uint8_t *dst, size_t dst_cap)
 *
 * Hex-encode `len` bytes from `src` into `dst` (capacity dst_cap). Each input
 * byte becomes two lowercase hex characters. Returns the number of bytes
 * written, or 0 if dst_cap is too small (also 0 for NULL args).
 */
size_t hex_encode(const uint8_t *src, size_t len, uint8_t *dst, size_t dst_cap) {
    size_t i;
    size_t out = 0;
    if (src == NULL || dst == NULL) {
        return 0;
    }
    if (len > dst_cap / 2) {
        return 0; /* would not fit */
    }
    for (i = 0; i < len; i++) {
        uint8_t b = src[i];
        dst[out++] = nibble_to_hex((uint8_t)(b >> 4));
        dst[out++] = nibble_to_hex((uint8_t)(b & 0x0F));
    }
    return out;
}

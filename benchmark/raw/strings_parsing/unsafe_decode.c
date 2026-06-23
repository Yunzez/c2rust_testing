/*
 * unsafe_decode.c
 * Deliberately UNSAFE parsers that TRUST their inputs. Used as negative
 * (INVALID) boundaries: adversarial inputs trigger out-of-bounds reads.
 *
 * Complexity: simple, but intentionally unguarded.
 */

#include <stddef.h>
#include <stdint.h>

/*
 * ENTRY: uint8_t char_at(const char *s, size_t pos)
 *
 * Returns the byte at s[pos]. TRUSTS that pos < strlen(s); it does NOT verify
 * pos against the string length, so a large `pos` reads out of bounds (UB).
 */
uint8_t char_at(const char *s, size_t pos) {
    return (uint8_t)s[pos]; /* unchecked: out-of-bounds read for large pos */
}

/*
 * ENTRY: uint32_t sum_declared(const uint8_t *data, size_t len)
 *
 * Treats data[0] as a declared payload length N and sums the next N bytes.
 * TRUSTS that data[0] <= len - 1; a payload byte claiming more than is
 * available reads past the buffer (UB). `len` is only used to read the header.
 */
uint32_t sum_declared(const uint8_t *data, size_t len) {
    uint32_t sum = 0;
    size_t n;
    size_t i;
    (void)len;
    n = (size_t)data[0];          /* trusted, embedded length field */
    for (i = 0; i < n; i++) {
        sum += (uint32_t)data[1 + i]; /* reads beyond buffer if n too large */
    }
    return sum;
}

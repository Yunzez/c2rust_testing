/*
 * byte_classify.c
 * Byte/character classification and counting over a length-prefixed buffer.
 *
 * All functions are total and in-bounds: they only read the `len` bytes the
 * caller declares and never index past them.
 *
 * Complexity: simple. Loops, no recursion.
 */

#include <stddef.h>
#include <stdint.h>

/* Return 1 if b is an ASCII decimal digit, else 0. */
static int is_digit(uint8_t b) {
    return (b >= (uint8_t)'0' && b <= (uint8_t)'9') ? 1 : 0;
}

/* Return 1 if b is an ASCII whitespace byte, else 0. */
static int is_space(uint8_t b) {
    return (b == (uint8_t)' ' || b == (uint8_t)'\t' ||
            b == (uint8_t)'\n' || b == (uint8_t)'\r' ||
            b == (uint8_t)'\f' || b == (uint8_t)'\v') ? 1 : 0;
}

/*
 * ENTRY: uint32_t count_class(const uint8_t *buf, size_t len, uint8_t class_id)
 *
 * Count bytes in `buf[0..len)` belonging to a class:
 *   0 = decimal digits, 1 = whitespace, 2 = ASCII letters, anything else = 0.
 * Returns the count. NULL or empty buffer yields 0.
 */
uint32_t count_class(const uint8_t *buf, size_t len, uint8_t class_id) {
    uint32_t n = 0;
    size_t i;
    if (buf == NULL) {
        return 0;
    }
    for (i = 0; i < len; i++) {
        uint8_t b = buf[i];
        int hit = 0;
        if (class_id == 0) {
            hit = is_digit(b);
        } else if (class_id == 1) {
            hit = is_space(b);
        } else if (class_id == 2) {
            hit = ((b >= (uint8_t)'A' && b <= (uint8_t)'Z') ||
                   (b >= (uint8_t)'a' && b <= (uint8_t)'z')) ? 1 : 0;
        }
        if (hit) {
            n++;
        }
    }
    return n;
}

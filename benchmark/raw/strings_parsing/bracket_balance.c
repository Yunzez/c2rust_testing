/*
 * bracket_balance.c
 * Balanced-bracket validator and FNV-1a checksum over a length-prefixed buffer.
 *
 * Both functions are total and in-bounds.
 *
 * Complexity: simple. Loop with a small fixed-size stack, no recursion.
 */

#include <stddef.h>
#include <stdint.h>

/* Return the matching opener for a closing bracket, or 0 if not a closer. */
static uint8_t opener_for(uint8_t c) {
    if (c == (uint8_t)')') return (uint8_t)'(';
    if (c == (uint8_t)']') return (uint8_t)'[';
    if (c == (uint8_t)'}') return (uint8_t)'{';
    return 0;
}

/*
 * ENTRY: int brackets_balanced(const uint8_t *buf, size_t len)
 *
 * Return 1 if all (), [], {} in buf[0..len) are properly nested and matched,
 * else 0. Non-bracket bytes are ignored. NULL buffer is treated as balanced
 * (returns 1). Uses a bounded stack; nesting deeper than the stack returns 0.
 */
int brackets_balanced(const uint8_t *buf, size_t len) {
    uint8_t stack[256];
    size_t sp = 0;
    size_t i;
    if (buf == NULL) {
        return 1;
    }
    for (i = 0; i < len; i++) {
        uint8_t c = buf[i];
        if (c == (uint8_t)'(' || c == (uint8_t)'[' || c == (uint8_t)'{') {
            if (sp >= sizeof(stack)) {
                return 0; /* too deep */
            }
            stack[sp++] = c;
        } else {
            uint8_t want = opener_for(c);
            if (want != 0) {
                if (sp == 0 || stack[sp - 1] != want) {
                    return 0;
                }
                sp--;
            }
        }
    }
    return (sp == 0) ? 1 : 0;
}

/*
 * ENTRY: uint32_t fnv1a_checksum(const uint8_t *buf, size_t len)
 *
 * 32-bit FNV-1a hash over buf[0..len). NULL buffer hashes as empty.
 */
uint32_t fnv1a_checksum(const uint8_t *buf, size_t len) {
    uint32_t h = 2166136261u;
    size_t i;
    if (buf == NULL) {
        return h;
    }
    for (i = 0; i < len; i++) {
        h ^= (uint32_t)buf[i];
        h *= 16777619u;
    }
    return h;
}

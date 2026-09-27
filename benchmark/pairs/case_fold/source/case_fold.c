/*
 * case_fold.c
 * In-place ASCII case folding and a NUL-string CSV field counter.
 *
 * Both functions are total and in-bounds.
 *
 * Complexity: simple. Loops, no recursion.
 */

#include <stddef.h>
#include <stdint.h>

/*
 * ENTRY: size_t to_upper_inplace(char *buf, size_t len)
 *
 * Uppercase ASCII lowercase letters in buf[0..len) in place. Other bytes are
 * left unchanged. Returns the number of bytes that were modified. NULL is a
 * no-op returning 0.
 */
size_t to_upper_inplace(char *buf, size_t len) {
    size_t i;
    size_t changed = 0;
    if (buf == NULL) {
        return 0;
    }
    for (i = 0; i < len; i++) {
        uint8_t b = (uint8_t)buf[i];
        if (b >= (uint8_t)'a' && b <= (uint8_t)'z') {
            buf[i] = (char)(b - 32);
            changed++;
        }
    }
    return changed;
}

/*
 * ENTRY: uint32_t csv_field_count(const char *s, char delim)
 *
 * Count comma-style fields in a NUL-terminated string `s`, where fields are
 * separated by `delim`. An empty string has 1 field (the empty field). Reads
 * only up to and including the terminating NUL. NULL returns 0.
 */
uint32_t csv_field_count(const char *s, char delim) {
    uint32_t fields = 1;
    size_t i = 0;
    if (s == NULL) {
        return 0;
    }
    while (s[i] != '\0') {
        if (s[i] == delim) {
            fields++;
        }
        i++;
    }
    return fields;
}

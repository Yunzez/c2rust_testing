/*
 * leb128.c - unsigned LEB128 (varint) encoder / decoder
 *
 * Theme: numeric & bit manipulation (loops + helpers, pointer-heavy).
 * Encodes a uint64_t into the variable-length LEB128 byte form and
 * decodes it back, with strict bounds checking so no out-of-range
 * read/write occurs.
 * Standard libc only. No I/O, no randomness, deterministic.
 */

#include <stdint.h>
#include <stddef.h>

/* Encode value as unsigned LEB128 into out (capacity cap bytes).
 * Returns the number of bytes written, or 0 if the buffer is too small
 * (a uint64_t needs at most 10 bytes). Never writes past cap. */
static size_t leb128_encode(uint64_t value, uint8_t *out, size_t cap)
{
    size_t n = 0;
    do {
        uint8_t byte = (uint8_t)(value & 0x7Fu);
        value >>= 7;
        if (value != 0)
            byte |= 0x80u; /* continuation bit */
        if (n >= cap)
            return 0; /* would overflow buffer */
        out[n++] = byte;
    } while (value != 0);
    return n;
}

/* Decode an unsigned LEB128 value from in[0..len).
 * On success returns the number of bytes consumed and stores the value
 * in *value_out. Returns 0 on failure: truncated input (continuation
 * bit set at end of buffer) or overflow of uint64_t (more than 10
 * groups, or the 10th group setting bits above 64). */
static size_t leb128_decode(const uint8_t *in, size_t len, uint64_t *value_out)
{
    uint64_t result = 0;
    unsigned shift = 0;
    size_t i = 0;

    while (i < len) {
        uint8_t byte = in[i];
        uint64_t low7 = (uint64_t)(byte & 0x7Fu);

        if (shift >= 64)
            return 0; /* too many bytes -> overflow */
        if (shift == 63 && low7 > 1u)
            return 0; /* would set bits above bit 63 */

        result |= low7 << shift;
        i++;

        if ((byte & 0x80u) == 0) {
            *value_out = result;
            return i;
        }
        shift += 7;
    }
    return 0; /* ran off the end with continuation bit set */
}

/* ENTRY: int leb128_roundtrip(const uint8_t *in, size_t len, uint8_t *out, size_t out_cap)
 *
 * Differential fuzz target. Decodes one unsigned LEB128 value from the
 * input buffer, then re-encodes it into out. Returns the number of bytes
 * written to out on success, or -1 on any decode/encode failure.
 *
 * Pointer/bounds safety:
 *   - if in == NULL || out == NULL, returns -1 with no dereference.
 *   - decode never reads past len; encode never writes past out_cap.
 * For a canonically-encoded input, the re-encoded output equals the
 * consumed prefix of the input (useful as a differential invariant). */
int leb128_roundtrip(const uint8_t *in, size_t len, uint8_t *out, size_t out_cap)
{
    uint64_t value;
    size_t consumed;
    size_t written;

    if (in == NULL || out == NULL)
        return -1;

    consumed = leb128_decode(in, len, &value);
    if (consumed == 0)
        return -1;

    written = leb128_encode(value, out, out_cap);
    if (written == 0)
        return -1;

    return (int)written;
}

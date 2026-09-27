/*
 * gap_buffer.c - fixed-capacity gap buffer / text cursor (POD struct).
 *
 * Theme: bounded containers. A struct {char text[N]; size_t gap_start, gap_end}
 * models an editor buffer: live text is everything outside [gap_start, gap_end).
 * Accessors TRUST gap_start <= gap_end <= N. A differential harness that builds
 * the struct field-by-field from random bytes violates that invariant ->
 * out-of-bounds reads = INVALID boundary. Distinct structural source (a cursor
 * gap, not a stack pointer).
 *
 * Accessors take a single `const GapBuf *` (no trailing size_t). Standard libc
 * only, deterministic.
 */

#include <stdint.h>
#include <stddef.h>

#define GB_CAP 32

typedef struct {
    char text[GB_CAP]; /* storage; the gap region holds no live text */
    size_t gap_start;  /* first index of the gap, trusted <= gap_end */
    size_t gap_end;    /* one past the gap, trusted <= GB_CAP */
} GapBuf;

/* ENTRY: char gb_char_before_gap(const GapBuf *g)
 * Returns the live character immediately before the gap. Trusts gap_start in
 * [1, GB_CAP] and reads text[gap_start-1] with NO bounds check, so a random
 * gap_start reads out of bounds -> an INVALID (negative) boundary. */
// ENTRY: char gb_char_before_gap(const GapBuf *g)
char gb_char_before_gap(const GapBuf *g)
{
    return g->text[g->gap_start - 1]; /* trusts 1 <= gap_start <= GB_CAP */
}

/* Returns the first live character after the gap. Trusts gap_end < GB_CAP ->
 * the same out-of-bounds exposure when gap_end is large. */
char gb_char_after_gap(const GapBuf *g)
{
    return g->text[g->gap_end]; /* trusts gap_end < GB_CAP */
}

/* Masked checksum of the live (non-gap) characters. gap_start/gap_end are
 * clamped so every read stays in bounds regardless of the fields -> total and
 * safe = a VALID (positive) boundary on the same struct. */
int64_t gb_live_checksum(const GapBuf *g)
{
    int64_t acc = 0;
    size_t gs = g->gap_start % (GB_CAP + 1); /* clamp to [0, GB_CAP] */
    size_t ge = g->gap_end % (GB_CAP + 1);   /* clamp to [0, GB_CAP] */
    if (ge < gs)
        ge = gs; /* normalize so [gs, ge) is a valid (possibly empty) gap */
    for (size_t i = 0; i < GB_CAP; i++) {
        if (i >= gs && i < ge)
            continue; /* skip the gap region */
        acc += (int64_t)(unsigned char)g->text[i];
    }
    return acc;
}

/*
 * histogram.c - fixed-capacity histogram / bitset bins (POD struct).
 *
 * Theme: bounded containers. A struct {uint32 bins[N]; size_t nbins} whose
 * accessors TRUST nbins <= N. A differential harness that builds the struct
 * field-by-field from random bytes violates that invariant -> out-of-bounds
 * reads = INVALID boundary. Distinct structural source (a counted histogram,
 * not a stack pointer).
 *
 * Accessors take a single `const Histogram *` (no trailing size_t). Standard
 * libc only, deterministic.
 */

#include <stdint.h>
#include <stddef.h>

#define HG_BINS 16

typedef struct {
    uint32_t bins[HG_BINS]; /* per-bin counts */
    size_t nbins;           /* number of active bins, trusted <= HG_BINS */
} Histogram;

/* ENTRY: uint64_t hg_total_trusting(const Histogram *h)
 * Sums the active bins. Trusts nbins <= HG_BINS and iterates [0, nbins) with NO
 * bounds check, so a random nbins reads past the array -> an INVALID (negative)
 * boundary. */
// ENTRY: uint64_t hg_total_trusting(const Histogram *h)
uint64_t hg_total_trusting(const Histogram *h)
{
    uint64_t acc = 0;
    for (size_t i = 0; i < h->nbins; i++) /* trusts nbins <= HG_BINS */
        acc += h->bins[i];
    return acc;
}

/* Returns the value of the last active bin. Trusts nbins in [1, HG_BINS] and
 * reads bins[nbins-1] -> the same out-of-bounds exposure. */
uint32_t hg_last_bin(const Histogram *h)
{
    return h->bins[h->nbins - 1]; /* trusts 1 <= nbins <= HG_BINS */
}

/* Masked total: nbins is clamped to [0, HG_BINS] so the loop never runs past
 * the array regardless of the field -> total and safe = a VALID (positive)
 * boundary on the same struct. */
uint64_t hg_total_masked(const Histogram *h)
{
    uint64_t acc = 0;
    size_t active = h->nbins % (HG_BINS + 1); /* clamp to [0, HG_BINS] */
    for (size_t i = 0; i < active; i++)
        acc += h->bins[i];
    return acc;
}

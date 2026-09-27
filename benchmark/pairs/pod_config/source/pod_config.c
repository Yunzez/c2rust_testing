/*
 * pod_config.c - purely-VALID POD-struct transforms (no trusted-index hazard).
 *
 * Theme: bounded containers. Two small POD structs with fixed-array fields, and
 * total accessors that iterate the WHOLE fixed array (no internal index/count is
 * trusted), so every read is in bounds for ANY field values -> VALID (positive)
 * boundaries. These broaden the positive set beyond the masked siblings.
 *
 * Accessors take a single `const Foo *` (no trailing size_t). Standard libc
 * only, deterministic.
 */

#include <stdint.h>
#include <stddef.h>

#define CFG_CHANNELS 8

typedef struct {
    int32_t gains[CFG_CHANNELS]; /* per-channel gain */
    int32_t offset;              /* applied to every channel */
} Config;

typedef struct {
    int32_t lo[CFG_CHANNELS]; /* lower bounds */
    int32_t hi[CFG_CHANNELS]; /* upper bounds */
} Range;

/* ENTRY: int64_t cfg_total_gain(const Config *c)
 * Sums all channel gains plus the per-channel offset. Iterates the full fixed
 * array, so it is total and in bounds for any input -> a VALID boundary. */
// ENTRY: int64_t cfg_total_gain(const Config *c)
int64_t cfg_total_gain(const Config *c)
{
    int64_t acc = 0;
    for (size_t i = 0; i < CFG_CHANNELS; i++)
        acc += (int64_t)c->gains[i] + (int64_t)c->offset;
    return acc;
}

/* Counts how many channels are inverted (hi < lo). Iterates the full fixed
 * arrays -> total and in bounds for any input = a VALID boundary. */
int32_t range_count_inverted(const Range *r)
{
    int32_t n = 0;
    for (size_t i = 0; i < CFG_CHANNELS; i++) {
        if (r->hi[i] < r->lo[i])
            n++;
    }
    return n;
}

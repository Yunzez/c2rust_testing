/*
 * bounded_queue.c - fixed-capacity FIFO queue (POD struct with trusted invariant).
 *
 * Theme: bounded containers. A struct {int32 data[CAP]; size_t head, len} whose
 * accessors TRUST head < CAP and len <= CAP. A differential harness that builds
 * the struct field-by-field from random bytes violates that invariant ->
 * out-of-bounds reads = INVALID boundary. Distinct structural source from the
 * VM stack pointer (a circular FIFO).
 *
 * Accessors take a single `const BQueue *` (no trailing size_t). Standard libc
 * only, deterministic.
 */

#include <stdint.h>
#include <stddef.h>

#define BQ_CAP 16

typedef struct {
    int32_t data[BQ_CAP]; /* circular storage */
    size_t head;          /* index of the front element, trusted < BQ_CAP */
    size_t len;           /* number of live elements, trusted <= BQ_CAP */
} BQueue;

/* ENTRY: int32_t bq_front(const BQueue *q)
 * Returns the front live element. Trusts head < BQ_CAP and reads data[head]
 * with NO bounds check, so a random head reads out of bounds -> an INVALID
 * (negative) boundary. */
// ENTRY: int32_t bq_front(const BQueue *q)
int32_t bq_front(const BQueue *q)
{
    return q->data[q->head]; /* trusts head < BQ_CAP */
}

/* Returns the back live element at (head + len - 1). Trusts both head and len
 * so the index can run off the end -> the same out-of-bounds exposure. */
int32_t bq_back_trusting(const BQueue *q)
{
    return q->data[q->head + q->len - 1]; /* trusts head + len - 1 < BQ_CAP */
}

/* Masked sum of the live elements. head and len are clamped and every access
 * is wrapped with % BQ_CAP, so reads stay in bounds regardless of the fields ->
 * total and safe = a VALID (positive) boundary on the same struct. */
int64_t bq_sum_masked(const BQueue *q)
{
    int64_t acc = 0;
    size_t live = q->len % (BQ_CAP + 1);  /* clamp to [0, BQ_CAP] */
    size_t h = q->head % BQ_CAP;          /* clamp to [0, BQ_CAP) */
    for (size_t i = 0; i < live; i++)
        acc += (int64_t)q->data[(h + i) % BQ_CAP];
    return acc;
}

/*
 * bounded_stack.c - fixed-capacity LIFO stack (POD struct with trusted invariant).
 *
 * Theme: bounded containers. A struct {int32 data[CAP]; size_t top} whose
 * peek/pop accessors TRUST `top` to lie within [0, CAP]. A differential harness
 * that builds the struct field-by-field from random bytes violates that
 * invariant -> out-of-bounds reads = INVALID boundary. This is the
 * input-contract / isolation-invariant negative class, but from a stack-of-data
 * (not a VM stack pointer).
 *
 * Each accessor takes a single `const BStack *` / `BStack *` with NO trailing
 * size_t, so it stays a single-struct boundary. Standard libc only,
 * deterministic.
 */

#include <stdint.h>
#include <stddef.h>

#define BS_CAP 16

typedef struct {
    int32_t data[BS_CAP]; /* element storage */
    size_t top;           /* number of live elements, trusted <= BS_CAP */
} BStack;

/* ENTRY: int32_t bs_peek(const BStack *s)
 * Returns the top live element. Trusts top in [1, BS_CAP] and reads
 * data[top-1] with NO bounds check, so a random top reads out of bounds ->
 * an INVALID (negative) boundary. */
// ENTRY: int32_t bs_peek(const BStack *s)
int32_t bs_peek(const BStack *s)
{
    return s->data[s->top - 1]; /* trusts 1 <= top <= BS_CAP */
}

/* Pop the top element into *out, decrementing top. Trusts top > 0 and
 * top <= BS_CAP -> the same out-of-bounds exposure as bs_peek. */
int32_t bs_pop_trusting(BStack *s, int32_t *out)
{
    s->top -= 1;             /* trusts top > 0 */
    *out = s->data[s->top];  /* trusts top < BS_CAP after decrement */
    return *out;
}

/* Masked sum of the live elements. `top` is clamped to [0, BS_CAP] and every
 * access stays in bounds regardless of the field -> total and safe = a VALID
 * (positive) boundary on the same struct. */
int64_t bs_sum_masked(const BStack *s)
{
    int64_t acc = 0;
    size_t live = s->top % (BS_CAP + 1); /* clamp to [0, BS_CAP] */
    for (size_t i = 0; i < live; i++)
        acc += (int64_t)s->data[i % BS_CAP];
    return acc;
}

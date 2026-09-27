/*
 * ring_buffer.c - fixed-capacity ring buffer (POD struct with trusted invariants).
 *
 * Theme: data structures. A struct {int32 buf[CAP]; size_t head, tail, count}
 * whose accessors TRUST head/count to lie within [0, CAP]. A differential
 * harness that builds the struct field-by-field from random bytes violates that
 * invariant -> out-of-bounds reads. This is the "input-contract / isolation
 * invariant" negative class, but from a DIFFERENT structural source than the VM
 * stack (a ring buffer, not a stack pointer). Standard libc only, deterministic.
 *
 * Note: accessors take a single `const RingBuf *` (no trailing size_t), so they
 * stay single-struct boundaries and do not collide with struct-array inference.
 */

#include <stdint.h>
#include <stddef.h>

#define RB_CAP 16

typedef struct {
    int32_t buf[RB_CAP];
    size_t head;  /* index of the oldest live element, trusted < RB_CAP */
    size_t tail;  /* index of the next free slot, trusted < RB_CAP */
    size_t count; /* number of live elements, trusted <= RB_CAP */
} RingBuf;

/* ENTRY: int32_t rb_first(const RingBuf *rb)
 * Returns the oldest live element. Trusts head < RB_CAP and is NOT bounds
 * checked, so a random head reads out of bounds -> a negative boundary
 * (isolation invariant), distinct from the VM stack-pointer case. */
int32_t rb_first(const RingBuf *rb)
{
    return rb->buf[rb->head]; /* trusts head < RB_CAP */
}

/* Sum of the live elements, but every access is masked with % RB_CAP, so the
 * read index is always in bounds regardless of head/count. Total and safe ->
 * a valid (positive) boundary even though the input is the same struct. */
int64_t rb_sum_masked(const RingBuf *rb)
{
    int64_t acc = 0;
    size_t live = rb->count % (RB_CAP + 1); /* clamp to [0, RB_CAP] */
    for (size_t i = 0; i < live; i++)
        acc += (int64_t)rb->buf[(rb->head + i) % RB_CAP];
    return acc;
}

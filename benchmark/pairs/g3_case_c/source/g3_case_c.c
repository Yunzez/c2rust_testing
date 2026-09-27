/* G3 Case C -- isolation invariant (struct cursor).
 * ring_at() reads buf[head] trusting head < CAP (an invariant the caller establishes).
 * Fuzzing ring_at standalone builds a Ring with a garbage head -> out-of-bounds read =
 * FALSE divergence (the invariant is never violated in the real program). ring_get()
 * ESTABLISHES the invariant (head %= CAP) before calling ring_at -> clean.
 * The "shield" here is a mask (% CAP), NOT a constant clamp -> rf_input_clamp will not
 * fire, so selector v2 is expected to NOT rise (a measured limitation, per architecture). */

#include <stdint.h>
#include <stddef.h>

#define CAP 16

typedef struct {
    int32_t buf[CAP];
    size_t  head;      /* trusted < CAP */
} Ring;

int32_t ring_at(const Ring *r) {
    return r->buf[r->head];        /* unmasked field index -> RISKY; trusts head < CAP */
}

int32_t ring_get(Ring *r) {
    r->head %= CAP;                /* establish the invariant head < CAP */
    return ring_at(r);
}

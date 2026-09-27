/*
 * sub_overflow.c - signed subtraction near the int32 boundaries
 *
 * Theme: arithmetic / UB variety.
 * Contrasts a signed subtraction that overflows on adversarial inputs
 * (UB) against an unsigned subtraction with defined wraparound (VALID).
 * Standard libc only. No I/O, no randomness, deterministic.
 */

#include <stdint.h>

/* VALID sibling: unsigned subtraction wraps modulo 2^32 by definition.
 * Total over the whole uint32_t x uint32_t domain. */
static uint32_t sub_wrap_u32(uint32_t a, uint32_t b)
{
    return a - b; /* defined unsigned wraparound */
}

/* ENTRY: int32_t sub_signed_i32(int32_t a, int32_t b)
 *
 * INVALID: signed subtraction. For e.g. a == INT32_MIN, b == 1 the true
 * result INT32_MIN-1 is not representable, so a - b is signed overflow (UB).
 * Likewise a == INT32_MAX, b == -1. On non-adversarial inputs it is the
 * ordinary difference. */
int32_t sub_signed_i32(int32_t a, int32_t b)
{
    return a - b; /* may overflow (UB) near INT32_MIN/INT32_MAX */
}

/* VALID: guarded difference that detects the overflow conditions and
 * clamps instead of executing the UB subtraction. Total. */
static int32_t sub_signed_sat(int32_t a, int32_t b)
{
    if (b < 0 && a > INT32_MAX + b)
        return INT32_MAX;
    if (b > 0 && a < INT32_MIN + b)
        return INT32_MIN;
    return a - b;
}

/* Helper exercising the valid siblings so they are not dead code. */
int32_t sub_overflow_safe(int32_t a, int32_t b)
{
    uint32_t w = sub_wrap_u32((uint32_t)a, (uint32_t)b);
    return sub_signed_sat(a, b) ^ (int32_t)w;
}

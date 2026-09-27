/*
 * negate_abs.c - signed negation and absolute value at INT_MIN
 *
 * Theme: arithmetic / UB variety.
 * The classic UB: negating INT32_MIN (and abs(INT32_MIN)) overflows the
 * signed range. Provides UB-exhibiting functions plus VALID unsigned /
 * guarded siblings. Standard libc only. Deterministic, no I/O.
 */

#include <stdint.h>

/* ENTRY: int32_t negate_i32(int32_t x)
 *
 * INVALID: -x. For x == INT32_MIN the magnitude 2147483648 is not
 * representable as int32_t, so the negation is signed overflow (UB).
 * For every other x it is the ordinary additive inverse. */
int32_t negate_i32(int32_t x)
{
    return -x; /* UB when x == INT32_MIN */
}

/* INVALID: textbook absolute value via branch. The x == INT32_MIN branch
 * performs -INT32_MIN which is signed overflow (UB). */
int32_t abs_i32(int32_t x)
{
    if (x < 0)
        return -x; /* UB when x == INT32_MIN */
    return x;
}

/* VALID: negation through unsigned arithmetic, then reinterpret. The
 * unsigned negate is defined for all inputs; result for INT32_MIN is
 * INT32_MIN itself (well-defined two's-complement round-trip here). */
static int32_t negate_u_safe(int32_t x)
{
    uint32_t u = (uint32_t)x;
    uint32_t neg = (uint32_t)(~u + 1u); /* defined unsigned wraparound */
    return (int32_t)neg;
}

/* VALID: absolute magnitude as an unsigned value, total over int32_t.
 * abs(INT32_MIN) == 2147483648 fits in uint32_t with no overflow. */
static uint32_t abs_magnitude_u32(int32_t x)
{
    if (x < 0)
        return (uint32_t)0 - (uint32_t)x; /* defined unsigned wraparound */
    return (uint32_t)x;
}

/* Helper exercising the valid siblings so they are not dead code. */
uint32_t negate_abs_safe(int32_t x)
{
    return (uint32_t)negate_u_safe(x) + abs_magnitude_u32(x);
}

/*
 * reduce_overflow.c - signed overflow inside buffer reductions
 *
 * Theme: arithmetic / UB variety.
 * A running signed sum or product over a buffer can overflow int32_t on
 * adversarial element values (UB). Provides UB-exhibiting reductions plus
 * VALID unsigned-accumulator siblings. Standard libc only. No I/O.
 */

#include <stdint.h>
#include <stddef.h>

/* ENTRY: int32_t reduce_sum_i32(const int32_t *a, size_t n)
 *
 * INVALID: accumulates a signed running total. If the partial sum leaves
 * the int32_t range (e.g. many large positive elements) the addition is
 * signed overflow (UB). For small/balanced buffers it is the plain sum.
 * An empty buffer (n == 0) sums to 0. */
int32_t reduce_sum_i32(const int32_t *a, size_t n)
{
    int32_t acc = 0;
    size_t i;
    for (i = 0; i < n; i++)
        acc += a[i]; /* may overflow (UB) */
    return acc;
}

/* INVALID: running signed product; overflows quickly on adversarial data. */
int32_t reduce_prod_i32(const int32_t *a, size_t n)
{
    int32_t acc = 1;
    size_t i;
    for (i = 0; i < n; i++)
        acc *= a[i]; /* may overflow (UB) */
    return acc;
}

/* VALID: the same sum carried out in an unsigned accumulator, where every
 * addition has defined wraparound. The reinterpreted result matches the
 * two's-complement total without ever invoking UB. */
static int32_t reduce_sum_wrap(const int32_t *a, size_t n)
{
    uint32_t acc = 0;
    size_t i;
    for (i = 0; i < n; i++)
        acc += (uint32_t)a[i]; /* defined unsigned wraparound */
    return (int32_t)acc;
}

/* VALID: unsigned running product, also total over any buffer. */
static int32_t reduce_prod_wrap(const int32_t *a, size_t n)
{
    uint32_t acc = 1;
    size_t i;
    for (i = 0; i < n; i++)
        acc *= (uint32_t)a[i]; /* defined unsigned wraparound */
    return (int32_t)acc;
}

/* Helper exercising the valid siblings so they are not dead code. */
int32_t reduce_overflow_safe(const int32_t *a, size_t n)
{
    return reduce_sum_wrap(a, n) ^ reduce_prod_wrap(a, n);
}

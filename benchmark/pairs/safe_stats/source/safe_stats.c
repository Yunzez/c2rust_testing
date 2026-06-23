/*
 * safe_stats.c - integer reductions over arrays + scalar arithmetic.
 *
 * Theme: numeric & bit manipulation. Deliberately mixes valid boundaries with
 * two DISTINCT undefined-behaviour boundaries (signed add overflow vs integer
 * division), to diversify the negative class beyond the existing multiply/stack
 * overflow cases. Standard libc only, deterministic, no I/O.
 */

#include <stdint.h>
#include <stddef.h>

/* ENTRY: int32_t sum_i32(const int32_t *a, size_t n)
 * Sums signed 32-bit values. `acc += a[i]` overflows int32 on large inputs ->
 * signed-overflow UB (c2rust copies `+` verbatim; Rust panics with overflow
 * checks). A negative boundary of the "intrinsic arithmetic UB" class. */
int32_t sum_i32(const int32_t *a, size_t n)
{
    int32_t acc = 0;
    for (size_t i = 0; i < n; i++)
        acc += a[i]; /* signed overflow UB */
    return acc;
}

/* Integer division: division by zero AND INT_MIN / -1 are both UB (a different
 * UB mechanism than overflow-by-arithmetic). A negative boundary. */
int32_t idiv(int32_t a, int32_t b)
{
    return a / b; /* b == 0 or INT_MIN/-1 -> UB */
}

/* Unsigned FNV-1a fold: multiply/xor wrap by definition (no UB). The buffer is
 * length-prefixed and read in bounds. A valid (positive) boundary. */
uint64_t xorshift_fold(const uint8_t *data, size_t n)
{
    uint64_t h = 1469598103934665603ULL;
    for (size_t i = 0; i < n; i++) {
        h ^= (uint64_t)data[i];
        h *= 1099511628211ULL; /* defined unsigned wraparound */
    }
    return h;
}

/* Count elements strictly greater than a threshold. Pure, in-bounds, total ->
 * a valid (positive) boundary with a scalar + buffer mix. */
size_t count_above(const int32_t *a, size_t n, int32_t threshold)
{
    size_t c = 0;
    for (size_t i = 0; i < n; i++)
        if (a[i] > threshold)
            c++;
    return c;
}

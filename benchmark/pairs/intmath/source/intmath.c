/*
 * intmath.c - small integer math library
 *
 * Theme: numeric & bit manipulation (simple arithmetic).
 * Provides gcd (recursive), integer power, and integer square root.
 * Standard libc only. No I/O, no randomness, deterministic.
 */

#include <stdint.h>

/* Recursive Euclidean GCD. Operates on absolute magnitudes.
 * Inputs are uint64_t so there is no signedness/overflow concern;
 * gcd(0,0) is defined here to return 0. */
static uint64_t gcd_u64(uint64_t a, uint64_t b)
{
    if (b == 0)
        return a;
    return gcd_u64(b, a % b);
}

/* Integer power with overflow guard.
 * Computes base^exp using binary exponentiation in modular space when
 * mod > 1; if mod == 0 the result wraps naturally in uint64_t (well-defined
 * for unsigned). Returns base^exp mod m (or full wrapped product if m==0). */
static uint64_t ipow_mod(uint64_t base, uint64_t exp, uint64_t mod)
{
    uint64_t result;

    if (mod == 1)
        return 0; /* anything mod 1 is 0 */

    result = 1;
    if (mod != 0)
        base %= mod;

    while (exp > 0) {
        if (exp & 1u) {
            if (mod != 0)
                result = (result * base) % mod;
            else
                result = result * base; /* defined unsigned wraparound */
        }
        exp >>= 1;
        if (exp > 0) {
            if (mod != 0)
                base = (base * base) % mod;
            else
                base = base * base;
        }
    }
    return result;
}

/* Integer square root (floor) via binary search. No floating point.
 * Defined for all uint64_t inputs. */
static uint64_t isqrt_u64(uint64_t n)
{
    uint64_t lo = 0;
    uint64_t hi = n;
    uint64_t ans = 0;

    if (n < 2)
        return n;

    while (lo <= hi) {
        uint64_t mid = lo + (hi - lo) / 2;
        /* avoid overflow: compare mid against n/mid instead of mid*mid */
        if (mid != 0 && mid <= n / mid) {
            ans = mid;
            lo = mid + 1;
        } else {
            if (mid == 0)
                break;
            hi = mid - 1;
        }
    }
    return ans;
}

/* ENTRY: uint64_t intmath_eval(int op, uint64_t a, uint64_t b)
 *
 * Differential fuzz target. Dispatches to one of the math helpers.
 *   op == 0 : gcd(a, b)
 *   op == 1 : ipow_mod(a, b, mod=0)  -> a^b with unsigned wraparound
 *   op == 2 : ipow_mod(a, b mod 1000000007, mod=1000000007)
 *   op == 3 : isqrt(a)
 *   default : 0
 * All operations are total / UB-free over the full uint64_t domain. */
uint64_t intmath_eval(int op, uint64_t a, uint64_t b)
{
    switch (op) {
    case 0:
        return gcd_u64(a, b);
    case 1:
        return ipow_mod(a, b, 0);
    case 2:
        return ipow_mod(a, b, 1000000007ULL);
    case 3:
        return isqrt_u64(a);
    default:
        return 0;
    }
}

/*
 * div_mod.c - signed division and modulo edge cases
 *
 * Theme: arithmetic / UB variety.
 * Division/modulo by zero is undefined, and INT32_MIN / -1 (and the
 * matching modulo) overflows the signed quotient. Provides UB-exhibiting
 * functions plus VALID guarded siblings. Standard libc only. No I/O.
 */

#include <stdint.h>

/* ENTRY: int32_t mod_signed_i32(int32_t a, int32_t b)
 *
 * INVALID: a % b. Undefined when b == 0, and INT32_MIN % -1 is undefined
 * because the associated quotient INT32_MIN / -1 overflows int32_t. For
 * other inputs it is the ordinary signed remainder. */
int32_t mod_signed_i32(int32_t a, int32_t b)
{
    return a % b; /* UB when b == 0 or (a==INT32_MIN && b==-1) */
}

/* INVALID: a / b with the same two undefined cases (divide-by-zero and
 * INT32_MIN / -1 overflow). */
int32_t div_signed_i32(int32_t a, int32_t b)
{
    return a / b; /* UB when b == 0 or (a==INT32_MIN && b==-1) */
}

/* VALID: guarded modulo. Treats a zero divisor as identity and special-
 * cases the overflowing INT32_MIN % -1 (whose mathematical remainder is 0).
 * Total over the whole int32_t x int32_t domain. */
static int32_t mod_signed_guarded(int32_t a, int32_t b)
{
    if (b == 0)
        return a;
    if (a == INT32_MIN && b == -1)
        return 0;
    return a % b;
}

/* VALID: guarded division with the symmetric protections. */
static int32_t div_signed_guarded(int32_t a, int32_t b)
{
    if (b == 0)
        return 0;
    if (a == INT32_MIN && b == -1)
        return INT32_MIN; /* defined fallback for the overflow case */
    return a / b;
}

/* Helper exercising the valid siblings so they are not dead code. */
int32_t div_mod_safe(int32_t a, int32_t b)
{
    return div_signed_guarded(a, b) + mod_signed_guarded(a, b);
}

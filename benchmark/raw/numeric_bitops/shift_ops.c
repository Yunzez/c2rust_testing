/*
 * shift_ops.c - left shifts at and beyond the bit width
 *
 * Theme: arithmetic / UB variety.
 * Shifting by >= the operand width, or left-shifting a negative value,
 * is undefined. Provides UB-exhibiting shifts plus VALID masked /
 * unsigned siblings. Standard libc only. Deterministic, no I/O.
 */

#include <stdint.h>

/* ENTRY: uint32_t shl_u32(uint32_t v, uint32_t count)
 *
 * INVALID: v << count. When count >= 32 (the width of uint32_t) the shift
 * amount is out of range, which is undefined behaviour. For count in 0..31
 * it is the ordinary logical left shift. */
uint32_t shl_u32(uint32_t v, uint32_t count)
{
    return v << count; /* UB when count >= 32 */
}

/* INVALID: left-shifting a (possibly negative) signed value. A negative
 * left operand makes v << count undefined; so does count >= 32. */
int32_t shl_i32(int32_t v, uint32_t count)
{
    return v << count; /* UB if v < 0 or count >= 32 */
}

/* VALID: masked shift count keeps the amount in [0,31], and unsigned left
 * shift of a non-negative pattern is well-defined for every input. */
static uint32_t shl_u32_masked(uint32_t v, uint32_t count)
{
    return v << (count & 31u);
}

/* VALID: shift performed entirely in unsigned space with a bounded count,
 * then reinterpreted. No negative left operand, no oversized count. */
static int32_t shl_i32_safe(int32_t v, uint32_t count)
{
    uint32_t u = (uint32_t)v;
    return (int32_t)(u << (count & 31u));
}

/* Helper exercising the valid siblings so they are not dead code. */
uint32_t shift_ops_safe(uint32_t v, uint32_t count)
{
    return shl_u32_masked(v, count) ^ (uint32_t)shl_i32_safe((int32_t)v, count);
}

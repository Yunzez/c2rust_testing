/*
 * bitutils.c - bit-twiddling utilities
 *
 * Theme: numeric & bit manipulation (bit-twiddly).
 * Provides population count, 32-bit reversal, and a bit-field packer.
 * Standard libc only. No I/O, no randomness, deterministic.
 */

#include <stdint.h>

/* Population count (number of set bits) via Brian Kernighan's loop. */
static uint32_t popcount32(uint32_t x)
{
    uint32_t count = 0;
    while (x) {
        x &= (x - 1u);
        count++;
    }
    return count;
}

/* Reverse the 32 bits of x (bit 0 <-> bit 31, etc.) using a parallel
 * swap network. Total / UB-free for all inputs. */
static uint32_t reverse32(uint32_t x)
{
    x = ((x & 0xAAAAAAAAu) >> 1) | ((x & 0x55555555u) << 1);
    x = ((x & 0xCCCCCCCCu) >> 2) | ((x & 0x33333333u) << 2);
    x = ((x & 0xF0F0F0F0u) >> 4) | ((x & 0x0F0F0F0Fu) << 4);
    x = ((x & 0xFF00FF00u) >> 8) | ((x & 0x00FF00FFu) << 8);
    x = (x >> 16) | (x << 16);
    return x;
}

/* Pack four byte-sized fields (each masked to 8 bits) into a single
 * 32-bit word, field a in the most significant byte. The explicit
 * & 0xFFu masking makes this defined regardless of the high bits of
 * the inputs. */
static uint32_t pack4(uint32_t a, uint32_t b, uint32_t c, uint32_t d)
{
    return ((a & 0xFFu) << 24) |
           ((b & 0xFFu) << 16) |
           ((c & 0xFFu) << 8)  |
           (d & 0xFFu);
}

/* ENTRY: uint32_t bitutils_eval(int op, uint32_t x, uint32_t y)
 *
 * Differential fuzz target. Dispatches to a bit utility.
 *   op == 0 : popcount32(x)
 *   op == 1 : reverse32(x)
 *   op == 2 : pack4 of the four bytes of x, then XOR with reverse32(y)
 *   op == 3 : popcount of (x ^ y)  (Hamming distance)
 *   default : 0
 * All operations are total / UB-free over the full uint32_t domain. */
uint32_t bitutils_eval(int op, uint32_t x, uint32_t y)
{
    switch (op) {
    case 0:
        return popcount32(x);
    case 1:
        return reverse32(x);
    case 2: {
        uint32_t packed = pack4(x >> 24, x >> 16, x >> 8, x);
        return packed ^ reverse32(y);
    }
    case 3:
        return popcount32(x ^ y);
    default:
        return 0;
    }
}

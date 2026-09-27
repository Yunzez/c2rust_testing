/* dynamic_array.c
 * A growable dynamic array (vector) of int values supporting push and pop.
 * The entry function interprets an opcode/value stream to drive the array and
 * returns an aggregate sum, freeing all memory before returning.
 *
 * Only standard libc is used. No I/O, no randomness, no global state.
 */

#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>

typedef struct {
    int *data;
    size_t len;
    size_t cap;
} DynArray;

/* Initialize an empty array. Returns 0 on success, -1 on allocation failure. */
static int da_init(DynArray *a) {
    a->data = NULL;
    a->len = 0;
    a->cap = 0;
    return 0;
}

/* Ensure capacity for at least one more element. Returns 0 ok, -1 on failure. */
static int da_grow(DynArray *a) {
    if (a->len < a->cap) {
        return 0;
    }
    size_t new_cap = (a->cap == 0) ? 4 : a->cap * 2;
    int *p = (int *)realloc(a->data, new_cap * sizeof(int));
    if (p == NULL) {
        return -1;
    }
    a->data = p;
    a->cap = new_cap;
    return 0;
}

/* Push a value. Returns 0 on success, -1 on failure (array left unchanged). */
static int da_push(DynArray *a, int value) {
    if (da_grow(a) != 0) {
        return -1;
    }
    a->data[a->len++] = value;
    return 0;
}

/* Pop the last value into *out. Returns 0 on success, -1 if empty. */
static int da_pop(DynArray *a, int *out) {
    if (a->len == 0) {
        return -1;
    }
    *out = a->data[--a->len];
    return 0;
}

/* Sum all current elements. */
static long da_sum(const DynArray *a) {
    long s = 0;
    for (size_t i = 0; i < a->len; i++) {
        s += a->data[i];
    }
    return s;
}

/* Free the backing storage and reset to empty. */
static void da_free(DynArray *a) {
    free(a->data);
    a->data = NULL;
    a->len = 0;
    a->cap = 0;
}

/* ENTRY: long da_run(const int *ops, size_t n)
 *
 * Drive the dynamic array from a flat operation stream. Each step reads one
 * opcode; if the opcode is a push it also consumes the next int as the value.
 *   opcode % 2 == 0 -> push (next int is the value to push)
 *   opcode % 2 == 1 -> pop  (popped value, if any, is subtracted from sum)
 * Returns: final sum of remaining elements plus the running pop accumulator.
 * Frees all memory before returning.
 */
// ENTRY: long da_run(const int *ops, size_t n)
long da_run(const int *ops, size_t n) {
    DynArray a;
    da_init(&a);

    long pop_acc = 0;
    size_t i = 0;
    while (i < n) {
        int opcode = ops[i++];
        if ((opcode & 1) == 0) {
            int value = 0;
            if (i < n) {
                value = ops[i++];
            }
            da_push(&a, value);
        } else {
            int out;
            if (da_pop(&a, &out) == 0) {
                pop_acc -= out;
            }
        }
    }

    long result = da_sum(&a) + pop_acc;
    da_free(&a);
    return result;
}

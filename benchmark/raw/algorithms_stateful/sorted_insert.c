/*
 * sorted_insert.c
 *
 * Insert a value into a sorted, fixed-capacity array while keeping it sorted.
 * The insertion position is found by a binary search over the *claimed* live
 * length `len`. The code TRUSTS that `len < cap` (i.e. that there is room)
 * and writes the new element without re-checking against the capacity.
 *
 * INVALID: when the array is already full (len == cap), the shift-and-store
 * loop writes at index `len == cap`, one past the end of the buffer -- an
 * out-of-bounds store (buffer overflow / UB) driven by the input length.
 *
 * Self-contained; standard libc only.
 *
 * Complexity: 2 functions, binary search + shifting insert.
 */

#include <stdint.h>
#include <stddef.h>

/* Lower-bound binary search: first index i in [0,len] with a[i] >= key. */
static size_t lower_bound(const int32_t *a, size_t len, int32_t key) {
    size_t lo = 0, hi = len;
    while (lo < hi) {
        size_t mid = lo + (hi - lo) / 2;
        if (a[mid] < key) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    return lo;
}

/*
 * Insert `key` into the sorted prefix a[0..len-1] of a buffer with capacity
 * `cap`, returning the new length. The position comes from a binary search.
 *
 * BUG: there is no `len < cap` guard. When len == cap the array is full, yet
 * the function still shifts elements up and stores at index `len`, writing
 * a[cap] -- one element past the allocation. The faulty index is derived from
 * the caller-supplied length, not re-bounds-checked against cap.
 */
// ENTRY: size_t sorted_insert(int32_t *a, size_t len, size_t cap, int32_t key)
size_t sorted_insert(int32_t *a, size_t len, size_t cap, int32_t key) {
    size_t pos = lower_bound(a, len, key);
    (void)cap; /* capacity is ignored -- the precondition is merely trusted */

    for (size_t i = len; i > pos; i--) {
        a[i] = a[i - 1];   /* when len == cap, first iter writes a[cap] (OOB) */
    }
    a[pos] = key;
    return len + 1;
}

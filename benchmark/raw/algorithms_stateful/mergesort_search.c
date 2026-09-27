/*
 * mergesort_search.c
 *
 * Recursive top-down mergesort over an int array, followed by a binary
 * search (with lower-bound variant). Self-contained; standard libc only.
 *
 * Complexity: 4 functions, recursive, moderately pointer-heavy.
 */

#include <stddef.h>
#include <stdlib.h>

/* Merge two adjacent sorted runs [lo,mid) and [mid,hi) of `a` using
 * scratch buffer `tmp` (must be at least `hi - lo` long). */
static void merge_runs(int *a, int *tmp, size_t lo, size_t mid, size_t hi) {
    size_t i = lo, j = mid, k = 0;
    while (i < mid && j < hi) {
        if (a[i] <= a[j]) {
            tmp[k++] = a[i++];
        } else {
            tmp[k++] = a[j++];
        }
    }
    while (i < mid) {
        tmp[k++] = a[i++];
    }
    while (j < hi) {
        tmp[k++] = a[j++];
    }
    for (size_t t = 0; t < k; t++) {
        a[lo + t] = tmp[t];
    }
}

/* Recursive sort of the half-open range [lo, hi). */
static void msort_range(int *a, int *tmp, size_t lo, size_t hi) {
    if (hi - lo < 2) {
        return; /* zero or one element: already sorted */
    }
    size_t mid = lo + (hi - lo) / 2;
    msort_range(a, tmp, lo, mid);
    msort_range(a, tmp, mid, hi);
    merge_runs(a, tmp, lo, mid, hi);
}

/* Lower-bound binary search: returns the index of the first element
 * >= key, or `n` if none. Assumes `a` is sorted ascending. */
static size_t lower_bound(const int *a, size_t n, int key) {
    size_t lo = 0, hi = n;
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
 * Entry function. Sorts `a` (length `n`) in place via mergesort, then
 * returns the index of the first occurrence of `key` in the sorted array,
 * or -1 if not present. Allocates a scratch buffer internally; on
 * allocation failure it leaves `a` untouched and returns -2.
 *
 * Bounds: n is size_t; all loops bounded by n. Safe for n == 0.
 */
// ENTRY: int sort_and_find(int *a, size_t n, int key)
int sort_and_find(int *a, size_t n, int key) {
    if (n == 0) {
        return -1;
    }
    int *tmp = (int *)malloc(n * sizeof(int));
    if (tmp == NULL) {
        return -2;
    }
    msort_range(a, tmp, 0, n);
    free(tmp);

    size_t idx = lower_bound(a, n, key);
    if (idx < n && a[idx] == key) {
        return (int)idx;
    }
    return -1;
}

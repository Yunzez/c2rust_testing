/* array_map_reduce.c
 * Generic array transformation pipeline driven by function-pointer callbacks.
 * The ENTRY function takes a unary map callback AND a binary fold callback.
 */
#include <stddef.h>
#include <stdint.h>

/* Apply a unary callback to every element of src, writing into dst. */
static void map_array(const int32_t *src, int32_t *dst, size_t n,
                      int32_t (*fn)(int32_t)) {
    for (size_t i = 0; i < n; i++) {
        dst[i] = fn(src[i]);
    }
}

/* Fold an array with a binary op, starting from acc0. */
static int32_t fold_array(const int32_t *src, size_t n, int32_t acc0,
                          int32_t (*op)(int32_t, int32_t)) {
    int32_t acc = acc0;
    for (size_t i = 0; i < n; i++) {
        acc = op(acc, src[i]);
    }
    return acc;
}

// ENTRY: int32_t map_then_reduce(const int32_t *src, int32_t *scratch, size_t n, int32_t (*map_fn)(int32_t), int32_t (*reduce_fn)(int32_t, int32_t), int32_t init)
int32_t map_then_reduce(const int32_t *src, int32_t *scratch, size_t n,
                        int32_t (*map_fn)(int32_t),
                        int32_t (*reduce_fn)(int32_t, int32_t),
                        int32_t init) {
    if (src == NULL || scratch == NULL || map_fn == NULL || reduce_fn == NULL) {
        return init;
    }
    map_array(src, scratch, n, map_fn);
    return fold_array(scratch, n, init, reduce_fn);
}

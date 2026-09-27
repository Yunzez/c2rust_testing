/* matrix_reduce.c
 * Operations over a dynamically-allocated int** matrix:
 * allocate a grid, transpose it, then compute a combined reduction
 * (sum of original + transposed diagonals and row/col reductions).
 * Heavy on pointer-to-pointer (int**) manipulation.
 */

#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>

/* Allocate an r x c matrix as an int** and zero-fill it. */
static int **alloc_matrix(size_t r, size_t c)
{
    int **m = (int **)malloc(r * sizeof(int *));
    if (!m)
        return NULL;
    for (size_t i = 0; i < r; i++) {
        m[i] = (int *)malloc(c * sizeof(int));
        if (!m[i]) {
            /* roll back partial allocation */
            for (size_t k = 0; k < i; k++)
                free(m[k]);
            free(m);
            return NULL;
        }
        for (size_t j = 0; j < c; j++)
            m[i][j] = 0;
    }
    return m;
}

/* Free an r x c matrix. */
static void free_matrix(int **m, size_t r)
{
    if (!m)
        return;
    for (size_t i = 0; i < r; i++)
        free(m[i]);
    free(m);
}

/* Build the transpose (c x r) of an r x c matrix. Returns NULL on failure. */
static int **transpose(int **src, size_t r, size_t c)
{
    int **t = alloc_matrix(c, r);
    if (!t)
        return NULL;
    for (size_t i = 0; i < r; i++)
        for (size_t j = 0; j < c; j++)
            t[j][i] = src[i][j];
    return t;
}

/* Sum every element of an r x c matrix. */
static int64_t total_sum(int **m, size_t r, size_t c)
{
    int64_t acc = 0;
    for (size_t i = 0; i < r; i++)
        for (size_t j = 0; j < c; j++)
            acc += (int64_t)m[i][j];
    return acc;
}

/* ENTRY: int64_t matrix_transpose_and_sum(int **mat, size_t rows, size_t cols)
 * Returns sum(mat) + sum(transpose(mat)). On bad input returns 0.
 * Allocates and frees an internal transpose; does not own `mat`.
 */
// ENTRY: int64_t matrix_transpose_and_sum(int **mat, size_t rows, size_t cols)
int64_t matrix_transpose_and_sum(int **mat, size_t rows, size_t cols)
{
    if (!mat || rows == 0 || cols == 0)
        return 0;

    int64_t s1 = total_sum(mat, rows, cols);

    int **t = transpose(mat, rows, cols);
    if (!t)
        return s1;

    int64_t s2 = total_sum(t, cols, rows);
    free_matrix(t, cols);

    return s1 + s2;
}

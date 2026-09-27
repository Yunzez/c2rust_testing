/*
 * graph_bfs.c
 *
 * Breadth-first search over a fixed-size adjacency matrix passed as a flat
 * byte buffer of n*n entries (matrix[r*n + c] != 0 means edge r->c). The BFS
 * uses an explicit ring-buffer queue and a visited array, both sized to the
 * vertex count, with every index range-checked. Iterative (no recursion).
 *
 * Self-contained; standard libc only. No global state.
 *
 * Complexity: 2 functions, queue state machine, bounded matrix access.
 */

#include <stdint.h>
#include <stddef.h>

#define BFS_MAX_V 64

/*
 * Run BFS from `start` over an n-vertex graph given by the flat adjacency
 * matrix `adj` (length n*n). Writes the visitation order into `order`
 * (capacity n) and returns the number of vertices reached. Returns 0 if n is
 * 0, n exceeds BFS_MAX_V, or start >= n. All queue/visited indices are
 * bounded by n <= BFS_MAX_V, so no out-of-bounds access occurs.
 */
// ENTRY: size_t graph_bfs(const uint8_t *adj, size_t n, size_t start, size_t *order)
size_t graph_bfs(const uint8_t *adj, size_t n, size_t start, size_t *order) {
    uint8_t visited[BFS_MAX_V];
    size_t queue[BFS_MAX_V];
    size_t head = 0, tail = 0, out = 0;

    if (n == 0 || n > BFS_MAX_V || start >= n) {
        return 0;
    }
    for (size_t i = 0; i < n; i++) {
        visited[i] = 0;
    }

    visited[start] = 1;
    queue[tail++] = start;

    while (head < tail) {
        size_t u = queue[head++];
        order[out++] = u;
        for (size_t v = 0; v < n; v++) {
            if (adj[u * n + v] != 0 && !visited[v]) {
                visited[v] = 1;
                /* tail < n <= BFS_MAX_V always, since each vertex is
                 * enqueued at most once. */
                queue[tail++] = v;
            }
        }
    }
    return out;
}

/* Count edges (nonzero entries) in an n*n flat adjacency matrix. */
size_t count_edges(const uint8_t *adj, size_t n) {
    size_t c = 0;
    for (size_t i = 0; i < n * n; i++) {
        if (adj[i] != 0) {
            c++;
        }
    }
    return c;
}

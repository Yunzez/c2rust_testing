/* graph_dfs.c
 * A small adjacency-list graph built from edge pairs. Vertices are
 * stored as Node** buckets (array of linked-list heads). A recursive
 * DFS visits all vertices reachable from 0 and sums their weights.
 * Heavy on pointer-to-pointer (Node**) structure.
 */

#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>

typedef struct Node {
    size_t to;          /* destination vertex */
    struct Node *next;  /* next edge in this bucket */
} Node;

/* Allocate the bucket array: an array of `n` Node* heads, all NULL. */
static Node **alloc_buckets(size_t n)
{
    Node **b = (Node **)malloc(n * sizeof(Node *));
    if (!b)
        return NULL;
    for (size_t i = 0; i < n; i++)
        b[i] = NULL;
    return b;
}

/* Prepend an edge from -> to into the bucket array. Returns 0 on success. */
static int add_edge(Node **buckets, size_t from, size_t to)
{
    Node *e = (Node *)malloc(sizeof(Node));
    if (!e)
        return -1;
    e->to = to;
    e->next = buckets[from];
    buckets[from] = e;
    return 0;
}

/* Free every edge node and the bucket array itself. */
static void free_buckets(Node **buckets, size_t n)
{
    if (!buckets)
        return;
    for (size_t i = 0; i < n; i++) {
        Node *e = buckets[i];
        while (e) {
            Node *nx = e->next;
            free(e);
            e = nx;
        }
    }
    free(buckets);
}

/* Recursive DFS: accumulate count of visited vertices (each counted once). */
static int64_t dfs(Node **buckets, unsigned char *seen, size_t v)
{
    if (seen[v])
        return 0;
    seen[v] = 1;
    int64_t acc = 1; /* count this vertex */
    for (Node *e = buckets[v]; e; e = e->next)
        acc += dfs(buckets, seen, e->to);
    return acc;
}

/* ENTRY: int64_t count_reachable(size_t n, const size_t (*edges)[2], size_t m)
 * Builds an adjacency list over `n` vertices from `m` directed edge pairs,
 * runs a recursive DFS from vertex 0, and returns the number of vertices
 * reachable from 0 (inclusive). Edges with out-of-range endpoints are
 * skipped. Allocates and frees the full graph.
 */
// ENTRY: int64_t count_reachable(size_t n, const size_t (*edges)[2], size_t m)
int64_t count_reachable(size_t n, const size_t (*edges)[2], size_t m)
{
    if (n == 0)
        return 0;

    Node **buckets = alloc_buckets(n);
    if (!buckets)
        return 0;

    for (size_t i = 0; i < m; i++) {
        size_t from = edges[i][0];
        size_t to = edges[i][1];
        if (from < n && to < n) {
            if (add_edge(buckets, from, to) != 0) {
                free_buckets(buckets, n);
                return 0;
            }
        }
    }

    unsigned char *seen = (unsigned char *)calloc(n, sizeof(unsigned char));
    if (!seen) {
        free_buckets(buckets, n);
        return 0;
    }

    int64_t reachable = dfs(buckets, seen, 0);

    free(seen);
    free_buckets(buckets, n);
    return reachable;
}

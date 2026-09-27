#include <stdio.h>
#include <stdlib.h>
#include "quadtree.h"

/* Deterministic driver: LCG-generated inserts into a fixed-bounds tree,
 * then LCG-generated searches; prints hit/miss and found coords. */
static unsigned long lcg_state;
static double lcg_next(void) {
    lcg_state = lcg_state * 6364136223846793005UL + 1442695040888963407UL;
    return (double)((lcg_state >> 33) % 1000000) / 10000.0; /* 0..100 */
}

int main(int argc, char *argv[]) {
    if (argc != 3) { printf("usage: driver <seed> <n_ops>\n"); return 1; }
    lcg_state = (unsigned long)atol(argv[1]);
    int n = atoi(argv[2]);

    quadtree_t *tree = quadtree_new(0.0, 0.0, 100.0, 100.0);
    if (!tree) return 2;

    /* inserts (key = small heap int so free path is exercised) */
    for (int i = 0; i < n; ++i) {
        double x = lcg_next(), y = lcg_next();
        int *key = malloc(sizeof(int));
        *key = i;
        int r = quadtree_insert(tree, x, y, key);
        printf("ins %.4f %.4f -> %d\n", x, y, r);
    }
    printf("length %d\n", tree->length);

    /* searches: half re-derived from same LCG start (hits), half fresh (misses) */
    lcg_state = (unsigned long)atol(argv[1]);
    for (int i = 0; i < n; ++i) {
        double x = lcg_next(), y = lcg_next();
        quadtree_point_t *p = quadtree_search(tree, x, y);
        if (p) printf("hit %.4f %.4f\n", p->x, p->y);
        else printf("miss %.4f %.4f\n", x, y);
    }
    quadtree_free(tree);
    printf("done\n");
    return 0;
}

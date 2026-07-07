#include <stdio.h>
#include <stdlib.h>
#include "genann.h"

/* Deterministic driver: 2-4-1 net, weights overwritten with a fixed sequence
 * (no rand dependency), forward pass + a few training steps, print outputs. */
int main(int argc, char *argv[]) {
    if (argc != 4) { printf("usage: driver <a> <b> <train_iters>\n"); return 1; }
    double in[2];
    in[0] = atof(argv[1]);
    in[1] = atof(argv[2]);
    int iters = atoi(argv[3]);

    genann *ann = genann_init(2, 1, 4, 1);
    if (!ann) return 2;
    for (int i = 0; i < ann->total_weights; ++i)
        ann->weight[i] = ((double)((i * 37) % 200) - 100.0) / 100.0;

    const double *out = genann_run(ann, in);
    printf("run %.12f\n", out[0]);

    /* deterministic XOR-ish training */
    const double tin[4][2] = {{0,0},{0,1},{1,0},{1,1}};
    const double tout[4] = {0, 1, 1, 0};
    for (int it = 0; it < iters; ++it)
        for (int k = 0; k < 4; ++k)
            genann_train(ann, tin[k], &tout[k], 0.5);

    out = genann_run(ann, in);
    printf("trained %.12f\n", out[0]);

    genann_free(ann);
    return 0;
}

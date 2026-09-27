#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "bzlib.h"
/* deterministic compress->decompress roundtrip; print sizes + checksum */
int main(int argc, char *argv[]) {
    if (argc != 3) { printf("usage: driver <n> <pattern>\n"); return 1; }
    unsigned n = (unsigned)atoi(argv[1]);
    int pat = atoi(argv[2]);
    char *src = malloc(n ? n : 1);
    for (unsigned i = 0; i < n; ++i) src[i] = (char)((i * pat + 3) & 0xFF);
    unsigned destLen = n + n / 100 + 600;
    char *dest = malloc(destLen);
    int r = BZ2_bzBuffToBuffCompress(dest, &destLen, src, n, 5, 0, 0);
    printf("comp %d %u\n", r, destLen);
    if (r == 0) {
        unsigned backLen = n ? n : 1;
        char *back = malloc(backLen);
        r = BZ2_bzBuffToBuffDecompress(back, &backLen, dest, destLen, 0, 0);
        unsigned long sum = 0;
        for (unsigned i = 0; i < backLen; ++i) sum += (unsigned char)back[i];
        printf("decomp %d %u %lu %d\n", r, backLen, sum, n ? memcmp(src, back, n) : 0);
        free(back);
    }
    free(src); free(dest);
    return 0;
}

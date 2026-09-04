#include <stdio.h>
#include <stdlib.h>
#include "lodepng.h"
/* deterministic encode->decode roundtrip; print sizes + checksum */
int main(int argc, char *argv[]) {
    if (argc != 3) { printf("usage: driver <w> <h>\n"); return 1; }
    unsigned w = (unsigned)atoi(argv[1]), h = (unsigned)atoi(argv[2]);
    unsigned char *img = malloc((size_t)w * h * 4);
    for (unsigned i = 0; i < w * h * 4; ++i) img[i] = (unsigned char)((i * 31 + 7) & 0xFF);
    unsigned char *png = NULL; size_t pngsize = 0;
    unsigned err = lodepng_encode32(&png, &pngsize, img, w, h);
    printf("enc %u %zu\n", err, pngsize);
    if (!err) {
        unsigned char *out = NULL; unsigned ow, oh;
        err = lodepng_decode32(&out, &ow, &oh, png, pngsize);
        unsigned long sum = 0;
        if (!err) for (unsigned i = 0; i < ow * oh * 4; ++i) sum += out[i];
        printf("dec %u %u %u %lu\n", err, ow, oh, sum);
        free(out);
    }
    free(png); free(img);
    return 0;
}

/* coverage driver: run one lil script through the whole-program eval entry */
#include <stdio.h>
#include <stdlib.h>
#include "lil.h"
int main(int argc, char** argv) {
    if (argc < 2) return 1;
    FILE* f = fopen(argv[1], "rb");
    if (!f) return 1;
    fseek(f, 0, SEEK_END); long n = ftell(f); fseek(f, 0, SEEK_SET);
    char* buf = malloc(n + 1);
    fread(buf, 1, n, f); buf[n] = 0; fclose(f);
    lil_t lil = lil_new();
    lil_value_t v = lil_parse(lil, buf, (size_t)n, 1);
    lil_free_value(v);
    lil_free(lil);
    free(buf);
    return 0;
}

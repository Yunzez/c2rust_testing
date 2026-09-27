#include <stdio.h>
#include "lil.h"
int main(int argc, char *argv[]) {
    if (argc != 2) { printf("usage: driver <script>\n"); return 1; }
    lil_t lil = lil_new();
    lil_value_t r = lil_parse(lil, argv[1], 0, 1);
    printf("%s\n", lil_to_string(r));
    lil_free_value(r);
    lil_free(lil);
    return 0;
}

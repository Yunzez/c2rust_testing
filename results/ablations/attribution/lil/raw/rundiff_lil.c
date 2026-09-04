#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lil.h"
/* in-loop UBSan gate shim (only linked in the "gate" variant; weak no-ops otherwise) */
__attribute__((weak)) void c2r_ub_reset(void) {}
__attribute__((weak)) int  c2r_ub_get(void) { return 0; }
static void run_one(const unsigned char* code, size_t len) {
    char* buf = malloc(len + 1); memcpy(buf, code, len); buf[len] = 0;
    c2r_ub_reset();
    lil_t lil = lil_new();
    lil_value_t r = lil_parse(lil, buf, len, 0);
    const char* s = lil_to_string(r);
    int ub = c2r_ub_get();
    if (ub) printf("[UB-EXCLUDED]\n");
    else if (!s) printf("[(null)]\n");
    else printf("[%s]\n", s);
    lil_free_value(r); lil_free(lil); free(buf);
    fflush(stdout);
}
int main(int argc, char** argv) {
    /* read all of stdin */
    size_t cap = 1 << 16, n = 0; unsigned char* inp = malloc(cap); size_t k;
    while ((k = fread(inp + n, 1, cap - n, stdin)) > 0) { n += k; if (n == cap) { cap *= 2; inp = realloc(inp, cap); } }
    if (argc > 1 && !strcmp(argv[1], "--single")) { run_one(inp, n); return 0; }
    size_t p = 0;
    while (p + 2 <= n) {
        size_t len = inp[p] | (inp[p + 1] << 8); p += 2;
        if (p + len > n) break;
        run_one(inp + p, len); p += len;
    }
    return 0;
}

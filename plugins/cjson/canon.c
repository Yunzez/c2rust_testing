/* Comparator plugin for cJSON — the C half.
 *
 * STABLE ABI (docs/harness_oracle_plan.md §5):
 *     size_t c2r_canon(const void *obj, char *out, size_t cap);
 * Writes the canonical byte form of *obj into `out` and returns the number of bytes the form
 * NEEDS. A return greater than `cap` means the buffer was too small and nothing may be compared.
 *
 * A plugin extends OUTPUT COMPARISON only. It never touches the InputPlan and it is not a
 * hand-written harness. It is compiled into the C oracle, so the generator's function renaming
 * applies to it too: a call to `cJSON_Delete` here becomes `c_cJSON_Delete`, i.e. the C side's
 * own function, which is what we want.
 *
 * This file was produced once by tools/stu_selector/contract_templates.py from a declaration of
 * cJSON's object graph, and is now OWNED AS SOURCE. It is the reference implementation of the
 * interface, not a template the generator re-derives.
 *
 * It deliberately does NOT call the library's own printer: a printer is translated code too, so a
 * defect the printer also mis-handles would be invisible (cf. S8, cJSON x PtrTrans, where the
 * success path assigns valuestring = None).
 */
#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include "cJSON.h"

typedef struct { char *b; size_t n, cap; } c2r_buf;
static void c2r_put(c2r_buf *o, const char *s, size_t k) {
    if (o->n + k <= o->cap) memcpy(o->b + o->n, s, k);
    o->n += k;
}
static void c2r_c(c2r_buf *o, char c) { c2r_put(o, &c, 1); }
static void c2r_int(c2r_buf *o, long long v) {
    char t[32]; int k = snprintf(t, sizeof t, "i%lld;", v); c2r_put(o, t, (size_t)k);
}
static void c2r_dbl(c2r_buf *o, unsigned long long u) {
    char t[32]; int k = snprintf(t, sizeof t, "d%016llx;", u); c2r_put(o, t, (size_t)k);
}
static void c2r_str(c2r_buf *o, const char *s) {
    static const char H[] = "0123456789abcdef";
    if (!s) { c2r_put(o, "sN;", 3); return; }
    size_t n = strlen(s); char t[32];
    int k = snprintf(t, sizeof t, "s%zu:", n); c2r_put(o, t, (size_t)k);
    for (size_t i = 0; i < n; i++) {
        char h[2]; h[0] = H[(unsigned char)s[i] >> 4]; h[1] = H[(unsigned char)s[i] & 15];
        c2r_put(o, h, 2);
    }
    c2r_c(o, ';');
}
static const void *c2r_seen[4096];
static size_t c2r_nseen, c2r_nodes;
static int c2r_mark(const void *p) {
    for (size_t i = 0; i < c2r_nseen; i++) if (c2r_seen[i] == p) return 0;
    if (c2r_nseen < 4096) c2r_seen[c2r_nseen++] = p;
    return 1;
}
static void c2r_node(c2r_buf *o, const cJSON *p, int depth) {
    if (!p) { c2r_c(o, 'N'); return; }
    if (depth > 64 || c2r_nodes >= 4096 || !c2r_mark((const void *)p)) { c2r_c(o, 'X'); return; }
    c2r_nodes++;
    c2r_c(o, '(');
    c2r_int(o, (long long)p->type);
    c2r_int(o, (long long)p->valueint);
    { double _d = p->valuedouble; unsigned long long _u; memcpy(&_u, &_d, 8); c2r_dbl(o, _u); }
    c2r_str(o, p->valuestring);
    c2r_str(o, p->string);
    c2r_c(o, '[');
    {
        const cJSON *q;
        for (q = p->child; q; q = q->next) {
            if (c2r_nodes >= 4096) { c2r_c(o, 'X'); break; }
            c2r_node(o, q, depth + 1);
        }
    }
    c2r_c(o, ']');
    c2r_c(o, ')');
}
size_t c2r_extract(const cJSON *p, char *out, size_t cap) {
    c2r_buf o; o.b = out; o.n = 0; o.cap = cap;
    c2r_nseen = 0; c2r_nodes = 0;
    c2r_node(&o, p, 0);
    return o.n;
}

/* ---- stable ABI entry point ---------------------------------------------------------------- */
size_t c2r_canon(const void *obj, char *out, size_t cap) {
    return c2r_extract((const cJSON *)obj, out, cap);
}

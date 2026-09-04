/* RQ4 coverage — Linux shims for the macOS symbols the transpiled bzip2 CLI references.
 * Derived from results/rq4_effectiveness/bugs/bzip2_crown/darwin_shims.c, extended with
 * __stdinp / __stdoutp / __error, which the CLI (bzip2.rs) needs and the differential
 * driver did not.  Representation-only: no bzip2 logic lives here. */
#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <errno.h>

FILE *__stdinp, *__stdoutp, *__stderrp;
__attribute__((constructor)) static void _init_stdio(void){
    __stdinp = stdin; __stdoutp = stdout; __stderrp = stderr;
}

int *__error(void){ return &errno; }

int __maskrune(int c, unsigned long f){
    unsigned r = 0;
    if (c >= 0 && c <= 255) {
        if (isalpha(c))  r |= 0x100;   if (iscntrl(c))  r |= 0x200;
        if (isdigit(c))  r |= 0x400;   if (isgraph(c))  r |= 0x800;
        if (islower(c))  r |= 0x1000;  if (ispunct(c))  r |= 0x2000;
        if (isspace(c))  r |= 0x4000;  if (isupper(c))  r |= 0x8000;
        if (isxdigit(c)) r |= 0x10000; if (c == ' ' || c == '\t') r |= 0x20000;
        if (isprint(c))  r |= 0x40000;
    }
    return (int)(r & f) != 0 ? (int)(r & f) : 0;
}

/* Layout mirrors the darwin _RuneLocale prefix used by the transpiled __istype. */
struct _rune_locale_prefix {
    char magic[8]; char encoding[32];
    void *sgetrune; void *sputrune;
    int invalid_rune;
    unsigned int runetype[256];
    int maplower[256]; int mapupper[256];
};
struct _rune_locale_prefix _DefaultRuneLocale;
__attribute__((constructor)) static void _init_runes(void){
    for (int c = 0; c < 256; c++)
        _DefaultRuneLocale.runetype[c] = (unsigned)__maskrune(c, ~0UL);
}

void __assert_rtn(const char *f, const char *file, int line, const char *e){
    fprintf(stderr, "assert_rtn: %s %s:%d %s\n", f, file, line, e);
    abort();
}

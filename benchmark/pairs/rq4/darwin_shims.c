/* RQ4 coverage — Linux definitions of the macOS libc symbols the macOS-transpiled c2rust bzip2
 * crate references.  Derived from results/rq4_effectiveness/bugs/bzip2_crown/darwin_shims.c.
 * Platform adapter only: no bzip2 logic here, and none of these names is defined by the bzip2
 * C sources, so the oracle-renaming defines in the generated build.rs never touch them. */
#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <errno.h>

FILE *__stdinp, *__stdoutp, *__stderrp;
__attribute__((constructor)) static void _c2r_init_stdio(void){
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

struct _rune_locale_prefix {
    char magic[8]; char encoding[32];
    void *sgetrune; void *sputrune;
    int invalid_rune;
    unsigned int runetype[256];
    int maplower[256]; int mapupper[256];
};
struct _rune_locale_prefix _DefaultRuneLocale;
__attribute__((constructor)) static void _c2r_init_runes(void){
    for (int c = 0; c < 256; c++)
        _DefaultRuneLocale.runetype[c] = (unsigned)__maskrune(c, ~0UL);
}

void __assert_rtn(const char *f, const char *file, int line, const char *e){
    fprintf(stderr, "assert_rtn: %s %s:%d %s\n", f, file, line, e);
    abort();
}

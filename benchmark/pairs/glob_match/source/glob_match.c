/*
 * glob_match.c
 * Recursive wildcard (glob) matcher.
 *
 * Supported metacharacters in the pattern:
 *    '*'  matches any sequence of characters (including empty)
 *    '?'  matches exactly one character
 *    '[' ... ']'  character class, e.g. [abc] or [a-z]; a leading '!' or
 *                 '^' negates the class. A ']' immediately after the
 *                 opening (or after the negation) is a literal ']'.
 *    '\\' escapes the next pattern character (literal match)
 *
 * Matching is anchored: the entire text must be consumed by the entire
 * pattern.
 *
 * Complexity: pointer/branch-heavy. Recursion on '*', nested class
 * scanning, many branches, loops.
 */

#include <stddef.h>
#include <stdint.h>

/* Try to match text[t..tlen) against character class starting at pat[p].
 * On success, returns 1 and sets *next_p to the index just past the ']'.
 * On failure to match the single text char, returns 0 but still sets
 * *next_p past ']' so the caller can advance correctly. Returns -1 if the
 * class is malformed (no closing ']'). Consumes exactly one text char on a
 * value-bearing result. */
static int match_class(const char *pat, size_t plen, size_t p,
                       char tc, size_t *next_p) {
    int negate = 0;
    int matched = 0;
    size_t i = p + 1; /* skip '[' */

    if (i < plen && (pat[i] == '!' || pat[i] == '^')) {
        negate = 1;
        i++;
    }

    /* A ']' here is literal. */
    int first = 1;
    while (i < plen && (pat[i] != ']' || first)) {
        char lo = pat[i];
        first = 0;
        /* range like a-z: ensure '-' is not trailing before ']' */
        if (i + 2 < plen && pat[i + 1] == '-' && pat[i + 2] != ']') {
            char hi = pat[i + 2];
            if ((unsigned char)tc >= (unsigned char)lo &&
                (unsigned char)tc <= (unsigned char)hi) {
                matched = 1;
            }
            i += 3;
        } else {
            if (tc == lo) {
                matched = 1;
            }
            i += 1;
        }
    }

    if (i >= plen || pat[i] != ']') {
        return -1; /* unterminated class */
    }
    *next_p = i + 1; /* past ']' */

    if (negate) {
        matched = !matched;
    }
    return matched ? 1 : 0;
}

/* Core recursive matcher over indices. */
static int match_at(const char *pat, size_t plen, size_t p,
                    const char *text, size_t tlen, size_t t) {
    while (p < plen) {
        char pc = pat[p];

        if (pc == '*') {
            /* Collapse consecutive '*'. */
            while (p < plen && pat[p] == '*') {
                p++;
            }
            if (p == plen) {
                return 1; /* trailing '*' matches the rest */
            }
            /* Try to match the remainder of the pattern at every position. */
            for (size_t k = t; k <= tlen; k++) {
                if (match_at(pat, plen, p, text, tlen, k)) {
                    return 1;
                }
            }
            return 0;
        } else if (pc == '?') {
            if (t >= tlen) {
                return 0;
            }
            p++;
            t++;
        } else if (pc == '[') {
            if (t >= tlen) {
                return 0;
            }
            size_t next_p;
            int r = match_class(pat, plen, p, text[t], &next_p);
            if (r < 0) {
                /* malformed class: treat '[' as a literal */
                if (text[t] != '[') {
                    return 0;
                }
                p++;
                t++;
            } else if (r == 0) {
                return 0;
            } else {
                p = next_p;
                t++;
            }
        } else if (pc == '\\') {
            /* Escape: next pattern char is literal. */
            p++;
            if (p >= plen) {
                /* trailing backslash: match a literal backslash */
                if (t >= tlen || text[t] != '\\') {
                    return 0;
                }
                t++;
                continue;
            }
            if (t >= tlen || text[t] != pat[p]) {
                return 0;
            }
            p++;
            t++;
        } else {
            if (t >= tlen || text[t] != pc) {
                return 0;
            }
            p++;
            t++;
        }
    }
    return t == tlen;
}

/*
 * ENTRY: int glob_match(const char *pattern, const uint8_t *text, size_t tlen)
 *
 * Return 1 if `pattern` (a NUL-terminated glob) matches the entire `text`
 * buffer of length `tlen`, else 0. The text is treated as raw bytes and may
 * contain embedded NULs.
 */
int glob_match(const char *pattern, const uint8_t *text, size_t tlen) {
    if (pattern == NULL || (text == NULL && tlen != 0)) {
        return 0;
    }
    size_t plen = 0;
    while (pattern[plen] != '\0') {
        plen++;
    }
    return match_at(pattern, plen, 0, (const char *)text, tlen, 0);
}

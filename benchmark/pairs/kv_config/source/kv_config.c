/*
 * kv_config.c
 * Tiny "key=value" config line parser.
 *
 * Given a single NUL-terminated line, this trims surrounding ASCII
 * whitespace from the key and value, optionally strips a single pair of
 * matching double quotes around the value, and ignores lines that are
 * comments (first non-space char is '#') or blank. The key and value are
 * copied into caller-provided buffers.
 *
 * Complexity: medium. Multiple helpers, branchy trimming/quoting logic,
 * loops, no recursion, moderate pointer use.
 */

#include <stddef.h>
#include <stdint.h>
#include <string.h>

static int is_space(char c) {
    return c == ' ' || c == '\t' || c == '\r' || c == '\n' || c == '\f' || c == '\v';
}

/* Advance *p past leading whitespace. */
static const char *skip_ws(const char *p) {
    while (*p && is_space(*p)) {
        p++;
    }
    return p;
}

/* Return pointer one past the last non-space char in [start,end). */
static const char *rtrim(const char *start, const char *end) {
    while (end > start && is_space(end[-1])) {
        end--;
    }
    return end;
}

/* Copy [start,end) into dst (capacity cap, includes NUL). Returns length
 * written, or (size_t)-1 if it would not fit. */
static size_t copy_span(const char *start, const char *end, char *dst, size_t cap) {
    size_t n = (size_t)(end - start);
    if (n + 1 > cap) {
        return (size_t)-1;
    }
    memcpy(dst, start, n);
    dst[n] = '\0';
    return n;
}

/*
 * ENTRY: int kv_parse(const char *line, char *key, size_t key_cap, char *val, size_t val_cap)
 *
 * Parse a single config line of the form "key = value".
 * Returns:
 *    1  -> a key/value pair was parsed (written to key/val)
 *    0  -> line was blank or a comment (nothing written)
 *   -1  -> malformed (no '=', empty key) or a buffer was too small
 */
int kv_parse(const char *line, char *key, size_t key_cap, char *val, size_t val_cap) {
    if (line == NULL || key == NULL || val == NULL) {
        return -1;
    }

    const char *p = skip_ws(line);
    if (*p == '\0' || *p == '#') {
        return 0; /* blank or comment */
    }

    const char *eq = strchr(p, '=');
    if (eq == NULL) {
        return -1;
    }

    /* Key span: from p up to eq, right-trimmed. */
    const char *key_start = p;
    const char *key_end = rtrim(key_start, eq);
    if (key_end == key_start) {
        return -1; /* empty key */
    }

    /* Value span: after '=', left- and right-trimmed. */
    const char *val_start = skip_ws(eq + 1);
    const char *val_end = rtrim(val_start, val_start + strlen(val_start));

    /* Strip a single pair of matching surrounding double quotes. */
    if (val_end - val_start >= 2 && val_start[0] == '"' && val_end[-1] == '"') {
        val_start++;
        val_end--;
    }

    if (copy_span(key_start, key_end, key, key_cap) == (size_t)-1) {
        return -1;
    }
    if (copy_span(val_start, val_end, val, val_cap) == (size_t)-1) {
        return -1;
    }
    return 1;
}

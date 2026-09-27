/* word_tokens.c
 * An argv-style char** processor: given an array of NUL-terminated
 * words, compute a folded hash over the lexicographically-sorted,
 * de-duplicated set of words. Heavy on char** (pointer-to-pointer).
 */

#include <string.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>

/* FNV-1a hash of a single word. */
static uint64_t hash_word(const char *w)
{
    uint64_t h = 1469598103934665603ULL;
    for (const char *p = w; *p; p++) {
        h ^= (uint64_t)(unsigned char)(*p);
        h *= 1099511628211ULL;
    }
    return h;
}

/* In-place insertion sort of a char** array by strcmp order. */
static void sort_words(char **words, size_t n)
{
    for (size_t i = 1; i < n; i++) {
        char *key = words[i];
        size_t j = i;
        while (j > 0 && strcmp(words[j - 1], key) > 0) {
            words[j] = words[j - 1];
            j--;
        }
        words[j] = key;
    }
}

/* Make a heap copy of the char** array (shallow: copies the pointers). */
static char **copy_view(char **words, size_t n)
{
    char **v = (char **)malloc(n * sizeof(char *));
    if (!v)
        return NULL;
    for (size_t i = 0; i < n; i++)
        v[i] = words[i];
    return v;
}

/* ENTRY: uint64_t fold_unique_words(char **words, size_t count)
 * Sorts a private copy of the pointer array, skips adjacent duplicates,
 * and folds per-word hashes into a single value. `words` is not modified.
 * Allocates and frees a temporary pointer-array view.
 */
// ENTRY: uint64_t fold_unique_words(char **words, size_t count)
uint64_t fold_unique_words(char **words, size_t count)
{
    if (!words || count == 0)
        return 0;

    char **view = copy_view(words, count);
    if (!view)
        return 0;

    sort_words(view, count);

    uint64_t acc = 0;
    const char *prev = NULL;
    for (size_t i = 0; i < count; i++) {
        if (prev && strcmp(prev, view[i]) == 0)
            continue; /* skip duplicate */
        acc = (acc * 31) ^ hash_word(view[i]);
        prev = view[i];
    }

    free(view);
    return acc;
}

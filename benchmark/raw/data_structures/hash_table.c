/* hash_table.c
 * A small open-addressing hash table mapping int keys to int values, using
 * linear probing with tombstone-free deletion-by-skip (this table only supports
 * insert and lookup, which is enough for a differential fuzz target). The table
 * grows and rehashes when the load factor gets high.
 *
 * The entry function consumes an operation stream and returns an aggregate that
 * depends on insert/lookup behaviour. All memory is freed before returning.
 *
 * Only standard libc is used. No I/O, no randomness, no global state.
 */

#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>

typedef struct {
    int key;
    int value;
    uint8_t used; /* 0 = empty slot, 1 = occupied */
} Slot;

typedef struct {
    Slot *slots;
    size_t cap;   /* always a power of two, > 0 */
    size_t count; /* number of occupied slots */
} HashTable;

/* Mix an int key into a table index. */
static size_t ht_hash(int key, size_t cap) {
    /* Treat key bits as unsigned and apply a simple integer mix. */
    uint32_t x = (uint32_t)key;
    x ^= x >> 16;
    x *= 0x7feb352dU;
    x ^= x >> 15;
    x *= 0x846ca68bU;
    x ^= x >> 16;
    return (size_t)x & (cap - 1);
}

/* Insert directly into a slot array (no growth check); used by insert+rehash.
 * Updates value if key already present. Returns 1 if a new key was added. */
static int ht_insert_into(Slot *slots, size_t cap, int key, int value) {
    size_t idx = ht_hash(key, cap);
    for (size_t probe = 0; probe < cap; probe++) {
        Slot *s = &slots[idx];
        if (!s->used) {
            s->key = key;
            s->value = value;
            s->used = 1;
            return 1;
        }
        if (s->key == key) {
            s->value = value;
            return 0;
        }
        idx = (idx + 1) & (cap - 1);
    }
    return 0; /* full table: should not happen given growth policy */
}

/* Initialize with a starting capacity. Returns 0 ok, -1 on failure. */
static int ht_init(HashTable *t) {
    t->cap = 8;
    t->count = 0;
    t->slots = (Slot *)calloc(t->cap, sizeof(Slot));
    return (t->slots == NULL) ? -1 : 0;
}

/* Double capacity and reinsert all entries. Returns 0 ok, -1 on failure. */
static int ht_grow(HashTable *t) {
    size_t new_cap = t->cap * 2;
    Slot *ns = (Slot *)calloc(new_cap, sizeof(Slot));
    if (ns == NULL) {
        return -1;
    }
    for (size_t i = 0; i < t->cap; i++) {
        if (t->slots[i].used) {
            ht_insert_into(ns, new_cap, t->slots[i].key, t->slots[i].value);
        }
    }
    free(t->slots);
    t->slots = ns;
    t->cap = new_cap;
    return 0;
}

/* Insert or update a key. Returns 0 ok, -1 on allocation failure. */
static int ht_insert(HashTable *t, int key, int value) {
    /* Grow when load factor would exceed ~0.7. */
    if ((t->count + 1) * 10 >= t->cap * 7) {
        if (ht_grow(t) != 0) {
            return -1;
        }
    }
    t->count += (size_t)ht_insert_into(t->slots, t->cap, key, value);
    return 0;
}

/* Look up a key. Returns 0 if found (and writes *out), -1 if absent. */
static int ht_lookup(const HashTable *t, int key, int *out) {
    size_t idx = ht_hash(key, t->cap);
    for (size_t probe = 0; probe < t->cap; probe++) {
        const Slot *s = &t->slots[idx];
        if (!s->used) {
            return -1;
        }
        if (s->key == key) {
            *out = s->value;
            return 0;
        }
        idx = (idx + 1) & (t->cap - 1);
    }
    return -1;
}

/* Free the table storage. */
static void ht_free(HashTable *t) {
    free(t->slots);
    t->slots = NULL;
    t->cap = 0;
    t->count = 0;
}

/* ENTRY: long ht_run(const int *ops, size_t n)
 *
 * Drive the hash table from a flat operation stream.
 *   At each step read one selector `sel`:
 *     sel % 2 == 0 -> insert: consume key, then value; store key->value.
 *     sel % 2 == 1 -> lookup: consume key; if present, add its value to acc.
 * Returns: acc (sum of values returned by successful lookups) plus the final
 * entry count. Frees all memory before returning.
 */
// ENTRY: long ht_run(const int *ops, size_t n)
long ht_run(const int *ops, size_t n) {
    HashTable t;
    if (ht_init(&t) != 0) {
        return -1;
    }

    long acc = 0;
    size_t i = 0;
    while (i < n) {
        int sel = ops[i++];
        if ((sel & 1) == 0) {
            int key = 0, value = 0;
            if (i < n) key = ops[i++];
            if (i < n) value = ops[i++];
            ht_insert(&t, key, value);
        } else {
            int key = 0, out = 0;
            if (i < n) key = ops[i++];
            if (ht_lookup(&t, key, &out) == 0) {
                acc += out;
            }
        }
    }

    long result = acc + (long)t.count;
    ht_free(&t);
    return result;
}

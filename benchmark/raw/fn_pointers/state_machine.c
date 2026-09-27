/* state_machine.c
 * A small turnstile-style finite state machine driven by an event stream.
 * Transitions are resolved through an internal 2D table of handler functions
 * (indirect calls), each returning the next state.
 */
#include <stddef.h>
#include <stdint.h>

typedef enum {
    ST_LOCKED = 0,
    ST_UNLOCKED = 1,
    ST_COUNT = 2
} state_t;

typedef enum {
    EV_COIN = 0,
    EV_PUSH = 1,
    EV_COUNT = 2
} event_t;

/* Shared context counters mutated by handlers. */
typedef struct {
    int32_t opened;     /* number of successful pushes through */
    int32_t coins;      /* coins inserted */
    int32_t rejected;   /* invalid attempts */
} ctx_t;

static state_t on_coin_locked(ctx_t *c) {
    c->coins++;
    return ST_UNLOCKED;
}

static state_t on_push_locked(ctx_t *c) {
    c->rejected++;
    return ST_LOCKED;
}

static state_t on_coin_unlocked(ctx_t *c) {
    c->coins++;     /* extra coin, stays unlocked */
    return ST_UNLOCKED;
}

static state_t on_push_unlocked(ctx_t *c) {
    c->opened++;
    return ST_LOCKED;
}

typedef state_t (*transition_fn)(ctx_t *);

static transition_fn lookup(state_t s, event_t e) {
    static transition_fn tbl[ST_COUNT][EV_COUNT] = {
        /* ST_LOCKED   */ { on_coin_locked,   on_push_locked   },
        /* ST_UNLOCKED */ { on_coin_unlocked, on_push_unlocked },
    };
    if (s < ST_COUNT && e < EV_COUNT) {
        return tbl[s][e];
    }
    return NULL;
}

// ENTRY: int32_t simulate(const uint8_t *events, size_t n, int32_t *out_opened)
int32_t simulate(const uint8_t *events, size_t n, int32_t *out_opened) {
    ctx_t c = { 0, 0, 0 };
    state_t s = ST_LOCKED;
    if (events == NULL) {
        if (out_opened != NULL) *out_opened = 0;
        return (int32_t)s;
    }
    for (size_t i = 0; i < n; i++) {
        event_t e = (events[i] & 1u) ? EV_PUSH : EV_COIN;
        transition_fn fn = lookup(s, e);
        if (fn != NULL) {
            s = fn(&c);
        }
    }
    if (out_opened != NULL) {
        *out_opened = c.opened;
    }
    return (int32_t)s;
}

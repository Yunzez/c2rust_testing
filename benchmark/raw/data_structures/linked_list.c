/* linked_list.c
 * A singly linked list of int values supporting head insertion, a recursive
 * in-place reverse, and a sum. The entry function builds a list from an int
 * array, reverses it, and returns the sum (order-independent, but reverse is
 * exercised and the result must match across translations). All nodes are freed.
 *
 * Only standard libc is used. No I/O, no randomness, no global state.
 */

#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>

typedef struct Node {
    int value;
    struct Node *next;
} Node;

/* Prepend a value, returning the new head. On allocation failure returns the
 * old head unchanged (the value is dropped). */
static Node *ll_push_front(Node *head, int value) {
    Node *n = (Node *)malloc(sizeof(Node));
    if (n == NULL) {
        return head;
    }
    n->value = value;
    n->next = head;
    return n;
}

/* Recursively reverse the list, returning the new head.
 * `prev` is the already-reversed prefix; call with prev == NULL. */
static Node *ll_reverse_rec(Node *cur, Node *prev) {
    if (cur == NULL) {
        return prev;
    }
    Node *next = cur->next;
    cur->next = prev;
    return ll_reverse_rec(next, cur);
}

/* Sum all values in the list. */
static long ll_sum(const Node *head) {
    long s = 0;
    for (const Node *p = head; p != NULL; p = p->next) {
        s += p->value;
    }
    return s;
}

/* Free every node in the list. */
static void ll_free(Node *head) {
    while (head != NULL) {
        Node *next = head->next;
        free(head);
        head = next;
    }
}

/* ENTRY: long ll_run(const int *vals, size_t n)
 *
 * Build a list by prepending each of the n values, recursively reverse it,
 * then return the sum of all values. The reversal does not change the sum but
 * is exercised for its pointer manipulation. Frees all nodes before returning.
 */
// ENTRY: long ll_run(const int *vals, size_t n)
long ll_run(const int *vals, size_t n) {
    Node *head = NULL;
    for (size_t i = 0; i < n; i++) {
        head = ll_push_front(head, vals[i]);
    }

    head = ll_reverse_rec(head, NULL);

    long result = ll_sum(head);
    ll_free(head);
    return result;
}

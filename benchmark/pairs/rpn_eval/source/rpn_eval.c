/*
 * rpn_eval.c
 *
 * Stack-based reverse-polish-notation (RPN) integer expression evaluator.
 * The program is given as a byte stream of tokens; numbers are encoded as
 * a 'N' tag byte followed by a 4-byte little-endian int32, and operators
 * as single ASCII bytes ('+', '-', '*', '/', '%'). The evaluator runs a
 * small fixed-capacity operand stack as a state machine.
 *
 * Self-contained; standard libc only. No recursion (iterative VM loop),
 * pointer-heavy over the input/stack.
 *
 * Complexity: 4 functions, no recursion, pointer-heavy, branchy dispatch.
 */

#include <stdint.h>
#include <stddef.h>
#include <string.h>

#define RPN_STACK_MAX 64

/* Status codes returned via the out-param of the entry function. */
enum {
    RPN_OK = 0,
    RPN_ERR_UNDERFLOW = 1,  /* operator with too few operands */
    RPN_ERR_OVERFLOW = 2,   /* operand stack exceeded capacity */
    RPN_ERR_DIVZERO = 3,    /* division or modulo by zero */
    RPN_ERR_TRUNC = 4,      /* number tag without enough following bytes */
    RPN_ERR_BADTOK = 5,     /* unrecognized token byte */
    RPN_ERR_RESULT = 6      /* did not end with exactly one value */
};

/* Decode a little-endian int32 from four bytes. */
static int32_t read_le32(const uint8_t *p) {
    uint32_t v = (uint32_t)p[0]
               | ((uint32_t)p[1] << 8)
               | ((uint32_t)p[2] << 16)
               | ((uint32_t)p[3] << 24);
    return (int32_t)v;
}

/* Apply binary operator `op` to (lhs, rhs), writing the result to *out.
 * Returns an RPN_* status (RPN_OK on success). */
static int apply_op(uint8_t op, int64_t lhs, int64_t rhs, int64_t *out) {
    switch (op) {
        case '+': *out = lhs + rhs; return RPN_OK;
        case '-': *out = lhs - rhs; return RPN_OK;
        case '*': *out = lhs * rhs; return RPN_OK;
        case '/':
            if (rhs == 0) return RPN_ERR_DIVZERO;
            *out = lhs / rhs;
            return RPN_OK;
        case '%':
            if (rhs == 0) return RPN_ERR_DIVZERO;
            *out = lhs % rhs;
            return RPN_OK;
        default:
            return RPN_ERR_BADTOK;
    }
}

/* Push a value onto the stack with capacity checking. */
static int stack_push(int64_t *stk, size_t *sp, int64_t v) {
    if (*sp >= RPN_STACK_MAX) {
        return RPN_ERR_OVERFLOW;
    }
    stk[(*sp)++] = v;
    return RPN_OK;
}

/*
 * Entry function. Evaluates the RPN token stream `prog` of length `len`.
 * On success returns RPN_OK and writes the final integer result to
 * *result (using 64-bit accumulation to avoid overflow UB). On any error
 * returns the corresponding RPN_* code and leaves *result unspecified.
 *
 * Bounds: the input cursor strictly advances each iteration, so the loop
 * is bounded by `len`; the stack is bounded by RPN_STACK_MAX. Safe for
 * len == 0 (returns RPN_ERR_RESULT) and for prog == NULL when len == 0.
 */
// ENTRY: int rpn_eval(const uint8_t *prog, size_t len, int64_t *result)
int rpn_eval(const uint8_t *prog, size_t len, int64_t *result) {
    int64_t stk[RPN_STACK_MAX];
    size_t sp = 0;
    size_t i = 0;

    while (i < len) {
        uint8_t tok = prog[i];
        if (tok == 'N') {
            if (len - i < 5) {
                return RPN_ERR_TRUNC;
            }
            int32_t num = read_le32(prog + i + 1);
            int st = stack_push(stk, &sp, (int64_t)num);
            if (st != RPN_OK) {
                return st;
            }
            i += 5;
        } else if (tok == '+' || tok == '-' || tok == '*'
                   || tok == '/' || tok == '%') {
            if (sp < 2) {
                return RPN_ERR_UNDERFLOW;
            }
            int64_t rhs = stk[--sp];
            int64_t lhs = stk[--sp];
            int64_t res;
            int st = apply_op(tok, lhs, rhs, &res);
            if (st != RPN_OK) {
                return st;
            }
            st = stack_push(stk, &sp, res);
            if (st != RPN_OK) {
                return st;
            }
            i += 1;
        } else if (tok == ' ' || tok == '\t' || tok == '\n') {
            i += 1; /* skip whitespace separators */
        } else {
            return RPN_ERR_BADTOK;
        }
    }

    if (sp != 1) {
        return RPN_ERR_RESULT;
    }
    *result = stk[0];
    return RPN_OK;
}

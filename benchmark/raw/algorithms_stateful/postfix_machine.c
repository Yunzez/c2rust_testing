/*
 * postfix_machine.c
 *
 * A tiny stack machine for a postfix opcode buffer. Opcodes:
 *   0x01 v : push the signed byte v
 *   0x02   : add (pop two, push sum)
 *   0x03   : dup (push a copy of the top)
 *   0x04   : pop (discard the top)
 * The operand stack is a fixed array, but the PUSH and DUP paths advance the
 * stack pointer WITHOUT checking it against the stack capacity.
 *
 * INVALID: a program with more pushes/dups than the stack can hold drives the
 * stack index `sp` past the end of the fixed `stack[]` array, causing an
 * out-of-bounds store (stack overflow write / UB). The bad index comes purely
 * from the input program, with no bounds check on the upper side.
 *
 * Self-contained; standard libc only. No recursion.
 *
 * Complexity: 2 functions, opcode dispatch, unchecked push.
 */

#include <stdint.h>
#include <stddef.h>

#define PM_STACK_MAX 16

/* Pop and return the top of stack, decrementing *sp (underflow guarded). */
static int64_t pop(int64_t *stack, size_t *sp) {
    if (*sp == 0) {
        return 0; /* underflow treated as 0 */
    }
    return stack[--(*sp)];
}

/*
 * Execute the postfix program `code[0..len-1]`, writing the final top-of-stack
 * to *out (0 if the stack ends empty). Returns the final stack depth.
 *
 * BUG: pushes (opcode 0x01) and dups (opcode 0x03) do `stack[sp++] = ...`
 * with NO check that sp < PM_STACK_MAX. A program of, say, 20 push opcodes
 * writes past stack[15] into stack[16], stack[17], ... -- an out-of-bounds
 * write whose index is fully controlled by the input length/contents.
 */
// ENTRY: size_t postfix_run(const uint8_t *code, size_t len, int64_t *out)
size_t postfix_run(const uint8_t *code, size_t len, int64_t *out) {
    int64_t stack[PM_STACK_MAX];
    size_t sp = 0;
    size_t i = 0;

    while (i < len) {
        uint8_t op = code[i++];
        switch (op) {
            case 0x01:
                if (i < len) {
                    int8_t v = (int8_t)code[i++];
                    stack[sp++] = (int64_t)v; /* no sp < MAX check (OOB) */
                }
                break;
            case 0x02: {
                int64_t b = pop(stack, &sp);
                int64_t a = pop(stack, &sp);
                stack[sp++] = a + b;          /* no sp < MAX check (OOB) */
                break;
            }
            case 0x03: {
                int64_t top = (sp > 0) ? stack[sp - 1] : 0;
                stack[sp++] = top;            /* no sp < MAX check (OOB) */
                break;
            }
            case 0x04:
                (void)pop(stack, &sp);
                break;
            default:
                break; /* ignore unknown opcodes */
        }
    }

    *out = (sp > 0) ? stack[sp - 1] : 0;
    return sp;
}

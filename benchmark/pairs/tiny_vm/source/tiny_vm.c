/*
 * tiny_vm.c
 *
 * A tiny stack-based bytecode virtual machine. The program is an array of
 * opcode bytes; the VM executes it as a state machine with a value stack,
 * a small register file, conditional/unconditional jumps, and a bounded
 * instruction budget that guarantees termination on every input.
 *
 * Instruction set (one byte opcode, some with operands):
 *   0x00 HALT
 *   0x01 PUSH  imm32 (LE)     push immediate
 *   0x02 POP                  drop top
 *   0x03 ADD                  a b -> a+b
 *   0x04 SUB                  a b -> a-b
 *   0x05 MUL                  a b -> a*b
 *   0x06 DIV                  a b -> a/b   (trap on b==0)
 *   0x07 DUP                  x -> x x
 *   0x08 SWAP                 a b -> b a
 *   0x09 LOAD  reg8           push reg[reg8]
 *   0x0A STORE reg8           pop -> reg[reg8]
 *   0x0B JMP   off16 (LE)     ip = off16 (absolute)
 *   0x0C JZ    off16 (LE)     pop; if ==0 ip = off16
 *   0x0D JNZ   off16 (LE)     pop; if !=0 ip = off16
 *   0x0E NEG                  x -> -x
 *   0x0F CMP                  a b -> (a<b?-1: a>b?1: 0)
 *
 * Self-contained; standard libc only. Recursion is used in a helper that
 * computes a checksum of the value stack (recursive folding) to add call
 * depth. Pointer-heavy over the program/stack/registers.
 *
 * Complexity: 5 functions, recursive helper, pointer-heavy, rich
 * control flow (dispatch + jumps + traps).
 */

#include <stdint.h>
#include <stddef.h>

#define VM_STACK_MAX  128
#define VM_NUM_REGS   8
#define VM_MAX_STEPS  100000

/* Trap / status codes for the VM result. */
enum {
    VM_OK = 0,            /* HALT reached cleanly */
    VM_TRAP_STACK_OVER = 1,
    VM_TRAP_STACK_UNDER = 2,
    VM_TRAP_DIVZERO = 3,
    VM_TRAP_BAD_OP = 4,
    VM_TRAP_OOB_OPERAND = 5, /* operand bytes run past program end */
    VM_TRAP_BAD_JUMP = 6,    /* jump target outside program */
    VM_TRAP_BAD_REG = 7,
    VM_TRAP_STEP_LIMIT = 8   /* exceeded instruction budget */
};

/* VM machine state, passed by pointer to keep functions cohesive. */
typedef struct {
    int32_t stack[VM_STACK_MAX];
    size_t  sp;
    int32_t regs[VM_NUM_REGS];
} vm_state;

/* Read a little-endian uint16 from program bytes. */
static uint32_t read_u16(const uint8_t *p) {
    return (uint32_t)p[0] | ((uint32_t)p[1] << 8);
}

/* Read a little-endian int32 from program bytes. */
static int32_t read_i32(const uint8_t *p) {
    uint32_t v = (uint32_t)p[0]
               | ((uint32_t)p[1] << 8)
               | ((uint32_t)p[2] << 16)
               | ((uint32_t)p[3] << 24);
    return (int32_t)v;
}

/* Recursive XOR-fold checksum of stack slots [0, count). Recursion depth
 * is bounded by the stack size (<= VM_STACK_MAX). */
static int32_t stack_checksum(const int32_t *s, size_t count) {
    if (count == 0) {
        return 0;
    }
    int32_t rest = stack_checksum(s, count - 1);
    /* mix in the next slot with a rotate-ish step */
    int32_t v = s[count - 1];
    return (int32_t)((uint32_t)rest * 31u + (uint32_t)v);
}

/* Execute a single instruction at *ip over program `prog`/`len`.
 * Advances *ip (including operand bytes / jumps). Returns VM_OK to
 * continue, a positive VM_TRAP_* code to abort, or -1 to signal HALT. */
static int vm_step(vm_state *vm, const uint8_t *prog, size_t len, size_t *ip) {
    uint8_t op = prog[*ip];
    size_t pc = *ip;

    switch (op) {
        case 0x00: /* HALT */
            *ip = pc + 1;
            return -1;

        case 0x01: { /* PUSH imm32 */
            if (len - pc < 5) return VM_TRAP_OOB_OPERAND;
            if (vm->sp >= VM_STACK_MAX) return VM_TRAP_STACK_OVER;
            vm->stack[vm->sp++] = read_i32(prog + pc + 1);
            *ip = pc + 5;
            return VM_OK;
        }
        case 0x02: /* POP */
            if (vm->sp < 1) return VM_TRAP_STACK_UNDER;
            vm->sp--;
            *ip = pc + 1;
            return VM_OK;

        case 0x03: /* ADD */
        case 0x04: /* SUB */
        case 0x05: /* MUL */
        case 0x06: /* DIV */
        case 0x0F: /* CMP */
        case 0x08: { /* SWAP (binary-shaped: needs two operands) */
            if (vm->sp < 2) return VM_TRAP_STACK_UNDER;
            int32_t b = vm->stack[vm->sp - 1];
            int32_t a = vm->stack[vm->sp - 2];
            int32_t r;
            if (op == 0x03) {
                r = (int32_t)((uint32_t)a + (uint32_t)b);
            } else if (op == 0x04) {
                r = (int32_t)((uint32_t)a - (uint32_t)b);
            } else if (op == 0x05) {
                r = (int32_t)((uint32_t)a * (uint32_t)b);
            } else if (op == 0x06) {
                if (b == 0) return VM_TRAP_DIVZERO;
                /* guard INT_MIN / -1 overflow */
                if (a == (int32_t)0x80000000 && b == -1) {
                    r = a;
                } else {
                    r = a / b;
                }
            } else if (op == 0x0F) {
                r = (a < b) ? -1 : (a > b ? 1 : 0);
            } else { /* SWAP */
                vm->stack[vm->sp - 1] = a;
                vm->stack[vm->sp - 2] = b;
                *ip = pc + 1;
                return VM_OK;
            }
            vm->sp -= 1;            /* pop two, push one => net -1 */
            vm->stack[vm->sp - 1] = r;
            *ip = pc + 1;
            return VM_OK;
        }

        case 0x07: /* DUP */
            if (vm->sp < 1) return VM_TRAP_STACK_UNDER;
            if (vm->sp >= VM_STACK_MAX) return VM_TRAP_STACK_OVER;
            vm->stack[vm->sp] = vm->stack[vm->sp - 1];
            vm->sp++;
            *ip = pc + 1;
            return VM_OK;

        case 0x09: { /* LOAD reg8 */
            if (len - pc < 2) return VM_TRAP_OOB_OPERAND;
            uint8_t reg = prog[pc + 1];
            if (reg >= VM_NUM_REGS) return VM_TRAP_BAD_REG;
            if (vm->sp >= VM_STACK_MAX) return VM_TRAP_STACK_OVER;
            vm->stack[vm->sp++] = vm->regs[reg];
            *ip = pc + 2;
            return VM_OK;
        }
        case 0x0A: { /* STORE reg8 */
            if (len - pc < 2) return VM_TRAP_OOB_OPERAND;
            uint8_t reg = prog[pc + 1];
            if (reg >= VM_NUM_REGS) return VM_TRAP_BAD_REG;
            if (vm->sp < 1) return VM_TRAP_STACK_UNDER;
            vm->regs[reg] = vm->stack[--vm->sp];
            *ip = pc + 2;
            return VM_OK;
        }

        case 0x0B: { /* JMP off16 */
            if (len - pc < 3) return VM_TRAP_OOB_OPERAND;
            uint32_t tgt = read_u16(prog + pc + 1);
            if (tgt >= len) return VM_TRAP_BAD_JUMP;
            *ip = (size_t)tgt;
            return VM_OK;
        }
        case 0x0C: /* JZ off16 */
        case 0x0D: { /* JNZ off16 */
            if (len - pc < 3) return VM_TRAP_OOB_OPERAND;
            if (vm->sp < 1) return VM_TRAP_STACK_UNDER;
            int32_t cond = vm->stack[--vm->sp];
            uint32_t tgt = read_u16(prog + pc + 1);
            int take = (op == 0x0C) ? (cond == 0) : (cond != 0);
            if (take) {
                if (tgt >= len) return VM_TRAP_BAD_JUMP;
                *ip = (size_t)tgt;
            } else {
                *ip = pc + 3;
            }
            return VM_OK;
        }

        case 0x0E: /* NEG */
            if (vm->sp < 1) return VM_TRAP_STACK_UNDER;
            vm->stack[vm->sp - 1] =
                (int32_t)(-(uint32_t)vm->stack[vm->sp - 1]);
            *ip = pc + 1;
            return VM_OK;

        default:
            return VM_TRAP_BAD_OP;
    }
}

/*
 * Entry function. Runs the bytecode program `prog` of length `len` to
 * completion (HALT), a trap, or the step-limit. Writes a deterministic
 * 32-bit summary of the final value stack (XOR/multiply fold via the
 * recursive helper) to *out_value. Returns VM_OK on clean HALT or a
 * positive VM_TRAP_* code otherwise.
 *
 * Bounds: each accepted step either advances ip or jumps within [0,len);
 * the explicit VM_MAX_STEPS budget guarantees termination even with
 * back-edges/loops in the bytecode. Safe for len == 0 (returns VM_OK
 * with checksum 0) and prog == NULL when len == 0.
 */
// ENTRY: int vm_run(const uint8_t *prog, size_t len, int32_t *out_value)
int vm_run(const uint8_t *prog, size_t len, int32_t *out_value) {
    vm_state vm;
    vm.sp = 0;
    for (size_t r = 0; r < VM_NUM_REGS; r++) {
        vm.regs[r] = 0;
    }

    size_t ip = 0;
    int status = VM_OK;
    size_t steps = 0;

    while (ip < len) {
        if (steps >= VM_MAX_STEPS) {
            status = VM_TRAP_STEP_LIMIT;
            break;
        }
        steps++;

        int rc = vm_step(&vm, prog, len, &ip);
        if (rc == -1) {        /* HALT */
            status = VM_OK;
            break;
        }
        if (rc != VM_OK) {     /* trap */
            status = rc;
            break;
        }
    }

    *out_value = stack_checksum(vm.stack, vm.sp);
    return status;
}

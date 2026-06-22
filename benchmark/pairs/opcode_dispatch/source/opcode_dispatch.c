/* opcode_dispatch.c
 * A tiny stack machine that executes a byte stream of opcodes by indexing
 * into an internal dispatch table of operation handlers (indirect calls).
 */
#include <stddef.h>
#include <stdint.h>

#define VM_STACK_MAX 64

typedef struct {
    int32_t stack[VM_STACK_MAX];
    int32_t sp; /* points to next free slot */
} VM;

static void vm_push(VM *vm, int32_t v) {
    if (vm->sp < VM_STACK_MAX) {
        vm->stack[vm->sp++] = v;
    }
}

static int32_t vm_pop(VM *vm) {
    if (vm->sp > 0) {
        return vm->stack[--vm->sp];
    }
    return 0;
}

/* Each handler consumes an operand byte and mutates the VM. */
static void op_push(VM *vm, uint8_t operand) {
    vm_push(vm, (int32_t)operand);
}

static void op_add(VM *vm, uint8_t operand) {
    (void)operand;
    int32_t b = vm_pop(vm);
    int32_t a = vm_pop(vm);
    vm_push(vm, a + b);
}

static void op_mul(VM *vm, uint8_t operand) {
    (void)operand;
    int32_t b = vm_pop(vm);
    int32_t a = vm_pop(vm);
    vm_push(vm, a * b);
}

static void op_dup(VM *vm, uint8_t operand) {
    (void)operand;
    int32_t a = vm_pop(vm);
    vm_push(vm, a);
    vm_push(vm, a);
}

typedef void (*op_handler)(VM *, uint8_t);

/* Opcodes: 0=PUSH(operand), 1=ADD, 2=MUL, 3=DUP. */
static op_handler dispatch_table(uint8_t opcode) {
    static op_handler table[4] = { op_push, op_add, op_mul, op_dup };
    if (opcode < 4) {
        return table[opcode];
    }
    return NULL;
}

// ENTRY: int32_t run_program(const uint8_t *code, size_t len)
int32_t run_program(const uint8_t *code, size_t len) {
    VM vm = { { 0 }, 0 };
    if (code == NULL) {
        return 0;
    }
    /* Stream is pairs of (opcode, operand). */
    for (size_t i = 0; i + 1 < len; i += 2) {
        uint8_t opcode = code[i];
        uint8_t operand = code[i + 1];
        op_handler h = dispatch_table(opcode);
        if (h != NULL) {
            h(&vm, operand);
        }
    }
    return vm.sp > 0 ? vm.stack[vm.sp - 1] : 0;
}

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VM {
    pub stack: [int32_t; 64],
    pub sp: int32_t,
}
pub type op_handler = Option<unsafe extern "C" fn(*mut VM, uint8_t) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const VM_STACK_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn vm_push(mut vm: *mut VM, mut v: int32_t) {
    if (*vm).sp < VM_STACK_MAX as int32_t {
        let fresh0 = (*vm).sp;
        (*vm).sp = (*vm).sp + 1;
        (*vm).stack[fresh0 as usize] = v;
    }
}
unsafe extern "C" fn vm_pop(mut vm: *mut VM) -> int32_t {
    if (*vm).sp > 0 as int32_t {
        (*vm).sp -= 1;
        return (*vm).stack[(*vm).sp as usize];
    }
    return 0 as int32_t;
}
unsafe extern "C" fn op_push(mut vm: *mut VM, mut operand: uint8_t) {
    vm_push(vm, operand as int32_t);
}
unsafe extern "C" fn op_add(mut vm: *mut VM, mut operand: uint8_t) {
    let mut b: int32_t = vm_pop(vm);
    let mut a: int32_t = vm_pop(vm);
    vm_push(vm, a + b);
}
unsafe extern "C" fn op_mul(mut vm: *mut VM, mut operand: uint8_t) {
    let mut b: int32_t = vm_pop(vm);
    let mut a: int32_t = vm_pop(vm);
    vm_push(vm, a * b);
}
unsafe extern "C" fn op_dup(mut vm: *mut VM, mut operand: uint8_t) {
    let mut a: int32_t = vm_pop(vm);
    vm_push(vm, a);
    vm_push(vm, a);
}
unsafe extern "C" fn dispatch_table(mut opcode: uint8_t) -> op_handler {
    static mut table: [op_handler; 4] = unsafe {
        [
            Some(op_push as unsafe extern "C" fn(*mut VM, uint8_t) -> ()),
            Some(op_add as unsafe extern "C" fn(*mut VM, uint8_t) -> ()),
            Some(op_mul as unsafe extern "C" fn(*mut VM, uint8_t) -> ()),
            Some(op_dup as unsafe extern "C" fn(*mut VM, uint8_t) -> ()),
        ]
    };
    if (opcode as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        return table[opcode as usize];
    }
    return None;
}
#[no_mangle]
pub unsafe extern "C" fn run_program(mut code: *const uint8_t, mut len: size_t) -> int32_t {
    let mut vm: VM = VM {
        stack: [0 as ::core::ffi::c_int; 64],
        sp: 0 as int32_t,
    };
    if code.is_null() {
        return 0 as int32_t;
    }
    let mut i: size_t = 0 as size_t;
    while i.wrapping_add(1 as size_t) < len {
        let mut opcode: uint8_t = *code.offset(i as isize);
        let mut operand: uint8_t = *code.offset(i.wrapping_add(1 as size_t) as isize);
        let mut h: op_handler = dispatch_table(opcode);
        if h.is_some() {
            h.expect("non-null function pointer")(&raw mut vm, operand);
        }
        i = i.wrapping_add(2 as size_t);
    }
    return if vm.sp > 0 as int32_t {
        vm.stack[(vm.sp - 1 as int32_t) as usize]
    } else {
        0 as int32_t
    };
}

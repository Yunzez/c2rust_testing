#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const VM_TRAP_STEP_LIMIT: C2RustUnnamed = 8;
pub const VM_TRAP_BAD_REG: C2RustUnnamed = 7;
pub const VM_TRAP_BAD_JUMP: C2RustUnnamed = 6;
pub const VM_TRAP_OOB_OPERAND: C2RustUnnamed = 5;
pub const VM_TRAP_BAD_OP: C2RustUnnamed = 4;
pub const VM_TRAP_DIVZERO: C2RustUnnamed = 3;
pub const VM_TRAP_STACK_UNDER: C2RustUnnamed = 2;
pub const VM_TRAP_STACK_OVER: C2RustUnnamed = 1;
pub const VM_OK: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vm_state {
    pub stack: [int32_t; 128],
    pub sp: size_t,
    pub regs: [int32_t; 8],
}
pub const VM_STACK_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const VM_NUM_REGS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VM_MAX_STEPS: ::core::ffi::c_int = 100000 as ::core::ffi::c_int;
unsafe extern "C" fn read_u16(mut p: *const uint8_t) -> uint32_t {
    return *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*p.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_i32(mut p: *const uint8_t) -> int32_t {
    let mut v: uint32_t = *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*p.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*p.offset(3 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int;
    return v as int32_t;
}
unsafe extern "C" fn stack_checksum(mut s: *const int32_t, mut count: size_t) -> int32_t {
    if count == 0 as size_t {
        return 0 as int32_t;
    }
    let mut rest: int32_t = stack_checksum(s, count.wrapping_sub(1 as size_t));
    let mut v: int32_t = *s.offset(count.wrapping_sub(1 as size_t) as isize);
    return (rest as uint32_t)
        .wrapping_mul(31 as uint32_t)
        .wrapping_add(v as uint32_t) as int32_t;
}
unsafe extern "C" fn vm_step(
    mut vm: *mut vm_state,
    mut prog: *const uint8_t,
    mut len: size_t,
    mut ip: *mut size_t,
) -> ::core::ffi::c_int {
    let mut op: uint8_t = *prog.offset(*ip as isize);
    let mut pc: size_t = *ip;
    let mut current_block_101: u64;
    match op as ::core::ffi::c_int {
        0 => {
            *ip = pc.wrapping_add(1 as size_t);
            return -(1 as ::core::ffi::c_int);
        }
        1 => {
            if len.wrapping_sub(pc) < 5 as size_t {
                return VM_TRAP_OOB_OPERAND as ::core::ffi::c_int;
            }
            if (*vm).sp >= VM_STACK_MAX as size_t {
                return VM_TRAP_STACK_OVER as ::core::ffi::c_int;
            }
            let fresh0 = (*vm).sp;
            (*vm).sp = (*vm).sp.wrapping_add(1);
            (*vm).stack[fresh0 as usize] = read_i32(
                prog.offset(pc as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            *ip = pc.wrapping_add(5 as size_t);
            return VM_OK as ::core::ffi::c_int;
        }
        2 => {
            if (*vm).sp < 1 as size_t {
                return VM_TRAP_STACK_UNDER as ::core::ffi::c_int;
            }
            (*vm).sp = (*vm).sp.wrapping_sub(1);
            *ip = pc.wrapping_add(1 as size_t);
            return VM_OK as ::core::ffi::c_int;
        }
        3 => {
            current_block_101 = 12865571900628044355;
        }
        4 => {
            current_block_101 = 12865571900628044355;
        }
        5 => {
            current_block_101 = 1065811845541784962;
        }
        6 => {
            current_block_101 = 14335897761403117671;
        }
        15 | 8 => {
            current_block_101 = 14427921128572470902;
        }
        7 => {
            if (*vm).sp < 1 as size_t {
                return VM_TRAP_STACK_UNDER as ::core::ffi::c_int;
            }
            if (*vm).sp >= VM_STACK_MAX as size_t {
                return VM_TRAP_STACK_OVER as ::core::ffi::c_int;
            }
            (*vm).stack[(*vm).sp as usize] =
                (*vm).stack[(*vm).sp.wrapping_sub(1 as size_t) as usize];
            (*vm).sp = (*vm).sp.wrapping_add(1);
            *ip = pc.wrapping_add(1 as size_t);
            return VM_OK as ::core::ffi::c_int;
        }
        9 => {
            if len.wrapping_sub(pc) < 2 as size_t {
                return VM_TRAP_OOB_OPERAND as ::core::ffi::c_int;
            }
            let mut reg: uint8_t = *prog.offset(pc.wrapping_add(1 as size_t) as isize);
            if reg as ::core::ffi::c_int >= VM_NUM_REGS {
                return VM_TRAP_BAD_REG as ::core::ffi::c_int;
            }
            if (*vm).sp >= VM_STACK_MAX as size_t {
                return VM_TRAP_STACK_OVER as ::core::ffi::c_int;
            }
            let fresh1 = (*vm).sp;
            (*vm).sp = (*vm).sp.wrapping_add(1);
            (*vm).stack[fresh1 as usize] = (*vm).regs[reg as usize];
            *ip = pc.wrapping_add(2 as size_t);
            return VM_OK as ::core::ffi::c_int;
        }
        10 => {
            if len.wrapping_sub(pc) < 2 as size_t {
                return VM_TRAP_OOB_OPERAND as ::core::ffi::c_int;
            }
            let mut reg_0: uint8_t = *prog.offset(pc.wrapping_add(1 as size_t) as isize);
            if reg_0 as ::core::ffi::c_int >= VM_NUM_REGS {
                return VM_TRAP_BAD_REG as ::core::ffi::c_int;
            }
            if (*vm).sp < 1 as size_t {
                return VM_TRAP_STACK_UNDER as ::core::ffi::c_int;
            }
            (*vm).sp = (*vm).sp.wrapping_sub(1);
            (*vm).regs[reg_0 as usize] = (*vm).stack[(*vm).sp as usize];
            *ip = pc.wrapping_add(2 as size_t);
            return VM_OK as ::core::ffi::c_int;
        }
        11 => {
            if len.wrapping_sub(pc) < 3 as size_t {
                return VM_TRAP_OOB_OPERAND as ::core::ffi::c_int;
            }
            let mut tgt: uint32_t = read_u16(
                prog.offset(pc as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            if tgt as size_t >= len {
                return VM_TRAP_BAD_JUMP as ::core::ffi::c_int;
            }
            *ip = tgt as size_t;
            return VM_OK as ::core::ffi::c_int;
        }
        12 | 13 => {
            if len.wrapping_sub(pc) < 3 as size_t {
                return VM_TRAP_OOB_OPERAND as ::core::ffi::c_int;
            }
            if (*vm).sp < 1 as size_t {
                return VM_TRAP_STACK_UNDER as ::core::ffi::c_int;
            }
            (*vm).sp = (*vm).sp.wrapping_sub(1);
            let mut cond: int32_t = (*vm).stack[(*vm).sp as usize];
            let mut tgt_0: uint32_t = read_u16(
                prog.offset(pc as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            let mut take: ::core::ffi::c_int =
                if op as ::core::ffi::c_int == 0xc as ::core::ffi::c_int {
                    (cond == 0 as int32_t) as ::core::ffi::c_int
                } else {
                    (cond != 0 as int32_t) as ::core::ffi::c_int
                };
            if take != 0 {
                if tgt_0 as size_t >= len {
                    return VM_TRAP_BAD_JUMP as ::core::ffi::c_int;
                }
                *ip = tgt_0 as size_t;
            } else {
                *ip = pc.wrapping_add(3 as size_t);
            }
            return VM_OK as ::core::ffi::c_int;
        }
        14 => {
            if (*vm).sp < 1 as size_t {
                return VM_TRAP_STACK_UNDER as ::core::ffi::c_int;
            }
            (*vm).stack[(*vm).sp.wrapping_sub(1 as size_t) as usize] =
                ((*vm).stack[(*vm).sp.wrapping_sub(1 as size_t) as usize] as uint32_t)
                    .wrapping_neg() as int32_t;
            *ip = pc.wrapping_add(1 as size_t);
            return VM_OK as ::core::ffi::c_int;
        }
        _ => return VM_TRAP_BAD_OP as ::core::ffi::c_int,
    }
    match current_block_101 {
        12865571900628044355 => {
            current_block_101 = 1065811845541784962;
        }
        _ => {}
    }
    match current_block_101 {
        1065811845541784962 => {
            current_block_101 = 14335897761403117671;
        }
        _ => {}
    }
    match current_block_101 {
        14335897761403117671 => {}
        _ => {}
    }
    if (*vm).sp < 2 as size_t {
        return VM_TRAP_STACK_UNDER as ::core::ffi::c_int;
    }
    let mut b: int32_t = (*vm).stack[(*vm).sp.wrapping_sub(1 as size_t) as usize];
    let mut a: int32_t = (*vm).stack[(*vm).sp.wrapping_sub(2 as size_t) as usize];
    let mut r: int32_t = 0;
    if op as ::core::ffi::c_int == 0x3 as ::core::ffi::c_int {
        r = (a as uint32_t).wrapping_add(b as uint32_t) as int32_t;
    } else if op as ::core::ffi::c_int == 0x4 as ::core::ffi::c_int {
        r = (a as uint32_t).wrapping_sub(b as uint32_t) as int32_t;
    } else if op as ::core::ffi::c_int == 0x5 as ::core::ffi::c_int {
        r = (a as uint32_t).wrapping_mul(b as uint32_t) as int32_t;
    } else if op as ::core::ffi::c_int == 0x6 as ::core::ffi::c_int {
        if b == 0 as int32_t {
            return VM_TRAP_DIVZERO as ::core::ffi::c_int;
        }
        if a == 0x80000000 as ::core::ffi::c_uint as int32_t && b == -(1 as int32_t) {
            r = a;
        } else {
            r = a / b;
        }
    } else if op as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
        r = (if a < b {
            -(1 as ::core::ffi::c_int)
        } else if a > b {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as int32_t;
    } else {
        (*vm).stack[(*vm).sp.wrapping_sub(1 as size_t) as usize] = a;
        (*vm).stack[(*vm).sp.wrapping_sub(2 as size_t) as usize] = b;
        *ip = pc.wrapping_add(1 as size_t);
        return VM_OK as ::core::ffi::c_int;
    }
    (*vm).sp = (*vm).sp.wrapping_sub(1 as size_t);
    (*vm).stack[(*vm).sp.wrapping_sub(1 as size_t) as usize] = r;
    *ip = pc.wrapping_add(1 as size_t);
    return VM_OK as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vm_run(
    mut prog: *const uint8_t,
    mut len: size_t,
    mut out_value: *mut int32_t,
) -> ::core::ffi::c_int {
    let mut vm: vm_state = vm_state {
        stack: [0; 128],
        sp: 0,
        regs: [0; 8],
    };
    vm.sp = 0 as size_t;
    let mut r: size_t = 0 as size_t;
    while r < VM_NUM_REGS as size_t {
        vm.regs[r as usize] = 0 as ::core::ffi::c_int as int32_t;
        r = r.wrapping_add(1);
    }
    let mut ip: size_t = 0 as size_t;
    let mut status: ::core::ffi::c_int = VM_OK as ::core::ffi::c_int;
    let mut steps: size_t = 0 as size_t;
    while ip < len {
        if steps >= VM_MAX_STEPS as size_t {
            status = VM_TRAP_STEP_LIMIT as ::core::ffi::c_int;
            break;
        } else {
            steps = steps.wrapping_add(1);
            let mut rc: ::core::ffi::c_int = vm_step(&raw mut vm, prog, len, &raw mut ip);
            if rc == -(1 as ::core::ffi::c_int) {
                status = VM_OK as ::core::ffi::c_int;
                break;
            } else {
                if !(rc != VM_OK as ::core::ffi::c_int) {
                    continue;
                }
                status = rc;
                break;
            }
        }
    }
    *out_value = stack_checksum(&raw mut vm.stack as *mut int32_t, vm.sp);
    return status;
}

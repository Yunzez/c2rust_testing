#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value, raw_ref_op)]
extern "C" {
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn {
    pub array: [uint32_t; 32],
}
pub type C2RustUnnamed = ::core::ffi::c_int;
pub const LARGER: C2RustUnnamed = 1;
pub const EQUAL: C2RustUnnamed = 0;
pub const SMALLER: C2RustUnnamed = -1;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WORD_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BN_ARRAY_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int / WORD_SIZE;
pub const SPRINTF_FORMAT_STR: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"%.08x\0") };
pub const SSCANF_FORMAT_STR: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"%8x\0") };
pub const MAX_VAL: uint64_t = 0xffffffff as ::core::ffi::c_uint as uint64_t;
#[no_mangle]
pub unsafe extern "C" fn bignum_init(mut n: *mut bn) {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_uint,
                b"void bignum_init(struct bn *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        (*n).array[i as usize] = 0 as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_from_int(mut n: *mut bn, mut i: uint64_t) {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                51 as ::core::ffi::c_uint,
                b"void bignum_from_int(struct bn *, uint64_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    bignum_init(n);
    (*n).array[0 as ::core::ffi::c_int as usize] = i as uint32_t;
    let mut num_32: uint64_t = 32 as uint64_t;
    let mut tmp: uint64_t = i >> num_32;
    (*n).array[1 as ::core::ffi::c_int as usize] = tmp as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn bignum_to_int(mut n: *mut bn) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                77 as ::core::ffi::c_uint,
                b"int bignum_to_int(struct bn *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret = (ret as uint32_t).wrapping_add((*n).array[0 as ::core::ffi::c_int as usize])
        as ::core::ffi::c_int as ::core::ffi::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bignum_from_string(
    mut n: *mut bn,
    mut str: *mut ::core::ffi::c_char,
    mut nbytes: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                100 as ::core::ffi::c_uint,
                b"void bignum_from_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !str.is_null()
            && !(b"str is null\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"str && \"str is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_uint,
                b"void bignum_from_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if nbytes > 0 as ::core::ffi::c_int
            && !(b"nbytes must be positive\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"nbytes > 0 && \"nbytes must be positive\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_uint,
                b"void bignum_from_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if nbytes & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && !(b"string format must be in hex -> equal number of bytes\0" as *const u8
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"(nbytes & 1) == 0 && \"string format must be in hex -> equal number of bytes\"\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_uint,
                b"void bignum_from_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (nbytes as usize)
            .wrapping_rem((::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(2 as usize))
            == 0 as usize
            && !(b"string length must be a multiple of (sizeof(DTYPE) * 2) characters\0"
                as *const u8 as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"(nbytes % (sizeof(uint32_t) * 2)) == 0 && \"string length must be a multiple of (sizeof(DTYPE) * 2) characters\"\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_uint,
                b"void bignum_from_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    bignum_init(n);
    let mut tmp: uint32_t = 0;
    let mut i: ::core::ffi::c_int = nbytes - 2 as ::core::ffi::c_int * WORD_SIZE;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        tmp = 0 as uint32_t;
        sscanf(
            str.offset(i as isize) as *mut ::core::ffi::c_char,
            SSCANF_FORMAT_STR.as_ptr(),
            &raw mut tmp,
        );
        (*n).array[j as usize] = tmp;
        i -= 2 as ::core::ffi::c_int * WORD_SIZE;
        j += 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_to_string(
    mut n: *mut bn,
    mut str: *mut ::core::ffi::c_char,
    mut nbytes: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_uint,
                b"void bignum_to_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !str.is_null()
            && !(b"str is null\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"str && \"str is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_uint,
                b"void bignum_to_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if nbytes > 0 as ::core::ffi::c_int
            && !(b"nbytes must be positive\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"nbytes > 0 && \"nbytes must be positive\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_uint,
                b"void bignum_to_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if nbytes & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && !(b"string format must be in hex -> equal number of bytes\0" as *const u8
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"(nbytes & 1) == 0 && \"string format must be in hex -> equal number of bytes\"\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                130 as ::core::ffi::c_uint,
                b"void bignum_to_string(struct bn *, char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut j: ::core::ffi::c_int = BN_ARRAY_SIZE - 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j >= 0 as ::core::ffi::c_int && nbytes > i + 1 as ::core::ffi::c_int {
        sprintf(
            str.offset(i as isize) as *mut ::core::ffi::c_char,
            SPRINTF_FORMAT_STR.as_ptr(),
            (*n).array[j as usize],
        );
        i += 2 as ::core::ffi::c_int * WORD_SIZE;
        j -= 1 as ::core::ffi::c_int;
    }
    j = 0 as ::core::ffi::c_int;
    while *str.offset(j as isize) as ::core::ffi::c_int == '0' as i32 {
        j += 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nbytes - j {
        *str.offset(i as isize) = *str.offset((i + j) as isize);
        i += 1;
    }
    *str.offset(i as isize) = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bignum_dec(mut n: *mut bn) {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                163 as ::core::ffi::c_uint,
                b"void bignum_dec(struct bn *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut tmp: uint32_t = 0;
    let mut res: uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        tmp = (*n).array[i as usize];
        res = tmp.wrapping_sub(1 as uint32_t);
        (*n).array[i as usize] = res;
        if !(res > tmp) {
            break;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_inc(mut n: *mut bn) {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                185 as ::core::ffi::c_uint,
                b"void bignum_inc(struct bn *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut res: uint32_t = 0;
    let mut tmp: uint64_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        tmp = (*n).array[i as usize] as uint64_t;
        res = tmp.wrapping_add(1 as uint64_t) as uint32_t;
        (*n).array[i as usize] = res;
        if res as uint64_t > tmp {
            break;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_add(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"void bignum_add(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                208 as ::core::ffi::c_uint,
                b"void bignum_add(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_uint,
                b"void bignum_add(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut tmp: uint64_t = 0;
    let mut carry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        tmp = ((*a).array[i as usize] as uint64_t)
            .wrapping_add((*b).array[i as usize] as uint64_t)
            .wrapping_add(carry as uint64_t);
        carry = (tmp > MAX_VAL) as ::core::ffi::c_int;
        (*c).array[i as usize] = (tmp & MAX_VAL) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_sub(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                225 as ::core::ffi::c_uint,
                b"void bignum_sub(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                226 as ::core::ffi::c_uint,
                b"void bignum_sub(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                227 as ::core::ffi::c_uint,
                b"void bignum_sub(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut res: uint64_t = 0;
    let mut tmp1: uint64_t = 0;
    let mut tmp2: uint64_t = 0;
    let mut borrow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        tmp1 =
            ((*a).array[i as usize] as uint64_t).wrapping_add(MAX_VAL.wrapping_add(1 as uint64_t));
        tmp2 = ((*b).array[i as usize] as uint64_t).wrapping_add(borrow as uint64_t);
        res = tmp1.wrapping_sub(tmp2);
        (*c).array[i as usize] = (res & MAX_VAL) as uint32_t;
        borrow = (res <= MAX_VAL) as ::core::ffi::c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_mul(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_uint,
                b"void bignum_mul(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                248 as ::core::ffi::c_uint,
                b"void bignum_mul(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                249 as ::core::ffi::c_uint,
                b"void bignum_mul(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut row: bn = bn { array: [0; 32] };
    let mut tmp: bn = bn { array: [0; 32] };
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    bignum_init(c);
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        bignum_init(&raw mut row);
        j = 0 as ::core::ffi::c_int;
        while j < BN_ARRAY_SIZE {
            if i + j < BN_ARRAY_SIZE {
                bignum_init(&raw mut tmp);
                let mut intermediate: uint64_t = ((*a).array[i as usize] as uint64_t)
                    .wrapping_mul((*b).array[j as usize] as uint64_t);
                bignum_from_int(&raw mut tmp, intermediate);
                _lshift_word(&raw mut tmp, i + j);
                bignum_add(&raw mut tmp, &raw mut row, &raw mut row);
            }
            j += 1;
        }
        bignum_add(c, &raw mut row, c);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_div(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                279 as ::core::ffi::c_uint,
                b"void bignum_div(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                280 as ::core::ffi::c_uint,
                b"void bignum_div(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                281 as ::core::ffi::c_uint,
                b"void bignum_div(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut current: bn = bn { array: [0; 32] };
    let mut denom: bn = bn { array: [0; 32] };
    let mut tmp: bn = bn { array: [0; 32] };
    bignum_from_int(&raw mut current, 1 as uint64_t);
    bignum_assign(&raw mut denom, b);
    bignum_assign(&raw mut tmp, a);
    let half_max: uint64_t = (1 as uint64_t).wrapping_add(MAX_VAL.wrapping_div(2 as uint64_t));
    let mut overflow: bool = false_0 != 0;
    while bignum_cmp(&raw mut denom, a) != LARGER as ::core::ffi::c_int {
        if denom.array[(BN_ARRAY_SIZE - 1 as ::core::ffi::c_int) as usize] as uint64_t >= half_max {
            overflow = true_0 != 0;
            break;
        } else {
            _lshift_one_bit(&raw mut current);
            _lshift_one_bit(&raw mut denom);
        }
    }
    if !overflow {
        _rshift_one_bit(&raw mut denom);
        _rshift_one_bit(&raw mut current);
    }
    bignum_init(c);
    while bignum_is_zero(&raw mut current) == 0 {
        if bignum_cmp(&raw mut tmp, &raw mut denom) != SMALLER as ::core::ffi::c_int {
            bignum_sub(&raw mut tmp, &raw mut denom, &raw mut tmp);
            bignum_or(c, &raw mut current, c);
        }
        _rshift_one_bit(&raw mut current);
        _rshift_one_bit(&raw mut denom);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_lshift(
    mut a: *mut bn,
    mut b: *mut bn,
    mut nbits: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                325 as ::core::ffi::c_uint,
                b"void bignum_lshift(struct bn *, struct bn *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                326 as ::core::ffi::c_uint,
                b"void bignum_lshift(struct bn *, struct bn *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if nbits >= 0 as ::core::ffi::c_int
            && !(b"no negative shifts\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"nbits >= 0 && \"no negative shifts\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                327 as ::core::ffi::c_uint,
                b"void bignum_lshift(struct bn *, struct bn *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    bignum_assign(b, a);
    let nbits_pr_word: ::core::ffi::c_int = WORD_SIZE * 8 as ::core::ffi::c_int;
    let mut nwords: ::core::ffi::c_int = nbits / nbits_pr_word;
    if nwords != 0 as ::core::ffi::c_int {
        _lshift_word(b, nwords);
        nbits -= nwords * nbits_pr_word;
    }
    if nbits != 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        i = BN_ARRAY_SIZE - 1 as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            (*b).array[i as usize] = (*b).array[i as usize] << nbits
                | (*b).array[(i - 1 as ::core::ffi::c_int) as usize]
                    >> 8 as ::core::ffi::c_int * WORD_SIZE - nbits;
            i -= 1;
        }
        (*b).array[i as usize] <<= nbits;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_rshift(
    mut a: *mut bn,
    mut b: *mut bn,
    mut nbits: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_uint,
                b"void bignum_rshift(struct bn *, struct bn *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                354 as ::core::ffi::c_uint,
                b"void bignum_rshift(struct bn *, struct bn *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if nbits >= 0 as ::core::ffi::c_int
            && !(b"no negative shifts\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"nbits >= 0 && \"no negative shifts\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_uint,
                b"void bignum_rshift(struct bn *, struct bn *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    bignum_assign(b, a);
    let nbits_pr_word: ::core::ffi::c_int = WORD_SIZE * 8 as ::core::ffi::c_int;
    let mut nwords: ::core::ffi::c_int = nbits / nbits_pr_word;
    if nwords != 0 as ::core::ffi::c_int {
        _rshift_word(b, nwords);
        nbits -= nwords * nbits_pr_word;
    }
    if nbits != 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < BN_ARRAY_SIZE - 1 as ::core::ffi::c_int {
            (*b).array[i as usize] = (*b).array[i as usize] >> nbits
                | (*b).array[(i + 1 as ::core::ffi::c_int) as usize]
                    << 8 as ::core::ffi::c_int * WORD_SIZE - nbits;
            i += 1;
        }
        (*b).array[i as usize] >>= nbits;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_mod(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                385 as ::core::ffi::c_uint,
                b"void bignum_mod(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                386 as ::core::ffi::c_uint,
                b"void bignum_mod(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                387 as ::core::ffi::c_uint,
                b"void bignum_mod(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut tmp: bn = bn { array: [0; 32] };
    bignum_divmod(a, b, &raw mut tmp, c);
}
#[no_mangle]
pub unsafe extern "C" fn bignum_divmod(
    mut a: *mut bn,
    mut b: *mut bn,
    mut c: *mut bn,
    mut d: *mut bn,
) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                405 as ::core::ffi::c_uint,
                b"void bignum_divmod(struct bn *, struct bn *, struct bn *, struct bn *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                406 as ::core::ffi::c_uint,
                b"void bignum_divmod(struct bn *, struct bn *, struct bn *, struct bn *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                407 as ::core::ffi::c_uint,
                b"void bignum_divmod(struct bn *, struct bn *, struct bn *, struct bn *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut tmp: bn = bn { array: [0; 32] };
    bignum_div(a, b, c);
    bignum_mul(c, b, &raw mut tmp);
    bignum_sub(a, &raw mut tmp, d);
}
#[no_mangle]
pub unsafe extern "C" fn bignum_and(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                424 as ::core::ffi::c_uint,
                b"void bignum_and(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                425 as ::core::ffi::c_uint,
                b"void bignum_and(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                426 as ::core::ffi::c_uint,
                b"void bignum_and(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        (*c).array[i as usize] = (*a).array[i as usize] & (*b).array[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_or(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                438 as ::core::ffi::c_uint,
                b"void bignum_or(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                439 as ::core::ffi::c_uint,
                b"void bignum_or(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                440 as ::core::ffi::c_uint,
                b"void bignum_or(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        (*c).array[i as usize] = (*a).array[i as usize] | (*b).array[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_xor(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                452 as ::core::ffi::c_uint,
                b"void bignum_xor(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                453 as ::core::ffi::c_uint,
                b"void bignum_xor(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                454 as ::core::ffi::c_uint,
                b"void bignum_xor(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        (*c).array[i as usize] = (*a).array[i as usize] ^ (*b).array[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bignum_cmp(mut a: *mut bn, mut b: *mut bn) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                466 as ::core::ffi::c_uint,
                b"int bignum_cmp(struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                467 as ::core::ffi::c_uint,
                b"int bignum_cmp(struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = BN_ARRAY_SIZE;
    loop {
        i -= 1 as ::core::ffi::c_int;
        if (*a).array[i as usize] > (*b).array[i as usize] {
            return LARGER as ::core::ffi::c_int;
        } else if (*a).array[i as usize] < (*b).array[i as usize] {
            return SMALLER as ::core::ffi::c_int;
        }
        if !(i != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return EQUAL as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bignum_is_zero(mut n: *mut bn) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !n.is_null() && !(b"n is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"n && \"n is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                490 as ::core::ffi::c_uint,
                b"int bignum_is_zero(struct bn *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        if (*n).array[i as usize] != 0 {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bignum_pow(mut a: *mut bn, mut b: *mut bn, mut c: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                507 as ::core::ffi::c_uint,
                b"void bignum_pow(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                508 as ::core::ffi::c_uint,
                b"void bignum_pow(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !c.is_null() && !(b"c is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"c && \"c is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                509 as ::core::ffi::c_uint,
                b"void bignum_pow(struct bn *, struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut tmp: bn = bn { array: [0; 32] };
    bignum_init(c);
    if bignum_cmp(b, c) == EQUAL as ::core::ffi::c_int {
        bignum_inc(c);
    } else {
        let mut bcopy: bn = bn { array: [0; 32] };
        bignum_assign(&raw mut bcopy, b);
        bignum_assign(&raw mut tmp, a);
        bignum_dec(&raw mut bcopy);
        while bignum_is_zero(&raw mut bcopy) == 0 {
            bignum_mul(&raw mut tmp, a, c);
            bignum_dec(&raw mut bcopy);
            bignum_assign(&raw mut tmp, c);
        }
        bignum_assign(c, &raw mut tmp);
    };
}
#[no_mangle]
pub unsafe extern "C" fn bignum_isqrt(mut a: *mut bn, mut b: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                549 as ::core::ffi::c_uint,
                b"void bignum_isqrt(struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !b.is_null() && !(b"b is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"b && \"b is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                550 as ::core::ffi::c_uint,
                b"void bignum_isqrt(struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut low: bn = bn { array: [0; 32] };
    let mut high: bn = bn { array: [0; 32] };
    let mut mid: bn = bn { array: [0; 32] };
    let mut tmp: bn = bn { array: [0; 32] };
    bignum_init(&raw mut low);
    bignum_assign(&raw mut high, a);
    bignum_rshift(&raw mut high, &raw mut mid, 1 as ::core::ffi::c_int);
    bignum_inc(&raw mut mid);
    while bignum_cmp(&raw mut high, &raw mut low) > 0 as ::core::ffi::c_int {
        bignum_mul(&raw mut mid, &raw mut mid, &raw mut tmp);
        if bignum_cmp(&raw mut tmp, a) > 0 as ::core::ffi::c_int {
            bignum_assign(&raw mut high, &raw mut mid);
            bignum_dec(&raw mut high);
        } else {
            bignum_assign(&raw mut low, &raw mut mid);
        }
        bignum_sub(&raw mut high, &raw mut low, &raw mut mid);
        _rshift_one_bit(&raw mut mid);
        bignum_add(&raw mut low, &raw mut mid, &raw mut mid);
        bignum_inc(&raw mut mid);
    }
    bignum_assign(b, &raw mut low);
}
#[no_mangle]
pub unsafe extern "C" fn bignum_assign(mut dst: *mut bn, mut src: *mut bn) {
    '_c2rust_label: {
        if !dst.is_null()
            && !(b"dst is null\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"dst && \"dst is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                582 as ::core::ffi::c_uint,
                b"void bignum_assign(struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !src.is_null()
            && !(b"src is null\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"src && \"src is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                583 as ::core::ffi::c_uint,
                b"void bignum_assign(struct bn *, struct bn *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE {
        (*dst).array[i as usize] = (*src).array[i as usize];
        i += 1;
    }
}
unsafe extern "C" fn _rshift_word(mut a: *mut bn, mut nwords: ::core::ffi::c_int) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                597 as ::core::ffi::c_uint,
                b"void _rshift_word(struct bn *, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if nwords >= 0 as ::core::ffi::c_int
            && !(b"no negative shifts\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"nwords >= 0 && \"no negative shifts\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                598 as ::core::ffi::c_uint,
                b"void _rshift_word(struct bn *, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    if nwords >= BN_ARRAY_SIZE {
        i = 0 as ::core::ffi::c_int;
        while i < BN_ARRAY_SIZE {
            (*a).array[i as usize] = 0 as uint32_t;
            i += 1;
        }
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE - nwords {
        (*a).array[i as usize] = (*a).array[(i + nwords) as usize];
        i += 1;
    }
    while i < BN_ARRAY_SIZE {
        (*a).array[i as usize] = 0 as uint32_t;
        i += 1;
    }
}
unsafe extern "C" fn _lshift_word(mut a: *mut bn, mut nwords: ::core::ffi::c_int) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                623 as ::core::ffi::c_uint,
                b"void _lshift_word(struct bn *, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if nwords >= 0 as ::core::ffi::c_int
            && !(b"no negative shifts\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"nwords >= 0 && \"no negative shifts\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                624 as ::core::ffi::c_uint,
                b"void _lshift_word(struct bn *, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = BN_ARRAY_SIZE - 1 as ::core::ffi::c_int;
    while i >= nwords {
        (*a).array[i as usize] = (*a).array[(i - nwords) as usize];
        i -= 1;
    }
    while i >= 0 as ::core::ffi::c_int {
        (*a).array[i as usize] = 0 as uint32_t;
        i -= 1;
    }
}
unsafe extern "C" fn _lshift_one_bit(mut a: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                642 as ::core::ffi::c_uint,
                b"void _lshift_one_bit(struct bn *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = BN_ARRAY_SIZE - 1 as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        (*a).array[i as usize] = (*a).array[i as usize] << 1 as ::core::ffi::c_int
            | (*a).array[(i - 1 as ::core::ffi::c_int) as usize]
                >> 8 as ::core::ffi::c_int * WORD_SIZE - 1 as ::core::ffi::c_int;
        i -= 1;
    }
    (*a).array[0 as ::core::ffi::c_int as usize] <<= 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _rshift_one_bit(mut a: *mut bn) {
    '_c2rust_label: {
        if !a.is_null() && !(b"a is null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        } else {
            __assert_fail(
                b"a && \"a is null\"\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/benchmark/pairs/bignum/source/bignum.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                655 as ::core::ffi::c_uint,
                b"void _rshift_one_bit(struct bn *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BN_ARRAY_SIZE - 1 as ::core::ffi::c_int {
        (*a).array[i as usize] = (*a).array[i as usize] >> 1 as ::core::ffi::c_int
            | (*a).array[(i + 1 as ::core::ffi::c_int) as usize]
                << 8 as ::core::ffi::c_int * WORD_SIZE - 1 as ::core::ffi::c_int;
        i += 1;
    }
    (*a).array[(BN_ARRAY_SIZE - 1 as ::core::ffi::c_int) as usize] >>= 1 as ::core::ffi::c_int;
}

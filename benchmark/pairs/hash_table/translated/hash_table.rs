#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Slot {
    pub key: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
    pub used: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashTable {
    pub slots: *mut Slot,
    pub cap: size_t,
    pub count: size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn ht_hash(mut key: ::core::ffi::c_int, mut cap: size_t) -> size_t {
    let mut x: uint32_t = key as uint32_t;
    x ^= x >> 16 as ::core::ffi::c_int;
    x = (x as ::core::ffi::c_uint).wrapping_mul(0x7feb352d as ::core::ffi::c_uint) as uint32_t
        as uint32_t;
    x ^= x >> 15 as ::core::ffi::c_int;
    x = (x as ::core::ffi::c_uint).wrapping_mul(0x846ca68b as ::core::ffi::c_uint) as uint32_t
        as uint32_t;
    x ^= x >> 16 as ::core::ffi::c_int;
    return x as size_t & cap.wrapping_sub(1 as size_t);
}
unsafe extern "C" fn ht_insert_into(
    mut slots: *mut Slot,
    mut cap: size_t,
    mut key: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut idx: size_t = ht_hash(key, cap);
    let mut probe: size_t = 0 as size_t;
    while probe < cap {
        let mut s: *mut Slot = slots.offset(idx as isize) as *mut Slot;
        if (*s).used == 0 {
            (*s).key = key;
            (*s).value = value;
            (*s).used = 1 as uint8_t;
            return 1 as ::core::ffi::c_int;
        }
        if (*s).key == key {
            (*s).value = value;
            return 0 as ::core::ffi::c_int;
        }
        idx = idx.wrapping_add(1 as size_t) & cap.wrapping_sub(1 as size_t);
        probe = probe.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ht_init(mut t: *mut HashTable) -> ::core::ffi::c_int {
    (*t).cap = 8 as size_t;
    (*t).count = 0 as size_t;
    (*t).slots = calloc((*t).cap, ::core::mem::size_of::<Slot>() as size_t) as *mut Slot;
    return if (*t).slots.is_null() {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn ht_grow(mut t: *mut HashTable) -> ::core::ffi::c_int {
    let mut new_cap: size_t = (*t).cap.wrapping_mul(2 as size_t);
    let mut ns: *mut Slot = calloc(new_cap, ::core::mem::size_of::<Slot>() as size_t) as *mut Slot;
    if ns.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: size_t = 0 as size_t;
    while i < (*t).cap {
        if (*(*t).slots.offset(i as isize)).used != 0 {
            ht_insert_into(
                ns,
                new_cap,
                (*(*t).slots.offset(i as isize)).key,
                (*(*t).slots.offset(i as isize)).value,
            );
        }
        i = i.wrapping_add(1);
    }
    free((*t).slots as *mut ::core::ffi::c_void);
    (*t).slots = ns;
    (*t).cap = new_cap;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ht_insert(
    mut t: *mut HashTable,
    mut key: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*t)
        .count
        .wrapping_add(1 as size_t)
        .wrapping_mul(10 as size_t)
        >= (*t).cap.wrapping_mul(7 as size_t)
    {
        if ht_grow(t) != 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*t).count = (*t)
        .count
        .wrapping_add(ht_insert_into((*t).slots, (*t).cap, key, value) as size_t);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ht_lookup(
    mut t: *const HashTable,
    mut key: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut idx: size_t = ht_hash(key, (*t).cap);
    let mut probe: size_t = 0 as size_t;
    while probe < (*t).cap {
        let mut s: *const Slot = (*t).slots.offset(idx as isize) as *mut Slot;
        if (*s).used == 0 {
            return -(1 as ::core::ffi::c_int);
        }
        if (*s).key == key {
            *out = (*s).value;
            return 0 as ::core::ffi::c_int;
        }
        idx = idx.wrapping_add(1 as size_t) & (*t).cap.wrapping_sub(1 as size_t);
        probe = probe.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn ht_free(mut t: *mut HashTable) {
    free((*t).slots as *mut ::core::ffi::c_void);
    (*t).slots = ::core::ptr::null_mut::<Slot>();
    (*t).cap = 0 as size_t;
    (*t).count = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ht_run(
    mut ops: *const ::core::ffi::c_int,
    mut n: size_t,
) -> ::core::ffi::c_long {
    let mut t: HashTable = HashTable {
        slots: ::core::ptr::null_mut::<Slot>(),
        cap: 0,
        count: 0,
    };
    if ht_init(&raw mut t) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    let mut acc: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut i: size_t = 0 as size_t;
    while i < n {
        let fresh0 = i;
        i = i.wrapping_add(1);
        let mut sel: ::core::ffi::c_int = *ops.offset(fresh0 as isize);
        if sel & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut key: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if i < n {
                let fresh1 = i;
                i = i.wrapping_add(1);
                key = *ops.offset(fresh1 as isize);
            }
            if i < n {
                let fresh2 = i;
                i = i.wrapping_add(1);
                value = *ops.offset(fresh2 as isize);
            }
            ht_insert(&raw mut t, key, value);
        } else {
            let mut key_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut out: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if i < n {
                let fresh3 = i;
                i = i.wrapping_add(1);
                key_0 = *ops.offset(fresh3 as isize);
            }
            if ht_lookup(&raw mut t, key_0, &raw mut out) == 0 as ::core::ffi::c_int {
                acc += out as ::core::ffi::c_long;
            }
        }
    }
    let mut result: ::core::ffi::c_long = acc + t.count as ::core::ffi::c_long;
    ht_free(&raw mut t);
    return result;
}

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
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan2(__y: ::core::ffi::c_double, __x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmod(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te_expr {
    pub type_0: ::core::ffi::c_int,
    pub c2rust_unnamed: C2RustUnnamed,
    pub parameters: [*mut ::core::ffi::c_void; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub value: ::core::ffi::c_double,
    pub bound: *const ::core::ffi::c_double,
    pub function: *const ::core::ffi::c_void,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TE_FLAG_PURE: C2RustUnnamed_0 = 32;
pub const TE_CLOSURE7: C2RustUnnamed_0 = 23;
pub const TE_CLOSURE6: C2RustUnnamed_0 = 22;
pub const TE_CLOSURE5: C2RustUnnamed_0 = 21;
pub const TE_CLOSURE4: C2RustUnnamed_0 = 20;
pub const TE_CLOSURE3: C2RustUnnamed_0 = 19;
pub const TE_CLOSURE2: C2RustUnnamed_0 = 18;
pub const TE_CLOSURE1: C2RustUnnamed_0 = 17;
pub const TE_CLOSURE0: C2RustUnnamed_0 = 16;
pub const TE_FUNCTION7: C2RustUnnamed_0 = 15;
pub const TE_FUNCTION6: C2RustUnnamed_0 = 14;
pub const TE_FUNCTION5: C2RustUnnamed_0 = 13;
pub const TE_FUNCTION4: C2RustUnnamed_0 = 12;
pub const TE_FUNCTION3: C2RustUnnamed_0 = 11;
pub const TE_FUNCTION2: C2RustUnnamed_0 = 10;
pub const TE_FUNCTION1: C2RustUnnamed_0 = 9;
pub const TE_FUNCTION0: C2RustUnnamed_0 = 8;
pub const TE_VARIABLE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te_variable {
    pub name: *const ::core::ffi::c_char,
    pub address: *const ::core::ffi::c_void,
    pub type_0: ::core::ffi::c_int,
    pub context: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub start: *const ::core::ffi::c_char,
    pub next: *const ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub context: *mut ::core::ffi::c_void,
    pub lookup: *const te_variable,
    pub lookup_len: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub value: ::core::ffi::c_double,
    pub bound: *const ::core::ffi::c_double,
    pub function: *const ::core::ffi::c_void,
}
pub const TOK_ERROR: C2RustUnnamed_3 = 25;
pub type size_t = usize;
pub const TOK_NULL: C2RustUnnamed_3 = 24;
pub const TOK_SEP: C2RustUnnamed_3 = 27;
pub const TOK_CLOSE: C2RustUnnamed_3 = 29;
pub const TOK_OPEN: C2RustUnnamed_3 = 28;
pub const TOK_INFIX: C2RustUnnamed_3 = 32;
pub const TOK_VARIABLE: C2RustUnnamed_3 = 31;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const TOK_NUMBER: C2RustUnnamed_3 = 30;
pub const TOK_END: C2RustUnnamed_3 = 26;
pub const TE_CONSTANT: C2RustUnnamed_4 = 1;
pub type te_fun2 = Option<
    unsafe extern "C" fn(::core::ffi::c_double, ::core::ffi::c_double) -> ::core::ffi::c_double,
>;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
unsafe extern "C" fn new_expr(
    type_0: ::core::ffi::c_int,
    mut parameters: *mut *const te_expr,
) -> *mut te_expr {
    let arity: ::core::ffi::c_int =
        if type_0 & (TE_FUNCTION0 as ::core::ffi::c_int | TE_CLOSURE0 as ::core::ffi::c_int) != 0 {
            type_0 & 0x7 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    let psize: ::core::ffi::c_int = (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
        .wrapping_mul(arity as usize) as ::core::ffi::c_int;
    let size: ::core::ffi::c_int = (::core::mem::size_of::<te_expr>() as usize)
        .wrapping_sub(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
        .wrapping_add(psize as usize)
        .wrapping_add(
            (if type_0 & TE_CLOSURE0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
            } else {
                0 as usize
            }),
        ) as ::core::ffi::c_int;
    let mut ret: *mut te_expr = malloc(size as size_t) as *mut te_expr;
    if ret.is_null() {
        return ::core::ptr::null_mut::<te_expr>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        size as size_t,
    );
    if arity != 0 && !parameters.is_null() {
        memcpy(
            &raw mut (*ret).parameters as *mut *mut ::core::ffi::c_void as *mut ::core::ffi::c_void,
            parameters as *const ::core::ffi::c_void,
            psize as size_t,
        );
    }
    (*ret).type_0 = type_0;
    (*ret).c2rust_unnamed.bound = ::core::ptr::null::<::core::ffi::c_double>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn te_free_parameters(mut n: *mut te_expr) {
    if n.is_null() {
        return;
    }
    let mut current_block_8: u64;
    match (*n).type_0 & 0x1f as ::core::ffi::c_int {
        15 | 23 => {
            te_free(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(6 as ::core::ffi::c_int as isize) as *mut te_expr,
            );
            current_block_8 = 3644872370092143722;
        }
        14 | 22 => {
            current_block_8 = 3644872370092143722;
        }
        13 | 21 => {
            current_block_8 = 5923537644671907846;
        }
        12 | 20 => {
            current_block_8 = 3694722799336667006;
        }
        11 | 19 => {
            current_block_8 = 11045804415818221452;
        }
        10 | 18 => {
            current_block_8 = 6560564072968658859;
        }
        9 | 17 => {
            current_block_8 = 17505394854308233263;
        }
        _ => {
            current_block_8 = 5399440093318478209;
        }
    }
    match current_block_8 {
        3644872370092143722 => {
            te_free(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(5 as ::core::ffi::c_int as isize) as *mut te_expr,
            );
            current_block_8 = 5923537644671907846;
        }
        _ => {}
    }
    match current_block_8 {
        5923537644671907846 => {
            te_free(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(4 as ::core::ffi::c_int as isize) as *mut te_expr,
            );
            current_block_8 = 3694722799336667006;
        }
        _ => {}
    }
    match current_block_8 {
        3694722799336667006 => {
            te_free(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(3 as ::core::ffi::c_int as isize) as *mut te_expr,
            );
            current_block_8 = 11045804415818221452;
        }
        _ => {}
    }
    match current_block_8 {
        11045804415818221452 => {
            te_free(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(2 as ::core::ffi::c_int as isize) as *mut te_expr,
            );
            current_block_8 = 6560564072968658859;
        }
        _ => {}
    }
    match current_block_8 {
        6560564072968658859 => {
            te_free(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(1 as ::core::ffi::c_int as isize) as *mut te_expr,
            );
            current_block_8 = 17505394854308233263;
        }
        _ => {}
    }
    match current_block_8 {
        17505394854308233263 => {
            te_free(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut te_expr,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn te_free(mut n: *mut te_expr) {
    if n.is_null() {
        return;
    }
    te_free_parameters(n);
    free(n as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn pi() -> ::core::ffi::c_double {
    return 3.14159265358979323846f64;
}
unsafe extern "C" fn e() -> ::core::ffi::c_double {
    return 2.71828182845904523536f64;
}
unsafe extern "C" fn fac(mut a: ::core::ffi::c_double) -> ::core::ffi::c_double {
    if a < 0.0f64 {
        return ::core::f32::NAN as ::core::ffi::c_double;
    }
    if a > UINT_MAX as ::core::ffi::c_double {
        return ::core::f32::INFINITY as ::core::ffi::c_double;
    }
    let mut ua: ::core::ffi::c_uint = a as ::core::ffi::c_uint;
    let mut result: ::core::ffi::c_ulong = 1 as ::core::ffi::c_ulong;
    let mut i: ::core::ffi::c_ulong = 0;
    i = 1 as ::core::ffi::c_ulong;
    while i <= ua as ::core::ffi::c_ulong {
        if i > ULONG_MAX.wrapping_div(result) {
            return ::core::f32::INFINITY as ::core::ffi::c_double;
        }
        result = result.wrapping_mul(i);
        i = i.wrapping_add(1);
    }
    return result as ::core::ffi::c_double;
}
unsafe extern "C" fn ncr(
    mut n: ::core::ffi::c_double,
    mut r: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if n < 0.0f64 || r < 0.0f64 || n < r {
        return ::core::f32::NAN as ::core::ffi::c_double;
    }
    if n > UINT_MAX as ::core::ffi::c_double || r > UINT_MAX as ::core::ffi::c_double {
        return ::core::f32::INFINITY as ::core::ffi::c_double;
    }
    let mut un: ::core::ffi::c_ulong = n as ::core::ffi::c_uint as ::core::ffi::c_ulong;
    let mut ur: ::core::ffi::c_ulong = r as ::core::ffi::c_uint as ::core::ffi::c_ulong;
    let mut i: ::core::ffi::c_ulong = 0;
    let mut result: ::core::ffi::c_ulong = 1 as ::core::ffi::c_ulong;
    if ur > un.wrapping_div(2 as ::core::ffi::c_ulong) {
        ur = un.wrapping_sub(ur);
    }
    i = 1 as ::core::ffi::c_ulong;
    while i <= ur {
        if result > ULONG_MAX.wrapping_div(un.wrapping_sub(ur).wrapping_add(i)) {
            return ::core::f32::INFINITY as ::core::ffi::c_double;
        }
        result = result.wrapping_mul(un.wrapping_sub(ur).wrapping_add(i));
        result = result.wrapping_div(i);
        i = i.wrapping_add(1);
    }
    return result as ::core::ffi::c_double;
}
unsafe extern "C" fn npr(
    mut n: ::core::ffi::c_double,
    mut r: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return ncr(n, r) * fac(r);
}
static mut functions: [te_variable; 25] = unsafe {
    [
        te_variable {
            name: b"abs\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                fabs as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"acos\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                acos as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"asin\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                asin as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"atan\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                atan as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"atan2\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                atan2
                    as unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"ceil\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                ceil as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"cos\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                cos as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"cosh\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                cosh as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"e\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(e as unsafe extern "C" fn() -> ::core::ffi::c_double)),
            type_0: TE_FUNCTION0 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"exp\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                exp as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"fac\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                fac as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"floor\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                floor as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"ln\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                log as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"log\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                log10 as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"log10\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                log10 as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"ncr\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                ncr as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"npr\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                npr as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"pi\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(pi as unsafe extern "C" fn() -> ::core::ffi::c_double)),
            type_0: TE_FUNCTION0 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"pow\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                pow as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"sin\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                sin as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"sinh\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                sinh as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"sqrt\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                sqrt as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"tan\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                tan as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: b"tanh\0" as *const u8 as *const ::core::ffi::c_char,
            address: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
                *const ::core::ffi::c_void,
            >(Some(
                tanh as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            )),
            type_0: TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
        te_variable {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            address: ::core::ptr::null::<::core::ffi::c_void>(),
            type_0: 0 as ::core::ffi::c_int,
            context: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        },
    ]
};
unsafe extern "C" fn find_builtin(
    mut name: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *const te_variable {
    let mut imin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut imax: ::core::ffi::c_int = (::core::mem::size_of::<[te_variable; 25]>() as usize)
        .wrapping_div(::core::mem::size_of::<te_variable>() as usize)
        .wrapping_sub(2 as usize) as ::core::ffi::c_int;
    while imax >= imin {
        let i: ::core::ffi::c_int = imin + (imax - imin) / 2 as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_int = strncmp(name, functions[i as usize].name, len as size_t);
        if c == 0 {
            c = '\0' as i32
                - *functions[i as usize].name.offset(len as isize) as ::core::ffi::c_int;
        }
        if c == 0 as ::core::ffi::c_int {
            return (&raw const functions as *const te_variable).offset(i as isize);
        } else if c > 0 as ::core::ffi::c_int {
            imin = i + 1 as ::core::ffi::c_int;
        } else {
            imax = i - 1 as ::core::ffi::c_int;
        }
    }
    return ::core::ptr::null::<te_variable>();
}
unsafe extern "C" fn find_lookup(
    mut s: *const state,
    mut name: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *const te_variable {
    let mut iters: ::core::ffi::c_int = 0;
    let mut var: *const te_variable = ::core::ptr::null::<te_variable>();
    if (*s).lookup.is_null() {
        return ::core::ptr::null::<te_variable>();
    }
    var = (*s).lookup;
    iters = (*s).lookup_len;
    while iters != 0 {
        if strncmp(name, (*var).name, len as size_t) == 0 as ::core::ffi::c_int
            && *(*var).name.offset(len as isize) as ::core::ffi::c_int == '\0' as i32
        {
            return var;
        }
        var = var.offset(1);
        iters -= 1;
    }
    return ::core::ptr::null::<te_variable>();
}
unsafe extern "C" fn add(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return a + b;
}
unsafe extern "C" fn sub(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return a - b;
}
unsafe extern "C" fn mul(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return a * b;
}
unsafe extern "C" fn divide(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return a / b;
}
unsafe extern "C" fn negate(mut a: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return -a;
}
unsafe extern "C" fn comma(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn next_token(mut s: *mut state) {
    (*s).type_0 = TOK_NULL as ::core::ffi::c_int;
    loop {
        if *(*s).next == 0 {
            (*s).type_0 = TOK_END as ::core::ffi::c_int;
            return;
        }
        if *(*s).next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
            && *(*s).next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= '9' as i32
            || *(*s).next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32
        {
            (*s).c2rust_unnamed.value = strtod(
                (*s).next,
                &raw mut (*s).next as *mut *mut ::core::ffi::c_char,
            );
            (*s).type_0 = TOK_NUMBER as ::core::ffi::c_int;
        } else if *(*__ctype_b_loc()).offset(
            *(*s).next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
            & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            start = (*s).next;
            while *(*__ctype_b_loc()).offset(
                *(*s).next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
                || *(*__ctype_b_loc()).offset(*(*s).next.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
                || *(*s).next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '_' as i32
            {
                (*s).next = (*s).next.offset(1);
            }
            let mut var: *const te_variable = find_lookup(
                s,
                start,
                (*s).next.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            if var.is_null() {
                var = find_builtin(
                    start,
                    (*s).next.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
            }
            if var.is_null() {
                (*s).type_0 = TOK_ERROR as ::core::ffi::c_int;
            } else {
                let mut current_block_19: u64;
                match (*var).type_0 & 0x1f as ::core::ffi::c_int {
                    0 => {
                        (*s).type_0 = TOK_VARIABLE as ::core::ffi::c_int;
                        (*s).c2rust_unnamed.bound = (*var).address as *const ::core::ffi::c_double;
                        current_block_19 = 18386322304582297246;
                    }
                    16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => {
                        (*s).context = (*var).context;
                        current_block_19 = 5362628258391840100;
                    }
                    8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
                        current_block_19 = 5362628258391840100;
                    }
                    _ => {
                        current_block_19 = 18386322304582297246;
                    }
                }
                match current_block_19 {
                    5362628258391840100 => {
                        (*s).type_0 = (*var).type_0;
                        (*s).c2rust_unnamed.function = (*var).address;
                    }
                    _ => {}
                }
            }
        } else {
            let fresh10 = (*s).next;
            (*s).next = (*s).next.offset(1);
            match *fresh10.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
                43 => {
                    (*s).type_0 = TOK_INFIX as ::core::ffi::c_int;
                    (*s).c2rust_unnamed.function = ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                        *const ::core::ffi::c_void,
                    >(Some(
                        add as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        )
                            -> ::core::ffi::c_double,
                    ));
                }
                45 => {
                    (*s).type_0 = TOK_INFIX as ::core::ffi::c_int;
                    (*s).c2rust_unnamed.function = ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                        *const ::core::ffi::c_void,
                    >(Some(
                        sub as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        )
                            -> ::core::ffi::c_double,
                    ));
                }
                42 => {
                    (*s).type_0 = TOK_INFIX as ::core::ffi::c_int;
                    (*s).c2rust_unnamed.function = ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                        *const ::core::ffi::c_void,
                    >(Some(
                        mul as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        )
                            -> ::core::ffi::c_double,
                    ));
                }
                47 => {
                    (*s).type_0 = TOK_INFIX as ::core::ffi::c_int;
                    (*s).c2rust_unnamed.function = ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                        *const ::core::ffi::c_void,
                    >(Some(
                        divide
                            as unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                    ));
                }
                94 => {
                    (*s).type_0 = TOK_INFIX as ::core::ffi::c_int;
                    (*s).c2rust_unnamed.function = ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                        *const ::core::ffi::c_void,
                    >(Some(
                        pow as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        )
                            -> ::core::ffi::c_double,
                    ));
                }
                37 => {
                    (*s).type_0 = TOK_INFIX as ::core::ffi::c_int;
                    (*s).c2rust_unnamed.function = ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                        *const ::core::ffi::c_void,
                    >(Some(
                        fmod as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        )
                            -> ::core::ffi::c_double,
                    ));
                }
                40 => {
                    (*s).type_0 = TOK_OPEN as ::core::ffi::c_int;
                }
                41 => {
                    (*s).type_0 = TOK_CLOSE as ::core::ffi::c_int;
                }
                44 => {
                    (*s).type_0 = TOK_SEP as ::core::ffi::c_int;
                }
                32 | 9 | 10 | 13 => {}
                _ => {
                    (*s).type_0 = TOK_ERROR as ::core::ffi::c_int;
                }
            }
        }
        if !((*s).type_0 == TOK_NULL as ::core::ffi::c_int) {
            break;
        }
    }
}
unsafe extern "C" fn base(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = ::core::ptr::null_mut::<te_expr>();
    let mut arity: ::core::ffi::c_int = 0;
    match (*s).type_0 & 0x1f as ::core::ffi::c_int {
        30 => {
            ret = new_expr(
                TE_CONSTANT as ::core::ffi::c_int,
                ::core::ptr::null_mut::<*const te_expr>(),
            );
            if ret.is_null() {
                return ::core::ptr::null_mut::<te_expr>();
            }
            (*ret).c2rust_unnamed.value = (*s).c2rust_unnamed.value;
            next_token(s);
        }
        31 => {
            ret = new_expr(
                TE_VARIABLE as ::core::ffi::c_int,
                ::core::ptr::null_mut::<*const te_expr>(),
            );
            if ret.is_null() {
                return ::core::ptr::null_mut::<te_expr>();
            }
            (*ret).c2rust_unnamed.bound = (*s).c2rust_unnamed.bound;
            next_token(s);
        }
        8 | 16 => {
            ret = new_expr((*s).type_0, ::core::ptr::null_mut::<*const te_expr>());
            if ret.is_null() {
                return ::core::ptr::null_mut::<te_expr>();
            }
            (*ret).c2rust_unnamed.function = (*s).c2rust_unnamed.function;
            if (*s).type_0 & TE_CLOSURE0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                let ref mut fresh5 = *(&raw mut (*ret).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(0 as ::core::ffi::c_int as isize);
                *fresh5 = (*s).context;
            }
            next_token(s);
            if (*s).type_0 == TOK_OPEN as ::core::ffi::c_int {
                next_token(s);
                if (*s).type_0 != TOK_CLOSE as ::core::ffi::c_int {
                    (*s).type_0 = TOK_ERROR as ::core::ffi::c_int;
                } else {
                    next_token(s);
                }
            }
        }
        9 | 17 => {
            ret = new_expr((*s).type_0, ::core::ptr::null_mut::<*const te_expr>());
            if ret.is_null() {
                return ::core::ptr::null_mut::<te_expr>();
            }
            (*ret).c2rust_unnamed.function = (*s).c2rust_unnamed.function;
            if (*s).type_0 & TE_CLOSURE0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                let ref mut fresh6 = *(&raw mut (*ret).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(1 as ::core::ffi::c_int as isize);
                *fresh6 = (*s).context;
            }
            next_token(s);
            let ref mut fresh7 = *(&raw mut (*ret).parameters as *mut *mut ::core::ffi::c_void)
                .offset(0 as ::core::ffi::c_int as isize);
            *fresh7 = power(s) as *mut ::core::ffi::c_void;
            if (*(&raw mut (*ret).parameters as *mut *mut ::core::ffi::c_void)
                .offset(0 as ::core::ffi::c_int as isize))
            .is_null()
            {
                te_free(ret);
                return ::core::ptr::null_mut::<te_expr>();
            }
        }
        10 | 11 | 12 | 13 | 14 | 15 | 18 | 19 | 20 | 21 | 22 | 23 => {
            arity = if (*s).type_0
                & (TE_FUNCTION0 as ::core::ffi::c_int | TE_CLOSURE0 as ::core::ffi::c_int)
                != 0
            {
                (*s).type_0 & 0x7 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            ret = new_expr((*s).type_0, ::core::ptr::null_mut::<*const te_expr>());
            if ret.is_null() {
                return ::core::ptr::null_mut::<te_expr>();
            }
            (*ret).c2rust_unnamed.function = (*s).c2rust_unnamed.function;
            if (*s).type_0 & TE_CLOSURE0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                let ref mut fresh8 = *(&raw mut (*ret).parameters as *mut *mut ::core::ffi::c_void)
                    .offset(arity as isize);
                *fresh8 = (*s).context;
            }
            next_token(s);
            if (*s).type_0 != TOK_OPEN as ::core::ffi::c_int {
                (*s).type_0 = TOK_ERROR as ::core::ffi::c_int;
            } else {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < arity {
                    next_token(s);
                    let ref mut fresh9 = *(&raw mut (*ret).parameters
                        as *mut *mut ::core::ffi::c_void)
                        .offset(i as isize);
                    *fresh9 = expr(s) as *mut ::core::ffi::c_void;
                    if (*(&raw mut (*ret).parameters as *mut *mut ::core::ffi::c_void)
                        .offset(i as isize))
                    .is_null()
                    {
                        te_free(ret);
                        return ::core::ptr::null_mut::<te_expr>();
                    }
                    if (*s).type_0 != TOK_SEP as ::core::ffi::c_int {
                        break;
                    }
                    i += 1;
                }
                if (*s).type_0 != TOK_CLOSE as ::core::ffi::c_int
                    || i != arity - 1 as ::core::ffi::c_int
                {
                    (*s).type_0 = TOK_ERROR as ::core::ffi::c_int;
                } else {
                    next_token(s);
                }
            }
        }
        28 => {
            next_token(s);
            ret = list(s);
            if ret.is_null() {
                return ::core::ptr::null_mut::<te_expr>();
            }
            if (*s).type_0 != TOK_CLOSE as ::core::ffi::c_int {
                (*s).type_0 = TOK_ERROR as ::core::ffi::c_int;
            } else {
                next_token(s);
            }
        }
        _ => {
            ret = new_expr(
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<*const te_expr>(),
            );
            if ret.is_null() {
                return ::core::ptr::null_mut::<te_expr>();
            }
            (*s).type_0 = TOK_ERROR as ::core::ffi::c_int;
            (*ret).c2rust_unnamed.value = ::core::f32::NAN as ::core::ffi::c_double;
        }
    }
    return ret;
}
unsafe extern "C" fn power(mut s: *mut state) -> *mut te_expr {
    let mut sign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while (*s).type_0 == TOK_INFIX as ::core::ffi::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                add as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            ))
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                    >,
                    *const ::core::ffi::c_void,
                >(Some(
                    sub as unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                )))
    {
        if (*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                sub as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            ))
        {
            sign = -sign;
        }
        next_token(s);
    }
    let mut ret: *mut te_expr = ::core::ptr::null_mut::<te_expr>();
    if sign == 1 as ::core::ffi::c_int {
        ret = base(s);
    } else {
        let mut b: *mut te_expr = base(s);
        if b.is_null() {
            return ::core::ptr::null_mut::<te_expr>();
        }
        let mut fresh4: [*const te_expr; 1] = [b as *const te_expr];
        ret = new_expr(
            TE_FUNCTION1 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            &raw mut fresh4 as *mut *const te_expr,
        );
        if ret.is_null() {
            te_free(b);
            return ::core::ptr::null_mut::<te_expr>();
        }
        (*ret).c2rust_unnamed.function = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
            *const ::core::ffi::c_void,
        >(Some(
            negate as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        ));
    }
    return ret;
}
unsafe extern "C" fn factor(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = power(s);
    if ret.is_null() {
        return ::core::ptr::null_mut::<te_expr>();
    }
    while (*s).type_0 == TOK_INFIX as ::core::ffi::c_int
        && (*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                pow as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            ))
    {
        let mut t: te_fun2 = ::core::mem::transmute::<*const ::core::ffi::c_void, te_fun2>(
            (*s).c2rust_unnamed.function,
        );
        next_token(s);
        let mut p: *mut te_expr = power(s);
        if p.is_null() {
            te_free(ret);
            return ::core::ptr::null_mut::<te_expr>();
        }
        let mut prev: *mut te_expr = ret;
        let mut fresh3: [*const te_expr; 2] = [ret as *const te_expr, p as *const te_expr];
        ret = new_expr(
            TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            &raw mut fresh3 as *mut *const te_expr,
        );
        if ret.is_null() {
            te_free(p);
            te_free(prev);
            return ::core::ptr::null_mut::<te_expr>();
        }
        (*ret).c2rust_unnamed.function =
            ::core::mem::transmute::<te_fun2, *const ::core::ffi::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn term(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = factor(s);
    if ret.is_null() {
        return ::core::ptr::null_mut::<te_expr>();
    }
    while (*s).type_0 == TOK_INFIX as ::core::ffi::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                mul as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            ))
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                    >,
                    *const ::core::ffi::c_void,
                >(Some(
                    divide
                        as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                ))
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                    >,
                    *const ::core::ffi::c_void,
                >(Some(
                    fmod as unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                )))
    {
        let mut t: te_fun2 = ::core::mem::transmute::<*const ::core::ffi::c_void, te_fun2>(
            (*s).c2rust_unnamed.function,
        );
        next_token(s);
        let mut f: *mut te_expr = factor(s);
        if f.is_null() {
            te_free(ret);
            return ::core::ptr::null_mut::<te_expr>();
        }
        let mut prev: *mut te_expr = ret;
        let mut fresh2: [*const te_expr; 2] = [ret as *const te_expr, f as *const te_expr];
        ret = new_expr(
            TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            &raw mut fresh2 as *mut *const te_expr,
        );
        if ret.is_null() {
            te_free(f);
            te_free(prev);
            return ::core::ptr::null_mut::<te_expr>();
        }
        (*ret).c2rust_unnamed.function =
            ::core::mem::transmute::<te_fun2, *const ::core::ffi::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn expr(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = term(s);
    if ret.is_null() {
        return ::core::ptr::null_mut::<te_expr>();
    }
    while (*s).type_0 == TOK_INFIX as ::core::ffi::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
                *const ::core::ffi::c_void,
            >(Some(
                add as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            ))
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                    >,
                    *const ::core::ffi::c_void,
                >(Some(
                    sub as unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                )))
    {
        let mut t: te_fun2 = ::core::mem::transmute::<*const ::core::ffi::c_void, te_fun2>(
            (*s).c2rust_unnamed.function,
        );
        next_token(s);
        let mut te: *mut te_expr = term(s);
        if te.is_null() {
            te_free(ret);
            return ::core::ptr::null_mut::<te_expr>();
        }
        let mut prev: *mut te_expr = ret;
        let mut fresh1: [*const te_expr; 2] = [ret as *const te_expr, te as *const te_expr];
        ret = new_expr(
            TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            &raw mut fresh1 as *mut *const te_expr,
        );
        if ret.is_null() {
            te_free(te);
            te_free(prev);
            return ::core::ptr::null_mut::<te_expr>();
        }
        (*ret).c2rust_unnamed.function =
            ::core::mem::transmute::<te_fun2, *const ::core::ffi::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn list(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = expr(s);
    if ret.is_null() {
        return ::core::ptr::null_mut::<te_expr>();
    }
    while (*s).type_0 == TOK_SEP as ::core::ffi::c_int {
        next_token(s);
        let mut e_0: *mut te_expr = expr(s);
        if e_0.is_null() {
            te_free(ret);
            return ::core::ptr::null_mut::<te_expr>();
        }
        let mut prev: *mut te_expr = ret;
        let mut fresh0: [*const te_expr; 2] = [ret as *const te_expr, e_0 as *const te_expr];
        ret = new_expr(
            TE_FUNCTION2 as ::core::ffi::c_int | TE_FLAG_PURE as ::core::ffi::c_int,
            &raw mut fresh0 as *mut *const te_expr,
        );
        if ret.is_null() {
            te_free(e_0);
            te_free(prev);
            return ::core::ptr::null_mut::<te_expr>();
        }
        (*ret).c2rust_unnamed.function = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            >,
            *const ::core::ffi::c_void,
        >(Some(
            comma
                as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
        ));
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn te_eval(mut n: *const te_expr) -> ::core::ffi::c_double {
    if n.is_null() {
        return ::core::f32::NAN as ::core::ffi::c_double;
    }
    match (*n).type_0 & 0x1f as ::core::ffi::c_int {
        1 => return (*n).c2rust_unnamed.value,
        0 => return *(*n).c2rust_unnamed.bound,
        8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            match if (*n).type_0
                & (TE_FUNCTION0 as ::core::ffi::c_int | TE_CLOSURE0 as ::core::ffi::c_int)
                != 0
            {
                (*n).type_0 & 0x7 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } {
                0 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<unsafe extern "C" fn() -> ::core::ffi::c_double>,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")();
                }
                1 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(te_eval(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *const te_expr,
                    ));
                }
                2 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                3 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                4 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                5 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                6 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(5 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                7 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(5 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(6 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                _ => return ::core::f32::NAN as ::core::ffi::c_double,
            }
        }
        16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => {
            match if (*n).type_0
                & (TE_FUNCTION0 as ::core::ffi::c_int | TE_CLOSURE0 as ::core::ffi::c_int)
                != 0
            {
                (*n).type_0 & 0x7 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } {
                0 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(0 as ::core::ffi::c_int as isize),
                    );
                }
                1 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(1 as ::core::ffi::c_int as isize),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                2 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(2 as ::core::ffi::c_int as isize),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                3 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(3 as ::core::ffi::c_int as isize),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                4 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(4 as ::core::ffi::c_int as isize),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                5 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(5 as ::core::ffi::c_int as isize),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                6 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(6 as ::core::ffi::c_int as isize),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(5 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                7 => {
                    return ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            )
                                -> ::core::ffi::c_double,
                        >,
                    >((*n).c2rust_unnamed.function)
                    .expect("non-null function pointer")(
                        *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                            .offset(7 as ::core::ffi::c_int as isize),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(5 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                                .offset(6 as ::core::ffi::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                _ => return ::core::f32::NAN as ::core::ffi::c_double,
            }
        }
        _ => return ::core::f32::NAN as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn optimize(mut n: *mut te_expr) {
    if (*n).type_0 == TE_CONSTANT as ::core::ffi::c_int {
        return;
    }
    if (*n).type_0 == TE_VARIABLE as ::core::ffi::c_int {
        return;
    }
    if (*n).type_0 & TE_FLAG_PURE as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let arity: ::core::ffi::c_int = if (*n).type_0
            & (TE_FUNCTION0 as ::core::ffi::c_int | TE_CLOSURE0 as ::core::ffi::c_int)
            != 0
        {
            (*n).type_0 & 0x7 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut known: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < arity {
            optimize(
                *(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void).offset(i as isize)
                    as *mut te_expr,
            );
            if (*(*(&raw mut (*n).parameters as *mut *mut ::core::ffi::c_void).offset(i as isize)
                as *mut te_expr))
                .type_0
                != TE_CONSTANT as ::core::ffi::c_int
            {
                known = 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if known != 0 {
            let value: ::core::ffi::c_double = te_eval(n) as ::core::ffi::c_double;
            te_free_parameters(n);
            (*n).type_0 = TE_CONSTANT as ::core::ffi::c_int;
            (*n).c2rust_unnamed.value = value;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn te_compile(
    mut expression: *const ::core::ffi::c_char,
    mut variables: *const te_variable,
    mut var_count: ::core::ffi::c_int,
    mut error: *mut ::core::ffi::c_int,
) -> *mut te_expr {
    let mut s: state = state {
        start: ::core::ptr::null::<::core::ffi::c_char>(),
        next: ::core::ptr::null::<::core::ffi::c_char>(),
        type_0: 0,
        c2rust_unnamed: C2RustUnnamed_1 { value: 0. },
        context: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        lookup: ::core::ptr::null::<te_variable>(),
        lookup_len: 0,
    };
    s.next = expression;
    s.start = s.next;
    s.lookup = variables;
    s.lookup_len = var_count;
    next_token(&raw mut s);
    let mut root: *mut te_expr = list(&raw mut s);
    if root.is_null() {
        if !error.is_null() {
            *error = -(1 as ::core::ffi::c_int);
        }
        return ::core::ptr::null_mut::<te_expr>();
    }
    if s.type_0 != TOK_END as ::core::ffi::c_int {
        te_free(root);
        if !error.is_null() {
            *error = s.next.offset_from(s.start) as ::core::ffi::c_long as ::core::ffi::c_int;
            if *error == 0 as ::core::ffi::c_int {
                *error = 1 as ::core::ffi::c_int;
            }
        }
        return ::core::ptr::null_mut::<te_expr>();
    } else {
        optimize(root);
        if !error.is_null() {
            *error = 0 as ::core::ffi::c_int;
        }
        return root;
    };
}
#[no_mangle]
pub unsafe extern "C" fn te_interp(
    mut expression: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut n: *mut te_expr = te_compile(
        expression,
        ::core::ptr::null::<te_variable>(),
        0 as ::core::ffi::c_int,
        error,
    );
    let mut ret: ::core::ffi::c_double = 0.;
    if !n.is_null() {
        ret = te_eval(n);
        te_free(n);
    } else {
        ret = ::core::f32::NAN as ::core::ffi::c_double;
    }
    return ret;
}
unsafe extern "C" fn pn(mut n: *const te_expr, mut depth: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut arity: ::core::ffi::c_int = 0;
    printf(
        b"%*s\0" as *const u8 as *const ::core::ffi::c_char,
        depth,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    match (*n).type_0 & 0x1f as ::core::ffi::c_int {
        1 => {
            printf(
                b"%f\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*n).c2rust_unnamed.value,
            );
        }
        0 => {
            printf(
                b"bound %p\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*n).c2rust_unnamed.bound,
            );
        }
        8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => {
            arity = if (*n).type_0
                & (TE_FUNCTION0 as ::core::ffi::c_int | TE_CLOSURE0 as ::core::ffi::c_int)
                != 0
            {
                (*n).type_0 & 0x7 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            printf(b"f%d\0" as *const u8 as *const ::core::ffi::c_char, arity);
            i = 0 as ::core::ffi::c_int;
            while i < arity {
                printf(
                    b" %p\0" as *const u8 as *const ::core::ffi::c_char,
                    *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                        .offset(i as isize),
                );
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            i = 0 as ::core::ffi::c_int;
            while i < arity {
                pn(
                    *(&raw const (*n).parameters as *const *mut ::core::ffi::c_void)
                        .offset(i as isize) as *const te_expr,
                    depth + 1 as ::core::ffi::c_int,
                );
                i += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn te_print(mut n: *const te_expr) {
    pn(n, 0 as ::core::ffi::c_int);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;

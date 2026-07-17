#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, internal_features, unused_imports)]
extern "C" {
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn tolower(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub free_fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuffer {
    pub buffer: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_int,
}
pub const cJSON_False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const cJSON_True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const cJSON_NULL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const cJSON_Number: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const cJSON_String: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const cJSON_Array: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const cJSON_Object: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const cJSON_IsReference: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const cJSON_StringIsConst: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
static mut ep: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub unsafe extern "C" fn cJSON_GetErrorPtr() -> *const ::core::ffi::c_char {
    return ep;
}
unsafe extern "C" fn cJSON_strcasecmp(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if s1.is_null() {
        return if s1 == s2 {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
    if s2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    while tolower(*s1 as ::core::ffi::c_int) == tolower(*s2 as ::core::ffi::c_int) {
        if *s1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    return tolower(*(s1 as *const ::core::ffi::c_uchar) as ::core::ffi::c_int)
        - tolower(*(s2 as *const ::core::ffi::c_uchar) as ::core::ffi::c_int);
}
static mut cJSON_malloc: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void> =
    unsafe { Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void) };
static mut cJSON_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> =
    unsafe { Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()) };
unsafe extern "C" fn cJSON_strdup(mut str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut len: size_t = 0;
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = strlen(str).wrapping_add(1 as size_t);
    copy = cJSON_malloc.expect("non-null function pointer")(len) as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(
        copy as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len,
    );
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_InitHooks(mut hooks: *mut cJSON_Hooks) {
    if hooks.is_null() {
        cJSON_malloc = Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
            as Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
        cJSON_free = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        return;
    }
    cJSON_malloc = (if (*hooks).malloc_fn.is_some() {
        (*hooks).malloc_fn
            as Option<unsafe extern "C" fn(::core::ffi::c_ulong) -> *mut ::core::ffi::c_void>
    } else {
        Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
    }) as Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
    cJSON_free = (if (*hooks).free_fn.is_some() {
        (*hooks).free_fn as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>
    } else {
        Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
    }) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
}
unsafe extern "C" fn cJSON_New_Item() -> *mut cJSON {
    let mut node: *mut cJSON =
        cJSON_malloc.expect("non-null function pointer")(::core::mem::size_of::<cJSON>() as size_t)
            as *mut cJSON;
    if !node.is_null() {
        memset(
            node as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<cJSON>() as size_t,
        );
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_Delete(mut c: *mut cJSON) {
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    while !c.is_null() {
        next = (*c).next as *mut cJSON;
        if (*c).type_0 & cJSON_IsReference == 0 && !(*c).child.is_null() {
            cJSON_Delete((*c).child as *mut cJSON);
        }
        if (*c).type_0 & cJSON_IsReference == 0 && !(*c).valuestring.is_null() {
            cJSON_free.expect("non-null function pointer")(
                (*c).valuestring as *mut ::core::ffi::c_void,
            );
        }
        if (*c).type_0 & cJSON_StringIsConst == 0 && !(*c).string.is_null() {
            cJSON_free.expect("non-null function pointer")((*c).string as *mut ::core::ffi::c_void);
        }
        cJSON_free.expect("non-null function pointer")(c as *mut ::core::ffi::c_void);
        c = next;
    }
}
unsafe extern "C" fn parse_number(
    mut item: *mut cJSON,
    mut num: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut n: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut sign: ::core::ffi::c_double = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut scale: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut subscale: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut signsubscale: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *num as ::core::ffi::c_int == '-' as i32 {
        sign = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        num = num.offset(1);
    }
    if *num as ::core::ffi::c_int == '0' as i32 {
        num = num.offset(1);
    }
    if *num as ::core::ffi::c_int >= '1' as i32 && *num as ::core::ffi::c_int <= '9' as i32 {
        loop {
            let fresh9 = num;
            num = num.offset(1);
            n = n * 10.0f64 + (*fresh9 as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_double;
            if !(*num as ::core::ffi::c_int >= '0' as i32
                && *num as ::core::ffi::c_int <= '9' as i32)
            {
                break;
            }
        }
    }
    if *num as ::core::ffi::c_int == '.' as i32
        && *num.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
        && *num.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
    {
        num = num.offset(1);
        loop {
            let fresh10 = num;
            num = num.offset(1);
            n = n * 10.0f64
                + (*fresh10 as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_double;
            scale -= 1.;
            if !(*num as ::core::ffi::c_int >= '0' as i32
                && *num as ::core::ffi::c_int <= '9' as i32)
            {
                break;
            }
        }
    }
    if *num as ::core::ffi::c_int == 'e' as i32 || *num as ::core::ffi::c_int == 'E' as i32 {
        num = num.offset(1);
        if *num as ::core::ffi::c_int == '+' as i32 {
            num = num.offset(1);
        } else if *num as ::core::ffi::c_int == '-' as i32 {
            signsubscale = -(1 as ::core::ffi::c_int);
            num = num.offset(1);
        }
        while *num as ::core::ffi::c_int >= '0' as i32 && *num as ::core::ffi::c_int <= '9' as i32 {
            let fresh11 = num;
            num = num.offset(1);
            subscale =
                subscale * 10 as ::core::ffi::c_int + (*fresh11 as ::core::ffi::c_int - '0' as i32);
        }
    }
    n = sign
        * n
        * pow(
            10.0f64,
            scale + (subscale * signsubscale) as ::core::ffi::c_double,
        );
    (*item).valuedouble = n;
    (*item).valueint = n as ::core::ffi::c_int;
    (*item).type_0 = cJSON_Number;
    return num;
}
unsafe extern "C" fn pow2gt(mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    x -= 1;
    x |= x >> 1 as ::core::ffi::c_int;
    x |= x >> 2 as ::core::ffi::c_int;
    x |= x >> 4 as ::core::ffi::c_int;
    x |= x >> 8 as ::core::ffi::c_int;
    x |= x >> 16 as ::core::ffi::c_int;
    return x + 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ensure(
    mut p: *mut printbuffer,
    mut needed: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut newbuffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut newsize: ::core::ffi::c_int = 0;
    if p.is_null() || (*p).buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    needed += (*p).offset;
    if needed <= (*p).length {
        return (*p).buffer.offset((*p).offset as isize);
    }
    newsize = pow2gt(needed);
    newbuffer = cJSON_malloc.expect("non-null function pointer")(newsize as size_t)
        as *mut ::core::ffi::c_char;
    if newbuffer.is_null() {
        cJSON_free.expect("non-null function pointer")((*p).buffer as *mut ::core::ffi::c_void);
        (*p).length = 0 as ::core::ffi::c_int;
        (*p).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !newbuffer.is_null() {
        memcpy(
            newbuffer as *mut ::core::ffi::c_void,
            (*p).buffer as *const ::core::ffi::c_void,
            (*p).length as size_t,
        );
    }
    cJSON_free.expect("non-null function pointer")((*p).buffer as *mut ::core::ffi::c_void);
    (*p).length = newsize;
    (*p).buffer = newbuffer;
    return newbuffer.offset((*p).offset as isize);
}
unsafe extern "C" fn update(mut p: *mut printbuffer) -> ::core::ffi::c_int {
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if p.is_null() || (*p).buffer.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    str = (*p).buffer.offset((*p).offset as isize);
    return ((*p).offset as size_t).wrapping_add(strlen(str)) as ::core::ffi::c_int;
}
unsafe extern "C" fn print_number(
    mut item: *mut cJSON,
    mut p: *mut printbuffer,
) -> *mut ::core::ffi::c_char {
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut d: ::core::ffi::c_double = (*item).valuedouble;
    if d == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if !p.is_null() {
            str = ensure(p, 2 as ::core::ffi::c_int);
        } else {
            str = cJSON_malloc.expect("non-null function pointer")(2 as size_t)
                as *mut ::core::ffi::c_char;
        }
        if !str.is_null() {
            strcpy(str, b"0\0" as *const u8 as *const ::core::ffi::c_char);
        }
    } else if fabs((*item).valueint as ::core::ffi::c_double - d) <= DBL_EPSILON
        && d <= INT_MAX as ::core::ffi::c_double
        && d >= INT_MIN as ::core::ffi::c_double
    {
        if !p.is_null() {
            str = ensure(p, 21 as ::core::ffi::c_int);
        } else {
            str = cJSON_malloc.expect("non-null function pointer")(21 as size_t)
                as *mut ::core::ffi::c_char;
        }
        if !str.is_null() {
            sprintf(
                str,
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*item).valueint,
            );
        }
    } else {
        if !p.is_null() {
            str = ensure(p, 64 as ::core::ffi::c_int);
        } else {
            str = cJSON_malloc.expect("non-null function pointer")(64 as size_t)
                as *mut ::core::ffi::c_char;
        }
        if !str.is_null() {
            if fabs(floor(d) - d) <= DBL_EPSILON && fabs(d) < 1.0e60f64 {
                sprintf(str, b"%.0f\0" as *const u8 as *const ::core::ffi::c_char, d);
            } else if fabs(d) < 1.0e-6f64 || fabs(d) > 1.0e9f64 {
                sprintf(str, b"%e\0" as *const u8 as *const ::core::ffi::c_char, d);
            } else {
                sprintf(str, b"%f\0" as *const u8 as *const ::core::ffi::c_char, d);
            }
        }
    }
    return str;
}
unsafe extern "C" fn parse_hex4(mut str: *const ::core::ffi::c_char) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if *str as ::core::ffi::c_int >= '0' as i32 && *str as ::core::ffi::c_int <= '9' as i32 {
        h = h.wrapping_add((*str as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint);
    } else if *str as ::core::ffi::c_int >= 'A' as i32 && *str as ::core::ffi::c_int <= 'F' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'A' as i32)
                as ::core::ffi::c_uint,
        );
    } else if *str as ::core::ffi::c_int >= 'a' as i32 && *str as ::core::ffi::c_int <= 'f' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'a' as i32)
                as ::core::ffi::c_uint,
        );
    } else {
        return 0 as ::core::ffi::c_uint;
    }
    h = h << 4 as ::core::ffi::c_int;
    str = str.offset(1);
    if *str as ::core::ffi::c_int >= '0' as i32 && *str as ::core::ffi::c_int <= '9' as i32 {
        h = h.wrapping_add((*str as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint);
    } else if *str as ::core::ffi::c_int >= 'A' as i32 && *str as ::core::ffi::c_int <= 'F' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'A' as i32)
                as ::core::ffi::c_uint,
        );
    } else if *str as ::core::ffi::c_int >= 'a' as i32 && *str as ::core::ffi::c_int <= 'f' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'a' as i32)
                as ::core::ffi::c_uint,
        );
    } else {
        return 0 as ::core::ffi::c_uint;
    }
    h = h << 4 as ::core::ffi::c_int;
    str = str.offset(1);
    if *str as ::core::ffi::c_int >= '0' as i32 && *str as ::core::ffi::c_int <= '9' as i32 {
        h = h.wrapping_add((*str as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint);
    } else if *str as ::core::ffi::c_int >= 'A' as i32 && *str as ::core::ffi::c_int <= 'F' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'A' as i32)
                as ::core::ffi::c_uint,
        );
    } else if *str as ::core::ffi::c_int >= 'a' as i32 && *str as ::core::ffi::c_int <= 'f' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'a' as i32)
                as ::core::ffi::c_uint,
        );
    } else {
        return 0 as ::core::ffi::c_uint;
    }
    h = h << 4 as ::core::ffi::c_int;
    str = str.offset(1);
    if *str as ::core::ffi::c_int >= '0' as i32 && *str as ::core::ffi::c_int <= '9' as i32 {
        h = h.wrapping_add((*str as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint);
    } else if *str as ::core::ffi::c_int >= 'A' as i32 && *str as ::core::ffi::c_int <= 'F' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'A' as i32)
                as ::core::ffi::c_uint,
        );
    } else if *str as ::core::ffi::c_int >= 'a' as i32 && *str as ::core::ffi::c_int <= 'f' as i32 {
        h = h.wrapping_add(
            (10 as ::core::ffi::c_int + *str as ::core::ffi::c_int - 'a' as i32)
                as ::core::ffi::c_uint,
        );
    } else {
        return 0 as ::core::ffi::c_uint;
    }
    return h;
}
static mut firstByteMark: [::core::ffi::c_uchar; 7] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
unsafe extern "C" fn parse_string(
    mut item: *mut cJSON,
    mut str: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut ptr: *const ::core::ffi::c_char = str.offset(1 as ::core::ffi::c_int as isize);
    let mut ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut uc: ::core::ffi::c_uint = 0;
    let mut uc2: ::core::ffi::c_uint = 0;
    if *str as ::core::ffi::c_int != '"' as i32 {
        ep = str;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while *ptr as ::core::ffi::c_int != '"' as i32 && *ptr as ::core::ffi::c_int != 0 && {
        len += 1;
        len != 0
    } {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        if *fresh0 as ::core::ffi::c_int == '\\' as i32 {
            ptr = ptr.offset(1);
        }
    }
    out =
        cJSON_malloc.expect("non-null function pointer")((len + 1 as ::core::ffi::c_int) as size_t)
            as *mut ::core::ffi::c_char;
    if out.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    ptr = str.offset(1 as ::core::ffi::c_int as isize);
    ptr2 = out;
    while *ptr as ::core::ffi::c_int != '"' as i32 && *ptr as ::core::ffi::c_int != 0 {
        if *ptr as ::core::ffi::c_int != '\\' as i32 {
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            let fresh2 = ptr2;
            ptr2 = ptr2.offset(1);
            *fresh2 = *fresh1;
        } else {
            ptr = ptr.offset(1);
            let mut current_block_41: u64;
            match *ptr as ::core::ffi::c_int {
                98 => {
                    let fresh3 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh3 = '\u{8}' as i32 as ::core::ffi::c_char;
                }
                102 => {
                    let fresh4 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh4 = '\u{c}' as i32 as ::core::ffi::c_char;
                }
                110 => {
                    let fresh5 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh5 = '\n' as i32 as ::core::ffi::c_char;
                }
                114 => {
                    let fresh6 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh6 = '\r' as i32 as ::core::ffi::c_char;
                }
                116 => {
                    let fresh7 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh7 = '\t' as i32 as ::core::ffi::c_char;
                }
                117 => {
                    uc = parse_hex4(ptr.offset(1 as ::core::ffi::c_int as isize));
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    if !(uc >= 0xdc00 as ::core::ffi::c_uint && uc <= 0xdfff as ::core::ffi::c_uint
                        || uc == 0 as ::core::ffi::c_uint)
                    {
                        if uc >= 0xd800 as ::core::ffi::c_uint
                            && uc <= 0xdbff as ::core::ffi::c_uint
                        {
                            if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                != '\\' as i32
                                || *ptr.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    != 'u' as i32
                            {
                                current_block_41 = 9441801433784995173;
                            } else {
                                uc2 = parse_hex4(ptr.offset(3 as ::core::ffi::c_int as isize));
                                ptr = ptr.offset(6 as ::core::ffi::c_int as isize);
                                if uc2 < 0xdc00 as ::core::ffi::c_uint
                                    || uc2 > 0xdfff as ::core::ffi::c_uint
                                {
                                    current_block_41 = 9441801433784995173;
                                } else {
                                    uc = (0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            (uc & 0x3ff as ::core::ffi::c_uint)
                                                << 10 as ::core::ffi::c_int
                                                | uc2 & 0x3ff as ::core::ffi::c_uint,
                                        );
                                    current_block_41 = 1608152415753874203;
                                }
                            }
                        } else {
                            current_block_41 = 1608152415753874203;
                        }
                        match current_block_41 {
                            9441801433784995173 => {}
                            _ => {
                                len = 4 as ::core::ffi::c_int;
                                if uc < 0x80 as ::core::ffi::c_uint {
                                    len = 1 as ::core::ffi::c_int;
                                } else if uc < 0x800 as ::core::ffi::c_uint {
                                    len = 2 as ::core::ffi::c_int;
                                } else if uc < 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    len = 3 as ::core::ffi::c_int;
                                }
                                ptr2 = ptr2.offset(len as isize);
                                let mut current_block_38: u64;
                                match len {
                                    4 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as ::core::ffi::c_uint)
                                            & 0xbf as ::core::ffi::c_uint)
                                            as ::core::ffi::c_char;
                                        uc >>= 6 as ::core::ffi::c_int;
                                        current_block_38 = 3612666327593570700;
                                    }
                                    3 => {
                                        current_block_38 = 3612666327593570700;
                                    }
                                    2 => {
                                        current_block_38 = 7136120942015263881;
                                    }
                                    1 => {
                                        current_block_38 = 7071798390900347288;
                                    }
                                    _ => {
                                        current_block_38 = 4567019141635105728;
                                    }
                                }
                                match current_block_38 {
                                    3612666327593570700 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as ::core::ffi::c_uint)
                                            & 0xbf as ::core::ffi::c_uint)
                                            as ::core::ffi::c_char;
                                        uc >>= 6 as ::core::ffi::c_int;
                                        current_block_38 = 7136120942015263881;
                                    }
                                    _ => {}
                                }
                                match current_block_38 {
                                    7136120942015263881 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as ::core::ffi::c_uint)
                                            & 0xbf as ::core::ffi::c_uint)
                                            as ::core::ffi::c_char;
                                        uc >>= 6 as ::core::ffi::c_int;
                                        current_block_38 = 7071798390900347288;
                                    }
                                    _ => {}
                                }
                                match current_block_38 {
                                    7071798390900347288 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = (uc
                                            | firstByteMark[len as usize] as ::core::ffi::c_uint)
                                            as ::core::ffi::c_char;
                                    }
                                    _ => {}
                                }
                                ptr2 = ptr2.offset(len as isize);
                            }
                        }
                    }
                }
                _ => {
                    let fresh8 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh8 = *ptr;
                }
            }
            ptr = ptr.offset(1);
        }
    }
    *ptr2 = 0 as ::core::ffi::c_char;
    if *ptr as ::core::ffi::c_int == '"' as i32 {
        ptr = ptr.offset(1);
    }
    (*item).valuestring = out;
    (*item).type_0 = cJSON_String;
    return ptr;
}
unsafe extern "C" fn print_string_ptr(
    mut str: *const ::core::ffi::c_char,
    mut p: *mut printbuffer,
) -> *mut ::core::ffi::c_char {
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut flag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut token: ::core::ffi::c_uchar = 0;
    ptr = str;
    while *ptr != 0 {
        flag |= if *ptr as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (*ptr as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            || *ptr as ::core::ffi::c_int == '"' as i32
            || *ptr as ::core::ffi::c_int == '\\' as i32
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        ptr = ptr.offset(1);
    }
    if flag == 0 {
        len = ptr.offset_from(str) as ::core::ffi::c_long as ::core::ffi::c_int;
        if !p.is_null() {
            out = ensure(p, len + 3 as ::core::ffi::c_int);
        } else {
            out = cJSON_malloc.expect("non-null function pointer")(
                (len + 3 as ::core::ffi::c_int) as size_t,
            ) as *mut ::core::ffi::c_char;
        }
        if out.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        ptr2 = out;
        let fresh38 = ptr2;
        ptr2 = ptr2.offset(1);
        *fresh38 = '"' as i32 as ::core::ffi::c_char;
        strcpy(ptr2, str);
        *ptr2.offset(len as isize) = '"' as i32 as ::core::ffi::c_char;
        *ptr2.offset((len + 1 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_char;
        return out;
    }
    if str.is_null() {
        if !p.is_null() {
            out = ensure(p, 3 as ::core::ffi::c_int);
        } else {
            out = cJSON_malloc.expect("non-null function pointer")(3 as size_t)
                as *mut ::core::ffi::c_char;
        }
        if out.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        strcpy(out, b"\"\"\0" as *const u8 as *const ::core::ffi::c_char);
        return out;
    }
    ptr = str;
    loop {
        token = *ptr as ::core::ffi::c_uchar;
        if !(token as ::core::ffi::c_int != 0 && {
            len += 1;
            len != 0
        }) {
            break;
        }
        if !strchr(
            b"\"\\\x08\x0C\n\r\t\0" as *const u8 as *const ::core::ffi::c_char,
            token as ::core::ffi::c_int,
        )
        .is_null()
        {
            len += 1;
        } else if (token as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            len += 5 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
    }
    if !p.is_null() {
        out = ensure(p, len + 3 as ::core::ffi::c_int);
    } else {
        out = cJSON_malloc.expect("non-null function pointer")(
            (len + 3 as ::core::ffi::c_int) as size_t,
        ) as *mut ::core::ffi::c_char;
    }
    if out.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ptr2 = out;
    ptr = str;
    let fresh39 = ptr2;
    ptr2 = ptr2.offset(1);
    *fresh39 = '"' as i32 as ::core::ffi::c_char;
    while *ptr != 0 {
        if *ptr as ::core::ffi::c_uchar as ::core::ffi::c_int > 31 as ::core::ffi::c_int
            && *ptr as ::core::ffi::c_int != '"' as i32
            && *ptr as ::core::ffi::c_int != '\\' as i32
        {
            let fresh40 = ptr;
            ptr = ptr.offset(1);
            let fresh41 = ptr2;
            ptr2 = ptr2.offset(1);
            *fresh41 = *fresh40;
        } else {
            let fresh42 = ptr2;
            ptr2 = ptr2.offset(1);
            *fresh42 = '\\' as i32 as ::core::ffi::c_char;
            let fresh43 = ptr;
            ptr = ptr.offset(1);
            token = *fresh43 as ::core::ffi::c_uchar;
            match token as ::core::ffi::c_int {
                92 => {
                    let fresh44 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh44 = '\\' as i32 as ::core::ffi::c_char;
                }
                34 => {
                    let fresh45 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh45 = '"' as i32 as ::core::ffi::c_char;
                }
                8 => {
                    let fresh46 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh46 = 'b' as i32 as ::core::ffi::c_char;
                }
                12 => {
                    let fresh47 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh47 = 'f' as i32 as ::core::ffi::c_char;
                }
                10 => {
                    let fresh48 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh48 = 'n' as i32 as ::core::ffi::c_char;
                }
                13 => {
                    let fresh49 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh49 = 'r' as i32 as ::core::ffi::c_char;
                }
                9 => {
                    let fresh50 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh50 = 't' as i32 as ::core::ffi::c_char;
                }
                _ => {
                    sprintf(
                        ptr2,
                        b"u%04x\0" as *const u8 as *const ::core::ffi::c_char,
                        token as ::core::ffi::c_int,
                    );
                    ptr2 = ptr2.offset(5 as ::core::ffi::c_int as isize);
                }
            }
        }
    }
    let fresh51 = ptr2;
    ptr2 = ptr2.offset(1);
    *fresh51 = '"' as i32 as ::core::ffi::c_char;
    let fresh52 = ptr2;
    ptr2 = ptr2.offset(1);
    *fresh52 = 0 as ::core::ffi::c_char;
    return out;
}
unsafe extern "C" fn print_string(
    mut item: *mut cJSON,
    mut p: *mut printbuffer,
) -> *mut ::core::ffi::c_char {
    return print_string_ptr((*item).valuestring, p);
}
unsafe extern "C" fn skip(mut in_0: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    while !in_0.is_null()
        && *in_0 as ::core::ffi::c_int != 0
        && *in_0 as ::core::ffi::c_uchar as ::core::ffi::c_int <= 32 as ::core::ffi::c_int
    {
        in_0 = in_0.offset(1);
    }
    return in_0;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_ParseWithOpts(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut c: *mut cJSON = cJSON_New_Item();
    ep = ::core::ptr::null::<::core::ffi::c_char>();
    if c.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    end = parse_value(c, skip(value));
    if end.is_null() {
        cJSON_Delete(c);
        return ::core::ptr::null_mut::<cJSON>();
    }
    if require_null_terminated != 0 {
        end = skip(end);
        if *end != 0 {
            cJSON_Delete(c);
            ep = end;
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    if !return_parse_end.is_null() {
        *return_parse_end = end;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_Parse(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    return cJSON_ParseWithOpts(
        value,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_Print(mut item: *mut cJSON) -> *mut ::core::ffi::c_char {
    return print_value(
        item,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<printbuffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_PrintUnformatted(mut item: *mut cJSON) -> *mut ::core::ffi::c_char {
    return print_value(
        item,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<printbuffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_PrintBuffered(
    mut item: *mut cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut p: printbuffer = printbuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        offset: 0,
    };
    p.buffer = cJSON_malloc.expect("non-null function pointer")(prebuffer as size_t)
        as *mut ::core::ffi::c_char;
    p.length = prebuffer;
    p.offset = 0 as ::core::ffi::c_int;
    return print_value(item, 0 as ::core::ffi::c_int, fmt, &raw mut p);
}
unsafe extern "C" fn parse_value(
    mut item: *mut cJSON,
    mut value: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if value.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if strncmp(
        value,
        b"null\0" as *const u8 as *const ::core::ffi::c_char,
        4 as size_t,
    ) == 0
    {
        (*item).type_0 = cJSON_NULL;
        return value.offset(4 as ::core::ffi::c_int as isize);
    }
    if strncmp(
        value,
        b"false\0" as *const u8 as *const ::core::ffi::c_char,
        5 as size_t,
    ) == 0
    {
        (*item).type_0 = cJSON_False;
        return value.offset(5 as ::core::ffi::c_int as isize);
    }
    if strncmp(
        value,
        b"true\0" as *const u8 as *const ::core::ffi::c_char,
        4 as size_t,
    ) == 0
    {
        (*item).type_0 = cJSON_True;
        (*item).valueint = 1 as ::core::ffi::c_int;
        return value.offset(4 as ::core::ffi::c_int as isize);
    }
    if *value as ::core::ffi::c_int == '"' as i32 {
        return parse_string(item, value);
    }
    if *value as ::core::ffi::c_int == '-' as i32
        || *value as ::core::ffi::c_int >= '0' as i32 && *value as ::core::ffi::c_int <= '9' as i32
    {
        return parse_number(item, value);
    }
    if *value as ::core::ffi::c_int == '[' as i32 {
        return parse_array(item, value);
    }
    if *value as ::core::ffi::c_int == '{' as i32 {
        return parse_object(item, value);
    }
    ep = value;
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn print_value(
    mut item: *mut cJSON,
    mut depth: ::core::ffi::c_int,
    mut fmt: ::core::ffi::c_int,
    mut p: *mut printbuffer,
) -> *mut ::core::ffi::c_char {
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if item.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !p.is_null() {
        match (*item).type_0 & 255 as ::core::ffi::c_int {
            cJSON_NULL => {
                out = ensure(p, 5 as ::core::ffi::c_int);
                if !out.is_null() {
                    strcpy(out, b"null\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
            cJSON_False => {
                out = ensure(p, 6 as ::core::ffi::c_int);
                if !out.is_null() {
                    strcpy(out, b"false\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
            cJSON_True => {
                out = ensure(p, 5 as ::core::ffi::c_int);
                if !out.is_null() {
                    strcpy(out, b"true\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
            cJSON_Number => {
                out = print_number(item, p);
            }
            cJSON_String => {
                out = print_string(item, p);
            }
            cJSON_Array => {
                out = print_array(item, depth, fmt, p);
            }
            cJSON_Object => {
                out = print_object(item, depth, fmt, p);
            }
            _ => {}
        }
    } else {
        match (*item).type_0 & 255 as ::core::ffi::c_int {
            cJSON_NULL => {
                out = cJSON_strdup(b"null\0" as *const u8 as *const ::core::ffi::c_char);
            }
            cJSON_False => {
                out = cJSON_strdup(b"false\0" as *const u8 as *const ::core::ffi::c_char);
            }
            cJSON_True => {
                out = cJSON_strdup(b"true\0" as *const u8 as *const ::core::ffi::c_char);
            }
            cJSON_Number => {
                out = print_number(item, ::core::ptr::null_mut::<printbuffer>());
            }
            cJSON_String => {
                out = print_string(item, ::core::ptr::null_mut::<printbuffer>());
            }
            cJSON_Array => {
                out = print_array(item, depth, fmt, ::core::ptr::null_mut::<printbuffer>());
            }
            cJSON_Object => {
                out = print_object(item, depth, fmt, ::core::ptr::null_mut::<printbuffer>());
            }
            _ => {}
        }
    }
    return out;
}
unsafe extern "C" fn parse_array(
    mut item: *mut cJSON,
    mut value: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if *value as ::core::ffi::c_int != '[' as i32 {
        ep = value;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*item).type_0 = cJSON_Array;
    value = skip(value.offset(1 as ::core::ffi::c_int as isize));
    if *value as ::core::ffi::c_int == ']' as i32 {
        return value.offset(1 as ::core::ffi::c_int as isize);
    }
    child = cJSON_New_Item();
    (*item).child = child as *mut cJSON;
    if (*item).child.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    value = skip(parse_value(child, skip(value)));
    if value.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while *value as ::core::ffi::c_int == ',' as i32 {
        let mut new_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        new_item = cJSON_New_Item();
        if new_item.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        (*child).next = new_item as *mut cJSON;
        (*new_item).prev = child as *mut cJSON;
        child = new_item;
        value = skip(parse_value(
            child,
            skip(value.offset(1 as ::core::ffi::c_int as isize)),
        ));
        if value.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
    }
    if *value as ::core::ffi::c_int == ']' as i32 {
        return value.offset(1 as ::core::ffi::c_int as isize);
    }
    ep = value;
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn print_array(
    mut item: *mut cJSON,
    mut depth: ::core::ffi::c_int,
    mut fmt: ::core::ffi::c_int,
    mut p: *mut printbuffer,
) -> *mut ::core::ffi::c_char {
    let mut entries: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut child: *mut cJSON = (*item).child as *mut cJSON;
    let mut numentries: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmplen: size_t = 0 as size_t;
    while !child.is_null() {
        numentries += 1;
        child = (*child).next as *mut cJSON;
    }
    if numentries == 0 {
        if !p.is_null() {
            out = ensure(p, 3 as ::core::ffi::c_int);
        } else {
            out = cJSON_malloc.expect("non-null function pointer")(3 as size_t)
                as *mut ::core::ffi::c_char;
        }
        if !out.is_null() {
            strcpy(out, b"[]\0" as *const u8 as *const ::core::ffi::c_char);
        }
        return out;
    }
    if !p.is_null() {
        i = (*p).offset;
        ptr = ensure(p, 1 as ::core::ffi::c_int);
        if ptr.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        *ptr = '[' as i32 as ::core::ffi::c_char;
        (*p).offset += 1;
        child = (*item).child as *mut cJSON;
        while !child.is_null() && fail == 0 {
            print_value(child, depth + 1 as ::core::ffi::c_int, fmt, p);
            (*p).offset = update(p);
            if !(*child).next.is_null() {
                len = if fmt != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                ptr = ensure(p, len + 1 as ::core::ffi::c_int);
                if ptr.is_null() {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                let fresh53 = ptr;
                ptr = ptr.offset(1);
                *fresh53 = ',' as i32 as ::core::ffi::c_char;
                if fmt != 0 {
                    let fresh54 = ptr;
                    ptr = ptr.offset(1);
                    *fresh54 = ' ' as i32 as ::core::ffi::c_char;
                }
                *ptr = 0 as ::core::ffi::c_char;
                (*p).offset += len;
            }
            child = (*child).next as *mut cJSON;
        }
        ptr = ensure(p, 2 as ::core::ffi::c_int);
        if ptr.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let fresh55 = ptr;
        ptr = ptr.offset(1);
        *fresh55 = ']' as i32 as ::core::ffi::c_char;
        *ptr = 0 as ::core::ffi::c_char;
        out = (*p).buffer.offset(i as isize);
    } else {
        entries = cJSON_malloc.expect("non-null function pointer")(
            (numentries as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        ) as *mut *mut ::core::ffi::c_char;
        if entries.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        memset(
            entries as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (numentries as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        );
        child = (*item).child as *mut cJSON;
        while !child.is_null() && fail == 0 {
            ret = print_value(
                child,
                depth + 1 as ::core::ffi::c_int,
                fmt,
                ::core::ptr::null_mut::<printbuffer>(),
            );
            let fresh56 = i;
            i = i + 1;
            let ref mut fresh57 = *entries.offset(fresh56 as isize);
            *fresh57 = ret;
            if !ret.is_null() {
                len = (len as size_t).wrapping_add(
                    strlen(ret).wrapping_add(2 as size_t).wrapping_add(
                        (if fmt != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as size_t,
                    ),
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
            } else {
                fail = 1 as ::core::ffi::c_int;
            }
            child = (*child).next as *mut cJSON;
        }
        if fail == 0 {
            out = cJSON_malloc.expect("non-null function pointer")(len as size_t)
                as *mut ::core::ffi::c_char;
        }
        if out.is_null() {
            fail = 1 as ::core::ffi::c_int;
        }
        if fail != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < numentries {
                if !(*entries.offset(i as isize)).is_null() {
                    cJSON_free.expect("non-null function pointer")(
                        *entries.offset(i as isize) as *mut ::core::ffi::c_void
                    );
                }
                i += 1;
            }
            cJSON_free.expect("non-null function pointer")(entries as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        *out = '[' as i32 as ::core::ffi::c_char;
        ptr = out.offset(1 as ::core::ffi::c_int as isize);
        *ptr = 0 as ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < numentries {
            tmplen = strlen(*entries.offset(i as isize));
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                *entries.offset(i as isize) as *const ::core::ffi::c_void,
                tmplen,
            );
            ptr = ptr.offset(tmplen as isize);
            if i != numentries - 1 as ::core::ffi::c_int {
                let fresh58 = ptr;
                ptr = ptr.offset(1);
                *fresh58 = ',' as i32 as ::core::ffi::c_char;
                if fmt != 0 {
                    let fresh59 = ptr;
                    ptr = ptr.offset(1);
                    *fresh59 = ' ' as i32 as ::core::ffi::c_char;
                }
                *ptr = 0 as ::core::ffi::c_char;
            }
            cJSON_free.expect("non-null function pointer")(
                *entries.offset(i as isize) as *mut ::core::ffi::c_void
            );
            i += 1;
        }
        cJSON_free.expect("non-null function pointer")(entries as *mut ::core::ffi::c_void);
        let fresh60 = ptr;
        ptr = ptr.offset(1);
        *fresh60 = ']' as i32 as ::core::ffi::c_char;
        let fresh61 = ptr;
        ptr = ptr.offset(1);
        *fresh61 = 0 as ::core::ffi::c_char;
    }
    return out;
}
unsafe extern "C" fn parse_object(
    mut item: *mut cJSON,
    mut value: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if *value as ::core::ffi::c_int != '{' as i32 {
        ep = value;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*item).type_0 = cJSON_Object;
    value = skip(value.offset(1 as ::core::ffi::c_int as isize));
    if *value as ::core::ffi::c_int == '}' as i32 {
        return value.offset(1 as ::core::ffi::c_int as isize);
    }
    child = cJSON_New_Item();
    (*item).child = child as *mut cJSON;
    if (*item).child.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    value = skip(parse_string(child, skip(value)));
    if value.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*child).string = (*child).valuestring;
    (*child).valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *value as ::core::ffi::c_int != ':' as i32 {
        ep = value;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    value = skip(parse_value(
        child,
        skip(value.offset(1 as ::core::ffi::c_int as isize)),
    ));
    if value.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while *value as ::core::ffi::c_int == ',' as i32 {
        let mut new_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        new_item = cJSON_New_Item();
        if new_item.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        (*child).next = new_item as *mut cJSON;
        (*new_item).prev = child as *mut cJSON;
        child = new_item;
        value = skip(parse_string(
            child,
            skip(value.offset(1 as ::core::ffi::c_int as isize)),
        ));
        if value.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        (*child).string = (*child).valuestring;
        (*child).valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if *value as ::core::ffi::c_int != ':' as i32 {
            ep = value;
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        value = skip(parse_value(
            child,
            skip(value.offset(1 as ::core::ffi::c_int as isize)),
        ));
        if value.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
    }
    if *value as ::core::ffi::c_int == '}' as i32 {
        return value.offset(1 as ::core::ffi::c_int as isize);
    }
    ep = value;
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn print_object(
    mut item: *mut cJSON,
    mut depth: ::core::ffi::c_int,
    mut fmt: ::core::ffi::c_int,
    mut p: *mut printbuffer,
) -> *mut ::core::ffi::c_char {
    let mut entries: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut child: *mut cJSON = (*item).child as *mut cJSON;
    let mut numentries: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmplen: size_t = 0 as size_t;
    while !child.is_null() {
        numentries += 1;
        child = (*child).next as *mut cJSON;
    }
    if numentries == 0 {
        if !p.is_null() {
            out = ensure(
                p,
                if fmt != 0 {
                    depth + 4 as ::core::ffi::c_int
                } else {
                    3 as ::core::ffi::c_int
                },
            );
        } else {
            out = cJSON_malloc.expect("non-null function pointer")(
                (if fmt != 0 {
                    depth + 4 as ::core::ffi::c_int
                } else {
                    3 as ::core::ffi::c_int
                }) as size_t,
            ) as *mut ::core::ffi::c_char;
        }
        if out.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        ptr = out;
        let fresh12 = ptr;
        ptr = ptr.offset(1);
        *fresh12 = '{' as i32 as ::core::ffi::c_char;
        if fmt != 0 {
            let fresh13 = ptr;
            ptr = ptr.offset(1);
            *fresh13 = '\n' as i32 as ::core::ffi::c_char;
            i = 0 as ::core::ffi::c_int;
            while i < depth - 1 as ::core::ffi::c_int {
                let fresh14 = ptr;
                ptr = ptr.offset(1);
                *fresh14 = '\t' as i32 as ::core::ffi::c_char;
                i += 1;
            }
        }
        let fresh15 = ptr;
        ptr = ptr.offset(1);
        *fresh15 = '}' as i32 as ::core::ffi::c_char;
        let fresh16 = ptr;
        ptr = ptr.offset(1);
        *fresh16 = 0 as ::core::ffi::c_char;
        return out;
    }
    if !p.is_null() {
        i = (*p).offset;
        len = if fmt != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        ptr = ensure(p, len + 1 as ::core::ffi::c_int);
        if ptr.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let fresh17 = ptr;
        ptr = ptr.offset(1);
        *fresh17 = '{' as i32 as ::core::ffi::c_char;
        if fmt != 0 {
            let fresh18 = ptr;
            ptr = ptr.offset(1);
            *fresh18 = '\n' as i32 as ::core::ffi::c_char;
        }
        *ptr = 0 as ::core::ffi::c_char;
        (*p).offset += len;
        child = (*item).child as *mut cJSON;
        depth += 1;
        while !child.is_null() {
            if fmt != 0 {
                ptr = ensure(p, depth);
                if ptr.is_null() {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                j = 0 as ::core::ffi::c_int;
                while j < depth {
                    let fresh19 = ptr;
                    ptr = ptr.offset(1);
                    *fresh19 = '\t' as i32 as ::core::ffi::c_char;
                    j += 1;
                }
                (*p).offset += depth;
            }
            print_string_ptr((*child).string, p);
            (*p).offset = update(p);
            len = if fmt != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            ptr = ensure(p, len);
            if ptr.is_null() {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            let fresh20 = ptr;
            ptr = ptr.offset(1);
            *fresh20 = ':' as i32 as ::core::ffi::c_char;
            if fmt != 0 {
                let fresh21 = ptr;
                ptr = ptr.offset(1);
                *fresh21 = '\t' as i32 as ::core::ffi::c_char;
            }
            (*p).offset += len;
            print_value(child, depth, fmt, p);
            (*p).offset = update(p);
            len = (if fmt != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) + (if !(*child).next.is_null() {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            });
            ptr = ensure(p, len + 1 as ::core::ffi::c_int);
            if ptr.is_null() {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            if !(*child).next.is_null() {
                let fresh22 = ptr;
                ptr = ptr.offset(1);
                *fresh22 = ',' as i32 as ::core::ffi::c_char;
            }
            if fmt != 0 {
                let fresh23 = ptr;
                ptr = ptr.offset(1);
                *fresh23 = '\n' as i32 as ::core::ffi::c_char;
            }
            *ptr = 0 as ::core::ffi::c_char;
            (*p).offset += len;
            child = (*child).next as *mut cJSON;
        }
        ptr = ensure(
            p,
            if fmt != 0 {
                depth + 1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            },
        );
        if ptr.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if fmt != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < depth - 1 as ::core::ffi::c_int {
                let fresh24 = ptr;
                ptr = ptr.offset(1);
                *fresh24 = '\t' as i32 as ::core::ffi::c_char;
                i += 1;
            }
        }
        let fresh25 = ptr;
        ptr = ptr.offset(1);
        *fresh25 = '}' as i32 as ::core::ffi::c_char;
        *ptr = 0 as ::core::ffi::c_char;
        out = (*p).buffer.offset(i as isize);
    } else {
        entries = cJSON_malloc.expect("non-null function pointer")(
            (numentries as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        ) as *mut *mut ::core::ffi::c_char;
        if entries.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        names = cJSON_malloc.expect("non-null function pointer")(
            (numentries as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        ) as *mut *mut ::core::ffi::c_char;
        if names.is_null() {
            cJSON_free.expect("non-null function pointer")(entries as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        memset(
            entries as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(numentries as size_t),
        );
        memset(
            names as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(numentries as size_t),
        );
        child = (*item).child as *mut cJSON;
        depth += 1;
        if fmt != 0 {
            len += depth;
        }
        while !child.is_null() {
            str = print_string_ptr((*child).string, ::core::ptr::null_mut::<printbuffer>());
            let ref mut fresh26 = *names.offset(i as isize);
            *fresh26 = str;
            ret = print_value(child, depth, fmt, ::core::ptr::null_mut::<printbuffer>());
            let fresh27 = i;
            i = i + 1;
            let ref mut fresh28 = *entries.offset(fresh27 as isize);
            *fresh28 = ret;
            if !str.is_null() && !ret.is_null() {
                len = (len as size_t).wrapping_add(
                    strlen(ret)
                        .wrapping_add(strlen(str))
                        .wrapping_add(2 as size_t)
                        .wrapping_add(
                            (if fmt != 0 {
                                2 as ::core::ffi::c_int + depth
                            } else {
                                0 as ::core::ffi::c_int
                            }) as size_t,
                        ),
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
            } else {
                fail = 1 as ::core::ffi::c_int;
            }
            child = (*child).next as *mut cJSON;
        }
        if fail == 0 {
            out = cJSON_malloc.expect("non-null function pointer")(len as size_t)
                as *mut ::core::ffi::c_char;
        }
        if out.is_null() {
            fail = 1 as ::core::ffi::c_int;
        }
        if fail != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < numentries {
                if !(*names.offset(i as isize)).is_null() {
                    cJSON_free.expect("non-null function pointer")(
                        *names.offset(i as isize) as *mut ::core::ffi::c_void
                    );
                }
                if !(*entries.offset(i as isize)).is_null() {
                    cJSON_free.expect("non-null function pointer")(
                        *entries.offset(i as isize) as *mut ::core::ffi::c_void
                    );
                }
                i += 1;
            }
            cJSON_free.expect("non-null function pointer")(names as *mut ::core::ffi::c_void);
            cJSON_free.expect("non-null function pointer")(entries as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        *out = '{' as i32 as ::core::ffi::c_char;
        ptr = out.offset(1 as ::core::ffi::c_int as isize);
        if fmt != 0 {
            let fresh29 = ptr;
            ptr = ptr.offset(1);
            *fresh29 = '\n' as i32 as ::core::ffi::c_char;
        }
        *ptr = 0 as ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < numentries {
            if fmt != 0 {
                j = 0 as ::core::ffi::c_int;
                while j < depth {
                    let fresh30 = ptr;
                    ptr = ptr.offset(1);
                    *fresh30 = '\t' as i32 as ::core::ffi::c_char;
                    j += 1;
                }
            }
            tmplen = strlen(*names.offset(i as isize));
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                *names.offset(i as isize) as *const ::core::ffi::c_void,
                tmplen,
            );
            ptr = ptr.offset(tmplen as isize);
            let fresh31 = ptr;
            ptr = ptr.offset(1);
            *fresh31 = ':' as i32 as ::core::ffi::c_char;
            if fmt != 0 {
                let fresh32 = ptr;
                ptr = ptr.offset(1);
                *fresh32 = '\t' as i32 as ::core::ffi::c_char;
            }
            strcpy(ptr, *entries.offset(i as isize));
            ptr = ptr.offset(strlen(*entries.offset(i as isize)) as isize);
            if i != numentries - 1 as ::core::ffi::c_int {
                let fresh33 = ptr;
                ptr = ptr.offset(1);
                *fresh33 = ',' as i32 as ::core::ffi::c_char;
            }
            if fmt != 0 {
                let fresh34 = ptr;
                ptr = ptr.offset(1);
                *fresh34 = '\n' as i32 as ::core::ffi::c_char;
            }
            *ptr = 0 as ::core::ffi::c_char;
            cJSON_free.expect("non-null function pointer")(
                *names.offset(i as isize) as *mut ::core::ffi::c_void
            );
            cJSON_free.expect("non-null function pointer")(
                *entries.offset(i as isize) as *mut ::core::ffi::c_void
            );
            i += 1;
        }
        cJSON_free.expect("non-null function pointer")(names as *mut ::core::ffi::c_void);
        cJSON_free.expect("non-null function pointer")(entries as *mut ::core::ffi::c_void);
        if fmt != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < depth - 1 as ::core::ffi::c_int {
                let fresh35 = ptr;
                ptr = ptr.offset(1);
                *fresh35 = '\t' as i32 as ::core::ffi::c_char;
                i += 1;
            }
        }
        let fresh36 = ptr;
        ptr = ptr.offset(1);
        *fresh36 = '}' as i32 as ::core::ffi::c_char;
        let fresh37 = ptr;
        ptr = ptr.offset(1);
        *fresh37 = 0 as ::core::ffi::c_char;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_GetArraySize(mut array: *mut cJSON) -> ::core::ffi::c_int {
    let mut c: *mut cJSON = (*array).child as *mut cJSON;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !c.is_null() {
        i += 1;
        c = (*c).next as *mut cJSON;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_GetArrayItem(
    mut array: *mut cJSON,
    mut item: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut c: *mut cJSON = (*array).child as *mut cJSON;
    while !c.is_null() && item > 0 as ::core::ffi::c_int {
        item -= 1;
        c = (*c).next as *mut cJSON;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_GetObjectItem(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut c: *mut cJSON = (*object).child as *mut cJSON;
    while !c.is_null() && cJSON_strcasecmp((*c).string, string) != 0 {
        c = (*c).next as *mut cJSON;
    }
    return c;
}
unsafe extern "C" fn suffix_object(mut prev: *mut cJSON, mut item: *mut cJSON) {
    (*prev).next = item as *mut cJSON;
    (*item).prev = prev as *mut cJSON;
}
unsafe extern "C" fn create_reference(mut item: *mut cJSON) -> *mut cJSON {
    let mut ref_0: *mut cJSON = cJSON_New_Item();
    if ref_0.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    memcpy(
        ref_0 as *mut ::core::ffi::c_void,
        item as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cJSON>() as size_t,
    );
    (*ref_0).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*ref_0).type_0 |= cJSON_IsReference;
    (*ref_0).prev = ::core::ptr::null_mut::<cJSON>();
    (*ref_0).next = (*ref_0).prev;
    return ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_AddItemToArray(mut array: *mut cJSON, mut item: *mut cJSON) {
    let mut c: *mut cJSON = (*array).child as *mut cJSON;
    if item.is_null() {
        return;
    }
    if c.is_null() {
        (*array).child = item as *mut cJSON;
    } else {
        while !c.is_null() && !(*c).next.is_null() {
            c = (*c).next as *mut cJSON;
        }
        suffix_object(c, item);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_AddItemToObject(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) {
    if item.is_null() {
        return;
    }
    if !(*item).string.is_null() {
        cJSON_free.expect("non-null function pointer")((*item).string as *mut ::core::ffi::c_void);
    }
    (*item).string = cJSON_strdup(string);
    cJSON_AddItemToArray(object, item);
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_AddItemToObjectCS(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) {
    if item.is_null() {
        return;
    }
    if (*item).type_0 & cJSON_StringIsConst == 0 && !(*item).string.is_null() {
        cJSON_free.expect("non-null function pointer")((*item).string as *mut ::core::ffi::c_void);
    }
    (*item).string = string as *mut ::core::ffi::c_char;
    (*item).type_0 |= cJSON_StringIsConst;
    cJSON_AddItemToArray(object, item);
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) {
    cJSON_AddItemToArray(array, create_reference(item));
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_AddItemReferenceToObject(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) {
    cJSON_AddItemToObject(object, string, create_reference(item));
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_DetachItemFromArray(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut c: *mut cJSON = (*array).child as *mut cJSON;
    while !c.is_null() && which > 0 as ::core::ffi::c_int {
        c = (*c).next as *mut cJSON;
        which -= 1;
    }
    if c.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if !(*c).prev.is_null() {
        (*(*c).prev).next = (*c).next;
    }
    if !(*c).next.is_null() {
        (*(*c).next).prev = (*c).prev;
    }
    if c == (*array).child {
        (*array).child = (*c).next;
    }
    (*c).next = ::core::ptr::null_mut::<cJSON>();
    (*c).prev = (*c).next;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    cJSON_Delete(cJSON_DetachItemFromArray(array, which));
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_DetachItemFromObject(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: *mut cJSON = (*object).child as *mut cJSON;
    while !c.is_null() && cJSON_strcasecmp((*c).string, string) != 0 {
        i += 1;
        c = (*c).next as *mut cJSON;
    }
    if !c.is_null() {
        return cJSON_DetachItemFromArray(object, i);
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    cJSON_Delete(cJSON_DetachItemFromObject(object, string));
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_InsertItemInArray(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) {
    let mut c: *mut cJSON = (*array).child as *mut cJSON;
    while !c.is_null() && which > 0 as ::core::ffi::c_int {
        c = (*c).next as *mut cJSON;
        which -= 1;
    }
    if c.is_null() {
        cJSON_AddItemToArray(array, newitem);
        return;
    }
    (*newitem).next = c as *mut cJSON;
    (*newitem).prev = (*c).prev;
    (*c).prev = newitem as *mut cJSON;
    if c == (*array).child {
        (*array).child = newitem as *mut cJSON;
    } else {
        (*(*newitem).prev).next = newitem as *mut cJSON;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) {
    let mut c: *mut cJSON = (*array).child as *mut cJSON;
    while !c.is_null() && which > 0 as ::core::ffi::c_int {
        c = (*c).next as *mut cJSON;
        which -= 1;
    }
    if c.is_null() {
        return;
    }
    (*newitem).next = (*c).next;
    (*newitem).prev = (*c).prev;
    if !(*newitem).next.is_null() {
        (*(*newitem).next).prev = newitem as *mut cJSON;
    }
    if c == (*array).child {
        (*array).child = newitem as *mut cJSON;
    } else {
        (*(*newitem).prev).next = newitem as *mut cJSON;
    }
    (*c).prev = ::core::ptr::null_mut::<cJSON>();
    (*c).next = (*c).prev;
    cJSON_Delete(c);
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_ReplaceItemInObject(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: *mut cJSON = (*object).child as *mut cJSON;
    while !c.is_null() && cJSON_strcasecmp((*c).string, string) != 0 {
        i += 1;
        c = (*c).next as *mut cJSON;
    }
    if !c.is_null() {
        (*newitem).string = cJSON_strdup(string);
        cJSON_ReplaceItemInArray(object, i, newitem);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateNull() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = cJSON_NULL;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateTrue() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = cJSON_True;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateFalse() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = cJSON_False;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateBool(mut b: ::core::ffi::c_int) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = if b != 0 { cJSON_True } else { cJSON_False };
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateNumber(mut num: ::core::ffi::c_double) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = cJSON_Number;
        (*item).valuedouble = num;
        (*item).valueint = num as ::core::ffi::c_int;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateString(mut string: *const ::core::ffi::c_char) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = cJSON_String;
        (*item).valuestring = cJSON_strdup(string);
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateArray() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = cJSON_Array;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateObject() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item();
    if !item.is_null() {
        (*item).type_0 = cJSON_Object;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateIntArray(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = cJSON_CreateArray();
    i = 0 as ::core::ffi::c_int;
    while !a.is_null() && i < count {
        n = cJSON_CreateNumber(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i += 1;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateFloatArray(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = cJSON_CreateArray();
    i = 0 as ::core::ffi::c_int;
    while !a.is_null() && i < count {
        n = cJSON_CreateNumber(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i += 1;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateDoubleArray(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = cJSON_CreateArray();
    i = 0 as ::core::ffi::c_int;
    while !a.is_null() && i < count {
        n = cJSON_CreateNumber(*numbers.offset(i as isize));
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i += 1;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_CreateStringArray(
    mut strings: *mut *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = cJSON_CreateArray();
    i = 0 as ::core::ffi::c_int;
    while !a.is_null() && i < count {
        n = cJSON_CreateString(*strings.offset(i as isize));
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i += 1;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_Duplicate(
    mut item: *mut cJSON,
    mut recurse: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut newitem: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut cptr: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut nptr: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut newchild: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    newitem = cJSON_New_Item();
    if newitem.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    (*newitem).type_0 = (*item).type_0 & !cJSON_IsReference;
    (*newitem).valueint = (*item).valueint;
    (*newitem).valuedouble = (*item).valuedouble;
    if !(*item).valuestring.is_null() {
        (*newitem).valuestring = cJSON_strdup((*item).valuestring);
        if (*newitem).valuestring.is_null() {
            cJSON_Delete(newitem);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    if !(*item).string.is_null() {
        (*newitem).string = cJSON_strdup((*item).string);
        if (*newitem).string.is_null() {
            cJSON_Delete(newitem);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    if recurse == 0 {
        return newitem;
    }
    cptr = (*item).child as *mut cJSON;
    while !cptr.is_null() {
        newchild = cJSON_Duplicate(cptr, 1 as ::core::ffi::c_int);
        if newchild.is_null() {
            cJSON_Delete(newitem);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if !nptr.is_null() {
            (*nptr).next = newchild as *mut cJSON;
            (*newchild).prev = nptr as *mut cJSON;
            nptr = newchild;
        } else {
            (*newitem).child = newchild as *mut cJSON;
            nptr = newchild;
        }
        cptr = (*cptr).next as *mut cJSON;
    }
    return newitem;
}
#[no_mangle]
pub unsafe extern "C" fn cJSON_Minify(mut json: *mut ::core::ffi::c_char) {
    let mut into: *mut ::core::ffi::c_char = json;
    while *json != 0 {
        if *json as ::core::ffi::c_int == ' ' as i32 {
            json = json.offset(1);
        } else if *json as ::core::ffi::c_int == '\t' as i32 {
            json = json.offset(1);
        } else if *json as ::core::ffi::c_int == '\r' as i32 {
            json = json.offset(1);
        } else if *json as ::core::ffi::c_int == '\n' as i32 {
            json = json.offset(1);
        } else if *json as ::core::ffi::c_int == '/' as i32
            && *json.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
        {
            while *json as ::core::ffi::c_int != 0 && *json as ::core::ffi::c_int != '\n' as i32 {
                json = json.offset(1);
            }
        } else if *json as ::core::ffi::c_int == '/' as i32
            && *json.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
        {
            while *json as ::core::ffi::c_int != 0
                && !(*json as ::core::ffi::c_int == '*' as i32
                    && *json.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '/' as i32)
            {
                json = json.offset(1);
            }
            json = json.offset(2 as ::core::ffi::c_int as isize);
        } else if *json as ::core::ffi::c_int == '"' as i32 {
            let fresh62 = json;
            json = json.offset(1);
            let fresh63 = into;
            into = into.offset(1);
            *fresh63 = *fresh62;
            while *json as ::core::ffi::c_int != 0 && *json as ::core::ffi::c_int != '"' as i32 {
                if *json as ::core::ffi::c_int == '\\' as i32 {
                    let fresh64 = json;
                    json = json.offset(1);
                    let fresh65 = into;
                    into = into.offset(1);
                    *fresh65 = *fresh64;
                }
                let fresh66 = json;
                json = json.offset(1);
                let fresh67 = into;
                into = into.offset(1);
                *fresh67 = *fresh66;
            }
            let fresh68 = json;
            json = json.offset(1);
            let fresh69 = into;
            into = into.offset(1);
            *fresh69 = *fresh68;
        } else {
            let fresh70 = json;
            json = json.offset(1);
            let fresh71 = into;
            into = into.offset(1);
            *fresh71 = *fresh70;
        }
    }
    *into = 0 as ::core::ffi::c_char;
}
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;

#[no_mangle] pub extern "C" fn __assert_rtn(_a: *const std::os::raw::c_char,_b: *const std::os::raw::c_char,_c: std::os::raw::c_int,_d: *const std::os::raw::c_char)->!{std::process::abort()}

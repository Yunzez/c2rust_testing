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
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex_t {
    pub type_0: ::core::ffi::c_uchar,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ch: ::core::ffi::c_uchar,
    pub ccl: *mut ::core::ffi::c_uchar,
}
pub type re_t = *mut regex_t;
pub const UNUSED: C2RustUnnamed_1 = 0;
pub const CHAR: C2RustUnnamed_1 = 7;
pub const CHAR_CLASS: C2RustUnnamed_1 = 8;
pub const INV_CHAR_CLASS: C2RustUnnamed_1 = 9;
pub const NOT_WHITESPACE: C2RustUnnamed_1 = 15;
pub const WHITESPACE: C2RustUnnamed_1 = 14;
pub const NOT_ALPHA: C2RustUnnamed_1 = 13;
pub const ALPHA: C2RustUnnamed_1 = 12;
pub const NOT_DIGIT: C2RustUnnamed_1 = 11;
pub const DIGIT: C2RustUnnamed_1 = 10;
pub const QUESTIONMARK: C2RustUnnamed_1 = 4;
pub const PLUS: C2RustUnnamed_1 = 6;
pub const STAR: C2RustUnnamed_1 = 5;
pub const DOT: C2RustUnnamed_1 = 1;
pub const END: C2RustUnnamed_1 = 3;
pub const BEGIN: C2RustUnnamed_1 = 2;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const MAX_REGEXP_OBJECTS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MAX_CHAR_CLASS_LEN: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn re_match(
    mut pattern: *const ::core::ffi::c_char,
    mut text: *const ::core::ffi::c_char,
    mut matchlength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return re_matchp(re_compile(pattern), text, matchlength);
}
#[no_mangle]
pub unsafe extern "C" fn re_matchp(
    mut pattern: re_t,
    mut text: *const ::core::ffi::c_char,
    mut matchlength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *matchlength = 0 as ::core::ffi::c_int;
    if !pattern.is_null() {
        if (*pattern.offset(0 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_int
            == BEGIN as ::core::ffi::c_int
        {
            return if matchpattern(
                pattern.offset(1 as ::core::ffi::c_int as isize) as *mut regex_t,
                text,
                matchlength,
            ) != 0
            {
                0 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
        } else {
            let mut idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            loop {
                idx += 1 as ::core::ffi::c_int;
                if matchpattern(pattern as *mut regex_t, text, matchlength) != 0 {
                    if *text.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '\0' as i32
                    {
                        return -(1 as ::core::ffi::c_int);
                    }
                    return idx;
                }
                let fresh4 = text;
                text = text.offset(1);
                if !(*fresh4 as ::core::ffi::c_int != '\0' as i32) {
                    break;
                }
            }
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn re_compile(mut pattern: *const ::core::ffi::c_char) -> re_t {
    static mut re_compiled: [regex_t; 30] = [regex_t {
        type_0: 0,
        u: C2RustUnnamed { ch: 0 },
    }; 30];
    static mut ccl_buf: [::core::ffi::c_uchar; 40] = [0; 40];
    let mut ccl_bufidx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_char = 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *pattern.offset(i as isize) as ::core::ffi::c_int != '\0' as i32
        && (j + 1 as ::core::ffi::c_int) < MAX_REGEXP_OBJECTS
    {
        c = *pattern.offset(i as isize);
        match c as ::core::ffi::c_int {
            94 => {
                re_compiled[j as usize].type_0 =
                    BEGIN as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            36 => {
                re_compiled[j as usize].type_0 = END as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            46 => {
                re_compiled[j as usize].type_0 = DOT as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            42 => {
                re_compiled[j as usize].type_0 = STAR as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            43 => {
                re_compiled[j as usize].type_0 = PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            63 => {
                re_compiled[j as usize].type_0 =
                    QUESTIONMARK as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            92 => {
                if *pattern.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    != '\0' as i32
                {
                    i += 1 as ::core::ffi::c_int;
                    match *pattern.offset(i as isize) as ::core::ffi::c_int {
                        100 => {
                            re_compiled[j as usize].type_0 =
                                DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        }
                        68 => {
                            re_compiled[j as usize].type_0 =
                                NOT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        }
                        119 => {
                            re_compiled[j as usize].type_0 =
                                ALPHA as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        }
                        87 => {
                            re_compiled[j as usize].type_0 =
                                NOT_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        }
                        115 => {
                            re_compiled[j as usize].type_0 =
                                WHITESPACE as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        }
                        83 => {
                            re_compiled[j as usize].type_0 =
                                NOT_WHITESPACE as ::core::ffi::c_int as ::core::ffi::c_uchar;
                        }
                        _ => {
                            re_compiled[j as usize].type_0 =
                                CHAR as ::core::ffi::c_int as ::core::ffi::c_uchar;
                            re_compiled[j as usize].u.ch =
                                *pattern.offset(i as isize) as ::core::ffi::c_uchar;
                        }
                    }
                }
            }
            91 => {
                let mut buf_begin: ::core::ffi::c_int = ccl_bufidx;
                if *pattern.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == '^' as i32
                {
                    re_compiled[j as usize].type_0 =
                        INV_CHAR_CLASS as ::core::ffi::c_int as ::core::ffi::c_uchar;
                    i += 1 as ::core::ffi::c_int;
                    if *pattern.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        return ::core::ptr::null_mut::<regex_t>();
                    }
                } else {
                    re_compiled[j as usize].type_0 =
                        CHAR_CLASS as ::core::ffi::c_int as ::core::ffi::c_uchar;
                }
                loop {
                    i += 1;
                    if !(*pattern.offset(i as isize) as ::core::ffi::c_int != ']' as i32
                        && *pattern.offset(i as isize) as ::core::ffi::c_int != '\0' as i32)
                    {
                        break;
                    }
                    if *pattern.offset(i as isize) as ::core::ffi::c_int == '\\' as i32 {
                        if ccl_bufidx >= MAX_CHAR_CLASS_LEN - 1 as ::core::ffi::c_int {
                            return ::core::ptr::null_mut::<regex_t>();
                        }
                        if *pattern.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            return ::core::ptr::null_mut::<regex_t>();
                        }
                        let fresh0 = i;
                        i = i + 1;
                        let fresh1 = ccl_bufidx;
                        ccl_bufidx = ccl_bufidx + 1;
                        ccl_buf[fresh1 as usize] =
                            *pattern.offset(fresh0 as isize) as ::core::ffi::c_uchar;
                    } else if ccl_bufidx >= MAX_CHAR_CLASS_LEN {
                        return ::core::ptr::null_mut::<regex_t>();
                    }
                    let fresh2 = ccl_bufidx;
                    ccl_bufidx = ccl_bufidx + 1;
                    ccl_buf[fresh2 as usize] = *pattern.offset(i as isize) as ::core::ffi::c_uchar;
                }
                if ccl_bufidx >= MAX_CHAR_CLASS_LEN {
                    return ::core::ptr::null_mut::<regex_t>();
                }
                let fresh3 = ccl_bufidx;
                ccl_bufidx = ccl_bufidx + 1;
                ccl_buf[fresh3 as usize] = 0 as ::core::ffi::c_uchar;
                re_compiled[j as usize].u.ccl = (&raw mut ccl_buf as *mut ::core::ffi::c_uchar)
                    .offset(buf_begin as isize)
                    as *mut ::core::ffi::c_uchar;
            }
            _ => {
                re_compiled[j as usize].type_0 = CHAR as ::core::ffi::c_int as ::core::ffi::c_uchar;
                re_compiled[j as usize].u.ch = c as ::core::ffi::c_uchar;
            }
        }
        if *pattern.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<regex_t>();
        }
        i += 1 as ::core::ffi::c_int;
        j += 1 as ::core::ffi::c_int;
    }
    re_compiled[j as usize].type_0 = UNUSED as ::core::ffi::c_int as ::core::ffi::c_uchar;
    return &raw mut re_compiled as *mut regex_t as re_t;
}
#[no_mangle]
pub unsafe extern "C" fn re_print(mut pattern: *mut regex_t) {
    let mut types: [*const ::core::ffi::c_char; 17] = [
        b"UNUSED\0" as *const u8 as *const ::core::ffi::c_char,
        b"DOT\0" as *const u8 as *const ::core::ffi::c_char,
        b"BEGIN\0" as *const u8 as *const ::core::ffi::c_char,
        b"END\0" as *const u8 as *const ::core::ffi::c_char,
        b"QUESTIONMARK\0" as *const u8 as *const ::core::ffi::c_char,
        b"STAR\0" as *const u8 as *const ::core::ffi::c_char,
        b"PLUS\0" as *const u8 as *const ::core::ffi::c_char,
        b"CHAR\0" as *const u8 as *const ::core::ffi::c_char,
        b"CHAR_CLASS\0" as *const u8 as *const ::core::ffi::c_char,
        b"INV_CHAR_CLASS\0" as *const u8 as *const ::core::ffi::c_char,
        b"DIGIT\0" as *const u8 as *const ::core::ffi::c_char,
        b"NOT_DIGIT\0" as *const u8 as *const ::core::ffi::c_char,
        b"ALPHA\0" as *const u8 as *const ::core::ffi::c_char,
        b"NOT_ALPHA\0" as *const u8 as *const ::core::ffi::c_char,
        b"WHITESPACE\0" as *const u8 as *const ::core::ffi::c_char,
        b"NOT_WHITESPACE\0" as *const u8 as *const ::core::ffi::c_char,
        b"BRANCH\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_char = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_REGEXP_OBJECTS {
        if (*pattern.offset(i as isize)).type_0 as ::core::ffi::c_int
            == UNUSED as ::core::ffi::c_int
        {
            break;
        }
        printf(
            b"type: %s\0" as *const u8 as *const ::core::ffi::c_char,
            types[(*pattern.offset(i as isize)).type_0 as usize],
        );
        if (*pattern.offset(i as isize)).type_0 as ::core::ffi::c_int
            == CHAR_CLASS as ::core::ffi::c_int
            || (*pattern.offset(i as isize)).type_0 as ::core::ffi::c_int
                == INV_CHAR_CLASS as ::core::ffi::c_int
        {
            printf(b" [\0" as *const u8 as *const ::core::ffi::c_char);
            j = 0 as ::core::ffi::c_int;
            while j < MAX_CHAR_CLASS_LEN {
                c = *(*pattern.offset(i as isize)).u.ccl.offset(j as isize) as ::core::ffi::c_char;
                if c as ::core::ffi::c_int == '\0' as i32 || c as ::core::ffi::c_int == ']' as i32 {
                    break;
                }
                printf(
                    b"%c\0" as *const u8 as *const ::core::ffi::c_char,
                    c as ::core::ffi::c_int,
                );
                j += 1;
            }
            printf(b"]\0" as *const u8 as *const ::core::ffi::c_char);
        } else if (*pattern.offset(i as isize)).type_0 as ::core::ffi::c_int
            == CHAR as ::core::ffi::c_int
        {
            printf(
                b" '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                (*pattern.offset(i as isize)).u.ch as ::core::ffi::c_int,
            );
        }
        printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        i += 1;
    }
}
unsafe extern "C" fn matchdigit(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int;
}
unsafe extern "C" fn matchalpha(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int;
}
unsafe extern "C" fn matchwhitespace(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int;
}
unsafe extern "C" fn matchalphanum(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int == '_' as i32 || matchalpha(c) != 0 || matchdigit(c) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn matchrange(
    mut c: ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int != '-' as i32
        && *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        && *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
        && *str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        && *str.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        && (c as ::core::ffi::c_int
            >= *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            && c as ::core::ffi::c_int
                <= *str.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn matchdot(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ismetachar(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int == 's' as i32
        || c as ::core::ffi::c_int == 'S' as i32
        || c as ::core::ffi::c_int == 'w' as i32
        || c as ::core::ffi::c_int == 'W' as i32
        || c as ::core::ffi::c_int == 'd' as i32
        || c as ::core::ffi::c_int == 'D' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn matchmetachar(
    mut c: ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        100 => return matchdigit(c),
        68 => return (matchdigit(c) == 0) as ::core::ffi::c_int,
        119 => return matchalphanum(c),
        87 => return (matchalphanum(c) == 0) as ::core::ffi::c_int,
        115 => return matchwhitespace(c),
        83 => return (matchwhitespace(c) == 0) as ::core::ffi::c_int,
        _ => {
            return (c as ::core::ffi::c_int
                == *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn matchcharclass(
    mut c: ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    loop {
        if matchrange(c, str) != 0 {
            return 1 as ::core::ffi::c_int;
        } else if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\\' as i32
        {
            str = str.offset(1 as ::core::ffi::c_int as isize);
            if matchmetachar(c, str) != 0 {
                return 1 as ::core::ffi::c_int;
            } else if c as ::core::ffi::c_int
                == *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                && ismetachar(c) == 0
            {
                return 1 as ::core::ffi::c_int;
            }
        } else if c as ::core::ffi::c_int
            == *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            if c as ::core::ffi::c_int == '-' as i32 {
                return (*str.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == '\0' as i32
                    || *str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '\0' as i32) as ::core::ffi::c_int;
            } else {
                return 1 as ::core::ffi::c_int;
            }
        }
        let fresh7 = str;
        str = str.offset(1);
        if !(*fresh7 as ::core::ffi::c_int != '\0' as i32) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn matchone(mut p: regex_t, mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match p.type_0 as ::core::ffi::c_int {
        1 => return matchdot(c),
        8 => return matchcharclass(c, p.u.ccl as *const ::core::ffi::c_char),
        9 => {
            return (matchcharclass(c, p.u.ccl as *const ::core::ffi::c_char) == 0)
                as ::core::ffi::c_int;
        }
        10 => return matchdigit(c),
        11 => return (matchdigit(c) == 0) as ::core::ffi::c_int,
        12 => return matchalphanum(c),
        13 => return (matchalphanum(c) == 0) as ::core::ffi::c_int,
        14 => return matchwhitespace(c),
        15 => return (matchwhitespace(c) == 0) as ::core::ffi::c_int,
        _ => {
            return (p.u.ch as ::core::ffi::c_int == c as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn matchstar(
    mut p: regex_t,
    mut pattern: *mut regex_t,
    mut text: *const ::core::ffi::c_char,
    mut matchlength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut prelen: ::core::ffi::c_int = *matchlength;
    let mut prepoint: *const ::core::ffi::c_char = text;
    while *text.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        && matchone(p, *text) != 0
    {
        text = text.offset(1);
        *matchlength += 1;
    }
    while text >= prepoint {
        let fresh9 = text;
        text = text.offset(-1);
        if matchpattern(pattern, fresh9, matchlength) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        *matchlength -= 1;
    }
    *matchlength = prelen;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn matchplus(
    mut p: regex_t,
    mut pattern: *mut regex_t,
    mut text: *const ::core::ffi::c_char,
    mut matchlength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut prepoint: *const ::core::ffi::c_char = text;
    while *text.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        && matchone(p, *text) != 0
    {
        text = text.offset(1);
        *matchlength += 1;
    }
    while text > prepoint {
        let fresh8 = text;
        text = text.offset(-1);
        if matchpattern(pattern, fresh8, matchlength) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        *matchlength -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn matchquestion(
    mut p: regex_t,
    mut pattern: *mut regex_t,
    mut text: *const ::core::ffi::c_char,
    mut matchlength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if p.type_0 as ::core::ffi::c_int == UNUSED as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if matchpattern(pattern, text, matchlength) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if *text as ::core::ffi::c_int != 0 && {
        let fresh10 = text;
        text = text.offset(1);
        matchone(p, *fresh10) != 0
    } {
        if matchpattern(pattern, text, matchlength) != 0 {
            *matchlength += 1;
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn matchpattern(
    mut pattern: *mut regex_t,
    mut text: *const ::core::ffi::c_char,
    mut matchlength: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pre: ::core::ffi::c_int = *matchlength;
    loop {
        if (*pattern.offset(0 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_int
            == UNUSED as ::core::ffi::c_int
            || (*pattern.offset(1 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_int
                == QUESTIONMARK as ::core::ffi::c_int
        {
            return matchquestion(
                *pattern.offset(0 as ::core::ffi::c_int as isize),
                pattern.offset(2 as ::core::ffi::c_int as isize) as *mut regex_t,
                text,
                matchlength,
            );
        } else if (*pattern.offset(1 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_int
            == STAR as ::core::ffi::c_int
        {
            return matchstar(
                *pattern.offset(0 as ::core::ffi::c_int as isize),
                pattern.offset(2 as ::core::ffi::c_int as isize) as *mut regex_t,
                text,
                matchlength,
            );
        } else if (*pattern.offset(1 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_int
            == PLUS as ::core::ffi::c_int
        {
            return matchplus(
                *pattern.offset(0 as ::core::ffi::c_int as isize),
                pattern.offset(2 as ::core::ffi::c_int as isize) as *mut regex_t,
                text,
                matchlength,
            );
        } else if (*pattern.offset(0 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_int
            == END as ::core::ffi::c_int
            && (*pattern.offset(1 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_int
                == UNUSED as ::core::ffi::c_int
        {
            return (*text.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32) as ::core::ffi::c_int;
        }
        *matchlength += 1;
        if !(*text.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
            && {
                let fresh5 = pattern;
                pattern = pattern.offset(1);
                let fresh6 = text;
                text = text.offset(1);
                matchone(*fresh5, *fresh6) != 0
            })
        {
            break;
        }
    }
    *matchlength = pre;
    return 0 as ::core::ffi::c_int;
}

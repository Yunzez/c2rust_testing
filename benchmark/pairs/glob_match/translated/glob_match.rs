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
pub type uint8_t = __uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn match_class(
    mut pat: *const ::core::ffi::c_char,
    mut plen: size_t,
    mut p: size_t,
    mut tc: ::core::ffi::c_char,
    mut next_p: *mut size_t,
) -> ::core::ffi::c_int {
    let mut negate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut matched: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = p.wrapping_add(1 as size_t);
    if i < plen
        && (*pat.offset(i as isize) as ::core::ffi::c_int == '!' as i32
            || *pat.offset(i as isize) as ::core::ffi::c_int == '^' as i32)
    {
        negate = 1 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    let mut first: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < plen && (*pat.offset(i as isize) as ::core::ffi::c_int != ']' as i32 || first != 0) {
        let mut lo: ::core::ffi::c_char = *pat.offset(i as isize);
        first = 0 as ::core::ffi::c_int;
        if i.wrapping_add(2 as size_t) < plen
            && *pat.offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int == '-' as i32
            && *pat.offset(i.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int != ']' as i32
        {
            let mut hi: ::core::ffi::c_char = *pat.offset(i.wrapping_add(2 as size_t) as isize);
            if tc as ::core::ffi::c_uchar as ::core::ffi::c_int
                >= lo as ::core::ffi::c_uchar as ::core::ffi::c_int
                && tc as ::core::ffi::c_uchar as ::core::ffi::c_int
                    <= hi as ::core::ffi::c_uchar as ::core::ffi::c_int
            {
                matched = 1 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(3 as size_t);
        } else {
            if tc as ::core::ffi::c_int == lo as ::core::ffi::c_int {
                matched = 1 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1 as size_t);
        }
    }
    if i >= plen || *pat.offset(i as isize) as ::core::ffi::c_int != ']' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    *next_p = i.wrapping_add(1 as size_t);
    if negate != 0 {
        matched = (matched == 0) as ::core::ffi::c_int;
    }
    return if matched != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn match_at(
    mut pat: *const ::core::ffi::c_char,
    mut plen: size_t,
    mut p: size_t,
    mut text: *const ::core::ffi::c_char,
    mut tlen: size_t,
    mut t: size_t,
) -> ::core::ffi::c_int {
    while p < plen {
        let mut pc: ::core::ffi::c_char = *pat.offset(p as isize);
        if pc as ::core::ffi::c_int == '*' as i32 {
            while p < plen && *pat.offset(p as isize) as ::core::ffi::c_int == '*' as i32 {
                p = p.wrapping_add(1);
            }
            if p == plen {
                return 1 as ::core::ffi::c_int;
            }
            let mut k: size_t = t;
            while k <= tlen {
                if match_at(pat, plen, p, text, tlen, k) != 0 {
                    return 1 as ::core::ffi::c_int;
                }
                k = k.wrapping_add(1);
            }
            return 0 as ::core::ffi::c_int;
        } else if pc as ::core::ffi::c_int == '?' as i32 {
            if t >= tlen {
                return 0 as ::core::ffi::c_int;
            }
            p = p.wrapping_add(1);
            t = t.wrapping_add(1);
        } else if pc as ::core::ffi::c_int == '[' as i32 {
            if t >= tlen {
                return 0 as ::core::ffi::c_int;
            }
            let mut next_p: size_t = 0;
            let mut r: ::core::ffi::c_int =
                match_class(pat, plen, p, *text.offset(t as isize), &raw mut next_p);
            if r < 0 as ::core::ffi::c_int {
                if *text.offset(t as isize) as ::core::ffi::c_int != '[' as i32 {
                    return 0 as ::core::ffi::c_int;
                }
                p = p.wrapping_add(1);
                t = t.wrapping_add(1);
            } else if r == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            } else {
                p = next_p;
                t = t.wrapping_add(1);
            }
        } else if pc as ::core::ffi::c_int == '\\' as i32 {
            p = p.wrapping_add(1);
            if p >= plen {
                if t >= tlen || *text.offset(t as isize) as ::core::ffi::c_int != '\\' as i32 {
                    return 0 as ::core::ffi::c_int;
                }
                t = t.wrapping_add(1);
            } else {
                if t >= tlen
                    || *text.offset(t as isize) as ::core::ffi::c_int
                        != *pat.offset(p as isize) as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                p = p.wrapping_add(1);
                t = t.wrapping_add(1);
            }
        } else {
            if t >= tlen
                || *text.offset(t as isize) as ::core::ffi::c_int != pc as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            p = p.wrapping_add(1);
            t = t.wrapping_add(1);
        }
    }
    return (t == tlen) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn glob_match(
    mut pattern: *const ::core::ffi::c_char,
    mut text: *const uint8_t,
    mut tlen: size_t,
) -> ::core::ffi::c_int {
    if pattern.is_null() || text.is_null() && tlen != 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    let mut plen: size_t = 0 as size_t;
    while *pattern.offset(plen as isize) as ::core::ffi::c_int != '\0' as i32 {
        plen = plen.wrapping_add(1);
    }
    return match_at(
        pattern,
        plen,
        0 as size_t,
        text as *const ::core::ffi::c_char,
        tlen,
        0 as size_t,
    );
}

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub const BASE64_PAD: ::core::ffi::c_int = '=' as i32;
pub const BASE64DE_FIRST: ::core::ffi::c_int = '+' as i32;
pub const BASE64DE_LAST: ::core::ffi::c_int = 'z' as i32;
static mut base64en: [::core::ffi::c_char; 64] = [
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'J' as i32 as ::core::ffi::c_char,
    'K' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'Q' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'Z' as i32 as ::core::ffi::c_char,
    'a' as i32 as ::core::ffi::c_char,
    'b' as i32 as ::core::ffi::c_char,
    'c' as i32 as ::core::ffi::c_char,
    'd' as i32 as ::core::ffi::c_char,
    'e' as i32 as ::core::ffi::c_char,
    'f' as i32 as ::core::ffi::c_char,
    'g' as i32 as ::core::ffi::c_char,
    'h' as i32 as ::core::ffi::c_char,
    'i' as i32 as ::core::ffi::c_char,
    'j' as i32 as ::core::ffi::c_char,
    'k' as i32 as ::core::ffi::c_char,
    'l' as i32 as ::core::ffi::c_char,
    'm' as i32 as ::core::ffi::c_char,
    'n' as i32 as ::core::ffi::c_char,
    'o' as i32 as ::core::ffi::c_char,
    'p' as i32 as ::core::ffi::c_char,
    'q' as i32 as ::core::ffi::c_char,
    'r' as i32 as ::core::ffi::c_char,
    's' as i32 as ::core::ffi::c_char,
    't' as i32 as ::core::ffi::c_char,
    'u' as i32 as ::core::ffi::c_char,
    'v' as i32 as ::core::ffi::c_char,
    'w' as i32 as ::core::ffi::c_char,
    'x' as i32 as ::core::ffi::c_char,
    'y' as i32 as ::core::ffi::c_char,
    'z' as i32 as ::core::ffi::c_char,
    '0' as i32 as ::core::ffi::c_char,
    '1' as i32 as ::core::ffi::c_char,
    '2' as i32 as ::core::ffi::c_char,
    '3' as i32 as ::core::ffi::c_char,
    '4' as i32 as ::core::ffi::c_char,
    '5' as i32 as ::core::ffi::c_char,
    '6' as i32 as ::core::ffi::c_char,
    '7' as i32 as ::core::ffi::c_char,
    '8' as i32 as ::core::ffi::c_char,
    '9' as i32 as ::core::ffi::c_char,
    '+' as i32 as ::core::ffi::c_char,
    '/' as i32 as ::core::ffi::c_char,
];
static mut base64de: [::core::ffi::c_uchar; 128] = [
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    54 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn base64_encode(
    mut in_0: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
    mut out: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut s: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uchar = 0;
    let mut l: ::core::ffi::c_uchar = 0;
    s = 0 as ::core::ffi::c_int;
    l = 0 as ::core::ffi::c_uchar;
    j = 0 as ::core::ffi::c_uint;
    i = j;
    while i < inlen {
        c = *in_0.offset(i as isize);
        match s {
            0 => {
                s = 1 as ::core::ffi::c_int;
                let fresh0 = j;
                j = j.wrapping_add(1);
                *out.offset(fresh0 as isize) =
                    base64en[(c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int) as usize];
            }
            1 => {
                s = 2 as ::core::ffi::c_int;
                let fresh1 = j;
                j = j.wrapping_add(1);
                *out.offset(fresh1 as isize) =
                    base64en[((l as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                        << 4 as ::core::ffi::c_int
                        | c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int) as usize];
            }
            2 => {
                s = 0 as ::core::ffi::c_int;
                let fresh2 = j;
                j = j.wrapping_add(1);
                *out.offset(fresh2 as isize) =
                    base64en[((l as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int
                        | c as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                            & 0x3 as ::core::ffi::c_int) as usize];
                let fresh3 = j;
                j = j.wrapping_add(1);
                *out.offset(fresh3 as isize) =
                    base64en[(c as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as usize];
            }
            _ => {}
        }
        l = c;
        i = i.wrapping_add(1);
    }
    match s {
        1 => {
            let fresh4 = j;
            j = j.wrapping_add(1);
            *out.offset(fresh4 as isize) =
                base64en[((l as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int) as usize];
            let fresh5 = j;
            j = j.wrapping_add(1);
            *out.offset(fresh5 as isize) = BASE64_PAD as ::core::ffi::c_char;
            let fresh6 = j;
            j = j.wrapping_add(1);
            *out.offset(fresh6 as isize) = BASE64_PAD as ::core::ffi::c_char;
        }
        2 => {
            let fresh7 = j;
            j = j.wrapping_add(1);
            *out.offset(fresh7 as isize) =
                base64en[((l as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) as usize];
            let fresh8 = j;
            j = j.wrapping_add(1);
            *out.offset(fresh8 as isize) = BASE64_PAD as ::core::ffi::c_char;
        }
        _ => {}
    }
    *out.offset(j as isize) = 0 as ::core::ffi::c_char;
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn base64_decode(
    mut in_0: *const ::core::ffi::c_char,
    mut inlen: ::core::ffi::c_uint,
    mut out: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uchar = 0;
    if inlen & 0x3 as ::core::ffi::c_uint != 0 {
        return 0 as ::core::ffi::c_uint;
    }
    j = 0 as ::core::ffi::c_uint;
    i = j;
    while i < inlen {
        if *in_0.offset(i as isize) as ::core::ffi::c_int == BASE64_PAD {
            break;
        }
        if (*in_0.offset(i as isize) as ::core::ffi::c_int) < BASE64DE_FIRST
            || *in_0.offset(i as isize) as ::core::ffi::c_int > BASE64DE_LAST
        {
            return 0 as ::core::ffi::c_uint;
        }
        c = base64de[*in_0.offset(i as isize) as ::core::ffi::c_uchar as usize];
        if c as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_uint;
        }
        match i & 0x3 as ::core::ffi::c_uint {
            0 => {
                *out.offset(j as isize) = ((c as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            1 => {
                let fresh9 = j;
                j = j.wrapping_add(1);
                let ref mut fresh10 = *out.offset(fresh9 as isize);
                *fresh10 = (*fresh10 as ::core::ffi::c_int
                    | c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        & 0x3 as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
                *out.offset(j as isize) = ((c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            2 => {
                let fresh11 = j;
                j = j.wrapping_add(1);
                let ref mut fresh12 = *out.offset(fresh11 as isize);
                *fresh12 = (*fresh12 as ::core::ffi::c_int
                    | c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
                *out.offset(j as isize) = ((c as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            3 => {
                let fresh13 = j;
                j = j.wrapping_add(1);
                let ref mut fresh14 = *out.offset(fresh13 as isize);
                *fresh14 = (*fresh14 as ::core::ffi::c_int | c as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    return j;
}

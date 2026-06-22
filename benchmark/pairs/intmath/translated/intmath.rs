#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
unsafe extern "C" fn gcd_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    if b == 0 as uint64_t {
        return a;
    }
    return gcd_u64(b, a.wrapping_rem(b));
}
unsafe extern "C" fn ipow_mod(
    mut base: uint64_t,
    mut exp: uint64_t,
    mut mod_0: uint64_t,
) -> uint64_t {
    let mut result: uint64_t = 0;
    if mod_0 == 1 as uint64_t {
        return 0 as uint64_t;
    }
    result = 1 as uint64_t;
    if mod_0 != 0 as uint64_t {
        base = base.wrapping_rem(mod_0);
    }
    while exp > 0 as uint64_t {
        if exp & 1 as uint64_t != 0 {
            if mod_0 != 0 as uint64_t {
                result = result.wrapping_mul(base).wrapping_rem(mod_0);
            } else {
                result = result.wrapping_mul(base);
            }
        }
        exp >>= 1 as ::core::ffi::c_int;
        if exp > 0 as uint64_t {
            if mod_0 != 0 as uint64_t {
                base = base.wrapping_mul(base).wrapping_rem(mod_0);
            } else {
                base = base.wrapping_mul(base);
            }
        }
    }
    return result;
}
unsafe extern "C" fn isqrt_u64(mut n: uint64_t) -> uint64_t {
    let mut lo: uint64_t = 0 as uint64_t;
    let mut hi: uint64_t = n;
    let mut ans: uint64_t = 0 as uint64_t;
    if n < 2 as uint64_t {
        return n;
    }
    while lo <= hi {
        let mut mid: uint64_t = lo.wrapping_add(hi.wrapping_sub(lo).wrapping_div(2 as uint64_t));
        if mid != 0 as uint64_t && mid <= n.wrapping_div(mid) {
            ans = mid;
            lo = mid.wrapping_add(1 as uint64_t);
        } else {
            if mid == 0 as uint64_t {
                break;
            }
            hi = mid.wrapping_sub(1 as uint64_t);
        }
    }
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn intmath_eval(
    mut op: ::core::ffi::c_int,
    mut a: uint64_t,
    mut b: uint64_t,
) -> uint64_t {
    match op {
        0 => return gcd_u64(a, b),
        1 => return ipow_mod(a, b, 0 as uint64_t),
        2 => return ipow_mod(a, b, 1000000007 as uint64_t),
        3 => return isqrt_u64(a),
        _ => return 0 as uint64_t,
    };
}

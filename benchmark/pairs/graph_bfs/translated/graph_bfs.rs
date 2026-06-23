#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type size_t = usize;
pub const BFS_MAX_V: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn graph_bfs(
    mut adj: *const uint8_t,
    mut n: size_t,
    mut start: size_t,
    mut order: *mut size_t,
) -> size_t {
    let mut visited: [uint8_t; 64] = [0; 64];
    let mut queue: [size_t; 64] = [0; 64];
    let mut head: size_t = 0 as size_t;
    let mut tail: size_t = 0 as size_t;
    let mut out: size_t = 0 as size_t;
    if n == 0 as size_t || n > BFS_MAX_V as size_t || start >= n {
        return 0 as size_t;
    }
    let mut i: size_t = 0 as size_t;
    while i < n {
        visited[i as usize] = 0 as uint8_t;
        i = i.wrapping_add(1);
    }
    visited[start as usize] = 1 as uint8_t;
    let fresh0 = tail;
    tail = tail.wrapping_add(1);
    queue[fresh0 as usize] = start;
    while head < tail {
        let fresh1 = head;
        head = head.wrapping_add(1);
        let mut u: size_t = queue[fresh1 as usize];
        let fresh2 = out;
        out = out.wrapping_add(1);
        *order.offset(fresh2 as isize) = u;
        let mut v: size_t = 0 as size_t;
        while v < n {
            if *adj.offset(u.wrapping_mul(n).wrapping_add(v) as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                && visited[v as usize] == 0
            {
                visited[v as usize] = 1 as uint8_t;
                let fresh3 = tail;
                tail = tail.wrapping_add(1);
                queue[fresh3 as usize] = v;
            }
            v = v.wrapping_add(1);
        }
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn count_edges(mut adj: *const uint8_t, mut n: size_t) -> size_t {
    let mut c: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < n.wrapping_mul(n) {
        if *adj.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            c = c.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return c;
}

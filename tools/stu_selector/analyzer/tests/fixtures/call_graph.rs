#![allow(dead_code)]

pub fn swap(a: &mut u32, b: &mut u32) {
    std::mem::swap(a, b);
}
pub fn leaf() {}
pub fn other() {}
pub fn direct(n: u32) {
    if n > 0 {
        direct(n - 1);
    }
}

pub fn wrapper() {
    fn helper() {
        leaf();
    }
    helper();
}
pub fn nested_recursive(n: u32) {
    fn inner(n: u32) {
        if n > 0 {
            inner(n - 1);
        } else {
            leaf();
        }
    }
    inner(n);
}
pub fn mutual_helpers(n: u32) {
    fn even(n: u32) {
        if n > 0 {
            odd(n - 1);
        }
    }
    fn odd(n: u32) {
        if n > 0 {
            even(n - 1);
        }
    }
    even(n);
}
pub fn outer_cycle(n: u32) {
    fn helper(n: u32) {
        if n > 0 {
            outer_cycle(n - 1);
        }
    }
    helper(n);
}
pub fn unused_helper() {
    fn unused() {
        unused();
        other();
    }
    leaf();
}
pub fn deep_wrapper() {
    fn level_one() {
        fn level_two() {
            leaf();
        }
        level_two();
    }
    level_one();
}
pub fn shadowing() {
    fn leaf() {
        other();
    }
    leaf();
}
pub fn second_wrapper() {
    fn helper() {
        other();
    }
    helper();
}
pub fn indirect(f: fn()) {
    f();
}

pub mod left {
    pub fn twin() {
        super::leaf();
    }
    pub fn invoke_left() {
        twin();
    }
}
pub mod right {
    pub fn twin() {
        super::other();
    }
    pub fn invoke_right() {
        twin();
    }
}

pub struct Boxed;
impl Boxed {
    pub fn method(&self) {
        leaf();
    }
    pub fn wrapped_method(&self) {
        fn helper() {
            other();
        }
        helper();
    }
}
pub fn use_method(b: &Boxed) {
    b.method();
    b.wrapped_method();
}

impl Default for Boxed {
    fn default() -> Self {
        fn hidden() {
            other();
        }
        hidden();
        Boxed
    }
}
#[cfg(test)]
mod checks {
    #[test]
    fn not_a_candidate() {
        fn hidden() {
            leaf();
        }
        hidden();
    }
    fn leaf() {
        super::other();
    }
}

pub mod foreign {
    unsafe extern "C" {
        pub fn leaf();
    }
}
pub fn foreign_call() {
    unsafe {
        foreign::leaf();
    }
}

pub struct Holder<T>(T);
impl Holder<u8> {
    pub fn access(&self) {
        leaf();
    }
}
impl Holder<u16> {
    pub fn access(&self) {
        other();
    }
}
pub fn use_u8(a: &Holder<u8>) {
    a.access();
}
pub fn use_u16(a: &Holder<u16>) {
    a.access();
}

mod exported {
    #[unsafe(no_mangle)]
    pub extern "C" fn published() {
        super::leaf();
    }
    #[export_name = "wire_alias"]
    pub extern "C" fn actual_name() {
        super::other();
    }
    // C ABI alone does not provide an unmangled symbol.
    pub extern "C" fn still_mangled() {
        super::leaf();
    }
}
mod declarations {
    unsafe extern "C" {
        pub fn published();
        #[link_name = "wire_alias"]
        pub fn local_alias();
        pub fn still_mangled();
    }
}
pub fn linked_call() {
    unsafe {
        declarations::published();
    }
}
pub fn linked_alias() {
    unsafe {
        declarations::local_alias();
    }
}
pub fn unlinked_mangled_call() {
    unsafe {
        declarations::still_mangled();
    }
}

#[no_mangle]
pub extern "C" fn exported_recursion(n: u32) {
    unsafe extern "C" {
        #[link_name = "exported_recursion"]
        fn declared_alias(n: u32);
    }
    if n > 0 {
        unsafe {
            declared_alias(n - 1);
        }
    }
}

#![feature(core_intrinsics)]

#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(linkage)]

#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(c_variadic)]








pub mod lil;
// main dropped (CLI)
use crate::laertes_rt::*;// This module is only injected into the rewritten programs, it is not used
// internally. So, disable unused function warnings.
#[allow(dead_code)]
pub(crate) mod __laertes_array {
    use std::{
        cell::{Ref, RefCell, RefMut},
        cmp::Ordering,
        convert::{TryFrom, TryInto},
        marker::PhantomData,
    };

    pub trait Get<'g, T: 'g> {
        fn get(&'g self, index: SliceIndex) -> T;

        fn get_add(&'g self, index: usize) -> T {
            self.get(SliceIndex::Add(index))
        }

        fn get_sub(&'g self, index: usize) -> T {
            self.get(SliceIndex::Sub(index))
        }

        fn get_offset(&'g self, index: isize) -> T {
            self.get(SliceIndex::Offset(index))
        }
    }

    pub trait GetMut<'g, T: 'g> {
        fn get_mut(&'g mut self, index: SliceIndex) -> T;

        fn get_add_mut(&'g mut self, index: usize) -> T {
            self.get_mut(SliceIndex::Add(index))
        }

        fn get_sub_mut(&'g mut self, index: usize) -> T {
            self.get_mut(SliceIndex::Sub(index))
        }

        fn get_offset_mut(&'g mut self, index: isize) -> T {
            self.get_mut(SliceIndex::Offset(index))
        }
    }

    pub trait IntoSlice<S>: Sized {
        fn into_slice(self, index: SliceIndex) -> S;

        fn into_slice_add(self, index: usize) -> S {
            self.into_slice(SliceIndex::Add(index))
        }

        fn into_slice_sub(self, index: usize) -> S {
            self.into_slice(SliceIndex::Sub(index))
        }

        fn into_slice_offset(self, index: isize) -> S {
            self.into_slice(SliceIndex::Offset(index))
        }
    }

    pub trait Slice<'s, S: 's> {
        fn slice(&'s self, index: SliceIndex) -> S;

        fn slice_add(&'s self, index: usize) -> S {
            self.slice(SliceIndex::Add(index))
        }

        fn slice_sub(&'s self, index: usize) -> S {
            self.slice(SliceIndex::Sub(index))
        }

        fn slice_offset(&'s self, index: isize) -> S {
            self.slice(SliceIndex::Offset(index))
        }
    }

    pub trait SliceMut<'s, S: 's> {
        fn slice_mut(&'s mut self, index: SliceIndex) -> S;

        fn slice_add_mut(&'s mut self, index: usize) -> S {
            self.slice_mut(SliceIndex::Add(index))
        }

        fn slice_sub_mut(&'s mut self, index: usize) -> S {
            self.slice_mut(SliceIndex::Sub(index))
        }

        fn slice_offset_mut(&'s mut self, index: isize) -> S {
            self.slice_mut(SliceIndex::Offset(index))
        }
    }

    #[derive(Clone, Copy)]
    pub enum SliceIndex {
        Add(usize),
        Sub(usize),
        Offset(isize),
    }

    impl SliceIndex {
        fn apply(self, offset: usize) -> usize {
            match self {
                SliceIndex::Add(i) => offset.checked_add(i),
                SliceIndex::Sub(i) => offset.checked_sub(i),
                SliceIndex::Offset(i) => {
                    if i < 0 {
                        offset.checked_sub(i.wrapping_abs() as usize)
                    } else {
                        offset.checked_add(i as usize)
                    }
                },
            }
            .unwrap()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct CustomSlice<'a, T: 'a, S: 'a> {
        offset: usize,
        slice: S,
        m: PhantomData<&'a T>,
    }

    impl<'a, T: 'a, S: 'a> CustomSlice<'a, T, S> {
        pub const fn new(slice: S) -> CustomSlice<'a, T, S> {
            CustomSlice {
                offset: 0,
                slice,
                m: PhantomData,
            }
        }
    }

    impl<'a, T: 'a> CustomSlice<'a, T, Box<[T]>> {
        pub fn boxed_from_vec(vec: Vec<T>) -> CustomSlice<'a, T, Box<[T]>> {
            CustomSlice {
                offset: 0,
                slice: vec.into_boxed_slice(),
                m: PhantomData::default(),
            }
        }
    }

    impl<'a, T: 'a, S: TryFrom<Vec<RefCell<T>>> + 'a> CustomSlice<'a, T, S> {
        pub fn from_vec(vec: Vec<T>) -> CustomSlice<'a, T, S> {
            CustomSlice::new(
                vec.into_iter()
                    .map(|v| RefCell::new(v))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!("couldn't construct backing slice")),
            )
        }
    }

    impl<'a, T, S> IntoSlice<CustomSlice<'a, T, S>> for CustomSlice<'a, T, S> {
        fn into_slice(self, index: SliceIndex) -> CustomSlice<'a, T, S> {
            let CustomSlice { offset, slice, m } = self;

            CustomSlice {
                offset: index.apply(offset),
                slice,
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsRef<[T]> + 's> Slice<'s, CustomSlice<'s, T, &'s [T]>>
        for CustomSlice<'a, T, S>
    {
        fn slice(&'s self, index: SliceIndex) -> CustomSlice<'s, T, &'s [T]> {
            let CustomSlice {
                offset,
                ref slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_ref(),
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsRef<[RefCell<T>]> + 's> Slice<'s, CustomSlice<'s, T, &'s [RefCell<T>]>>
        for CustomSlice<'a, T, S>
    {
        fn slice(&'s self, index: SliceIndex) -> CustomSlice<'s, T, &'s [RefCell<T>]> {
            let CustomSlice {
                offset,
                ref slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_ref(),
                m,
            }
        }
    }

    impl<'a, 's, T, S: AsMut<[T]> + 's> SliceMut<'s, CustomSlice<'s, T, &'s mut [T]>>
        for CustomSlice<'a, T, S>
    {
        fn slice_mut(&'s mut self, index: SliceIndex) -> CustomSlice<'s, T, &'s mut [T]> {
            let CustomSlice {
                offset,
                ref mut slice,
                m,
            } = *self;

            CustomSlice {
                offset: index.apply(offset),
                slice: slice.as_mut(),
                m,
            }
        }
    }

    impl<'a, 'g, T, S: AsRef<[T]> + 'g> Get<'g, &'g T> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> &'g T {
            &self.slice.as_ref()[index.apply(self.offset)]
        }
    }

    impl<'a, 'g, T, S: AsMut<[T]> + 'g> GetMut<'g, &'g mut T> for CustomSlice<'a, T, S> {
        fn get_mut(&'g mut self, index: SliceIndex) -> &'g mut T {
            &mut self.slice.as_mut()[index.apply(self.offset)]
        }
    }

    impl<'a, 'g, T, S: AsRef<[RefCell<T>]> + 'g> Get<'g, Ref<'g, T>> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> Ref<'g, T> {
            self.slice.as_ref()[index.apply(self.offset)].borrow()
        }
    }

    impl<'a, 'g, T, S: AsRef<[RefCell<T>]> + 'g> Get<'g, RefMut<'g, T>> for CustomSlice<'a, T, S> {
        fn get(&'g self, index: SliceIndex) -> RefMut<'g, T> {
            self.slice.as_ref()[index.apply(self.offset)].borrow_mut()
        }
    }

    pub trait GetPtr<T: ?Sized> {
        fn get_ptr(&self) -> *const ();
    }

    impl<T: ?Sized> GetPtr<T> for &T {
        fn get_ptr(&self) -> *const () {
            *self as *const T as *const ()
        }
    }

    impl<T: ?Sized> GetPtr<T> for &mut T {
        fn get_ptr(&self) -> *const () {
            *self as *const T as *const ()
        }
    }

    impl<T> GetPtr<T> for &[T] {
        fn get_ptr(&self) -> *const () {
            self.as_ptr() as *const ()
        }
    }

    impl<T> GetPtr<T> for &mut [T] {
        fn get_ptr(&self) -> *const () {
            self.as_ptr() as *const ()
        }
    }

    impl<T> GetPtr<T> for Box<[T]> {
        fn get_ptr(&self) -> *const () {
            (*self).as_ptr() as *const ()
        }
    }

    impl<T: ?Sized> GetPtr<T> for Box<T> {
        fn get_ptr(&self) -> *const () {
            self.as_ref() as *const T as *const ()
        }
    }

    pub fn offset_from<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> isize {
        if l.is_none() && r.is_none() {
            return 0; // case for NULL - NULL
        }

        l.as_ref()
            .and_then(|l| {
                r.as_ref().map(|r| {
                    if l.slice.get_ptr() == r.slice.get_ptr() {
                        l.offset as isize - r.offset as isize
                    } else {
                        panic!("trying to compute offsets from two unrelated slices");
                    }
                })
            })
            .expect("tried to compare non-null and null pointers")
    }

    pub fn partial_cmp<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<Ordering> {
        l.as_ref().and_then(|l| {
            r.as_ref().and_then(|r| {
                if l.slice.get_ptr() == r.slice.get_ptr() {
                    Some(l.offset.cmp(&r.offset))
                } else {
                    None
                }
            })
        })
    }

    pub fn partial_lt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c == Ordering::Less)
    }
    pub fn partial_gt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c == Ordering::Greater)
    }
    pub fn partial_le<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c != Ordering::Greater)
    }
    pub fn partial_ge<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> Option<bool> {
        partial_cmp(l, r).map(|c| c != Ordering::Less)
    }

    pub fn compare_eq<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_cmp(l, r).map_or(false, |c| c == Ordering::Equal)
    }
    pub fn compare_ne<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        !compare_eq(l, r)
    }

    pub fn compare_lt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_lt(l, r).unwrap()
    }
    pub fn compare_gt<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_gt(l, r).unwrap()
    }
    pub fn compare_le<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_le(l, r).unwrap()
    }
    pub fn compare_ge<'a, T, L: GetPtr<T>, R: GetPtr<T>>(
        l: &Option<CustomSlice<'a, T, L>>,
        r: &Option<CustomSlice<'a, T, R>>,
    ) -> bool {
        partial_ge(l, r).unwrap()
    }

    pub fn borrow<'a, 'b: 'a, S: Slice<'a, T>, T: 'a>(s: &'b Option<S>) -> Option<T> {
        s.as_ref().map(|s| s.slice_add(0))
    }
    pub fn borrow_mut<'a, 'b: 'a, S: SliceMut<'a, T>, T: 'a>(s: &'b mut Option<S>) -> Option<T> {
        s.as_mut().map(|s| s.slice_add_mut(0))
    }
}

#[allow(dead_code)]
pub(crate) mod laertes_rt {
    use super::__laertes_array::GetPtr;

    pub trait Borrow<'b, T: ?Sized> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a;
    }

    pub trait BorrowMut<'b, T: ?Sized> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a;
    }

    impl<'b, T: ?Sized> Borrow<'b, T> for Option<&'b mut T> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a,
        {
            self.as_ref().map(|x| &**x)
        }
    }

    impl<'b, T: ?Sized> BorrowMut<'b, T> for Option<&'b mut T> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a,
        {
            self.as_mut().map(|x| &mut **x)
        }
    }

    impl<'b, T: ?Sized> Borrow<'b, T> for Option<Box<T>> {
        fn borrow<'a>(&'a self) -> Option<&'a T>
        where
            'b: 'a,
        {
            self.as_ref().map(|x| x.as_ref())
        }
    }

    impl<'b, T: ?Sized> BorrowMut<'b, T> for Option<Box<T>> {
        fn borrow_mut<'a>(&'a mut self) -> Option<&'a mut T>
        where
            'b: 'a,
        {
            self.as_mut().map(|x| x.as_mut())
        }
    }

    pub fn borrow<'a, 'b: 'a, T: ?Sized, P: Borrow<'b, T>>(p: &'a P) -> Option<&'a T> {
        Borrow::borrow(p)
    }

    pub fn borrow_mut<'a, 'b: 'a, T: ?Sized, P: BorrowMut<'b, T>>(
        p: &'a mut P,
    ) -> Option<&'a mut T> {
        BorrowMut::borrow_mut(p)
    }

    pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
        p.as_ref().map(|x| x.as_ref())
    }

    pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
        p.as_mut().map(|x| x.as_mut())
    }

    pub fn option_to_raw<T: ?Sized, P: GetPtr<T>>(p: &Option<P>) -> *const () {
        p.as_ref()
            .map_or(core::ptr::null(), |p| p.get_ptr() as *const ())
    }

    pub fn _ref_eq<T: ?Sized, L: GetPtr<T>, R: GetPtr<T>>(p: Option<L>, q: Option<R>) -> bool {
        option_to_raw(&p) == option_to_raw(&q)
    }

    pub fn _ref_ne<T: ?Sized, L: GetPtr<T>, R: GetPtr<T>>(p: Option<L>, q: Option<R>) -> bool {
        option_to_raw(&p) != option_to_raw(&q)
    }

    // functions to convert references to raw pointers
    pub fn _as_ptr<T, P: GetPtr<T>>(r: &Option<P>) -> *const T {
        option_to_raw(r) as *const T
    }

    pub fn _as_mut_ptr<T, P: GetPtr<T>>(r: &mut Option<P>) -> *mut T {
        option_to_raw(r) as *const T as *mut T
    }

    pub fn _move_to_ptr<T>(b: Option<Box<T>>) -> *mut T {
        b.map_or(core::ptr::null_mut(), Box::into_raw)
    }

    pub fn trans<'a, 'b, T: 'a, U: 'b>(_: T) -> U
    where
        'a: 'b,
    {
        loop {}
    }
}



#[no_mangle] pub extern "C" fn __assert_rtn(_f: *const std::os::raw::c_char, _fl: *const std::os::raw::c_char, _l: std::os::raw::c_int, _e: *const std::os::raw::c_char) -> ! { std::process::abort() }
#[no_mangle] pub static mut __stderrp: *mut std::os::raw::c_void = std::ptr::null_mut();
#[no_mangle] pub extern "C" fn __maskrune(_c: std::os::raw::c_int, _f: std::os::raw::c_ulong) -> std::os::raw::c_int { 0 }
// E3 harness shim for the macOS-transpiled ctype path: `_DefaultRuneLocale` with a POPULATED
// `__runetype` (the 2026-08-25 lil pilot: a zeroed table makes isspace/ispunct/isdigit always false
// -> every script returns [] = 313 false divergences; fill ported from
// results/pilots/attr/lil/raw/rune_fill.rs). Layout = the leading fields of lil::_RuneLocale
// (magic[8], encoding[32], 2 fn ptrs, invalid_rune, __runetype[256], __maplower[256],
// __mapupper[256]); the zero tail keeps the object as large as the original 64 KiB shim.
#[repr(C)]
pub struct E3RuneLocale {
    pub __magic: [u8; 8],
    pub __encoding: [u8; 32],
    pub __sgetrune: usize,
    pub __sputrune: usize,
    pub __invalid_rune: i32,
    pub __runetype: [u32; 256],
    pub __maplower: [i32; 256],
    pub __mapupper: [i32; 256],
    pub __tail: [u8; 65536 - 60 - 3 * 1024],
}
impl E3RuneLocale {
    pub const fn filled() -> Self {
        let mut t = [0u32; 256];
        let mut lo = [0i32; 256];
        let mut up = [0i32; 256];
        let mut i = 0usize;
        while i < 256 {
            let c = i as u8;
            let mut f = 0u32;
            if c.is_ascii_alphabetic() { f |= 0x100; }
            if c.is_ascii_control() { f |= 0x200; }
            if c.is_ascii_digit() { f |= 0x400; }
            if c.is_ascii_graphic() { f |= 0x800; }
            if c.is_ascii_lowercase() { f |= 0x1000; }
            if c.is_ascii_punctuation() { f |= 0x2000; }
            if c.is_ascii_whitespace() || c == 0x0b { f |= 0x4000; }
            if c.is_ascii_uppercase() { f |= 0x8000; }
            if c.is_ascii_hexdigit() { f |= 0x10000; }
            if c == b' ' || c == b'\t' { f |= 0x20000; }
            if c.is_ascii_graphic() || c == b' ' { f |= 0x40000; }
            t[i] = f;
            lo[i] = c.to_ascii_lowercase() as i32;
            up[i] = c.to_ascii_uppercase() as i32;
            i += 1;
        }
        E3RuneLocale { __magic: *b"RuneMagA", __encoding: [0; 32], __sgetrune: 0, __sputrune: 0,
                       __invalid_rune: 0xFFFD, __runetype: t, __maplower: lo, __mapupper: up,
                       __tail: [0; 65536 - 60 - 3 * 1024] }
    }
}
#[no_mangle] pub static _DefaultRuneLocale: E3RuneLocale = E3RuneLocale::filled();

use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn atoll(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_longlong;
    fn rand() -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn fmod(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type lilint_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_value_t {
    pub l: size_t,
    pub d: *mut ::core::ffi::c_char,
}
pub type lil_value_t = *mut _lil_value_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_func_t {
    pub name: *mut ::core::ffi::c_char,
    pub code: lil_value_t,
    pub argnames: lil_list_t,
    pub proc_0: lil_func_proc_t,
}
pub type lil_func_proc_t = Option<
    unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
>;
pub type lil_t = *mut _lil_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_t {
    pub code: *const ::core::ffi::c_char,
    pub rootcode: *const ::core::ffi::c_char,
    pub clen: size_t,
    pub head: size_t,
    pub ignoreeol: ::core::ffi::c_int,
    pub cmd: *mut lil_func_t,
    pub cmds: size_t,
    pub syscmds: size_t,
    pub catcher: *mut ::core::ffi::c_char,
    pub in_catcher: ::core::ffi::c_int,
    pub dollarprefix: *mut ::core::ffi::c_char,
    pub env: lil_env_t,
    pub rootenv: lil_env_t,
    pub downenv: lil_env_t,
    pub empty: lil_value_t,
    pub error: ::core::ffi::c_int,
    pub err_head: size_t,
    pub err_msg: *mut ::core::ffi::c_char,
    pub callback: [lil_callback_proc_t; 8],
    pub parse_depth: size_t,
    pub data: *mut ::core::ffi::c_void,
}
pub type lil_callback_proc_t = Option<unsafe extern "C" fn() -> ()>;
pub type lil_env_t = *mut _lil_env_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_env_t {
    pub parent: *mut _lil_env_t,
    pub func: lil_func_t,
    pub catcher_for: lil_value_t,
    pub var: *mut lil_var_t,
    pub vars: size_t,
    pub retval: lil_value_t,
    pub retval_set: ::core::ffi::c_int,
    pub breakrun: ::core::ffi::c_int,
}
pub type lil_var_t = *mut _lil_var_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_var_t {
    pub n: *mut ::core::ffi::c_char,
    pub env: *mut _lil_env_t,
    pub v: lil_value_t,
}
pub type lil_func_t = *mut _lil_func_t;
pub type lil_list_t = *mut _lil_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_list_t {
    pub v: *mut lil_value_t,
    pub c: size_t,
}
pub type lil_exit_callback_proc_t = Option<
    unsafe extern "C" fn(lil_t, lil_value_t) -> (),
>;
pub type lil_write_callback_proc_t = Option<
    unsafe extern "C" fn(lil_t, *const ::core::ffi::c_char) -> (),
>;
pub type lil_read_callback_proc_t = Option<
    unsafe extern "C" fn(lil_t, *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
>;
pub type lil_source_callback_proc_t = Option<
    unsafe extern "C" fn(lil_t, *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
>;
pub type lil_store_callback_proc_t = Option<
    unsafe extern "C" fn(
        lil_t,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> (),
>;
pub type lil_error_callback_proc_t = Option<
    unsafe extern "C" fn(lil_t, size_t, *const ::core::ffi::c_char) -> (),
>;
pub type lil_setvar_callback_proc_t = Option<
    unsafe extern "C" fn(
        lil_t,
        *const ::core::ffi::c_char,
        *mut lil_value_t,
    ) -> ::core::ffi::c_int,
>;
pub type lil_getvar_callback_proc_t = Option<
    unsafe extern "C" fn(
        lil_t,
        *const ::core::ffi::c_char,
        *mut lil_value_t,
    ) -> ::core::ffi::c_int,
>;
pub type expreval_t = _expreval_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _expreval_t {
    pub code: *const ::core::ffi::c_char,
    pub len: size_t,
    pub head: size_t,
    pub ival: lilint_t,
    pub dval: ::core::ffi::c_double,
    pub type_0: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
}
pub const RAND_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LIL_VERSION_STRING: [::core::ffi::c_char; 4] = unsafe {
    ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"0.1\0")
};
pub const LIL_SETVAR_GLOBAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIL_SETVAR_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LIL_SETVAR_LOCAL_NEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LIL_SETVAR_LOCAL_ONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LIL_CALLBACK_EXIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIL_CALLBACK_WRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LIL_CALLBACK_READ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LIL_CALLBACK_STORE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LIL_CALLBACK_SOURCE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LIL_CALLBACK_ERROR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LIL_CALLBACK_SETVAR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LIL_CALLBACK_GETVAR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const ERROR_NOERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERROR_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ERROR_FIXHEAD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CALLBACKS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAX_CATCHER_DEPTH: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
unsafe extern "C" fn strclone(
    mut s: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut len: size_t = strlen(s).wrapping_add(1 as size_t);
    let mut ns: *mut ::core::ffi::c_char = malloc(len) as *mut ::core::ffi::c_char;
    if ns.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(ns as *mut ::core::ffi::c_void, s as *const ::core::ffi::c_void, len);
    return ns;
}
unsafe extern "C" fn alloc_value(mut str: *const ::core::ffi::c_char) -> lil_value_t {
    let mut val: lil_value_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<_lil_value_t>() as size_t,
    ) as lil_value_t;
    if val.is_null() {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if !str.is_null() {
        (*val).l = strlen(str);
        (*val).d = malloc((*val).l.wrapping_add(1 as size_t))
            as *mut ::core::ffi::c_char;
        if (*val).d.is_null() {
            free(val as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        memcpy(
            (*val).d as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            (*val).l.wrapping_add(1 as size_t),
        );
    } else {
        (*val).l = 0 as size_t;
        (*val).d = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_clone_value(mut src: lil_value_t) -> lil_value_t {
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if src.is_null() {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    val = calloc(1 as size_t, ::core::mem::size_of::<_lil_value_t>() as size_t)
        as lil_value_t;
    if val.is_null() {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    (*val).l = (*src).l;
    if (*src).l != 0 {
        (*val).d = malloc((*val).l.wrapping_add(1 as size_t))
            as *mut ::core::ffi::c_char;
        if (*val).d.is_null() {
            free(val as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        memcpy(
            (*val).d as *mut ::core::ffi::c_void,
            (*src).d as *const ::core::ffi::c_void,
            (*val).l.wrapping_add(1 as size_t),
        );
    } else {
        (*val).d = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_char(
    mut val: lil_value_t,
    mut ch: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut new: *mut ::core::ffi::c_char = realloc(
        (*val).d as *mut ::core::ffi::c_void,
        (*val).l.wrapping_add(2 as size_t),
    ) as *mut ::core::ffi::c_char;
    if new.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let fresh12 = (*val).l;
    (*val).l = (*val).l.wrapping_add(1);
    *new.offset(fresh12 as isize) = ch;
    *new.offset((*val).l as isize) = 0 as ::core::ffi::c_char;
    (*val).d = new;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_string(
    mut val: lil_value_t,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut new: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut len: size_t = 0;
    if s.is_null() || *s.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return 1 as ::core::ffi::c_int;
    }
    len = strlen(s);
    new = realloc(
        (*val).d as *mut ::core::ffi::c_void,
        (*val).l.wrapping_add(len).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if new.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        new.offset((*val).l as isize) as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    (*val).l = (*val).l.wrapping_add(len);
    (*val).d = new;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_val(
    mut val: lil_value_t,
    mut v: lil_value_t,
) -> ::core::ffi::c_int {
    let mut new: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if v.is_null() || (*v).l == 0 {
        return 1 as ::core::ffi::c_int;
    }
    new = realloc(
        (*val).d as *mut ::core::ffi::c_void,
        (*val).l.wrapping_add((*v).l).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if new.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        new.offset((*val).l as isize) as *mut ::core::ffi::c_void,
        (*v).d as *const ::core::ffi::c_void,
        (*v).l.wrapping_add(1 as size_t),
    );
    (*val).l = (*val).l.wrapping_add((*v).l);
    (*val).d = new;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_value(mut val: lil_value_t) {
    if val.is_null() {
        return;
    }
    free((*val).d as *mut ::core::ffi::c_void);
    free(val as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_list() -> lil_list_t {
    let mut list: lil_list_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<_lil_list_t>() as size_t,
    ) as lil_list_t;
    (*list).v = ::core::ptr::null_mut::<lil_value_t>();
    (*list).c = 0 as size_t;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_list(mut list: lil_list_t) {
    let mut i: size_t = 0;
    if list.is_null() {
        return;
    }
    i = 0 as size_t;
    while i < (*list).c {
        lil_free_value(*(*list).v.offset(i as isize));
        i = i.wrapping_add(1);
    }
    free((*list).v as *mut ::core::ffi::c_void);
    free(list as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_append(mut list: lil_list_t, mut val: lil_value_t) {
    let mut nv: *mut lil_value_t = realloc(
        (*list).v as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<lil_value_t>() as size_t)
            .wrapping_mul((*list).c.wrapping_add(1 as size_t)),
    ) as *mut lil_value_t;
    if nv.is_null() {
        return;
    }
    (*list).v = nv;
    let fresh7 = (*list).c;
    (*list).c = (*list).c.wrapping_add(1);
    let ref mut fresh8 = *nv.offset(fresh7 as isize);
    *fresh8 = val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_size(mut list: lil_list_t) -> size_t {
    return (*list).c;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_get(
    mut list: lil_list_t,
    mut index: size_t,
) -> lil_value_t {
    return if index >= (*list).c {
        ::core::ptr::null_mut::<_lil_value_t>()
    } else {
        *(*list).v.offset(index as isize)
    };
}
unsafe extern "C" fn needs_escape(
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    if str.is_null() || *str.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while *str.offset(i as isize) != 0 {
        if *(*__ctype_b_loc())
            .offset(*str.offset(i as isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
            || *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_to_value(
    mut list: lil_list_t,
    mut do_escape: ::core::ffi::c_int,
) -> lil_value_t {
    let mut val: lil_value_t = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*list).c {
        let mut escape: ::core::ffi::c_int = if do_escape != 0 {
            needs_escape(lil_to_string(*(*list).v.offset(i as isize)))
        } else {
            0 as ::core::ffi::c_int
        };
        if i != 0 {
            lil_append_char(val, ' ' as i32 as ::core::ffi::c_char);
        }
        if escape != 0 {
            lil_append_char(val, '{' as i32 as ::core::ffi::c_char);
        }
        lil_append_val(val, *(*list).v.offset(i as isize));
        if escape != 0 {
            lil_append_char(val, '}' as i32 as ::core::ffi::c_char);
        }
        i = i.wrapping_add(1);
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_env(mut parent: lil_env_t) -> lil_env_t {
    let mut env: lil_env_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<_lil_env_t>() as size_t,
    ) as lil_env_t;
    (*env).parent = parent as *mut _lil_env_t;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_env(mut env: lil_env_t) {
    let mut i: size_t = 0;
    if env.is_null() {
        return;
    }
    lil_free_value((*env).retval);
    i = 0 as size_t;
    while i < (*env).vars {
        free((**(*env).var.offset(i as isize)).n as *mut ::core::ffi::c_void);
        lil_free_value((**(*env).var.offset(i as isize)).v);
        free(*(*env).var.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free((*env).var as *mut ::core::ffi::c_void);
    free(env as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn lil_find_local_var(
    mut lil: lil_t,
    mut env: lil_env_t,
    mut name: *const ::core::ffi::c_char,
) -> lil_var_t {
    if (*env).vars > 0 as size_t {
        let mut i: size_t = (*env).vars.wrapping_sub(1 as size_t);
        loop {
            if strcmp((**(*env).var.offset(i as isize)).n, name) == 0 {
                return *(*env).var.offset(i as isize);
            }
            if i == 0 {
                break;
            }
            i = i.wrapping_sub(1);
        }
    }
    return ::core::ptr::null_mut::<_lil_var_t>();
}
unsafe extern "C" fn lil_find_var(
    mut lil: lil_t,
    mut env: lil_env_t,
    mut name: *const ::core::ffi::c_char,
) -> lil_var_t {
    let mut r: lil_var_t = lil_find_local_var(lil, env, name);
    return if !r.is_null() {
        r
    } else if env == (*lil).rootenv {
        ::core::ptr::null_mut::<_lil_var_t>()
    } else {
        lil_find_var(lil, (*lil).rootenv, name)
    };
}
unsafe extern "C" fn find_cmd(
    mut lil: lil_t,
    mut name: *const ::core::ffi::c_char,
) -> lil_func_t {
    if (*lil).cmds > 0 as size_t {
        let mut i: size_t = (*lil).cmds.wrapping_sub(1 as size_t);
        loop {
            if strcmp((**(*lil).cmd.offset(i as isize)).name, name) == 0 {
                return *(*lil).cmd.offset(i as isize);
            }
            if i == 0 {
                break;
            }
            i = i.wrapping_sub(1);
        }
    }
    return ::core::ptr::null_mut::<_lil_func_t>();
}
unsafe extern "C" fn add_func(
    mut lil: lil_t,
    mut name: *const ::core::ffi::c_char,
) -> lil_func_t {
    let mut cmd: lil_func_t = ::core::ptr::null_mut::<_lil_func_t>();
    let mut ncmd: *mut lil_func_t = ::core::ptr::null_mut::<lil_func_t>();
    cmd = find_cmd(lil, name);
    if !cmd.is_null() {
        return cmd;
    }
    cmd = calloc(1 as size_t, ::core::mem::size_of::<_lil_func_t>() as size_t)
        as lil_func_t;
    (*cmd).name = strclone(name);
    ncmd = realloc(
        (*lil).cmd as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<lil_func_t>() as size_t)
            .wrapping_mul((*lil).cmds.wrapping_add(1 as size_t)),
    ) as *mut lil_func_t;
    if ncmd.is_null() {
        free(cmd as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<_lil_func_t>();
    }
    (*lil).cmd = ncmd;
    let fresh0 = (*lil).cmds;
    (*lil).cmds = (*lil).cmds.wrapping_add(1);
    let ref mut fresh1 = *ncmd.offset(fresh0 as isize);
    *fresh1 = cmd;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn lil_register(
    mut lil: lil_t,
    mut name: *const ::core::ffi::c_char,
    mut proc_0: lil_func_proc_t,
) -> ::core::ffi::c_int {
    let mut cmd: lil_func_t = add_func(lil, name);
    if cmd.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*cmd).proc_0 = proc_0;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_var(
    mut lil: lil_t,
    mut name: *const ::core::ffi::c_char,
    mut val: lil_value_t,
    mut local: ::core::ffi::c_int,
) -> lil_var_t {
    let mut nvar: *mut lil_var_t = ::core::ptr::null_mut::<lil_var_t>();
    let mut env: lil_env_t = if local == LIL_SETVAR_GLOBAL {
        (*lil).rootenv
    } else {
        (*lil).env
    };
    let mut freeval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *name.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return ::core::ptr::null_mut::<_lil_var_t>();
    }
    if local != LIL_SETVAR_LOCAL_NEW {
        let mut var: lil_var_t = lil_find_var(lil, env, name);
        if local == LIL_SETVAR_LOCAL_ONLY && !var.is_null()
            && (*var).env == (*lil).rootenv && (*var).env != env
        {
            var = ::core::ptr::null_mut::<_lil_var_t>();
        }
        if (var.is_null() && env == (*lil).rootenv
            || !var.is_null() && (*var).env == (*lil).rootenv)
            && (*lil).callback[LIL_CALLBACK_SETVAR as usize].is_some()
        {
            let mut proc_0: lil_setvar_callback_proc_t = ::core::mem::transmute::<
                lil_callback_proc_t,
                lil_setvar_callback_proc_t,
            >((*lil).callback[LIL_CALLBACK_SETVAR as usize]);
            let mut newval: lil_value_t = val;
            let mut r: ::core::ffi::c_int = proc_0
                .expect("non-null function pointer")(lil, name, &raw mut newval);
            if r < 0 as ::core::ffi::c_int {
                return ::core::ptr::null_mut::<_lil_var_t>();
            }
            if r != 0 {
                val = newval;
                freeval = 1 as ::core::ffi::c_int;
            }
        }
        if !var.is_null() {
            lil_free_value((*var).v);
            (*var).v = if freeval != 0 { val } else { lil_clone_value(val) };
            return var;
        }
    }
    nvar = realloc(
        (*env).var as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<lil_var_t>() as size_t)
            .wrapping_mul((*env).vars.wrapping_add(1 as size_t)),
    ) as *mut lil_var_t;
    if nvar.is_null() {
        return ::core::ptr::null_mut::<_lil_var_t>();
    }
    (*env).var = nvar;
    let ref mut fresh2 = *nvar.offset((*env).vars as isize);
    *fresh2 = calloc(1 as size_t, ::core::mem::size_of::<_lil_var_t>() as size_t)
        as lil_var_t;
    let ref mut fresh3 = (**nvar.offset((*env).vars as isize)).n;
    *fresh3 = strclone(name);
    let ref mut fresh4 = (**nvar.offset((*env).vars as isize)).env;
    *fresh4 = env as *mut _lil_env_t;
    let ref mut fresh5 = (**nvar.offset((*env).vars as isize)).v;
    *fresh5 = if freeval != 0 { val } else { lil_clone_value(val) };
    let fresh6 = (*env).vars;
    (*env).vars = (*env).vars.wrapping_add(1);
    return *nvar.offset(fresh6 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_var(
    mut lil: lil_t,
    mut name: *const ::core::ffi::c_char,
) -> lil_value_t {
    return lil_get_var_or(lil, name, (*lil).empty);
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_var_or(
    mut lil: lil_t,
    mut name: *const ::core::ffi::c_char,
    mut defvalue: lil_value_t,
) -> lil_value_t {
    let mut var: lil_var_t = lil_find_var(lil, (*lil).env, name);
    let mut retval: lil_value_t = if !var.is_null() { (*var).v } else { defvalue };
    if (*lil).callback[LIL_CALLBACK_GETVAR as usize].is_some()
        && (var.is_null() || (*var).env == (*lil).rootenv)
    {
        let mut proc_0: lil_getvar_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_getvar_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_GETVAR as usize]);
        let mut newretval: lil_value_t = retval;
        if proc_0.expect("non-null function pointer")(lil, name, &raw mut newretval) != 0
        {
            retval = newretval;
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn lil_push_env(mut lil: lil_t) -> lil_env_t {
    let mut env: lil_env_t = lil_alloc_env((*lil).env);
    (*lil).env = env;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn lil_pop_env(mut lil: lil_t) {
    if !(*(*lil).env).parent.is_null() {
        let mut next: lil_env_t = (*(*lil).env).parent as lil_env_t;
        lil_free_env((*lil).env);
        (*lil).env = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lil_new() -> lil_t {
    let mut lil: lil_t = calloc(1 as size_t, ::core::mem::size_of::<_lil_t>() as size_t)
        as lil_t;
    (*lil).env = lil_alloc_env(::core::ptr::null_mut::<_lil_env_t>());
    (*lil).rootenv = (*lil).env;
    (*lil).empty = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
    (*lil).dollarprefix = strclone(b"set \0" as *const u8 as *const ::core::ffi::c_char);
    register_stdcmds(lil);
    return lil;
}
unsafe extern "C" fn islilspecial(mut ch: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (ch as ::core::ffi::c_int == ';' as i32
        || ch as ::core::ffi::c_int == '$' as i32
        || ch as ::core::ffi::c_int == '[' as i32
        || ch as ::core::ffi::c_int == ']' as i32
        || ch as ::core::ffi::c_int == '{' as i32
        || ch as ::core::ffi::c_int == '}' as i32
        || ch as ::core::ffi::c_int == '"' as i32
        || ch as ::core::ffi::c_int == '\'' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn ateol(mut lil: lil_t) -> ::core::ffi::c_int {
    return ((*lil).ignoreeol == 0
        && (*(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
            == '\n' as i32
            || *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == '\r' as i32
            || *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == ';' as i32)) as ::core::ffi::c_int;
}
unsafe extern "C" fn skip_spaces(mut lil: lil_t) {
    while (*lil).head < (*lil).clen
        && (*(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
            == '\\' as i32
            || *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == '#' as i32
            || *(*__ctype_b_loc())
                .offset(
                    *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                        as isize,
                ) as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
                && ((*lil).ignoreeol != 0
                    || !(*(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                        == '\r' as i32
                        || *(*lil).code.offset((*lil).head as isize)
                            as ::core::ffi::c_int == '\n' as i32)))
    {
        if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int == '#' as i32
        {
            while (*lil).head < (*lil).clen && ateol(lil) == 0 {
                (*lil).head = (*lil).head.wrapping_add(1);
            }
        } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
            == '\\' as i32
            && (*(*lil).code.offset((*lil).head.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int == '\r' as i32
                || *(*lil).code.offset((*lil).head.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_int == '\n' as i32)
        {
            (*lil).head = (*lil).head.wrapping_add(1);
            while (*lil).head < (*lil).clen && ateol(lil) != 0 {
                (*lil).head = (*lil).head.wrapping_add(1);
            }
        } else {
            (*lil).head = (*lil).head.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn get_bracketpart(mut lil: lil_t) -> lil_value_t {
    let mut cnt: size_t = 1 as size_t;
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut cmd: lil_value_t = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
    (*lil).head = (*lil).head.wrapping_add(1);
    while (*lil).head < (*lil).clen {
        if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int == '[' as i32
        {
            (*lil).head = (*lil).head.wrapping_add(1);
            cnt = cnt.wrapping_add(1);
            lil_append_char(cmd, '[' as i32 as ::core::ffi::c_char);
        } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
            == ']' as i32
        {
            (*lil).head = (*lil).head.wrapping_add(1);
            cnt = cnt.wrapping_sub(1);
            if cnt == 0 as size_t {
                break;
            }
            lil_append_char(cmd, ']' as i32 as ::core::ffi::c_char);
        } else {
            let fresh13 = (*lil).head;
            (*lil).head = (*lil).head.wrapping_add(1);
            lil_append_char(cmd, *(*lil).code.offset(fresh13 as isize));
        }
    }
    val = lil_parse_value(lil, cmd, 0 as ::core::ffi::c_int);
    lil_free_value(cmd);
    return val;
}
unsafe extern "C" fn get_dollarpart(mut lil: lil_t) -> lil_value_t {
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut name: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut tmp: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    (*lil).head = (*lil).head.wrapping_add(1);
    name = next_word(lil);
    tmp = alloc_value((*lil).dollarprefix);
    lil_append_val(tmp, name);
    lil_free_value(name);
    val = lil_parse_value(lil, tmp, 0 as ::core::ffi::c_int);
    lil_free_value(tmp);
    return val;
}
unsafe extern "C" fn next_word(mut lil: lil_t) -> lil_value_t {
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    skip_spaces(lil);
    if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int == '$' as i32 {
        val = get_dollarpart(lil);
    } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
        == '{' as i32
    {
        let mut cnt: size_t = 1 as size_t;
        (*lil).head = (*lil).head.wrapping_add(1);
        val = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
        while (*lil).head < (*lil).clen {
            if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == '{' as i32
            {
                (*lil).head = (*lil).head.wrapping_add(1);
                cnt = cnt.wrapping_add(1);
                lil_append_char(val, '{' as i32 as ::core::ffi::c_char);
            } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == '}' as i32
            {
                (*lil).head = (*lil).head.wrapping_add(1);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 as size_t {
                    break;
                }
                lil_append_char(val, '}' as i32 as ::core::ffi::c_char);
            } else {
                let fresh9 = (*lil).head;
                (*lil).head = (*lil).head.wrapping_add(1);
                lil_append_char(val, *(*lil).code.offset(fresh9 as isize));
            }
        }
    } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
        == '[' as i32
    {
        val = get_bracketpart(lil);
    } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
        == '"' as i32
        || *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int == '\'' as i32
    {
        let fresh10 = (*lil).head;
        (*lil).head = (*lil).head.wrapping_add(1);
        let mut sc: ::core::ffi::c_char = *(*lil).code.offset(fresh10 as isize);
        val = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
        while (*lil).head < (*lil).clen {
            if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == '[' as i32
                || *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                    == '$' as i32
            {
                let mut tmp: lil_value_t = if *(*lil).code.offset((*lil).head as isize)
                    as ::core::ffi::c_int == '$' as i32
                {
                    get_dollarpart(lil)
                } else {
                    get_bracketpart(lil)
                };
                lil_append_val(val, tmp);
                lil_free_value(tmp);
                (*lil).head = (*lil).head.wrapping_sub(1);
            } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == '\\' as i32
            {
                (*lil).head = (*lil).head.wrapping_add(1);
                match *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int {
                    98 => {
                        lil_append_char(val, '\u{8}' as i32 as ::core::ffi::c_char);
                    }
                    116 => {
                        lil_append_char(val, '\t' as i32 as ::core::ffi::c_char);
                    }
                    110 => {
                        lil_append_char(val, '\n' as i32 as ::core::ffi::c_char);
                    }
                    118 => {
                        lil_append_char(val, '\u{b}' as i32 as ::core::ffi::c_char);
                    }
                    102 => {
                        lil_append_char(val, '\u{c}' as i32 as ::core::ffi::c_char);
                    }
                    114 => {
                        lil_append_char(val, '\r' as i32 as ::core::ffi::c_char);
                    }
                    48 => {
                        lil_append_char(val, 0 as ::core::ffi::c_char);
                    }
                    97 => {
                        lil_append_char(val, '\u{7}' as i32 as ::core::ffi::c_char);
                    }
                    99 => {
                        lil_append_char(val, '}' as i32 as ::core::ffi::c_char);
                    }
                    111 => {
                        lil_append_char(val, '{' as i32 as ::core::ffi::c_char);
                    }
                    _ => {
                        lil_append_char(val, *(*lil).code.offset((*lil).head as isize));
                    }
                }
            } else if *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                == sc as ::core::ffi::c_int
            {
                (*lil).head = (*lil).head.wrapping_add(1);
                break;
            } else {
                lil_append_char(val, *(*lil).code.offset((*lil).head as isize));
            }
            (*lil).head = (*lil).head.wrapping_add(1);
        }
    } else {
        val = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
        while (*lil).head < (*lil).clen
            && *(*__ctype_b_loc())
                .offset(
                    *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                        as isize,
                ) as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0
            && islilspecial(*(*lil).code.offset((*lil).head as isize)) == 0
        {
            let fresh11 = (*lil).head;
            (*lil).head = (*lil).head.wrapping_add(1);
            lil_append_char(val, *(*lil).code.offset(fresh11 as isize));
        }
    }
    return if !val.is_null() {
        val
    } else {
        alloc_value(::core::ptr::null::<::core::ffi::c_char>())
    };
}
unsafe extern "C" fn substitute(mut lil: lil_t) -> lil_list_t {
    let mut words: lil_list_t = lil_alloc_list();
    skip_spaces(lil);
    while (*lil).head < (*lil).clen && ateol(lil) == 0 && (*lil).error == 0 {
        let mut w: lil_value_t = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
        loop {
            let mut head: size_t = (*lil).head;
            let mut wp: lil_value_t = next_word(lil);
            if head == (*lil).head {
                lil_free_value(w);
                lil_free_value(wp);
                lil_free_list(words);
                return ::core::ptr::null_mut::<_lil_list_t>();
            }
            lil_append_val(w, wp);
            lil_free_value(wp);
            if !((*lil).head < (*lil).clen && ateol(lil) == 0
                && *(*__ctype_b_loc())
                    .offset(
                        *(*lil).code.offset((*lil).head as isize) as ::core::ffi::c_int
                            as isize,
                    ) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int == 0 && (*lil).error == 0)
            {
                break;
            }
        }
        skip_spaces(lil);
        lil_list_append(words, w);
    }
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn lil_subst_to_list(
    mut lil: lil_t,
    mut code: lil_value_t,
) -> lil_list_t {
    let mut save_code: *const ::core::ffi::c_char = (*lil).code;
    let mut save_clen: size_t = (*lil).clen;
    let mut save_head: size_t = (*lil).head;
    let mut save_igeol: ::core::ffi::c_int = (*lil).ignoreeol;
    let mut words: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    (*lil).code = lil_to_string(code);
    (*lil).clen = (*code).l;
    (*lil).head = 0 as size_t;
    (*lil).ignoreeol = 1 as ::core::ffi::c_int;
    words = substitute(lil);
    (*lil).code = save_code;
    (*lil).clen = save_clen;
    (*lil).head = save_head;
    (*lil).ignoreeol = save_igeol;
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn lil_subst_to_value(
    mut lil: lil_t,
    mut code: lil_value_t,
) -> lil_value_t {
    let mut words: lil_list_t = lil_subst_to_list(lil, code);
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if words.is_null() {
        return lil_clone_value(code);
    }
    val = lil_list_to_value(words, 0 as ::core::ffi::c_int);
    lil_free_list(words);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_parse(
    mut lil: lil_t,
    mut code: *const ::core::ffi::c_char,
    mut codelen: size_t,
    mut funclevel: ::core::ffi::c_int,
) -> lil_value_t {
    let mut save_code: *const ::core::ffi::c_char = (*lil).code;
    let mut save_clen: size_t = (*lil).clen;
    let mut save_head: size_t = (*lil).head;
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut words: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    if save_code.is_null() {
        (*lil).rootcode = code;
    }
    (*lil).code = code;
    (*lil).clen = if codelen != 0 { codelen } else { strlen(code) };
    (*lil).head = 0 as size_t;
    skip_spaces(lil);
    (*lil).parse_depth = (*lil).parse_depth.wrapping_add(1);
    if (*lil).parse_depth == 1 as size_t {
        (*lil).error = 0 as ::core::ffi::c_int;
    }
    if funclevel != 0 {
        (*(*lil).env).breakrun = 0 as ::core::ffi::c_int;
    }
    while (*lil).head < (*lil).clen && (*lil).error == 0 {
        if !words.is_null() {
            lil_free_list(words);
        }
        if !val.is_null() {
            lil_free_value(val);
        }
        val = ::core::ptr::null_mut::<_lil_value_t>();
        words = substitute(lil);
        if words.is_null() || (*lil).error != 0 {
            break;
        }
        if (*words).c != 0 {
            let mut cmd: lil_func_t = find_cmd(
                lil,
                lil_to_string(*(*words).v.offset(0 as ::core::ffi::c_int as isize)),
            );
            if cmd.is_null() {
                if (**(*words).v.offset(0 as ::core::ffi::c_int as isize)).l != 0 {
                    if !(*lil).catcher.is_null() {
                        if (*lil).in_catcher < MAX_CATCHER_DEPTH {
                            let mut args: lil_value_t = ::core::ptr::null_mut::<
                                _lil_value_t,
                            >();
                            (*lil).in_catcher += 1;
                            lil_push_env(lil);
                            (*(*lil).env).catcher_for = *(*words)
                                .v
                                .offset(0 as ::core::ffi::c_int as isize);
                            args = lil_list_to_value(words, 1 as ::core::ffi::c_int);
                            lil_set_var(
                                lil,
                                b"args\0" as *const u8 as *const ::core::ffi::c_char,
                                args,
                                LIL_SETVAR_LOCAL_NEW,
                            );
                            lil_free_value(args);
                            val = lil_parse(
                                lil,
                                (*lil).catcher,
                                0 as size_t,
                                1 as ::core::ffi::c_int,
                            );
                            lil_pop_env(lil);
                            (*lil).in_catcher -= 1;
                        } else {
                            let mut msg: *mut ::core::ffi::c_char = malloc(
                                (**(*words).v.offset(0 as ::core::ffi::c_int as isize))
                                    .l
                                    .wrapping_add(64 as size_t),
                            ) as *mut ::core::ffi::c_char;
                            sprintf(
                                msg,
                                b"catcher limit reached while trying to call unknown function %s\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                (**(*words).v.offset(0 as ::core::ffi::c_int as isize)).d,
                            );
                            lil_set_error_at(lil, (*lil).head, msg);
                            free(msg as *mut ::core::ffi::c_void);
                            break;
                        }
                    } else {
                        let mut msg_0: *mut ::core::ffi::c_char = malloc(
                            (**(*words).v.offset(0 as ::core::ffi::c_int as isize))
                                .l
                                .wrapping_add(32 as size_t),
                        ) as *mut ::core::ffi::c_char;
                        sprintf(
                            msg_0,
                            b"unknown function %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (**(*words).v.offset(0 as ::core::ffi::c_int as isize)).d,
                        );
                        lil_set_error_at(lil, (*lil).head, msg_0);
                        free(msg_0 as *mut ::core::ffi::c_void);
                        break;
                    }
                }
            }
            if !cmd.is_null() {
                if (*cmd).proc_0.is_some() {
                    let mut shead: size_t = (*lil).head;
                    val = (*cmd)
                        .proc_0
                        .expect(
                            "non-null function pointer",
                        )(
                        lil,
                        (*words).c.wrapping_sub(1 as size_t),
                        (*words).v.offset(1 as ::core::ffi::c_int as isize),
                    );
                    if (*lil).error == ERROR_FIXHEAD {
                        (*lil).error = ERROR_DEFAULT;
                        (*lil).err_head = shead;
                    }
                } else {
                    lil_push_env(lil);
                    (*(*lil).env).func = cmd;
                    if (*(*cmd).argnames).c == 1 as size_t
                        && strcmp(
                            lil_to_string(
                                *(*(*cmd).argnames)
                                    .v
                                    .offset(0 as ::core::ffi::c_int as isize),
                            ),
                            b"args\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0
                    {
                        let mut args_0: lil_value_t = lil_list_to_value(
                            words,
                            1 as ::core::ffi::c_int,
                        );
                        lil_set_var(
                            lil,
                            b"args\0" as *const u8 as *const ::core::ffi::c_char,
                            args_0,
                            LIL_SETVAR_LOCAL_NEW,
                        );
                        lil_free_value(args_0);
                    } else {
                        let mut i: size_t = 0;
                        i = 0 as size_t;
                        while i < (*(*cmd).argnames).c {
                            lil_set_var(
                                lil,
                                lil_to_string(*(*(*cmd).argnames).v.offset(i as isize)),
                                if i < (*words).c.wrapping_sub(1 as size_t) {
                                    *(*words).v.offset(i.wrapping_add(1 as size_t) as isize)
                                } else {
                                    (*lil).empty
                                },
                                LIL_SETVAR_LOCAL_NEW,
                            );
                            i = i.wrapping_add(1);
                        }
                    }
                    val = lil_parse_value(lil, (*cmd).code, 1 as ::core::ffi::c_int);
                    lil_pop_env(lil);
                }
            }
        }
        if (*(*lil).env).breakrun != 0 {
            break;
        }
        skip_spaces(lil);
        while ateol(lil) != 0 {
            (*lil).head = (*lil).head.wrapping_add(1);
        }
        skip_spaces(lil);
    }
    if (*lil).error != 0 && (*lil).callback[LIL_CALLBACK_ERROR as usize].is_some()
        && (*lil).parse_depth == 1 as size_t
    {
        let mut proc_0: lil_error_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_error_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_ERROR as usize]);
        proc_0.expect("non-null function pointer")(lil, (*lil).err_head, (*lil).err_msg);
    }
    if !words.is_null() {
        lil_free_list(words);
    }
    (*lil).code = save_code;
    (*lil).clen = save_clen;
    (*lil).head = save_head;
    if funclevel != 0 && (*(*lil).env).retval_set != 0 {
        if !val.is_null() {
            lil_free_value(val);
        }
        val = (*(*lil).env).retval;
        (*(*lil).env).retval = ::core::ptr::null_mut::<_lil_value_t>();
        (*(*lil).env).retval_set = 0 as ::core::ffi::c_int;
        (*(*lil).env).breakrun = 0 as ::core::ffi::c_int;
    }
    (*lil).parse_depth = (*lil).parse_depth.wrapping_sub(1);
    return if !val.is_null() {
        val
    } else {
        alloc_value(::core::ptr::null::<::core::ffi::c_char>())
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_parse_value(
    mut lil: lil_t,
    mut val: lil_value_t,
    mut funclevel: ::core::ffi::c_int,
) -> lil_value_t {
    if val.is_null() || (*val).d.is_null() || (*val).l == 0 {
        return alloc_value(::core::ptr::null::<::core::ffi::c_char>());
    }
    return lil_parse(lil, (*val).d, (*val).l, funclevel);
}
#[no_mangle]
pub unsafe extern "C" fn lil_callback(
    mut lil: lil_t,
    mut cb: ::core::ffi::c_int,
    mut proc_0: lil_callback_proc_t,
) {
    if cb < 0 as ::core::ffi::c_int || cb > CALLBACKS {
        return;
    }
    (*lil).callback[cb as usize] = proc_0;
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_error(
    mut lil: lil_t,
    mut msg: *const ::core::ffi::c_char,
) {
    if (*lil).error != 0 {
        return;
    }
    free((*lil).err_msg as *mut ::core::ffi::c_void);
    (*lil).error = ERROR_FIXHEAD;
    (*lil).err_head = 0 as size_t;
    (*lil).err_msg = strclone(
        if !msg.is_null() {
            msg
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_error_at(
    mut lil: lil_t,
    mut pos: size_t,
    mut msg: *const ::core::ffi::c_char,
) {
    if (*lil).error != 0 {
        return;
    }
    free((*lil).err_msg as *mut ::core::ffi::c_void);
    (*lil).error = ERROR_DEFAULT;
    (*lil).err_head = pos;
    (*lil).err_msg = strclone(
        if !msg.is_null() {
            msg
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn lil_error(
    mut lil: lil_t,
    mut msg: *mut *const ::core::ffi::c_char,
    mut pos: *mut size_t,
) -> ::core::ffi::c_int {
    if (*lil).error == 0 {
        return 0 as ::core::ffi::c_int;
    }
    *msg = (*lil).err_msg;
    *pos = (*lil).err_head;
    (*lil).error = ERROR_NOERROR;
    return 1 as ::core::ffi::c_int;
}
pub const EE_INT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EE_FLOAT: ::core::ffi::c_int = 1;
pub const EERR_NO_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EERR_SYNTAX_ERROR: ::core::ffi::c_int = 1;
pub const EERR_INVALID_TYPE: ::core::ffi::c_int = 2;
pub const EERR_DIVISION_BY_ZERO: ::core::ffi::c_int = 3;
pub const EERR_INVALID_EXPRESSION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn ee_skip_spaces(mut ee: *mut expreval_t) {
    while (*ee).head < (*ee).len
        && *(*__ctype_b_loc())
            .offset(
                *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        (*ee).head = (*ee).head.wrapping_add(1);
    }
}
unsafe extern "C" fn ee_numeric_element(mut ee: *mut expreval_t) {
    let mut fpart: lilint_t = 0 as lilint_t;
    let mut fpartlen: lilint_t = 1 as lilint_t;
    (*ee).type_0 = EE_INT;
    ee_skip_spaces(ee);
    (*ee).ival = 0 as lilint_t;
    (*ee).dval = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    while (*ee).head < (*ee).len {
        if *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '.' as i32 {
            if (*ee).type_0 == EE_FLOAT {
                break;
            }
            (*ee).type_0 = EE_FLOAT;
            (*ee).head = (*ee).head.wrapping_add(1);
        } else if *(*__ctype_b_loc())
            .offset(
                *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0
        {
            break;
        }
        if (*ee).type_0 == EE_INT {
            (*ee).ival = (*ee).ival * 10 as lilint_t
                + (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                    - '0' as i32) as lilint_t;
        } else {
            fpart = fpart * 10 as lilint_t
                + (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                    - '0' as i32) as lilint_t;
            fpartlen *= 10 as lilint_t;
        }
        (*ee).head = (*ee).head.wrapping_add(1);
    }
    if (*ee).type_0 == EE_FLOAT {
        (*ee).dval = (*ee).ival as ::core::ffi::c_double
            + fpart as ::core::ffi::c_double / fpartlen as ::core::ffi::c_double;
    }
}
unsafe extern "C" fn ee_element(mut ee: *mut expreval_t) {
    if *(*__ctype_b_loc())
        .offset(*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        ee_numeric_element(ee);
        return;
    }
    (*ee).type_0 = EE_INT;
    (*ee).ival = 1 as lilint_t;
    (*ee).error = EERR_INVALID_EXPRESSION;
}
unsafe extern "C" fn ee_paren(mut ee: *mut expreval_t) {
    ee_skip_spaces(ee);
    if *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '(' as i32 {
        (*ee).head = (*ee).head.wrapping_add(1);
        ee_expr(ee);
        ee_skip_spaces(ee);
        if *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == ')' as i32 {
            (*ee).head = (*ee).head.wrapping_add(1);
        } else {
            (*ee).error = EERR_SYNTAX_ERROR;
        }
    } else {
        ee_element(ee);
    };
}
unsafe extern "C" fn ee_unary(mut ee: *mut expreval_t) {
    ee_skip_spaces(ee);
    if (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '-' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '+' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '~' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '!' as i32)
    {
        let fresh14 = (*ee).head;
        (*ee).head = (*ee).head.wrapping_add(1);
        let mut op: ::core::ffi::c_char = *(*ee).code.offset(fresh14 as isize);
        ee_unary(ee);
        if (*ee).error != 0 {
            return;
        }
        match op as ::core::ffi::c_int {
            45 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).dval = -(*ee).dval;
                    }
                    EE_INT => {
                        (*ee).ival = -(*ee).ival;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            126 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = !((*ee).dval as lilint_t);
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = !(*ee).ival;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            33 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).dval = ((*ee).dval == 0.) as ::core::ffi::c_int
                            as ::core::ffi::c_double;
                    }
                    EE_INT => {
                        (*ee).ival = ((*ee).ival == 0) as ::core::ffi::c_int as lilint_t;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            43 | _ => {}
        }
    } else {
        ee_paren(ee);
    };
}
unsafe extern "C" fn ee_muldiv(mut ee: *mut expreval_t) {
    ee_unary(ee);
    if (*ee).error != 0 {
        return;
    }
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && *(*__ctype_b_loc())
            .offset(
                *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '*' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '/' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '\\' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '%' as i32)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        match *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int {
            42 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).dval = (*ee).dval * odval;
                            }
                            EE_INT => {
                                (*ee).dval = (*ee).ival as ::core::ffi::c_double * odval;
                                (*ee).type_0 = EE_FLOAT;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).dval = (*ee).dval * oival as ::core::ffi::c_double;
                                (*ee).type_0 = EE_FLOAT;
                            }
                            EE_INT => {
                                (*ee).ival = (*ee).ival * oival;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            37 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).dval = fmod(odval, (*ee).dval);
                                }
                            }
                            EE_INT => {
                                if (*ee).ival == 0 as lilint_t {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).dval = fmod(
                                        odval,
                                        (*ee).ival as ::core::ffi::c_double,
                                    );
                                }
                                (*ee).type_0 = EE_FLOAT;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).dval = fmod(
                                        oival as ::core::ffi::c_double,
                                        (*ee).dval,
                                    );
                                }
                            }
                            EE_INT => {
                                if (*ee).ival == 0 as lilint_t {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).ival = oival % (*ee).ival;
                                }
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {}
                }
            }
            47 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).dval = odval / (*ee).dval;
                                }
                            }
                            EE_INT => {
                                if (*ee).ival == 0 as lilint_t {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).dval = odval / (*ee).ival as ::core::ffi::c_double;
                                }
                                (*ee).type_0 = EE_FLOAT;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).dval = oival as ::core::ffi::c_double / (*ee).dval;
                                }
                            }
                            EE_INT => {
                                if (*ee).ival == 0 as lilint_t {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).dval = oival as ::core::ffi::c_double
                                        / (*ee).ival as ::core::ffi::c_double;
                                }
                                (*ee).type_0 = EE_FLOAT;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {}
                }
            }
            92 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).ival = (odval / (*ee).dval) as lilint_t;
                                }
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                if (*ee).ival == 0 as lilint_t {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).ival = (odval / (*ee).ival as ::core::ffi::c_double)
                                        as lilint_t;
                                }
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).ival = (oival as ::core::ffi::c_double / (*ee).dval)
                                        as lilint_t;
                                }
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                if (*ee).ival == 0 as lilint_t {
                                    (*ee).error = EERR_DIVISION_BY_ZERO;
                                } else {
                                    (*ee).ival = oival / (*ee).ival;
                                }
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_addsub(mut ee: *mut expreval_t) {
    ee_muldiv(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && *(*__ctype_b_loc())
            .offset(
                *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '+' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '-' as i32)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        match *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int {
            43 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).dval = (*ee).dval + odval;
                            }
                            EE_INT => {
                                (*ee).dval = (*ee).ival as ::core::ffi::c_double + odval;
                                (*ee).type_0 = EE_FLOAT;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).dval = (*ee).dval + oival as ::core::ffi::c_double;
                                (*ee).type_0 = EE_FLOAT;
                            }
                            EE_INT => {
                                (*ee).ival = (*ee).ival + oival;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            45 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).dval = odval - (*ee).dval;
                            }
                            EE_INT => {
                                (*ee).dval = odval - (*ee).ival as ::core::ffi::c_double;
                                (*ee).type_0 = EE_FLOAT;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).dval = oival as ::core::ffi::c_double - (*ee).dval;
                                (*ee).type_0 = EE_FLOAT;
                            }
                            EE_INT => {
                                (*ee).ival = oival - (*ee).ival;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_shift(mut ee: *mut expreval_t) {
    ee_addsub(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '<' as i32
            && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int == '<' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '>' as i32
                && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_int == '>' as i32)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(1);
        match *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int {
            60 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (odval as lilint_t) << (*ee).dval as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (odval as lilint_t) << (*ee).ival;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = oival << (*ee).dval as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = oival << (*ee).ival;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            62 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = odval as lilint_t >> (*ee).dval as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = odval as lilint_t >> (*ee).ival;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = oival >> (*ee).dval as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = oival >> (*ee).ival;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_compare(mut ee: *mut expreval_t) {
    ee_shift(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '<' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                        as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '>' as i32
                && *(*__ctype_b_loc())
                    .offset(
                        *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                    & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int == 0
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '<' as i32
                && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_int == '=' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '>' as i32
                && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_int == '=' as i32)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        let mut op: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
        if *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '<' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                        as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0
        {
            op = 1 as ::core::ffi::c_int;
        } else if *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
            == '>' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                        as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0
        {
            op = 2 as ::core::ffi::c_int;
        } else if *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
            == '<' as i32
            && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int == '=' as i32
        {
            op = 3 as ::core::ffi::c_int;
        }
        (*ee).head = (*ee)
            .head
            .wrapping_add(
                (if op > 2 as ::core::ffi::c_int {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as size_t,
            );
        match op {
            1 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if odval < (*ee).dval {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if odval < (*ee).ival as ::core::ffi::c_double
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if (oival as ::core::ffi::c_double)
                                    < (*ee).dval
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if oival < (*ee).ival {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            2 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if odval > (*ee).dval {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if odval > (*ee).ival as ::core::ffi::c_double
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if oival as ::core::ffi::c_double > (*ee).dval
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if oival > (*ee).ival {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            3 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if odval <= (*ee).dval {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if odval
                                    <= (*ee).ival as ::core::ffi::c_double
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if oival as ::core::ffi::c_double
                                    <= (*ee).dval
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if oival <= (*ee).ival {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            4 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if odval >= (*ee).dval {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if odval
                                    >= (*ee).ival as ::core::ffi::c_double
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if oival as ::core::ffi::c_double
                                    >= (*ee).dval
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if oival >= (*ee).ival {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_equals(mut ee: *mut expreval_t) {
    ee_compare(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '=' as i32
            && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int == '=' as i32
            || *(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int
                == '!' as i32
                && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_int == '=' as i32)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        let mut op: ::core::ffi::c_int = if *(*ee).code.offset((*ee).head as isize)
            as ::core::ffi::c_int == '=' as i32
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
        (*ee).head = (*ee).head.wrapping_add(2 as size_t);
        match op {
            1 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if odval == (*ee).dval {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if odval
                                    == (*ee).ival as ::core::ffi::c_double
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if oival as ::core::ffi::c_double
                                    == (*ee).dval
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if oival == (*ee).ival {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            2 => {
                match (*ee).type_0 {
                    EE_FLOAT => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if odval != (*ee).dval {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if odval
                                    != (*ee).ival as ::core::ffi::c_double
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    EE_INT => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            EE_FLOAT => {
                                (*ee).ival = (if oival as ::core::ffi::c_double
                                    != (*ee).dval
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                                (*ee).type_0 = EE_INT;
                            }
                            EE_INT => {
                                (*ee).ival = (if oival != (*ee).ival {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee).error = EERR_INVALID_TYPE;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_bitand(mut ee: *mut expreval_t) {
    ee_equals(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '&' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                        as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(1);
        match (*ee).type_0 {
            EE_FLOAT => {
                ee_equals(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = odval as lilint_t & (*ee).dval as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = odval as lilint_t & (*ee).ival;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            EE_INT => {
                ee_equals(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = oival & (*ee).dval as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = oival & (*ee).ival;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {
                (*ee).error = EERR_INVALID_TYPE;
            }
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_bitor(mut ee: *mut expreval_t) {
    ee_bitand(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '|' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                        as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & _ISpunct as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(1);
        match (*ee).type_0 {
            EE_FLOAT => {
                ee_bitand(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = odval as lilint_t | (*ee).dval as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = odval as lilint_t | (*ee).ival;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            EE_INT => {
                ee_bitand(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = oival | (*ee).dval as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = oival | (*ee).ival;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {
                (*ee).error = EERR_INVALID_TYPE;
            }
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_logand(mut ee: *mut expreval_t) {
    ee_bitor(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '&' as i32
            && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int == '&' as i32)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(2 as size_t);
        match (*ee).type_0 {
            EE_FLOAT => {
                ee_bitor(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = (if odval != 0. && (*ee).dval != 0. {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = (if odval != 0. && (*ee).ival != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            EE_INT => {
                ee_bitor(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = (if oival != 0 && (*ee).dval != 0. {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = (if oival != 0 && (*ee).ival != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {
                (*ee).error = EERR_INVALID_TYPE;
            }
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_logor(mut ee: *mut expreval_t) {
    ee_logand(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0
        && (*(*ee).code.offset((*ee).head as isize) as ::core::ffi::c_int == '|' as i32
            && *(*ee).code.offset((*ee).head.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int == '|' as i32)
    {
        let mut odval: ::core::ffi::c_double = (*ee).dval;
        let mut oival: lilint_t = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(2 as size_t);
        match (*ee).type_0 {
            EE_FLOAT => {
                ee_logand(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = (if odval != 0. || (*ee).dval != 0. {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = (if odval != 0. || (*ee).ival != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            EE_INT => {
                ee_logand(ee);
                if (*ee).error != 0 {
                    return;
                }
                match (*ee).type_0 {
                    EE_FLOAT => {
                        (*ee).ival = (if oival != 0 || (*ee).dval != 0. {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                        (*ee).type_0 = EE_INT;
                    }
                    EE_INT => {
                        (*ee).ival = (if oival != 0 || (*ee).ival != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee).error = EERR_INVALID_TYPE;
                    }
                }
            }
            _ => {
                (*ee).error = EERR_INVALID_TYPE;
            }
        }
        ee_skip_spaces(ee);
    }
}
unsafe extern "C" fn ee_expr(mut ee: *mut expreval_t) {
    ee_logor(ee);
    if (*ee).error == EERR_INVALID_EXPRESSION {
        (*ee).error = EERR_NO_ERROR;
        (*ee).ival = 1 as lilint_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lil_eval_expr(
    mut lil: lil_t,
    mut code: lil_value_t,
) -> lil_value_t {
    let mut ee: expreval_t = _expreval_t {
        code: ::core::ptr::null::<::core::ffi::c_char>(),
        len: 0,
        head: 0,
        ival: 0,
        dval: 0.,
        type_0: 0,
        error: 0,
    };
    code = lil_subst_to_value(lil, code);
    if (*lil).error != 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    ee.code = lil_to_string(code);
    if *ee.code.offset(0 as ::core::ffi::c_int as isize) == 0 {
        lil_free_value(code);
        return lil_alloc_integer(0 as lilint_t);
    }
    ee.head = 0 as size_t;
    ee.len = (*code).l;
    ee.ival = 0 as lilint_t;
    ee.dval = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    ee.type_0 = EE_INT;
    ee.error = 0 as ::core::ffi::c_int;
    ee_expr(&raw mut ee);
    lil_free_value(code);
    if ee.error != 0 {
        match ee.error {
            EERR_DIVISION_BY_ZERO => {
                lil_set_error(
                    lil,
                    b"division by zero in expression\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            EERR_INVALID_TYPE => {
                lil_set_error(
                    lil,
                    b"mixing invalid types in expression\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            EERR_SYNTAX_ERROR => {
                lil_set_error(
                    lil,
                    b"expression syntax error\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if ee.type_0 == EE_INT {
        return lil_alloc_integer(ee.ival)
    } else {
        return lil_alloc_double(ee.dval)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_unused_name(
    mut lil: lil_t,
    mut part: *const ::core::ffi::c_char,
) -> lil_value_t {
    let mut name: *mut ::core::ffi::c_char = malloc(
        strlen(part).wrapping_add(64 as size_t),
    ) as *mut ::core::ffi::c_char;
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < -(1 as ::core::ffi::c_int) as size_t {
        sprintf(
            name,
            b"!!un!%s!%09u!nu!!\0" as *const u8 as *const ::core::ffi::c_char,
            part,
            i as ::core::ffi::c_uint,
        );
        if find_cmd(lil, name).is_null() {
            if lil_find_var(lil, (*lil).env, name).is_null() {
                val = lil_alloc_string(name);
                free(name as *mut ::core::ffi::c_void);
                return val;
            }
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
#[no_mangle]
pub unsafe extern "C" fn lil_arg(
    mut argv: *mut lil_value_t,
    mut index: size_t,
) -> lil_value_t {
    return if !argv.is_null() {
        *argv.offset(index as isize)
    } else {
        ::core::ptr::null_mut::<_lil_value_t>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_string(
    mut val: lil_value_t,
) -> *const ::core::ffi::c_char {
    return if !val.is_null() && !(*val).d.is_null() {
        (*val).d as *const ::core::ffi::c_char
    } else {
        b"\0" as *const u8 as *const ::core::ffi::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_double(mut val: lil_value_t) -> ::core::ffi::c_double {
    return atof(lil_to_string(val));
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_integer(mut val: lil_value_t) -> lilint_t {
    return atoll(lil_to_string(val)) as lilint_t;
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_boolean(mut val: lil_value_t) -> ::core::ffi::c_int {
    let mut s: *const ::core::ffi::c_char = lil_to_string(val);
    let mut i: size_t = 0;
    let mut dots: size_t = 0 as size_t;
    if *s.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while *s.offset(i as isize) != 0 {
        if *s.offset(i as isize) as ::core::ffi::c_int != '0' as i32
            && *s.offset(i as isize) as ::core::ffi::c_int != '.' as i32
        {
            return 1 as ::core::ffi::c_int;
        }
        if *s.offset(i as isize) as ::core::ffi::c_int == '.' as i32 {
            if dots != 0 {
                return 1 as ::core::ffi::c_int;
            }
            dots = 1 as size_t;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_string(
    mut str: *const ::core::ffi::c_char,
) -> lil_value_t {
    return alloc_value(str);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_double(
    mut num: ::core::ffi::c_double,
) -> lil_value_t {
    let mut buff: [::core::ffi::c_char; 128] = [0; 128];
    sprintf(
        &raw mut buff as *mut ::core::ffi::c_char,
        b"%f\0" as *const u8 as *const ::core::ffi::c_char,
        num,
    );
    return alloc_value(&raw mut buff as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_integer(mut num: lilint_t) -> lil_value_t {
    let mut buff: [::core::ffi::c_char; 128] = [0; 128];
    sprintf(
        &raw mut buff as *mut ::core::ffi::c_char,
        b"%lli\0" as *const u8 as *const ::core::ffi::c_char,
        num,
    );
    return alloc_value(&raw mut buff as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn lil_free(mut lil: lil_t) {
    let mut i: size_t = 0;
    if lil.is_null() {
        return;
    }
    free((*lil).err_msg as *mut ::core::ffi::c_void);
    lil_free_value((*lil).empty);
    while !(*lil).env.is_null() {
        let mut next: lil_env_t = (*(*lil).env).parent as lil_env_t;
        lil_free_env((*lil).env);
        (*lil).env = next;
    }
    i = 0 as size_t;
    while i < (*lil).cmds {
        if !(**(*lil).cmd.offset(i as isize)).argnames.is_null() {
            lil_free_list((**(*lil).cmd.offset(i as isize)).argnames);
        }
        lil_free_value((**(*lil).cmd.offset(i as isize)).code);
        free((**(*lil).cmd.offset(i as isize)).name as *mut ::core::ffi::c_void);
        free(*(*lil).cmd.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free((*lil).cmd as *mut ::core::ffi::c_void);
    free((*lil).dollarprefix as *mut ::core::ffi::c_void);
    free((*lil).catcher as *mut ::core::ffi::c_void);
    free(lil as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_data(
    mut lil: lil_t,
    mut data: *mut ::core::ffi::c_void,
) {
    (*lil).data = data;
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_data(mut lil: lil_t) -> *mut ::core::ffi::c_void {
    return (*lil).data;
}
unsafe extern "C" fn fnc_reflect(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut func: lil_func_t = ::core::ptr::null_mut::<_lil_func_t>();
    let mut type_0: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut i: size_t = 0;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    type_0 = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    if strcmp(type_0, b"version\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return lil_alloc_string(LIL_VERSION_STRING.as_ptr());
    }
    if strcmp(type_0, b"args\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if argc < 2 as size_t {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        func = find_cmd(
            lil,
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
        );
        if func.is_null() || (*func).argnames.is_null() {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        return lil_list_to_value((*func).argnames, 1 as ::core::ffi::c_int);
    }
    if strcmp(type_0, b"body\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if argc < 2 as size_t {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        func = find_cmd(
            lil,
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
        );
        if func.is_null() || (*func).proc_0.is_some() {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        return lil_clone_value((*func).code);
    }
    if strcmp(type_0, b"func-count\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return lil_alloc_integer((*lil).cmds as lilint_t);
    }
    if strcmp(type_0, b"funcs\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut funcs: lil_list_t = lil_alloc_list();
        i = 0 as size_t;
        while i < (*lil).cmds {
            lil_list_append(
                funcs,
                lil_alloc_string((**(*lil).cmd.offset(i as isize)).name),
            );
            i = i.wrapping_add(1);
        }
        r = lil_list_to_value(funcs, 1 as ::core::ffi::c_int);
        lil_free_list(funcs);
        return r;
    }
    if strcmp(type_0, b"vars\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut vars: lil_list_t = lil_alloc_list();
        let mut env: lil_env_t = (*lil).env;
        while !env.is_null() {
            i = 0 as size_t;
            while i < (*env).vars {
                lil_list_append(
                    vars,
                    lil_alloc_string((**(*env).var.offset(i as isize)).n),
                );
                i = i.wrapping_add(1);
            }
            env = (*env).parent as lil_env_t;
        }
        r = lil_list_to_value(vars, 1 as ::core::ffi::c_int);
        lil_free_list(vars);
        return r;
    }
    if strcmp(type_0, b"globals\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut vars_0: lil_list_t = lil_alloc_list();
        i = 0 as size_t;
        while i < (*(*lil).rootenv).vars {
            lil_list_append(
                vars_0,
                lil_alloc_string((**(*(*lil).rootenv).var.offset(i as isize)).n),
            );
            i = i.wrapping_add(1);
        }
        r = lil_list_to_value(vars_0, 1 as ::core::ffi::c_int);
        lil_free_list(vars_0);
        return r;
    }
    if strcmp(type_0, b"has-func\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut target: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if argc == 1 as size_t {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        target = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
        i = 0 as size_t;
        while i < (*lil).cmds {
            if strcmp(target, (**(*lil).cmd.offset(i as isize)).name) == 0 {
                return lil_alloc_string(
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            i = i.wrapping_add(1);
        }
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if strcmp(type_0, b"has-var\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut target_0: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut env_0: lil_env_t = (*lil).env;
        if argc == 1 as size_t {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        target_0 = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
        while !env_0.is_null() {
            i = 0 as size_t;
            while i < (*env_0).vars {
                if strcmp(target_0, (**(*env_0).var.offset(i as isize)).n) == 0 {
                    return lil_alloc_string(
                        b"1\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                i = i.wrapping_add(1);
            }
            env_0 = (*env_0).parent as lil_env_t;
        }
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if strcmp(type_0, b"has-global\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut target_1: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if argc == 1 as size_t {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        target_1 = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
        i = 0 as size_t;
        while i < (*(*lil).rootenv).vars {
            if strcmp(target_1, (**(*(*lil).rootenv).var.offset(i as isize)).n) == 0 {
                return lil_alloc_string(
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            i = i.wrapping_add(1);
        }
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if strcmp(type_0, b"error\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return if !(*lil).err_msg.is_null() {
            lil_alloc_string((*lil).err_msg)
        } else {
            ::core::ptr::null_mut::<_lil_value_t>()
        };
    }
    if strcmp(type_0, b"dollar-prefix\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        let mut r_0: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
        if argc == 1 as size_t {
            return lil_alloc_string((*lil).dollarprefix);
        }
        r_0 = lil_alloc_string((*lil).dollarprefix);
        free((*lil).dollarprefix as *mut ::core::ffi::c_void);
        (*lil).dollarprefix = strclone(
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
        );
        return r_0;
    }
    if strcmp(type_0, b"this\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut env_1: lil_env_t = (*lil).env;
        while env_1 != (*lil).rootenv && (*env_1).catcher_for.is_null()
            && (*env_1).func.is_null()
        {
            env_1 = (*env_1).parent as lil_env_t;
        }
        if !(*env_1).catcher_for.is_null() {
            return lil_alloc_string((*lil).catcher);
        }
        if env_1 == (*lil).rootenv {
            return lil_alloc_string((*lil).rootcode);
        }
        return if !(*env_1).func.is_null() {
            (*(*env_1).func).code
        } else {
            ::core::ptr::null_mut::<_lil_value_t>()
        };
    }
    if strcmp(type_0, b"name\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut env_2: lil_env_t = (*lil).env;
        while env_2 != (*lil).rootenv && (*env_2).catcher_for.is_null()
            && (*env_2).func.is_null()
        {
            env_2 = (*env_2).parent as lil_env_t;
        }
        if !(*env_2).catcher_for.is_null() {
            return (*env_2).catcher_for;
        }
        if env_2 == (*lil).rootenv {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        return if !(*env_2).func.is_null() {
            lil_alloc_string((*(*env_2).func).name)
        } else {
            ::core::ptr::null_mut::<_lil_value_t>()
        };
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_func(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut name: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut cmd: lil_func_t = ::core::ptr::null_mut::<_lil_func_t>();
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if argc == 3 as size_t {
        name = lil_clone_value(*argv.offset(0 as ::core::ffi::c_int as isize));
        cmd = add_func(
            lil,
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        );
        (*cmd).argnames = lil_subst_to_list(
            lil,
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        (*cmd).code = lil_clone_value(*argv.offset(2 as ::core::ffi::c_int as isize));
    } else {
        name = lil_unused_name(
            lil,
            b"anonymous-function\0" as *const u8 as *const ::core::ffi::c_char,
        );
        cmd = add_func(lil, lil_to_string(name));
        if argc < 2 as size_t {
            let mut tmp: lil_value_t = lil_alloc_string(
                b"args\0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*cmd).argnames = lil_subst_to_list(lil, tmp);
            lil_free_value(tmp);
            (*cmd).code = lil_clone_value(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
        } else {
            (*cmd).argnames = lil_subst_to_list(
                lil,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            (*cmd).code = lil_clone_value(
                *argv.offset(1 as ::core::ffi::c_int as isize),
            );
        }
    }
    return name;
}
unsafe extern "C" fn fnc_rename(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut func: lil_func_t = ::core::ptr::null_mut::<_lil_func_t>();
    let mut oldname: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut newname: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    oldname = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    newname = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
    func = find_cmd(lil, oldname);
    if func.is_null() {
        let mut msg: *mut ::core::ffi::c_char = malloc(
            (24 as size_t).wrapping_add(strlen(oldname)),
        ) as *mut ::core::ffi::c_char;
        sprintf(
            msg,
            b"unknown function '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            oldname,
        );
        lil_set_error_at(lil, (*lil).head, msg);
        free(msg as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    r = lil_alloc_string((*func).name);
    free((*func).name as *mut ::core::ffi::c_void);
    (*func).name = strclone(newname);
    return r;
}
unsafe extern "C" fn fnc_unusedname(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    return lil_unused_name(
        lil,
        if argc > 0 as size_t {
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize))
        } else {
            b"unusedname\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn fnc_quote(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut i: size_t = 0;
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    r = alloc_value(::core::ptr::null::<::core::ffi::c_char>());
    i = 0 as size_t;
    while i < argc {
        if i != 0 {
            lil_append_char(r, ' ' as i32 as ::core::ffi::c_char);
        }
        lil_append_val(r, *argv.offset(i as isize));
        i = i.wrapping_add(1);
    }
    return r;
}
unsafe extern "C" fn fnc_set(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut i: size_t = 0 as size_t;
    let mut var: lil_var_t = ::core::ptr::null_mut::<_lil_var_t>();
    let mut access: ::core::ffi::c_int = LIL_SETVAR_LOCAL;
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        b"global\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        i = 1 as size_t;
        access = LIL_SETVAR_GLOBAL;
    }
    while i < argc {
        if argc == i.wrapping_add(1 as size_t) {
            return lil_clone_value(
                lil_get_var(lil, lil_to_string(*argv.offset(i as isize))),
            );
        }
        var = lil_set_var(
            lil,
            lil_to_string(*argv.offset(i as isize)),
            *argv.offset(i.wrapping_add(1 as size_t) as isize),
            access,
        );
        i = i.wrapping_add(2 as size_t);
    }
    return if !var.is_null() {
        lil_clone_value((*var).v)
    } else {
        ::core::ptr::null_mut::<_lil_value_t>()
    };
}
unsafe extern "C" fn fnc_local(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < argc {
        let mut varname: *const ::core::ffi::c_char = lil_to_string(
            *argv.offset(i as isize),
        );
        if lil_find_local_var(lil, (*lil).env, varname).is_null() {
            lil_set_var(lil, varname, (*lil).empty, LIL_SETVAR_LOCAL_NEW);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_write(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut i: size_t = 0;
    let mut msg: lil_value_t = lil_alloc_string(
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    i = 0 as size_t;
    while i < argc {
        if i != 0 {
            lil_append_char(msg, ' ' as i32 as ::core::ffi::c_char);
        }
        lil_append_val(msg, *argv.offset(i as isize));
        i = i.wrapping_add(1);
    }
    if (*lil).callback[LIL_CALLBACK_WRITE as usize].is_some() {
        let mut proc_0: lil_write_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_write_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_WRITE as usize]);
        proc_0.expect("non-null function pointer")(lil, lil_to_string(msg));
    } else {
        printf(b"%s\0" as *const u8 as *const ::core::ffi::c_char, lil_to_string(msg));
    }
    lil_free_value(msg);
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_print(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    fnc_write(lil, argc, argv);
    if (*lil).callback[LIL_CALLBACK_WRITE as usize].is_some() {
        let mut proc_0: lil_write_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_write_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_WRITE as usize]);
        proc_0
            .expect(
                "non-null function pointer",
            )(lil, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_eval(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 1 as size_t {
        return lil_parse_value(
            lil,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
        );
    }
    if argc > 1 as size_t {
        let mut val: lil_value_t = alloc_value(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < argc {
            if i != 0 {
                lil_append_char(val, ' ' as i32 as ::core::ffi::c_char);
            }
            lil_append_val(val, *argv.offset(i as isize));
            i = i.wrapping_add(1);
        }
        r = lil_parse_value(lil, val, 0 as ::core::ffi::c_int);
        lil_free_value(val);
        return r;
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_topeval(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut thisenv: lil_env_t = (*lil).env;
    let mut thisdownenv: lil_env_t = (*lil).downenv;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    (*lil).env = (*lil).rootenv;
    (*lil).downenv = thisenv;
    r = fnc_eval(lil, argc, argv);
    (*lil).downenv = thisdownenv;
    (*lil).env = thisenv;
    return r;
}
unsafe extern "C" fn fnc_upeval(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut thisenv: lil_env_t = (*lil).env;
    let mut thisdownenv: lil_env_t = (*lil).downenv;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if (*lil).rootenv == thisenv {
        return fnc_eval(lil, argc, argv);
    }
    (*lil).env = (*thisenv).parent as lil_env_t;
    (*lil).downenv = thisenv;
    r = fnc_eval(lil, argc, argv);
    (*lil).env = thisenv;
    (*lil).downenv = thisdownenv;
    return r;
}
unsafe extern "C" fn fnc_downeval(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut upenv: lil_env_t = (*lil).env;
    let mut downenv: lil_env_t = (*lil).downenv;
    if downenv.is_null() {
        return fnc_eval(lil, argc, argv);
    }
    (*lil).downenv = ::core::ptr::null_mut::<_lil_env_t>();
    (*lil).env = downenv;
    r = fnc_eval(lil, argc, argv);
    (*lil).downenv = downenv;
    (*lil).env = upenv;
    return r;
}
unsafe extern "C" fn fnc_enveval(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut invars: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut outvars: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut varvalues: *mut lil_value_t = ::core::ptr::null_mut::<lil_value_t>();
    let mut codeindex: ::core::ffi::c_int = 0;
    let mut i: size_t = 0;
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if argc == 1 as size_t {
        codeindex = 0 as ::core::ffi::c_int;
    } else if argc >= 2 as size_t {
        invars = lil_subst_to_list(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
        varvalues = malloc(
            (::core::mem::size_of::<lil_value_t>() as size_t)
                .wrapping_mul(lil_list_size(invars)),
        ) as *mut lil_value_t;
        i = 0 as size_t;
        while i < lil_list_size(invars) {
            let ref mut fresh15 = *varvalues.offset(i as isize);
            *fresh15 = lil_clone_value(
                lil_get_var(lil, lil_to_string(lil_list_get(invars, i))),
            );
            i = i.wrapping_add(1);
        }
        if argc > 2 as size_t {
            codeindex = 2 as ::core::ffi::c_int;
            outvars = lil_subst_to_list(
                lil,
                *argv.offset(1 as ::core::ffi::c_int as isize),
            );
        } else {
            codeindex = 1 as ::core::ffi::c_int;
        }
    }
    lil_push_env(lil);
    if !invars.is_null() {
        i = 0 as size_t;
        while i < lil_list_size(invars) {
            lil_set_var(
                lil,
                lil_to_string(lil_list_get(invars, i)),
                *varvalues.offset(i as isize),
                LIL_SETVAR_LOCAL_NEW,
            );
            lil_free_value(*varvalues.offset(i as isize));
            i = i.wrapping_add(1);
        }
    }
    r = lil_parse_value(lil, *argv.offset(codeindex as isize), 0 as ::core::ffi::c_int);
    if !invars.is_null() || !outvars.is_null() {
        if !outvars.is_null() {
            varvalues = realloc(
                varvalues as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<lil_value_t>() as size_t)
                    .wrapping_mul(lil_list_size(outvars)),
            ) as *mut lil_value_t;
            i = 0 as size_t;
            while i < lil_list_size(outvars) {
                let ref mut fresh16 = *varvalues.offset(i as isize);
                *fresh16 = lil_clone_value(
                    lil_get_var(lil, lil_to_string(lil_list_get(outvars, i))),
                );
                i = i.wrapping_add(1);
            }
        } else {
            i = 0 as size_t;
            while i < lil_list_size(invars) {
                let ref mut fresh17 = *varvalues.offset(i as isize);
                *fresh17 = lil_clone_value(
                    lil_get_var(lil, lil_to_string(lil_list_get(invars, i))),
                );
                i = i.wrapping_add(1);
            }
        }
    }
    lil_pop_env(lil);
    if !invars.is_null() {
        if !outvars.is_null() {
            i = 0 as size_t;
            while i < lil_list_size(outvars) {
                lil_set_var(
                    lil,
                    lil_to_string(lil_list_get(outvars, i)),
                    *varvalues.offset(i as isize),
                    LIL_SETVAR_LOCAL,
                );
                lil_free_value(*varvalues.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            i = 0 as size_t;
            while i < lil_list_size(invars) {
                lil_set_var(
                    lil,
                    lil_to_string(lil_list_get(invars, i)),
                    *varvalues.offset(i as isize),
                    LIL_SETVAR_LOCAL,
                );
                lil_free_value(*varvalues.offset(i as isize));
                i = i.wrapping_add(1);
            }
        }
        lil_free_list(invars);
        if !outvars.is_null() {
            lil_free_list(outvars);
        }
        free(varvalues as *mut ::core::ffi::c_void);
    }
    return r;
}
unsafe extern "C" fn fnc_jaileval(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut i: size_t = 0;
    let mut sublil: lil_t = ::core::ptr::null_mut::<_lil_t>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut base: size_t = 0 as size_t;
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        b"clean\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        base = 1 as size_t;
        if argc == 1 as size_t {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
    }
    sublil = lil_new();
    if base != 1 as size_t {
        i = (*lil).syscmds;
        while i < (*lil).cmds {
            let mut fnc: lil_func_t = *(*lil).cmd.offset(i as isize);
            if !(*fnc).proc_0.is_none() {
                lil_register(sublil, (*fnc).name, (*fnc).proc_0);
            }
            i = i.wrapping_add(1);
        }
    }
    r = lil_parse_value(sublil, *argv.offset(base as isize), 1 as ::core::ffi::c_int);
    lil_free(sublil);
    return r;
}
unsafe extern "C" fn fnc_count(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut buff: [::core::ffi::c_char; 64] = [0; 64];
    if argc == 0 {
        return alloc_value(b"0\0" as *const u8 as *const ::core::ffi::c_char);
    }
    list = lil_subst_to_list(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
    sprintf(
        &raw mut buff as *mut ::core::ffi::c_char,
        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
        (*list).c as ::core::ffi::c_uint,
    );
    lil_free_list(list);
    return alloc_value(&raw mut buff as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn fnc_index(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut index: size_t = 0;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    list = lil_subst_to_list(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
    index = lil_to_integer(*argv.offset(1 as ::core::ffi::c_int as isize)) as size_t;
    if index >= (*list).c {
        r = ::core::ptr::null_mut::<_lil_value_t>();
    } else {
        r = lil_clone_value(*(*list).v.offset(index as isize));
    }
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_indexof(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut index: size_t = 0;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    list = lil_subst_to_list(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
    index = 0 as size_t;
    while index < (*list).c {
        if strcmp(
            lil_to_string(*(*list).v.offset(index as isize)),
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
        ) == 0
        {
            r = lil_alloc_integer(index as lilint_t);
            break;
        } else {
            index = index.wrapping_add(1);
        }
    }
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_append(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut i: size_t = 0;
    let mut base: size_t = 1 as size_t;
    let mut access: ::core::ffi::c_int = LIL_SETVAR_LOCAL;
    let mut varname: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    varname = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    if strcmp(varname, b"global\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if argc < 3 as size_t {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        varname = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
        base = 2 as size_t;
        access = LIL_SETVAR_GLOBAL;
    }
    list = lil_subst_to_list(lil, lil_get_var(lil, varname));
    i = base;
    while i < argc {
        lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
        i = i.wrapping_add(1);
    }
    r = lil_list_to_value(list, 1 as ::core::ffi::c_int);
    lil_free_list(list);
    lil_set_var(lil, varname, r, access);
    return r;
}
unsafe extern "C" fn fnc_slice(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut slice: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut i: size_t = 0;
    let mut from: lilint_t = 0;
    let mut to: lilint_t = 0;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if argc < 2 as size_t {
        return lil_clone_value(*argv.offset(0 as ::core::ffi::c_int as isize));
    }
    from = lil_to_integer(*argv.offset(1 as ::core::ffi::c_int as isize));
    if from < 0 as lilint_t {
        from = 0 as lilint_t;
    }
    list = lil_subst_to_list(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
    to = if argc > 2 as size_t {
        lil_to_integer(*argv.offset(2 as ::core::ffi::c_int as isize))
    } else {
        (*list).c as lilint_t
    };
    if to > (*list).c as lilint_t {
        to = (*list).c as lilint_t;
    }
    if to < from {
        to = from;
    }
    slice = lil_alloc_list();
    i = from as size_t;
    while i < to as size_t {
        lil_list_append(slice, lil_clone_value(*(*list).v.offset(i as isize)));
        i = i.wrapping_add(1);
    }
    lil_free_list(list);
    r = lil_list_to_value(slice, 1 as ::core::ffi::c_int);
    lil_free_list(slice);
    return r;
}
unsafe extern "C" fn fnc_filter(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut filtered: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut i: size_t = 0;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut varname: *const ::core::ffi::c_char = b"x\0" as *const u8
        as *const ::core::ffi::c_char;
    let mut base: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if argc < 2 as size_t {
        return lil_clone_value(*argv.offset(0 as ::core::ffi::c_int as isize));
    }
    if argc > 2 as size_t {
        base = 1 as ::core::ffi::c_int;
        varname = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    }
    list = lil_subst_to_list(lil, *argv.offset(base as isize));
    filtered = lil_alloc_list();
    i = 0 as size_t;
    while i < (*list).c && (*(*lil).env).breakrun == 0 {
        lil_set_var(lil, varname, *(*list).v.offset(i as isize), LIL_SETVAR_LOCAL_ONLY);
        r = lil_eval_expr(lil, *argv.offset((base + 1 as ::core::ffi::c_int) as isize));
        if lil_to_boolean(r) != 0 {
            lil_list_append(filtered, lil_clone_value(*(*list).v.offset(i as isize)));
        }
        lil_free_value(r);
        i = i.wrapping_add(1);
    }
    lil_free_list(list);
    r = lil_list_to_value(filtered, 1 as ::core::ffi::c_int);
    lil_free_list(filtered);
    return r;
}
unsafe extern "C" fn fnc_list(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = lil_alloc_list();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < argc {
        lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
        i = i.wrapping_add(1);
    }
    r = lil_list_to_value(list, 1 as ::core::ffi::c_int);
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_subst(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return lil_subst_to_value(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn fnc_concat(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut tmp: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut i: size_t = 0;
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    r = lil_alloc_string(b"\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as size_t;
    while i < argc {
        list = lil_subst_to_list(lil, *argv.offset(i as isize));
        tmp = lil_list_to_value(list, 1 as ::core::ffi::c_int);
        lil_free_list(list);
        lil_append_val(r, tmp);
        lil_free_value(tmp);
        i = i.wrapping_add(1);
    }
    return r;
}
unsafe extern "C" fn fnc_foreach(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut rlist: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut i: size_t = 0;
    let mut listidx: size_t = 0 as size_t;
    let mut codeidx: size_t = 1 as size_t;
    let mut varname: *const ::core::ffi::c_char = b"i\0" as *const u8
        as *const ::core::ffi::c_char;
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if argc >= 3 as size_t {
        varname = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
        listidx = 1 as size_t;
        codeidx = 2 as size_t;
    }
    rlist = lil_alloc_list();
    list = lil_subst_to_list(lil, *argv.offset(listidx as isize));
    i = 0 as size_t;
    while i < (*list).c {
        let mut rv: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
        lil_set_var(lil, varname, *(*list).v.offset(i as isize), LIL_SETVAR_LOCAL_ONLY);
        rv = lil_parse_value(
            lil,
            *argv.offset(codeidx as isize),
            0 as ::core::ffi::c_int,
        );
        if (*rv).l != 0 {
            lil_list_append(rlist, rv);
        } else {
            lil_free_value(rv);
        }
        if (*(*lil).env).breakrun != 0 || (*lil).error != 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    r = lil_list_to_value(rlist, 1 as ::core::ffi::c_int);
    lil_free_list(list);
    lil_free_list(rlist);
    return r;
}
unsafe extern "C" fn fnc_return(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    (*(*lil).env).breakrun = 1 as ::core::ffi::c_int;
    lil_free_value((*(*lil).env).retval);
    (*(*lil).env).retval = if argc < 1 as size_t {
        ::core::ptr::null_mut::<_lil_value_t>()
    } else {
        lil_clone_value(*argv.offset(0 as ::core::ffi::c_int as isize))
    };
    (*(*lil).env).retval_set = 1 as ::core::ffi::c_int;
    return if argc < 1 as size_t {
        ::core::ptr::null_mut::<_lil_value_t>()
    } else {
        lil_clone_value(*argv.offset(0 as ::core::ffi::c_int as isize))
    };
}
unsafe extern "C" fn fnc_result(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc > 0 as size_t {
        lil_free_value((*(*lil).env).retval);
        (*(*lil).env).retval = lil_clone_value(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        (*(*lil).env).retval_set = 1 as ::core::ffi::c_int;
    }
    return if (*(*lil).env).retval_set != 0 {
        lil_clone_value((*(*lil).env).retval)
    } else {
        ::core::ptr::null_mut::<_lil_value_t>()
    };
}
unsafe extern "C" fn fnc_expr(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 1 as size_t {
        return lil_eval_expr(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
    }
    if argc > 1 as size_t {
        let mut val: lil_value_t = alloc_value(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < argc {
            if i != 0 {
                lil_append_char(val, ' ' as i32 as ::core::ffi::c_char);
            }
            lil_append_val(val, *argv.offset(i as isize));
            i = i.wrapping_add(1);
        }
        r = lil_eval_expr(lil, val);
        lil_free_value(val);
        return r;
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn real_inc(
    mut lil: lil_t,
    mut varname: *const ::core::ffi::c_char,
    mut v: ::core::ffi::c_float,
) -> lil_value_t {
    let mut pv: lil_value_t = lil_get_var(lil, varname);
    let mut dv: ::core::ffi::c_double = lil_to_double(pv) + v as ::core::ffi::c_double;
    if fmod(dv, 1 as ::core::ffi::c_int as ::core::ffi::c_double) != 0. {
        pv = lil_alloc_double(dv);
    } else {
        pv = lil_alloc_integer(
            (lil_to_integer(pv) as ::core::ffi::c_float + v) as lilint_t,
        );
    }
    lil_set_var(lil, varname, pv, LIL_SETVAR_LOCAL);
    return pv;
}
unsafe extern "C" fn fnc_inc(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return real_inc(
        lil,
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        (if argc > 1 as size_t {
            lil_to_double(*argv.offset(1 as ::core::ffi::c_int as isize))
        } else {
            1 as ::core::ffi::c_int as ::core::ffi::c_double
        }) as ::core::ffi::c_float,
    );
}
unsafe extern "C" fn fnc_dec(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return real_inc(
        lil,
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        -if argc > 1 as size_t {
            lil_to_double(*argv.offset(1 as ::core::ffi::c_int as isize))
        } else {
            1 as ::core::ffi::c_int as ::core::ffi::c_double
        } as ::core::ffi::c_float,
    );
}
unsafe extern "C" fn fnc_read(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut size: size_t = 0;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if (*lil).callback[LIL_CALLBACK_READ as usize].is_some() {
        let mut proc_0: lil_read_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_read_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_READ as usize]);
        buffer = proc_0
            .expect(
                "non-null function pointer",
            )(lil, lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)));
    } else {
        f = fopen(
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if f.is_null() {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        fseek(f, 0 as ::core::ffi::c_long, SEEK_END);
        size = ftell(f) as size_t;
        fseek(f, 0 as ::core::ffi::c_long, SEEK_SET);
        buffer = malloc(size.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        fread(buffer as *mut ::core::ffi::c_void, 1 as size_t, size, f);
        *buffer.offset(size as isize) = 0 as ::core::ffi::c_char;
        fclose(f);
    }
    r = lil_alloc_string(buffer);
    free(buffer as *mut ::core::ffi::c_void);
    return r;
}
unsafe extern "C" fn fnc_store(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut buffer: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if (*lil).callback[LIL_CALLBACK_STORE as usize].is_some() {
        let mut proc_0: lil_store_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_store_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_STORE as usize]);
        proc_0
            .expect(
                "non-null function pointer",
            )(
            lil,
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
        );
    } else {
        f = fopen(
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
            b"wb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if f.is_null() {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        buffer = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
        fwrite(buffer as *const ::core::ffi::c_void, 1 as size_t, strlen(buffer), f);
        fclose(f);
    }
    return lil_clone_value(*argv.offset(1 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn fnc_if(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut base: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut not: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v: ::core::ffi::c_int = 0;
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        b"not\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        not = 1 as ::core::ffi::c_int;
        base = not;
    }
    if argc < (base as size_t).wrapping_add(2 as size_t) {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    val = lil_eval_expr(lil, *argv.offset(base as isize));
    if val.is_null() || (*lil).error != 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    v = lil_to_boolean(val);
    if not != 0 {
        v = (v == 0) as ::core::ffi::c_int;
    }
    if v != 0 {
        r = lil_parse_value(
            lil,
            *argv.offset((base + 1 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
    } else if argc > (base as size_t).wrapping_add(2 as size_t) {
        r = lil_parse_value(
            lil,
            *argv.offset((base + 2 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
    }
    lil_free_value(val);
    return r;
}
unsafe extern "C" fn fnc_while(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut base: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut not: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v: ::core::ffi::c_int = 0;
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        b"not\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        not = 1 as ::core::ffi::c_int;
        base = not;
    }
    if argc < (base as size_t).wrapping_add(2 as size_t) {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
        val = lil_eval_expr(lil, *argv.offset(base as isize));
        if val.is_null() || (*lil).error != 0 {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        v = lil_to_boolean(val);
        if not != 0 {
            v = (v == 0) as ::core::ffi::c_int;
        }
        if v == 0 {
            lil_free_value(val);
            break;
        } else {
            if !r.is_null() {
                lil_free_value(r);
            }
            r = lil_parse_value(
                lil,
                *argv.offset((base + 1 as ::core::ffi::c_int) as isize),
                0 as ::core::ffi::c_int,
            );
            lil_free_value(val);
        }
    }
    return r;
}
unsafe extern "C" fn fnc_for(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 4 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    lil_free_value(
        lil_parse_value(
            lil,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
        ),
    );
    while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
        val = lil_eval_expr(lil, *argv.offset(1 as ::core::ffi::c_int as isize));
        if val.is_null() || (*lil).error != 0 {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        if lil_to_boolean(val) == 0 {
            lil_free_value(val);
            break;
        } else {
            if !r.is_null() {
                lil_free_value(r);
            }
            r = lil_parse_value(
                lil,
                *argv.offset(3 as ::core::ffi::c_int as isize),
                0 as ::core::ffi::c_int,
            );
            lil_free_value(val);
            lil_free_value(
                lil_parse_value(
                    lil,
                    *argv.offset(2 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                ),
            );
        }
    }
    return r;
}
unsafe extern "C" fn fnc_char(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut s: [::core::ffi::c_char; 2] = [0; 2];
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    s[0 as ::core::ffi::c_int as usize] = lil_to_integer(
        *argv.offset(0 as ::core::ffi::c_int as isize),
    ) as ::core::ffi::c_char;
    s[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    return lil_alloc_string(&raw mut s as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn fnc_charat(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut index: size_t = 0;
    let mut chstr: [::core::ffi::c_char; 2] = [0; 2];
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    str = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    index = lil_to_integer(*argv.offset(1 as ::core::ffi::c_int as isize)) as size_t;
    if index >= strlen(str) {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    chstr[0 as ::core::ffi::c_int as usize] = *str.offset(index as isize);
    chstr[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    return lil_alloc_string(&raw mut chstr as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn fnc_codeat(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut index: size_t = 0;
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    str = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    index = lil_to_integer(*argv.offset(1 as ::core::ffi::c_int as isize)) as size_t;
    if index >= strlen(str) {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return lil_alloc_integer(*str.offset(index as isize) as lilint_t);
}
unsafe extern "C" fn fnc_substr(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    let mut i: size_t = 0;
    let mut slen: size_t = 0;
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    str = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    if *str.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    slen = strlen(str);
    start = atoll(lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)))
        as size_t;
    end = if argc > 2 as size_t {
        atoll(lil_to_string(*argv.offset(2 as ::core::ffi::c_int as isize))) as size_t
    } else {
        slen
    };
    if end > slen {
        end = slen;
    }
    if start >= end {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    r = lil_alloc_string(b"\0" as *const u8 as *const ::core::ffi::c_char);
    i = start;
    while i < end {
        lil_append_char(r, *str.offset(i as isize));
        i = i.wrapping_add(1);
    }
    return r;
}
unsafe extern "C" fn fnc_strpos(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut hay: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut min: size_t = 0 as size_t;
    if argc < 2 as size_t {
        return lil_alloc_integer(-(1 as ::core::ffi::c_int) as lilint_t);
    }
    hay = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    if argc > 2 as size_t {
        min = atoll(lil_to_string(*argv.offset(2 as ::core::ffi::c_int as isize)))
            as size_t;
        if min >= strlen(hay) {
            return lil_alloc_integer(-(1 as ::core::ffi::c_int) as lilint_t);
        }
    }
    str = strstr(
        hay.offset(min as isize),
        lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
    );
    if str.is_null() {
        return lil_alloc_integer(-(1 as ::core::ffi::c_int) as lilint_t);
    }
    return lil_alloc_integer(str.offset_from(hay) as lilint_t);
}
unsafe extern "C" fn fnc_length(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut i: size_t = 0;
    let mut total: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < argc {
        if i != 0 {
            total = total.wrapping_add(1);
        }
        total = total.wrapping_add(strlen(lil_to_string(*argv.offset(i as isize))));
        i = i.wrapping_add(1);
    }
    return lil_alloc_integer(total as lilint_t);
}
unsafe extern "C" fn real_trim(
    mut str: *const ::core::ffi::c_char,
    mut chars: *const ::core::ffi::c_char,
    mut left: ::core::ffi::c_int,
    mut right: ::core::ffi::c_int,
) -> lil_value_t {
    let mut base: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if left != 0 {
        while *str.offset(base as isize) as ::core::ffi::c_int != 0
            && !strchr(chars, *str.offset(base as isize) as ::core::ffi::c_int).is_null()
        {
            base += 1;
        }
        if right == 0 {
            r = lil_alloc_string(
                if *str.offset(base as isize) as ::core::ffi::c_int != 0 {
                    str.offset(base as isize)
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
            );
        }
    }
    if right != 0 {
        let mut len: size_t = 0;
        let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        s = strclone(str.offset(base as isize));
        len = strlen(s);
        while len != 0
            && !strchr(
                    chars,
                    *s.offset(len.wrapping_sub(1 as size_t) as isize)
                        as ::core::ffi::c_int,
                )
                .is_null()
        {
            len = len.wrapping_sub(1);
        }
        *s.offset(len as isize) = 0 as ::core::ffi::c_char;
        r = lil_alloc_string(s);
        free(s as *mut ::core::ffi::c_void);
    }
    return r;
}
unsafe extern "C" fn fnc_trim(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return real_trim(
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        if argc < 2 as size_t {
            b" \x0C\n\r\t\x0B\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize))
        },
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn fnc_ltrim(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return real_trim(
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        if argc < 2 as size_t {
            b" \x0C\n\r\t\x0B\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize))
        },
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn fnc_rtrim(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return real_trim(
        lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
        if argc < 2 as size_t {
            b" \x0C\n\r\t\x0B\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize))
        },
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn fnc_strcmp(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return lil_alloc_integer(
        strcmp(
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
        ) as lilint_t,
    );
}
unsafe extern "C" fn fnc_streq(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    return lil_alloc_integer(
        (if strcmp(
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
            lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize)),
        ) != 0
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as lilint_t,
    );
}
unsafe extern "C" fn fnc_repstr(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut from: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut to: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut src: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut sub: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut idx: size_t = 0;
    let mut fromlen: size_t = 0;
    let mut tolen: size_t = 0;
    let mut srclen: size_t = 0;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if argc < 3 as size_t {
        return lil_clone_value(*argv.offset(0 as ::core::ffi::c_int as isize));
    }
    from = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
    to = lil_to_string(*argv.offset(2 as ::core::ffi::c_int as isize));
    if *from.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    src = strclone(lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)));
    srclen = strlen(src);
    fromlen = strlen(from);
    tolen = strlen(to);
    loop {
        sub = strstr(src, from);
        if sub.is_null() {
            break;
        }
        let mut newsrc: *mut ::core::ffi::c_char = malloc(
            srclen.wrapping_sub(fromlen).wrapping_add(tolen).wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        idx = sub.offset_from(src) as ::core::ffi::c_long as size_t;
        if idx != 0 {
            memcpy(
                newsrc as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                idx,
            );
        }
        memcpy(
            newsrc.offset(idx as isize) as *mut ::core::ffi::c_void,
            to as *const ::core::ffi::c_void,
            tolen,
        );
        memcpy(
            newsrc.offset(idx as isize).offset(tolen as isize)
                as *mut ::core::ffi::c_void,
            src.offset(idx as isize).offset(fromlen as isize)
                as *const ::core::ffi::c_void,
            srclen.wrapping_sub(idx).wrapping_sub(fromlen),
        );
        srclen = srclen.wrapping_sub(fromlen).wrapping_add(tolen);
        free(src as *mut ::core::ffi::c_void);
        src = newsrc;
        *src.offset(srclen as isize) = 0 as ::core::ffi::c_char;
    }
    r = lil_alloc_string(src);
    free(src as *mut ::core::ffi::c_void);
    return r;
}
unsafe extern "C" fn fnc_split(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut sep: *const ::core::ffi::c_char = b" \0" as *const u8
        as *const ::core::ffi::c_char;
    let mut i: size_t = 0;
    let mut val: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if argc == 0 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if argc > 1 as size_t {
        sep = lil_to_string(*argv.offset(1 as ::core::ffi::c_int as isize));
        if sep.is_null() || *sep.offset(0 as ::core::ffi::c_int as isize) == 0 {
            return lil_clone_value(*argv.offset(0 as ::core::ffi::c_int as isize));
        }
    }
    val = lil_alloc_string(b"\0" as *const u8 as *const ::core::ffi::c_char);
    str = lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize));
    list = lil_alloc_list();
    i = 0 as size_t;
    while *str.offset(i as isize) != 0 {
        if !strchr(sep, *str.offset(i as isize) as ::core::ffi::c_int).is_null() {
            lil_list_append(list, val);
            val = lil_alloc_string(b"\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            lil_append_char(val, *str.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
    lil_list_append(list, val);
    val = lil_list_to_value(list, 1 as ::core::ffi::c_int);
    lil_free_list(list);
    return val;
}
unsafe extern "C" fn fnc_try(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if (*lil).error != 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    r = lil_parse_value(
        lil,
        *argv.offset(0 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_int,
    );
    if (*lil).error != 0 {
        (*lil).error = ERROR_NOERROR;
        lil_free_value(r);
        if argc > 1 as size_t {
            r = lil_parse_value(
                lil,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                0 as ::core::ffi::c_int,
            );
        } else {
            r = ::core::ptr::null_mut::<_lil_value_t>();
        }
    }
    return r;
}
unsafe extern "C" fn fnc_error(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    lil_set_error(
        lil,
        if argc > 0 as size_t {
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize))
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
    );
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_exit(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if (*lil).callback[LIL_CALLBACK_EXIT as usize].is_some() {
        let mut proc_0: lil_exit_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_exit_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_EXIT as usize]);
        proc_0
            .expect(
                "non-null function pointer",
            )(
            lil,
            if argc > 0 as size_t {
                *argv.offset(0 as ::core::ffi::c_int as isize)
            } else {
                ::core::ptr::null_mut::<_lil_value_t>()
            },
        );
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_source(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut size: size_t = 0;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    if argc < 1 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    if (*lil).callback[LIL_CALLBACK_SOURCE as usize].is_some() {
        let mut proc_0: lil_source_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_source_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_SOURCE as usize]);
        buffer = proc_0
            .expect(
                "non-null function pointer",
            )(lil, lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)));
    } else if (*lil).callback[LIL_CALLBACK_READ as usize].is_some() {
        let mut proc_1: lil_read_callback_proc_t = ::core::mem::transmute::<
            lil_callback_proc_t,
            lil_read_callback_proc_t,
        >((*lil).callback[LIL_CALLBACK_READ as usize]);
        buffer = proc_1
            .expect(
                "non-null function pointer",
            )(lil, lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)));
    } else {
        f = fopen(
            lil_to_string(*argv.offset(0 as ::core::ffi::c_int as isize)),
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if f.is_null() {
            return ::core::ptr::null_mut::<_lil_value_t>();
        }
        fseek(f, 0 as ::core::ffi::c_long, SEEK_END);
        size = ftell(f) as size_t;
        fseek(f, 0 as ::core::ffi::c_long, SEEK_SET);
        buffer = malloc(size.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        fread(buffer as *mut ::core::ffi::c_void, 1 as size_t, size, f);
        *buffer.offset(size as isize) = 0 as ::core::ffi::c_char;
        fclose(f);
    }
    r = lil_parse(lil, buffer, 0 as size_t, 0 as ::core::ffi::c_int);
    free(buffer as *mut ::core::ffi::c_void);
    return r;
}
unsafe extern "C" fn fnc_lmap(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut list: lil_list_t = ::core::ptr::null_mut::<_lil_list_t>();
    let mut i: size_t = 0;
    if argc < 2 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    list = lil_subst_to_list(lil, *argv.offset(0 as ::core::ffi::c_int as isize));
    i = 1 as size_t;
    while i < argc {
        lil_set_var(
            lil,
            lil_to_string(*argv.offset(i as isize)),
            lil_list_get(list, i.wrapping_sub(1 as size_t)),
            LIL_SETVAR_LOCAL,
        );
        i = i.wrapping_add(1);
    }
    lil_free_list(list);
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_rand(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    return lil_alloc_double(
        rand() as ::core::ffi::c_double / RAND_MAX as ::core::ffi::c_double,
    );
}
unsafe extern "C" fn fnc_catcher(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 0 as size_t {
        return lil_alloc_string((*lil).catcher)
    } else {
        let mut catcher: *const ::core::ffi::c_char = lil_to_string(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        free((*lil).catcher as *mut ::core::ffi::c_void);
        (*lil).catcher = if *catcher.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int != 0
        {
            strclone(catcher)
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    }
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn register_stdcmds(mut lil: lil_t) {
    lil_register(
        lil,
        b"reflect\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_reflect
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"func\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_func
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"rename\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_rename
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"unusedname\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_unusedname
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"quote\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_quote
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"set\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_set
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"local\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_local
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"write\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_write
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"print\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_print
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"eval\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_eval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"topeval\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_topeval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"upeval\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_upeval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"downeval\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_downeval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"enveval\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_enveval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"jaileval\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_jaileval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"count\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_count
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"index\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_index
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"indexof\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_indexof
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"filter\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_filter
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"list\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_list
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"append\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_append
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"slice\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_slice
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"subst\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_subst
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"concat\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_concat
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"foreach\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_foreach
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"return\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_return
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"result\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_result
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"expr\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_expr
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"inc\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_inc
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"dec\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_dec
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"read\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_read
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"store\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_store
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"if\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_if
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"while\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_while
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"for\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_for
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"char\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_char
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"charat\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_charat
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"codeat\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_codeat
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"substr\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_substr
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"strpos\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_strpos
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"length\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_length
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"trim\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_trim
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"ltrim\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_ltrim
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"rtrim\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_rtrim
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"strcmp\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_strcmp
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"streq\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_streq
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"repstr\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_repstr
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"split\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_split
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"try\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_try
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"error\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_error
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"exit\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_exit
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"source\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_source
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"lmap\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_lmap
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"rand\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_rand
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"catcher\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            fnc_catcher
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    (*lil).syscmds = (*lil).cmds;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();

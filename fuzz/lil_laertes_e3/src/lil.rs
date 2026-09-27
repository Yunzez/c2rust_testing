
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn atof(_: * const std::os::raw::c_char) -> std::os::raw::c_double;
    #[no_mangle]
    fn atoll(_: * const std::os::raw::c_char) -> std::os::raw::c_longlong;
    #[no_mangle]
    fn rand() -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn free(_: * mut core::ffi::c_void);
    #[no_mangle]
    fn realloc(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn fclose(_: * mut crate::lil::__sFILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::lil::__sFILE;
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::lil::__sFILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fseek(_: * mut crate::lil::__sFILE, _: std::os::raw::c_long, _: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn ftell(_: * mut crate::lil::__sFILE) -> std::os::raw::c_long;
    #[no_mangle]
    fn fwrite(_: * const core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: * mut crate::lil::__sFILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn sprintf(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: * mut core::ffi::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn strchr(_: * const std::os::raw::c_char, _: std::os::raw::c_int) -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: * const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strstr(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char)
     -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn memcpy(_: * mut core::ffi::c_void, _: * const core::ffi::c_void, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn __maskrune(_: std::os::raw::c_int, _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    static mut _DefaultRuneLocale: crate::lil::_RuneLocale;
    #[no_mangle]
    fn fmod(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = std::os::raw::c_int;
pub type __darwin_off_t = std::os::raw::c_longlong;
pub type int64_t = std::os::raw::c_longlong;
pub type size_t = std::os::raw::c_ulong;
pub type fpos_t = std::os::raw::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: * mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
impl __sbuf {
    pub const fn new() -> Self {
        __sbuf {
        _base: (0 as * mut std::os::raw::c_uchar),
        _size: 0
        }
    }
}

impl std::default::Default for __sbuf {
    fn default() -> Self { __sbuf::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: * mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: crate::lil::__sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: * mut core::ffi::c_void,
    pub _close: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut std::os::raw::c_char,_: std::os::raw::c_int,) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_longlong,_: std::os::raw::c_int,) -> std::os::raw::c_longlong>,
    pub _write: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const std::os::raw::c_char,_: std::os::raw::c_int,) -> std::os::raw::c_int>,
    pub _ub: crate::lil::__sbuf,
    pub _extra: * mut crate::lil::__sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: crate::lil::__sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: std::os::raw::c_longlong,
}
impl __sFILE {
    pub const fn new() -> Self {
        __sFILE {
        _p: (0 as * mut std::os::raw::c_uchar),
        _r: 0,
        _w: 0,
        _flags: 0,
        _file: 0,
        _bf: crate::lil::__sbuf::new(),
        _lbfsize: 0,
        _cookie: (0 as * mut core::ffi::c_void),
        _close: None,
        _read: None,
        _seek: None,
        _write: None,
        _ub: crate::lil::__sbuf::new(),
        _extra: (0 as * mut crate::lil::__sFILEX),
        _ur: 0,
        _ubuf: [0,0,0,],
        _nbuf: [0,],
        _lb: crate::lil::__sbuf::new(),
        _blksize: 0,
        _offset: 0
        }
    }
}

impl std::default::Default for __sFILE {
    fn default() -> Self { __sFILE::new() }
}

pub type FILE = crate::lil::__sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: std::os::raw::c_int,
    pub __max: std::os::raw::c_int,
    pub __map: std::os::raw::c_int,
    pub __types: * mut std::os::raw::c_uint,
}
impl _RuneEntry {
    pub const fn new() -> Self {
        _RuneEntry {
        __min: 0,
        __max: 0,
        __map: 0,
        __types: (0 as * mut std::os::raw::c_uint)
        }
    }
}

impl std::default::Default for _RuneEntry {
    fn default() -> Self { _RuneEntry::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: std::os::raw::c_int,
    pub __ranges: * mut crate::lil::_RuneEntry,
}
impl _RuneRange {
    pub const fn new() -> Self {
        _RuneRange {
        __nranges: 0,
        __ranges: (0 as * mut crate::lil::_RuneEntry)
        }
    }
}

impl std::default::Default for _RuneRange {
    fn default() -> Self { _RuneRange::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [std::os::raw::c_char; 14],
    pub __mask: std::os::raw::c_uint,
}
impl _RuneCharClass {
    pub const fn new() -> Self {
        _RuneCharClass {
        __name: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        __mask: 0
        }
    }
}

impl std::default::Default for _RuneCharClass {
    fn default() -> Self { _RuneCharClass::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [std::os::raw::c_char; 8],
    pub __encoding: [std::os::raw::c_char; 32],
    pub __sgetrune: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,_: std::os::raw::c_ulong,_: * mut * const std::os::raw::c_char,) -> std::os::raw::c_int>,
    pub __sputrune: Option<unsafe extern "C"  fn(_: std::os::raw::c_int,_: * mut std::os::raw::c_char,_: std::os::raw::c_ulong,_: * mut * mut std::os::raw::c_char,) -> std::os::raw::c_int>,
    pub __invalid_rune: std::os::raw::c_int,
    pub __runetype: [std::os::raw::c_uint; 256],
    pub __maplower: [std::os::raw::c_int; 256],
    pub __mapupper: [std::os::raw::c_int; 256],
    pub __runetype_ext: crate::lil::_RuneRange,
    pub __maplower_ext: crate::lil::_RuneRange,
    pub __mapupper_ext: crate::lil::_RuneRange,
    pub __variable: * mut core::ffi::c_void,
    pub __variable_len: std::os::raw::c_int,
    pub __ncharclasses: std::os::raw::c_int,
    pub __charclasses: * mut crate::lil::_RuneCharClass,
}
impl _RuneLocale {
    pub const fn new() -> Self {
        _RuneLocale {
        __magic: [0,0,0,0,0,0,0,0,],
        __encoding: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        __sgetrune: None,
        __sputrune: None,
        __invalid_rune: 0,
        __runetype: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        __maplower: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        __mapupper: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        __runetype_ext: crate::lil::_RuneRange::new(),
        __maplower_ext: crate::lil::_RuneRange::new(),
        __mapupper_ext: crate::lil::_RuneRange::new(),
        __variable: (0 as * mut core::ffi::c_void),
        __variable_len: 0,
        __ncharclasses: 0,
        __charclasses: (0 as * mut crate::lil::_RuneCharClass)
        }
    }
}

impl std::default::Default for _RuneLocale {
    fn default() -> Self { _RuneLocale::new() }
}

pub type lilint_t = std::os::raw::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_value_t {
    pub l: std::os::raw::c_ulong,
    pub d: * mut std::os::raw::c_char,
}
impl _lil_value_t {
    pub const fn new() -> Self {
        _lil_value_t {
        l: 0,
        d: (0 as * mut std::os::raw::c_char)
        }
    }
}

impl std::default::Default for _lil_value_t {
    fn default() -> Self { _lil_value_t::new() }
}

pub type lil_value_t = * mut crate::lil::_lil_value_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_func_t {
    pub name: * mut std::os::raw::c_char,
    pub code: * mut crate::lil::_lil_value_t,
    pub argnames: * mut crate::lil::_lil_list_t,
    pub proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: std::os::raw::c_ulong,_: * mut * mut crate::lil::_lil_value_t,) -> * mut crate::lil::_lil_value_t>,
}
impl _lil_func_t {
    pub const fn new() -> Self {
        _lil_func_t {
        name: (0 as * mut std::os::raw::c_char),
        code: (0 as * mut crate::lil::_lil_value_t),
        argnames: (0 as * mut crate::lil::_lil_list_t),
        proc_0: None
        }
    }
}

impl std::default::Default for _lil_func_t {
    fn default() -> Self { _lil_func_t::new() }
}

pub type lil_func_proc_t<'a1, 'a2, 'a3, 'a4>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: std::os::raw::c_ulong,_: Option<&'a2 mut Option<&'a3 mut crate::lil::_lil_value_t>>,) -> Option<&'a4 mut crate::lil::_lil_value_t>>;
pub type lil_t = * mut crate::lil::_lil_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_t {
    pub code: * const std::os::raw::c_char,
    pub rootcode: * const std::os::raw::c_char,
    pub clen: std::os::raw::c_ulong,
    pub head: std::os::raw::c_ulong,
    pub ignoreeol: std::os::raw::c_int,
    pub cmd: * mut * mut crate::lil::_lil_func_t,
    pub cmds: std::os::raw::c_ulong,
    pub syscmds: std::os::raw::c_ulong,
    pub cmdmap: crate::lil::_hashmap_t,
    pub catcher: * mut std::os::raw::c_char,
    pub in_catcher: std::os::raw::c_int,
    pub dollarprefix: * mut std::os::raw::c_char,
    pub env: * mut crate::lil::_lil_env_t,
    pub rootenv: * mut crate::lil::_lil_env_t,
    pub downenv: * mut crate::lil::_lil_env_t,
    pub empty: * mut crate::lil::_lil_value_t,
    pub error: std::os::raw::c_int,
    pub err_head: std::os::raw::c_ulong,
    pub err_msg: * mut std::os::raw::c_char,
    pub callback: [Option<unsafe extern "C"  fn() -> ()>; 8],
    pub parse_depth: std::os::raw::c_ulong,
    pub data: * mut core::ffi::c_void,
    pub embed: * mut std::os::raw::c_char,
    pub embedlen: std::os::raw::c_ulong,
}
impl _lil_t {
    pub const fn new() -> Self {
        _lil_t {
        code: (0 as * const std::os::raw::c_char),
        rootcode: (0 as * const std::os::raw::c_char),
        clen: 0,
        head: 0,
        ignoreeol: 0,
        cmd: (0 as * mut * mut crate::lil::_lil_func_t),
        cmds: 0,
        syscmds: 0,
        cmdmap: crate::lil::_hashmap_t::new(),
        catcher: (0 as * mut std::os::raw::c_char),
        in_catcher: 0,
        dollarprefix: (0 as * mut std::os::raw::c_char),
        env: (0 as * mut crate::lil::_lil_env_t),
        rootenv: (0 as * mut crate::lil::_lil_env_t),
        downenv: (0 as * mut crate::lil::_lil_env_t),
        empty: (0 as * mut crate::lil::_lil_value_t),
        error: 0,
        err_head: 0,
        err_msg: (0 as * mut std::os::raw::c_char),
        callback: [None,None,None,None,None,None,None,None,],
        parse_depth: 0,
        data: (0 as * mut core::ffi::c_void),
        embed: (0 as * mut std::os::raw::c_char),
        embedlen: 0
        }
    }
}

impl std::default::Default for _lil_t {
    fn default() -> Self { _lil_t::new() }
}

pub type lil_callback_proc_t = Option<unsafe extern "C"  fn() -> ()>;
pub type lil_env_t = * mut crate::lil::_lil_env_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_env_t {
    pub parent: * mut crate::lil::_lil_env_t,
    pub func: * mut crate::lil::_lil_func_t,
    pub catcher_for: * mut crate::lil::_lil_value_t,
    pub var: * mut * mut crate::lil::_lil_var_t,
    pub vars: std::os::raw::c_ulong,
    pub varmap: crate::lil::_hashmap_t,
    pub retval: * mut crate::lil::_lil_value_t,
    pub retval_set: std::os::raw::c_int,
    pub breakrun: std::os::raw::c_int,
}
impl _lil_env_t {
    pub const fn new() -> Self {
        _lil_env_t {
        parent: (0 as * mut crate::lil::_lil_env_t),
        func: (0 as * mut crate::lil::_lil_func_t),
        catcher_for: (0 as * mut crate::lil::_lil_value_t),
        var: (0 as * mut * mut crate::lil::_lil_var_t),
        vars: 0,
        varmap: crate::lil::_hashmap_t::new(),
        retval: (0 as * mut crate::lil::_lil_value_t),
        retval_set: 0,
        breakrun: 0
        }
    }
}

impl std::default::Default for _lil_env_t {
    fn default() -> Self { _lil_env_t::new() }
}

pub type hashmap_t = crate::lil::_hashmap_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hashmap_t {
    pub cell: [crate::lil::hashcell_t; 256],
}
impl _hashmap_t {
    pub const fn new() -> Self {
        _hashmap_t {
        cell: [crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),crate::lil::hashcell_t::new(),]
        }
    }
}

impl std::default::Default for _hashmap_t {
    fn default() -> Self { _hashmap_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashcell_t {
    pub e: * mut crate::lil::hashentry_t,
    pub c: std::os::raw::c_ulong,
}
impl hashcell_t {
    pub const fn new() -> Self {
        hashcell_t {
        e: (0 as * mut crate::lil::hashentry_t),
        c: 0
        }
    }
}

impl std::default::Default for hashcell_t {
    fn default() -> Self { hashcell_t::new() }
}

/* note: static lil_xxx functions might become public later */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashentry_t {
    pub k: * mut std::os::raw::c_char,
    pub v: * mut core::ffi::c_void,
}
impl hashentry_t {
    pub const fn new() -> Self {
        hashentry_t {
        k: (0 as * mut std::os::raw::c_char),
        v: (0 as * mut core::ffi::c_void)
        }
    }
}

impl std::default::Default for hashentry_t {
    fn default() -> Self { hashentry_t::new() }
}

pub type lil_var_t = * mut crate::lil::_lil_var_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_var_t {
    pub n: * mut std::os::raw::c_char,
    pub w: * mut std::os::raw::c_char,
    pub env: * mut crate::lil::_lil_env_t,
    pub v: * mut crate::lil::_lil_value_t,
}
impl _lil_var_t {
    pub const fn new() -> Self {
        _lil_var_t {
        n: (0 as * mut std::os::raw::c_char),
        w: (0 as * mut std::os::raw::c_char),
        env: (0 as * mut crate::lil::_lil_env_t),
        v: (0 as * mut crate::lil::_lil_value_t)
        }
    }
}

impl std::default::Default for _lil_var_t {
    fn default() -> Self { _lil_var_t::new() }
}

pub type lil_func_t = * mut crate::lil::_lil_func_t;
pub type lil_list_t = * mut crate::lil::_lil_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_list_t {
    pub v: * mut * mut crate::lil::_lil_value_t,
    pub c: std::os::raw::c_ulong,
    pub cap: std::os::raw::c_ulong,
}
impl _lil_list_t {
    pub const fn new() -> Self {
        _lil_list_t {
        v: (0 as * mut * mut crate::lil::_lil_value_t),
        c: 0,
        cap: 0
        }
    }
}

impl std::default::Default for _lil_list_t {
    fn default() -> Self { _lil_list_t::new() }
}

pub type lil_exit_callback_proc_t<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: Option<&'a2 mut crate::lil::_lil_value_t>,) -> ()>;
pub type lil_write_callback_proc_t<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: Option<&'a2 std::os::raw::c_char>,) -> ()>;
pub type lil_read_callback_proc_t<'a1, 'a2, 'a3>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: Option<&'a2 std::os::raw::c_char>,) -> Option<&'a3 mut std::os::raw::c_char>>;
pub type lil_source_callback_proc_t<'a1, 'a2, 'a3>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: Option<&'a2 std::os::raw::c_char>,) -> Option<&'a3 mut std::os::raw::c_char>>;
pub type lil_store_callback_proc_t<'a1, 'a2, 'a3>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: Option<&'a2 std::os::raw::c_char>,_: Option<&'a3 std::os::raw::c_char>,) -> ()>;
pub type lil_error_callback_proc_t<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: std::os::raw::c_ulong,_: Option<&'a2 std::os::raw::c_char>,) -> ()>;
pub type lil_setvar_callback_proc_t<'a1, 'a2, 'a3, 'a4>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: Option<&'a2 std::os::raw::c_char>,_: Option<&'a3 mut Option<&'a4 mut crate::lil::_lil_value_t>>,) -> std::os::raw::c_int>;
pub type lil_getvar_callback_proc_t<'a1, 'a2, 'a3, 'a4>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::lil::_lil_t>,_: Option<&'a2 std::os::raw::c_char>,_: Option<&'a3 mut Option<&'a4 mut crate::lil::_lil_value_t>>,) -> std::os::raw::c_int>;
pub type expreval_t = crate::lil::_expreval_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _expreval_t {
    pub code: * const std::os::raw::c_char,
    pub len: std::os::raw::c_ulong,
    pub head: std::os::raw::c_ulong,
    pub ival: std::os::raw::c_longlong,
    pub dval: std::os::raw::c_double,
    pub type_0: std::os::raw::c_int,
    pub error: std::os::raw::c_int,
}
impl _expreval_t {
    pub const fn new() -> Self {
        _expreval_t {
        code: (0 as * const std::os::raw::c_char),
        len: 0,
        head: 0,
        ival: 0,
        dval: 0.0,
        type_0: 0,
        error: 0
        }
    }
}

impl std::default::Default for _expreval_t {
    fn default() -> Self { _expreval_t::new() }
}

#[inline]
 extern "C" fn isascii(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (_c & !(0x7f as std::os::raw::c_int) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn __istype(mut _c: std::os::raw::c_int,
                              mut _f: std::os::raw::c_ulong) -> std::os::raw::c_int {
    return ((__e3_rune_mask(_c) & _f) != 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn __isctype(mut _c: std::os::raw::c_int,
                               mut _f: std::os::raw::c_ulong) -> std::os::raw::c_int {
    return ((__e3_rune_mask(_c) & _f) != 0) as std::os::raw::c_int;
}
// E3 harness shim: macOS rune-table classification -> Rust-equivalent (isdigit/ispunct/isspace)
#[inline]
unsafe fn __e3_rune_mask(_c: std::os::raw::c_int) -> std::os::raw::c_ulong {
    if _c < 0 || _c >= 256 { return 0; }
    let b = _c as u8; let mut m: std::os::raw::c_ulong = 0;
    if b.is_ascii_digit() { m |= 0x400; }
    if b.is_ascii_punctuation() { m |= 0x2000; }
    if matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') { m |= 0x4000; }
    m
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __isctype(_c, 0x400 as std::os::raw::c_long as std::os::raw::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn ispunct(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __istype(_c, 0x2000 as std::os::raw::c_long as std::os::raw::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __istype(_c, 0x4000 as std::os::raw::c_long as std::os::raw::c_ulong);
}
unsafe extern "C" fn strclone(mut s: * const std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    let mut len: u64 =
        strlen(s).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    let mut ns: * mut i8 = malloc(len) as *mut std::os::raw::c_char;
    if ns.is_null() { return 0 as *mut std::os::raw::c_char }
    memcpy(ns as *mut std::os::raw::c_void, s as *const std::os::raw::c_void, len);
    return ns;
}
unsafe extern "C" fn hm_hash(mut key: * const std::os::raw::c_char) -> std::os::raw::c_ulong {
    let mut hash: u64 = 5381 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut c: i32 = 0;
    loop  {
        let mut fresh0 = key;
        key = key.offset(1);
        c = *fresh0 as std::os::raw::c_int;
        if !(c != 0) { break ; }
        hash =
            (hash <<
                 5 as
                     std::os::raw::c_int).wrapping_add(hash).wrapping_add(c as
                                                                      std::os::raw::c_ulong)
    }
    return hash;
}
unsafe extern "C" fn hm_init(mut hm: * mut crate::lil::_hashmap_t) {
    memset(hm as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<hashmap_t>() as std::os::raw::c_ulong);
}
unsafe extern "C" fn hm_destroy<'a1>(mut hm: Option<&'a1 mut crate::lil::_hashmap_t>) {
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < 256 as std::os::raw::c_int as std::os::raw::c_ulong {
        j = 0 as std::os::raw::c_int as size_t;
        while j < (*(borrow(& hm)).unwrap()).cell[i as usize].c {
            free((*(*(borrow(& hm)).unwrap()).cell[i as usize].e.offset(j as isize)).k as
                     *mut std::os::raw::c_void);
            j = j.wrapping_add(1)
        }
        free((*(borrow_mut(&mut hm)).unwrap()).cell[i as usize].e as *mut std::os::raw::c_void);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn hm_put<'a1>(mut hm: Option<&'a1 mut crate::lil::_hashmap_t>,
                            mut key: * const std::os::raw::c_char,
                            mut value: * mut core::ffi::c_void) {
    let mut cell: * mut crate::lil::hashcell_t =
        (*(borrow_mut(&mut hm)).unwrap()).cell.as_mut_ptr().offset((hm_hash(key) &
                                            0xff as std::os::raw::c_int as
                                                std::os::raw::c_ulong) as isize);
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*cell).c {
        if strcmp(key, (*(*cell).e.offset(i as isize)).k) == 0 {
            let ref mut fresh1 = (*(*cell).e.offset(i as isize)).v;
            *fresh1 = value;
            return
        }
        i = i.wrapping_add(1)
    }
    (*cell).e =
        realloc((*cell).e as *mut std::os::raw::c_void,
                (::std::mem::size_of::<hashentry_t>() as
                     std::os::raw::c_ulong).wrapping_mul((*cell).c.wrapping_add(1 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong)))
            as *mut hashentry_t;
    let ref mut fresh2 = (*(*cell).e.offset((*cell).c as isize)).k;
    *fresh2 = strclone(key);
    let ref mut fresh3 = (*(*cell).e.offset((*cell).c as isize)).v;
    *fresh3 = value;
    (*cell).c = (*cell).c.wrapping_add(1);
}
unsafe extern "C" fn hm_get<'a1>(mut hm: Option<&'a1 mut crate::lil::_hashmap_t>,
                            mut key: * const std::os::raw::c_char)
 -> * mut core::ffi::c_void {
    let mut cell: * mut crate::lil::hashcell_t =
        (*(borrow_mut(&mut hm)).unwrap()).cell.as_mut_ptr().offset((hm_hash(key) &
                                            0xff as std::os::raw::c_int as
                                                std::os::raw::c_ulong) as isize);
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*cell).c {
        if strcmp(key, (*(*cell).e.offset(i as isize)).k) == 0 {
            return (*(*cell).e.offset(i as isize)).v
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut std::os::raw::c_void;
}
unsafe extern "C" fn hm_has<'a1>(mut hm: Option<&'a1 mut crate::lil::_hashmap_t>,
                            mut key: * const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut cell: * mut crate::lil::hashcell_t =
        (*(borrow_mut(&mut hm)).unwrap()).cell.as_mut_ptr().offset((hm_hash(key) &
                                            0xff as std::os::raw::c_int as
                                                std::os::raw::c_ulong) as isize);
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*cell).c {
        if strcmp(key, (*(*cell).e.offset(i as isize)).k) == 0 {
            return 1 as std::os::raw::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn alloc_value_len(mut str: * const std::os::raw::c_char,
                                     mut len: std::os::raw::c_ulong) -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<_lil_value_t>() as std::os::raw::c_ulong) as
            lil_value_t;
    if val.is_null() { return 0 as lil_value_t }
    if !str.is_null() {
        (*val).l = len;
        (*val).d =
            malloc(len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)) as
                *mut std::os::raw::c_char;
        if (*val).d.is_null() {
            free(val as *mut std::os::raw::c_void);
            return 0 as lil_value_t
        }
        memcpy((*val).d as *mut std::os::raw::c_void, str as *const std::os::raw::c_void,
               len);
        *(*val).d.offset(len as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char
    } else {
        (*val).l = 0 as std::os::raw::c_int as size_t;
        (*val).d = 0 as *mut std::os::raw::c_char
    }
    return val;
}
unsafe extern "C" fn alloc_value(mut str: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_value_t {
    return alloc_value_len(str,
                           if !str.is_null() {
                               strlen(str)
                           } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong });
}
#[no_mangle]
pub unsafe extern "C" fn lil_clone_value(mut src: * mut crate::lil::_lil_value_t)
 -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if src.is_null() { return 0 as lil_value_t }
    val =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<_lil_value_t>() as std::os::raw::c_ulong) as
            lil_value_t;
    if val.is_null() { return 0 as lil_value_t }
    (*val).l = (*src).l;
    if (*src).l != 0 {
        (*val).d =
            malloc((*val).l.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong))
                as *mut std::os::raw::c_char;
        if (*val).d.is_null() {
            free(val as *mut std::os::raw::c_void);
            return 0 as lil_value_t
        }
        memcpy((*val).d as *mut std::os::raw::c_void, (*src).d as *const std::os::raw::c_void,
               (*val).l.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
    } else { (*val).d = 0 as *mut std::os::raw::c_char }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_char(mut val: * mut crate::lil::_lil_value_t,
                                         mut ch: std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut new: * mut i8 =
        realloc((*val).d as *mut std::os::raw::c_void,
                (*val).l.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if new.is_null() { return 0 as std::os::raw::c_int }
    let mut fresh4 = (*val).l;
    (*val).l = (*val).l.wrapping_add(1);
    *new.offset(fresh4 as isize) = ch;
    *new.offset((*val).l as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    (*val).d = new;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_string_len(mut val: * mut crate::lil::_lil_value_t,
                                               mut s: * const std::os::raw::c_char,
                                               mut len: std::os::raw::c_ulong)
 -> std::os::raw::c_int {
    let mut new: * mut i8 = 0 as *mut std::os::raw::c_char;
    if s.is_null() || *s.offset(0 as std::os::raw::c_int as isize) == 0 {
        return 1 as std::os::raw::c_int
    }
    new =
        realloc((*val).d as *mut std::os::raw::c_void,
                (*val).l.wrapping_add(len).wrapping_add(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if new.is_null() { return 0 as std::os::raw::c_int }
    memcpy(new.offset((*val).l as isize) as *mut std::os::raw::c_void,
           s as *const std::os::raw::c_void,
           len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
    (*val).d = new;
    (*val).l =
        ((*val).l as std::os::raw::c_ulong).wrapping_add(len) as size_t as size_t;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_string(mut val: * mut crate::lil::_lil_value_t,
                                           mut s: * const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return lil_append_string_len(val, s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_val(mut val: * mut crate::lil::_lil_value_t,
                                        mut v: * mut crate::lil::_lil_value_t) -> std::os::raw::c_int {
    let mut new: * mut i8 = 0 as *mut std::os::raw::c_char;
    if v.is_null() || (*v).l == 0 { return 1 as std::os::raw::c_int }
    new =
        realloc((*val).d as *mut std::os::raw::c_void,
                (*val).l.wrapping_add((*v).l).wrapping_add(1 as std::os::raw::c_int as
                                                               std::os::raw::c_ulong))
            as *mut std::os::raw::c_char;
    if new.is_null() { return 0 as std::os::raw::c_int }
    memcpy(new.offset((*val).l as isize) as *mut std::os::raw::c_void,
           (*v).d as *const std::os::raw::c_void,
           (*v).l.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
    (*val).d = new;
    (*val).l =
        ((*val).l as std::os::raw::c_ulong).wrapping_add((*v).l) as size_t as size_t;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_value(mut val: * mut crate::lil::_lil_value_t) {
    if val.is_null() { return }
    free((*val).d as *mut std::os::raw::c_void);
    free(val as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_list() -> * mut crate::lil::_lil_list_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    list =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<_lil_list_t>() as std::os::raw::c_ulong) as
            lil_list_t;
    (*list).v = 0 as *mut lil_value_t;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_list(mut list: * mut crate::lil::_lil_list_t) {
    let mut i: u64 = 0;
    if list.is_null() { return }
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*list).c {
        lil_free_value(*(*list).v.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*list).v as *mut std::os::raw::c_void);
    free(list as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_append(mut list: * mut crate::lil::_lil_list_t,
                                         mut val: * mut crate::lil::_lil_value_t) {
    if (*list).c == (*list).cap {
        let mut cap: u64 =
            if (*list).cap != 0 {
                (*list).cap.wrapping_add((*list).cap.wrapping_div(2 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulong))
            } else { 32 as std::os::raw::c_int as std::os::raw::c_ulong };
        let mut nv: * mut * mut crate::lil::_lil_value_t =
            realloc((*list).v as *mut std::os::raw::c_void,
                    (::std::mem::size_of::<lil_value_t>() as
                         std::os::raw::c_ulong).wrapping_mul(cap)) as
                *mut lil_value_t;
        if nv.is_null() { return }
        (*list).cap = cap;
        (*list).v = nv
    }
    let mut fresh5 = (*list).c;
    (*list).c = (*list).c.wrapping_add(1);
    let ref mut fresh6 = *(*list).v.offset(fresh5 as isize);
    *fresh6 = val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_size(mut list: * mut crate::lil::_lil_list_t) -> std::os::raw::c_ulong {
    return (*list).c;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_get(mut list: * mut crate::lil::_lil_list_t, mut index: std::os::raw::c_ulong)
 -> * mut crate::lil::_lil_value_t {
    return if index >= (*list).c {
               0 as lil_value_t
           } else { *(*list).v.offset(index as isize) };
}
unsafe extern "C" fn needs_escape(mut str: * const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: u64 = 0;
    if str.is_null() || *str.offset(0 as std::os::raw::c_int as isize) == 0 {
        return 1 as std::os::raw::c_int
    }
    i = 0 as std::os::raw::c_int as size_t;
    while *str.offset(i as isize) != 0 {
        if ispunct(*str.offset(i as isize) as std::os::raw::c_int) != 0 ||
               isspace(*str.offset(i as isize) as std::os::raw::c_int) != 0 {
            return 1 as std::os::raw::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_to_value(mut list: * mut crate::lil::_lil_list_t,
                                           mut do_escape: std::os::raw::c_int)
 -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t = alloc_value(0 as *const std::os::raw::c_char);
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*list).c {
        let mut escape: i32 =
            if do_escape != 0 {
                needs_escape(lil_to_string(*(*list).v.offset(i as isize)))
            } else { 0 as std::os::raw::c_int };
        if i != 0 { lil_append_char(val, ' ' as i32 as std::os::raw::c_char); }
        if escape != 0 {
            lil_append_char(val, '{' as i32 as std::os::raw::c_char);
            j = 0 as std::os::raw::c_int as size_t;
            while j < (**(*list).v.offset(i as isize)).l {
                if *(**(*list).v.offset(i as isize)).d.offset(j as isize) as
                       std::os::raw::c_int == '{' as i32 {
                    lil_append_string(val,
                                      b"}\"\\o\"{\x00" as *const u8 as
                                          *const std::os::raw::c_char);
                } else if *(**(*list).v.offset(i as
                                                   isize)).d.offset(j as
                                                                        isize)
                              as std::os::raw::c_int == '}' as i32 {
                    lil_append_string(val,
                                      b"}\"\\c\"{\x00" as *const u8 as
                                          *const std::os::raw::c_char);
                } else {
                    lil_append_char(val,
                                    *(**(*list).v.offset(i as
                                                             isize)).d.offset(j
                                                                                  as
                                                                                  isize));
                }
                j = j.wrapping_add(1)
            }
            lil_append_char(val, '}' as i32 as std::os::raw::c_char);
        } else { lil_append_val(val, *(*list).v.offset(i as isize)); }
        i = i.wrapping_add(1)
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_env(mut parent: * mut crate::lil::_lil_env_t) -> * mut crate::lil::_lil_env_t {
    let mut env: * mut crate::lil::_lil_env_t = 0 as *mut _lil_env_t;
    env =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<_lil_env_t>() as std::os::raw::c_ulong) as
            lil_env_t;
    (*env).parent = parent;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_env(mut env: * mut crate::lil::_lil_env_t) {
    let mut i: u64 = 0;
    if env.is_null() { return }
    lil_free_value((*env).retval);
    hm_destroy(Some(&mut (*env).varmap));
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*env).vars {
        free((**(*env).var.offset(i as isize)).n as *mut std::os::raw::c_void);
        lil_free_value((**(*env).var.offset(i as isize)).v);
        free((**(*env).var.offset(i as isize)).w as *mut std::os::raw::c_void);
        free(*(*env).var.offset(i as isize) as *mut std::os::raw::c_void);
        i = i.wrapping_add(1)
    }
    free((*env).var as *mut std::os::raw::c_void);
    free(env as *mut std::os::raw::c_void);
}
unsafe extern "C" fn lil_find_local_var(mut lil: * mut crate::lil::_lil_t, mut env: * mut crate::lil::_lil_env_t,
                                        mut name: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_var_t {
    return hm_get(Some(&mut (*env).varmap), name) as lil_var_t;
}
unsafe extern "C" fn lil_find_var(mut lil: * mut crate::lil::_lil_t, mut env: * mut crate::lil::_lil_env_t,
                                  mut name: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_var_t {
    let mut r: * mut crate::lil::_lil_var_t = lil_find_local_var(lil, env, name);
    return if !r.is_null() {
               r
           } else if env == (*lil).rootenv {
               0 as lil_var_t
           } else { lil_find_var(lil, (*lil).rootenv, name) };
}
unsafe extern "C" fn find_cmd(mut lil: * mut crate::lil::_lil_t, mut name: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_func_t {
    return hm_get(Some(&mut (*lil).cmdmap), name) as lil_func_t;
}
unsafe extern "C" fn add_func(mut lil: * mut crate::lil::_lil_t, mut name: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_func_t {
    let mut cmd: * mut crate::lil::_lil_func_t = 0 as *mut _lil_func_t;
    let mut ncmd: * mut * mut crate::lil::_lil_func_t = 0 as *mut lil_func_t;
    cmd = find_cmd(lil, name);
    if !cmd.is_null() {
        if !(*cmd).argnames.is_null() { lil_free_list((*cmd).argnames); }
        lil_free_value((*cmd).code);
        (*cmd).argnames = 0 as lil_list_t;
        (*cmd).code = 0 as lil_value_t;
        (*cmd).proc_0 = None;
        return cmd
    }
    cmd =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<_lil_func_t>() as std::os::raw::c_ulong) as
            lil_func_t;
    (*cmd).name = strclone(name);
    ncmd =
        realloc((*lil).cmd as *mut std::os::raw::c_void,
                (::std::mem::size_of::<lil_func_t>() as
                     std::os::raw::c_ulong).wrapping_mul((*lil).cmds.wrapping_add(1 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong)))
            as *mut lil_func_t;
    if ncmd.is_null() {
        free(cmd as *mut std::os::raw::c_void);
        return 0 as lil_func_t
    }
    (*lil).cmd = ncmd;
    let mut fresh7 = (*lil).cmds;
    (*lil).cmds = (*lil).cmds.wrapping_add(1);
    let ref mut fresh8 = *ncmd.offset(fresh7 as isize);
    *fresh8 = cmd;
    hm_put(Some(&mut (*lil).cmdmap), name, cmd as *mut std::os::raw::c_void);
    return cmd;
}
unsafe extern "C" fn del_func(mut lil: * mut crate::lil::_lil_t, mut cmd: * mut crate::lil::_lil_func_t) {
    let mut i: u64 = 0;
    let mut index: u64 = (*lil).cmds;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*lil).cmds {
        if *(*lil).cmd.offset(i as isize) == cmd {
            index = i;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    if index == (*lil).cmds { return }
    hm_put(Some(&mut (*lil).cmdmap), (*cmd).name, 0 as *mut std::os::raw::c_void);
    if !(*cmd).argnames.is_null() { lil_free_list((*cmd).argnames); }
    lil_free_value((*cmd).code);
    free((*cmd).name as *mut std::os::raw::c_void);
    free(cmd as *mut std::os::raw::c_void);
    (*lil).cmds = (*lil).cmds.wrapping_sub(1);
    i = index;
    while i < (*lil).cmds {
        let ref mut fresh9 = *(*lil).cmd.offset(i as isize);
        *fresh9 =
            *(*lil).cmd.offset(i.wrapping_add(1 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong) as isize);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_register(mut lil: * mut crate::lil::_lil_t,
                                      mut name: * const std::os::raw::c_char,
                                      mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: std::os::raw::c_ulong,_: * mut * mut crate::lil::_lil_value_t,) -> * mut crate::lil::_lil_value_t>)
 -> std::os::raw::c_int {
    let mut cmd: * mut crate::lil::_lil_func_t = add_func(lil, name);
    if cmd.is_null() { return 0 as std::os::raw::c_int }
    (*cmd).proc_0 = proc_0;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_var(mut lil: * mut crate::lil::_lil_t,
                                     mut name: * const std::os::raw::c_char,
                                     mut val: * mut crate::lil::_lil_value_t,
                                     mut local: std::os::raw::c_int) -> * mut crate::lil::_lil_var_t {
    let mut nvar: * mut * mut crate::lil::_lil_var_t = 0 as *mut lil_var_t;
    let mut env: * mut crate::lil::_lil_env_t =
        if local == 0 as std::os::raw::c_int { (*lil).rootenv } else { (*lil).env };
    let mut freeval: i32 = 0 as std::os::raw::c_int;
    if *name.offset(0 as std::os::raw::c_int as isize) == 0 { return 0 as lil_var_t }
    if local != 2 as std::os::raw::c_int {
        let mut var: * mut crate::lil::_lil_var_t = lil_find_var(lil, env, name);
        if local == 3 as std::os::raw::c_int && !var.is_null() &&
               (*var).env == (*lil).rootenv && (*var).env != env {
            var = 0 as lil_var_t
        }
        if (var.is_null() && env == (*lil).rootenv ||
                !var.is_null() && (*var).env == (*lil).rootenv) &&
               (*lil).callback[6 as std::os::raw::c_int as usize].is_some() {
            let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,_: * mut * mut crate::lil::_lil_value_t,) -> i32> =
                core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,_: * mut * mut crate::lil::_lil_value_t,) -> i32>>((*lil).callback[6
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        usize]);
            let mut newval: * mut crate::lil::_lil_value_t = val;
            let mut r: i32 =
                proc_0.expect("non-null function pointer")(lil, name,
                                                           &mut newval);
            if r < 0 as std::os::raw::c_int { return 0 as lil_var_t }
            if r != 0 { val = newval; freeval = 1 as std::os::raw::c_int }
        }
        if !var.is_null() {
            lil_free_value((*var).v);
            (*var).v = if freeval != 0 { val } else { lil_clone_value(val) };
            if !(*var).w.is_null() {
                let mut save_env: * mut crate::lil::_lil_env_t = 0 as *mut _lil_env_t;
                save_env = (*lil).env;
                (*lil).env = (*var).env;
                lil_free_value(lil_parse(lil, (*var).w,
                                         0 as std::os::raw::c_int as size_t,
                                         1 as std::os::raw::c_int));
                (*lil).env = save_env
            }
            return var
        }
    }
    nvar =
        realloc((*env).var as *mut std::os::raw::c_void,
                (::std::mem::size_of::<lil_var_t>() as
                     std::os::raw::c_ulong).wrapping_mul((*env).vars.wrapping_add(1 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong)))
            as *mut lil_var_t;
    if nvar.is_null() {
        /* TODO: report memory error */
        return 0 as lil_var_t
    }
    (*env).var = nvar;
    let ref mut fresh10 = *nvar.offset((*env).vars as isize);
    *fresh10 =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<_lil_var_t>() as std::os::raw::c_ulong) as
            lil_var_t;
    let ref mut fresh11 = (**nvar.offset((*env).vars as isize)).n;
    *fresh11 = strclone(name);
    let ref mut fresh12 = (**nvar.offset((*env).vars as isize)).w;
    *fresh12 = 0 as *mut std::os::raw::c_char;
    let ref mut fresh13 = (**nvar.offset((*env).vars as isize)).env;
    *fresh13 = env;
    let ref mut fresh14 = (**nvar.offset((*env).vars as isize)).v;
    *fresh14 = if freeval != 0 { val } else { lil_clone_value(val) };
    hm_put(Some(&mut (*env).varmap), name,
           *nvar.offset((*env).vars as isize) as *mut std::os::raw::c_void);
    let mut fresh15 = (*env).vars;
    (*env).vars = (*env).vars.wrapping_add(1);
    return *nvar.offset(fresh15 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_var(mut lil: * mut crate::lil::_lil_t,
                                     mut name: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_value_t {
    return lil_get_var_or(lil, name, (*lil).empty);
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_var_or(mut lil: * mut crate::lil::_lil_t,
                                        mut name: * const std::os::raw::c_char,
                                        mut defvalue: * mut crate::lil::_lil_value_t)
 -> * mut crate::lil::_lil_value_t {
    let mut var: * mut crate::lil::_lil_var_t = lil_find_var(lil, (*lil).env, name);
    let mut retval: * mut crate::lil::_lil_value_t =
        if !var.is_null() { (*var).v } else { defvalue };
    if (*lil).callback[7 as std::os::raw::c_int as usize].is_some() &&
           (var.is_null() || (*var).env == (*lil).rootenv) {
        let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,_: * mut * mut crate::lil::_lil_value_t,) -> i32> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,_: * mut * mut crate::lil::_lil_value_t,) -> i32>>((*lil).callback[7
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    usize]);
        let mut newretval: * mut crate::lil::_lil_value_t = retval;
        if proc_0.expect("non-null function pointer")(lil, name,
                                                      &mut newretval) != 0 {
            retval = newretval
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn lil_push_env(mut lil: * mut crate::lil::_lil_t) -> * mut crate::lil::_lil_env_t {
    let mut env: * mut crate::lil::_lil_env_t = lil_alloc_env((*lil).env);
    (*lil).env = env;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn lil_pop_env(mut lil: * mut crate::lil::_lil_t) {
    if !(*(*lil).env).parent.is_null() {
        let mut next: * mut crate::lil::_lil_env_t = (*(*lil).env).parent;
        lil_free_env((*lil).env);
        (*lil).env = next
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_new() -> * mut crate::lil::_lil_t {
    let mut lil: * mut crate::lil::_lil_t =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<_lil_t>() as std::os::raw::c_ulong) as lil_t;
    (*lil).env = lil_alloc_env(0 as lil_env_t);
    (*lil).rootenv = (*lil).env;
    (*lil).empty = alloc_value(0 as *const std::os::raw::c_char);
    (*lil).dollarprefix =
        strclone(b"set \x00" as *const u8 as *const std::os::raw::c_char);
    hm_init(&mut (*lil).cmdmap);
    register_stdcmds(lil);
    return lil;
}
 extern "C" fn islilspecial(mut ch: std::os::raw::c_char) -> std::os::raw::c_int {
    return (ch as std::os::raw::c_int == '$' as i32 || ch as std::os::raw::c_int == '{' as i32
                || ch as std::os::raw::c_int == '}' as i32 ||
                ch as std::os::raw::c_int == '[' as i32 ||
                ch as std::os::raw::c_int == ']' as i32 ||
                ch as std::os::raw::c_int == '\"' as i32 ||
                ch as std::os::raw::c_int == '\'' as i32 ||
                ch as std::os::raw::c_int == ';' as i32) as std::os::raw::c_int;
}
unsafe extern "C" fn ateol(mut lil: * mut crate::lil::_lil_t) -> std::os::raw::c_int {
    return ((*lil).ignoreeol == 0 &&
                (*(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                     '\n' as i32 ||
                     *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int
                         == '\r' as i32 ||
                     *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int
                         == ';' as i32)) as std::os::raw::c_int;
}
unsafe extern "C" fn skip_spaces(mut lil: * mut crate::lil::_lil_t) {
    while (*lil).head < (*lil).clen {
        if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
               '#' as i32 {
            if *(*lil).code.offset((*lil).head.wrapping_add(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_ulong)
                                       as isize) as std::os::raw::c_int == '#' as i32
                   &&
                   *(*lil).code.offset((*lil).head.wrapping_add(2 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                           as isize) as std::os::raw::c_int !=
                       '#' as i32 {
                (*lil).head =
                    ((*lil).head as
                         std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        size_t as size_t;
                while (*lil).head < (*lil).clen {
                    if *(*lil).code.offset((*lil).head as isize) as
                           std::os::raw::c_int == '#' as i32 &&
                           *(*lil).code.offset((*lil).head.wrapping_add(1 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong)
                                                   as isize) as std::os::raw::c_int ==
                               '#' as i32 &&
                           *(*lil).code.offset((*lil).head.wrapping_add(2 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong)
                                                   as isize) as std::os::raw::c_int !=
                               '#' as i32 {
                        (*lil).head =
                            ((*lil).head as
                                 std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int
                                                                 as
                                                                 std::os::raw::c_ulong)
                                as size_t as size_t;
                        break ;
                    } else { (*lil).head = (*lil).head.wrapping_add(1) }
                }
            } else {
                while (*lil).head < (*lil).clen && ateol(lil) == 0 {
                    (*lil).head = (*lil).head.wrapping_add(1)
                }
            }
        } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                      '\\' as i32 &&
                      (*(*lil).code.offset((*lil).head.wrapping_add(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_ulong)
                                               as isize) as std::os::raw::c_int ==
                           '\r' as i32 ||
                           *(*lil).code.offset((*lil).head.wrapping_add(1 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong)
                                                   as isize) as std::os::raw::c_int ==
                               '\n' as i32) {
            (*lil).head = (*lil).head.wrapping_add(1);
            while (*lil).head < (*lil).clen && ateol(lil) != 0 {
                (*lil).head = (*lil).head.wrapping_add(1)
            }
        } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                      '\r' as i32 ||
                      *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int
                          == '\n' as i32 {
            if !((*lil).ignoreeol != 0) { break ; }
            (*lil).head = (*lil).head.wrapping_add(1)
        } else {
            if !(isspace(*(*lil).code.offset((*lil).head as isize) as
                             std::os::raw::c_int) != 0) {
                break ;
            }
            (*lil).head = (*lil).head.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn get_bracketpart(mut lil: * mut crate::lil::_lil_t) -> * mut crate::lil::_lil_value_t {
    let mut cnt: u64 = 1 as std::os::raw::c_int as size_t;
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut cmd: * mut crate::lil::_lil_value_t = alloc_value(0 as *const std::os::raw::c_char);
    (*lil).head = (*lil).head.wrapping_add(1);
    while (*lil).head < (*lil).clen {
        if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
               '[' as i32 {
            (*lil).head = (*lil).head.wrapping_add(1);
            cnt = cnt.wrapping_add(1);
            lil_append_char(cmd, '[' as i32 as std::os::raw::c_char);
        } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                      ']' as i32 {
            (*lil).head = (*lil).head.wrapping_add(1);
            cnt = cnt.wrapping_sub(1);
            if cnt == 0 as std::os::raw::c_int as std::os::raw::c_ulong { break ; }
            lil_append_char(cmd, ']' as i32 as std::os::raw::c_char);
        } else {
            let mut fresh16 = (*lil).head;
            (*lil).head = (*lil).head.wrapping_add(1);
            lil_append_char(cmd, *(*lil).code.offset(fresh16 as isize));
        }
    }
    val = lil_parse_value(lil, cmd, 0 as std::os::raw::c_int);
    lil_free_value(cmd);
    return val;
}
unsafe extern "C" fn get_dollarpart(mut lil: * mut crate::lil::_lil_t) -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut name: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut tmp: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    (*lil).head = (*lil).head.wrapping_add(1);
    name = next_word(lil);
    tmp = alloc_value((*lil).dollarprefix);
    lil_append_val(tmp, name);
    lil_free_value(name);
    val = lil_parse_value(lil, tmp, 0 as std::os::raw::c_int);
    lil_free_value(tmp);
    return val;
}
unsafe extern "C" fn next_word(mut lil: * mut crate::lil::_lil_t) -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut start: u64 = 0;
    skip_spaces(lil);
    if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int == '$' as i32
       {
        val = get_dollarpart(lil)
    } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                  '{' as i32 {
        let mut cnt: u64 = 1 as std::os::raw::c_int as size_t;
        (*lil).head = (*lil).head.wrapping_add(1);
        val = alloc_value(0 as *const std::os::raw::c_char);
        while (*lil).head < (*lil).clen {
            if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                   '{' as i32 {
                (*lil).head = (*lil).head.wrapping_add(1);
                cnt = cnt.wrapping_add(1);
                lil_append_char(val, '{' as i32 as std::os::raw::c_char);
            } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int
                          == '}' as i32 {
                (*lil).head = (*lil).head.wrapping_add(1);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 as std::os::raw::c_int as std::os::raw::c_ulong { break ; }
                lil_append_char(val, '}' as i32 as std::os::raw::c_char);
            } else {
                let mut fresh17 = (*lil).head;
                (*lil).head = (*lil).head.wrapping_add(1);
                lil_append_char(val, *(*lil).code.offset(fresh17 as isize));
            }
        }
    } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                  '[' as i32 {
        val = get_bracketpart(lil)
    } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                  '\"' as i32 ||
                  *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                      '\'' as i32 {
        let mut fresh18 = (*lil).head;
        (*lil).head = (*lil).head.wrapping_add(1);
        let mut sc: i8 = *(*lil).code.offset(fresh18 as isize);
        val = alloc_value(0 as *const std::os::raw::c_char);
        while (*lil).head < (*lil).clen {
            if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                   '[' as i32 ||
                   *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int ==
                       '$' as i32 {
                let mut tmp: * mut crate::lil::_lil_value_t =
                    if *(*lil).code.offset((*lil).head as isize) as
                           std::os::raw::c_int == '$' as i32 {
                        get_dollarpart(lil)
                    } else { get_bracketpart(lil) };
                lil_append_val(val, tmp);
                lil_free_value(tmp);
                (*lil).head = (*lil).head.wrapping_sub(1)
                /* avoid skipping the char below */
            } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int
                          == '\\' as i32 {
                (*lil).head = (*lil).head.wrapping_add(1);
                match *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int
                    {
                    98 => {
                        lil_append_char(val, '\u{8}' as i32 as std::os::raw::c_char);
                    }
                    116 => {
                        lil_append_char(val, '\t' as i32 as std::os::raw::c_char);
                    }
                    110 => {
                        lil_append_char(val, '\n' as i32 as std::os::raw::c_char);
                    }
                    118 => {
                        lil_append_char(val, '\u{b}' as i32 as std::os::raw::c_char);
                    }
                    102 => {
                        lil_append_char(val, '\u{c}' as i32 as std::os::raw::c_char);
                    }
                    114 => {
                        lil_append_char(val, '\r' as i32 as std::os::raw::c_char);
                    }
                    48 => {
                        lil_append_char(val,
                                        0 as std::os::raw::c_int as std::os::raw::c_char);
                    }
                    97 => {
                        lil_append_char(val, '\u{7}' as i32 as std::os::raw::c_char);
                    }
                    99 => {
                        lil_append_char(val, '}' as i32 as std::os::raw::c_char);
                    }
                    111 => {
                        lil_append_char(val, '{' as i32 as std::os::raw::c_char);
                    }
                    _ => {
                        lil_append_char(val,
                                        *(*lil).code.offset((*lil).head as
                                                                isize));
                    }
                }
            } else if *(*lil).code.offset((*lil).head as isize) as std::os::raw::c_int
                          == sc as std::os::raw::c_int {
                (*lil).head = (*lil).head.wrapping_add(1);
                break ;
            } else {
                lil_append_char(val,
                                *(*lil).code.offset((*lil).head as isize));
            }
            (*lil).head = (*lil).head.wrapping_add(1)
        }
    } else {
        start = (*lil).head;
        while (*lil).head < (*lil).clen &&
                  isspace(*(*lil).code.offset((*lil).head as isize) as
                              std::os::raw::c_int) == 0 &&
                  islilspecial(*(*lil).code.offset((*lil).head as isize)) == 0
              {
            (*lil).head = (*lil).head.wrapping_add(1)
        }
        val =
            alloc_value_len((*lil).code.offset(start as isize),
                            (*lil).head.wrapping_sub(start))
    }
    return if !val.is_null() {
               val
           } else { alloc_value(0 as *const std::os::raw::c_char) };
}
unsafe extern "C" fn substitute(mut lil: * mut crate::lil::_lil_t) -> * mut crate::lil::_lil_list_t {
    let mut words: * mut crate::lil::_lil_list_t = lil_alloc_list();
    skip_spaces(lil);
    while (*lil).head < (*lil).clen && ateol(lil) == 0 && (*lil).error == 0 {
        let mut w: * mut crate::lil::_lil_value_t = alloc_value(0 as *const std::os::raw::c_char);
        loop  {
            let mut head: u64 = (*lil).head;
            let mut wp: * mut crate::lil::_lil_value_t = next_word(lil);
            if head == (*lil).head {
                /* something wrong, the parser can't proceed */
                lil_free_value(w);
                lil_free_value(wp);
                lil_free_list(words);
                return 0 as lil_list_t
            }
            lil_append_val(w, wp);
            lil_free_value(wp);
            if !((*lil).head < (*lil).clen && ateol(lil) == 0 &&
                     isspace(*(*lil).code.offset((*lil).head as isize) as
                                 std::os::raw::c_int) == 0 && (*lil).error == 0) {
                break ;
            }
        }
        skip_spaces(lil);
        lil_list_append(words, w);
    }
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn lil_subst_to_list(mut lil: * mut crate::lil::_lil_t,
                                           mut code: * mut crate::lil::_lil_value_t)
 -> * mut crate::lil::_lil_list_t {
    let mut save_code: * const i8 = (*lil).code;
    let mut save_clen: u64 = (*lil).clen;
    let mut save_head: u64 = (*lil).head;
    let mut save_igeol: i32 = (*lil).ignoreeol;
    let mut words: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    (*lil).code = lil_to_string(code);
    (*lil).clen = (*code).l;
    (*lil).head = 0 as std::os::raw::c_int as size_t;
    (*lil).ignoreeol = 1 as std::os::raw::c_int;
    words = substitute(lil);
    if words.is_null() { words = lil_alloc_list() }
    (*lil).code = save_code;
    (*lil).clen = save_clen;
    (*lil).head = save_head;
    (*lil).ignoreeol = save_igeol;
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn lil_subst_to_value(mut lil: * mut crate::lil::_lil_t,
                                            mut code: * mut crate::lil::_lil_value_t)
 -> * mut crate::lil::_lil_value_t {
    let mut words: * mut crate::lil::_lil_list_t = lil_subst_to_list(lil, code);
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    val = lil_list_to_value(words, 0 as std::os::raw::c_int);
    lil_free_list(words);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_parse(mut lil: * mut crate::lil::_lil_t,
                                   mut code: * const std::os::raw::c_char,
                                   mut codelen: std::os::raw::c_ulong,
                                   mut funclevel: std::os::raw::c_int)
 -> * mut crate::lil::_lil_value_t {
    let mut save_code: * const i8 = (*lil).code;
    let mut save_clen: u64 = (*lil).clen;
    let mut save_head: u64 = (*lil).head;
    let mut val: * mut crate::lil::_lil_value_t = 0 as lil_value_t;
    let mut words: * mut crate::lil::_lil_list_t = 0 as lil_list_t;
    if save_code.is_null() { (*lil).rootcode = code }
    (*lil).code = code;
    (*lil).clen = if codelen != 0 { codelen } else { strlen(code) };
    (*lil).head = 0 as std::os::raw::c_int as size_t;
    skip_spaces(lil);
    (*lil).parse_depth = (*lil).parse_depth.wrapping_add(1);
    if (*lil).parse_depth == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        (*lil).error = 0 as std::os::raw::c_int
    }
    if funclevel != 0 { (*(*lil).env).breakrun = 0 as std::os::raw::c_int }
    while (*lil).head < (*lil).clen && (*lil).error == 0 {
        if !words.is_null() { lil_free_list(words); }
        if !val.is_null() { lil_free_value(val); }
        val = 0 as lil_value_t;
        words = substitute(lil);
        if words.is_null() || (*lil).error != 0 { break ; }
        if (*words).c != 0 {
            let mut cmd: * mut crate::lil::_lil_func_t =
                find_cmd(lil,
                         lil_to_string(*(*words).v.offset(0 as std::os::raw::c_int as
                                                              isize)));
            if cmd.is_null() {
                if (**(*words).v.offset(0 as std::os::raw::c_int as isize)).l != 0 {
                    if !(*lil).catcher.is_null() {
                        if (*lil).in_catcher < 16384 as std::os::raw::c_int {
                            let mut args: * mut crate::lil::_lil_value_t =
                                0 as *mut _lil_value_t;
                            (*lil).in_catcher += 1;
                            lil_push_env(lil);
                            (*(*lil).env).catcher_for =
                                *(*words).v.offset(0 as std::os::raw::c_int as isize);
                            args = lil_list_to_value(words, 1 as std::os::raw::c_int);
                            lil_set_var(lil,
                                        b"args\x00" as *const u8 as
                                            *const std::os::raw::c_char, args,
                                        2 as std::os::raw::c_int);
                            lil_free_value(args);
                            val =
                                lil_parse(lil, (*lil).catcher,
                                          0 as std::os::raw::c_int as size_t,
                                          1 as std::os::raw::c_int);
                            lil_pop_env(lil);
                            (*lil).in_catcher -= 1
                        } else {
                            let mut msg: * mut i8 =
                                malloc((**(*words).v.offset(0 as std::os::raw::c_int
                                                                as
                                                                isize)).l.wrapping_add(64
                                                                                           as
                                                                                           std::os::raw::c_int
                                                                                           as
                                                                                           std::os::raw::c_ulong))
                                    as *mut std::os::raw::c_char;
                            sprintf(msg,
                                    b"catcher limit reached while trying to call unknown function %s\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    (**(*words).v.offset(0 as std::os::raw::c_int as
                                                             isize)).d);
                            lil_set_error_at(lil, (*lil).head, msg);
                            free(msg as *mut std::os::raw::c_void);
                            break ;
                        }
                    } else {
                        let mut msg_0: * mut i8 =
                            malloc((**(*words).v.offset(0 as std::os::raw::c_int as
                                                            isize)).l.wrapping_add(32
                                                                                       as
                                                                                       std::os::raw::c_int
                                                                                       as
                                                                                       std::os::raw::c_ulong))
                                as *mut std::os::raw::c_char;
                        sprintf(msg_0,
                                b"unknown function %s\x00" as *const u8 as
                                    *const std::os::raw::c_char,
                                (**(*words).v.offset(0 as std::os::raw::c_int as
                                                         isize)).d);
                        lil_set_error_at(lil, (*lil).head, msg_0);
                        free(msg_0 as *mut std::os::raw::c_void);
                        break ;
                    }
                }
            }
            if !cmd.is_null() {
                if (*cmd).proc_0.is_some() {
                    let mut shead: u64 = (*lil).head;
                    val =
                        (*cmd).proc_0.expect("non-null function pointer")(lil,
                                                                          (*words).c.wrapping_sub(1
                                                                                                      as
                                                                                                      std::os::raw::c_int
                                                                                                      as
                                                                                                      std::os::raw::c_ulong),
                                                                          (*words).v.offset(1
                                                                                                as
                                                                                                std::os::raw::c_int
                                                                                                as
                                                                                                isize));
                    if (*lil).error == 2 as std::os::raw::c_int {
                        (*lil).error = 1 as std::os::raw::c_int;
                        (*lil).err_head = shead
                    }
                } else {
                    lil_push_env(lil);
                    (*(*lil).env).func = cmd;
                    if (*(*cmd).argnames).c ==
                           1 as std::os::raw::c_int as std::os::raw::c_ulong &&
                           strcmp(lil_to_string(*(*(*cmd).argnames).v.offset(0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 isize)),
                                  b"args\x00" as *const u8 as
                                      *const std::os::raw::c_char) == 0 {
                        let mut args_0: * mut crate::lil::_lil_value_t =
                            lil_list_to_value(words, 1 as std::os::raw::c_int);
                        lil_set_var(lil,
                                    b"args\x00" as *const u8 as
                                        *const std::os::raw::c_char, args_0,
                                    2 as std::os::raw::c_int);
                        lil_free_value(args_0);
                    } else {
                        let mut i: u64 = 0;
                        i = 0 as std::os::raw::c_int as size_t;
                        while i < (*(*cmd).argnames).c {
                            lil_set_var(lil,
                                        lil_to_string(*(*(*cmd).argnames).v.offset(i
                                                                                       as
                                                                                       isize)),
                                        if i <
                                               (*words).c.wrapping_sub(1 as
                                                                           std::os::raw::c_int
                                                                           as
                                                                           std::os::raw::c_ulong)
                                           {
                                            *(*words).v.offset(i.wrapping_add(1
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulong)
                                                                   as isize)
                                        } else { (*lil).empty },
                                        2 as std::os::raw::c_int);
                            i = i.wrapping_add(1)
                        }
                    }
                    val = lil_parse_value(lil, (*cmd).code, 1 as std::os::raw::c_int);
                    lil_pop_env(lil);
                }
            }
        }
        if (*(*lil).env).breakrun != 0 { break ; }
        skip_spaces(lil);
        while ateol(lil) != 0 { (*lil).head = (*lil).head.wrapping_add(1) }
        skip_spaces(lil);
    }
    if (*lil).error != 0 &&
           (*lil).callback[5 as std::os::raw::c_int as usize].is_some() &&
           (*lil).parse_depth == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: u64,_: * const i8,) -> ()> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: u64,_: * const i8,) -> ()>>((*lil).callback[5
                                                                                   as
                                                                                   std::os::raw::c_int
                                                                                   as
                                                                                   usize]);
        proc_0.expect("non-null function pointer")(lil, (*lil).err_head,
                                                   (*lil).err_msg);
    }
    if !words.is_null() { lil_free_list(words); }
    (*lil).code = save_code;
    (*lil).clen = save_clen;
    (*lil).head = save_head;
    if funclevel != 0 && (*(*lil).env).retval_set != 0 {
        if !val.is_null() { lil_free_value(val); }
        val = (*(*lil).env).retval;
        (*(*lil).env).retval = 0 as lil_value_t;
        (*(*lil).env).retval_set = 0 as std::os::raw::c_int;
        (*(*lil).env).breakrun = 0 as std::os::raw::c_int
    }
    (*lil).parse_depth = (*lil).parse_depth.wrapping_sub(1);
    return if !val.is_null() {
               val
           } else { alloc_value(0 as *const std::os::raw::c_char) };
}
#[no_mangle]
pub unsafe extern "C" fn lil_parse_value(mut lil: * mut crate::lil::_lil_t, mut val: * mut crate::lil::_lil_value_t,
                                         mut funclevel: std::os::raw::c_int)
 -> * mut crate::lil::_lil_value_t {
    if val.is_null() || (*val).d.is_null() || (*val).l == 0 {
        return alloc_value(0 as *const std::os::raw::c_char)
    }
    return lil_parse(lil, (*val).d, (*val).l, funclevel);
}
#[no_mangle]
pub unsafe extern "C" fn lil_call(mut lil: * mut crate::lil::_lil_t,
                                  mut funcname: * const std::os::raw::c_char,
                                  mut argc: std::os::raw::c_ulong,
                                  mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut cmd: * mut crate::lil::_lil_func_t = find_cmd(lil, funcname);
    let mut r: * mut crate::lil::_lil_value_t = 0 as lil_value_t;
    if !cmd.is_null() {
        if (*cmd).proc_0.is_some() {
            r =
                (*cmd).proc_0.expect("non-null function pointer")(lil, argc,
                                                                  argv)
        } else {
            let mut i: u64 = 0;
            lil_push_env(lil);
            (*(*lil).env).func = cmd;
            if (*(*cmd).argnames).c == 1 as std::os::raw::c_int as std::os::raw::c_ulong &&
                   strcmp(lil_to_string(*(*(*cmd).argnames).v.offset(0 as
                                                                         std::os::raw::c_int
                                                                         as
                                                                         isize)),
                          b"args\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 {
                let mut args: * mut crate::lil::_lil_list_t = lil_alloc_list();
                let mut argsval: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
                i = 0 as std::os::raw::c_int as size_t;
                while i < argc {
                    lil_list_append(args,
                                    lil_clone_value(*argv.offset(i as
                                                                     isize)));
                    i = i.wrapping_add(1)
                }
                argsval = lil_list_to_value(args, 0 as std::os::raw::c_int);
                lil_set_var(lil,
                            b"args\x00" as *const u8 as *const std::os::raw::c_char,
                            argsval, 2 as std::os::raw::c_int);
                lil_free_value(argsval);
                lil_free_list(args);
            } else {
                i = 0 as std::os::raw::c_int as size_t;
                while i < (*(*cmd).argnames).c {
                    lil_set_var(lil,
                                lil_to_string(*(*(*cmd).argnames).v.offset(i
                                                                               as
                                                                               isize)),
                                if i < argc {
                                    *argv.offset(i as isize)
                                } else { 0 as lil_value_t },
                                2 as std::os::raw::c_int);
                    i = i.wrapping_add(1)
                }
            }
            r = lil_parse_value(lil, (*cmd).code, 1 as std::os::raw::c_int);
            lil_pop_env(lil);
        }
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn lil_callback(mut lil: * mut crate::lil::_lil_t, mut cb: std::os::raw::c_int,
                                      mut proc_0: Option<unsafe extern "C"  fn() -> ()>) {
    if cb < 0 as std::os::raw::c_int || cb > 8 as std::os::raw::c_int { return }
    (*lil).callback[cb as usize] = proc_0;
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_error(mut lil: * mut crate::lil::_lil_t,
                                       mut msg: * const std::os::raw::c_char) {
    if (*lil).error != 0 { return }
    free((*lil).err_msg as *mut std::os::raw::c_void);
    (*lil).error = 2 as std::os::raw::c_int;
    (*lil).err_head = 0 as std::os::raw::c_int as size_t;
    (*lil).err_msg =
        strclone(if !msg.is_null() {
                     msg
                 } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_error_at(mut lil: * mut crate::lil::_lil_t, mut pos: std::os::raw::c_ulong,
                                          mut msg: * const std::os::raw::c_char) {
    if (*lil).error != 0 { return }
    free((*lil).err_msg as *mut std::os::raw::c_void);
    (*lil).error = 1 as std::os::raw::c_int;
    (*lil).err_head = pos;
    (*lil).err_msg =
        strclone(if !msg.is_null() {
                     msg
                 } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
}
#[no_mangle]
pub unsafe extern "C" fn lil_error<'a1, 'a2>(mut lil: * mut crate::lil::_lil_t,
                                   mut msg: Option<&'a1 mut * const std::os::raw::c_char>,
                                   mut pos: Option<&'a2 mut std::os::raw::c_ulong>) -> std::os::raw::c_int {
    if (*lil).error == 0 { return 0 as std::os::raw::c_int }
    *(borrow_mut(&mut msg)).unwrap() = (*lil).err_msg;
    *(borrow_mut(&mut pos)).unwrap() = (*lil).err_head;
    (*lil).error = 0 as std::os::raw::c_int;
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn ee_invalidpunct(mut ch: std::os::raw::c_int) -> std::os::raw::c_int {
    return (ispunct(ch) != 0 && ch != '!' as i32 && ch != '~' as i32 &&
                ch != '(' as i32 && ch != ')' as i32 && ch != '-' as i32 &&
                ch != '+' as i32) as std::os::raw::c_int;
}
unsafe extern "C" fn ee_skip_spaces(mut ee: * mut crate::lil::_expreval_t) {
    while (*ee).head < (*ee).len &&
              isspace(*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int)
                  != 0 {
        (*ee).head = (*ee).head.wrapping_add(1)
    };
}
unsafe extern "C" fn ee_numeric_element(mut ee: * mut crate::lil::_expreval_t) {
    let mut fpart: i64 = 0 as std::os::raw::c_int as lilint_t;
    let mut fpartlen: i64 = 1 as std::os::raw::c_int as lilint_t;
    (*ee).type_0 = 0 as std::os::raw::c_int;
    ee_skip_spaces(ee);
    (*ee).ival = 0 as std::os::raw::c_int as lilint_t;
    (*ee).dval = 0 as std::os::raw::c_int as std::os::raw::c_double;
    while (*ee).head < (*ee).len {
        if *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
               '.' as i32 {
            if (*ee).type_0 == 1 as std::os::raw::c_int { break ; }
            (*ee).type_0 = 1 as std::os::raw::c_int;
            (*ee).head = (*ee).head.wrapping_add(1)
        } else if isdigit(*(*ee).code.offset((*ee).head as isize) as
                              std::os::raw::c_int) == 0 {
            break ;
        }
        if (*ee).type_0 == 0 as std::os::raw::c_int {
            (*ee).ival =
                (*ee).ival * 10 as std::os::raw::c_int as std::os::raw::c_longlong +
                    (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int -
                         '0' as i32) as std::os::raw::c_longlong
        } else {
            fpart =
                fpart * 10 as std::os::raw::c_int as std::os::raw::c_longlong +
                    (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int -
                         '0' as i32) as std::os::raw::c_longlong;
            fpartlen *= 10 as std::os::raw::c_int as std::os::raw::c_longlong
        }
        (*ee).head = (*ee).head.wrapping_add(1)
    }
    if (*ee).type_0 == 1 as std::os::raw::c_int {
        (*ee).dval =
            (*ee).ival as std::os::raw::c_double +
                fpart as std::os::raw::c_double / fpartlen as std::os::raw::c_double
    };
}
unsafe extern "C" fn ee_element(mut ee: * mut crate::lil::_expreval_t) {
    if isdigit(*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int) != 0 {
        ee_numeric_element(ee);
        return
    }
    /* for anything else that might creep in (usually from strings), we set the
     * value to 1 so that strings evaluate as "true" when used in conditional
     * expressions */
    (*ee).type_0 = 0 as std::os::raw::c_int;
    (*ee).ival = 1 as std::os::raw::c_int as lilint_t;
    (*ee).error = 4 as std::os::raw::c_int;
    /* special flag, will be cleared */
}
unsafe extern "C" fn ee_paren(mut ee: * mut crate::lil::_expreval_t) {
    ee_skip_spaces(ee);
    if *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int == '(' as i32 {
        (*ee).head = (*ee).head.wrapping_add(1);
        ee_expr(ee);
        ee_skip_spaces(ee);
        if *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
               ')' as i32 {
            (*ee).head = (*ee).head.wrapping_add(1)
        } else { (*ee).error = 1 as std::os::raw::c_int }
    } else { ee_element(ee); };
}
unsafe extern "C" fn ee_unary(mut ee: * mut crate::lil::_expreval_t) {
    ee_skip_spaces(ee);
    if (*ee).head < (*ee).len && (*ee).error == 0 &&
           (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                '-' as i32 ||
                *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                    '+' as i32 ||
                *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                    '~' as i32 ||
                *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                    '!' as i32) {
        let mut fresh19 = (*ee).head;
        (*ee).head = (*ee).head.wrapping_add(1);
        let mut op: i8 = *(*ee).code.offset(fresh19 as isize);
        ee_unary(ee);
        if (*ee).error != 0 { return }
        match op as std::os::raw::c_int {
            45 => {
                match (*ee).type_0 {
                    1 => { (*ee).dval = -(*ee).dval }
                    0 => { (*ee).ival = -(*ee).ival }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            126 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival = !((*ee).dval as lilint_t);
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => { (*ee).ival = !(*ee).ival }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            33 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).dval =
                            ((*ee).dval == 0.) as std::os::raw::c_int as
                                std::os::raw::c_double
                    }
                    0 => {
                        (*ee).ival =
                            ((*ee).ival == 0) as std::os::raw::c_int as lilint_t
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            43 | _ => { }
        }
    } else { ee_paren(ee); };
}
unsafe extern "C" fn ee_muldiv(mut ee: * mut crate::lil::_expreval_t) {
    ee_unary(ee);
    if (*ee).error != 0 { return }
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_ulong)
                                                     as isize) as std::os::raw::c_int)
                  == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '*' as i32 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '/' as i32 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '\\' as i32 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '%' as i32) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        match *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int {
            42 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => { (*ee).dval = (*ee).dval * odval }
                            0 => {
                                (*ee).dval =
                                    (*ee).ival as std::os::raw::c_double * odval;
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval =
                                    (*ee).dval * oival as std::os::raw::c_double;
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            0 => { (*ee).ival = (*ee).ival * oival }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            37 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).dval = fmod(odval, (*ee).dval)
                                }
                            }
                            0 => {
                                if (*ee).ival ==
                                       0 as std::os::raw::c_int as std::os::raw::c_longlong {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).dval =
                                        fmod(odval,
                                             (*ee).ival as std::os::raw::c_double)
                                }
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).dval =
                                        fmod(oival as std::os::raw::c_double,
                                             (*ee).dval)
                                }
                            }
                            0 => {
                                if (*ee).ival ==
                                       0 as std::os::raw::c_int as std::os::raw::c_longlong {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else { (*ee).ival = oival % (*ee).ival }
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { }
                }
            }
            47 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else { (*ee).dval = odval / (*ee).dval }
                            }
                            0 => {
                                if (*ee).ival ==
                                       0 as std::os::raw::c_int as std::os::raw::c_longlong {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).dval =
                                        odval / (*ee).ival as std::os::raw::c_double
                                }
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).dval =
                                        oival as std::os::raw::c_double / (*ee).dval
                                }
                            }
                            0 => {
                                if (*ee).ival ==
                                       0 as std::os::raw::c_int as std::os::raw::c_longlong {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).dval =
                                        oival as std::os::raw::c_double /
                                            (*ee).ival as std::os::raw::c_double
                                }
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { }
                }
            }
            92 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).ival =
                                        (odval / (*ee).dval) as lilint_t
                                }
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                if (*ee).ival ==
                                       0 as std::os::raw::c_int as std::os::raw::c_longlong {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).ival =
                                        (odval / (*ee).ival as std::os::raw::c_double)
                                            as lilint_t
                                }
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else {
                                    (*ee).ival =
                                        (oival as std::os::raw::c_double / (*ee).dval)
                                            as lilint_t
                                }
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                if (*ee).ival ==
                                       0 as std::os::raw::c_int as std::os::raw::c_longlong {
                                    (*ee).error = 3 as std::os::raw::c_int
                                } else { (*ee).ival = oival / (*ee).ival }
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_addsub(mut ee: * mut crate::lil::_expreval_t) {
    ee_muldiv(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_ulong)
                                                     as isize) as std::os::raw::c_int)
                  == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '+' as i32 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '-' as i32) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        match *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int {
            43 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => { (*ee).dval = (*ee).dval + odval }
                            0 => {
                                (*ee).dval =
                                    (*ee).ival as std::os::raw::c_double + odval;
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval =
                                    (*ee).dval + oival as std::os::raw::c_double;
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            0 => { (*ee).ival = (*ee).ival + oival }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            45 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => { (*ee).dval = odval - (*ee).dval }
                            0 => {
                                (*ee).dval =
                                    odval - (*ee).ival as std::os::raw::c_double;
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval =
                                    oival as std::os::raw::c_double - (*ee).dval;
                                (*ee).type_0 = 1 as std::os::raw::c_int
                            }
                            0 => { (*ee).ival = oival - (*ee).ival }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_shift(mut ee: * mut crate::lil::_expreval_t) {
    ee_addsub(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '<' as i32 &&
                   *(*ee).code.offset((*ee).head.wrapping_add(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong)
                                          as isize) as std::os::raw::c_int ==
                       '<' as i32 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '>' as i32 &&
                       *(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulong)
                                              as isize) as std::os::raw::c_int ==
                           '>' as i32) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(1);
        match *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int {
            60 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    (odval as lilint_t) <<
                                        (*ee).dval as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival = (odval as lilint_t) << (*ee).ival
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = oival << (*ee).dval as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => { (*ee).ival = oival << (*ee).ival }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            62 => {
                match (*ee).type_0 {
                    1 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    odval as lilint_t >>
                                        (*ee).dval as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival = odval as lilint_t >> (*ee).ival
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        (*ee).head = (*ee).head.wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = oival >> (*ee).dval as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => { (*ee).ival = oival >> (*ee).ival }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_compare(mut ee: * mut crate::lil::_expreval_t) {
    ee_shift(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '<' as i32 &&
                   ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulong)
                                                          as isize) as
                                       std::os::raw::c_int) == 0 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '>' as i32 &&
                       ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      as
                                                                                      std::os::raw::c_ulong)
                                                              as isize) as
                                           std::os::raw::c_int) == 0 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '<' as i32 &&
                       *(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulong)
                                              as isize) as std::os::raw::c_int ==
                           '=' as i32 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '>' as i32 &&
                       *(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulong)
                                              as isize) as std::os::raw::c_int ==
                           '=' as i32) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        let mut op: i32 = 4 as std::os::raw::c_int;
        if *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
               '<' as i32 &&
               ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong)
                                                      as isize) as
                                   std::os::raw::c_int) == 0 {
            op = 1 as std::os::raw::c_int
        } else if *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                      '>' as i32 &&
                      ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_ulong)
                                                             as isize) as
                                          std::os::raw::c_int) == 0 {
            op = 2 as std::os::raw::c_int
        } else if *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                      '<' as i32 &&
                      *(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     std::os::raw::c_ulong)
                                             as isize) as std::os::raw::c_int ==
                          '=' as i32 {
            op = 3 as std::os::raw::c_int
        }
        (*ee).head =
            ((*ee).head as
                 std::os::raw::c_ulong).wrapping_add(if op > 2 as std::os::raw::c_int {
                                                 2 as std::os::raw::c_int
                                             } else { 1 as std::os::raw::c_int } as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        match op {
            1 => {
                match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if odval < (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if odval < (*ee).ival as std::os::raw::c_double {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if (oival as std::os::raw::c_double) < (*ee).dval
                                       {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if oival < (*ee).ival {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            2 => {
                match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if odval > (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if odval > (*ee).ival as std::os::raw::c_double {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if oival as std::os::raw::c_double > (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if oival > (*ee).ival {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            3 => {
                match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if odval <= (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if odval <= (*ee).ival as std::os::raw::c_double {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if oival as std::os::raw::c_double <= (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if oival <= (*ee).ival {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            4 => {
                match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if odval >= (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if odval >= (*ee).ival as std::os::raw::c_double {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if oival as std::os::raw::c_double >= (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if oival >= (*ee).ival {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_equals(mut ee: * mut crate::lil::_expreval_t) {
    ee_compare(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '=' as i32 &&
                   *(*ee).code.offset((*ee).head.wrapping_add(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong)
                                          as isize) as std::os::raw::c_int ==
                       '=' as i32 ||
                   *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                       '!' as i32 &&
                       *(*ee).code.offset((*ee).head.wrapping_add(1 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulong)
                                              as isize) as std::os::raw::c_int ==
                           '=' as i32) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        let mut op: i32 =
            if *(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '=' as i32 {
                1 as std::os::raw::c_int
            } else { 2 as std::os::raw::c_int };
        (*ee).head =
            ((*ee).head as
                 std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        match op {
            1 => {
                match (*ee).type_0 {
                    1 => {
                        ee_compare(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if odval == (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if odval == (*ee).ival as std::os::raw::c_double {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        ee_compare(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if oival as std::os::raw::c_double == (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if oival == (*ee).ival {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            2 => {
                match (*ee).type_0 {
                    1 => {
                        ee_compare(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if odval != (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if odval != (*ee).ival as std::os::raw::c_double {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    0 => {
                        ee_compare(ee);
                        if (*ee).error != 0 { return }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    if oival as std::os::raw::c_double != (*ee).dval {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t;
                                (*ee).type_0 = 0 as std::os::raw::c_int
                            }
                            0 => {
                                (*ee).ival =
                                    if oival != (*ee).ival {
                                        1 as std::os::raw::c_int
                                    } else { 0 as std::os::raw::c_int } as lilint_t
                            }
                            _ => { (*ee).error = 2 as std::os::raw::c_int }
                        }
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_bitand(mut ee: * mut crate::lil::_expreval_t) {
    ee_equals(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '&' as i32 &&
                   ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulong)
                                                          as isize) as
                                       std::os::raw::c_int) == 0) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(1);
        match (*ee).type_0 {
            1 => {
                ee_equals(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival =
                            odval as lilint_t & (*ee).dval as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => { (*ee).ival = odval as lilint_t & (*ee).ival }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            0 => {
                ee_equals(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival = oival & (*ee).dval as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => { (*ee).ival = oival & (*ee).ival }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { (*ee).error = 2 as std::os::raw::c_int }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_bitor(mut ee: * mut crate::lil::_expreval_t) {
    ee_bitand(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '|' as i32 &&
                   ee_invalidpunct(*(*ee).code.offset((*ee).head.wrapping_add(1
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulong)
                                                          as isize) as
                                       std::os::raw::c_int) == 0) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        (*ee).head = (*ee).head.wrapping_add(1);
        match (*ee).type_0 {
            1 => {
                ee_bitand(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival =
                            odval as lilint_t | (*ee).dval as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => { (*ee).ival = odval as lilint_t | (*ee).ival }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            0 => {
                ee_bitand(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival = oival | (*ee).dval as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => { (*ee).ival = oival | (*ee).ival }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { (*ee).error = 2 as std::os::raw::c_int }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_logand(mut ee: * mut crate::lil::_expreval_t) {
    ee_bitor(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '&' as i32 &&
                   *(*ee).code.offset((*ee).head.wrapping_add(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong)
                                          as isize) as std::os::raw::c_int ==
                       '&' as i32) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        (*ee).head =
            ((*ee).head as
                 std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        match (*ee).type_0 {
            1 => {
                ee_bitor(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival =
                            if odval != 0. && (*ee).dval != 0. {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => {
                        (*ee).ival =
                            if odval != 0. && (*ee).ival != 0 {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            0 => {
                ee_bitor(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival =
                            if oival != 0 && (*ee).dval != 0. {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => {
                        (*ee).ival =
                            if oival != 0 && (*ee).ival != 0 {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { (*ee).error = 2 as std::os::raw::c_int }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_logor(mut ee: * mut crate::lil::_expreval_t) {
    ee_logand(ee);
    ee_skip_spaces(ee);
    while (*ee).head < (*ee).len && (*ee).error == 0 &&
              (*(*ee).code.offset((*ee).head as isize) as std::os::raw::c_int ==
                   '|' as i32 &&
                   *(*ee).code.offset((*ee).head.wrapping_add(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong)
                                          as isize) as std::os::raw::c_int ==
                       '|' as i32) {
        let mut odval: f64 = (*ee).dval;
        let mut oival: i64 = (*ee).ival;
        (*ee).head =
            ((*ee).head as
                 std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        match (*ee).type_0 {
            1 => {
                ee_logand(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival =
                            if odval != 0. || (*ee).dval != 0. {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => {
                        (*ee).ival =
                            if odval != 0. || (*ee).ival != 0 {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            0 => {
                ee_logand(ee);
                if (*ee).error != 0 { return }
                match (*ee).type_0 {
                    1 => {
                        (*ee).ival =
                            if oival != 0 || (*ee).dval != 0. {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t;
                        (*ee).type_0 = 0 as std::os::raw::c_int
                    }
                    0 => {
                        (*ee).ival =
                            if oival != 0 || (*ee).ival != 0 {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int } as lilint_t
                    }
                    _ => { (*ee).error = 2 as std::os::raw::c_int }
                }
            }
            _ => { (*ee).error = 2 as std::os::raw::c_int }
        }
        ee_skip_spaces(ee);
    };
}
unsafe extern "C" fn ee_expr(mut ee: * mut crate::lil::_expreval_t) {
    ee_logor(ee);
    /* invalid expression doesn't really matter, it is only used to stop
     * the expression parsing. */
    if (*ee).error == 4 as std::os::raw::c_int {
        (*ee).error = 0 as std::os::raw::c_int;
        (*ee).ival = 1 as std::os::raw::c_int as lilint_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_eval_expr(mut lil: * mut crate::lil::_lil_t, mut code: * mut crate::lil::_lil_value_t)
 -> * mut crate::lil::_lil_value_t {
    let mut ee: crate::lil::_expreval_t =
        expreval_t{code: 0 as *const std::os::raw::c_char,
                   len: 0,
                   head: 0,
                   ival: 0,
                   dval: 0.,
                   type_0: 0,
                   error: 0,};
    code = lil_subst_to_value(lil, code);
    if (*lil).error != 0 { return 0 as lil_value_t }
    ee.code = lil_to_string(code);
    /* an empty expression equals to 0 so that it can be used as a false value
     * in conditionals */
    if *ee.code.offset(0 as std::os::raw::c_int as isize) == 0 {
        lil_free_value(code);
        return lil_alloc_integer(0 as std::os::raw::c_int as lilint_t)
    }
    ee.head = 0 as std::os::raw::c_int as size_t;
    ee.len = (*code).l;
    ee.ival = 0 as std::os::raw::c_int as lilint_t;
    ee.dval = 0 as std::os::raw::c_int as std::os::raw::c_double;
    ee.type_0 = 0 as std::os::raw::c_int;
    ee.error = 0 as std::os::raw::c_int;
    ee_expr(&mut ee);
    lil_free_value(code);
    if ee.error != 0 {
        match ee.error {
            3 => {
                lil_set_error(lil,
                              b"division by zero in expression\x00" as
                                  *const u8 as *const std::os::raw::c_char);
            }
            2 => {
                lil_set_error(lil,
                              b"mixing invalid types in expression\x00" as
                                  *const u8 as *const std::os::raw::c_char);
            }
            1 => {
                lil_set_error(lil,
                              b"expression syntax error\x00" as *const u8 as
                                  *const std::os::raw::c_char);
            }
            _ => { }
        }
        return 0 as lil_value_t
    }
    if ee.type_0 == 0 as std::os::raw::c_int {
        return lil_alloc_integer(ee.ival)
    } else { return lil_alloc_double(ee.dval) };
}
#[no_mangle]
pub unsafe extern "C" fn lil_unused_name(mut lil: * mut crate::lil::_lil_t,
                                         mut part: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_value_t {
    let mut name: * mut i8 =
        malloc(strlen(part).wrapping_add(64 as std::os::raw::c_int as std::os::raw::c_ulong))
            as *mut std::os::raw::c_char;
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < -(1 as std::os::raw::c_int) as size_t {
        sprintf(name,
                b"!!un!%s!%09u!nu!!\x00" as *const u8 as *const std::os::raw::c_char,
                part, i as std::os::raw::c_uint);
        if find_cmd(lil, name).is_null() {
            if lil_find_var(lil, (*lil).env, name).is_null() {
                val = lil_alloc_string(name);
                free(name as *mut std::os::raw::c_void);
                return val
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as lil_value_t;
}
#[no_mangle]
pub unsafe extern "C" fn lil_arg(mut argv: * mut * mut crate::lil::_lil_value_t,
                                 mut index: std::os::raw::c_ulong) -> * mut crate::lil::_lil_value_t {
    return if !argv.is_null() {
               *argv.offset(index as isize)
           } else { (0 as * mut crate::lil::_lil_value_t) };
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_string(mut val: * mut crate::lil::_lil_value_t)
 -> * const std::os::raw::c_char {
    return if !val.is_null() && (*val).l != 0 {
               (*val).d as *const std::os::raw::c_char
           } else { b"\x00" as *const u8 as *const std::os::raw::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_double(mut val: * mut crate::lil::_lil_value_t)
 -> std::os::raw::c_double {
    return atof(lil_to_string(val));
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_integer(mut val: * mut crate::lil::_lil_value_t) -> std::os::raw::c_longlong {
    return atoll(lil_to_string(val));
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_boolean(mut val: * mut crate::lil::_lil_value_t) -> std::os::raw::c_int {
    let mut s: * const i8 = lil_to_string(val);
    let mut i: u64 = 0;
    let mut dots: u64 = 0 as std::os::raw::c_int as size_t;
    if *s.offset(0 as std::os::raw::c_int as isize) == 0 { return 0 as std::os::raw::c_int }
    i = 0 as std::os::raw::c_int as size_t;
    while *s.offset(i as isize) != 0 {
        if *s.offset(i as isize) as std::os::raw::c_int != '0' as i32 &&
               *s.offset(i as isize) as std::os::raw::c_int != '.' as i32 {
            return 1 as std::os::raw::c_int
        }
        if *s.offset(i as isize) as std::os::raw::c_int == '.' as i32 {
            if dots != 0 { return 1 as std::os::raw::c_int }
            dots = 1 as std::os::raw::c_int as size_t
        }
        i = i.wrapping_add(1)
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_string(mut str: * const std::os::raw::c_char)
 -> * mut crate::lil::_lil_value_t {
    return alloc_value(str);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_string_len(mut str: * const std::os::raw::c_char,
                                              mut len: std::os::raw::c_ulong)
 -> * mut crate::lil::_lil_value_t {
    return alloc_value_len(str, len);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_double(mut num: std::os::raw::c_double)
 -> * mut crate::lil::_lil_value_t {
    let mut buff: [i8; 128] = [0; 128];
    sprintf(buff.as_mut_ptr(), b"%f\x00" as *const u8 as *const std::os::raw::c_char,
            num);
    return alloc_value(buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_integer(mut num: std::os::raw::c_longlong) -> * mut crate::lil::_lil_value_t {
    let mut buff: [i8; 128] = [0; 128];
    sprintf(buff.as_mut_ptr(),
            b"%lli\x00" as *const u8 as *const std::os::raw::c_char, num);
    return alloc_value(buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn lil_free(mut lil: * mut crate::lil::_lil_t) {
    let mut i: u64 = 0;
    if lil.is_null() { return }
    free((*lil).err_msg as *mut std::os::raw::c_void);
    lil_free_value((*lil).empty);
    while !(*lil).env.is_null() {
        let mut next: * mut crate::lil::_lil_env_t = (*(*lil).env).parent;
        lil_free_env((*lil).env);
        (*lil).env = next
    }
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*lil).cmds {
        if !(**(*lil).cmd.offset(i as isize)).argnames.is_null() {
            lil_free_list((**(*lil).cmd.offset(i as isize)).argnames);
        }
        lil_free_value((**(*lil).cmd.offset(i as isize)).code);
        free((**(*lil).cmd.offset(i as isize)).name as *mut std::os::raw::c_void);
        free(*(*lil).cmd.offset(i as isize) as *mut std::os::raw::c_void);
        i = i.wrapping_add(1)
    }
    hm_destroy(Some(&mut (*lil).cmdmap));
    free((*lil).cmd as *mut std::os::raw::c_void);
    free((*lil).dollarprefix as *mut std::os::raw::c_void);
    free((*lil).catcher as *mut std::os::raw::c_void);
    free(lil as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_data<'a1>(mut lil: Option<&'a1 mut crate::lil::_lil_t>,
                                      mut data: * mut core::ffi::c_void) {
    (*(borrow_mut(&mut lil)).unwrap()).data = data;
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_data<'a1>(mut lil: Option<&'a1 mut crate::lil::_lil_t>) -> * mut core::ffi::c_void {
    return (*(borrow_mut(&mut lil)).unwrap()).data;
}
unsafe extern "C" fn fnc_embed_write(mut lil: * mut crate::lil::_lil_t,
                                     mut msg: * const std::os::raw::c_char) {
    let mut len: u64 =
        strlen(msg).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    (*lil).embed =
        realloc((*lil).embed as *mut std::os::raw::c_void,
                (*lil).embedlen.wrapping_add(len)) as *mut std::os::raw::c_char;
    memcpy((*lil).embed.offset((*lil).embedlen as isize) as *mut std::os::raw::c_void,
           msg as *const std::os::raw::c_void, len);
    (*lil).embedlen =
        ((*lil).embedlen as
             std::os::raw::c_ulong).wrapping_add(len.wrapping_sub(1 as std::os::raw::c_int as
                                                              std::os::raw::c_ulong))
            as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn lil_embedded(mut lil: * mut crate::lil::_lil_t,
                                      mut code: * const std::os::raw::c_char,
                                      mut flags: std::os::raw::c_uint)
 -> * mut std::os::raw::c_char {
    let mut prev_embed: * mut i8 = (*lil).embed;
    let mut prev_embedlen: u64 = (*lil).embedlen;
    let mut prev_write: Option<unsafe extern "C"  fn() -> ()> =
        (*lil).callback[1 as std::os::raw::c_int as usize];
    let mut lilcode: * mut i8 = 0 as *mut std::os::raw::c_char;
    let mut lilcodelen: u64 = 0 as std::os::raw::c_int as size_t;
    let mut cont: * mut i8 = 0 as *mut std::os::raw::c_char;
    let mut contlen: u64 = 0 as std::os::raw::c_int as size_t;
    let mut head: u64 = 0 as std::os::raw::c_int as size_t;
    let mut codelen: u64 = strlen(code);
    let mut result: * mut i8 = 0 as *mut std::os::raw::c_char;
    (*lil).callback[1 as std::os::raw::c_int as usize] =
        core::intrinsics::transmute::<Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> ()>, Option<unsafe extern "C"  fn() -> ()>>(Some(fnc_embed_write));
    (*lil).embed = 0 as *mut std::os::raw::c_char;
    (*lil).embedlen = 0 as std::os::raw::c_int as size_t;
    while head < codelen {
        if head < codelen.wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_ulong) &&
               *code.offset(head as isize) as std::os::raw::c_int == '<' as i32 &&
               *code.offset(head.wrapping_add(1 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong) as isize) as
                   std::os::raw::c_int == '?' as i32 &&
               *code.offset(head.wrapping_add(2 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong) as isize) as
                   std::os::raw::c_int == 'l' as i32 &&
               *code.offset(head.wrapping_add(3 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong) as isize) as
                   std::os::raw::c_int == 'i' as i32 &&
               *code.offset(head.wrapping_add(4 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong) as isize) as
                   std::os::raw::c_int == 'l' as i32 {
            head =
                (head as
                     std::os::raw::c_ulong).wrapping_add(5 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t;
            if contlen != 0 {
                lilcode =
                    realloc(lilcode as *mut std::os::raw::c_void,
                            lilcodelen.wrapping_add(contlen).wrapping_add(10
                                                                              as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong))
                        as *mut std::os::raw::c_char;
                memcpy(lilcode.offset(lilcodelen as isize) as
                           *mut std::os::raw::c_void,
                       b"\nwrite {\x00" as *const u8 as *const std::os::raw::c_char as
                           *const std::os::raw::c_void,
                       8 as std::os::raw::c_int as std::os::raw::c_ulong);
                memcpy(lilcode.offset(lilcodelen as
                                          isize).offset(8 as std::os::raw::c_int as
                                                            isize) as
                           *mut std::os::raw::c_void, cont as *const std::os::raw::c_void,
                       contlen);
                *lilcode.offset(lilcodelen.wrapping_add(contlen).wrapping_add(8
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulong)
                                    as isize) = '}' as i32 as std::os::raw::c_char;
                *lilcode.offset(lilcodelen.wrapping_add(contlen).wrapping_add(9
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulong)
                                    as isize) = '\n' as i32 as std::os::raw::c_char;
                lilcodelen =
                    (lilcodelen as
                         std::os::raw::c_ulong).wrapping_add(contlen.wrapping_add(10
                                                                              as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong))
                        as size_t as size_t;
                free(cont as *mut std::os::raw::c_void);
                cont = 0 as *mut std::os::raw::c_char;
                contlen = 0 as std::os::raw::c_int as size_t
            }
            while head < codelen {
                if head <
                       codelen.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                       &&
                       *code.offset(head as isize) as std::os::raw::c_int ==
                           '?' as i32 &&
                       *code.offset(head.wrapping_add(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) as
                                        isize) as std::os::raw::c_int == '>' as i32 {
                    head =
                        (head as
                             std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                             std::os::raw::c_ulong) as
                            size_t as size_t;
                    break ;
                } else {
                    lilcode =
                        realloc(lilcode as *mut std::os::raw::c_void,
                                lilcodelen.wrapping_add(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong)) as
                            *mut std::os::raw::c_char;
                    let mut fresh20 = head;
                    head = head.wrapping_add(1);
                    let mut fresh21 = lilcodelen;
                    lilcodelen = lilcodelen.wrapping_add(1);
                    *lilcode.offset(fresh21 as isize) =
                        *code.offset(fresh20 as isize)
                }
            }
            lilcode =
                realloc(lilcode as *mut std::os::raw::c_void,
                        lilcodelen.wrapping_add(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong)) as
                    *mut std::os::raw::c_char;
            let mut fresh22 = lilcodelen;
            lilcodelen = lilcodelen.wrapping_add(1);
            *lilcode.offset(fresh22 as isize) = '\n' as i32 as std::os::raw::c_char
        } else if *code.offset(head as isize) as std::os::raw::c_int == '{' as i32 ||
                      *code.offset(head as isize) as std::os::raw::c_int == '}' as i32
         {
            cont =
                realloc(cont as *mut std::os::raw::c_void,
                        contlen.wrapping_add(6 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong)) as
                    *mut std::os::raw::c_char;
            let mut fresh23 = contlen;
            contlen = contlen.wrapping_add(1);
            *cont.offset(fresh23 as isize) = '}' as i32 as std::os::raw::c_char;
            let mut fresh24 = contlen;
            contlen = contlen.wrapping_add(1);
            *cont.offset(fresh24 as isize) = '\"' as i32 as std::os::raw::c_char;
            let mut fresh25 = contlen;
            contlen = contlen.wrapping_add(1);
            *cont.offset(fresh25 as isize) = '\\' as i32 as std::os::raw::c_char;
            if *code.offset(head as isize) as std::os::raw::c_int == '{' as i32 {
                let mut fresh26 = contlen;
                contlen = contlen.wrapping_add(1);
                *cont.offset(fresh26 as isize) = 'o' as i32 as std::os::raw::c_char
            } else {
                let mut fresh27 = contlen;
                contlen = contlen.wrapping_add(1);
                *cont.offset(fresh27 as isize) = 'c' as i32 as std::os::raw::c_char
            }
            let mut fresh28 = contlen;
            contlen = contlen.wrapping_add(1);
            *cont.offset(fresh28 as isize) = '\"' as i32 as std::os::raw::c_char;
            let mut fresh29 = contlen;
            contlen = contlen.wrapping_add(1);
            *cont.offset(fresh29 as isize) = '{' as i32 as std::os::raw::c_char;
            head = head.wrapping_add(1)
        } else {
            cont =
                realloc(cont as *mut std::os::raw::c_void,
                        contlen.wrapping_add(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong)) as
                    *mut std::os::raw::c_char;
            let mut fresh30 = head;
            head = head.wrapping_add(1);
            let mut fresh31 = contlen;
            contlen = contlen.wrapping_add(1);
            *cont.offset(fresh31 as isize) = *code.offset(fresh30 as isize)
        }
    }
    if contlen != 0 {
        lilcode =
            realloc(lilcode as *mut std::os::raw::c_void,
                    lilcodelen.wrapping_add(contlen).wrapping_add(10 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulong))
                as *mut std::os::raw::c_char;
        memcpy(lilcode.offset(lilcodelen as isize) as *mut std::os::raw::c_void,
               b"\nwrite {\x00" as *const u8 as *const std::os::raw::c_char as
                   *const std::os::raw::c_void, 8 as std::os::raw::c_int as std::os::raw::c_ulong);
        memcpy(lilcode.offset(lilcodelen as
                                  isize).offset(8 as std::os::raw::c_int as isize) as
                   *mut std::os::raw::c_void, cont as *const std::os::raw::c_void, contlen);
        *lilcode.offset(lilcodelen.wrapping_add(contlen).wrapping_add(8 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          std::os::raw::c_ulong)
                            as isize) = '}' as i32 as std::os::raw::c_char;
        *lilcode.offset(lilcodelen.wrapping_add(contlen).wrapping_add(9 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          std::os::raw::c_ulong)
                            as isize) = '\n' as i32 as std::os::raw::c_char;
        lilcodelen =
            (lilcodelen as
                 std::os::raw::c_ulong).wrapping_add(contlen.wrapping_add(10 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulong))
                as size_t as size_t;
        free(cont as *mut std::os::raw::c_void);
    }
    lilcode =
        realloc(lilcode as *mut std::os::raw::c_void,
                lilcodelen.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    *lilcode.offset(lilcodelen as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    lil_free_value(lil_parse(lil, lilcode, 0 as std::os::raw::c_int as size_t,
                             1 as std::os::raw::c_int));
    free(lilcode as *mut std::os::raw::c_void);
    result =
        if !(*lil).embed.is_null() {
            (*lil).embed
        } else { strclone(b"\x00" as *const u8 as *const std::os::raw::c_char) };
    (*lil).embed = prev_embed;
    (*lil).embedlen = prev_embedlen;
    (*lil).callback[1 as std::os::raw::c_int as usize] = prev_write;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn lil_freemem(mut ptr: * mut core::ffi::c_void) {
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn lil_write(mut lil: * mut crate::lil::_lil_t,
                                   mut msg: * const std::os::raw::c_char) {
    if (*lil).callback[1 as std::os::raw::c_int as usize].is_some() {
        let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> ()> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> ()>>((*lil).callback[1
                                                                                   as
                                                                                   std::os::raw::c_int
                                                                                   as
                                                                                   usize]);
        proc_0.expect("non-null function pointer")(lil, msg);
    } else { printf(b"%s\x00" as *const u8 as *const std::os::raw::c_char, msg); };
}
unsafe extern "C" fn fnc_reflect(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                 mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut func: * mut crate::lil::_lil_func_t = 0 as *mut _lil_func_t;
    let mut type_0: * const i8 = 0 as *const std::os::raw::c_char;
    let mut i: u64 = 0;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if argc == 0 { return 0 as lil_value_t }
    type_0 = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    if strcmp(type_0, b"version\x00" as *const u8 as *const std::os::raw::c_char) == 0
       {
        return lil_alloc_string(b"0.1\x00" as *const u8 as
                                    *const std::os::raw::c_char)
    }
    if strcmp(type_0, b"args\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as lil_value_t
        }
        func =
            find_cmd(lil,
                     lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize)));
        if func.is_null() || (*func).argnames.is_null() {
            return 0 as lil_value_t
        }
        return lil_list_to_value((*func).argnames, 1 as std::os::raw::c_int)
    }
    if strcmp(type_0, b"body\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as lil_value_t
        }
        func =
            find_cmd(lil,
                     lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize)));
        if func.is_null() || (*func).proc_0.is_some() {
            return 0 as lil_value_t
        }
        return lil_clone_value((*func).code)
    }
    if strcmp(type_0, b"func-count\x00" as *const u8 as *const std::os::raw::c_char)
           == 0 {
        return lil_alloc_integer((*lil).cmds as lilint_t)
    }
    if strcmp(type_0, b"funcs\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        let mut funcs: * mut crate::lil::_lil_list_t = lil_alloc_list();
        i = 0 as std::os::raw::c_int as size_t;
        while i < (*lil).cmds {
            lil_list_append(funcs,
                            lil_alloc_string((**(*lil).cmd.offset(i as
                                                                      isize)).name));
            i = i.wrapping_add(1)
        }
        r = lil_list_to_value(funcs, 1 as std::os::raw::c_int);
        lil_free_list(funcs);
        return r
    }
    if strcmp(type_0, b"vars\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        let mut vars: * mut crate::lil::_lil_list_t = lil_alloc_list();
        let mut env: * mut crate::lil::_lil_env_t = (*lil).env;
        while !env.is_null() {
            i = 0 as std::os::raw::c_int as size_t;
            while i < (*env).vars {
                lil_list_append(vars,
                                lil_alloc_string((**(*env).var.offset(i as
                                                                          isize)).n));
                i = i.wrapping_add(1)
            }
            env = (*env).parent
        }
        r = lil_list_to_value(vars, 1 as std::os::raw::c_int);
        lil_free_list(vars);
        return r
    }
    if strcmp(type_0, b"globals\x00" as *const u8 as *const std::os::raw::c_char) == 0
       {
        let mut vars_0: * mut crate::lil::_lil_list_t = lil_alloc_list();
        i = 0 as std::os::raw::c_int as size_t;
        while i < (*(*lil).rootenv).vars {
            lil_list_append(vars_0,
                            lil_alloc_string((**(*(*lil).rootenv).var.offset(i
                                                                                 as
                                                                                 isize)).n));
            i = i.wrapping_add(1)
        }
        r = lil_list_to_value(vars_0, 1 as std::os::raw::c_int);
        lil_free_list(vars_0);
        return r
    }
    if strcmp(type_0, b"has-func\x00" as *const u8 as *const std::os::raw::c_char) ==
           0 {
        let mut target: * const i8 = 0 as *const std::os::raw::c_char;
        if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as lil_value_t
        }
        target = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
        return if hm_has(Some(&mut (*lil).cmdmap), target) != 0 {
                   lil_alloc_string(b"1\x00" as *const u8 as
                                        *const std::os::raw::c_char)
               } else { 0 as lil_value_t }
    }
    if strcmp(type_0, b"has-var\x00" as *const u8 as *const std::os::raw::c_char) == 0
       {
        let mut target_0: * const i8 = 0 as *const std::os::raw::c_char;
        let mut env_0: * mut crate::lil::_lil_env_t = (*lil).env;
        if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as lil_value_t
        }
        target_0 = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
        while !env_0.is_null() {
            if hm_has(Some(&mut (*env_0).varmap), target_0) != 0 {
                return lil_alloc_string(b"1\x00" as *const u8 as
                                            *const std::os::raw::c_char)
            }
            env_0 = (*env_0).parent
        }
        return 0 as lil_value_t
    }
    if strcmp(type_0, b"has-global\x00" as *const u8 as *const std::os::raw::c_char)
           == 0 {
        let mut target_1: * const i8 = 0 as *const std::os::raw::c_char;
        if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as lil_value_t
        }
        target_1 = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
        i = 0 as std::os::raw::c_int as size_t;
        while i < (*(*lil).rootenv).vars {
            if strcmp(target_1,
                      (**(*(*lil).rootenv).var.offset(i as isize)).n) == 0 {
                return lil_alloc_string(b"1\x00" as *const u8 as
                                            *const std::os::raw::c_char)
            }
            i = i.wrapping_add(1)
        }
        return 0 as lil_value_t
    }
    if strcmp(type_0, b"error\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return if !(*lil).err_msg.is_null() {
                   lil_alloc_string((*lil).err_msg)
               } else { 0 as lil_value_t }
    }
    if strcmp(type_0,
              b"dollar-prefix\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        let mut r_0: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
        if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            return lil_alloc_string((*lil).dollarprefix)
        }
        r_0 = lil_alloc_string((*lil).dollarprefix);
        free((*lil).dollarprefix as *mut std::os::raw::c_void);
        (*lil).dollarprefix =
            strclone(lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize)));
        return r_0
    }
    if strcmp(type_0, b"this\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        let mut env_1: * mut crate::lil::_lil_env_t = (*lil).env;
        while env_1 != (*lil).rootenv && (*env_1).catcher_for.is_null() &&
                  (*env_1).func.is_null() {
            env_1 = (*env_1).parent
        }
        if !(*env_1).catcher_for.is_null() {
            return lil_alloc_string((*lil).catcher)
        }
        if env_1 == (*lil).rootenv {
            return lil_alloc_string((*lil).rootcode)
        }
        return if !(*env_1).func.is_null() {
                   (*(*env_1).func).code
               } else { 0 as lil_value_t }
    }
    if strcmp(type_0, b"name\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        let mut env_2: * mut crate::lil::_lil_env_t = (*lil).env;
        while env_2 != (*lil).rootenv && (*env_2).catcher_for.is_null() &&
                  (*env_2).func.is_null() {
            env_2 = (*env_2).parent
        }
        if !(*env_2).catcher_for.is_null() { return (*env_2).catcher_for }
        if env_2 == (*lil).rootenv { return 0 as lil_value_t }
        return if !(*env_2).func.is_null() {
                   lil_alloc_string((*(*env_2).func).name)
               } else { 0 as lil_value_t }
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_func(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut name: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut cmd: * mut crate::lil::_lil_func_t = 0 as *mut _lil_func_t;
    let mut fargs: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if argc >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
        name = lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize));
        fargs =
            lil_subst_to_list(lil, *argv.offset(1 as std::os::raw::c_int as isize));
        cmd =
            add_func(lil,
                     lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)));
        (*cmd).argnames = fargs;
        (*cmd).code = lil_clone_value(*argv.offset(2 as std::os::raw::c_int as isize))
    } else {
        name =
            lil_unused_name(lil,
                            b"anonymous-function\x00" as *const u8 as
                                *const std::os::raw::c_char);
        if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            let mut tmp: * mut crate::lil::_lil_value_t =
                lil_alloc_string(b"args\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            fargs = lil_subst_to_list(lil, tmp);
            lil_free_value(tmp);
            cmd = add_func(lil, lil_to_string(name));
            (*cmd).argnames = fargs;
            (*cmd).code =
                lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize))
        } else {
            fargs =
                lil_subst_to_list(lil,
                                  *argv.offset(0 as std::os::raw::c_int as isize));
            cmd = add_func(lil, lil_to_string(name));
            (*cmd).argnames = fargs;
            (*cmd).code =
                lil_clone_value(*argv.offset(1 as std::os::raw::c_int as isize))
        }
    }
    return name;
}
unsafe extern "C" fn fnc_rename(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut func: * mut crate::lil::_lil_func_t = 0 as *mut _lil_func_t;
    let mut oldname: * const i8 = 0 as *const std::os::raw::c_char;
    let mut newname: * const i8 = 0 as *const std::os::raw::c_char;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    oldname = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    newname = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
    func = find_cmd(lil, oldname);
    if func.is_null() {
        let mut msg: * mut i8 =
            malloc((24 as std::os::raw::c_int as
                        std::os::raw::c_ulong).wrapping_add(strlen(oldname))) as
                *mut std::os::raw::c_char;
        sprintf(msg,
                b"unknown function \'%s\'\x00" as *const u8 as
                    *const std::os::raw::c_char, oldname);
        lil_set_error_at(lil, (*lil).head, msg);
        free(msg as *mut std::os::raw::c_void);
        return 0 as lil_value_t
    }
    r = lil_alloc_string((*func).name);
    if *newname.offset(0 as std::os::raw::c_int as isize) != 0 {
        hm_put(Some(&mut (*lil).cmdmap), oldname, 0 as *mut std::os::raw::c_void);
        hm_put(Some(&mut (*lil).cmdmap), newname, func as *mut std::os::raw::c_void);
        free((*func).name as *mut std::os::raw::c_void);
        (*func).name = strclone(newname)
    } else { del_func(lil, func); }
    return r;
}
unsafe extern "C" fn fnc_unusedname(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                    mut argv: * mut * mut crate::lil::_lil_value_t)
 -> * mut crate::lil::_lil_value_t {
    return lil_unused_name(lil,
                           if argc > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                               lil_to_string(*argv.offset(0 as std::os::raw::c_int as
                                                              isize))
                           } else {
                               b"unusedname\x00" as *const u8 as
                                   *const std::os::raw::c_char
                           });
}
unsafe extern "C" fn fnc_quote(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut i: u64 = 0;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    r = alloc_value(0 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int as size_t;
    while i < argc {
        if i != 0 { lil_append_char(r, ' ' as i32 as std::os::raw::c_char); }
        lil_append_val(r, *argv.offset(i as isize));
        i = i.wrapping_add(1)
    }
    return r;
}
unsafe extern "C" fn fnc_set(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                             mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut i: u64 = 0 as std::os::raw::c_int as size_t;
    let mut var: * mut crate::lil::_lil_var_t = 0 as lil_var_t;
    let mut access: i32 = 1 as std::os::raw::c_int;
    if argc == 0 { return 0 as lil_value_t }
    if strcmp(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
              b"global\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        i = 1 as std::os::raw::c_int as size_t;
        access = 0 as std::os::raw::c_int
    }
    while i < argc {
        if argc == i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
            return lil_clone_value(lil_get_var(lil,
                                               lil_to_string(*argv.offset(i as
                                                                              isize))))
        }
        var =
            lil_set_var(lil, lil_to_string(*argv.offset(i as isize)),
                        *argv.offset(i.wrapping_add(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                                         isize), access);
        i =
            (i as
                 std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t
    }
    return if !var.is_null() {
               lil_clone_value((*var).v)
           } else { 0 as lil_value_t };
}
unsafe extern "C" fn fnc_local(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < argc {
        let mut varname: * const i8 =
            lil_to_string(*argv.offset(i as isize));
        if lil_find_local_var(lil, (*lil).env, varname).is_null() {
            lil_set_var(lil, varname, (*lil).empty, 2 as std::os::raw::c_int);
        }
        i = i.wrapping_add(1)
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_write(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut i: u64 = 0;
    let mut msg: * mut crate::lil::_lil_value_t = lil_alloc_string(0 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int as size_t;
    while i < argc {
        if i != 0 { lil_append_char(msg, ' ' as i32 as std::os::raw::c_char); }
        lil_append_val(msg, *argv.offset(i as isize));
        i = i.wrapping_add(1)
    }
    lil_write(lil, lil_to_string(msg));
    lil_free_value(msg);
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_print(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    fnc_write(lil, argc, argv);
    lil_write(lil, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_eval(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return lil_parse_value(lil, *argv.offset(0 as std::os::raw::c_int as isize),
                               0 as std::os::raw::c_int)
    }
    if argc > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        let mut val: * mut crate::lil::_lil_value_t = alloc_value(0 as *const std::os::raw::c_char);
        let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        i = 0 as std::os::raw::c_int as size_t;
        while i < argc {
            if i != 0 { lil_append_char(val, ' ' as i32 as std::os::raw::c_char); }
            lil_append_val(val, *argv.offset(i as isize));
            i = i.wrapping_add(1)
        }
        r = lil_parse_value(lil, val, 0 as std::os::raw::c_int);
        lil_free_value(val);
        return r
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_topeval(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                 mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut thisenv: * mut crate::lil::_lil_env_t = (*lil).env;
    let mut thisdownenv: * mut crate::lil::_lil_env_t = (*lil).downenv;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    (*lil).env = (*lil).rootenv;
    (*lil).downenv = thisenv;
    r = fnc_eval(lil, argc, argv);
    (*lil).downenv = thisdownenv;
    (*lil).env = thisenv;
    return r;
}
unsafe extern "C" fn fnc_upeval(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut thisenv: * mut crate::lil::_lil_env_t = (*lil).env;
    let mut thisdownenv: * mut crate::lil::_lil_env_t = (*lil).downenv;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if (*lil).rootenv == thisenv { return fnc_eval(lil, argc, argv) }
    (*lil).env = (*thisenv).parent;
    (*lil).downenv = thisenv;
    r = fnc_eval(lil, argc, argv);
    (*lil).env = thisenv;
    (*lil).downenv = thisdownenv;
    return r;
}
unsafe extern "C" fn fnc_downeval(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                  mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut upenv: * mut crate::lil::_lil_env_t = (*lil).env;
    let mut downenv: * mut crate::lil::_lil_env_t = (*lil).downenv;
    if downenv.is_null() { return fnc_eval(lil, argc, argv) }
    (*lil).downenv = 0 as lil_env_t;
    (*lil).env = downenv;
    r = fnc_eval(lil, argc, argv);
    (*lil).downenv = downenv;
    (*lil).env = upenv;
    return r;
}
unsafe extern "C" fn fnc_enveval(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                 mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut invars: * mut crate::lil::_lil_list_t = 0 as lil_list_t;
    let mut outvars: * mut crate::lil::_lil_list_t = 0 as lil_list_t;
    let mut varvalues: * mut * mut crate::lil::_lil_value_t = 0 as *mut lil_value_t;
    let mut codeindex: i32 = 0;
    let mut i: u64 = 0;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        codeindex = 0 as std::os::raw::c_int
    } else if argc >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        invars =
            lil_subst_to_list(lil, *argv.offset(0 as std::os::raw::c_int as isize));
        varvalues =
            malloc((::std::mem::size_of::<lil_value_t>() as
                        std::os::raw::c_ulong).wrapping_mul(lil_list_size(invars))) as
                *mut lil_value_t;
        i = 0 as std::os::raw::c_int as size_t;
        while i < lil_list_size(invars) {
            let ref mut fresh32 = *varvalues.offset(i as isize);
            *fresh32 =
                lil_clone_value(lil_get_var(lil,
                                            lil_to_string(lil_list_get(invars,
                                                                       i))));
            i = i.wrapping_add(1)
        }
        if argc > 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            codeindex = 2 as std::os::raw::c_int;
            outvars =
                lil_subst_to_list(lil,
                                  *argv.offset(1 as std::os::raw::c_int as isize))
        } else { codeindex = 1 as std::os::raw::c_int }
    }
    lil_push_env(lil);
    if !invars.is_null() {
        i = 0 as std::os::raw::c_int as size_t;
        while i < lil_list_size(invars) {
            lil_set_var(lil, lil_to_string(lil_list_get(invars, i)),
                        *varvalues.offset(i as isize), 2 as std::os::raw::c_int);
            lil_free_value(*varvalues.offset(i as isize));
            i = i.wrapping_add(1)
        }
    }
    r =
        lil_parse_value(lil, *argv.offset(codeindex as isize),
                        0 as std::os::raw::c_int);
    if !invars.is_null() || !outvars.is_null() {
        if !outvars.is_null() {
            varvalues =
                realloc(varvalues as *mut std::os::raw::c_void,
                        (::std::mem::size_of::<lil_value_t>() as
                             std::os::raw::c_ulong).wrapping_mul(lil_list_size(outvars)))
                    as *mut lil_value_t;
            i = 0 as std::os::raw::c_int as size_t;
            while i < lil_list_size(outvars) {
                let ref mut fresh33 = *varvalues.offset(i as isize);
                *fresh33 =
                    lil_clone_value(lil_get_var(lil,
                                                lil_to_string(lil_list_get(outvars,
                                                                           i))));
                i = i.wrapping_add(1)
            }
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i < lil_list_size(invars) {
                let ref mut fresh34 = *varvalues.offset(i as isize);
                *fresh34 =
                    lil_clone_value(lil_get_var(lil,
                                                lil_to_string(lil_list_get(invars,
                                                                           i))));
                i = i.wrapping_add(1)
            }
        }
    }
    lil_pop_env(lil);
    if !invars.is_null() {
        if !outvars.is_null() {
            i = 0 as std::os::raw::c_int as size_t;
            while i < lil_list_size(outvars) {
                lil_set_var(lil, lil_to_string(lil_list_get(outvars, i)),
                            *varvalues.offset(i as isize), 1 as std::os::raw::c_int);
                lil_free_value(*varvalues.offset(i as isize));
                i = i.wrapping_add(1)
            }
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i < lil_list_size(invars) {
                lil_set_var(lil, lil_to_string(lil_list_get(invars, i)),
                            *varvalues.offset(i as isize), 1 as std::os::raw::c_int);
                lil_free_value(*varvalues.offset(i as isize));
                i = i.wrapping_add(1)
            }
        }
        lil_free_list(invars);
        if !outvars.is_null() { lil_free_list(outvars); }
        free(varvalues as *mut std::os::raw::c_void);
    }
    return r;
}
unsafe extern "C" fn fnc_jaileval(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                  mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut i: u64 = 0;
    let mut sublil: * mut crate::lil::_lil_t = 0 as *mut _lil_t;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut base: u64 = 0 as std::os::raw::c_int as size_t;
    if argc == 0 { return 0 as lil_value_t }
    if strcmp(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
              b"clean\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        base = 1 as std::os::raw::c_int as size_t;
        if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as lil_value_t
        }
    }
    sublil = lil_new();
    if base != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        i = (*lil).syscmds;
        while i < (*lil).cmds {
            let mut fnc: * mut crate::lil::_lil_func_t = *(*lil).cmd.offset(i as isize);
            if !(*fnc).proc_0.is_none() {
                lil_register(sublil, (*fnc).name, (*fnc).proc_0);
            }
            i = i.wrapping_add(1)
        }
    }
    r =
        lil_parse_value(sublil, *argv.offset(base as isize),
                        1 as std::os::raw::c_int);
    lil_free(sublil);
    return r;
}
unsafe extern "C" fn fnc_count(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut buff: [i8; 64] = [0; 64];
    if argc == 0 {
        return alloc_value(b"0\x00" as *const u8 as *const std::os::raw::c_char)
    }
    list = lil_subst_to_list(lil, *argv.offset(0 as std::os::raw::c_int as isize));
    sprintf(buff.as_mut_ptr(), b"%u\x00" as *const u8 as *const std::os::raw::c_char,
            (*list).c as std::os::raw::c_uint);
    lil_free_list(list);
    return alloc_value(buff.as_mut_ptr());
}
unsafe extern "C" fn fnc_index(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut index: u64 = 0;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    list = lil_subst_to_list(lil, *argv.offset(0 as std::os::raw::c_int as isize));
    index = lil_to_integer(*argv.offset(1 as std::os::raw::c_int as isize)) as size_t;
    if index >= (*list).c {
        r = 0 as lil_value_t
    } else { r = lil_clone_value(*(*list).v.offset(index as isize)) }
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_indexof(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                 mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut index: u64 = 0;
    let mut r: * mut crate::lil::_lil_value_t = 0 as lil_value_t;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    list = lil_subst_to_list(lil, *argv.offset(0 as std::os::raw::c_int as isize));
    index = 0 as std::os::raw::c_int as size_t;
    while index < (*list).c {
        if strcmp(lil_to_string(*(*list).v.offset(index as isize)),
                  lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize))) == 0
           {
            r = lil_alloc_integer(index as lilint_t);
            break ;
        } else { index = index.wrapping_add(1) }
    }
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_append(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut i: u64 = 0;
    let mut base: u64 = 1 as std::os::raw::c_int as size_t;
    let mut access: i32 = 1 as std::os::raw::c_int;
    let mut varname: * const i8 = 0 as *const std::os::raw::c_char;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    varname = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    if strcmp(varname, b"global\x00" as *const u8 as *const std::os::raw::c_char) == 0
       {
        if argc < 3 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as lil_value_t
        }
        varname = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
        base = 2 as std::os::raw::c_int as size_t;
        access = 0 as std::os::raw::c_int
    }
    list = lil_subst_to_list(lil, lil_get_var(lil, varname));
    i = base;
    while i < argc {
        lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
        i = i.wrapping_add(1)
    }
    r = lil_list_to_value(list, 1 as std::os::raw::c_int);
    lil_free_list(list);
    lil_set_var(lil, varname, r, access);
    return r;
}
unsafe extern "C" fn fnc_slice(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut slice: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut i: u64 = 0;
    let mut from: i64 = 0;
    let mut to: i64 = 0;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        return lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize))
    }
    from = lil_to_integer(*argv.offset(1 as std::os::raw::c_int as isize));
    if from < 0 as std::os::raw::c_int as std::os::raw::c_longlong {
        from = 0 as std::os::raw::c_int as lilint_t
    }
    list = lil_subst_to_list(lil, *argv.offset(0 as std::os::raw::c_int as isize));
    to =
        if argc > 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            lil_to_integer(*argv.offset(2 as std::os::raw::c_int as isize))
        } else { (*list).c as lilint_t };
    if to > (*list).c as lilint_t { to = (*list).c as lilint_t }
    if to < from { to = from }
    slice = lil_alloc_list();
    i = from as size_t;
    while i < to as size_t {
        lil_list_append(slice,
                        lil_clone_value(*(*list).v.offset(i as isize)));
        i = i.wrapping_add(1)
    }
    lil_free_list(list);
    r = lil_list_to_value(slice, 1 as std::os::raw::c_int);
    lil_free_list(slice);
    return r;
}
unsafe extern "C" fn fnc_filter(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut filtered: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut i: u64 = 0;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut varname: * const i8 =
        b"x\x00" as *const u8 as *const std::os::raw::c_char;
    let mut base: i32 = 0 as std::os::raw::c_int;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        return lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize))
    }
    if argc > 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        base = 1 as std::os::raw::c_int;
        varname = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize))
    }
    list = lil_subst_to_list(lil, *argv.offset(base as isize));
    filtered = lil_alloc_list();
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*list).c && (*(*lil).env).breakrun == 0 {
        lil_set_var(lil, varname, *(*list).v.offset(i as isize),
                    3 as std::os::raw::c_int);
        r =
            lil_eval_expr(lil,
                          *argv.offset((base + 1 as std::os::raw::c_int) as isize));
        if lil_to_boolean(r) != 0 {
            lil_list_append(filtered,
                            lil_clone_value(*(*list).v.offset(i as isize)));
        }
        lil_free_value(r);
        i = i.wrapping_add(1)
    }
    lil_free_list(list);
    r = lil_list_to_value(filtered, 1 as std::os::raw::c_int);
    lil_free_list(filtered);
    return r;
}
unsafe extern "C" fn fnc_list(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = lil_alloc_list();
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < argc {
        lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
        i = i.wrapping_add(1)
    }
    r = lil_list_to_value(list, 1 as std::os::raw::c_int);
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_subst(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    return lil_subst_to_value(lil, *argv.offset(0 as std::os::raw::c_int as isize));
}
unsafe extern "C" fn fnc_concat(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut tmp: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut i: u64 = 0;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    r = lil_alloc_string(b"\x00" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int as size_t;
    while i < argc {
        list = lil_subst_to_list(lil, *argv.offset(i as isize));
        tmp = lil_list_to_value(list, 1 as std::os::raw::c_int);
        lil_free_list(list);
        lil_append_val(r, tmp);
        lil_free_value(tmp);
        i = i.wrapping_add(1)
    }
    return r;
}
unsafe extern "C" fn fnc_foreach(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                 mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut rlist: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut i: u64 = 0;
    let mut listidx: u64 = 0 as std::os::raw::c_int as size_t;
    let mut codeidx: u64 = 1 as std::os::raw::c_int as size_t;
    let mut varname: * const i8 =
        b"i\x00" as *const u8 as *const std::os::raw::c_char;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if argc >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
        varname = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
        listidx = 1 as std::os::raw::c_int as size_t;
        codeidx = 2 as std::os::raw::c_int as size_t
    }
    rlist = lil_alloc_list();
    list = lil_subst_to_list(lil, *argv.offset(listidx as isize));
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*list).c {
        let mut rv: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
        lil_set_var(lil, varname, *(*list).v.offset(i as isize),
                    3 as std::os::raw::c_int);
        rv =
            lil_parse_value(lil, *argv.offset(codeidx as isize),
                            0 as std::os::raw::c_int);
        if (*rv).l != 0 {
            lil_list_append(rlist, rv);
        } else { lil_free_value(rv); }
        if (*(*lil).env).breakrun != 0 || (*lil).error != 0 { break ; }
        i = i.wrapping_add(1)
    }
    r = lil_list_to_value(rlist, 1 as std::os::raw::c_int);
    lil_free_list(list);
    lil_free_list(rlist);
    return r;
}
unsafe extern "C" fn fnc_return(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    (*(*lil).env).breakrun = 1 as std::os::raw::c_int;
    lil_free_value((*(*lil).env).retval);
    (*(*lil).env).retval =
        if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            0 as lil_value_t
        } else { lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize)) };
    (*(*lil).env).retval_set = 1 as std::os::raw::c_int;
    return if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
               0 as lil_value_t
           } else {
               lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize))
           };
}
unsafe extern "C" fn fnc_result(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        lil_free_value((*(*lil).env).retval);
        (*(*lil).env).retval =
            lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize));
        (*(*lil).env).retval_set = 1 as std::os::raw::c_int
    }
    return if (*(*lil).env).retval_set != 0 {
               lil_clone_value((*(*lil).env).retval)
           } else { 0 as lil_value_t };
}
unsafe extern "C" fn fnc_expr(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return lil_eval_expr(lil, *argv.offset(0 as std::os::raw::c_int as isize))
    }
    if argc > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        let mut val: * mut crate::lil::_lil_value_t = alloc_value(0 as *const std::os::raw::c_char);
        let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        i = 0 as std::os::raw::c_int as size_t;
        while i < argc {
            if i != 0 { lil_append_char(val, ' ' as i32 as std::os::raw::c_char); }
            lil_append_val(val, *argv.offset(i as isize));
            i = i.wrapping_add(1)
        }
        r = lil_eval_expr(lil, val);
        lil_free_value(val);
        return r
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn real_inc(mut lil: * mut crate::lil::_lil_t,
                              mut varname: * const std::os::raw::c_char,
                              mut v: std::os::raw::c_float) -> * mut crate::lil::_lil_value_t {
    let mut pv: * mut crate::lil::_lil_value_t = lil_get_var(lil, varname);
    let mut dv: f64 = lil_to_double(pv) + v as std::os::raw::c_double;
    if fmod(dv, 1 as std::os::raw::c_int as std::os::raw::c_double) != 0. {
        pv = lil_alloc_double(dv)
    } else { pv = lil_alloc_integer(dv as lilint_t) }
    lil_set_var(lil, varname, pv, 1 as std::os::raw::c_int);
    return pv;
}
unsafe extern "C" fn fnc_inc(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                             mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    return real_inc(lil,
                    lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                    if argc > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                        lil_to_double(*argv.offset(1 as std::os::raw::c_int as isize))
                    } else { 1 as std::os::raw::c_int as std::os::raw::c_double } as
                        std::os::raw::c_float);
}
unsafe extern "C" fn fnc_dec(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                             mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    return real_inc(lil,
                    lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                    -if argc > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                         lil_to_double(*argv.offset(1 as std::os::raw::c_int as
                                                        isize))
                     } else { 1 as std::os::raw::c_int as std::os::raw::c_double } as
                        std::os::raw::c_float);
}
unsafe extern "C" fn fnc_read(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut f: * mut crate::lil::__sFILE = 0 as *mut FILE;
    let mut size: u64 = 0;
    let mut buffer: * mut i8 = 0 as *mut std::os::raw::c_char;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if (*lil).callback[2 as std::os::raw::c_int as usize].is_some() {
        let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> * mut i8> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> * mut i8>>((*lil).callback[2
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  usize]);
        buffer =
            proc_0.expect("non-null function pointer")(lil,
                                                       lil_to_string(*argv.offset(0
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      as
                                                                                      isize)))
    } else {
        f =
            fopen(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                  b"rb\x00" as *const u8 as *const std::os::raw::c_char);
        if f.is_null() { return 0 as lil_value_t }
        fseek(f, 0 as std::os::raw::c_int as std::os::raw::c_long, 2 as std::os::raw::c_int);
        size = ftell(f) as size_t;
        fseek(f, 0 as std::os::raw::c_int as std::os::raw::c_long, 0 as std::os::raw::c_int);
        buffer =
            malloc(size.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)) as
                *mut std::os::raw::c_char;
        fread(buffer as *mut std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
              size, f);
        *buffer.offset(size as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
        fclose(f);
    }
    r = lil_alloc_string(buffer);
    free(buffer as *mut std::os::raw::c_void);
    return r;
}
unsafe extern "C" fn fnc_store(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut f: * mut crate::lil::__sFILE = 0 as *mut FILE;
    let mut buffer: * const i8 = 0 as *const std::os::raw::c_char;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if (*lil).callback[3 as std::os::raw::c_int as usize].is_some() {
        let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,_: * const i8,) -> ()> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,_: * const i8,) -> ()>>((*lil).callback[3
                                                                                   as
                                                                                   std::os::raw::c_int
                                                                                   as
                                                                                   usize]);
        proc_0.expect("non-null function pointer")(lil,
                                                   lil_to_string(*argv.offset(0
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  isize)),
                                                   lil_to_string(*argv.offset(1
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  isize)));
    } else {
        f =
            fopen(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                  b"wb\x00" as *const u8 as *const std::os::raw::c_char);
        if f.is_null() { return 0 as lil_value_t }
        buffer = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
        fwrite(buffer as *const std::os::raw::c_void,
               1 as std::os::raw::c_int as std::os::raw::c_ulong, strlen(buffer), f);
        fclose(f);
    }
    return lil_clone_value(*argv.offset(1 as std::os::raw::c_int as isize));
}
unsafe extern "C" fn fnc_if(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                            mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut r: * mut crate::lil::_lil_value_t = 0 as lil_value_t;
    let mut base: i32 = 0 as std::os::raw::c_int;
    let mut not: i32 = 0 as std::os::raw::c_int;
    let mut v: i32 = 0;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if strcmp(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
              b"not\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        not = 1 as std::os::raw::c_int;
        base = not
    }
    if argc < (base as size_t).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
       {
        return 0 as lil_value_t
    }
    val = lil_eval_expr(lil, *argv.offset(base as isize));
    if val.is_null() || (*lil).error != 0 { return 0 as lil_value_t }
    v = lil_to_boolean(val);
    if not != 0 { v = (v == 0) as std::os::raw::c_int }
    if v != 0 {
        r =
            lil_parse_value(lil,
                            *argv.offset((base + 1 as std::os::raw::c_int) as isize),
                            0 as std::os::raw::c_int)
    } else if argc >
                  (base as
                       size_t).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
     {
        r =
            lil_parse_value(lil,
                            *argv.offset((base + 2 as std::os::raw::c_int) as isize),
                            0 as std::os::raw::c_int)
    }
    lil_free_value(val);
    return r;
}
unsafe extern "C" fn fnc_while(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut r: * mut crate::lil::_lil_value_t = 0 as lil_value_t;
    let mut base: i32 = 0 as std::os::raw::c_int;
    let mut not: i32 = 0 as std::os::raw::c_int;
    let mut v: i32 = 0;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if strcmp(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
              b"not\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        not = 1 as std::os::raw::c_int;
        base = not
    }
    if argc < (base as size_t).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
       {
        return 0 as lil_value_t
    }
    while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
        val = lil_eval_expr(lil, *argv.offset(base as isize));
        if val.is_null() || (*lil).error != 0 { return 0 as lil_value_t }
        v = lil_to_boolean(val);
        if not != 0 { v = (v == 0) as std::os::raw::c_int }
        if v == 0 {
            lil_free_value(val);
            break ;
        } else {
            if !r.is_null() { lil_free_value(r); }
            r =
                lil_parse_value(lil,
                                *argv.offset((base + 1 as std::os::raw::c_int) as
                                                 isize), 0 as std::os::raw::c_int);
            lil_free_value(val);
        }
    }
    return r;
}
unsafe extern "C" fn fnc_for(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                             mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut r: * mut crate::lil::_lil_value_t = 0 as lil_value_t;
    if argc < 4 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    lil_free_value(lil_parse_value(lil,
                                   *argv.offset(0 as std::os::raw::c_int as isize),
                                   0 as std::os::raw::c_int));
    while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
        val = lil_eval_expr(lil, *argv.offset(1 as std::os::raw::c_int as isize));
        if val.is_null() || (*lil).error != 0 { return 0 as lil_value_t }
        if lil_to_boolean(val) == 0 {
            lil_free_value(val);
            break ;
        } else {
            if !r.is_null() { lil_free_value(r); }
            r =
                lil_parse_value(lil, *argv.offset(3 as std::os::raw::c_int as isize),
                                0 as std::os::raw::c_int);
            lil_free_value(val);
            lil_free_value(lil_parse_value(lil,
                                           *argv.offset(2 as std::os::raw::c_int as
                                                            isize),
                                           0 as std::os::raw::c_int));
        }
    }
    return r;
}
unsafe extern "C" fn fnc_char(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut s: [i8; 2] = [0; 2];
    if argc == 0 { return 0 as lil_value_t }
    s[0 as std::os::raw::c_int as usize] =
        lil_to_integer(*argv.offset(0 as std::os::raw::c_int as isize)) as
            std::os::raw::c_char;
    s[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return lil_alloc_string(s.as_mut_ptr());
}
unsafe extern "C" fn fnc_charat(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut index: u64 = 0;
    let mut chstr: [i8; 2] = [0; 2];
    let mut str: * const i8 = 0 as *const std::os::raw::c_char;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    str = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    index = lil_to_integer(*argv.offset(1 as std::os::raw::c_int as isize)) as size_t;
    if index >= strlen(str) { return 0 as lil_value_t }
    chstr[0 as std::os::raw::c_int as usize] = *str.offset(index as isize);
    chstr[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return lil_alloc_string(chstr.as_mut_ptr());
}
unsafe extern "C" fn fnc_codeat(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut index: u64 = 0;
    let mut str: * const i8 = 0 as *const std::os::raw::c_char;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    str = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    index = lil_to_integer(*argv.offset(1 as std::os::raw::c_int as isize)) as size_t;
    if index >= strlen(str) { return 0 as lil_value_t }
    return lil_alloc_integer(*str.offset(index as isize) as lilint_t);
}
unsafe extern "C" fn fnc_substr(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut str: * const i8 = 0 as *const std::os::raw::c_char;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut start: u64 = 0;
    let mut end: u64 = 0;
    let mut i: u64 = 0;
    let mut slen: u64 = 0;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    str = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    if *str.offset(0 as std::os::raw::c_int as isize) == 0 { return 0 as lil_value_t }
    slen = strlen(str);
    start =
        atoll(lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize))) as
            size_t;
    end =
        if argc > 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            atoll(lil_to_string(*argv.offset(2 as std::os::raw::c_int as isize))) as
                size_t
        } else { slen };
    if end > slen { end = slen }
    if start >= end { return 0 as lil_value_t }
    r = lil_alloc_string(b"\x00" as *const u8 as *const std::os::raw::c_char);
    i = start;
    while i < end {
        lil_append_char(r, *str.offset(i as isize));
        i = i.wrapping_add(1)
    }
    return r;
}
unsafe extern "C" fn fnc_strpos(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut hay: * const i8 = 0 as *const std::os::raw::c_char;
    let mut str: * const i8 = 0 as *const std::os::raw::c_char;
    let mut min: u64 = 0 as std::os::raw::c_int as size_t;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        return lil_alloc_integer(-(1 as std::os::raw::c_int) as lilint_t)
    }
    hay = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    if argc > 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        min =
            atoll(lil_to_string(*argv.offset(2 as std::os::raw::c_int as isize))) as
                size_t;
        if min >= strlen(hay) {
            return lil_alloc_integer(-(1 as std::os::raw::c_int) as lilint_t)
        }
    }
    str =
        strstr(hay.offset(min as isize),
               lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize)));
    if str.is_null() {
        return lil_alloc_integer(-(1 as std::os::raw::c_int) as lilint_t)
    }
    return lil_alloc_integer(str.offset_from(hay) as std::os::raw::c_long as
                                 lilint_t);
}
unsafe extern "C" fn fnc_length(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut i: u64 = 0;
    let mut total: u64 = 0 as std::os::raw::c_int as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i < argc {
        if i != 0 { total = total.wrapping_add(1) }
        total =
            (total as
                 std::os::raw::c_ulong).wrapping_add(strlen(lil_to_string(*argv.offset(i
                                                                                   as
                                                                                   isize))))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    return lil_alloc_integer(total as lilint_t);
}
unsafe extern "C" fn real_trim(mut str: * const std::os::raw::c_char,
                               mut chars: * const std::os::raw::c_char,
                               mut left: std::os::raw::c_int, mut right: std::os::raw::c_int)
 -> * mut crate::lil::_lil_value_t {
    let mut base: i32 = 0 as std::os::raw::c_int;
    let mut r: * mut crate::lil::_lil_value_t = 0 as lil_value_t;
    if left != 0 {
        while *str.offset(base as isize) as std::os::raw::c_int != 0 &&
                  !strchr(chars,
                          *str.offset(base as isize) as std::os::raw::c_int).is_null()
              {
            base += 1
        }
        if right == 0 {
            r =
                lil_alloc_string(if *str.offset(base as isize) as std::os::raw::c_int
                                        != 0 {
                                     str.offset(base as isize)
                                 } else { 0 as *const std::os::raw::c_char })
        }
    }
    if right != 0 {
        let mut len: u64 = 0;
        let mut s: * mut i8 = 0 as *mut std::os::raw::c_char;
        s = strclone(str.offset(base as isize));
        len = strlen(s);
        while len != 0 &&
                  !strchr(chars,
                          *s.offset(len.wrapping_sub(1 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                        isize) as std::os::raw::c_int).is_null() {
            len = len.wrapping_sub(1)
        }
        *s.offset(len as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
        r = lil_alloc_string(s);
        free(s as *mut std::os::raw::c_void);
    }
    return r;
}
unsafe extern "C" fn fnc_trim(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc == 0 { return 0 as lil_value_t }
    return real_trim(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                     if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                         b" \x0c\n\r\t\x0b\x00" as *const u8 as
                             *const std::os::raw::c_char
                     } else {
                         lil_to_string(*argv.offset(1 as std::os::raw::c_int as
                                                        isize))
                     }, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int);
}
unsafe extern "C" fn fnc_ltrim(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc == 0 { return 0 as lil_value_t }
    return real_trim(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                     if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                         b" \x0c\n\r\t\x0b\x00" as *const u8 as
                             *const std::os::raw::c_char
                     } else {
                         lil_to_string(*argv.offset(1 as std::os::raw::c_int as
                                                        isize))
                     }, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int);
}
unsafe extern "C" fn fnc_rtrim(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc == 0 { return 0 as lil_value_t }
    return real_trim(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                     if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                         b" \x0c\n\r\t\x0b\x00" as *const u8 as
                             *const std::os::raw::c_char
                     } else {
                         lil_to_string(*argv.offset(1 as std::os::raw::c_int as
                                                        isize))
                     }, 0 as std::os::raw::c_int, 1 as std::os::raw::c_int);
}
unsafe extern "C" fn fnc_strcmp(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    return lil_alloc_integer(strcmp(lil_to_string(*argv.offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize)),
                                    lil_to_string(*argv.offset(1 as
                                                                   std::os::raw::c_int
                                                                   as isize)))
                                 as lilint_t);
}
unsafe extern "C" fn fnc_streq(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    return lil_alloc_integer(if strcmp(lil_to_string(*argv.offset(0 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      isize)),
                                       lil_to_string(*argv.offset(1 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      isize)))
                                    != 0 {
                                 0 as std::os::raw::c_int
                             } else { 1 as std::os::raw::c_int } as lilint_t);
}
unsafe extern "C" fn fnc_repstr(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut from: * const i8 = 0 as *const std::os::raw::c_char;
    let mut to: * const i8 = 0 as *const std::os::raw::c_char;
    let mut src: * mut i8 = 0 as *mut std::os::raw::c_char;
    let mut sub: * const i8 = 0 as *const std::os::raw::c_char;
    let mut idx: u64 = 0;
    let mut fromlen: u64 = 0;
    let mut tolen: u64 = 0;
    let mut srclen: u64 = 0;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if argc < 3 as std::os::raw::c_int as std::os::raw::c_ulong {
        return lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize))
    }
    from = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
    to = lil_to_string(*argv.offset(2 as std::os::raw::c_int as isize));
    if *from.offset(0 as std::os::raw::c_int as isize) == 0 {
        return 0 as lil_value_t
    }
    src = strclone(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)));
    srclen = strlen(src);
    fromlen = strlen(from);
    tolen = strlen(to);
    loop  {
        sub = strstr(src, from);
        if sub.is_null() { break ; }
        let mut newsrc: * mut i8 =
            malloc(srclen.wrapping_sub(fromlen).wrapping_add(tolen).wrapping_add(1
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_ulong))
                as *mut std::os::raw::c_char;
        idx = sub.offset_from(src) as std::os::raw::c_long as size_t;
        if idx != 0 {
            memcpy(newsrc as *mut std::os::raw::c_void, src as *const std::os::raw::c_void,
                   idx);
        }
        memcpy(newsrc.offset(idx as isize) as *mut std::os::raw::c_void,
               to as *const std::os::raw::c_void, tolen);
        memcpy(newsrc.offset(idx as isize).offset(tolen as isize) as
                   *mut std::os::raw::c_void,
               src.offset(idx as isize).offset(fromlen as isize) as
                   *const std::os::raw::c_void,
               srclen.wrapping_sub(idx).wrapping_sub(fromlen));
        srclen = srclen.wrapping_sub(fromlen).wrapping_add(tolen);
        free(src as *mut std::os::raw::c_void);
        src = newsrc;
        *src.offset(srclen as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char
    }
    r = lil_alloc_string(src);
    free(src as *mut std::os::raw::c_void);
    return r;
}
unsafe extern "C" fn fnc_split(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut sep: * const i8 =
        b" \x00" as *const u8 as *const std::os::raw::c_char;
    let mut i: u64 = 0;
    let mut val: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    let mut str: * const i8 = 0 as *const std::os::raw::c_char;
    if argc == 0 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if argc > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        sep = lil_to_string(*argv.offset(1 as std::os::raw::c_int as isize));
        if sep.is_null() || *sep.offset(0 as std::os::raw::c_int as isize) == 0 {
            return lil_clone_value(*argv.offset(0 as std::os::raw::c_int as isize))
        }
    }
    val = lil_alloc_string(b"\x00" as *const u8 as *const std::os::raw::c_char);
    str = lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
    list = lil_alloc_list();
    i = 0 as std::os::raw::c_int as size_t;
    while *str.offset(i as isize) != 0 {
        if !strchr(sep, *str.offset(i as isize) as std::os::raw::c_int).is_null() {
            lil_list_append(list, val);
            val =
                lil_alloc_string(b"\x00" as *const u8 as *const std::os::raw::c_char)
        } else { lil_append_char(val, *str.offset(i as isize)); }
        i = i.wrapping_add(1)
    }
    lil_list_append(list, val);
    val = lil_list_to_value(list, 1 as std::os::raw::c_int);
    lil_free_list(list);
    return val;
}
unsafe extern "C" fn fnc_try(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                             mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if (*lil).error != 0 { return 0 as lil_value_t }
    r =
        lil_parse_value(lil, *argv.offset(0 as std::os::raw::c_int as isize),
                        0 as std::os::raw::c_int);
    if (*lil).error != 0 {
        (*lil).error = 0 as std::os::raw::c_int;
        lil_free_value(r);
        if argc > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            r =
                lil_parse_value(lil, *argv.offset(1 as std::os::raw::c_int as isize),
                                0 as std::os::raw::c_int)
        } else { r = 0 as lil_value_t }
    }
    return r;
}
unsafe extern "C" fn fnc_error(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    lil_set_error(lil,
                  if argc > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                      lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize))
                  } else { 0 as *const std::os::raw::c_char });
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_exit(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if (*lil).callback[0 as std::os::raw::c_int as usize].is_some() {
        let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * mut crate::lil::_lil_value_t,) -> ()> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * mut crate::lil::_lil_value_t,) -> ()>>((*lil).callback[0
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  usize]);
        proc_0.expect("non-null function pointer")(lil,
                                                   if argc >
                                                          0 as std::os::raw::c_int as
                                                              std::os::raw::c_ulong {
                                                       *argv.offset(0 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        isize)
                                                   } else {
                                                       0 as lil_value_t
                                                   });
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_source(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut f: * mut crate::lil::__sFILE = 0 as *mut FILE;
    let mut size: u64 = 0;
    let mut buffer: * mut i8 = 0 as *mut std::os::raw::c_char;
    let mut r: * mut crate::lil::_lil_value_t = 0 as *mut _lil_value_t;
    if argc < 1 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    if (*lil).callback[4 as std::os::raw::c_int as usize].is_some() {
        let mut proc_0: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> * mut i8> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> * mut i8>>((*lil).callback[4
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    usize]);
        buffer =
            proc_0.expect("non-null function pointer")(lil,
                                                       lil_to_string(*argv.offset(0
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      as
                                                                                      isize)))
    } else if (*lil).callback[2 as std::os::raw::c_int as usize].is_some() {
        let mut proc_1: Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> * mut i8> =
            core::intrinsics::transmute::<Option<unsafe extern "C"  fn() -> ()>, Option<unsafe extern "C"  fn(_: * mut crate::lil::_lil_t,_: * const i8,) -> * mut i8>>((*lil).callback[2
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  usize]);
        buffer =
            proc_1.expect("non-null function pointer")(lil,
                                                       lil_to_string(*argv.offset(0
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      as
                                                                                      isize)))
    } else {
        f =
            fopen(lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize)),
                  b"rb\x00" as *const u8 as *const std::os::raw::c_char);
        if f.is_null() { return 0 as lil_value_t }
        fseek(f, 0 as std::os::raw::c_int as std::os::raw::c_long, 2 as std::os::raw::c_int);
        size = ftell(f) as size_t;
        fseek(f, 0 as std::os::raw::c_int as std::os::raw::c_long, 0 as std::os::raw::c_int);
        buffer =
            malloc(size.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)) as
                *mut std::os::raw::c_char;
        fread(buffer as *mut std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
              size, f);
        *buffer.offset(size as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
        fclose(f);
    }
    r = lil_parse(lil, buffer, 0 as std::os::raw::c_int as size_t, 0 as std::os::raw::c_int);
    free(buffer as *mut std::os::raw::c_void);
    return r;
}
unsafe extern "C" fn fnc_lmap(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut list: * mut crate::lil::_lil_list_t = 0 as *mut _lil_list_t;
    let mut i: u64 = 0;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    list = lil_subst_to_list(lil, *argv.offset(0 as std::os::raw::c_int as isize));
    i = 1 as std::os::raw::c_int as size_t;
    while i < argc {
        lil_set_var(lil, lil_to_string(*argv.offset(i as isize)),
                    lil_list_get(list,
                                 i.wrapping_sub(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong)),
                    1 as std::os::raw::c_int);
        i = i.wrapping_add(1)
    }
    lil_free_list(list);
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_rand(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                              mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    return lil_alloc_double(rand() as std::os::raw::c_double /
                                0x7fffffff as std::os::raw::c_int as std::os::raw::c_double);
}
unsafe extern "C" fn fnc_catcher(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                                 mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    if argc == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return lil_alloc_string((*lil).catcher)
    } else {
        let mut catcher: * const i8 =
            lil_to_string(*argv.offset(0 as std::os::raw::c_int as isize));
        free((*lil).catcher as *mut std::os::raw::c_void);
        (*lil).catcher =
            if *catcher.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != 0
               {
                strclone(catcher)
            } else { 0 as *mut std::os::raw::c_char }
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_watch(mut lil: * mut crate::lil::_lil_t, mut argc: std::os::raw::c_ulong,
                               mut argv: * mut * mut crate::lil::_lil_value_t) -> * mut crate::lil::_lil_value_t {
    let mut i: u64 = 0;
    let mut wcode: * const i8 = 0 as *const std::os::raw::c_char;
    if argc < 2 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as lil_value_t }
    wcode =
        lil_to_string(*argv.offset(argc.wrapping_sub(1 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                       isize));
    i = 0 as std::os::raw::c_int as size_t;
    while i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) < argc {
        let mut vname: * const i8 =
            lil_to_string(*argv.offset(i as isize));
        let mut v: * mut crate::lil::_lil_var_t = 0 as *mut _lil_var_t;
        if !(*vname.offset(0 as std::os::raw::c_int as isize) == 0) {
            v =
                lil_find_var(lil, (*lil).env,
                             lil_to_string(*argv.offset(i as isize)));
            if v.is_null() {
                v =
                    lil_set_var(lil, vname, 0 as lil_value_t,
                                2 as std::os::raw::c_int)
            }
            free((*v).w as *mut std::os::raw::c_void);
            (*v).w =
                if *wcode.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                       0 {
                    strclone(wcode)
                } else { 0 as *mut std::os::raw::c_char }
        }
        i = i.wrapping_add(1)
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn register_stdcmds(mut lil: * mut crate::lil::_lil_t) {
    lil_register(lil, b"reflect\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_reflect));
    lil_register(lil, b"func\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_func));
    lil_register(lil, b"rename\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_rename));
    lil_register(lil, b"unusedname\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_unusedname));
    lil_register(lil, b"quote\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_quote));
    lil_register(lil, b"set\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_set));
    lil_register(lil, b"local\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_local));
    lil_register(lil, b"write\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_write));
    lil_register(lil, b"print\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_print));
    lil_register(lil, b"eval\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_eval));
    lil_register(lil, b"topeval\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_topeval));
    lil_register(lil, b"upeval\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_upeval));
    lil_register(lil, b"downeval\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_downeval));
    lil_register(lil, b"enveval\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_enveval));
    lil_register(lil, b"jaileval\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_jaileval));
    lil_register(lil, b"count\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_count));
    lil_register(lil, b"index\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_index));
    lil_register(lil, b"indexof\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_indexof));
    lil_register(lil, b"filter\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_filter));
    lil_register(lil, b"list\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_list));
    lil_register(lil, b"append\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_append));
    lil_register(lil, b"slice\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_slice));
    lil_register(lil, b"subst\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_subst));
    lil_register(lil, b"concat\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_concat));
    lil_register(lil, b"foreach\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_foreach));
    lil_register(lil, b"return\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_return));
    lil_register(lil, b"result\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_result));
    lil_register(lil, b"expr\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_expr));
    lil_register(lil, b"inc\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_inc));
    lil_register(lil, b"dec\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_dec));
    lil_register(lil, b"read\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_read));
    lil_register(lil, b"store\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_store));
    lil_register(lil, b"if\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_if));
    lil_register(lil, b"while\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_while));
    lil_register(lil, b"for\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_for));
    lil_register(lil, b"char\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_char));
    lil_register(lil, b"charat\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_charat));
    lil_register(lil, b"codeat\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_codeat));
    lil_register(lil, b"substr\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_substr));
    lil_register(lil, b"strpos\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_strpos));
    lil_register(lil, b"length\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_length));
    lil_register(lil, b"trim\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_trim));
    lil_register(lil, b"ltrim\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_ltrim));
    lil_register(lil, b"rtrim\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_rtrim));
    lil_register(lil, b"strcmp\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_strcmp));
    lil_register(lil, b"streq\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_streq));
    lil_register(lil, b"repstr\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_repstr));
    lil_register(lil, b"split\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_split));
    lil_register(lil, b"try\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_try));
    lil_register(lil, b"error\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_error));
    lil_register(lil, b"exit\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_exit));
    lil_register(lil, b"source\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_source));
    lil_register(lil, b"lmap\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_lmap));
    lil_register(lil, b"rand\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_rand));
    lil_register(lil, b"catcher\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_catcher));
    lil_register(lil, b"watch\x00" as *const u8 as *const std::os::raw::c_char,
                 Some(fnc_watch));
    (*lil).syscmds = (*lil).cmds;
}
use crate::laertes_rt::*;

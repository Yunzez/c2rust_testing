
extern "C" {
    
    
    
    #[no_mangle]
    static mut stderr: * mut crate::src::gifread::gifread::_IO_FILE;
    #[no_mangle]
    fn fprintf(_: * mut crate::src::gifread::gifread::_IO_FILE, _: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn getc(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn ferror(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn realloc(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: * mut core::ffi::c_void);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn memcmp(_: * const core::ffi::c_void, _: * const core::ffi::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
}
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: * mut std::os::raw::c_char,
    pub _IO_read_end: * mut std::os::raw::c_char,
    pub _IO_read_base: * mut std::os::raw::c_char,
    pub _IO_write_base: * mut std::os::raw::c_char,
    pub _IO_write_ptr: * mut std::os::raw::c_char,
    pub _IO_write_end: * mut std::os::raw::c_char,
    pub _IO_buf_base: * mut std::os::raw::c_char,
    pub _IO_buf_end: * mut std::os::raw::c_char,
    pub _IO_save_base: * mut std::os::raw::c_char,
    pub _IO_backup_base: * mut std::os::raw::c_char,
    pub _IO_save_end: * mut std::os::raw::c_char,
    pub _markers: * mut crate::src::pngxtern::pngxread::_IO_marker,
    pub _chain: * mut crate::src::gifread::gifread::_IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: std::os::raw::c_long,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: * mut core::ffi::c_void,
    pub _offset: std::os::raw::c_long,
    pub _codecvt: * mut crate::src::pnmio::pnmout::_IO_codecvt,
    pub _wide_data: * mut crate::src::pngxtern::pngxrpnm::_IO_wide_data,
    pub _freeres_list: * mut crate::src::gifread::gifread::_IO_FILE,
    pub _freeres_buf: * mut core::ffi::c_void,
    pub __pad5: std::os::raw::c_ulong,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
impl _IO_FILE {
    pub const fn new() -> Self {
        _IO_FILE {
        _flags: 0,
        _IO_read_ptr: (0 as * mut std::os::raw::c_char),
        _IO_read_end: (0 as * mut std::os::raw::c_char),
        _IO_read_base: (0 as * mut std::os::raw::c_char),
        _IO_write_base: (0 as * mut std::os::raw::c_char),
        _IO_write_ptr: (0 as * mut std::os::raw::c_char),
        _IO_write_end: (0 as * mut std::os::raw::c_char),
        _IO_buf_base: (0 as * mut std::os::raw::c_char),
        _IO_buf_end: (0 as * mut std::os::raw::c_char),
        _IO_save_base: (0 as * mut std::os::raw::c_char),
        _IO_backup_base: (0 as * mut std::os::raw::c_char),
        _IO_save_end: (0 as * mut std::os::raw::c_char),
        _markers: (0 as * mut crate::src::pngxtern::pngxread::_IO_marker),
        _chain: (0 as * mut crate::src::gifread::gifread::_IO_FILE),
        _fileno: 0,
        _flags2: 0,
        _old_offset: 0,
        _cur_column: 0,
        _vtable_offset: 0,
        _shortbuf: [0,],
        _lock: (0 as * mut core::ffi::c_void),
        _offset: 0,
        _codecvt: (0 as * mut crate::src::pnmio::pnmout::_IO_codecvt),
        _wide_data: (0 as * mut crate::src::pngxtern::pngxrpnm::_IO_wide_data),
        _freeres_list: (0 as * mut crate::src::gifread::gifread::_IO_FILE),
        _freeres_buf: (0 as * mut core::ffi::c_void),
        __pad5: 0,
        _mode: 0,
        _unused2: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for _IO_FILE {
    fn default() -> Self { _IO_FILE::new() }
}

pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFScreen {
    pub Width: std::os::raw::c_uint,
    pub Height: std::os::raw::c_uint,
    pub GlobalColorFlag: std::os::raw::c_uint,
    pub ColorResolution: std::os::raw::c_uint,
    pub SortFlag: std::os::raw::c_uint,
    pub GlobalNumColors: std::os::raw::c_uint,
    pub Background: std::os::raw::c_uint,
    pub PixelAspectRatio: std::os::raw::c_uint,
    pub GlobalColorTable: [std::os::raw::c_uchar; 768],
}
impl GIFScreen {
    pub const fn new() -> Self {
        GIFScreen {
        Width: 0,
        Height: 0,
        GlobalColorFlag: 0,
        ColorResolution: 0,
        SortFlag: 0,
        GlobalNumColors: 0,
        Background: 0,
        PixelAspectRatio: 0,
        GlobalColorTable: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for GIFScreen {
    fn default() -> Self { GIFScreen::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFImage {
    pub Screen: * mut crate::src::gifread::gifread::GIFScreen,
    pub LeftPos: std::os::raw::c_uint,
    pub TopPos: std::os::raw::c_uint,
    pub Width: std::os::raw::c_uint,
    pub Height: std::os::raw::c_uint,
    pub LocalColorFlag: std::os::raw::c_uint,
    pub InterlaceFlag: std::os::raw::c_uint,
    pub SortFlag: std::os::raw::c_uint,
    pub LocalNumColors: std::os::raw::c_uint,
    pub LocalColorTable: [std::os::raw::c_uchar; 768],
    pub Rows: * mut * mut std::os::raw::c_uchar,
}
impl GIFImage {
    pub const fn new() -> Self {
        GIFImage {
        Screen: (0 as * mut crate::src::gifread::gifread::GIFScreen),
        LeftPos: 0,
        TopPos: 0,
        Width: 0,
        Height: 0,
        LocalColorFlag: 0,
        InterlaceFlag: 0,
        SortFlag: 0,
        LocalNumColors: 0,
        LocalColorTable: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        Rows: (0 as * mut * mut std::os::raw::c_uchar)
        }
    }
}

impl std::default::Default for GIFImage {
    fn default() -> Self { GIFImage::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFExtension {
    pub Screen: * mut crate::src::gifread::gifread::GIFScreen,
    pub Buffer: * mut std::os::raw::c_uchar,
    pub BufferSize: std::os::raw::c_uint,
    pub Label: std::os::raw::c_uchar,
}
impl GIFExtension {
    pub const fn new() -> Self {
        GIFExtension {
        Screen: (0 as * mut crate::src::gifread::gifread::GIFScreen),
        Buffer: (0 as * mut std::os::raw::c_uchar),
        BufferSize: 0,
        Label: 0
        }
    }
}

impl std::default::Default for GIFExtension {
    fn default() -> Self { GIFExtension::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFGraphicCtlExt {
    pub DisposalMethod: std::os::raw::c_uint,
    pub InputFlag: std::os::raw::c_uint,
    pub TransparentFlag: std::os::raw::c_uint,
    pub DelayTime: std::os::raw::c_uint,
    pub Transparent: std::os::raw::c_uint,
}
impl GIFGraphicCtlExt {
    pub const fn new() -> Self {
        GIFGraphicCtlExt {
        DisposalMethod: 0,
        InputFlag: 0,
        TransparentFlag: 0,
        DelayTime: 0,
        Transparent: 0
        }
    }
}

impl std::default::Default for GIFGraphicCtlExt {
    fn default() -> Self { GIFGraphicCtlExt::new() }
}

/*
 * Reads the GIF screen and the global color table.
 */
#[no_mangle]
pub unsafe extern "C" fn GIFReadScreen<'a1>(mut screen: Option<&'a1 mut crate::src::gifread::gifread::GIFScreen>,
                                       mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut buffer: [u8; 7] = [0; 7];
    ReadBytes(buffer.as_mut_ptr(), 6 as std::os::raw::c_int as std::os::raw::c_uint, stream);
    if memcmp(buffer.as_mut_ptr() as *const std::os::raw::c_void,
              b"GIF\x00" as *const u8 as *const std::os::raw::c_char as
                  *const std::os::raw::c_void, 3 as std::os::raw::c_int as std::os::raw::c_ulong) !=
           0 as std::os::raw::c_int {
        GIFError.expect("non-null function pointer")(b"Not a GIF file\x00" as
                                                         *const u8 as
                                                         *const std::os::raw::c_char);
    }
    if memcmp(buffer.as_mut_ptr().offset(3 as std::os::raw::c_int as isize) as
                  *const std::os::raw::c_void,
              b"87a\x00" as *const u8 as *const std::os::raw::c_char as
                  *const std::os::raw::c_void, 3 as std::os::raw::c_int as std::os::raw::c_ulong) !=
           0 as std::os::raw::c_int &&
           memcmp(buffer.as_mut_ptr().offset(3 as std::os::raw::c_int as isize) as
                      *const std::os::raw::c_void,
                  b"89a\x00" as *const u8 as *const std::os::raw::c_char as
                      *const std::os::raw::c_void, 3 as std::os::raw::c_int as std::os::raw::c_ulong)
               != 0 as std::os::raw::c_int {
        GIFWarning.expect("non-null function pointer")(b"Invalid GIF version number, not \"87a\" or \"89a\"\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
    }
    ReadBytes(buffer.as_mut_ptr(), 7 as std::os::raw::c_int as std::os::raw::c_uint, stream);
    (*(borrow_mut(&mut screen)).unwrap()).Width =
        (*buffer.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                         isize).offset(0 as std::os::raw::c_int as
                                                           isize) as
             std::os::raw::c_int +
             ((*buffer.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                               isize).offset(1 as std::os::raw::c_int
                                                                 as isize) as
                   std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*(borrow_mut(&mut screen)).unwrap()).Height =
        (*buffer.as_mut_ptr().offset(2 as std::os::raw::c_int as
                                         isize).offset(0 as std::os::raw::c_int as
                                                           isize) as
             std::os::raw::c_int +
             ((*buffer.as_mut_ptr().offset(2 as std::os::raw::c_int as
                                               isize).offset(1 as std::os::raw::c_int
                                                                 as isize) as
                   std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*(borrow_mut(&mut screen)).unwrap()).GlobalColorFlag =
        if buffer[4 as std::os::raw::c_int as usize] as std::os::raw::c_int &
               0x80 as std::os::raw::c_int != 0 {
            1 as std::os::raw::c_int
        } else { 0 as std::os::raw::c_int } as std::os::raw::c_uint;
    (*(borrow_mut(&mut screen)).unwrap()).ColorResolution =
        (((buffer[4 as std::os::raw::c_int as usize] as std::os::raw::c_int &
               0x70 as std::os::raw::c_int) >> 3 as std::os::raw::c_int) + 1 as std::os::raw::c_int)
            as std::os::raw::c_uint;
    (*(borrow_mut(&mut screen)).unwrap()).SortFlag =
        if buffer[4 as std::os::raw::c_int as usize] as std::os::raw::c_int &
               0x8 as std::os::raw::c_int != 0 {
            1 as std::os::raw::c_int
        } else { 0 as std::os::raw::c_int } as std::os::raw::c_uint;
    (*(borrow_mut(&mut screen)).unwrap()).GlobalNumColors =
        ((2 as std::os::raw::c_int) <<
             (buffer[4 as std::os::raw::c_int as usize] as std::os::raw::c_int &
                  0x7 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*(borrow_mut(&mut screen)).unwrap()).Background = buffer[5 as std::os::raw::c_int as usize] as std::os::raw::c_uint;
    (*(borrow_mut(&mut screen)).unwrap()).PixelAspectRatio =
        buffer[6 as std::os::raw::c_int as usize] as std::os::raw::c_uint;
    if (*(borrow(& screen)).unwrap()).GlobalColorFlag != 0 {
        ReadBytes((*(borrow_mut(&mut screen)).unwrap()).GlobalColorTable.as_mut_ptr(),
                  (3 as std::os::raw::c_int as
                       std::os::raw::c_uint).wrapping_mul((*(borrow(& screen)).unwrap()).GlobalNumColors),
                  stream);
    }
    if (*(borrow(& screen)).unwrap()).Width == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           (*(borrow(& screen)).unwrap()).Height == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        GIFError.expect("non-null function pointer")(b"Invalid dimensions in GIF image\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
    }
    if (*(borrow(& screen)).unwrap()).Background > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*(borrow(& screen)).unwrap()).GlobalColorFlag != 0 &&
               (*(borrow(& screen)).unwrap()).Background >= (*(borrow(& screen)).unwrap()).GlobalNumColors ||
               (*(borrow(& screen)).unwrap()).GlobalColorFlag == 0 {
            /* too noisy */
            (*(borrow_mut(&mut screen)).unwrap()).Background = 0 as std::os::raw::c_int as std::os::raw::c_uint
        }
    };
}
/*
 * Initializes a GIF image object.
 */
#[no_mangle]
pub unsafe extern "C" fn GIFInitImage<'a1>(mut image: Option<&'a1 mut crate::src::gifread::gifread::GIFImage>,
                                      mut screen: * mut crate::src::gifread::gifread::GIFScreen,
                                      mut rows: * mut * mut std::os::raw::c_uchar) {
    (*(borrow_mut(&mut image)).unwrap()).Screen = screen;
    (*(borrow_mut(&mut image)).unwrap()).Rows = rows;
}
/*
 * Destroys a GIF image object.
 */
#[no_mangle]
pub extern "C" fn GIFDestroyImage(mut image: * mut crate::src::gifread::gifread::GIFImage) {
    /* nothing to do */
}
/*
 * Reads the next GIF block into an image or extension object.
 */
#[no_mangle]
pub unsafe extern "C" fn GIFReadNextBlock<'a1>(mut image: * mut crate::src::gifread::gifread::GIFImage,
                                          mut ext: Option<&'a1 mut crate::src::gifread::gifread::GIFExtension>,
                                          mut stream: * mut crate::src::gifread::gifread::_IO_FILE)
 -> std::os::raw::c_int {
    let mut ch: i32 = 0;
    let mut foundBogus: i32 = 0;
    foundBogus = 0 as std::os::raw::c_int;
    loop  {
        ch = GetByte(stream);
        match ch {
            44 => {
                /* ',' */
                GIFReadNextImage(image, stream);
                return ch
            }
            33 => {
                /* '!' */
                GIFReadNextExtension(borrow_mut(&mut ext), stream);
                return ch
            }
            59 => {
                /* ';' */
                return ch
            }
            _ => {
                if foundBogus == 0 {
                    GIFWarning.expect("non-null function pointer")(b"Bogus data in GIF file\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
                }
                foundBogus = 1 as std::os::raw::c_int
            }
        }
    };
}
/*
 * Reads the next GIF image and local color table.
 */
unsafe extern "C" fn GIFReadNextImage(mut image: * mut crate::src::gifread::gifread::GIFImage,
                                      mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut screen: * mut crate::src::gifread::gifread::GIFScreen = (0 as * mut crate::src::gifread::gifread::GIFScreen);
    let mut buffer: [u8; 9] = [0; 9];
    ReadBytes(buffer.as_mut_ptr(), 9 as std::os::raw::c_int as std::os::raw::c_uint, stream);
    if image.is_null() { GIFSkipDataBlocks(stream); return }
    (*image).LeftPos =
        (*buffer.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                         isize).offset(0 as std::os::raw::c_int as
                                                           isize) as
             std::os::raw::c_int +
             ((*buffer.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                               isize).offset(1 as std::os::raw::c_int
                                                                 as isize) as
                   std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*image).TopPos =
        (*buffer.as_mut_ptr().offset(2 as std::os::raw::c_int as
                                         isize).offset(0 as std::os::raw::c_int as
                                                           isize) as
             std::os::raw::c_int +
             ((*buffer.as_mut_ptr().offset(2 as std::os::raw::c_int as
                                               isize).offset(1 as std::os::raw::c_int
                                                                 as isize) as
                   std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*image).Width =
        (*buffer.as_mut_ptr().offset(4 as std::os::raw::c_int as
                                         isize).offset(0 as std::os::raw::c_int as
                                                           isize) as
             std::os::raw::c_int +
             ((*buffer.as_mut_ptr().offset(4 as std::os::raw::c_int as
                                               isize).offset(1 as std::os::raw::c_int
                                                                 as isize) as
                   std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*image).Height =
        (*buffer.as_mut_ptr().offset(6 as std::os::raw::c_int as
                                         isize).offset(0 as std::os::raw::c_int as
                                                           isize) as
             std::os::raw::c_int +
             ((*buffer.as_mut_ptr().offset(6 as std::os::raw::c_int as
                                               isize).offset(1 as std::os::raw::c_int
                                                                 as isize) as
                   std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*image).LocalColorFlag =
        if buffer[8 as std::os::raw::c_int as usize] as std::os::raw::c_int &
               0x80 as std::os::raw::c_int != 0 {
            1 as std::os::raw::c_int
        } else { 0 as std::os::raw::c_int } as std::os::raw::c_uint;
    (*image).InterlaceFlag =
        if buffer[8 as std::os::raw::c_int as usize] as std::os::raw::c_int &
               0x40 as std::os::raw::c_int != 0 {
            1 as std::os::raw::c_int
        } else { 0 as std::os::raw::c_int } as std::os::raw::c_uint;
    (*image).SortFlag =
        if buffer[8 as std::os::raw::c_int as usize] as std::os::raw::c_int &
               0x20 as std::os::raw::c_int != 0 {
            1 as std::os::raw::c_int
        } else { 0 as std::os::raw::c_int } as std::os::raw::c_uint;
    (*image).LocalNumColors =
        if (*image).LocalColorFlag != 0 {
            ((2 as std::os::raw::c_int)) <<
                (buffer[8 as std::os::raw::c_int as usize] as std::os::raw::c_int &
                     0x7 as std::os::raw::c_int)
        } else { 0 as std::os::raw::c_int } as std::os::raw::c_uint;
    if (*image).LocalColorFlag != 0 {
        ReadBytes((*image).LocalColorTable.as_mut_ptr(),
                  (3 as std::os::raw::c_int as
                       std::os::raw::c_uint).wrapping_mul((*image).LocalNumColors),
                  stream);
    }
    screen = (*image).Screen;
    if (*image).Width == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           (*image).Height == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           (*image).LeftPos.wrapping_add((*image).Width) > (*screen).Width ||
           (*image).TopPos.wrapping_add((*image).Height) > (*screen).Height {
        GIFError.expect("non-null function pointer")(b"Invalid dimensions in GIF image\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
    }
    GIFReadImageData(image, stream);
}
unsafe extern "C" fn GIFReadImageData(mut image: * mut crate::src::gifread::gifread::GIFImage,
                                      mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut minCodeSize: i32 = 0;
    let mut rows: * mut * mut u8 = 0 as *mut *mut std::os::raw::c_uchar;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut interlaced: u32 = 0;
    let mut colors: * mut u8 = 0 as *mut std::os::raw::c_uchar;
    let mut numColors: u32 = 0;
    let mut xpos: u32 = 0;
    let mut ypos: u32 = 0;
    let mut pass: i32 = 0;
    let mut val: i32 = 0;
    /* Initialize the compression routines. */
    minCodeSize = GetByte(stream);
    if minCodeSize >= 12 as std::os::raw::c_int {
        GIFError.expect("non-null function pointer")(b"Invalid LZW code size\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
    }
    if LZWDecodeByte(1 as std::os::raw::c_int, minCodeSize, stream) < 0 as std::os::raw::c_int
       {
        GIFError.expect("non-null function pointer")(b"Error decoding GIF image\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
    }
    /* Ignore the picture if it is "uninteresting". */
    rows = (*image).Rows;
    if rows.is_null() {
        /* This is faster, but possible LZW errors may go undetected. */
        GIFSkipDataBlocks(stream);
        return
    }
    width = (*image).Width;
    height = (*image).Height;
    interlaced = (*image).InterlaceFlag;
    GIFGetColorTable(Some(&mut colors), Some(&mut numColors), image);
    ypos = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    xpos = ypos;
    pass = 0 as std::os::raw::c_int;
    loop  {
        val = LZWDecodeByte(0 as std::os::raw::c_int, minCodeSize, stream);
        if !(val >= 0 as std::os::raw::c_int) { break ; }
        if val as std::os::raw::c_uint >= numColors {
            GIFWarning.expect("non-null function pointer")(b"Pixel value out of range in GIF image\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            val =
                numColors.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as
                    std::os::raw::c_int
        }
        *(*rows.offset(ypos as isize)).offset(xpos as isize) =
            val as std::os::raw::c_uchar;
        xpos = xpos.wrapping_add(1);
        if xpos == width {
            xpos = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            if interlaced != 0 {
                match pass {
                    0 | 1 => {
                        ypos =
                            ypos.wrapping_add(8 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    2 => {
                        ypos =
                            ypos.wrapping_add(4 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    3 => {
                        ypos =
                            ypos.wrapping_add(2 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    _ => { }
                }
                if ypos >= height {
                    pass += 1;
                    match pass {
                        1 => { ypos = 4 as std::os::raw::c_int as std::os::raw::c_uint }
                        2 => { ypos = 2 as std::os::raw::c_int as std::os::raw::c_uint }
                        3 => { ypos = 1 as std::os::raw::c_int as std::os::raw::c_uint }
                        _ => { break ; }
                    }
                }
            } else { ypos = ypos.wrapping_add(1) }
        }
        if ypos >= height { break ; }
    }
    /* Ignore the trailing garbage. */
    while LZWDecodeByte(0 as std::os::raw::c_int, minCodeSize, stream) >=
              0 as std::os::raw::c_int {
    };
}
static mut DataBlockSize: std::os::raw::c_int = 0; unsafe fn laertes_init_DataBlockSize() {
DataBlockSize = 0 as std::os::raw::c_int;}//;
unsafe extern "C" fn GIFReadDataBlock(mut buffer: * mut std::os::raw::c_uchar,
                                      mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int {
    let mut count: i32 = 0;
    count = GetByte(stream);
    DataBlockSize = count;
    if count > 0 as std::os::raw::c_int {
        ReadBytes(buffer, count as std::os::raw::c_uint, stream);
    }
    return count;
}
unsafe extern "C" fn GIFSkipDataBlocks(mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut count: i32 = 0;
    let mut buffer: [u8; 256] = [0; 256];
    loop  {
        count = GetByte(stream);
        if count > 0 as std::os::raw::c_int {
            ReadBytes(buffer.as_mut_ptr(), count as std::os::raw::c_uint, stream);
        } else { return }
    };
}
unsafe extern "C" fn LZWGetCode(mut code_size: std::os::raw::c_int,
                                mut init_flag: std::os::raw::c_int,
                                mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int {
    static mut buffer: [std::os::raw::c_uchar; 280] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_buffer() {
buffer = [0; 280];}//;
    static mut curbit: std::os::raw::c_int = 0; unsafe fn laertes_init_curbit() {
curbit = 0;}//;
    static mut lastbit: std::os::raw::c_int = 0; unsafe fn laertes_init_lastbit() {
lastbit = 0;}//;
    static mut done: std::os::raw::c_int = 0; unsafe fn laertes_init_done() {
done = 0;}//;
    static mut last_byte: std::os::raw::c_int = 0; unsafe fn laertes_init_last_byte() {
last_byte = 0;}//;
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    if init_flag != 0 {
        curbit = 0 as std::os::raw::c_int;
        lastbit = 0 as std::os::raw::c_int;
        last_byte = 2 as std::os::raw::c_int;
        done = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    if curbit + code_size >= lastbit {
        if done != 0 {
            if curbit >= lastbit {
                GIFError.expect("non-null function pointer")(b"Ran off the end of input bits in LZW decoding\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const std::os::raw::c_char);
            }
            return -(1 as std::os::raw::c_int)
        }
        buffer[0 as std::os::raw::c_int as usize] =
            buffer[(last_byte - 2 as std::os::raw::c_int) as usize];
        buffer[1 as std::os::raw::c_int as usize] =
            buffer[(last_byte - 1 as std::os::raw::c_int) as usize];
        count =
            GIFReadDataBlock(&mut *buffer.as_mut_ptr().offset(2 as std::os::raw::c_int
                                                                  as isize),
                             stream);
        if count == 0 as std::os::raw::c_int { done = 1 as std::os::raw::c_int }
        last_byte = 2 as std::os::raw::c_int + count;
        curbit = curbit - lastbit + 16 as std::os::raw::c_int;
        lastbit = (2 as std::os::raw::c_int + count) * 8 as std::os::raw::c_int
    }
    ret = 0 as std::os::raw::c_int;
    i = curbit;
    j = 0 as std::os::raw::c_int;
    while j < code_size {
        ret |=
            ((buffer[(i / 8 as std::os::raw::c_int) as usize] as std::os::raw::c_int &
                  (1 as std::os::raw::c_int) << i % 8 as std::os::raw::c_int !=
                  0 as std::os::raw::c_int) as std::os::raw::c_int) << j;
        i += 1;
        j += 1
    }
    curbit += code_size;
    return ret;
}
unsafe extern "C" fn LZWDecodeByte(mut init_flag: std::os::raw::c_int,
                                   mut input_code_size: std::os::raw::c_int,
                                   mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int {
    static mut fresh: std::os::raw::c_int = 0; unsafe fn laertes_init_fresh() {
fresh = 0 as std::os::raw::c_int;}//;
    static mut code_size: std::os::raw::c_int = 0; unsafe fn laertes_init_code_size() {
code_size = 0;}//;
    static mut set_code_size: std::os::raw::c_int = 0; unsafe fn laertes_init_set_code_size() {
set_code_size = 0;}//;
    static mut max_code: std::os::raw::c_int = 0; unsafe fn laertes_init_max_code() {
max_code = 0;}//;
    static mut max_code_size: std::os::raw::c_int = 0; unsafe fn laertes_init_max_code_size() {
max_code_size = 0;}//;
    static mut firstcode: std::os::raw::c_int = 0; unsafe fn laertes_init_firstcode() {
firstcode = 0;}//;
    static mut oldcode: std::os::raw::c_int = 0; unsafe fn laertes_init_oldcode() {
oldcode = 0;}//;
    static mut clear_code: std::os::raw::c_int = 0; unsafe fn laertes_init_clear_code() {
clear_code = 0;}//;
    static mut end_code: std::os::raw::c_int = 0; unsafe fn laertes_init_end_code() {
end_code = 0;}//;
    static mut table: [[std::os::raw::c_int; 4096]; 2] = [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],]; unsafe fn laertes_init_table() {
table = [[0; 4096]; 2];}//;
    static mut stack: [std::os::raw::c_int; 8192] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_stack() {
stack = [0; 8192];}//;
    static mut sp: * mut std::os::raw::c_int =
        (0 as * mut std::os::raw::c_int); unsafe fn laertes_init_sp() {
sp = (0 as * mut i32);}//;
    let mut code: i32 = 0;
    let mut incode: i32 = 0;
    let mut i: i32 = 0;
    if init_flag != 0 {
        fresh = 1 as std::os::raw::c_int;
        set_code_size = input_code_size;
        code_size = set_code_size + 1 as std::os::raw::c_int;
        clear_code = (1 as std::os::raw::c_int) << set_code_size;
        end_code = clear_code + 1 as std::os::raw::c_int;
        max_code_size = 2 as std::os::raw::c_int * clear_code;
        max_code = clear_code + 2 as std::os::raw::c_int;
        LZWGetCode(0 as std::os::raw::c_int, 1 as std::os::raw::c_int, stream);
        i = 0 as std::os::raw::c_int;
        while i < clear_code {
            table[0 as std::os::raw::c_int as usize][i as usize] = 0 as std::os::raw::c_int;
            table[1 as std::os::raw::c_int as usize][i as usize] = i;
            i += 1
        }
        while i <=
                  ((1 as std::os::raw::c_int) << 12 as std::os::raw::c_int) - 1 as std::os::raw::c_int
              {
            table[0 as std::os::raw::c_int as usize][i as usize] = 0 as std::os::raw::c_int;
            table[1 as std::os::raw::c_int as usize][i as usize] = 0 as std::os::raw::c_int;
            i += 1
        }
        sp = stack.as_mut_ptr();
        return 0 as std::os::raw::c_int
    } else {
        if fresh != 0 {
            fresh = 0 as std::os::raw::c_int;
            loop  {
                oldcode = LZWGetCode(code_size, 0 as std::os::raw::c_int, stream);
                firstcode = oldcode;
                if !(firstcode == clear_code) { break ; }
            }
            return firstcode
        }
    }
    if sp > stack.as_mut_ptr() { sp = sp.offset(-1); return *sp }
    loop  {
        code = LZWGetCode(code_size, 0 as std::os::raw::c_int, stream);
        if !(code >= 0 as std::os::raw::c_int) { break ; }
        if code == clear_code {
            i = 0 as std::os::raw::c_int;
            while i < clear_code {
                table[0 as std::os::raw::c_int as usize][i as usize] =
                    0 as std::os::raw::c_int;
                table[1 as std::os::raw::c_int as usize][i as usize] = i;
                i += 1
            }
            while i <=
                      ((1 as std::os::raw::c_int) << 12 as std::os::raw::c_int) -
                          1 as std::os::raw::c_int {
                table[0 as std::os::raw::c_int as usize][i as usize] =
                    0 as std::os::raw::c_int;
                table[1 as std::os::raw::c_int as usize][i as usize] =
                    0 as std::os::raw::c_int;
                i += 1
            }
            code_size = set_code_size + 1 as std::os::raw::c_int;
            max_code_size = 2 as std::os::raw::c_int * clear_code;
            max_code = clear_code + 2 as std::os::raw::c_int;
            sp = stack.as_mut_ptr();
            oldcode = LZWGetCode(code_size, 0 as std::os::raw::c_int, stream);
            firstcode = oldcode;
            return firstcode
        } else {
            if code == end_code {
                let mut count: i32 = 0;
                let mut buffer: [u8; 260] = [0; 260];
                if DataBlockSize == 0 as std::os::raw::c_int {
                    return -(2 as std::os::raw::c_int)
                }
                loop  {
                    count = GIFReadDataBlock(buffer.as_mut_ptr(), stream);
                    if !(count > 0 as std::os::raw::c_int) { break ; }
                }
                /* too noisy */
                return -(2 as std::os::raw::c_int)
            }
        }
        incode = code;
        if code >= max_code {
            let mut fresh0 = sp;
            sp = sp.offset(1);
            *fresh0 = firstcode;
            code = oldcode
        }
        while code >= clear_code {
            let mut fresh1 = sp;
            sp = sp.offset(1);
            *fresh1 = table[1 as std::os::raw::c_int as usize][code as usize];
            if code == table[0 as std::os::raw::c_int as usize][code as usize] ||
                   sp.offset_from(stack.as_mut_ptr()) as std::os::raw::c_long
                       as size_t >=
                       (::std::mem::size_of::<[std::os::raw::c_int; 8192]>() as
                            std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_int>()
                                                            as std::os::raw::c_ulong)
               {
                GIFError.expect("non-null function pointer")(b"Circular dependency found in LZW table\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const std::os::raw::c_char);
            }
            code = table[0 as std::os::raw::c_int as usize][code as usize]
        }
        firstcode = table[1 as std::os::raw::c_int as usize][code as usize];
        let mut fresh2 = sp;
        sp = sp.offset(1);
        *fresh2 = firstcode;
        code = max_code;
        if code <=
               ((1 as std::os::raw::c_int) << 12 as std::os::raw::c_int) - 1 as std::os::raw::c_int {
            table[0 as std::os::raw::c_int as usize][code as usize] = oldcode;
            table[1 as std::os::raw::c_int as usize][code as usize] = firstcode;
            max_code += 1;
            if max_code >= max_code_size &&
                   max_code_size <=
                       ((1 as std::os::raw::c_int) << 12 as std::os::raw::c_int) -
                           1 as std::os::raw::c_int {
                max_code_size *= 2 as std::os::raw::c_int;
                code_size += 1
            }
        }
        oldcode = incode;
        if sp > stack.as_mut_ptr() { sp = sp.offset(-1); return *sp }
    }
    return code;
}
/*
 * The GIF spec says that if neither global nor local
 * color maps are present, the decoder should use a system
 * default map, which should have black and white as the
 * first two colors. So we use black, white, red, green, blue,
 * yellow, purple and cyan.
 * Missing color tables are not a common case, and are not
 * handled by most GIF readers.
 */
static mut DefaultColorTable: [std::os::raw::c_uchar; 24] =
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_DefaultColorTable() {
DefaultColorTable = [0 as std::os::raw::c_int as std::os::raw::c_uchar, 0 as std::os::raw::c_int as std::os::raw::c_uchar,
     0 as std::os::raw::c_int as std::os::raw::c_uchar, 255 as std::os::raw::c_int as std::os::raw::c_uchar,
     255 as std::os::raw::c_int as std::os::raw::c_uchar, 255 as std::os::raw::c_int as std::os::raw::c_uchar,
     255 as std::os::raw::c_int as std::os::raw::c_uchar, 0 as std::os::raw::c_int as std::os::raw::c_uchar,
     0 as std::os::raw::c_int as std::os::raw::c_uchar, 0 as std::os::raw::c_int as std::os::raw::c_uchar,
     255 as std::os::raw::c_int as std::os::raw::c_uchar, 255 as std::os::raw::c_int as std::os::raw::c_uchar,
     0 as std::os::raw::c_int as std::os::raw::c_uchar, 255 as std::os::raw::c_int as std::os::raw::c_uchar,
     0 as std::os::raw::c_int as std::os::raw::c_uchar, 255 as std::os::raw::c_int as std::os::raw::c_uchar,
     0 as std::os::raw::c_int as std::os::raw::c_uchar, 255 as std::os::raw::c_int as std::os::raw::c_uchar,
     0 as std::os::raw::c_int as std::os::raw::c_uchar, 0 as std::os::raw::c_int as std::os::raw::c_uchar,
     255 as std::os::raw::c_int as std::os::raw::c_uchar, 255 as std::os::raw::c_int as std::os::raw::c_uchar,
     255 as std::os::raw::c_int as std::os::raw::c_uchar, 0 as std::os::raw::c_int as std::os::raw::c_uchar];}//;
/*
 * Returns the local or the global color table (whichever is applicable),
 * or a predefined color table if both of these tables are missing.
 */
#[no_mangle]
pub unsafe extern "C" fn GIFGetColorTable<'a1, 'a2>(mut colors: Option<&'a1 mut * mut std::os::raw::c_uchar>,
                                          mut numColors: Option<&'a2 mut std::os::raw::c_uint>,
                                          mut image: * mut crate::src::gifread::gifread::GIFImage) {
    let mut screen: * mut crate::src::gifread::gifread::GIFScreen = (0 as * mut crate::src::gifread::gifread::GIFScreen);
    if (*image).LocalColorFlag != 0 {
        *(borrow_mut(&mut colors)).unwrap() = (*image).LocalColorTable.as_mut_ptr();
        *(borrow_mut(&mut numColors)).unwrap() = (*image).LocalNumColors;
        return
    }
    screen = (*image).Screen;
    if (*screen).GlobalColorFlag != 0 {
        *(borrow_mut(&mut colors)).unwrap() = (*screen).GlobalColorTable.as_mut_ptr();
        *(borrow_mut(&mut numColors)).unwrap() = (*screen).GlobalNumColors;
        return
    }
    *(borrow_mut(&mut colors)).unwrap() = DefaultColorTable.as_mut_ptr();
    *(borrow_mut(&mut numColors)).unwrap() =
        (::std::mem::size_of::<[std::os::raw::c_uchar; 24]>() as
             std::os::raw::c_ulong).wrapping_div(3 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_uint;
}
/*
 * Initializes a GIF extension object.
 */
#[no_mangle]
pub unsafe extern "C" fn GIFInitExtension<'a1>(mut ext: Option<&'a1 mut crate::src::gifread::gifread::GIFExtension>,
                                          mut screen: * mut crate::src::gifread::gifread::GIFScreen,
                                          mut initBufferSize: std::os::raw::c_uint) {
    let mut newBuffer: * mut u8 = 0 as *mut std::os::raw::c_uchar;
    (*(borrow_mut(&mut ext)).unwrap()).Screen = screen;
    if initBufferSize > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        newBuffer =
            malloc(initBufferSize as std::os::raw::c_ulong) as *mut std::os::raw::c_uchar;
        if newBuffer.is_null() { ErrorAlloc(); }
        (*(borrow_mut(&mut ext)).unwrap()).Buffer = newBuffer;
        (*(borrow_mut(&mut ext)).unwrap()).BufferSize = initBufferSize
    } else {
        (*(borrow_mut(&mut ext)).unwrap()).Buffer = 0 as *mut std::os::raw::c_uchar;
        (*(borrow_mut(&mut ext)).unwrap()).BufferSize = 0 as std::os::raw::c_int as std::os::raw::c_uint
    };
}
/*
 * Destroys a GIF extension object.
 */
#[no_mangle]
pub unsafe extern "C" fn GIFDestroyExtension(mut ext: * mut crate::src::gifread::gifread::GIFExtension) {
    free((*ext).Buffer as *mut std::os::raw::c_void);
}
/*
 * Reads the next GIF extension.
 */
unsafe extern "C" fn GIFReadNextExtension<'a1>(mut ext: Option<&'a1 mut crate::src::gifread::gifread::GIFExtension>,
                                          mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut newBuffer: * mut u8 = 0 as *mut std::os::raw::c_uchar;
    let mut newBufferSize: u32 = 0;
    let mut offset: u32 = 0;
    let mut len: u32 = 0;
    let mut count: i32 = 0;
    let mut label: i32 = 0;
    label = GetByte(stream);
    if borrow(& ext).is_none() { GIFSkipDataBlocks(stream); return }
    (*(borrow_mut(&mut ext)).unwrap()).Label = label as std::os::raw::c_uchar;
    offset = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    len = (*(borrow_mut(&mut ext)).unwrap()).BufferSize;
    loop  {
        if len < 255 as std::os::raw::c_int as std::os::raw::c_uint {
            newBufferSize =
                (*(borrow(& ext)).unwrap()).BufferSize.wrapping_add(1024 as std::os::raw::c_int as
                                                   std::os::raw::c_uint);
            newBuffer =
                realloc((*(borrow_mut(&mut ext)).unwrap()).Buffer as *mut std::os::raw::c_void,
                        newBufferSize as std::os::raw::c_ulong) as *mut std::os::raw::c_uchar;
            if newBuffer.is_null() { ErrorAlloc(); }
            (*(borrow_mut(&mut ext)).unwrap()).BufferSize = newBufferSize;
            (*(borrow_mut(&mut ext)).unwrap()).Buffer = newBuffer;
            len = len.wrapping_add(1024 as std::os::raw::c_int as std::os::raw::c_uint)
        }
        count =
            GIFReadDataBlock((*(borrow(& ext)).unwrap()).Buffer.offset(offset as isize), stream);
        if count == 0 as std::os::raw::c_int { break ; }
        offset = offset.wrapping_add(count as std::os::raw::c_uint);
        len = len.wrapping_sub(count as std::os::raw::c_uint)
    };
}
/*
 * Constructs a GIF graphic control extension object
 * from a raw extension object.
 */
#[no_mangle]
pub unsafe extern "C" fn GIFGetGraphicCtl<'a1, 'a2>(mut graphicExt:
                                              Option<&'a1 mut crate::src::gifread::gifread::GIFGraphicCtlExt>,
                                          mut ext: Option<&'a2 mut crate::src::gifread::gifread::GIFExtension>) {
    let mut buffer: * mut u8 = 0 as *mut std::os::raw::c_uchar;
    if (*(borrow(& ext)).unwrap()).Label as std::os::raw::c_int != 0xf9 as std::os::raw::c_int {
        GIFWarning.expect("non-null function pointer")(b"Not a graphic control extension in GIF file\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
        return
    }
    if (*(borrow(& ext)).unwrap()).BufferSize < 4 as std::os::raw::c_int as std::os::raw::c_uint {
        GIFWarning.expect("non-null function pointer")(b"Broken graphic control extension in GIF file\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
        return
    }
    buffer = (*(borrow_mut(&mut ext)).unwrap()).Buffer;
    (*(borrow_mut(&mut graphicExt)).unwrap()).DisposalMethod =
        (*buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int >>
             2 as std::os::raw::c_int & 0x7 as std::os::raw::c_int) as std::os::raw::c_uint;
    (*(borrow_mut(&mut graphicExt)).unwrap()).InputFlag =
        (*buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int >>
             1 as std::os::raw::c_int & 0x1 as std::os::raw::c_int) as std::os::raw::c_uint;
    (*(borrow_mut(&mut graphicExt)).unwrap()).TransparentFlag =
        (*buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int &
             0x1 as std::os::raw::c_int) as std::os::raw::c_uint;
    (*(borrow_mut(&mut graphicExt)).unwrap()).DelayTime =
        (*buffer.offset(1 as std::os::raw::c_int as
                            isize).offset(0 as std::os::raw::c_int as isize) as
             std::os::raw::c_int +
             ((*buffer.offset(1 as std::os::raw::c_int as
                                  isize).offset(1 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
    (*(borrow_mut(&mut graphicExt)).unwrap()).Transparent =
        *buffer.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
}
/*
 * Reads a byte.
 */
unsafe extern "C" fn GetByte(mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int {
    let mut ch: i32 = 0;
    ch = getc(stream);
    if ch == -(1 as std::os::raw::c_int) { ErrorRead(stream); }
    return ch;
}
/*
 * Reads a sequence of bytes.
 */
unsafe extern "C" fn ReadBytes(mut buffer: * mut std::os::raw::c_uchar,
                               mut count: std::os::raw::c_uint,
                               mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    if fread(buffer as *mut std::os::raw::c_void, count as std::os::raw::c_ulong,
             1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        ErrorRead(stream);
    };
}
/*
 * Fails with an out-of-memory error.
 */
unsafe extern "C" fn ErrorAlloc() {
    GIFError.expect("non-null function pointer")(b"Out of memory in GIF decoder\x00"
                                                     as *const u8 as
                                                     *const std::os::raw::c_char);
}
/*
 * Fails with a read error.
 */
unsafe extern "C" fn ErrorRead(mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    if ferror(stream) != 0 {
        GIFError.expect("non-null function pointer")(b"Error reading GIF file\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
    } else {
        GIFError.expect("non-null function pointer")(b"Unexpected end of GIF file\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
    };
}
/*
 * The default error handler.
 */
unsafe extern "C" fn DefaultError(mut message: * const std::os::raw::c_char) {
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const std::os::raw::c_char, message);
    exit(1 as std::os::raw::c_int);
}
/*
 * The default warning handler.
 */
unsafe extern "C" fn DefaultWarning(mut message: * const std::os::raw::c_char) {
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const std::os::raw::c_char, message);
}
/*
 * The error handling callback.
 */
#[no_mangle]
pub static mut GIFError:
           Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()> =
    None; unsafe fn laertes_init_GIFError() {
GIFError = unsafe {
        Some(DefaultError)
    };}//;
/*
 * The warning handling callback.
 */
#[no_mangle]
pub static mut GIFWarning:
           Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()> =
    None; unsafe fn laertes_init_GIFWarning() {
GIFWarning = unsafe {
        Some(DefaultWarning)
    };}//;
use crate::laertes_rt::*;

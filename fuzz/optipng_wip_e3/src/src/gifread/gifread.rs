










use std::os::raw::c_int;

use std::os::raw::c_char;

extern "C" {
    
    
    
    
    static mut stderr: *mut FILE;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn getc(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    
    fn ferror(__stream: *mut FILE) -> std::os::raw::c_int;
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn free(__ptr: *mut std::os::raw::c_void);
    
    fn exit(_: std::os::raw::c_int) -> !;
    
    fn memcmp(_: *const std::os::raw::c_void, _: *const std::os::raw::c_void,
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
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::os::raw::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFImage {
    pub Screen: *mut GIFScreen,
    pub LeftPos: std::os::raw::c_uint,
    pub TopPos: std::os::raw::c_uint,
    pub Width: std::os::raw::c_uint,
    pub Height: std::os::raw::c_uint,
    pub LocalColorFlag: std::os::raw::c_uint,
    pub InterlaceFlag: std::os::raw::c_uint,
    pub SortFlag: std::os::raw::c_uint,
    pub LocalNumColors: std::os::raw::c_uint,
    pub LocalColorTable: [std::os::raw::c_uchar; 768],
    pub Rows: *mut *mut std::os::raw::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFExtension {
    pub Screen: *mut GIFScreen,
    pub Buffer: *mut std::os::raw::c_uchar,
    pub BufferSize: std::os::raw::c_uint,
    pub Label: std::os::raw::c_uchar,
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
/*
 * Reads the GIF screen and the global color table.
 */

pub fn GIFReadScreen(screen: &mut GIFScreen, stream: *mut FILE) {
    let mut buffer: [u8; 7] = [0; 7];
    unsafe { ReadBytes(buffer.as_mut_ptr(), 6, stream); }
    
    if &buffer[0..3] != b"GIF" {
        unsafe {
            GIFError.expect("non-null function pointer")(b"Not a GIF file\x00" as *const u8 as *const c_char);
        }
    }
    
    if &buffer[3..6] != b"87a" && &buffer[3..6] != b"89a" {
        unsafe {
            GIFWarning.expect("non-null function pointer")(b"Invalid GIF version number, not \"87a\" or \"89a\"\x00" as *const u8 as *const c_char);
        }
    }
    
    unsafe { ReadBytes(buffer.as_mut_ptr(), 7, stream); }
    
    screen.Width = u32::from(buffer[0]) + (u32::from(buffer[1]) << 8);
    screen.Height = u32::from(buffer[2]) + (u32::from(buffer[3]) << 8);
    
    screen.GlobalColorFlag = if buffer[4] & 0x80 != 0 { 1 } else { 0 };
    screen.ColorResolution = ((buffer[4] & 0x70) >> 3) as u32 + 1;
    screen.SortFlag = if buffer[4] & 0x08 != 0 { 1 } else { 0 };
    screen.GlobalNumColors = 1 << (buffer[4] & 0x07);
    screen.Background = u32::from(buffer[5]);
    screen.PixelAspectRatio = u32::from(buffer[6]);
    
    if screen.GlobalColorFlag != 0 {
        unsafe { ReadBytes(screen.GlobalColorTable.as_mut_ptr(), 3 * screen.GlobalNumColors, stream); }
    }
    
    if screen.Width == 0 || screen.Height == 0 {
        unsafe {
            GIFError.expect("non-null function pointer")(b"Invalid dimensions in GIF image\x00" as *const u8 as *const c_char);
        }
    }
    
    if screen.Background > 0 {
        if screen.GlobalColorFlag != 0 && screen.Background >= screen.GlobalNumColors {
            screen.Background = 0;
        }
    }
}

/*
 * Initializes a GIF image object.
 */

pub fn GIFInitImage(image: &mut GIFImage, screen: &mut GIFScreen, rows: &mut Vec<*mut u8>) {
    image.Screen = screen;
    image.Rows = rows.as_mut_ptr();
}

/*
 * Destroys a GIF image object.
 */

pub fn GIFDestroyImage(image: &mut GIFImage) {
    /* nothing to do */
}

/*
 * Reads the next GIF block into an image or extension object.
 */

pub fn GIFReadNextBlock(image: &mut GIFImage, ext: &mut GIFExtension, stream: *mut FILE) -> c_int {
    let mut ch: c_int;
    let mut found_bogus: bool = false;

    loop {
        ch = GetByte(stream);
        match ch {
            44 => {
                // ','
                unsafe {
                    GIFReadNextImage(image, stream);
                }
                return ch;
            }
            33 => {
                // '!'
                unsafe {
                    GIFReadNextExtension(ext, stream);
                }
                return ch;
            }
            59 => {
                // ';'
                return ch;
            }
            _ => {
                if !found_bogus {
                    unsafe {
                        if let Some(warning_fn) = GIFWarning {
                            warning_fn(b"Bogus data in GIF file\x00" as *const u8 as *const c_char);
                        }
                    }
                }
                found_bogus = true;
            }
        }
    }
}

/*
 * Reads the next GIF image and local color table.
 */
unsafe extern "C" fn GIFReadNextImage(mut image: *mut GIFImage,
                                      mut stream: *mut FILE) {
    let mut screen: *mut GIFScreen = 0 as *mut GIFScreen;
    let mut buffer: [std::os::raw::c_uchar; 9] = [0; 9];
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
    GIFReadImageData(&mut *image, &mut *stream);
}
fn GIFReadImageData(image: &mut GIFImage, stream: &mut FILE) {
    let mut min_code_size: i32 = 0;
    let width: u32 = image.Width;
    let height: u32 = image.Height;
    let interlaced: u32 = image.InterlaceFlag;
    let mut colors: Vec<u8> = Vec::new();
    let mut num_colors: u32 = 0;
    let mut xpos: u32 = 0;
    let mut ypos: u32 = 0;
    let mut pass: i32 = 0;

    /* Initialize the compression routines. */
    min_code_size = GetByte(stream);
    if min_code_size >= 12 {
        unsafe {
            if let Some(error_fn) = GIFError {
                error_fn(b"Invalid LZW code size\x00" as *const u8 as *const i8);
            }
        }
    }
    if unsafe { LZWDecodeByte(1, min_code_size, stream) } < 0 {
        unsafe {
            if let Some(error_fn) = GIFError {
                error_fn(b"Error decoding GIF image\x00" as *const u8 as *const i8);
            }
        }
    }

    /* Ignore the picture if it is "uninteresting". */
    let rows_ptr = unsafe { (*image).Rows };
    if rows_ptr.is_null() {
        /* This is faster, but possible LZW errors may go undetected. */
        unsafe { GIFSkipDataBlocks(stream) };
        return;
    }
    
    let rows_len = height as usize;
    let rows: &mut [*mut u8] = unsafe { std::slice::from_raw_parts_mut(rows_ptr, rows_len) };

    unsafe {
        GIFGetColorTable(colors.as_mut_ptr() as *mut *mut u8, &mut num_colors, image);
    }
    ypos = 0;
    xpos = ypos;

    loop {
        let val = unsafe { LZWDecodeByte(0, min_code_size, stream) };
        if val < 0 { break; }
        let mut pixel_value = val as u32;
        if pixel_value >= num_colors {
            unsafe {
                if let Some(warning_fn) = GIFWarning {
                    warning_fn(b"Pixel value out of range in GIF image\x00" as *const u8 as *const i8);
                }
            }
            pixel_value = num_colors.wrapping_sub(1);
        }
        unsafe {
            *rows[ypos as usize].offset(xpos as isize) = pixel_value as u8;
        }
        xpos = xpos.wrapping_add(1);
        if xpos == width {
            xpos = 0;
            if interlaced != 0 {
                match pass {
                    0 | 1 => {
                        ypos = ypos.wrapping_add(8);
                    }
                    2 => {
                        ypos = ypos.wrapping_add(4);
                    }
                    3 => {
                        ypos = ypos.wrapping_add(2);
                    }
                    _ => {}
                }
                if ypos >= height {
                    pass += 1;
                    match pass {
                        1 => { ypos = 4; }
                        2 => { ypos = 2; }
                        3 => { ypos = 1; }
                        _ => { break; }
                    }
                }
            } else {
                ypos = ypos.wrapping_add(1);
            }
        }
        if ypos >= height { break; }
    }

    /* Ignore the trailing garbage. */
    while unsafe { LZWDecodeByte(0, min_code_size, stream) } >= 0 {}
}

static mut DataBlockSize: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn GIFReadDataBlock(mut buffer: *mut std::os::raw::c_uchar,
                                      mut stream: *mut FILE) -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let count = GetByte(stream);
    DataBlockSize = count;
    if count > 0 as std::os::raw::c_int {
        ReadBytes(buffer, count as std::os::raw::c_uint, stream);
    }
    return count;
}
fn GIFSkipDataBlocks(stream: *mut FILE) {
    let mut count: std::os::raw::c_int;
    let mut buffer: [std::os::raw::c_uchar; 256] = [0; 256];
    loop {
        count = GetByte(stream);
        if count > 0 {
            unsafe {
                ReadBytes(buffer.as_mut_ptr(), count as std::os::raw::c_uint, stream);
            }
        } else {
            return;
        }
    }
}

unsafe extern "C" fn LZWGetCode(mut code_size: std::os::raw::c_int,
                                mut init_flag: std::os::raw::c_int,
                                mut stream: *mut FILE) -> std::os::raw::c_int {
    static mut buffer: [std::os::raw::c_uchar; 280] = [0; 280];
    static mut curbit: std::os::raw::c_int = 0;
    static mut lastbit: std::os::raw::c_int = 0;
    static mut done: std::os::raw::c_int = 0;
    static mut last_byte: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
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
                                   mut stream: *mut FILE) -> std::os::raw::c_int {
    static mut fresh: std::os::raw::c_int = 0 as std::os::raw::c_int;
    static mut code_size: std::os::raw::c_int = 0;
    static mut set_code_size: std::os::raw::c_int = 0;
    static mut max_code: std::os::raw::c_int = 0;
    static mut max_code_size: std::os::raw::c_int = 0;
    static mut firstcode: std::os::raw::c_int = 0;
    static mut oldcode: std::os::raw::c_int = 0;
    static mut clear_code: std::os::raw::c_int = 0;
    static mut end_code: std::os::raw::c_int = 0;
    static mut table: [[std::os::raw::c_int; 4096]; 2] = [[0; 4096]; 2];
    static mut stack: [std::os::raw::c_int; 8192] = [0; 8192];
    static mut sp: *mut std::os::raw::c_int =
        0 as *const std::os::raw::c_int as *mut std::os::raw::c_int;
    let mut code: std::os::raw::c_int = 0;
    let mut incode: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
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
                let mut count: std::os::raw::c_int = 0;
                let mut buffer: [std::os::raw::c_uchar; 260] = [0; 260];
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
            let fresh0 = sp;
            sp = sp.offset(1);
            *fresh0 = firstcode;
            code = oldcode
        }
        while code >= clear_code {
            let fresh1 = sp;
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
        let fresh2 = sp;
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
    [0 as std::os::raw::c_int as std::os::raw::c_uchar, 0 as std::os::raw::c_int as std::os::raw::c_uchar,
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
     255 as std::os::raw::c_int as std::os::raw::c_uchar, 0 as std::os::raw::c_int as std::os::raw::c_uchar];
/*
 * Returns the local or the global color table (whichever is applicable),
 * or a predefined color table if both of these tables are missing.
 */

pub unsafe extern "C" fn GIFGetColorTable(mut colors: *mut *mut std::os::raw::c_uchar,
                                          mut numColors: *mut std::os::raw::c_uint,
                                          mut image: *mut GIFImage) {
    let mut screen: *mut GIFScreen = 0 as *mut GIFScreen;
    if (*image).LocalColorFlag != 0 {
        *colors = (*image).LocalColorTable.as_mut_ptr();
        *numColors = (*image).LocalNumColors;
        return
    }
    screen = (*image).Screen;
    if (*screen).GlobalColorFlag != 0 {
        *colors = (*screen).GlobalColorTable.as_mut_ptr();
        *numColors = (*screen).GlobalNumColors;
        return
    }
    *colors = DefaultColorTable.as_mut_ptr();
    *numColors =
        (::std::mem::size_of::<[std::os::raw::c_uchar; 24]>() as
             std::os::raw::c_ulong).wrapping_div(3 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_uint;
}
/*
 * Initializes a GIF extension object.
 */

pub unsafe extern "C" fn GIFInitExtension(mut ext: *mut GIFExtension,
                                          mut screen: *mut GIFScreen,
                                          mut initBufferSize: std::os::raw::c_uint) {
    let mut newBuffer: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    (*ext).Screen = screen;
    if initBufferSize > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        newBuffer =
            malloc(initBufferSize as std::os::raw::c_ulong) as *mut std::os::raw::c_uchar;
        if newBuffer.is_null() { ErrorAlloc(); }
        (*ext).Buffer = newBuffer;
        (*ext).BufferSize = initBufferSize
    } else {
        (*ext).Buffer = 0 as *mut std::os::raw::c_uchar;
        (*ext).BufferSize = 0 as std::os::raw::c_int as std::os::raw::c_uint
    };
}
/*
 * Destroys a GIF extension object.
 */

pub fn GIFDestroyExtension(ext: &mut GIFExtension) {
    let buffer = ext.Buffer;
    std::mem::drop(buffer); // Assuming Buffer is a type that implements Drop
}

/*
 * Reads the next GIF extension.
 */
fn GIFReadNextExtension(ext: *mut GIFExtension, stream: *mut FILE) {
    let mut new_buffer: Vec<u8> = Vec::new();
    let mut offset: usize = 0;
    let mut len: usize;

    let label: i32 = GetByte(stream);
    
    if ext.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    
    let ext_ref = unsafe { &mut *ext };
    ext_ref.Label = label as u8;
    len = ext_ref.BufferSize as usize;

    loop {
        if len < 255 {
            let new_buffer_size = ext_ref.BufferSize.wrapping_add(1024) as usize;
            new_buffer.resize(new_buffer_size, 0);
            ext_ref.BufferSize = new_buffer_size as u32;
            len += 1024;
        }
        
        let count;
        unsafe {
            count = GIFReadDataBlock(new_buffer.as_mut_ptr().offset(offset as isize), stream);
        }
        
        if count == 0 {
            break;
        }
        
        offset += count as usize;
        len -= count as usize;
    }
}

/*
 * Constructs a GIF graphic control extension object
 * from a raw extension object.
 */

pub fn GIFGetGraphicCtl(graphic_ext: &mut GIFGraphicCtlExt, ext: &GIFExtension) {
    if ext.Label != 0xf9 {
        if let Some(warning) = unsafe { GIFWarning } {
            unsafe {
                warning(b"Not a graphic control extension in GIF file\x00" as *const u8 as *const c_char);
            }
        }
        return;
    }
    if ext.BufferSize < 4 {
        if let Some(warning) = unsafe { GIFWarning } {
            unsafe {
                warning(b"Broken graphic control extension in GIF file\x00" as *const u8 as *const c_char);
            }
        }
        return;
    }
    
    let buffer = unsafe { std::slice::from_raw_parts(ext.Buffer, ext.BufferSize as usize) };
    graphic_ext.DisposalMethod = ((buffer[0] >> 2) & 0x7) as u32;
    graphic_ext.InputFlag = ((buffer[0] >> 1) & 0x1) as u32;
    graphic_ext.TransparentFlag = (buffer[0] & 0x1) as u32;
    graphic_ext.DelayTime = (buffer[1] as u32 + ((buffer[2] as u32) << 8)) as u32;
    graphic_ext.Transparent = buffer[3] as u32;
}

/*
 * Reads a byte.
 */
fn GetByte(stream: *mut FILE) -> std::os::raw::c_int {
    let mut ch: std::os::raw::c_int = 0;
    ch = unsafe { getc(stream) };
    if ch == -(1 as std::os::raw::c_int) { 
        unsafe { ErrorRead(stream) }; 
    }
    ch
}

/*
 * Reads a sequence of bytes.
 */
unsafe extern "C" fn ReadBytes(mut buffer: *mut std::os::raw::c_uchar,
                               mut count: std::os::raw::c_uint,
                               mut stream: *mut FILE) {
    if fread(buffer as *mut std::os::raw::c_void, count as std::os::raw::c_ulong,
             1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        ErrorRead(stream);
    };
}
/*
 * Fails with an out-of-memory error.
 */
fn ErrorAlloc() {
    unsafe {
        if let Some(error_handler) = GIFError {
            error_handler("Out of memory in GIF decoder".as_ptr() as *const c_char);
        }
    }
}

/*
 * Fails with a read error.
 */
fn ErrorRead(stream: *mut FILE) {
    unsafe {
        if ferror(stream) != 0 {
            if let Some(error_handler) = GIFError {
                error_handler("Error reading GIF file\0".as_ptr() as *const std::os::raw::c_char);
            }
        } else {
            if let Some(error_handler) = GIFError {
                error_handler("Unexpected end of GIF file\0".as_ptr() as *const std::os::raw::c_char);
            }
        }
    }
}

/*
 * The default error handler.
 */
unsafe extern "C" fn DefaultError(mut message: *const std::os::raw::c_char) {
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const std::os::raw::c_char, message);
    exit(1 as std::os::raw::c_int);
}
/*
 * The default warning handler.
 */
unsafe extern "C" fn DefaultWarning(mut message: *const std::os::raw::c_char) {
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const std::os::raw::c_char, message);
}
/*
 * The error handling callback.
 */

pub static mut GIFError:
           Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> ()> =
    unsafe {
        Some(DefaultError as
                 unsafe extern "C" fn(_: *const std::os::raw::c_char) -> ())
    };
/*
 * The warning handling callback.
 */

pub static mut GIFWarning:
           Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> ()> =
    unsafe {
        Some(DefaultWarning as
                 unsafe extern "C" fn(_: *const std::os::raw::c_char) -> ())
    };

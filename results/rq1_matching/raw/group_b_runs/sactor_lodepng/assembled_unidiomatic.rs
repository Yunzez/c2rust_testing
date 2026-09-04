// SACTOR × lodepng (run 1, 2026-09-02, PARTIAL, non-building): verbatim concatenation of the
// UNIDIOMATIC-phase output for the 74 functions SACTOR reached before it stopped (of 235 in lodepng.c):
//   - enums/global_vars/structs/functions/*.rs from run1_result/lodepng.c__06d02e56/translated_code_unidiomatic/
//     (53 functions, each SACTOR-verified against the 4 encode→decode samples), plus
//   - the LAST attempt of the 21 functions SACTOR did not verify (20 failed 6/6 on the duplicate
//     `LodePNGColorType` scaffold conflict, lodepng_gtofl compiled but failed to link), extracted from
//     run1_result/logs/sactor-20260902T043148.jsonl by scripts/rq1_sactor_extract_log_rust.py
//     (run1_extracted_rust/). See RUN.md. The idiomatic phase was never reached (no tool name map).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]

// --- enums/LodePNGColorType.rs
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum LodePNGColorType {
    /// grayscale: 1,2,4,8,16 bit
    LCT_GREY = 0,
    /// RGB: 8,16 bit
    LCT_RGB = 2,
    /// palette: 1,2,4,8 bit
    LCT_PALETTE = 3,
    /// grayscale with alpha: 8,16 bit
    LCT_GREY_ALPHA = 4,
    /// RGB with alpha: 8,16 bit
    LCT_RGBA = 6,
    /// See C docs: may represent any invalid byte value 0–255; don't use directly.
    LCT_MAX_OCTET_VALUE = 255,
}

// --- enums/LodePNGFilterStrategy.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LodePNGFilterStrategy {
    LFS_ZERO = 0,
    LFS_ONE = 1,
    LFS_TWO = 2,
    LFS_THREE = 3,
    LFS_FOUR = 4,
    LFS_MINSUM = 5,
    LFS_ENTROPY = 6,
    LFS_BRUTE_FORCE = 7,
    LFS_PREDEFINED = 8,
}

// --- global_vars/ADAM7_DX.rs
static ADAM7_DX: [u32; 7] = [8, 8, 4, 4, 2, 2, 1];

// --- global_vars/ADAM7_DY.rs
static ADAM7_DY: [u32; 7] = [8, 8, 8, 4, 4, 2, 2];

// --- global_vars/ADAM7_IX.rs
static ADAM7_IX: [u32; 7] = [0, 4, 0, 2, 0, 1, 0];

// --- global_vars/ADAM7_IY.rs
static ADAM7_IY: [u32; 7] = [0, 0, 4, 0, 2, 0, 1];

// --- global_vars/HASH_BIT_MASK.rs
const HASH_BIT_MASK: libc::c_uint = 65535;

// --- global_vars/MAX_SUPPORTED_DEFLATE_LENGTH.rs
static MAX_SUPPORTED_DEFLATE_LENGTH: libc::size_t = 258;

// --- global_vars/lodepng_crc32_table.rs
static lodepng_crc32_table: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    1996959894 as ::core::ffi::c_uint,
    3993919788 as ::core::ffi::c_uint,
    2567524794 as ::core::ffi::c_uint,
    124634137 as ::core::ffi::c_uint,
    1886057615 as ::core::ffi::c_uint,
    3915621685 as ::core::ffi::c_uint,
    2657392035 as ::core::ffi::c_uint,
    249268274 as ::core::ffi::c_uint,
    2044508324 as ::core::ffi::c_uint,
    3772115230 as ::core::ffi::c_uint,
    2547177864 as ::core::ffi::c_uint,
    162941995 as ::core::ffi::c_uint,
    2125561021 as ::core::ffi::c_uint,
    3887607047 as ::core::ffi::c_uint,
    2428444049 as ::core::ffi::c_uint,
    498536548 as ::core::ffi::c_uint,
    1789927666 as ::core::ffi::c_uint,
    4089016648 as ::core::ffi::c_uint,
    2227061214 as ::core::ffi::c_uint,
    450548861 as ::core::ffi::c_uint,
    1843258603 as ::core::ffi::c_uint,
    4107580753 as ::core::ffi::c_uint,
    2211677639 as ::core::ffi::c_uint,
    325883990 as ::core::ffi::c_uint,
    1684777152 as ::core::ffi::c_uint,
    4251122042 as ::core::ffi::c_uint,
    2321926636 as ::core::ffi::c_uint,
    335633487 as ::core::ffi::c_uint,
    1661365465 as ::core::ffi::c_uint,
    4195302755 as ::core::ffi::c_uint,
    2366115317 as ::core::ffi::c_uint,
    997073096 as ::core::ffi::c_uint,
    1281953886 as ::core::ffi::c_uint,
    3579855332 as ::core::ffi::c_uint,
    2724688242 as ::core::ffi::c_uint,
    1006888145 as ::core::ffi::c_uint,
    1258607687 as ::core::ffi::c_uint,
    3524101629 as ::core::ffi::c_uint,
    2768942443 as ::core::ffi::c_uint,
    901097722 as ::core::ffi::c_uint,
    1119000684 as ::core::ffi::c_uint,
    3686517206 as ::core::ffi::c_uint,
    2898065728 as ::core::ffi::c_uint,
    853044451 as ::core::ffi::c_uint,
    1172266101 as ::core::ffi::c_uint,
    3705015759 as ::core::ffi::c_uint,
    2882616665 as ::core::ffi::c_uint,
    651767980 as ::core::ffi::c_uint,
    1373503546 as ::core::ffi::c_uint,
    3369554304 as ::core::ffi::c_uint,
    3218104598 as ::core::ffi::c_uint,
    565507253 as ::core::ffi::c_uint,
    1454621731 as ::core::ffi::c_uint,
    3485111705 as ::core::ffi::c_uint,
    3099436303 as ::core::ffi::c_uint,
    671266974 as ::core::ffi::c_uint,
    1594198024 as ::core::ffi::c_uint,
    3322730930 as ::core::ffi::c_uint,
    2970347812 as ::core::ffi::c_uint,
    795835527 as ::core::ffi::c_uint,
    1483230225 as ::core::ffi::c_uint,
    3244367275 as ::core::ffi::c_uint,
    3060149565 as ::core::ffi::c_uint,
    1994146192 as ::core::ffi::c_uint,
    31158534 as ::core::ffi::c_uint,
    2563907772 as ::core::ffi::c_uint,
    4023717930 as ::core::ffi::c_uint,
    1907459465 as ::core::ffi::c_uint,
    112637215 as ::core::ffi::c_uint,
    2680153253 as ::core::ffi::c_uint,
    3904427059 as ::core::ffi::c_uint,
    2013776290 as ::core::ffi::c_uint,
    251722036 as ::core::ffi::c_uint,
    2517215374 as ::core::ffi::c_uint,
    3775830040 as ::core::ffi::c_uint,
    2137656763 as ::core::ffi::c_uint,
    141376813 as ::core::ffi::c_uint,
    2439277719 as ::core::ffi::c_uint,
    3865271297 as ::core::ffi::c_uint,
    1802195444 as ::core::ffi::c_uint,
    476864866 as ::core::ffi::c_uint,
    2238001368 as ::core::ffi::c_uint,
    4066508878 as ::core::ffi::c_uint,
    1812370925 as ::core::ffi::c_uint,
    453092731 as ::core::ffi::c_uint,
    2181625025 as ::core::ffi::c_uint,
    4111451223 as ::core::ffi::c_uint,
    1706088902 as ::core::ffi::c_uint,
    314042704 as ::core::ffi::c_uint,
    2344532202 as ::core::ffi::c_uint,
    4240017532 as ::core::ffi::c_uint,
    1658658271 as ::core::ffi::c_uint,
    366619977 as ::core::ffi::c_uint,
    2362670323 as ::core::ffi::c_uint,
    4224994405 as ::core::ffi::c_uint,
    1303535960 as ::core::ffi::c_uint,
    984961486 as ::core::ffi::c_uint,
    2747007092 as ::core::ffi::c_uint,
    3569037538 as ::core::ffi::c_uint,
    1256170817 as ::core::ffi::c_uint,
    1037604311 as ::core::ffi::c_uint,
    2765210733 as ::core::ffi::c_uint,
    3554079995 as ::core::ffi::c_uint,
    1131014506 as ::core::ffi::c_uint,
    879679996 as ::core::ffi::c_uint,
    2909243462 as ::core::ffi::c_uint,
    3663771856 as ::core::ffi::c_uint,
    1141124467 as ::core::ffi::c_uint,
    855842277 as ::core::ffi::c_uint,
    2852801631 as ::core::ffi::c_uint,
    3708648649 as ::core::ffi::c_uint,
    1342533948 as ::core::ffi::c_uint,
    654459306 as ::core::ffi::c_uint,
    3188396048 as ::core::ffi::c_uint,
    3373015174 as ::core::ffi::c_uint,
    1466479909 as ::core::ffi::c_uint,
    544179635 as ::core::ffi::c_uint,
    3110523913 as ::core::ffi::c_uint,
    3462522015 as ::core::ffi::c_uint,
    1591671054 as ::core::ffi::c_uint,
    702138776 as ::core::ffi::c_uint,
    2966460450 as ::core::ffi::c_uint,
    3352799412 as ::core::ffi::c_uint,
    1504918807 as ::core::ffi::c_uint,
    783551873 as ::core::ffi::c_uint,
    3082640443 as ::core::ffi::c_uint,
    3233442989 as ::core::ffi::c_uint,
    3988292384 as ::core::ffi::c_uint,
    2596254646 as ::core::ffi::c_uint,
    62317068 as ::core::ffi::c_uint,
    1957810842 as ::core::ffi::c_uint,
    3939845945 as ::core::ffi::c_uint,
    2647816111 as ::core::ffi::c_uint,
    81470997 as ::core::ffi::c_uint,
    1943803523 as ::core::ffi::c_uint,
    3814918930 as ::core::ffi::c_uint,
    2489596804 as ::core::ffi::c_uint,
    225274430 as ::core::ffi::c_uint,
    2053790376 as ::core::ffi::c_uint,
    3826175755 as ::core::ffi::c_uint,
    2466906013 as ::core::ffi::c_uint,
    167816743 as ::core::ffi::c_uint,
    2097651377 as ::core::ffi::c_uint,
    4027552580 as ::core::ffi::c_uint,
    2265490386 as ::core::ffi::c_uint,
    503444072 as ::core::ffi::c_uint,
    1762050814 as ::core::ffi::c_uint,
    4150417245 as ::core::ffi::c_uint,
    2154129355 as ::core::ffi::c_uint,
    426522225 as ::core::ffi::c_uint,
    1852507879 as ::core::ffi::c_uint,
    4275313526 as ::core::ffi::c_uint,
    2312317920 as ::core::ffi::c_uint,
    282753626 as ::core::ffi::c_uint,
    1742555852 as ::core::ffi::c_uint,
    4189708143 as ::core::ffi::c_uint,
    2394877945 as ::core::ffi::c_uint,
    397917763 as ::core::ffi::c_uint,
    1622183637 as ::core::ffi::c_uint,
    3604390888 as ::core::ffi::c_uint,
    2714866558 as ::core::ffi::c_uint,
    953729732 as ::core::ffi::c_uint,
    1340076626 as ::core::ffi::c_uint,
    3518719985 as ::core::ffi::c_uint,
    2797360999 as ::core::ffi::c_uint,
    1068828381 as ::core::ffi::c_uint,
    1219638859 as ::core::ffi::c_uint,
    3624741850 as ::core::ffi::c_uint,
    2936675148 as ::core::ffi::c_uint,
    906185462 as ::core::ffi::c_uint,
    1090812512 as ::core::ffi::c_uint,
    3747672003 as ::core::ffi::c_uint,
    2825379669 as ::core::ffi::c_uint,
    829329135 as ::core::ffi::c_uint,
    1181335161 as ::core::ffi::c_uint,
    3412177804 as ::core::ffi::c_uint,
    3160834842 as ::core::ffi::c_uint,
    628085408 as ::core::ffi::c_uint,
    1382605366 as ::core::ffi::c_uint,
    3423369109 as ::core::ffi::c_uint,
    3138078467 as ::core::ffi::c_uint,
    570562233 as ::core::ffi::c_uint,
    1426400815 as ::core::ffi::c_uint,
    3317316542 as ::core::ffi::c_uint,
    2998733608 as ::core::ffi::c_uint,
    733239954 as ::core::ffi::c_uint,
    1555261956 as ::core::ffi::c_uint,
    3268935591 as ::core::ffi::c_uint,
    3050360625 as ::core::ffi::c_uint,
    752459403 as ::core::ffi::c_uint,
    1541320221 as ::core::ffi::c_uint,
    2607071920 as ::core::ffi::c_uint,
    3965973030 as ::core::ffi::c_uint,
    1969922972 as ::core::ffi::c_uint,
    40735498 as ::core::ffi::c_uint,
    2617837225 as ::core::ffi::c_uint,
    3943577151 as ::core::ffi::c_uint,
    1913087877 as ::core::ffi::c_uint,
    83908371 as ::core::ffi::c_uint,
    2512341634 as ::core::ffi::c_uint,
    3803740692 as ::core::ffi::c_uint,
    2075208622 as ::core::ffi::c_uint,
    213261112 as ::core::ffi::c_uint,
    2463272603 as ::core::ffi::c_uint,
    3855990285 as ::core::ffi::c_uint,
    2094854071 as ::core::ffi::c_uint,
    198958881 as ::core::ffi::c_uint,
    2262029012 as ::core::ffi::c_uint,
    4057260610 as ::core::ffi::c_uint,
    1759359992 as ::core::ffi::c_uint,
    534414190 as ::core::ffi::c_uint,
    2176718541 as ::core::ffi::c_uint,
    4139329115 as ::core::ffi::c_uint,
    1873836001 as ::core::ffi::c_uint,
    414664567 as ::core::ffi::c_uint,
    2282248934 as ::core::ffi::c_uint,
    4279200368 as ::core::ffi::c_uint,
    1711684554 as ::core::ffi::c_uint,
    285281116 as ::core::ffi::c_uint,
    2405801727 as ::core::ffi::c_uint,
    4167216745 as ::core::ffi::c_uint,
    1634467795 as ::core::ffi::c_uint,
    376229701 as ::core::ffi::c_uint,
    2685067896 as ::core::ffi::c_uint,
    3608007406 as ::core::ffi::c_uint,
    1308918612 as ::core::ffi::c_uint,
    956543938 as ::core::ffi::c_uint,
    2808555105 as ::core::ffi::c_uint,
    3495958263 as ::core::ffi::c_uint,
    1231636301 as ::core::ffi::c_uint,
    1047427035 as ::core::ffi::c_uint,
    2932959818 as ::core::ffi::c_uint,
    3654703836 as ::core::ffi::c_uint,
    1088359270 as ::core::ffi::c_uint,
    936918000 as ::core::ffi::c_uint,
    2847714899 as ::core::ffi::c_uint,
    3736837829 as ::core::ffi::c_uint,
    1202900863 as ::core::ffi::c_uint,
    817233897 as ::core::ffi::c_uint,
    3183342108 as ::core::ffi::c_uint,
    3401237130 as ::core::ffi::c_uint,
    1404277552 as ::core::ffi::c_uint,
    615818150 as ::core::ffi::c_uint,
    3134207493 as ::core::ffi::c_uint,
    3453421203 as ::core::ffi::c_uint,
    1423857449 as ::core::ffi::c_uint,
    601450431 as ::core::ffi::c_uint,
    3009837614 as ::core::ffi::c_uint,
    3294710456 as ::core::ffi::c_uint,
    1567103746 as ::core::ffi::c_uint,
    711928724 as ::core::ffi::c_uint,
    3020668471 as ::core::ffi::c_uint,
    3272380065 as ::core::ffi::c_uint,
    1510334235 as ::core::ffi::c_uint,
    755167117 as ::core::ffi::c_uint,
];

// --- structs/BPMLists.rs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BPMNode {
    pub weight: ::core::ffi::c_int,
    pub index: ::core::ffi::c_uint,
    pub tail: *mut BPMNode,
    pub in_use: ::core::ffi::c_int,
}
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
pub type FILE = libc::FILE;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct BPMLists {
    pub memsize: ::core::ffi::c_uint,
    pub memory: *mut BPMNode,
    pub numfree: ::core::ffi::c_uint,
    pub nextfree: ::core::ffi::c_uint,
    pub freelist: *mut *mut BPMNode,
    pub listsize: ::core::ffi::c_uint,
    pub chains0: *mut *mut BPMNode,
    pub chains1: *mut *mut BPMNode,
}

// --- structs/BPMNode.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
pub type FILE = libc::FILE;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct BPMNode {
    pub weight: ::core::ffi::c_int,
    pub index: ::core::ffi::c_uint,
    pub tail: *mut BPMNode,
    pub in_use: ::core::ffi::c_int,
}

// --- structs/ColorTree.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ColorTree {
    pub children: [*mut ColorTree; 16],
    pub index: ::core::ffi::c_int,
}

// --- structs/Hash.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
pub type FILE = libc::FILE;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Hash {
    pub head: *mut ::core::ffi::c_int,
    pub chain: *mut ::core::ffi::c_ushort,
    pub val: *mut ::core::ffi::c_int,
    pub headz: *mut ::core::ffi::c_int,
    pub chainz: *mut ::core::ffi::c_ushort,
    pub zeros: *mut ::core::ffi::c_ushort,
}

// --- structs/HuffmanTree.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct HuffmanTree {
    pub codes: *mut ::core::ffi::c_uint,
    pub lengths: *mut ::core::ffi::c_uint,
    pub maxbitlen: ::core::ffi::c_uint,
    pub numcodes: ::core::ffi::c_uint,
    pub table_len: *mut ::core::ffi::c_uchar,
    pub table_value: *mut ::core::ffi::c_ushort,
}

// --- structs/LodePNGBitReader.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGBitReader {
    pub data: *const ::core::ffi::c_uchar,
    pub size: size_t,
    pub bitsize: size_t,
    pub bp: size_t,
    pub buffer: ::core::ffi::c_uint,
}

// --- structs/LodePNGBitWriter.rs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucvector {
    pub data: *mut ::core::ffi::c_uchar,
    pub size: size_t,
    pub allocsize: size_t,
}
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
pub type FILE = libc::FILE;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGBitWriter {
    pub data: *mut ucvector,
    pub bp: ::core::ffi::c_uchar,
}

// --- structs/LodePNGColorMode.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGColorMode {
    pub colortype: LodePNGColorType,
    pub bitdepth: ::core::ffi::c_uint,
    pub palette: *mut ::core::ffi::c_uchar,
    pub palettesize: size_t,
    pub key_defined: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_uint,
    pub key_g: ::core::ffi::c_uint,
    pub key_b: ::core::ffi::c_uint,
}

// --- structs/LodePNGColorStats.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
pub type FILE = libc::FILE;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGColorStats {
    pub colored: ::core::ffi::c_uint,
    pub key: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_ushort,
    pub key_g: ::core::ffi::c_ushort,
    pub key_b: ::core::ffi::c_ushort,
    pub alpha: ::core::ffi::c_uint,
    pub numcolors: ::core::ffi::c_uint,
    pub palette: [::core::ffi::c_uchar; 1024],
    pub bits: ::core::ffi::c_uint,
    pub numpixels: size_t,
    pub allow_palette: ::core::ffi::c_uint,
    pub allow_greyscale: ::core::ffi::c_uint,
}

// --- structs/LodePNGCompressSettings.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGCompressSettings {
    pub btype: ::core::ffi::c_uint,
    pub use_lz77: ::core::ffi::c_uint,
    pub windowsize: ::core::ffi::c_uint,
    pub minmatch: ::core::ffi::c_uint,
    pub nicematch: ::core::ffi::c_uint,
    pub lazymatching: ::core::ffi::c_uint,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_deflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}

// --- structs/LodePNGDecoderSettings.rs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecompressSettings {
    pub ignore_adler32: ::core::ffi::c_uint,
    pub ignore_nlen: ::core::ffi::c_uint,
    pub max_output_size: size_t,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_inflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGDecoderSettings {
    pub zlibsettings: LodePNGDecompressSettings,
    pub ignore_crc: ::core::ffi::c_uint,
    pub ignore_critical: ::core::ffi::c_uint,
    pub ignore_end: ::core::ffi::c_uint,
    pub color_convert: ::core::ffi::c_uint,
    pub read_text_chunks: ::core::ffi::c_uint,
    pub remember_unknown_chunks: ::core::ffi::c_uint,
    pub max_text_size: size_t,
    pub max_icc_size: size_t,
}

// --- structs/LodePNGDecompressSettings.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGDecompressSettings {
    pub ignore_adler32: ::core::ffi::c_uint,
    pub ignore_nlen: ::core::ffi::c_uint,
    pub max_output_size: size_t,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_inflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}

// --- structs/LodePNGEncoderSettings.rs
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGCompressSettings {
    pub btype: ::core::ffi::c_uint,
    pub use_lz77: ::core::ffi::c_uint,
    pub windowsize: ::core::ffi::c_uint,
    pub minmatch: ::core::ffi::c_uint,
    pub nicematch: ::core::ffi::c_uint,
    pub lazymatching: ::core::ffi::c_uint,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_deflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGEncoderSettings {
    pub zlibsettings: LodePNGCompressSettings,
    pub auto_convert: ::core::ffi::c_uint,
    pub filter_palette_zero: ::core::ffi::c_uint,
    pub filter_strategy: LodePNGFilterStrategy,
    pub predefined_filters: *const ::core::ffi::c_uchar,
    pub force_palette: ::core::ffi::c_uint,
    pub add_id: ::core::ffi::c_uint,
    pub text_compression: ::core::ffi::c_uint,
}

// --- structs/LodePNGInfo.rs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGTime {
    pub year: ::core::ffi::c_uint,
    pub month: ::core::ffi::c_uint,
    pub day: ::core::ffi::c_uint,
    pub hour: ::core::ffi::c_uint,
    pub minute: ::core::ffi::c_uint,
    pub second: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorMode {
    pub colortype: LodePNGColorType,
    pub bitdepth: ::core::ffi::c_uint,
    pub palette: *mut ::core::ffi::c_uchar,
    pub palettesize: size_t,
    pub key_defined: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_uint,
    pub key_g: ::core::ffi::c_uint,
    pub key_b: ::core::ffi::c_uint,
}
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGInfo {
    pub compression_method: ::core::ffi::c_uint,
    pub filter_method: ::core::ffi::c_uint,
    pub interlace_method: ::core::ffi::c_uint,
    pub color: LodePNGColorMode,
    pub background_defined: ::core::ffi::c_uint,
    pub background_r: ::core::ffi::c_uint,
    pub background_g: ::core::ffi::c_uint,
    pub background_b: ::core::ffi::c_uint,
    pub text_num: size_t,
    pub text_keys: *mut *mut ::core::ffi::c_char,
    pub text_strings: *mut *mut ::core::ffi::c_char,
    pub itext_num: size_t,
    pub itext_keys: *mut *mut ::core::ffi::c_char,
    pub itext_langtags: *mut *mut ::core::ffi::c_char,
    pub itext_transkeys: *mut *mut ::core::ffi::c_char,
    pub itext_strings: *mut *mut ::core::ffi::c_char,
    pub time_defined: ::core::ffi::c_uint,
    pub time: LodePNGTime,
    pub phys_defined: ::core::ffi::c_uint,
    pub phys_x: ::core::ffi::c_uint,
    pub phys_y: ::core::ffi::c_uint,
    pub phys_unit: ::core::ffi::c_uint,
    pub gama_defined: ::core::ffi::c_uint,
    pub gama_gamma: ::core::ffi::c_uint,
    pub chrm_defined: ::core::ffi::c_uint,
    pub chrm_white_x: ::core::ffi::c_uint,
    pub chrm_white_y: ::core::ffi::c_uint,
    pub chrm_red_x: ::core::ffi::c_uint,
    pub chrm_red_y: ::core::ffi::c_uint,
    pub chrm_green_x: ::core::ffi::c_uint,
    pub chrm_green_y: ::core::ffi::c_uint,
    pub chrm_blue_x: ::core::ffi::c_uint,
    pub chrm_blue_y: ::core::ffi::c_uint,
    pub srgb_defined: ::core::ffi::c_uint,
    pub srgb_intent: ::core::ffi::c_uint,
    pub iccp_defined: ::core::ffi::c_uint,
    pub iccp_name: *mut ::core::ffi::c_char,
    pub iccp_profile: *mut ::core::ffi::c_uchar,
    pub iccp_profile_size: ::core::ffi::c_uint,
    pub sbit_defined: ::core::ffi::c_uint,
    pub sbit_r: ::core::ffi::c_uint,
    pub sbit_g: ::core::ffi::c_uint,
    pub sbit_b: ::core::ffi::c_uint,
    pub sbit_a: ::core::ffi::c_uint,
    pub unknown_chunks_data: [*mut ::core::ffi::c_uchar; 3],
    pub unknown_chunks_size: [size_t; 3],
}

// --- structs/LodePNGState.rs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGTime {
    pub year: ::core::ffi::c_uint,
    pub month: ::core::ffi::c_uint,
    pub day: ::core::ffi::c_uint,
    pub hour: ::core::ffi::c_uint,
    pub minute: ::core::ffi::c_uint,
    pub second: ::core::ffi::c_uint,
}
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecompressSettings {
    pub ignore_adler32: ::core::ffi::c_uint,
    pub ignore_nlen: ::core::ffi::c_uint,
    pub max_output_size: size_t,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_inflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGCompressSettings {
    pub btype: ::core::ffi::c_uint,
    pub use_lz77: ::core::ffi::c_uint,
    pub windowsize: ::core::ffi::c_uint,
    pub minmatch: ::core::ffi::c_uint,
    pub nicematch: ::core::ffi::c_uint,
    pub lazymatching: ::core::ffi::c_uint,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_deflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGInfo {
    pub compression_method: ::core::ffi::c_uint,
    pub filter_method: ::core::ffi::c_uint,
    pub interlace_method: ::core::ffi::c_uint,
    pub color: LodePNGColorMode,
    pub background_defined: ::core::ffi::c_uint,
    pub background_r: ::core::ffi::c_uint,
    pub background_g: ::core::ffi::c_uint,
    pub background_b: ::core::ffi::c_uint,
    pub text_num: size_t,
    pub text_keys: *mut *mut ::core::ffi::c_char,
    pub text_strings: *mut *mut ::core::ffi::c_char,
    pub itext_num: size_t,
    pub itext_keys: *mut *mut ::core::ffi::c_char,
    pub itext_langtags: *mut *mut ::core::ffi::c_char,
    pub itext_transkeys: *mut *mut ::core::ffi::c_char,
    pub itext_strings: *mut *mut ::core::ffi::c_char,
    pub time_defined: ::core::ffi::c_uint,
    pub time: LodePNGTime,
    pub phys_defined: ::core::ffi::c_uint,
    pub phys_x: ::core::ffi::c_uint,
    pub phys_y: ::core::ffi::c_uint,
    pub phys_unit: ::core::ffi::c_uint,
    pub gama_defined: ::core::ffi::c_uint,
    pub gama_gamma: ::core::ffi::c_uint,
    pub chrm_defined: ::core::ffi::c_uint,
    pub chrm_white_x: ::core::ffi::c_uint,
    pub chrm_white_y: ::core::ffi::c_uint,
    pub chrm_red_x: ::core::ffi::c_uint,
    pub chrm_red_y: ::core::ffi::c_uint,
    pub chrm_green_x: ::core::ffi::c_uint,
    pub chrm_green_y: ::core::ffi::c_uint,
    pub chrm_blue_x: ::core::ffi::c_uint,
    pub chrm_blue_y: ::core::ffi::c_uint,
    pub srgb_defined: ::core::ffi::c_uint,
    pub srgb_intent: ::core::ffi::c_uint,
    pub iccp_defined: ::core::ffi::c_uint,
    pub iccp_name: *mut ::core::ffi::c_char,
    pub iccp_profile: *mut ::core::ffi::c_uchar,
    pub iccp_profile_size: ::core::ffi::c_uint,
    pub sbit_defined: ::core::ffi::c_uint,
    pub sbit_r: ::core::ffi::c_uint,
    pub sbit_g: ::core::ffi::c_uint,
    pub sbit_b: ::core::ffi::c_uint,
    pub sbit_a: ::core::ffi::c_uint,
    pub unknown_chunks_data: [*mut ::core::ffi::c_uchar; 3],
    pub unknown_chunks_size: [size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGEncoderSettings {
    pub zlibsettings: LodePNGCompressSettings,
    pub auto_convert: ::core::ffi::c_uint,
    pub filter_palette_zero: ::core::ffi::c_uint,
    pub filter_strategy: LodePNGFilterStrategy,
    pub predefined_filters: *const ::core::ffi::c_uchar,
    pub force_palette: ::core::ffi::c_uint,
    pub add_id: ::core::ffi::c_uint,
    pub text_compression: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecoderSettings {
    pub zlibsettings: LodePNGDecompressSettings,
    pub ignore_crc: ::core::ffi::c_uint,
    pub ignore_critical: ::core::ffi::c_uint,
    pub ignore_end: ::core::ffi::c_uint,
    pub color_convert: ::core::ffi::c_uint,
    pub read_text_chunks: ::core::ffi::c_uint,
    pub remember_unknown_chunks: ::core::ffi::c_uint,
    pub max_text_size: size_t,
    pub max_icc_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorMode {
    pub colortype: LodePNGColorType,
    pub bitdepth: ::core::ffi::c_uint,
    pub palette: *mut ::core::ffi::c_uchar,
    pub palettesize: size_t,
    pub key_defined: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_uint,
    pub key_g: ::core::ffi::c_uint,
    pub key_b: ::core::ffi::c_uint,
}
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGState {
    pub decoder: LodePNGDecoderSettings,
    pub encoder: LodePNGEncoderSettings,
    pub info_raw: LodePNGColorMode,
    pub info_png: LodePNGInfo,
    pub error: ::core::ffi::c_uint,
}

// --- structs/LodePNGTime.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LodePNGTime {
    pub year: ::core::ffi::c_uint,
    pub month: ::core::ffi::c_uint,
    pub day: ::core::ffi::c_uint,
    pub hour: ::core::ffi::c_uint,
    pub minute: ::core::ffi::c_uint,
    pub second: ::core::ffi::c_uint,
}

// --- structs/ucvector.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ucvector {
    pub data: *mut ::core::ffi::c_uchar,
    pub size: size_t,
    pub allocsize: size_t,
}

// --- structs/uivector.rs
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
pub type FILE = libc::FILE;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct uivector {
    pub data: *mut ::core::ffi::c_uint,
    pub size: size_t,
    pub allocsize: size_t,
}

// --- functions/Adam7_getpassvalues.rs (SACTOR-verified)
pub unsafe fn Adam7_getpassvalues(
    passw: *mut u32,
    passh: *mut u32,
    filter_passstart: *mut libc::size_t,
    padded_passstart: *mut libc::size_t,
    passstart: *mut libc::size_t,
    w: u32,
    h: u32,
    bpp: u32,
) {
    let mut i: u32 = 0;
    while i != 7 {
        *passw.add(i as usize) =
            (w + ADAM7_DX[i as usize] - ADAM7_IX[i as usize] - 1) / ADAM7_DX[i as usize];
        *passh.add(i as usize) =
            (h + ADAM7_DY[i as usize] - ADAM7_IY[i as usize] - 1) / ADAM7_DY[i as usize];
        if *passw.add(i as usize) == 0 {
            *passh.add(i as usize) = 0;
        }
        if *passh.add(i as usize) == 0 {
            *passw.add(i as usize) = 0;
        }
        i += 1;
    }
    *filter_passstart = 0;
    *padded_passstart = 0;
    *passstart = 0;
    i = 0;
    while i != 7 {
        let idx = i as usize;
        let next = (i + 1) as usize;
        let pw = *passw.add(idx);
        let ph = *passh.add(idx);
        *filter_passstart.add(next) = *filter_passstart.add(idx)
            + if pw != 0 && ph != 0 {
                (ph as libc::size_t)
                    * (1u64 + ((pw as u64 * bpp as u64 + 7u64) / 8u64)) as libc::size_t
            } else {
                0
            };
        *padded_passstart.add(next) = *padded_passstart.add(idx)
            + (ph as libc::size_t) * (((pw as u64 * bpp as u64 + 7u64) / 8u64) as libc::size_t);
        *passstart.add(next) = *passstart.add(idx)
            + (((ph as u64 * pw as u64 * bpp as u64 + 7u64) / 8u64) as libc::size_t);
        i += 1;
    }
}

// --- functions/HuffmanTree_init.rs (SACTOR-verified)
pub unsafe fn HuffmanTree_init(tree: *mut HuffmanTree) {
    if !tree.is_null() {
        (*tree).codes = core::ptr::null_mut();
        (*tree).lengths = core::ptr::null_mut();
        (*tree).table_len = core::ptr::null_mut();
        (*tree).table_value = core::ptr::null_mut();
    }
}

// --- functions/LodePNGBitWriter_init.rs (SACTOR-verified)
pub unsafe fn LodePNGBitWriter_init(writer: *mut LodePNGBitWriter, data: *mut ucvector) {
    if writer.is_null() {
        return;
    }
    (*writer).data = data;
    (*writer).bp = 0;
}

// --- functions/addColorBits.rs (SACTOR-verified)
pub unsafe fn addColorBits(out: *mut u8, index: libc::size_t, bits: u32, mut input: u32) {
    fn size_t_to_u32(x: libc::size_t) -> u32 {
        x as u32
    }
    fn compute_m(bits: u32) -> u32 {
        if bits == 1 {
            7
        } else if bits == 2 {
            3
        } else {
            1
        }
    }
    let m = compute_m(bits);
    let p: u32 = size_t_to_u32(index) & m;
    input &= (1u32 << bits) - 1u32;
    input <<= bits * (m - p);
    let byte_index: usize = ((index as u64 * bits as u64) / 8u64) as usize;
    let byte_ptr = out.add(byte_index);
    if p == 0 {
        *byte_ptr = input as u8;
    } else {
        *byte_ptr |= input as u8;
    }
}

// --- functions/advanceBits.rs (SACTOR-verified)
#[inline]
pub unsafe fn advanceBits(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    (*reader).buffer >>= nbits;
    (*reader).bp = (*reader).bp.wrapping_add(nbits);
}

// --- functions/bpmnode_create.rs (SACTOR-verified)
pub unsafe fn bpmnode_create(
    lists: *mut BPMLists,
    weight: ::core::ffi::c_int,
    index: ::core::ffi::c_uint,
    tail: *mut BPMNode,
) -> *mut BPMNode {
    unsafe fn get_memory_node(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut BPMNode {
        (*lists).memory.add(i as usize)
    }
    unsafe fn get_freelist_slot(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut *mut BPMNode {
        (*lists).freelist.add(i as usize)
    }
    unsafe fn get_chain0_entry(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut *mut BPMNode {
        (*lists).chains0.add(i as usize)
    }
    unsafe fn get_chain1_entry(lists: *mut BPMLists, i: ::core::ffi::c_uint) -> *mut *mut BPMNode {
        (*lists).chains1.add(i as usize)
    }
    let mut i: ::core::ffi::c_uint;
    if (*lists).nextfree >= (*lists).numfree {
        i = 0;
        while i != (*lists).memsize {
            (*get_memory_node(lists, i)).in_use = 0;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i != (*lists).listsize {
            let mut node: *mut BPMNode;
            node = *get_chain0_entry(lists, i);
            while !node.is_null() {
                (*node).in_use = 1;
                node = (*node).tail;
            }
            node = *get_chain1_entry(lists, i);
            while !node.is_null() {
                (*node).in_use = 1;
                node = (*node).tail;
            }
            i = i.wrapping_add(1);
        }
        (*lists).numfree = 0;
        i = 0;
        while i != (*lists).memsize {
            let mem_node = get_memory_node(lists, i);
            if (*mem_node).in_use == 0 {
                let nf = (*lists).numfree;
                *get_freelist_slot(lists, nf) = mem_node;
                (*lists).numfree = nf.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*lists).nextfree = 0;
    }
    let nf = (*lists).nextfree;
    let result = *get_freelist_slot(lists, nf);
    (*lists).nextfree = nf.wrapping_add(1);
    (*result).weight = weight;
    (*result).index = index;
    (*result).tail = tail;
    result
}

// --- functions/checkColorValidity.rs (SACTOR-verified)
pub fn checkColorValidity(colortype: LodePNGColorType, bd: u32) -> u32 {
    match colortype {
        LodePNGColorType::LCT_GREY => {
            if !(bd == 1 || bd == 2 || bd == 4 || bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_RGB => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_PALETTE => {
            if !(bd == 1 || bd == 2 || bd == 4 || bd == 8) {
                return 37;
            }
        }
        LodePNGColorType::LCT_GREY_ALPHA => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_RGBA => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        LodePNGColorType::LCT_MAX_OCTET_VALUE => {
            return 31;
        }
        _ => {
            return 31;
        }
    }
    0
}

// --- functions/color_tree_get.rs (SACTOR-verified)
pub unsafe fn color_tree_get(
    tree: *mut ColorTree,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) -> ::core::ffi::c_int {
    let mut tree = tree;
    let mut bit: ::core::ffi::c_int = 0;
    while bit < 8 {
        let i = 8 * (((r >> bit) & 1) as ::core::ffi::c_int)
            + 4 * (((g >> bit) & 1) as ::core::ffi::c_int)
            + 2 * (((b >> bit) & 1) as ::core::ffi::c_int)
            + 1 * (((a >> bit) & 1) as ::core::ffi::c_int);
        if tree.is_null() {
            return -1;
        }
        let child = (*tree).children[i as usize];
        if child.is_null() {
            return -1;
        } else {
            tree = child;
        }
        bit += 1;
    }
    if tree.is_null() {
        -1
    } else {
        (*tree).index
    }
}

// --- functions/countZeros.rs (SACTOR-verified)
#[inline]
pub unsafe fn countZeros(
    data: *const libc::c_uchar,
    size: libc::size_t,
    pos: libc::size_t,
) -> libc::c_uint {
    let start: *const libc::c_uchar = data.add(pos);
    let mut end: *const libc::c_uchar = start.add(MAX_SUPPORTED_DEFLATE_LENGTH);
    let data_end: *const libc::c_uchar = data.add(size);
    if end > data_end {
        end = data_end;
    }
    let mut current: *const libc::c_uchar = start;
    while current != end && *current == 0 {
        current = current.add(1);
    }
    (current.offset_from(start) as libc::size_t) as libc::c_uint
}

// --- functions/ensureBits17.rs (SACTOR-verified)
#[inline]
pub unsafe fn ensureBits17(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    #[inline]
    unsafe fn read_byte(reader: *mut LodePNGBitReader, index: libc::size_t) -> u32 {
        let r = &*reader;
        if index < r.size {
            *r.data.add(index) as u32
        } else {
            0
        }
    }
    let r = &mut *reader;
    let start: libc::size_t = r.bp >> 3;
    let size: libc::size_t = r.size;
    if start + 2 < size {
        let b0 = read_byte(reader, start + 0);
        let b1 = read_byte(reader, start + 1);
        let b2 = read_byte(reader, start + 2);
        r.buffer = (b0 | (b1 << 8) | (b2 << 16)) as libc::c_uint;
        r.buffer >>= (r.bp & 7) as libc::c_uint;
    } else {
        r.buffer = 0;
        if start + 0 < size {
            r.buffer |= read_byte(reader, start + 0) as libc::c_uint;
        }
        if start + 1 < size {
            r.buffer |= (read_byte(reader, start + 1) << 8) as libc::c_uint;
        }
        r.buffer >>= (r.bp & 7) as libc::c_uint;
    }
    let _ = nbits;
}

// --- functions/ensureBits25.rs (SACTOR-verified)
#[inline]
pub unsafe fn ensureBits25(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    let reader_ref: &mut LodePNGBitReader = &mut *reader;
    let start: libc::size_t = reader_ref.bp >> 3usize;
    let size: libc::size_t = reader_ref.size;
    unsafe fn get_data_byte(ptr: *const ::core::ffi::c_uchar, idx: libc::size_t) -> u32 {
        *ptr.add(idx) as u32
    }
    if start + 3usize < size {
        let b0 = get_data_byte(reader_ref.data, start + 0);
        let b1 = get_data_byte(reader_ref.data, start + 1);
        let b2 = get_data_byte(reader_ref.data, start + 2);
        let b3 = get_data_byte(reader_ref.data, start + 3);
        reader_ref.buffer = b0 | (b1 << 8u32) | (b2 << 16u32) | (b3 << 24u32);
        reader_ref.buffer >>= (reader_ref.bp & 7usize) as u32;
    } else {
        reader_ref.buffer = 0;
        if start + 0usize < size {
            reader_ref.buffer |= get_data_byte(reader_ref.data, start + 0);
        }
        if start + 1usize < size {
            reader_ref.buffer |= get_data_byte(reader_ref.data, start + 1) << 8u32;
        }
        if start + 2usize < size {
            reader_ref.buffer |= get_data_byte(reader_ref.data, start + 2) << 16u32;
        }
        reader_ref.buffer >>= (reader_ref.bp & 7usize) as u32;
    }
    let _ = nbits;
}

// --- functions/ensureBits32.rs (SACTOR-verified)
#[inline]
pub unsafe fn ensureBits32(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    #[inline]
    unsafe fn get_byte(base: *const ::core::ffi::c_uchar, idx: libc::size_t) -> u32 {
        *base.add(idx) as u32
    }
    let reader_ref: &mut LodePNGBitReader = &mut *reader;
    let start: libc::size_t = reader_ref.bp >> 3usize;
    let size: libc::size_t = reader_ref.size;
    if start + 4usize < size {
        let base = reader_ref.data;
        let mut buffer: u32 = get_byte(base, start + 0usize)
            | (get_byte(base, start + 1usize) << 8u32)
            | (get_byte(base, start + 2usize) << 16u32)
            | (get_byte(base, start + 3usize) << 24u32);
        buffer >>= (reader_ref.bp & 7usize) as u32;
        buffer |=
            (get_byte(base, start + 4usize) << 24u32) << (8u32 - (reader_ref.bp & 7usize) as u32);
        reader_ref.buffer = buffer as ::core::ffi::c_uint;
    } else {
        let base = reader_ref.data;
        let mut buffer: u32 = 0;
        if start + 0usize < size {
            buffer |= get_byte(base, start + 0usize);
        }
        if start + 1usize < size {
            buffer |= get_byte(base, start + 1usize) << 8u32;
        }
        if start + 2usize < size {
            buffer |= get_byte(base, start + 2usize) << 16u32;
        }
        if start + 3usize < size {
            buffer |= get_byte(base, start + 3usize) << 24u32;
        }
        buffer >>= (reader_ref.bp & 7usize) as u32;
        reader_ref.buffer = buffer as ::core::ffi::c_uint;
    }
    let _ = nbits;
}

// --- functions/ensureBits9.rs (SACTOR-verified)
#[inline]
pub unsafe fn ensureBits9(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    unsafe fn get_data(reader: *mut LodePNGBitReader, index: libc::size_t) -> u32 {
        *(*reader).data.add(index) as u32
    }
    let start: libc::size_t = (*reader).bp >> 3;
    let size: libc::size_t = (*reader).size;
    if start + 1 < size {
        let v0 = get_data(reader, start + 0);
        let v1 = get_data(reader, start + 1);
        (*reader).buffer = (v0 | (v1 << 8)) as libc::c_uint;
        (*reader).buffer >>= ((*reader).bp & 7) as libc::c_uint;
    } else {
        (*reader).buffer = 0;
        if start < size {
            let v0 = get_data(reader, start + 0);
            (*reader).buffer = v0 as libc::c_uint;
        }
        (*reader).buffer >>= ((*reader).bp & 7) as libc::c_uint;
    }
    let _ = nbits;
}

// --- functions/getHash.rs (SACTOR-verified)
#[inline]
pub unsafe fn getHash(
    data: *const libc::c_uchar,
    size: libc::size_t,
    pos: libc::size_t,
) -> libc::c_uint {
    let mut result: libc::c_uint = 0;
    if pos.wrapping_add(2) < size {
        result ^= (*(data.add(pos as usize)) as libc::c_uint) << 0u32;
        result ^= (*(data.add(pos as usize + 1)) as libc::c_uint) << 4u32;
        result ^= (*(data.add(pos as usize + 2)) as libc::c_uint) << 8u32;
    } else {
        let mut amount: libc::size_t;
        let mut i: libc::size_t;
        if pos >= size {
            return 0;
        }
        amount = size.wrapping_sub(pos);
        i = 0;
        while i != amount {
            result ^= (*(data.add((pos.wrapping_add(i)) as usize)) as libc::c_uint)
                << ((i as libc::c_uint) * 8u32);
            i = i.wrapping_add(1);
        }
    }
    result & HASH_BIT_MASK
}

// --- functions/getNumColorChannels.rs (SACTOR-verified)
pub fn getNumColorChannels(colortype: LodePNGColorType) -> u32 {
    match colortype {
        LodePNGColorType::LCT_GREY => 1,
        LodePNGColorType::LCT_RGB => 3,
        LodePNGColorType::LCT_PALETTE => 1,
        LodePNGColorType::LCT_GREY_ALPHA => 2,
        LodePNGColorType::LCT_RGBA => 4,
        LodePNGColorType::LCT_MAX_OCTET_VALUE => 0,
        _ => 0,
    }
}

// --- functions/getValueRequiredBits.rs (SACTOR-verified)
pub fn getValueRequiredBits(value: u8) -> u32 {
    if value == 0 || value == 255 {
        return 1;
    }
    if value % 17 == 0 {
        return if value % 85 == 0 { 2 } else { 4 };
    }
    8
}

// --- functions/ilog2.rs (SACTOR-verified)
pub unsafe fn ilog2(mut i: libc::size_t) -> libc::size_t {
    let mut result: libc::size_t = 0;
    if i >= 65536 {
        result += 16;
        i >>= 16;
    }
    if i >= 256 {
        result += 8;
        i >>= 8;
    }
    if i >= 16 {
        result += 4;
        i >>= 4;
    }
    if i >= 4 {
        result += 2;
        i >>= 2;
    }
    if i >= 2 {
        result += 1;
    }
    result
}

// --- functions/isGrayICCProfile.rs (SACTOR-verified)
pub fn isGrayICCProfile(profile: *const u8, size: u32) -> u32 {
    unsafe {
        if size < 20 {
            return 0;
        }
        if *profile.add(16) == b'G'
            && *profile.add(17) == b'R'
            && *profile.add(18) == b'A'
            && *profile.add(19) == b'Y'
        {
            1
        } else {
            0
        }
    }
}

// --- functions/isRGBICCProfile.rs (SACTOR-verified)
pub fn isRGBICCProfile(profile: *const u8, size: u32) -> u32 {
    if size < 20 {
        return 0;
    }
    unsafe {
        if *profile.add(16) == b'R'
            && *profile.add(17) == b'G'
            && *profile.add(18) == b'B'
            && *profile.add(19) == b' '
        {
            1
        } else {
            0
        }
    }
}

// --- functions/lodepng_addofl.rs (SACTOR-verified)
pub fn lodepng_addofl(a: libc::size_t, b: libc::size_t, result: *mut libc::size_t) -> libc::c_int {
    unsafe {
        *result = a.wrapping_add(b);
        if *result < a {
            1
        } else {
            0
        }
    }
}

// --- functions/lodepng_buffer_file.rs (SACTOR-verified)
pub unsafe fn lodepng_buffer_file(
    out: *mut u8,
    size: libc::size_t,
    filename: *const libc::c_char,
) -> u32 {
    let mut file: *mut libc::FILE;
    file = libc::fopen(filename, b"rb\0".as_ptr() as *const libc::c_char);
    if file.is_null() {
        return 78;
    }
    let readsize = libc::fread(out as *mut libc::c_void, 1, size, file);
    libc::fclose(file);
    if readsize != size {
        return 78;
    }
    0
}

// --- functions/lodepng_chunk_ancillary.rs (SACTOR-verified)
pub unsafe fn lodepng_chunk_ancillary(chunk: *const u8) -> u8 {
    if *chunk.add(4) & 32 != 0 {
        1
    } else {
        0
    }
}

// --- functions/lodepng_chunk_data.rs (SACTOR-verified)
pub unsafe fn lodepng_chunk_data(chunk: *mut u8) -> *mut u8 {
    chunk.add(8)
}

// --- functions/lodepng_chunk_data_const.rs (SACTOR-verified)
pub unsafe fn lodepng_chunk_data_const(chunk: *const libc::c_uchar) -> *const libc::c_uchar {
    chunk.add(8)
}

// --- functions/lodepng_chunk_private.rs (SACTOR-verified)
pub unsafe fn lodepng_chunk_private(chunk: *const u8) -> u8 {
    if *chunk.add(6) & 32 != 0 {
        1
    } else {
        0
    }
}

// --- functions/lodepng_chunk_safetocopy.rs (SACTOR-verified)
pub unsafe fn lodepng_chunk_safetocopy(chunk: *const u8) -> u8 {
    if *chunk.add(7) & 32 != 0 {
        1
    } else {
        0
    }
}

// --- functions/lodepng_chunk_type.rs (SACTOR-verified)
use libc;
pub unsafe fn lodepng_chunk_type(r#type: *mut libc::c_char, chunk: *const libc::c_uchar) {
    unsafe fn get_chunk_byte(chunk: *const libc::c_uchar, index: usize) -> libc::c_uchar {
        *chunk.add(index)
    }
    let mut i: libc::c_uint = 0;
    while i != 4 {
        *r#type.add(i as usize) = get_chunk_byte(chunk, 4 + i as usize) as libc::c_char;
        i = i.wrapping_add(1);
    }
    *r#type.add(4) = 0;
}

// --- functions/lodepng_color_stats_init.rs (SACTOR-verified)
pub unsafe fn lodepng_color_stats_init(stats: *mut LodePNGColorStats) {
    if stats.is_null() {
        return;
    }
    (*stats).colored = 0;
    (*stats).key = 0;
    (*stats).key_r = 0;
    (*stats).key_g = 0;
    (*stats).key_b = 0;
    (*stats).alpha = 0;
    (*stats).numcolors = 0;
    (*stats).bits = 1;
    (*stats).numpixels = 0;
    (*stats).allow_palette = 1;
    (*stats).allow_greyscale = 1;
}

// --- functions/lodepng_compress_settings_init.rs (SACTOR-verified)
pub unsafe fn lodepng_compress_settings_init(settings: *mut LodePNGCompressSettings) {
    (*settings).btype = 2;
    (*settings).use_lz77 = 1;
    (*settings).windowsize = 2048;
    (*settings).minmatch = 3;
    (*settings).nicematch = 128;
    (*settings).lazymatching = 1;
    (*settings).custom_zlib = None;
    (*settings).custom_deflate = None;
    (*settings).custom_context = core::ptr::null();
}

// --- functions/lodepng_crc32.rs (SACTOR-verified)
pub unsafe fn lodepng_crc32(data: *const u8, length: usize) -> u32 {
    let mut r: u32 = 0xffffffffu32;
    let mut i: usize = 0;
    while i < length {
        let byte = *data.add(i);
        let index = ((r ^ byte as u32) & 0xffu32) as usize;
        r = lodepng_crc32_table[index] ^ (r >> 8);
        i += 1;
    }
    r ^ 0xffffffffu32
}

// --- functions/lodepng_decompress_settings_init.rs (SACTOR-verified)
pub unsafe fn lodepng_decompress_settings_init(settings: *mut LodePNGDecompressSettings) {
    if settings.is_null() {
        return;
    }
    (*settings).ignore_adler32 = 0;
    (*settings).ignore_nlen = 0;
    (*settings).max_output_size = 0;
    (*settings).custom_zlib = None;
    (*settings).custom_inflate = None;
    (*settings).custom_context = core::ptr::null();
}

// --- functions/lodepng_error_text.rs (SACTOR-verified)
pub unsafe fn lodepng_error_text(code: libc::c_uint) -> *const libc::c_char {
    match code {
        0 => b"no error, everything went ok\0".as_ptr() as *const libc::c_char,
        1 => b"nothing done yet\0".as_ptr() as *const libc::c_char,
        10 => {
            b"end of input memory reached without huffman end code\0".as_ptr()
                as *const libc::c_char
        }
        11 => {
            b"error in code tree made it jump outside of huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        13 => {
            b"problem while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        14 => {
            b"problem while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        15 => {
            b"problem while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        16 => {
            b"invalid code while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        17 => {
            b"end of out buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        18 => b"invalid distance code while inflating\0".as_ptr() as *const libc::c_char,
        19 => {
            b"end of out buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        20 => {
            b"invalid deflate block BTYPE encountered while decoding\0".as_ptr()
                as *const libc::c_char
        }
        21 => {
            b"NLEN is not ones complement of LEN in a deflate block\0".as_ptr()
                as *const libc::c_char
        }
        22 => {
            b"end of out buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        23 => {
            b"end of in buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        24 => b"invalid FCHECK in zlib header\0".as_ptr() as *const libc::c_char,
        25 => {
            b"invalid compression method in zlib header\0".as_ptr()
                as *const libc::c_char
        }
        26 => {
            b"FDICT encountered in zlib header while it's not used for PNG\0".as_ptr()
                as *const libc::c_char
        }
        27 => b"PNG file is smaller than a PNG header\0".as_ptr() as *const libc::c_char,
        28 => {
            b"incorrect PNG signature, it's no PNG or corrupted\0".as_ptr()
                as *const libc::c_char
        }
        29 => b"first chunk is not the header chunk\0".as_ptr() as *const libc::c_char,
        30 => {
            b"chunk length too large, chunk broken off at end of file\0".as_ptr()
                as *const libc::c_char
        }
        31 => b"illegal PNG color type or bpp\0".as_ptr() as *const libc::c_char,
        32 => b"illegal PNG compression method\0".as_ptr() as *const libc::c_char,
        33 => b"illegal PNG filter method\0".as_ptr() as *const libc::c_char,
        34 => b"illegal PNG interlace method\0".as_ptr() as *const libc::c_char,
        35 => {
            b"chunk length of a chunk is too large or the chunk too small\0".as_ptr()
                as *const libc::c_char
        }
        36 => b"illegal PNG filter type encountered\0".as_ptr() as *const libc::c_char,
        37 => {
            b"illegal bit depth for this color type given\0".as_ptr()
                as *const libc::c_char
        }
        38 => b"the palette is too small or too big\0".as_ptr() as *const libc::c_char,
        39 => {
            b"tRNS chunk before PLTE or has more entries than palette size\0".as_ptr()
                as *const libc::c_char
        }
        40 => {
            b"tRNS chunk has wrong size for grayscale image\0".as_ptr()
                as *const libc::c_char
        }
        41 => {
            b"tRNS chunk has wrong size for RGB image\0".as_ptr() as *const libc::c_char
        }
        42 => {
            b"tRNS chunk appeared while it was not allowed for this color type\0"
                .as_ptr() as *const libc::c_char
        }
        43 => {
            b"bKGD chunk has wrong size for palette image\0".as_ptr()
                as *const libc::c_char
        }
        44 => {
            b"bKGD chunk has wrong size for grayscale image\0".as_ptr()
                as *const libc::c_char
        }
        45 => {
            b"bKGD chunk has wrong size for RGB image\0".as_ptr() as *const libc::c_char
        }
        48 => {
            b"empty input buffer given to decoder. Maybe caused by non-existing file?\0"
                .as_ptr() as *const libc::c_char
        }
        49 => {
            b"jumped past memory while generating dynamic huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        50 => {
            b"jumped past memory while generating dynamic huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        51 => {
            b"jumped past memory while inflating huffman block\0".as_ptr()
                as *const libc::c_char
        }
        52 => b"jumped past memory while inflating\0".as_ptr() as *const libc::c_char,
        53 => b"size of zlib data too small\0".as_ptr() as *const libc::c_char,
        54 => {
            b"repeat symbol in tree while there was no value symbol yet\0".as_ptr()
                as *const libc::c_char
        }
        55 => {
            b"jumped past tree while generating huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        56 => {
            b"given output image colortype or bitdepth not supported for color conversion\0"
                .as_ptr() as *const libc::c_char
        }
        57 => {
            b"invalid CRC encountered (checking CRC can be disabled)\0".as_ptr()
                as *const libc::c_char
        }
        58 => {
            b"invalid ADLER32 encountered (checking ADLER32 can be disabled)\0".as_ptr()
                as *const libc::c_char
        }
        59 => {
            b"requested color conversion not supported\0".as_ptr() as *const libc::c_char
        }
        60 => {
            b"invalid window size given in the settings of the encoder (must be 0-32768)\0"
                .as_ptr() as *const libc::c_char
        }
        61 => {
            b"invalid BTYPE given in the settings of the encoder (only 0, 1 and 2 are allowed)\0"
                .as_ptr() as *const libc::c_char
        }
        62 => {
            b"conversion from color to grayscale not supported\0".as_ptr()
                as *const libc::c_char
        }
        63 => {
            b"length of a chunk too long, max allowed for PNG is 2147483647 bytes per chunk\0"
                .as_ptr() as *const libc::c_char
        }
        64 => {
            b"the length of the END symbol 256 in the Huffman tree is 0\0".as_ptr()
                as *const libc::c_char
        }
        66 => {
            b"the length of a text chunk keyword given to the encoder is longer than the maximum of 79 bytes\0"
                .as_ptr() as *const libc::c_char
        }
        67 => {
            b"the length of a text chunk keyword given to the encoder is smaller than the minimum of 1 byte\0"
                .as_ptr() as *const libc::c_char
        }
        68 => {
            b"tried to encode a PLTE chunk with a palette that has less than 1 or more than 256 colors\0"
                .as_ptr() as *const libc::c_char
        }
        69 => {
            b"unknown chunk type with 'critical' flag encountered by the decoder\0"
                .as_ptr() as *const libc::c_char
        }
        71 => {
            b"invalid interlace mode given to encoder (must be 0 or 1)\0".as_ptr()
                as *const libc::c_char
        }
        72 => {
            b"while decoding, invalid compression method encountering in zTXt or iTXt chunk (it must be 0)\0"
                .as_ptr() as *const libc::c_char
        }
        73 => b"invalid tIME chunk size\0".as_ptr() as *const libc::c_char,
        74 => b"invalid pHYs chunk size\0".as_ptr() as *const libc::c_char,
        75 => {
            b"no null termination char found while decoding text chunk\0".as_ptr()
                as *const libc::c_char
        }
        76 => {
            b"iTXt chunk too short to contain required bytes\0".as_ptr()
                as *const libc::c_char
        }
        77 => b"integer overflow in buffer size\0".as_ptr() as *const libc::c_char,
        78 => b"failed to open file for reading\0".as_ptr() as *const libc::c_char,
        79 => b"failed to open file for writing\0".as_ptr() as *const libc::c_char,
        80 => b"tried creating a tree of 0 symbols\0".as_ptr() as *const libc::c_char,
        81 => b"lazy matching at pos 0 is impossible\0".as_ptr() as *const libc::c_char,
        82 => {
            b"color conversion to palette requested while a color isn't in palette, or index out of bounds\0"
                .as_ptr() as *const libc::c_char
        }
        83 => b"memory allocation failed\0".as_ptr() as *const libc::c_char,
        84 => {
            b"given image too small to contain all pixels to be encoded\0".as_ptr()
                as *const libc::c_char
        }
        86 => {
            b"impossible offset in lz77 encoding (internal bug)\0".as_ptr()
                as *const libc::c_char
        }
        87 => {
            b"must provide custom zlib function pointer if LODEPNG_COMPILE_ZLIB is not defined\0"
                .as_ptr() as *const libc::c_char
        }
        88 => {
            b"invalid filter strategy given for LodePNGEncoderSettings.filter_strategy\0"
                .as_ptr() as *const libc::c_char
        }
        89 => {
            b"text chunk keyword too short or long: must have size 1-79\0".as_ptr()
                as *const libc::c_char
        }
        90 => b"windowsize must be a power of two\0".as_ptr() as *const libc::c_char,
        91 => b"invalid decompressed idat size\0".as_ptr() as *const libc::c_char,
        92 => {
            b"integer overflow due to too many pixels\0".as_ptr() as *const libc::c_char
        }
        93 => b"zero width or height is invalid\0".as_ptr() as *const libc::c_char,
        94 => {
            b"header chunk must have a size of 13 bytes\0".as_ptr()
                as *const libc::c_char
        }
        95 => {
            b"integer overflow with combined idat chunk size\0".as_ptr()
                as *const libc::c_char
        }
        96 => b"invalid gAMA chunk size\0".as_ptr() as *const libc::c_char,
        97 => b"invalid cHRM chunk size\0".as_ptr() as *const libc::c_char,
        98 => b"invalid sRGB chunk size\0".as_ptr() as *const libc::c_char,
        99 => b"invalid sRGB rendering intent\0".as_ptr() as *const libc::c_char,
        100 => {
            b"invalid ICC profile color type, the PNG specification only allows RGB or GRAY\0"
                .as_ptr() as *const libc::c_char
        }
        101 => {
            b"PNG specification does not allow RGB ICC profile on gray color types and vice versa\0"
                .as_ptr() as *const libc::c_char
        }
        102 => {
            b"not allowed to set grayscale ICC profile with colored pixels by PNG specification\0"
                .as_ptr() as *const libc::c_char
        }
        103 => {
            b"invalid palette index in bKGD chunk. Maybe it came before PLTE chunk?\0"
                .as_ptr() as *const libc::c_char
        }
        104 => {
            b"invalid bKGD color while encoding (e.g. palette index out of range)\0"
                .as_ptr() as *const libc::c_char
        }
        105 => b"integer overflow of bitsize\0".as_ptr() as *const libc::c_char,
        106 => {
            b"PNG file must have PLTE chunk if color type is palette\0".as_ptr()
                as *const libc::c_char
        }
        107 => {
            b"color convert from palette mode requested without setting the palette data in it\0"
                .as_ptr() as *const libc::c_char
        }
        108 => {
            b"tried to add more than 256 values to a palette\0".as_ptr()
                as *const libc::c_char
        }
        109 => {
            b"tried to decompress zlib or deflate data larger than desired max_output_size\0"
                .as_ptr() as *const libc::c_char
        }
        110 => {
            b"custom zlib or inflate decompression failed\0".as_ptr()
                as *const libc::c_char
        }
        111 => {
            b"custom zlib or deflate compression failed\0".as_ptr()
                as *const libc::c_char
        }
        112 => b"compressed text unreasonably large\0".as_ptr() as *const libc::c_char,
        113 => b"ICC profile unreasonably large\0".as_ptr() as *const libc::c_char,
        114 => {
            b"sBIT chunk has wrong size for the color type of the image\0".as_ptr()
                as *const libc::c_char
        }
        115 => b"sBIT value out of range\0".as_ptr() as *const libc::c_char,
        _ => b"unknown error code\0".as_ptr() as *const libc::c_char,
    }
}

// --- functions/lodepng_filesize.rs (SACTOR-verified)
pub fn lodepng_filesize(filename: *const libc::c_char) -> libc::c_long {
    unsafe {
        let mode = b"rb\0";
        let file: *mut libc::FILE = libc::fopen(filename, mode.as_ptr() as *const libc::c_char);
        if file.is_null() {
            return -1;
        }
        if libc::fseek(file, 0, 2) != 0 {
            libc::fclose(file);
            return -1;
        }
        let mut size: libc::c_long = libc::ftell(file);
        if size == libc::c_long::MAX {
            size = -1;
        }
        libc::fclose(file);
        size
    }
}

// --- functions/lodepng_free.rs (SACTOR-verified)
pub fn lodepng_free(ptr: *mut libc::c_void) {
    unsafe {
        libc::free(ptr);
    }
}

// --- functions/lodepng_get_raw_size_idat.rs (SACTOR-verified)
pub fn lodepng_get_raw_size_idat(w: u32, h: u32, bpp: u32) -> usize {
    let line: usize =
        (w / 8u32) as usize * bpp as usize + 1usize + (((w & 7u32) * bpp + 7u32) / 8u32) as usize;
    h as usize * line
}

// --- functions/lodepng_malloc.rs (SACTOR-verified)
use libc::{c_void, malloc, size_t};
pub unsafe fn lodepng_malloc(size: size_t) -> *mut c_void {
    malloc(size)
}

// --- functions/lodepng_memcpy.rs (SACTOR-verified)
pub fn lodepng_memcpy(dst: *mut libc::c_void, src: *const libc::c_void, size: libc::size_t) {
    unsafe {
        let dst = dst as *mut libc::c_char;
        let src = src as *const libc::c_char;
        let mut i: libc::size_t = 0;
        while i < size {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}

// --- functions/lodepng_memset.rs (SACTOR-verified)
pub unsafe fn lodepng_memset(dst: *mut libc::c_void, value: libc::c_int, num: libc::size_t) {
    let mut i: libc::size_t = 0;
    while i < num {
        *(dst as *mut libc::c_char).add(i) = value as libc::c_char;
        i += 1;
    }
}

// --- functions/lodepng_mulofl.rs (SACTOR-verified)
pub fn lodepng_mulofl(a: libc::size_t, b: libc::size_t, result: &mut libc::size_t) -> libc::c_int {
    *result = a.wrapping_mul(b);
    ((a != 0) && (*result / a != b)) as libc::c_int
}

// --- functions/lodepng_read32bitInt.rs (SACTOR-verified)
pub fn lodepng_read32bitInt(buffer: *const u8) -> u32 {
    unsafe {
        ((*buffer.offset(0) as u32) << 24)
            | ((*buffer.offset(1) as u32) << 16)
            | ((*buffer.offset(2) as u32) << 8)
            | (*buffer.offset(3) as u32)
    }
}

// --- functions/lodepng_realloc.rs (SACTOR-verified)
use libc::{c_void, realloc, size_t};
#[inline]
pub unsafe fn lodepng_realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void {
    #[cfg(LODEPNG_MAX_ALLOC)]
    {
        extern "C" {
            static LODEPNG_MAX_ALLOC: size_t;
        }
        if new_size > LODEPNG_MAX_ALLOC {
            return core::ptr::null_mut();
        }
    }
    realloc(ptr, new_size)
}

// --- functions/lodepng_save_file.rs (SACTOR-verified)
use libc::{c_char, c_uchar, size_t, FILE};
pub unsafe fn lodepng_save_file(
    buffer: *const c_uchar,
    buffersize: size_t,
    filename: *const c_char,
) -> u32 {
    extern "C" {
        fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
        fn fwrite(
            ptr: *const libc::c_void,
            size: size_t,
            nmemb: size_t,
            stream: *mut FILE,
        ) -> size_t;
        fn fclose(stream: *mut FILE) -> libc::c_int;
    }
    let mode = b"wb\0" as *const u8 as *const c_char;
    let file = fopen(filename, mode);
    if file.is_null() {
        return 79;
    }
    fwrite(buffer as *const libc::c_void, 1, buffersize, file);
    fclose(file);
    0
}

// --- functions/lodepng_set32bitInt.rs (SACTOR-verified)
pub fn lodepng_set32bitInt(buffer: *mut u8, value: libc::c_uint) {
    unsafe {
        *buffer.add(0) = ((value >> 24) & 0xff) as u8;
        *buffer.add(1) = ((value >> 16) & 0xff) as u8;
        *buffer.add(2) = ((value >> 8) & 0xff) as u8;
        *buffer.add(3) = (value & 0xff) as u8;
    }
}

// --- functions/paethPredictor.rs (SACTOR-verified)
pub fn paethPredictor(a: i16, b: i16, c: i16) -> u8 {
    fn lodepng_abs(x: i16) -> i16 {
        if x < 0 {
            -x
        } else {
            x
        }
    }
    let mut a_mut = a;
    let mut pa = lodepng_abs(b.wrapping_sub(c));
    let pb = lodepng_abs(a.wrapping_sub(c));
    let pc = lodepng_abs(a.wrapping_add(b).wrapping_sub(c).wrapping_sub(c));
    if pb < pa {
        a_mut = b;
        pa = pb;
    }
    let result = if pc < pa { c } else { a_mut };
    result as u8
}

// --- functions/peekBits.rs (SACTOR-verified)
#[inline]
pub unsafe fn peekBits(reader: *mut LodePNGBitReader, nbits: libc::size_t) -> libc::c_uint {
    (*reader).buffer & ((1u32 << nbits) - 1u32)
}

// --- functions/readBitFromReversedStream.rs (SACTOR-verified)
pub fn readBitFromReversedStream(bitpointer: *mut libc::size_t, bitstream: *const u8) -> u8 {
    unsafe {
        let byte_index = (*bitpointer >> 3) as isize;
        let bit_index = 7 - (*bitpointer & 0x7);
        let byte = *bitstream.offset(byte_index);
        let result = ((byte >> bit_index) & 1) as u8;
        *bitpointer = *bitpointer + 1;
        result
    }
}

// --- functions/reverseBits.rs (SACTOR-verified)
pub fn reverseBits(bits: u32, num: u32) -> u32 {
    fn inner(bits: u32, num: u32) -> u32 {
        let mut i: u32 = 0;
        let mut result: u32 = 0;
        while i < num {
            result |= ((bits >> (num - i - 1)) & 1) << i;
            i += 1;
        }
        result
    }
    inner(bits, num)
}

// --- functions/searchCodeIndex.rs (SACTOR-verified)
pub fn searchCodeIndex(array: *const u32, array_size: usize, value: usize) -> usize {
    unsafe {
        let mut left: usize = 1;
        let mut right: usize = array_size - 1;
        while left <= right {
            let mid: usize = (left + right) >> 1;
            if (*array.add(mid)) as usize >= value {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        if left >= array_size || (*array.add(left)) as usize > value {
            left -= 1;
        }
        left
    }
}

// --- functions/setBitOfReversedStream.rs (SACTOR-verified)
use libc::size_t;
pub fn setBitOfReversedStream(bitpointer: *mut size_t, bitstream: *mut u8, bit: u8) {
    unsafe {
        let index = (*bitpointer >> 3) as isize;
        let ptr = bitstream.offset(index);
        let bit_pos = 7u8 - (((*bitpointer) & 7) as u8);
        if bit == 0 {
            *ptr &= !(1u8 << bit_pos);
        } else {
            *ptr |= 1u8 << bit_pos;
        }
        *bitpointer += 1;
    }
}

// --- functions/ucvector_init.rs (SACTOR-verified)
#[inline]
pub unsafe fn ucvector_init(buffer: *mut libc::c_uchar, size: libc::size_t) -> ucvector {
    let mut v: ucvector = ucvector {
        data: core::ptr::null_mut(),
        size: 0,
        allocsize: 0,
    };
    v.data = buffer;
    v.allocsize = size;
    v.size = size;
    v
}

// --- functions/uivector_init.rs (SACTOR-verified)
#[no_mangle]
pub unsafe fn uivector_init(p: *mut uivector) {
    if p.is_null() {
        return;
    }
    (*p).data = core::ptr::null_mut();
    (*p).size = 0;
    (*p).allocsize = 0;
}

// --- functions/updateHashChain.rs (SACTOR-verified)
pub unsafe fn updateHashChain(hash: *mut Hash, wpos: usize, hashval: u32, numzeros: u16) {
    *(*hash).val.add(wpos) = hashval as ::core::ffi::c_int;
    if *(*hash).head.add(hashval as usize) != -1 {
        *(*hash).chain.add(wpos) = *(*hash).head.add(hashval as usize) as ::core::ffi::c_ushort;
    }
    *(*hash).head.add(hashval as usize) = wpos as ::core::ffi::c_int;
    *(*hash).zeros.add(wpos) = numzeros as ::core::ffi::c_ushort;
    if *(*hash).headz.add(numzeros as usize) != -1 {
        *(*hash).chainz.add(wpos) = *(*hash).headz.add(numzeros as usize) as ::core::ffi::c_ushort;
    }
    *(*hash).headz.add(numzeros as usize) = wpos as ::core::ffi::c_int;
}

// --- functions/update_adler32.rs (SACTOR-verified)
pub fn update_adler32(mut adler: u32, mut data: *const u8, mut len: u32) -> u32 {
    unsafe {
        let mut s1: u32 = adler & 0xffffu32;
        let mut s2: u32 = (adler >> 16) & 0xffffu32;
        while len != 0 {
            let amount: u32 = if len > 5552 { 5552 } else { len };
            len -= amount;
            let mut i: u32 = 0;
            while i != amount {
                s1 = s1.wrapping_add(*data as u32);
                data = data.add(1);
                s2 = s2.wrapping_add(s1);
                i += 1;
            }
            s1 %= 65521;
            s2 %= 65521;
        }
        (s2 << 16) | s1
    }
}

// --- log-extracted LodePNGIText_init__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn LodePNGIText_init(info: *mut LodePNGInfo) {
    (*info).itext_num = 0;
    (*info).itext_keys = ::core::ptr::null_mut();
    (*info).itext_langtags = ::core::ptr::null_mut();
    (*info).itext_transkeys = ::core::ptr::null_mut();
    (*info).itext_strings = ::core::ptr::null_mut();
}

// --- log-extracted LodePNGText_init__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn LodePNGText_init(info: *mut LodePNGInfo) {
    (*info).text_num = 0;
    (*info).text_keys = ::core::ptr::null_mut();
    (*info).text_strings = ::core::ptr::null_mut();
}

// --- log-extracted LodePNGUnknownChunks_init__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn LodePNGUnknownChunks_init(info: *mut LodePNGInfo) {
    let mut i: ::core::ffi::c_uint = 0;
    while i != 3 {
        (*info).unknown_chunks_data[i as usize] = ::core::ptr::null_mut();
        i = i.wrapping_add(1);
    }
    i = 0;
    while i != 3 {
        (*info).unknown_chunks_size[i as usize] = 0;
        i = i.wrapping_add(1);
    }
}

// --- log-extracted getPixelColorRGBA16__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn getPixelColorRGBA16(
    r: *mut u16,
    g: *mut u16,
    b: *mut u16,
    a: *mut u16,
    in_ptr: *const u8,
    i: usize,
    mode: *const LodePNGColorMode,
) {
    unsafe fn read_u16_be(base: *const u8, offset: usize) -> u16 {
        (256u16 * *base.add(offset) as u16) + *base.add(offset + 1) as u16
    }
    let mode_ref = &*mode;
    if mode_ref.colortype == LodePNGColorType::LCT_GREY {
        let gray = read_u16_be(in_ptr, i * 2);
        *r = gray;
        *g = gray;
        *b = gray;
        if mode_ref.key_defined != 0 && gray as u32 == mode_ref.key_r {
            *a = 0;
        } else {
            *a = 65535;
        }
    } else if mode_ref.colortype == LodePNGColorType::LCT_RGB {
        let r_val = read_u16_be(in_ptr, i * 6);
        let g_val = read_u16_be(in_ptr, i * 6 + 2);
        let b_val = read_u16_be(in_ptr, i * 6 + 4);
        *r = r_val;
        *g = g_val;
        *b = b_val;
        if mode_ref.key_defined != 0 && r_val as u32 == mode_ref.key_r
            && g_val as u32 == mode_ref.key_g && b_val as u32 == mode_ref.key_b
        {
            *a = 0;
        } else {
            *a = 65535;
        }
    } else if mode_ref.colortype == LodePNGColorType::LCT_GREY_ALPHA {
        let gray = read_u16_be(in_ptr, i * 4);
        let alpha = read_u16_be(in_ptr, i * 4 + 2);
        *r = gray;
        *g = gray;
        *b = gray;
        *a = alpha;
    } else if mode_ref.colortype == LodePNGColorType::LCT_RGBA {
        *r = read_u16_be(in_ptr, i * 8);
        *g = read_u16_be(in_ptr, i * 8 + 2);
        *b = read_u16_be(in_ptr, i * 8 + 4);
        *a = read_u16_be(in_ptr, i * 8 + 6);
    }
}

// --- log-extracted lodepng_color_mode_equal__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn lodepng_color_mode_equal(
    a: *const LodePNGColorMode,
    b: *const LodePNGColorMode,
) -> ::core::ffi::c_int {
    use crate::size_t;
    let a_ref: &LodePNGColorMode = &*a;
    let b_ref: &LodePNGColorMode = &*b;
    if a_ref.colortype != b_ref.colortype {
        return 0;
    }
    if a_ref.bitdepth != b_ref.bitdepth {
        return 0;
    }
    if a_ref.key_defined != b_ref.key_defined {
        return 0;
    }
    if a_ref.key_defined != 0 {
        if a_ref.key_r != b_ref.key_r {
            return 0;
        }
        if a_ref.key_g != b_ref.key_g {
            return 0;
        }
        if a_ref.key_b != b_ref.key_b {
            return 0;
        }
    }
    if a_ref.palettesize != b_ref.palettesize {
        return 0;
    }
    let mut i: size_t = 0;
    let total: size_t = a_ref.palettesize.wrapping_mul(4);
    while i != total {
        let ai = *a_ref.palette.add(i as usize);
        let bi = *b_ref.palette.add(i as usize);
        if ai != bi {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

// --- log-extracted lodepng_color_mode_init__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn lodepng_color_mode_init(info: *mut LodePNGColorMode) {
    (*info).key_defined = 0;
    (*info).key_r = 0;
    (*info).key_g = 0;
    (*info).key_b = 0;
    (*info).colortype = LodePNGColorType::LCT_RGBA;
    (*info).bitdepth = 8;
    (*info).palette = core::ptr::null_mut();
    (*info).palettesize = 0;
}

// --- log-extracted lodepng_convert_rgb__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn lodepng_convert_rgb(
    r_out: *mut u32,
    g_out: *mut u32,
    b_out: *mut u32,
    r_in: u32,
    g_in: u32,
    b_in: u32,
    mode_out: *const LodePNGColorMode,
    mode_in: *const LodePNGColorMode,
) -> u32 {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    let mode_in_ref = &*mode_in;
    let mode_out_ref = &*mode_out;
    let mul: u32 = 65535u32 / ((1u32 << mode_in_ref.bitdepth) - 1u32);
    let shift: u32 = 16u32 - mode_out_ref.bitdepth;
    if mode_in_ref.colortype as u8 == LodePNGColorType::LCT_GREY as u8
        || mode_in_ref.colortype as u8 == LodePNGColorType::LCT_GREY_ALPHA as u8
    {
        r = r_in.wrapping_mul(mul);
        g = r;
        b = r;
    } else if mode_in_ref.colortype as u8 == LodePNGColorType::LCT_RGB as u8
        || mode_in_ref.colortype as u8 == LodePNGColorType::LCT_RGBA as u8
    {
        r = r_in.wrapping_mul(mul);
        g = g_in.wrapping_mul(mul);
        b = b_in.wrapping_mul(mul);
    } else if mode_in_ref.colortype as u8 == LodePNGColorType::LCT_PALETTE as u8 {
        if (r_in as usize) >= mode_in_ref.palettesize as usize {
            return 82;
        }
        let idx = (r_in as usize) * 4;
        let pal_ptr = mode_in_ref.palette;
        r = (*pal_ptr.add(idx + 0)) as u32 * 257u32;
        g = (*pal_ptr.add(idx + 1)) as u32 * 257u32;
        b = (*pal_ptr.add(idx + 2)) as u32 * 257u32;
    } else {
        return 31;
    }
    if mode_out_ref.colortype as u8 == LodePNGColorType::LCT_GREY as u8
        || mode_out_ref.colortype as u8 == LodePNGColorType::LCT_GREY_ALPHA as u8
    {
        *r_out = r >> shift;
    } else if mode_out_ref.colortype as u8 == LodePNGColorType::LCT_RGB as u8
        || mode_out_ref.colortype as u8 == LodePNGColorType::LCT_RGBA as u8
    {
        *r_out = r >> shift;
        *g_out = g >> shift;
        *b_out = b >> shift;
    } else if mode_out_ref.colortype as u8 == LodePNGColorType::LCT_PALETTE as u8 {
        if (r >> 8) != (r & 255) || (g >> 8) != (g & 255) || (b >> 8) != (b & 255) {
            return 82;
        }
        let mut i: u32 = 0;
        while (i as usize) < mode_out_ref.palettesize as usize {
            let j = (i as usize) * 4;
            let pal_ptr = mode_out_ref.palette;
            let pr = (*pal_ptr.add(j + 0)) as u32;
            let pg = (*pal_ptr.add(j + 1)) as u32;
            let pb = (*pal_ptr.add(j + 2)) as u32;
            if (r >> 8) == pr && (g >> 8) == pg && (b >> 8) == pb {
                *r_out = i;
                return 0;
            }
            i = i.wrapping_add(1);
        }
        return 82;
    } else {
        return 31;
    }
    0
}

// --- log-extracted lodepng_gtofl.rs (attempt 1 of 1, verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_lodepng/lodepng.c: Error: Failed to link project-level harness fo; status untranslated)
fn lodepng_gtofl(a: libc::size_t, b: libc::size_t, c: libc::size_t) -> libc::c_int {
    let mut d: libc::size_t = 0;
    if unsafe { lodepng_addofl(a, b, &mut d as *mut libc::size_t) } != 0 {
        return 1;
    }
    if d > c { 1 } else { 0 }
}

// --- log-extracted lodepng_has_palette_alpha__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn lodepng_has_palette_alpha(
    info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let mut i: libc::size_t = 0;
    while i != (*info).palettesize {
        let alpha = *(*info).palette.add(i.wrapping_mul(4).wrapping_add(3));
        if alpha < 255 {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}

// --- log-extracted lodepng_is_alpha_type__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn lodepng_is_alpha_type(
    info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    (((*info).colortype as u8) & 4 != 0) as ::core::ffi::c_uint
}

// --- log-extracted lodepng_is_greyscale_type__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn lodepng_is_greyscale_type(
    info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let info_ref = &*info;
    (info_ref.colortype == LodePNGColorType::LCT_GREY
        || info_ref.colortype == LodePNGColorType::LCT_GREY_ALPHA) as ::core::ffi::c_uint
}

// --- log-extracted lodepng_is_palette_type__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn lodepng_is_palette_type(info: *const LodePNGColorMode) -> libc::c_uint {
    if (*info).colortype == 3 { 1 } else { 0 }
}

// --- log-extracted readChunk_bKGD__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_bKGD(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    unsafe fn byte_at(
        ptr: *const ::core::ffi::c_uchar,
        idx: usize,
    ) -> ::core::ffi::c_uint {
        *ptr.add(idx) as ::core::ffi::c_uint
    }
    let info_ref: &mut LodePNGInfo = &mut *info;
    let color: &LodePNGColorMode = &info_ref.color;
    if color.colortype == LodePNGColorType::LCT_PALETTE {
        if chunkLength != 1 {
            return 43;
        }
        if byte_at(data, 0) as usize >= color.palettesize {
            return 103;
        }
        info_ref.background_defined = 1;
        let v = byte_at(data, 0);
        info_ref.background_r = v;
        info_ref.background_g = v;
        info_ref.background_b = v;
    } else if color.colortype == LodePNGColorType::LCT_GREY
        || color.colortype == LodePNGColorType::LCT_GREY_ALPHA
    {
        if chunkLength != 2 {
            return 44;
        }
        info_ref.background_defined = 1;
        let v = 256u32 * byte_at(data, 0) + byte_at(data, 1);
        info_ref.background_r = v;
        info_ref.background_g = v;
        info_ref.background_b = v;
    } else if color.colortype == LodePNGColorType::LCT_RGB
        || color.colortype == LodePNGColorType::LCT_RGBA
    {
        if chunkLength != 6 {
            return 45;
        }
        info_ref.background_defined = 1;
        info_ref.background_r = 256u32 * byte_at(data, 0) + byte_at(data, 1);
        info_ref.background_g = 256u32 * byte_at(data, 2) + byte_at(data, 3);
        info_ref.background_b = 256u32 * byte_at(data, 4) + byte_at(data, 5);
    }
    0
}

// --- log-extracted readChunk_cHRM__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_cHRM(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    if chunkLength != 32 {
        return 97;
    }
    unsafe fn read_be_u32(
        ptr: *const ::core::ffi::c_uchar,
        offset: usize,
    ) -> ::core::ffi::c_uint {
        (16777216u32 * *ptr.add(offset + 0) as u32)
            + (65536u32 * *ptr.add(offset + 1) as u32)
            + (256u32 * *ptr.add(offset + 2) as u32) + (*ptr.add(offset + 3) as u32)
    }
    (*info).chrm_defined = 1;
    (*info).chrm_white_x = read_be_u32(data, 0);
    (*info).chrm_white_y = read_be_u32(data, 4);
    (*info).chrm_red_x = read_be_u32(data, 8);
    (*info).chrm_red_y = read_be_u32(data, 12);
    (*info).chrm_green_x = read_be_u32(data, 16);
    (*info).chrm_green_y = read_be_u32(data, 20);
    (*info).chrm_blue_x = read_be_u32(data, 24);
    (*info).chrm_blue_y = read_be_u32(data, 28);
    0
}

// --- log-extracted readChunk_gAMA__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_gAMA(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> u32 {
    if chunkLength != 4 {
        return 96;
    }
    unsafe fn at(ptr: *const ::core::ffi::c_uchar, idx: usize) -> u32 {
        *ptr.add(idx) as u32
    }
    (*info).gama_defined = 1;
    (*info).gama_gamma = 16_777_216u32 * at(data, 0) + 65_536u32 * at(data, 1)
        + 256u32 * at(data, 2) + at(data, 3);
    0
}

// --- log-extracted readChunk_pHYs__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_pHYs(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    if chunkLength != 9 {
        return 74;
    }
    unsafe fn get_byte(
        ptr: *const ::core::ffi::c_uchar,
        offset: isize,
    ) -> ::core::ffi::c_uint {
        *ptr.offset(offset) as ::core::ffi::c_uint
    }
    (*info).phys_defined = 1;
    (*info).phys_x = 16_777_216u32 * get_byte(data, 0) + 65_536u32 * get_byte(data, 1)
        + 256u32 * get_byte(data, 2) + get_byte(data, 3);
    (*info).phys_y = 16_777_216u32 * get_byte(data, 4) + 65_536u32 * get_byte(data, 5)
        + 256u32 * get_byte(data, 6) + get_byte(data, 7);
    (*info).phys_unit = get_byte(data, 8);
    0
}

// --- log-extracted readChunk_sBIT__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_sBIT(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: size_t,
) -> ::core::ffi::c_uint {
    #[inline]
    unsafe fn byte_at(
        ptr: *const ::core::ffi::c_uchar,
        idx: size_t,
    ) -> ::core::ffi::c_uchar {
        *ptr.add(idx)
    }
    let info_ref: &mut LodePNGInfo = &mut *info;
    let colortype = info_ref.color.colortype;
    let bitdepth: ::core::ffi::c_uint = if colortype == LodePNGColorType::LCT_PALETTE {
        8
    } else {
        info_ref.color.bitdepth
    };
    if colortype == LodePNGColorType::LCT_GREY {
        if chunkLength != 1 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        if d0 == 0 || d0 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d0;
        info_ref.sbit_b = d0;
    } else if colortype == LodePNGColorType::LCT_RGB
        || colortype == LodePNGColorType::LCT_PALETTE
    {
        if chunkLength != 3 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        let d1 = byte_at(data, 1) as ::core::ffi::c_uint;
        let d2 = byte_at(data, 2) as ::core::ffi::c_uint;
        if d0 == 0 || d1 == 0 || d2 == 0 {
            return 115;
        }
        if d0 > bitdepth || d1 > bitdepth || d2 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d1;
        info_ref.sbit_b = d2;
    } else if colortype == LodePNGColorType::LCT_GREY_ALPHA {
        if chunkLength != 2 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        let d1 = byte_at(data, 1) as ::core::ffi::c_uint;
        if d0 == 0 || d1 == 0 {
            return 115;
        }
        if d0 > bitdepth || d1 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d0;
        info_ref.sbit_b = d0;
        info_ref.sbit_a = d1;
    } else if colortype == LodePNGColorType::LCT_RGBA {
        if chunkLength != 4 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        let d1 = byte_at(data, 1) as ::core::ffi::c_uint;
        let d2 = byte_at(data, 2) as ::core::ffi::c_uint;
        let d3 = byte_at(data, 3) as ::core::ffi::c_uint;
        if d0 == 0 || d1 == 0 || d2 == 0 || d3 == 0 {
            return 115;
        }
        if d0 > bitdepth || d1 > bitdepth || d2 > bitdepth || d3 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d1;
        info_ref.sbit_b = d2;
        info_ref.sbit_a = d3;
    }
    0
}

// --- log-extracted readChunk_sRGB__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_sRGB(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 1 {
        return 98;
    }
    (*info).srgb_defined = 1 as ::core::ffi::c_uint;
    (*info).srgb_intent = *data as ::core::ffi::c_uint;
    0
}

// --- log-extracted readChunk_tIME__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_tIME(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    if chunkLength != 7 {
        return 73;
    }
    let info_ref = &mut *info;
    info_ref.time_defined = 1;
    let d0 = *data.add(0) as ::core::ffi::c_uint;
    let d1 = *data.add(1) as ::core::ffi::c_uint;
    info_ref.time.year = 256u32.wrapping_mul(d0) + d1;
    info_ref.time.month = *data.add(2) as ::core::ffi::c_uint;
    info_ref.time.day = *data.add(3) as ::core::ffi::c_uint;
    info_ref.time.hour = *data.add(4) as ::core::ffi::c_uint;
    info_ref.time.minute = *data.add(5) as ::core::ffi::c_uint;
    info_ref.time.second = *data.add(6) as ::core::ffi::c_uint;
    0
}

// --- log-extracted readChunk_tRNS__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn readChunk_tRNS(
    color: *mut LodePNGColorMode,
    data: *const ::core::ffi::c_uchar,
    chunkLength: libc::size_t,
) -> libc::c_uint {
    if (*color).colortype == LodePNGColorType::LCT_PALETTE {
        if chunkLength > (*color).palettesize {
            return 39;
        }
        let mut i: libc::size_t = 0;
        while i != chunkLength {
            *(*color).palette.add(4usize.wrapping_mul(i).wrapping_add(3)) = *data.add(i);
            i = i.wrapping_add(1);
        }
    } else if (*color).colortype == LodePNGColorType::LCT_GREY {
        if chunkLength != 2 {
            return 30;
        }
        (*color).key_defined = 1;
        let v: libc::c_uint = 256u32.wrapping_mul(*data.add(0) as libc::c_uint)
            + (*data.add(1) as libc::c_uint);
        (*color).key_r = v;
        (*color).key_g = v;
        (*color).key_b = v;
    } else if (*color).colortype == LodePNGColorType::LCT_RGB {
        if chunkLength != 6 {
            return 41;
        }
        (*color).key_defined = 1;
        (*color).key_r = 256u32.wrapping_mul(*data.add(0) as libc::c_uint)
            + (*data.add(1) as libc::c_uint);
        (*color).key_g = 256u32.wrapping_mul(*data.add(2) as libc::c_uint)
            + (*data.add(3) as libc::c_uint);
        (*color).key_b = 256u32.wrapping_mul(*data.add(4) as libc::c_uint)
            + (*data.add(5) as libc::c_uint);
    } else {
        return 42;
    }
    0
}

// --- log-extracted rgba16ToPixel__attempt6.rs (attempt 6 of 6, verdict: Rust code failed to compile; status failure)
pub unsafe fn rgba16ToPixel(
    out: *mut ::core::ffi::c_uchar,
    i: usize,
    mode: *const LodePNGColorMode,
    r: u16,
    g: u16,
    b: u16,
    a: u16,
) {
    let mode_ref = &*mode;
    if mode_ref.colortype == 0 {
        let gray: u16 = r;
        *out.add(i * 2 + 0) = ((gray >> 8) & 255) as u8;
        *out.add(i * 2 + 1) = (gray & 255) as u8;
    } else if mode_ref.colortype == 2 {
        *out.add(i * 6 + 0) = ((r >> 8) & 255) as u8;
        *out.add(i * 6 + 1) = (r & 255) as u8;
        *out.add(i * 6 + 2) = ((g >> 8) & 255) as u8;
        *out.add(i * 6 + 3) = (g & 255) as u8;
        *out.add(i * 6 + 4) = ((b >> 8) & 255) as u8;
        *out.add(i * 6 + 5) = (b & 255) as u8;
    } else if mode_ref.colortype == 4 {
        let gray: u16 = r;
        *out.add(i * 4 + 0) = ((gray >> 8) & 255) as u8;
        *out.add(i * 4 + 1) = (gray & 255) as u8;
        *out.add(i * 4 + 2) = ((a >> 8) & 255) as u8;
        *out.add(i * 4 + 3) = (a & 255) as u8;
    } else if mode_ref.colortype == 6 {
        *out.add(i * 8 + 0) = ((r >> 8) & 255) as u8;
        *out.add(i * 8 + 1) = (r & 255) as u8;
        *out.add(i * 8 + 2) = ((g >> 8) & 255) as u8;
        *out.add(i * 8 + 3) = (g & 255) as u8;
        *out.add(i * 8 + 4) = ((b >> 8) & 255) as u8;
        *out.add(i * 8 + 5) = (b & 255) as u8;
        *out.add(i * 8 + 6) = ((a >> 8) & 255) as u8;
        *out.add(i * 8 + 7) = (a & 255) as u8;
    }
}

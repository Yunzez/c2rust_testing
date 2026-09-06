// GENERATED for the RQ4 coverage experiment by scratchpad/rq4_cov/flatten_rust.py.
// Module bodies are copied byte-for-byte from
// tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/ (== fuzz/bzip2_c2rust_e3/src/).
// Only the module wrappers and the root re-exports below are added.
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut, internal_features,
         unused_imports, unpredictable_function_pointer_comparisons)]

pub mod blocksort {

extern "C" {
    pub type __sFILEX;
    
    
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub use crate::bzlib::BZ2_bz__AssertH__fail;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut std::os::raw::c_char,
    pub avail_in: std::os::raw::c_uint,
    pub total_in_lo32: std::os::raw::c_uint,
    pub total_in_hi32: std::os::raw::c_uint,
    pub next_out: *mut std::os::raw::c_char,
    pub avail_out: std::os::raw::c_uint,
    pub total_out_lo32: std::os::raw::c_uint,
    pub total_out_hi32: std::os::raw::c_uint,
    pub state: *mut std::os::raw::c_void,
    pub bzalloc: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                             _: std::os::raw::c_int, _: std::os::raw::c_int)
                            -> *mut std::os::raw::c_void>,
    pub bzfree: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *mut std::os::raw::c_void) -> ()>,
    pub opaque: *mut std::os::raw::c_void,
}
pub type Bool = std::os::raw::c_uchar;
pub type UChar = std::os::raw::c_uchar;
pub type Int32 = std::os::raw::c_int;
pub type UInt32 = std::os::raw::c_uint;
pub type UInt16 = std::os::raw::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: Int32,
    pub state: Int32,
    pub avail_in_expect: UInt32,
    pub arr1: *mut UInt32,
    pub arr2: *mut UInt32,
    pub ftab: *mut UInt32,
    pub origPtr: Int32,
    pub ptr: *mut UInt32,
    pub block: *mut UChar,
    pub mtfv: *mut UInt16,
    pub zbits: *mut UChar,
    pub workFactor: Int32,
    pub state_in_ch: UInt32,
    pub state_in_len: Int32,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub nblock: Int32,
    pub nblockMAX: Int32,
    pub numZ: Int32,
    pub state_out_pos: Int32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub unseqToSeq: [UChar; 256],
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockCRC: UInt32,
    pub combinedCRC: UInt32,
    pub verbosity: Int32,
    pub blockNo: Int32,
    pub blockSize100k: Int32,
    pub nMTF: Int32,
    pub mtfFreq: [Int32; 258],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub code: [[Int32; 258]; 6],
    pub rfreq: [[Int32; 258]; 6],
    pub len_pack: [[UInt32; 4]; 258],
}
/*-------------------------------------------------------------*/
/*--- Block sorting machinery                               ---*/
/*---                                           blocksort.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/*---------------------------------------------*/
/*--- Fallback O(N log(N)^2) sorting        ---*/
/*--- algorithm, for repetitive blocks      ---*/
/*---------------------------------------------*/
/*---------------------------------------------*/
#[inline]
unsafe extern "C" fn fallbackSimpleSort(mut fmap: *mut UInt32,
                                        mut eclass: *mut UInt32,
                                        mut lo: Int32, mut hi: Int32) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: Int32 = 0;
    let mut ec_tmp: UInt32 = 0;
    if lo == hi { return }
    if hi - lo > 3 as std::os::raw::c_int {
        i = hi - 4 as std::os::raw::c_int;
        while i >= lo {
            tmp = *fmap.offset(i as isize) as Int32;
            ec_tmp = *eclass.offset(tmp as isize);
            j = i + 4 as std::os::raw::c_int;
            while j <= hi &&
                      ec_tmp >
                          *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j - 4 as std::os::raw::c_int) as isize) =
                    *fmap.offset(j as isize);
                j += 4 as std::os::raw::c_int
            }
            *fmap.offset((j - 4 as std::os::raw::c_int) as isize) = tmp as UInt32;
            i -= 1
        }
    }
    i = hi - 1 as std::os::raw::c_int;
    while i >= lo {
        tmp = *fmap.offset(i as isize) as Int32;
        ec_tmp = *eclass.offset(tmp as isize);
        j = i + 1 as std::os::raw::c_int;
        while j <= hi &&
                  ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
            *fmap.offset((j - 1 as std::os::raw::c_int) as isize) =
                *fmap.offset(j as isize);
            j += 1
        }
        *fmap.offset((j - 1 as std::os::raw::c_int) as isize) = tmp as UInt32;
        i -= 1
    };
}
unsafe extern "C" fn fallbackQSort3(mut fmap: *mut UInt32,
                                    mut eclass: *mut UInt32, mut loSt: Int32,
                                    mut hiSt: Int32) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut med: UInt32 = 0;
    let mut r: UInt32 = 0;
    let mut r3: UInt32 = 0;
    let mut stackLo: [Int32; 100] = [0; 100];
    let mut stackHi: [Int32; 100] = [0; 100];
    r = 0 as std::os::raw::c_int as UInt32;
    sp = 0 as std::os::raw::c_int;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    sp += 1;
    while sp > 0 as std::os::raw::c_int {
        if !(sp < 100 as std::os::raw::c_int - 1 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(1004 as std::os::raw::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        if hi - lo < 10 as std::os::raw::c_int {
            fallbackSimpleSort(fmap, eclass, lo, hi);
        } else {
            /* Random partitioning.  Median of 3 sometimes fails to
         avoid bad cases.  Median of 9 seems to help but 
         looks rather expensive.  This too seems to work but
         is cheaper.  Guidance for the magic constants 
         7621 and 32768 is taken from Sedgewick's algorithms
         book, chapter 35.
      */
            r =
                r.wrapping_mul(7621 as std::os::raw::c_int as
                                   std::os::raw::c_uint).wrapping_add(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_uint).wrapping_rem(32768
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_uint);
            r3 = r.wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint);
            if r3 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                med = *eclass.offset(*fmap.offset(lo as isize) as isize)
            } else if r3 == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                med =
                    *eclass.offset(*fmap.offset((lo + hi >> 1 as std::os::raw::c_int)
                                                    as isize) as isize)
            } else {
                med = *eclass.offset(*fmap.offset(hi as isize) as isize)
            }
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            loop  {
                while !(unLo > unHi) {
                    n =
                        *eclass.offset(*fmap.offset(unLo as isize) as isize)
                            as Int32 - med as Int32;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp: Int32 =
                            *fmap.offset(unLo as isize) as Int32;
                        *fmap.offset(unLo as isize) =
                            *fmap.offset(ltLo as isize);
                        *fmap.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1
                    } else { if n > 0 as std::os::raw::c_int { break ; } unLo += 1 }
                }
                while !(unLo > unHi) {
                    n =
                        *eclass.offset(*fmap.offset(unHi as isize) as isize)
                            as Int32 - med as Int32;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp_0: Int32 =
                            *fmap.offset(unHi as isize) as Int32;
                        *fmap.offset(unHi as isize) =
                            *fmap.offset(gtHi as isize);
                        *fmap.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1
                    } else { if n < 0 as std::os::raw::c_int { break ; } unHi -= 1 }
                }
                if unLo > unHi { break ; }
                let mut zztmp_1: Int32 = *fmap.offset(unLo as isize) as Int32;
                *fmap.offset(unLo as isize) = *fmap.offset(unHi as isize);
                *fmap.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1
            }
            if gtHi < ltLo { continue ; }
            n =
                if ltLo - lo < unLo - ltLo {
                    (ltLo) - lo
                } else { (unLo) - ltLo };
            let mut yyp1: Int32 = lo;
            let mut yyp2: Int32 = unLo - n;
            let mut yyn: Int32 = n;
            while yyn > 0 as std::os::raw::c_int {
                let mut zztmp_2: Int32 = *fmap.offset(yyp1 as isize) as Int32;
                *fmap.offset(yyp1 as isize) = *fmap.offset(yyp2 as isize);
                *fmap.offset(yyp2 as isize) = zztmp_2 as UInt32;
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1
            }
            m =
                if hi - gtHi < gtHi - unHi {
                    (hi) - gtHi
                } else { (gtHi) - unHi };
            let mut yyp1_0: Int32 = unLo;
            let mut yyp2_0: Int32 = hi - m + 1 as std::os::raw::c_int;
            let mut yyn_0: Int32 = m;
            while yyn_0 > 0 as std::os::raw::c_int {
                let mut zztmp_3: Int32 =
                    *fmap.offset(yyp1_0 as isize) as Int32;
                *fmap.offset(yyp1_0 as isize) = *fmap.offset(yyp2_0 as isize);
                *fmap.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                yyp1_0 += 1;
                yyp2_0 += 1;
                yyn_0 -= 1
            }
            n = lo + unLo - ltLo - 1 as std::os::raw::c_int;
            m = hi - (gtHi - unHi) + 1 as std::os::raw::c_int;
            if n - lo > hi - m {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1;
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1
            } else {
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1;
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1
            }
        }
    };
}
unsafe extern "C" fn fallbackSort(mut fmap: *mut UInt32,
                                  mut eclass: *mut UInt32,
                                  mut bhtab: *mut UInt32, mut nblock: Int32,
                                  mut verb: Int32) {
    let mut ftab: [Int32; 257] = [0; 257];
    let mut ftabCopy: [Int32; 256] = [0; 256];
    let mut H: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut l: Int32 = 0;
    let mut r: Int32 = 0;
    let mut cc: Int32 = 0;
    let mut cc1: Int32 = 0;
    let mut nNotDone: Int32 = 0;
    let mut nBhtab: Int32 = 0;
    let mut eclass8: *mut UChar = eclass as *mut UChar;
    /*--
      Initial 1-char radix sort to generate
      initial fmap and initial BH bits.
   --*/
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        bucket sorting ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    i = 0 as std::os::raw::c_int;
    while i < 257 as std::os::raw::c_int {
        ftab[i as usize] = 0 as std::os::raw::c_int;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nblock {
        ftab[*eclass8.offset(i as isize) as usize] += 1;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        ftabCopy[i as usize] = ftab[i as usize];
        i += 1
    }
    i = 1 as std::os::raw::c_int;
    while i < 257 as std::os::raw::c_int {
        ftab[i as usize] += ftab[(i - 1 as std::os::raw::c_int) as usize];
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nblock {
        j = *eclass8.offset(i as isize) as Int32;
        k = ftab[j as usize] - 1 as std::os::raw::c_int;
        ftab[j as usize] = k;
        *fmap.offset(k as isize) = i as UInt32;
        i += 1
    }
    nBhtab = 2 as std::os::raw::c_int + nblock / 32 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < nBhtab {
        *bhtab.offset(i as isize) = 0 as std::os::raw::c_int as UInt32;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        let ref mut fresh0 =
            *bhtab.offset((ftab[i as usize] >> 5 as std::os::raw::c_int) as isize);
        *fresh0 |=
            (1 as std::os::raw::c_int as UInt32) <<
                (ftab[i as usize] & 31 as std::os::raw::c_int);
        i += 1
    }
    /*--
      Inductively refine the buckets.  Kind-of an
      "exponential radix sort" (!), inspired by the
      Manber-Myers suffix array construction algorithm.
   --*/
    /*-- set sentinel bits for block-end detection --*/
    i = 0 as std::os::raw::c_int;
    while i < 32 as std::os::raw::c_int {
        let ref mut fresh1 =
            *bhtab.offset((nblock + 2 as std::os::raw::c_int * i >> 5 as std::os::raw::c_int)
                              as isize);
        *fresh1 |=
            (1 as std::os::raw::c_int as UInt32) <<
                (nblock + 2 as std::os::raw::c_int * i & 31 as std::os::raw::c_int);
        let ref mut fresh2 =
            *bhtab.offset((nblock + 2 as std::os::raw::c_int * i + 1 as std::os::raw::c_int >>
                               5 as std::os::raw::c_int) as isize);
        *fresh2 &=
            !((1 as std::os::raw::c_int as UInt32) <<
                  (nblock + 2 as std::os::raw::c_int * i + 1 as std::os::raw::c_int &
                       31 as std::os::raw::c_int));
        i += 1
    }
    /*-- the log(N) loop --*/
    H = 1 as std::os::raw::c_int;
    loop  {
        if verb >= 4 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"        depth %6d has \x00" as *const u8 as
                        *const std::os::raw::c_char, H);
        }
        j = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < nblock {
            if *bhtab.offset((i >> 5 as std::os::raw::c_int) as isize) &
                   (1 as std::os::raw::c_int as UInt32) << (i & 31 as std::os::raw::c_int) !=
                   0 {
                j = i
            }
            k =
                (*fmap.offset(i as isize)).wrapping_sub(H as std::os::raw::c_uint) as
                    Int32;
            if k < 0 as std::os::raw::c_int { k += nblock }
            *eclass.offset(k as isize) = j as UInt32;
            i += 1
        }
        nNotDone = 0 as std::os::raw::c_int;
        r = -(1 as std::os::raw::c_int);
        loop  {
            /*-- find the next non-singleton bucket --*/
            k = r + 1 as std::os::raw::c_int;
            while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                      (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int)
                      != 0 && k & 0x1f as std::os::raw::c_int != 0 {
                k += 1
            }
            if *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                   (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int) !=
                   0 {
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) ==
                          0xffffffff as std::os::raw::c_uint {
                    k += 32 as std::os::raw::c_int
                }
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                          (1 as std::os::raw::c_int as UInt32) <<
                              (k & 31 as std::os::raw::c_int) != 0 {
                    k += 1
                }
            }
            l = k - 1 as std::os::raw::c_int;
            if l >= nblock { break ; }
            while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                      (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int)
                      == 0 && k & 0x1f as std::os::raw::c_int != 0 {
                k += 1
            }
            if *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                   (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int) ==
                   0 {
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) ==
                          0 as std::os::raw::c_int as std::os::raw::c_uint {
                    k += 32 as std::os::raw::c_int
                }
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                          (1 as std::os::raw::c_int as UInt32) <<
                              (k & 31 as std::os::raw::c_int) == 0 {
                    k += 1
                }
            }
            r = k - 1 as std::os::raw::c_int;
            if r >= nblock { break ; }
            /*-- now [l, r] bracket current bucket --*/
            if r > l {
                nNotDone += r - l + 1 as std::os::raw::c_int;
                fallbackQSort3(fmap, eclass, l, r);
                /*-- scan bucket and generate header bits-- */
                cc = -(1 as std::os::raw::c_int);
                i = l;
                while i <= r {
                    cc1 =
                        *eclass.offset(*fmap.offset(i as isize) as isize) as
                            Int32;
                    if cc != cc1 {
                        let ref mut fresh3 =
                            *bhtab.offset((i >> 5 as std::os::raw::c_int) as isize);
                        *fresh3 |=
                            (1 as std::os::raw::c_int as UInt32) <<
                                (i & 31 as std::os::raw::c_int);
                        cc = cc1
                    }
                    i += 1
                }
            }
        }
        if verb >= 4 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%6d unresolved strings\n\x00" as *const u8 as
                        *const std::os::raw::c_char, nNotDone);
        }
        H *= 2 as std::os::raw::c_int;
        if H > nblock || nNotDone == 0 as std::os::raw::c_int { break ; }
    }
    /*-- 
      Reconstruct the original block in
      eclass8 [0 .. nblock-1], since the
      previous phase destroyed it.
   --*/
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        reconstructing block ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    j = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < nblock {
        while ftabCopy[j as usize] == 0 as std::os::raw::c_int { j += 1 }
        ftabCopy[j as usize] -= 1;
        *eclass8.offset(*fmap.offset(i as isize) as isize) = j as UChar;
        i += 1
    }
    if !(j < 256 as std::os::raw::c_int) {
        BZ2_bz__AssertH__fail(1005 as std::os::raw::c_int);
    };
}
/*---------------------------------------------*/
/*--- The main, O(N^2 log(N)) sorting       ---*/
/*--- algorithm.  Faster for "normal"       ---*/
/*--- non-repetitive blocks.                ---*/
/*---------------------------------------------*/
/*---------------------------------------------*/
#[inline]
unsafe extern "C" fn mainGtU(mut i1: UInt32, mut i2: UInt32,
                             mut block: *mut UChar, mut quadrant: *mut UInt16,
                             mut nblock: UInt32, mut budget: *mut Int32)
 -> Bool {
    let mut k: Int32 = 0;
    let mut c1: UChar = 0;
    let mut c2: UChar = 0;
    let mut s1: UInt16 = 0;
    let mut s2: UInt16 = 0;
    /* 1 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 2 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 3 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 4 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 5 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 6 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 7 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 8 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 9 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 10 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 11 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 12 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    k = nblock.wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_uint) as Int32;
    loop  {
        /* 1 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 2 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 3 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 4 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 5 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 6 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 7 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 8 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        if i1 >= nblock {
            i1 = (i1 as std::os::raw::c_uint).wrapping_sub(nblock) as UInt32 as UInt32
        }
        if i2 >= nblock {
            i2 = (i2 as std::os::raw::c_uint).wrapping_sub(nblock) as UInt32 as UInt32
        }
        k -= 8 as std::os::raw::c_int;
        *budget -= 1;
        if !(k >= 0 as std::os::raw::c_int) { break ; }
    }
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
/*--
   Knuth's increments seem to work better
   than Incerpi-Sedgewick here.  Possibly
   because the number of elems to sort is
   usually small, typically <= 20.
--*/
static mut incs: [Int32; 14] =
    [1 as std::os::raw::c_int, 4 as std::os::raw::c_int, 13 as std::os::raw::c_int, 40 as std::os::raw::c_int,
     121 as std::os::raw::c_int, 364 as std::os::raw::c_int, 1093 as std::os::raw::c_int,
     3280 as std::os::raw::c_int, 9841 as std::os::raw::c_int, 29524 as std::os::raw::c_int,
     88573 as std::os::raw::c_int, 265720 as std::os::raw::c_int, 797161 as std::os::raw::c_int,
     2391484 as std::os::raw::c_int];
unsafe extern "C" fn mainSimpleSort(mut ptr: *mut UInt32,
                                    mut block: *mut UChar,
                                    mut quadrant: *mut UInt16,
                                    mut nblock: Int32, mut lo: Int32,
                                    mut hi: Int32, mut d: Int32,
                                    mut budget: *mut Int32) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut h: Int32 = 0;
    let mut bigN: Int32 = 0;
    let mut hp: Int32 = 0;
    let mut v: UInt32 = 0;
    bigN = hi - lo + 1 as std::os::raw::c_int;
    if bigN < 2 as std::os::raw::c_int { return }
    hp = 0 as std::os::raw::c_int;
    while incs[hp as usize] < bigN { hp += 1 }
    hp -= 1;
    while hp >= 0 as std::os::raw::c_int {
        h = incs[hp as usize];
        i = lo + h;
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*-- copy 1 --*/
            if i > hi { break ; }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU((*ptr.offset((j - h) as
                                           isize)).wrapping_add(d as
                                                                    std::os::raw::c_uint),
                          v.wrapping_add(d as std::os::raw::c_uint), block, quadrant,
                          nblock as UInt32, budget) != 0 {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as std::os::raw::c_int { break ; }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            /*-- copy 2 --*/
            if i > hi { break ; }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU((*ptr.offset((j - h) as
                                           isize)).wrapping_add(d as
                                                                    std::os::raw::c_uint),
                          v.wrapping_add(d as std::os::raw::c_uint), block, quadrant,
                          nblock as UInt32, budget) != 0 {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as std::os::raw::c_int { break ; }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            /*-- copy 3 --*/
            if i > hi { break ; }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU((*ptr.offset((j - h) as
                                           isize)).wrapping_add(d as
                                                                    std::os::raw::c_uint),
                          v.wrapping_add(d as std::os::raw::c_uint), block, quadrant,
                          nblock as UInt32, budget) != 0 {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as std::os::raw::c_int { break ; }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if *budget < 0 as std::os::raw::c_int { return }
        }
        hp -= 1
    };
}
/*---------------------------------------------*/
/*--
   The following is an implementation of
   an elegant 3-way quicksort for strings,
   described in a paper "Fast Algorithms for
   Sorting and Searching Strings", by Robert
   Sedgewick and Jon L. Bentley.
--*/
#[inline]
 extern "C" fn mmed3(mut a: UChar, mut b: UChar, mut c: UChar)
 -> UChar {
    let mut t: UChar = 0;
    if a as std::os::raw::c_int > b as std::os::raw::c_int { t = a; a = b; b = t }
    if b as std::os::raw::c_int > c as std::os::raw::c_int {
        b = c;
        if a as std::os::raw::c_int > b as std::os::raw::c_int { b = a }
    }
    return b;
}
unsafe extern "C" fn mainQSort3(mut ptr: *mut UInt32, mut block: *mut UChar,
                                mut quadrant: *mut UInt16, mut nblock: Int32,
                                mut loSt: Int32, mut hiSt: Int32,
                                mut dSt: Int32, mut budget: *mut Int32) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut med: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut d: Int32 = 0;
    let mut stackLo: [Int32; 100] = [0; 100];
    let mut stackHi: [Int32; 100] = [0; 100];
    let mut stackD: [Int32; 100] = [0; 100];
    let mut nextLo: [Int32; 3] = [0; 3];
    let mut nextHi: [Int32; 3] = [0; 3];
    let mut nextD: [Int32; 3] = [0; 3];
    sp = 0 as std::os::raw::c_int;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    stackD[sp as usize] = dSt;
    sp += 1;
    while sp > 0 as std::os::raw::c_int {
        if !(sp < 100 as std::os::raw::c_int - 2 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(1001 as std::os::raw::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        d = stackD[sp as usize];
        if hi - lo < 20 as std::os::raw::c_int ||
               d > 2 as std::os::raw::c_int + 12 as std::os::raw::c_int {
            mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 as std::os::raw::c_int { return }
        } else {
            med =
                mmed3(*block.offset((*ptr.offset(lo as
                                                     isize)).wrapping_add(d as
                                                                              std::os::raw::c_uint)
                                        as isize),
                      *block.offset((*ptr.offset(hi as
                                                     isize)).wrapping_add(d as
                                                                              std::os::raw::c_uint)
                                        as isize),
                      *block.offset((*ptr.offset((lo + hi >> 1 as std::os::raw::c_int)
                                                     as
                                                     isize)).wrapping_add(d as
                                                                              std::os::raw::c_uint)
                                        as isize)) as Int32;
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            while 1 as std::os::raw::c_int as Bool != 0 {
                while 1 as std::os::raw::c_int as Bool != 0 {
                    if unLo > unHi { break ; }
                    n =
                        *block.offset((*ptr.offset(unLo as
                                                       isize)).wrapping_add(d
                                                                                as
                                                                                std::os::raw::c_uint)
                                          as isize) as Int32 - med;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp: Int32 =
                            *ptr.offset(unLo as isize) as Int32;
                        *ptr.offset(unLo as isize) =
                            *ptr.offset(ltLo as isize);
                        *ptr.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1
                    } else { if n > 0 as std::os::raw::c_int { break ; } unLo += 1 }
                }
                while 1 as std::os::raw::c_int as Bool != 0 {
                    if unLo > unHi { break ; }
                    n =
                        *block.offset((*ptr.offset(unHi as
                                                       isize)).wrapping_add(d
                                                                                as
                                                                                std::os::raw::c_uint)
                                          as isize) as Int32 - med;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp_0: Int32 =
                            *ptr.offset(unHi as isize) as Int32;
                        *ptr.offset(unHi as isize) =
                            *ptr.offset(gtHi as isize);
                        *ptr.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1
                    } else { if n < 0 as std::os::raw::c_int { break ; } unHi -= 1 }
                }
                if unLo > unHi { break ; }
                let mut zztmp_1: Int32 = *ptr.offset(unLo as isize) as Int32;
                *ptr.offset(unLo as isize) = *ptr.offset(unHi as isize);
                *ptr.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1
            }
            if gtHi < ltLo {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = hi;
                stackD[sp as usize] = d + 1 as std::os::raw::c_int;
                sp += 1
            } else {
                n =
                    if ltLo - lo < unLo - ltLo {
                        (ltLo) - lo
                    } else { (unLo) - ltLo };
                let mut yyp1: Int32 = lo;
                let mut yyp2: Int32 = unLo - n;
                let mut yyn: Int32 = n;
                while yyn > 0 as std::os::raw::c_int {
                    let mut zztmp_2: Int32 =
                        *ptr.offset(yyp1 as isize) as Int32;
                    *ptr.offset(yyp1 as isize) = *ptr.offset(yyp2 as isize);
                    *ptr.offset(yyp2 as isize) = zztmp_2 as UInt32;
                    yyp1 += 1;
                    yyp2 += 1;
                    yyn -= 1
                }
                m =
                    if hi - gtHi < gtHi - unHi {
                        (hi) - gtHi
                    } else { (gtHi) - unHi };
                let mut yyp1_0: Int32 = unLo;
                let mut yyp2_0: Int32 = hi - m + 1 as std::os::raw::c_int;
                let mut yyn_0: Int32 = m;
                while yyn_0 > 0 as std::os::raw::c_int {
                    let mut zztmp_3: Int32 =
                        *ptr.offset(yyp1_0 as isize) as Int32;
                    *ptr.offset(yyp1_0 as isize) =
                        *ptr.offset(yyp2_0 as isize);
                    *ptr.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                    yyp1_0 += 1;
                    yyp2_0 += 1;
                    yyn_0 -= 1
                }
                n = lo + unLo - ltLo - 1 as std::os::raw::c_int;
                m = hi - (gtHi - unHi) + 1 as std::os::raw::c_int;
                nextLo[0 as std::os::raw::c_int as usize] = lo;
                nextHi[0 as std::os::raw::c_int as usize] = n;
                nextD[0 as std::os::raw::c_int as usize] = d;
                nextLo[1 as std::os::raw::c_int as usize] = m;
                nextHi[1 as std::os::raw::c_int as usize] = hi;
                nextD[1 as std::os::raw::c_int as usize] = d;
                nextLo[2 as std::os::raw::c_int as usize] = n + 1 as std::os::raw::c_int;
                nextHi[2 as std::os::raw::c_int as usize] = m - 1 as std::os::raw::c_int;
                nextD[2 as std::os::raw::c_int as usize] = d + 1 as std::os::raw::c_int;
                if nextHi[0 as std::os::raw::c_int as usize] -
                       nextLo[0 as std::os::raw::c_int as usize] <
                       nextHi[1 as std::os::raw::c_int as usize] -
                           nextLo[1 as std::os::raw::c_int as usize] {
                    let mut tz: Int32 = 0;
                    tz = nextLo[0 as std::os::raw::c_int as usize];
                    nextLo[0 as std::os::raw::c_int as usize] =
                        nextLo[1 as std::os::raw::c_int as usize];
                    nextLo[1 as std::os::raw::c_int as usize] = tz;
                    tz = nextHi[0 as std::os::raw::c_int as usize];
                    nextHi[0 as std::os::raw::c_int as usize] =
                        nextHi[1 as std::os::raw::c_int as usize];
                    nextHi[1 as std::os::raw::c_int as usize] = tz;
                    tz = nextD[0 as std::os::raw::c_int as usize];
                    nextD[0 as std::os::raw::c_int as usize] =
                        nextD[1 as std::os::raw::c_int as usize];
                    nextD[1 as std::os::raw::c_int as usize] = tz
                }
                if nextHi[1 as std::os::raw::c_int as usize] -
                       nextLo[1 as std::os::raw::c_int as usize] <
                       nextHi[2 as std::os::raw::c_int as usize] -
                           nextLo[2 as std::os::raw::c_int as usize] {
                    let mut tz_0: Int32 = 0;
                    tz_0 = nextLo[1 as std::os::raw::c_int as usize];
                    nextLo[1 as std::os::raw::c_int as usize] =
                        nextLo[2 as std::os::raw::c_int as usize];
                    nextLo[2 as std::os::raw::c_int as usize] = tz_0;
                    tz_0 = nextHi[1 as std::os::raw::c_int as usize];
                    nextHi[1 as std::os::raw::c_int as usize] =
                        nextHi[2 as std::os::raw::c_int as usize];
                    nextHi[2 as std::os::raw::c_int as usize] = tz_0;
                    tz_0 = nextD[1 as std::os::raw::c_int as usize];
                    nextD[1 as std::os::raw::c_int as usize] =
                        nextD[2 as std::os::raw::c_int as usize];
                    nextD[2 as std::os::raw::c_int as usize] = tz_0
                }
                if nextHi[0 as std::os::raw::c_int as usize] -
                       nextLo[0 as std::os::raw::c_int as usize] <
                       nextHi[1 as std::os::raw::c_int as usize] -
                           nextLo[1 as std::os::raw::c_int as usize] {
                    let mut tz_1: Int32 = 0;
                    tz_1 = nextLo[0 as std::os::raw::c_int as usize];
                    nextLo[0 as std::os::raw::c_int as usize] =
                        nextLo[1 as std::os::raw::c_int as usize];
                    nextLo[1 as std::os::raw::c_int as usize] = tz_1;
                    tz_1 = nextHi[0 as std::os::raw::c_int as usize];
                    nextHi[0 as std::os::raw::c_int as usize] =
                        nextHi[1 as std::os::raw::c_int as usize];
                    nextHi[1 as std::os::raw::c_int as usize] = tz_1;
                    tz_1 = nextD[0 as std::os::raw::c_int as usize];
                    nextD[0 as std::os::raw::c_int as usize] =
                        nextD[1 as std::os::raw::c_int as usize];
                    nextD[1 as std::os::raw::c_int as usize] = tz_1
                }
                stackLo[sp as usize] = nextLo[0 as std::os::raw::c_int as usize];
                stackHi[sp as usize] = nextHi[0 as std::os::raw::c_int as usize];
                stackD[sp as usize] = nextD[0 as std::os::raw::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[1 as std::os::raw::c_int as usize];
                stackHi[sp as usize] = nextHi[1 as std::os::raw::c_int as usize];
                stackD[sp as usize] = nextD[1 as std::os::raw::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[2 as std::os::raw::c_int as usize];
                stackHi[sp as usize] = nextHi[2 as std::os::raw::c_int as usize];
                stackD[sp as usize] = nextD[2 as std::os::raw::c_int as usize];
                sp += 1
            }
        }
    };
}
unsafe extern "C" fn mainSort(mut ptr: *mut UInt32, mut block: *mut UChar,
                              mut quadrant: *mut UInt16,
                              mut ftab: *mut UInt32, mut nblock: Int32,
                              mut verb: Int32, mut budget: *mut Int32) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut ss: Int32 = 0;
    let mut sb: Int32 = 0;
    let mut runningOrder: [Int32; 256] = [0; 256];
    let mut bigDone: [Bool; 256] = [0; 256];
    let mut copyStart: [Int32; 256] = [0; 256];
    let mut copyEnd: [Int32; 256] = [0; 256];
    let mut c1: UChar = 0;
    let mut numQSorted: Int32 = 0;
    let mut s: UInt16 = 0;
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        main sort initialise ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    /*-- set up the 2-byte frequency table --*/
    i = 65536 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        *ftab.offset(i as isize) = 0 as std::os::raw::c_int as UInt32;
        i -= 1
    }
    j =
        (*block.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) <<
            8 as std::os::raw::c_int;
    i = nblock - 1 as std::os::raw::c_int;
    while i >= 3 as std::os::raw::c_int {
        *quadrant.offset(i as isize) = 0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset(i as isize) as UInt16 as std::os::raw::c_int) <<
                    8 as std::os::raw::c_int;
        let ref mut fresh4 = *ftab.offset(j as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *quadrant.offset((i - 1 as std::os::raw::c_int) as isize) =
            0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset((i - 1 as std::os::raw::c_int) as isize) as UInt16 as
                     std::os::raw::c_int) << 8 as std::os::raw::c_int;
        let ref mut fresh5 = *ftab.offset(j as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        *quadrant.offset((i - 2 as std::os::raw::c_int) as isize) =
            0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset((i - 2 as std::os::raw::c_int) as isize) as UInt16 as
                     std::os::raw::c_int) << 8 as std::os::raw::c_int;
        let ref mut fresh6 = *ftab.offset(j as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        *quadrant.offset((i - 3 as std::os::raw::c_int) as isize) =
            0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset((i - 3 as std::os::raw::c_int) as isize) as UInt16 as
                     std::os::raw::c_int) << 8 as std::os::raw::c_int;
        let ref mut fresh7 = *ftab.offset(j as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
        i -= 4 as std::os::raw::c_int
    }
    while i >= 0 as std::os::raw::c_int {
        *quadrant.offset(i as isize) = 0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset(i as isize) as UInt16 as std::os::raw::c_int) <<
                    8 as std::os::raw::c_int;
        let ref mut fresh8 = *ftab.offset(j as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
        i -= 1
    }
    /*-- (emphasises close relationship of block & quadrant) --*/
    i = 0 as std::os::raw::c_int;
    while i <
              2 as std::os::raw::c_int + 12 as std::os::raw::c_int + 18 as std::os::raw::c_int +
                  2 as std::os::raw::c_int {
        *block.offset((nblock + i) as isize) = *block.offset(i as isize);
        *quadrant.offset((nblock + i) as isize) = 0 as std::os::raw::c_int as UInt16;
        i += 1
    }
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        bucket sorting ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    /*-- Complete the initial radix sort --*/
    i = 1 as std::os::raw::c_int;
    while i <= 65536 as std::os::raw::c_int {
        let ref mut fresh9 = *ftab.offset(i as isize);
        *fresh9 =
            (*fresh9 as
                 std::os::raw::c_uint).wrapping_add(*ftab.offset((i -
                                                              1 as
                                                                  std::os::raw::c_int)
                                                             as isize)) as
                UInt32 as UInt32;
        i += 1
    }
    s =
        ((*block.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) <<
             8 as std::os::raw::c_int) as UInt16;
    i = nblock - 1 as std::os::raw::c_int;
    while i >= 3 as std::os::raw::c_int {
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset(i as isize) as std::os::raw::c_int) <<
                     8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset((i - 1 as std::os::raw::c_int) as isize) as
                      std::os::raw::c_int) << 8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 1 as std::os::raw::c_int) as UInt32;
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset((i - 2 as std::os::raw::c_int) as isize) as
                      std::os::raw::c_int) << 8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 2 as std::os::raw::c_int) as UInt32;
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset((i - 3 as std::os::raw::c_int) as isize) as
                      std::os::raw::c_int) << 8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 3 as std::os::raw::c_int) as UInt32;
        i -= 4 as std::os::raw::c_int
    }
    while i >= 0 as std::os::raw::c_int {
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset(i as isize) as std::os::raw::c_int) <<
                     8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        i -= 1
    }
    /*--
      Now ftab contains the first loc of every small bucket.
      Calculate the running order, from smallest to largest
      big bucket.
   --*/
    i = 0 as std::os::raw::c_int;
    while i <= 255 as std::os::raw::c_int {
        bigDone[i as usize] = 0 as std::os::raw::c_int as Bool;
        runningOrder[i as usize] = i;
        i += 1
    }
    let mut vv: Int32 = 0;
    let mut h: Int32 = 1 as std::os::raw::c_int;
    loop  {
        h = 3 as std::os::raw::c_int * h + 1 as std::os::raw::c_int;
        if !(h <= 256 as std::os::raw::c_int) { break ; }
    }
    loop  {
        h = h / 3 as std::os::raw::c_int;
        i = h;
        while i <= 255 as std::os::raw::c_int {
            vv = runningOrder[i as usize];
            j = i;
            while (*ftab.offset(((runningOrder[(j - h) as usize] +
                                      1 as std::os::raw::c_int) << 8 as std::os::raw::c_int)
                                    as
                                    isize)).wrapping_sub(*ftab.offset((runningOrder[(j
                                                                                         -
                                                                                         h)
                                                                                        as
                                                                                        usize]
                                                                           <<
                                                                           8
                                                                               as
                                                                               std::os::raw::c_int)
                                                                          as
                                                                          isize))
                      >
                      (*ftab.offset(((vv + 1 as std::os::raw::c_int) <<
                                         8 as std::os::raw::c_int) as
                                        isize)).wrapping_sub(*ftab.offset((vv
                                                                               <<
                                                                               8
                                                                                   as
                                                                                   std::os::raw::c_int)
                                                                              as
                                                                              isize))
                  {
                runningOrder[j as usize] = runningOrder[(j - h) as usize];
                j = j - h;
                if j <= h - 1 as std::os::raw::c_int { break ; }
            }
            runningOrder[j as usize] = vv;
            i += 1
        }
        if !(h != 1 as std::os::raw::c_int) { break ; }
    }
    /*--
      The main sorting loop.
   --*/
    numQSorted = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i <= 255 as std::os::raw::c_int {
        /*--
         Process big buckets, starting with the least full.
         Basically this is a 3-step process in which we call
         mainQSort3 to sort the small buckets [ss, j], but
         also make a big effort to avoid the calls if we can.
      --*/
        ss = runningOrder[i as usize];
        /*--
         Step 1:
         Complete the big bucket [ss] by quicksorting
         any unsorted small buckets [ss, j], for j != ss.  
         Hopefully previous pointer-scanning phases have already
         completed many of the small buckets [ss, j], so
         we don't have to sort them at all.
      --*/
        j = 0 as std::os::raw::c_int;
        while j <= 255 as std::os::raw::c_int {
            if j != ss {
                sb = (ss << 8 as std::os::raw::c_int) + j;
                if *ftab.offset(sb as isize) &
                       ((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                           std::os::raw::c_uint == 0 {
                    let mut lo: Int32 =
                        (*ftab.offset(sb as isize) &
                             !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                                 std::os::raw::c_uint) as Int32;
                    let mut hi: Int32 =
                        (*ftab.offset((sb + 1 as std::os::raw::c_int) as isize) &
                             !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                                 std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_uint)
                            as Int32;
                    if hi > lo {
                        if verb >= 4 as std::os::raw::c_int {
                            fprintf(__stderrp,
                                    b"        qsort [0x%x, 0x%x]   done %d   this %d\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    ss, j, numQSorted,
                                    hi - lo + 1 as std::os::raw::c_int);
                        }
                        mainQSort3(ptr, block, quadrant, nblock, lo, hi,
                                   2 as std::os::raw::c_int, budget);
                        numQSorted += hi - lo + 1 as std::os::raw::c_int;
                        if *budget < 0 as std::os::raw::c_int { return }
                    }
                }
                let ref mut fresh10 = *ftab.offset(sb as isize);
                *fresh10 |=
                    ((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as std::os::raw::c_uint
            }
            j += 1
        }
        if bigDone[ss as usize] != 0 {
            BZ2_bz__AssertH__fail(1006 as std::os::raw::c_int);
        }
        /*--
         Step 2:
         Now scan this big bucket [ss] so as to synthesise the
         sorted order for small buckets [t, ss] for all t,
         including, magically, the bucket [ss,ss] too.
         This will avoid doing Real Work in subsequent Step 1's.
      --*/
        j = 0 as std::os::raw::c_int;
        while j <= 255 as std::os::raw::c_int {
            copyStart[j as usize] =
                (*ftab.offset(((j << 8 as std::os::raw::c_int) + ss) as isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint) as Int32;
            copyEnd[j as usize] =
                (*ftab.offset(((j << 8 as std::os::raw::c_int) + ss +
                                   1 as std::os::raw::c_int) as isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_uint) as
                    Int32;
            j += 1
        }
        j =
            (*ftab.offset((ss << 8 as std::os::raw::c_int) as isize) &
                 !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as std::os::raw::c_uint)
                as Int32;
        while j < copyStart[ss as usize] {
            k =
                (*ptr.offset(j as
                                 isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_uint) as
                    Int32;
            if k < 0 as std::os::raw::c_int { k += nblock }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh11 = copyStart[c1 as usize];
                copyStart[c1 as usize] = copyStart[c1 as usize] + 1;
                *ptr.offset(fresh11 as isize) = k as UInt32
            }
            j += 1
        }
        j =
            (*ftab.offset(((ss + 1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) as
                              isize) &
                 !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                     std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int as
                                                    std::os::raw::c_uint) as Int32;
        while j > copyEnd[ss as usize] {
            k =
                (*ptr.offset(j as
                                 isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_uint) as
                    Int32;
            if k < 0 as std::os::raw::c_int { k += nblock }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh12 = copyEnd[c1 as usize];
                copyEnd[c1 as usize] = copyEnd[c1 as usize] - 1;
                *ptr.offset(fresh12 as isize) = k as UInt32
            }
            j -= 1
        }
        if !(copyStart[ss as usize] - 1 as std::os::raw::c_int == copyEnd[ss as usize]
                 ||
                 copyStart[ss as usize] == 0 as std::os::raw::c_int &&
                     copyEnd[ss as usize] == nblock - 1 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(1007 as std::os::raw::c_int);
        }
        j = 0 as std::os::raw::c_int;
        while j <= 255 as std::os::raw::c_int {
            let ref mut fresh13 =
                *ftab.offset(((j << 8 as std::os::raw::c_int) + ss) as isize);
            *fresh13 |=
                ((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as std::os::raw::c_uint;
            j += 1
        }
        /*--
         Step 3:
         The [ss] big bucket is now done.  Record this fact,
         and update the quadrant descriptors.  Remember to
         update quadrants in the overshoot area too, if
         necessary.  The "if (i < 255)" test merely skips
         this updating for the last bucket processed, since
         updating for the last bucket is pointless.

         The quadrant array provides a way to incrementally
         cache sort orderings, as they appear, so as to 
         make subsequent comparisons in fullGtU() complete
         faster.  For repetitive blocks this makes a big
         difference (but not big enough to be able to avoid
         the fallback sorting mechanism, exponential radix sort).

         The precise meaning is: at all times:

            for 0 <= i < nblock and 0 <= j <= nblock

            if block[i] != block[j], 

               then the relative values of quadrant[i] and 
                    quadrant[j] are meaningless.

               else {
                  if quadrant[i] < quadrant[j]
                     then the string starting at i lexicographically
                     precedes the string starting at j

                  else if quadrant[i] > quadrant[j]
                     then the string starting at j lexicographically
                     precedes the string starting at i

                  else
                     the relative ordering of the strings starting
                     at i and j has not yet been determined.
               }
      --*/
        bigDone[ss as usize] = 1 as std::os::raw::c_int as Bool;
        if i < 255 as std::os::raw::c_int {
            let mut bbStart: Int32 =
                (*ftab.offset((ss << 8 as std::os::raw::c_int) as isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint) as Int32;
            let mut bbSize: Int32 =
                (*ftab.offset(((ss + 1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) as
                                  isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint).wrapping_sub(bbStart as std::os::raw::c_uint)
                    as Int32;
            let mut shifts: Int32 = 0 as std::os::raw::c_int;
            while bbSize >> shifts > 65534 as std::os::raw::c_int { shifts += 1 }
            j = bbSize - 1 as std::os::raw::c_int;
            while j >= 0 as std::os::raw::c_int {
                let mut a2update: Int32 =
                    *ptr.offset((bbStart + j) as isize) as Int32;
                let mut qVal: UInt16 = (j >> shifts) as UInt16;
                *quadrant.offset(a2update as isize) = qVal;
                if a2update <
                       2 as std::os::raw::c_int + 12 as std::os::raw::c_int +
                           18 as std::os::raw::c_int + 2 as std::os::raw::c_int {
                    *quadrant.offset((a2update + nblock) as isize) = qVal
                }
                j -= 1
            }
            if !(bbSize - 1 as std::os::raw::c_int >> shifts <= 65535 as std::os::raw::c_int)
               {
                BZ2_bz__AssertH__fail(1002 as std::os::raw::c_int);
            }
        }
        i += 1
    }
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        %d pointers, %d sorted, %d scanned\n\x00" as
                    *const u8 as *const std::os::raw::c_char, nblock, numQSorted,
                nblock - numQSorted);
    };
}
/*---------------------------------------------*/
/* Pre:
      nblock > 0
      arr2 exists for [0 .. nblock-1 +N_OVERSHOOT]
      ((UChar*)arr2)  [0 .. nblock-1] holds block
      arr1 exists for [0 .. nblock-1]

   Post:
      ((UChar*)arr2) [0 .. nblock-1] holds block
      All other areas of block destroyed
      ftab [ 0 .. 65536 ] destroyed
      arr1 [0 .. nblock-1] holds sorted order
*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_blockSort(mut s: *mut EState) {
    let mut ptr: *mut UInt32 = (*s).ptr;
    let mut block: *mut UChar = (*s).block;
    let mut ftab: *mut UInt32 = (*s).ftab;
    let mut nblock: Int32 = (*s).nblock;
    let mut verb: Int32 = (*s).verbosity;
    let mut wfact: Int32 = (*s).workFactor;
    let mut quadrant: *mut UInt16 = 0 as *mut UInt16;
    let mut budget: Int32 = 0;
    let mut budgetInit: Int32 = 0;
    let mut i: Int32 = 0;
    if nblock < 10000 as std::os::raw::c_int {
        fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
    } else {
        /* Calculate the location for quadrant, remembering to get
         the alignment right.  Assumes that &(block[0]) is at least
         2-byte aligned -- this should be ok since block is really
         the first section of arr2.
      */
        i =
            nblock +
                (2 as std::os::raw::c_int + 12 as std::os::raw::c_int + 18 as std::os::raw::c_int +
                     2 as std::os::raw::c_int);
        if i & 1 as std::os::raw::c_int != 0 { i += 1 }
        quadrant =
            &mut *block.offset(i as isize) as *mut UChar as *mut UInt16;
        /* (wfact-1) / 3 puts the default-factor-30
         transition point at very roughly the same place as 
         with v0.1 and v0.9.0.  
         Not that it particularly matters any more, since the
         resulting compressed stream is now the same regardless
         of whether or not we use the main sort or fallback sort.
      */
        if wfact < 1 as std::os::raw::c_int { wfact = 1 as std::os::raw::c_int }
        if wfact > 100 as std::os::raw::c_int { wfact = 100 as std::os::raw::c_int }
        budgetInit = nblock * ((wfact - 1 as std::os::raw::c_int) / 3 as std::os::raw::c_int);
        budget = budgetInit;
        mainSort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);
        if verb >= 3 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"      %d work, %d block, ratio %5.2f\n\x00" as *const u8
                        as *const std::os::raw::c_char, budgetInit - budget, nblock,
                    ((budgetInit - budget) as std::os::raw::c_float /
                         (if nblock == 0 as std::os::raw::c_int {
                              1 as std::os::raw::c_int
                          } else { nblock }) as std::os::raw::c_float) as
                        std::os::raw::c_double);
        }
        if budget < 0 as std::os::raw::c_int {
            if verb >= 2 as std::os::raw::c_int {
                fprintf(__stderrp,
                        b"    too repetitive; using fallback sorting algorithm\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
            }
            fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
        }
    }
    (*s).origPtr = -(1 as std::os::raw::c_int);
    i = 0 as std::os::raw::c_int;
    while i < (*s).nblock {
        if *ptr.offset(i as isize) == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*s).origPtr = i;
            break ;
        } else { i += 1 }
    }
    if !((*s).origPtr != -(1 as std::os::raw::c_int)) {
        BZ2_bz__AssertH__fail(1003 as std::os::raw::c_int);
    };
}
/*-------------------------------------------------------------*/
/*--- end                                       blocksort.c ---*/
/*-------------------------------------------------------------*/

}

pub mod bzlib {

extern "C" {
    
    #[no_mangle]
    static mut __stdinp: *mut FILE;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn ferror(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgetc(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn ungetc(_: std::os::raw::c_int, _: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn fdopen(_: std::os::raw::c_int, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    
    #[no_mangle]
    static mut BZ2_crc32Table: [UInt32; 256];
    
    
    #[no_mangle]
    static mut BZ2_rNums: [Int32; 512];
}
pub use crate::compress::BZ2_compressBlock;
pub use crate::decompress::BZ2_decompress;
pub use crate::blocksort::__sFILEX;
pub type __uint32_t = crate::bzip2::__uint32_t;
pub type __int64_t = crate::blocksort::__int64_t;
pub type __darwin_ct_rune_t = crate::bzip2::__darwin_ct_rune_t;
pub type __darwin_size_t = crate::bzip2::__darwin_size_t;
pub type __darwin_wchar_t = crate::bzip2::__darwin_wchar_t;
pub type __darwin_rune_t = crate::bzip2::__darwin_rune_t;
pub type __darwin_off_t = crate::blocksort::__darwin_off_t;
pub type fpos_t = crate::blocksort::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::blocksort::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::blocksort::__sFILE;
pub type FILE = crate::blocksort::FILE;
// #[derive(Copy, Clone)]

pub type _RuneEntry = crate::bzip2::_RuneEntry;
// #[derive(Copy, Clone)]

pub type _RuneRange = crate::bzip2::_RuneRange;
// #[derive(Copy, Clone)]

pub type _RuneCharClass = crate::bzip2::_RuneCharClass;
// #[derive(Copy, Clone)]

pub type _RuneLocale = crate::bzip2::_RuneLocale;
// #[derive(Copy, Clone)]

pub type bz_stream = crate::blocksort::bz_stream;
// #[derive(Copy, Clone)]

pub type EState = crate::blocksort::EState;
pub type UInt32 = crate::blocksort::UInt32;
pub type Int32 = crate::blocksort::Int32;
pub type UChar = crate::blocksort::UChar;
pub type Bool = crate::blocksort::Bool;
pub type UInt16 = crate::blocksort::UInt16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DState {
    pub strm: *mut bz_stream,
    pub state: Int32,
    pub state_out_ch: UChar,
    pub state_out_len: Int32,
    pub blockRandomised: Bool,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockSize100k: Int32,
    pub smallDecompress: Bool,
    pub currBlockNo: Int32,
    pub verbosity: Int32,
    pub origPtr: Int32,
    pub tPos: UInt32,
    pub k0: Int32,
    pub unzftab: [Int32; 256],
    pub nblock_used: Int32,
    pub cftab: [Int32; 257],
    pub cftabCopy: [Int32; 257],
    pub tt: *mut UInt32,
    pub ll16: *mut UInt16,
    pub ll4: *mut UChar,
    pub storedBlockCRC: UInt32,
    pub storedCombinedCRC: UInt32,
    pub calculatedBlockCRC: UInt32,
    pub calculatedCombinedCRC: UInt32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub inUse16: [Bool; 16],
    pub seqToUnseq: [UChar; 256],
    pub mtfa: [UChar; 4096],
    pub mtfbase: [Int32; 16],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub limit: [[Int32; 258]; 6],
    pub base: [[Int32; 258]; 6],
    pub perm: [[Int32; 258]; 6],
    pub minLens: [Int32; 6],
    pub save_i: Int32,
    pub save_j: Int32,
    pub save_t: Int32,
    pub save_alphaSize: Int32,
    pub save_nGroups: Int32,
    pub save_nSelectors: Int32,
    pub save_EOB: Int32,
    pub save_groupNo: Int32,
    pub save_groupPos: Int32,
    pub save_nextSym: Int32,
    pub save_nblockMAX: Int32,
    pub save_nblock: Int32,
    pub save_es: Int32,
    pub save_N: Int32,
    pub save_curr: Int32,
    pub save_zt: Int32,
    pub save_zn: Int32,
    pub save_zvec: Int32,
    pub save_zj: Int32,
    pub save_gSel: Int32,
    pub save_gMinlen: Int32,
    pub save_gLimit: *mut Int32,
    pub save_gBase: *mut Int32,
    pub save_gPerm: *mut Int32,
}
pub type BZFILE = crate::bzip2::BZFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bzFile {
    pub handle: *mut FILE,
    pub buf: [Char; 5000],
    pub bufN: Int32,
    pub writing: Bool,
    pub strm: bz_stream,
    pub lastErr: Int32,
    pub initialisedOk: Bool,
}
pub type Char = crate::bzip2::Char;
#[inline]
unsafe extern "C" fn __isctype(mut _c: __darwin_ct_rune_t,
                               mut _f: std::os::raw::c_ulong) -> __darwin_ct_rune_t {
    return if _c < 0 as std::os::raw::c_int ||
                  _c >= (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int {
               0 as std::os::raw::c_int
           } else {
               (_DefaultRuneLocale.__runetype[_c as usize] as std::os::raw::c_ulong &
                    _f != 0) as std::os::raw::c_int
           };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __isctype(_c, 0x400 as std::os::raw::c_long as std::os::raw::c_ulong);
}
/*-------------------------------------------------------------*/
/*--- Library top-level functions.                          ---*/
/*---                                               bzlib.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/* CHANGES
   0.9.0    -- original version.
   0.9.0a/b -- no changes in this file.
   0.9.0c   -- made zero-length BZ_FLUSH work correctly in bzCompress().
     fixed bzWrite/bzRead to ignore zero-length requests.
     fixed bzread to correctly handle read requests after EOF.
     wrong parameter order in call to bzDecompressInit in
     bzBuffToBuffDecompress.  Fixed.
*/
/*---------------------------------------------------*/
/*--- Compression stuff                           ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bz__AssertH__fail(mut errcode: std::os::raw::c_int) {
    fprintf(__stderrp,
            b"\n\nbzip2/libbzip2: internal error number %d.\nThis is a bug in bzip2/libbzip2, %s.\nPlease report it to: bzip2-devel@sourceware.org.  If this happened\nwhen you were using some program which uses libbzip2 as a\ncomponent, you should also report this bug to the author(s)\nof that program.  Please make an effort to report this bug;\ntimely and accurate bug reports eventually lead to higher\nquality software.  Thanks.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char, errcode,
            BZ2_bzlibVersion());
    if errcode == 1007 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"\n*** A special note about internal error number 1007 ***\n\nExperience suggests that a common cause of i.e. 1007\nis unreliable memory or other hardware.  The 1007 assertion\njust happens to cross-check the results of huge numbers of\nmemory reads/writes, and so acts (unintendedly) as a stress\ntest of your memory system.\n\nI suggest the following: try compressing the file again,\npossibly monitoring progress in detail with the -vv flag.\n\n* If the error cannot be reproduced, and/or happens at different\n  points in compression, you may have a flaky memory system.\n  Try a memory-test program.  I have used Memtest86\n  (www.memtest86.com).  At the time of writing it is free (GPLd).\n  Memtest86 tests memory much more thorougly than your BIOSs\n  power-on test, and may find failures that the BIOS doesn\'t.\n\n* If the error can be repeatably reproduced, this is a bug in\n  bzip2, and I would very much like to hear about it.  Please\n  let me know, and, ideally, save a copy of the file causing the\n  problem -- without which I will be unable to investigate it.\n\n\x00"
                    as *const u8 as *const std::os::raw::c_char);
    }
    exit(3 as std::os::raw::c_int);
}
/*---------------------------------------------------*/
 extern "C" fn bz_config_ok() -> std::os::raw::c_int {
    if ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong !=
           4 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    if ::std::mem::size_of::<std::os::raw::c_short>() as std::os::raw::c_ulong !=
           2 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
unsafe extern "C" fn default_bzalloc(mut opaque: *mut std::os::raw::c_void,
                                     mut items: Int32, mut size: Int32)
 -> *mut std::os::raw::c_void {
    let mut v: *mut std::os::raw::c_void = malloc((items * size) as std::os::raw::c_ulong);
    return v;
}
unsafe extern "C" fn default_bzfree(mut opaque: *mut std::os::raw::c_void,
                                    mut addr: *mut std::os::raw::c_void) {
    if !addr.is_null() { free(addr); };
}
/*---------------------------------------------------*/
unsafe extern "C" fn prepare_new_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nblock = 0 as std::os::raw::c_int;
    (*s).numZ = 0 as std::os::raw::c_int;
    (*s).state_out_pos = 0 as std::os::raw::c_int;
    (*s).blockCRC = 0xffffffff as std::os::raw::c_long as UInt32;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        (*s).inUse[i as usize] = 0 as std::os::raw::c_int as Bool;
        i += 1
    }
    (*s).blockNo += 1;
}
/*---------------------------------------------------*/
unsafe extern "C" fn init_RL(mut s: *mut EState) {
    (*s).state_in_ch = 256 as std::os::raw::c_int as UInt32;
    (*s).state_in_len = 0 as std::os::raw::c_int;
}
unsafe extern "C" fn isempty_RL(mut s: *mut EState) -> Bool {
    if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*s).state_in_len > 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as Bool
    } else { return 1 as std::os::raw::c_int as Bool };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressInit(mut strm: *mut bz_stream,
                                            mut blockSize100k: std::os::raw::c_int,
                                            mut verbosity: std::os::raw::c_int,
                                            mut workFactor: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut n: Int32 = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if bz_config_ok() == 0 { return -(9 as std::os::raw::c_int) }
    if strm.is_null() || blockSize100k < 1 as std::os::raw::c_int ||
           blockSize100k > 9 as std::os::raw::c_int || workFactor < 0 as std::os::raw::c_int
           || workFactor > 250 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if workFactor == 0 as std::os::raw::c_int { workFactor = 30 as std::os::raw::c_int }
    if (*strm).bzalloc.is_none() {
        (*strm).bzalloc =
            Some(default_bzalloc as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: Int32,
                                          _: Int32) -> *mut std::os::raw::c_void)
    }
    if (*strm).bzfree.is_none() {
        (*strm).bzfree =
            Some(default_bzfree as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *mut std::os::raw::c_void) -> ())
    }
    s =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            ::std::mem::size_of::<EState>()
                                                                as
                                                                std::os::raw::c_ulong
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut EState;
    if s.is_null() { return -(3 as std::os::raw::c_int) }
    (*s).strm = strm;
    (*s).arr1 = 0 as *mut UInt32;
    (*s).arr2 = 0 as *mut UInt32;
    (*s).ftab = 0 as *mut UInt32;
    n = 100000 as std::os::raw::c_int * blockSize100k;
    (*s).arr1 =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            (n as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt32>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong)
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut UInt32;
    (*s).arr2 =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            ((n +
                                                                  (2 as
                                                                       std::os::raw::c_int
                                                                       +
                                                                       12 as
                                                                           std::os::raw::c_int
                                                                       +
                                                                       18 as
                                                                           std::os::raw::c_int
                                                                       +
                                                                       2 as
                                                                           std::os::raw::c_int))
                                                                 as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt32>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong)
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut UInt32;
    (*s).ftab =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            (65537 as
                                                                 std::os::raw::c_int
                                                                 as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt32>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong)
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut UInt32;
    if (*s).arr1.is_null() || (*s).arr2.is_null() || (*s).ftab.is_null() {
        if !(*s).arr1.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               (*s).arr1 as
                                                                   *mut std::os::raw::c_void);
        }
        if !(*s).arr2.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               (*s).arr2 as
                                                                   *mut std::os::raw::c_void);
        }
        if !(*s).ftab.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               (*s).ftab as
                                                                   *mut std::os::raw::c_void);
        }
        if !s.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               s as
                                                                   *mut std::os::raw::c_void);
        }
        return -(3 as std::os::raw::c_int)
    }
    (*s).blockNo = 0 as std::os::raw::c_int;
    (*s).state = 2 as std::os::raw::c_int;
    (*s).mode = 2 as std::os::raw::c_int;
    (*s).combinedCRC = 0 as std::os::raw::c_int as UInt32;
    (*s).blockSize100k = blockSize100k;
    (*s).nblockMAX =
        100000 as std::os::raw::c_int * blockSize100k - 19 as std::os::raw::c_int;
    (*s).verbosity = verbosity;
    (*s).workFactor = workFactor;
    (*s).block = (*s).arr2 as *mut UChar;
    (*s).mtfv = (*s).arr1 as *mut UInt16;
    (*s).zbits = 0 as *mut UChar;
    (*s).ptr = (*s).arr1;
    (*strm).state = s as *mut std::os::raw::c_void;
    (*strm).total_in_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_in_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    init_RL(s);
    prepare_new_block(s);
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
unsafe extern "C" fn add_pair_to_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    let mut ch: UChar = (*s).state_in_ch as UChar;
    i = 0 as std::os::raw::c_int;
    while i < (*s).state_in_len {
        (*s).blockCRC =
            (*s).blockCRC << 8 as std::os::raw::c_int ^
                BZ2_crc32Table[((*s).blockCRC >> 24 as std::os::raw::c_int ^
                                    ch as std::os::raw::c_uint) as usize];
        i += 1
    }
    (*s).inUse[(*s).state_in_ch as usize] = 1 as std::os::raw::c_int as Bool;
    match (*s).state_in_len {
        1 => {
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1
        }
        2 => {
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1
        }
        3 => {
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1
        }
        _ => {
            (*s).inUse[((*s).state_in_len - 4 as std::os::raw::c_int) as usize] =
                1 as std::os::raw::c_int as Bool;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) =
                ((*s).state_in_len - 4 as std::os::raw::c_int) as UChar;
            (*s).nblock += 1
        }
    };
}
/*---------------------------------------------------*/
unsafe extern "C" fn flush_RL(mut s: *mut EState) {
    if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint {
        add_pair_to_block(s);
    }
    init_RL(s);
}
/*---------------------------------------------------*/
/*-- fast track the common case --*/
/*-- general, uncommon cases --*/
/*---------------------------------------------------*/
unsafe extern "C" fn copy_input_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_in: Bool = 0 as std::os::raw::c_int as Bool;
    if (*s).mode == 2 as std::os::raw::c_int {
        /*-- fast track the common case --*/
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*-- block full? --*/
            if (*s).nblock >= (*s).nblockMAX { break ; }
            /*-- no input? --*/
            if (*(*s).strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                break ;
            }
            progress_in = 1 as std::os::raw::c_int as Bool;
            let mut zchh: UInt32 =
                *((*(*s).strm).next_in as *mut UChar) as UInt32;
            if zchh != (*s).state_in_ch &&
                   (*s).state_in_len == 1 as std::os::raw::c_int {
                let mut ch: UChar = (*s).state_in_ch as UChar;
                (*s).blockCRC =
                    (*s).blockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).blockCRC >> 24 as std::os::raw::c_int ^
                                            ch as std::os::raw::c_uint) as usize];
                (*s).inUse[(*s).state_in_ch as usize] =
                    1 as std::os::raw::c_int as Bool;
                *(*s).block.offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).state_in_ch = zchh
            } else if zchh != (*s).state_in_ch ||
                          (*s).state_in_len == 255 as std::os::raw::c_int {
                if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint {
                    add_pair_to_block(s);
                }
                (*s).state_in_ch = zchh;
                (*s).state_in_len = 1 as std::os::raw::c_int
            } else { (*s).state_in_len += 1 }
            (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
            (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
            (*(*s).strm).total_in_lo32 =
                (*(*s).strm).total_in_lo32.wrapping_add(1);
            if (*(*s).strm).total_in_lo32 == 0 as std::os::raw::c_int as std::os::raw::c_uint
               {
                (*(*s).strm).total_in_hi32 =
                    (*(*s).strm).total_in_hi32.wrapping_add(1)
            }
        }
    } else {
        /*-- general, uncommon case --*/
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*-- block full? --*/
            if (*s).nblock >= (*s).nblockMAX { break ; }
            /*-- no input? --*/
            if (*(*s).strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                break ;
            }
            /*-- flush/finish end? --*/
            if (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                break ;
            }
            progress_in = 1 as std::os::raw::c_int as Bool;
            let mut zchh_0: UInt32 =
                *((*(*s).strm).next_in as *mut UChar) as UInt32;
            if zchh_0 != (*s).state_in_ch &&
                   (*s).state_in_len == 1 as std::os::raw::c_int {
                let mut ch_0: UChar = (*s).state_in_ch as UChar;
                (*s).blockCRC =
                    (*s).blockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).blockCRC >> 24 as std::os::raw::c_int ^
                                            ch_0 as std::os::raw::c_uint) as usize];
                (*s).inUse[(*s).state_in_ch as usize] =
                    1 as std::os::raw::c_int as Bool;
                *(*s).block.offset((*s).nblock as isize) = ch_0;
                (*s).nblock += 1;
                (*s).state_in_ch = zchh_0
            } else if zchh_0 != (*s).state_in_ch ||
                          (*s).state_in_len == 255 as std::os::raw::c_int {
                if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint {
                    add_pair_to_block(s);
                }
                (*s).state_in_ch = zchh_0;
                (*s).state_in_len = 1 as std::os::raw::c_int
            } else { (*s).state_in_len += 1 }
            (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
            (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
            (*(*s).strm).total_in_lo32 =
                (*(*s).strm).total_in_lo32.wrapping_add(1);
            if (*(*s).strm).total_in_lo32 == 0 as std::os::raw::c_int as std::os::raw::c_uint
               {
                (*(*s).strm).total_in_hi32 =
                    (*(*s).strm).total_in_hi32.wrapping_add(1)
            }
            (*s).avail_in_expect = (*s).avail_in_expect.wrapping_sub(1)
        }
    }
    return progress_in;
}
/*---------------------------------------------------*/
unsafe extern "C" fn copy_output_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_out: Bool = 0 as std::os::raw::c_int as Bool;
    while 1 as std::os::raw::c_int as Bool != 0 {
        /*-- no output space? --*/
        if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            break ;
        }
        /*-- block done? --*/
        if (*s).state_out_pos >= (*s).numZ { break ; }
        progress_out = 1 as std::os::raw::c_int as Bool;
        *(*(*s).strm).next_out =
            *(*s).zbits.offset((*s).state_out_pos as isize) as std::os::raw::c_char;
        (*s).state_out_pos += 1;
        (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(1);
        (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
        (*(*s).strm).total_out_lo32 =
            (*(*s).strm).total_out_lo32.wrapping_add(1);
        if (*(*s).strm).total_out_lo32 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*(*s).strm).total_out_hi32 =
                (*(*s).strm).total_out_hi32.wrapping_add(1)
        }
    }
    return progress_out;
}
/*---------------------------------------------------*/
unsafe extern "C" fn handle_compress(mut strm: *mut bz_stream) -> Bool {
    let mut progress_in: Bool = 0 as std::os::raw::c_int as Bool;
    let mut progress_out: Bool = 0 as std::os::raw::c_int as Bool;
    let mut s: *mut EState = (*strm).state as *mut EState;
    while 1 as std::os::raw::c_int as Bool != 0 {
        if (*s).state == 1 as std::os::raw::c_int {
            progress_out =
                (progress_out as std::os::raw::c_int |
                     copy_output_until_stop(s) as std::os::raw::c_int) as Bool;
            if (*s).state_out_pos < (*s).numZ { break ; }
            if (*s).mode == 4 as std::os::raw::c_int &&
                   (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   isempty_RL(s) as std::os::raw::c_int != 0 {
                break ;
            }
            prepare_new_block(s);
            (*s).state = 2 as std::os::raw::c_int;
            if (*s).mode == 3 as std::os::raw::c_int &&
                   (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   isempty_RL(s) as std::os::raw::c_int != 0 {
                break ;
            }
        }
        if !((*s).state == 2 as std::os::raw::c_int) { continue ; }
        progress_in =
            (progress_in as std::os::raw::c_int |
                 copy_input_until_stop(s) as std::os::raw::c_int) as Bool;
        if (*s).mode != 2 as std::os::raw::c_int &&
               (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            flush_RL(s);
            BZ2_compressBlock(s,
                              ((*s).mode == 4 as std::os::raw::c_int) as std::os::raw::c_int
                                  as Bool);
            (*s).state = 1 as std::os::raw::c_int
        } else if (*s).nblock >= (*s).nblockMAX {
            BZ2_compressBlock(s, 0 as std::os::raw::c_int as Bool);
            (*s).state = 1 as std::os::raw::c_int
        } else if (*(*s).strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            break ;
        }
    }
    return (progress_in as std::os::raw::c_int != 0 ||
                progress_out as std::os::raw::c_int != 0) as std::os::raw::c_int as Bool;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompress(mut strm: *mut bz_stream,
                                        mut action: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut progress: Bool = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut EState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    loop  {
        match (*s).mode {
            1 => { return -(1 as std::os::raw::c_int) }
            2 => {
                if action == 0 as std::os::raw::c_int {
                    progress = handle_compress(strm);
                    return if progress as std::os::raw::c_int != 0 {
                               1 as std::os::raw::c_int
                           } else { -(2 as std::os::raw::c_int) }
                } else if action == 1 as std::os::raw::c_int {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 3 as std::os::raw::c_int
                } else if action == 2 as std::os::raw::c_int {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 4 as std::os::raw::c_int
                } else { return -(2 as std::os::raw::c_int) }
            }
            3 => {
                if action != 1 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as std::os::raw::c_int)
                }
                progress = handle_compress(strm);
                if (*s).avail_in_expect > 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ {
                    return 2 as std::os::raw::c_int
                }
                (*s).mode = 2 as std::os::raw::c_int;
                return 1 as std::os::raw::c_int
            }
            4 => {
                if action != 2 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as std::os::raw::c_int)
                }
                progress = handle_compress(strm);
                if progress == 0 { return -(1 as std::os::raw::c_int) }
                if (*s).avail_in_expect > 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ {
                    return 3 as std::os::raw::c_int
                }
                (*s).mode = 1 as std::os::raw::c_int;
                return 4 as std::os::raw::c_int
            }
            _ => { return 0 as std::os::raw::c_int }
        }
    };
    /*--not reached--*/
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressEnd(mut strm: *mut bz_stream)
 -> std::os::raw::c_int {
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut EState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    if !(*s).arr1.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).arr1 as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).arr2.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).arr2 as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).ftab.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).ftab as
                                                               *mut std::os::raw::c_void);
    }
    (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                       (*strm).state);
    (*strm).state = 0 as *mut std::os::raw::c_void;
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
/*--- Decompression stuff                         ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressInit(mut strm: *mut bz_stream,
                                              mut verbosity: std::os::raw::c_int,
                                              mut small: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut s: *mut DState = 0 as *mut DState;
    if bz_config_ok() == 0 { return -(9 as std::os::raw::c_int) }
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    if small != 0 as std::os::raw::c_int && small != 1 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if (*strm).bzalloc.is_none() {
        (*strm).bzalloc =
            Some(default_bzalloc as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: Int32,
                                          _: Int32) -> *mut std::os::raw::c_void)
    }
    if (*strm).bzfree.is_none() {
        (*strm).bzfree =
            Some(default_bzfree as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *mut std::os::raw::c_void) -> ())
    }
    s =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            ::std::mem::size_of::<DState>()
                                                                as
                                                                std::os::raw::c_ulong
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut DState;
    if s.is_null() { return -(3 as std::os::raw::c_int) }
    (*s).strm = strm;
    (*strm).state = s as *mut std::os::raw::c_void;
    (*s).state = 10 as std::os::raw::c_int;
    (*s).bsLive = 0 as std::os::raw::c_int;
    (*s).bsBuff = 0 as std::os::raw::c_int as UInt32;
    (*s).calculatedCombinedCRC = 0 as std::os::raw::c_int as UInt32;
    (*strm).total_in_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_in_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*s).smallDecompress = small as Bool;
    (*s).ll4 = 0 as *mut UChar;
    (*s).ll16 = 0 as *mut UInt16;
    (*s).tt = 0 as *mut UInt32;
    (*s).currBlockNo = 0 as std::os::raw::c_int;
    (*s).verbosity = verbosity;
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
/* Return  True iff data corruption is discovered.
   Returns False if there is no problem.
*/
unsafe extern "C" fn unRLE_obuf_to_output_FAST(mut s: *mut DState) -> Bool {
    let mut current_block: u64;
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as std::os::raw::c_int as Bool != 0 {
            /* try to finish existing run */
            while 1 as std::os::raw::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint
                   {
                    return 0 as std::os::raw::c_int as Bool
                }
                if (*s).state_out_len == 0 as std::os::raw::c_int { break ; }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC =
                    (*s).calculatedBlockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).calculatedBlockCRC >>
                                            24 as std::os::raw::c_int ^
                                            (*s).state_out_ch as std::os::raw::c_uint)
                                           as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
                (*(*s).strm).avail_out =
                    (*(*s).strm).avail_out.wrapping_sub(1);
                (*(*s).strm).total_out_lo32 =
                    (*(*s).strm).total_out_lo32.wrapping_add(1);
                if (*(*s).strm).total_out_lo32 ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    (*(*s).strm).total_out_hi32 =
                        (*(*s).strm).total_out_hi32.wrapping_add(1)
                }
            }
            /* can a new run be started? */
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int as Bool
            }
            /* Only caused by corrupt data stream? */
            if (*s).nblock_used > (*s).save_nblock + 1 as std::os::raw::c_int {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).state_out_len = 1 as std::os::raw::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos >=
                   (100000 as std::os::raw::c_int as
                        UInt32).wrapping_mul((*s).blockSize100k as UInt32) {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
            k1 = ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
            (*s).tPos >>= 8 as std::os::raw::c_int;
            if (*s).rNToGo == 0 as std::os::raw::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                (*s).rTPos += 1;
                if (*s).rTPos == 512 as std::os::raw::c_int {
                    (*s).rTPos = 0 as std::os::raw::c_int
                }
            }
            (*s).rNToGo -= 1;
            k1 =
                (k1 as std::os::raw::c_int ^
                     if (*s).rNToGo == 1 as std::os::raw::c_int {
                         1 as std::os::raw::c_int
                     } else { 0 as std::os::raw::c_int }) as UChar;
            (*s).nblock_used += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                continue ;
            }
            if k1 as std::os::raw::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32
            } else {
                (*s).state_out_len = 2 as std::os::raw::c_int;
                if (*s).tPos >=
                       (100000 as std::os::raw::c_int as
                            UInt32).wrapping_mul((*s).blockSize100k as UInt32)
                   {
                    return 1 as std::os::raw::c_int as Bool
                }
                (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                k1 =
                    ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UChar;
                (*s).tPos >>= 8 as std::os::raw::c_int;
                if (*s).rNToGo == 0 as std::os::raw::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 as std::os::raw::c_int {
                        (*s).rTPos = 0 as std::os::raw::c_int
                    }
                }
                (*s).rNToGo -= 1;
                k1 =
                    (k1 as std::os::raw::c_int ^
                         if (*s).rNToGo == 1 as std::os::raw::c_int {
                             1 as std::os::raw::c_int
                         } else { 0 as std::os::raw::c_int }) as UChar;
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                    continue ;
                }
                if k1 as std::os::raw::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32
                } else {
                    (*s).state_out_len = 3 as std::os::raw::c_int;
                    if (*s).tPos >=
                           (100000 as std::os::raw::c_int as
                                UInt32).wrapping_mul((*s).blockSize100k as
                                                         UInt32) {
                        return 1 as std::os::raw::c_int as Bool
                    }
                    (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                    k1 =
                        ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                            UChar;
                    (*s).tPos >>= 8 as std::os::raw::c_int;
                    if (*s).rNToGo == 0 as std::os::raw::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        (*s).rTPos += 1;
                        if (*s).rTPos == 512 as std::os::raw::c_int {
                            (*s).rTPos = 0 as std::os::raw::c_int
                        }
                    }
                    (*s).rNToGo -= 1;
                    k1 =
                        (k1 as std::os::raw::c_int ^
                             if (*s).rNToGo == 1 as std::os::raw::c_int {
                                 1 as std::os::raw::c_int
                             } else { 0 as std::os::raw::c_int }) as UChar;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int
                       {
                        continue ;
                    }
                    if k1 as std::os::raw::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32
                    } else {
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                        k1 =
                            ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                as UChar;
                        (*s).tPos >>= 8 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        k1 =
                            (k1 as std::os::raw::c_int ^
                                 if (*s).rNToGo == 1 as std::os::raw::c_int {
                                     1 as std::os::raw::c_int
                                 } else { 0 as std::os::raw::c_int }) as UChar;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                        (*s).k0 =
                            ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                as UChar as Int32;
                        (*s).tPos >>= 8 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        (*s).k0 ^=
                            if (*s).rNToGo == 1 as std::os::raw::c_int {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int };
                        (*s).nblock_used += 1
                    }
                }
            }
        }
    } else {
        /* restore */
        let mut c_calculatedBlockCRC: UInt32 = (*s).calculatedBlockCRC;
        let mut c_state_out_ch: UChar = (*s).state_out_ch;
        let mut c_state_out_len: Int32 = (*s).state_out_len;
        let mut c_nblock_used: Int32 = (*s).nblock_used;
        let mut c_k0: Int32 = (*s).k0;
        let mut c_tt: *mut UInt32 = (*s).tt;
        let mut c_tPos: UInt32 = (*s).tPos;
        let mut cs_next_out: *mut std::os::raw::c_char = (*(*s).strm).next_out;
        let mut cs_avail_out: std::os::raw::c_uint = (*(*s).strm).avail_out;
        let mut ro_blockSize100k: Int32 = (*s).blockSize100k;
        /* end restore */
        let mut avail_out_INIT: UInt32 = cs_avail_out;
        let mut s_save_nblockPP: Int32 = (*s).save_nblock + 1 as std::os::raw::c_int;
        let mut total_out_lo32_old: std::os::raw::c_uint = 0;
        's_569:
            while 1 as std::os::raw::c_int as Bool != 0 {
                /* try to finish existing run */
                if c_state_out_len > 0 as std::os::raw::c_int {
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if cs_avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            break 's_569 ;
                        }
                        if c_state_out_len == 1 as std::os::raw::c_int { break ; }
                        *(cs_next_out as *mut UChar) = c_state_out_ch;
                        c_calculatedBlockCRC =
                            c_calculatedBlockCRC << 8 as std::os::raw::c_int ^
                                BZ2_crc32Table[(c_calculatedBlockCRC >>
                                                    24 as std::os::raw::c_int ^
                                                    c_state_out_ch as
                                                        std::os::raw::c_uint) as
                                                   usize];
                        c_state_out_len -= 1;
                        cs_next_out = cs_next_out.offset(1);
                        cs_avail_out = cs_avail_out.wrapping_sub(1)
                    }
                    current_block = 16910810822589621899;
                } else { current_block = 3024573345131975588; }
                loop  {
                    match current_block {
                        16910810822589621899 => {
                            if cs_avail_out ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                c_state_out_len = 1 as std::os::raw::c_int;
                                break 's_569 ;
                            } else {
                                *(cs_next_out as *mut UChar) = c_state_out_ch;
                                c_calculatedBlockCRC =
                                    c_calculatedBlockCRC << 8 as std::os::raw::c_int ^
                                        BZ2_crc32Table[(c_calculatedBlockCRC
                                                            >>
                                                            24 as std::os::raw::c_int
                                                            ^
                                                            c_state_out_ch as
                                                                std::os::raw::c_uint)
                                                           as usize];
                                cs_next_out = cs_next_out.offset(1);
                                cs_avail_out = cs_avail_out.wrapping_sub(1);
                                current_block = 3024573345131975588;
                            }
                        }
                        _ => {
                            /* Only caused by corrupt data stream? */
                            if c_nblock_used > s_save_nblockPP {
                                return 1 as std::os::raw::c_int as Bool
                            }
                            /* can a new run be started? */
                            if c_nblock_used == s_save_nblockPP {
                                c_state_out_len = 0 as std::os::raw::c_int;
                                break 's_569 ;
                            } else {
                                c_state_out_ch = c_k0 as UChar;
                                if c_tPos >=
                                       (100000 as std::os::raw::c_int as
                                            UInt32).wrapping_mul(ro_blockSize100k
                                                                     as
                                                                     UInt32) {
                                    return 1 as std::os::raw::c_int as Bool
                                }
                                c_tPos = *c_tt.offset(c_tPos as isize);
                                k1 =
                                    (c_tPos &
                                         0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                        as UChar;
                                c_tPos >>= 8 as std::os::raw::c_int;
                                c_nblock_used += 1;
                                if k1 as std::os::raw::c_int != c_k0 {
                                    c_k0 = k1 as Int32;
                                    current_block = 16910810822589621899;
                                } else {
                                    if c_nblock_used == s_save_nblockPP {
                                        current_block = 16910810822589621899;
                                        continue ;
                                    }
                                    c_state_out_len = 2 as std::os::raw::c_int;
                                    if c_tPos >=
                                           (100000 as std::os::raw::c_int as
                                                UInt32).wrapping_mul(ro_blockSize100k
                                                                         as
                                                                         UInt32)
                                       {
                                        return 1 as std::os::raw::c_int as Bool
                                    }
                                    c_tPos = *c_tt.offset(c_tPos as isize);
                                    k1 =
                                        (c_tPos &
                                             0xff as std::os::raw::c_int as
                                                 std::os::raw::c_uint) as UChar;
                                    c_tPos >>= 8 as std::os::raw::c_int;
                                    c_nblock_used += 1;
                                    if c_nblock_used == s_save_nblockPP {
                                        continue 's_569 ;
                                    }
                                    if k1 as std::os::raw::c_int != c_k0 {
                                        current_block = 18139099716546303047;
                                        break ;
                                    } else {
                                        current_block = 919396821984190499;
                                        break ;
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    18139099716546303047 => { c_k0 = k1 as Int32 }
                    _ => {
                        c_state_out_len = 3 as std::os::raw::c_int;
                        if c_tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul(ro_blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        c_tPos = *c_tt.offset(c_tPos as isize);
                        k1 =
                            (c_tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                UChar;
                        c_tPos >>= 8 as std::os::raw::c_int;
                        c_nblock_used += 1;
                        if c_nblock_used == s_save_nblockPP { continue ; }
                        if k1 as std::os::raw::c_int != c_k0 {
                            c_k0 = k1 as Int32
                        } else {
                            if c_tPos >=
                                   (100000 as std::os::raw::c_int as
                                        UInt32).wrapping_mul(ro_blockSize100k
                                                                 as UInt32) {
                                return 1 as std::os::raw::c_int as Bool
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            k1 =
                                (c_tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                    as UChar;
                            c_tPos >>= 8 as std::os::raw::c_int;
                            c_nblock_used += 1;
                            c_state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                            if c_tPos >=
                                   (100000 as std::os::raw::c_int as
                                        UInt32).wrapping_mul(ro_blockSize100k
                                                                 as UInt32) {
                                return 1 as std::os::raw::c_int as Bool
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            c_k0 =
                                (c_tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                    as UChar as Int32;
                            c_tPos >>= 8 as std::os::raw::c_int;
                            c_nblock_used += 1
                        }
                    }
                }
            }
        total_out_lo32_old = (*(*s).strm).total_out_lo32;
        (*(*s).strm).total_out_lo32 =
            (*(*s).strm).total_out_lo32.wrapping_add(avail_out_INIT.wrapping_sub(cs_avail_out));
        if (*(*s).strm).total_out_lo32 < total_out_lo32_old {
            (*(*s).strm).total_out_hi32 =
                (*(*s).strm).total_out_hi32.wrapping_add(1)
        }
        /* save */
        (*s).calculatedBlockCRC = c_calculatedBlockCRC;
        (*s).state_out_ch = c_state_out_ch;
        (*s).state_out_len = c_state_out_len;
        (*s).nblock_used = c_nblock_used;
        (*s).k0 = c_k0;
        (*s).tt = c_tt;
        (*s).tPos = c_tPos;
        (*(*s).strm).next_out = cs_next_out;
        (*(*s).strm).avail_out = cs_avail_out
        /* end save */
    }
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------------*/
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn BZ2_indexIntoF(mut indx: Int32,
                                        mut cftab: *mut Int32) -> Int32 {
    let mut nb: Int32 = 0;
    let mut na: Int32 = 0;
    let mut mid: Int32 = 0;
    nb = 0 as std::os::raw::c_int;
    na = 256 as std::os::raw::c_int;
    loop  {
        mid = nb + na >> 1 as std::os::raw::c_int;
        if indx >= *cftab.offset(mid as isize) { nb = mid } else { na = mid }
        if !(na - nb != 1 as std::os::raw::c_int) { break ; }
    }
    return nb;
}
/*---------------------------------------------------*/
/* Return  True iff data corruption is discovered.
   Returns False if there is no problem.
*/
unsafe extern "C" fn unRLE_obuf_to_output_SMALL(mut s: *mut DState) -> Bool {
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as std::os::raw::c_int as Bool != 0 {
            /* try to finish existing run */
            while 1 as std::os::raw::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint
                   {
                    return 0 as std::os::raw::c_int as Bool
                }
                if (*s).state_out_len == 0 as std::os::raw::c_int { break ; }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC =
                    (*s).calculatedBlockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).calculatedBlockCRC >>
                                            24 as std::os::raw::c_int ^
                                            (*s).state_out_ch as std::os::raw::c_uint)
                                           as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
                (*(*s).strm).avail_out =
                    (*(*s).strm).avail_out.wrapping_sub(1);
                (*(*s).strm).total_out_lo32 =
                    (*(*s).strm).total_out_lo32.wrapping_add(1);
                if (*(*s).strm).total_out_lo32 ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    (*(*s).strm).total_out_hi32 =
                        (*(*s).strm).total_out_hi32.wrapping_add(1)
                }
            }
            /* can a new run be started? */
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int as Bool
            }
            /* Only caused by corrupt data stream? */
            if (*s).nblock_used > (*s).save_nblock + 1 as std::os::raw::c_int {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).state_out_len = 1 as std::os::raw::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos >=
                   (100000 as std::os::raw::c_int as
                        UInt32).wrapping_mul((*s).blockSize100k as UInt32) {
                return 1 as std::os::raw::c_int as Bool
            }
            k1 =
                BZ2_indexIntoF((*s).tPos as Int32, (*s).cftab.as_mut_ptr()) as
                    UChar;
            (*s).tPos =
                *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                    (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                          isize) as UInt32 >>
                         ((*s).tPos << 2 as std::os::raw::c_int &
                              0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                         0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                        16 as std::os::raw::c_int;
            if (*s).rNToGo == 0 as std::os::raw::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                (*s).rTPos += 1;
                if (*s).rTPos == 512 as std::os::raw::c_int {
                    (*s).rTPos = 0 as std::os::raw::c_int
                }
            }
            (*s).rNToGo -= 1;
            k1 =
                (k1 as std::os::raw::c_int ^
                     if (*s).rNToGo == 1 as std::os::raw::c_int {
                         1 as std::os::raw::c_int
                     } else { 0 as std::os::raw::c_int }) as UChar;
            (*s).nblock_used += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                continue ;
            }
            if k1 as std::os::raw::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32
            } else {
                (*s).state_out_len = 2 as std::os::raw::c_int;
                if (*s).tPos >=
                       (100000 as std::os::raw::c_int as
                            UInt32).wrapping_mul((*s).blockSize100k as UInt32)
                   {
                    return 1 as std::os::raw::c_int as Bool
                }
                k1 =
                    BZ2_indexIntoF((*s).tPos as Int32,
                                   (*s).cftab.as_mut_ptr()) as UChar;
                (*s).tPos =
                    *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                        (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                              isize) as UInt32 >>
                             ((*s).tPos << 2 as std::os::raw::c_int &
                                  0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                             0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                            16 as std::os::raw::c_int;
                if (*s).rNToGo == 0 as std::os::raw::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 as std::os::raw::c_int {
                        (*s).rTPos = 0 as std::os::raw::c_int
                    }
                }
                (*s).rNToGo -= 1;
                k1 =
                    (k1 as std::os::raw::c_int ^
                         if (*s).rNToGo == 1 as std::os::raw::c_int {
                             1 as std::os::raw::c_int
                         } else { 0 as std::os::raw::c_int }) as UChar;
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                    continue ;
                }
                if k1 as std::os::raw::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32
                } else {
                    (*s).state_out_len = 3 as std::os::raw::c_int;
                    if (*s).tPos >=
                           (100000 as std::os::raw::c_int as
                                UInt32).wrapping_mul((*s).blockSize100k as
                                                         UInt32) {
                        return 1 as std::os::raw::c_int as Bool
                    }
                    k1 =
                        BZ2_indexIntoF((*s).tPos as Int32,
                                       (*s).cftab.as_mut_ptr()) as UChar;
                    (*s).tPos =
                        *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                            (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int)
                                                  as isize) as UInt32 >>
                                 ((*s).tPos << 2 as std::os::raw::c_int &
                                      0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                                 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                16 as std::os::raw::c_int;
                    if (*s).rNToGo == 0 as std::os::raw::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        (*s).rTPos += 1;
                        if (*s).rTPos == 512 as std::os::raw::c_int {
                            (*s).rTPos = 0 as std::os::raw::c_int
                        }
                    }
                    (*s).rNToGo -= 1;
                    k1 =
                        (k1 as std::os::raw::c_int ^
                             if (*s).rNToGo == 1 as std::os::raw::c_int {
                                 1 as std::os::raw::c_int
                             } else { 0 as std::os::raw::c_int }) as UChar;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int
                       {
                        continue ;
                    }
                    if k1 as std::os::raw::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32
                    } else {
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        k1 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr()) as UChar;
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        k1 =
                            (k1 as std::os::raw::c_int ^
                                 if (*s).rNToGo == 1 as std::os::raw::c_int {
                                     1 as std::os::raw::c_int
                                 } else { 0 as std::os::raw::c_int }) as UChar;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).k0 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr());
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        (*s).k0 ^=
                            if (*s).rNToGo == 1 as std::os::raw::c_int {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int };
                        (*s).nblock_used += 1
                    }
                }
            }
        }
    } else {
        while 1 as std::os::raw::c_int as Bool != 0 {
            /* try to finish existing run */
            while 1 as std::os::raw::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint
                   {
                    return 0 as std::os::raw::c_int as Bool
                }
                if (*s).state_out_len == 0 as std::os::raw::c_int { break ; }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC =
                    (*s).calculatedBlockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).calculatedBlockCRC >>
                                            24 as std::os::raw::c_int ^
                                            (*s).state_out_ch as std::os::raw::c_uint)
                                           as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
                (*(*s).strm).avail_out =
                    (*(*s).strm).avail_out.wrapping_sub(1);
                (*(*s).strm).total_out_lo32 =
                    (*(*s).strm).total_out_lo32.wrapping_add(1);
                if (*(*s).strm).total_out_lo32 ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    (*(*s).strm).total_out_hi32 =
                        (*(*s).strm).total_out_hi32.wrapping_add(1)
                }
            }
            /* can a new run be started? */
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int as Bool
            }
            /* Only caused by corrupt data stream? */
            if (*s).nblock_used > (*s).save_nblock + 1 as std::os::raw::c_int {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).state_out_len = 1 as std::os::raw::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos >=
                   (100000 as std::os::raw::c_int as
                        UInt32).wrapping_mul((*s).blockSize100k as UInt32) {
                return 1 as std::os::raw::c_int as Bool
            }
            k1 =
                BZ2_indexIntoF((*s).tPos as Int32, (*s).cftab.as_mut_ptr()) as
                    UChar;
            (*s).tPos =
                *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                    (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                          isize) as UInt32 >>
                         ((*s).tPos << 2 as std::os::raw::c_int &
                              0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                         0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                        16 as std::os::raw::c_int;
            (*s).nblock_used += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                continue ;
            }
            if k1 as std::os::raw::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32
            } else {
                (*s).state_out_len = 2 as std::os::raw::c_int;
                if (*s).tPos >=
                       (100000 as std::os::raw::c_int as
                            UInt32).wrapping_mul((*s).blockSize100k as UInt32)
                   {
                    return 1 as std::os::raw::c_int as Bool
                }
                k1 =
                    BZ2_indexIntoF((*s).tPos as Int32,
                                   (*s).cftab.as_mut_ptr()) as UChar;
                (*s).tPos =
                    *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                        (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                              isize) as UInt32 >>
                             ((*s).tPos << 2 as std::os::raw::c_int &
                                  0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                             0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                            16 as std::os::raw::c_int;
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                    continue ;
                }
                if k1 as std::os::raw::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32
                } else {
                    (*s).state_out_len = 3 as std::os::raw::c_int;
                    if (*s).tPos >=
                           (100000 as std::os::raw::c_int as
                                UInt32).wrapping_mul((*s).blockSize100k as
                                                         UInt32) {
                        return 1 as std::os::raw::c_int as Bool
                    }
                    k1 =
                        BZ2_indexIntoF((*s).tPos as Int32,
                                       (*s).cftab.as_mut_ptr()) as UChar;
                    (*s).tPos =
                        *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                            (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int)
                                                  as isize) as UInt32 >>
                                 ((*s).tPos << 2 as std::os::raw::c_int &
                                      0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                                 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                16 as std::os::raw::c_int;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int
                       {
                        continue ;
                    }
                    if k1 as std::os::raw::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32
                    } else {
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        k1 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr()) as UChar;
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).k0 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr());
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        (*s).nblock_used += 1
                    }
                }
            }
        }
    }
    panic!("Reached end of non-void function without returning");
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompress(mut strm: *mut bz_stream)
 -> std::os::raw::c_int {
    let mut corrupt: Bool = 0;
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut DState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    while 1 as std::os::raw::c_int as Bool != 0 {
        if (*s).state == 1 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        if (*s).state == 2 as std::os::raw::c_int {
            if (*s).smallDecompress != 0 {
                corrupt = unRLE_obuf_to_output_SMALL(s)
            } else { corrupt = unRLE_obuf_to_output_FAST(s) }
            if corrupt != 0 { return -(4 as std::os::raw::c_int) }
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int &&
                   (*s).state_out_len == 0 as std::os::raw::c_int {
                (*s).calculatedBlockCRC = !(*s).calculatedBlockCRC;
                if (*s).verbosity >= 3 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b" {0x%08x, 0x%08x}\x00" as *const u8 as
                                *const std::os::raw::c_char, (*s).storedBlockCRC,
                            (*s).calculatedBlockCRC);
                }
                if (*s).verbosity >= 2 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"]\x00" as *const u8 as *const std::os::raw::c_char);
                }
                if (*s).calculatedBlockCRC != (*s).storedBlockCRC {
                    return -(4 as std::os::raw::c_int)
                }
                (*s).calculatedCombinedCRC =
                    (*s).calculatedCombinedCRC << 1 as std::os::raw::c_int |
                        (*s).calculatedCombinedCRC >> 31 as std::os::raw::c_int;
                (*s).calculatedCombinedCRC ^= (*s).calculatedBlockCRC;
                (*s).state = 14 as std::os::raw::c_int
            } else { return 0 as std::os::raw::c_int }
        }
        if (*s).state >= 10 as std::os::raw::c_int {
            let mut r: Int32 = BZ2_decompress(s);
            if r == 4 as std::os::raw::c_int {
                if (*s).verbosity >= 3 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"\n    combined CRCs: stored = 0x%08x, computed = 0x%08x\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*s).storedCombinedCRC,
                            (*s).calculatedCombinedCRC);
                }
                if (*s).calculatedCombinedCRC != (*s).storedCombinedCRC {
                    return -(4 as std::os::raw::c_int)
                }
                return r
            }
            if (*s).state != 2 as std::os::raw::c_int { return r }
        }
    }
    if 0 as std::os::raw::c_int == 0 { BZ2_bz__AssertH__fail(6001 as std::os::raw::c_int); }
    return 0 as std::os::raw::c_int;
    /*NOTREACHED*/
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressEnd(mut strm: *mut bz_stream)
 -> std::os::raw::c_int {
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut DState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    if !(*s).tt.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).tt as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).ll16.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).ll16 as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).ll4.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).ll4 as
                                                               *mut std::os::raw::c_void);
    }
    (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                       (*strm).state);
    (*strm).state = 0 as *mut std::os::raw::c_void;
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------*/
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c: Int32 = fgetc(f);
    if c == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int as Bool }
    ungetc(c, f);
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteOpen(mut bzerror: *mut std::os::raw::c_int,
                                         mut f: *mut FILE,
                                         mut blockSize100k: std::os::raw::c_int,
                                         mut verbosity: std::os::raw::c_int,
                                         mut workFactor: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if f.is_null() ||
           (blockSize100k < 1 as std::os::raw::c_int ||
                blockSize100k > 9 as std::os::raw::c_int) ||
           (workFactor < 0 as std::os::raw::c_int || workFactor > 250 as std::os::raw::c_int)
           || (verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int) {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if ferror(f) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    bzf =
        malloc(::std::mem::size_of::<bzFile>() as std::os::raw::c_ulong) as
            *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = -(3 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(3 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    (*bzf).initialisedOk = 0 as std::os::raw::c_int as Bool;
    (*bzf).bufN = 0 as std::os::raw::c_int;
    (*bzf).handle = f;
    (*bzf).writing = 1 as std::os::raw::c_int as Bool;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut std::os::raw::c_void;
    if workFactor == 0 as std::os::raw::c_int { workFactor = 30 as std::os::raw::c_int }
    ret =
        BZ2_bzCompressInit(&mut (*bzf).strm, blockSize100k, verbosity,
                           workFactor);
    if ret != 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = ret }
        if !bzf.is_null() { (*bzf).lastErr = ret }
        free(bzf as *mut std::os::raw::c_void);
        return 0 as *mut std::os::raw::c_void
    }
    (*bzf).strm.avail_in = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*bzf).initialisedOk = 1 as std::os::raw::c_int as Bool;
    return bzf as *mut std::os::raw::c_void;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWrite(mut bzerror: *mut std::os::raw::c_int,
                                     mut b: *mut std::os::raw::c_void,
                                     mut buf: *mut std::os::raw::c_void,
                                     mut len: std::os::raw::c_int) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if bzf.is_null() || buf.is_null() || len < 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if ferror((*bzf).handle) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return
    }
    if len == 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return
    }
    (*bzf).strm.avail_in = len as std::os::raw::c_uint;
    (*bzf).strm.next_in = buf as *mut std::os::raw::c_char;
    while 1 as std::os::raw::c_int as Bool != 0 {
        (*bzf).strm.avail_out = 5000 as std::os::raw::c_int as std::os::raw::c_uint;
        (*bzf).strm.next_out = (*bzf).buf.as_mut_ptr();
        ret = BZ2_bzCompress(&mut (*bzf).strm, 0 as std::os::raw::c_int);
        if ret != 1 as std::os::raw::c_int {
            if !bzerror.is_null() { *bzerror = ret }
            if !bzf.is_null() { (*bzf).lastErr = ret }
            return
        }
        if (*bzf).strm.avail_out < 5000 as std::os::raw::c_int as std::os::raw::c_uint {
            n =
                (5000 as std::os::raw::c_int as
                     std::os::raw::c_uint).wrapping_sub((*bzf).strm.avail_out) as
                    Int32;
            n2 =
                fwrite((*bzf).buf.as_mut_ptr() as *mut std::os::raw::c_void,
                       ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                       n as std::os::raw::c_ulong, (*bzf).handle) as Int32;
            if n != n2 || ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
                if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
                return
            }
        }
        if (*bzf).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
            if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
            return
        }
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose(mut bzerror: *mut std::os::raw::c_int,
                                          mut b: *mut std::os::raw::c_void,
                                          mut abandon: std::os::raw::c_int,
                                          mut nbytes_in: *mut std::os::raw::c_uint,
                                          mut nbytes_out: *mut std::os::raw::c_uint) {
    BZ2_bzWriteClose64(bzerror, b, abandon, nbytes_in, 0 as *mut std::os::raw::c_uint,
                       nbytes_out, 0 as *mut std::os::raw::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose64(mut bzerror: *mut std::os::raw::c_int,
                                            mut b: *mut std::os::raw::c_void,
                                            mut abandon: std::os::raw::c_int,
                                            mut nbytes_in_lo32:
                                                *mut std::os::raw::c_uint,
                                            mut nbytes_in_hi32:
                                                *mut std::os::raw::c_uint,
                                            mut nbytes_out_lo32:
                                                *mut std::os::raw::c_uint,
                                            mut nbytes_out_hi32:
                                                *mut std::os::raw::c_uint) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if ferror((*bzf).handle) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if abandon == 0 && (*bzf).lastErr == 0 as std::os::raw::c_int {
        while 1 as std::os::raw::c_int as Bool != 0 {
            (*bzf).strm.avail_out = 5000 as std::os::raw::c_int as std::os::raw::c_uint;
            (*bzf).strm.next_out = (*bzf).buf.as_mut_ptr();
            ret = BZ2_bzCompress(&mut (*bzf).strm, 2 as std::os::raw::c_int);
            if ret != 3 as std::os::raw::c_int && ret != 4 as std::os::raw::c_int {
                if !bzerror.is_null() { *bzerror = ret }
                if !bzf.is_null() { (*bzf).lastErr = ret }
                return
            }
            if (*bzf).strm.avail_out < 5000 as std::os::raw::c_int as std::os::raw::c_uint {
                n =
                    (5000 as std::os::raw::c_int as
                         std::os::raw::c_uint).wrapping_sub((*bzf).strm.avail_out) as
                        Int32;
                n2 =
                    fwrite((*bzf).buf.as_mut_ptr() as *mut std::os::raw::c_void,
                           ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                           n as std::os::raw::c_ulong, (*bzf).handle) as Int32;
                if n != n2 || ferror((*bzf).handle) != 0 {
                    if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
                    if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
                    return
                }
            }
            if ret == 4 as std::os::raw::c_int { break ; }
        }
    }
    if abandon == 0 && ferror((*bzf).handle) == 0 {
        fflush((*bzf).handle);
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
            if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
            return
        }
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = (*bzf).strm.total_in_lo32
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = (*bzf).strm.total_in_hi32
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = (*bzf).strm.total_out_lo32
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = (*bzf).strm.total_out_hi32
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    BZ2_bzCompressEnd(&mut (*bzf).strm);
    free(bzf as *mut std::os::raw::c_void);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadOpen(mut bzerror: *mut std::os::raw::c_int,
                                        mut f: *mut FILE,
                                        mut verbosity: std::os::raw::c_int,
                                        mut small: std::os::raw::c_int,
                                        mut unused: *mut std::os::raw::c_void,
                                        mut nUnused: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    let mut ret: std::os::raw::c_int = 0;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if f.is_null() || small != 0 as std::os::raw::c_int && small != 1 as std::os::raw::c_int
           || (verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int)
           || unused.is_null() && nUnused != 0 as std::os::raw::c_int ||
           !unused.is_null() &&
               (nUnused < 0 as std::os::raw::c_int || nUnused > 5000 as std::os::raw::c_int) {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if ferror(f) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    bzf =
        malloc(::std::mem::size_of::<bzFile>() as std::os::raw::c_ulong) as
            *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = -(3 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(3 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    (*bzf).initialisedOk = 0 as std::os::raw::c_int as Bool;
    (*bzf).handle = f;
    (*bzf).bufN = 0 as std::os::raw::c_int;
    (*bzf).writing = 0 as std::os::raw::c_int as Bool;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut std::os::raw::c_void;
    while nUnused > 0 as std::os::raw::c_int {
        (*bzf).buf[(*bzf).bufN as usize] = *(unused as *mut UChar) as Char;
        (*bzf).bufN += 1;
        unused =
            (unused as *mut UChar).offset(1 as std::os::raw::c_int as isize) as
                *mut std::os::raw::c_void;
        nUnused -= 1
    }
    ret = BZ2_bzDecompressInit(&mut (*bzf).strm, verbosity, small);
    if ret != 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = ret }
        if !bzf.is_null() { (*bzf).lastErr = ret }
        free(bzf as *mut std::os::raw::c_void);
        return 0 as *mut std::os::raw::c_void
    }
    (*bzf).strm.avail_in = (*bzf).bufN as std::os::raw::c_uint;
    (*bzf).strm.next_in = (*bzf).buf.as_mut_ptr();
    (*bzf).initialisedOk = 1 as std::os::raw::c_int as Bool;
    return bzf as *mut std::os::raw::c_void;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadClose(mut bzerror: *mut std::os::raw::c_int,
                                         mut b: *mut std::os::raw::c_void) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if (*bzf).initialisedOk != 0 { BZ2_bzDecompressEnd(&mut (*bzf).strm); }
    free(bzf as *mut std::os::raw::c_void);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzRead(mut bzerror: *mut std::os::raw::c_int,
                                    mut b: *mut std::os::raw::c_void,
                                    mut buf: *mut std::os::raw::c_void,
                                    mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut n: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if bzf.is_null() || buf.is_null() || len < 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return 0 as std::os::raw::c_int
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return 0 as std::os::raw::c_int
    }
    if len == 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return 0 as std::os::raw::c_int
    }
    (*bzf).strm.avail_out = len as std::os::raw::c_uint;
    (*bzf).strm.next_out = buf as *mut std::os::raw::c_char;
    while 1 as std::os::raw::c_int as Bool != 0 {
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
            if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
            return 0 as std::os::raw::c_int
        }
        if (*bzf).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
               myfeof((*bzf).handle) == 0 {
            n =
                fread((*bzf).buf.as_mut_ptr() as *mut std::os::raw::c_void,
                      ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                      5000 as std::os::raw::c_int as std::os::raw::c_ulong, (*bzf).handle) as
                    Int32;
            if ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
                if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
                return 0 as std::os::raw::c_int
            }
            (*bzf).bufN = n;
            (*bzf).strm.avail_in = (*bzf).bufN as std::os::raw::c_uint;
            (*bzf).strm.next_in = (*bzf).buf.as_mut_ptr()
        }
        ret = BZ2_bzDecompress(&mut (*bzf).strm);
        if ret != 0 as std::os::raw::c_int && ret != 4 as std::os::raw::c_int {
            if !bzerror.is_null() { *bzerror = ret }
            if !bzf.is_null() { (*bzf).lastErr = ret }
            return 0 as std::os::raw::c_int
        }
        if ret == 0 as std::os::raw::c_int &&
               myfeof((*bzf).handle) as std::os::raw::c_int != 0 &&
               (*bzf).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
               (*bzf).strm.avail_out > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if !bzerror.is_null() { *bzerror = -(7 as std::os::raw::c_int) }
            if !bzf.is_null() { (*bzf).lastErr = -(7 as std::os::raw::c_int) }
            return 0 as std::os::raw::c_int
        }
        if ret == 4 as std::os::raw::c_int {
            if !bzerror.is_null() { *bzerror = 4 as std::os::raw::c_int }
            if !bzf.is_null() { (*bzf).lastErr = 4 as std::os::raw::c_int }
            return (len as std::os::raw::c_uint).wrapping_sub((*bzf).strm.avail_out)
                       as std::os::raw::c_int
        }
        if (*bzf).strm.avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
            if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
            return len
        }
    }
    return 0 as std::os::raw::c_int;
    /*not reached*/
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadGetUnused(mut bzerror: *mut std::os::raw::c_int,
                                             mut b: *mut std::os::raw::c_void,
                                             mut unused:
                                                 *mut *mut std::os::raw::c_void,
                                             mut nUnused: *mut std::os::raw::c_int) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return
    }
    if (*bzf).lastErr != 4 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if unused.is_null() || nUnused.is_null() {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    *nUnused = (*bzf).strm.avail_in as std::os::raw::c_int;
    *unused = (*bzf).strm.next_in as *mut std::os::raw::c_void;
}
/*---------------------------------------------------*/
/*--- Misc convenience stuff                      ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffCompress(mut dest: *mut std::os::raw::c_char,
                                                  mut destLen:
                                                      *mut std::os::raw::c_uint,
                                                  mut source:
                                                      *mut std::os::raw::c_char,
                                                  mut sourceLen: std::os::raw::c_uint,
                                                  mut blockSize100k:
                                                      std::os::raw::c_int,
                                                  mut verbosity: std::os::raw::c_int,
                                                  mut workFactor: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut strm: bz_stream =
        bz_stream{next_in: 0 as *mut std::os::raw::c_char,
                  avail_in: 0,
                  total_in_lo32: 0,
                  total_in_hi32: 0,
                  next_out: 0 as *mut std::os::raw::c_char,
                  avail_out: 0,
                  total_out_lo32: 0,
                  total_out_hi32: 0,
                  state: 0 as *mut std::os::raw::c_void,
                  bzalloc: None,
                  bzfree: None,
                  opaque: 0 as *mut std::os::raw::c_void,};
    let mut ret: std::os::raw::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null() ||
           blockSize100k < 1 as std::os::raw::c_int ||
           blockSize100k > 9 as std::os::raw::c_int || verbosity < 0 as std::os::raw::c_int ||
           verbosity > 4 as std::os::raw::c_int || workFactor < 0 as std::os::raw::c_int ||
           workFactor > 250 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if workFactor == 0 as std::os::raw::c_int { workFactor = 30 as std::os::raw::c_int }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut std::os::raw::c_void;
    ret = BZ2_bzCompressInit(&mut strm, blockSize100k, verbosity, workFactor);
    if ret != 0 as std::os::raw::c_int { return ret }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzCompress(&mut strm, 2 as std::os::raw::c_int);
    if ret == 3 as std::os::raw::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return -(8 as std::os::raw::c_int)
    } else if ret != 4 as std::os::raw::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return ret
    } else {
        /* normal termination */
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzCompressEnd(&mut strm);
        return 0 as std::os::raw::c_int
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffDecompress(mut dest:
                                                        *mut std::os::raw::c_char,
                                                    mut destLen:
                                                        *mut std::os::raw::c_uint,
                                                    mut source:
                                                        *mut std::os::raw::c_char,
                                                    mut sourceLen:
                                                        std::os::raw::c_uint,
                                                    mut small: std::os::raw::c_int,
                                                    mut verbosity:
                                                        std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut strm: bz_stream =
        bz_stream{next_in: 0 as *mut std::os::raw::c_char,
                  avail_in: 0,
                  total_in_lo32: 0,
                  total_in_hi32: 0,
                  next_out: 0 as *mut std::os::raw::c_char,
                  avail_out: 0,
                  total_out_lo32: 0,
                  total_out_hi32: 0,
                  state: 0 as *mut std::os::raw::c_void,
                  bzalloc: None,
                  bzfree: None,
                  opaque: 0 as *mut std::os::raw::c_void,};
    let mut ret: std::os::raw::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null() ||
           small != 0 as std::os::raw::c_int && small != 1 as std::os::raw::c_int ||
           verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut std::os::raw::c_void;
    ret = BZ2_bzDecompressInit(&mut strm, verbosity, small);
    if ret != 0 as std::os::raw::c_int { return ret }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzDecompress(&mut strm);
    if ret == 0 as std::os::raw::c_int {
        if strm.avail_out > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            BZ2_bzDecompressEnd(&mut strm);
            return -(7 as std::os::raw::c_int)
        } else { BZ2_bzDecompressEnd(&mut strm); return -(8 as std::os::raw::c_int) }
    } else if ret != 4 as std::os::raw::c_int {
        BZ2_bzDecompressEnd(&mut strm);
        return ret
    } else {
        /* normal termination */
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzDecompressEnd(&mut strm);
        return 0 as std::os::raw::c_int
    };
}
/*---------------------------------------------------*/
/*--
   Code contributed by Yoshioka Tsuneo (tsuneo@rr.iij4u.or.jp)
   to support better zlib compatibility.
   This code is not _officially_ part of libbzip2 (yet);
   I haven't tested it, documented it, or considered the
   threading-safeness of it.
   If this code breaks, please contact both Yoshioka and me.
--*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
/*--
   return version like "0.9.5d, 4-Sept-1999".
--*/
#[no_mangle]
pub extern "C" fn BZ2_bzlibVersion() -> *const std::os::raw::c_char {
    return b"1.0.8, 13-Jul-2019\x00" as *const u8 as *const std::os::raw::c_char;
}
/*---------------------------------------------------*/
unsafe extern "C" fn bzopen_or_bzdopen(mut path: *const std::os::raw::c_char,
                                       mut fd: std::os::raw::c_int,
                                       mut mode: *const std::os::raw::c_char,
                                       mut open_mode: std::os::raw::c_int)
 -> *mut std::os::raw::c_void 
 /* bzopen: 0, bzdopen:1 */
 {
    let mut bzerr: std::os::raw::c_int = 0; /* binary mode */
    let mut unused: [std::os::raw::c_char; 5000] = [0; 5000];
    let mut blockSize100k: std::os::raw::c_int = 9 as std::os::raw::c_int;
    let mut writing: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut mode2: [std::os::raw::c_char; 10] =
        *::std::mem::transmute::<&[u8; 10],
                                 &mut [std::os::raw::c_char; 10]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut bzfp: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut verbosity: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut workFactor: std::os::raw::c_int = 30 as std::os::raw::c_int;
    let mut smallMode: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut nUnused: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if mode.is_null() { return 0 as *mut std::os::raw::c_void }
    while *mode != 0 {
        match *mode as std::os::raw::c_int {
            114 => { writing = 0 as std::os::raw::c_int }
            119 => { writing = 1 as std::os::raw::c_int }
            115 => { smallMode = 1 as std::os::raw::c_int }
            _ => {
                if isdigit(*mode as std::os::raw::c_int) != 0 {
                    blockSize100k = *mode as std::os::raw::c_int - 0x30 as std::os::raw::c_int
                }
            }
        }
        mode = mode.offset(1)
    }
    strcat(mode2.as_mut_ptr(),
           if writing != 0 {
               b"w\x00" as *const u8 as *const std::os::raw::c_char
           } else { b"r\x00" as *const u8 as *const std::os::raw::c_char });
    strcat(mode2.as_mut_ptr(), b"b\x00" as *const u8 as *const std::os::raw::c_char);
    if open_mode == 0 as std::os::raw::c_int {
        if path.is_null() ||
               strcmp(path, b"\x00" as *const u8 as *const std::os::raw::c_char) ==
                   0 as std::os::raw::c_int {
            fp = if writing != 0 { __stdoutp } else { __stdinp }
        } else { fp = fopen(path, mode2.as_mut_ptr()) }
    } else { fp = fdopen(fd, mode2.as_mut_ptr()) }
    if fp.is_null() { return 0 as *mut std::os::raw::c_void }
    if writing != 0 {
        /* Guard against total chaos and anarchy -- JRS */
        if blockSize100k < 1 as std::os::raw::c_int {
            blockSize100k = 1 as std::os::raw::c_int
        }
        if blockSize100k > 9 as std::os::raw::c_int {
            blockSize100k = 9 as std::os::raw::c_int
        }
        bzfp =
            BZ2_bzWriteOpen(&mut bzerr, fp, blockSize100k, verbosity,
                            workFactor)
    } else {
        bzfp =
            BZ2_bzReadOpen(&mut bzerr, fp, verbosity, smallMode,
                           unused.as_mut_ptr() as *mut std::os::raw::c_void, nUnused)
    }
    if bzfp.is_null() {
        if fp != __stdinp && fp != __stdoutp { fclose(fp); }
        return 0 as *mut std::os::raw::c_void
    }
    return bzfp;
}
/*---------------------------------------------------*/
/*--
   open file for read or write.
      ex) bzopen("file","w9")
      case path="" or NULL => use stdin or stdout.
--*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzopen(mut path: *const std::os::raw::c_char,
                                    mut mode: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    return bzopen_or_bzdopen(path, -(1 as std::os::raw::c_int), mode,
                             0 as std::os::raw::c_int);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzdopen(mut fd: std::os::raw::c_int,
                                     mut mode: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    return bzopen_or_bzdopen(0 as *const std::os::raw::c_char, fd, mode,
                             1 as std::os::raw::c_int);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzread(mut b: *mut std::os::raw::c_void,
                                    mut buf: *mut std::os::raw::c_void,
                                    mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut bzerr: std::os::raw::c_int = 0;
    let mut nread: std::os::raw::c_int = 0;
    if (*(b as *mut bzFile)).lastErr == 4 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    nread = BZ2_bzRead(&mut bzerr, b, buf, len);
    if bzerr == 0 as std::os::raw::c_int || bzerr == 4 as std::os::raw::c_int {
        return nread
    } else { return -(1 as std::os::raw::c_int) };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzwrite(mut b: *mut std::os::raw::c_void,
                                     mut buf: *mut std::os::raw::c_void,
                                     mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut bzerr: std::os::raw::c_int = 0;
    BZ2_bzWrite(&mut bzerr, b, buf, len);
    if bzerr == 0 as std::os::raw::c_int {
        return len
    } else { return -(1 as std::os::raw::c_int) };
}
/*---------------------------------------------------*/
#[no_mangle]
pub extern "C" fn BZ2_bzflush(mut b: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    /* do nothing now... */
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzclose(mut b: *mut std::os::raw::c_void) {
    let mut bzerr: std::os::raw::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if b.is_null() { return }
    fp = (*(b as *mut bzFile)).handle;
    if (*(b as *mut bzFile)).writing != 0 {
        BZ2_bzWriteClose(&mut bzerr, b, 0 as std::os::raw::c_int,
                         0 as *mut std::os::raw::c_uint, 0 as *mut std::os::raw::c_uint);
        if bzerr != 0 as std::os::raw::c_int {
            BZ2_bzWriteClose(0 as *mut std::os::raw::c_int, b, 1 as std::os::raw::c_int,
                             0 as *mut std::os::raw::c_uint, 0 as *mut std::os::raw::c_uint);
        }
    } else { BZ2_bzReadClose(&mut bzerr, b); }
    if fp != __stdinp && fp != __stdoutp { fclose(fp); };
}
/*---------------------------------------------------*/
/*--
   return last error code 
--*/
static mut bzerrorstrings: [*const std::os::raw::c_char; 16] =
    [b"OK\x00" as *const u8 as *const std::os::raw::c_char,
     b"SEQUENCE_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"PARAM_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"MEM_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"DATA_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"DATA_ERROR_MAGIC\x00" as *const u8 as *const std::os::raw::c_char,
     b"IO_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"UNEXPECTED_EOF\x00" as *const u8 as *const std::os::raw::c_char,
     b"OUTBUFF_FULL\x00" as *const u8 as *const std::os::raw::c_char,
     b"CONFIG_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzerror(mut b: *mut std::os::raw::c_void,
                                     mut errnum: *mut std::os::raw::c_int)
 -> *const std::os::raw::c_char {
    let mut err: std::os::raw::c_int = (*(b as *mut bzFile)).lastErr;
    if err > 0 as std::os::raw::c_int { err = 0 as std::os::raw::c_int }
    *errnum = err;
    return bzerrorstrings[(err * -(1 as std::os::raw::c_int)) as usize];
}
/*-------------------------------------------------------------*/
/*--- end                                           bzlib.c ---*/
/*-------------------------------------------------------------*/

}

pub mod compress {

extern "C" {
    
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    
    
    
    
    
    
    
    
}
pub use crate::blocksort::BZ2_blockSort;
pub use crate::bzlib::BZ2_bz__AssertH__fail;
pub use crate::huffman::BZ2_hbAssignCodes;
pub use crate::huffman::BZ2_hbMakeCodeLengths;
pub use crate::blocksort::__sFILEX;
pub type __int64_t = crate::blocksort::__int64_t;
pub type __darwin_off_t = crate::blocksort::__darwin_off_t;
pub type fpos_t = crate::blocksort::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::blocksort::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::blocksort::__sFILE;
pub type FILE = crate::blocksort::FILE;
// #[derive(Copy, Clone)]

pub type bz_stream = crate::blocksort::bz_stream;
pub type Bool = crate::blocksort::Bool;
pub type UChar = crate::blocksort::UChar;
pub type Int32 = crate::blocksort::Int32;
pub type UInt32 = crate::blocksort::UInt32;
pub type UInt16 = crate::blocksort::UInt16;
// #[derive(Copy, Clone)]

pub type EState = crate::blocksort::EState;
/*-------------------------------------------------------------*/
/*--- Compression machinery (not incl block sorting)        ---*/
/*---                                            compress.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/* CHANGES
    0.9.0    -- original version.
    0.9.0a/b -- no changes in this file.
    0.9.0c   -- changed setting of nGroups in sendMTFValues() 
                so as to do a bit better on small files
*/
/*---------------------------------------------------*/
/*--- Bit stream I/O                              ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bsInitWrite(mut s: *mut EState) {
    (*s).bsLive = 0 as std::os::raw::c_int;
    (*s).bsBuff = 0 as std::os::raw::c_int as UInt32;
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsFinishWrite(mut s: *mut EState) {
    while (*s).bsLive > 0 as std::os::raw::c_int {
        *(*s).zbits.offset((*s).numZ as isize) =
            ((*s).bsBuff >> 24 as std::os::raw::c_int) as UChar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8 as std::os::raw::c_int;
        (*s).bsLive -= 8 as std::os::raw::c_int
    };
}
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[inline]
unsafe extern "C" fn bsW(mut s: *mut EState, mut n: Int32, mut v: UInt32) {
    while (*s).bsLive >= 8 as std::os::raw::c_int {
        *(*s).zbits.offset((*s).numZ as isize) =
            ((*s).bsBuff >> 24 as std::os::raw::c_int) as UChar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8 as std::os::raw::c_int;
        (*s).bsLive -= 8 as std::os::raw::c_int
    }
    (*s).bsBuff |= v << 32 as std::os::raw::c_int - (*s).bsLive - n;
    (*s).bsLive += n;
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsPutUInt32(mut s: *mut EState, mut u: UInt32) {
    bsW(s, 8 as std::os::raw::c_int,
        ((u >> 24 as std::os::raw::c_int) as std::os::raw::c_long & 0xff as std::os::raw::c_long) as
            UInt32);
    bsW(s, 8 as std::os::raw::c_int,
        ((u >> 16 as std::os::raw::c_int) as std::os::raw::c_long & 0xff as std::os::raw::c_long) as
            UInt32);
    bsW(s, 8 as std::os::raw::c_int,
        ((u >> 8 as std::os::raw::c_int) as std::os::raw::c_long & 0xff as std::os::raw::c_long) as
            UInt32);
    bsW(s, 8 as std::os::raw::c_int,
        (u as std::os::raw::c_long & 0xff as std::os::raw::c_long) as UInt32);
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsPutUChar(mut s: *mut EState, mut c: UChar) {
    bsW(s, 8 as std::os::raw::c_int, c as UInt32);
}
/*---------------------------------------------------*/
/*--- The back end proper                         ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
unsafe extern "C" fn makeMaps_e(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).unseqToSeq[i as usize] = (*s).nInUse as UChar;
            (*s).nInUse += 1
        }
        i += 1
    };
}
/*---------------------------------------------------*/
unsafe extern "C" fn generateMTFValues(mut s: *mut EState) {
    let mut yy: [UChar; 256] = [0; 256];
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut zPend: Int32 = 0;
    let mut wr: Int32 = 0;
    let mut EOB: Int32 = 0;
    /* 
      After sorting (eg, here),
         s->arr1 [ 0 .. s->nblock-1 ] holds sorted order,
         and
         ((UChar*)s->arr2) [ 0 .. s->nblock-1 ] 
         holds the original block data.

      The first thing to do is generate the MTF values,
      and put them in
         ((UInt16*)s->arr1) [ 0 .. s->nblock-1 ].
      Because there are strictly fewer or equal MTF values
      than block values, ptr values in this area are overwritten
      with MTF values only when they are no longer needed.

      The final compressed bitstream is generated into the
      area starting at
         (UChar*) (&((UChar*)s->arr2)[s->nblock])

      These storage aliases are set up in bzCompressInit(),
      except for the last one, which is arranged in 
      compressBlock().
   */
    let mut ptr: *mut UInt32 = (*s).ptr;
    let mut block: *mut UChar = (*s).block;
    let mut mtfv: *mut UInt16 = (*s).mtfv;
    makeMaps_e(s);
    EOB = (*s).nInUse + 1 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i <= EOB { (*s).mtfFreq[i as usize] = 0 as std::os::raw::c_int; i += 1 }
    wr = 0 as std::os::raw::c_int;
    zPend = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < (*s).nInUse { yy[i as usize] = i as UChar; i += 1 }
    i = 0 as std::os::raw::c_int;
    while i < (*s).nblock {
        let mut ll_i: UChar = 0;
        j =
            (*ptr.offset(i as
                             isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                      std::os::raw::c_uint) as Int32;
        if j < 0 as std::os::raw::c_int { j += (*s).nblock }
        ll_i = (*s).unseqToSeq[*block.offset(j as isize) as usize];
        if yy[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == ll_i as std::os::raw::c_int
           {
            zPend += 1
        } else {
            if zPend > 0 as std::os::raw::c_int {
                zPend -= 1;
                while 1 as std::os::raw::c_int as Bool != 0 {
                    if zPend & 1 as std::os::raw::c_int != 0 {
                        *mtfv.offset(wr as isize) =
                            1 as std::os::raw::c_int as UInt16;
                        wr += 1;
                        (*s).mtfFreq[1 as std::os::raw::c_int as usize] += 1
                    } else {
                        *mtfv.offset(wr as isize) =
                            0 as std::os::raw::c_int as UInt16;
                        wr += 1;
                        (*s).mtfFreq[0 as std::os::raw::c_int as usize] += 1
                    }
                    if zPend < 2 as std::os::raw::c_int { break ; }
                    zPend = (zPend - 2 as std::os::raw::c_int) / 2 as std::os::raw::c_int
                }
                zPend = 0 as std::os::raw::c_int
            }
            let mut rtmp: UChar = 0;
            let mut ryy_j: *mut UChar = 0 as *mut UChar;
            let mut rll_i: UChar = 0;
            rtmp = yy[1 as std::os::raw::c_int as usize];
            yy[1 as std::os::raw::c_int as usize] = yy[0 as std::os::raw::c_int as usize];
            ryy_j =
                &mut *yy.as_mut_ptr().offset(1 as std::os::raw::c_int as isize) as
                    *mut UChar;
            rll_i = ll_i;
            while rll_i as std::os::raw::c_int != rtmp as std::os::raw::c_int {
                let mut rtmp2: UChar = 0;
                ryy_j = ryy_j.offset(1);
                rtmp2 = rtmp;
                rtmp = *ryy_j;
                *ryy_j = rtmp2
            }
            yy[0 as std::os::raw::c_int as usize] = rtmp;
            j =
                ryy_j.offset_from(&mut *yy.as_mut_ptr().offset(0 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            isize)
                                               as *mut UChar) as std::os::raw::c_long
                    as Int32;
            *mtfv.offset(wr as isize) = (j + 1 as std::os::raw::c_int) as UInt16;
            wr += 1;
            (*s).mtfFreq[(j + 1 as std::os::raw::c_int) as usize] += 1
        }
        i += 1
    }
    if zPend > 0 as std::os::raw::c_int {
        zPend -= 1;
        while 1 as std::os::raw::c_int as Bool != 0 {
            if zPend & 1 as std::os::raw::c_int != 0 {
                *mtfv.offset(wr as isize) = 1 as std::os::raw::c_int as UInt16;
                wr += 1;
                (*s).mtfFreq[1 as std::os::raw::c_int as usize] += 1
            } else {
                *mtfv.offset(wr as isize) = 0 as std::os::raw::c_int as UInt16;
                wr += 1;
                (*s).mtfFreq[0 as std::os::raw::c_int as usize] += 1
            }
            if zPend < 2 as std::os::raw::c_int { break ; }
            zPend = (zPend - 2 as std::os::raw::c_int) / 2 as std::os::raw::c_int
        }
        zPend = 0 as std::os::raw::c_int
    }
    *mtfv.offset(wr as isize) = EOB as UInt16;
    wr += 1;
    (*s).mtfFreq[EOB as usize] += 1;
    (*s).nMTF = wr;
}
unsafe extern "C" fn sendMTFValues(mut s: *mut EState) {
    let mut v: Int32 = 0;
    let mut t: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut gs: Int32 = 0;
    let mut ge: Int32 = 0;
    let mut totc: Int32 = 0;
    let mut bt: Int32 = 0;
    let mut bc: Int32 = 0;
    let mut iter: Int32 = 0;
    let mut nSelectors: Int32 = 0;
    let mut alphaSize: Int32 = 0;
    let mut minLen: Int32 = 0;
    let mut maxLen: Int32 = 0;
    let mut selCtr: Int32 = 0;
    let mut nGroups: Int32 = 0;
    let mut nBytes: Int32 = 0;
    /*--
   UChar  len [BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   is a global since the decoder also needs it.

   Int32  code[BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   Int32  rfreq[BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   are also globals only used in this proc.
   Made global to keep stack frame size small.
   --*/
    let mut cost: [UInt16; 6] = [0; 6];
    let mut fave: [Int32; 6] = [0; 6];
    let mut mtfv: *mut UInt16 = (*s).mtfv;
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"      %d in block, %d after MTF & 1-2 coding, %d+2 syms in use\n\x00"
                    as *const u8 as *const std::os::raw::c_char, (*s).nblock,
                (*s).nMTF, (*s).nInUse);
    }
    alphaSize = (*s).nInUse + 2 as std::os::raw::c_int;
    t = 0 as std::os::raw::c_int;
    while t < 6 as std::os::raw::c_int {
        v = 0 as std::os::raw::c_int;
        while v < alphaSize {
            (*s).len[t as usize][v as usize] = 15 as std::os::raw::c_int as UChar;
            v += 1
        }
        t += 1
    }
    /*--- Decide how many coding tables to use ---*/
    if !((*s).nMTF > 0 as std::os::raw::c_int) {
        BZ2_bz__AssertH__fail(3001 as std::os::raw::c_int);
    }
    if (*s).nMTF < 200 as std::os::raw::c_int {
        nGroups = 2 as std::os::raw::c_int
    } else if (*s).nMTF < 600 as std::os::raw::c_int {
        nGroups = 3 as std::os::raw::c_int
    } else if (*s).nMTF < 1200 as std::os::raw::c_int {
        nGroups = 4 as std::os::raw::c_int
    } else if (*s).nMTF < 2400 as std::os::raw::c_int {
        nGroups = 5 as std::os::raw::c_int
    } else { nGroups = 6 as std::os::raw::c_int }
    /*--- Generate an initial set of coding tables ---*/
    let mut nPart: Int32 = 0;
    let mut remF: Int32 = 0;
    let mut tFreq: Int32 = 0;
    let mut aFreq: Int32 = 0;
    nPart = nGroups;
    remF = (*s).nMTF;
    gs = 0 as std::os::raw::c_int;
    while nPart > 0 as std::os::raw::c_int {
        tFreq = remF / nPart;
        ge = gs - 1 as std::os::raw::c_int;
        aFreq = 0 as std::os::raw::c_int;
        while aFreq < tFreq && ge < alphaSize - 1 as std::os::raw::c_int {
            ge += 1;
            aFreq += (*s).mtfFreq[ge as usize]
        }
        if ge > gs && nPart != nGroups && nPart != 1 as std::os::raw::c_int &&
               (nGroups - nPart) % 2 as std::os::raw::c_int == 1 as std::os::raw::c_int {
            aFreq -= (*s).mtfFreq[ge as usize];
            ge -= 1
        }
        if (*s).verbosity >= 3 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"      initial group %d, [%d .. %d], has %d syms (%4.1f%%)\n\x00"
                        as *const u8 as *const std::os::raw::c_char, nPart, gs, ge,
                    aFreq,
                    100.0f64 * aFreq as std::os::raw::c_float as std::os::raw::c_double /
                        (*s).nMTF as std::os::raw::c_float as std::os::raw::c_double);
        }
        v = 0 as std::os::raw::c_int;
        while v < alphaSize {
            if v >= gs && v <= ge {
                (*s).len[(nPart - 1 as std::os::raw::c_int) as usize][v as usize] =
                    0 as std::os::raw::c_int as UChar
            } else {
                (*s).len[(nPart - 1 as std::os::raw::c_int) as usize][v as usize] =
                    15 as std::os::raw::c_int as UChar
            }
            v += 1
        }
        nPart -= 1;
        gs = ge + 1 as std::os::raw::c_int;
        remF -= aFreq
    }
    /*--- 
      Iterate up to BZ_N_ITERS times to improve the tables.
   ---*/
    iter = 0 as std::os::raw::c_int;
    while iter < 4 as std::os::raw::c_int {
        t = 0 as std::os::raw::c_int;
        while t < nGroups { fave[t as usize] = 0 as std::os::raw::c_int; t += 1 }
        t = 0 as std::os::raw::c_int;
        while t < nGroups {
            v = 0 as std::os::raw::c_int;
            while v < alphaSize {
                (*s).rfreq[t as usize][v as usize] = 0 as std::os::raw::c_int;
                v += 1
            }
            t += 1
        }
        /*---
        Set up an auxiliary length table which is used to fast-track
	the common case (nGroups == 6). 
      ---*/
        if nGroups == 6 as std::os::raw::c_int {
            v = 0 as std::os::raw::c_int;
            while v < alphaSize {
                (*s).len_pack[v as usize][0 as std::os::raw::c_int as usize] =
                    (((*s).len[1 as std::os::raw::c_int as usize][v as usize] as
                          std::os::raw::c_int) << 16 as std::os::raw::c_int |
                         (*s).len[0 as std::os::raw::c_int as usize][v as usize] as
                             std::os::raw::c_int) as UInt32;
                (*s).len_pack[v as usize][1 as std::os::raw::c_int as usize] =
                    (((*s).len[3 as std::os::raw::c_int as usize][v as usize] as
                          std::os::raw::c_int) << 16 as std::os::raw::c_int |
                         (*s).len[2 as std::os::raw::c_int as usize][v as usize] as
                             std::os::raw::c_int) as UInt32;
                (*s).len_pack[v as usize][2 as std::os::raw::c_int as usize] =
                    (((*s).len[5 as std::os::raw::c_int as usize][v as usize] as
                          std::os::raw::c_int) << 16 as std::os::raw::c_int |
                         (*s).len[4 as std::os::raw::c_int as usize][v as usize] as
                             std::os::raw::c_int) as UInt32;
                v += 1
            }
        }
        nSelectors = 0 as std::os::raw::c_int;
        totc = 0 as std::os::raw::c_int;
        gs = 0 as std::os::raw::c_int;
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*--- Set group start & end marks. --*/
            if gs >= (*s).nMTF { break ; }
            ge = gs + 50 as std::os::raw::c_int - 1 as std::os::raw::c_int;
            if ge >= (*s).nMTF { ge = (*s).nMTF - 1 as std::os::raw::c_int }
            /*-- 
            Calculate the cost of this group as coded
            by each of the coding tables.
         --*/
            t = 0 as std::os::raw::c_int;
            while t < nGroups {
                cost[t as usize] = 0 as std::os::raw::c_int as UInt16;
                t += 1
            }
            if nGroups == 6 as std::os::raw::c_int &&
                   50 as std::os::raw::c_int == ge - gs + 1 as std::os::raw::c_int {
                /*--- fast track the common case ---*/
                let mut cost01: UInt32 = 0;
                let mut cost23: UInt32 = 0;
                let mut cost45: UInt32 = 0;
                let mut icv: UInt16 = 0;
                cost45 = 0 as std::os::raw::c_int as UInt32;
                cost23 = cost45;
                cost01 = cost23;
                icv = *mtfv.offset((gs + 0 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 1 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 2 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 3 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 4 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 5 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 6 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 7 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 8 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 9 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 10 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 11 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 12 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 13 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 14 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 15 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 16 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 17 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 18 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 19 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 20 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 21 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 22 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 23 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 24 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 25 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 26 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 27 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 28 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 29 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 30 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 31 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 32 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 33 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 34 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 35 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 36 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 37 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 38 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 39 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 40 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 41 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 42 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 43 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 44 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 45 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 46 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 47 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 48 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 49 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost[0 as std::os::raw::c_int as usize] =
                    (cost01 & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UInt16;
                cost[1 as std::os::raw::c_int as usize] =
                    (cost01 >> 16 as std::os::raw::c_int) as UInt16;
                cost[2 as std::os::raw::c_int as usize] =
                    (cost23 & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UInt16;
                cost[3 as std::os::raw::c_int as usize] =
                    (cost23 >> 16 as std::os::raw::c_int) as UInt16;
                cost[4 as std::os::raw::c_int as usize] =
                    (cost45 & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UInt16;
                cost[5 as std::os::raw::c_int as usize] =
                    (cost45 >> 16 as std::os::raw::c_int) as UInt16
            } else {
                /*--- slow version which correctly handles all situations ---*/
                i = gs;
                while i <= ge {
                    let mut icv_0: UInt16 = *mtfv.offset(i as isize);
                    t = 0 as std::os::raw::c_int;
                    while t < nGroups {
                        cost[t as usize] =
                            (cost[t as usize] as std::os::raw::c_int +
                                 (*s).len[t as usize][icv_0 as usize] as
                                     std::os::raw::c_int) as UInt16;
                        t += 1
                    }
                    i += 1
                }
            }
            /*-- 
            Find the coding table which is best for this group,
            and record its identity in the selector table.
         --*/
            bc = 999999999 as std::os::raw::c_int;
            bt = -(1 as std::os::raw::c_int);
            t = 0 as std::os::raw::c_int;
            while t < nGroups {
                if (cost[t as usize] as std::os::raw::c_int) < bc {
                    bc = cost[t as usize] as Int32;
                    bt = t
                }
                t += 1
            }
            totc += bc;
            fave[bt as usize] += 1;
            (*s).selector[nSelectors as usize] = bt as UChar;
            nSelectors += 1;
            /*-- 
            Increment the symbol frequencies for the selected table.
          --*/
            if nGroups == 6 as std::os::raw::c_int &&
                   50 as std::os::raw::c_int == ge - gs + 1 as std::os::raw::c_int {
                /*--- fast track the common case ---*/
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 0 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 1 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 2 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 3 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 4 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 5 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 6 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 7 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 8 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 9 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 10 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 11 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 12 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 13 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 14 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 15 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 16 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 17 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 18 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 19 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 20 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 21 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 22 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 23 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 24 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 25 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 26 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 27 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 28 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 29 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 30 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 31 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 32 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 33 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 34 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 35 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 36 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 37 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 38 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 39 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 40 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 41 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 42 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 43 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 44 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 45 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 46 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 47 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 48 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 49 as std::os::raw::c_int) as
                                                       isize) as usize] += 1
            } else {
                /*--- slow version which correctly handles all situations ---*/
                i = gs;
                while i <= ge {
                    (*s).rfreq[bt as usize][*mtfv.offset(i as isize) as usize]
                        += 1;
                    i += 1
                }
            }
            gs = ge + 1 as std::os::raw::c_int
        }
        if (*s).verbosity >= 3 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"      pass %d: size is %d, grp uses are \x00" as
                        *const u8 as *const std::os::raw::c_char,
                    iter + 1 as std::os::raw::c_int, totc / 8 as std::os::raw::c_int);
            t = 0 as std::os::raw::c_int;
            while t < nGroups {
                fprintf(__stderrp,
                        b"%d \x00" as *const u8 as *const std::os::raw::c_char,
                        fave[t as usize]);
                t += 1
            }
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        /*--
        Recompute the tables based on the accumulated frequencies.
      --*/
      /* maxLen was changed from 20 to 17 in bzip2-1.0.3.  See 
         comment in huffman.c for details. */
        t = 0 as std::os::raw::c_int;
        while t < nGroups {
            BZ2_hbMakeCodeLengths(&mut *(*(*s).len.as_mut_ptr().offset(t as
                                                                           isize)).as_mut_ptr().offset(0
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           isize),
                                  &mut *(*(*s).rfreq.as_mut_ptr().offset(t as
                                                                             isize)).as_mut_ptr().offset(0
                                                                                                             as
                                                                                                             std::os::raw::c_int
                                                                                                             as
                                                                                                             isize),
                                  alphaSize, 17 as std::os::raw::c_int);
            t += 1
        }
        iter += 1
    }
    if !(nGroups < 8 as std::os::raw::c_int) {
        BZ2_bz__AssertH__fail(3002 as std::os::raw::c_int);
    }
    if !(nSelectors < 32768 as std::os::raw::c_int &&
             nSelectors <=
                 2 as std::os::raw::c_int + 900000 as std::os::raw::c_int / 50 as std::os::raw::c_int)
       {
        BZ2_bz__AssertH__fail(3003 as std::os::raw::c_int);
    }
    /*--- Compute MTF values for the selectors. ---*/
    let mut pos: [UChar; 6] = [0; 6];
    let mut ll_i: UChar = 0;
    let mut tmp2: UChar = 0;
    let mut tmp: UChar = 0;
    i = 0 as std::os::raw::c_int;
    while i < nGroups { pos[i as usize] = i as UChar; i += 1 }
    i = 0 as std::os::raw::c_int;
    while i < nSelectors {
        ll_i = (*s).selector[i as usize];
        j = 0 as std::os::raw::c_int;
        tmp = pos[j as usize];
        while ll_i as std::os::raw::c_int != tmp as std::os::raw::c_int {
            j += 1;
            tmp2 = tmp;
            tmp = pos[j as usize];
            pos[j as usize] = tmp2
        }
        pos[0 as std::os::raw::c_int as usize] = tmp;
        (*s).selectorMtf[i as usize] = j as UChar;
        i += 1
    }
    /*--- Assign actual codes for the tables. --*/
    t = 0 as std::os::raw::c_int;
    while t < nGroups {
        minLen = 32 as std::os::raw::c_int;
        maxLen = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < alphaSize {
            if (*s).len[t as usize][i as usize] as std::os::raw::c_int > maxLen {
                maxLen = (*s).len[t as usize][i as usize] as Int32
            }
            if ((*s).len[t as usize][i as usize] as std::os::raw::c_int) < minLen {
                minLen = (*s).len[t as usize][i as usize] as Int32
            }
            i += 1
        }
        if maxLen > 17 as std::os::raw::c_int {
            BZ2_bz__AssertH__fail(3004 as std::os::raw::c_int);
        }
        if minLen < 1 as std::os::raw::c_int {
            BZ2_bz__AssertH__fail(3005 as std::os::raw::c_int);
        }
        BZ2_hbAssignCodes(&mut *(*(*s).code.as_mut_ptr().offset(t as
                                                                    isize)).as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    isize),
                          &mut *(*(*s).len.as_mut_ptr().offset(t as
                                                                   isize)).as_mut_ptr().offset(0
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                                   as
                                                                                                   isize),
                          minLen, maxLen, alphaSize);
        t += 1
    }
    /*--- Transmit the mapping table. ---*/
    let mut inUse16: [Bool; 16] = [0; 16];
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        inUse16[i as usize] = 0 as std::os::raw::c_int as Bool;
        j = 0 as std::os::raw::c_int;
        while j < 16 as std::os::raw::c_int {
            if (*s).inUse[(i * 16 as std::os::raw::c_int + j) as usize] != 0 {
                inUse16[i as usize] = 1 as std::os::raw::c_int as Bool
            }
            j += 1
        }
        i += 1
    }
    nBytes = (*s).numZ;
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        if inUse16[i as usize] != 0 {
            bsW(s, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int as UInt32);
        } else { bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32); }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        if inUse16[i as usize] != 0 {
            j = 0 as std::os::raw::c_int;
            while j < 16 as std::os::raw::c_int {
                if (*s).inUse[(i * 16 as std::os::raw::c_int + j) as usize] != 0 {
                    bsW(s, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int as UInt32);
                } else {
                    bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
                }
                j += 1
            }
        }
        i += 1
    }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"      bytes: mapping %d, \x00" as *const u8 as
                    *const std::os::raw::c_char, (*s).numZ - nBytes);
    }
    /*--- Now the selectors. ---*/
    nBytes = (*s).numZ;
    bsW(s, 3 as std::os::raw::c_int, nGroups as UInt32);
    bsW(s, 15 as std::os::raw::c_int, nSelectors as UInt32);
    i = 0 as std::os::raw::c_int;
    while i < nSelectors {
        j = 0 as std::os::raw::c_int;
        while j < (*s).selectorMtf[i as usize] as std::os::raw::c_int {
            bsW(s, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int as UInt32);
            j += 1
        }
        bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
        i += 1
    }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"selectors %d, \x00" as *const u8 as *const std::os::raw::c_char,
                (*s).numZ - nBytes);
    }
    /*--- Now the coding tables. ---*/
    nBytes = (*s).numZ;
    t = 0 as std::os::raw::c_int;
    while t < nGroups {
        let mut curr: Int32 =
            (*s).len[t as usize][0 as std::os::raw::c_int as usize] as Int32;
        bsW(s, 5 as std::os::raw::c_int, curr as UInt32);
        i = 0 as std::os::raw::c_int;
        while i < alphaSize {
            while curr < (*s).len[t as usize][i as usize] as std::os::raw::c_int {
                bsW(s, 2 as std::os::raw::c_int, 2 as std::os::raw::c_int as UInt32);
                curr += 1
                /* 10 */
            }
            while curr > (*s).len[t as usize][i as usize] as std::os::raw::c_int {
                bsW(s, 2 as std::os::raw::c_int, 3 as std::os::raw::c_int as UInt32);
                curr -= 1
                /* 11 */
            }
            bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
            i += 1
        }
        t += 1
    }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"code lengths %d, \x00" as *const u8 as *const std::os::raw::c_char,
                (*s).numZ - nBytes);
    }
    /*--- And finally, the block data proper ---*/
    nBytes = (*s).numZ;
    selCtr = 0 as std::os::raw::c_int;
    gs = 0 as std::os::raw::c_int;
    while 1 as std::os::raw::c_int as Bool != 0 {
        if gs >= (*s).nMTF { break ; }
        ge = gs + 50 as std::os::raw::c_int - 1 as std::os::raw::c_int;
        if ge >= (*s).nMTF { ge = (*s).nMTF - 1 as std::os::raw::c_int }
        if !(((*s).selector[selCtr as usize] as std::os::raw::c_int) < nGroups) {
            BZ2_bz__AssertH__fail(3006 as std::os::raw::c_int);
        }
        if nGroups == 6 as std::os::raw::c_int &&
               50 as std::os::raw::c_int == ge - gs + 1 as std::os::raw::c_int {
            /*--- fast track the common case ---*/
            let mut mtfv_i: UInt16 = 0;
            let mut s_len_sel_selCtr: *mut UChar =
                &mut *(*(*s).len.as_mut_ptr().offset(*(*s).selector.as_mut_ptr().offset(selCtr
                                                                                            as
                                                                                            isize)
                                                         as
                                                         isize)).as_mut_ptr().offset(0
                                                                                         as
                                                                                         std::os::raw::c_int
                                                                                         as
                                                                                         isize)
                    as *mut UChar;
            let mut s_code_sel_selCtr: *mut Int32 =
                &mut *(*(*s).code.as_mut_ptr().offset(*(*s).selector.as_mut_ptr().offset(selCtr
                                                                                             as
                                                                                             isize)
                                                          as
                                                          isize)).as_mut_ptr().offset(0
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          isize)
                    as *mut Int32;
            mtfv_i = *mtfv.offset((gs + 0 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 1 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 2 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 3 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 4 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 5 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 6 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 7 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 8 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 9 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 10 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 11 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 12 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 13 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 14 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 15 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 16 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 17 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 18 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 19 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 20 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 21 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 22 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 23 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 24 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 25 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 26 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 27 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 28 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 29 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 30 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 31 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 32 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 33 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 34 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 35 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 36 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 37 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 38 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 39 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 40 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 41 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 42 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 43 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 44 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 45 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 46 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 47 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 48 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 49 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
        } else {
            /*--- slow version which correctly handles all situations ---*/
            i = gs;
            while i <= ge {
                bsW(s,
                    (*s).len[(*s).selector[selCtr as usize] as
                                 usize][*mtfv.offset(i as isize) as usize] as
                        Int32,
                    (*s).code[(*s).selector[selCtr as usize] as
                                  usize][*mtfv.offset(i as isize) as usize] as
                        UInt32);
                i += 1
            }
        }
        gs = ge + 1 as std::os::raw::c_int;
        selCtr += 1
    }
    if !(selCtr == nSelectors) { BZ2_bz__AssertH__fail(3007 as std::os::raw::c_int); }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"codes %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                (*s).numZ - nBytes);
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_compressBlock(mut s: *mut EState,
                                           mut is_last_block: Bool) {
    if (*s).nblock > 0 as std::os::raw::c_int {
        (*s).blockCRC = !(*s).blockCRC;
        (*s).combinedCRC =
            (*s).combinedCRC << 1 as std::os::raw::c_int |
                (*s).combinedCRC >> 31 as std::os::raw::c_int;
        (*s).combinedCRC ^= (*s).blockCRC;
        if (*s).blockNo > 1 as std::os::raw::c_int { (*s).numZ = 0 as std::os::raw::c_int }
        if (*s).verbosity >= 2 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"    block %d: crc = 0x%08x, combined CRC = 0x%08x, size = %d\n\x00"
                        as *const u8 as *const std::os::raw::c_char, (*s).blockNo,
                    (*s).blockCRC, (*s).combinedCRC, (*s).nblock);
        }
        BZ2_blockSort(s);
    }
    (*s).zbits =
        &mut *((*s).arr2 as *mut UChar).offset((*s).nblock as isize) as
            *mut UChar;
    /*-- If this is the first block, create the stream header. --*/
    if (*s).blockNo == 1 as std::os::raw::c_int {
        BZ2_bsInitWrite(s);
        bsPutUChar(s, 0x42 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x5a as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x68 as std::os::raw::c_int as UChar);
        bsPutUChar(s, (0x30 as std::os::raw::c_int + (*s).blockSize100k) as UChar);
    }
    if (*s).nblock > 0 as std::os::raw::c_int {
        bsPutUChar(s, 0x31 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x41 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x59 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x26 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x53 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x59 as std::os::raw::c_int as UChar);
        /*-- Now the block's CRC, so it is in a known place. --*/
        bsPutUInt32(s, (*s).blockCRC);
        /*-- 
         Now a single bit indicating (non-)randomisation. 
         As of version 0.9.5, we use a better sorting algorithm
         which makes randomisation unnecessary.  So always set
         the randomised bit to 'no'.  Of course, the decoder
         still needs to be able to handle randomised blocks
         so as to maintain backwards compatibility with
         older versions of bzip2.
      --*/
        bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
        bsW(s, 24 as std::os::raw::c_int, (*s).origPtr as UInt32);
        generateMTFValues(s);
        sendMTFValues(s);
    }
    /*-- If this is the last block, add the stream trailer. --*/
    if is_last_block != 0 {
        bsPutUChar(s, 0x17 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x72 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x45 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x38 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x50 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x90 as std::os::raw::c_int as UChar);
        bsPutUInt32(s, (*s).combinedCRC);
        if (*s).verbosity >= 2 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"    final combined CRC = 0x%08x\n   \x00" as *const u8
                        as *const std::os::raw::c_char, (*s).combinedCRC);
        }
        bsFinishWrite(s);
    };
}
/*-------------------------------------------------------------*/
/*--- end                                        compress.c ---*/
/*-------------------------------------------------------------*/

}

pub mod crctable {

pub type UInt32 = crate::blocksort::UInt32;
/*-------------------------------------------------------------*/
/*--- Table for doing CRCs                                  ---*/
/*---                                            crctable.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/*--
  I think this is an implementation of the AUTODIN-II,
  Ethernet & FDDI 32-bit CRC standard.  Vaguely derived
  from code by Rob Warnock, in Section 51 of the
  comp.compression FAQ.
--*/
#[no_mangle]
pub static mut BZ2_crc32Table: [UInt32; 256] =
    [0 as std::os::raw::c_long as UInt32, 0x4c11db7 as std::os::raw::c_long as UInt32,
     0x9823b6e as std::os::raw::c_long as UInt32, 0xd4326d9 as std::os::raw::c_long as UInt32,
     0x130476dc as std::os::raw::c_long as UInt32,
     0x17c56b6b as std::os::raw::c_long as UInt32,
     0x1a864db2 as std::os::raw::c_long as UInt32,
     0x1e475005 as std::os::raw::c_long as UInt32,
     0x2608edb8 as std::os::raw::c_long as UInt32,
     0x22c9f00f as std::os::raw::c_long as UInt32,
     0x2f8ad6d6 as std::os::raw::c_long as UInt32,
     0x2b4bcb61 as std::os::raw::c_long as UInt32,
     0x350c9b64 as std::os::raw::c_long as UInt32,
     0x31cd86d3 as std::os::raw::c_long as UInt32,
     0x3c8ea00a as std::os::raw::c_long as UInt32,
     0x384fbdbd as std::os::raw::c_long as UInt32,
     0x4c11db70 as std::os::raw::c_long as UInt32,
     0x48d0c6c7 as std::os::raw::c_long as UInt32,
     0x4593e01e as std::os::raw::c_long as UInt32,
     0x4152fda9 as std::os::raw::c_long as UInt32,
     0x5f15adac as std::os::raw::c_long as UInt32,
     0x5bd4b01b as std::os::raw::c_long as UInt32,
     0x569796c2 as std::os::raw::c_long as UInt32,
     0x52568b75 as std::os::raw::c_long as UInt32,
     0x6a1936c8 as std::os::raw::c_long as UInt32,
     0x6ed82b7f as std::os::raw::c_long as UInt32,
     0x639b0da6 as std::os::raw::c_long as UInt32,
     0x675a1011 as std::os::raw::c_long as UInt32,
     0x791d4014 as std::os::raw::c_long as UInt32,
     0x7ddc5da3 as std::os::raw::c_long as UInt32,
     0x709f7b7a as std::os::raw::c_long as UInt32,
     0x745e66cd as std::os::raw::c_long as UInt32,
     0x9823b6e0 as std::os::raw::c_long as UInt32,
     0x9ce2ab57 as std::os::raw::c_long as UInt32,
     0x91a18d8e as std::os::raw::c_long as UInt32,
     0x95609039 as std::os::raw::c_long as UInt32,
     0x8b27c03c as std::os::raw::c_long as UInt32,
     0x8fe6dd8b as std::os::raw::c_long as UInt32,
     0x82a5fb52 as std::os::raw::c_long as UInt32,
     0x8664e6e5 as std::os::raw::c_long as UInt32,
     0xbe2b5b58 as std::os::raw::c_long as UInt32,
     0xbaea46ef as std::os::raw::c_long as UInt32,
     0xb7a96036 as std::os::raw::c_long as UInt32,
     0xb3687d81 as std::os::raw::c_long as UInt32,
     0xad2f2d84 as std::os::raw::c_long as UInt32,
     0xa9ee3033 as std::os::raw::c_long as UInt32,
     0xa4ad16ea as std::os::raw::c_long as UInt32,
     0xa06c0b5d as std::os::raw::c_long as UInt32,
     0xd4326d90 as std::os::raw::c_long as UInt32,
     0xd0f37027 as std::os::raw::c_long as UInt32,
     0xddb056fe as std::os::raw::c_long as UInt32,
     0xd9714b49 as std::os::raw::c_long as UInt32,
     0xc7361b4c as std::os::raw::c_long as UInt32,
     0xc3f706fb as std::os::raw::c_long as UInt32,
     0xceb42022 as std::os::raw::c_long as UInt32,
     0xca753d95 as std::os::raw::c_long as UInt32,
     0xf23a8028 as std::os::raw::c_long as UInt32,
     0xf6fb9d9f as std::os::raw::c_long as UInt32,
     0xfbb8bb46 as std::os::raw::c_long as UInt32,
     0xff79a6f1 as std::os::raw::c_long as UInt32,
     0xe13ef6f4 as std::os::raw::c_long as UInt32,
     0xe5ffeb43 as std::os::raw::c_long as UInt32,
     0xe8bccd9a as std::os::raw::c_long as UInt32,
     0xec7dd02d as std::os::raw::c_long as UInt32,
     0x34867077 as std::os::raw::c_long as UInt32,
     0x30476dc0 as std::os::raw::c_long as UInt32,
     0x3d044b19 as std::os::raw::c_long as UInt32,
     0x39c556ae as std::os::raw::c_long as UInt32,
     0x278206ab as std::os::raw::c_long as UInt32,
     0x23431b1c as std::os::raw::c_long as UInt32,
     0x2e003dc5 as std::os::raw::c_long as UInt32,
     0x2ac12072 as std::os::raw::c_long as UInt32,
     0x128e9dcf as std::os::raw::c_long as UInt32,
     0x164f8078 as std::os::raw::c_long as UInt32,
     0x1b0ca6a1 as std::os::raw::c_long as UInt32,
     0x1fcdbb16 as std::os::raw::c_long as UInt32,
     0x18aeb13 as std::os::raw::c_long as UInt32, 0x54bf6a4 as std::os::raw::c_long as UInt32,
     0x808d07d as std::os::raw::c_long as UInt32, 0xcc9cdca as std::os::raw::c_long as UInt32,
     0x7897ab07 as std::os::raw::c_long as UInt32,
     0x7c56b6b0 as std::os::raw::c_long as UInt32,
     0x71159069 as std::os::raw::c_long as UInt32,
     0x75d48dde as std::os::raw::c_long as UInt32,
     0x6b93dddb as std::os::raw::c_long as UInt32,
     0x6f52c06c as std::os::raw::c_long as UInt32,
     0x6211e6b5 as std::os::raw::c_long as UInt32,
     0x66d0fb02 as std::os::raw::c_long as UInt32,
     0x5e9f46bf as std::os::raw::c_long as UInt32,
     0x5a5e5b08 as std::os::raw::c_long as UInt32,
     0x571d7dd1 as std::os::raw::c_long as UInt32,
     0x53dc6066 as std::os::raw::c_long as UInt32,
     0x4d9b3063 as std::os::raw::c_long as UInt32,
     0x495a2dd4 as std::os::raw::c_long as UInt32,
     0x44190b0d as std::os::raw::c_long as UInt32,
     0x40d816ba as std::os::raw::c_long as UInt32,
     0xaca5c697 as std::os::raw::c_long as UInt32,
     0xa864db20 as std::os::raw::c_long as UInt32,
     0xa527fdf9 as std::os::raw::c_long as UInt32,
     0xa1e6e04e as std::os::raw::c_long as UInt32,
     0xbfa1b04b as std::os::raw::c_long as UInt32,
     0xbb60adfc as std::os::raw::c_long as UInt32,
     0xb6238b25 as std::os::raw::c_long as UInt32,
     0xb2e29692 as std::os::raw::c_long as UInt32,
     0x8aad2b2f as std::os::raw::c_long as UInt32,
     0x8e6c3698 as std::os::raw::c_long as UInt32,
     0x832f1041 as std::os::raw::c_long as UInt32,
     0x87ee0df6 as std::os::raw::c_long as UInt32,
     0x99a95df3 as std::os::raw::c_long as UInt32,
     0x9d684044 as std::os::raw::c_long as UInt32,
     0x902b669d as std::os::raw::c_long as UInt32,
     0x94ea7b2a as std::os::raw::c_long as UInt32,
     0xe0b41de7 as std::os::raw::c_long as UInt32,
     0xe4750050 as std::os::raw::c_long as UInt32,
     0xe9362689 as std::os::raw::c_long as UInt32,
     0xedf73b3e as std::os::raw::c_long as UInt32,
     0xf3b06b3b as std::os::raw::c_long as UInt32,
     0xf771768c as std::os::raw::c_long as UInt32,
     0xfa325055 as std::os::raw::c_long as UInt32,
     0xfef34de2 as std::os::raw::c_long as UInt32,
     0xc6bcf05f as std::os::raw::c_long as UInt32,
     0xc27dede8 as std::os::raw::c_long as UInt32,
     0xcf3ecb31 as std::os::raw::c_long as UInt32,
     0xcbffd686 as std::os::raw::c_long as UInt32,
     0xd5b88683 as std::os::raw::c_long as UInt32,
     0xd1799b34 as std::os::raw::c_long as UInt32,
     0xdc3abded as std::os::raw::c_long as UInt32,
     0xd8fba05a as std::os::raw::c_long as UInt32,
     0x690ce0ee as std::os::raw::c_long as UInt32,
     0x6dcdfd59 as std::os::raw::c_long as UInt32,
     0x608edb80 as std::os::raw::c_long as UInt32,
     0x644fc637 as std::os::raw::c_long as UInt32,
     0x7a089632 as std::os::raw::c_long as UInt32,
     0x7ec98b85 as std::os::raw::c_long as UInt32,
     0x738aad5c as std::os::raw::c_long as UInt32,
     0x774bb0eb as std::os::raw::c_long as UInt32,
     0x4f040d56 as std::os::raw::c_long as UInt32,
     0x4bc510e1 as std::os::raw::c_long as UInt32,
     0x46863638 as std::os::raw::c_long as UInt32,
     0x42472b8f as std::os::raw::c_long as UInt32,
     0x5c007b8a as std::os::raw::c_long as UInt32,
     0x58c1663d as std::os::raw::c_long as UInt32,
     0x558240e4 as std::os::raw::c_long as UInt32,
     0x51435d53 as std::os::raw::c_long as UInt32,
     0x251d3b9e as std::os::raw::c_long as UInt32,
     0x21dc2629 as std::os::raw::c_long as UInt32,
     0x2c9f00f0 as std::os::raw::c_long as UInt32,
     0x285e1d47 as std::os::raw::c_long as UInt32,
     0x36194d42 as std::os::raw::c_long as UInt32,
     0x32d850f5 as std::os::raw::c_long as UInt32,
     0x3f9b762c as std::os::raw::c_long as UInt32,
     0x3b5a6b9b as std::os::raw::c_long as UInt32,
     0x315d626 as std::os::raw::c_long as UInt32, 0x7d4cb91 as std::os::raw::c_long as UInt32,
     0xa97ed48 as std::os::raw::c_long as UInt32, 0xe56f0ff as std::os::raw::c_long as UInt32,
     0x1011a0fa as std::os::raw::c_long as UInt32,
     0x14d0bd4d as std::os::raw::c_long as UInt32,
     0x19939b94 as std::os::raw::c_long as UInt32,
     0x1d528623 as std::os::raw::c_long as UInt32,
     0xf12f560e as std::os::raw::c_long as UInt32,
     0xf5ee4bb9 as std::os::raw::c_long as UInt32,
     0xf8ad6d60 as std::os::raw::c_long as UInt32,
     0xfc6c70d7 as std::os::raw::c_long as UInt32,
     0xe22b20d2 as std::os::raw::c_long as UInt32,
     0xe6ea3d65 as std::os::raw::c_long as UInt32,
     0xeba91bbc as std::os::raw::c_long as UInt32,
     0xef68060b as std::os::raw::c_long as UInt32,
     0xd727bbb6 as std::os::raw::c_long as UInt32,
     0xd3e6a601 as std::os::raw::c_long as UInt32,
     0xdea580d8 as std::os::raw::c_long as UInt32,
     0xda649d6f as std::os::raw::c_long as UInt32,
     0xc423cd6a as std::os::raw::c_long as UInt32,
     0xc0e2d0dd as std::os::raw::c_long as UInt32,
     0xcda1f604 as std::os::raw::c_long as UInt32,
     0xc960ebb3 as std::os::raw::c_long as UInt32,
     0xbd3e8d7e as std::os::raw::c_long as UInt32,
     0xb9ff90c9 as std::os::raw::c_long as UInt32,
     0xb4bcb610 as std::os::raw::c_long as UInt32,
     0xb07daba7 as std::os::raw::c_long as UInt32,
     0xae3afba2 as std::os::raw::c_long as UInt32,
     0xaafbe615 as std::os::raw::c_long as UInt32,
     0xa7b8c0cc as std::os::raw::c_long as UInt32,
     0xa379dd7b as std::os::raw::c_long as UInt32,
     0x9b3660c6 as std::os::raw::c_long as UInt32,
     0x9ff77d71 as std::os::raw::c_long as UInt32,
     0x92b45ba8 as std::os::raw::c_long as UInt32,
     0x9675461f as std::os::raw::c_long as UInt32,
     0x8832161a as std::os::raw::c_long as UInt32,
     0x8cf30bad as std::os::raw::c_long as UInt32,
     0x81b02d74 as std::os::raw::c_long as UInt32,
     0x857130c3 as std::os::raw::c_long as UInt32,
     0x5d8a9099 as std::os::raw::c_long as UInt32,
     0x594b8d2e as std::os::raw::c_long as UInt32,
     0x5408abf7 as std::os::raw::c_long as UInt32,
     0x50c9b640 as std::os::raw::c_long as UInt32,
     0x4e8ee645 as std::os::raw::c_long as UInt32,
     0x4a4ffbf2 as std::os::raw::c_long as UInt32,
     0x470cdd2b as std::os::raw::c_long as UInt32,
     0x43cdc09c as std::os::raw::c_long as UInt32,
     0x7b827d21 as std::os::raw::c_long as UInt32,
     0x7f436096 as std::os::raw::c_long as UInt32,
     0x7200464f as std::os::raw::c_long as UInt32,
     0x76c15bf8 as std::os::raw::c_long as UInt32,
     0x68860bfd as std::os::raw::c_long as UInt32,
     0x6c47164a as std::os::raw::c_long as UInt32,
     0x61043093 as std::os::raw::c_long as UInt32,
     0x65c52d24 as std::os::raw::c_long as UInt32,
     0x119b4be9 as std::os::raw::c_long as UInt32,
     0x155a565e as std::os::raw::c_long as UInt32,
     0x18197087 as std::os::raw::c_long as UInt32,
     0x1cd86d30 as std::os::raw::c_long as UInt32,
     0x29f3d35 as std::os::raw::c_long as UInt32, 0x65e2082 as std::os::raw::c_long as UInt32,
     0xb1d065b as std::os::raw::c_long as UInt32, 0xfdc1bec as std::os::raw::c_long as UInt32,
     0x3793a651 as std::os::raw::c_long as UInt32,
     0x3352bbe6 as std::os::raw::c_long as UInt32,
     0x3e119d3f as std::os::raw::c_long as UInt32,
     0x3ad08088 as std::os::raw::c_long as UInt32,
     0x2497d08d as std::os::raw::c_long as UInt32,
     0x2056cd3a as std::os::raw::c_long as UInt32,
     0x2d15ebe3 as std::os::raw::c_long as UInt32,
     0x29d4f654 as std::os::raw::c_long as UInt32,
     0xc5a92679 as std::os::raw::c_long as UInt32,
     0xc1683bce as std::os::raw::c_long as UInt32,
     0xcc2b1d17 as std::os::raw::c_long as UInt32,
     0xc8ea00a0 as std::os::raw::c_long as UInt32,
     0xd6ad50a5 as std::os::raw::c_long as UInt32,
     0xd26c4d12 as std::os::raw::c_long as UInt32,
     0xdf2f6bcb as std::os::raw::c_long as UInt32,
     0xdbee767c as std::os::raw::c_long as UInt32,
     0xe3a1cbc1 as std::os::raw::c_long as UInt32,
     0xe760d676 as std::os::raw::c_long as UInt32,
     0xea23f0af as std::os::raw::c_long as UInt32,
     0xeee2ed18 as std::os::raw::c_long as UInt32,
     0xf0a5bd1d as std::os::raw::c_long as UInt32,
     0xf464a0aa as std::os::raw::c_long as UInt32,
     0xf9278673 as std::os::raw::c_long as UInt32,
     0xfde69bc4 as std::os::raw::c_long as UInt32,
     0x89b8fd09 as std::os::raw::c_long as UInt32,
     0x8d79e0be as std::os::raw::c_long as UInt32,
     0x803ac667 as std::os::raw::c_long as UInt32,
     0x84fbdbd0 as std::os::raw::c_long as UInt32,
     0x9abc8bd5 as std::os::raw::c_long as UInt32,
     0x9e7d9662 as std::os::raw::c_long as UInt32,
     0x933eb0bb as std::os::raw::c_long as UInt32,
     0x97ffad0c as std::os::raw::c_long as UInt32,
     0xafb010b1 as std::os::raw::c_long as UInt32,
     0xab710d06 as std::os::raw::c_long as UInt32,
     0xa6322bdf as std::os::raw::c_long as UInt32,
     0xa2f33668 as std::os::raw::c_long as UInt32,
     0xbcb4666d as std::os::raw::c_long as UInt32,
     0xb8757bda as std::os::raw::c_long as UInt32,
     0xb5365d03 as std::os::raw::c_long as UInt32,
     0xb1f740b4 as std::os::raw::c_long as UInt32];
/*-------------------------------------------------------------*/
/*--- end                                        crctable.c ---*/
/*-------------------------------------------------------------*/

}

pub mod decompress {

extern "C" {
    
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    #[no_mangle]
    static mut BZ2_rNums: [Int32; 512];
    
    
    
    
}
pub use crate::bzlib::BZ2_bz__AssertH__fail;
pub use crate::bzlib::BZ2_indexIntoF;
pub use crate::huffman::BZ2_hbCreateDecodeTables;
pub use crate::blocksort::__sFILEX;
pub type __int64_t = crate::blocksort::__int64_t;
pub type __darwin_off_t = crate::blocksort::__darwin_off_t;
pub type fpos_t = crate::blocksort::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::blocksort::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::blocksort::__sFILE;
pub type FILE = crate::blocksort::FILE;
// #[derive(Copy, Clone)]

pub type bz_stream = crate::blocksort::bz_stream;
pub type Bool = crate::blocksort::Bool;
pub type UChar = crate::blocksort::UChar;
pub type Int32 = crate::blocksort::Int32;
pub type UInt32 = crate::blocksort::UInt32;
pub type UInt16 = crate::blocksort::UInt16;
// #[derive(Copy, Clone)]

pub type DState = crate::bzlib::DState;
/*-------------------------------------------------------------*/
/*--- Decompression machinery                               ---*/
/*---                                          decompress.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/*---------------------------------------------------*/
unsafe extern "C" fn makeMaps_d(mut s: *mut DState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).seqToUnseq[(*s).nInUse as usize] = i as UChar;
            (*s).nInUse += 1
        }
        i += 1
    };
}
/*---------------------------------------------------*/
/*---------------------------------------------------*/
/* the longest code */
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_decompress(mut s: *mut DState) -> Int32 {
    let mut current_block: u64;
    let mut uc: UChar = 0;
    let mut retVal: Int32 = 0;
    let mut minLen: Int32 = 0;
    let mut maxLen: Int32 = 0;
    let mut strm: *mut bz_stream = (*s).strm;
    /* stuff that needs to be saved/restored */
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut t: Int32 = 0;
    let mut alphaSize: Int32 = 0;
    let mut nGroups: Int32 = 0;
    let mut nSelectors: Int32 = 0;
    let mut EOB: Int32 = 0;
    let mut groupNo: Int32 = 0;
    let mut groupPos: Int32 = 0;
    let mut nextSym: Int32 = 0;
    let mut nblockMAX: Int32 = 0;
    let mut nblock: Int32 = 0;
    let mut es: Int32 = 0;
    let mut N: Int32 = 0;
    let mut curr: Int32 = 0;
    let mut zt: Int32 = 0;
    let mut zn: Int32 = 0;
    let mut zvec: Int32 = 0;
    let mut zj: Int32 = 0;
    let mut gSel: Int32 = 0;
    let mut gMinlen: Int32 = 0;
    let mut gLimit: *mut Int32 = 0 as *mut Int32;
    let mut gBase: *mut Int32 = 0 as *mut Int32;
    let mut gPerm: *mut Int32 = 0 as *mut Int32;
    if (*s).state == 10 as std::os::raw::c_int {
        /*initialise the save area*/
        (*s).save_i = 0 as std::os::raw::c_int;
        (*s).save_j = 0 as std::os::raw::c_int;
        (*s).save_t = 0 as std::os::raw::c_int;
        (*s).save_alphaSize = 0 as std::os::raw::c_int;
        (*s).save_nGroups = 0 as std::os::raw::c_int;
        (*s).save_nSelectors = 0 as std::os::raw::c_int;
        (*s).save_EOB = 0 as std::os::raw::c_int;
        (*s).save_groupNo = 0 as std::os::raw::c_int;
        (*s).save_groupPos = 0 as std::os::raw::c_int;
        (*s).save_nextSym = 0 as std::os::raw::c_int;
        (*s).save_nblockMAX = 0 as std::os::raw::c_int;
        (*s).save_nblock = 0 as std::os::raw::c_int;
        (*s).save_es = 0 as std::os::raw::c_int;
        (*s).save_N = 0 as std::os::raw::c_int;
        (*s).save_curr = 0 as std::os::raw::c_int;
        (*s).save_zt = 0 as std::os::raw::c_int;
        (*s).save_zn = 0 as std::os::raw::c_int;
        (*s).save_zvec = 0 as std::os::raw::c_int;
        (*s).save_zj = 0 as std::os::raw::c_int;
        (*s).save_gSel = 0 as std::os::raw::c_int;
        (*s).save_gMinlen = 0 as std::os::raw::c_int;
        (*s).save_gLimit = 0 as *mut Int32;
        (*s).save_gBase = 0 as *mut Int32;
        (*s).save_gPerm = 0 as *mut Int32
    }
    /*restore from the save area*/
    i = (*s).save_i;
    j = (*s).save_j;
    t = (*s).save_t;
    alphaSize = (*s).save_alphaSize;
    nGroups = (*s).save_nGroups;
    nSelectors = (*s).save_nSelectors;
    EOB = (*s).save_EOB;
    groupNo = (*s).save_groupNo;
    groupPos = (*s).save_groupPos;
    nextSym = (*s).save_nextSym;
    nblockMAX = (*s).save_nblockMAX;
    nblock = (*s).save_nblock;
    es = (*s).save_es;
    N = (*s).save_N;
    curr = (*s).save_curr;
    zt = (*s).save_zt;
    zn = (*s).save_zn;
    zvec = (*s).save_zvec;
    zj = (*s).save_zj;
    gSel = (*s).save_gSel;
    gMinlen = (*s).save_gMinlen;
    gLimit = (*s).save_gLimit;
    gBase = (*s).save_gBase;
    gPerm = (*s).save_gPerm;
    retVal = 0 as std::os::raw::c_int;
    match (*s).state {
        10 => {
            (*s).state = 10 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 5658374378798827547;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v: UInt32 = 0;
                    v =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v as UChar;
                    current_block = 5658374378798827547;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x42 as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 12259750428863723923; }
                }
            }
        }
        11 => { current_block = 12259750428863723923; }
        12 => { current_block = 15146946972525368609; }
        13 => { current_block = 13504760517129887221; }
        14 => { current_block = 7948568793456312728; }
        15 => { current_block = 16703841960874917807; }
        16 => { current_block = 12231332282017165356; }
        17 => { current_block = 7286555771988341860; }
        18 => { current_block = 6656868271313165664; }
        19 => { current_block = 15902903523132075486; }
        20 => { current_block = 16204949703499709801; }
        21 => { current_block = 5505795673017046993; }
        22 => { current_block = 14563596112884461881; }
        23 => { current_block = 12051594319698232578; }
        24 => { current_block = 14315698657705028467; }
        25 => { current_block = 640681092829779800; }
        26 => { current_block = 588075840077989673; }
        27 => { current_block = 34749046854646975; }
        28 => { current_block = 16487873541482693172; }
        29 => { current_block = 1422779171932145779; }
        30 => { current_block = 3906616468301123675; }
        31 => { current_block = 5769007513321684282; }
        32 => { current_block = 4874723077730206021; }
        33 => { current_block = 10945178116989557996; }
        34 => { current_block = 1736021991379636935; }
        35 => { current_block = 5008197131544113214; }
        36 => { current_block = 16722720626876144162; }
        37 => { current_block = 14744029255125744966; }
        38 => { current_block = 5374617794059532979; }
        39 => { current_block = 13999925517074022731; }
        40 => { current_block = 2629672494974161066; }
        41 => { current_block = 1050378859040334210; }
        42 => { current_block = 10200488719709598753; }
        43 => { current_block = 9864403379770423142; }
        44 => { current_block = 8489059574810375089; }
        45 => { current_block = 12998570369541158573; }
        46 => { current_block = 10541196509243133637; }
        47 => { current_block = 8760950161942609538; }
        48 => { current_block = 3131443096645543054; }
        49 => { current_block = 1975408140333322065; }
        50 => { current_block = 15818179691129344165; }
        _ => {
            if 0 as std::os::raw::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4001 as std::os::raw::c_int);
            }
            if 0 as std::os::raw::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4002 as std::os::raw::c_int);
            }
            current_block = 15885526978618306830;
        }
    }
    match current_block {
        12259750428863723923 => {
            (*s).state = 11 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 1658462350791934405;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_0: UInt32 = 0;
                    v_0 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_0 as UChar;
                    current_block = 1658462350791934405;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x5a as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 15146946972525368609; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        15146946972525368609 => {
            (*s).state = 12 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 16314074004867283505;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_1: UInt32 = 0;
                    v_1 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_1 as UChar;
                    current_block = 16314074004867283505;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x68 as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 13504760517129887221; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        13504760517129887221 => {
            (*s).state = 13 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 1915186496383530739;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_2: UInt32 = 0;
                    v_2 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    (*s).blockSize100k = v_2 as Int32;
                    current_block = 1915186496383530739;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if (*s).blockSize100k <
                           0x30 as std::os::raw::c_int + 1 as std::os::raw::c_int ||
                           (*s).blockSize100k >
                               0x30 as std::os::raw::c_int + 9 as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        (*s).blockSize100k -= 0x30 as std::os::raw::c_int;
                        if (*s).smallDecompress != 0 {
                            (*s).ll16 =
                                (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                                                    (((*s).blockSize100k
                                                                                          *
                                                                                          100000
                                                                                              as
                                                                                              std::os::raw::c_int)
                                                                                         as
                                                                                         std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt16>()
                                                                                                                         as
                                                                                                                         std::os::raw::c_ulong)
                                                                                        as
                                                                                        std::os::raw::c_int,
                                                                                    1
                                                                                        as
                                                                                        std::os::raw::c_int)
                                    as *mut UInt16;
                            (*s).ll4 =
                                (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                                                    ((1
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          +
                                                                                          (*s).blockSize100k
                                                                                              *
                                                                                              100000
                                                                                                  as
                                                                                                  std::os::raw::c_int
                                                                                          >>
                                                                                          1
                                                                                              as
                                                                                              std::os::raw::c_int)
                                                                                         as
                                                                                         std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UChar>()
                                                                                                                         as
                                                                                                                         std::os::raw::c_ulong)
                                                                                        as
                                                                                        std::os::raw::c_int,
                                                                                    1
                                                                                        as
                                                                                        std::os::raw::c_int)
                                    as *mut UChar;
                            if (*s).ll16.is_null() || (*s).ll4.is_null() {
                                retVal = -(3 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                            } else { current_block = 7948568793456312728; }
                        } else {
                            (*s).tt =
                                (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                                                    (((*s).blockSize100k
                                                                                          *
                                                                                          100000
                                                                                              as
                                                                                              std::os::raw::c_int)
                                                                                         as
                                                                                         std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<Int32>()
                                                                                                                         as
                                                                                                                         std::os::raw::c_ulong)
                                                                                        as
                                                                                        std::os::raw::c_int,
                                                                                    1
                                                                                        as
                                                                                        std::os::raw::c_int)
                                    as *mut UInt32;
                            if (*s).tt.is_null() {
                                retVal = -(3 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                            } else { current_block = 7948568793456312728; }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    match current_block {
        7948568793456312728 => {
            (*s).state = 14 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 9846950269610550213;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_3: UInt32 = 0;
                    v_3 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_3 as UChar;
                    current_block = 9846950269610550213;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int == 0x17 as std::os::raw::c_int {
                        current_block = 10200488719709598753;
                    } else if uc as std::os::raw::c_int != 0x31 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 16703841960874917807; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        10200488719709598753 => {
            (*s).state = 42 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 13262463590990658200;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_32: UInt32 = 0;
                    v_32 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_32 as UChar;
                    current_block = 13262463590990658200;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x72 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 9864403379770423142; }
                }
            }
        }
        16703841960874917807 => {
            (*s).state = 15 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 3569141194949357899;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_4: UInt32 = 0;
                    v_4 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_4 as UChar;
                    current_block = 3569141194949357899;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x41 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 12231332282017165356; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        9864403379770423142 => {
            (*s).state = 43 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 10756506701594629759;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_33: UInt32 = 0;
                    v_33 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_33 as UChar;
                    current_block = 10756506701594629759;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x45 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 8489059574810375089; }
                }
            }
        }
        12231332282017165356 => {
            (*s).state = 16 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 16517180880614114163;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_5: UInt32 = 0;
                    v_5 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_5 as UChar;
                    current_block = 16517180880614114163;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x59 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 7286555771988341860; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        8489059574810375089 => {
            (*s).state = 44 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 9819403752380335018;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_34: UInt32 = 0;
                    v_34 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_34 as UChar;
                    current_block = 9819403752380335018;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x38 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 12998570369541158573; }
                }
            }
        }
        7286555771988341860 => {
            (*s).state = 17 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 2606663910910355487;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_6: UInt32 = 0;
                    v_6 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_6 as UChar;
                    current_block = 2606663910910355487;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x26 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 6656868271313165664; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        12998570369541158573 => {
            (*s).state = 45 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 9454797012561717444;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_35: UInt32 = 0;
                    v_35 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_35 as UChar;
                    current_block = 9454797012561717444;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x50 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 10541196509243133637; }
                }
            }
        }
        6656868271313165664 => {
            (*s).state = 18 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 8125779086361653720;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_7: UInt32 = 0;
                    v_7 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_7 as UChar;
                    current_block = 8125779086361653720;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x53 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 15902903523132075486; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        10541196509243133637 => {
            (*s).state = 46 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 724777313732190959;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_36: UInt32 = 0;
                    v_36 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_36 as UChar;
                    current_block = 724777313732190959;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x90 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        (*s).storedCombinedCRC = 0 as std::os::raw::c_int as UInt32;
                        current_block = 8760950161942609538;
                    }
                }
            }
        }
        15902903523132075486 => {
            (*s).state = 19 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 958128786106592581;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_8: UInt32 = 0;
                    v_8 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_8 as UChar;
                    current_block = 958128786106592581;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x59 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        (*s).currBlockNo += 1;
                        if (*s).verbosity >= 2 as std::os::raw::c_int {
                            fprintf(__stderrp,
                                    b"\n    [%d: huff+mtf \x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    (*s).currBlockNo);
                        }
                        (*s).storedBlockCRC = 0 as std::os::raw::c_int as UInt32;
                        current_block = 16204949703499709801;
                    }
                }
            }
        }
        _ => { }
    }
    match current_block {
        8760950161942609538 => {
            (*s).state = 47 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 14486187473704332379;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_37: UInt32 = 0;
                    v_37 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_37 as UChar;
                    current_block = 14486187473704332379;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 3131443096645543054;
                }
            }
        }
        16204949703499709801 => {
            (*s).state = 20 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 3790734079518302164;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_9: UInt32 = 0;
                    v_9 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_9 as UChar;
                    current_block = 3790734079518302164;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 5505795673017046993;
                }
            }
        }
        _ => { }
    }
    match current_block {
        3131443096645543054 => {
            (*s).state = 48 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 3659807904093622879;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_38: UInt32 = 0;
                    v_38 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_38 as UChar;
                    current_block = 3659807904093622879;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 1975408140333322065;
                }
            }
        }
        5505795673017046993 => {
            (*s).state = 21 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 16711521214030637000;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_10: UInt32 = 0;
                    v_10 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_10 as UChar;
                    current_block = 16711521214030637000;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 14563596112884461881;
                }
            }
        }
        _ => { }
    }
    match current_block {
        1975408140333322065 => {
            (*s).state = 49 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 2394045633138979148;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_39: UInt32 = 0;
                    v_39 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_39 as UChar;
                    current_block = 2394045633138979148;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 15818179691129344165;
                }
            }
        }
        14563596112884461881 => {
            (*s).state = 22 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 17870985093275900527;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_11: UInt32 = 0;
                    v_11 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_11 as UChar;
                    current_block = 17870985093275900527;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 12051594319698232578;
                }
            }
        }
        _ => { }
    }
    match current_block {
        12051594319698232578 => {
            (*s).state = 23 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 13734492969709581318;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_12: UInt32 = 0;
                    v_12 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_12 as UChar;
                    current_block = 13734492969709581318;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 14315698657705028467;
                }
            }
        }
        15818179691129344165 => {
            (*s).state = 50 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 1904329045571868869;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_40: UInt32 = 0;
                    v_40 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_40 as UChar;
                    current_block = 1904329045571868869;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    (*s).state = 1 as std::os::raw::c_int;
                    retVal = 4 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                }
            }
        }
        _ => { }
    }
    match current_block {
        14315698657705028467 => {
            (*s).state = 24 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 15030729790988239748;
                    break ;
                }
                if (*s).bsLive >= 1 as std::os::raw::c_int {
                    let mut v_13: UInt32 = 0;
                    v_13 =
                        (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 1 as std::os::raw::c_int;
                    (*s).blockRandomised = v_13 as Bool;
                    current_block = 15030729790988239748;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr = 0 as std::os::raw::c_int;
                    current_block = 640681092829779800;
                }
            }
        }
        _ => { }
    }
    match current_block {
        640681092829779800 => {
            (*s).state = 25 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 8260322496947496197;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_14: UInt32 = 0;
                    v_14 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_14 as UChar;
                    current_block = 8260322496947496197;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr =
                        (*s).origPtr << 8 as std::os::raw::c_int | uc as Int32;
                    current_block = 588075840077989673;
                }
            }
        }
        _ => { }
    }
    match current_block {
        588075840077989673 => {
            (*s).state = 26 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 5561851013817067674;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_15: UInt32 = 0;
                    v_15 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_15 as UChar;
                    current_block = 5561851013817067674;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr =
                        (*s).origPtr << 8 as std::os::raw::c_int | uc as Int32;
                    current_block = 34749046854646975;
                }
            }
        }
        _ => { }
    }
    match current_block {
        34749046854646975 => {
            (*s).state = 27 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 10471999855724930313;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_16: UInt32 = 0;
                    v_16 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_16 as UChar;
                    current_block = 10471999855724930313;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr =
                        (*s).origPtr << 8 as std::os::raw::c_int | uc as Int32;
                    if (*s).origPtr < 0 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else if (*s).origPtr >
                                  10 as std::os::raw::c_int +
                                      100000 as std::os::raw::c_int *
                                          (*s).blockSize100k {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        /*--- Receive the mapping table ---*/
                        i = 0 as std::os::raw::c_int;
                        current_block = 17262312153619709241;
                    }
                }
            }
        }
        _ => { }
    }
    'c_10532:
        loop  {
            match current_block {
                15885526978618306830 => { (*s).save_i = i; break ; }
                2629672494974161066 => {
                    (*s).state = 40 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_30: UInt32 = 0;
                            v_30 =
                                (*s).bsBuff >> (*s).bsLive - zn &
                                    (((1 as std::os::raw::c_int) << zn) -
                                         1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= zn;
                            zvec = v_30 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    current_block = 9078889872071895942;
                }
                13999925517074022731 => {
                    (*s).state = 39 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_29: UInt32 = 0;
                            v_29 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            zj = v_29 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    zvec = zvec << 1 as std::os::raw::c_int | zj;
                    current_block = 13605767259572914371;
                }
                5374617794059532979 => {
                    (*s).state = 38 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_28: UInt32 = 0;
                            v_28 =
                                (*s).bsBuff >> (*s).bsLive - zn &
                                    (((1 as std::os::raw::c_int) << zn) -
                                         1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= zn;
                            zvec = v_28 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    current_block = 13605767259572914371;
                }
                14744029255125744966 => {
                    (*s).state = 37 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_27: UInt32 = 0;
                            v_27 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            zj = v_27 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    zvec = zvec << 1 as std::os::raw::c_int | zj;
                    current_block = 1550405138573481750;
                }
                16722720626876144162 => {
                    (*s).state = 36 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_26: UInt32 = 0;
                            v_26 =
                                (*s).bsBuff >> (*s).bsLive - zn &
                                    (((1 as std::os::raw::c_int) << zn) -
                                         1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= zn;
                            zvec = v_26 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    current_block = 1550405138573481750;
                }
                5008197131544113214 => {
                    (*s).state = 35 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_25: UInt32 = 0;
                            v_25 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_25 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        curr += 1
                    } else { curr -= 1 }
                    current_block = 11858046780433112516;
                }
                1736021991379636935 => {
                    (*s).state = 34 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_24: UInt32 = 0;
                            v_24 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_24 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if !(uc as std::os::raw::c_int == 0 as std::os::raw::c_int) {
                        current_block = 5008197131544113214;
                        continue ;
                    }
                    current_block = 17503523010989424999;
                }
                10945178116989557996 => {
                    (*s).state = 33 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 5 as std::os::raw::c_int {
                            let mut v_23: UInt32 = 0;
                            v_23 =
                                (*s).bsBuff >> (*s).bsLive - 5 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 5 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 5 as std::os::raw::c_int;
                            curr = v_23 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    i = 0 as std::os::raw::c_int;
                    current_block = 3770765986603902964;
                }
                4874723077730206021 => {
                    (*s).state = 32 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_21: UInt32 = 0;
                            v_21 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_21 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        current_block = 5281038271658253520;
                    } else {
                        j += 1;
                        if j >= nGroups {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else { current_block = 6927328446518169316; }
                    }
                }
                5769007513321684282 => {
                    (*s).state = 31 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 15 as std::os::raw::c_int {
                            let mut v_20: UInt32 = 0;
                            v_20 =
                                (*s).bsBuff >> (*s).bsLive - 15 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 15 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 15 as std::os::raw::c_int;
                            nSelectors = v_20 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if nSelectors < 1 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else { i = 0 as std::os::raw::c_int }
                    current_block = 6591141407893725683;
                }
                3906616468301123675 =>
                /*--- Now the selectors ---*/
                {
                    (*s).state = 30 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 3 as std::os::raw::c_int {
                            let mut v_19: UInt32 = 0;
                            v_19 =
                                (*s).bsBuff >> (*s).bsLive - 3 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 3 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 3 as std::os::raw::c_int;
                            nGroups = v_19 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if !(nGroups < 2 as std::os::raw::c_int ||
                             nGroups > 6 as std::os::raw::c_int) {
                        current_block = 5769007513321684282;
                        continue ;
                    }
                    retVal = -(4 as std::os::raw::c_int);
                    current_block = 15885526978618306830;
                    continue ;
                }
                1422779171932145779 => {
                    (*s).state = 29 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_18: UInt32 = 0;
                            v_18 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_18 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 1 as std::os::raw::c_int {
                        (*s).inUse[(i * 16 as std::os::raw::c_int + j) as usize] =
                            1 as std::os::raw::c_int as Bool
                    }
                    j += 1;
                    current_block = 3854024847017804838;
                }
                17262312153619709241 => {
                    if i < 16 as std::os::raw::c_int {
                        current_block = 16487873541482693172;
                        continue ;
                    }
                    i = 0 as std::os::raw::c_int;
                    while i < 256 as std::os::raw::c_int {
                        (*s).inUse[i as usize] = 0 as std::os::raw::c_int as Bool;
                        i += 1
                    }
                    i = 0 as std::os::raw::c_int;
                    current_block = 3472349144349095221;
                }
                16487873541482693172 => {
                    (*s).state = 28 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_17: UInt32 = 0;
                            v_17 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_17 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 1 as std::os::raw::c_int {
                        (*s).inUse16[i as usize] = 1 as std::os::raw::c_int as Bool
                    } else {
                        (*s).inUse16[i as usize] = 0 as std::os::raw::c_int as Bool
                    }
                    i += 1;
                    current_block = 17262312153619709241;
                    continue ;
                }
                _ => {
                    (*s).state = 41 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_31: UInt32 = 0;
                            v_31 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            zj = v_31 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    zvec = zvec << 1 as std::os::raw::c_int | zj;
                    current_block = 9078889872071895942;
                }
            }
            match current_block {
                9078889872071895942 => {
                    if zn > 20 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) <
                               0 as std::os::raw::c_int ||
                               zvec - *gBase.offset(zn as isize) >=
                                   258 as std::os::raw::c_int {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            nextSym =
                                *gPerm.offset((zvec -
                                                   *gBase.offset(zn as isize))
                                                  as isize)
                        }
                    } else {
                        zn += 1;
                        current_block = 1050378859040334210;
                        continue ;
                    }
                    current_block = 15093386068129942558;
                }
                13605767259572914371 => {
                    if zn > 20 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) <
                               0 as std::os::raw::c_int ||
                               zvec - *gBase.offset(zn as isize) >=
                                   258 as std::os::raw::c_int {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            nextSym =
                                *gPerm.offset((zvec -
                                                   *gBase.offset(zn as isize))
                                                  as isize);
                            if nextSym == 0 as std::os::raw::c_int ||
                                   nextSym == 1 as std::os::raw::c_int {
                                current_block = 4550729491376650574;
                            } else {
                                es += 1;
                                uc =
                                    (*s).seqToUnseq[(*s).mtfa[(*s).mtfbase[0
                                                                               as
                                                                               std::os::raw::c_int
                                                                               as
                                                                               usize]
                                                                  as usize] as
                                                        usize];
                                (*s).unzftab[uc as usize] += es;
                                if (*s).smallDecompress != 0 {
                                    while es > 0 as std::os::raw::c_int {
                                        if nblock >= nblockMAX {
                                            retVal = -(4 as std::os::raw::c_int);
                                            current_block =
                                                15885526978618306830;
                                            continue 'c_10532 ;
                                        } else {
                                            *(*s).ll16.offset(nblock as isize)
                                                = uc as UInt16;
                                            nblock += 1;
                                            es -= 1
                                        }
                                    }
                                } else {
                                    while es > 0 as std::os::raw::c_int {
                                        if nblock >= nblockMAX {
                                            retVal = -(4 as std::os::raw::c_int);
                                            current_block =
                                                15885526978618306830;
                                            continue 'c_10532 ;
                                        } else {
                                            *(*s).tt.offset(nblock as isize) =
                                                uc as UInt32;
                                            nblock += 1;
                                            es -= 1
                                        }
                                    }
                                }
                                current_block = 15093386068129942558;
                            }
                        }
                    } else {
                        zn += 1;
                        current_block = 13999925517074022731;
                        continue ;
                    }
                }
                1550405138573481750 => {
                    if zn > 20 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) <
                               0 as std::os::raw::c_int ||
                               zvec - *gBase.offset(zn as isize) >=
                                   258 as std::os::raw::c_int {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            nextSym =
                                *gPerm.offset((zvec -
                                                   *gBase.offset(zn as isize))
                                                  as isize)
                        }
                    } else {
                        zn += 1;
                        current_block = 14744029255125744966;
                        continue ;
                    }
                    current_block = 15093386068129942558;
                }
                _ => { }
            }
            match current_block {
                15093386068129942558 => {
                    if 1 as std::os::raw::c_int as Bool != 0 {
                        if nextSym == EOB {
                            current_block = 12118509005321596519;
                        } else {
                            if nextSym == 0 as std::os::raw::c_int ||
                                   nextSym == 1 as std::os::raw::c_int {
                                es = -(1 as std::os::raw::c_int);
                                N = 1 as std::os::raw::c_int
                            } else if nblock >= nblockMAX {
                                retVal = -(4 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                                continue ;
                            } else {
                                /*-- uc = MTF ( nextSym-1 ) --*/
                                let mut ii_0: Int32 = 0;
                                let mut jj_0: Int32 = 0;
                                let mut kk_0: Int32 = 0;
                                let mut pp: Int32 = 0;
                                let mut lno: Int32 = 0;
                                let mut off: Int32 = 0;
                                let mut nn: UInt32 = 0;
                                nn = (nextSym - 1 as std::os::raw::c_int) as UInt32;
                                if nn < 16 as std::os::raw::c_int as std::os::raw::c_uint {
                                    /* avoid general-case expense */
                                    pp =
                                        (*s).mtfbase[0 as std::os::raw::c_int as
                                                         usize];
                                    uc =
                                        (*s).mtfa[(pp as
                                                       std::os::raw::c_uint).wrapping_add(nn)
                                                      as usize];
                                    while nn >
                                              3 as std::os::raw::c_int as std::os::raw::c_uint
                                          {
                                        let mut z: Int32 =
                                            (pp as
                                                 std::os::raw::c_uint).wrapping_add(nn)
                                                as Int32;
                                        (*s).mtfa[z as usize] =
                                            (*s).mtfa[(z - 1 as std::os::raw::c_int)
                                                          as usize];
                                        (*s).mtfa[(z - 1 as std::os::raw::c_int) as
                                                      usize] =
                                            (*s).mtfa[(z - 2 as std::os::raw::c_int)
                                                          as usize];
                                        (*s).mtfa[(z - 2 as std::os::raw::c_int) as
                                                      usize] =
                                            (*s).mtfa[(z - 3 as std::os::raw::c_int)
                                                          as usize];
                                        (*s).mtfa[(z - 3 as std::os::raw::c_int) as
                                                      usize] =
                                            (*s).mtfa[(z - 4 as std::os::raw::c_int)
                                                          as usize];
                                        nn =
                                            (nn as
                                                 std::os::raw::c_uint).wrapping_sub(4
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_uint)
                                                as UInt32 as UInt32
                                    }
                                    while nn >
                                              0 as std::os::raw::c_int as std::os::raw::c_uint
                                          {
                                        (*s).mtfa[(pp as
                                                       std::os::raw::c_uint).wrapping_add(nn)
                                                      as usize] =
                                            (*s).mtfa[(pp as
                                                           std::os::raw::c_uint).wrapping_add(nn).wrapping_sub(1
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           std::os::raw::c_uint)
                                                          as usize];
                                        nn = nn.wrapping_sub(1)
                                    }
                                    (*s).mtfa[pp as usize] = uc
                                } else {
                                    /* general case */
                                    lno =
                                        nn.wrapping_div(16 as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                                            Int32;
                                    off =
                                        nn.wrapping_rem(16 as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                                            Int32;
                                    pp = (*s).mtfbase[lno as usize] + off;
                                    uc = (*s).mtfa[pp as usize];
                                    while pp > (*s).mtfbase[lno as usize] {
                                        (*s).mtfa[pp as usize] =
                                            (*s).mtfa[(pp - 1 as std::os::raw::c_int)
                                                          as usize];
                                        pp -= 1
                                    }
                                    (*s).mtfbase[lno as usize] += 1;
                                    while lno > 0 as std::os::raw::c_int {
                                        (*s).mtfbase[lno as usize] -= 1;
                                        (*s).mtfa[(*s).mtfbase[lno as usize]
                                                      as usize] =
                                            (*s).mtfa[((*s).mtfbase[(lno -
                                                                         1 as
                                                                             std::os::raw::c_int)
                                                                        as
                                                                        usize]
                                                           + 16 as std::os::raw::c_int
                                                           - 1 as std::os::raw::c_int)
                                                          as usize];
                                        lno -= 1
                                    }
                                    (*s).mtfbase[0 as std::os::raw::c_int as usize] -=
                                        1;
                                    (*s).mtfa[(*s).mtfbase[0 as std::os::raw::c_int as
                                                               usize] as
                                                  usize] = uc;
                                    if (*s).mtfbase[0 as std::os::raw::c_int as usize]
                                           == 0 as std::os::raw::c_int {
                                        kk_0 =
                                            4096 as std::os::raw::c_int -
                                                1 as std::os::raw::c_int;
                                        ii_0 =
                                            256 as std::os::raw::c_int /
                                                16 as std::os::raw::c_int -
                                                1 as std::os::raw::c_int;
                                        while ii_0 >= 0 as std::os::raw::c_int {
                                            jj_0 =
                                                16 as std::os::raw::c_int -
                                                    1 as std::os::raw::c_int;
                                            while jj_0 >= 0 as std::os::raw::c_int {
                                                (*s).mtfa[kk_0 as usize] =
                                                    (*s).mtfa[((*s).mtfbase[ii_0
                                                                                as
                                                                                usize]
                                                                   + jj_0) as
                                                                  usize];
                                                kk_0 -= 1;
                                                jj_0 -= 1
                                            }
                                            (*s).mtfbase[ii_0 as usize] =
                                                kk_0 + 1 as std::os::raw::c_int;
                                            ii_0 -= 1
                                        }
                                    }
                                }
                                /*-- end uc = MTF ( nextSym-1 ) --*/
                                (*s).unzftab[(*s).seqToUnseq[uc as usize] as
                                                 usize] += 1;
                                if (*s).smallDecompress != 0 {
                                    *(*s).ll16.offset(nblock as isize) =
                                        (*s).seqToUnseq[uc as usize] as UInt16
                                } else {
                                    *(*s).tt.offset(nblock as isize) =
                                        (*s).seqToUnseq[uc as usize] as UInt32
                                }
                                nblock += 1;
                                if groupPos == 0 as std::os::raw::c_int {
                                    groupNo += 1;
                                    if groupNo >= nSelectors {
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue ;
                                    } else {
                                        groupPos = 50 as std::os::raw::c_int;
                                        gSel =
                                            (*s).selector[groupNo as usize] as
                                                Int32;
                                        gMinlen = (*s).minLens[gSel as usize];
                                        gLimit =
                                            &mut *(*(*s).limit.as_mut_ptr().offset(gSel
                                                                                       as
                                                                                       isize)).as_mut_ptr().offset(0
                                                                                                                       as
                                                                                                                       std::os::raw::c_int
                                                                                                                       as
                                                                                                                       isize)
                                                as *mut Int32;
                                        gPerm =
                                            &mut *(*(*s).perm.as_mut_ptr().offset(gSel
                                                                                      as
                                                                                      isize)).as_mut_ptr().offset(0
                                                                                                                      as
                                                                                                                      std::os::raw::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                as *mut Int32;
                                        gBase =
                                            &mut *(*(*s).base.as_mut_ptr().offset(gSel
                                                                                      as
                                                                                      isize)).as_mut_ptr().offset(0
                                                                                                                      as
                                                                                                                      std::os::raw::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                as *mut Int32
                                    }
                                }
                                groupPos -= 1;
                                zn = gMinlen;
                                current_block = 2629672494974161066;
                                continue ;
                            }
                            current_block = 4550729491376650574;
                        }
                    } else { current_block = 12118509005321596519; }
                    match current_block {
                        4550729491376650574 => { }
                        _ =>
                        /* Now we know what nblock is, we can do a better sanity
         check on s->origPtr.
      */
                        {
                            if (*s).origPtr < 0 as std::os::raw::c_int ||
                                   (*s).origPtr >= nblock {
                                retVal = -(4 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                                continue ;
                            } else {
                                /*-- Set up cftab to facilitate generation of T^(-1) --*/
      /* Check: unzftab entries in range. */
                                i = 0 as std::os::raw::c_int;
                                while i <= 255 as std::os::raw::c_int {
                                    if (*s).unzftab[i as usize] <
                                           0 as std::os::raw::c_int ||
                                           (*s).unzftab[i as usize] > nblock {
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue 'c_10532 ;
                                    } else { i += 1 }
                                }
                                /* Actually generate cftab. */
                                (*s).cftab[0 as std::os::raw::c_int as usize] =
                                    0 as std::os::raw::c_int;
                                i = 1 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    (*s).cftab[i as usize] =
                                        (*s).unzftab[(i - 1 as std::os::raw::c_int) as
                                                         usize];
                                    i += 1
                                }
                                i = 1 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    (*s).cftab[i as usize] +=
                                        (*s).cftab[(i - 1 as std::os::raw::c_int) as
                                                       usize];
                                    i += 1
                                }
                                /* Check: cftab entries in range. */
                                i = 0 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    if (*s).cftab[i as usize] <
                                           0 as std::os::raw::c_int ||
                                           (*s).cftab[i as usize] > nblock {
                                        /* s->cftab[i] can legitimately be == nblock */
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue 'c_10532 ;
                                    } else { i += 1 }
                                }
                                /* Check: cftab entries non-descending. */
                                i = 1 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    if (*s).cftab[(i - 1 as std::os::raw::c_int) as
                                                      usize] >
                                           (*s).cftab[i as usize] {
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue 'c_10532 ;
                                    } else { i += 1 }
                                }
                                (*s).state_out_len = 0 as std::os::raw::c_int;
                                (*s).state_out_ch = 0 as std::os::raw::c_int as UChar;
                                (*s).calculatedBlockCRC =
                                    0xffffffff as std::os::raw::c_long as UInt32;
                                (*s).state = 2 as std::os::raw::c_int;
                                if (*s).verbosity >= 2 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"rt+rld\x00" as *const u8 as
                                                *const std::os::raw::c_char);
                                }
                                if (*s).smallDecompress != 0 {
                                    /*-- Make a copy of cftab, used in generation of T --*/
                                    i = 0 as std::os::raw::c_int;
                                    while i <= 256 as std::os::raw::c_int {
                                        (*s).cftabCopy[i as usize] =
                                            (*s).cftab[i as usize];
                                        i += 1
                                    }
                                    /*-- compute the T vector --*/
                                    i = 0 as std::os::raw::c_int;
                                    while i < nblock {
                                        uc =
                                            *(*s).ll16.offset(i as isize) as
                                                UChar;
                                        *(*s).ll16.offset(i as isize) =
                                            ((*s).cftabCopy[uc as usize] &
                                                 0xffff as std::os::raw::c_int) as
                                                UInt16;
                                        if i & 0x1 as std::os::raw::c_int ==
                                               0 as std::os::raw::c_int {
                                            *(*s).ll4.offset((i >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((i >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf0 as std::os::raw::c_int |
                                                     (*s).cftabCopy[uc as
                                                                        usize]
                                                         >> 16 as std::os::raw::c_int)
                                                    as UChar
                                        } else {
                                            *(*s).ll4.offset((i >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((i >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf as std::os::raw::c_int |
                                                     ((*s).cftabCopy[uc as
                                                                         usize]
                                                          >>
                                                          16 as std::os::raw::c_int)
                                                         << 4 as std::os::raw::c_int)
                                                    as UChar
                                        }
                                        (*s).cftabCopy[uc as usize] += 1;
                                        i += 1
                                    }
                                    /*-- Compute T^(-1) by pointer reversal on T --*/
                                    i = (*s).origPtr;
                                    j =
                                        (*(*s).ll16.offset(i as isize) as
                                             UInt32 |
                                             (*(*s).ll4.offset((i >>
                                                                    1 as
                                                                        std::os::raw::c_int)
                                                                   as isize)
                                                  as UInt32 >>
                                                  (i << 2 as std::os::raw::c_int &
                                                       0x4 as std::os::raw::c_int) &
                                                  0xf as std::os::raw::c_int as
                                                      std::os::raw::c_uint) <<
                                                 16 as std::os::raw::c_int) as Int32;
                                    loop  {
                                        let mut tmp_0: Int32 =
                                            (*(*s).ll16.offset(j as isize) as
                                                 UInt32 |
                                                 (*(*s).ll4.offset((j >>
                                                                        1 as
                                                                            std::os::raw::c_int)
                                                                       as
                                                                       isize)
                                                      as UInt32 >>
                                                      (j << 2 as std::os::raw::c_int &
                                                           0x4 as std::os::raw::c_int)
                                                      &
                                                      0xf as std::os::raw::c_int as
                                                          std::os::raw::c_uint) <<
                                                     16 as std::os::raw::c_int) as
                                                Int32;
                                        *(*s).ll16.offset(j as isize) =
                                            (i & 0xffff as std::os::raw::c_int) as
                                                UInt16;
                                        if j & 0x1 as std::os::raw::c_int ==
                                               0 as std::os::raw::c_int {
                                            *(*s).ll4.offset((j >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((j >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf0 as std::os::raw::c_int |
                                                     i >> 16 as std::os::raw::c_int)
                                                    as UChar
                                        } else {
                                            *(*s).ll4.offset((j >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((j >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf as std::os::raw::c_int |
                                                     (i >> 16 as std::os::raw::c_int)
                                                         << 4 as std::os::raw::c_int)
                                                    as UChar
                                        }
                                        i = j;
                                        j = tmp_0;
                                        if !(i != (*s).origPtr) { break ; }
                                    }
                                    (*s).tPos = (*s).origPtr as UInt32;
                                    (*s).nblock_used = 0 as std::os::raw::c_int;
                                    if (*s).blockRandomised != 0 {
                                        (*s).rNToGo = 0 as std::os::raw::c_int;
                                        (*s).rTPos = 0 as std::os::raw::c_int;
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).k0 =
                                            BZ2_indexIntoF((*s).tPos as Int32,
                                                           (*s).cftab.as_mut_ptr());
                                        (*s).tPos =
                                            *(*s).ll16.offset((*s).tPos as
                                                                  isize) as
                                                UInt32 |
                                                (*(*s).ll4.offset(((*s).tPos
                                                                       >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as UInt32 >>
                                                     ((*s).tPos <<
                                                          2 as std::os::raw::c_int &
                                                          0x4 as std::os::raw::c_int
                                                              as std::os::raw::c_uint)
                                                     &
                                                     0xf as std::os::raw::c_int as
                                                         std::os::raw::c_uint) <<
                                                    16 as std::os::raw::c_int;
                                        (*s).nblock_used += 1;
                                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                                            (*s).rNToGo =
                                                BZ2_rNums[(*s).rTPos as
                                                              usize];
                                            (*s).rTPos += 1;
                                            if (*s).rTPos ==
                                                   512 as std::os::raw::c_int {
                                                (*s).rTPos = 0 as std::os::raw::c_int
                                            }
                                        }
                                        (*s).rNToGo -= 1;
                                        (*s).k0 ^=
                                            if (*s).rNToGo == 1 as std::os::raw::c_int
                                               {
                                                1 as std::os::raw::c_int
                                            } else { 0 as std::os::raw::c_int }
                                    } else {
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).k0 =
                                            BZ2_indexIntoF((*s).tPos as Int32,
                                                           (*s).cftab.as_mut_ptr());
                                        (*s).tPos =
                                            *(*s).ll16.offset((*s).tPos as
                                                                  isize) as
                                                UInt32 |
                                                (*(*s).ll4.offset(((*s).tPos
                                                                       >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as UInt32 >>
                                                     ((*s).tPos <<
                                                          2 as std::os::raw::c_int &
                                                          0x4 as std::os::raw::c_int
                                                              as std::os::raw::c_uint)
                                                     &
                                                     0xf as std::os::raw::c_int as
                                                         std::os::raw::c_uint) <<
                                                    16 as std::os::raw::c_int;
                                        (*s).nblock_used += 1
                                    }
                                } else {
                                    /*-- compute the T^(-1) vector --*/
                                    i = 0 as std::os::raw::c_int;
                                    while i < nblock {
                                        uc =
                                            (*(*s).tt.offset(i as isize) &
                                                 0xff as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as UChar;
                                        let ref mut fresh0 =
                                            *(*s).tt.offset((*s).cftab[uc as
                                                                           usize]
                                                                as isize);
                                        *fresh0 |=
                                            (i << 8 as std::os::raw::c_int) as
                                                std::os::raw::c_uint;
                                        (*s).cftab[uc as usize] += 1;
                                        i += 1
                                    }
                                    (*s).tPos =
                                        *(*s).tt.offset((*s).origPtr as isize)
                                            >> 8 as std::os::raw::c_int;
                                    (*s).nblock_used = 0 as std::os::raw::c_int;
                                    if (*s).blockRandomised != 0 {
                                        (*s).rNToGo = 0 as std::os::raw::c_int;
                                        (*s).rTPos = 0 as std::os::raw::c_int;
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).tPos =
                                            *(*s).tt.offset((*s).tPos as
                                                                isize);
                                        (*s).k0 =
                                            ((*s).tPos &
                                                 0xff as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as UChar as
                                                Int32;
                                        (*s).tPos >>= 8 as std::os::raw::c_int;
                                        (*s).nblock_used += 1;
                                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                                            (*s).rNToGo =
                                                BZ2_rNums[(*s).rTPos as
                                                              usize];
                                            (*s).rTPos += 1;
                                            if (*s).rTPos ==
                                                   512 as std::os::raw::c_int {
                                                (*s).rTPos = 0 as std::os::raw::c_int
                                            }
                                        }
                                        (*s).rNToGo -= 1;
                                        (*s).k0 ^=
                                            if (*s).rNToGo == 1 as std::os::raw::c_int
                                               {
                                                1 as std::os::raw::c_int
                                            } else { 0 as std::os::raw::c_int }
                                    } else {
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).tPos =
                                            *(*s).tt.offset((*s).tPos as
                                                                isize);
                                        (*s).k0 =
                                            ((*s).tPos &
                                                 0xff as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as UChar as
                                                Int32;
                                        (*s).tPos >>= 8 as std::os::raw::c_int;
                                        (*s).nblock_used += 1
                                    }
                                }
                                retVal = 0 as std::os::raw::c_int;
                                current_block = 15885526978618306830;
                                continue ;
                            }
                        }
                    }
                }
                _ => { }
            }
            match current_block {
                4550729491376650574 =>
                /* Check that N doesn't get too big, so that es doesn't
                  go negative.  The maximum value that can be
                  RUNA/RUNB encoded is equal to the block size (post
                  the initial RLE), viz, 900k, so bounding N at 2
                  million should guard against overflow without
                  rejecting any legitimate inputs. */
                {
                    if N >=
                           2 as std::os::raw::c_int * 1024 as std::os::raw::c_int *
                               1024 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else {
                        if nextSym == 0 as std::os::raw::c_int {
                            es =
                                es + (0 as std::os::raw::c_int + 1 as std::os::raw::c_int) * N
                        } else if nextSym == 1 as std::os::raw::c_int {
                            es =
                                es + (1 as std::os::raw::c_int + 1 as std::os::raw::c_int) * N
                        }
                        N = N * 2 as std::os::raw::c_int;
                        if groupPos == 0 as std::os::raw::c_int {
                            groupNo += 1;
                            if groupNo >= nSelectors {
                                retVal = -(4 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                                continue ;
                            } else {
                                groupPos = 50 as std::os::raw::c_int;
                                gSel =
                                    (*s).selector[groupNo as usize] as Int32;
                                gMinlen = (*s).minLens[gSel as usize];
                                gLimit =
                                    &mut *(*(*s).limit.as_mut_ptr().offset(gSel
                                                                               as
                                                                               isize)).as_mut_ptr().offset(0
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               isize)
                                        as *mut Int32;
                                gPerm =
                                    &mut *(*(*s).perm.as_mut_ptr().offset(gSel
                                                                              as
                                                                              isize)).as_mut_ptr().offset(0
                                                                                                              as
                                                                                                              std::os::raw::c_int
                                                                                                              as
                                                                                                              isize)
                                        as *mut Int32;
                                gBase =
                                    &mut *(*(*s).base.as_mut_ptr().offset(gSel
                                                                              as
                                                                              isize)).as_mut_ptr().offset(0
                                                                                                              as
                                                                                                              std::os::raw::c_int
                                                                                                              as
                                                                                                              isize)
                                        as *mut Int32
                            }
                        }
                        groupPos -= 1;
                        zn = gMinlen;
                        current_block = 5374617794059532979;
                        continue ;
                    }
                }
                _ => { }
            }
            loop  {
                match current_block {
                    3854024847017804838 => {
                        if j < 16 as std::os::raw::c_int {
                            current_block = 1422779171932145779;
                            continue 'c_10532 ;
                        }
                    }
                    6591141407893725683 => {
                        if i < nSelectors {
                            j = 0 as std::os::raw::c_int;
                            current_block = 6927328446518169316;
                            continue ;
                        } else {
                            if nSelectors >
                                   2 as std::os::raw::c_int +
                                       900000 as std::os::raw::c_int /
                                           50 as std::os::raw::c_int {
                                nSelectors =
                                    2 as std::os::raw::c_int +
                                        900000 as std::os::raw::c_int /
                                            50 as std::os::raw::c_int
                            }
                            /*--- Undo the MTF values for the selectors. ---*/
                            let mut pos: [UChar; 6] = [0; 6];
                            let mut tmp: UChar = 0;
                            let mut v_22: UChar = 0;
                            v_22 = 0 as std::os::raw::c_int as UChar;
                            while (v_22 as std::os::raw::c_int) < nGroups {
                                pos[v_22 as usize] = v_22;
                                v_22 = v_22.wrapping_add(1)
                            }
                            i = 0 as std::os::raw::c_int;
                            while i < nSelectors {
                                v_22 = (*s).selectorMtf[i as usize];
                                tmp = pos[v_22 as usize];
                                while v_22 as std::os::raw::c_int > 0 as std::os::raw::c_int {
                                    pos[v_22 as usize] =
                                        pos[(v_22 as std::os::raw::c_int -
                                                 1 as std::os::raw::c_int) as usize];
                                    v_22 = v_22.wrapping_sub(1)
                                }
                                pos[0 as std::os::raw::c_int as usize] = tmp;
                                (*s).selector[i as usize] = tmp;
                                i += 1
                            }
                            /*--- Now the coding tables ---*/
                            t = 0 as std::os::raw::c_int;
                            current_block = 16916874950763617094;
                            break ;
                        }
                    }
                    3472349144349095221 => {
                        if i < 16 as std::os::raw::c_int {
                            if (*s).inUse16[i as usize] != 0 {
                                j = 0 as std::os::raw::c_int;
                                current_block = 3854024847017804838;
                                continue ;
                            }
                        } else {
                            makeMaps_d(s);
                            if (*s).nInUse == 0 as std::os::raw::c_int {
                                current_block = 11906008669688594715;
                                break ;
                            } else {
                                current_block = 7606051654693192361;
                                break ;
                            }
                        }
                    }
                    17503523010989424999 => {
                        (*s).len[t as usize][i as usize] = curr as UChar;
                        i += 1;
                        current_block = 3770765986603902964;
                        continue ;
                    }
                    3770765986603902964 => {
                        if i < alphaSize {
                            current_block = 11858046780433112516;
                            continue ;
                        }
                        t += 1;
                        current_block = 16916874950763617094;
                        break ;
                    }
                    5281038271658253520 => {
                        /* Having more than BZ_MAX_SELECTORS doesn't make much sense
            since they will never be used, but some implementations might
            "round up" the number of selectors, so just ignore those. */
                        if i <
                               2 as std::os::raw::c_int +
                                   900000 as std::os::raw::c_int / 50 as std::os::raw::c_int {
                            (*s).selectorMtf[i as usize] = j as UChar
                        }
                        i += 1;
                        current_block = 6591141407893725683;
                        continue ;
                    }
                    6927328446518169316 => {
                        if 1 as std::os::raw::c_int as Bool != 0 {
                            current_block = 4874723077730206021;
                            continue 'c_10532 ;
                        } else {
                            current_block = 5281038271658253520;
                            continue ;
                        }
                    }
                    _ => {
                        if !(1 as std::os::raw::c_int as Bool != 0) {
                            current_block = 17503523010989424999;
                            continue ;
                        }
                        if !(curr < 1 as std::os::raw::c_int ||
                                 curr > 20 as std::os::raw::c_int) {
                            current_block = 1736021991379636935;
                            continue 'c_10532 ;
                        }
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue 'c_10532 ;
                    }
                }
                i += 1;
                current_block = 3472349144349095221;
            }
            match current_block {
                7606051654693192361 => {
                    alphaSize = (*s).nInUse + 2 as std::os::raw::c_int;
                    current_block = 3906616468301123675;
                }
                11906008669688594715 => {
                    retVal = -(4 as std::os::raw::c_int);
                    current_block = 15885526978618306830;
                }
                _ => {
                    if t < nGroups {
                        current_block = 10945178116989557996;
                        continue ;
                    }
                    /*--- Create the Huffman decoding tables ---*/
                    t = 0 as std::os::raw::c_int;
                    while t < nGroups {
                        minLen = 32 as std::os::raw::c_int;
                        maxLen = 0 as std::os::raw::c_int;
                        i = 0 as std::os::raw::c_int;
                        while i < alphaSize {
                            if (*s).len[t as usize][i as usize] as std::os::raw::c_int
                                   > maxLen {
                                maxLen =
                                    (*s).len[t as usize][i as usize] as Int32
                            }
                            if ((*s).len[t as usize][i as usize] as
                                    std::os::raw::c_int) < minLen {
                                minLen =
                                    (*s).len[t as usize][i as usize] as Int32
                            }
                            i += 1
                        }
                        BZ2_hbCreateDecodeTables(&mut *(*(*s).limit.as_mut_ptr().offset(t
                                                                                            as
                                                                                            isize)).as_mut_ptr().offset(0
                                                                                                                            as
                                                                                                                            std::os::raw::c_int
                                                                                                                            as
                                                                                                                            isize),
                                                 &mut *(*(*s).base.as_mut_ptr().offset(t
                                                                                           as
                                                                                           isize)).as_mut_ptr().offset(0
                                                                                                                           as
                                                                                                                           std::os::raw::c_int
                                                                                                                           as
                                                                                                                           isize),
                                                 &mut *(*(*s).perm.as_mut_ptr().offset(t
                                                                                           as
                                                                                           isize)).as_mut_ptr().offset(0
                                                                                                                           as
                                                                                                                           std::os::raw::c_int
                                                                                                                           as
                                                                                                                           isize),
                                                 &mut *(*(*s).len.as_mut_ptr().offset(t
                                                                                          as
                                                                                          isize)).as_mut_ptr().offset(0
                                                                                                                          as
                                                                                                                          std::os::raw::c_int
                                                                                                                          as
                                                                                                                          isize),
                                                 minLen, maxLen, alphaSize);
                        (*s).minLens[t as usize] = minLen;
                        t += 1
                    }
                    /*--- Now the MTF values ---*/
                    EOB = (*s).nInUse + 1 as std::os::raw::c_int;
                    nblockMAX = 100000 as std::os::raw::c_int * (*s).blockSize100k;
                    groupNo = -(1 as std::os::raw::c_int);
                    groupPos = 0 as std::os::raw::c_int;
                    i = 0 as std::os::raw::c_int;
                    while i <= 255 as std::os::raw::c_int {
                        (*s).unzftab[i as usize] = 0 as std::os::raw::c_int;
                        i += 1
                    }
                    /*-- MTF init --*/
                    let mut ii: Int32 = 0;
                    let mut jj: Int32 = 0;
                    let mut kk: Int32 = 0;
                    kk = 4096 as std::os::raw::c_int - 1 as std::os::raw::c_int;
                    ii =
                        256 as std::os::raw::c_int / 16 as std::os::raw::c_int -
                            1 as std::os::raw::c_int;
                    while ii >= 0 as std::os::raw::c_int {
                        jj = 16 as std::os::raw::c_int - 1 as std::os::raw::c_int;
                        while jj >= 0 as std::os::raw::c_int {
                            (*s).mtfa[kk as usize] =
                                (ii * 16 as std::os::raw::c_int + jj) as UChar;
                            kk -= 1;
                            jj -= 1
                        }
                        (*s).mtfbase[ii as usize] = kk + 1 as std::os::raw::c_int;
                        ii -= 1
                    }
                    /*-- end MTF init --*/
                    nblock = 0 as std::os::raw::c_int;
                    if groupPos == 0 as std::os::raw::c_int {
                        groupNo += 1;
                        if groupNo >= nSelectors {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            groupPos = 50 as std::os::raw::c_int;
                            gSel = (*s).selector[groupNo as usize] as Int32;
                            gMinlen = (*s).minLens[gSel as usize];
                            gLimit =
                                &mut *(*(*s).limit.as_mut_ptr().offset(gSel as
                                                                           isize)).as_mut_ptr().offset(0
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           isize)
                                    as *mut Int32;
                            gPerm =
                                &mut *(*(*s).perm.as_mut_ptr().offset(gSel as
                                                                          isize)).as_mut_ptr().offset(0
                                                                                                          as
                                                                                                          std::os::raw::c_int
                                                                                                          as
                                                                                                          isize)
                                    as *mut Int32;
                            gBase =
                                &mut *(*(*s).base.as_mut_ptr().offset(gSel as
                                                                          isize)).as_mut_ptr().offset(0
                                                                                                          as
                                                                                                          std::os::raw::c_int
                                                                                                          as
                                                                                                          isize)
                                    as *mut Int32
                        }
                    }
                    groupPos -= 1;
                    zn = gMinlen;
                    current_block = 16722720626876144162;
                }
            }
        }
    (*s).save_j = j;
    (*s).save_t = t;
    (*s).save_alphaSize = alphaSize;
    (*s).save_nGroups = nGroups;
    (*s).save_nSelectors = nSelectors;
    (*s).save_EOB = EOB;
    (*s).save_groupNo = groupNo;
    (*s).save_groupPos = groupPos;
    (*s).save_nextSym = nextSym;
    (*s).save_nblockMAX = nblockMAX;
    (*s).save_nblock = nblock;
    (*s).save_es = es;
    (*s).save_N = N;
    (*s).save_curr = curr;
    (*s).save_zt = zt;
    (*s).save_zn = zn;
    (*s).save_zvec = zvec;
    (*s).save_zj = zj;
    (*s).save_gSel = gSel;
    (*s).save_gMinlen = gMinlen;
    (*s).save_gLimit = gLimit;
    (*s).save_gBase = gBase;
    (*s).save_gPerm = gPerm;
    return retVal;
}
/*-------------------------------------------------------------*/
/*--- end                                      decompress.c ---*/
/*-------------------------------------------------------------*/

}

pub mod huffman {

extern "C" {
    
    
}
pub use crate::bzlib::BZ2_bz__AssertH__fail;
pub type Bool = crate::blocksort::Bool;
pub type UChar = crate::blocksort::UChar;
pub type Int32 = crate::blocksort::Int32;
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbMakeCodeLengths(mut len: *mut UChar,
                                               mut freq: *mut Int32,
                                               mut alphaSize: Int32,
                                               mut maxLen: Int32) {
    /*--
      Nodes and heap entries run from 1.  Entry 0
      for both the heap and nodes is a sentinel.
   --*/
    let mut nNodes: Int32 = 0;
    let mut nHeap: Int32 = 0;
    let mut n1: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut tooLong: Bool = 0;
    let mut heap: [Int32; 260] = [0; 260];
    let mut weight: [Int32; 516] = [0; 516];
    let mut parent: [Int32; 516] = [0; 516];
    i = 0 as std::os::raw::c_int;
    while i < alphaSize {
        weight[(i + 1 as std::os::raw::c_int) as usize] =
            (if *freq.offset(i as isize) == 0 as std::os::raw::c_int {
                 1 as std::os::raw::c_int
             } else { *freq.offset(i as isize) }) << 8 as std::os::raw::c_int;
        i += 1
    }
    while 1 as std::os::raw::c_int as Bool != 0 {
        nNodes = alphaSize;
        nHeap = 0 as std::os::raw::c_int;
        heap[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int;
        weight[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int;
        parent[0 as std::os::raw::c_int as usize] = -(2 as std::os::raw::c_int);
        i = 1 as std::os::raw::c_int;
        while i <= alphaSize {
            parent[i as usize] = -(1 as std::os::raw::c_int);
            nHeap += 1;
            heap[nHeap as usize] = i;
            let mut zz: Int32 = 0;
            let mut tmp: Int32 = 0;
            zz = nHeap;
            tmp = heap[zz as usize];
            while weight[tmp as usize] <
                      weight[heap[(zz >> 1 as std::os::raw::c_int) as usize] as usize]
                  {
                heap[zz as usize] = heap[(zz >> 1 as std::os::raw::c_int) as usize];
                zz >>= 1 as std::os::raw::c_int
            }
            heap[zz as usize] = tmp;
            i += 1
        }
        if !(nHeap < 258 as std::os::raw::c_int + 2 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(2001 as std::os::raw::c_int);
        }
        while nHeap > 1 as std::os::raw::c_int {
            n1 = heap[1 as std::os::raw::c_int as usize];
            heap[1 as std::os::raw::c_int as usize] = heap[nHeap as usize];
            nHeap -= 1;
            let mut zz_0: Int32 = 0;
            let mut yy: Int32 = 0;
            let mut tmp_0: Int32 = 0;
            zz_0 = 1 as std::os::raw::c_int;
            tmp_0 = heap[zz_0 as usize];
            while 1 as std::os::raw::c_int as Bool != 0 {
                yy = zz_0 << 1 as std::os::raw::c_int;
                if yy > nHeap { break ; }
                if yy < nHeap &&
                       weight[heap[(yy + 1 as std::os::raw::c_int) as usize] as usize]
                           < weight[heap[yy as usize] as usize] {
                    yy += 1
                }
                if weight[tmp_0 as usize] < weight[heap[yy as usize] as usize]
                   {
                    break ;
                }
                heap[zz_0 as usize] = heap[yy as usize];
                zz_0 = yy
            }
            heap[zz_0 as usize] = tmp_0;
            n2 = heap[1 as std::os::raw::c_int as usize];
            heap[1 as std::os::raw::c_int as usize] = heap[nHeap as usize];
            nHeap -= 1;
            let mut zz_1: Int32 = 0;
            let mut yy_0: Int32 = 0;
            let mut tmp_1: Int32 = 0;
            zz_1 = 1 as std::os::raw::c_int;
            tmp_1 = heap[zz_1 as usize];
            while 1 as std::os::raw::c_int as Bool != 0 {
                yy_0 = zz_1 << 1 as std::os::raw::c_int;
                if yy_0 > nHeap { break ; }
                if yy_0 < nHeap &&
                       weight[heap[(yy_0 + 1 as std::os::raw::c_int) as usize] as
                                  usize] <
                           weight[heap[yy_0 as usize] as usize] {
                    yy_0 += 1
                }
                if weight[tmp_1 as usize] <
                       weight[heap[yy_0 as usize] as usize] {
                    break ;
                }
                heap[zz_1 as usize] = heap[yy_0 as usize];
                zz_1 = yy_0
            }
            heap[zz_1 as usize] = tmp_1;
            nNodes += 1;
            parent[n2 as usize] = nNodes;
            parent[n1 as usize] = parent[n2 as usize];
            weight[nNodes as usize] =
                ((weight[n1 as usize] as std::os::raw::c_uint &
                      0xffffff00 as
                          std::os::raw::c_uint).wrapping_add(weight[n2 as usize] as
                                                         std::os::raw::c_uint &
                                                         0xffffff00 as
                                                             std::os::raw::c_uint) |
                     (1 as std::os::raw::c_int +
                          (if weight[n1 as usize] & 0xff as std::os::raw::c_int >
                                  weight[n2 as usize] & 0xff as std::os::raw::c_int {
                               (weight[n1 as usize]) & 0xff as std::os::raw::c_int
                           } else {
                               (weight[n2 as usize]) & 0xff as std::os::raw::c_int
                           })) as std::os::raw::c_uint) as Int32;
            parent[nNodes as usize] = -(1 as std::os::raw::c_int);
            nHeap += 1;
            heap[nHeap as usize] = nNodes;
            let mut zz_2: Int32 = 0;
            let mut tmp_2: Int32 = 0;
            zz_2 = nHeap;
            tmp_2 = heap[zz_2 as usize];
            while weight[tmp_2 as usize] <
                      weight[heap[(zz_2 >> 1 as std::os::raw::c_int) as usize] as
                                 usize] {
                heap[zz_2 as usize] =
                    heap[(zz_2 >> 1 as std::os::raw::c_int) as usize];
                zz_2 >>= 1 as std::os::raw::c_int
            }
            heap[zz_2 as usize] = tmp_2
        }
        if !(nNodes < 258 as std::os::raw::c_int * 2 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(2002 as std::os::raw::c_int);
        }
        tooLong = 0 as std::os::raw::c_int as Bool;
        i = 1 as std::os::raw::c_int;
        while i <= alphaSize {
            j = 0 as std::os::raw::c_int;
            k = i;
            while parent[k as usize] >= 0 as std::os::raw::c_int {
                k = parent[k as usize];
                j += 1
            }
            *len.offset((i - 1 as std::os::raw::c_int) as isize) = j as UChar;
            if j > maxLen { tooLong = 1 as std::os::raw::c_int as Bool }
            i += 1
        }
        if tooLong == 0 { break ; }
        /* 17 Oct 04: keep-going condition for the following loop used
         to be 'i < alphaSize', which missed the last element,
         theoretically leading to the possibility of the compressor
         looping.  However, this count-scaling step is only needed if
         one of the generated Huffman code words is longer than
         maxLen, which up to and including version 1.0.2 was 20 bits,
         which is extremely unlikely.  In version 1.0.3 maxLen was
         changed to 17 bits, which has minimal effect on compression
         ratio, but does mean this scaling step is used from time to
         time, enough to verify that it works.

         This means that bzip2-1.0.3 and later will only produce
         Huffman codes with a maximum length of 17 bits.  However, in
         order to preserve backwards compatibility with bitstreams
         produced by versions pre-1.0.3, the decompressor must still
         handle lengths of up to 20. */
        i = 1 as std::os::raw::c_int;
        while i <= alphaSize {
            j = weight[i as usize] >> 8 as std::os::raw::c_int;
            j = 1 as std::os::raw::c_int + j / 2 as std::os::raw::c_int;
            weight[i as usize] = j << 8 as std::os::raw::c_int;
            i += 1
        }
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbAssignCodes(mut code: *mut Int32,
                                           mut length: *mut UChar,
                                           mut minLen: Int32,
                                           mut maxLen: Int32,
                                           mut alphaSize: Int32) {
    let mut n: Int32 = 0;
    let mut vec: Int32 = 0;
    let mut i: Int32 = 0;
    vec = 0 as std::os::raw::c_int;
    n = minLen;
    while n <= maxLen {
        i = 0 as std::os::raw::c_int;
        while i < alphaSize {
            if *length.offset(i as isize) as std::os::raw::c_int == n {
                *code.offset(i as isize) = vec;
                vec += 1
            }
            i += 1
        }
        vec <<= 1 as std::os::raw::c_int;
        n += 1
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbCreateDecodeTables(mut limit: *mut Int32,
                                                  mut base: *mut Int32,
                                                  mut perm: *mut Int32,
                                                  mut length: *mut UChar,
                                                  mut minLen: Int32,
                                                  mut maxLen: Int32,
                                                  mut alphaSize: Int32) {
    let mut pp: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut vec: Int32 = 0;
    pp = 0 as std::os::raw::c_int;
    i = minLen;
    while i <= maxLen {
        j = 0 as std::os::raw::c_int;
        while j < alphaSize {
            if *length.offset(j as isize) as std::os::raw::c_int == i {
                *perm.offset(pp as isize) = j;
                pp += 1
            }
            j += 1
        }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 23 as std::os::raw::c_int {
        *base.offset(i as isize) = 0 as std::os::raw::c_int;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < alphaSize {
        let ref mut fresh0 =
            *base.offset((*length.offset(i as isize) as std::os::raw::c_int +
                              1 as std::os::raw::c_int) as isize);
        *fresh0 += 1;
        i += 1
    }
    i = 1 as std::os::raw::c_int;
    while i < 23 as std::os::raw::c_int {
        let ref mut fresh1 = *base.offset(i as isize);
        *fresh1 += *base.offset((i - 1 as std::os::raw::c_int) as isize);
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 23 as std::os::raw::c_int {
        *limit.offset(i as isize) = 0 as std::os::raw::c_int;
        i += 1
    }
    vec = 0 as std::os::raw::c_int;
    i = minLen;
    while i <= maxLen {
        vec +=
            *base.offset((i + 1 as std::os::raw::c_int) as isize) -
                *base.offset(i as isize);
        *limit.offset(i as isize) = vec - 1 as std::os::raw::c_int;
        vec <<= 1 as std::os::raw::c_int;
        i += 1
    }
    i = minLen + 1 as std::os::raw::c_int;
    while i <= maxLen {
        *base.offset(i as isize) =
            ((*limit.offset((i - 1 as std::os::raw::c_int) as isize) +
                  1 as std::os::raw::c_int) << 1 as std::os::raw::c_int) -
                *base.offset(i as isize);
        i += 1
    };
}
/*-------------------------------------------------------------*/
/*--- end                                         huffman.c ---*/
/*-------------------------------------------------------------*/

}

pub mod randtable {

pub type Int32 = crate::blocksort::Int32;
/*-------------------------------------------------------------*/
/*--- Table for randomising repetitive blocks               ---*/
/*---                                           randtable.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/*---------------------------------------------*/
#[no_mangle]
pub static mut BZ2_rNums: [Int32; 512] =
    [619 as std::os::raw::c_int, 720 as std::os::raw::c_int, 127 as std::os::raw::c_int,
     481 as std::os::raw::c_int, 931 as std::os::raw::c_int, 816 as std::os::raw::c_int,
     813 as std::os::raw::c_int, 233 as std::os::raw::c_int, 566 as std::os::raw::c_int,
     247 as std::os::raw::c_int, 985 as std::os::raw::c_int, 724 as std::os::raw::c_int,
     205 as std::os::raw::c_int, 454 as std::os::raw::c_int, 863 as std::os::raw::c_int,
     491 as std::os::raw::c_int, 741 as std::os::raw::c_int, 242 as std::os::raw::c_int,
     949 as std::os::raw::c_int, 214 as std::os::raw::c_int, 733 as std::os::raw::c_int,
     859 as std::os::raw::c_int, 335 as std::os::raw::c_int, 708 as std::os::raw::c_int,
     621 as std::os::raw::c_int, 574 as std::os::raw::c_int, 73 as std::os::raw::c_int,
     654 as std::os::raw::c_int, 730 as std::os::raw::c_int, 472 as std::os::raw::c_int,
     419 as std::os::raw::c_int, 436 as std::os::raw::c_int, 278 as std::os::raw::c_int,
     496 as std::os::raw::c_int, 867 as std::os::raw::c_int, 210 as std::os::raw::c_int,
     399 as std::os::raw::c_int, 680 as std::os::raw::c_int, 480 as std::os::raw::c_int,
     51 as std::os::raw::c_int, 878 as std::os::raw::c_int, 465 as std::os::raw::c_int,
     811 as std::os::raw::c_int, 169 as std::os::raw::c_int, 869 as std::os::raw::c_int,
     675 as std::os::raw::c_int, 611 as std::os::raw::c_int, 697 as std::os::raw::c_int,
     867 as std::os::raw::c_int, 561 as std::os::raw::c_int, 862 as std::os::raw::c_int,
     687 as std::os::raw::c_int, 507 as std::os::raw::c_int, 283 as std::os::raw::c_int,
     482 as std::os::raw::c_int, 129 as std::os::raw::c_int, 807 as std::os::raw::c_int,
     591 as std::os::raw::c_int, 733 as std::os::raw::c_int, 623 as std::os::raw::c_int,
     150 as std::os::raw::c_int, 238 as std::os::raw::c_int, 59 as std::os::raw::c_int,
     379 as std::os::raw::c_int, 684 as std::os::raw::c_int, 877 as std::os::raw::c_int,
     625 as std::os::raw::c_int, 169 as std::os::raw::c_int, 643 as std::os::raw::c_int,
     105 as std::os::raw::c_int, 170 as std::os::raw::c_int, 607 as std::os::raw::c_int,
     520 as std::os::raw::c_int, 932 as std::os::raw::c_int, 727 as std::os::raw::c_int,
     476 as std::os::raw::c_int, 693 as std::os::raw::c_int, 425 as std::os::raw::c_int,
     174 as std::os::raw::c_int, 647 as std::os::raw::c_int, 73 as std::os::raw::c_int,
     122 as std::os::raw::c_int, 335 as std::os::raw::c_int, 530 as std::os::raw::c_int,
     442 as std::os::raw::c_int, 853 as std::os::raw::c_int, 695 as std::os::raw::c_int,
     249 as std::os::raw::c_int, 445 as std::os::raw::c_int, 515 as std::os::raw::c_int,
     909 as std::os::raw::c_int, 545 as std::os::raw::c_int, 703 as std::os::raw::c_int,
     919 as std::os::raw::c_int, 874 as std::os::raw::c_int, 474 as std::os::raw::c_int,
     882 as std::os::raw::c_int, 500 as std::os::raw::c_int, 594 as std::os::raw::c_int,
     612 as std::os::raw::c_int, 641 as std::os::raw::c_int, 801 as std::os::raw::c_int,
     220 as std::os::raw::c_int, 162 as std::os::raw::c_int, 819 as std::os::raw::c_int,
     984 as std::os::raw::c_int, 589 as std::os::raw::c_int, 513 as std::os::raw::c_int,
     495 as std::os::raw::c_int, 799 as std::os::raw::c_int, 161 as std::os::raw::c_int,
     604 as std::os::raw::c_int, 958 as std::os::raw::c_int, 533 as std::os::raw::c_int,
     221 as std::os::raw::c_int, 400 as std::os::raw::c_int, 386 as std::os::raw::c_int,
     867 as std::os::raw::c_int, 600 as std::os::raw::c_int, 782 as std::os::raw::c_int,
     382 as std::os::raw::c_int, 596 as std::os::raw::c_int, 414 as std::os::raw::c_int,
     171 as std::os::raw::c_int, 516 as std::os::raw::c_int, 375 as std::os::raw::c_int,
     682 as std::os::raw::c_int, 485 as std::os::raw::c_int, 911 as std::os::raw::c_int,
     276 as std::os::raw::c_int, 98 as std::os::raw::c_int, 553 as std::os::raw::c_int,
     163 as std::os::raw::c_int, 354 as std::os::raw::c_int, 666 as std::os::raw::c_int,
     933 as std::os::raw::c_int, 424 as std::os::raw::c_int, 341 as std::os::raw::c_int,
     533 as std::os::raw::c_int, 870 as std::os::raw::c_int, 227 as std::os::raw::c_int,
     730 as std::os::raw::c_int, 475 as std::os::raw::c_int, 186 as std::os::raw::c_int,
     263 as std::os::raw::c_int, 647 as std::os::raw::c_int, 537 as std::os::raw::c_int,
     686 as std::os::raw::c_int, 600 as std::os::raw::c_int, 224 as std::os::raw::c_int,
     469 as std::os::raw::c_int, 68 as std::os::raw::c_int, 770 as std::os::raw::c_int,
     919 as std::os::raw::c_int, 190 as std::os::raw::c_int, 373 as std::os::raw::c_int,
     294 as std::os::raw::c_int, 822 as std::os::raw::c_int, 808 as std::os::raw::c_int,
     206 as std::os::raw::c_int, 184 as std::os::raw::c_int, 943 as std::os::raw::c_int,
     795 as std::os::raw::c_int, 384 as std::os::raw::c_int, 383 as std::os::raw::c_int,
     461 as std::os::raw::c_int, 404 as std::os::raw::c_int, 758 as std::os::raw::c_int,
     839 as std::os::raw::c_int, 887 as std::os::raw::c_int, 715 as std::os::raw::c_int,
     67 as std::os::raw::c_int, 618 as std::os::raw::c_int, 276 as std::os::raw::c_int,
     204 as std::os::raw::c_int, 918 as std::os::raw::c_int, 873 as std::os::raw::c_int,
     777 as std::os::raw::c_int, 604 as std::os::raw::c_int, 560 as std::os::raw::c_int,
     951 as std::os::raw::c_int, 160 as std::os::raw::c_int, 578 as std::os::raw::c_int,
     722 as std::os::raw::c_int, 79 as std::os::raw::c_int, 804 as std::os::raw::c_int,
     96 as std::os::raw::c_int, 409 as std::os::raw::c_int, 713 as std::os::raw::c_int,
     940 as std::os::raw::c_int, 652 as std::os::raw::c_int, 934 as std::os::raw::c_int,
     970 as std::os::raw::c_int, 447 as std::os::raw::c_int, 318 as std::os::raw::c_int,
     353 as std::os::raw::c_int, 859 as std::os::raw::c_int, 672 as std::os::raw::c_int,
     112 as std::os::raw::c_int, 785 as std::os::raw::c_int, 645 as std::os::raw::c_int,
     863 as std::os::raw::c_int, 803 as std::os::raw::c_int, 350 as std::os::raw::c_int,
     139 as std::os::raw::c_int, 93 as std::os::raw::c_int, 354 as std::os::raw::c_int,
     99 as std::os::raw::c_int, 820 as std::os::raw::c_int, 908 as std::os::raw::c_int,
     609 as std::os::raw::c_int, 772 as std::os::raw::c_int, 154 as std::os::raw::c_int,
     274 as std::os::raw::c_int, 580 as std::os::raw::c_int, 184 as std::os::raw::c_int,
     79 as std::os::raw::c_int, 626 as std::os::raw::c_int, 630 as std::os::raw::c_int,
     742 as std::os::raw::c_int, 653 as std::os::raw::c_int, 282 as std::os::raw::c_int,
     762 as std::os::raw::c_int, 623 as std::os::raw::c_int, 680 as std::os::raw::c_int,
     81 as std::os::raw::c_int, 927 as std::os::raw::c_int, 626 as std::os::raw::c_int,
     789 as std::os::raw::c_int, 125 as std::os::raw::c_int, 411 as std::os::raw::c_int,
     521 as std::os::raw::c_int, 938 as std::os::raw::c_int, 300 as std::os::raw::c_int,
     821 as std::os::raw::c_int, 78 as std::os::raw::c_int, 343 as std::os::raw::c_int,
     175 as std::os::raw::c_int, 128 as std::os::raw::c_int, 250 as std::os::raw::c_int,
     170 as std::os::raw::c_int, 774 as std::os::raw::c_int, 972 as std::os::raw::c_int,
     275 as std::os::raw::c_int, 999 as std::os::raw::c_int, 639 as std::os::raw::c_int,
     495 as std::os::raw::c_int, 78 as std::os::raw::c_int, 352 as std::os::raw::c_int,
     126 as std::os::raw::c_int, 857 as std::os::raw::c_int, 956 as std::os::raw::c_int,
     358 as std::os::raw::c_int, 619 as std::os::raw::c_int, 580 as std::os::raw::c_int,
     124 as std::os::raw::c_int, 737 as std::os::raw::c_int, 594 as std::os::raw::c_int,
     701 as std::os::raw::c_int, 612 as std::os::raw::c_int, 669 as std::os::raw::c_int,
     112 as std::os::raw::c_int, 134 as std::os::raw::c_int, 694 as std::os::raw::c_int,
     363 as std::os::raw::c_int, 992 as std::os::raw::c_int, 809 as std::os::raw::c_int,
     743 as std::os::raw::c_int, 168 as std::os::raw::c_int, 974 as std::os::raw::c_int,
     944 as std::os::raw::c_int, 375 as std::os::raw::c_int, 748 as std::os::raw::c_int,
     52 as std::os::raw::c_int, 600 as std::os::raw::c_int, 747 as std::os::raw::c_int,
     642 as std::os::raw::c_int, 182 as std::os::raw::c_int, 862 as std::os::raw::c_int,
     81 as std::os::raw::c_int, 344 as std::os::raw::c_int, 805 as std::os::raw::c_int,
     988 as std::os::raw::c_int, 739 as std::os::raw::c_int, 511 as std::os::raw::c_int,
     655 as std::os::raw::c_int, 814 as std::os::raw::c_int, 334 as std::os::raw::c_int,
     249 as std::os::raw::c_int, 515 as std::os::raw::c_int, 897 as std::os::raw::c_int,
     955 as std::os::raw::c_int, 664 as std::os::raw::c_int, 981 as std::os::raw::c_int,
     649 as std::os::raw::c_int, 113 as std::os::raw::c_int, 974 as std::os::raw::c_int,
     459 as std::os::raw::c_int, 893 as std::os::raw::c_int, 228 as std::os::raw::c_int,
     433 as std::os::raw::c_int, 837 as std::os::raw::c_int, 553 as std::os::raw::c_int,
     268 as std::os::raw::c_int, 926 as std::os::raw::c_int, 240 as std::os::raw::c_int,
     102 as std::os::raw::c_int, 654 as std::os::raw::c_int, 459 as std::os::raw::c_int,
     51 as std::os::raw::c_int, 686 as std::os::raw::c_int, 754 as std::os::raw::c_int,
     806 as std::os::raw::c_int, 760 as std::os::raw::c_int, 493 as std::os::raw::c_int,
     403 as std::os::raw::c_int, 415 as std::os::raw::c_int, 394 as std::os::raw::c_int,
     687 as std::os::raw::c_int, 700 as std::os::raw::c_int, 946 as std::os::raw::c_int,
     670 as std::os::raw::c_int, 656 as std::os::raw::c_int, 610 as std::os::raw::c_int,
     738 as std::os::raw::c_int, 392 as std::os::raw::c_int, 760 as std::os::raw::c_int,
     799 as std::os::raw::c_int, 887 as std::os::raw::c_int, 653 as std::os::raw::c_int,
     978 as std::os::raw::c_int, 321 as std::os::raw::c_int, 576 as std::os::raw::c_int,
     617 as std::os::raw::c_int, 626 as std::os::raw::c_int, 502 as std::os::raw::c_int,
     894 as std::os::raw::c_int, 679 as std::os::raw::c_int, 243 as std::os::raw::c_int,
     440 as std::os::raw::c_int, 680 as std::os::raw::c_int, 879 as std::os::raw::c_int,
     194 as std::os::raw::c_int, 572 as std::os::raw::c_int, 640 as std::os::raw::c_int,
     724 as std::os::raw::c_int, 926 as std::os::raw::c_int, 56 as std::os::raw::c_int,
     204 as std::os::raw::c_int, 700 as std::os::raw::c_int, 707 as std::os::raw::c_int,
     151 as std::os::raw::c_int, 457 as std::os::raw::c_int, 449 as std::os::raw::c_int,
     797 as std::os::raw::c_int, 195 as std::os::raw::c_int, 791 as std::os::raw::c_int,
     558 as std::os::raw::c_int, 945 as std::os::raw::c_int, 679 as std::os::raw::c_int,
     297 as std::os::raw::c_int, 59 as std::os::raw::c_int, 87 as std::os::raw::c_int,
     824 as std::os::raw::c_int, 713 as std::os::raw::c_int, 663 as std::os::raw::c_int,
     412 as std::os::raw::c_int, 693 as std::os::raw::c_int, 342 as std::os::raw::c_int,
     606 as std::os::raw::c_int, 134 as std::os::raw::c_int, 108 as std::os::raw::c_int,
     571 as std::os::raw::c_int, 364 as std::os::raw::c_int, 631 as std::os::raw::c_int,
     212 as std::os::raw::c_int, 174 as std::os::raw::c_int, 643 as std::os::raw::c_int,
     304 as std::os::raw::c_int, 329 as std::os::raw::c_int, 343 as std::os::raw::c_int,
     97 as std::os::raw::c_int, 430 as std::os::raw::c_int, 751 as std::os::raw::c_int,
     497 as std::os::raw::c_int, 314 as std::os::raw::c_int, 983 as std::os::raw::c_int,
     374 as std::os::raw::c_int, 822 as std::os::raw::c_int, 928 as std::os::raw::c_int,
     140 as std::os::raw::c_int, 206 as std::os::raw::c_int, 73 as std::os::raw::c_int,
     263 as std::os::raw::c_int, 980 as std::os::raw::c_int, 736 as std::os::raw::c_int,
     876 as std::os::raw::c_int, 478 as std::os::raw::c_int, 430 as std::os::raw::c_int,
     305 as std::os::raw::c_int, 170 as std::os::raw::c_int, 514 as std::os::raw::c_int,
     364 as std::os::raw::c_int, 692 as std::os::raw::c_int, 829 as std::os::raw::c_int,
     82 as std::os::raw::c_int, 855 as std::os::raw::c_int, 953 as std::os::raw::c_int,
     676 as std::os::raw::c_int, 246 as std::os::raw::c_int, 369 as std::os::raw::c_int,
     970 as std::os::raw::c_int, 294 as std::os::raw::c_int, 750 as std::os::raw::c_int,
     807 as std::os::raw::c_int, 827 as std::os::raw::c_int, 150 as std::os::raw::c_int,
     790 as std::os::raw::c_int, 288 as std::os::raw::c_int, 923 as std::os::raw::c_int,
     804 as std::os::raw::c_int, 378 as std::os::raw::c_int, 215 as std::os::raw::c_int,
     828 as std::os::raw::c_int, 592 as std::os::raw::c_int, 281 as std::os::raw::c_int,
     565 as std::os::raw::c_int, 555 as std::os::raw::c_int, 710 as std::os::raw::c_int,
     82 as std::os::raw::c_int, 896 as std::os::raw::c_int, 831 as std::os::raw::c_int,
     547 as std::os::raw::c_int, 261 as std::os::raw::c_int, 524 as std::os::raw::c_int,
     462 as std::os::raw::c_int, 293 as std::os::raw::c_int, 465 as std::os::raw::c_int,
     502 as std::os::raw::c_int, 56 as std::os::raw::c_int, 661 as std::os::raw::c_int,
     821 as std::os::raw::c_int, 976 as std::os::raw::c_int, 991 as std::os::raw::c_int,
     658 as std::os::raw::c_int, 869 as std::os::raw::c_int, 905 as std::os::raw::c_int,
     758 as std::os::raw::c_int, 745 as std::os::raw::c_int, 193 as std::os::raw::c_int,
     768 as std::os::raw::c_int, 550 as std::os::raw::c_int, 608 as std::os::raw::c_int,
     933 as std::os::raw::c_int, 378 as std::os::raw::c_int, 286 as std::os::raw::c_int,
     215 as std::os::raw::c_int, 979 as std::os::raw::c_int, 792 as std::os::raw::c_int,
     961 as std::os::raw::c_int, 61 as std::os::raw::c_int, 688 as std::os::raw::c_int,
     793 as std::os::raw::c_int, 644 as std::os::raw::c_int, 986 as std::os::raw::c_int,
     403 as std::os::raw::c_int, 106 as std::os::raw::c_int, 366 as std::os::raw::c_int,
     905 as std::os::raw::c_int, 644 as std::os::raw::c_int, 372 as std::os::raw::c_int,
     567 as std::os::raw::c_int, 466 as std::os::raw::c_int, 434 as std::os::raw::c_int,
     645 as std::os::raw::c_int, 210 as std::os::raw::c_int, 389 as std::os::raw::c_int,
     550 as std::os::raw::c_int, 919 as std::os::raw::c_int, 135 as std::os::raw::c_int,
     780 as std::os::raw::c_int, 773 as std::os::raw::c_int, 635 as std::os::raw::c_int,
     389 as std::os::raw::c_int, 707 as std::os::raw::c_int, 100 as std::os::raw::c_int,
     626 as std::os::raw::c_int, 958 as std::os::raw::c_int, 165 as std::os::raw::c_int,
     504 as std::os::raw::c_int, 920 as std::os::raw::c_int, 176 as std::os::raw::c_int,
     193 as std::os::raw::c_int, 713 as std::os::raw::c_int, 857 as std::os::raw::c_int,
     265 as std::os::raw::c_int, 203 as std::os::raw::c_int, 50 as std::os::raw::c_int,
     668 as std::os::raw::c_int, 108 as std::os::raw::c_int, 645 as std::os::raw::c_int,
     990 as std::os::raw::c_int, 626 as std::os::raw::c_int, 197 as std::os::raw::c_int,
     510 as std::os::raw::c_int, 357 as std::os::raw::c_int, 358 as std::os::raw::c_int,
     850 as std::os::raw::c_int, 858 as std::os::raw::c_int, 364 as std::os::raw::c_int,
     936 as std::os::raw::c_int, 638 as std::os::raw::c_int];
/*-------------------------------------------------------------*/
/*--- end                                       randtable.c ---*/
/*-------------------------------------------------------------*/

}

pub mod bzip2 {

extern "C" {
    
    #[no_mangle]
    static mut __stdinp: *mut FILE;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn ferror(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgetc(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn perror(_: *const std::os::raw::c_char);
    #[no_mangle]
    fn remove(_: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn rewind(_: *mut FILE);
    #[no_mangle]
    fn ungetc(_: std::os::raw::c_int, _: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fdopen(_: std::os::raw::c_int, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fileno(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn getenv(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn signal(_: std::os::raw::c_int,
              _: Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>)
     -> Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strerror(_: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strncpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __maskrune(_: __darwin_ct_rune_t, _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn open(_: *const std::os::raw::c_char, _: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn utime(_: *const std::os::raw::c_char, _: *const utimbuf) -> std::os::raw::c_int;
    #[no_mangle]
    fn _exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn close(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn isatty(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __nbyte: size_t)
     -> ssize_t;
    #[no_mangle]
    fn fchown(_: std::os::raw::c_int, _: uid_t, _: gid_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn fchmod(_: std::os::raw::c_int, _: mode_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn lstat(_: *const std::os::raw::c_char, _: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn stat(_: *const std::os::raw::c_char, _: *mut stat) -> std::os::raw::c_int;
}
pub use crate::bzlib::BZ2_bzRead;
pub use crate::bzlib::BZ2_bzReadClose;
pub use crate::bzlib::BZ2_bzReadGetUnused;
pub use crate::bzlib::BZ2_bzReadOpen;
pub use crate::bzlib::BZ2_bzWrite;
pub use crate::bzlib::BZ2_bzWriteClose64;
pub use crate::bzlib::BZ2_bzWriteOpen;
pub use crate::bzlib::BZ2_bzlibVersion;
pub use crate::blocksort::__sFILEX;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = crate::blocksort::__int64_t;
pub type __uint64_t = std::os::raw::c_ulonglong;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_ssize_t = std::os::raw::c_long;
pub type __darwin_time_t = std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = crate::blocksort::__darwin_off_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = crate::blocksort::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::blocksort::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::blocksort::__sFILE;
pub type FILE = crate::blocksort::FILE;
pub type off_t = __darwin_off_t;
pub type ssize_t = __darwin_ssize_t;
pub type uid_t = __darwin_uid_t;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: std::os::raw::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [std::os::raw::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [std::os::raw::c_char; 8],
    pub __encoding: [std::os::raw::c_char; 32],
    pub __sgetrune: Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *const std::os::raw::c_char)
                               -> __darwin_rune_t>,
    pub __sputrune: Option<unsafe extern "C" fn(_: __darwin_rune_t,
                                                _: *mut std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *mut std::os::raw::c_char)
                               -> std::os::raw::c_int>,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut std::os::raw::c_void,
    pub __variable_len: std::os::raw::c_int,
    pub __ncharclasses: std::os::raw::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: std::os::raw::c_long,
}
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type nlink_t = __uint16_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
/* */
/* BZ_UNIX */
/* BZ_LCCWIN32 */
/*---------------------------------------------*/
/*--
  Some more stuff for all platforms :-)
--*/
pub type Char = std::os::raw::c_char;
pub type Bool = crate::blocksort::Bool;
pub type UChar = crate::blocksort::UChar;
pub type Int32 = crate::blocksort::Int32;
pub type UInt32 = crate::blocksort::UInt32;
pub type Int16 = std::os::raw::c_short;
pub type UInt16 = crate::blocksort::UInt16;
/*--
  IntNative is your platform's `native' int size.
  Only here to avoid probs with 64-bit platforms.
--*/
pub type IntNative = std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UInt64 {
    pub b: [UChar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zzzz {
    pub name: *mut Char,
    pub link: *mut zzzz,
}
/*---------------------------------------------*/
/*--
  All the garbage from here to main() is purely to
  implement a linked list of command-line arguments,
  into which main() copies argv[1 .. argc-1].

  The purpose of this exercise is to facilitate 
  the expansion of wildcard characters * and ? in 
  filenames for OSs which don't know how to do it
  themselves, like MSDOS, Windows 95 and NT.

  The actual Dirty Work is done by the platform-
  specific macro APPEND_FILESPEC.
--*/
pub type Cell = zzzz;
#[inline]
 extern "C" fn isascii(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (_c & !(0x7f as std::os::raw::c_int) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t,
                              mut _f: std::os::raw::c_ulong) -> std::os::raw::c_int {
    return if isascii(_c) != 0 {
               (_DefaultRuneLocale.__runetype[_c as usize] as std::os::raw::c_ulong &
                    _f != 0) as std::os::raw::c_int
           } else { (__maskrune(_c, _f) != 0) as std::os::raw::c_int };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __istype(_c, 0x4000 as std::os::raw::c_long as std::os::raw::c_ulong);
}
/*---------------------------------------------------*/
/*--- Misc (file handling) data decls             ---*/
/*---------------------------------------------------*/
#[no_mangle]
pub static mut verbosity: Int32 = 0;
#[no_mangle]
pub static mut keepInputFiles: Bool = 0;
#[no_mangle]
pub static mut smallMode: Bool = 0;
#[no_mangle]
pub static mut deleteOutputOnInterrupt: Bool = 0;
#[no_mangle]
pub static mut forceOverwrite: Bool = 0;
#[no_mangle]
pub static mut testFailsExist: Bool = 0;
#[no_mangle]
pub static mut unzFailsExist: Bool = 0;
#[no_mangle]
pub static mut noisy: Bool = 0;
#[no_mangle]
pub static mut numFileNames: Int32 = 0;
#[no_mangle]
pub static mut numFilesProcessed: Int32 = 0;
#[no_mangle]
pub static mut blockSize100k: Int32 = 0;
#[no_mangle]
pub static mut exitValue: Int32 = 0;
#[no_mangle]
pub static mut opMode: Int32 = 0;
#[no_mangle]
pub static mut srcMode: Int32 = 0;
#[no_mangle]
pub static mut longestFileName: Int32 = 0;
#[no_mangle]
pub static mut inName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut outName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut tmpName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut progName: *mut Char = 0 as *const Char as *mut Char;
#[no_mangle]
pub static mut progNameReally: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut outputHandleJustInCase: *mut FILE =
    0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut workFactor: Int32 = 0;
unsafe extern "C" fn uInt64_from_UInt32s(mut n: *mut UInt64, mut lo32: UInt32,
                                         mut hi32: UInt32) {
    (*n).b[7 as std::os::raw::c_int as usize] =
        (hi32 >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[6 as std::os::raw::c_int as usize] =
        (hi32 >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[5 as std::os::raw::c_int as usize] =
        (hi32 >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[4 as std::os::raw::c_int as usize] =
        (hi32 & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
    (*n).b[3 as std::os::raw::c_int as usize] =
        (lo32 >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[2 as std::os::raw::c_int as usize] =
        (lo32 >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[1 as std::os::raw::c_int as usize] =
        (lo32 >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[0 as std::os::raw::c_int as usize] =
        (lo32 & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
}
unsafe extern "C" fn uInt64_to_double(mut n: *mut UInt64) -> std::os::raw::c_double {
    let mut i: Int32 = 0;
    let mut base: std::os::raw::c_double = 1.0f64;
    let mut sum: std::os::raw::c_double = 0.0f64;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        sum += base * (*n).b[i as usize] as std::os::raw::c_double;
        base *= 256.0f64;
        i += 1
    }
    return sum;
}
unsafe extern "C" fn uInt64_isZero(mut n: *mut UInt64) -> Bool {
    let mut i: Int32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        if (*n).b[i as usize] as std::os::raw::c_int != 0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int as Bool
        }
        i += 1
    }
    return 1 as std::os::raw::c_int as Bool;
}
/* Divide *n by 10, and return the remainder.  */
unsafe extern "C" fn uInt64_qrm10(mut n: *mut UInt64) -> Int32 {
    let mut rem: UInt32 = 0;
    let mut tmp: UInt32 = 0;
    let mut i: Int32 = 0;
    rem = 0 as std::os::raw::c_int as UInt32;
    i = 7 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        tmp =
            rem.wrapping_mul(256 as std::os::raw::c_int as
                                 std::os::raw::c_uint).wrapping_add((*n).b[i as usize]
                                                                as
                                                                std::os::raw::c_uint);
        (*n).b[i as usize] =
            tmp.wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
        rem = tmp.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_uint);
        i -= 1
    }
    return rem as Int32;
}
/* ... and the Whole Entire Point of all this UInt64 stuff is
   so that we can supply the following function.
*/
unsafe extern "C" fn uInt64_toAscii(mut outbuf: *mut std::os::raw::c_char,
                                    mut n: *mut UInt64) {
    let mut i: Int32 = 0;
    let mut q: Int32 = 0;
    let mut buf: [UChar; 32] = [0; 32];
    let mut nBuf: Int32 = 0 as std::os::raw::c_int;
    let mut n_copy: UInt64 = *n;
    loop  {
        q = uInt64_qrm10(&mut n_copy);
        buf[nBuf as usize] = (q + '0' as i32) as UChar;
        nBuf += 1;
        if !(uInt64_isZero(&mut n_copy) == 0) { break ; }
    }
    *outbuf.offset(nBuf as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    while i < nBuf {
        *outbuf.offset(i as isize) =
            buf[(nBuf - i - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_char;
        i += 1
    };
}
/*---------------------------------------------------*/
/*--- Processing of complete files and streams    ---*/
/*---------------------------------------------------*/
/*---------------------------------------------*/
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c: Int32 = fgetc(f);
    if c == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int as Bool }
    ungetc(c, f);
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
unsafe extern "C" fn compressStream(mut stream: *mut FILE,
                                    mut zStream: *mut FILE) {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut ibuf: [UChar; 5000] = [0; 5000];
    let mut nIbuf: Int32 = 0;
    let mut nbytes_in_lo32: UInt32 = 0;
    let mut nbytes_in_hi32: UInt32 = 0;
    let mut nbytes_out_lo32: UInt32 = 0;
    let mut nbytes_out_hi32: UInt32 = 0;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            bzf =
                BZ2_bzWriteOpen(&mut bzerr, zStream, blockSize100k, verbosity,
                                workFactor);
            if bzerr != 0 as std::os::raw::c_int {
                current_block = 660242869387099075;
            } else {
                if verbosity >= 2 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"\n\x00" as *const u8 as *const std::os::raw::c_char);
                }
                loop  {
                    if !(1 as std::os::raw::c_int as Bool != 0) {
                        current_block = 13242334135786603907;
                        break ;
                    }
                    if myfeof(stream) != 0 {
                        current_block = 13242334135786603907;
                        break ;
                    }
                    nIbuf =
                        fread(ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                              ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                              5000 as std::os::raw::c_int as std::os::raw::c_ulong, stream) as
                            Int32;
                    if ferror(stream) != 0 {
                        current_block = 18225113528933273530;
                        break ;
                    }
                    if nIbuf > 0 as std::os::raw::c_int {
                        BZ2_bzWrite(&mut bzerr, bzf,
                                    ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                                    nIbuf);
                    }
                    if bzerr != 0 as std::os::raw::c_int {
                        current_block = 660242869387099075;
                        break ;
                    }
                }
                match current_block {
                    18225113528933273530 => { }
                    660242869387099075 => { }
                    _ => {
                        BZ2_bzWriteClose64(&mut bzerr, bzf, 0 as std::os::raw::c_int,
                                           &mut nbytes_in_lo32,
                                           &mut nbytes_in_hi32,
                                           &mut nbytes_out_lo32,
                                           &mut nbytes_out_hi32);
                        if bzerr != 0 as std::os::raw::c_int {
                            current_block = 660242869387099075;
                        } else if ferror(zStream) != 0 {
                            current_block = 18225113528933273530;
                        } else {
                            ret = fflush(zStream);
                            if ret == -(1 as std::os::raw::c_int) {
                                current_block = 18225113528933273530;
                            } else {
                                if zStream != __stdoutp {
                                    let mut fd: Int32 = fileno(zStream);
                                    if fd < 0 as std::os::raw::c_int {
                                        current_block = 18225113528933273530;
                                    } else {
                                        applySavedFileAttrToOutputFile(fd);
                                        ret = fclose(zStream);
                                        outputHandleJustInCase =
                                            0 as *mut FILE;
                                        if ret == -(1 as std::os::raw::c_int) {
                                            current_block =
                                                18225113528933273530;
                                        } else {
                                            current_block =
                                                17281240262373992796;
                                        }
                                    }
                                } else {
                                    current_block = 17281240262373992796;
                                }
                                match current_block {
                                    18225113528933273530 => { }
                                    _ => {
                                        outputHandleJustInCase =
                                            0 as *mut FILE;
                                        if ferror(stream) != 0 {
                                            current_block =
                                                18225113528933273530;
                                        } else {
                                            ret = fclose(stream);
                                            if ret == -(1 as std::os::raw::c_int) {
                                                current_block =
                                                    18225113528933273530;
                                            } else {
                                                if verbosity >=
                                                       1 as std::os::raw::c_int {
                                                    if nbytes_in_lo32 ==
                                                           0 as std::os::raw::c_int as
                                                               std::os::raw::c_uint &&
                                                           nbytes_in_hi32 ==
                                                               0 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_uint
                                                       {
                                                        fprintf(__stderrp,
                                                                b" no data compressed.\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const std::os::raw::c_char);
                                                    } else {
                                                        let mut buf_nin:
                                                                [Char; 32] =
                                                            [0; 32];
                                                        let mut buf_nout:
                                                                [Char; 32] =
                                                            [0; 32];
                                                        let mut nbytes_in:
                                                                UInt64 =
                                                            UInt64{b:
                                                                       [0;
                                                                           8],};
                                                        let mut nbytes_out:
                                                                UInt64 =
                                                            UInt64{b:
                                                                       [0;
                                                                           8],};
                                                        let mut nbytes_in_d:
                                                                std::os::raw::c_double =
                                                            0.;
                                                        let mut nbytes_out_d:
                                                                std::os::raw::c_double =
                                                            0.;
                                                        uInt64_from_UInt32s(&mut nbytes_in,
                                                                            nbytes_in_lo32,
                                                                            nbytes_in_hi32);
                                                        uInt64_from_UInt32s(&mut nbytes_out,
                                                                            nbytes_out_lo32,
                                                                            nbytes_out_hi32);
                                                        nbytes_in_d =
                                                            uInt64_to_double(&mut nbytes_in);
                                                        nbytes_out_d =
                                                            uInt64_to_double(&mut nbytes_out);
                                                        uInt64_toAscii(buf_nin.as_mut_ptr(),
                                                                       &mut nbytes_in);
                                                        uInt64_toAscii(buf_nout.as_mut_ptr(),
                                                                       &mut nbytes_out);
                                                        fprintf(__stderrp,
                                                                b"%6.3f:1, %6.3f bits/byte, %5.2f%% saved, %s in, %s out.\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const std::os::raw::c_char,
                                                                nbytes_in_d /
                                                                    nbytes_out_d,
                                                                8.0f64 *
                                                                    nbytes_out_d
                                                                    /
                                                                    nbytes_in_d,
                                                                100.0f64 *
                                                                    (1.0f64 -
                                                                         nbytes_out_d
                                                                             /
                                                                             nbytes_in_d),
                                                                buf_nin.as_mut_ptr(),
                                                                buf_nout.as_mut_ptr());
                                                    }
                                                }
                                                return
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                18225113528933273530 => { }
                _ => {
                    BZ2_bzWriteClose64(&mut bzerr_dummy, bzf,
                                       1 as std::os::raw::c_int, &mut nbytes_in_lo32,
                                       &mut nbytes_in_hi32,
                                       &mut nbytes_out_lo32,
                                       &mut nbytes_out_hi32);
                    match bzerr {
                        -9 => {
                            current_block = 5122324059762049690;
                            match current_block {
                                4323399205346619401 => {
                                    panic(b"compress:unexpected error\x00" as
                                              *const u8 as
                                              *const std::os::raw::c_char);
                                }
                                2380987886157893679 => { outOfMemory(); }
                                _ => { configError(); }
                            }
                        }
                        -3 => {
                            current_block = 2380987886157893679;
                            match current_block {
                                4323399205346619401 => {
                                    panic(b"compress:unexpected error\x00" as
                                              *const u8 as
                                              *const std::os::raw::c_char);
                                }
                                2380987886157893679 => { outOfMemory(); }
                                _ => { configError(); }
                            }
                        }
                        -6 => { }
                        _ => {
                            current_block = 4323399205346619401;
                            match current_block {
                                4323399205346619401 => {
                                    panic(b"compress:unexpected error\x00" as
                                              *const u8 as
                                              *const std::os::raw::c_char);
                                }
                                2380987886157893679 => { outOfMemory(); }
                                _ => { configError(); }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------*/
unsafe extern "C" fn uncompressStream(mut zStream: *mut FILE,
                                      mut stream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut nread: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut UChar = 0 as *mut UChar;
    nUnused = 0 as std::os::raw::c_int;
    streamNo = 0 as std::os::raw::c_int;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            's_51:
                loop  {
                    if !(1 as std::os::raw::c_int as Bool != 0) {
                        current_block = 926243229934402080;
                        break ;
                    }
                    bzf =
                        BZ2_bzReadOpen(&mut bzerr, zStream, verbosity,
                                       smallMode as std::os::raw::c_int,
                                       unused.as_mut_ptr() as
                                           *mut std::os::raw::c_void, nUnused);
                    if bzf.is_null() || bzerr != 0 as std::os::raw::c_int {
                        current_block = 673979509383251364;
                        break ;
                    }
                    streamNo += 1;
                    while bzerr == 0 as std::os::raw::c_int {
                        nread =
                            BZ2_bzRead(&mut bzerr, bzf,
                                       obuf.as_mut_ptr() as *mut std::os::raw::c_void,
                                       5000 as std::os::raw::c_int);
                        if bzerr == -(5 as std::os::raw::c_int) {
                            current_block = 18063049917807660484;
                            break 's_51 ;
                        }
                        if (bzerr == 0 as std::os::raw::c_int ||
                                bzerr == 4 as std::os::raw::c_int) &&
                               nread > 0 as std::os::raw::c_int {
                            fwrite(obuf.as_mut_ptr() as *const std::os::raw::c_void,
                                   ::std::mem::size_of::<UChar>() as
                                       std::os::raw::c_ulong, nread as std::os::raw::c_ulong,
                                   stream);
                        }
                        if ferror(stream) != 0 {
                            current_block = 5049394217699438129;
                            break 's_51 ;
                        }
                    }
                    if bzerr != 4 as std::os::raw::c_int {
                        current_block = 673979509383251364;
                        break ;
                    }
                    BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV,
                                        &mut nUnused);
                    if bzerr != 0 as std::os::raw::c_int {
                        panic(b"decompress:bzReadGetUnused\x00" as *const u8
                                  as *const std::os::raw::c_char);
                    }
                    unusedTmp = unusedTmpV as *mut UChar;
                    i = 0 as std::os::raw::c_int;
                    while i < nUnused {
                        unused[i as usize] = *unusedTmp.offset(i as isize);
                        i += 1
                    }
                    BZ2_bzReadClose(&mut bzerr, bzf);
                    if bzerr != 0 as std::os::raw::c_int {
                        panic(b"decompress:bzReadGetUnused\x00" as *const u8
                                  as *const std::os::raw::c_char);
                    }
                    if nUnused == 0 as std::os::raw::c_int &&
                           myfeof(zStream) as std::os::raw::c_int != 0 {
                        current_block = 926243229934402080;
                        break ;
                    }
                }
            match current_block {
                5049394217699438129 => { }
                _ => {
                    match current_block {
                        18063049917807660484 => {
                            if forceOverwrite != 0 {
                                rewind(zStream);
                                loop  {
                                    if !(1 as std::os::raw::c_int as Bool != 0) {
                                        current_block = 926243229934402080;
                                        break ;
                                    }
                                    if myfeof(zStream) != 0 {
                                        current_block = 926243229934402080;
                                        break ;
                                    }
                                    nread =
                                        fread(obuf.as_mut_ptr() as
                                                  *mut std::os::raw::c_void,
                                              ::std::mem::size_of::<UChar>()
                                                  as std::os::raw::c_ulong,
                                              5000 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong, zStream) as
                                            Int32;
                                    if ferror(zStream) != 0 {
                                        current_block = 5049394217699438129;
                                        break ;
                                    }
                                    if nread > 0 as std::os::raw::c_int {
                                        fwrite(obuf.as_mut_ptr() as
                                                   *const std::os::raw::c_void,
                                               ::std::mem::size_of::<UChar>()
                                                   as std::os::raw::c_ulong,
                                               nread as std::os::raw::c_ulong,
                                               stream);
                                    }
                                    if ferror(stream) != 0 {
                                        current_block = 5049394217699438129;
                                        break ;
                                    }
                                }
                            } else { current_block = 673979509383251364; }
                        }
                        _ => { }
                    }
                    match current_block {
                        5049394217699438129 => { }
                        _ => {
                            match current_block {
                                673979509383251364 => {
                                    BZ2_bzReadClose(&mut bzerr_dummy, bzf);
                                    match bzerr {
                                        -9 => {
                                            current_block =
                                                3642457097893642164;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -6 => { }
                                        -4 => {
                                            current_block =
                                                10766414566319669440;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -3 => {
                                            current_block =
                                                16178635849926953562;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -7 => {
                                            current_block =
                                                5517467152645906530;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -5 => {
                                            current_block =
                                                7372986856480808103;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            current_block =
                                                6455255476181645667;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    if !(ferror(zStream) != 0) {
                                        if stream != __stdoutp {
                                            let mut fd: Int32 =
                                                fileno(stream);
                                            if fd < 0 as std::os::raw::c_int {
                                                current_block =
                                                    5049394217699438129;
                                            } else {
                                                applySavedFileAttrToOutputFile(fd);
                                                current_block =
                                                    14832935472441733737;
                                            }
                                        } else {
                                            current_block =
                                                14832935472441733737;
                                        }
                                        match current_block {
                                            5049394217699438129 => { }
                                            _ => {
                                                ret = fclose(zStream);
                                                if !(ret ==
                                                         -(1 as std::os::raw::c_int))
                                                   {
                                                    if !(ferror(stream) != 0)
                                                       {
                                                        ret = fflush(stream);
                                                        if !(ret !=
                                                                 0 as
                                                                     std::os::raw::c_int)
                                                           {
                                                            if stream !=
                                                                   __stdoutp {
                                                                ret =
                                                                    fclose(stream);
                                                                outputHandleJustInCase
                                                                    =
                                                                    0 as
                                                                        *mut FILE;
                                                                if ret ==
                                                                       -(1 as
                                                                             std::os::raw::c_int)
                                                                   {
                                                                    current_block
                                                                        =
                                                                        5049394217699438129;
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        14775119014532381840;
                                                                }
                                                            } else {
                                                                current_block
                                                                    =
                                                                    14775119014532381840;
                                                            }
                                                            match current_block
                                                                {
                                                                5049394217699438129
                                                                => {
                                                                }
                                                                _ => {
                                                                    outputHandleJustInCase
                                                                        =
                                                                        0 as
                                                                            *mut FILE;
                                                                    if verbosity
                                                                           >=
                                                                           2
                                                                               as
                                                                               std::os::raw::c_int
                                                                       {
                                                                        fprintf(__stderrp,
                                                                                b"\n    \x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const std::os::raw::c_char);
                                                                    }
                                                                    return 1
                                                                               as
                                                                               std::os::raw::c_int
                                                                               as
                                                                               Bool
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------*/
unsafe extern "C" fn testStream(mut zStream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut UChar = 0 as *mut UChar;
    nUnused = 0 as std::os::raw::c_int;
    streamNo = 0 as std::os::raw::c_int;
    if !(ferror(zStream) != 0) {
        's_41:
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 15125582407903384992;
                    break ;
                }
                bzf =
                    BZ2_bzReadOpen(&mut bzerr, zStream, verbosity,
                                   smallMode as std::os::raw::c_int,
                                   unused.as_mut_ptr() as *mut std::os::raw::c_void,
                                   nUnused);
                if bzf.is_null() || bzerr != 0 as std::os::raw::c_int {
                    current_block = 10905486111603547446;
                    break ;
                }
                streamNo += 1;
                while bzerr == 0 as std::os::raw::c_int {
                    BZ2_bzRead(&mut bzerr, bzf,
                               obuf.as_mut_ptr() as *mut std::os::raw::c_void,
                               5000 as std::os::raw::c_int);
                    if bzerr == -(5 as std::os::raw::c_int) {
                        current_block = 10905486111603547446;
                        break 's_41 ;
                    }
                }
                if bzerr != 4 as std::os::raw::c_int {
                    current_block = 10905486111603547446;
                    break ;
                }
                BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV,
                                    &mut nUnused);
                if bzerr != 0 as std::os::raw::c_int {
                    panic(b"test:bzReadGetUnused\x00" as *const u8 as
                              *const std::os::raw::c_char);
                }
                unusedTmp = unusedTmpV as *mut UChar;
                i = 0 as std::os::raw::c_int;
                while i < nUnused {
                    unused[i as usize] = *unusedTmp.offset(i as isize);
                    i += 1
                }
                BZ2_bzReadClose(&mut bzerr, bzf);
                if bzerr != 0 as std::os::raw::c_int {
                    panic(b"test:bzReadGetUnused\x00" as *const u8 as
                              *const std::os::raw::c_char);
                }
                if nUnused == 0 as std::os::raw::c_int &&
                       myfeof(zStream) as std::os::raw::c_int != 0 {
                    current_block = 15125582407903384992;
                    break ;
                }
            }
        match current_block {
            15125582407903384992 => {
                if !(ferror(zStream) != 0) {
                    ret = fclose(zStream);
                    if !(ret == -(1 as std::os::raw::c_int)) {
                        if verbosity >= 2 as std::os::raw::c_int {
                            fprintf(__stderrp,
                                    b"\n    \x00" as *const u8 as
                                        *const std::os::raw::c_char);
                        }
                        return 1 as std::os::raw::c_int as Bool
                    }
                }
            }
            _ => {
                BZ2_bzReadClose(&mut bzerr_dummy, bzf);
                if verbosity == 0 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"%s: %s: \x00" as *const u8 as
                                *const std::os::raw::c_char, progName,
                            inName.as_mut_ptr());
                }
                match bzerr {
                    -9 => {
                        current_block = 18238374633732057650;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -6 => { }
                    -4 => {
                        current_block = 11224962462315262049;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -3 => {
                        current_block = 13802719682174684861;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -7 => {
                        current_block = 10380742613918245393;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -5 => {
                        current_block = 2539039579982765382;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    _ => {
                        current_block = 6101827300316655396;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------------*/
/*--- Error [non-] handling grunge                ---*/
/*---------------------------------------------------*/
/*---------------------------------------------*/
unsafe extern "C" fn setExit(mut v: Int32) {
    if v > exitValue { exitValue = v };
}
/*---------------------------------------------*/
unsafe extern "C" fn cadvise() {
    if noisy != 0 {
        fprintf(__stderrp,
                b"\nIt is possible that the compressed file(s) have become corrupted.\nYou can use the -tvv option to test integrity of such files.\n\nYou can use the `bzip2recover\' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\x00"
                    as *const u8 as *const std::os::raw::c_char);
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn showFileNames() {
    if noisy != 0 {
        fprintf(__stderrp,
                b"\tInput file = %s, output file = %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, inName.as_mut_ptr(),
                outName.as_mut_ptr());
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn cleanUpAndFail(mut ec: Int32) -> ! {
    let mut retVal: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    if srcMode == 3 as std::os::raw::c_int && opMode != 3 as std::os::raw::c_int &&
           deleteOutputOnInterrupt as std::os::raw::c_int != 0 {
        /* Check whether input file still exists.  Delete output file
         only if input exists to avoid loss of data.  Joerg Prante, 5
         January 2002.  (JRS 06-Jan-2002: other changes in 1.0.2 mean
         this is less likely to happen.  But to be ultra-paranoid, we
         do the check anyway.)  */
        retVal = stat(inName.as_mut_ptr(), &mut statBuf);
        if retVal == 0 as std::os::raw::c_int {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"%s: Deleting output file %s, if it exists.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr());
            }
            if !outputHandleJustInCase.is_null() {
                fclose(outputHandleJustInCase);
            }
            retVal = remove(outName.as_mut_ptr());
            if retVal != 0 as std::os::raw::c_int {
                fprintf(__stderrp,
                        b"%s: WARNING: deletion of output file (apparently) failed.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
            }
        } else {
            fprintf(__stderrp,
                    b"%s: WARNING: deletion of output file suppressed\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
            fprintf(__stderrp,
                    b"%s:    since input file no longer exists.  Output file\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
            fprintf(__stderrp,
                    b"%s:    `%s\' may be incomplete.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, outName.as_mut_ptr());
            fprintf(__stderrp,
                    b"%s:    I suggest doing an integrity test (bzip2 -tv) of it.\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
        }
    }
    if noisy as std::os::raw::c_int != 0 && numFileNames > 0 as std::os::raw::c_int &&
           numFilesProcessed < numFileNames {
        fprintf(__stderrp,
                b"%s: WARNING: some files have not been processed:\n%s:    %d specified on command line, %d not processed yet.\n\n\x00"
                    as *const u8 as *const std::os::raw::c_char, progName, progName,
                numFileNames, numFileNames - numFilesProcessed);
    }
    setExit(ec);
    exit(exitValue);
}
/*---------------------------------------------*/
unsafe extern "C" fn panic(mut s: *const Char) -> ! {
    fprintf(__stderrp,
            b"\n%s: PANIC -- internal consistency error:\n\t%s\n\tThis is a BUG.  Please report it to:\n\tbzip2-devel@sourceware.org\n\x00"
                as *const u8 as *const std::os::raw::c_char, progName, s);
    showFileNames();
    cleanUpAndFail(3 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn crcError() -> ! {
    fprintf(__stderrp,
            b"\n%s: Data integrity error when decompressing.\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName);
    showFileNames();
    cadvise();
    cleanUpAndFail(2 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn compressedStreamEOF() -> ! {
    if noisy != 0 {
        fprintf(__stderrp,
                b"\n%s: Compressed file ends unexpectedly;\n\tperhaps it is corrupted?  *Possible* reason follows.\n\x00"
                    as *const u8 as *const std::os::raw::c_char, progName);
        perror(progName);
        showFileNames();
        cadvise();
    }
    cleanUpAndFail(2 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn ioError() -> ! {
    fprintf(__stderrp,
            b"\n%s: I/O or other error, bailing out.  Possible reason follows.\n\x00"
                as *const u8 as *const std::os::raw::c_char, progName);
    perror(progName);
    showFileNames();
    cleanUpAndFail(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn mySignalCatcher(mut n: IntNative) {
    fprintf(__stderrp,
            b"\n%s: Control-C or similar caught, quitting.\n\x00" as *const u8
                as *const std::os::raw::c_char, progName);
    cleanUpAndFail(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn mySIGSEGVorSIGBUScatcher(mut n: IntNative) {
    let mut msg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if opMode == 1 as std::os::raw::c_int {
        msg =
            b": Caught a SIGSEGV or SIGBUS whilst compressing.\n\n   Possible causes are (most likely first):\n   (1) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (2) A bug in the compiler used to create this executable\n       (unlikely, if you didn\'t compile bzip2 yourself.)\n   (3) A real bug in bzip2 -- I hope this should never be the case.\n   The user\'s manual, Section 4.3, has more info on (1) and (2).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (1)\n   or (2), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user\'s manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don\'t\n   have the manual or can\'t be bothered to read it, mail me anyway.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char
    } else {
        msg =
            b": Caught a SIGSEGV or SIGBUS whilst decompressing.\n\n   Possible causes are (most likely first):\n   (1) The compressed data is corrupted, and bzip2\'s usual checks\n       failed to detect this.  Try bzip2 -tvv my_file.bz2.\n   (2) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (3) A bug in the compiler used to create this executable\n       (unlikely, if you didn\'t compile bzip2 yourself.)\n   (4) A real bug in bzip2 -- I hope this should never be the case.\n   The user\'s manual, Section 4.3, has more info on (2) and (3).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (2)\n   or (3), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user\'s manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don\'t\n   have the manual or can\'t be bothered to read it, mail me anyway.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char
    }
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    write(2 as std::os::raw::c_int, progName as *const std::os::raw::c_void,
          strlen(progName));
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    msg = b"\tInput file = \x00" as *const u8 as *const std::os::raw::c_char;
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    write(2 as std::os::raw::c_int, inName.as_mut_ptr() as *const std::os::raw::c_void,
          strlen(inName.as_mut_ptr()));
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    msg = b"\tOutput file = \x00" as *const u8 as *const std::os::raw::c_char;
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    write(2 as std::os::raw::c_int, outName.as_mut_ptr() as *const std::os::raw::c_void,
          strlen(outName.as_mut_ptr()));
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    /* Don't call cleanupAndFail. If we ended up here something went
      terribly wrong. Trying to clean up might fail spectacularly. */
    if opMode == 1 as std::os::raw::c_int {
        setExit(3 as std::os::raw::c_int);
    } else { setExit(2 as std::os::raw::c_int); }
    _exit(exitValue);
}
/*---------------------------------------------*/
unsafe extern "C" fn outOfMemory() -> ! {
    fprintf(__stderrp,
            b"\n%s: couldn\'t allocate enough memory\n\x00" as *const u8 as
                *const std::os::raw::c_char, progName);
    showFileNames();
    cleanUpAndFail(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn configError() -> ! {
    fprintf(__stderrp,
            b"bzip2: I\'m not configured correctly for this platform!\n\tI require Int32, Int16 and Char to have sizes\n\tof 4, 2 and 1 bytes to run properly, and they don\'t.\n\tProbably you can fix this by defining them correctly,\n\tand recompiling.  Bye!\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    setExit(3 as std::os::raw::c_int);
    exit(exitValue);
}
/*---------------------------------------------------*/
/*--- The main driver machinery                   ---*/
/*---------------------------------------------------*/
/* All rather crufty.  The main problem is that input files
   are stat()d multiple times before use.  This should be
   cleaned up. 
*/
/*---------------------------------------------*/
unsafe extern "C" fn pad(mut s: *mut Char) {
    let mut i: Int32 = 0;
    if strlen(s) as Int32 >= longestFileName { return }
    i = 1 as std::os::raw::c_int;
    while i <= longestFileName - strlen(s) as Int32 {
        fprintf(__stderrp, b" \x00" as *const u8 as *const std::os::raw::c_char);
        i += 1
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn copyFileName(mut to: *mut Char, mut from: *mut Char) {
    if strlen(from) >
           (1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as std::os::raw::c_ulong {
        fprintf(__stderrp,
                b"bzip2: file name\n`%s\'\nis suspiciously (more than %d chars) long.\nTry using a reasonable file name instead.  Sorry! :-)\n\x00"
                    as *const u8 as *const std::os::raw::c_char, from,
                1034 as std::os::raw::c_int - 10 as std::os::raw::c_int);
        setExit(1 as std::os::raw::c_int);
        exit(exitValue);
    }
    strncpy(to, from,
            (1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as std::os::raw::c_ulong);
    *to.offset((1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as isize) =
        '\u{0}' as i32 as Char;
}
/*---------------------------------------------*/
unsafe extern "C" fn fileExists(mut name: *mut Char) -> Bool {
    let mut tmp: *mut FILE =
        fopen(name, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    let mut exists: Bool =
        (tmp != 0 as *mut std::os::raw::c_void as *mut FILE) as std::os::raw::c_int as Bool;
    if !tmp.is_null() { fclose(tmp); }
    return exists;
}
/*---------------------------------------------*/
/* Open an output file safely with O_EXCL and good permissions.
   This avoids a race condition in versions < 1.0.2, in which
   the file was first opened and then had its interim permissions
   set safely.  We instead use open() to create the file with
   the interim permissions required. (--- --- rw-).

   For non-Unix platforms, if we are not worrying about
   security issues, simple this simply behaves like fopen.
*/
unsafe extern "C" fn fopen_output_safely(mut name: *mut Char,
                                         mut mode: *const std::os::raw::c_char)
 -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fh: IntNative = 0;
    fh =
        open(name,
             0x1 as std::os::raw::c_int | 0x200 as std::os::raw::c_int | 0x800 as std::os::raw::c_int,
             0o200 as std::os::raw::c_int | 0o400 as std::os::raw::c_int);
    if fh == -(1 as std::os::raw::c_int) { return 0 as *mut FILE }
    fp = fdopen(fh, mode);
    if fp.is_null() { close(fh); }
    return fp;
}
/*---------------------------------------------*/
/*--
  if in doubt, return True
--*/
unsafe extern "C" fn notAStandardFile(mut name: *mut Char) -> Bool {
    let mut i: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    i = lstat(name, &mut statBuf);
    if i != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int as Bool }
    if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
           0o100000 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as Bool
    }
    return 1 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
/*--
  rac 11/21/98 see if file has hard links to it
--*/
unsafe extern "C" fn countHardLinks(mut name: *mut Char) -> Int32 {
    let mut i: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    i = lstat(name, &mut statBuf);
    if i != 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return statBuf.st_nlink as std::os::raw::c_int - 1 as std::os::raw::c_int;
}
/*---------------------------------------------*/
/* Copy modification date, access date, permissions and owner from the
   source to destination file.  We have to copy this meta-info off
   into fileMetaInfo before starting to compress / decompress it,
   because doing it afterwards means we get the wrong access time.

   To complicate matters, in compress() and decompress() below, the
   sequence of tests preceding the call to saveInputFileMetaInfo()
   involves calling fileExists(), which in turn establishes its result
   by attempting to fopen() the file, and if successful, immediately
   fclose()ing it again.  So we have to assume that the fopen() call
   does not cause the access time field to be updated.

   Reading of the man page for stat() (man 2 stat) on RedHat 7.2 seems
   to imply that merely doing open() will not affect the access time.
   Therefore we merely need to hope that the C library only does
   open() as a result of fopen(), and not any kind of read()-ahead
   cleverness.

   It sounds pretty fragile to me.  Whether this carries across
   robustly to arbitrary Unix-like platforms (or even works robustly
   on this one, RedHat 7.2) is unknown to me.  Nevertheless ...  
*/
static mut fileMetaInfo: stat =
    stat{st_dev: 0,
         st_mode: 0,
         st_nlink: 0,
         st_ino: 0,
         st_uid: 0,
         st_gid: 0,
         st_rdev: 0,
         st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_size: 0,
         st_blocks: 0,
         st_blksize: 0,
         st_flags: 0,
         st_gen: 0,
         st_lspare: 0,
         st_qspare: [0; 2],};
unsafe extern "C" fn saveInputFileMetaInfo(mut srcName: *mut Char) {
    let mut retVal: IntNative = 0;
    /* Note use of stat here, not lstat. */
    retVal = stat(srcName, &mut fileMetaInfo);
    if retVal != 0 as std::os::raw::c_int { ioError(); };
}
unsafe extern "C" fn applySavedTimeInfoToOutputFile(mut dstName: *mut Char) {
    let mut retVal: IntNative = 0;
    let mut uTimBuf: utimbuf = utimbuf{actime: 0, modtime: 0,};
    uTimBuf.actime = fileMetaInfo.st_atimespec.tv_sec;
    uTimBuf.modtime = fileMetaInfo.st_mtimespec.tv_sec;
    retVal = utime(dstName, &mut uTimBuf);
    if retVal != 0 as std::os::raw::c_int { ioError(); };
}
unsafe extern "C" fn applySavedFileAttrToOutputFile(mut fd: IntNative) {
    let mut retVal: IntNative = 0;
    retVal = fchmod(fd, fileMetaInfo.st_mode);
    if retVal != 0 as std::os::raw::c_int { ioError(); }
    fchown(fd, fileMetaInfo.st_uid, fileMetaInfo.st_gid);
    /* chown() will in many cases return with EPERM, which can
      be safely ignored.
   */
}
/*---------------------------------------------*/
 extern "C" fn containsDubiousChars(mut name: *mut Char) -> Bool {
    /* On unix, files can contain any characters and the file expansion
    * is performed by the shell.
    */
    return 0 as std::os::raw::c_int as Bool;
    /* ! BZ_UNIX */
    /* BZ_UNIX */
}
#[no_mangle]
pub static mut zSuffix: [*const Char; 4] =
    [b".bz2\x00" as *const u8 as *const std::os::raw::c_char,
     b".bz\x00" as *const u8 as *const std::os::raw::c_char,
     b".tbz2\x00" as *const u8 as *const std::os::raw::c_char,
     b".tbz\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub static mut unzSuffix: [*const Char; 4] =
    [b"\x00" as *const u8 as *const std::os::raw::c_char,
     b"\x00" as *const u8 as *const std::os::raw::c_char,
     b".tar\x00" as *const u8 as *const std::os::raw::c_char,
     b".tar\x00" as *const u8 as *const std::os::raw::c_char];
unsafe extern "C" fn hasSuffix(mut s: *mut Char, mut suffix: *const Char)
 -> Bool {
    let mut ns: Int32 = strlen(s) as Int32;
    let mut nx: Int32 = strlen(suffix) as Int32;
    if ns < nx { return 0 as std::os::raw::c_int as Bool }
    if strcmp(s.offset(ns as isize).offset(-(nx as isize)), suffix) ==
           0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int as Bool
    }
    return 0 as std::os::raw::c_int as Bool;
}
unsafe extern "C" fn mapSuffix(mut name: *mut Char,
                               mut oldSuffix: *const Char,
                               mut newSuffix: *const Char) -> Bool {
    if hasSuffix(name, oldSuffix) == 0 { return 0 as std::os::raw::c_int as Bool }
    *name.offset(strlen(name).wrapping_sub(strlen(oldSuffix)) as isize) =
        0 as std::os::raw::c_int as Char;
    strcat(name, newSuffix);
    return 1 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
unsafe extern "C" fn compress(mut name: *mut Char) {
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic(b"compress: bad modes\n\x00" as *const u8 as
                  *const std::os::raw::c_char);
    }
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            strcat(outName.as_mut_ptr(),
                   b".bz2\x00" as *const u8 as *const std::os::raw::c_char);
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input file %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1 as std::os::raw::c_int);
        return
    }
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        if hasSuffix(inName.as_mut_ptr(), zSuffix[i as usize]) != 0 {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"%s: Input file %s already has %s suffix.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), zSuffix[i as usize]);
            }
            setExit(1 as std::os::raw::c_int);
            return
        }
        i += 1
    }
    if srcMode == 3 as std::os::raw::c_int || srcMode == 2 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           notAStandardFile(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Input file %s is not a normal file.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int &&
           fileExists(outName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(__stderrp,
                    b"%s: Output file %s already exists.\n\x00" as *const u8
                        as *const std::os::raw::c_char, progName,
                    outName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           { n = countHardLinks(inName.as_mut_ptr()); (n) > 0 as std::os::raw::c_int }
       {
        fprintf(__stderrp,
                b"%s: Input file %s has %d other link%s.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName, inName.as_mut_ptr(), n,
                if n > 1 as std::os::raw::c_int {
                    b"s\x00" as *const u8 as *const std::os::raw::c_char
                } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int {
        /* Save the file's meta-info before we open it.  Doing it later
         means we mess up the access times. */
        saveInputFileMetaInfo(inName.as_mut_ptr());
    }
    match srcMode {
        1 => {
            inStr = __stdinp;
            outStr = __stdoutp;
            if isatty(fileno(__stdoutp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t write compressed data to a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        2 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr = __stdoutp;
            if isatty(fileno(__stdoutp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t write compressed data to a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr =
                fopen_output_safely(outName.as_mut_ptr(),
                                    b"wb\x00" as *const u8 as
                                        *const std::os::raw::c_char);
            if outStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t create output file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !outStr.is_null() { fclose(outStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        _ => {
            panic(b"compress: bad srcMode\x00" as *const u8 as
                      *const std::os::raw::c_char);
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(inName.as_mut_ptr());
        fflush(__stderrp);
    }
    /*--- Now the input and output handles are sane.  Do the Biz. ---*/
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as std::os::raw::c_int as Bool;
    compressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    /*--- If there was an I/O error, we won't get here. ---*/
    if srcMode == 3 as std::os::raw::c_int {
        applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
        deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
        if keepInputFiles == 0 {
            let mut retVal: IntNative = remove(inName.as_mut_ptr());
            if retVal != 0 as std::os::raw::c_int { ioError(); }
        }
    }
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
unsafe extern "C" fn uncompress(mut name: *mut Char) {
    let mut current_block: u64;
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut magicNumberOK: Bool = 0;
    let mut cantGuess: Bool = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic(b"uncompress: bad modes\n\x00" as *const u8 as
                  *const std::os::raw::c_char);
    }
    cantGuess = 0 as std::os::raw::c_int as Bool;
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            i = 0 as std::os::raw::c_int;
            loop  {
                if !(i < 4 as std::os::raw::c_int) {
                    current_block = 17860125682698302841;
                    break ;
                }
                if mapSuffix(outName.as_mut_ptr(), zSuffix[i as usize],
                             unzSuffix[i as usize]) != 0 {
                    current_block = 15314513098708193206;
                    break ;
                }
                i += 1
            }
            match current_block {
                15314513098708193206 => { }
                _ => {
                    cantGuess = 1 as std::os::raw::c_int as Bool;
                    strcat(outName.as_mut_ptr(),
                           b".out\x00" as *const u8 as *const std::os::raw::c_char);
                }
            }
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input file %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int || srcMode == 2 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           notAStandardFile(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Input file %s is not a normal file.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if cantGuess != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Can\'t guess original name for %s -- using %s\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr(), outName.as_mut_ptr());
        }
        /* just a warning, no return */
    }
    if srcMode == 3 as std::os::raw::c_int &&
           fileExists(outName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(__stderrp,
                    b"%s: Output file %s already exists.\n\x00" as *const u8
                        as *const std::os::raw::c_char, progName,
                    outName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           { n = countHardLinks(inName.as_mut_ptr()); (n) > 0 as std::os::raw::c_int }
       {
        fprintf(__stderrp,
                b"%s: Input file %s has %d other link%s.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName, inName.as_mut_ptr(), n,
                if n > 1 as std::os::raw::c_int {
                    b"s\x00" as *const u8 as *const std::os::raw::c_char
                } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int {
        /* Save the file's meta-info before we open it.  Doing it later
         means we mess up the access times. */
        saveInputFileMetaInfo(inName.as_mut_ptr());
    }
    match srcMode {
        1 => {
            inStr = __stdinp;
            outStr = __stdoutp;
            if isatty(fileno(__stdinp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t read compressed data from a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        2 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr = __stdoutp;
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s:%s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr =
                fopen_output_safely(outName.as_mut_ptr(),
                                    b"wb\x00" as *const u8 as
                                        *const std::os::raw::c_char);
            if outStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t create output file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !outStr.is_null() { fclose(outStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        _ => {
            panic(b"uncompress: bad srcMode\x00" as *const u8 as
                      *const std::os::raw::c_char);
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(inName.as_mut_ptr());
        fflush(__stderrp);
    }
    /*--- Now the input and output handles are sane.  Do the Biz. ---*/
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as std::os::raw::c_int as Bool;
    magicNumberOK = uncompressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    /*--- If there was an I/O error, we won't get here. ---*/
    if magicNumberOK != 0 {
        if srcMode == 3 as std::os::raw::c_int {
            applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
            deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
            if keepInputFiles == 0 {
                let mut retVal: IntNative = remove(inName.as_mut_ptr());
                if retVal != 0 as std::os::raw::c_int { ioError(); }
            }
        }
    } else {
        unzFailsExist = 1 as std::os::raw::c_int as Bool;
        deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
        if srcMode == 3 as std::os::raw::c_int {
            let mut retVal_0: IntNative = remove(outName.as_mut_ptr());
            if retVal_0 != 0 as std::os::raw::c_int { ioError(); }
        }
    }
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if magicNumberOK != 0 {
        if verbosity >= 1 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"done\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
    } else {
        setExit(2 as std::os::raw::c_int);
        if verbosity >= 1 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"not a bzip2 file.\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        } else {
            fprintf(__stderrp,
                    b"%s: %s is not a bzip2 file.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
        }
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn testf(mut name: *mut Char) {
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut allOK: Bool = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic(b"testf: bad modes\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    copyFileName(outName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
        }
        3 => { copyFileName(inName.as_mut_ptr(), name); }
        2 => { copyFileName(inName.as_mut_ptr(), name); }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    match srcMode {
        1 => {
            if isatty(fileno(__stdinp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t read compressed data from a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1 as std::os::raw::c_int);
                return
            }
            inStr = __stdinp
        }
        2 | 3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s:%s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        _ => {
            panic(b"testf: bad srcMode\x00" as *const u8 as
                      *const std::os::raw::c_char);
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(inName.as_mut_ptr());
        fflush(__stderrp);
    }
    /*--- Now the input handle is sane.  Do the Biz. ---*/
    outputHandleJustInCase = 0 as *mut FILE;
    allOK = testStream(inStr);
    if allOK as std::os::raw::c_int != 0 && verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"ok\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    if allOK == 0 { testFailsExist = 1 as std::os::raw::c_int as Bool };
}
/*---------------------------------------------*/
unsafe extern "C" fn license() {
    fprintf(__stderrp,
            b"bzip2, a block-sorting file compressor.  Version %s.\n   \n   Copyright (C) 1996-2019 by Julian Seward.\n   \n   This program is free software; you can redistribute it and/or modify\n   it under the terms set out in the LICENSE file, which is included\n   in the bzip2 source distribution.\n   \n   This program is distributed in the hope that it will be useful,\n   but WITHOUT ANY WARRANTY; without even the implied warranty of\n   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n   LICENSE file for more details.\n   \n\x00"
                as *const u8 as *const std::os::raw::c_char, BZ2_bzlibVersion());
}
/*---------------------------------------------*/
unsafe extern "C" fn usage(mut fullProgName: *mut Char) {
    fprintf(__stderrp,
            b"bzip2, a block-sorting file compressor.  Version %s.\n\n   usage: %s [flags and input files in any order]\n\n   -h --help           print this message\n   -d --decompress     force decompression\n   -z --compress       force compression\n   -k --keep           keep (don\'t delete) input files\n   -f --force          overwrite existing output files\n   -t --test           test compressed file integrity\n   -c --stdout         output to standard out\n   -q --quiet          suppress noncritical error messages\n   -v --verbose        be verbose (a 2nd -v gives more)\n   -L --license        display software version & license\n   -V --version        display software version & license\n   -s --small          use less memory (at most 2500k)\n   -1 .. -9            set block size to 100k .. 900k\n   --fast              alias for -1\n   --best              alias for -9\n\n   If invoked as `bzip2\', default action is to compress.\n              as `bunzip2\',  default action is to decompress.\n              as `bzcat\', default action is to decompress to stdout.\n\n   If no file names are given, bzip2 compresses or decompresses\n   from standard input to standard output.  You can combine\n   short flags, so `-v -4\' means the same as -v4 or -4v, &c.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char, BZ2_bzlibVersion(),
            fullProgName);
}
/*---------------------------------------------*/
unsafe extern "C" fn redundant(mut flag: *mut Char) {
    fprintf(__stderrp,
            b"%s: %s is redundant in versions 0.9.5 and above\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName, flag);
}
/*---------------------------------------------*/
unsafe extern "C" fn myMalloc(mut n: Int32) -> *mut std::os::raw::c_void {
    let mut p: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    p = malloc(n as size_t);
    if p.is_null() { outOfMemory(); }
    return p;
}
/*---------------------------------------------*/
unsafe extern "C" fn mkCell() -> *mut Cell {
    let mut c: *mut Cell = 0 as *mut Cell;
    c =
        myMalloc(::std::mem::size_of::<Cell>() as std::os::raw::c_ulong as Int32) as
            *mut Cell;
    (*c).name = 0 as *mut Char;
    (*c).link = 0 as *mut zzzz;
    return c;
}
/*---------------------------------------------*/
unsafe extern "C" fn snocString(mut root: *mut Cell, mut name: *mut Char)
 -> *mut Cell {
    if root.is_null() {
        let mut tmp: *mut Cell = mkCell();
        (*tmp).name =
            myMalloc((5 as std::os::raw::c_int as
                          std::os::raw::c_ulong).wrapping_add(strlen(name)) as Int32)
                as *mut Char;
        strcpy((*tmp).name, name);
        return tmp
    } else {
        let mut tmp_0: *mut Cell = root;
        while !(*tmp_0).link.is_null() { tmp_0 = (*tmp_0).link }
        (*tmp_0).link = snocString((*tmp_0).link, name);
        return root
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn addFlagsFromEnvVar(mut argList: *mut *mut Cell,
                                        mut varName: *mut Char) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut envbase: *mut Char = 0 as *mut Char;
    let mut p: *mut Char = 0 as *mut Char;
    envbase = getenv(varName);
    if !envbase.is_null() {
        p = envbase;
        i = 0 as std::os::raw::c_int;
        while 1 as std::os::raw::c_int as Bool != 0 {
            if *p.offset(i as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int {
                break ;
            }
            p = p.offset(i as isize);
            i = 0 as std::os::raw::c_int;
            while isspace(*p.offset(0 as std::os::raw::c_int as isize) as Int32) != 0
                  {
                p = p.offset(1)
            }
            while *p.offset(i as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      isspace(*p.offset(i as isize) as Int32) == 0 {
                i += 1
            }
            if i > 0 as std::os::raw::c_int {
                k = i;
                if k > 1034 as std::os::raw::c_int - 10 as std::os::raw::c_int {
                    k = 1034 as std::os::raw::c_int - 10 as std::os::raw::c_int
                }
                j = 0 as std::os::raw::c_int;
                while j < k {
                    tmpName[j as usize] = *p.offset(j as isize);
                    j += 1
                }
                tmpName[k as usize] = 0 as std::os::raw::c_int as Char;
                *argList = snocString(*argList, tmpName.as_mut_ptr())
            }
        }
    };
}
unsafe fn main_0(mut argc: IntNative, mut argv: *mut *mut Char) -> IntNative {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: *mut Char = 0 as *mut Char;
    let mut argList: *mut Cell = 0 as *mut Cell;
    let mut aa: *mut Cell = 0 as *mut Cell;
    let mut decode: Bool = 0;
    /*-- Be really really really paranoid :-) --*/
    if ::std::mem::size_of::<Int32>() as std::os::raw::c_ulong !=
           4 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UInt32>() as std::os::raw::c_ulong !=
               4 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<Int16>() as std::os::raw::c_ulong !=
               2 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UInt16>() as std::os::raw::c_ulong !=
               2 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<Char>() as std::os::raw::c_ulong !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong {
        configError();
    }
    /*-- Initialise --*/
    outputHandleJustInCase =
        0 as *mut FILE; /* avoid bogus warning from egcs-1.1.X */
    smallMode = 0 as std::os::raw::c_int as Bool;
    keepInputFiles = 0 as std::os::raw::c_int as Bool;
    forceOverwrite = 0 as std::os::raw::c_int as Bool;
    noisy = 1 as std::os::raw::c_int as Bool;
    verbosity = 0 as std::os::raw::c_int;
    blockSize100k = 9 as std::os::raw::c_int;
    testFailsExist = 0 as std::os::raw::c_int as Bool;
    unzFailsExist = 0 as std::os::raw::c_int as Bool;
    numFileNames = 0 as std::os::raw::c_int;
    numFilesProcessed = 0 as std::os::raw::c_int;
    workFactor = 30 as std::os::raw::c_int;
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    exitValue = 0 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    i = j;
    /*-- Set up signal handlers for mem access errors --*/
    signal(11 as std::os::raw::c_int,
           Some(mySIGSEGVorSIGBUScatcher as
                    unsafe extern "C" fn(_: IntNative) -> ()));
    signal(10 as std::os::raw::c_int,
           Some(mySIGSEGVorSIGBUScatcher as
                    unsafe extern "C" fn(_: IntNative) -> ()));
    copyFileName(inName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    copyFileName(outName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    copyFileName(progNameReally.as_mut_ptr(),
                 *argv.offset(0 as std::os::raw::c_int as isize));
    progName =
        &mut *progNameReally.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
            *mut Char;
    tmp =
        &mut *progNameReally.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
            *mut Char;
    while *tmp as std::os::raw::c_int != '\u{0}' as i32 {
        if *tmp as std::os::raw::c_int == '/' as i32 {
            progName = tmp.offset(1 as std::os::raw::c_int as isize)
        }
        tmp = tmp.offset(1)
    }
    /*-- Copy flags from env var BZIP2, and 
        expand filename wildcards in arg list.
   --*/
    argList = 0 as *mut Cell;
    addFlagsFromEnvVar(&mut argList,
                       b"BZIP2\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut Char);
    addFlagsFromEnvVar(&mut argList,
                       b"BZIP\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut Char);
    i = 1 as std::os::raw::c_int;
    while i <= argc - 1 as std::os::raw::c_int {
        argList = snocString(argList, *argv.offset(i as isize));
        i += 1
    }
    /*-- Find the length of the longest filename --*/
    longestFileName = 7 as std::os::raw::c_int;
    numFileNames = 0 as std::os::raw::c_int;
    decode = 1 as std::os::raw::c_int as Bool;
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            decode = 0 as std::os::raw::c_int as Bool
        } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_int == '-' as i32 &&
                        decode as std::os::raw::c_int != 0) {
            numFileNames += 1;
            if longestFileName < strlen((*aa).name) as Int32 {
                longestFileName = strlen((*aa).name) as Int32
            }
        }
        aa = (*aa).link
    }
    /*-- Determine source modes; flag handling may change this too. --*/
    if numFileNames == 0 as std::os::raw::c_int {
        srcMode = 1 as std::os::raw::c_int
    } else { srcMode = 3 as std::os::raw::c_int }
    /*-- Determine what to do (compress/uncompress/test/cat). --*/
   /*-- Note that subsequent flag handling may change this. --*/
    opMode = 1 as std::os::raw::c_int;
    if !strstr(progName,
               b"unzip\x00" as *const u8 as *const std::os::raw::c_char).is_null() ||
           !strstr(progName,
                   b"UNZIP\x00" as *const u8 as *const std::os::raw::c_char).is_null()
       {
        opMode = 2 as std::os::raw::c_int
    }
    if !strstr(progName,
               b"z2cat\x00" as *const u8 as *const std::os::raw::c_char).is_null() ||
           !strstr(progName,
                   b"Z2CAT\x00" as *const u8 as *const std::os::raw::c_char).is_null()
           ||
           !strstr(progName,
                   b"zcat\x00" as *const u8 as *const std::os::raw::c_char).is_null()
           ||
           !strstr(progName,
                   b"ZCAT\x00" as *const u8 as *const std::os::raw::c_char).is_null()
       {
        opMode = 2 as std::os::raw::c_int;
        srcMode =
            if numFileNames == 0 as std::os::raw::c_int {
                1 as std::os::raw::c_int
            } else { 2 as std::os::raw::c_int }
    }
    /*-- Look at the flags. --*/
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            break ;
        }
        if *(*aa).name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '-' as i32 &&
               *(*aa).name.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                   '-' as i32 {
            j = 1 as std::os::raw::c_int;
            while *(*aa).name.offset(j as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
                match *(*aa).name.offset(j as isize) as std::os::raw::c_int {
                    99 => { srcMode = 2 as std::os::raw::c_int }
                    100 => { opMode = 2 as std::os::raw::c_int }
                    122 => { opMode = 1 as std::os::raw::c_int }
                    102 => { forceOverwrite = 1 as std::os::raw::c_int as Bool }
                    116 => { opMode = 3 as std::os::raw::c_int }
                    107 => { keepInputFiles = 1 as std::os::raw::c_int as Bool }
                    115 => { smallMode = 1 as std::os::raw::c_int as Bool }
                    113 => { noisy = 0 as std::os::raw::c_int as Bool }
                    49 => { blockSize100k = 1 as std::os::raw::c_int }
                    50 => { blockSize100k = 2 as std::os::raw::c_int }
                    51 => { blockSize100k = 3 as std::os::raw::c_int }
                    52 => { blockSize100k = 4 as std::os::raw::c_int }
                    53 => { blockSize100k = 5 as std::os::raw::c_int }
                    54 => { blockSize100k = 6 as std::os::raw::c_int }
                    55 => { blockSize100k = 7 as std::os::raw::c_int }
                    56 => { blockSize100k = 8 as std::os::raw::c_int }
                    57 => { blockSize100k = 9 as std::os::raw::c_int }
                    86 | 76 => { license(); }
                    118 => { verbosity += 1 }
                    104 => { usage(progName); exit(0 as std::os::raw::c_int); }
                    _ => {
                        fprintf(__stderrp,
                                b"%s: Bad flag `%s\'\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, progName,
                                (*aa).name);
                        usage(progName);
                        exit(1 as std::os::raw::c_int);
                    }
                }
                j += 1
            }
        }
        aa = (*aa).link
    }
    /*-- And again ... --*/
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            break ;
        }
        if strcmp((*aa).name,
                  b"--stdout\x00" as *const u8 as *const std::os::raw::c_char) ==
               0 as std::os::raw::c_int {
            srcMode = 2 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--decompress\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            opMode = 2 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--compress\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            opMode = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--force\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            forceOverwrite = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--test\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            opMode = 3 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--keep\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            keepInputFiles = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--small\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            smallMode = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--quiet\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            noisy = 0 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--version\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            license();
        } else if strcmp((*aa).name,
                         b"--license\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            license();
        } else if strcmp((*aa).name,
                         b"--exponential\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            workFactor = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--repetitive-best\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            redundant((*aa).name);
        } else if strcmp((*aa).name,
                         b"--repetitive-fast\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            redundant((*aa).name);
        } else if strcmp((*aa).name,
                         b"--fast\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            blockSize100k = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--best\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            blockSize100k = 9 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--verbose\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            verbosity += 1
        } else if strcmp((*aa).name,
                         b"--help\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            usage(progName);
            exit(0 as std::os::raw::c_int);
        } else {
            if strncmp((*aa).name,
                       b"--\x00" as *const u8 as *const std::os::raw::c_char,
                       2 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int
               {
                fprintf(__stderrp,
                        b"%s: Bad flag `%s\'\n\x00" as *const u8 as
                            *const std::os::raw::c_char, progName, (*aa).name);
                usage(progName);
                exit(1 as std::os::raw::c_int);
            }
        }
        aa = (*aa).link
    }
    if verbosity > 4 as std::os::raw::c_int { verbosity = 4 as std::os::raw::c_int }
    if opMode == 1 as std::os::raw::c_int && smallMode as std::os::raw::c_int != 0 &&
           blockSize100k > 2 as std::os::raw::c_int {
        blockSize100k = 2 as std::os::raw::c_int
    }
    if opMode == 3 as std::os::raw::c_int && srcMode == 2 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"%s: -c and -t cannot be used together.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName);
        exit(1 as std::os::raw::c_int);
    }
    if srcMode == 2 as std::os::raw::c_int && numFileNames == 0 as std::os::raw::c_int {
        srcMode = 1 as std::os::raw::c_int
    }
    if opMode != 1 as std::os::raw::c_int { blockSize100k = 0 as std::os::raw::c_int }
    if srcMode == 3 as std::os::raw::c_int {
        signal(2 as std::os::raw::c_int,
               Some(mySignalCatcher as
                        unsafe extern "C" fn(_: IntNative) -> ()));
        signal(15 as std::os::raw::c_int,
               Some(mySignalCatcher as
                        unsafe extern "C" fn(_: IntNative) -> ()));
        signal(1 as std::os::raw::c_int,
               Some(mySignalCatcher as
                        unsafe extern "C" fn(_: IntNative) -> ()));
    }
    if opMode == 1 as std::os::raw::c_int {
        if srcMode == 1 as std::os::raw::c_int {
            compress(0 as *mut Char);
        } else {
            decode = 1 as std::os::raw::c_int as Bool;
            aa = argList;
            while !aa.is_null() {
                if strcmp((*aa).name,
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    decode = 0 as std::os::raw::c_int as Bool
                } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int == '-' as i32 &&
                                decode as std::os::raw::c_int != 0) {
                    numFilesProcessed += 1;
                    compress((*aa).name);
                }
                aa = (*aa).link
            }
        }
    } else if opMode == 2 as std::os::raw::c_int {
        unzFailsExist = 0 as std::os::raw::c_int as Bool;
        if srcMode == 1 as std::os::raw::c_int {
            uncompress(0 as *mut Char);
        } else {
            decode = 1 as std::os::raw::c_int as Bool;
            aa = argList;
            while !aa.is_null() {
                if strcmp((*aa).name,
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    decode = 0 as std::os::raw::c_int as Bool
                } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int == '-' as i32 &&
                                decode as std::os::raw::c_int != 0) {
                    numFilesProcessed += 1;
                    uncompress((*aa).name);
                }
                aa = (*aa).link
            }
        }
        if unzFailsExist != 0 { setExit(2 as std::os::raw::c_int); exit(exitValue); }
    } else {
        testFailsExist = 0 as std::os::raw::c_int as Bool;
        if srcMode == 1 as std::os::raw::c_int {
            testf(0 as *mut Char);
        } else {
            decode = 1 as std::os::raw::c_int as Bool;
            aa = argList;
            while !aa.is_null() {
                if strcmp((*aa).name,
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    decode = 0 as std::os::raw::c_int as Bool
                } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int == '-' as i32 &&
                                decode as std::os::raw::c_int != 0) {
                    numFilesProcessed += 1;
                    testf((*aa).name);
                }
                aa = (*aa).link
            }
        }
        if testFailsExist != 0 {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"\nYou can use the `bzip2recover\' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
            }
            setExit(2 as std::os::raw::c_int);
            exit(exitValue);
        }
    }
    /* Free the argument list memory to mollify leak detectors 
      (eg) Purify, Checker.  Serves no other useful purpose.
   */
    aa = argList;
    while !aa.is_null() {
        let mut aa2: *mut Cell = (*aa).link;
        if !(*aa).name.is_null() { free((*aa).name as *mut std::os::raw::c_void); }
        free(aa as *mut std::os::raw::c_void);
        aa = aa2
    }
    return exitValue;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as IntNative,
                                    args.as_mut_ptr() as *mut *mut Char) as
                                 i32)
    }
}
/*-----------------------------------------------------------*/
/*--- end                                         bzip2.c ---*/
/*-----------------------------------------------------------*/

}

// root re-exports so the generated harness's `translated::<entry>` resolves
pub use crate::blocksort::BZ2_blockSort;
pub use crate::compress::BZ2_bsInitWrite;
pub use crate::bzlib::BZ2_bzBuffToBuffCompress;
pub use crate::bzlib::BZ2_bzBuffToBuffDecompress;
pub use crate::bzlib::BZ2_bzCompress;
pub use crate::bzlib::BZ2_bzCompressEnd;
pub use crate::bzlib::BZ2_bzCompressInit;
pub use crate::bzlib::BZ2_bzDecompress;
pub use crate::bzlib::BZ2_bzDecompressEnd;
pub use crate::bzlib::BZ2_bzDecompressInit;
pub use crate::bzlib::BZ2_bzRead;
pub use crate::bzlib::BZ2_bzReadClose;
pub use crate::bzlib::BZ2_bzReadGetUnused;
pub use crate::bzlib::BZ2_bzReadOpen;
pub use crate::bzlib::BZ2_bzWrite;
pub use crate::bzlib::BZ2_bzWriteClose;
pub use crate::bzlib::BZ2_bzWriteClose64;
pub use crate::bzlib::BZ2_bzWriteOpen;
pub use crate::bzlib::BZ2_bz__AssertH__fail;
pub use crate::bzlib::BZ2_bzclose;
pub use crate::bzlib::BZ2_bzdopen;
pub use crate::bzlib::BZ2_bzerror;
pub use crate::bzlib::BZ2_bzflush;
pub use crate::bzlib::BZ2_bzlibVersion;
pub use crate::bzlib::BZ2_bzopen;
pub use crate::bzlib::BZ2_bzread;
pub use crate::bzlib::BZ2_bzwrite;
pub use crate::compress::BZ2_compressBlock;
pub use crate::decompress::BZ2_decompress;
pub use crate::huffman::BZ2_hbAssignCodes;
pub use crate::huffman::BZ2_hbCreateDecodeTables;
pub use crate::huffman::BZ2_hbMakeCodeLengths;
pub use crate::bzlib::BZ2_indexIntoF;
// __isctype: private in bzlib — exposed per-entry by --expose-entry
// add_pair_to_block: private in bzlib — exposed per-entry by --expose-entry
// bsFinishWrite: private in compress — exposed per-entry by --expose-entry
// bsPutUChar: private in compress — exposed per-entry by --expose-entry
// bsPutUInt32: private in compress — exposed per-entry by --expose-entry
// bsW: private in compress — exposed per-entry by --expose-entry
// bz_config_ok: private in bzlib — exposed per-entry by --expose-entry
// bzopen_or_bzdopen: private in bzlib — exposed per-entry by --expose-entry
// copy_input_until_stop: private in bzlib — exposed per-entry by --expose-entry
// copy_output_until_stop: private in bzlib — exposed per-entry by --expose-entry
// default_bzalloc: private in bzlib — exposed per-entry by --expose-entry
// default_bzfree: private in bzlib — exposed per-entry by --expose-entry
// fallbackQSort3: private in blocksort — exposed per-entry by --expose-entry
// fallbackSimpleSort: private in blocksort — exposed per-entry by --expose-entry
// fallbackSort: private in blocksort — exposed per-entry by --expose-entry
// flush_RL: private in bzlib — exposed per-entry by --expose-entry
// generateMTFValues: private in compress — exposed per-entry by --expose-entry
// handle_compress: private in bzlib — exposed per-entry by --expose-entry
// init_RL: private in bzlib — exposed per-entry by --expose-entry
pub use crate::bzlib::isdigit;
// isempty_RL: private in bzlib — exposed per-entry by --expose-entry
// mainGtU: private in blocksort — exposed per-entry by --expose-entry
// mainQSort3: private in blocksort — exposed per-entry by --expose-entry
// mainSimpleSort: private in blocksort — exposed per-entry by --expose-entry
// mainSort: private in blocksort — exposed per-entry by --expose-entry
// makeMaps_d: private in decompress — exposed per-entry by --expose-entry
// makeMaps_e: private in compress — exposed per-entry by --expose-entry
// mmed3: private in blocksort — exposed per-entry by --expose-entry
// myfeof: private in bzlib — exposed per-entry by --expose-entry
// prepare_new_block: private in bzlib — exposed per-entry by --expose-entry
// sendMTFValues: private in compress — exposed per-entry by --expose-entry
// unRLE_obuf_to_output_FAST: private in bzlib — exposed per-entry by --expose-entry
// unRLE_obuf_to_output_SMALL: private in bzlib — exposed per-entry by --expose-entry

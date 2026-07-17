




















use std::slice;

extern "C" {
    
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
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

pub type __sbuf = crate::bzip2::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::bzip2::__sFILE;
pub type FILE = crate::blocksort::FILE;
// #[derive(Copy, Clone)]

pub type bz_stream = crate::bzlib::bz_stream;
pub type Bool = crate::blocksort::Bool;
pub type UChar = crate::blocksort::UChar;
pub type Int32 = crate::blocksort::Int32;
pub type UInt32 = crate::blocksort::UInt32;
pub type UInt16 = crate::blocksort::UInt16;
// #[derive(Copy, Clone)]

pub type EState = crate::bzlib::EState;
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

pub fn BZ2_bsInitWrite(s: &mut EState) {
    s.bsLive = 0;
    s.bsBuff = 0;
}

/*---------------------------------------------------*/
fn bsFinishWrite(s: &mut EState) {
    let zbits_slice = unsafe { std::slice::from_raw_parts_mut(s.zbits, s.numZ as usize + s.bsLive as usize / 8) };
    while s.bsLive > 0 {
        zbits_slice[s.numZ as usize] = (s.bsBuff >> 24) as UChar;
        s.numZ += 1;
        s.bsBuff <<= 8;
        s.bsLive -= 8;
    }
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
fn bsPutUInt32(s: &mut EState, u: u32) {
    let bytes = [
        (u >> 24) as u8,
        (u >> 16) as u8,
        (u >> 8) as u8,
        (u & 0xff) as u8,
    ];
    for byte in bytes.iter() {
        unsafe {
            bsW(s, 8, *byte as u32);
        }
    }
}

/*---------------------------------------------------*/
fn bsPutUChar(s: &mut EState, c: UChar) {
    unsafe {
        bsW(s, 8, c as UInt32);
    }
}

/*---------------------------------------------------*/
/*--- The back end proper                         ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
fn makeMaps_e(s: *mut EState) {
    let s = unsafe { &mut *s };
    s.nInUse = 0;
    for i in 0..256 {
        if s.inUse[i] != 0 {
            s.unseqToSeq[i] = s.nInUse as UChar;
            s.nInUse += 1;
        }
    }
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
        BZ2_bz__AssertH__fail(3001);
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
        let mut t = 0;
while t < nGroups {
    fave[t as usize] = 0;
    t += 1;
}

let mut t = 0;
while t < nGroups {
    let mut v = 0;
    while v < alphaSize {
        (*s).rfreq[t as usize][v as usize] = 0;
        v += 1;
    }
    t += 1;
}

/*---
Set up an auxiliary length table which is used to fast-track
the common case (nGroups == 6). 
---*/
if nGroups == 6 {
    let mut v = 0;
    while v < alphaSize {
        let len0 = (*s).len[0][v as usize] as u32;
        let len1 = (*s).len[1][v as usize] as u32;
        (*s).len_pack[v as usize][0] = (len1 << 16) | len0;

        let len2 = (*s).len[2][v as usize] as u32;
        let len3 = (*s).len[3][v as usize] as u32;
        (*s).len_pack[v as usize][1] = (len3 << 16) | len2;

        let len4 = (*s).len[4][v as usize] as u32;
        let len5 = (*s).len[5][v as usize] as u32;
        (*s).len_pack[v as usize][2] = (len5 << 16) | len4;

        v += 1;
    }
}

let mut nSelectors = 0;
let mut totc = 0;
let mut gs = 0;
loop {
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
                let len_pack = unsafe { &(*s).len_pack }; // Dereferencing the raw pointer to access len_pack
let mtfv_length =  /* Define the length of mtfv appropriately based on your context */ 1024; // Placeholder for the length of mtfv
let mtfv_slice = unsafe { std::slice::from_raw_parts(mtfv, mtfv_length) }; // Create a slice from the raw pointer
let mut icv_index = gs + 6;

for i in 0..5 {
    let icv = mtfv_slice[icv_index as usize];
    cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
    cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
    cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);
    icv_index += 1;
}

                let len_pack = unsafe { &(*s).len_pack }; // Dereferencing the raw pointer to access len_pack
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);
icv = unsafe { *mtfv.offset((gs + 11) as isize) }; // Using offset to access the value
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);
icv = unsafe { *mtfv.offset((gs + 12) as isize) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);
icv = unsafe { *mtfv.offset((gs + 13) as isize) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);
icv = unsafe { *mtfv.offset((gs + 14) as isize) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);
icv = unsafe { *mtfv.offset((gs + 15) as isize) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);

                let s = unsafe { &*s }; // Dereference the raw pointer to get a reference to EState
let mut cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);
let mut icv: u16;

icv = unsafe { *mtfv.offset((gs + 16) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 17) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 18) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 19) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 20) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 21) as isize) };

                let len_pack = &s.len_pack;
let mut icv_index = |index: i32| (gs + index) as isize;

cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset(icv_index(22)) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset(icv_index(23)) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset(icv_index(24)) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset(icv_index(25)) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset(icv_index(26)) };
cost01 = cost01.wrapping_add(len_pack[icv as usize][0]);

                let len_pack = &s.len_pack;

for offset in 27..=31 {
    let icv_index = (gs + offset) as isize;
    let icv = unsafe { *mtfv.offset(icv_index) } as usize;

    cost01 = cost01.wrapping_add(len_pack[icv][0]);
    cost23 = cost23.wrapping_add(len_pack[icv][1]);
    cost45 = cost45.wrapping_add(len_pack[icv][2]);
}

                let mut cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);
let mut icv = unsafe { *mtfv.offset((gs + 32) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let mut icv = unsafe { *mtfv.offset((gs + 33) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let mut icv = unsafe { *mtfv.offset((gs + 34) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let mut icv = unsafe { *mtfv.offset((gs + 35) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let mut icv = unsafe { *mtfv.offset((gs + 36) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let mut icv = unsafe { *mtfv.offset((gs + 37) as isize) };

                cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 38) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 39) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 40) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 41) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

icv = unsafe { *mtfv.offset((gs + 42) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);

                let mut cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
let mut cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let icv_index = (gs + 43) as usize;
let mut icv = unsafe { *mtfv.add(icv_index) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let icv_index = (gs + 44) as usize;
icv = unsafe { *mtfv.add(icv_index) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let icv_index = (gs + 45) as usize;
icv = unsafe { *mtfv.add(icv_index) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let icv_index = (gs + 46) as usize;
icv = unsafe { *mtfv.add(icv_index) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

let icv_index = (gs + 47) as usize;
icv = unsafe { *mtfv.add(icv_index) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);

                let mut cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);
icv = unsafe { *mtfv.offset((gs + 48) as isize) };
let mut cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
let mut cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);
icv = unsafe { *mtfv.offset((gs + 49) as isize) };
cost01 = cost01.wrapping_add(s.len_pack[icv as usize][0]);
cost23 = cost23.wrapping_add(s.len_pack[icv as usize][1]);
cost45 = cost45.wrapping_add(s.len_pack[icv as usize][2]);

cost[0] = (cost01 & 0xffff) as u16;
cost[1] = (cost01 >> 16) as u16;
cost[2] = (cost23 & 0xffff) as u16;
cost[3] = (cost23 >> 16) as u16;
cost[4] = (cost45 & 0xffff) as u16;
cost[5] = (cost45 >> 16) as u16;

            } else {
                /*--- slow version which correctly handles all situations ---*/
                let mut i = gs;
while i <= ge {
    let icv_0 = unsafe { *mtfv.offset(i as isize) };
    for t in 0..nGroups {
        cost[t as usize] = cost[t as usize].wrapping_add(unsafe { (*s).len[t as usize][icv_0 as usize] as u16 });
    }
    i += 1;
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
                let mtfv_slice = unsafe { std::slice::from_raw_parts(mtfv, 50) }; // Assuming mtfv has at least 50 elements
for i in 0..50 {
    let index = gs + i;
    let value = mtfv_slice[index as usize];
    (*s).rfreq[bt as usize][value as usize] += 1;
}

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

if (*s).verbosity >= 3 {
    eprintln!(
        "      pass {}: size is {}, grp uses are ",
        iter + 1,
        totc / 8
    );
    let mut t = 0;
    while t < nGroups {
        eprint!("{} ", fave[t as usize]);
        t += 1;
    }
    eprintln!();
}

/*--
Recompute the tables based on the accumulated frequencies.
--*/
/* maxLen was changed from 20 to 17 in bzip2-1.0.3.  See 
   comment in huffman.c for details. */
let mut t = 0;
while t < nGroups {
    unsafe {
        BZ2_hbMakeCodeLengths(
            (*s).len[t as usize].as_mut_ptr(),
            (*s).rfreq[t as usize].as_mut_ptr(),
            alphaSize,
            17,
        );
    }
    t += 1;
}
iter += 1;

    }
    if !(nGroups < 8 as std::os::raw::c_int) {
        BZ2_bz__AssertH__fail(3002);
    }
    if !(nSelectors < 32768 as std::os::raw::c_int &&
             nSelectors <=
                 2 as std::os::raw::c_int + 900000 as std::os::raw::c_int / 50 as std::os::raw::c_int)
       {
        BZ2_bz__AssertH__fail(3003);
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
    minLen = 32;
    maxLen = 0;
    i = 0;
    while i < alphaSize {
        let len_value = (*s).len[t as usize][i as usize] as i32; // Convert to i32
        if len_value > maxLen {
            maxLen = len_value;
        }
        if len_value < minLen {
            minLen = len_value;
        }
        i += 1;
    }
    if maxLen > 17 {
        BZ2_bz__AssertH__fail(3004);
    }
    if minLen < 1 {
        BZ2_bz__AssertH__fail(3005);
    }
    unsafe {
        BZ2_hbAssignCodes(
            (*s).code[t as usize].as_mut_ptr(),
            (*s).len[t as usize].as_mut_ptr(),
            minLen,
            maxLen,
            alphaSize,
        );
    }
    t += 1;
}

/*--- Transmit the mapping table. ---*/
let mut inUse16: [bool; 16] = [false; 16];
i = 0;
while i < 16 {
    inUse16[i as usize] = false;
    j = 0;
    while j < 16 {
        if (*s).inUse[(i * 16 + j) as usize] != 0 {
            inUse16[i as usize] = true;
        }
        j += 1;
    }
    i += 1;
}
nBytes = (*s).numZ;
i = 0;
while i < 16 {
    if inUse16[i as usize] {
        bsW(s, 1, 1);
    } else {
        bsW(s, 1, 0);
    }
    i += 1;
}
i = 0;
while i < 16 {
    if inUse16[i as usize] {
        j = 0;
        while j < 16 {
            if (*s).inUse[(i * 16 + j) as usize] != 0 {
                bsW(s, 1, 1);
            } else {
                bsW(s, 1, 0);
            }
            j += 1;
        }
    }
    i += 1;
}
if (*s).verbosity >= 3 {
    eprintln!("      bytes: mapping {}, ", (*s).numZ - nBytes);
}

/*--- Now the selectors. ---*/
nBytes = (*s).numZ;
bsW(s, 3, nGroups as u32);
bsW(s, 15, nSelectors as u32);
i = 0;
while i < nSelectors {
    j = 0;
    while j < (*s).selectorMtf[i as usize] as i32 { // Convert to i32
        bsW(s, 1, 1);
        j += 1;
    }
    bsW(s, 1, 0);
    i += 1;
}
if (*s).verbosity >= 3 {
    eprintln!("selectors {}, ", (*s).numZ - nBytes);
}

/*--- Now the coding tables. ---*/
nBytes = (*s).numZ;
t = 0;
while t < nGroups {
    let mut curr: i32 = (*s).len[t as usize][0] as i32; // Convert to i32
    bsW(s, 5, curr as u32);
    i = 0;
    while i < alphaSize {
        while curr < (*s).len[t as usize][i as usize] as i32 { // Convert to i32
            bsW(s, 2, 2);
            curr += 1;
        }
        while curr > (*s).len[t as usize][i as usize] as i32 { // Convert to i32
            bsW(s, 2, 3);
            curr -= 1;
        }
        bsW(s, 1, 0);
        i += 1;
    }
    t += 1;
}
if (*s).verbosity >= 3 {
    eprintln!("code lengths {}, ", (*s).numZ - nBytes);
}

/*--- And finally, the block data proper ---*/
nBytes = (*s).numZ;
selCtr = 0;
gs = 0;
while true {
     if gs >= unsafe { (*s).nMTF } { break; }
ge = gs + 50 - 1;
if ge >= unsafe { (*s).nMTF } { ge = unsafe { (*s).nMTF } - 1; }
if !(i32::from(unsafe { (*s).selector[selCtr as usize] }) < nGroups) {
    BZ2_bz__AssertH__fail(3006);
}
if nGroups == 6 && 50 == ge - gs + 1 {
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
            let len_sel_selCtr = s_len_sel_selCtr as *mut u8;
let code_sel_selCtr = s_code_sel_selCtr as *mut i32;

unsafe {
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));

    mtfv_i = *mtfv.offset((gs + 43) as isize);
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));

    mtfv_i = *mtfv.offset((gs + 44) as isize);
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));

    mtfv_i = *mtfv.offset((gs + 45) as isize);
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));

    mtfv_i = *mtfv.offset((gs + 46) as isize);
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));

    mtfv_i = *mtfv.offset((gs + 47) as isize);
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));

    mtfv_i = *mtfv.offset((gs + 48) as isize);
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));

    mtfv_i = *mtfv.offset((gs + 49) as isize);
    bsW(s, *(len_sel_selCtr.offset(mtfv_i as isize) as *mut i32), *(code_sel_selCtr.offset(mtfv_i as isize) as *mut u32));
}

        
} else {
    /*--- slow version which correctly handles all situations ---*/
     let mut i = gs;
while i <= ge {
    let selector_index = selCtr as usize;
    let mtfv_index = unsafe { *mtfv.offset(i as isize) }; // Accessing the value at the pointer
    
    bsW(
        s,
        (*s).len[(*s).selector[selector_index] as usize][mtfv_index as usize] as Int32,
        (*s).code[(*s).selector[selector_index] as usize][mtfv_index as usize] as UInt32,
    );
    i += 1;
}

        
}
gs = ge + 1;
selCtr += 1;


}
if selCtr != nSelectors {
    BZ2_bz__AssertH__fail(3007);
}
if (*s).verbosity >= 3 {
    eprintln!("codes {}", (*s).numZ - nBytes);
}

}
/*---------------------------------------------------*/

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
        BZ2_blockSort(&mut *s);
    }
    (*s).zbits =
        &mut *((*s).arr2 as *mut UChar).offset((*s).nblock as isize) as
            *mut UChar;
    /*-- If this is the first block, create the stream header. --*/
    if (*s).blockNo == 1 as std::os::raw::c_int {
        BZ2_bsInitWrite(&mut *s);
        let c8: UChar = 0x42 as UChar;
bsPutUChar(&mut *s, c8);
        let c2: UChar = 0x5a as UChar;
bsPutUChar(&mut *s, c2);
        let c3: UChar = 0x68 as UChar;
bsPutUChar(&mut *s, c3);
        let c16: UChar = (0x30 as std::os::raw::c_int + (*s).blockSize100k) as UChar;
bsPutUChar(&mut *s, c16);
    }
    if (*s).nblock > 0 as std::os::raw::c_int {
        let c13: UChar = 0x31 as UChar;
bsPutUChar(&mut *s, c13);
        let c1: UChar = 0x41 as UChar;
bsPutUChar(&mut *s, c1);
        let c6: UChar = 0x59 as UChar;
bsPutUChar(&mut *s, c6);
        let c7: UChar = 0x26 as UChar;
bsPutUChar(&mut *s, c7);
        let c4: UChar = 0x53 as UChar;
bsPutUChar(&mut *s, c4);
        let c10: UChar = 0x59 as UChar;
bsPutUChar(&mut *s, c10);
        /*-- Now the block's CRC, so it is in a known place. --*/
        bsPutUInt32(&mut *s, (*s).blockCRC);
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
        let c12: UChar = 0x17 as UChar;
bsPutUChar(&mut *s, c12);
        let c14: UChar = 0x72 as UChar;
bsPutUChar(&mut *s, c14);
        let c11: UChar = 0x45 as UChar;
bsPutUChar(&mut *s, c11);
        let c15: UChar = 0x38 as UChar;
bsPutUChar(&mut *s, c15);
        let c9: UChar = 0x50 as UChar;
bsPutUChar(&mut *s, c9);
        let c5: UChar = 0x90 as UChar;
bsPutUChar(&mut *s, c5);
        bsPutUInt32(&mut *s, (*s).combinedCRC);
        if (*s).verbosity >= 2 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"    final combined CRC = 0x%08x\n   \x00" as *const u8
                        as *const std::os::raw::c_char, (*s).combinedCRC);
        }
        bsFinishWrite(&mut *s);
    };
}
/*-------------------------------------------------------------*/
/*--- end                                        compress.c ---*/
/*-------------------------------------------------------------*/

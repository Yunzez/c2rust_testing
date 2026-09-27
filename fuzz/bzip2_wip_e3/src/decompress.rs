







































extern "C" {
    
    
    static mut __stderrp: *mut FILE;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
        
    
    
    
}
pub use crate::randtable::BZ2_rNums;
pub use crate::bzlib::BZ2_bz__AssertH__fail;
pub use crate::bzlib::BZ2_indexIntoF;
pub use crate::huffman::BZ2_hbCreateDecodeTables;
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
fn makeMaps_d(s: &mut *mut DState) {
    let s_ref = unsafe { &mut **s };
    let mut n_in_use = 0;
    for i in 0..256 {
        if s_ref.inUse[i] != 0 {
            s_ref.seqToUnseq[n_in_use] = i as UChar;
            n_in_use += 1;
        }
    }
    s_ref.nInUse = n_in_use as std::os::raw::c_int;
}

/*---------------------------------------------------*/
/*---------------------------------------------------*/
/* the longest code */
/*---------------------------------------------------*/

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
    match unsafe { (*s).state } {
    10 => {
        unsafe { (*s).state = 10; }
        loop {
            if !(true) {
                current_block = 5658374378798827547;
                break;
            }
            if unsafe { (*s).bsLive } >= 8 {
                let v: u32 = (unsafe { (*s).bsBuff } >> (unsafe { (*s).bsLive } - 8)) & ((1 << 8) - 1);
                unsafe { (*s).bsLive -= 8; }
                uc = v as u8;
                current_block = 5658374378798827547;
                break;
            } else if unsafe { (*(*s).strm).avail_in } == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                unsafe {
                    (*s).bsBuff = (unsafe { (*s).bsBuff } << 8) | (*(*(*s).strm).next_in as u32);
                    (*s).bsLive += 8;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                    (*(*s).strm).avail_in -= 1;
                    (*(*s).strm).total_in_lo32 += 1;
                    if (*(*s).strm).total_in_lo32 == 0 {
                        (*(*s).strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc as i32 != 0x42 {
                    retVal = -5;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 12259750428863723923;
                }
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
        if false {
            BZ2_bz__AssertH__fail(4001);
        }
        if false {
            BZ2_bz__AssertH__fail(4002);
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
        (*s).state = 13;
        loop {
            if !(true) {
                current_block = 1915186496383530739;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_2: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                (*s).blockSize100k = v_2 as i32;
                current_block = 1915186496383530739;
                break;
            } else if (*strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | unsafe { *(*strm).next_in } as u32;
                (*s).bsLive += 8;
                (*strm).next_in = (*strm).next_in.add(1);
                (*strm).avail_in -= 1;
                (*strm).total_in_lo32 += 1;
                if (*strm).total_in_lo32 == 0 {
                    (*strm).total_in_hi32 += 1;
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if (*s).blockSize100k < 0x30 + 1 || (*s).blockSize100k > 0x30 + 9 {
                    retVal = -5;
                    current_block = 15885526978618306830;
                } else {
                    (*s).blockSize100k -= 0x30;
                    if (*s).smallDecompress != 0 {
                        (*s).ll16 = (*strm).bzalloc.expect("non-null function pointer")(
                            (*strm).opaque,
                            ((*s).blockSize100k as usize * 100000 * std::mem::size_of::<u16>()) as i32,
                            1,
                        ) as *mut u16;
                        (*s).ll4 = (*strm).bzalloc.expect("non-null function pointer")(
                            (*strm).opaque,
                            ((1 + ((*s).blockSize100k as usize * 100000) >> 1) * std::mem::size_of::<u8>()) as i32,
                            1,
                        ) as *mut u8;
                        if (*s).ll16.is_null() || (*s).ll4.is_null() {
                            retVal = -3;
                            current_block = 15885526978618306830;
                        } else {
                            current_block = 7948568793456312728;
                        }
                    } else {
                        (*s).tt = (*strm).bzalloc.expect("non-null function pointer")(
                            (*strm).opaque,
                            ((*s).blockSize100k as usize * 100000 * std::mem::size_of::<i32>()) as i32,
                            1,
                        ) as *mut u32;
                        if (*s).tt.is_null() {
                            retVal = -3;
                            current_block = 15885526978618306830;
                        } else {
                            current_block = 7948568793456312728;
                        }
                    }
                }
            }
        }
    }
    _ => {}
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
        let state = unsafe { &mut *s };
        state.state = 42;
        loop {
            if !(true) {
                current_block = 13262463590990658200;
                break;
            }
            if state.bsLive >= 8 {
                let v_32: u32 = (state.bsBuff >> (state.bsLive - 8)) & ((1 << 8) - 1);
                state.bsLive -= 8;
                uc = v_32 as u8;
                current_block = 13262463590990658200;
                break;
            } else if unsafe { (*state.strm).avail_in } == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                let next_in = unsafe { (*state.strm).next_in };
                state.bsBuff = (state.bsBuff << 8) | (next_in as *const i8 as *const u8).read() as u32;
                state.bsLive += 8;
                unsafe {
                    (*state.strm).next_in = next_in.add(1);
                    (*state.strm).avail_in -= 1;
                    (*state.strm).total_in_lo32 += 1;
                    if (*state.strm).total_in_lo32 == 0 {
                        (*state.strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc != 0x72 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 9864403379770423142;
                }
            }
        }
    }
    16703841960874917807 => {
        let state = unsafe { &mut *s };
        state.state = 15;
        loop {
            if !(true) {
                current_block = 3569141194949357899;
                break;
            }
            if state.bsLive >= 8 {
                let v_4: u32 = (state.bsBuff >> (state.bsLive - 8)) & ((1 << 8) - 1);
                state.bsLive -= 8;
                uc = v_4 as u8;
                current_block = 3569141194949357899;
                break;
            } else if unsafe { (*state.strm).avail_in } == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                let next_in = unsafe { (*state.strm).next_in };
                state.bsBuff = (state.bsBuff << 8) | (next_in as *const i8 as *const u8).read() as u32;
                state.bsLive += 8;
                unsafe {
                    (*state.strm).next_in = next_in.add(1);
                    (*state.strm).avail_in -= 1;
                    (*state.strm).total_in_lo32 += 1;
                    if (*state.strm).total_in_lo32 == 0 {
                        (*state.strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc != 0x41 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 12231332282017165356;
                }
            }
        }
    }
    _ => {}
}

    match current_block {
    9864403379770423142 => {
        let state = unsafe { &mut *s };
        state.state = 43;
        loop {
            if !(true) {
                current_block = 10756506701594629759;
                break;
            }
            if state.bsLive >= 8 {
                let v_33: u32 = (state.bsBuff >> (state.bsLive - 8)) & ((1 << 8) - 1);
                state.bsLive -= 8;
                uc = v_33 as u8;
                current_block = 10756506701594629759;
                break;
            } else if unsafe { (*state.strm).avail_in } == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                state.bsBuff = (state.bsBuff << 8) | (unsafe { *(*state.strm).next_in } as u8) as u32;
                state.bsLive += 8;
                unsafe { (*state.strm).next_in = (*state.strm).next_in.add(1) };
                unsafe { (*state.strm).avail_in = (*state.strm).avail_in.wrapping_sub(1) };
                unsafe { (*state.strm).total_in_lo32 = (*state.strm).total_in_lo32.wrapping_add(1) };
                if unsafe { (*state.strm).total_in_lo32 } == 0 {
                    unsafe { (*state.strm).total_in_hi32 = (*state.strm).total_in_hi32.wrapping_add(1) };
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc != 0x45 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 8489059574810375089;
                }
            }
        }
    }
    12231332282017165356 => {
        let state = unsafe { &mut *s };
        state.state = 16;
        loop {
            if !(true) {
                current_block = 16517180880614114163;
                break;
            }
            if state.bsLive >= 8 {
                let v_5: u32 = (state.bsBuff >> (state.bsLive - 8)) & ((1 << 8) - 1);
                state.bsLive -= 8;
                uc = v_5 as u8;
                current_block = 16517180880614114163;
                break;
            } else if unsafe { (*state.strm).avail_in } == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                state.bsBuff = (state.bsBuff << 8) | (unsafe { *(*state.strm).next_in } as u8) as u32;
                state.bsLive += 8;
                unsafe { (*state.strm).next_in = (*state.strm).next_in.add(1) };
                unsafe { (*state.strm).avail_in = (*state.strm).avail_in.wrapping_sub(1) };
                unsafe { (*state.strm).total_in_lo32 = (*state.strm).total_in_lo32.wrapping_add(1) };
                if unsafe { (*state.strm).total_in_lo32 } == 0 {
                    unsafe { (*state.strm).total_in_hi32 = (*state.strm).total_in_hi32.wrapping_add(1) };
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc != 0x59 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 7286555771988341860;
                }
            }
        }
    }
    _ => {}
}

    match current_block {
    8489059574810375089 => {
        (*s).state = 44;
        loop {
            if !(true) {
                current_block = 9819403752380335018;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_34: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_34 as u8;
                current_block = 9819403752380335018;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc as i32 != 0x38 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 12998570369541158573;
                }
            }
        }
    }
    7286555771988341860 => {
        (*s).state = 17;
        loop {
            if !(true) {
                current_block = 2606663910910355487;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_6: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_6 as u8;
                current_block = 2606663910910355487;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc as i32 != 0x26 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 6656868271313165664;
                }
            }
        }
    }
    _ => {}
}

    match current_block {
    12998570369541158573 => {
        let state = unsafe { &mut *s };
        state.state = 45;
        loop {
            if !(true) {
                current_block = 9454797012561717444;
                break;
            }
            if state.bsLive >= 8 {
                let v_35: u32 = (state.bsBuff >> (state.bsLive - 8)) & ((1 << 8) - 1);
                state.bsLive -= 8;
                uc = v_35 as u8;
                current_block = 9454797012561717444;
                break;
            } else if unsafe { (*state.strm).avail_in } == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                let next_in = unsafe { (*state.strm).next_in };
                state.bsBuff = (state.bsBuff << 8) | (next_in as *const i8 as *const u8).read() as u32;
                state.bsLive += 8;
                unsafe {
                    (*state.strm).next_in = next_in.add(1);
                    (*state.strm).avail_in -= 1;
                    (*state.strm).total_in_lo32 += 1;
                    if (*state.strm).total_in_lo32 == 0 {
                        (*state.strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc as i32 != 0x50 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 10541196509243133637;
                }
            }
        }
    }
    6656868271313165664 => {
        let state = unsafe { &mut *s };
        state.state = 18;
        loop {
            if !(true) {
                current_block = 8125779086361653720;
                break;
            }
            if state.bsLive >= 8 {
                let v_7: u32 = (state.bsBuff >> (state.bsLive - 8)) & ((1 << 8) - 1);
                state.bsLive -= 8;
                uc = v_7 as u8;
                current_block = 8125779086361653720;
                break;
            } else if unsafe { (*state.strm).avail_in } == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                let next_in = unsafe { (*state.strm).next_in };
                state.bsBuff = (state.bsBuff << 8) | (next_in as *const i8 as *const u8).read() as u32;
                state.bsLive += 8;
                unsafe {
                    (*state.strm).next_in = next_in.add(1);
                    (*state.strm).avail_in -= 1;
                    (*state.strm).total_in_lo32 += 1;
                    if (*state.strm).total_in_lo32 == 0 {
                        (*state.strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                if uc as i32 != 0x53 {
                    retVal = -4;
                    current_block = 15885526978618306830;
                } else {
                    current_block = 15902903523132075486;
                }
            }
        }
    }
    _ => {}
}

    match current_block {
    10541196509243133637 => {
        unsafe { (*s).state = 46; }
        loop {
            if !(true) {
                current_block = 724777313732190959;
                break;
            }
            unsafe {
                if (*s).bsLive >= 8 {
                    let v_36: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                    (*s).bsLive -= 8;
                    uc = v_36 as u8;
                    current_block = 724777313732190959;
                    break;
                } else if (*(*s).strm).avail_in == 0 {
                    retVal = 0;
                    current_block = 15885526978618306830;
                    break;
                } else {
                    let next_in = &(*(*s).strm).next_in;
                    (*s).bsBuff = ((*s).bsBuff << 8) | (*next_in.offset(0)) as u32;
                    (*s).bsLive += 8;
                    (*(*s).strm).next_in = next_in.offset(1);
                    (*(*s).strm).avail_in -= 1;
                    (*(*s).strm).total_in_lo32 += 1;
                    if (*(*s).strm).total_in_lo32 == 0 {
                        (*(*s).strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                unsafe {
                    if uc != 0x90 {
                        retVal = -4;
                        current_block = 15885526978618306830;
                    } else {
                        (*s).storedCombinedCRC = 0;
                        current_block = 8760950161942609538;
                    }
                }
            }
        }
    }
    15902903523132075486 => {
        unsafe { (*s).state = 19; }
        loop {
            if !(true) {
                current_block = 958128786106592581;
                break;
            }
            unsafe {
                if (*s).bsLive >= 8 {
                    let v_8: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                    (*s).bsLive -= 8;
                    uc = v_8 as u8;
                    current_block = 958128786106592581;
                    break;
                } else if (*(*s).strm).avail_in == 0 {
                    retVal = 0;
                    current_block = 15885526978618306830;
                    break;
                } else {
                    let next_in = &(*(*s).strm).next_in;
                    (*s).bsBuff = ((*s).bsBuff << 8) | (*next_in.offset(0)) as u32;
                    (*s).bsLive += 8;
                    (*(*s).strm).next_in = next_in.offset(1);
                    (*(*s).strm).avail_in -= 1;
                    (*(*s).strm).total_in_lo32 += 1;
                    if (*(*s).strm).total_in_lo32 == 0 {
                        (*(*s).strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                unsafe {
                    if uc != 0x59 {
                        retVal = -4;
                        current_block = 15885526978618306830;
                    } else {
                        (*s).currBlockNo += 1;
                        if (*s).verbosity >= 2 {
                            eprintln!("\n    [{}: huff+mtf ", (*s).currBlockNo);
                        }
                        (*s).storedBlockCRC = 0;
                        current_block = 16204949703499709801;
                    }
                }
            }
        }
    }
    _ => {}
}

    match current_block {
    8760950161942609538 => {
        (*s).state = 47;
        loop {
            if !(true) {
                current_block = 14486187473704332379;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_37: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_37 as u8;
                current_block = 14486187473704332379;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in -= 1;
                (*(*s).strm).total_in_lo32 += 1;
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 += 1;
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedCombinedCRC = ((*s).storedCombinedCRC << 8) | (uc as u32);
                current_block = 3131443096645543054;
            }
        }
    }
    16204949703499709801 => {
        (*s).state = 20;
        loop {
            if !(true) {
                current_block = 3790734079518302164;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_9: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_9 as u8;
                current_block = 3790734079518302164;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in -= 1;
                (*(*s).strm).total_in_lo32 += 1;
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 += 1;
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedBlockCRC = ((*s).storedBlockCRC << 8) | (uc as u32);
                current_block = 5505795673017046993;
            }
        }
    }
    _ => {}
}

    match current_block {
    3131443096645543054 => {
        (*s).state = 48;
        loop {
            if !(true) {
                current_block = 3659807904093622879;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_38: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_38 as u8;
                current_block = 3659807904093622879;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedCombinedCRC = ((*s).storedCombinedCRC << 8) | (uc as u32);
                current_block = 1975408140333322065;
            }
        }
    }
    5505795673017046993 => {
        (*s).state = 21;
        loop {
            if !(true) {
                current_block = 16711521214030637000;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_10: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_10 as u8;
                current_block = 16711521214030637000;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedBlockCRC = ((*s).storedBlockCRC << 8) | (uc as u32);
                current_block = 14563596112884461881;
            }
        }
    }
    _ => {}
}

    match current_block {
    1975408140333322065 => {
        (*s).state = 49;
        loop {
            if !(true) {
                current_block = 2394045633138979148;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_39: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_39 as u8;
                current_block = 2394045633138979148;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u8 as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedCombinedCRC = ((*s).storedCombinedCRC << 8) | uc as u32;
                current_block = 15818179691129344165;
            }
        }
    }
    14563596112884461881 => {
        (*s).state = 22;
        loop {
            if !(true) {
                current_block = 17870985093275900527;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_11: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_11 as u8;
                current_block = 17870985093275900527;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u8 as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedBlockCRC = ((*s).storedBlockCRC << 8) | uc as u32;
                current_block = 12051594319698232578;
            }
        }
    }
    _ => {}
}

    match current_block {
    12051594319698232578 => {
        (*s).state = 23;
        loop {
            if !(true) {
                current_block = 13734492969709581318;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_12: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & 0xFF;
                (*s).bsLive -= 8;
                uc = v_12 as u8;
                current_block = 13734492969709581318;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                let next_in = unsafe { &*(*(*s).strm).next_in };
                (*s).bsBuff = ((*s).bsBuff << 8) | (*next_in as u32);
                (*s).bsLive += 8;
                (*(*s).strm).next_in = unsafe { (*(*s).strm).next_in.add(1) };
                (*(*s).strm).avail_in -= 1;
                (*(*s).strm).total_in_lo32 += 1;
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 += 1;
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedBlockCRC = ((*s).storedBlockCRC << 8) | uc as u32;
                current_block = 14315698657705028467;
            }
        }
    }
    15818179691129344165 => {
        (*s).state = 50;
        loop {
            if !(true) {
                current_block = 1904329045571868869;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_40: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & 0xFF;
                (*s).bsLive -= 8;
                uc = v_40 as u8;
                current_block = 1904329045571868869;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                let next_in = unsafe { &*(*(*s).strm).next_in };
                (*s).bsBuff = ((*s).bsBuff << 8) | (*next_in as u32);
                (*s).bsLive += 8;
                (*(*s).strm).next_in = unsafe { (*(*s).strm).next_in.add(1) };
                (*(*s).strm).avail_in -= 1;
                (*(*s).strm).total_in_lo32 += 1;
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 += 1;
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).storedCombinedCRC = ((*s).storedCombinedCRC << 8) | uc as u32;
                (*s).state = 1;
                retVal = 4;
                current_block = 15885526978618306830;
            }
        }
    }
    _ => {}
}

    match current_block {
    14315698657705028467 => {
        (*s).state = 24;
        loop {
            if !(true) {
                current_block = 15030729790988239748;
                break;
            }
            if (*s).bsLive >= 1 {
                let v_13: u32 = ((*s).bsBuff >> ((*s).bsLive - 1)) & ((1 << 1) - 1);
                (*s).bsLive -= 1;
                (*s).blockRandomised = v_13 as u8; // Assuming blockRandomised is of type u8
                current_block = 15030729790988239748;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).origPtr = 0;
                current_block = 640681092829779800;
            }
        }
    }
    _ => {}
}

match current_block {
    640681092829779800 => {
        (*s).state = 25;
        loop {
            if !(true) {
                current_block = 8260322496947496197;
                break;
            }
            if (*s).bsLive >= 8 {
                let v_14: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                (*s).bsLive -= 8;
                uc = v_14 as u8;
                current_block = 8260322496947496197;
                break;
            } else if (*(*s).strm).avail_in == 0 {
                retVal = 0;
                current_block = 15885526978618306830;
                break;
            } else {
                (*s).bsBuff = ((*s).bsBuff << 8) | (*(*s).strm).next_in as u32;
                (*s).bsLive += 8;
                (*(*s).strm).next_in = (*(*s).strm).next_in.add(1);
                (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
                (*(*s).strm).total_in_lo32 = (*(*s).strm).total_in_lo32.wrapping_add(1);
                if (*(*s).strm).total_in_lo32 == 0 {
                    (*(*s).strm).total_in_hi32 = (*(*s).strm).total_in_hi32.wrapping_add(1);
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                (*s).origPtr = ((*s).origPtr << 8) | uc as i32;
                current_block = 588075840077989673;
            }
        }
    }
    _ => {}
}

    match current_block {
    588075840077989673 => {
        unsafe { (*s).state = 26; }
        loop {
            if !(true) {
                current_block = 5561851013817067674;
                break;
            }
            unsafe {
                if (*s).bsLive >= 8 {
                    let v_15: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                    (*s).bsLive -= 8;
                    uc = v_15 as u8;
                    current_block = 5561851013817067674;
                    break;
                } else if (*(*s).strm).avail_in == 0 {
                    retVal = 0;
                    current_block = 15885526978618306830;
                    break;
                } else {
                    let next_in = (*(*s).strm).next_in;
                    (*s).bsBuff = ((*s).bsBuff << 8) | unsafe { *next_in as u32 };
                    (*s).bsLive += 8;
                    (*(*s).strm).next_in = next_in.add(1);
                    (*(*s).strm).avail_in -= 1;
                    (*(*s).strm).total_in_lo32 += 1;
                    if (*(*s).strm).total_in_lo32 == 0 {
                        (*(*s).strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                unsafe {
                    (*s).origPtr = ((*s).origPtr << 8) | (uc as i32);
                }
                current_block = 34749046854646975;
            }
        }
    }
    _ => {}
}
match current_block {
    34749046854646975 => {
        unsafe { (*s).state = 27; }
        loop {
            if !(true) {
                current_block = 10471999855724930313;
                break;
            }
            unsafe {
                if (*s).bsLive >= 8 {
                    let v_16: u32 = ((*s).bsBuff >> ((*s).bsLive - 8)) & ((1 << 8) - 1);
                    (*s).bsLive -= 8;
                    uc = v_16 as u8;
                    current_block = 10471999855724930313;
                    break;
                } else if (*(*s).strm).avail_in == 0 {
                    retVal = 0;
                    current_block = 15885526978618306830;
                    break;
                } else {
                    let next_in = (*(*s).strm).next_in;
                    (*s).bsBuff = ((*s).bsBuff << 8) | unsafe { *next_in as u32 };
                    (*s).bsLive += 8;
                    (*(*s).strm).next_in = next_in.add(1);
                    (*(*s).strm).avail_in -= 1;
                    (*(*s).strm).total_in_lo32 += 1;
                    if (*(*s).strm).total_in_lo32 == 0 {
                        (*(*s).strm).total_in_hi32 += 1;
                    }
                }
            }
        }
        match current_block {
            15885526978618306830 => {}
            _ => {
                unsafe {
                    (*s).origPtr = ((*s).origPtr << 8) | (uc as i32);
                    if (*s).origPtr < 0 {
                        retVal = -4;
                        current_block = 15885526978618306830;
                    } else if (*s).origPtr > 10 + 100000 * (*s).blockSize100k {
                        retVal = -4;
                        current_block = 15885526978618306830;
                    } else {
                        /*--- Receive the mapping table ---*/
                        i = 0;
                        current_block = 17262312153619709241;
                    }
                }
            }
        }
    }
    _ => {}
}
'c_10532:
loop {
     match current_block {
    15885526978618306830 => { (*s).save_i = i; break; }
    2629672494974161066 => {
         {
    let s = unsafe { &mut *s }; // Dereference the raw pointer to get a mutable reference
    let strm = unsafe { &mut *strm }; // Dereference the raw pointer to get a mutable reference

    s.state = 40;
    loop {
        if s.bsLive >= zn {
            let v_30: u32 = (s.bsBuff >> (s.bsLive - zn)) & ((1 << zn) - 1);
            s.bsLive -= zn;
            zvec = v_30 as i32;
            break;
        } else if strm.avail_in == 0 {
            retVal = 0;
            current_block = 15885526978618306830;
            continue 'c_10532;
        } else {
            s.bsBuff = (s.bsBuff << 8) | (strm.next_in as u32);
            s.bsLive += 8;
            strm.next_in = strm.next_in.add(1);
            strm.avail_in -= 1;
            strm.total_in_lo32 += 1;
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 += 1;
            }
        }
    }
    current_block = 9078889872071895942;
}


    }
    13999925517074022731 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 39;

loop {
    let strm = &mut *s.strm; // Dereference the raw pointer to access the stream

    if s.bsLive >= 1 {
        let v_29: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
        s.bsLive -= 1;
        zj = v_29 as i32;
        break;
    } else if strm.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (*strm.next_in as u32);
        s.bsLive += 8;
        strm.next_in = strm.next_in.add(1);
        strm.avail_in -= 1;
        strm.total_in_lo32 += 1;
        if strm.total_in_lo32 == 0 {
            strm.total_in_hi32 += 1;
        }
    }
}

zvec = (zvec << 1) | zj;
current_block = 13605767259572914371;


    }
    5374617794059532979 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 38;

loop {
    let strm = &mut *s.strm; // Dereference the raw pointer to access the stream

    if s.bsLive >= zn {
        let v_28: u32 = (s.bsBuff >> (s.bsLive - zn)) & ((1 << zn) - 1);
        s.bsLive -= zn;
        zvec = v_28 as i32;
        break;
    } else if strm.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (strm.next_in as u32);
        s.bsLive += 8;
        strm.next_in = strm.next_in.add(1);
        strm.avail_in -= 1;
        strm.total_in_lo32 += 1;

        if strm.total_in_lo32 == 0 {
            strm.total_in_hi32 += 1;
        }
    }
}
current_block = 13605767259572914371;


    }
    14744029255125744966 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 37;

loop {
    if s.bsLive >= 1 {
        let v_27: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
        s.bsLive -= 1;
        zj = v_27 as i32;
        break;
    } else if unsafe { (*s.strm).avail_in } == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (unsafe { *(*s.strm).next_in } as u32);
        s.bsLive += 8;
        unsafe { (*s.strm).next_in = (*s.strm).next_in.add(1) };
        unsafe { (*s.strm).avail_in -= 1 };
        unsafe { (*s.strm).total_in_lo32 += 1 };

        if unsafe { (*s.strm).total_in_lo32 } == 0 {
            unsafe { (*s.strm).total_in_hi32 += 1 };
        }
    }
}

zvec = (zvec << 1) | zj;
current_block = 1550405138573481750;


    }
    16722720626876144162 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 36;

loop {
    let strm = &mut *s.strm; // Dereference the raw pointer to access the stream

    if s.bsLive >= zn {
        let v_26: u32 = (s.bsBuff >> (s.bsLive - zn)) & ((1 << zn) - 1) as u32;
        s.bsLive -= zn;
        zvec = v_26 as i32;
        break;
    } else if strm.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue; // Assuming 'continue' is meant to continue the outer loop
    } else {
        s.bsBuff = (s.bsBuff << 8) | (strm.next_in as u32);
        s.bsLive += 8;
        strm.next_in = strm.next_in.add(1);
        strm.avail_in -= 1;
        strm.total_in_lo32 += 1;

        if strm.total_in_lo32 == 0 {
            strm.total_in_hi32 += 1;
        }
    }
}
current_block = 1550405138573481750;


    }
    5008197131544113214 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 35;

loop {
    let strm = &mut *s.strm; // Dereference the raw pointer to access the stream

    if s.bsLive >= 1 {
        let v_25: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
        s.bsLive -= 1;
        uc = v_25 as u8;
        break;
    } else if strm.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (*strm.next_in as u32);
        s.bsLive += 8;
        strm.next_in = strm.next_in.add(1);
        strm.avail_in -= 1;
        strm.total_in_lo32 += 1;
        if strm.total_in_lo32 == 0 {
            strm.total_in_hi32 += 1;
        }
    }
}

if uc == 0 {
    curr += 1;
} else {
    curr -= 1;
}
current_block = 11858046780433112516;


    }
    1736021991379636935 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 34;

loop {
    let strm = &mut *s.strm; // Dereference the raw pointer to access the stream

    if s.bsLive >= 1 {
        let v_24: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
        s.bsLive -= 1;
        uc = v_24 as u8;
        break;
    } else if strm.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (*strm.next_in as u32);
        s.bsLive += 8;
        strm.next_in = strm.next_in.add(1);
        strm.avail_in -= 1;
        strm.total_in_lo32 += 1;

        if strm.total_in_lo32 == 0 {
            strm.total_in_hi32 += 1;
        }
    }
}

if uc != 0 {
    current_block = 5008197131544113214;
    continue;
}
current_block = 17503523010989424999;


    }
    10945178116989557996 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to DState
s.state = 33;

loop {
    let strm = &mut *s.strm; // Dereference the raw pointer to access the fields

    if s.bsLive >= 5 {
        let v_23: u32 = (s.bsBuff >> (s.bsLive - 5)) & ((1 << 5) - 1);
        s.bsLive -= 5;
        curr = v_23 as i32;
        break;
    } else if strm.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (strm.next_in as u32);
        s.bsLive += 8;
        strm.next_in = strm.next_in.add(1);
        strm.avail_in -= 1;
        strm.total_in_lo32 += 1;
        if strm.total_in_lo32 == 0 {
            strm.total_in_hi32 += 1;
        }
    }
}

i = 0;
current_block = 3770765986603902964;


    }
    4874723077730206021 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 32;

loop {
    if s.bsLive >= 1 {
        let v_21: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
        s.bsLive -= 1;
        uc = v_21 as u8;
        break;
    } else if (*s.strm).avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (*(*s.strm).next_in as u32);
        s.bsLive += 8;
        (*s.strm).next_in = (*s.strm).next_in.add(1);
        (*s.strm).avail_in -= 1;
        (*s.strm).total_in_lo32 += 1;
        if (*s.strm).total_in_lo32 == 0 {
            (*s.strm).total_in_hi32 += 1;
        }
    }
}

if uc == 0 {
    current_block = 5281038271658253520;
} else {
    j += 1;
    if j >= nGroups {
        retVal = -4;
        current_block = 15885526978618306830;
        continue;
    } else {
        current_block = 6927328446518169316;
    }
}


    }
    5769007513321684282 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 31;

loop {
    let strm = &mut *s.strm; // Dereference the raw pointer to access the stream

    if s.bsLive >= 15 {
        let v_20: u32 = (s.bsBuff >> (s.bsLive - 15)) & ((1 << 15) - 1);
        s.bsLive -= 15;
        nSelectors = v_20 as i32;
        break;
    } else if strm.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (*strm.next_in as u32);
        s.bsLive += 8;
        strm.next_in = strm.next_in.add(1);
        strm.avail_in -= 1;
        strm.total_in_lo32 += 1;
        if strm.total_in_lo32 == 0 {
            strm.total_in_hi32 += 1;
        }
    }
}

if nSelectors < 1 {
    retVal = -4;
    current_block = 15885526978618306830;
    continue;
} else {
    i = 0;
}
current_block = 6591141407893725683;


    }
    3906616468301123675 => {
         while true {
    let s_ref = unsafe { &mut *s }; // Dereference the raw pointer to get a mutable reference
    let strm_ref = unsafe { &mut *s_ref.strm }; // Dereference the raw pointer to get a mutable reference to strm

    if s_ref.bsLive >= 3 {
        let v_19: u32 = (s_ref.bsBuff >> (s_ref.bsLive - 3)) & ((1 << 3) - 1);
        s_ref.bsLive -= 3;
        nGroups = v_19 as i32;
        break;
    } else if strm_ref.avail_in == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s_ref.bsBuff = (s_ref.bsBuff << 8) | (strm_ref.next_in as u32);
        s_ref.bsLive += 8;
        strm_ref.next_in = strm_ref.next_in.add(1);
        strm_ref.avail_in -= 1;
        strm_ref.total_in_lo32 += 1;
        if strm_ref.total_in_lo32 == 0 {
            strm_ref.total_in_hi32 += 1;
        }
    }
}

if nGroups >= 2 && nGroups <= 6 {
    current_block = 5769007513321684282;
    continue;
}

retVal = -4;
current_block = 15885526978618306830;


    }
    1422779171932145779 => {
         {
    let s = unsafe { &mut *s }; // Dereference the raw pointer to get a mutable reference
    s.state = 29;
    loop {
        if s.bsLive >= 1 {
            let v_18: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
            s.bsLive -= 1;
            uc = v_18 as u8;
            break;
        } else if unsafe { (*s.strm).avail_in } == 0 {
            retVal = 0;
            current_block = 15885526978618306830;
            continue 'c_10532;
        } else {
            s.bsBuff = (s.bsBuff << 8) | (unsafe { *(*s.strm).next_in } as u32);
            s.bsLive += 8;
            (*s.strm).next_in = (*s.strm).next_in.add(1);
            (*s.strm).avail_in -= 1;
            (*s.strm).total_in_lo32 += 1;
            if (*s.strm).total_in_lo32 == 0 {
                (*s.strm).total_in_hi32 += 1;
            }
        }
    }
    if uc as i32 == 1 {
        s.inUse[(i * 16 + j) as usize] = 1; // Assuming inUse is of type u8
    }
    j += 1;
    current_block = 3854024847017804838;
}


    }
    17262312153619709241 => {
         if i < 16 {
    current_block = 16487873541482693172;
    continue;
}
i = 0;
while i < 256 {
    (*s).inUse[i as usize] = 0; // Assuming inUse is a field of DState that is of type u8
    i += 1;
}
i = 0;
current_block = 3472349144349095221;


    }
    16487873541482693172 => {
         let mut s = &mut *s; // Assuming s is a mutable reference to a DState
s.state = 28;

loop {
    if s.bsLive >= 1 {
        let v_17: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
        s.bsLive -= 1;
        uc = v_17 as u8;
        break;
    } else if unsafe { (*s.strm).avail_in } == 0 {
        retVal = 0;
        current_block = 15885526978618306830;
        continue 'c_10532;
    } else {
        s.bsBuff = (s.bsBuff << 8) | (unsafe { *(*s.strm).next_in } as u32);
        s.bsLive += 8;
        unsafe { (*s.strm).next_in = (*s.strm).next_in.add(1) };
        unsafe { (*s.strm).avail_in -= 1 };
        unsafe { (*s.strm).total_in_lo32 += 1 };
        if unsafe { (*s.strm).total_in_lo32 } == 0 {
            unsafe { (*s.strm).total_in_hi32 += 1 };
        }
    }
}

s.inUse16[i as usize] = if uc == 1 { 1 } else { 0 };
i += 1;
current_block = 17262312153619709241;
continue;


    }
    _ => {
         {
    let s = unsafe { &mut *s }; // Dereference the raw pointer to get a mutable reference
    let strm = unsafe { &mut *s.strm }; // Dereference the raw pointer to get a mutable reference to strm
    s.state = 41;
    loop {
        if s.bsLive >= 1 {
            let v_31: u32 = (s.bsBuff >> (s.bsLive - 1)) & ((1 << 1) - 1);
            s.bsLive -= 1;
            zj = v_31 as i32;
            break;
        } else if strm.avail_in == 0 {
            retVal = 0;
            current_block = 15885526978618306830;
            continue 'c_10532;
        } else {
            s.bsBuff = (s.bsBuff << 8) | (*strm.next_in as u32);
            s.bsLive += 8;
            strm.next_in = strm.next_in.add(1);
            strm.avail_in -= 1;
            strm.total_in_lo32 += 1;
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 += 1;
            }
        }
    }
    zvec = (zvec << 1) | zj;
    current_block = 9078889872071895942;
}


    }
}
/*
The variables live at this point are:
(s: &mut bzlib::DState, current_block: u64, uc: u8, retVal: i32, strm: &mut blocksort::bz_stream, i: i32, j: i32, nGroups: i32, nSelectors: i32, curr: i32, zn: i32, zvec: i32, zj: i32)
*/

            match current_block {
    9078889872071895942 => {
        if zn > 20 {
            retVal = -4;
            current_block = 15885526978618306830;
            continue;
        } else if zvec <= unsafe { *gLimit.offset(zn as isize) } {
            let offset_value = zvec - unsafe { *gBase.offset(zn as isize) };
            if offset_value < 0 || offset_value >= 258 {
                retVal = -4;
                current_block = 15885526978618306830;
                continue;
            } else {
                nextSym = unsafe { *gPerm.offset(offset_value as isize) };
            }
        } else {
            zn += 1;
            current_block = 1050378859040334210;
            continue;
        }
        current_block = 15093386068129942558;
    }
    13605767259572914371 => {
        if zn > 20 {
            retVal = -4;
            current_block = 15885526978618306830;
            continue;
        } else if zvec <= unsafe { *gLimit.offset(zn as isize) } {
            let offset_value = zvec - unsafe { *gBase.offset(zn as isize) };
            if offset_value < 0 || offset_value >= 258 {
                retVal = -4;
                current_block = 15885526978618306830;
                continue;
            } else {
                nextSym = unsafe { *gPerm.offset(offset_value as isize) };
                if nextSym == 0 || nextSym == 1 {
                    current_block = 4550729491376650574;
                } else {
                    es += 1;
                    let s_ref = unsafe { &mut *s };
                    uc = s_ref.seqToUnseq[s_ref.mtfa[s_ref.mtfbase[0] as usize] as usize];
                    s_ref.unzftab[uc as usize] += es;
                    if s_ref.smallDecompress != 0 {
                        while es > 0 {
                            if nblock >= nblockMAX {
                                retVal = -4;
                                current_block = 15885526978618306830;
                                continue 'c_10532;
                            } else {
                                unsafe { *s_ref.ll16.offset(nblock as isize) = uc as u16; }
                                nblock += 1;
                                es -= 1;
                            }
                        }
                    } else {
                        while es > 0 {
                            if nblock >= nblockMAX {
                                retVal = -4;
                                current_block = 15885526978618306830;
                                continue 'c_10532;
                            } else {
                                unsafe { *s_ref.tt.offset(nblock as isize) = uc as u32; }
                                nblock += 1;
                                es -= 1;
                            }
                        }
                    }
                    current_block = 15093386068129942558;
                }
            }
        } else {
            zn += 1;
            current_block = 13999925517074022731;
            continue;
        }
    }
    1550405138573481750 => {
        if zn > 20 {
            retVal = -4;
            current_block = 15885526978618306830;
            continue;
        } else if zvec <= unsafe { *gLimit.offset(zn as isize) } {
            let offset_value = zvec - unsafe { *gBase.offset(zn as isize) };
            if offset_value < 0 || offset_value >= 258 {
                retVal = -4;
                current_block = 15885526978618306830;
                continue;
            } else {
                nextSym = unsafe { *gPerm.offset(offset_value as isize) };
            }
        } else {
            zn += 1;
            current_block = 14744029255125744966;
            continue;
        }
        current_block = 15093386068129942558;
    }
    _ => {}
}
match current_block {
    15093386068129942558 => {
         if true {
    if nextSym == EOB {
        current_block = 12118509005321596519;
    } else {
         if nextSym == 0 || nextSym == 1 {
    es = -1;
    N = 1;
} else if nblock >= nblockMAX {
    retVal = -4;
    current_block = 15885526978618306830;
    continue;
} else {
    /*-- uc = MTF ( nextSym - 1 ) --*/
     let mut ii_0: i32 = 0;
let mut jj_0: i32 = 0;
let mut kk_0: i32 = 0;
let mut pp: i32 = 0;
let mut lno: i32 = 0;
let mut off: i32 = 0;
let mut nn: u32 = (nextSym - 1) as u32;

let s = unsafe { &mut *s }; // Dereference the raw pointer

if nn < 16 {
    pp = s.mtfbase[0];
    uc = s.mtfa[(pp as u32 + nn) as usize];

    while nn > 3 {
        let z: i32 = (pp as u32 + nn) as i32;
        s.mtfa[z as usize] = s.mtfa[(z - 1) as usize];
        s.mtfa[(z - 1) as usize] = s.mtfa[(z - 2) as usize];
        s.mtfa[(z - 2) as usize] = s.mtfa[(z - 3) as usize];
        s.mtfa[(z - 3) as usize] = s.mtfa[(z - 4) as usize];
        nn -= 4;
    }

    while nn > 0 {
        s.mtfa[(pp as u32 + nn) as usize] = s.mtfa[(pp as u32 + nn - 1) as usize];
        nn -= 1;
    }
    s.mtfa[pp as usize] = uc;
} else {
    lno = (nn / 16) as i32;
    off = (nn % 16) as i32;
    pp = s.mtfbase[lno as usize] + off;
    uc = s.mtfa[pp as usize];

    while pp > s.mtfbase[lno as usize] {
        s.mtfa[pp as usize] = s.mtfa[(pp - 1) as usize];
        pp -= 1;
    }
    s.mtfbase[lno as usize] += 1;

    while lno > 0 {
        s.mtfbase[lno as usize] -= 1;
        s.mtfa[s.mtfbase[lno as usize] as usize] = s.mtfa[(s.mtfbase[(lno - 1) as usize] + 16 - 1) as usize];
        lno -= 1;
    }
    s.mtfbase[0] -= 1;
    s.mtfa[s.mtfbase[0] as usize] = uc;

    if s.mtfbase[0] == 0 {
        kk_0 = 4096 - 1;
        ii_0 = 256 / 16 - 1;

        while ii_0 >= 0 {
            jj_0 = 16 - 1;
            while jj_0 >= 0 {
                s.mtfa[kk_0 as usize] = s.mtfa[(s.mtfbase[ii_0 as usize] + jj_0) as usize];
                kk_0 -= 1;
                jj_0 -= 1;
            }
            s.mtfbase[ii_0 as usize] = kk_0 + 1;
            ii_0 -= 1;
        }
    }
}

s.unzftab[s.seqToUnseq[uc as usize] as usize] += 1;

if s.smallDecompress != 0 {
    let ll16 = unsafe { &mut *(s.ll16 as *mut [u16; 256]) }; // Assuming 256 is the size of ll16
    ll16[nblock as usize] = s.seqToUnseq[uc as usize] as u16;
} else {
    let tt = unsafe { &mut *(s.tt as *mut [u32; 256]) }; // Assuming 256 is the size of tt
    tt[nblock as usize] = s.seqToUnseq[uc as usize] as u32;
}
nblock += 1;

                                if groupPos == 0 {
    groupNo += 1;
    if groupNo >= nSelectors {
        retVal = -4;
        current_block = 15885526978618306830;
        continue;
    } else {
        groupPos = 50;
        gSel = s.selector[groupNo as usize] as i32;
        gMinlen = s.minLens[gSel as usize];
        gLimit = &mut s.limit[gSel as usize][0];
        gPerm = &mut s.perm[gSel as usize][0];
        gBase = &mut s.base[gSel as usize][0];
    }
}
groupPos -= 1;
zn = gMinlen;
current_block = 2629672494974161066;
continue;

                            
}
current_block = 4550729491376650574;
/*
The variables live at this point are:
(mut s: &mut bzlib::DState, mut current_block: u64, mut uc: u8, mut retVal: i32, mut nSelectors: i32, mut groupNo: i32, mut groupPos: i32, mut nextSym: i32, mut nblockMAX: i32, mut nblock: i32, mut es: i32, mut N: i32, mut zn: i32, mut gSel: i32, mut gMinlen: i32, mut gLimit: &mut i32, mut gBase: &mut i32, mut gPerm: &mut i32)
*/


    }
} else {
    current_block = 12118509005321596519;
}

match current_block {
    4550729491376650574 => {}
    _ => {
        /* Now we know what nblock is, we can do a better sanity
        check on s->origPtr.
        */
        if unsafe { (*s).origPtr } < 0 || unsafe { (*s).origPtr } >= nblock {
            retVal = -4;
            current_block = 15885526978618306830;
            continue;
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
                                    fn process_data(s: &mut DState, nblock: i32) {
    let mut uc: u8;
    let mut i: i32;
    let mut j: i32;

    // Copy cftab to cftabCopy
    for i in 0..=256 {
        s.cftabCopy[i as usize] = s.cftab[i as usize];
    }

    // Compute the T vector
    for i in 0..nblock {
        uc = unsafe { *s.ll16.offset(i as isize) } as u8;
        unsafe {
            *s.ll16.offset(i as isize) = (s.cftabCopy[uc as usize] & 0xffff) as u16;
        }

        if i % 2 == 0 {
            unsafe {
                *s.ll4.offset((i >> 1) as isize) = (*s.ll4.offset((i >> 1) as isize) & 0xf0 | ((s.cftabCopy[uc as usize] >> 16) as u8)) as u8;
            }
        } else {
            unsafe {
                *s.ll4.offset((i >> 1) as isize) = (*s.ll4.offset((i >> 1) as isize) & 0xf | (((s.cftabCopy[uc as usize] >> 16) as u8) << 4)) as u8;
            }
        }
        s.cftabCopy[uc as usize] += 1;
    }

    // Compute T^(-1) by pointer reversal on T
    i = s.origPtr;
    j = (unsafe { *s.ll16.offset(i as isize) } as u32 | (unsafe { *s.ll4.offset((i >> 1) as isize) } as u32 >> (i << 2 & 0x4) & 0xf) << 16) as i32;

    loop {
        let tmp_0 = (unsafe { *s.ll16.offset(j as isize) } as u32 | (unsafe { *s.ll4.offset((j >> 1) as isize) } as u32 >> (j << 2 & 0x4) & 0xf) << 16) as i32;
        unsafe {
            *s.ll16.offset(j as isize) = (i & 0xffff) as u16;
        }

        if j % 2 == 0 {
            unsafe {
                *s.ll4.offset((j >> 1) as isize) = (*s.ll4.offset((j >> 1) as isize) & 0xf0 | ((i >> 16) as u8)) as u8;
            }
        } else {
            unsafe {
                *s.ll4.offset((j >> 1) as isize) = (*s.ll4.offset((j >> 1) as isize) & 0xf | (((i >> 16) as u8) << 4)) as u8;
            }
        }
        i = j;
        j = tmp_0;

        if i == s.origPtr {
            break;
        }
    }

    s.tPos = s.origPtr as u32;
    s.nblock_used = 0;
}

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
                                    let mut i = 0;
while i < nblock {
    let uc = unsafe { *(*s).tt.offset(i as isize) } & 0xff;
    let fresh0 = unsafe { &mut *(*s).tt.offset((*s).cftab[uc as usize] as isize) };
    *fresh0 |= (i << 8) as u32;
    (*s).cftab[uc as usize] += 1;
    i += 1;
}

(*s).tPos = unsafe { *(*s).tt.offset((*s).origPtr as isize) } >> 8;
(*s).nblock_used = 0;

if (*s).blockRandomised != 0 {
    (*s).rNToGo = 0;
    (*s).rTPos = 0;

    if (*s).tPos >= (100000 * (*s).blockSize100k) as u32 {
        return 1;
    }

    (*s).tPos = unsafe { *(*s).tt.offset((*s).tPos as isize) };
    (*s).k0 = ((*s).tPos & 0xff) as u8 as i32;
    (*s).tPos >>= 8;
    (*s).nblock_used += 1;

    if (*s).rNToGo == 0 {
        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
        (*s).rTPos += 1;
        if (*s).rTPos == 512 {
            (*s).rTPos = 0;
        }
    }
    (*s).rNToGo -= 1;
    (*s).k0 ^= if (*s).rNToGo == 1 { 1 } else { 0 };
} else {
    if (*s).tPos >= (100000 * (*s).blockSize100k) as u32 {
        return 1;
    }

    (*s).tPos = unsafe { *(*s).tt.offset((*s).tPos as isize) };
    (*s).k0 = ((*s).tPos & 0xff) as u8 as i32;
    (*s).tPos >>= 8;
    (*s).nblock_used += 1;
}

                                }
                                retVal = 0 as std::os::raw::c_int;
                                current_block = 15885526978618306830;
                                continue ;

        }
    }
}
/*
The variables live at this point are:
(mut s: &mut bzlib::DState, mut current_block: u64, mut uc: u8, mut retVal: i32, mut i: i32, mut j: i32, mut nSelectors: i32, mut EOB: i32, mut groupNo: i32, mut groupPos: i32, mut nextSym: i32, mut nblockMAX: i32, mut nblock: i32, mut es: i32, mut N: i32, mut zn: i32, mut gSel: i32, mut gMinlen: i32, mut gLimit: &mut i32, mut gBase: &mut i32, mut gPerm: &mut i32)
*/


    }
    _ => {}
}

            match current_block {
    4550729491376650574 => {
        // Check that N doesn't get too big, so that es doesn't go negative.
        // The maximum value that can be RUNA/RUNB encoded is equal to the block size
        // (post the initial RLE), viz, 900k, so bounding N at 2 million should guard
        // against overflow without rejecting any legitimate inputs.
        if N >= 2 * 1024 * 1024 {
            retVal = -4;
            current_block = 15885526978618306830;
            continue;
        } else {
            es += match nextSym {
                0 => (0 + 1) * N,
                1 => (1 + 1) * N,
                _ => es, // Handle unexpected nextSym values gracefully
            };

            N *= 2;

            if groupPos == 0 {
                groupNo += 1;
                if groupNo >= nSelectors {
                    retVal = -4;
                    current_block = 15885526978618306830;
                    continue;
                } else {
                    groupPos = 50;
                    let state = unsafe { &mut *s }; // Dereference the raw pointer
                    gSel = state.selector[groupNo as usize] as i32; // Cast to i32
                    gMinlen = state.minLens[gSel as usize];
                    gLimit = &mut state.limit[gSel as usize] as *mut i32; // Get pointer to the limit
                    gPerm = &mut state.perm[gSel as usize] as *mut i32; // Get pointer to the perm
                    gBase = &mut state.base[gSel as usize] as *mut i32; // Get pointer to the base
                }
            }
            groupPos -= 1;
            zn = gMinlen;
            current_block = 5374617794059532979;
            continue;
        }
    }
    _ => {}
}

            loop {
    match current_block {
        3854024847017804838 => {
            if j < 16 {
                current_block = 1422779171932145779;
                continue;
            }
        }
        6591141407893725683 => {
            if i < nSelectors {
                j = 0;
                current_block = 6927328446518169316;
                continue;
            } else {
                if nSelectors > 2 + 900000 / 50 {
                    nSelectors = 2 + 900000 / 50;
                }
                /*--- Undo the MTF values for the selectors. ---*/
                let mut pos: [u8; 6] = [0; 6];
                let mut tmp: u8;
                let mut v_22: usize = 0;
                while v_22 < nGroups as usize {
                    pos[v_22] = v_22 as u8;
                    v_22 += 1;
                }
                i = 0;
                while i < nSelectors {
                    v_22 = unsafe { (*s).selectorMtf[i as usize] } as usize;
                    tmp = pos[v_22];
                    while v_22 > 0 {
                        pos[v_22] = pos[v_22 - 1];
                        v_22 -= 1;
                    }
                    pos[0] = tmp;
                    unsafe {
                        (*s).selector[i as usize] = tmp;
                    }
                    i += 1;
                }
                /*--- Now the coding tables ---*/
                t = 0;
                current_block = 16916874950763617094;
                break;
            }
        }
        3472349144349095221 => {
            if i < 16 {
                if unsafe { (*s).inUse16[i as usize] } != 0 {
                    j = 0;
                    current_block = 3854024847017804838;
                    continue;
                }
            } else {
                makeMaps_d(&mut s);
                if unsafe { (*s).nInUse } == 0 {
                    current_block = 11906008669688594715;
                    break;
                } else {
                    current_block = 7606051654693192361;
                    break;
                }
            }
        }
        17503523010989424999 => {
            unsafe {
                (*s).len[t as usize][i as usize] = curr as u8;
            }
            i += 1;
            current_block = 3770765986603902964;
            continue;
        }
        3770765986603902964 => {
            if i < alphaSize {
                current_block = 11858046780433112516;
                continue;
            }
            t += 1;
            current_block = 16916874950763617094;
            break;
        }
        5281038271658253520 => {
            if i < 2 + 900000 / 50 {
                unsafe {
                    (*s).selectorMtf[i as usize] = j as u8;
                }
            }
            i += 1;
            current_block = 6591141407893725683;
            continue;
        }
        6927328446518169316 => {
            if true {
                current_block = 4874723077730206021;
                continue;
            } else {
                current_block = 5281038271658253520;
                continue;
            }
        }
        _ => {
            if !true {
                current_block = 17503523010989424999;
                continue;
            }
            if !(curr < 1 || curr > 20) {
                current_block = 1736021991379636935;
                continue;
            }
            retVal = -4;
            current_block = 15885526978618306830;
            continue;
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
unsafe {
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
}
retVal

}
/*-------------------------------------------------------------*/
/*--- end                                      decompress.c ---*/
/*-------------------------------------------------------------*/

// SACTOR × bzip2 (run 2, 2026-09-02, PARTIAL, non-building): verbatim concatenation of the
// UNIDIOMATIC-phase output for the 32 functions SACTOR attempted (of 64 in the 5 library TUs):
//   - structs/global_vars/functions/*.rs from run2_result/<tu>/translated_code_unidiomatic/
//     (27 functions, each SACTOR-verified against the 4 driver samples). SACTOR re-translates
//     the shared structs per TU (bz_stream x4, EState x3, DState x2, textually different); only
//     the bzlib.c copies are included here, the others stay in run2_result/.
//   - the LAST attempt of the 5 functions SACTOR did not verify (BZ2_bzReadGetUnused, BZ2_bzerror,
//     BZ2_bzCompressInit failed 6/6 on tool-side scaffold conflicts; flush_RL and bsPutUInt32 compiled
//     but failed SACTOR's harness link on static C helpers), extracted from
//     run2_result/logs/sactor-20260902T143332.jsonl by scripts/rq1_sactor_extract_log_rust.py
//     (run2_extracted_rust/). See RUN.md. The idiomatic phase was never reached (no tool name map).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]


// --- blocksort.c__e857d15e/functions/fallbackSimpleSort.rs  [SACTOR-verified]
#[inline]
pub unsafe fn fallbackSimpleSort(
    fmap: *mut libc::c_uint,
    eclass: *const libc::c_uint,
    lo: libc::c_int,
    hi: libc::c_int,
) {
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut tmp: libc::c_int;
    let mut ec_tmp: libc::c_uint;
    if lo == hi {
        return;
    }
    if hi - lo > 3 {
        i = hi - 4;
        while i >= lo {
            tmp = *fmap.add(i as usize) as libc::c_int;
            ec_tmp = *eclass.add(tmp as usize);
            j = i + 4;
            while j <= hi && {
                let idx = *fmap.add(j as usize) as usize;
                ec_tmp > *eclass.add(idx)
            } {
                let val = *fmap.add(j as usize);
                *fmap.add((j - 4) as usize) = val;
                j += 4;
            }
            *fmap.add((j - 4) as usize) = tmp as libc::c_uint;
            i -= 1;
        }
    }
    i = hi - 1;
    while i >= lo {
        tmp = *fmap.add(i as usize) as libc::c_int;
        ec_tmp = *eclass.add(tmp as usize);
        j = i + 1;
        while j <= hi && {
            let idx = *fmap.add(j as usize) as usize;
            ec_tmp > *eclass.add(idx)
        } {
            let val = *fmap.add(j as usize);
            *fmap.add((j - 1) as usize) = val;
            j += 1;
        }
        *fmap.add((j - 1) as usize) = tmp as libc::c_uint;
        i -= 1;
    }
}


// --- blocksort.c__e857d15e/functions/mainGtU.rs  [SACTOR-verified]
pub unsafe fn mainGtU(
    mut i1: u32,
    mut i2: u32,
    block: *const u8,
    quadrant: *const u16,
    nblock: u32,
    budget: *mut i32,
) -> bool {
    let mut k: i32;
    let mut c1: u8;
    let mut c2: u8;
    let mut s1: u16;
    let mut s2: u16;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    c1 = *block.add(i1 as usize);
    c2 = *block.add(i2 as usize);
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;
    k = (nblock + 8) as i32;
    loop {
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        c1 = *block.add(i1 as usize);
        c2 = *block.add(i2 as usize);
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = *quadrant.add(i1 as usize);
        s2 = *quadrant.add(i2 as usize);
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;
        if i1 >= nblock {
            i1 -= nblock;
        }
        if i2 >= nblock {
            i2 -= nblock;
        }
        k -= 8;
        *budget -= 1;
        if k < 0 {
            break;
        }
    }
    false
}


// --- blocksort.c__e857d15e/functions/mmed3.rs  [SACTOR-verified]
#[inline]
pub fn mmed3(mut a: libc::c_uchar, mut b: libc::c_uchar, c: libc::c_uchar) -> libc::c_uchar {
    let mut t: libc::c_uchar;
    if a > b {
        t = a;
        a = b;
        b = t;
    }
    if b > c {
        b = c;
        if a > b {
            b = a;
        }
    }
    b
}


// --- bzlib.c__c5155842/global_vars/BZ2_crc32Table.rs  [SACTOR-verified]
#[no_mangle]
pub static BZ2_crc32Table: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4c11db7 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9823b6e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd4326d9 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x130476dc as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x17c56b6b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x1a864db2 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x1e475005 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2608edb8 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x22c9f00f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2f8ad6d6 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2b4bcb61 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x350c9b64 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x31cd86d3 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3c8ea00a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x384fbdbd as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4c11db70 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x48d0c6c7 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4593e01e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4152fda9 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x5f15adac as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x5bd4b01b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x569796c2 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x52568b75 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x6a1936c8 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x6ed82b7f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x639b0da6 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x675a1011 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x791d4014 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7ddc5da3 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x709f7b7a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x745e66cd as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9823b6e0 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9ce2ab57 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x91a18d8e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x95609039 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8b27c03c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8fe6dd8b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x82a5fb52 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8664e6e5 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xbe2b5b58 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xbaea46ef as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb7a96036 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb3687d81 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xad2f2d84 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa9ee3033 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa4ad16ea as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa06c0b5d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd4326d90 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd0f37027 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xddb056fe as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd9714b49 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc7361b4c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc3f706fb as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xceb42022 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xca753d95 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf23a8028 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf6fb9d9f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xfbb8bb46 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xff79a6f1 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe13ef6f4 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe5ffeb43 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe8bccd9a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xec7dd02d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x34867077 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x30476dc0 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3d044b19 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x39c556ae as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x278206ab as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x23431b1c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2e003dc5 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2ac12072 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x128e9dcf as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x164f8078 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x1b0ca6a1 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x1fcdbb16 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x18aeb13 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x54bf6a4 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x808d07d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xcc9cdca as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7897ab07 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7c56b6b0 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x71159069 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x75d48dde as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x6b93dddb as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x6f52c06c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x6211e6b5 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x66d0fb02 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x5e9f46bf as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x5a5e5b08 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x571d7dd1 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x53dc6066 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4d9b3063 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x495a2dd4 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x44190b0d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x40d816ba as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xaca5c697 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa864db20 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa527fdf9 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa1e6e04e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xbfa1b04b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xbb60adfc as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb6238b25 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb2e29692 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8aad2b2f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8e6c3698 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x832f1041 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x87ee0df6 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x99a95df3 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9d684044 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x902b669d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x94ea7b2a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe0b41de7 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe4750050 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe9362689 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xedf73b3e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf3b06b3b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf771768c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xfa325055 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xfef34de2 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc6bcf05f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc27dede8 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xcf3ecb31 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xcbffd686 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd5b88683 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd1799b34 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xdc3abded as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd8fba05a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x690ce0ee as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x6dcdfd59 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x608edb80 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x644fc637 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7a089632 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7ec98b85 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x738aad5c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x774bb0eb as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4f040d56 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4bc510e1 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x46863638 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x42472b8f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x5c007b8a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x58c1663d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x558240e4 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x51435d53 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x251d3b9e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x21dc2629 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2c9f00f0 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x285e1d47 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x36194d42 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x32d850f5 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3f9b762c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3b5a6b9b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x315d626 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7d4cb91 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa97ed48 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe56f0ff as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x1011a0fa as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x14d0bd4d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x19939b94 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x1d528623 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf12f560e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf5ee4bb9 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf8ad6d60 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xfc6c70d7 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe22b20d2 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe6ea3d65 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xeba91bbc as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xef68060b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd727bbb6 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd3e6a601 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xdea580d8 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xda649d6f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc423cd6a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc0e2d0dd as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xcda1f604 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc960ebb3 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xbd3e8d7e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb9ff90c9 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb4bcb610 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb07daba7 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xae3afba2 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xaafbe615 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa7b8c0cc as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa379dd7b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9b3660c6 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9ff77d71 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x92b45ba8 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9675461f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8832161a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8cf30bad as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x81b02d74 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x857130c3 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x5d8a9099 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x594b8d2e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x5408abf7 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x50c9b640 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4e8ee645 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x4a4ffbf2 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x470cdd2b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x43cdc09c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7b827d21 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7f436096 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x7200464f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x76c15bf8 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x68860bfd as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x6c47164a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x61043093 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x65c52d24 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x119b4be9 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x155a565e as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x18197087 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x1cd86d30 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x29f3d35 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x65e2082 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb1d065b as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xfdc1bec as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3793a651 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3352bbe6 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3e119d3f as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x3ad08088 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2497d08d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2056cd3a as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x2d15ebe3 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x29d4f654 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc5a92679 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc1683bce as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xcc2b1d17 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xc8ea00a0 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd6ad50a5 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xd26c4d12 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xdf2f6bcb as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xdbee767c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe3a1cbc1 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xe760d676 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xea23f0af as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xeee2ed18 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf0a5bd1d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf464a0aa as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xf9278673 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xfde69bc4 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x89b8fd09 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x8d79e0be as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x803ac667 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x84fbdbd0 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9abc8bd5 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x9e7d9662 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x933eb0bb as ::core::ffi::c_long as ::core::ffi::c_uint,
    0x97ffad0c as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xafb010b1 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xab710d06 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa6322bdf as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xa2f33668 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xbcb4666d as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb8757bda as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb5365d03 as ::core::ffi::c_long as ::core::ffi::c_uint,
    0xb1f740b4 as ::core::ffi::c_long as ::core::ffi::c_uint,
];


// --- bzlib.c__c5155842/global_vars/BZ2_rNums.rs  [SACTOR-verified]
#[no_mangle]
pub static BZ2_rNums: [::core::ffi::c_int; 512] = [
    619 as ::core::ffi::c_int,
    720 as ::core::ffi::c_int,
    127 as ::core::ffi::c_int,
    481 as ::core::ffi::c_int,
    931 as ::core::ffi::c_int,
    816 as ::core::ffi::c_int,
    813 as ::core::ffi::c_int,
    233 as ::core::ffi::c_int,
    566 as ::core::ffi::c_int,
    247 as ::core::ffi::c_int,
    985 as ::core::ffi::c_int,
    724 as ::core::ffi::c_int,
    205 as ::core::ffi::c_int,
    454 as ::core::ffi::c_int,
    863 as ::core::ffi::c_int,
    491 as ::core::ffi::c_int,
    741 as ::core::ffi::c_int,
    242 as ::core::ffi::c_int,
    949 as ::core::ffi::c_int,
    214 as ::core::ffi::c_int,
    733 as ::core::ffi::c_int,
    859 as ::core::ffi::c_int,
    335 as ::core::ffi::c_int,
    708 as ::core::ffi::c_int,
    621 as ::core::ffi::c_int,
    574 as ::core::ffi::c_int,
    73 as ::core::ffi::c_int,
    654 as ::core::ffi::c_int,
    730 as ::core::ffi::c_int,
    472 as ::core::ffi::c_int,
    419 as ::core::ffi::c_int,
    436 as ::core::ffi::c_int,
    278 as ::core::ffi::c_int,
    496 as ::core::ffi::c_int,
    867 as ::core::ffi::c_int,
    210 as ::core::ffi::c_int,
    399 as ::core::ffi::c_int,
    680 as ::core::ffi::c_int,
    480 as ::core::ffi::c_int,
    51 as ::core::ffi::c_int,
    878 as ::core::ffi::c_int,
    465 as ::core::ffi::c_int,
    811 as ::core::ffi::c_int,
    169 as ::core::ffi::c_int,
    869 as ::core::ffi::c_int,
    675 as ::core::ffi::c_int,
    611 as ::core::ffi::c_int,
    697 as ::core::ffi::c_int,
    867 as ::core::ffi::c_int,
    561 as ::core::ffi::c_int,
    862 as ::core::ffi::c_int,
    687 as ::core::ffi::c_int,
    507 as ::core::ffi::c_int,
    283 as ::core::ffi::c_int,
    482 as ::core::ffi::c_int,
    129 as ::core::ffi::c_int,
    807 as ::core::ffi::c_int,
    591 as ::core::ffi::c_int,
    733 as ::core::ffi::c_int,
    623 as ::core::ffi::c_int,
    150 as ::core::ffi::c_int,
    238 as ::core::ffi::c_int,
    59 as ::core::ffi::c_int,
    379 as ::core::ffi::c_int,
    684 as ::core::ffi::c_int,
    877 as ::core::ffi::c_int,
    625 as ::core::ffi::c_int,
    169 as ::core::ffi::c_int,
    643 as ::core::ffi::c_int,
    105 as ::core::ffi::c_int,
    170 as ::core::ffi::c_int,
    607 as ::core::ffi::c_int,
    520 as ::core::ffi::c_int,
    932 as ::core::ffi::c_int,
    727 as ::core::ffi::c_int,
    476 as ::core::ffi::c_int,
    693 as ::core::ffi::c_int,
    425 as ::core::ffi::c_int,
    174 as ::core::ffi::c_int,
    647 as ::core::ffi::c_int,
    73 as ::core::ffi::c_int,
    122 as ::core::ffi::c_int,
    335 as ::core::ffi::c_int,
    530 as ::core::ffi::c_int,
    442 as ::core::ffi::c_int,
    853 as ::core::ffi::c_int,
    695 as ::core::ffi::c_int,
    249 as ::core::ffi::c_int,
    445 as ::core::ffi::c_int,
    515 as ::core::ffi::c_int,
    909 as ::core::ffi::c_int,
    545 as ::core::ffi::c_int,
    703 as ::core::ffi::c_int,
    919 as ::core::ffi::c_int,
    874 as ::core::ffi::c_int,
    474 as ::core::ffi::c_int,
    882 as ::core::ffi::c_int,
    500 as ::core::ffi::c_int,
    594 as ::core::ffi::c_int,
    612 as ::core::ffi::c_int,
    641 as ::core::ffi::c_int,
    801 as ::core::ffi::c_int,
    220 as ::core::ffi::c_int,
    162 as ::core::ffi::c_int,
    819 as ::core::ffi::c_int,
    984 as ::core::ffi::c_int,
    589 as ::core::ffi::c_int,
    513 as ::core::ffi::c_int,
    495 as ::core::ffi::c_int,
    799 as ::core::ffi::c_int,
    161 as ::core::ffi::c_int,
    604 as ::core::ffi::c_int,
    958 as ::core::ffi::c_int,
    533 as ::core::ffi::c_int,
    221 as ::core::ffi::c_int,
    400 as ::core::ffi::c_int,
    386 as ::core::ffi::c_int,
    867 as ::core::ffi::c_int,
    600 as ::core::ffi::c_int,
    782 as ::core::ffi::c_int,
    382 as ::core::ffi::c_int,
    596 as ::core::ffi::c_int,
    414 as ::core::ffi::c_int,
    171 as ::core::ffi::c_int,
    516 as ::core::ffi::c_int,
    375 as ::core::ffi::c_int,
    682 as ::core::ffi::c_int,
    485 as ::core::ffi::c_int,
    911 as ::core::ffi::c_int,
    276 as ::core::ffi::c_int,
    98 as ::core::ffi::c_int,
    553 as ::core::ffi::c_int,
    163 as ::core::ffi::c_int,
    354 as ::core::ffi::c_int,
    666 as ::core::ffi::c_int,
    933 as ::core::ffi::c_int,
    424 as ::core::ffi::c_int,
    341 as ::core::ffi::c_int,
    533 as ::core::ffi::c_int,
    870 as ::core::ffi::c_int,
    227 as ::core::ffi::c_int,
    730 as ::core::ffi::c_int,
    475 as ::core::ffi::c_int,
    186 as ::core::ffi::c_int,
    263 as ::core::ffi::c_int,
    647 as ::core::ffi::c_int,
    537 as ::core::ffi::c_int,
    686 as ::core::ffi::c_int,
    600 as ::core::ffi::c_int,
    224 as ::core::ffi::c_int,
    469 as ::core::ffi::c_int,
    68 as ::core::ffi::c_int,
    770 as ::core::ffi::c_int,
    919 as ::core::ffi::c_int,
    190 as ::core::ffi::c_int,
    373 as ::core::ffi::c_int,
    294 as ::core::ffi::c_int,
    822 as ::core::ffi::c_int,
    808 as ::core::ffi::c_int,
    206 as ::core::ffi::c_int,
    184 as ::core::ffi::c_int,
    943 as ::core::ffi::c_int,
    795 as ::core::ffi::c_int,
    384 as ::core::ffi::c_int,
    383 as ::core::ffi::c_int,
    461 as ::core::ffi::c_int,
    404 as ::core::ffi::c_int,
    758 as ::core::ffi::c_int,
    839 as ::core::ffi::c_int,
    887 as ::core::ffi::c_int,
    715 as ::core::ffi::c_int,
    67 as ::core::ffi::c_int,
    618 as ::core::ffi::c_int,
    276 as ::core::ffi::c_int,
    204 as ::core::ffi::c_int,
    918 as ::core::ffi::c_int,
    873 as ::core::ffi::c_int,
    777 as ::core::ffi::c_int,
    604 as ::core::ffi::c_int,
    560 as ::core::ffi::c_int,
    951 as ::core::ffi::c_int,
    160 as ::core::ffi::c_int,
    578 as ::core::ffi::c_int,
    722 as ::core::ffi::c_int,
    79 as ::core::ffi::c_int,
    804 as ::core::ffi::c_int,
    96 as ::core::ffi::c_int,
    409 as ::core::ffi::c_int,
    713 as ::core::ffi::c_int,
    940 as ::core::ffi::c_int,
    652 as ::core::ffi::c_int,
    934 as ::core::ffi::c_int,
    970 as ::core::ffi::c_int,
    447 as ::core::ffi::c_int,
    318 as ::core::ffi::c_int,
    353 as ::core::ffi::c_int,
    859 as ::core::ffi::c_int,
    672 as ::core::ffi::c_int,
    112 as ::core::ffi::c_int,
    785 as ::core::ffi::c_int,
    645 as ::core::ffi::c_int,
    863 as ::core::ffi::c_int,
    803 as ::core::ffi::c_int,
    350 as ::core::ffi::c_int,
    139 as ::core::ffi::c_int,
    93 as ::core::ffi::c_int,
    354 as ::core::ffi::c_int,
    99 as ::core::ffi::c_int,
    820 as ::core::ffi::c_int,
    908 as ::core::ffi::c_int,
    609 as ::core::ffi::c_int,
    772 as ::core::ffi::c_int,
    154 as ::core::ffi::c_int,
    274 as ::core::ffi::c_int,
    580 as ::core::ffi::c_int,
    184 as ::core::ffi::c_int,
    79 as ::core::ffi::c_int,
    626 as ::core::ffi::c_int,
    630 as ::core::ffi::c_int,
    742 as ::core::ffi::c_int,
    653 as ::core::ffi::c_int,
    282 as ::core::ffi::c_int,
    762 as ::core::ffi::c_int,
    623 as ::core::ffi::c_int,
    680 as ::core::ffi::c_int,
    81 as ::core::ffi::c_int,
    927 as ::core::ffi::c_int,
    626 as ::core::ffi::c_int,
    789 as ::core::ffi::c_int,
    125 as ::core::ffi::c_int,
    411 as ::core::ffi::c_int,
    521 as ::core::ffi::c_int,
    938 as ::core::ffi::c_int,
    300 as ::core::ffi::c_int,
    821 as ::core::ffi::c_int,
    78 as ::core::ffi::c_int,
    343 as ::core::ffi::c_int,
    175 as ::core::ffi::c_int,
    128 as ::core::ffi::c_int,
    250 as ::core::ffi::c_int,
    170 as ::core::ffi::c_int,
    774 as ::core::ffi::c_int,
    972 as ::core::ffi::c_int,
    275 as ::core::ffi::c_int,
    999 as ::core::ffi::c_int,
    639 as ::core::ffi::c_int,
    495 as ::core::ffi::c_int,
    78 as ::core::ffi::c_int,
    352 as ::core::ffi::c_int,
    126 as ::core::ffi::c_int,
    857 as ::core::ffi::c_int,
    956 as ::core::ffi::c_int,
    358 as ::core::ffi::c_int,
    619 as ::core::ffi::c_int,
    580 as ::core::ffi::c_int,
    124 as ::core::ffi::c_int,
    737 as ::core::ffi::c_int,
    594 as ::core::ffi::c_int,
    701 as ::core::ffi::c_int,
    612 as ::core::ffi::c_int,
    669 as ::core::ffi::c_int,
    112 as ::core::ffi::c_int,
    134 as ::core::ffi::c_int,
    694 as ::core::ffi::c_int,
    363 as ::core::ffi::c_int,
    992 as ::core::ffi::c_int,
    809 as ::core::ffi::c_int,
    743 as ::core::ffi::c_int,
    168 as ::core::ffi::c_int,
    974 as ::core::ffi::c_int,
    944 as ::core::ffi::c_int,
    375 as ::core::ffi::c_int,
    748 as ::core::ffi::c_int,
    52 as ::core::ffi::c_int,
    600 as ::core::ffi::c_int,
    747 as ::core::ffi::c_int,
    642 as ::core::ffi::c_int,
    182 as ::core::ffi::c_int,
    862 as ::core::ffi::c_int,
    81 as ::core::ffi::c_int,
    344 as ::core::ffi::c_int,
    805 as ::core::ffi::c_int,
    988 as ::core::ffi::c_int,
    739 as ::core::ffi::c_int,
    511 as ::core::ffi::c_int,
    655 as ::core::ffi::c_int,
    814 as ::core::ffi::c_int,
    334 as ::core::ffi::c_int,
    249 as ::core::ffi::c_int,
    515 as ::core::ffi::c_int,
    897 as ::core::ffi::c_int,
    955 as ::core::ffi::c_int,
    664 as ::core::ffi::c_int,
    981 as ::core::ffi::c_int,
    649 as ::core::ffi::c_int,
    113 as ::core::ffi::c_int,
    974 as ::core::ffi::c_int,
    459 as ::core::ffi::c_int,
    893 as ::core::ffi::c_int,
    228 as ::core::ffi::c_int,
    433 as ::core::ffi::c_int,
    837 as ::core::ffi::c_int,
    553 as ::core::ffi::c_int,
    268 as ::core::ffi::c_int,
    926 as ::core::ffi::c_int,
    240 as ::core::ffi::c_int,
    102 as ::core::ffi::c_int,
    654 as ::core::ffi::c_int,
    459 as ::core::ffi::c_int,
    51 as ::core::ffi::c_int,
    686 as ::core::ffi::c_int,
    754 as ::core::ffi::c_int,
    806 as ::core::ffi::c_int,
    760 as ::core::ffi::c_int,
    493 as ::core::ffi::c_int,
    403 as ::core::ffi::c_int,
    415 as ::core::ffi::c_int,
    394 as ::core::ffi::c_int,
    687 as ::core::ffi::c_int,
    700 as ::core::ffi::c_int,
    946 as ::core::ffi::c_int,
    670 as ::core::ffi::c_int,
    656 as ::core::ffi::c_int,
    610 as ::core::ffi::c_int,
    738 as ::core::ffi::c_int,
    392 as ::core::ffi::c_int,
    760 as ::core::ffi::c_int,
    799 as ::core::ffi::c_int,
    887 as ::core::ffi::c_int,
    653 as ::core::ffi::c_int,
    978 as ::core::ffi::c_int,
    321 as ::core::ffi::c_int,
    576 as ::core::ffi::c_int,
    617 as ::core::ffi::c_int,
    626 as ::core::ffi::c_int,
    502 as ::core::ffi::c_int,
    894 as ::core::ffi::c_int,
    679 as ::core::ffi::c_int,
    243 as ::core::ffi::c_int,
    440 as ::core::ffi::c_int,
    680 as ::core::ffi::c_int,
    879 as ::core::ffi::c_int,
    194 as ::core::ffi::c_int,
    572 as ::core::ffi::c_int,
    640 as ::core::ffi::c_int,
    724 as ::core::ffi::c_int,
    926 as ::core::ffi::c_int,
    56 as ::core::ffi::c_int,
    204 as ::core::ffi::c_int,
    700 as ::core::ffi::c_int,
    707 as ::core::ffi::c_int,
    151 as ::core::ffi::c_int,
    457 as ::core::ffi::c_int,
    449 as ::core::ffi::c_int,
    797 as ::core::ffi::c_int,
    195 as ::core::ffi::c_int,
    791 as ::core::ffi::c_int,
    558 as ::core::ffi::c_int,
    945 as ::core::ffi::c_int,
    679 as ::core::ffi::c_int,
    297 as ::core::ffi::c_int,
    59 as ::core::ffi::c_int,
    87 as ::core::ffi::c_int,
    824 as ::core::ffi::c_int,
    713 as ::core::ffi::c_int,
    663 as ::core::ffi::c_int,
    412 as ::core::ffi::c_int,
    693 as ::core::ffi::c_int,
    342 as ::core::ffi::c_int,
    606 as ::core::ffi::c_int,
    134 as ::core::ffi::c_int,
    108 as ::core::ffi::c_int,
    571 as ::core::ffi::c_int,
    364 as ::core::ffi::c_int,
    631 as ::core::ffi::c_int,
    212 as ::core::ffi::c_int,
    174 as ::core::ffi::c_int,
    643 as ::core::ffi::c_int,
    304 as ::core::ffi::c_int,
    329 as ::core::ffi::c_int,
    343 as ::core::ffi::c_int,
    97 as ::core::ffi::c_int,
    430 as ::core::ffi::c_int,
    751 as ::core::ffi::c_int,
    497 as ::core::ffi::c_int,
    314 as ::core::ffi::c_int,
    983 as ::core::ffi::c_int,
    374 as ::core::ffi::c_int,
    822 as ::core::ffi::c_int,
    928 as ::core::ffi::c_int,
    140 as ::core::ffi::c_int,
    206 as ::core::ffi::c_int,
    73 as ::core::ffi::c_int,
    263 as ::core::ffi::c_int,
    980 as ::core::ffi::c_int,
    736 as ::core::ffi::c_int,
    876 as ::core::ffi::c_int,
    478 as ::core::ffi::c_int,
    430 as ::core::ffi::c_int,
    305 as ::core::ffi::c_int,
    170 as ::core::ffi::c_int,
    514 as ::core::ffi::c_int,
    364 as ::core::ffi::c_int,
    692 as ::core::ffi::c_int,
    829 as ::core::ffi::c_int,
    82 as ::core::ffi::c_int,
    855 as ::core::ffi::c_int,
    953 as ::core::ffi::c_int,
    676 as ::core::ffi::c_int,
    246 as ::core::ffi::c_int,
    369 as ::core::ffi::c_int,
    970 as ::core::ffi::c_int,
    294 as ::core::ffi::c_int,
    750 as ::core::ffi::c_int,
    807 as ::core::ffi::c_int,
    827 as ::core::ffi::c_int,
    150 as ::core::ffi::c_int,
    790 as ::core::ffi::c_int,
    288 as ::core::ffi::c_int,
    923 as ::core::ffi::c_int,
    804 as ::core::ffi::c_int,
    378 as ::core::ffi::c_int,
    215 as ::core::ffi::c_int,
    828 as ::core::ffi::c_int,
    592 as ::core::ffi::c_int,
    281 as ::core::ffi::c_int,
    565 as ::core::ffi::c_int,
    555 as ::core::ffi::c_int,
    710 as ::core::ffi::c_int,
    82 as ::core::ffi::c_int,
    896 as ::core::ffi::c_int,
    831 as ::core::ffi::c_int,
    547 as ::core::ffi::c_int,
    261 as ::core::ffi::c_int,
    524 as ::core::ffi::c_int,
    462 as ::core::ffi::c_int,
    293 as ::core::ffi::c_int,
    465 as ::core::ffi::c_int,
    502 as ::core::ffi::c_int,
    56 as ::core::ffi::c_int,
    661 as ::core::ffi::c_int,
    821 as ::core::ffi::c_int,
    976 as ::core::ffi::c_int,
    991 as ::core::ffi::c_int,
    658 as ::core::ffi::c_int,
    869 as ::core::ffi::c_int,
    905 as ::core::ffi::c_int,
    758 as ::core::ffi::c_int,
    745 as ::core::ffi::c_int,
    193 as ::core::ffi::c_int,
    768 as ::core::ffi::c_int,
    550 as ::core::ffi::c_int,
    608 as ::core::ffi::c_int,
    933 as ::core::ffi::c_int,
    378 as ::core::ffi::c_int,
    286 as ::core::ffi::c_int,
    215 as ::core::ffi::c_int,
    979 as ::core::ffi::c_int,
    792 as ::core::ffi::c_int,
    961 as ::core::ffi::c_int,
    61 as ::core::ffi::c_int,
    688 as ::core::ffi::c_int,
    793 as ::core::ffi::c_int,
    644 as ::core::ffi::c_int,
    986 as ::core::ffi::c_int,
    403 as ::core::ffi::c_int,
    106 as ::core::ffi::c_int,
    366 as ::core::ffi::c_int,
    905 as ::core::ffi::c_int,
    644 as ::core::ffi::c_int,
    372 as ::core::ffi::c_int,
    567 as ::core::ffi::c_int,
    466 as ::core::ffi::c_int,
    434 as ::core::ffi::c_int,
    645 as ::core::ffi::c_int,
    210 as ::core::ffi::c_int,
    389 as ::core::ffi::c_int,
    550 as ::core::ffi::c_int,
    919 as ::core::ffi::c_int,
    135 as ::core::ffi::c_int,
    780 as ::core::ffi::c_int,
    773 as ::core::ffi::c_int,
    635 as ::core::ffi::c_int,
    389 as ::core::ffi::c_int,
    707 as ::core::ffi::c_int,
    100 as ::core::ffi::c_int,
    626 as ::core::ffi::c_int,
    958 as ::core::ffi::c_int,
    165 as ::core::ffi::c_int,
    504 as ::core::ffi::c_int,
    920 as ::core::ffi::c_int,
    176 as ::core::ffi::c_int,
    193 as ::core::ffi::c_int,
    713 as ::core::ffi::c_int,
    857 as ::core::ffi::c_int,
    265 as ::core::ffi::c_int,
    203 as ::core::ffi::c_int,
    50 as ::core::ffi::c_int,
    668 as ::core::ffi::c_int,
    108 as ::core::ffi::c_int,
    645 as ::core::ffi::c_int,
    990 as ::core::ffi::c_int,
    626 as ::core::ffi::c_int,
    197 as ::core::ffi::c_int,
    510 as ::core::ffi::c_int,
    357 as ::core::ffi::c_int,
    358 as ::core::ffi::c_int,
    850 as ::core::ffi::c_int,
    858 as ::core::ffi::c_int,
    364 as ::core::ffi::c_int,
    936 as ::core::ffi::c_int,
    638 as ::core::ffi::c_int,
];


// --- bzlib.c__c5155842/global_vars/bzerrorstrings.rs  [SACTOR-verified]
static bzerrorstrings: [*const ::core::ffi::c_char; 16] = [
    b"OK\0" as *const u8 as *const ::core::ffi::c_char,
    b"SEQUENCE_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
    b"PARAM_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
    b"MEM_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
    b"DATA_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
    b"DATA_ERROR_MAGIC\0" as *const u8 as *const ::core::ffi::c_char,
    b"IO_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNEXPECTED_EOF\0" as *const u8 as *const ::core::ffi::c_char,
    b"OUTBUFF_FULL\0" as *const u8 as *const ::core::ffi::c_char,
    b"CONFIG_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
    b"???\0" as *const u8 as *const ::core::ffi::c_char,
    b"???\0" as *const u8 as *const ::core::ffi::c_char,
    b"???\0" as *const u8 as *const ::core::ffi::c_char,
    b"???\0" as *const u8 as *const ::core::ffi::c_char,
    b"???\0" as *const u8 as *const ::core::ffi::c_char,
    b"???\0" as *const u8 as *const ::core::ffi::c_char,
];


// --- bzlib.c__c5155842/structs/DState.rs  [SACTOR-verified]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut ::core::ffi::c_char,
    pub avail_in: ::core::ffi::c_uint,
    pub total_in_lo32: ::core::ffi::c_uint,
    pub total_in_hi32: ::core::ffi::c_uint,
    pub next_out: *mut ::core::ffi::c_char,
    pub avail_out: ::core::ffi::c_uint,
    pub total_out_lo32: ::core::ffi::c_uint,
    pub total_out_hi32: ::core::ffi::c_uint,
    pub state: *mut ::core::ffi::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub bzfree:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
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
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DState {
    pub strm: *mut bz_stream,
    pub state: ::core::ffi::c_int,
    pub state_out_ch: ::core::ffi::c_uchar,
    pub state_out_len: ::core::ffi::c_int,
    pub blockRandomised: ::core::ffi::c_uchar,
    pub rNToGo: ::core::ffi::c_int,
    pub rTPos: ::core::ffi::c_int,
    pub bsBuff: ::core::ffi::c_uint,
    pub bsLive: ::core::ffi::c_int,
    pub blockSize100k: ::core::ffi::c_int,
    pub smallDecompress: ::core::ffi::c_uchar,
    pub currBlockNo: ::core::ffi::c_int,
    pub verbosity: ::core::ffi::c_int,
    pub origPtr: ::core::ffi::c_int,
    pub tPos: ::core::ffi::c_uint,
    pub k0: ::core::ffi::c_int,
    pub unzftab: [::core::ffi::c_int; 256],
    pub nblock_used: ::core::ffi::c_int,
    pub cftab: [::core::ffi::c_int; 257],
    pub cftabCopy: [::core::ffi::c_int; 257],
    pub tt: *mut ::core::ffi::c_uint,
    pub ll16: *mut ::core::ffi::c_ushort,
    pub ll4: *mut ::core::ffi::c_uchar,
    pub storedBlockCRC: ::core::ffi::c_uint,
    pub storedCombinedCRC: ::core::ffi::c_uint,
    pub calculatedBlockCRC: ::core::ffi::c_uint,
    pub calculatedCombinedCRC: ::core::ffi::c_uint,
    pub nInUse: ::core::ffi::c_int,
    pub inUse: [::core::ffi::c_uchar; 256],
    pub inUse16: [::core::ffi::c_uchar; 16],
    pub seqToUnseq: [::core::ffi::c_uchar; 256],
    pub mtfa: [::core::ffi::c_uchar; 4096],
    pub mtfbase: [::core::ffi::c_int; 16],
    pub selector: [::core::ffi::c_uchar; 18002],
    pub selectorMtf: [::core::ffi::c_uchar; 18002],
    pub len: [[::core::ffi::c_uchar; 258]; 6],
    pub limit: [[::core::ffi::c_int; 258]; 6],
    pub base: [[::core::ffi::c_int; 258]; 6],
    pub perm: [[::core::ffi::c_int; 258]; 6],
    pub minLens: [::core::ffi::c_int; 6],
    pub save_i: ::core::ffi::c_int,
    pub save_j: ::core::ffi::c_int,
    pub save_t: ::core::ffi::c_int,
    pub save_alphaSize: ::core::ffi::c_int,
    pub save_nGroups: ::core::ffi::c_int,
    pub save_nSelectors: ::core::ffi::c_int,
    pub save_EOB: ::core::ffi::c_int,
    pub save_groupNo: ::core::ffi::c_int,
    pub save_groupPos: ::core::ffi::c_int,
    pub save_nextSym: ::core::ffi::c_int,
    pub save_nblockMAX: ::core::ffi::c_int,
    pub save_nblock: ::core::ffi::c_int,
    pub save_es: ::core::ffi::c_int,
    pub save_N: ::core::ffi::c_int,
    pub save_curr: ::core::ffi::c_int,
    pub save_zt: ::core::ffi::c_int,
    pub save_zn: ::core::ffi::c_int,
    pub save_zvec: ::core::ffi::c_int,
    pub save_zj: ::core::ffi::c_int,
    pub save_gSel: ::core::ffi::c_int,
    pub save_gMinlen: ::core::ffi::c_int,
    pub save_gLimit: *mut ::core::ffi::c_int,
    pub save_gBase: *mut ::core::ffi::c_int,
    pub save_gPerm: *mut ::core::ffi::c_int,
}


// --- bzlib.c__c5155842/structs/EState.rs  [SACTOR-verified]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut ::core::ffi::c_char,
    pub avail_in: ::core::ffi::c_uint,
    pub total_in_lo32: ::core::ffi::c_uint,
    pub total_in_hi32: ::core::ffi::c_uint,
    pub next_out: *mut ::core::ffi::c_char,
    pub avail_out: ::core::ffi::c_uint,
    pub total_out_lo32: ::core::ffi::c_uint,
    pub total_out_hi32: ::core::ffi::c_uint,
    pub state: *mut ::core::ffi::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub bzfree:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
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
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: ::core::ffi::c_int,
    pub state: ::core::ffi::c_int,
    pub avail_in_expect: ::core::ffi::c_uint,
    pub arr1: *mut ::core::ffi::c_uint,
    pub arr2: *mut ::core::ffi::c_uint,
    pub ftab: *mut ::core::ffi::c_uint,
    pub origPtr: ::core::ffi::c_int,
    pub ptr: *mut ::core::ffi::c_uint,
    pub block: *mut ::core::ffi::c_uchar,
    pub mtfv: *mut ::core::ffi::c_ushort,
    pub zbits: *mut ::core::ffi::c_uchar,
    pub workFactor: ::core::ffi::c_int,
    pub state_in_ch: ::core::ffi::c_uint,
    pub state_in_len: ::core::ffi::c_int,
    pub rNToGo: ::core::ffi::c_int,
    pub rTPos: ::core::ffi::c_int,
    pub nblock: ::core::ffi::c_int,
    pub nblockMAX: ::core::ffi::c_int,
    pub numZ: ::core::ffi::c_int,
    pub state_out_pos: ::core::ffi::c_int,
    pub nInUse: ::core::ffi::c_int,
    pub inUse: [::core::ffi::c_uchar; 256],
    pub unseqToSeq: [::core::ffi::c_uchar; 256],
    pub bsBuff: ::core::ffi::c_uint,
    pub bsLive: ::core::ffi::c_int,
    pub blockCRC: ::core::ffi::c_uint,
    pub combinedCRC: ::core::ffi::c_uint,
    pub verbosity: ::core::ffi::c_int,
    pub blockNo: ::core::ffi::c_int,
    pub blockSize100k: ::core::ffi::c_int,
    pub nMTF: ::core::ffi::c_int,
    pub mtfFreq: [::core::ffi::c_int; 258],
    pub selector: [::core::ffi::c_uchar; 18002],
    pub selectorMtf: [::core::ffi::c_uchar; 18002],
    pub len: [[::core::ffi::c_uchar; 258]; 6],
    pub code: [[::core::ffi::c_int; 258]; 6],
    pub rfreq: [[::core::ffi::c_int; 258]; 6],
    pub len_pack: [[::core::ffi::c_uint; 4]; 258],
}


// --- bzlib.c__c5155842/structs/bzFile.rs  [SACTOR-verified]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut ::core::ffi::c_char,
    pub avail_in: ::core::ffi::c_uint,
    pub total_in_lo32: ::core::ffi::c_uint,
    pub total_in_hi32: ::core::ffi::c_uint,
    pub next_out: *mut ::core::ffi::c_char,
    pub avail_out: ::core::ffi::c_uint,
    pub total_out_lo32: ::core::ffi::c_uint,
    pub total_out_hi32: ::core::ffi::c_uint,
    pub state: *mut ::core::ffi::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub bzfree:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
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
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct bzFile {
    pub handle: *mut FILE,
    pub buf: [::core::ffi::c_char; 5000],
    pub bufN: ::core::ffi::c_int,
    pub writing: ::core::ffi::c_uchar,
    pub strm: bz_stream,
    pub lastErr: ::core::ffi::c_int,
    pub initialisedOk: ::core::ffi::c_uchar,
}


// --- bzlib.c__c5155842/structs/bz_stream.rs  [SACTOR-verified]
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
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
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut ::core::ffi::c_char,
    pub avail_in: ::core::ffi::c_uint,
    pub total_in_lo32: ::core::ffi::c_uint,
    pub total_in_hi32: ::core::ffi::c_uint,
    pub next_out: *mut ::core::ffi::c_char,
    pub avail_out: ::core::ffi::c_uint,
    pub total_out_lo32: ::core::ffi::c_uint,
    pub total_out_hi32: ::core::ffi::c_uint,
    pub state: *mut ::core::ffi::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub bzfree:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}


// --- bzlib.c__c5155842/functions/BZ2_bzCompressEnd.rs  [SACTOR-verified]
pub unsafe fn BZ2_bzCompressEnd(strm: *mut bz_stream) -> ::core::ffi::c_int {
    use core::ffi::{c_int, c_void};
    const BZ_OK: c_int = 0;
    const BZ_PARAM_ERROR: c_int = -2;
    if strm.is_null() {
        return BZ_PARAM_ERROR;
    }
    let s_ptr = (*strm).state as *mut EState;
    if s_ptr.is_null() {
        return BZ_PARAM_ERROR;
    }
    if (*s_ptr).strm != strm {
        return BZ_PARAM_ERROR;
    }
    unsafe fn bzfree_call(strm: *mut bz_stream, ptr: *mut c_void) {
        if ptr.is_null() {
            return;
        }
        if let Some(f) = (*strm).bzfree {
            f((*strm).opaque, ptr);
        }
    }
    bzfree_call(strm, (*s_ptr).arr1 as *mut c_void);
    bzfree_call(strm, (*s_ptr).arr2 as *mut c_void);
    bzfree_call(strm, (*s_ptr).ftab as *mut c_void);
    let state_ptr = (*strm).state;
    if !state_ptr.is_null() {
        bzfree_call(strm, state_ptr);
    }
    (*strm).state = ::core::ptr::null_mut::<c_void>();
    BZ_OK
}


// --- bzlib.c__c5155842/functions/BZ2_bzDecompressEnd.rs  [SACTOR-verified]
pub unsafe fn BZ2_bzDecompressEnd(strm: *mut bz_stream) -> ::core::ffi::c_int {
    const BZ_OK: ::core::ffi::c_int = 0;
    const BZ_PARAM_ERROR: ::core::ffi::c_int = -2;
    if strm.is_null() {
        return BZ_PARAM_ERROR;
    }
    let s_ptr = (*strm).state as *mut DState;
    if s_ptr.is_null() {
        return BZ_PARAM_ERROR;
    }
    let s = &mut *s_ptr;
    if s.strm != strm {
        return BZ_PARAM_ERROR;
    }
    if !s.tt.is_null() {
        if let Some(bzfree_fn) = (*strm).bzfree {
            bzfree_fn((*strm).opaque, s.tt as *mut ::core::ffi::c_void);
        }
    }
    if !s.ll16.is_null() {
        if let Some(bzfree_fn) = (*strm).bzfree {
            bzfree_fn((*strm).opaque, s.ll16 as *mut ::core::ffi::c_void);
        }
    }
    if !s.ll4.is_null() {
        if let Some(bzfree_fn) = (*strm).bzfree {
            bzfree_fn((*strm).opaque, s.ll4 as *mut ::core::ffi::c_void);
        }
    }
    if let Some(bzfree_fn) = (*strm).bzfree {
        bzfree_fn((*strm).opaque, (*strm).state as *mut ::core::ffi::c_void);
    }
    (*strm).state = ::core::ptr::null_mut();
    BZ_OK
}


// --- bzlib.c__c5155842/functions/BZ2_bz__AssertH__fail.rs  [SACTOR-verified]
pub unsafe fn BZ2_bz__AssertH__fail(errcode: libc::c_int) {
    libc::fprintf(
        stderr,
        b"\n\nbzip2/libbzip2: internal error number %d.\n\
          This is a bug in bzip2/libbzip2, %s.\n\
          Please report it to: bzip2-devel@sourceware.org.  If this happened\n\
          when you were using some program which uses libbzip2 as a\n\
          component, you should also report this bug to the author(s)\n\
          of that program.  Please make an effort to report this bug;\n\
          timely and accurate bug reports eventually lead to higher\n\
          quality software.  Thanks.\n\n\0" as *const u8 as *const libc::c_char,
        errcode,
        BZ2_bzlibVersion(),
    );
    if errcode == 1007 {
        libc::fprintf(
            stderr,
            b"\n*** A special note about internal error number 1007 ***\n\
              \n\
              Experience suggests that a common cause of i.e. 1007\n\
              is unreliable memory or other hardware.  The 1007 assertion\n\
              just happens to cross-check the results of huge numbers of\n\
              memory reads/writes, and so acts (unintendedly) as a stress\n\
              test of your memory system.\n\
              \n\
              I suggest the following: try compressing the file again,\n\
              possibly monitoring progress in detail with the -vv flag.\n\
              \n\
              * If the error cannot be reproduced, and/or happens at different\n\
              *  points in compression, you may have a flaky memory system.\n\
              *  Try a memory-test program.  I have used Memtest86\n\
              *  (www.memtest86.com).  At the time of writing it is free (GPLd).\n\
              *  Memtest86 tests memory much more thorougly than your BIOSs\n\
              *  power-on test, and may find failures that the BIOS doesn't.\n\
              \n\
              * If the error can be repeatably reproduced, this is a bug in\n\
              *  bzip2, and I would very much like to hear about it.  Please\n\
              *  let me know, and, ideally, save a copy of the file causing the\n\
              *  problem -- without which I will be unable to investigate it.\n\
              \n\0" as *const u8 as *const libc::c_char,
        );
    }
    libc::exit(3);
}


// --- bzlib.c__c5155842/functions/BZ2_bzflush.rs  [SACTOR-verified]
pub unsafe fn BZ2_bzflush(b: *mut libc::c_void) -> libc::c_int {
    0
}


// --- bzlib.c__c5155842/functions/BZ2_bzlibVersion.rs  [SACTOR-verified]
pub unsafe fn BZ2_bzlibVersion() -> *const libc::c_char {
    b"1.0.8, 13-Jul-2019\0".as_ptr() as *const libc::c_char
}


// --- bzlib.c__c5155842/functions/BZ2_indexIntoF.rs  [SACTOR-verified]
pub unsafe fn BZ2_indexIntoF(indx: i32, cftab: *mut i32) -> i32 {
    let mut nb: i32 = 0;
    let mut na: i32 = 256;
    let mut mid: i32;
    loop {
        mid = (nb + na) >> 1;
        if indx >= *cftab.add(mid as usize) {
            nb = mid;
        } else {
            na = mid;
        }
        if na - nb == 1 {
            break;
        }
    }
    nb
}


// --- bzlib.c__c5155842/functions/add_pair_to_block.rs  [SACTOR-verified]
pub unsafe fn add_pair_to_block(s: *mut EState) {
    unsafe fn bz_update_crc(crc_var: &mut u32, cha: u8) {
        let idx = ((*crc_var >> 24) ^ cha as u32) as usize;
        *crc_var = (*crc_var << 8) ^ BZ2_crc32Table[idx];
    }
    let s_ref: &mut EState = &mut *s;
    let mut i: i32 = 0;
    let ch: u8 = s_ref.state_in_ch as u8;
    while i < s_ref.state_in_len {
        bz_update_crc(&mut s_ref.blockCRC, ch);
        i += 1;
    }
    s_ref.inUse[s_ref.state_in_ch as usize] = 1u8;
    match s_ref.state_in_len {
        1 => {
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
        }
        2 => {
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
        }
        3 => {
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
        }
        _ => {
            let idx = (s_ref.state_in_len - 4) as usize;
            s_ref.inUse[idx] = 1u8;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = ch;
            s_ref.nblock += 1;
            *s_ref.block.add(s_ref.nblock as usize) = (s_ref.state_in_len - 4) as u8;
            s_ref.nblock += 1;
        }
    }
}


// --- bzlib.c__c5155842/functions/bz_config_ok.rs  [SACTOR-verified]
pub fn bz_config_ok() -> libc::c_int {
    fn size_of<T>() -> usize {
        std::mem::size_of::<T>()
    }
    if size_of::<libc::c_int>() != 4 {
        return 0;
    }
    if size_of::<libc::c_short>() != 2 {
        return 0;
    }
    if size_of::<libc::c_char>() != 1 {
        return 0;
    }
    1
}


// --- bzlib.c__c5155842/functions/copy_output_until_stop.rs  [SACTOR-verified]
pub unsafe fn copy_output_until_stop(s: *mut EState) -> bool {
    let mut progress_out: bool = false;
    loop {
        if (*(*s).strm).avail_out == 0 {
            break;
        }
        if (*s).state_out_pos >= (*s).numZ {
            break;
        }
        progress_out = true;
        *(*(*s).strm).next_out =
            *(*s).zbits.add((*s).state_out_pos as usize) as ::core::ffi::c_char;
        (*s).state_out_pos += 1;
        (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(1);
        (*(*s).strm).next_out = (*(*s).strm).next_out.add(1);
        (*(*s).strm).total_out_lo32 = (*(*s).strm).total_out_lo32.wrapping_add(1);
        if (*(*s).strm).total_out_lo32 == 0 {
            (*(*s).strm).total_out_hi32 = (*(*s).strm).total_out_hi32.wrapping_add(1);
        }
    }
    progress_out
}


// --- bzlib.c__c5155842/functions/default_bzalloc.rs  [SACTOR-verified]
use libc::{c_int, c_void, malloc, size_t};
pub unsafe fn default_bzalloc(opaque: *mut c_void, items: c_int, size: c_int) -> *mut c_void {
    unsafe fn mul_to_size_t(a: c_int, b: c_int) -> size_t {
        (a as size_t).wrapping_mul(b as size_t)
    }
    let total_size: size_t = mul_to_size_t(items, size);
    let v = malloc(total_size);
    v
}


// --- bzlib.c__c5155842/functions/default_bzfree.rs  [SACTOR-verified]
pub unsafe fn default_bzfree(opaque: *mut libc::c_void, addr: *mut libc::c_void) {
    if !addr.is_null() {
        libc::free(addr);
    }
}


// --- bzlib.c__c5155842/functions/init_RL.rs  [SACTOR-verified]
pub unsafe fn init_RL(s: *mut EState) {
    if !s.is_null() {
        (*s).state_in_ch = 256;
        (*s).state_in_len = 0;
    }
}


// --- bzlib.c__c5155842/functions/isempty_RL.rs  [SACTOR-verified]
pub unsafe fn isempty_RL(s: *mut EState) -> ::core::ffi::c_int {
    if !s.is_null() && (*s).state_in_ch < 256 && (*s).state_in_len > 0 {
        0
    } else {
        1
    }
}


// --- bzlib.c__c5155842/functions/myfeof.rs  [SACTOR-verified]
pub unsafe fn myfeof(f: *mut libc::FILE) -> libc::c_int {
    let c: libc::c_int = libc::fgetc(f);
    if c == -1 {
        return 1;
    }
    libc::ungetc(c, f);
    0
}


// --- bzlib.c__c5155842/functions/prepare_new_block.rs  [SACTOR-verified]
pub unsafe fn prepare_new_block(s: *mut EState) {
    (*s).nblock = 0;
    (*s).numZ = 0;
    (*s).state_out_pos = 0;
    (*s).blockCRC = 0xffffffffu32;
    let mut i: ::core::ffi::c_int = 0;
    while i < 256 {
        (*s).inUse[i as usize] = 0u8;
        i += 1;
    }
    (*s).blockNo += 1;
}


// --- bzlib.c__c5155842/functions/unRLE_obuf_to_output_FAST.rs  [SACTOR-verified]
#[no_mangle]
pub unsafe fn unRLE_obuf_to_output_FAST(s: *mut DState) -> ::core::ffi::c_int {
    use core::ffi::{c_int, c_uchar, c_uint};
    const TRUE: c_int = 1;
    const FALSE: c_int = 0;
    #[inline]
    unsafe fn bz_update_crc(crc_var: &mut c_uint, cha: c_uchar) {
        let idx = ((*crc_var >> 24) ^ cha as c_uint) as usize;
        *crc_var = (*crc_var << 8) ^ BZ2_crc32Table[idx];
    }
    #[inline]
    unsafe fn bz_get_fast(s: *mut DState, out: &mut c_uchar) -> c_int {
        let s_ref = &mut *s;
        if s_ref.tPos >= 100000u32.wrapping_mul(s_ref.blockSize100k as c_uint) {
            return TRUE;
        }
        let t_pos = s_ref.tPos as usize;
        s_ref.tPos = *s_ref.tt.add(t_pos);
        *out = (s_ref.tPos & 0xff) as c_uchar;
        s_ref.tPos >>= 8;
        FALSE
    }
    #[inline]
    unsafe fn bz_rand_upd_mask(s: *mut DState) {
        let s_ref = &mut *s;
        if s_ref.rNToGo == 0 {
            s_ref.rNToGo = BZ2_rNums[s_ref.rTPos as usize];
            s_ref.rTPos += 1;
            if s_ref.rTPos == 512 {
                s_ref.rTPos = 0;
            }
        }
        s_ref.rNToGo -= 1;
    }
    #[inline]
    unsafe fn bz_get_fast_c(
        c_tPos: &mut c_uint,
        c_tt: *mut c_uint,
        ro_blockSize100k: c_int,
        out: &mut c_uchar,
    ) -> c_int {
        if *c_tPos >= 100000u32.wrapping_mul(ro_blockSize100k as c_uint) {
            return TRUE;
        }
        let idx = *c_tPos as usize;
        *c_tPos = *c_tt.add(idx);
        *out = (*c_tPos & 0xff) as c_uchar;
        *c_tPos >>= 8;
        FALSE
    }
    let s_ref = &mut *s;
    if s_ref.blockRandomised != 0 {
        let mut k1: c_uchar = 0;
        loop {
            loop {
                if (*s_ref.strm).avail_out == 0 {
                    return FALSE;
                }
                if s_ref.state_out_len == 0 {
                    break;
                }
                *(*s_ref.strm).next_out.cast::<c_uchar>() = s_ref.state_out_ch;
                bz_update_crc(&mut s_ref.calculatedBlockCRC, s_ref.state_out_ch);
                s_ref.state_out_len -= 1;
                (*s_ref.strm).next_out = (*s_ref.strm).next_out.add(1);
                (*s_ref.strm).avail_out = (*s_ref.strm).avail_out.wrapping_sub(1);
                (*s_ref.strm).total_out_lo32 = (*s_ref.strm).total_out_lo32.wrapping_add(1);
                if (*s_ref.strm).total_out_lo32 == 0 {
                    (*s_ref.strm).total_out_hi32 = (*s_ref.strm).total_out_hi32.wrapping_add(1);
                }
            }
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                return FALSE;
            }
            if s_ref.nblock_used > s_ref.save_nblock + 1 {
                return TRUE;
            }
            s_ref.state_out_len = 1;
            s_ref.state_out_ch = s_ref.k0 as c_uchar;
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                continue;
            }
            if k1 as c_int != s_ref.k0 {
                s_ref.k0 = k1 as c_int;
                continue;
            }
            s_ref.state_out_len = 2;
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                continue;
            }
            if k1 as c_int != s_ref.k0 {
                s_ref.k0 = k1 as c_int;
                continue;
            }
            s_ref.state_out_len = 3;
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            if s_ref.nblock_used == s_ref.save_nblock + 1 {
                continue;
            }
            if k1 as c_int != s_ref.k0 {
                s_ref.k0 = k1 as c_int;
                continue;
            }
            if bz_get_fast(s, &mut k1) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            k1 ^= rand_mask;
            s_ref.nblock_used += 1;
            s_ref.state_out_len = (k1 as c_int) + 4;
            let mut new_k0: c_uchar = 0;
            if bz_get_fast(s, &mut new_k0) != FALSE {
                return TRUE;
            }
            bz_rand_upd_mask(s);
            let rand_mask: c_uchar = if s_ref.rNToGo == 1 { 1 } else { 0 };
            s_ref.k0 = (new_k0 ^ rand_mask) as c_int;
            s_ref.nblock_used += 1;
        }
    } else {
        let mut c_calculatedBlockCRC: c_uint = s_ref.calculatedBlockCRC;
        let mut c_state_out_ch: c_uchar = s_ref.state_out_ch;
        let mut c_state_out_len: c_int = s_ref.state_out_len;
        let mut c_nblock_used: c_int = s_ref.nblock_used;
        let mut c_k0: c_int = s_ref.k0;
        let c_tt: *mut c_uint = s_ref.tt;
        let mut c_tPos: c_uint = s_ref.tPos;
        let mut cs_next_out: *mut ::core::ffi::c_char = (*s_ref.strm).next_out;
        let mut cs_avail_out: c_uint = (*s_ref.strm).avail_out;
        let ro_blockSize100k: c_int = s_ref.blockSize100k;
        let avail_out_INIT: c_uint = cs_avail_out;
        let s_save_nblockPP: c_int = s_ref.save_nblock + 1;
        let mut total_out_lo32_old: c_uint;
        let mut k1: c_uchar = 0;
        'main_loop: loop {
            if c_state_out_len > 0 {
                loop {
                    if cs_avail_out == 0 {
                        break 'main_loop;
                    }
                    if c_state_out_len == 1 {
                        break;
                    }
                    *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                    bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                    c_state_out_len -= 1;
                    cs_next_out = cs_next_out.add(1);
                    cs_avail_out = cs_avail_out.wrapping_sub(1);
                }
                {
                    if cs_avail_out == 0 {
                        c_state_out_len = 1;
                        break 'main_loop;
                    }
                    *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                    bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                    cs_next_out = cs_next_out.add(1);
                    cs_avail_out = cs_avail_out.wrapping_sub(1);
                }
            }
            if c_nblock_used > s_save_nblockPP {
                return TRUE;
            }
            if c_nblock_used == s_save_nblockPP {
                c_state_out_len = 0;
                break 'main_loop;
            }
            c_state_out_ch = c_k0 as c_uchar;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            if k1 as c_int != c_k0 {
                c_k0 = k1 as c_int;
                if cs_avail_out == 0 {
                    c_state_out_len = 1;
                    break 'main_loop;
                }
                *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                cs_next_out = cs_next_out.add(1);
                cs_avail_out = cs_avail_out.wrapping_sub(1);
                continue;
            }
            if c_nblock_used == s_save_nblockPP {
                if cs_avail_out == 0 {
                    c_state_out_len = 1;
                    break 'main_loop;
                }
                *(cs_next_out as *mut c_uchar) = c_state_out_ch;
                bz_update_crc(&mut c_calculatedBlockCRC, c_state_out_ch);
                cs_next_out = cs_next_out.add(1);
                cs_avail_out = cs_avail_out.wrapping_sub(1);
                continue;
            }
            c_state_out_len = 2;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            if c_nblock_used == s_save_nblockPP {
                continue;
            }
            if k1 as c_int != c_k0 {
                c_k0 = k1 as c_int;
                continue;
            }
            c_state_out_len = 3;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            if c_nblock_used == s_save_nblockPP {
                continue;
            }
            if k1 as c_int != c_k0 {
                c_k0 = k1 as c_int;
                continue;
            }
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut k1) != FALSE {
                return TRUE;
            }
            c_nblock_used += 1;
            c_state_out_len = (k1 as c_int) + 4;
            let mut tmp_k0: c_uchar = 0;
            if bz_get_fast_c(&mut c_tPos, c_tt, ro_blockSize100k, &mut tmp_k0) != FALSE {
                return TRUE;
            }
            c_k0 = tmp_k0 as c_int;
            c_nblock_used += 1;
        }
        total_out_lo32_old = (*s_ref.strm).total_out_lo32;
        (*s_ref.strm).total_out_lo32 = (*s_ref.strm)
            .total_out_lo32
            .wrapping_add(avail_out_INIT.wrapping_sub(cs_avail_out));
        if (*s_ref.strm).total_out_lo32 < total_out_lo32_old {
            (*s_ref.strm).total_out_hi32 = (*s_ref.strm).total_out_hi32.wrapping_add(1);
        }
        s_ref.calculatedBlockCRC = c_calculatedBlockCRC;
        s_ref.state_out_ch = c_state_out_ch;
        s_ref.state_out_len = c_state_out_len;
        s_ref.nblock_used = c_nblock_used;
        s_ref.k0 = c_k0;
        s_ref.tt = c_tt;
        s_ref.tPos = c_tPos;
        (*s_ref.strm).next_out = cs_next_out;
        (*s_ref.strm).avail_out = cs_avail_out;
    }
    FALSE
}


// --- compress.c__150c159b/functions/BZ2_bsInitWrite.rs  [SACTOR-verified]
#[no_mangle]
pub unsafe fn BZ2_bsInitWrite(s: *mut EState) {
    if !s.is_null() {
        (*s).bsLive = 0;
        (*s).bsBuff = 0;
    }
}


// --- compress.c__150c159b/functions/bsFinishWrite.rs  [SACTOR-verified]
pub unsafe fn bsFinishWrite(s: *mut EState) {
    while (*s).bsLive > 0 {
        *(*s).zbits.add((*s).numZ as usize) = (((*s).bsBuff >> 24) & 0xFF) as ::core::ffi::c_uchar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8;
        (*s).bsLive -= 8;
    }
}


// --- compress.c__150c159b/functions/bsW.rs  [SACTOR-verified]
#[inline]
pub unsafe fn bsW(s: *mut EState, n: i32, v: u32) {
    while (*s).bsLive >= 8 {
        *(*s).zbits.add((*s).numZ as usize) = ((*s).bsBuff >> 24) as u8;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8;
        (*s).bsLive -= 8;
    }
    (*s).bsBuff |= v << (32 - (*s).bsLive - n);
    (*s).bsLive += n;
}


// --- compress.c__150c159b/functions/makeMaps_e.rs  [SACTOR-verified]
pub unsafe fn makeMaps_e(s: *mut EState) {
    if s.is_null() {
        return;
    }
    unsafe fn body(s: &mut EState) {
        let mut i: i32;
        s.nInUse = 0;
        i = 0;
        while i < 256 {
            if s.inUse[i as usize] != 0 {
                s.unseqToSeq[i as usize] = s.nInUse as u8;
                s.nInUse += 1;
            }
            i += 1;
        }
    }
    body(&mut *s);
}


// --- decompress.c__951eb532/functions/makeMaps_d.rs  [SACTOR-verified]
pub unsafe fn makeMaps_d(s: *mut DState) {
    if s.is_null() {
        return;
    }
    unsafe fn body(s: &mut DState) {
        s.nInUse = 0;
        let mut i: ::core::ffi::c_int = 0;
        while i < 256 {
            if s.inUse[i as usize] != 0 {
                s.seqToUnseq[s.nInUse as usize] = i as ::core::ffi::c_uchar;
                s.nInUse += 1;
            }
            i += 1;
        }
    }
    body(&mut *s);
}


// --- huffman.c__3211dfb8/functions/BZ2_hbAssignCodes.rs  [SACTOR-verified]
pub unsafe fn BZ2_hbAssignCodes(
    code: *mut libc::c_int,
    length: *mut libc::c_uchar,
    minLen: libc::c_int,
    maxLen: libc::c_int,
    alphaSize: libc::c_int,
) {
    let mut n: libc::c_int;
    let mut vec: libc::c_int;
    let mut i: libc::c_int;
    vec = 0;
    n = minLen;
    while n <= maxLen {
        i = 0;
        while i < alphaSize {
            if *length.add(i as usize) == n as libc::c_uchar {
                *code.add(i as usize) = vec;
                vec += 1;
            }
            i += 1;
        }
        vec <<= 1;
        n += 1;
    }
}


// --- huffman.c__3211dfb8/functions/BZ2_hbCreateDecodeTables.rs  [SACTOR-verified]
pub unsafe fn BZ2_hbCreateDecodeTables(
    limit: *mut libc::c_int,
    base: *mut libc::c_int,
    perm: *mut libc::c_int,
    length: *mut libc::c_uchar,
    minLen: libc::c_int,
    maxLen: libc::c_int,
    alphaSize: libc::c_int,
) {
    const BZ_MAX_CODE_LEN: libc::c_int = 23;
    let mut pp: libc::c_int = 0;
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut vec: libc::c_int;
    i = minLen;
    while i <= maxLen {
        j = 0;
        while j < alphaSize {
            if *length.add(j as usize) == i as libc::c_uchar {
                *perm.add(pp as usize) = j;
                pp += 1;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < BZ_MAX_CODE_LEN {
        *base.add(i as usize) = 0;
        i += 1;
    }
    i = 0;
    while i < alphaSize {
        let len_i = *length.add(i as usize) as libc::c_int;
        *base.add((len_i + 1) as usize) += 1;
        i += 1;
    }
    i = 1;
    while i < BZ_MAX_CODE_LEN {
        let prev = *base.add((i - 1) as usize);
        let cur_ptr = base.add(i as usize);
        *cur_ptr += prev;
        i += 1;
    }
    i = 0;
    while i < BZ_MAX_CODE_LEN {
        *limit.add(i as usize) = 0;
        i += 1;
    }
    vec = 0;
    i = minLen;
    while i <= maxLen {
        let bi1 = *base.add((i + 1) as usize);
        let bi = *base.add(i as usize);
        vec += bi1 - bi;
        *limit.add(i as usize) = vec - 1;
        vec <<= 1;
        i += 1;
    }
    i = minLen + 1;
    while i <= maxLen {
        let lim_prev = *limit.add((i - 1) as usize);
        let bi_ptr = base.add(i as usize);
        *bi_ptr = (((lim_prev + 1) << 1) - *bi_ptr);
        i += 1;
    }
}


// --- huffman.c__3211dfb8/functions/BZ2_hbMakeCodeLengths.rs  [SACTOR-verified]
pub unsafe fn BZ2_hbMakeCodeLengths(
    len: *mut libc::c_uchar,
    freq: *mut libc::c_int,
    alphaSize: libc::c_int,
    maxLen: libc::c_int,
) {
    #[inline(always)]
    fn weight_of(zz0: libc::c_int) -> libc::c_int {
        zz0 & (0xffffff00u32 as libc::c_int)
    }
    #[inline(always)]
    fn depth_of(zz1: libc::c_int) -> libc::c_int {
        zz1 & 0x000000ff
    }
    #[inline(always)]
    fn my_max(zz2: libc::c_int, zz3: libc::c_int) -> libc::c_int {
        if zz2 > zz3 {
            zz2
        } else {
            zz3
        }
    }
    #[inline(always)]
    fn add_weights(zw1: libc::c_int, zw2: libc::c_int) -> libc::c_int {
        (weight_of(zw1) + weight_of(zw2)) | (1 + my_max(depth_of(zw1), depth_of(zw2)))
    }
    #[inline(always)]
    unsafe fn assert_h(cond: bool, errcode: libc::c_int) {
        if !cond {
            BZ2_bz__AssertH__fail(errcode);
        }
    }
    const BZ_MAX_ALPHA_SIZE: usize = 258;
    let mut heap: [libc::c_int; BZ_MAX_ALPHA_SIZE + 2] = [0; BZ_MAX_ALPHA_SIZE + 2];
    let mut weight: [libc::c_int; BZ_MAX_ALPHA_SIZE * 2] = [0; BZ_MAX_ALPHA_SIZE * 2];
    let mut parent: [libc::c_int; BZ_MAX_ALPHA_SIZE * 2] = [0; BZ_MAX_ALPHA_SIZE * 2];
    fn upheap(z: libc::c_int, heap: &mut [libc::c_int], weight: &mut [libc::c_int]) {
        let mut zz: libc::c_int = z;
        let tmp: libc::c_int = heap[zz as usize];
        while weight[tmp as usize] < weight[heap[(zz >> 1) as usize] as usize] {
            heap[zz as usize] = heap[(zz >> 1) as usize];
            zz >>= 1;
        }
        heap[zz as usize] = tmp;
    }
    fn downheap(
        z: libc::c_int,
        heap: &mut [libc::c_int],
        weight: &mut [libc::c_int],
        n_heap: libc::c_int,
    ) {
        let mut zz: libc::c_int = z;
        let tmp: libc::c_int = heap[zz as usize];
        loop {
            let mut yy: libc::c_int = zz << 1;
            if yy > n_heap {
                break;
            }
            if yy < n_heap
                && weight[heap[(yy + 1) as usize] as usize] < weight[heap[yy as usize] as usize]
            {
                yy += 1;
            }
            if weight[tmp as usize] < weight[heap[yy as usize] as usize] {
                break;
            }
            heap[zz as usize] = heap[yy as usize];
            zz = yy;
        }
        heap[zz as usize] = tmp;
    }
    let mut n_nodes: libc::c_int;
    let mut n_heap: libc::c_int;
    let mut n1: libc::c_int;
    let mut n2: libc::c_int;
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let mut k: libc::c_int;
    let mut too_long: bool;
    i = 0;
    while i < alphaSize {
        let fi = *freq.add(i as usize);
        let v = if fi == 0 { 1 } else { fi };
        weight[(i + 1) as usize] = v << 8;
        i += 1;
    }
    loop {
        n_nodes = alphaSize;
        n_heap = 0;
        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;
        i = 1;
        while i <= alphaSize {
            parent[i as usize] = -1;
            n_heap += 1;
            heap[n_heap as usize] = i;
            upheap(n_heap, &mut heap, &mut weight);
            i += 1;
        }
        assert_h(n_heap < (BZ_MAX_ALPHA_SIZE as libc::c_int + 2), 2001);
        while n_heap > 1 {
            n1 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            downheap(1, &mut heap, &mut weight, n_heap);
            n2 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            downheap(1, &mut heap, &mut weight, n_heap);
            n_nodes += 1;
            parent[n1 as usize] = n_nodes;
            parent[n2 as usize] = n_nodes;
            weight[n_nodes as usize] = add_weights(weight[n1 as usize], weight[n2 as usize]);
            parent[n_nodes as usize] = -1;
            n_heap += 1;
            heap[n_heap as usize] = n_nodes;
            upheap(n_heap, &mut heap, &mut weight);
        }
        assert_h(n_nodes < (BZ_MAX_ALPHA_SIZE as libc::c_int * 2), 2002);
        too_long = false;
        i = 1;
        while i <= alphaSize {
            j = 0;
            k = i;
            while parent[k as usize] >= 0 {
                k = parent[k as usize];
                j += 1;
            }
            *len.add((i - 1) as usize) = j as libc::c_uchar;
            if j > maxLen {
                too_long = true;
            }
            i += 1;
        }
        if !too_long {
            break;
        }
        i = 1;
        while i <= alphaSize {
            j = weight[i as usize] >> 8;
            j = 1 + (j / 2);
            weight[i as usize] = j << 8;
            i += 1;
        }
    }
}


// --- run2_extracted_rust/BZ2_bzCompressInit__attempt6.rs  [UNVERIFIED last attempt 6: Rust code failed to compile]
// SACTOR unidiomatic translation of `BZ2_bzCompressInit` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:42:32; attempt 6). Verification verdict: Rust code failed to compile
pub unsafe fn BZ2_bzCompressInit(
    strm: *mut bz_stream,
    blockSize100k: libc::c_int,
    verbosity: libc::c_int,
    mut workFactor: libc::c_int,
) -> libc::c_int {
    use libc::{c_int, c_uint, c_uchar, c_ushort, c_void};
    type Int32 = c_int;
    type UInt32 = c_uint;
    type UChar = c_uchar;
    const BZ_CONFIG_ERROR: c_int = -9;
    const BZ_MEM_ERROR: c_int = -3;
    const BZ_PARAM_ERROR: c_int = -2;
    const BZ_OK: c_int = 0;
    const BZ_S_INPUT: c_int = 2;
    const BZ_M_RUNNING: c_int = 2;
    const BZ_N_RADIX: c_int = 2;
    const BZ_N_QSORT: c_int = 12;
    const BZ_N_SHELL: c_int = 18;
    const BZ_N_OVERSHOOT: c_int = BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2;
    unsafe fn call_bzalloc(strm: *mut bz_stream, nbytes: c_int) -> *mut c_void {
        if strm.is_null() {
            return core::ptr::null_mut();
        }
        let s = &mut *strm;
        match s.bzalloc {
            Some(f) => f(s.opaque, nbytes, 1),
            None => core::ptr::null_mut(),
        }
    }
    unsafe fn call_bzfree(strm: *mut bz_stream, p: *mut c_void) {
        if strm.is_null() || p.is_null() {
            return;
        }
        let s = &mut *strm;
        if let Some(f) = s.bzfree {
            f(s.opaque, p);
        }
    }
    let n: Int32;
    let s_ptr: *mut EState;
    if bz_config_ok() == 0 {
        return BZ_CONFIG_ERROR;
    }
    if strm.is_null() || blockSize100k < 1 || blockSize100k > 9 || workFactor < 0
        || workFactor > 250
    {
        return BZ_PARAM_ERROR;
    }
    if workFactor == 0 {
        workFactor = 30;
    }
    {
        let strm_ref = &mut *strm;
        if strm_ref.bzalloc.is_none() {
            strm_ref.bzalloc = Some(default_bzalloc);
        }
        if strm_ref.bzfree.is_none() {
            strm_ref.bzfree = Some(default_bzfree);
        }
    }
    s_ptr = call_bzalloc(strm, core::mem::size_of::<EState>() as c_int) as *mut EState;
    if s_ptr.is_null() {
        return BZ_MEM_ERROR;
    }
    (*s_ptr).strm = strm;
    (*s_ptr).arr1 = core::ptr::null_mut();
    (*s_ptr).arr2 = core::ptr::null_mut();
    (*s_ptr).ftab = core::ptr::null_mut();
    n = 100000 * blockSize100k;
    (*s_ptr).arr1 = call_bzalloc(
        strm,
        (n as usize * core::mem::size_of::<UInt32>()) as c_int,
    ) as *mut UInt32;
    (*s_ptr).arr2 = call_bzalloc(
        strm,
        ((n + BZ_N_OVERSHOOT) as usize * core::mem::size_of::<UInt32>()) as c_int,
    ) as *mut UInt32;
    (*s_ptr).ftab = call_bzalloc(
        strm,
        (65537usize * core::mem::size_of::<UInt32>()) as c_int,
    ) as *mut UInt32;
    if (*s_ptr).arr1.is_null() || (*s_ptr).arr2.is_null() || (*s_ptr).ftab.is_null() {
        if !(*s_ptr).arr1.is_null() {
            call_bzfree(strm, (*s_ptr).arr1 as *mut c_void);
        }
        if !(*s_ptr).arr2.is_null() {
            call_bzfree(strm, (*s_ptr).arr2 as *mut c_void);
        }
        if !(*s_ptr).ftab.is_null() {
            call_bzfree(strm, (*s_ptr).ftab as *mut c_void);
        }
        call_bzfree(strm, s_ptr as *mut c_void);
        return BZ_MEM_ERROR;
    }
    (*s_ptr).blockNo = 0;
    (*s_ptr).state = BZ_S_INPUT;
    (*s_ptr).mode = BZ_M_RUNNING;
    (*s_ptr).combinedCRC = 0;
    (*s_ptr).blockSize100k = blockSize100k;
    (*s_ptr).nblockMAX = 100000 * blockSize100k - 19;
    (*s_ptr).verbosity = verbosity;
    (*s_ptr).workFactor = workFactor;
    (*s_ptr).block = (*s_ptr).arr2 as *mut UChar;
    (*s_ptr).mtfv = (*s_ptr).arr1 as *mut c_ushort;
    (*s_ptr).zbits = core::ptr::null_mut();
    (*s_ptr).ptr = (*s_ptr).arr1 as *mut UInt32;
    (*strm).state = s_ptr as *mut c_void;
    (*strm).total_in_lo32 = 0;
    (*strm).total_in_hi32 = 0;
    (*strm).total_out_lo32 = 0;
    (*strm).total_out_hi32 = 0;
    init_RL(s_ptr);
    prepare_new_block(s_ptr);
    BZ_OK
}


// --- run2_extracted_rust/BZ2_bzReadGetUnused__attempt6.rs  [UNVERIFIED last attempt 6: Rust code failed to compile]
// SACTOR unidiomatic translation of `BZ2_bzReadGetUnused` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:39:43; attempt 6). Verification verdict: Rust code failed to compile
use libc::{c_int, c_void};
pub type BZFILE = c_void;
pub unsafe extern "C" fn BZ2_bzReadGetUnused(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    unused: *mut *mut c_void,
    nUnused: *mut c_int,
) {
    type BzFileAlias = crate::bzFile;
    unsafe fn bz_seterr(bzerror: *mut c_int, bzf: *mut BzFileAlias, eee: c_int) {
        if !bzerror.is_null() {
            *bzerror = eee;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = eee;
        }
    }
    const BZ_STREAM_END: c_int = 4;
    const BZ_SEQUENCE_ERROR: c_int = -1;
    const BZ_OK: c_int = 0;
    const BZ_PARAM_ERROR: c_int = -2;
    let bzf = b as *mut BzFileAlias;
    if bzf.is_null() {
        bz_seterr(bzerror, bzf, BZ_PARAM_ERROR);
        return;
    }
    if (*bzf).lastErr != BZ_STREAM_END {
        bz_seterr(bzerror, bzf, BZ_SEQUENCE_ERROR);
        return;
    }
    if unused.is_null() || nUnused.is_null() {
        bz_seterr(bzerror, bzf, BZ_PARAM_ERROR);
        return;
    }
    bz_seterr(bzerror, bzf, BZ_OK);
    *nUnused = (*bzf).strm.avail_in as c_int;
    *unused = (*bzf).strm.next_in as *mut c_void;
}


// --- run2_extracted_rust/BZ2_bzerror__attempt6.rs  [UNVERIFIED last attempt 6: Rust code failed to compile]
// SACTOR unidiomatic translation of `BZ2_bzerror` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:41:01; attempt 6). Verification verdict: Rust code failed to compile
pub unsafe extern "C" fn BZ2_bzerror(
    b: *mut bzFile,
    errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    extern "C" {
        static bzerrorstrings: [*const ::core::ffi::c_char; 16];
    }
    let mut err = (*b).lastErr;
    if err > 0 {
        err = 0;
    }
    *errnum = err;
    let idx = (-err) as usize;
    bzerrorstrings[idx]
}


// --- run2_extracted_rust/bsPutUInt32.rs  [UNVERIFIED last attempt 1: Unidiomatic translation failed for /tmp/claude-1000/-home-yu]
// SACTOR unidiomatic translation of `bsPutUInt32` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:43:52; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_bzip2/compress.c: Error: Failed to link project-level harness for
pub unsafe fn bsPutUInt32(s: *mut EState, u: u32) {
    bsW(s, 8, (u >> 24) & 0xff);
    bsW(s, 8, (u >> 16) & 0xff);
    bsW(s, 8, (u >> 8) & 0xff);
    bsW(s, 8, u & 0xff);
}


// --- run2_extracted_rust/flush_RL.rs  [UNVERIFIED last attempt 1: Unidiomatic translation failed for /tmp/claude-1000/-home-yu]
// SACTOR unidiomatic translation of `flush_RL` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:42:34; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_bzip2/bzlib.c: Error: Failed to link project-level harness for fu
pub unsafe fn flush_RL(s: *mut EState) {
    if !s.is_null() && (*s).state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_RL(s);
}

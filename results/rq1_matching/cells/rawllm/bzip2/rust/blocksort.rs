#![allow(clippy::needless_range_loop)]

// Safe, idiomatic Rust translation of blocksort.c's functionality.
// This is a self-contained library that re-implements the block-sorting
// machinery used in bzip2. The public API is a minimal, Rust-friendly
// wrapper around the original C entry point `BZ2_blockSort`.

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UChar = u8;
pub type Bool = bool;

const TRUE: Bool = true;
const FALSE: Bool = false;

const BZ_N_RADIX: Int32 = 2;
const BZ_N_QSORT: Int32 = 12;
const BZ_N_SHELL: Int32 = 18;
const BZ_N_OVERSHOOT: Int32 = BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2; // 34

/// Rust representation of the EState fields used by block sorting.
/// This corresponds to the relevant subset of the C `EState` struct.
#[derive(Debug)]
pub struct BlockSortState<'a> {
    /// Alias for `arr1` in C. For `fallbackSort` this is the destination fmap.
    pub arr1: &'a mut [UInt32],
    /// Alias for `arr2` in C. For `fallbackSort` this is the temporary eclass
    /// (same length as `arr1`, but viewed as bytes).
    pub arr2: &'a mut [UChar],
    /// Frequency / bucket table. Must have length at least 65537 for main sort,
    /// and enough for `2 + (nblock/32)` for fallback sort.
    pub ftab: &'a mut [UInt32],
    /// Number of bytes in the block.
    pub nblock: Int32,
    /// Work factor (1..100), as in bzip2.
    pub work_factor: Int32,
    /// Verbosity (only used for diagnostic printing in the original C;
    /// ignored here).
    pub verbosity: Int32,
}

fn assert_h(cond: bool, _errcode: Int32) {
    if !cond {
        panic!("internal blocksort assertion failed (code {})", _errcode);
    }
}

fn assert_d(cond: bool, _msg: &str) {
    if !cond {
        panic!("internal blocksort debug assertion failed: {}", _msg);
    }
}

// --------------------------
// Fallback O(N log(N)^2) sort
// --------------------------

fn fallback_simple_sort(fmap: &mut [UInt32], eclass: &[UInt32], lo: Int32, hi: Int32) {
    let mut i: Int32;
    let mut j: Int32;
    let mut tmp: Int32;
    let mut ec_tmp: UInt32;

    if lo == hi {
        return;
    }

    let hi_usize = hi as usize;

    if hi - lo > 3 {
        i = hi - 4;
        while i >= lo {
            tmp = fmap[i as usize] as Int32;
            ec_tmp = eclass[tmp as usize];
            j = i + 4;
            while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
                let j_us = j as usize;
                fmap[j_us - 4] = fmap[j_us];
                j += 4;
            }
            fmap[(j - 4) as usize] = tmp as UInt32;
            if i == lo { break; }
            i -= 1;
        }
    }

    i = hi - 1;
    while i >= lo {
        tmp = fmap[i as usize] as Int32;
        ec_tmp = eclass[tmp as usize];
        j = i + 1;
        while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
            let j_us = j as usize;
            fmap[j_us - 1] = fmap[j_us];
            j += 1;
        }
        fmap[(j - 1) as usize] = tmp as UInt32;
        if i == lo { break; }
        i -= 1;
    }
}

fn fallback_qsort3(fmap: &mut [UInt32], eclass: &[UInt32], lo_st: Int32, hi_st: Int32) {
    const FALLBACK_QSORT_SMALL_THRESH: Int32 = 10;
    const FALLBACK_QSORT_STACK_SIZE: usize = 100;

    let mut stack_lo = [0i32; FALLBACK_QSORT_STACK_SIZE];
    let mut stack_hi = [0i32; FALLBACK_QSORT_STACK_SIZE];

    let mut r: UInt32 = 0;
    let mut sp: usize = 0;

    // fpush
    stack_lo[sp] = lo_st;
    stack_hi[sp] = hi_st;
    sp += 1;

    while sp > 0 {
        assert_h(sp < FALLBACK_QSORT_STACK_SIZE - 1, 1004);

        // fpop
        sp -= 1;
        let mut lo = stack_lo[sp];
        let mut hi = stack_hi[sp];

        if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
            fallback_simple_sort(fmap, eclass, lo, hi);
            continue;
        }

        // Random partitioning
        r = ((r.wrapping_mul(7621)).wrapping_add(1)) % 32768;
        let r3 = (r % 3) as i32;
        let med: UInt32 = if r3 == 0 {
            eclass[fmap[lo as usize] as usize]
        } else if r3 == 1 {
            eclass[fmap[((lo + hi) >> 1) as usize] as usize]
        } else {
            eclass[fmap[hi as usize] as usize]
        };

        let mut un_lo = lo;
        let mut un_hi = hi;
        let mut lt_lo = lo;
        let mut gt_hi = hi;

        loop {
            loop {
                if un_lo > un_hi {
                    break;
                }
                let n = eclass[fmap[un_lo as usize] as usize] as Int32 - med as Int32;
                if n == 0 {
                    fmap.swap(un_lo as usize, lt_lo as usize);
                    lt_lo += 1;
                    un_lo += 1;
                    continue;
                }
                if n > 0 {
                    break;
                }
                un_lo += 1;
            }
            loop {
                if un_lo > un_hi {
                    break;
                }
                let n = eclass[fmap[un_hi as usize] as usize] as Int32 - med as Int32;
                if n == 0 {
                    fmap.swap(un_hi as usize, gt_hi as usize);
                    gt_hi -= 1;
                    un_hi -= 1;
                    continue;
                }
                if n < 0 {
                    break;
                }
                un_hi -= 1;
            }
            if un_lo > un_hi {
                break;
            }
            fmap.swap(un_lo as usize, un_hi as usize);
            un_lo += 1;
            un_hi -= 1;
        }

        assert_d(un_hi == un_lo - 1, "fallbackQSort3(2)");

        if gt_hi < lt_lo {
            continue;
        }

        let mut n = (lt_lo - lo).min(un_lo - lt_lo);
        {
            let mut yyp1 = lo;
            let mut yyp2 = un_lo - n;
            let mut yyn = n;
            while yyn > 0 {
                fmap.swap(yyp1 as usize, yyp2 as usize);
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1;
            }
        }

        let mut m = (hi - gt_hi).min(gt_hi - un_hi);
        {
            let mut yyp1 = un_lo;
            let mut yyp2 = hi - m + 1;
            let mut yyn = m;
            while yyn > 0 {
                fmap.swap(yyp1 as usize, yyp2 as usize);
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1;
            }
        }

        n = lo + un_lo - lt_lo - 1;
        m = hi - (gt_hi - un_hi) + 1;

        if n - lo > hi - m {
            stack_lo[sp] = lo;
            stack_hi[sp] = n;
            sp += 1;
            stack_lo[sp] = m;
            stack_hi[sp] = hi;
            sp += 1;
        } else {
            stack_lo[sp] = m;
            stack_hi[sp] = hi;
            sp += 1;
            stack_lo[sp] = lo;
            stack_hi[sp] = n;
            sp += 1;
        }
    }
}

fn fallback_sort(
    fmap: &mut [UInt32],
    eclass_buf: &mut [UInt32],
    bhtab: &mut [UInt32],
    nblock: Int32,
    _verb: Int32,
) {
    let nblock_usize = nblock as usize;
    let mut ftab = [0i32; 257];
    let mut ftab_copy = [0i32; 256];
    let mut h: Int32;
    let mut i: Int32;
    let mut j: Int32;
    let mut k: Int32;
    let mut r: Int32;
    let mut cc: Int32;
    let mut cc1: Int32;
    let mut n_not_done: Int32;
    let mut n_bhtab: Int32;

    // Treat eclass as bytes
    let eclass8: &mut [UChar] = unsafe {
        // eclass_buf length is nblock; reinterpret as bytes of same length
        core::slice::from_raw_parts_mut(eclass_buf.as_mut_ptr() as *mut UChar, nblock_usize)
    };

    // Initial 1-char radix sort
    for x in ftab.iter_mut() {
        *x = 0;
    }
    for &b in &*eclass8 {
        ftab[b as usize] += 1;
    }
    for i in 0..256 {
        ftab_copy[i] = ftab[i];
    }
    for i in 1..257 {
        let prev = ftab[i - 1];
        ftab[i] += prev;
    }

    for (idx, &b) in eclass8.iter().enumerate() {
        j = b as Int32;
        k = ftab[j as usize] - 1;
        ftab[j as usize] = k;
        fmap[k as usize] = idx as UInt32;
    }

    n_bhtab = 2 + (nblock / 32);
    for i in 0..(n_bhtab as usize) {
        bhtab[i] = 0;
    }
    for i in 0..256 {
        let zz = ftab[i] as UInt32;
        let word_idx = (zz >> 5) as usize;
        let bit = zz & 31;
        bhtab[word_idx] |= 1u32 << bit;
    }

    // set sentinel bits for block-end detection
    for i in 0..32 {
        let zz = (nblock + 2 * i) as UInt32;
        let word_idx = (zz >> 5) as usize;
        let bit = zz & 31;
        bhtab[word_idx] |= 1u32 << bit;
        let zz1 = (nblock + 2 * i + 1) as UInt32;
        let word_idx1 = (zz1 >> 5) as usize;
        let bit1 = zz1 & 31;
        bhtab[word_idx1] &= !(1u32 << bit1);
    }

    h = 1;
    loop {
        // j = 0; but j is only used for assignment from i when bit set
        let mut j_last = 0i32;
        for i32_i in 0..nblock {
            let i = i32_i;
            let zz = i as UInt32;
            let word_idx = (zz >> 5) as usize;
            let bit = zz & 31;
            if (bhtab[word_idx] & (1u32 << bit)) != 0 {
                j_last = i;
            }
            let mut k32 = fmap[i as usize] as Int32 - h;
            if k32 < 0 {
                k32 += nblock;
            }
            eclass_buf[k32 as usize] = j_last as UInt32;
        }

        n_not_done = 0;
        r = -1;
        loop {
            // find next non-singleton bucket
            k = r + 1;
            // while ISSET_BH(k) && UNALIGNED_BH(k)
            while {
                let zz = k as UInt32;
                let word_idx = (zz >> 5) as usize;
                let bit = zz & 31;
                (bhtab[word_idx] & (1u32 << bit)) != 0 && (k & 0x1f) != 0
            } {
                k += 1;
            }
            // if ISSET_BH(k)
            if {
                let zz = k as UInt32;
                let word_idx = (zz >> 5) as usize;
                let bit = zz & 31;
                (bhtab[word_idx] & (1u32 << bit)) != 0
            } {
                while {
                    let word = bhtab[(k as UInt32 >> 5) as usize];
                    word == 0xffffffff
                } {
                    k += 32;
                }
                while {
                    let zz = k as UInt32;
                    let word_idx = (zz >> 5) as usize;
                    let bit = zz & 31;
                    (bhtab[word_idx] & (1u32 << bit)) != 0
                } {
                    k += 1;
                }
            }
            let mut l = k - 1;
            if l >= nblock {
                break;
            }
            while {
                let zz = k as UInt32;
                let word_idx = (zz >> 5) as usize;
                let bit = zz & 31;
                (bhtab[word_idx] & (1u32 << bit)) != 0 && (k & 0x1f) != 0
            } {
                k += 1;
            }
            if {
                let zz = k as UInt32;
                let word_idx = (zz >> 5) as usize;
                let bit = zz & 31;
                (bhtab[word_idx] & (1u32 << bit)) == 0
            } {
                while {
                    let word = bhtab[(k as UInt32 >> 5) as usize];
                    word == 0x00000000
                } {
                    k += 32;
                }
                while {
                    let zz = k as UInt32;
                    let word_idx = (zz >> 5) as usize;
                    let bit = zz & 31;
                    (bhtab[word_idx] & (1u32 << bit)) == 0
                } {
                    k += 1;
                }
            }
            r = k - 1;
            if r >= nblock {
                break;
            }

            if r > l {
                n_not_done += r - l + 1;
                fallback_qsort3(fmap, eclass_buf, l, r);

                // scan bucket and generate header bits
                cc = -1;
                for i2 in l..=r {
                    cc1 = eclass_buf[fmap[i2 as usize] as usize] as Int32;
                    if cc != cc1 {
                        let zz = i2 as UInt32;
                        let word_idx = (zz >> 5) as usize;
                        let bit = zz & 31;
                        bhtab[word_idx] |= 1u32 << bit;
                        cc = cc1;
                    }
                }
            }
        }

        h *= 2;
        if h > nblock || n_not_done == 0 {
            break;
        }
    }

    // Reconstruct original block into eclass8
    let mut j_idx = 0usize;
    for i in 0..nblock_usize {
        while ftab_copy[j_idx] == 0 {
            j_idx += 1;
        }
        ftab_copy[j_idx] -= 1;
        eclass8[fmap[i] as usize] = j_idx as UChar;
    }
    assert_h(j_idx < 256, 1005);
}

// --------------------------
// Main O(N^2 log N) sorter
// --------------------------

fn main_gt_u(
    mut i1: UInt32,
    mut i2: UInt32,
    block: &[UChar],
    quadrant: &[UInt16],
    nblock: UInt32,
    budget: &mut Int32,
) -> Bool {
    assert_d(i1 != i2, "mainGtU");

    // Inline first 12 byte comparisons
    for _ in 0..12 {
        let c1 = block[i1 as usize];
        let c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        i1 += 1;
        i2 += 1;
    }

    let mut k: Int32 = nblock as Int32 + 8;

    while k >= 0 {
        let c1 = block[i1 as usize];
        let c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        let s1 = quadrant[i1 as usize];
        let s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        let c1 = block[i1 as usize];
        let c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        let s1 = quadrant[i1 as usize];
        let s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        let c1 = block[i1 as usize];
        let c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        let s1 = quadrant[i1 as usize];
        let s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        let c1 = block[i1 as usize];
        let c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        let s1 = quadrant[i1 as usize];
        let s2 = quadrant[i2 as usize];
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
        if *budget < 0 {
            break;
        }
    }

    FALSE
}

// Knuth increments for shell sort
static INCS: [Int32; 14] = [
    1,
    4,
    13,
    40,
    121,
    364,
    1093,
    3280,
    9841,
    29524,
    88573,
    265720,
    797161,
    2391484,
];

fn main_simple_sort(
    ptr: &mut [UInt32],
    block: &[UChar],
    quadrant: &[UInt16],
    nblock: Int32,
    lo: Int32,
    hi: Int32,
    d: Int32,
    budget: &mut Int32,
) {
    let big_n = hi - lo + 1;
    if big_n < 2 {
        return;
    }

    let mut hp = 0usize;
    while INCS[hp] < big_n {
        hp += 1;
        if hp == INCS.len() {
            break;
        }
    }
    if hp > 0 {
        hp -= 1;
    }

    while hp < INCS.len() {
        let h = INCS[hp];
        let mut i = lo + h;
        loop {
            if i > hi {
                break;
            }
            let mut v = ptr[i as usize];
            let mut j = i;
            while main_gt_u(
                ptr[(j - h) as usize] + d as UInt32,
                v + d as UInt32,
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j -= h;
                if j <= lo + h - 1 {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;
            while main_gt_u(
                ptr[(j - h) as usize] + d as UInt32,
                v + d as UInt32,
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j -= h;
                if j <= lo + h - 1 {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;
            while main_gt_u(
                ptr[(j - h) as usize] + d as UInt32,
                v + d as UInt32,
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j -= h;
                if j <= lo + h - 1 {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if *budget < 0 {
                return;
            }
        }

        if hp == 0 {
            break;
        }
        hp -= 1;
    }
}

fn mmed3(a: UChar, b: UChar, c: UChar) -> UChar {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    if a > b {
        core::mem::swap(&mut a, &mut b);
    }
    if b > c {
        b = c;
        if a > b {
            b = a;
        }
    }
    b
}

fn main_qsort3(
    ptr: &mut [UInt32],
    block: &[UChar],
    quadrant: &[UInt16],
    nblock: Int32,
    lo_st: Int32,
    hi_st: Int32,
    d_st: Int32,
    budget: &mut Int32,
) {
    const MAIN_QSORT_SMALL_THRESH: Int32 = 20;
    const MAIN_QSORT_DEPTH_THRESH: Int32 = (BZ_N_RADIX + BZ_N_QSORT) as Int32;
    const MAIN_QSORT_STACK_SIZE: usize = 100;

    let mut stack_lo = [0i32; MAIN_QSORT_STACK_SIZE];
    let mut stack_hi = [0i32; MAIN_QSORT_STACK_SIZE];
    let mut stack_d = [0i32; MAIN_QSORT_STACK_SIZE];

    let mut next_lo = [0i32; 3];
    let mut next_hi = [0i32; 3];
    let mut next_d = [0i32; 3];

    let mut sp: usize = 0;
    stack_lo[sp] = lo_st;
    stack_hi[sp] = hi_st;
    stack_d[sp] = d_st;
    sp += 1;

    while sp > 0 {
        assert_h(sp < MAIN_QSORT_STACK_SIZE - 2, 1001);

        sp -= 1;
        let mut lo = stack_lo[sp];
        let mut hi = stack_hi[sp];
        let mut d = stack_d[sp];

        if hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH {
            main_simple_sort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 {
                return;
            }
            continue;
        }

        let med = mmed3(
            block[(ptr[lo as usize] + d as UInt32) as usize],
            block[(ptr[hi as usize] + d as UInt32) as usize],
            block[(ptr[((lo + hi) >> 1) as usize] + d as UInt32) as usize],
        ) as Int32;

        let mut un_lo = lo;
        let mut un_hi = hi;
        let mut lt_lo = lo;
        let mut gt_hi = hi;

        loop {
            loop {
                if un_lo > un_hi {
                    break;
                }
                let n = block[(ptr[un_lo as usize] + d as UInt32) as usize] as Int32 - med;
                if n == 0 {
                    ptr.swap(un_lo as usize, lt_lo as usize);
                    lt_lo += 1;
                    un_lo += 1;
                    continue;
                }
                if n > 0 {
                    break;
                }
                un_lo += 1;
            }
            loop {
                if un_lo > un_hi {
                    break;
                }
                let n = block[(ptr[un_hi as usize] + d as UInt32) as usize] as Int32 - med;
                if n == 0 {
                    ptr.swap(un_hi as usize, gt_hi as usize);
                    gt_hi -= 1;
                    un_hi -= 1;
                    continue;
                }
                if n < 0 {
                    break;
                }
                un_hi -= 1;
            }
            if un_lo > un_hi {
                break;
            }
            ptr.swap(un_lo as usize, un_hi as usize);
            un_lo += 1;
            un_hi -= 1;
        }

        assert_d(un_hi == un_lo - 1, "mainQSort3(2)");

        if gt_hi < lt_lo {
            stack_lo[sp] = lo;
            stack_hi[sp] = hi;
            stack_d[sp] = d + 1;
            sp += 1;
            continue;
        }

        let mut n = (lt_lo - lo).min(un_lo - lt_lo);
        {
            let mut yyp1 = lo;
            let mut yyp2 = un_lo - n;
            let mut yyn = n;
            while yyn > 0 {
                ptr.swap(yyp1 as usize, yyp2 as usize);
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1;
            }
        }
        let mut m = (hi - gt_hi).min(gt_hi - un_hi);
        {
            let mut yyp1 = un_lo;
            let mut yyp2 = hi - m + 1;
            let mut yyn = m;
            while yyn > 0 {
                ptr.swap(yyp1 as usize, yyp2 as usize);
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1;
            }
        }

        n = lo + un_lo - lt_lo - 1;
        m = hi - (gt_hi - un_hi) + 1;

        next_lo[0] = lo;
        next_hi[0] = n;
        next_d[0] = d;
        next_lo[1] = m;
        next_hi[1] = hi;
        next_d[1] = d;
        next_lo[2] = n + 1;
        next_hi[2] = m - 1;
        next_d[2] = d + 1;

        let mnextsize = |az: usize, next_lo: &[Int32], next_hi: &[Int32]| -> Int32 {
            next_hi[az] - next_lo[az]
        };

        let mut mnextswap = |az: usize, bz: usize, next_lo: &mut [Int32], next_hi: &mut [Int32], next_d: &mut [Int32]| {
            let tz = next_lo[az];
            next_lo[az] = next_lo[bz];
            next_lo[bz] = tz;
            let tz = next_hi[az];
            next_hi[az] = next_hi[bz];
            next_hi[bz] = tz;
            let tz = next_d[az];
            next_d[az] = next_d[bz];
            next_d[bz] = tz;
        };

        if mnextsize(0, &next_lo, &next_hi) < mnextsize(1, &next_lo, &next_hi) {
            mnextswap(0, 1, &mut next_lo, &mut next_hi, &mut next_d);
        }
        if mnextsize(1, &next_lo, &next_hi) < mnextsize(2, &next_lo, &next_hi) {
            mnextswap(1, 2, &mut next_lo, &mut next_hi, &mut next_d);
        }
        if mnextsize(0, &next_lo, &next_hi) < mnextsize(1, &next_lo, &next_hi) {
            mnextswap(0, 1, &mut next_lo, &mut next_hi, &mut next_d);
        }

        assert_d(
            mnextsize(0, &next_lo, &next_hi) >= mnextsize(1, &next_lo, &next_hi),
            "mainQSort3(8)",
        );
        assert_d(
            mnextsize(1, &next_lo, &next_hi) >= mnextsize(2, &next_lo, &next_hi),
            "mainQSort3(9)",
        );

        for z in 0..3 {
            stack_lo[sp] = next_lo[z];
            stack_hi[sp] = next_hi[z];
            stack_d[sp] = next_d[z];
            sp += 1;
        }
    }
}

fn main_sort(
    ptr: &mut [UInt32],
    block: &mut [UChar],
    quadrant: &mut [UInt16],
    ftab: &mut [UInt32],
    nblock: Int32,
    _verb: Int32,
    budget: &mut Int32,
) {
    let nblock_usize = nblock as usize;

    let mut running_order = [0i32; 256];
    let mut big_done = [FALSE; 256];
    let mut copy_start = [0i32; 256];
    let mut copy_end = [0i32; 256];

    // set up 2-byte frequency table
    for v in ftab.iter_mut().take(65537) {
        *v = 0;
    }

    let mut j: UInt16 = (block[0] as UInt16) << 8;
    let mut i = nblock - 1;

    while i >= 3 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as UInt16) << 8);
        ftab[j as usize] += 1;
        quadrant[(i - 1) as usize] = 0;
        j = (j >> 8) | ((block[(i - 1) as usize] as UInt16) << 8);
        ftab[j as usize] += 1;
        quadrant[(i - 2) as usize] = 0;
        j = (j >> 8) | ((block[(i - 2) as usize] as UInt16) << 8);
        ftab[j as usize] += 1;
        quadrant[(i - 3) as usize] = 0;
        j = (j >> 8) | ((block[(i - 3) as usize] as UInt16) << 8);
        ftab[j as usize] += 1;
        i -= 4;
    }
    while i >= 0 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as UInt16) << 8);
        ftab[j as usize] += 1;
        if i == 0 {
            break;
        }
        i -= 1;
    }

    // overshoot
    for i in 0..BZ_N_OVERSHOOT {
        let idx = nblock_usize + i as usize;
        block[idx] = block[i as usize];
        quadrant[idx] = 0;
    }

    // Complete initial radix sort
    for i in 1..=65536 {
        let prev = ftab[i - 1];
        ftab[i] = ftab[i].wrapping_add(prev);
    }

    let mut s: UInt16 = (block[0] as UInt16) << 8;
    let mut i32i = nblock - 1;
    while i32i >= 3 {
        s = (s >> 8) | ((block[i32i as usize] as UInt16) << 8);
        j = ftab[s as usize] as UInt16 - 1;
        ftab[s as usize] = j as UInt32;
        ptr[j as usize] = i32i as UInt32;
        s = (s >> 8) | ((block[(i32i - 1) as usize] as UInt16) << 8);
        j = ftab[s as usize] as UInt16 - 1;
        ftab[s as usize] = j as UInt32;
        ptr[j as usize] = (i32i - 1) as UInt32;
        s = (s >> 8) | ((block[(i32i - 2) as usize] as UInt16) << 8);
        j = ftab[s as usize] as UInt16 - 1;
        ftab[s as usize] = j as UInt32;
        ptr[j as usize] = (i32i - 2) as UInt32;
        s = (s >> 8) | ((block[(i32i - 3) as usize] as UInt16) << 8);
        j = ftab[s as usize] as UInt16 - 1;
        ftab[s as usize] = j as UInt32;
        ptr[j as usize] = (i32i - 3) as UInt32;
        i32i -= 4;
    }
    while i32i >= 0 {
        s = (s >> 8) | ((block[i32i as usize] as UInt16) << 8);
        j = ftab[s as usize] as UInt16 - 1;
        ftab[s as usize] = j as UInt32;
        ptr[j as usize] = i32i as UInt32;
        if i32i == 0 {
            break;
        }
        i32i -= 1;
    }

    // running order
    for i in 0..=255 {
        big_done[i] = FALSE;
        running_order[i] = i as Int32;
    }

    {
        let mut h = 1i32;
        while h <= 256 {
            h = 3 * h + 1;
        }
        loop {
            h /= 3;
            for i in h..=255 {
                let vv = running_order[i];
                let mut j = i;
                while {
                    let lhs_b = running_order[(j - h) as usize] as UInt32;
                    let rhs_b = vv as UInt32;
                    let bigfreq = |b: UInt32, ftab: &[UInt32]| -> Int32 {
                        let b_us = b as usize;
                        (ftab[((b_us + 1) << 8)] - ftab[b_us << 8]) as Int32
                    };
                    bigfreq(lhs_b, ftab) > bigfreq(rhs_b, ftab)
                } {
                    running_order[j] = running_order[(j - h) as usize];
                    j -= h;
                    if j <= h - 1 {
                        break;
                    }
                }
                running_order[j] = vv;
            }
            if h == 1 {
                break;
            }
        }
    }

    let mut num_qsorted = 0i32;

    for idx in 0..=255 {
        let ss = running_order[idx] as usize;

        // Step 1: complete big bucket [ss]
        for j_idx in 0..=255 {
            if j_idx != ss {
                let sb = ((ss << 8) + j_idx) as usize;
                const SETMASK: UInt32 = 1 << 21;
                const CLEARMASK: UInt32 = !SETMASK;

                if (ftab[sb] & SETMASK) == 0 {
                    let lo = (ftab[sb] & CLEARMASK) as Int32;
                    let hi = ((ftab[sb + 1] & CLEARMASK) as Int32) - 1;
                    if hi > lo {
                        main_qsort3(
                            ptr,
                            block,
                            quadrant,
                            nblock,
                            lo,
                            hi,
                            BZ_N_RADIX,
                            budget,
                        );
                        num_qsorted += hi - lo + 1;
                        if *budget < 0 {
                            return;
                        }
                    }
                }
                ftab[sb] |= SETMASK;
            }
        }

        assert_h(!big_done[ss], 1006);

        // Step 2: scan big bucket [ss]
        const SETMASK: UInt32 = 1 << 21;
        const CLEARMASK: UInt32 = !SETMASK;

        for j in 0..=255 {
            let idx = ((j << 8) + ss) as usize;
            copy_start[j] = (ftab[idx] & CLEARMASK) as Int32;
            copy_end[j] = ((ftab[idx + 1] & CLEARMASK) as Int32) - 1;
        }

        let start = (ftab[ss << 8] & CLEARMASK) as Int32;
        let mut j = start;
        while j < copy_start[ss] {
            let mut k = ptr[j as usize] as Int32 - 1;
            if k < 0 {
                k += nblock;
            }
            let c1 = block[k as usize] as usize;
            if !big_done[c1] {
                let idx = copy_start[c1];
                ptr[idx as usize] = k as UInt32;
                copy_start[c1] += 1;
            }
            j += 1;
        }

        let end = ((ftab[(ss + 1) << 8] & CLEARMASK) as Int32) - 1;
        j = end;
        while j > copy_end[ss] {
            let mut k = ptr[j as usize] as Int32 - 1;
            if k < 0 {
                k += nblock;
            }
            let c1 = block[k as usize] as usize;
            if !big_done[c1] {
                let idx = copy_end[c1];
                ptr[idx as usize] = k as UInt32;
                copy_end[c1] -= 1;
            }
            j -= 1;
        }

        assert_h(
            (copy_start[ss] - 1 == copy_end[ss])
                || (copy_start[ss] == 0 && copy_end[ss] == nblock - 1),
            1007,
        );

        for j in 0..=255 {
            let idx = ((j << 8) + ss) as usize;
            ftab[idx] |= SETMASK;
        }

        big_done[ss] = TRUE;

        if idx < 255 {
            let bb_start = (ftab[ss << 8] & CLEARMASK) as Int32;
            let bb_size = ((ftab[(ss + 1) << 8] & CLEARMASK) as Int32) - bb_start;
            let mut shifts = 0i32;
            while (bb_size >> shifts) > 65534 {
                shifts += 1;
            }
            for j in (0..bb_size).rev() {
                let a2update = ptr[(bb_start + j) as usize] as usize;
                let q_val = (j >> shifts) as UInt16;
                quadrant[a2update] = q_val;
                if a2update < BZ_N_OVERSHOOT as usize {
                    quadrant[a2update + nblock_usize] = q_val;
                }
            }
            assert_h(((bb_size - 1) >> shifts) <= 65535, 1002);
        }
    }
}

/// Perform block sorting (Burrows–Wheeler transform order) on the given state.
///
/// On return:
/// * `arr2[0..nblock]` still contains the original block bytes.
/// * `arr1[0..nblock]` contains the sorted suffix pointers (fmap).
/// * Returns `orig_ptr`, the index in `arr1` where the original (unsuffixed)
///   block position 0 ended up.
pub fn block_sort(state: &mut BlockSortState<'_>) -> Int32 {
    let nblock = state.nblock;
    assert_h(nblock > 0, 1000);

    let ptr = &mut state.arr1[..nblock as usize];
    let block = &mut state.arr2[..(nblock as usize + BZ_N_OVERSHOOT as usize)];
    let ftab = &mut state.ftab;

    let verb = state.verbosity;
    let mut wfact = state.work_factor;

    if nblock < 10000 {
        let n_bhtab = 2 + (nblock / 32);
        assert_h(ftab.len() as Int32 >= n_bhtab, 9001);
        let (bhtab_slice, _) = ftab.split_at_mut(n_bhtab as usize);

        // reuse arr1 as fmap, and reinterpret arr2 bytes as eclass UInt32
        let fmap = &mut state.arr1[..nblock as usize];
        let eclass_u32: &mut [UInt32] = unsafe {
            core::slice::from_raw_parts_mut(state.arr2.as_mut_ptr() as *mut UInt32, nblock as usize)
        };

        fallback_sort(fmap, eclass_u32, bhtab_slice, nblock, verb);
    } else {
        // quadrant is placed after block[nblock + BZ_N_OVERSHOOT], aligned to 2 bytes
        let mut i = nblock + BZ_N_OVERSHOOT;
        if (i & 1) != 0 {
            i += 1;
        }
        let quad_offset = i as usize;
        let total_needed = quad_offset + (nblock as usize + BZ_N_OVERSHOOT as usize);
        assert_h(block.len() >= total_needed, 9002);

        let quadrant_ptr = unsafe { block.as_mut_ptr().add(quad_offset) as *mut UInt16 };
        let quadrant_len = nblock as usize + BZ_N_OVERSHOOT as usize;
        let quadrant: &mut [UInt16] = unsafe {
            core::slice::from_raw_parts_mut(quadrant_ptr, quadrant_len)
        };

        if wfact < 1 {
            wfact = 1;
        }
        if wfact > 100 {
            wfact = 100;
        }
        let budget_init: Int32 = nblock * ((wfact - 1) / 3);
        let mut budget = budget_init;

        main_sort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);
        if budget < 0 {
            let n_bhtab = 2 + (nblock / 32);
            assert_h(ftab.len() as Int32 >= n_bhtab, 9003);
            let (bhtab_slice, _) = ftab.split_at_mut(n_bhtab as usize);

            let fmap = &mut state.arr1[..nblock as usize];
            let eclass_u32: &mut [UInt32] = unsafe {
                core::slice::from_raw_parts_mut(state.arr2.as_mut_ptr() as *mut UInt32, nblock as usize)
            };

            fallback_sort(fmap, eclass_u32, bhtab_slice, nblock, verb);
        }
    }

    // find origPtr: position where suffix starting at 0 ends up
    let mut orig_ptr: Int32 = -1;
    for (i, &p) in ptr.iter().enumerate().take(nblock as usize) {
        if p == 0 {
            orig_ptr = i as Int32;
            break;
        }
    }
    assert_h(orig_ptr != -1, 1003);
    orig_ptr
}

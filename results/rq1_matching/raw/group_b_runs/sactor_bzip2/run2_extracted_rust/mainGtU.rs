// SACTOR unidiomatic translation of `mainGtU` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:34:19; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_bzip2/blocksort.c: Dependency 'BZ2_bz__AssertH__fail' of type 'fu
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

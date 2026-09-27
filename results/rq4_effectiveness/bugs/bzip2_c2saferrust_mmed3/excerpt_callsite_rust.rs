    d = stackD[sp as usize];
    if hi - lo < 20 || d > 2 + 12 {
        mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
        if *budget < 0 { return; }
    } else {
         let med = mmed3(
    unsafe { *block.add(*ptr.add(lo as usize) as usize + d as usize) },
    unsafe { *block.add(*ptr.add(hi as usize) as usize + d as usize) },
    unsafe { *block.add(*ptr.add((lo + (hi >> 1)) as usize) as usize + d as usize) },
) as i32;


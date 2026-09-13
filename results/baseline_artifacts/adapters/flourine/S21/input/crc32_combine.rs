type c_ulong = std::os::raw::c_ulong;
type uLong = std::os::raw::c_ulong;
type off_t = std::os::raw::c_long;

unsafe extern "C" fn gf2_matrix_times(mut mat: *mut std::os::raw::c_ulong,
                                      mut vec: std::os::raw::c_ulong)
 -> std::os::raw::c_ulong {
    let mut sum: std::os::raw::c_ulong = 0;
    sum = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    while vec != 0 {
        if vec & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 { sum ^= *mat }
        vec >>= 1 as std::os::raw::c_int;
        mat = mat.offset(1)
    }
    return sum;
}
/* ========================================================================= */
fn gf2_matrix_square(square: &mut [c_ulong; 32], mat: &[c_ulong; 32]) {
    for n in 0..32 {
        square[n] = unsafe { gf2_matrix_times(square.as_mut_ptr(), mat[n]) };
    }
}

/* ========================================================================= */
fn crc32_combine_(mut crc1: u64, crc2: u64, len2: i64) -> u64 {
    let mut n: i32 = 0; /* even-power-of-two zeros operator */
    let mut row: u64 = 0; /* odd-power-of-two zeros operator */
    let mut even: [u64; 32] = [0; 32];
    let mut odd: [u64; 32] = [0; 32];
    
    /* degenerate case (also disallow negative lengths) */
    if len2 <= 0 { return crc1; }
    
    /* put operator for one zero bit in odd */
    odd[0] = 0xedb88320; /* CRC-32 polynomial */
    row = 1;
    n = 1;
    
    while n < 32 {
        odd[n as usize] = row;
        row <<= 1;
        n += 1;
    }
    
    /* put operator for two zero bits in even */
    gf2_matrix_square(&mut even, &odd);
    /* put operator for four zero bits in odd */
    gf2_matrix_square(&mut odd, &even);
    
    let mut len2_copy = len2; // Create a mutable copy of len2 for manipulation
    
    loop {
        /* apply len2 zeros to crc1 (first square will put the operator for one
       zero byte, eight zero bits, in even) */
        /* apply zeros operator for this bit of len2 */
        gf2_matrix_square(&mut even, &odd);
        if len2_copy & 1 != 0 {
            crc1 = unsafe { gf2_matrix_times(odd.as_mut_ptr(), crc1) };
        }
        len2_copy >>= 1;
        
        /* if no more bits set, then done */
        if len2_copy == 0 { break; }
        
        /* another iteration of the loop with odd and even swapped */
        gf2_matrix_square(&mut odd, &even);
        if len2_copy & 1 != 0 {
            crc1 = unsafe { gf2_matrix_times(even.as_mut_ptr(), crc1) };
        }
        len2_copy >>= 1;
        
        if len2_copy == 0 { break; }
    }
    
    /* return combined crc */
    crc1 ^ crc2
}

/* ========================================================================= */

pub unsafe extern "C" fn crc32_combine(mut crc1: uLong, mut crc2: uLong,
                                       mut len2: off_t) -> uLong {
    return crc32_combine_(crc1, crc2, len2);
}

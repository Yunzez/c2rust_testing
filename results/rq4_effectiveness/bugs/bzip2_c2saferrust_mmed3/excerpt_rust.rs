   Sorting and Searching Strings", by Robert
   Sedgewick and Jon L. Bentley.
--*/
#[inline]
 fn mmed3(a: u8, b: u8, c: u8) -> u8 {
    let mut min = a;

    if b < min {
        min = b;
    }
    if c < min {
        min = c;
    }

    min
}

unsafe extern "C" fn mainQSort3(mut ptr: *mut UInt32, mut block: *mut UChar,
                                mut quadrant: *mut UInt16, mut nblock: Int32,

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

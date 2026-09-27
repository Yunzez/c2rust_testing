pub fn searchCodeIndex(array: *const u32, array_size: usize, value: usize) -> usize {
    unsafe {
        let mut left: usize = 1;
        let mut right: usize = array_size - 1;
        while left <= right {
            let mid: usize = (left + right) >> 1;
            if (*array.add(mid)) as usize >= value {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        if left >= array_size || (*array.add(left)) as usize > value {
            left -= 1;
        }
        left
    }
}

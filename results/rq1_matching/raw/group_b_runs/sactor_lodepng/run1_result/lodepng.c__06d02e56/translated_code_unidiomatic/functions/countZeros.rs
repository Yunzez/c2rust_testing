#[inline]
pub unsafe fn countZeros(
    data: *const libc::c_uchar,
    size: libc::size_t,
    pos: libc::size_t,
) -> libc::c_uint {
    let start: *const libc::c_uchar = data.add(pos);
    let mut end: *const libc::c_uchar = start.add(MAX_SUPPORTED_DEFLATE_LENGTH);
    let data_end: *const libc::c_uchar = data.add(size);
    if end > data_end {
        end = data_end;
    }
    let mut current: *const libc::c_uchar = start;
    while current != end && *current == 0 {
        current = current.add(1);
    }
    (current.offset_from(start) as libc::size_t) as libc::c_uint
}

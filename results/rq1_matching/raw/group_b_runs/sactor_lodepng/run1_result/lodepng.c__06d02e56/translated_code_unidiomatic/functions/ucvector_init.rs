#[inline]
pub unsafe fn ucvector_init(buffer: *mut libc::c_uchar, size: libc::size_t) -> ucvector {
    let mut v: ucvector = ucvector {
        data: core::ptr::null_mut(),
        size: 0,
        allocsize: 0,
    };
    v.data = buffer;
    v.allocsize = size;
    v.size = size;
    v
}

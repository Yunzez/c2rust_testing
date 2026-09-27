#[inline]
pub unsafe fn ensureBits9(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    unsafe fn get_data(reader: *mut LodePNGBitReader, index: libc::size_t) -> u32 {
        *(*reader).data.add(index) as u32
    }
    let start: libc::size_t = (*reader).bp >> 3;
    let size: libc::size_t = (*reader).size;
    if start + 1 < size {
        let v0 = get_data(reader, start + 0);
        let v1 = get_data(reader, start + 1);
        (*reader).buffer = (v0 | (v1 << 8)) as libc::c_uint;
        (*reader).buffer >>= ((*reader).bp & 7) as libc::c_uint;
    } else {
        (*reader).buffer = 0;
        if start < size {
            let v0 = get_data(reader, start + 0);
            (*reader).buffer = v0 as libc::c_uint;
        }
        (*reader).buffer >>= ((*reader).bp & 7) as libc::c_uint;
    }
    let _ = nbits;
}

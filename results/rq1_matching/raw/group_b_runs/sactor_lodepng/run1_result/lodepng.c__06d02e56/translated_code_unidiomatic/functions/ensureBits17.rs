#[inline]
pub unsafe fn ensureBits17(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    #[inline]
    unsafe fn read_byte(reader: *mut LodePNGBitReader, index: libc::size_t) -> u32 {
        let r = &*reader;
        if index < r.size {
            *r.data.add(index) as u32
        } else {
            0
        }
    }
    let r = &mut *reader;
    let start: libc::size_t = r.bp >> 3;
    let size: libc::size_t = r.size;
    if start + 2 < size {
        let b0 = read_byte(reader, start + 0);
        let b1 = read_byte(reader, start + 1);
        let b2 = read_byte(reader, start + 2);
        r.buffer = (b0 | (b1 << 8) | (b2 << 16)) as libc::c_uint;
        r.buffer >>= (r.bp & 7) as libc::c_uint;
    } else {
        r.buffer = 0;
        if start + 0 < size {
            r.buffer |= read_byte(reader, start + 0) as libc::c_uint;
        }
        if start + 1 < size {
            r.buffer |= (read_byte(reader, start + 1) << 8) as libc::c_uint;
        }
        r.buffer >>= (r.bp & 7) as libc::c_uint;
    }
    let _ = nbits;
}

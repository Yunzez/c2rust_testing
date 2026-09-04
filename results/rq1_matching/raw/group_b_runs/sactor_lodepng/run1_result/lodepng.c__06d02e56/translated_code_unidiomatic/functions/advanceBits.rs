#[inline]
pub unsafe fn advanceBits(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    (*reader).buffer >>= nbits;
    (*reader).bp = (*reader).bp.wrapping_add(nbits);
}

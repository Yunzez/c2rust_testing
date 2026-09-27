pub fn lodepng_set32bitInt(buffer: *mut u8, value: libc::c_uint) {
    unsafe {
        *buffer.add(0) = ((value >> 24) & 0xff) as u8;
        *buffer.add(1) = ((value >> 16) & 0xff) as u8;
        *buffer.add(2) = ((value >> 8) & 0xff) as u8;
        *buffer.add(3) = (value & 0xff) as u8;
    }
}

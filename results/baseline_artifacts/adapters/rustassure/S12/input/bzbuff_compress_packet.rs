#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/bzbuff_compress_packet_raw.inc"];

#[no_mangle]
pub fn bzbuff_compress_packet(packet: &mut [i8; 64]) -> i32 {
    let source_length = (packet[0] as u8 % 32) as usize;
    let source = unsafe {
        std::slice::from_raw_parts(packet.as_ptr().add(1) as *const u8, source_length)
    };
    let mut destination = vec![0u8; 128];
    BZ2_bzBuffToBuffCompress(&mut destination, source, 1, 0, 30)
}

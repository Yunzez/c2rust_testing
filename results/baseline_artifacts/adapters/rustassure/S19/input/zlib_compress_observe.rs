#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/zlib_compress_observe_raw.inc"];

#[no_mangle]
pub fn zlib_compress_observe(packet: &mut [i8; 512]) -> i32 {
    let source_length = (packet[0] as u8 % 128) as u64;
    let mut destination_length = 383u64;
    let result = unsafe {
        compress2(
            packet.as_mut_ptr().add(129) as *mut u8,
            &mut destination_length,
            packet.as_ptr().add(1) as *const u8,
            source_length,
            6,
        )
    };
    for (slot, value) in packet[..8]
        .iter_mut()
        .zip(destination_length.to_ne_bytes())
    {
        *slot = value as i8;
    }
    result
}

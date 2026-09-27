#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/zlib_uncompress_observe_raw.inc"];

#[no_mangle]
pub fn zlib_uncompress_observe(packet: &mut [i8; 512]) -> i32 {
    let source_length = 1 + (packet[0] as u8 % 192) as u64;
    let mut destination_length = 319u64;
    let result = unsafe {
        uncompress(
            packet.as_mut_ptr().add(193) as *mut u8,
            &mut destination_length,
            packet.as_ptr().add(1) as *const u8,
            source_length,
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

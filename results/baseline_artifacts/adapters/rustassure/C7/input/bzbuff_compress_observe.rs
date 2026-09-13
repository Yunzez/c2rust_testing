#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/bzbuff_compress_observe_raw.inc"];

#[no_mangle]
pub fn bzbuff_compress_observe(packet: &mut [i8; 322]) -> i32 {
    let source_length = (packet[0] as u8 % 64) as u32;
    let mut destination_length = 256u32;
    let result = unsafe {
        BZ2_bzBuffToBuffCompress(
            packet.as_mut_ptr().add(65),
            Some(&mut destination_length),
            packet.as_mut_ptr().add(1),
            source_length,
            1,
            0,
            30,
        )
    };
    let bytes = destination_length.to_ne_bytes();
    for (slot, value) in packet[..4].iter_mut().zip(bytes) {
        *slot = value as i8;
    }
    result
}

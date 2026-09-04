pub fn update_adler32(mut adler: u32, mut data: *const u8, mut len: u32) -> u32 {
    unsafe {
        let mut s1: u32 = adler & 0xffffu32;
        let mut s2: u32 = (adler >> 16) & 0xffffu32;
        while len != 0 {
            let amount: u32 = if len > 5552 { 5552 } else { len };
            len -= amount;
            let mut i: u32 = 0;
            while i != amount {
                s1 = s1.wrapping_add(*data as u32);
                data = data.add(1);
                s2 = s2.wrapping_add(s1);
                i += 1;
            }
            s1 %= 65521;
            s2 %= 65521;
        }
        (s2 << 16) | s1
    }
}

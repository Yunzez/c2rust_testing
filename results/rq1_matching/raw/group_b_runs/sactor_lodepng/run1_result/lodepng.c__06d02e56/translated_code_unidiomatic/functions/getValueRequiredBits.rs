pub fn getValueRequiredBits(value: u8) -> u32 {
    if value == 0 || value == 255 {
        return 1;
    }
    if value % 17 == 0 {
        return if value % 85 == 0 { 2 } else { 4 };
    }
    8
}

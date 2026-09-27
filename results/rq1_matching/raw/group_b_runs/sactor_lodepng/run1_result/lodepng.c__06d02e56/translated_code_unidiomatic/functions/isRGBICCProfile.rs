pub fn isRGBICCProfile(profile: *const u8, size: u32) -> u32 {
    if size < 20 {
        return 0;
    }
    unsafe {
        if *profile.add(16) == b'R'
            && *profile.add(17) == b'G'
            && *profile.add(18) == b'B'
            && *profile.add(19) == b' '
        {
            1
        } else {
            0
        }
    }
}

pub fn isGrayICCProfile(profile: *const u8, size: u32) -> u32 {
    unsafe {
        if size < 20 {
            return 0;
        }
        if *profile.add(16) == b'G'
            && *profile.add(17) == b'R'
            && *profile.add(18) == b'A'
            && *profile.add(19) == b'Y'
        {
            1
        } else {
            0
        }
    }
}

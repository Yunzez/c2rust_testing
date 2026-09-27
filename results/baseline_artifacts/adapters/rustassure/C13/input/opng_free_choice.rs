include!["/input/opng_free_choice_raw.inc"];

#[no_mangle]
pub fn opng_free_choice(choose_nonnull: u8) -> i32 {
    let ptr = if choose_nonnull != 0 {
        Box::into_raw(Box::new(0u8)) as *mut std::os::raw::c_void
    } else {
        std::ptr::null_mut()
    };
    opng_free(ptr);
    0
}



include!["/input/opng_free_null_raw.inc"];

#[no_mangle]
pub fn opng_free_null() { opng_free(std::ptr::null_mut()); }

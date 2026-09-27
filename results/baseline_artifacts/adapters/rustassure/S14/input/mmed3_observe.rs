include!["/input/mmed3_observe_raw.inc"];

#[no_mangle]
pub fn mmed3_observe(a: u8, b: u8, c: u8) -> u8 {
    mmed3(a, b, c)
}

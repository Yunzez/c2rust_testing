#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| { let _=obs_crc32::run(data); });

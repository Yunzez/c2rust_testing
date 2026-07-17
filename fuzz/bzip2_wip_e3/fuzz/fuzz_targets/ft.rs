#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use bzip2_wip_e3::bzlib as translated;
fuzz_target!(|data: &[u8]| {
    if data.len() < 3 { return; }
    let bs = ((data[0] % 9) + 1) as i32; let wf = (data[1] % 251) as i32;
    let src = data[2..].to_vec();
    if src.is_empty() { return; }
    let mut dest: Vec<u8> = vec![0u8; src.len()*2 + 1024];
    let _ = translated::BZ2_bzBuffToBuffCompress(&mut dest, &src, bs, 0, wf);
});

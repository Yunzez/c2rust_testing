#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use bzip2_crown_e3::bzlib as translated;
fuzz_target!(|data: &[u8]| {
    if data.len() < 3 { return; }
    let bs = ((data[0] % 9) + 1) as i32; let wf = (data[1] % 251) as i32;
    let src = &data[2..];
    let mut dest = vec![0i8; src.len()*2 + 1024]; let mut dlen = dest.len() as u32;
    unsafe {
        let rc = translated::BZ2_bzBuffToBuffCompress(dest.as_mut_ptr(), Some(&mut dlen), src.as_ptr() as *mut i8, src.len() as u32, bs, 0, wf);
    }
});

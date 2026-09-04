#![no_main]
#![allow(unused, non_snake_case)]
// OBS corpus generator: same decoding as the drivers (whole input = src, cap 64 KiB, bs=1, wf=30), Laertes compress only.
use libfuzzer_sys::fuzz_target;
use bzip2_laertes_e3::bzlib::BZ2_bzBuffToBuffCompress;
fuzz_target!(|data: &[u8]| {
    let src = if data.len()>65536 { &data[..65536] } else { data };
    let mut dest = vec![0i8; 65536*2+1200]; let mut dlen = dest.len() as u32;
    unsafe { let _ = BZ2_bzBuffToBuffCompress(dest.as_mut_ptr(), Some(&mut dlen), src.as_ptr() as *mut i8, src.len() as u32, 1, 0, 30); }
});

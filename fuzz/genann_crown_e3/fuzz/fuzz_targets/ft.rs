#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use genann_crown_e3::src::genann as translated;
fuzz_target!(|data: &[u8]| {
    if data.len() < 40 { return; }
    let rd = |o: usize| f64::from_le_bytes(data[o..o+8].try_into().unwrap());
    let inp = [rd(0), rd(8)]; let out = [rd(16)]; let lr = rd(24);
    unsafe {
        let ann = translated::genann_init(2, 1, 3, 1);
        if ann.is_null() { return; }
        for _ in 0..3 { translated::genann_train(ann, inp.as_ptr(), out.as_ptr(), lr); }
        let _ = translated::genann_run(Some(&mut *ann), inp.as_ptr());
        let ann2 = translated::genann_copy(ann);
        if !ann2.is_null() { translated::genann_free(ann2); }
        translated::genann_free(ann);
    }
});

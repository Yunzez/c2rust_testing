pub unsafe fn genann_init_sigmoid_lookup(ann: *const genann) {
    let f: f64 = (sigmoid_dom_max - sigmoid_dom_min) / 4096.0;
    interval = 4096.0 / (sigmoid_dom_max - sigmoid_dom_min);
    let mut i: libc::c_int = 0;
    while i < 4096 {
        let ptr = &lookup as *const [f64; 4096] as *mut [f64; 4096];
        (*ptr)[i as usize] = genann_act_sigmoid(ann, sigmoid_dom_min + f * (i as f64));
        i += 1;
    }
}

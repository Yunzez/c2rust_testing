pub type genann_actfun =
    Option<unsafe extern "C" fn(*const genann, ::core::ffi::c_double) -> ::core::ffi::c_double>;
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct genann {
    pub inputs: ::core::ffi::c_int,
    pub hidden_layers: ::core::ffi::c_int,
    pub hidden: ::core::ffi::c_int,
    pub outputs: ::core::ffi::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: ::core::ffi::c_int,
    pub total_neurons: ::core::ffi::c_int,
    pub weight: *mut ::core::ffi::c_double,
    pub output: *mut ::core::ffi::c_double,
    pub delta: *mut ::core::ffi::c_double,
}
pub const sigmoid_dom_max: f64 = 15.0;
pub const sigmoid_dom_min: libc::c_double = -15.0f64;
pub static mut interval: f64 = 0.0;
static lookup: [f64; 4096] = [0.0; 4096];
pub unsafe extern "C" fn genann_act_hidden_indirect(
    ann: *const genann,
    a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let f = (*ann)
        .activation_hidden
        .expect("activation_hidden function pointer is null");
    f(ann, a)
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_output_indirect(ann: *const genann, a: f64) -> f64 {
    if let Some(func) = (*ann).activation_output {
        func(ann, a)
    } else {
        let func: unsafe extern "C" fn(*const genann, f64) -> f64 =
            core::mem::transmute::<genann_actfun, unsafe extern "C" fn(*const genann, f64) -> f64>(
                (*ann).activation_output,
            );
        func(ann, a)
    }
}
pub unsafe extern "C" fn genann_act_sigmoid(ann: *const genann, a: f64) -> f64 {
    let _ = ann;
    if a < -45.0 {
        return 0.0;
    }
    if a > 45.0 {
        return 1.0;
    }
    1.0 / (1.0 + (-a).exp())
}
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
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid_cached(
    ann: *const crate::genann,
    a: libc::c_double,
) -> libc::c_double {
    extern "C" {
        fn __assert_fail(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        ) -> !;
    }
    if a != a {
        static ASSERT_EXPR: &[u8] = b"!(a != a)\0";
        static ASSERT_FILE: &[u8] = b"genann.c\0";
        static ASSERT_FUNC: &[u8] = b"genann_act_sigmoid_cached\0";
        __assert_fail(
            ASSERT_EXPR.as_ptr() as *const libc::c_char,
            ASSERT_FILE.as_ptr() as *const libc::c_char,
            0 as libc::c_uint,
            ASSERT_FUNC.as_ptr() as *const libc::c_char,
        );
    }
    if a < crate::sigmoid_dom_min {
        return crate::lookup[0];
    }
    if a >= crate::sigmoid_dom_max {
        return crate::lookup[4096usize - 1];
    }
    let j: usize = {
        let diff = a - crate::sigmoid_dom_min;
        let interval_val = crate::interval;
        ((diff * interval_val) + 0.5f64) as usize
    };
    if j >= 4096usize {
        return crate::lookup[4096usize - 1];
    }
    crate::lookup[j]
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_linear(ann: *const genann, a: libc::c_double) -> libc::c_double {
    let _ = ann;
    a
}
pub unsafe extern "C" fn genann_act_threshold(ann: *const genann, a: f64) -> f64 {
    let _ = ann;
    (a > 0.0) as i32 as f64
}
#[no_mangle]
pub unsafe fn genann_init(
    inputs: libc::c_int,
    hidden_layers: libc::c_int,
    hidden: libc::c_int,
    outputs: libc::c_int,
) -> *mut genann {
    if hidden_layers < 0 {
        return core::ptr::null_mut();
    }
    if inputs < 1 {
        return core::ptr::null_mut();
    }
    if outputs < 1 {
        return core::ptr::null_mut();
    }
    if hidden_layers > 0 && hidden < 1 {
        return core::ptr::null_mut();
    }
    let hidden_weights: libc::c_int = if hidden_layers != 0 {
        (inputs + 1) * hidden + (hidden_layers - 1) * (hidden + 1) * hidden
    } else {
        0
    };
    let output_weights: libc::c_int = (if hidden_layers != 0 {
        hidden + 1
    } else {
        inputs + 1
    }) * outputs;
    let total_weights: libc::c_int = hidden_weights + output_weights;
    let total_neurons: libc::c_int = inputs + hidden * hidden_layers + outputs;
    let size: usize = core::mem::size_of::<genann>()
        + core::mem::size_of::<libc::c_double>()
            * (total_weights as usize + total_neurons as usize + (total_neurons - inputs) as usize);
    let ret: *mut genann = libc::malloc(size) as *mut genann;
    if ret.is_null() {
        return core::ptr::null_mut();
    }
    (*ret).inputs = inputs;
    (*ret).hidden_layers = hidden_layers;
    (*ret).hidden = hidden;
    (*ret).outputs = outputs;
    (*ret).total_weights = total_weights;
    (*ret).total_neurons = total_neurons;
    let base_ptr =
        (ret as *mut libc::c_char).add(core::mem::size_of::<genann>()) as *mut libc::c_double;
    (*ret).weight = base_ptr;
    (*ret).output = base_ptr.add((*ret).total_weights as usize);
    (*ret).delta = (*ret).output.add((*ret).total_neurons as usize);
    genann_randomize(ret);
    (*ret).activation_hidden = Some(genann_act_sigmoid_cached);
    (*ret).activation_output = Some(genann_act_sigmoid_cached);
    genann_init_sigmoid_lookup(ret as *const genann);
    ret
}
pub unsafe fn genann_read(in_: *mut libc::FILE) -> *mut genann {
    let mut inputs: libc::c_int = 0;
    let mut hidden_layers: libc::c_int = 0;
    let mut hidden: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    unsafe fn errno_location() -> *mut libc::c_int {
        libc::__errno_location()
    }
    unsafe {
        *errno_location() = 0;
        rc = libc::fscanf(
            in_,
            b"%d %d %d %d\0".as_ptr() as *const libc::c_char,
            &mut inputs as *mut libc::c_int,
            &mut hidden_layers as *mut libc::c_int,
            &mut hidden as *mut libc::c_int,
            &mut outputs as *mut libc::c_int,
        );
        if rc < 4 || *errno_location() != 0 {
            libc::perror(b"fscanf\0".as_ptr() as *const libc::c_char);
            return core::ptr::null_mut();
        }
        let ann: *mut genann = genann_init(inputs, hidden_layers, hidden, outputs);
        let mut i: libc::c_int = 0;
        while i < (*ann).total_weights {
            *errno_location() = 0;
            rc = libc::fscanf(
                in_,
                b" %le\0".as_ptr() as *const libc::c_char,
                (*ann).weight.offset(i as isize),
            );
            if rc < 1 || *errno_location() != 0 {
                libc::perror(b"fscanf\0".as_ptr() as *const libc::c_char);
                genann_free(ann);
                return core::ptr::null_mut();
            }
            i += 1;
        }
        ann
    }
}
pub unsafe fn genann_copy(ann: *const genann) -> *mut genann {
    use core::mem::size_of;
    use libc::{c_void, malloc, memcpy};
    if ann.is_null() {
        return core::ptr::null_mut();
    }
    let ann_ref: &genann = &*ann;
    let elem_count: i32 =
        ann_ref.total_weights + ann_ref.total_neurons + (ann_ref.total_neurons - ann_ref.inputs);
    if elem_count < 0 {
        return core::ptr::null_mut();
    }
    let size: libc::size_t = (size_of::<genann>()
        + size_of::<core::ffi::c_double>() * (elem_count as usize))
        as libc::size_t;
    let ret = malloc(size) as *mut genann;
    if ret.is_null() {
        return core::ptr::null_mut();
    }
    memcpy(ret as *mut c_void, ann as *const c_void, size);
    let ret_ref: &mut genann = &mut *ret;
    ret_ref.weight = (ret as *mut u8).add(size_of::<genann>()) as *mut core::ffi::c_double;
    ret_ref.output = ret_ref.weight.add(ret_ref.total_weights as usize);
    ret_ref.delta = ret_ref.output.add(ret_ref.total_neurons as usize);
    ret
}
pub unsafe fn genann_randomize(ann: *mut genann) {
    unsafe fn genann_random() -> libc::c_double {
        let r = libc::rand() as libc::c_double;
        r / 2147483647.0f64
    }
    if ann.is_null() {
        return;
    }
    let ann_ref = &mut *ann;
    let total = ann_ref.total_weights;
    let weights = ann_ref.weight;
    if weights.is_null() {
        return;
    }
    let mut i: libc::c_int = 0;
    while i < total {
        let r = genann_random();
        *weights.add(i as usize) = r - 0.5f64;
        i += 1;
    }
}
pub unsafe fn genann_free(ann: *mut genann) {
    libc::free(ann.cast());
}
pub unsafe fn genann_run(ann: *const genann, inputs: *const f64) -> *const f64 {
    use libc::memcpy;
    let mut w: *const f64 = (*ann).weight;
    let mut o: *mut f64 = (*ann).output.add((*ann).inputs as usize);
    let mut i: *const f64 = (*ann).output;
    memcpy(
        (*ann).output as *mut libc::c_void,
        inputs as *const libc::c_void,
        ::core::mem::size_of::<f64>() * (*ann).inputs as usize,
    );
    let mut h: libc::c_int;
    let mut j: libc::c_int;
    let mut k: libc::c_int;
    if (*ann).hidden_layers == 0 {
        let ret: *const f64 = o;
        j = 0;
        while j < (*ann).outputs {
            let mut sum: f64 = *w * -1.0;
            w = w.add(1);
            k = 0;
            while k < (*ann).inputs {
                sum += *w * *i.add(k as usize);
                w = w.add(1);
                k += 1;
            }
            *o = genann_act_output_indirect(ann, sum);
            o = o.add(1);
            j += 1;
        }
        return ret;
    }
    j = 0;
    while j < (*ann).hidden {
        let mut sum: f64 = *w * -1.0;
        w = w.add(1);
        k = 0;
        while k < (*ann).inputs {
            sum += *w * *i.add(k as usize);
            w = w.add(1);
            k += 1;
        }
        *o = genann_act_hidden_indirect(ann, sum);
        o = o.add(1);
        j += 1;
    }
    i = i.add((*ann).inputs as usize);
    h = 1;
    while h < (*ann).hidden_layers {
        j = 0;
        while j < (*ann).hidden {
            let mut sum: f64 = *w * -1.0;
            w = w.add(1);
            k = 0;
            while k < (*ann).hidden {
                sum += *w * *i.add(k as usize);
                w = w.add(1);
                k += 1;
            }
            *o = genann_act_hidden_indirect(ann, sum);
            o = o.add(1);
            j += 1;
        }
        i = i.add((*ann).hidden as usize);
        h += 1;
    }
    let ret: *const f64 = o;
    j = 0;
    while j < (*ann).outputs {
        let mut sum: f64 = *w * -1.0;
        w = w.add(1);
        k = 0;
        while k < (*ann).hidden {
            sum += *w * *i.add(k as usize);
            w = w.add(1);
            k += 1;
        }
        *o = genann_act_output_indirect(ann, sum);
        o = o.add(1);
        j += 1;
    }
    extern "C" {
        fn __assert_fail(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        ) -> !;
    }
    unsafe fn assert_expr(
        expr: bool,
        assertion: &::core::ffi::CStr,
        file: &::core::ffi::CStr,
        line: libc::c_uint,
        function: &::core::ffi::CStr,
    ) {
        if !expr {
            __assert_fail(assertion.as_ptr(), file.as_ptr(), line, function.as_ptr());
        }
    }
    {
        use core::ffi::CStr;
        static ASSERT1: &CStr = unsafe {
            CStr::from_bytes_with_nul_unchecked(b"w - ann->weight == ann->total_weights\0")
        };
        static FILE_NAME: &CStr =
            unsafe { CStr::from_bytes_with_nul_unchecked(b"<rust_genann_run>\0") };
        static FUNC_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"genann_run\0") };
        let used_weights = w.offset_from((*ann).weight) as libc::c_int;
        assert_expr(
            used_weights == (*ann).total_weights,
            ASSERT1,
            FILE_NAME,
            0,
            FUNC_NAME,
        );
    }
    {
        use core::ffi::CStr;
        static ASSERT2: &CStr = unsafe {
            CStr::from_bytes_with_nul_unchecked(b"o - ann->output == ann->total_neurons\0")
        };
        static FILE_NAME: &CStr =
            unsafe { CStr::from_bytes_with_nul_unchecked(b"<rust_genann_run>\0") };
        static FUNC_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"genann_run\0") };
        let used_neurons = o.offset_from((*ann).output) as libc::c_int;
        assert_expr(
            used_neurons == (*ann).total_neurons,
            ASSERT2,
            FILE_NAME,
            0,
            FUNC_NAME,
        );
    }
    ret
}
pub unsafe fn genann_train(
    ann: *const genann,
    inputs: *const f64,
    desired_outputs: *const f64,
    learning_rate: f64,
) {
    use libc::{c_double, c_int};
    genann_run(ann, inputs);
    let ann_ref: &genann = &*ann;
    let mut h: c_int;
    let mut j: c_int;
    let mut k: c_int;
    {
        let mut o: *const c_double = ann_ref
            .output
            .add(ann_ref.inputs as usize + (ann_ref.hidden * ann_ref.hidden_layers) as usize);
        let mut d: *mut c_double = ann_ref
            .delta
            .add((ann_ref.hidden * ann_ref.hidden_layers) as usize);
        let mut t: *const c_double = desired_outputs;
        let use_linear = {
            let act_out_ptr: *const () =
                core::mem::transmute::<genann_actfun, *const ()>(ann_ref.activation_output);
            let lin_ptr: *const () = core::mem::transmute::<
                unsafe extern "C" fn(*const genann, f64) -> f64,
                *const (),
            >(genann_act_linear);
            act_out_ptr == lin_ptr
        };
        if use_linear {
            j = 0;
            while j < ann_ref.outputs {
                *d = *t - *o;
                d = d.add(1);
                t = t.add(1);
                o = o.add(1);
                j += 1;
            }
        } else {
            j = 0;
            while j < ann_ref.outputs {
                let o_val = *o;
                *d = (*t - o_val) * o_val * (1.0 - o_val);
                o = o.add(1);
                t = t.add(1);
                d = d.add(1);
                j += 1;
            }
        }
    }
    h = ann_ref.hidden_layers - 1;
    while h >= 0 {
        let h_usize = h as usize;
        let mut o: *const c_double = ann_ref
            .output
            .add(ann_ref.inputs as usize + h_usize * ann_ref.hidden as usize);
        let mut d: *mut c_double = ann_ref.delta.add(h_usize * ann_ref.hidden as usize);
        let dd: *const c_double = ann_ref.delta.add(((h + 1) * ann_ref.hidden) as usize);
        let ww: *const c_double = ann_ref.weight.add(
            ((ann_ref.inputs + 1) * ann_ref.hidden + (ann_ref.hidden + 1) * ann_ref.hidden * h)
                as usize,
        );
        j = 0;
        while j < ann_ref.hidden {
            let mut delta: c_double = 0.0;
            let next_len = if h == ann_ref.hidden_layers - 1 {
                ann_ref.outputs
            } else {
                ann_ref.hidden
            };
            k = 0;
            while k < next_len {
                let forward_delta = *dd.add(k as usize);
                let windex = (k * (ann_ref.hidden + 1) + (j + 1)) as usize;
                let forward_weight = *ww.add(windex);
                delta += forward_delta * forward_weight;
                k += 1;
            }
            let o_val = *o;
            *d = o_val * (1.0 - o_val) * delta;
            d = d.add(1);
            o = o.add(1);
            j += 1;
        }
        h -= 1;
    }
    {
        let mut d: *const c_double = ann_ref
            .delta
            .add((ann_ref.hidden * ann_ref.hidden_layers) as usize);
        let mut w: *mut c_double = if ann_ref.hidden_layers != 0 {
            ann_ref.weight.add(
                ((ann_ref.inputs + 1) * ann_ref.hidden
                    + (ann_ref.hidden + 1) * ann_ref.hidden * (ann_ref.hidden_layers - 1))
                    as usize,
            )
        } else {
            ann_ref.weight
        };
        let i: *const c_double = if ann_ref.hidden_layers != 0 {
            ann_ref
                .output
                .add((ann_ref.inputs + ann_ref.hidden * (ann_ref.hidden_layers - 1)) as usize)
        } else {
            ann_ref.output
        };
        j = 0;
        while j < ann_ref.outputs {
            let d_val = *d;
            *w += d_val * learning_rate * -1.0;
            w = w.add(1);
            let input_count = if ann_ref.hidden_layers != 0 {
                ann_ref.hidden
            } else {
                ann_ref.inputs
            };
            k = 1;
            while k < input_count + 1 {
                let input_val = *i.add((k - 1) as usize);
                *w += d_val * learning_rate * input_val;
                w = w.add(1);
                k += 1;
            }
            d = d.add(1);
            j += 1;
        }
        let diff = w.offset_from(ann_ref.weight);
        if diff != ann_ref.total_weights as isize {
            libc::abort();
        }
    }
    h = ann_ref.hidden_layers - 1;
    while h >= 0 {
        let h_usize = h as usize;
        let mut d: *const c_double = ann_ref.delta.add(h_usize * ann_ref.hidden as usize);
        let i: *const c_double = if h != 0 {
            ann_ref
                .output
                .add((ann_ref.inputs + ann_ref.hidden * (h - 1)) as usize)
        } else {
            ann_ref.output
        };
        let mut w: *mut c_double = if h != 0 {
            ann_ref.weight.add(
                ((ann_ref.inputs + 1) * ann_ref.hidden
                    + (ann_ref.hidden + 1) * ann_ref.hidden * (h - 1)) as usize,
            )
        } else {
            ann_ref.weight
        };
        j = 0;
        while j < ann_ref.hidden {
            let d_val = *d;
            *w += d_val * learning_rate * -1.0;
            w = w.add(1);
            let input_count = if h == 0 {
                ann_ref.inputs
            } else {
                ann_ref.hidden
            };
            k = 1;
            while k < input_count + 1 {
                let input_val = *i.add((k - 1) as usize);
                *w += d_val * learning_rate * input_val;
                w = w.add(1);
                k += 1;
            }
            d = d.add(1);
            j += 1;
        }
        h -= 1;
    }
}
pub unsafe fn genann_write(ann: *const genann, out: *mut libc::FILE) {
    use core::ffi::c_char;
    use libc::{c_double, c_int, fprintf};
    unsafe fn fmt_str(s: &str) -> *const c_char {
        match s {
            "%d %d %d %d" => {
                static FORMAT: [c_char; 12] = [
                    b'%' as c_char,
                    b'd' as c_char,
                    b' ' as c_char,
                    b'%' as c_char,
                    b'd' as c_char,
                    b' ' as c_char,
                    b'%' as c_char,
                    b'd' as c_char,
                    b' ' as c_char,
                    b'%' as c_char,
                    b'd' as c_char,
                    0,
                ];
                FORMAT.as_ptr()
            }
            " %.20e" => {
                static FORMAT: [c_char; 8] = [
                    b' ' as c_char,
                    b'%' as c_char,
                    b'.' as c_char,
                    b'2' as c_char,
                    b'0' as c_char,
                    b'e' as c_char,
                    0,
                    0,
                ];
                FORMAT.as_ptr()
            }
            _ => core::ptr::null(),
        }
    }
    if ann.is_null() || out.is_null() {
        return;
    }
    fprintf(
        out,
        fmt_str("%d %d %d %d"),
        (*ann).inputs as c_int,
        (*ann).hidden_layers as c_int,
        (*ann).hidden as c_int,
        (*ann).outputs as c_int,
    );
    let mut i: c_int = 0;
    while i < (*ann).total_weights {
        let w_ptr = (*ann).weight.add(i as usize);
        fprintf(out, fmt_str(" %.20e"), *w_ptr as c_double);
        i += 1;
    }
}

// ==== driver TU ====
pub fn main() -> () {
    use std::env;
    use std::process;
    let args: Vec<String> = env::args().collect();
    let argc = args.len() as i32;
    if argc != 4 {
        unsafe {
            libc::printf(b"usage: driver <a> <b> <train_iters>\n\0".as_ptr() as *const libc::c_char);
        }
        process::exit(1);
    }
    let mut in_values: [f64; 2] = [0.0, 0.0];
    in_values[0] = args[1].parse::<f64>().unwrap_or(0.0);
    in_values[1] = args[2].parse::<f64>().unwrap_or(0.0);
    let iters: i32 = args[3].parse::<i32>().unwrap_or(0);
    let ann = unsafe { genann_init(2, 1, 4, 1) };
    if ann.is_null() {
        process::exit(2);
    }
    unsafe {
        let total_weights = (*ann).total_weights as isize;
        let weight_ptr = (*ann).weight;
        for i in 0..total_weights {
            let val = (((i as i32 * 37) % 200) as f64 - 100.0) / 100.0;
            *weight_ptr.offset(i) = val;
        }
    }
    let mut out: *const f64 = std::ptr::null();
    unsafe {
        out = genann_run(ann as *const _, in_values.as_ptr());
        libc::printf(
            b"run %.12f\n\0".as_ptr() as *const libc::c_char,
            *out.add(0),
        );
    }
    let tin: [[f64; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let tout: [f64; 4] = [0.0, 1.0, 1.0, 0.0];
    unsafe {
        for _it in 0..iters {
            for k in 0..4 {
                genann_train(
                    ann as *const _,
                    tin[k].as_ptr(),
                    &tout[k] as *const f64,
                    0.5,
                );
            }
        }
        out = genann_run(ann as *const _, in_values.as_ptr());
        libc::printf(
            b"trained %.12f\n\0".as_ptr() as *const libc::c_char,
            *out.add(0),
        );
        genann_free(ann);
    }
}

#[no_mangle]
pub extern "C" fn __assert_rtn(_f: *const std::os::raw::c_char, _fl: *const std::os::raw::c_char, _l: std::os::raw::c_int, _e: *const std::os::raw::c_char) -> ! { std::process::abort() }

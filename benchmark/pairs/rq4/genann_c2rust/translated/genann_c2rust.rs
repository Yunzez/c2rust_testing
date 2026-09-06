// GENERATED for the RQ4 coverage experiment by scratchpad/rq4_cov/flatten_rust.py.
// Module bodies are copied byte-for-byte from
// tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/ (== fuzz/bzip2_c2rust_e3/src/).
// Only the module wrappers and the root re-exports below are added.
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut, internal_features,
         unused_imports, unpredictable_function_pointer_comparisons)]

pub mod genann {

extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(_: *const std::os::raw::c_char);
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn exp(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn rand() -> std::os::raw::c_int;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
}
pub type __int64_t = crate::example3::__int64_t;
pub type __darwin_off_t = crate::example3::__darwin_off_t;
pub type fpos_t = crate::example3::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example3::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example3::__sFILE;
pub type FILE = crate::example3::FILE;
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    if a < -45.0f64 { return 0 as std::os::raw::c_int as std::os::raw::c_double }
    if a > 45.0f64 { return 1 as std::os::raw::c_int as std::os::raw::c_double }
    return 1.0f64 / (1 as std::os::raw::c_int as std::os::raw::c_double + exp(-a));
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid_cached(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    /* If you're optimizing for memory usage, just
     * delete this entire function and replace references
     * of genann_act_sigmoid_cached to genann_act_sigmoid
     */
    let min: std::os::raw::c_double = -15.0f64;
    let max: std::os::raw::c_double = 15.0f64;
    static mut interval: std::os::raw::c_double = 0.;
    static mut initialized: std::os::raw::c_int = 0 as std::os::raw::c_int;
    static mut lookup: [std::os::raw::c_double; 4096] = [0.; 4096];
    /* Calculate entire lookup table on first run. */
    if initialized == 0 {
        interval = (max - min) / 4096 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < 4096 as std::os::raw::c_int {
            lookup[i as usize] =
                genann_act_sigmoid(min + interval * i as std::os::raw::c_double);
            i += 1
        }
        /* This is down here to make this thread safe. */
        initialized = 1 as std::os::raw::c_int
    }
    let mut i_0: std::os::raw::c_int = 0;
    i_0 = ((a - min) / interval + 0.5f64) as std::os::raw::c_int;
    if i_0 <= 0 as std::os::raw::c_int { return lookup[0 as std::os::raw::c_int as usize] }
    if i_0 >= 4096 as std::os::raw::c_int {
        return lookup[(4096 as std::os::raw::c_int - 1 as std::os::raw::c_int) as usize]
    }
    return lookup[i_0 as usize];
}
#[no_mangle]
pub extern "C" fn genann_act_threshold(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    return (a > 0 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int as
               std::os::raw::c_double;
}
#[no_mangle]
pub extern "C" fn genann_act_linear(mut a: std::os::raw::c_double)
 -> std::os::raw::c_double {
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn genann_init(mut inputs: std::os::raw::c_int,
                                     mut hidden_layers: std::os::raw::c_int,
                                     mut hidden: std::os::raw::c_int,
                                     mut outputs: std::os::raw::c_int)
 -> *mut genann {
    if hidden_layers < 0 as std::os::raw::c_int { return 0 as *mut genann }
    if inputs < 1 as std::os::raw::c_int { return 0 as *mut genann }
    if outputs < 1 as std::os::raw::c_int { return 0 as *mut genann }
    if hidden_layers > 0 as std::os::raw::c_int && hidden < 1 as std::os::raw::c_int {
        return 0 as *mut genann
    }
    let hidden_weights: std::os::raw::c_int =
        if hidden_layers != 0 {
            ((inputs + 1 as std::os::raw::c_int) * hidden) +
                (hidden_layers - 1 as std::os::raw::c_int) *
                    (hidden + 1 as std::os::raw::c_int) * hidden
        } else { 0 as std::os::raw::c_int };
    let output_weights: std::os::raw::c_int =
        (if hidden_layers != 0 {
             (hidden) + 1 as std::os::raw::c_int
         } else { (inputs) + 1 as std::os::raw::c_int }) * outputs;
    let total_weights: std::os::raw::c_int = hidden_weights + output_weights;
    let total_neurons: std::os::raw::c_int =
        inputs + hidden * hidden_layers + outputs;
    /* Allocate extra size for weights, outputs, and deltas. */
    let size: std::os::raw::c_int =
        (::std::mem::size_of::<genann>() as
             std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                              as
                                              std::os::raw::c_ulong).wrapping_mul((total_weights
                                                                               +
                                                                               total_neurons
                                                                               +
                                                                               (total_neurons
                                                                                    -
                                                                                    inputs))
                                                                              as
                                                                              std::os::raw::c_ulong))
            as std::os::raw::c_int;
    let mut ret: *mut genann = malloc(size as std::os::raw::c_ulong) as *mut genann;
    if ret.is_null() { return 0 as *mut genann }
    (*ret).inputs = inputs;
    (*ret).hidden_layers = hidden_layers;
    (*ret).hidden = hidden;
    (*ret).outputs = outputs;
    (*ret).total_weights = total_weights;
    (*ret).total_neurons = total_neurons;
    /* Set pointers. */
    (*ret).weight =
        (ret as
             *mut std::os::raw::c_char).offset(::std::mem::size_of::<genann>() as
                                           std::os::raw::c_ulong as isize) as
            *mut std::os::raw::c_double;
    (*ret).output = (*ret).weight.offset((*ret).total_weights as isize);
    (*ret).delta = (*ret).output.offset((*ret).total_neurons as isize);
    genann_randomize(ret);
    (*ret).activation_hidden =
        Some(genann_act_sigmoid_cached as
                 unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double);
    (*ret).activation_output =
        Some(genann_act_sigmoid_cached as
                 unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_read(mut in_0: *mut FILE) -> *mut genann {
    let mut inputs: std::os::raw::c_int = 0;
    let mut hidden_layers: std::os::raw::c_int = 0;
    let mut hidden: std::os::raw::c_int = 0;
    let mut outputs: std::os::raw::c_int = 0;
    let mut rc: std::os::raw::c_int = 0;
    *__error() = 0 as std::os::raw::c_int;
    rc =
        fscanf(in_0, b"%d %d %d %d\x00" as *const u8 as *const std::os::raw::c_char,
               &mut inputs as *mut std::os::raw::c_int,
               &mut hidden_layers as *mut std::os::raw::c_int,
               &mut hidden as *mut std::os::raw::c_int,
               &mut outputs as *mut std::os::raw::c_int);
    if rc < 4 as std::os::raw::c_int || *__error() != 0 as std::os::raw::c_int {
        perror(b"fscanf\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as *mut genann
    }
    let mut ann: *mut genann =
        genann_init(inputs, hidden_layers, hidden, outputs);
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*ann).total_weights {
        *__error() = 0 as std::os::raw::c_int;
        rc =
            fscanf(in_0, b" %le\x00" as *const u8 as *const std::os::raw::c_char,
                   (*ann).weight.offset(i as isize));
        if rc < 1 as std::os::raw::c_int || *__error() != 0 as std::os::raw::c_int {
            perror(b"fscanf\x00" as *const u8 as *const std::os::raw::c_char);
            genann_free(ann);
            return 0 as *mut genann
        }
        i += 1
    }
    return ann;
}
#[no_mangle]
pub unsafe extern "C" fn genann_copy(mut ann: *const genann) -> *mut genann {
    let size: std::os::raw::c_int =
        (::std::mem::size_of::<genann>() as
             std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                              as
                                              std::os::raw::c_ulong).wrapping_mul(((*ann).total_weights
                                                                               +
                                                                               (*ann).total_neurons
                                                                               +
                                                                               ((*ann).total_neurons
                                                                                    -
                                                                                    (*ann).inputs))
                                                                              as
                                                                              std::os::raw::c_ulong))
            as std::os::raw::c_int;
    let mut ret: *mut genann = malloc(size as std::os::raw::c_ulong) as *mut genann;
    if ret.is_null() { return 0 as *mut genann }
    memcpy(ret as *mut std::os::raw::c_void, ann as *const std::os::raw::c_void,
           size as std::os::raw::c_ulong);
    /* Set pointers. */
    (*ret).weight =
        (ret as
             *mut std::os::raw::c_char).offset(::std::mem::size_of::<genann>() as
                                           std::os::raw::c_ulong as isize) as
            *mut std::os::raw::c_double;
    (*ret).output = (*ret).weight.offset((*ret).total_weights as isize);
    (*ret).delta = (*ret).output.offset((*ret).total_neurons as isize);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_randomize(mut ann: *mut genann) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*ann).total_weights {
        let mut r: std::os::raw::c_double =
            rand() as std::os::raw::c_double /
                0x7fffffff as std::os::raw::c_int as std::os::raw::c_double;
        /* Sets weights from -0.5 to 0.5. */
        *(*ann).weight.offset(i as isize) = r - 0.5f64;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn genann_free(mut ann: *mut genann) {
    /* The weight, output, and delta pointers go to the same buffer. */
    free(ann as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn genann_run(mut ann: *const genann,
                                    mut inputs: *const std::os::raw::c_double)
 -> *const std::os::raw::c_double {
    let mut w: *const std::os::raw::c_double = (*ann).weight;
    let mut o: *mut std::os::raw::c_double =
        (*ann).output.offset((*ann).inputs as isize);
    let mut i: *const std::os::raw::c_double = (*ann).output;
    /* Copy the inputs to the scratch area, where we also store each neuron's
     * output, for consistency. This way the first layer isn't a special case. */
    memcpy((*ann).output as *mut std::os::raw::c_void, inputs as *const std::os::raw::c_void,
           (::std::mem::size_of::<std::os::raw::c_double>() as
                std::os::raw::c_ulong).wrapping_mul((*ann).inputs as std::os::raw::c_ulong));
    let mut h: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let act: genann_actfun = (*ann).activation_hidden;
    let acto: genann_actfun = (*ann).activation_output;
    /* Figure hidden layers, if any. */
    h = 0 as std::os::raw::c_int;
    while h < (*ann).hidden_layers {
        j = 0 as std::os::raw::c_int;
        while j < (*ann).hidden {
            let fresh0 = w;
            w = w.offset(1);
            let mut sum: std::os::raw::c_double = *fresh0 * -1.0f64;
            k = 0 as std::os::raw::c_int;
            while k <
                      (if h == 0 as std::os::raw::c_int {
                           (*ann).inputs
                       } else { (*ann).hidden }) {
                let fresh1 = w;
                w = w.offset(1);
                sum += *fresh1 * *i.offset(k as isize);
                k += 1
            }
            let fresh2 = o;
            o = o.offset(1);
            *fresh2 = act.expect("non-null function pointer")(sum);
            j += 1
        }
        i =
            i.offset(if h == 0 as std::os::raw::c_int {
                         (*ann).inputs
                     } else { (*ann).hidden } as isize);
        h += 1
    }
    let mut ret: *const std::os::raw::c_double = o;
    /* Figure output layer. */
    j = 0 as std::os::raw::c_int;
    while j < (*ann).outputs {
        let fresh3 = w;
        w = w.offset(1);
        let mut sum_0: std::os::raw::c_double = *fresh3 * -1.0f64;
        k = 0 as std::os::raw::c_int;
        while k <
                  (if (*ann).hidden_layers != 0 {
                       (*ann).hidden
                   } else { (*ann).inputs }) {
            let fresh4 = w;
            w = w.offset(1);
            sum_0 += *fresh4 * *i.offset(k as isize);
            k += 1
        }
        let fresh5 = o;
        o = o.offset(1);
        *fresh5 = acto.expect("non-null function pointer")(sum_0);
        j += 1
    }
    /* Sanity check that we used all weights and wrote all outputs. */
    if !(w.offset_from((*ann).weight) as std::os::raw::c_long ==
             (*ann).total_weights as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 11],
                                               &[std::os::raw::c_char; 11]>(b"genann_run\x00")).as_ptr(),
                     b"genann.c\x00" as *const u8 as *const std::os::raw::c_char,
                     225 as std::os::raw::c_int,
                     b"w - ann->weight == ann->total_weights\x00" as *const u8
                         as *const std::os::raw::c_char);
    } else { };
    if !(o.offset_from((*ann).output) as std::os::raw::c_long ==
             (*ann).total_neurons as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 11],
                                               &[std::os::raw::c_char; 11]>(b"genann_run\x00")).as_ptr(),
                     b"genann.c\x00" as *const u8 as *const std::os::raw::c_char,
                     226 as std::os::raw::c_int,
                     b"o - ann->output == ann->total_neurons\x00" as *const u8
                         as *const std::os::raw::c_char);
    } else { };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_train(mut ann: *const genann,
                                      mut inputs: *const std::os::raw::c_double,
                                      mut desired_outputs:
                                          *const std::os::raw::c_double,
                                      mut learning_rate: std::os::raw::c_double) {
    /* To begin with, we must run the network forward. */
    genann_run(ann, inputs);
    let mut h: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    /* First set the output layer deltas. */
    let mut o: *const std::os::raw::c_double =
        (*ann).output.offset((*ann).inputs as
                                 isize).offset(((*ann).hidden *
                                                    (*ann).hidden_layers) as
                                                   isize); /* First output. */
    let mut d: *mut std::os::raw::c_double =
        (*ann).delta.offset(((*ann).hidden * (*ann).hidden_layers) as
                                isize); /* First delta. */
    let mut t: *const std::os::raw::c_double =
        desired_outputs; /* First desired output. */
    if (*ann).activation_output ==
           Some(genann_act_linear as
                    unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double)
       {
        j = 0 as std::os::raw::c_int;
        while j < (*ann).outputs {
            let fresh6 = t;
            t = t.offset(1);
            let fresh7 = o;
            o = o.offset(1);
            let fresh8 = d;
            d = d.offset(1);
            *fresh8 = *fresh6 - *fresh7;
            j += 1
        }
    } else {
        j = 0 as std::os::raw::c_int;
        while j < (*ann).outputs {
            let fresh9 = d;
            d = d.offset(1);
            *fresh9 = (*t - *o) * *o * (1.0f64 - *o);
            o = o.offset(1);
            t = t.offset(1);
            j += 1
        }
    }
    /* Set output layer deltas. */
    /* Set hidden layer deltas, start on last layer and work backwards. */
    /* Note that loop is skipped in the case of hidden_layers == 0. */
    h = (*ann).hidden_layers - 1 as std::os::raw::c_int;
    while h >= 0 as std::os::raw::c_int {
        /* Find first output and delta in this layer. */
        let mut o_0: *const std::os::raw::c_double =
            (*ann).output.offset((*ann).inputs as
                                     isize).offset((h * (*ann).hidden) as
                                                       isize);
        let mut d_0: *mut std::os::raw::c_double =
            (*ann).delta.offset((h * (*ann).hidden) as isize);
        /* Find first delta in following layer (which may be hidden or output). */
        let dd: *const std::os::raw::c_double =
            (*ann).delta.offset(((h + 1 as std::os::raw::c_int) * (*ann).hidden) as
                                    isize);
        /* Find first weight in following layer (which may be hidden or output). */
        let ww: *const std::os::raw::c_double =
            (*ann).weight.offset((((*ann).inputs + 1 as std::os::raw::c_int) *
                                      (*ann).hidden) as
                                     isize).offset((((*ann).hidden +
                                                         1 as std::os::raw::c_int) *
                                                        (*ann).hidden * h) as
                                                       isize);
        j = 0 as std::os::raw::c_int;
        while j < (*ann).hidden {
            let mut delta: std::os::raw::c_double =
                0 as std::os::raw::c_int as std::os::raw::c_double;
            k = 0 as std::os::raw::c_int;
            while k <
                      (if h == (*ann).hidden_layers - 1 as std::os::raw::c_int {
                           (*ann).outputs
                       } else { (*ann).hidden }) {
                let forward_delta: std::os::raw::c_double = *dd.offset(k as isize);
                let windex: std::os::raw::c_int =
                    k * ((*ann).hidden + 1 as std::os::raw::c_int) +
                        (j + 1 as std::os::raw::c_int);
                let forward_weight: std::os::raw::c_double =
                    *ww.offset(windex as isize);
                delta += forward_delta * forward_weight;
                k += 1
            }
            *d_0 = *o_0 * (1.0f64 - *o_0) * delta;
            d_0 = d_0.offset(1);
            o_0 = o_0.offset(1);
            j += 1
        }
        h -= 1
    }
    /* Train the outputs. */
    /* Find first output delta. */
    let mut d_1: *const std::os::raw::c_double =
        (*ann).delta.offset(((*ann).hidden * (*ann).hidden_layers) as
                                isize); /* First output delta. */
    let mut w: *mut std::os::raw::c_double =
        (*ann).weight.offset((if (*ann).hidden_layers != 0 {
                                  (((*ann).inputs + 1 as std::os::raw::c_int) *
                                       (*ann).hidden) +
                                      ((*ann).hidden + 1 as std::os::raw::c_int) *
                                          (*ann).hidden *
                                          ((*ann).hidden_layers -
                                               1 as std::os::raw::c_int)
                              } else { 0 as std::os::raw::c_int }) as isize);
    let i: *const std::os::raw::c_double =
        (*ann).output.offset((if (*ann).hidden_layers != 0 {
                                  ((*ann).inputs) +
                                      (*ann).hidden *
                                          ((*ann).hidden_layers -
                                               1 as std::os::raw::c_int)
                              } else { 0 as std::os::raw::c_int }) as isize);
    j = 0 as std::os::raw::c_int;
    while j < (*ann).outputs {
        k = 0 as std::os::raw::c_int;
        while k <
                  (if (*ann).hidden_layers != 0 {
                       (*ann).hidden
                   } else { (*ann).inputs }) + 1 as std::os::raw::c_int {
            if k == 0 as std::os::raw::c_int {
                let fresh10 = w;
                w = w.offset(1);
                *fresh10 += *d_1 * learning_rate * -1.0f64
            } else {
                let fresh11 = w;
                w = w.offset(1);
                *fresh11 +=
                    *d_1 * learning_rate *
                        *i.offset((k - 1 as std::os::raw::c_int) as isize)
            }
            k += 1
        }
        d_1 = d_1.offset(1);
        j += 1
    }
    if !(w.offset_from((*ann).weight) as std::os::raw::c_long ==
             (*ann).total_weights as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                               &[std::os::raw::c_char; 13]>(b"genann_train\x00")).as_ptr(),
                     b"genann.c\x00" as *const u8 as *const std::os::raw::c_char,
                     318 as std::os::raw::c_int,
                     b"w - ann->weight == ann->total_weights\x00" as *const u8
                         as *const std::os::raw::c_char);
    } else { };
    /* Find first weight to first output delta. */
    /* Find first output in previous layer. */
    /* Set output layer weights. */
    /* Train the hidden layers. */
    h = (*ann).hidden_layers - 1 as std::os::raw::c_int;
    while h >= 0 as std::os::raw::c_int {
        /* Find first delta in this layer. */
        let mut d_2: *const std::os::raw::c_double =
            (*ann).delta.offset((h * (*ann).hidden) as isize);
        /* Find first input to this layer. */
        let mut i_0: *const std::os::raw::c_double =
            (*ann).output.offset((if h != 0 {
                                      ((*ann).inputs) +
                                          (*ann).hidden *
                                              (h - 1 as std::os::raw::c_int)
                                  } else { 0 as std::os::raw::c_int }) as isize);
        /* Find first weight to this layer. */
        let mut w_0: *mut std::os::raw::c_double =
            (*ann).weight.offset((if h != 0 {
                                      (((*ann).inputs + 1 as std::os::raw::c_int) *
                                           (*ann).hidden) +
                                          ((*ann).hidden + 1 as std::os::raw::c_int) *
                                              (*ann).hidden *
                                              (h - 1 as std::os::raw::c_int)
                                  } else { 0 as std::os::raw::c_int }) as isize);
        j = 0 as std::os::raw::c_int;
        while j < (*ann).hidden {
            k = 0 as std::os::raw::c_int;
            while k <
                      (if h == 0 as std::os::raw::c_int {
                           (*ann).inputs
                       } else { (*ann).hidden }) + 1 as std::os::raw::c_int {
                if k == 0 as std::os::raw::c_int {
                    let fresh12 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh12 += *d_2 * learning_rate * -1.0f64
                } else {
                    let fresh13 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh13 +=
                        *d_2 * learning_rate *
                            *i_0.offset((k - 1 as std::os::raw::c_int) as isize)
                }
                k += 1
            }
            d_2 = d_2.offset(1);
            j += 1
        }
        h -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn genann_write(mut ann: *const genann,
                                      mut out: *mut FILE) {
    fprintf(out, b"%d %d %d %d\x00" as *const u8 as *const std::os::raw::c_char,
            (*ann).inputs, (*ann).hidden_layers, (*ann).hidden,
            (*ann).outputs);
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*ann).total_weights {
        fprintf(out, b" %.20e\x00" as *const u8 as *const std::os::raw::c_char,
                *(*ann).weight.offset(i as isize));
        i += 1
    };
}

}

pub mod example1 {

extern "C" {
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub type genann_actfun
    =
    Option<unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: std::os::raw::c_int,
    pub hidden_layers: std::os::raw::c_int,
    pub hidden: std::os::raw::c_int,
    pub outputs: std::os::raw::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: std::os::raw::c_int,
    pub total_neurons: std::os::raw::c_int,
    pub weight: *mut std::os::raw::c_double,
    pub output: *mut std::os::raw::c_double,
    pub delta: *mut std::os::raw::c_double,
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 1.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train a small ANN to the XOR function using backpropagation.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    /* Input and expected out data for the XOR function. */
    let input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let output: [std::os::raw::c_double; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    let mut i: std::os::raw::c_int = 0;
    /* New network with 2 inputs,
     * 1 hidden layer of 2 neurons,
     * and 1 output. */
    let mut ann: *mut genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    /* Train on the four labeled data points many times. */
    i = 0 as std::os::raw::c_int;
    while i < 300 as std::os::raw::c_int {
        genann_train(ann, input[0 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(0 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[1 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(1 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[2 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(2 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[3 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(3 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        i += 1
    }
    /* Run the network and see what it predicts. */
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()));
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}

}

pub mod example2 {

extern "C" {
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn rand() -> std::os::raw::c_int;
    #[no_mangle]
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_copy;
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_randomize;
pub use crate::genann::genann_run;
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 2.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train a small ANN to the XOR function using random search.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    /* Input and expected out data for the XOR function. */
    let input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let output: [std::os::raw::c_double; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    let mut i: std::os::raw::c_int = 0;
    /* New network with 2 inputs,
     * 1 hidden layer of 2 neurons,
     * and 1 output. */
    let mut ann: *mut genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut err: std::os::raw::c_double = 0.;
    let mut last_err: std::os::raw::c_double = 1000 as std::os::raw::c_int as std::os::raw::c_double;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    loop  {
        count += 1;
        if count % 1000 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            /* We're stuck, start over. */
            genann_randomize(ann);
        }
        let mut save: *mut genann = genann_copy(ann);
        /* Take a random guess at the ANN weights. */
        i = 0 as std::os::raw::c_int;
        while i < (*ann).total_weights {
            *(*ann).weight.offset(i as isize) +=
                rand() as std::os::raw::c_double /
                    0x7fffffff as std::os::raw::c_int as std::os::raw::c_double - 0.5f64;
            i += 1
        }
        /* See how we did. */
        err = 0 as std::os::raw::c_int as std::os::raw::c_double;
        err +=
            pow(*genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()) -
                    output[0 as std::os::raw::c_int as usize], 2.0f64);
        err +=
            pow(*genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()) -
                    output[1 as std::os::raw::c_int as usize], 2.0f64);
        err +=
            pow(*genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()) -
                    output[2 as std::os::raw::c_int as usize], 2.0f64);
        err +=
            pow(*genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()) -
                    output[3 as std::os::raw::c_int as usize], 2.0f64);
        /* Keep these weights if they're an improvement. */
        if err < last_err {
            genann_free(save);
            last_err = err
        } else { genann_free(ann); ann = save }
        if !(err > 0.01f64) { break ; }
    }
    printf(b"Finished in %d loops.\n\x00" as *const u8 as *const std::os::raw::c_char,
           count);
    /* Run the network and see what it predicts. */
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()));
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}

}

pub mod example3 {

extern "C" {
    
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_read;
pub use crate::genann::genann_run;
pub use crate::genann::__sFILEX;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
#[no_mangle]
pub static mut save_name: *const std::os::raw::c_char =
    b"example/xor.ann\x00" as *const u8 as *const std::os::raw::c_char;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 3.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Load a saved ANN to solve the XOR function.\n\x00" as *const u8
               as *const std::os::raw::c_char);
    let mut saved: *mut FILE =
        fopen(save_name, b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if saved.is_null() {
        printf(b"Couldn\'t open file: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, save_name);
        exit(1 as std::os::raw::c_int);
    }
    let mut ann: *mut genann = genann_read(saved);
    fclose(saved);
    if ann.is_null() {
        printf(b"Error loading ANN from file: %s.\x00" as *const u8 as
                   *const std::os::raw::c_char, save_name);
        exit(1 as std::os::raw::c_int);
    }
    /* Input data for the XOR function. */
    let input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    /* Run the network and see what it predicts. */
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()));
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}

}

pub mod example4 {

extern "C" {
    
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn feof(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgets(_: *mut std::os::raw::c_char, _: std::os::raw::c_int, _: *mut FILE)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fseek(_: *mut FILE, _: std::os::raw::c_long, _: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(_: *const std::os::raw::c_char);
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn atof(_: *const std::os::raw::c_char) -> std::os::raw::c_double;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strtok(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub use crate::genann::__sFILEX;
pub type __int64_t = crate::example3::__int64_t;
pub type __darwin_off_t = crate::example3::__darwin_off_t;
pub type fpos_t = crate::example3::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example3::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example3::__sFILE;
pub type FILE = crate::example3::FILE;
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
/* This example is to illustrate how to use GENANN.
 * It is NOT an example of good machine learning techniques.
 */
#[no_mangle]
pub static mut iris_data: *const std::os::raw::c_char =
    b"example/iris.data\x00" as *const u8 as *const std::os::raw::c_char;
#[no_mangle]
pub static mut input: *mut std::os::raw::c_double =
    0 as *const std::os::raw::c_double as *mut std::os::raw::c_double;
#[no_mangle]
pub static mut class: *mut std::os::raw::c_double =
    0 as *const std::os::raw::c_double as *mut std::os::raw::c_double;
#[no_mangle]
pub static mut samples: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut class_names: [*const std::os::raw::c_char; 3] =
    [b"Iris-setosa\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-versicolor\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-virginica\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn load_data() {
    /* Load the iris data-set. */
    let mut in_0: *mut FILE =
        fopen(b"example/iris.data\x00" as *const u8 as *const std::os::raw::c_char,
              b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if in_0.is_null() {
        printf(b"Could not open file: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, iris_data);
        exit(1 as std::os::raw::c_int);
    }
    /* Loop through the data to get a count. */
    let mut line: [std::os::raw::c_char; 1024] = [0; 1024];
    while feof(in_0) == 0 &&
              !fgets(line.as_mut_ptr(), 1024 as std::os::raw::c_int, in_0).is_null() {
        samples += 1
    }
    fseek(in_0, 0 as std::os::raw::c_int as std::os::raw::c_long, 0 as std::os::raw::c_int);
    printf(b"Loading %d data points from %s\n\x00" as *const u8 as
               *const std::os::raw::c_char, samples, iris_data);
    /* Allocate memory for input and output data. */
    input =
        malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                    std::os::raw::c_ulong).wrapping_mul(samples as
                                                    std::os::raw::c_ulong).wrapping_mul(4
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_ulong))
            as *mut std::os::raw::c_double;
    class =
        malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                    std::os::raw::c_ulong).wrapping_mul(samples as
                                                    std::os::raw::c_ulong).wrapping_mul(3
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_ulong))
            as *mut std::os::raw::c_double;
    /* Read the file into our arrays. */
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < samples {
        let mut p: *mut std::os::raw::c_double =
            input.offset((i * 4 as std::os::raw::c_int) as isize);
        let mut c: *mut std::os::raw::c_double =
            class.offset((i * 3 as std::os::raw::c_int) as isize);
        let ref mut fresh0 = *c.offset(2 as std::os::raw::c_int as isize);
        *fresh0 = 0.0f64;
        let ref mut fresh1 = *c.offset(1 as std::os::raw::c_int as isize);
        *fresh1 = *fresh0;
        *c.offset(0 as std::os::raw::c_int as isize) = *fresh1;
        if fgets(line.as_mut_ptr(), 1024 as std::os::raw::c_int, in_0).is_null() {
            perror(b"fgets\x00" as *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        let mut split: *mut std::os::raw::c_char =
            strtok(line.as_mut_ptr(),
                   b",\x00" as *const u8 as *const std::os::raw::c_char);
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            *p.offset(j as isize) = atof(split);
            split =
                strtok(0 as *mut std::os::raw::c_char,
                       b",\x00" as *const u8 as *const std::os::raw::c_char);
            j += 1
        }
        *split.offset(strlen(split).wrapping_sub(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
            = 0 as std::os::raw::c_int as std::os::raw::c_char;
        if strcmp(split, class_names[0 as std::os::raw::c_int as usize]) ==
               0 as std::os::raw::c_int {
            *c.offset(0 as std::os::raw::c_int as isize) = 1.0f64
        } else if strcmp(split, class_names[1 as std::os::raw::c_int as usize]) ==
                      0 as std::os::raw::c_int {
            *c.offset(1 as std::os::raw::c_int as isize) = 1.0f64
        } else if strcmp(split, class_names[2 as std::os::raw::c_int as usize]) ==
                      0 as std::os::raw::c_int {
            *c.offset(2 as std::os::raw::c_int as isize) = 1.0f64
        } else {
            printf(b"Unknown class %s.\n\x00" as *const u8 as
                       *const std::os::raw::c_char, split);
            exit(1 as std::os::raw::c_int);
        }
        i += 1
        /* printf("Data point %d is %f %f %f %f  ->   %f %f %f\n", i, p[0], p[1], p[2], p[3], c[0], c[1], c[2]); */
    }
    fclose(in_0);
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 4.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train an ANN on the IRIS dataset using backpropagation.\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    /* Load the data from file. */
    load_data();
    /* 4 inputs.
     * 1 hidden layer(s) of 4 neurons.
     * 3 outputs (1 per class)
     */
    let mut ann: *mut genann =
        genann_init(4 as std::os::raw::c_int, 1 as std::os::raw::c_int, 4 as std::os::raw::c_int,
                    3 as std::os::raw::c_int);
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut loops: std::os::raw::c_int = 5000 as std::os::raw::c_int;
    /* Train the network with backpropagation. */
    printf(b"Training for %d loops over data.\n\x00" as *const u8 as
               *const std::os::raw::c_char, loops);
    i = 0 as std::os::raw::c_int;
    while i < loops {
        j = 0 as std::os::raw::c_int;
        while j < samples {
            genann_train(ann, input.offset((j * 4 as std::os::raw::c_int) as isize),
                         class.offset((j * 3 as std::os::raw::c_int) as isize),
                         0.01f64);
            j += 1
        }
        i += 1
        /* printf("%1.2f ", xor_score(ann)); */
    }
    let mut correct: std::os::raw::c_int = 0 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    while j < samples {
        let mut guess: *const std::os::raw::c_double =
            genann_run(ann, input.offset((j * 4 as std::os::raw::c_int) as isize));
        if *class.offset((j * 3 as std::os::raw::c_int + 0 as std::os::raw::c_int) as isize)
               == 1.0f64 {
            if *guess.offset(0 as std::os::raw::c_int as isize) >
                   *guess.offset(1 as std::os::raw::c_int as isize) &&
                   *guess.offset(0 as std::os::raw::c_int as isize) >
                       *guess.offset(2 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else if *class.offset((j * 3 as std::os::raw::c_int + 1 as std::os::raw::c_int) as
                                    isize) == 1.0f64 {
            if *guess.offset(1 as std::os::raw::c_int as isize) >
                   *guess.offset(0 as std::os::raw::c_int as isize) &&
                   *guess.offset(1 as std::os::raw::c_int as isize) >
                       *guess.offset(2 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else if *class.offset((j * 3 as std::os::raw::c_int + 2 as std::os::raw::c_int) as
                                    isize) == 1.0f64 {
            if *guess.offset(2 as std::os::raw::c_int as isize) >
                   *guess.offset(0 as std::os::raw::c_int as isize) &&
                   *guess.offset(2 as std::os::raw::c_int as isize) >
                       *guess.offset(1 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else {
            printf(b"Logic error.\n\x00" as *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        j += 1
    }
    printf(b"%d/%d correct (%0.1f%%).\n\x00" as *const u8 as
               *const std::os::raw::c_char, correct, samples,
           correct as std::os::raw::c_double / samples as std::os::raw::c_double * 100.0f64);
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}

}

pub mod test {

extern "C" {
    
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    
    
    
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn srand(_: std::os::raw::c_uint);
}
pub use crate::genann::genann_act_sigmoid;
pub use crate::genann::genann_act_sigmoid_cached;
pub use crate::genann::genann_act_threshold;
pub use crate::genann::genann_copy;
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_randomize;
pub use crate::genann::genann_read;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub use crate::genann::genann_write;
pub use crate::genann::__sFILEX;
pub type __int64_t = crate::example3::__int64_t;
pub type __darwin_clock_t = std::os::raw::c_ulong;
pub type __darwin_off_t = crate::example3::__darwin_off_t;
pub type fpos_t = crate::example3::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example3::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example3::__sFILE;
pub type FILE = crate::example3::FILE;
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
pub type clock_t = __darwin_clock_t;
static mut lfails: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut ltests: std::os::raw::c_int = 0 as std::os::raw::c_int;
/*
 * GENANN - Minimal C Artificial Neural Network
 *
 * Copyright (c) 2015, 2016 Lewis Van Winkle
 *
 * http://CodePlea.com
 *
 * This software is provided 'as-is', without any express or implied
 * warranty. In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgement in the product documentation would be
 *    appreciated but is not required.
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 * 3. This notice may not be removed or altered from any source distribution.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn basic() {
    let mut ann: *mut genann =
        genann_init(1 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    ltests += 1;
    if (*ann).total_weights != 2 as std::os::raw::c_int {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               37 as std::os::raw::c_int, (*ann).total_weights, 2 as std::os::raw::c_int);
    }
    let mut a: std::os::raw::c_double = 0.;
    a = 0 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               44 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 1 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               47 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 11 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               50 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               55 as std::os::raw::c_int, 0.5f64, *genann_run(ann, &mut a));
    }
    a = 10 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    ltests += 1;
    if fabs(1.0f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               60 as std::os::raw::c_int, 1.0f64, *genann_run(ann, &mut a));
    }
    a = -(10 as std::os::raw::c_int) as std::os::raw::c_double;
    ltests += 1;
    if fabs(0.0f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               63 as std::os::raw::c_int, 0.0f64, *genann_run(ann, &mut a));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn xor() {
    let mut ann: *mut genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    (*ann).activation_hidden =
        Some(genann_act_threshold as
                 unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double);
    (*ann).activation_output =
        Some(genann_act_threshold as
                 unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double);
    ltests += 1;
    if (*ann).total_weights != 9 as std::os::raw::c_int {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               74 as std::os::raw::c_int, (*ann).total_weights, 9 as std::os::raw::c_int);
    }
    /* First hidden. */
    *(*ann).weight.offset(0 as std::os::raw::c_int as isize) = 0.5f64;
    *(*ann).weight.offset(1 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(2 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    /* Second hidden. */
    *(*ann).weight.offset(3 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(4 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(5 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    /* Output. */
    *(*ann).weight.offset(6 as std::os::raw::c_int as isize) = 0.5f64;
    *(*ann).weight.offset(7 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as std::os::raw::c_double;
    *(*ann).weight.offset(8 as std::os::raw::c_int as isize) =
        -(1 as std::os::raw::c_int) as std::os::raw::c_double;
    let mut input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [std::os::raw::c_double; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               95 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               96 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               97 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               98 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn backprop() {
    let mut ann: *mut genann =
        genann_init(1 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut input: std::os::raw::c_double = 0.;
    let mut output: std::os::raw::c_double = 0.;
    input = 0.5f64;
    output = 1 as std::os::raw::c_int as std::os::raw::c_double;
    let mut first_try: std::os::raw::c_double = *genann_run(ann, &mut input);
    genann_train(ann, &mut input, &mut output, 0.5f64);
    let mut second_try: std::os::raw::c_double = *genann_run(ann, &mut input);
    ltests += 1;
    if !(fabs(first_try - output) > fabs(second_try - output)) {
        lfails += 1;
        printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               114 as std::os::raw::c_int);
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_and() {
    let mut input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [std::os::raw::c_double; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double];
    let mut ann: *mut genann =
        genann_init(2 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 50 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            genann_train(ann, input[j as usize].as_mut_ptr(),
                         output.as_mut_ptr().offset(j as isize), 0.8f64);
            j += 1
        }
        i += 1
    }
    (*ann).activation_output =
        Some(genann_act_threshold as
                 unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double);
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               135 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               136 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               137 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               138 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_or() {
    let mut input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [std::os::raw::c_double; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double];
    let mut ann: *mut genann =
        genann_init(2 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    genann_randomize(ann);
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 50 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            genann_train(ann, input[j as usize].as_mut_ptr(),
                         output.as_mut_ptr().offset(j as isize), 0.8f64);
            j += 1
        }
        i += 1
    }
    (*ann).activation_output =
        Some(genann_act_threshold as
                 unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double);
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               160 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               161 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               162 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               163 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_xor() {
    let mut input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let mut output: [std::os::raw::c_double; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    let mut ann: *mut genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 500 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            genann_train(ann, input[j as usize].as_mut_ptr(),
                         output.as_mut_ptr().offset(j as isize),
                         3 as std::os::raw::c_int as std::os::raw::c_double);
            j += 1
        }
        i += 1
        /* printf("%1.2f ", xor_score(ann)); */
    }
    (*ann).activation_output =
        Some(genann_act_threshold as
                 unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double);
    ltests += 1;
    if fabs(output[0 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[0 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               186 as std::os::raw::c_int, output[0 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[0 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[1 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[1 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               187 as std::os::raw::c_int, output[1 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[1 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[2 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[2 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               188 as std::os::raw::c_int, output[2 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[2 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    ltests += 1;
    if fabs(output[3 as std::os::raw::c_int as usize] -
                *genann_run(ann,
                            input[3 as std::os::raw::c_int as usize].as_mut_ptr())) >
           0.001f64 {
        lfails += 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               189 as std::os::raw::c_int, output[3 as std::os::raw::c_int as usize],
               *genann_run(ann,
                           input[3 as std::os::raw::c_int as usize].as_mut_ptr()));
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn persist() {
    let mut first: *mut genann =
        genann_init(1000 as std::os::raw::c_int, 5 as std::os::raw::c_int, 50 as std::os::raw::c_int,
                    10 as std::os::raw::c_int);
    let mut out: *mut FILE =
        fopen(b"persist.txt\x00" as *const u8 as *const std::os::raw::c_char,
              b"w\x00" as *const u8 as *const std::os::raw::c_char);
    genann_write(first, out);
    fclose(out);
    let mut in_0: *mut FILE =
        fopen(b"persist.txt\x00" as *const u8 as *const std::os::raw::c_char,
              b"r\x00" as *const u8 as *const std::os::raw::c_char);
    let mut second: *mut genann = genann_read(in_0);
    fclose(out);
    ltests += 1;
    if (*first).inputs != (*second).inputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               208 as std::os::raw::c_int, (*first).inputs, (*second).inputs);
    }
    ltests += 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               209 as std::os::raw::c_int, (*first).hidden_layers,
               (*second).hidden_layers);
    }
    ltests += 1;
    if (*first).hidden != (*second).hidden {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               210 as std::os::raw::c_int, (*first).hidden, (*second).hidden);
    }
    ltests += 1;
    if (*first).outputs != (*second).outputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               211 as std::os::raw::c_int, (*first).outputs, (*second).outputs);
    }
    ltests += 1;
    if (*first).total_weights != (*second).total_weights {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               212 as std::os::raw::c_int, (*first).total_weights,
               (*second).total_weights);
    }
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*first).total_weights {
        ltests += 1;
        if !(*(*first).weight.offset(i as isize) ==
                 *(*second).weight.offset(i as isize)) {
            lfails += 1;
            printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
                   b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                   216 as std::os::raw::c_int);
        }
        i += 1
    }
    genann_free(first);
    genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn copy() {
    let mut first: *mut genann =
        genann_init(1000 as std::os::raw::c_int, 5 as std::os::raw::c_int, 50 as std::os::raw::c_int,
                    10 as std::os::raw::c_int);
    let mut second: *mut genann = genann_copy(first);
    ltests += 1;
    if (*first).inputs != (*second).inputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               229 as std::os::raw::c_int, (*first).inputs, (*second).inputs);
    }
    ltests += 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               230 as std::os::raw::c_int, (*first).hidden_layers,
               (*second).hidden_layers);
    }
    ltests += 1;
    if (*first).hidden != (*second).hidden {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               231 as std::os::raw::c_int, (*first).hidden, (*second).hidden);
    }
    ltests += 1;
    if (*first).outputs != (*second).outputs {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               232 as std::os::raw::c_int, (*first).outputs, (*second).outputs);
    }
    ltests += 1;
    if (*first).total_weights != (*second).total_weights {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
               233 as std::os::raw::c_int, (*first).total_weights,
               (*second).total_weights);
    }
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*first).total_weights {
        ltests += 1;
        if fabs(*(*first).weight.offset(i as isize) -
                    *(*second).weight.offset(i as isize)) > 0.001f64 {
            lfails += 1;
            printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                   237 as std::os::raw::c_int, *(*first).weight.offset(i as isize),
                   *(*second).weight.offset(i as isize));
        }
        i += 1
    }
    genann_free(first);
    genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn sigmoid() {
    let mut i: std::os::raw::c_double = -(20 as std::os::raw::c_int) as std::os::raw::c_double;
    let max: std::os::raw::c_double = 20 as std::os::raw::c_int as std::os::raw::c_double;
    let d: std::os::raw::c_double = 0.0001f64;
    while i < max {
        ltests += 1;
        if fabs(genann_act_sigmoid(i) - genann_act_sigmoid_cached(i)) >
               0.001f64 {
            lfails += 1;
            printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                   251 as std::os::raw::c_int, genann_act_sigmoid(i),
                   genann_act_sigmoid_cached(i));
        }
        i += d
    };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN TEST SUITE\n\x00" as *const u8 as *const std::os::raw::c_char);
    srand(100 as std::os::raw::c_int as std::os::raw::c_uint);
    let ts: std::os::raw::c_int = ltests;
    let fs: std::os::raw::c_int = lfails;
    let start: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"basic\x00" as *const u8 as *const std::os::raw::c_char);
    basic();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts - (lfails - fs), lfails - fs,
           clock().wrapping_sub(start).wrapping_mul(1000 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong).wrapping_div(1000000
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_0: std::os::raw::c_int = ltests;
    let fs_0: std::os::raw::c_int = lfails;
    let start_0: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"xor\x00" as *const u8 as *const std::os::raw::c_char);
    xor();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_0 - (lfails - fs_0),
           lfails - fs_0,
           clock().wrapping_sub(start_0).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_1: std::os::raw::c_int = ltests;
    let fs_1: std::os::raw::c_int = lfails;
    let start_1: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"backprop\x00" as *const u8 as *const std::os::raw::c_char);
    backprop();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_1 - (lfails - fs_1),
           lfails - fs_1,
           clock().wrapping_sub(start_1).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_2: std::os::raw::c_int = ltests;
    let fs_2: std::os::raw::c_int = lfails;
    let start_2: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"train and\x00" as *const u8 as *const std::os::raw::c_char);
    train_and();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_2 - (lfails - fs_2),
           lfails - fs_2,
           clock().wrapping_sub(start_2).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_3: std::os::raw::c_int = ltests;
    let fs_3: std::os::raw::c_int = lfails;
    let start_3: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"train or\x00" as *const u8 as *const std::os::raw::c_char);
    train_or();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_3 - (lfails - fs_3),
           lfails - fs_3,
           clock().wrapping_sub(start_3).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_4: std::os::raw::c_int = ltests;
    let fs_4: std::os::raw::c_int = lfails;
    let start_4: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"train xor\x00" as *const u8 as *const std::os::raw::c_char);
    train_xor();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_4 - (lfails - fs_4),
           lfails - fs_4,
           clock().wrapping_sub(start_4).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_5: std::os::raw::c_int = ltests;
    let fs_5: std::os::raw::c_int = lfails;
    let start_5: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"persist\x00" as *const u8 as *const std::os::raw::c_char);
    persist();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_5 - (lfails - fs_5),
           lfails - fs_5,
           clock().wrapping_sub(start_5).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_6: std::os::raw::c_int = ltests;
    let fs_6: std::os::raw::c_int = lfails;
    let start_6: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"copy\x00" as *const u8 as *const std::os::raw::c_char);
    copy();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_6 - (lfails - fs_6),
           lfails - fs_6,
           clock().wrapping_sub(start_6).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_7: std::os::raw::c_int = ltests;
    let fs_7: std::os::raw::c_int = lfails;
    let start_7: clock_t = clock();
    printf(b"\t%-14s\x00" as *const u8 as *const std::os::raw::c_char,
           b"sigmoid\x00" as *const u8 as *const std::os::raw::c_char);
    sigmoid();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, ltests - ts_7 - (lfails - fs_7),
           lfails - fs_7,
           clock().wrapping_sub(start_7).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    if lfails == 0 as std::os::raw::c_int {
        printf(b"ALL TESTS PASSED (%d/%d)\n\x00" as *const u8 as
                   *const std::os::raw::c_char, ltests, ltests);
    } else {
        printf(b"SOME TESTS FAILED (%d/%d)\n\x00" as *const u8 as
                   *const std::os::raw::c_char, ltests - lfails, ltests);
    }
    return (lfails != 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}

}

// root re-exports so the generated harness's `translated::<entry>` resolves
pub use crate::genann::genann_act_linear;
pub use crate::genann::genann_act_sigmoid;
pub use crate::genann::genann_act_sigmoid_cached;
pub use crate::genann::genann_act_threshold;
pub use crate::genann::genann_copy;
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_randomize;
pub use crate::genann::genann_read;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub use crate::genann::genann_write;

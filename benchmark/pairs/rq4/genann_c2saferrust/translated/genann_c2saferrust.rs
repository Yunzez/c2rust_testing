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




use std::f64;

use std::fs::File;
use std::io::Write;

use std::alloc;

extern "C" {
    pub type __sFILEX;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn fscanf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn perror(_: *const std::os::raw::c_char);
    
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    fn __error() -> *mut std::os::raw::c_int;
    
    fn exp(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn rand() -> std::os::raw::c_int;
    
    fn free(_: *mut std::os::raw::c_void);
    
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
}
pub type __int64_t = crate::example3::__int64_t;
pub type __darwin_off_t = crate::example3::__darwin_off_t;
pub type fpos_t = crate::example3::fpos_t;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example4::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example4::__sFILE;
pub type FILE = crate::example3::FILE;
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example2::genann;

pub fn genann_act_sigmoid(a: f64) -> f64 {
    if a < -45.0 { return 0.0; }
    if a > 45.0 { return 1.0; }
    return 1.0 / (1.0 + (-a).exp());
}


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

pub extern "C" fn genann_act_threshold(a: f64) -> f64 {
    (a > 0.0) as i32 as f64
}


pub extern "C" fn genann_act_linear(a: f64) -> f64 {
    a
}


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

pub fn genann_copy(ann: &genann) -> *mut genann {
    let total_size = std::mem::size_of::<genann>() +
        (std::mem::size_of::<f64>() * (ann.total_weights + ann.total_neurons + (ann.total_neurons - ann.inputs)) as usize);

    let ret: *mut genann = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(total_size, std::mem::align_of::<genann>()).unwrap()) as *mut genann };
    if ret.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        std::ptr::copy_nonoverlapping(ann as *const genann, ret, 1);
        (*ret).weight = ((ret as *mut u8).add(std::mem::size_of::<genann>())) as *mut f64;
        (*ret).output = (*ret).weight.add(ann.total_weights as usize);
        (*ret).delta = (*ret).output.add(ann.total_neurons as usize);
    }
    ret
}


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

pub fn genann_free(ann: *mut genann) {
    // The weight, output, and delta pointers go to the same buffer.
    unsafe {
        // Convert the raw pointer to a Box to ensure proper memory management
        let _ = Box::from_raw(ann);
    }
}


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

pub fn genann_write(ann: &genann, out: &mut std::fs::File) {
    writeln!(out, "{} {} {} {}", ann.inputs, ann.hidden_layers, ann.hidden, ann.outputs).unwrap();
    for i in 0..ann.total_weights {
        writeln!(out, "{:.20e}", unsafe { *ann.weight.offset(i as isize) }).unwrap();
    }
}


}

pub mod example1 {



extern "C" {
    
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
fn main_0(argc: i32, argv: Vec<String>) -> i32 {
    println!("GENANN example 1.");
    println!("Train a small ANN to the XOR function using backpropagation.");
    
    let input: [[f64; 2]; 4] = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    
    let output: [f64; 4] = [0.0, 1.0, 1.0, 0.0];
    
    let mut ann: *mut genann;
    
    unsafe {
        ann = genann_init(2, 1, 2, 1);
        
        for _ in 0..300 {
            for i in 0..4 {
                genann_train(ann, &input[i][0] as *const f64, &output[i] as *const f64, 3.0);
            }
        }
        
        for i in 0..4 {
            let prediction = *genann_run(ann, &input[i][0] as *const f64);
            println!("Output for [{:.1}, {:.1}] is {:.1}.", input[i][0], input[i][1], prediction);
        }
        
        genann_free(ann);
    }
    
    0
}

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = args.len() as std::os::raw::c_int;

    // Call the main_0 function directly with the collected arguments
    ::std::process::exit(main_0(arg_count - 1, args) as i32);
}


}

pub mod example2 {

use std::ffi::CString;

extern "C" {
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn rand() -> std::os::raw::c_int;
    
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
        let mut save: *mut genann = genann_copy(&*ann);
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
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = args.len() as std::os::raw::c_int;

    // Convert Vec<String> to Vec<CString> and then to Vec<*mut c_char>
    let c_args: Vec<std::ffi::CString> = args.iter()
        .map(|arg| std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();

    // Create a mutable vector of raw pointers
    let mut raw_args: Vec<*mut std::os::raw::c_char> = c_args.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .collect();

    // Push a null pointer to the end of the arguments
    raw_args.push(std::ptr::null_mut());

    // Call the main_0 function within an unsafe block
    unsafe {
        ::std::process::exit(main_0(arg_count - 1, raw_args.as_mut_ptr()) as i32);
    }
}


}

pub mod example3 {


use std::ffi::CString;

extern "C" {
    
    
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
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

pub type genann = crate::example2::genann;

pub static mut save_name: *const std::os::raw::c_char =
    b"example/xor.ann\x00" as *const u8 as *const std::os::raw::c_char;
fn main_0(argc: i32, argv: Vec<*mut std::os::raw::c_char>) -> i32 {
    println!("GENANN example 3.");
    println!("Load a saved ANN to solve the XOR function.");
    
    let save_name_cstr = std::ffi::CString::new("example/xor.ann").unwrap();
    let saved = unsafe { fopen(save_name_cstr.as_ptr(), b"r\x00".as_ptr() as *const i8) };
    
    if saved.is_null() {
        eprintln!("Couldn't open file: {}.", save_name_cstr.to_str().unwrap());
        std::process::exit(1);
    }
    
    let ann = unsafe { genann_read(saved) };
    unsafe { fclose(saved) };
    
    if ann.is_null() {
        eprintln!("Error loading ANN from file: {}.", save_name_cstr.to_str().unwrap());
        std::process::exit(1);
    }
    
    /* Input data for the XOR function. */
    let input: [[f64; 2]; 4] = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    
    /* Run the network and see what it predicts. */
    for i in 0..4 {
        let output = unsafe { *genann_run(ann, input[i].as_ptr()) };
        println!("Output for [{:.1}, {:.1}] is {:.1}.", input[i][0], input[i][1], output);
    }
    
    unsafe { genann_free(ann) };
    0
}

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = (args.len() - 1) as std::os::raw::c_int;
    let arg_ptrs: Vec<std::ffi::CString> = args.iter()
        .map(|arg| ::std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    
    let mut raw_ptrs: Vec<*mut std::os::raw::c_char> = arg_ptrs.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .collect();
    
    raw_ptrs.push(std::ptr::null_mut());

    ::std::process::exit(main_0(arg_count, raw_ptrs) as i32);
}


}

pub mod example4 {



use std::ffi::CString;
use std::fs::File;
use std::io::{self, BufRead};

extern "C" {
    
    
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    
    fn feof(_: *mut FILE) -> std::os::raw::c_int;
    
    fn fgets(_: *mut std::os::raw::c_char, _: std::os::raw::c_int, _: *mut FILE)
     -> *mut std::os::raw::c_char;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn fseek(_: *mut FILE, _: std::os::raw::c_long, _: std::os::raw::c_int) -> std::os::raw::c_int;
    
    fn perror(_: *const std::os::raw::c_char);
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn atof(_: *const std::os::raw::c_char) -> std::os::raw::c_double;
    
    fn exit(_: std::os::raw::c_int) -> !;
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    
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

pub type genann = crate::example2::genann;
/* This example is to illustrate how to use GENANN.
 * It is NOT an example of good machine learning techniques.
 */

pub static mut iris_data: *const std::os::raw::c_char =
    b"example/iris.data\x00" as *const u8 as *const std::os::raw::c_char;

pub static mut input: *mut std::os::raw::c_double =
    0 as *const std::os::raw::c_double as *mut std::os::raw::c_double;

pub static mut class: *mut std::os::raw::c_double =
    0 as *const std::os::raw::c_double as *mut std::os::raw::c_double;

pub static mut samples: std::os::raw::c_int = 0;

pub static mut class_names: [*const std::os::raw::c_char; 3] =
    [b"Iris-setosa\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-versicolor\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-virginica\x00" as *const u8 as *const std::os::raw::c_char];

pub fn load_data() {
    /* Load the iris data-set. */
    let iris_data_path = std::ffi::CString::new("example/iris.data").unwrap();
    let in_0 = std::fs::File::open(iris_data_path.to_str().unwrap()).expect("Could not open file");
    let reader = std::io::BufReader::new(in_0);
    
    let mut local_samples = 0;
    let mut local_input: Vec<f64> = Vec::new();
    let mut local_class: Vec<f64> = Vec::new();

    /* Loop through the data to get a count. */
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        local_samples += 1;
        let mut values: Vec<&str> = line.split(',').collect();
        
        // Parse input values
        let input_values: Vec<f64> = values[..4].iter().map(|&s| s.parse().unwrap()).collect();
        local_input.extend(input_values);
        
        // Initialize class vector
        let mut class_vector = vec![0.0; 3];
        match values[4] {
            "class1" => class_vector[0] = 1.0,
            "class2" => class_vector[1] = 1.0,
            "class3" => class_vector[2] = 1.0,
            _ => panic!("Unknown class {}", values[4]),
        }
        local_class.extend(class_vector);
    }

    println!("Loading {} data points from {}", local_samples, iris_data_path.to_str().unwrap());
    
    // Store the input and class data in global variables
    unsafe {
        input = local_input.as_mut_ptr();
        class = local_class.as_mut_ptr();
        samples = local_samples;
    }
}

fn main_0(argc: i32, argv: Vec<String>) -> i32 {
    println!("GENANN example 4.");
    println!("Train an ANN on the IRIS dataset using backpropagation.");
    
    // Load the data from file.
    load_data();
    
    // 4 inputs, 1 hidden layer of 4 neurons, 3 outputs (1 per class)
    let mut ann = unsafe { genann_init(4, 1, 4, 3) };
    let loops = 5000;
    
    // Train the network with backpropagation.
    println!("Training for {} loops over data.", loops);
    
    for _ in 0..loops {
        for j in 0..unsafe { samples } {
            let input_slice = unsafe { std::slice::from_raw_parts(input.offset((j * 4) as isize), 4) };
            let class_slice = unsafe { std::slice::from_raw_parts(class.offset((j * 3) as isize), 3) };
            unsafe { genann_train(ann, input_slice.as_ptr(), class_slice.as_ptr(), 0.01) };
        }
    }
    
    let mut correct = 0;
    
    for j in 0..unsafe { samples } {
        let guess = unsafe { std::slice::from_raw_parts(genann_run(ann, input.offset((j * 4) as isize)), 3) };
        
        if unsafe { *class.offset((j * 3) as isize) } == 1.0 {
            if guess[0] > guess[1] && guess[0] > guess[2] {
                correct += 1;
            }
        } else if unsafe { *class.offset((j * 3 + 1) as isize) } == 1.0 {
            if guess[1] > guess[0] && guess[1] > guess[2] {
                correct += 1;
            }
        } else if unsafe { *class.offset((j * 3 + 2) as isize) } == 1.0 {
            if guess[2] > guess[0] && guess[2] > guess[1] {
                correct += 1;
            }
        } else {
            eprintln!("Logic error.");
            std::process::exit(1);
        }
    }
    
    println!("{}/{} correct ({:.1}%).", correct, unsafe { samples }, correct as f64 / unsafe { samples } as f64 * 100.0);
    unsafe { genann_free(ann) };
    0
}

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let argc = args.len() as std::os::raw::c_int;

    // Convert Vec<String> to Vec<CString>
    let argv: Vec<CString> = args.iter()
        .map(|arg| CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();

    // Create a vector of raw pointers for the C function
    let argv_ptr: Vec<*mut std::os::raw::c_char> = argv.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .chain(std::iter::once(std::ptr::null_mut()))
        .collect();

    // Call the C function with the original Vec<String>
    ::std::process::exit(main_0(argc, args) as i32)
}


}

pub mod test {








use std::ffi::CString;

use std::println;

use std::ptr;

extern "C" {
    
    
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    
    
    
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn clock() -> clock_t;
    
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

pub type __sbuf = crate::example4::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example4::__sFILE;
pub type FILE = crate::example3::FILE;
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example2::genann;
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

pub fn basic() {
    unsafe {
        let mut ann = genann_init(1, 0, 0, 1);
        ltests += 1;
        if (*ann).total_weights != 2 {
            lfails += 1;
            println!("test.c:37 ({} != {})", (*ann).total_weights, 2);
        }
        
        let mut a: f64 = 0.0;
        *(*ann).weight.offset(0) = 0.0;
        *(*ann).weight.offset(1) = 0.0;
        
        ltests += 1;
        if (0.5 - *genann_run(ann, &a)).abs() > 0.001 {
            lfails += 1;
            println!("test.c:44 ({} != {})", 0.5, *genann_run(ann, &a));
        }
        
        a = 1.0;
        ltests += 1;
        if (0.5 - *genann_run(ann, &a)).abs() > 0.001 {
            lfails += 1;
            println!("test.c:47 ({} != {})", 0.5, *genann_run(ann, &a));
        }
        
        a = 11.0;
        ltests += 1;
        if (0.5 - *genann_run(ann, &a)).abs() > 0.001 {
            lfails += 1;
            println!("test.c:50 ({} != {})", 0.5, *genann_run(ann, &a));
        }
        
        a = 1.0;
        *(*ann).weight.offset(0) = 1.0;
        *(*ann).weight.offset(1) = 1.0;
        
        ltests += 1;
        if (0.5 - *genann_run(ann, &a)).abs() > 0.001 {
            lfails += 1;
            println!("test.c:55 ({} != {})", 0.5, *genann_run(ann, &a));
        }
        
        a = 10.0;
        *(*ann).weight.offset(0) = 1.0;
        *(*ann).weight.offset(1) = 1.0;
        
        ltests += 1;
        if (1.0 - *genann_run(ann, &a)).abs() > 0.001 {
            lfails += 1;
            println!("test.c:60 ({} != {})", 1.0, *genann_run(ann, &a));
        }
        
        a = -10.0;
        ltests += 1;
        if (0.0 - *genann_run(ann, &a)).abs() > 0.001 {
            lfails += 1;
            println!("test.c:63 ({} != {})", 0.0, *genann_run(ann, &a));
        }
        
        genann_free(ann);
    }
}


pub fn xor() {
    unsafe {
        let mut ann = genann_init(2, 1, 2, 1);
        (*ann).activation_hidden = Some(genann_act_threshold);
        (*ann).activation_output = Some(genann_act_threshold);
        
        ltests += 1;
        if (*ann).total_weights != 9 {
            lfails += 1;
            println!("test.c:74 ({:?} != 9)", (*ann).total_weights);
        }
        
        // First hidden.
        *(*ann).weight.offset(0) = 0.5;
        *(*ann).weight.offset(1) = 1.0;
        *(*ann).weight.offset(2) = 1.0;
        
        // Second hidden.
        *(*ann).weight.offset(3) = 1.0;
        *(*ann).weight.offset(4) = 1.0;
        *(*ann).weight.offset(5) = 1.0;
        
        // Output.
        *(*ann).weight.offset(6) = 0.5;
        *(*ann).weight.offset(7) = 1.0;
        *(*ann).weight.offset(8) = -1.0;
        
        let input: [[f64; 2]; 4] = [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 1.0],
        ];
        
        let output: [f64; 4] = [0.0, 1.0, 1.0, 0.0];
        
        for i in 0..4 {
            ltests += 1;
            let result = *genann_run(ann, input[i].as_ptr());
            if (output[i] - result).abs() > 0.001 {
                lfails += 1;
                println!("test.c:{} ({:?} != {:?})", 95 + i, output[i], result);
            }
        }
        
        genann_free(ann);
    }
}


pub fn backprop() {
    unsafe {
        let mut ann = genann_init(1, 0, 0, 1);
        let input: f64 = 0.5;
        let output: f64 = 1.0;
        
        let first_try = *genann_run(ann, &input);
        genann_train(ann, &input, &output, 0.5);
        let second_try = *genann_run(ann, &input);
        
        ltests += 1;
        if !(f64::abs(first_try - output) > f64::abs(second_try - output)) {
            lfails += 1;
            println!("{}:{} error", "test.c", 114);
        }
        genann_free(ann);
    }
}


pub fn train_and() {
    let input: [[f64; 2]; 4] = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    let mut output: [f64; 4] = [
        0.0,
        0.0,
        0.0,
        1.0,
    ];
    
    let ann = unsafe { genann_init(2, 0, 0, 1) };
    
    for _ in 0..50 {
        for j in 0..4 {
            unsafe {
                genann_train(ann, input[j].as_ptr(), &mut output[j], 0.8);
            }
        }
    }
    
    unsafe {
        (*ann).activation_output = Some(genann_act_threshold);
    }
    
    unsafe {
        ltests += 1;
    }

    for (i, &expected) in output.iter().enumerate() {
        let result = unsafe { *genann_run(ann, input[i].as_ptr()) };
        
        unsafe {
            ltests += 1;
        }
        
        if (expected - result).abs() > 0.001 {
            unsafe {
                lfails += 1;
            }
            println!("test.c:{} ({} != {})", 135 + i as i32, expected, result);
        }
    }
    
    unsafe {
        genann_free(ann);
    }
}


pub fn train_or() {
    let input: [[f64; 2]; 4] = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    let output: [f64; 4] = [
        0.0,
        1.0,
        1.0,
        1.0,
    ];
    
    let mut ann: *mut genann;
    
    unsafe {
        ann = genann_init(2, 0, 0, 1);
        genann_randomize(ann);
        
        for _ in 0..50 {
            for j in 0..4 {
                genann_train(ann, input[j].as_ptr(), &output[j], 0.8);
            }
        }
        
        (*ann).activation_output = Some(genann_act_threshold);
    }
    
    unsafe {
        ltests += 1;
        if (output[0] - *genann_run(ann, input[0].as_ptr())).abs() > 0.001 {
            lfails += 1;
            println!("test.c:160 ({:?} != {:?})", output[0], *genann_run(ann, input[0].as_ptr()));
        }
        
        ltests += 1;
        if (output[1] - *genann_run(ann, input[1].as_ptr())).abs() > 0.001 {
            lfails += 1;
            println!("test.c:161 ({:?} != {:?})", output[1], *genann_run(ann, input[1].as_ptr()));
        }
        
        ltests += 1;
        if (output[2] - *genann_run(ann, input[2].as_ptr())).abs() > 0.001 {
            lfails += 1;
            println!("test.c:162 ({:?} != {:?})", output[2], *genann_run(ann, input[2].as_ptr()));
        }
        
        ltests += 1;
        if (output[3] - *genann_run(ann, input[3].as_ptr())).abs() > 0.001 {
            lfails += 1;
            println!("test.c:163 ({:?} != {:?})", output[3], *genann_run(ann, input[3].as_ptr()));
        }
        
        genann_free(ann);
    }
}


pub fn train_xor() {
    let input: [[f64; 2]; 4] = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    let output: [f64; 4] = [
        0.0,
        1.0,
        1.0,
        0.0,
    ];
    
    unsafe {
        let mut ann = genann_init(2, 1, 2, 1);
        
        for _ in 0..500 {
            for j in 0..4 {
                genann_train(ann, input[j].as_ptr(), &output[j], 3.0);
            }
        }
        
        (*ann).activation_output = Some(genann_act_threshold);
        ltests += 1;

        for (i, &expected) in output.iter().enumerate() {
            ltests += 1;
            let result = *genann_run(ann, input[i].as_ptr());
            if (expected - result).abs() > 0.001 {
                lfails += 1;
                println!("test.c:{} ({} != {})", 186 + i, expected, result);
            }
        }
        
        genann_free(ann);
    }
}


pub unsafe extern "C" fn persist() {
    let mut first: *mut genann =
        genann_init(1000 as std::os::raw::c_int, 5 as std::os::raw::c_int, 50 as std::os::raw::c_int,
                    10 as std::os::raw::c_int);
    let mut out: *mut FILE =
        fopen(b"persist.txt\x00" as *const u8 as *const std::os::raw::c_char,
              b"w\x00" as *const u8 as *const std::os::raw::c_char);
    genann_write(&*first, &mut *(out as *mut std::os::raw::c_void as *mut std::fs::File));
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

pub fn copy() {
    unsafe {
        let first = genann_init(1000, 5, 50, 10);
        let first_ref = &*first; // Dereference to get a reference
        let second = genann_copy(first_ref);
        
        ltests += 1;
        if first_ref.inputs != (*second).inputs {
            lfails += 1;
            println!("test.c:229 ({} != {})", first_ref.inputs, (*second).inputs);
        }
        
        ltests += 1;
        if first_ref.hidden_layers != (*second).hidden_layers {
            lfails += 1;
            println!("test.c:230 ({} != {})", first_ref.hidden_layers, (*second).hidden_layers);
        }
        
        ltests += 1;
        if first_ref.hidden != (*second).hidden {
            lfails += 1;
            println!("test.c:231 ({} != {})", first_ref.hidden, (*second).hidden);
        }
        
        ltests += 1;
        if first_ref.outputs != (*second).outputs {
            lfails += 1;
            println!("test.c:232 ({} != {})", first_ref.outputs, (*second).outputs);
        }
        
        ltests += 1;
        if first_ref.total_weights != (*second).total_weights {
            lfails += 1;
            println!("test.c:233 ({} != {})", first_ref.total_weights, (*second).total_weights);
        }
        
        for i in 0..first_ref.total_weights {
            ltests += 1;
            if ( *(*first).weight.offset(i as isize) - *(*second).weight.offset(i as isize) ).abs() > 0.001 {
                lfails += 1;
                println!("test.c:237 ({} != {})", *(*first).weight.offset(i as isize), *(*second).weight.offset(i as isize));
            }
        }
        
        genann_free(first);
        genann_free(second);
    }
}


pub fn sigmoid() {
    let mut i: f64 = -20.0;
    let max: f64 = 20.0;
    let d: f64 = 0.0001;
    let mut local_ltests = 0;
    let mut local_lfails = 0;

    while i < max {
        local_ltests += 1;
        unsafe {
            if (genann_act_sigmoid(i) - genann_act_sigmoid_cached(i)).abs() > 0.001 {
                local_lfails += 1;
                println!("test.c:251 ({:?} != {:?})", genann_act_sigmoid(i), genann_act_sigmoid_cached(i));
            }
        }
        i += d;
    }
}

unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    use std::time::Instant;

println!("GENANN TEST SUITE");
let ts = ltests;
let fs = lfails;
let start = Instant::now();
println!("{:<14}", "basic");
basic();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts - (lfails - fs), lfails - fs,
         start.elapsed().as_millis());

let ts_0 = ltests;
let fs_0 = lfails;
let start_0 = Instant::now();
println!("{:<14}", "xor");
xor();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_0 - (lfails - fs_0), lfails - fs_0,
         start_0.elapsed().as_millis());

let ts_1 = ltests;
let fs_1 = lfails;
let start_1 = Instant::now();
println!("{:<14}", "backprop");
backprop();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_1 - (lfails - fs_1), lfails - fs_1,
         start_1.elapsed().as_millis());

let ts_2 = ltests;
let fs_2 = lfails;
let start_2 = Instant::now();
println!("{:<14}", "train and");
train_and();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_2 - (lfails - fs_2), lfails - fs_2,
         start_2.elapsed().as_millis());

let ts_3 = ltests;
let fs_3 = lfails;
let start_3 = Instant::now();
println!("{:<14}", "train or");
train_or();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_3 - (lfails - fs_3), lfails - fs_3,
         start_3.elapsed().as_millis());

let ts_4 = ltests;
let fs_4 = lfails;
let start_4 = Instant::now();
println!("{:<14}", "train xor");
train_xor();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_4 - (lfails - fs_4), lfails - fs_4,
         start_4.elapsed().as_millis());

let ts_5 = ltests;
let fs_5 = lfails;
let start_5 = Instant::now();
println!("{:<14}", "persist");
persist();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_5 - (lfails - fs_5), lfails - fs_5,
         start_5.elapsed().as_millis());

let ts_6 = ltests;
let fs_6 = lfails;
let start_6 = Instant::now();
println!("{:<14}", "copy");
copy();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_6 - (lfails - fs_6), lfails - fs_6,
         start_6.elapsed().as_millis());

let ts_7 = ltests;
let fs_7 = lfails;
let start_7 = Instant::now();
println!("{:<14}", "sigmoid");
sigmoid();
println!("pass:{:2}   fail:{:2}   {:4}ms", ltests - ts_7 - (lfails - fs_7), lfails - fs_7,
         start_7.elapsed().as_millis());

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
    let args: Vec<String> = ::std::env::args().collect();
    let argc = args.len() as std::os::raw::c_int;
    let argv: Vec<std::ffi::CString> = args.iter()
        .map(|arg| ::std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();

    let mut argv_ptr: Vec<*mut std::os::raw::c_char> = argv.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .chain(std::iter::once(std::ptr::null_mut()))
        .collect();

    let argv_ptr_mut: *mut *mut std::os::raw::c_char = argv_ptr.as_mut_ptr();
    
    unsafe {
        ::std::process::exit(main_0(argc, argv_ptr_mut) as i32)
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

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

pub mod src {
pub mod genann {
use ::libc;
extern "C" {
    
    
    
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn rand() -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor4 { dummy: () }
pub type _IO_lock_t = ();
pub type FILE = crate::src::example3::_IO_FILE;
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor5 { dummy: () }
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid(mut a: libc::c_double) -> libc::c_double {
    if a < -45.0f64 {
        return 0 as libc::c_int as libc::c_double;
    }
    if a > 45.0f64 {
        return 1 as libc::c_int as libc::c_double;
    }
    return 1.0f64 / (1 as libc::c_int as libc::c_double + exp(-a));
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid_cached(
    mut a: libc::c_double,
) -> libc::c_double {
    let min = -15.0f64;
    let max = 15.0f64;
    static mut interval: libc::c_double = 0.;
    static mut initialized: libc::c_int = 0 as libc::c_int;
    static mut lookup: [libc::c_double; 4096] = [0.; 4096];
    if initialized == 0 {
        interval= (max - min) / 4096 as libc::c_int as libc::c_double;
        let mut i: libc::c_int = 0;
        i= 0 as libc::c_int;
        while i < 4096 as libc::c_int {
            lookup[i
                as usize]= genann_act_sigmoid(min + interval * i as libc::c_double);
            i+= 1;
        }
        initialized= 1 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0;
    i_0= ((a - min) / interval + 0.5f64) as libc::c_int;
    if i_0 <= 0 as libc::c_int {
        return lookup[0 as libc::c_int as usize];
    }
    if i_0 >= 4096 as libc::c_int {
        return lookup[(4096 as libc::c_int - 1 as libc::c_int) as usize];
    }
    return lookup[i_0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_threshold(mut a: libc::c_double) -> libc::c_double {
    return (a > 0 as libc::c_int as libc::c_double) as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_linear(mut a: libc::c_double) -> libc::c_double {
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn genann_init(
    mut inputs: libc::c_int,
    mut hidden_layers: libc::c_int,
    mut hidden: libc::c_int,
    mut outputs: libc::c_int,
) -> *mut /* owning */ crate::src::example1::genann {
    if hidden_layers < 0 as libc::c_int {
        return 0 as *mut crate::src::example1::genann;
    }
    if inputs < 1 as libc::c_int {
        return 0 as *mut crate::src::example1::genann;
    }
    if outputs < 1 as libc::c_int {
        return 0 as *mut crate::src::example1::genann;
    }
    if hidden_layers > 0 as libc::c_int && hidden < 1 as libc::c_int {
        return 0 as *mut crate::src::example1::genann;
    }
    let hidden_weights = if hidden_layers != 0 {
        (inputs + 1 as libc::c_int) * hidden
            + (hidden_layers - 1 as libc::c_int) * (hidden + 1 as libc::c_int) * hidden
    } else {
        0 as libc::c_int
    };
    let output_weights = (if hidden_layers != 0 {
        hidden + 1 as libc::c_int
    } else {
        inputs + 1 as libc::c_int
    }) * outputs;
    let total_weights = hidden_weights + output_weights;
    let total_neurons = inputs + hidden * hidden_layers + outputs;
    let size = (::std::mem::size_of::<crate::src::example1::genann>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(
                    (total_weights + total_neurons + (total_neurons - inputs))
                        as libc::c_ulong,
                ),
        ) as libc::c_int;
    let mut ret = malloc(size as libc::c_ulong) as *mut crate::src::example1::genann;
    if ret.is_null() {();
        return 0 as *mut crate::src::example1::genann;
    }
    (*ret).inputs= inputs;
    (*ret).hidden_layers= hidden_layers;
    (*ret).hidden= hidden;
    (*ret).outputs= outputs;
    (*ret).total_weights= total_weights;
    (*ret).total_neurons= total_neurons;
    (*ret).weight= (ret as *mut libc::c_char)
        .offset(::std::mem::size_of::<crate::src::example1::genann>() as libc::c_ulong as isize)
        as *mut libc::c_double;
    (*ret).output= (*ret).weight.offset((*ret).total_weights as isize);
    (*ret).delta= (*ret).output.offset((*ret).total_neurons as isize);
    genann_randomize(ret.as_mut());
    (*ret).activation_hidden= Some(
        genann_act_sigmoid_cached
            as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    (*ret).activation_output= Some(
        genann_act_sigmoid_cached
            as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_read(mut in_0: *mut FILE) -> *mut /* owning */ crate::src::example1::genann {
    let mut inputs: libc::c_int = 0;
    let mut hidden_layers: libc::c_int = 0;
    let mut hidden: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    rc= fscanf(
        in_0,
        b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
        core::ptr::addr_of_mut!(inputs) as *mut libc::c_int,
        core::ptr::addr_of_mut!(hidden_layers) as *mut libc::c_int,
        core::ptr::addr_of_mut!(hidden) as *mut libc::c_int,
        core::ptr::addr_of_mut!(outputs) as *mut libc::c_int,
    );
    if rc < 4 as libc::c_int || *__errno_location() != 0 as libc::c_int {
        perror(b"fscanf\0" as *const u8 as *const libc::c_char);
        return 0 as *mut crate::src::example1::genann;
    }
    let mut ann = genann_init(inputs, hidden_layers, hidden, outputs);
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < (*ann).total_weights {
        *__errno_location() = 0 as libc::c_int;
        rc= fscanf(
            in_0,
            b" %le\0" as *const u8 as *const libc::c_char,
            (*ann).weight.offset(i as isize),
        );
        if rc < 1 as libc::c_int || *__errno_location() != 0 as libc::c_int {
            perror(b"fscanf\0" as *const u8 as *const libc::c_char);
            genann_free(ann);
            return 0 as *mut crate::src::example1::genann;
        }
        i+= 1;
    }
    return ann;
}
#[no_mangle]
pub unsafe extern "C" fn genann_copy(mut ann: *const crate::src::example1::genann) -> *mut /* owning */ crate::src::example1::genann {
    let size = (::std::mem::size_of::<crate::src::example1::genann>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(
                    ((*ann).total_weights + (*ann).total_neurons
                        + ((*ann).total_neurons - (*ann).inputs)) as libc::c_ulong,
                ),
        ) as libc::c_int;
    let mut ret = malloc(size as libc::c_ulong) as *mut crate::src::example1::genann;
    if ret.is_null() {();
        return 0 as *mut crate::src::example1::genann;
    }
    memcpy(ret as *mut libc::c_void, ann as *const libc::c_void, size as libc::c_ulong);
    (*ret).weight= (ret as *mut libc::c_char)
        .offset(::std::mem::size_of::<crate::src::example1::genann>() as libc::c_ulong as isize)
        as *mut libc::c_double;
    (*ret).output= (*ret).weight.offset((*ret).total_weights as isize);
    (*ret).delta= (*ret).output.offset((*ret).total_neurons as isize);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_randomize(mut ann: Option<&mut crate::src::example1::genann>) {
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < (*ann.as_deref().unwrap()).total_weights {
        let mut r = rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double;
        *(*ann.as_deref().unwrap()).weight.offset(i as isize) = r - 0.5f64;
        i+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn genann_free(mut ann: *mut /* owning */ crate::src::example1::genann) {
    free(ann as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn genann_run(
    mut ann: Option<&mut crate::src::example1::genann>,
    mut inputs: *const libc::c_double,
) -> *const libc::c_double {
    let mut w: *const libc::c_double = (*ann.as_deref().unwrap()).weight;
    let mut o = (*ann.as_deref().unwrap()).output.offset((*ann.as_deref().unwrap()).inputs as isize);
    let mut i: *const libc::c_double = (*ann.as_deref().unwrap()).output;
    memcpy(
        (*ann.as_deref().unwrap()).output as *mut libc::c_void,
        inputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*ann.as_deref().unwrap()).inputs as libc::c_ulong),
    );
    let mut h: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let act: genann_actfun = (*ann.as_deref().unwrap()).activation_hidden;
    let acto: genann_actfun = (*ann.as_deref().unwrap()).activation_output;
    h= 0 as libc::c_int;
    while h < (*ann.as_deref().unwrap()).hidden_layers {
        j= 0 as libc::c_int;
        while j < (*ann.as_deref().unwrap()).hidden {
            let fresh8 = w;
            w= w.offset(1);
            let mut sum = (*fresh8) * -1.0f64;
            k= 0 as libc::c_int;
            while k < (if h == 0 as libc::c_int { (*ann.as_deref().unwrap()).inputs } else { (*ann.as_deref().unwrap()).hidden })
            {
                let fresh9 = w;
                w= w.offset(1);
                sum+= (*fresh9) * *i.offset(k as isize);
                k+= 1;
            }
            let fresh10 = o;
            o= o.offset(1);
            *fresh10= act.expect("non-null function pointer")(sum);
            j+= 1;
        }
        i= i
            .offset(
                (if h == 0 as libc::c_int { (*ann.as_deref().unwrap()).inputs } else { (*ann.as_deref().unwrap()).hidden })
                    as isize,
            );
        h+= 1;
    }
    let mut ret: *const libc::c_double = o;
    j= 0 as libc::c_int;
    while j < (*ann.as_deref().unwrap()).outputs {
        let fresh11 = w;
        w= w.offset(1);
        let mut sum_0 = (*fresh11) * -1.0f64;
        k= 0 as libc::c_int;
        while k < (if (*ann.as_deref().unwrap()).hidden_layers != 0 { (*ann.as_deref().unwrap()).hidden } else { (*ann.as_deref().unwrap()).inputs })
        {
            let fresh12 = w;
            w= w.offset(1);
            sum_0+= (*fresh12) * *i.offset(k as isize);
            k+= 1;
        }
        let fresh13 = o;
        o= o.offset(1);
        *fresh13= acto.expect("non-null function pointer")(sum_0);
        j+= 1;
    }
    if w.offset_from((*ann.as_deref().unwrap()).weight) as libc::c_long
        == (*ann.as_deref().unwrap()).total_weights as libc::c_long
    {} else {
        __assert_fail(
            b"w - ann->weight == ann->total_weights\0" as *const u8
                as *const libc::c_char,
            b"genann.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            b"const double *genann_run(const genann *, const double *)\0" as *const u8 as *const libc::c_char,
        );
    }
    if o.offset_from((*ann.as_deref().unwrap()).output) as libc::c_long
        == (*ann.as_deref().unwrap()).total_neurons as libc::c_long
    {} else {
        __assert_fail(
            b"o - ann->output == ann->total_neurons\0" as *const u8
                as *const libc::c_char,
            b"genann.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            b"const double *genann_run(const genann *, const double *)\0" as *const u8 as *const libc::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_train(
    mut ann: *mut crate::src::example1::genann,
    mut inputs: *const libc::c_double,
    mut desired_outputs: *const libc::c_double,
    mut learning_rate: libc::c_double,
) {
    genann_run(ann.as_mut(), inputs);
    let mut h: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut o: *const libc::c_double = (*ann).output
        .offset((*ann).inputs as isize)
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut d = (*ann).delta.offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut t = desired_outputs;
    if (*ann).activation_output
        == Some(
            genann_act_linear as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
        )
    {
        j= 0 as libc::c_int;
        while j < (*ann).outputs {
            let fresh14 = t;
            t= t.offset(1);
            let fresh15 = o;
            o= o.offset(1);
            let fresh16 = d;
            d= d.offset(1);
            *fresh16= (*fresh14) - (*fresh15);
            j+= 1;
        }
    } else {
        j= 0 as libc::c_int;
        while j < (*ann).outputs {
            let fresh17 = d;
            d= d.offset(1);
            *fresh17= ((*t) - (*o)) * (*o) * (1.0f64 - (*o));
            o= o.offset(1);
            t= t.offset(1);
            j+= 1;
        }
    }
    h= (*ann).hidden_layers - 1 as libc::c_int;
    while h >= 0 as libc::c_int {
        let mut o_0: *const libc::c_double = (*ann).output
            .offset((*ann).inputs as isize)
            .offset((h * (*ann).hidden) as isize);
        let mut d_0 = (*ann).delta.offset((h * (*ann).hidden) as isize);
        let dd: *const libc::c_double = (*ann).delta
            .offset(((h + 1 as libc::c_int) * (*ann).hidden) as isize);
        let ww: *const libc::c_double = (*ann).weight
            .offset((((*ann).inputs + 1 as libc::c_int) * (*ann).hidden) as isize)
            .offset((((*ann).hidden + 1 as libc::c_int) * (*ann).hidden * h) as isize);
        j= 0 as libc::c_int;
        while j < (*ann).hidden {
            let mut delta = 0 as libc::c_int as libc::c_double;
            k= 0 as libc::c_int;
            while k
                < (if h == (*ann).hidden_layers - 1 as libc::c_int {
                    (*ann).outputs
                } else {
                    (*ann).hidden
                })
            {
                let forward_delta = *dd.offset(k as isize);
                let windex = k * ((*ann).hidden + 1 as libc::c_int)
                    + (j + 1 as libc::c_int);
                let forward_weight = *ww.offset(windex as isize);
                delta+= forward_delta * forward_weight;
                k+= 1;
            }
            *d_0= (*o_0) * (1.0f64 - (*o_0)) * delta;
            d_0= d_0.offset(1);
            o_0= o_0.offset(1);
            j+= 1;
        }
        h-= 1;
    }
    let mut d_1: *const libc::c_double = (*ann).delta
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut w = (*ann).weight
        .offset(
            (if (*ann).hidden_layers != 0 {
                ((*ann).inputs + 1 as libc::c_int) * (*ann).hidden
                    + ((*ann).hidden + 1 as libc::c_int) * (*ann).hidden
                        * ((*ann).hidden_layers - 1 as libc::c_int)
            } else {
                0 as libc::c_int
            }) as isize,
        );
    let i: *const libc::c_double = (*ann).output
        .offset(
            (if (*ann).hidden_layers != 0 {
                (*ann).inputs + (*ann).hidden * ((*ann).hidden_layers - 1 as libc::c_int)
            } else {
                0 as libc::c_int
            }) as isize,
        );
    j= 0 as libc::c_int;
    while j < (*ann).outputs {
        k= 0 as libc::c_int;
        while k
            < (if (*ann).hidden_layers != 0 { (*ann).hidden } else { (*ann).inputs })
                + 1 as libc::c_int
        {
            if k == 0 as libc::c_int {
                let fresh18 = w;
                w= w.offset(1);
                *fresh18+= (*d_1) * learning_rate * -1.0f64;
            } else {
                let fresh19 = w;
                w= w.offset(1);
                *fresh19+= (*d_1) * learning_rate * *i.offset((k - 1 as libc::c_int) as isize);
            }
            k+= 1;
        }
        d_1= d_1.offset(1);
        j+= 1;
    }
    if w.offset_from((*ann).weight) as libc::c_long
        == (*ann).total_weights as libc::c_long
    {} else {
        __assert_fail(
            b"w - ann->weight == ann->total_weights\0" as *const u8
                as *const libc::c_char,
            b"genann.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int as libc::c_uint,
            b"void genann_train(const genann *, const double *, const double *, double)\0" as *const u8 as *const libc::c_char,
        );
    }
    h= (*ann).hidden_layers - 1 as libc::c_int;
    while h >= 0 as libc::c_int {
        let mut d_2: *const libc::c_double = (*ann).delta
            .offset((h * (*ann).hidden) as isize);
        let mut i_0: *const libc::c_double = (*ann).output
            .offset(
                (if h != 0 {
                    (*ann).inputs + (*ann).hidden * (h - 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        let mut w_0 = (*ann).weight
            .offset(
                (if h != 0 {
                    ((*ann).inputs + 1 as libc::c_int) * (*ann).hidden
                        + ((*ann).hidden + 1 as libc::c_int) * (*ann).hidden
                            * (h - 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        j= 0 as libc::c_int;
        while j < (*ann).hidden {
            k= 0 as libc::c_int;
            while k
                < (if h == 0 as libc::c_int { (*ann).inputs } else { (*ann).hidden })
                    + 1 as libc::c_int
            {
                if k == 0 as libc::c_int {
                    let fresh20 = w_0;
                    w_0= w_0.offset(1);
                    *fresh20+= (*d_2) * learning_rate * -1.0f64;
                } else {
                    let fresh21 = w_0;
                    w_0= w_0.offset(1);
                    *fresh21+= (*d_2) * learning_rate
                            * *i_0.offset((k - 1 as libc::c_int) as isize);
                }
                k+= 1;
            }
            d_2= d_2.offset(1);
            j+= 1;
        }
        h-= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn genann_write(mut ann: *const crate::src::example1::genann, mut out: *mut FILE) {
    fprintf(
        out,
        b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
        (*ann).inputs,
        (*ann).hidden_layers,
        (*ann).hidden,
        (*ann).outputs,
    );
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < (*ann).total_weights {
        fprintf(
            out,
            b" %.20e\0" as *const u8 as *const libc::c_char,
            *(*ann).weight.offset(i as isize),
        );
        i+= 1;
    }
}

}

pub mod example1 {
use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    
    
    
    
}
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: libc::c_int,
    pub hidden_layers: libc::c_int,
    pub hidden: libc::c_int,
    pub outputs: libc::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: libc::c_int,
    pub total_neurons: libc::c_int,
    pub weight: *mut libc::c_double,
    pub output: *mut libc::c_double,
    pub delta: *mut libc::c_double,
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 1.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Train a small ANN to the XOR function using backpropagation.\n\0" as *const u8
            as *const libc::c_char,
    );
    let input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut i: libc::c_int = 0;
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    i= 0 as libc::c_int;
    while i < 300 as libc::c_int {
        crate::src::genann::genann_train(
            ann,
            (input[0 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(0 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        crate::src::genann::genann_train(
            ann,
            (input[1 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(1 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        crate::src::genann::genann_train(
            ann,
            (input[2 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(2 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        crate::src::genann::genann_train(
            ann,
            (input[3 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(3 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        i+= 1;
    }
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[0 as libc::c_int as usize][0 as libc::c_int as usize],
        input[0 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[0 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[1 as libc::c_int as usize][0 as libc::c_int as usize],
        input[1 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[1 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[2 as libc::c_int as usize][0 as libc::c_int as usize],
        input[2 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[2 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[3 as libc::c_int as usize][0 as libc::c_int as usize],
        input[3 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[3 as libc::c_int as usize]).as_ptr()),
    );
    crate::src::genann::genann_free(ann);
    return 0 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }

}

pub mod example2 {
use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    
    
    
    
    
}
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 2.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Train a small ANN to the XOR function using random search.\n\0" as *const u8
            as *const libc::c_char,
    );
    let input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut i: libc::c_int = 0;
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    let mut err: libc::c_double = 0.;
    let mut last_err = 1000 as libc::c_int as libc::c_double;
    let mut count = 0 as libc::c_int;
    loop {
        count+= 1;
        if count % 1000 as libc::c_int == 0 as libc::c_int {
            crate::src::genann::genann_randomize(ann.as_mut());
        }
        let mut save = crate::src::genann::genann_copy(ann);
        i= 0 as libc::c_int;
        while i < (*ann).total_weights {
            *(*ann).weight.offset(i as isize)
                += rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double
                    - 0.5f64;
            i+= 1;
        }
        err= 0 as libc::c_int as libc::c_double;
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[0 as libc::c_int as usize]).as_ptr())
                    - output[0 as libc::c_int as usize],
                2.0f64,
            );
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[1 as libc::c_int as usize]).as_ptr())
                    - output[1 as libc::c_int as usize],
                2.0f64,
            );
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[2 as libc::c_int as usize]).as_ptr())
                    - output[2 as libc::c_int as usize],
                2.0f64,
            );
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[3 as libc::c_int as usize]).as_ptr())
                    - output[3 as libc::c_int as usize],
                2.0f64,
            );
        if err < last_err {
            crate::src::genann::genann_free(save);
            last_err= err;
        } else {
            crate::src::genann::genann_free(ann);
            ann= save;
        }
        if !(err > 0.01f64) {
            break;
        }
    }
    printf(b"Finished in %d loops.\n\0" as *const u8 as *const libc::c_char, count);
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[0 as libc::c_int as usize][0 as libc::c_int as usize],
        input[0 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[0 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[1 as libc::c_int as usize][0 as libc::c_int as usize],
        input[1 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[1 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[2 as libc::c_int as usize][0 as libc::c_int as usize],
        input[2 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[2 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[3 as libc::c_int as usize][0 as libc::c_int as usize],
        input[3 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[3 as libc::c_int as usize]).as_ptr()),
    );
    crate::src::genann::genann_free(ann);
    return 0 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }

}

pub mod example3 {
use ::libc;
extern "C" {
    
    
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut crate::src::example4::_IO_codecvt,
    pub _wide_data: *mut crate::src::test::_IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1 { dummy: () }
#[no_mangle]
pub static mut save_name: *const libc::c_char = b"example/xor.ann\0" as *const u8
    as *const libc::c_char;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 3.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Load a saved ANN to solve the XOR function.\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut saved = fopen(crate::src::example3::save_name, b"r\0" as *const u8 as *const libc::c_char);
    if saved.is_null() {();
        printf(
            b"Couldn't open file: %s\n\0" as *const u8 as *const libc::c_char,
            crate::src::example3::save_name,
        );
        exit(1 as libc::c_int);
    }
    let mut ann = crate::src::genann::genann_read(saved);
    fclose(saved);
    if ann.is_null() {();
        printf(
            b"Error loading ANN from file: %s.\0" as *const u8 as *const libc::c_char,
            crate::src::example3::save_name,
        );
        exit(1 as libc::c_int);
    }
    let input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[0 as libc::c_int as usize][0 as libc::c_int as usize],
        input[0 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[0 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[1 as libc::c_int as usize][0 as libc::c_int as usize],
        input[1 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[1 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[2 as libc::c_int as usize][0 as libc::c_int as usize],
        input[2 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[2 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[3 as libc::c_int as usize][0 as libc::c_int as usize],
        input[3 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[3 as libc::c_int as usize]).as_ptr()),
    );
    crate::src::genann::genann_free(ann);
    return 0 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }

}

pub mod example4 {
use ::libc;
extern "C" {
    
    pub type _IO_codecvt;
    
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor2 { dummy: () }
pub type _IO_lock_t = ();
pub type FILE = crate::src::example3::_IO_FILE;
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor3 { dummy: () }
#[no_mangle]
pub static mut iris_data: *const libc::c_char = b"example/iris.data\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut input: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
#[no_mangle]
pub static mut class: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
#[no_mangle]
pub static mut samples: libc::c_int = 0;
#[no_mangle]
pub static mut class_names: [*const libc::c_char; 3] = [
    b"Iris-setosa\0" as *const u8 as *const libc::c_char,
    b"Iris-versicolor\0" as *const u8 as *const libc::c_char,
    b"Iris-virginica\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn load_data() {
    let mut in_0 = fopen(
        b"example/iris.data\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if in_0.is_null() {();
        printf(
            b"Could not open file: %s\n\0" as *const u8 as *const libc::c_char,
            crate::src::example4::iris_data,
        );
        exit(1 as libc::c_int);
    }
    let mut line: [libc::c_char; 1024] = [0; 1024];
    while feof(in_0) == 0
        && !(fgets(line.as_mut_ptr(), 1024 as libc::c_int, in_0)).is_null()
    {
        crate::src::example4::samples+= 1;
    }
    fseek(in_0, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    printf(
        b"Loading %d data points from %s\n\0" as *const u8 as *const libc::c_char,
        crate::src::example4::samples,
        crate::src::example4::iris_data,
    );
    crate::src::example4::input= malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(crate::src::example4::samples as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_double;
    crate::src::example4::class= malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(crate::src::example4::samples as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < crate::src::example4::samples {
        let mut p = crate::src::example4::input.offset((i * 4 as libc::c_int) as isize);
        let mut c = crate::src::example4::class.offset((i * 3 as libc::c_int) as isize);
        *c.offset(2 as libc::c_int as isize) = 0.0f64; *c.offset(1 as libc::c_int as isize) = *c.offset(2 as libc::c_int as isize); *c.offset(0 as libc::c_int as isize) = *c.offset(1 as libc::c_int as isize);
        if (fgets(line.as_mut_ptr(), 1024 as libc::c_int, in_0)).is_null() {();
            perror(b"fgets\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        let mut split = strtok(
            line.as_mut_ptr(),
            b",\0" as *const u8 as *const libc::c_char,
        );
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            *p.offset(j as isize) = atof(split);
            split= strtok(
                0 as *mut libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
            );
            j+= 1;
        }
        *split
            .offset(
                (strlen(split)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        if strcmp(split, crate::src::example4::class_names[0 as libc::c_int as usize]) == 0 as libc::c_int {
            *c.offset(0 as libc::c_int as isize) = 1.0f64;
        } else if strcmp(split, crate::src::example4::class_names[1 as libc::c_int as usize])
            == 0 as libc::c_int
        {
            *c.offset(1 as libc::c_int as isize) = 1.0f64;
        } else if strcmp(split, crate::src::example4::class_names[2 as libc::c_int as usize])
            == 0 as libc::c_int
        {
            *c.offset(2 as libc::c_int as isize) = 1.0f64;
        } else {
            printf(b"Unknown class %s.\n\0" as *const u8 as *const libc::c_char, split);
            exit(1 as libc::c_int);
        }
        i+= 1;
    }
    fclose(in_0);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 4.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Train an ANN on the IRIS dataset using backpropagation.\n\0" as *const u8
            as *const libc::c_char,
    );
    load_data();
    let mut ann = crate::src::genann::genann_init(
        4 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut loops = 5000 as libc::c_int;
    printf(
        b"Training for %d loops over data.\n\0" as *const u8 as *const libc::c_char,
        loops,
    );
    i= 0 as libc::c_int;
    while i < loops {
        j= 0 as libc::c_int;
        while j < crate::src::example4::samples {
            crate::src::genann::genann_train(
                ann,
                crate::src::example4::input.offset((j * 4 as libc::c_int) as isize),
                crate::src::example4::class.offset((j * 3 as libc::c_int) as isize),
                0.01f64,
            );
            j+= 1;
        }
        i+= 1;
    }
    let mut correct = 0 as libc::c_int;
    j= 0 as libc::c_int;
    while j < crate::src::example4::samples {
        let mut guess = crate::src::genann::genann_run(ann.as_mut(), crate::src::example4::input.offset((j * 4 as libc::c_int) as isize));
        if *crate::src::example4::class.offset((j * 3 as libc::c_int + 0 as libc::c_int) as isize) == 1.0f64 {
            if *guess.offset(0 as libc::c_int as isize)
                > *guess.offset(1 as libc::c_int as isize)
                && *guess.offset(0 as libc::c_int as isize)
                    > *guess.offset(2 as libc::c_int as isize)
            {
                correct+= 1;
            }
        } else if *crate::src::example4::class.offset((j * 3 as libc::c_int + 1 as libc::c_int) as isize)
            == 1.0f64
        {
            if *guess.offset(1 as libc::c_int as isize)
                > *guess.offset(0 as libc::c_int as isize)
                && *guess.offset(1 as libc::c_int as isize)
                    > *guess.offset(2 as libc::c_int as isize)
            {
                correct+= 1;
            }
        } else if *crate::src::example4::class.offset((j * 3 as libc::c_int + 2 as libc::c_int) as isize)
            == 1.0f64
        {
            if *guess.offset(2 as libc::c_int as isize)
                > *guess.offset(0 as libc::c_int as isize)
                && *guess.offset(2 as libc::c_int as isize)
                    > *guess.offset(1 as libc::c_int as isize)
            {
                correct+= 1;
            }
        } else {
            printf(b"Logic error.\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        j+= 1;
    }
    printf(
        b"%d/%d correct (%0.1f%%).\n\0" as *const u8 as *const libc::c_char,
        correct,
        crate::src::example4::samples,
        correct as libc::c_double / crate::src::example4::samples as libc::c_double * 100.0f64,
    );
    crate::src::genann::genann_free(ann);
    return 0 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }

}

pub mod test {
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    
    
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    
    
    
    
    
    
    
    
    
    
    
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn clock() -> clock_t;
    fn srand(__seed: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor6 { dummy: () }
pub type _IO_lock_t = ();
pub type FILE = crate::src::example3::_IO_FILE;
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7 { dummy: () }
pub type clock_t = __clock_t;
static mut ltests: libc::c_int = 0 as libc::c_int;
static mut lfails: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn basic() {
    let mut ann = crate::src::genann::genann_init(
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    crate::src::test::ltests+= 1;
    if (*ann).total_weights != 2 as libc::c_int {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            (*ann).total_weights,
            2 as libc::c_int,
        );
    }
    let mut a: libc::c_double = 0.;
    a= 0 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 1 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 11 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 10 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(1.0f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            1.0f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= -(10 as libc::c_int) as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.0f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            0.0f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn xor() {
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    (*ann).activation_hidden= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if (*ann).total_weights != 9 as libc::c_int {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            (*ann).total_weights,
            9 as libc::c_int,
        );
    }
    *(*ann).weight.offset(0 as libc::c_int as isize) = 0.5f64;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(5 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight.offset(6 as libc::c_int as isize) = 0.5f64;
    *(*ann).weight
        .offset(7 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(8 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn backprop() {
    let mut ann = crate::src::genann::genann_init(
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut input: libc::c_double = 0.;
    let mut output: libc::c_double = 0.;
    input= 0.5f64;
    output= 1 as libc::c_int as libc::c_double;
    let mut first_try = *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(input));
    crate::src::genann::genann_train(ann, core::ptr::addr_of!(input), core::ptr::addr_of!(output), 0.5f64);
    let mut second_try = *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(input));
    crate::src::test::ltests+= 1;
    if !(fabs(first_try - output) > fabs(second_try - output)) {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_and() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 50 as libc::c_int {
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            crate::src::genann::genann_train(
                ann,
                input[j as usize].as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                0.8f64,
            );
            j+= 1;
        }
        i+= 1;
    }
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_or() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    crate::src::genann::genann_randomize(ann.as_mut());
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 50 as libc::c_int {
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            crate::src::genann::genann_train(
                ann,
                input[j as usize].as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                0.8f64,
            );
            j+= 1;
        }
        i+= 1;
    }
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_xor() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 500 as libc::c_int {
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            crate::src::genann::genann_train(
                ann,
                input[j as usize].as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                3 as libc::c_int as libc::c_double,
            );
            j+= 1;
        }
        i+= 1;
    }
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn persist() {
    let mut first = crate::src::genann::genann_init(
        1000 as libc::c_int,
        5 as libc::c_int,
        50 as libc::c_int,
        10 as libc::c_int,
    );
    let mut out = fopen(
        b"persist.txt\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    crate::src::genann::genann_write(first as *const crate::src::example1::genann, out);
    fclose(out);
    let mut in_0 = fopen(
        b"persist.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut second = crate::src::genann::genann_read(in_0);
    fclose(out);
    crate::src::test::ltests+= 1;
    if (*first).inputs != (*second).inputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            (*first).inputs,
            (*second).inputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            (*first).hidden_layers,
            (*second).hidden_layers,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden != (*second).hidden {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            (*first).hidden,
            (*second).hidden,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).outputs != (*second).outputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            (*first).outputs,
            (*second).outputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).total_weights != (*second).total_weights {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            (*first).total_weights,
            (*second).total_weights,
        );
    }
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < (*first).total_weights {
        crate::src::test::ltests+= 1;
        if !(*(*first).weight.offset(i as isize)
            == *(*second).weight.offset(i as isize))
        {
            crate::src::test::lfails+= 1;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int,
            );
        }
        i+= 1;
    }
    crate::src::genann::genann_free(first);
    crate::src::genann::genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn copy() {
    let mut first = crate::src::genann::genann_init(
        1000 as libc::c_int,
        5 as libc::c_int,
        50 as libc::c_int,
        10 as libc::c_int,
    );
    let mut second = crate::src::genann::genann_copy(first as *const crate::src::example1::genann);
    crate::src::test::ltests+= 1;
    if (*first).inputs != (*second).inputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int,
            (*first).inputs,
            (*second).inputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            (*first).hidden_layers,
            (*second).hidden_layers,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden != (*second).hidden {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            (*first).hidden,
            (*second).hidden,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).outputs != (*second).outputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            (*first).outputs,
            (*second).outputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).total_weights != (*second).total_weights {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            (*first).total_weights,
            (*second).total_weights,
        );
    }
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < (*first).total_weights {
        crate::src::test::ltests+= 1;
        if fabs(
            *(*first).weight.offset(i as isize)
                - *(*second).weight.offset(i as isize),
        ) > 0.001f64
        {
            crate::src::test::lfails+= 1;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                237 as libc::c_int,
                *(*first).weight.offset(i as isize),
                *(*second).weight.offset(i as isize),
            );
        }
        i+= 1;
    }
    crate::src::genann::genann_free(first);
    crate::src::genann::genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn sigmoid() {
    let mut i = -(20 as libc::c_int) as libc::c_double;
    let max = 20 as libc::c_int as libc::c_double;
    let d = 0.0001f64;
    while i < max {
        crate::src::test::ltests+= 1;
        if fabs(crate::src::genann::genann_act_sigmoid(i) - crate::src::genann::genann_act_sigmoid_cached(i)) > 0.001f64 {
            crate::src::test::lfails+= 1;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int,
                crate::src::genann::genann_act_sigmoid(i),
                crate::src::genann::genann_act_sigmoid_cached(i),
            );
        }
        i+= d;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN TEST SUITE\n\0" as *const u8 as *const libc::c_char);
    srand(100 as libc::c_int as libc::c_uint);
    let ts = crate::src::test::ltests;
    let fs = crate::src::test::lfails;
    let start = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"basic\0" as *const u8 as *const libc::c_char,
    );
    basic();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts - (crate::src::test::lfails - fs),
        crate::src::test::lfails - fs,
        ((clock() - start) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_0 = crate::src::test::ltests;
    let fs_0 = crate::src::test::lfails;
    let start_0 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"xor\0" as *const u8 as *const libc::c_char,
    );
    xor();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_0 - (crate::src::test::lfails - fs_0),
        crate::src::test::lfails - fs_0,
        ((clock() - start_0) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_1 = crate::src::test::ltests;
    let fs_1 = crate::src::test::lfails;
    let start_1 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"backprop\0" as *const u8 as *const libc::c_char,
    );
    backprop();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_1 - (crate::src::test::lfails - fs_1),
        crate::src::test::lfails - fs_1,
        ((clock() - start_1) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_2 = crate::src::test::ltests;
    let fs_2 = crate::src::test::lfails;
    let start_2 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train and\0" as *const u8 as *const libc::c_char,
    );
    train_and();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_2 - (crate::src::test::lfails - fs_2),
        crate::src::test::lfails - fs_2,
        ((clock() - start_2) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_3 = crate::src::test::ltests;
    let fs_3 = crate::src::test::lfails;
    let start_3 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train or\0" as *const u8 as *const libc::c_char,
    );
    train_or();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_3 - (crate::src::test::lfails - fs_3),
        crate::src::test::lfails - fs_3,
        ((clock() - start_3) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_4 = crate::src::test::ltests;
    let fs_4 = crate::src::test::lfails;
    let start_4 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train xor\0" as *const u8 as *const libc::c_char,
    );
    train_xor();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_4 - (crate::src::test::lfails - fs_4),
        crate::src::test::lfails - fs_4,
        ((clock() - start_4) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_5 = crate::src::test::ltests;
    let fs_5 = crate::src::test::lfails;
    let start_5 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"persist\0" as *const u8 as *const libc::c_char,
    );
    persist();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_5 - (crate::src::test::lfails - fs_5),
        crate::src::test::lfails - fs_5,
        ((clock() - start_5) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_6 = crate::src::test::ltests;
    let fs_6 = crate::src::test::lfails;
    let start_6 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"copy\0" as *const u8 as *const libc::c_char,
    );
    copy();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_6 - (crate::src::test::lfails - fs_6),
        crate::src::test::lfails - fs_6,
        ((clock() - start_6) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_7 = crate::src::test::ltests;
    let fs_7 = crate::src::test::lfails;
    let start_7 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"sigmoid\0" as *const u8 as *const libc::c_char,
    );
    sigmoid();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_7 - (crate::src::test::lfails - fs_7),
        crate::src::test::lfails - fs_7,
        ((clock() - start_7) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    if crate::src::test::lfails == 0 as libc::c_int {
        printf(
            b"ALL TESTS PASSED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            crate::src::test::ltests,
            crate::src::test::ltests,
        );
    } else {
        printf(
            b"SOME TESTS FAILED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            crate::src::test::ltests - crate::src::test::lfails,
            crate::src::test::ltests,
        );
    }
    return (crate::src::test::lfails != 0 as libc::c_int) as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }

}

} // mod src

// root re-exports so the generated harness's `translated::<entry>` resolves
pub use crate::src::genann::genann_act_linear;
pub use crate::src::genann::genann_act_sigmoid;
pub use crate::src::genann::genann_act_sigmoid_cached;
pub use crate::src::genann::genann_act_threshold;
pub use crate::src::genann::genann_copy;
pub use crate::src::genann::genann_free;
pub use crate::src::genann::genann_init;
pub use crate::src::genann::genann_randomize;
pub use crate::src::genann::genann_read;
pub use crate::src::genann::genann_run;
pub use crate::src::genann::genann_train;
pub use crate::src::genann::genann_write;

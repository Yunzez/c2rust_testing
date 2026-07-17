


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


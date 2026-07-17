


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


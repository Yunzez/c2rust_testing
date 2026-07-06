unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let argv1 = unsafe { CStr::from_ptr(*argv.offset(1)).to_str().unwrap() };
let info = ti_indicators.iter().find(|indicator| unsafe { CStr::from_ptr(indicator.name).to_str().unwrap() } == argv1);
if argc < 2 {
    println!("No indicator given.");
    println!("Example:");
    println!("\tsample ma 5");
    return 1;
}
if argv1 == "--version" {
    println!("TI VERSION: {}, TI BUILD: {}", "0.8.4", 1537377628);
    return 0;

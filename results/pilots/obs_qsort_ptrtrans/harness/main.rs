// OBS pilot driver (Rust side, PtrTrans translation verbatim in ptrans_qsort.rs).
// Same argv/decoding/state-file protocol as driver.c.
mod ptrans_qsort;
use std::io::{self, Read, Write};
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len()<3 { std::process::exit(2); }
    let printing = args[1]=="print";
    let mut data=Vec::new(); io::stdin().read_to_end(&mut data).unwrap();
    let mut a: Vec<i32> = data.chunks_exact(4)
        .map(|c| i32::from_le_bytes([c[0],c[1],c[2],c[3]])).take(256).collect();
    let n=a.len();
    ptrans_qsort::quick_sort(Some(&mut a[..]), 0, n as i32 - 1);
    let mut st=String::from("ret:void\nglobals:none\narr:");
    for x in &a { st.push_str(&x.to_string()); st.push(' '); }
    st.push('\n');
    std::fs::write(&args[2], st).unwrap();
    if printing {
        let mut out=String::new();
        for x in &a { out.push_str(&x.to_string()); out.push(' '); }
        out.push('\n');
        io::stdout().write_all(out.as_bytes()).unwrap();
    }
}

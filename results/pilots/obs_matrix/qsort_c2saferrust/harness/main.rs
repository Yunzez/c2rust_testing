// OBS matrix driver (Rust side, C2SaferRust WIP quickSort verbatim in ../lib/src/lib.rs).
// Same argv/decoding/state-file protocol as the qsort template driver.c / main.rs.
use std::io::{self, Read, Write};
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len()<3 { std::process::exit(2); }
    let printing = args[1]=="print";
    let mut data=Vec::new(); io::stdin().read_to_end(&mut data).unwrap();
    let mut a: Vec<i32> = data.chunks_exact(4)
        .map(|c| i32::from_le_bytes([c[0],c[1],c[2],c[3]])).take(256).collect();
    let n=a.len();
    // C contract is quickSort(a,0,n-1); with usize indices n==0 would be high=usize::MAX (not a legal
    // range for the C oracle), so for n<=1 we mirror the C "low<high false" no-op by skipping the call.
    if n>=2 { qsort_c2saferrust_obs::quickSort(&mut a[..], 0, n-1); }
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

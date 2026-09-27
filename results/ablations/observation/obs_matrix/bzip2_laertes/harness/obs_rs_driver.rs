// OBS driver (Rust side, Laertes bzip2 crate verbatim). Same argv/decoding/state-file protocol as driver.c.
#![allow(non_snake_case)]
use std::io::{self, Read, Write};
use bzip2_laertes_e3::bzlib::BZ2_bzBuffToBuffCompress;
fn hex(b:&[u8])->String{ let mut s=String::with_capacity(b.len()*2); for x in b { s.push_str(&format!("{:02x}",x)); } s }
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len()<3 { std::process::exit(2); }
    let printing = args[1]=="print";
    let mut data=Vec::new(); io::stdin().read_to_end(&mut data).unwrap();
    data.truncate(65536);
    let mut out=vec![0u8; 65536*2+1200]; let mut dl:u32=out.len() as u32;
    let rc = unsafe { BZ2_bzBuffToBuffCompress(out.as_mut_ptr() as *mut i8, Some(&mut dl),
        data.as_ptr() as *mut i8, data.len() as u32, 1, 0, 30) };
    if rc!=0 { dl=0; }
    let h=hex(&out[..dl as usize]);
    std::fs::write(&args[2], format!("ret:{}\ndestLen:{}\nglobals:none\nout:{}\n",rc,dl,h)).unwrap();
    if printing { io::stdout().write_all(format!("rc={} len={}\n{}\n",rc,dl,h).as_bytes()).unwrap(); }
}

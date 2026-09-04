// OBS matrix driver (Rust side, C2SaferRust crc32_z verbatim in wip.rs). Same protocol as driver.c.
use std::io::{self,Read,Write};
fn main(){
    let args:Vec<String>=std::env::args().collect(); if args.len()<3 { std::process::exit(2); }
    let printing=args[1]=="print";
    let mut data=Vec::new(); io::stdin().read_to_end(&mut data).unwrap();
    let (crc,n)=obs_crc32::run(&data);
    std::fs::write(&args[2],format!("ret:0x{:08x}\nchunks:{}\nglobals:none\n",crc,n)).unwrap();
    if printing { io::stdout().write_all(format!("crc=0x{:08x}\n",crc).as_bytes()).unwrap(); }
}

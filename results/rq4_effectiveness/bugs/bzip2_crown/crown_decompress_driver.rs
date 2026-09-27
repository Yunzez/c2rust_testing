// Decompress-only: stdin = raw .bz2 bytes; try small=0 and small=1; print rc + output.
use std::io::Read;
use c2rust_out::bzlib::BZ2_bzBuffToBuffDecompress;
fn main(){
    let mut comp=Vec::new(); std::io::stdin().read_to_end(&mut comp).unwrap();
    for small in [0i32,1] {
        let mut back=vec![0u8; 1<<20];
        let mut bl: u32 = back.len() as u32;
        let rd = unsafe { BZ2_bzBuffToBuffDecompress(back.as_mut_ptr() as *mut i8, Some(&mut bl),
            comp.as_ptr() as *mut i8, comp.len() as u32, small, 0) };
        println!("small={} rd={} bl={} out={:?}", small, rd, bl,
            if rd==0 { String::from_utf8_lossy(&back[..bl as usize]).to_string() } else { "-".into() });
    }
}

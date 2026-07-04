use std::io::{Read, Write};
use c2rust_out::bzlib::BZ2_bzBuffToBuffCompress;
fn fnv(p:&[u8])->u64{let mut h=1469598103934665603u64;for &b in p{h^=b as u64;h=h.wrapping_mul(1099511628211);}h}
fn main(){
    let mut inp=Vec::new(); std::io::stdin().read_to_end(&mut inp).unwrap();
    let mut out=vec![0u8;(1<<20)+1200000];
    let mut p=0usize; let mut idx=0;
    while p+4<=inp.len(){
        let len=u32::from_le_bytes([inp[p],inp[p+1],inp[p+2],inp[p+3]]) as usize; p+=4;
        if p+len+2>inp.len(){break;}
        let src=inp[p..p+len].to_vec(); p+=len;
        let bs=inp[p] as i32; let wf=inp[p+1] as i32; p+=2;
        eprint!("REC {} len={} bs={} wf={}\r", idx, len, bs, wf);
        unsafe{
            let mut dl:u32=out.len() as u32;
            let rc=BZ2_bzBuffToBuffCompress(out.as_mut_ptr() as *mut i8,Some(&mut dl),
                src.as_ptr() as *mut i8,len as u32,bs,0,wf);
            if let Ok(pth)=std::env::var("CROWN_OUT"){ if rc==0 {std::fs::write(pth,&out[..dl as usize]).unwrap();} }
            if rc!=0{println!("rc={}",rc);}
            else{println!("ok len={} fnv={:016x}",dl,fnv(&out[..dl as usize]));}
        }
        std::io::stdout().flush().unwrap();
        idx+=1;
    }
    eprintln!("\nDONE {} records", idx);
}

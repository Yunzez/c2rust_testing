use std::io::{Read, Write, BufWriter};
use c2rust_out::src::zlib::crc32::crc32;
use c2rust_out::src::zlib::adler32::adler32;
fn main(){
    let mut inp=Vec::new(); std::io::stdin().read_to_end(&mut inp).unwrap();
    let so=std::io::stdout(); let mut o=BufWriter::new(so.lock());
    let mut p=0;
    while p+2<=inp.len(){
        let len=(inp[p] as usize)|((inp[p+1] as usize)<<8); p+=2;
        if p+len>inp.len(){break;}
        let buf=&inp[p..p+len]; p+=len;
        unsafe{
            let c=crc32(0, buf.as_ptr(), len as u32);
            let a=adler32(1, buf.as_ptr(), len as u32);
            writeln!(o,"crc={:08x} adler={:08x}", c as u32, a as u32).unwrap();
        }
    }
}

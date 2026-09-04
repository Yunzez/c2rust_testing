use std::io::{Read, Write};
use std::ffi::CStr;
use lil_rs::src::lil::{lil_new, lil_parse, lil_to_string, lil_free_value, lil_free};
fn main(){
    let mut inp=Vec::new(); std::io::stdin().read_to_end(&mut inp).unwrap();
    let mut p=0usize;
    let so=std::io::stdout(); let mut o=std::io::BufWriter::new(so.lock());
    while p+2<=inp.len(){
        let len=(inp[p] as usize)|((inp[p+1] as usize)<<8); p+=2;
        if p+len>inp.len(){break;}
        let mut code=inp[p..p+len].to_vec(); code.push(0); p+=len;
        unsafe{
            let lil=lil_new();
            let r=lil_parse(lil, code.as_ptr() as *const libc::c_char, len as libc::c_ulong, 0);
            let s=lil_to_string(r);
            if s.is_null(){ writeln!(o,"[(null)]").unwrap(); }
            else { writeln!(o,"[{}]", CStr::from_ptr(s).to_string_lossy()).unwrap(); }
            lil_free_value(r); lil_free(lil);
        }
        o.flush().unwrap();
    }
}

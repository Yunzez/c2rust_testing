use std::ffi::CString;
use std::os::raw::{c_char, c_int};
extern "C" { fn c_u8next(t: *const c_char, ch: *mut c_int) -> c_int; }
// RustAssure gpt-3.5-turbo u8next_ (verbatim):
fn u8next_gpt35(txt: *const c_char, ch: Option<&mut i32>) -> i32 {
    let mut len = 0; let s = txt as *const u8; let first = unsafe { *s }; let mut val = 0;
    if first != 0 { val = first as i32; let s = s;
        loop { match unsafe { *s } {
            0x00..=0x7F => { len = 1; break; } 0xC0 => { len = 2; break; } 0xC1 => break,
            0xC2..=0xDF => { val &= 0x1F; len = 2; break; } 0xE0 => { val &= 0x0F; len = 3; break; }
            0xE1..=0xEC => { val &= 0x0F; len = 3; break; } 0xED => { val &= 0x0F; len = 3; break; }
            0xEE..=0xEF => { val &= 0x0F; len = 3; break; } 0xF0 => { val &= 0x07; len = 4; break; }
            0xF1..=0xF3 => { val &= 0x07; len = 4; break; } 0xF4 => { val &= 0x07; len = 4; break; } _ => break,
        } }
        if ch.is_some() { *ch.unwrap() = val; } }
    len
}
fn main() {
    let tests = ["a","©","€","日","😀","café"];
    println!("{:<8} {:>12} {:>12}", "input", "C (len,cp)", "gpt3.5 (len,cp)");
    let mut diffs=0;
    for s in tests {
        let cs = CString::new(s).unwrap();
        let mut c_ch=0i32; let cl = unsafe { c_u8next(cs.as_ptr(), &mut c_ch) };
        let mut g_ch=0i32; let gl = u8next_gpt35(cs.as_ptr(), Some(&mut g_ch));
        let bad = cl!=gl || c_ch!=g_ch;
        println!("{:<8?} ({:>3},{:>6}) ({:>3},{:>6})  {}", s, cl, c_ch, gl, g_ch, if bad {"*** DIFF ***"} else {""});
        if bad {diffs+=1;}
    }
    // exhaustive over all single codepoints U+0020..U+10FFFF (valid, skip surrogates)
    let mut tot=0; let mut d=0;
    for cp in 0x20u32..=0x10FFFF { if (0xD800..=0xDFFF).contains(&cp) { continue; }
        if let Some(ch)=char::from_u32(cp) { let s=ch.to_string();
            if let Ok(cs)=CString::new(s) {
                let mut c_ch=0i32; let cl=unsafe{c_u8next(cs.as_ptr(),&mut c_ch)};
                let mut g_ch=0i32; let gl=u8next_gpt35(cs.as_ptr(),Some(&mut g_ch));
                tot+=1; if cl!=gl||c_ch!=g_ch {d+=1;} } } }
    println!("\nexhaustive single codepoints U+0020..U+10FFFF: {}/{} diverge", d, tot);
}

use std::ffi::CString;
use std::os::raw::c_char;
extern "C" { fn c_u8strlen(s: *const c_char) -> i32; }
// RustAssure gpt-3.5-turbo translation (verbatim):
fn u8strlen_gpt35(s: &str) -> usize {
    s.chars().filter(|c| (*c as u8 & 0xC0) != 0x80).count()
}
// gpt-4o translation (verbatim, for contrast):
fn u8strlen_gpt4o(s: &str) -> usize {
    let mut len = 0;
    for &byte in s.as_bytes() { if (byte & 0xC0) != 0x80 { len += 1; } }
    len
}
fn c_call(s: &str) -> i32 { let cs = CString::new(s).unwrap(); unsafe { c_u8strlen(cs.as_ptr()) } }
fn main() {
    let tests = ["", "a", "abc", "©", "café", "©©©", "naïve", "日本語", "À", "\u{0080}\u{00BF}", "hello €world"];
    println!("{:<16} {:>5} {:>10} {:>10}", "input", "C", "gpt-3.5", "gpt-4o");
    let mut d35=0; let mut d4o=0;
    for s in tests {
        let c = c_call(s); let g35 = u8strlen_gpt35(s) as i32; let g4 = u8strlen_gpt4o(s) as i32;
        println!("{:<16?} {:>5} {:>10} {:>10}  {}", s, c, g35, g4,
            format!("{}{}", if g35!=c {"gpt3.5=BUG "} else {""}, if g4!=c {"gpt4o=BUG"} else {""}));
        if g35!=c {d35+=1;} if g4!=c {d4o+=1;}
    }
    // exhaustive-ish: all single codepoints U+0000..U+FFFF (valid, skip surrogates)
    for cp in 0x20u32..=0xFFFF {
        if (0xD800..=0xDFFF).contains(&cp) { continue; }
        if let Some(ch)=char::from_u32(cp) {
            let s=ch.to_string();
            let c=c_call(&s); let g35=u8strlen_gpt35(&s) as i32; let g4=u8strlen_gpt4o(&s) as i32;
            if g35!=c {d35+=1;} if g4!=c {d4o+=1;}
        }
    }
    println!("\nover U+0020..U+FFFF single codepoints + samples: gpt-3.5 diffs={} gpt-4o diffs={}", d35, d4o);
}

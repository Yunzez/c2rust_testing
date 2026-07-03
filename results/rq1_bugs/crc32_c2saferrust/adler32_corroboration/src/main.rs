#![allow(non_upper_case_globals,non_camel_case_types,non_snake_case,dead_code,unused_mut,unused_assignments,unused_unsafe,unused_parens,unused_imports)]
mod base;
mod wip;
// Corroboration of the empty-input checksum-reset pattern in the sibling zlib checksum adler32_z.
// (Only the len==0 path is exercised here: adler32_z's NON-empty path is *separately* grossly
//  miscompiled by C2SaferRust — wrong sums + an OOB panic — so we isolate the clean len==0 finding.)
fn main() {
    println!("== adler32_z(adler, valid buf, len=0): does it preserve the running checksum? ==");
    for &a in &[1u64, 0x00010001, 0x12345678, 0xabcdef01u64] {
        let b = unsafe { base::adler32_z(a, b"".as_ptr(), 0) };
        let w = unsafe { wip::adler32_z(a, b"".as_ptr(), 0) };
        println!("adler=0x{:08x}  base=0x{:08x}  wip=0x{:08x}  {}", a, b, w,
                 if b==w {"ok"} else {"*** DIFF (checksum reset to seed 1) ***"});
    }
}

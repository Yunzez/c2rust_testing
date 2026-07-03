#![allow(non_upper_case_globals,non_camel_case_types,non_snake_case,dead_code,unused_mut,unused_assignments,unused_unsafe,unused_parens,unused_imports)]
mod base;
mod wip;

fn main() {
    // 1) The len==0 (empty buffer, NONZERO crc) case — the suspected NULL-vs-empty bug.
    println!("== len==0 empty-buffer test (crc preserved?) ==");
    for &c in &[0u64, 1, 0x12345678, 0xffffffff, 0xdeadbeef] {
        let b = unsafe { base::crc32_z(c, b"".as_ptr(), 0) };
        let w = wip::crc32_z(c, &[], 0);
        println!("crc=0x{:08x}  base=0x{:08x}  wip=0x{:08x}  {}", c, b, w, if b==w {"ok"} else {"*** DIFF ***"});
    }
    // 2) Non-empty faithfulness + fuzz-ish sweep to confirm the ONLY diff is the empty case.
    println!("== sweep: random-ish buffers, all lengths 0..=40, several seed crcs ==");
    let mut diffs=0u64; let mut total=0u64; let mut empty_diffs=0u64;
    let mut x=0x9e3779b97f4a7c15u64;
    for trial in 0..1000000u64 {
        x ^= x<<13; x ^= x>>7; x ^= x<<17;  // xorshift
        let len=(x % 301) as usize;
        let mut buf=vec![0u8; len];
        for i in 0..len { x ^= x<<13; x ^= x>>7; x ^= x<<17; buf[i]=(x & 0xff) as u8; }
        let seed = if trial & 1 == 0 { 0u64 } else { (x & 0xffffffff) as u64 };
        let b = unsafe { base::crc32_z(seed, if len==0 { b"".as_ptr() } else { buf.as_ptr() }, len as std::os::raw::c_ulong) };
        let w = wip::crc32_z(seed, &buf, len);
        total+=1;
        if b!=w { diffs+=1; if len==0 { empty_diffs+=1; }
            if diffs<=8 { println!("DIFF len={} seed=0x{:08x}  base=0x{:08x} wip=0x{:08x}", len, seed, b, w); } }
    }
    println!("sweep: total={} diffs={} (of which len==0: {})", total, diffs, empty_diffs);
}

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
    idat_demo();
}

// ---- appended: end-to-end demo — optipng IDAT CRC accumulation (optim.rs:1581/1612) ----
// C accumulates the IDAT chunk CRC as: crc32(0,"IDAT",4) then per-write-segment crc32(crc,seg,len).
// A zero-length segment (legal: libpng can flush a 0-byte write; empty IDAT split is spec-valid)
// resets the WIP running CRC to 0 -> the CRC written into the PNG file is WRONG.
fn idat_crc_base(idat_data_segments: &[&[u8]]) -> u64 {
    let mut crc = unsafe { base::crc32_z(0, b"IDAT".as_ptr(), 4) };
    for seg in idat_data_segments {
        crc = unsafe { base::crc32_z(crc, if seg.is_empty() { b"".as_ptr() } else { seg.as_ptr() }, seg.len() as std::os::raw::c_ulong) };
    }
    crc
}
fn idat_crc_wip(idat_data_segments: &[&[u8]]) -> u64 {
    let mut crc = wip::crc32_z(0, b"IDAT", 4);
    for seg in idat_data_segments {
        crc = wip::crc32_z(crc, seg, seg.len());
    }
    crc
}
#[allow(dead_code)]
fn idat_demo() {
    println!("\n== END-TO-END: optipng IDAT chunk CRC (optim.rs:1612 accumulation) ==");
    // Same IDAT payload, two ways libpng might segment the writes:
    let payload: &[u8] = b"\x08\xd7\x63\x60\x00\x00\x00\x02\x00\x01"; // a small zlib/IDAT-ish blob
    let no_empty:  Vec<&[u8]> = vec![&payload[..5], &payload[5..]];
    let with_empty: Vec<&[u8]> = vec![&payload[..5], &[], &payload[5..]]; // a 0-byte flush in the middle
    let b1 = idat_crc_base(&no_empty);   let w1 = idat_crc_wip(&no_empty);
    let b2 = idat_crc_base(&with_empty); let w2 = idat_crc_wip(&with_empty);
    println!("segmented [5,5]      : base=0x{:08x} wip=0x{:08x}  {}", b1, w1, if b1==w1 {"ok"} else {"DIFF"});
    println!("segmented [5,0,5]    : base=0x{:08x} wip=0x{:08x}  {}", b2, w2, if b2==w2 {"ok"} else {"*** WRONG CRC WRITTEN TO PNG ***"});
    println!("(base is stable regardless of segmentation; WIP's CRC changes when a 0-byte segment appears -> corrupt IDAT)");
}

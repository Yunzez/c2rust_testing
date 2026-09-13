#!/usr/bin/env python3
"""Package a second set of first-order defects for released FLOURINE."""

from __future__ import annotations

import importlib.util
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
SPEC = importlib.util.spec_from_file_location(
    "easy", ROOT / "scripts/baseline_artifacts/make_flourine_easy_expansion.py"
)
assert SPEC and SPEC.loader
easy = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(easy)


def through(text: str, start_marker: str, end_marker: str) -> str:
    start = text.index(start_marker)
    end = text.index(end_marker, start) + len(end_marker)
    return text[start:end]


def make_s1() -> None:
    c_path = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/source/zlib/crc32.c"
    h_path = c_path.with_name("crc32.h")
    r_path = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs"
    c_fn = easy.brace_item(c_path.read_text(), "unsigned long ZEXPORT crc32_z")
    rust_text = r_path.read_text()
    table = through(rust_text, "static mut crc_table:", "]];")
    r_fn = easy.brace_item(rust_text, "pub fn crc32_z")
    c_prelude = """typedef unsigned int z_crc_t;
typedef unsigned long z_size_t;
#define local static
#define FAR
#define TBLS 1
#define ZEXPORT
#define Z_NULL 0
#define DO1 crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8)
#define DO8 DO1; DO1; DO1; DO1; DO1; DO1; DO1; DO1
"""
    wrapper = """
unsigned long crc32_z_packet(unsigned char packet[25]) {
    unsigned long crc = 0;
    memcpy(&crc, packet, sizeof(crc));
    z_size_t len = packet[24] % 17;
    return crc32_z(crc, packet + 8, len);
}
"""
    r_prelude = "pub type z_crc_t = u32;\n" + table
    r_wrapper = """#[no_mangle]
pub fn crc32_z_packet(packet: &mut [u8; 25]) -> u64 {
    let crc = u64::from_ne_bytes(packet[0..8].try_into().unwrap());
    let len = (packet[24] % 17) as usize;
    crc32_z(crc, &packet[8..8 + len], len)
}"""
    easy.emit(
        "S1", "crc32_z_packet", ["#include <string.h>"], [],
        ["unsigned long crc32_z_packet(unsigned char packet[25]);"],
        c_prelude + h_path.read_text() + "\n" + c_fn + wrapper,
        r_prelude, r_fn, r_wrapper,
        {"c": str(c_path.relative_to(ROOT)), "c_table": str(h_path.relative_to(ROOT)), "rust": str(r_path.relative_to(ROOT))},
        "Decode a generated packet into CRC, a 16-byte non-NULL buffer, and a length in [0,16]; call the exact target on both sides.",
    )


def make_s2() -> None:
    c_path = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/source/zlib/adler32.c"
    r_path = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs"
    c_fn = easy.brace_item(c_path.read_text(), "uLong ZEXPORT adler32_z")
    r_fn = easy.brace_item(r_path.read_text(), 'pub unsafe extern "C" fn adler32_z')
    c_prelude = """typedef unsigned long uLong;
typedef unsigned char Bytef;
typedef unsigned long z_size_t;
#define ZEXPORT
#define Z_NULL 0
#define BASE 65521U
#define NMAX 5552
#define MOD(a) a %= BASE
#define MOD28(a) a %= BASE
#define MOD63(a) a %= BASE
#define DO1(buf,i) {adler += (buf)[i]; sum2 += adler;}
#define DO2(buf,i) DO1(buf,i); DO1(buf,i+1);
#define DO4(buf,i) DO2(buf,i); DO2(buf,i+2);
#define DO8(buf,i) DO4(buf,i); DO4(buf,i+4);
#define DO16(buf) DO8(buf,0); DO8(buf,8);
"""
    wrapper = """
unsigned long adler32_z_packet(unsigned char packet[25]) {
    unsigned long adler = 0;
    memcpy(&adler, packet, sizeof(adler));
    z_size_t len = packet[24] % 16;
    return adler32_z(adler, packet + 8, len);
}
"""
    r_prelude = "pub type uLong = u64; pub type Bytef = u8; pub type z_size_t = u64;"
    r_wrapper = """#[no_mangle]
pub fn adler32_z_packet(packet: &mut [u8; 25]) -> u64 {
    let adler = u64::from_ne_bytes(packet[0..8].try_into().unwrap());
    let len = (packet[24] % 16) as u64;
    unsafe { adler32_z(adler, packet[8..].as_ptr(), len) }
}"""
    easy.emit(
        "S2", "adler32_z_packet", ["#include <string.h>"], [],
        ["unsigned long adler32_z_packet(unsigned char packet[25]);"],
        c_prelude + c_fn + wrapper,
        r_prelude, r_fn, r_wrapper,
        {"c": str(c_path.relative_to(ROOT)), "rust": str(r_path.relative_to(ROOT))},
        "Decode a generated packet into Adler state, a 16-byte non-NULL buffer, and a length in [0,15]; call the exact target on both sides.",
    )


def make_s14_retry() -> None:
    c_path = ROOT / "benchmark/pairs/rq4/bzip2_c2saferrust/source/blocksort.c"
    r_path = ROOT / "benchmark/pairs/rq4/bzip2_c2saferrust/translated/bzip2_c2saferrust.rs"
    c_fn = easy.brace_item(c_path.read_text(), "UChar mmed3")
    r_fn = easy.brace_item(r_path.read_text(), " fn mmed3")
    easy.emit(
        "S14", "mmed3_packet", [], [], ["unsigned int mmed3_packet(unsigned char values[3]);"],
        "typedef unsigned char UChar;\n" + c_fn + "\nunsigned int mmed3_packet(unsigned char values[3]) { return (unsigned int)mmed3(values[0], values[1], values[2]); }",
        "", r_fn,
        "#[no_mangle]\npub fn mmed3_packet(values: &mut [u8; 3]) -> u32 { u32::from(mmed3(values[0], values[1], values[2])) }",
        {"c": str(c_path.relative_to(ROOT)), "rust": str(r_path.relative_to(ROOT))},
        "Bundle the three independently generated bytes into one fixed array and losslessly widen the u8 result to u32, avoiding the released artifact's primitive-argument and char-return ABI emitter defects.",
    )


def make_c13_retry() -> None:
    c_path = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/source/optipng/optim.c"
    r_path = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs"
    c_fn = easy.brace_item(c_path.read_text(), "opng_free(void *ptr)")
    r_fn = easy.brace_item(r_path.read_text(), "fn opng_free(ptr:")
    easy.emit(
        "C13", "opng_free_null_packet", ["#include <stdlib.h>"], [],
        ["void opng_free_null_packet(unsigned char ignored[1]);"],
        c_fn + "\nvoid opng_free_null_packet(unsigned char ignored[1]) { (void)ignored; opng_free(NULL); }",
        "", r_fn,
        "#[no_mangle]\npub fn opng_free_null_packet(_ignored: &mut [u8; 1]) { opng_free(std::ptr::null_mut()); }",
        {"c": str(c_path.relative_to(ROOT)), "rust": str(r_path.relative_to(ROOT))},
        "Add one ignored generated byte solely to bypass the released zero-argument C emitter defect; exercise the documented free(NULL) call.",
    )


def main() -> None:
    make_s1()
    make_s2()
    make_s14_retry()
    make_c13_retry()


if __name__ == "__main__":
    main()

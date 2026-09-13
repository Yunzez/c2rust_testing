#!/usr/bin/env python3
"""Package additional exact-source defects for the released FLOURINE oracle."""

from __future__ import annotations

import importlib.util
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
EASY_SPEC = importlib.util.spec_from_file_location(
    "easy", ROOT / "scripts/baseline_artifacts/make_flourine_easy_expansion.py"
)
WAVE2_SPEC = importlib.util.spec_from_file_location(
    "wave2", ROOT / "scripts/baseline_artifacts/make_flourine_wave2.py"
)
assert EASY_SPEC and EASY_SPEC.loader and WAVE2_SPEC and WAVE2_SPEC.loader
easy = importlib.util.module_from_spec(EASY_SPEC)
EASY_SPEC.loader.exec_module(easy)
wave2 = importlib.util.module_from_spec(WAVE2_SPEC)
WAVE2_SPEC.loader.exec_module(wave2)


def make_s4() -> None:
    c_path = ROOT / "benchmark/pairs/rq4/optipng_laertes/source/zlib/crc32.c"
    h_path = c_path.with_name("crc32.h")
    r_path = ROOT / "benchmark/pairs/rq4/optipng_laertes/translated/optipng_laertes.rs"
    c_text = c_path.read_text()
    r_text = r_path.read_text()

    c_target = easy.brace_item(c_text, "unsigned long ZEXPORT crc32_z(crc, buf, len)")
    c_little = easy.brace_item(c_text, "local unsigned long crc32_little(crc, buf, len)")
    c_big = easy.brace_item(c_text, "local unsigned long crc32_big(crc, buf, len)")
    r_table = wave2.through(r_text, "static mut crc_table:", ";}//;")
    r_target = easy.brace_item(r_text, 'pub unsafe extern "C" fn crc32_z')
    r_little = easy.brace_item(r_text, 'unsafe extern "C" fn crc32_little')
    r_big = easy.brace_item(r_text, 'unsafe extern "C" fn crc32_big')

    c_prelude = r"""
typedef unsigned int z_crc_t;
typedef unsigned long z_size_t;
#define local static
#define FAR
#define TBLS 8
#define BYFOUR
#define ZEXPORT
#define Z_NULL 0
#define DO1 crc = crc_table[0][((int)crc ^ (*buf++)) & 0xff] ^ (crc >> 8)
#define DO8 DO1; DO1; DO1; DO1; DO1; DO1; DO1; DO1
#define DOLIT4 c ^= *buf4++; c = crc_table[3][c & 0xff] ^ crc_table[2][(c >> 8) & 0xff] ^ crc_table[1][(c >> 16) & 0xff] ^ crc_table[0][c >> 24]
#define DOLIT32 DOLIT4; DOLIT4; DOLIT4; DOLIT4; DOLIT4; DOLIT4; DOLIT4; DOLIT4
#define DOBIG4 c ^= *buf4++; c = crc_table[4][c & 0xff] ^ crc_table[5][(c >> 8) & 0xff] ^ crc_table[6][(c >> 16) & 0xff] ^ crc_table[7][c >> 24]
#define DOBIG32 DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4
#define ZSWAP32(q) ((((q) >> 24) & 0xff) + (((q) >> 8) & 0xff00) + (((q) & 0xff00) << 8) + (((q) & 0xff) << 24))
static unsigned long crc32_little(unsigned long, const unsigned char *, z_size_t);
static unsigned long crc32_big(unsigned long, const unsigned char *, z_size_t);
"""
    c_wrapper = r"""
unsigned long crc32_z_packet(unsigned char packet[25]) {
    unsigned long crc = 0;
    memcpy(&crc, packet, sizeof(crc));
    z_size_t len = packet[24] % 17;
    return crc32_z(crc, packet + 8, len);
}
"""
    r_prelude = "pub type z_crc_t = u32; pub type z_size_t = u64; pub type ptrdiff_t = i64;"
    r_wrapper = r"""#[no_mangle]
pub fn crc32_z_packet(packet: &mut [u8; 25]) -> u64 {
    let crc = u64::from_ne_bytes(packet[0..8].try_into().unwrap());
    let len = (packet[24] % 17) as u64;
    unsafe { crc32_z(crc, packet[8..].as_ptr(), len) }
}"""
    easy.emit(
        "S4",
        "crc32_z_packet",
        ["#include <stddef.h>", "#include <string.h>"],
        [],
        ["unsigned long crc32_z_packet(unsigned char packet[25]);"],
        c_prelude
        + "\n"
        + h_path.read_text()
        + "\n"
        + c_target
        + "\n"
        + c_little
        + "\n"
        + c_big
        + c_wrapper,
        r_prelude,
        r_table + "\n" + r_target + "\n" + r_little + "\n" + r_big,
        r_wrapper,
        {
            "c": str(c_path.relative_to(ROOT)),
            "c_table": str(h_path.relative_to(ROOT)),
            "rust": str(r_path.relative_to(ROOT)),
        },
        "Decode a generated packet into CRC state, a 16-byte non-NULL buffer, and a length in [0,16]; call the exact target and its exact BYFOUR helpers on both sides.",
    )


def main() -> None:
    make_s4()


if __name__ == "__main__":
    main()

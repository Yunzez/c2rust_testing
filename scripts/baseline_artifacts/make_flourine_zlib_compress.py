#!/usr/bin/env python3
"""Package the C2SaferRust zlib compression defect for FLOURINE."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
PAIR = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/S19"
TARGET = "zlib_compress_observe"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def without_includes(source: str) -> str:
    return re.sub(
        r'^\s*#\s*include\s*[<"][^>"]+[>"].*$',
        "",
        source,
        flags=re.MULTILINE,
    )


def c_dependency_unit() -> str:
    source = PAIR / "source/zlib"
    headers = [
        "zconf.h",
        "zlib.h",
        "zutil.h",
        "deflate.h",
    ]
    implementations = [
        "adler32.c",
        "crc32.c",
        "zutil.c",
        "trees.c",
        "deflate.c",
        "compress.c",
    ]
    units = [without_includes((source / name).read_text()) for name in headers]
    for name in implementations:
        text = (source / name).read_text()
        if name == "trees.c":
            text = re.sub(
                r'^\s*#\s*include\s*"trees\.h".*$',
                (source / "trees.h").read_text(),
                text,
                flags=re.MULTILINE,
            )
        elif name == "crc32.c":
            text = re.sub(
                r'^\s*#\s*include\s*"crc32\.h".*$',
                (source / "crc32.h").read_text(),
                text,
                flags=re.MULTILINE,
            )
        units.append(without_includes(text))
    return "\n\n".join(units)


def main() -> None:
    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    json_path = input_dir / f"{TARGET}.json"
    rust_path = input_dir / f"{TARGET}.rs"
    raw_path = input_dir / f"{TARGET}_raw.inc"

    c_wrapper = r'''

int zlib_compress_observe(int8_t packet[512])
{
    uLong source_length = ((unsigned char)packet[0]) % 128;
    uLongf destination_length = 383;
    int result = compress2(
        (Bytef *)&packet[129],
        &destination_length,
        (const Bytef *)&packet[1],
        source_length,
        6);
    memcpy(&packet[0], &destination_length, sizeof(destination_length));
    return result;
}
'''
    document = {
        "Includes": [
            "#include <assert.h>",
            "#include <errno.h>",
            "#include <limits.h>",
            "#include <stdint.h>",
            "#include <stddef.h>",
            "#include <stdio.h>",
            "#include <stdlib.h>",
            "#include <string.h>",
            "#include <sys/types.h>",
        ],
        "Defines": [],
        "TypeDefs": [],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [f"int {TARGET}(int8_t packet[512]);"],
        "Function Implementations": [c_dependency_unit() + c_wrapper],
        "Enums": [],
    }
    json_path.write_text(json.dumps(document, indent=2) + "\n")

    translated_path = PAIR / "translated/optipng_c2saferrust.rs"
    translated = re.sub(
        r"(?ms)^#!\[[^\]]*\]\s*", "", translated_path.read_text()
    )
    raw_path.write_text(translated.rstrip() + "\n")
    rust_path.write_text(
        f'''#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/{raw_path.name}"];

#[no_mangle]
pub fn {TARGET}(packet: &mut [i8; 512]) -> i32 {{
    let source_length = (packet[0] as u8 % 128) as u64;
    let mut destination_length = 383u64;
    let result = unsafe {{
        compress2(
            packet.as_mut_ptr().add(129) as *mut u8,
            &mut destination_length,
            packet.as_ptr().add(1) as *const u8,
            source_length,
            6,
        )
    }};
    for (slot, value) in packet[..8]
        .iter_mut()
        .zip(destination_length.to_ne_bytes())
    {{
        *slot = value as i8;
    }}
    result
}}
'''
    )
    metadata = {
        "schema_version": 1,
        "defect": "S19",
        "baseline": "flourine",
        "target": TARGET,
        "adapter_kind": "documented_per_function_json_with_bounded_buffer_wrapper",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": "benchmark/pairs/rq4/optipng_c2saferrust/source/zlib/compress.c",
            "rust": "benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs",
        },
        "wrapper_contract": (
            "Generate all source and destination bytes, derive source length in "
            "[0,127], provide 383 destination bytes, and expose the original return, "
            "written length, and destination content."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()

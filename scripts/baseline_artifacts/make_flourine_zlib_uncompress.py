#!/usr/bin/env python3
"""Package zlib uncompress defects without a valid-stream seed."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
OUT_ROOT = ROOT / "results/baseline_artifacts/adapters/flourine"
TARGET = "zlib_uncompress_observe"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def without_includes(source: str) -> str:
    return re.sub(
        r'^\s*#\s*include\s*[<"][^>"]+[>"].*$',
        "",
        source,
        flags=re.MULTILINE,
    )


def c_dependency_unit(pair: Path) -> str:
    source = pair / "source/zlib"
    headers = [
        "zconf.h",
        "zlib.h",
        "zutil.h",
        "inftrees.h",
        "inflate.h",
        "inffast.h",
    ]
    implementations = [
        "adler32.c",
        "crc32.c",
        "zutil.c",
        "inftrees.c",
        "inffast.c",
        "inflate.c",
        "uncompr.c",
    ]
    units = [without_includes((source / name).read_text()) for name in headers]
    for name in implementations:
        text = (source / name).read_text()
        if name == "crc32.c":
            text = re.sub(
                r'^\s*#\s*include\s*"crc32\.h".*$',
                (source / "crc32.h").read_text(),
                text,
                flags=re.MULTILINE,
            )
        elif name == "inflate.c":
            text = re.sub(
                r'^\s*#\s*include\s*"inffixed\.h".*$',
                (source / "inffixed.h").read_text(),
                text,
                flags=re.MULTILINE,
            )
        units.append(without_includes(text))
    return "\n\n".join(units)


def rust_surface(raw_name: str) -> str:
    return f'''#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/{raw_name}"];

#[no_mangle]
pub fn {TARGET}(packet: &mut [i8; 512]) -> i32 {{
    let source_length = 1 + (packet[0] as u8 % 192) as u64;
    let mut destination_length = 319u64;
    let result = unsafe {{
        uncompress(
            packet.as_mut_ptr().add(193) as *mut u8,
            &mut destination_length,
            packet.as_ptr().add(1) as *const u8,
            source_length,
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


def emit(defect: str, pair_name: str) -> None:
    pair = ROOT / "benchmark/pairs/rq4" / pair_name
    out = OUT_ROOT / defect
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    json_path = input_dir / f"{TARGET}.json"
    rust_path = input_dir / f"{TARGET}.rs"
    raw_path = input_dir / f"{TARGET}_raw.inc"

    c_wrapper = r'''

int zlib_uncompress_observe(int8_t packet[512])
{
    uLong source_length = 1 + ((unsigned char)packet[0]) % 192;
    uLongf destination_length = 319;
    int result = uncompress(
        (Bytef *)&packet[193],
        &destination_length,
        (const Bytef *)&packet[1],
        source_length);
    memcpy(&packet[0], &destination_length, sizeof(destination_length));
    return result;
}
'''
    document = {
        "Includes": [
            "#include <assert.h>",
            "#include <errno.h>",
            "#include <limits.h>",
            "#include <stddef.h>",
            "#include <stdint.h>",
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
        "Function Implementations": [c_dependency_unit(pair) + c_wrapper],
        "Enums": [],
    }
    json_path.write_text(json.dumps(document, indent=2) + "\n")

    translated_path = next((pair / "translated").glob("*.rs"))
    translated = re.sub(
        r"(?ms)^#!\[[^\]]*\]\s*", "", translated_path.read_text()
    )
    raw_path.write_text(translated.rstrip() + "\n")
    rust_path.write_text(rust_surface(raw_path.name))
    metadata = {
        "schema_version": 1,
        "defect": defect,
        "baseline": "flourine",
        "target": TARGET,
        "adapter_kind": "documented_per_function_json_with_bounded_buffer_wrapper",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": str((pair / "source/zlib/uncompr.c").relative_to(ROOT)),
            "rust": str(translated_path.relative_to(ROOT)),
        },
        "wrapper_contract": (
            "Generate all source and destination bytes, derive source length in "
            "[1,192], provide 319 destination bytes, and expose the original "
            "return, written length, and destination content. No valid stream or "
            "known defect witness is seeded."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, raw_path)
        },
    }
    (out / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


def main() -> None:
    emit("S18", "optipng_laertes")
    emit("S20", "optipng_c2saferrust")


if __name__ == "__main__":
    main()

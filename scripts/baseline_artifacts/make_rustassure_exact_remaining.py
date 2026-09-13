#!/usr/bin/env python3
"""Build fresh exact-source RustAssure inputs for four remaining defects.

These inputs are deliberately not derived from earlier FLOURINE workarounds.
Each package retains the frozen C and Rust target bodies and adds only a small
same-contract surface that RustAssure can symbolize.  In particular, S14 keeps
three scalar arguments and C16 keeps the target's original void return type.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
from dataclasses import dataclass
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
OUT_ROOT = ROOT / "results/baseline_artifacts/adapters/rustassure"
RUSTASSURE_IMAGE = "c2r-baseline-rustassure:39618406"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def preprocess_c(source: str) -> str:
    """Preprocess with the same frozen Clang toolchain that consumes `.i`."""
    completed = subprocess.run(
        [
            "docker",
            "run",
            "--rm",
            "--cpus",
            "1",
            "-i",
            RUSTASSURE_IMAGE,
            "clang",
            "-E",
            "-P",
            "-x",
            "c",
            "-",
        ],
        input=source,
        text=True,
        capture_output=True,
        check=True,
    )
    return completed.stdout


def balanced_function(path: Path, marker: str) -> str:
    """Return the exact function text beginning at marker through its body."""
    text = path.read_text()
    start = text.index(marker)
    opening = text.index("{", start)
    depth = 0
    for index in range(opening, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise RuntimeError(f"unterminated function after {marker!r} in {path}")


@dataclass(frozen=True)
class Package:
    defect: str
    target: str
    arity: int
    c_source: Path
    rust_source: Path
    c_marker: str
    rust_marker: str
    c_prelude: str
    c_surface: str
    rust_surface: str
    notes: tuple[str, ...]


OPTIPNG_C = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/source"
OPTIPNG_R = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs"
BZIP2_C = ROOT / "benchmark/pairs/rq4/bzip2_c2saferrust/source/blocksort.c"
BZIP2_R = ROOT / "benchmark/pairs/rq4/bzip2_c2saferrust/translated/bzip2_c2saferrust.rs"


PACKAGES = (
    Package(
        defect="C16",
        target="optimize_cmf_packet",
        arity=2,
        c_source=OPTIPNG_C / "libpng/pngwutil.c",
        rust_source=OPTIPNG_R,
        c_marker="static void\noptimize_cmf(",
        rust_marker="fn optimize_cmf(",
        c_prelude="""#include <stddef.h>\n#include <stdint.h>\n\ntypedef uint8_t png_byte;\ntypedef png_byte *png_bytep;\ntypedef size_t png_alloc_size_t;""",
        c_surface="""void optimize_cmf_packet(uint8_t data[2], size_t data_size)\n{\n    optimize_cmf(data, data_size);\n}""",
        rust_surface="""#[no_mangle]\npub fn optimize_cmf_packet(data: &mut [u8; 2], data_size: usize) {\n    optimize_cmf(data, data_size);\n}""",
        notes=(
            "The exact optimize_cmf bodies are copied from the frozen C2SaferRust pair.",
            "The two-byte array supplies exactly the storage read and written by the target; data_size remains symbolic and unconstrained.",
            "No valid zlib header or defect-triggering size is seeded.",
        ),
    ),
    Package(
        defect="S16",
        target="opng_strcasecmp_packet",
        arity=2,
        c_source=OPTIPNG_C / "optipng/optipng.c",
        rust_source=OPTIPNG_R,
        c_marker="static int\nopng_strcasecmp(",
        rust_marker="fn opng_strcasecmp(",
        c_prelude="#include <ctype.h>\n#include <stdint.h>",
        c_surface="""int opng_strcasecmp_packet(int8_t str1[16], int8_t str2[16])\n{\n    str1[15] = 0;\n    str2[15] = 0;\n    return opng_strcasecmp((const char *)str1, (const char *)str2);\n}""",
        rust_surface="""#[no_mangle]\npub fn opng_strcasecmp_packet(str1: &mut [i8; 16], str2: &mut [i8; 16]) -> i32 {\n    str1[15] = 0;\n    str2[15] = 0;\n    opng_strcasecmp(str1.as_ptr(), str2.as_ptr())\n}""",
        notes=(
            "The exact opng_strcasecmp bodies are copied from the frozen C2SaferRust pair.",
            "Both generated arrays are terminated only at their final byte, realizing the original C-string contract without fixing their contents.",
        ),
    ),
    Package(
        defect="S14",
        target="mmed3_observe",
        arity=3,
        c_source=BZIP2_C,
        rust_source=BZIP2_R,
        c_marker="static \n__inline__\nUChar mmed3",
        rust_marker=" fn mmed3(",
        c_prelude="typedef unsigned char UChar;",
        c_surface="""UChar mmed3_observe(UChar a, UChar b, UChar c)\n{\n    return mmed3(a, b, c);\n}""",
        rust_surface="""#[no_mangle]\npub fn mmed3_observe(a: u8, b: u8, c: u8) -> u8 {\n    mmed3(a, b, c)\n}""",
        notes=(
            "The exact mmed3 bodies are copied from the frozen C2SaferRust pair.",
            "The observation surface preserves the original three scalar inputs and return value; it does not bundle or widen them.",
        ),
    ),
    Package(
        defect="C13",
        target="opng_free_choice",
        arity=1,
        c_source=OPTIPNG_C / "optipng/optim.c",
        rust_source=OPTIPNG_R,
        c_marker="static void\nopng_free(",
        rust_marker="fn opng_free(",
        c_prelude="#include <stdint.h>\n#include <stdlib.h>",
        c_surface="""int opng_free_choice(uint8_t choose_nonnull)\n{\n    void *ptr = choose_nonnull ? malloc(1) : NULL;\n    opng_free(ptr);\n    return 0;\n}""",
        rust_surface="""#[no_mangle]\npub fn opng_free_choice(choose_nonnull: u8) -> i32 {\n    let ptr = if choose_nonnull != 0 {\n        Box::into_raw(Box::new(0u8)) as *mut std::os::raw::c_void\n    } else {\n        std::ptr::null_mut()\n    };\n    opng_free(ptr);\n    0\n}""",
        notes=(
            "The exact opng_free bodies are copied from the frozen C2SaferRust pair.",
            "A symbolic scalar selects either NULL or a valid one-byte heap allocation, covering both branches of free's original contract without hard-coding the defect witness.",
        ),
    ),
)


def emit(package: Package) -> None:
    out = OUT_ROOT / package.defect
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    for path in input_dir.iterdir():
        if path.is_file():
            path.unlink()

    c_body = balanced_function(package.c_source, package.c_marker)
    rust_body = balanced_function(package.rust_source, package.rust_marker)
    raw_name = f"{package.target}_raw.inc"
    c_path = input_dir / f"{package.target}.i"
    rust_path = input_dir / f"{package.target}.rs"
    raw_path = input_dir / raw_name

    c_path.write_text(
        preprocess_c(
            f"{package.c_prelude}\n\n{c_body}\n\n{package.c_surface}\n"
        )
    )
    raw_path.write_text(rust_body + "\n")
    rust_path.write_text(
        f'include!["/input/{raw_name}"];\n\n{package.rust_surface}\n'
    )

    map_path = out / "argument_order_map.json"
    map_path.write_text(
        json.dumps(
            {
                package.target: {
                    str(index): str(index) for index in range(package.arity)
                }
            },
            indent=2,
        )
        + "\n"
    )

    copied = (c_path, rust_path, raw_path, map_path)
    source = lambda path: str(path.relative_to(ROOT))
    record = {
        "schema_version": 1,
        "defect": package.defect,
        "baseline": "rustassure",
        "target": package.target,
        "adapter_kind": "documented_individual_function_input_with_contract_surface",
        "semantic_rewrite": False,
        "sources": {
            "c": source(package.c_source),
            "rust": source(package.rust_source),
        },
        "notes": list(package.notes),
        "preprocessing": {
            "command": "docker run --rm --cpus 1 -i c2r-baseline-rustassure:39618406 clang -E -P -x c -",
            "purpose": "RustAssure documents preprocessed .i files as its C input format; using its frozen Clang 14 avoids host-header drift",
        },
        "input_sha256": {path.name: sha256(path) for path in copied},
    }
    (out / "adapter.json").write_text(json.dumps(record, indent=2) + "\n")


def main() -> None:
    for package in PACKAGES:
        emit(package)


if __name__ == "__main__":
    main()

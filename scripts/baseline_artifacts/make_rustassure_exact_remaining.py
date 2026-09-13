#!/usr/bin/env python3
"""Build fresh exact-source RustAssure inputs for remaining defects.

These inputs are deliberately not derived from earlier FLOURINE workarounds.
Each package retains the frozen C and Rust target bodies and adds only a small
same-contract surface that RustAssure can symbolize.  In particular, S14 keeps
three scalar arguments and C16 keeps the target's original void return type.
"""

from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import tempfile
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
LIL_C = ROOT / "tools/frameworks/crown/c-code/lil/main.c"
LIL_R = ROOT / "benchmark/pairs/rq4/lil_c2saferrust/translated/lil_c2saferrust.rs"
CJSON_C = ROOT / "benchmark/pairs/rq4/cjson_ptrtrans/source/cJSON.c"
CJSON_H = ROOT / "benchmark/pairs/rq4/cjson_ptrtrans/source/cJSON.h"
CJSON_R = ROOT / "benchmark/pairs/rq4/cjson_ptrtrans/translated/cjson_ptrtrans.rs"
TULIP_REPO = ROOT / "tools/frameworks/tulipindicators"
TULIP_WIP = ROOT / "tools/frameworks/c2saferrust/laertes_benchmarks/tulipindicators_WIP"
TULIP_PAIR_C = ROOT / "benchmark/pairs/rq4/tulip_c2saferrust/source"
TULIP_SOURCE_COMMIT = "41e59fb33cef5bc97b03d2751dab2b006525c23f"


PACKAGES = (
    Package(
        defect="C3",
        target="do_system_two",
        arity=2,
        c_source=LIL_C,
        rust_source=LIL_R,
        c_marker="static char* do_system(",
        rust_marker="fn do_system(",
        c_prelude="""#include <stddef.h>\n#include <stdint.h>\n#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n#include <sys/types.h>""",
        c_surface="""char *do_system_two(int8_t first[16], int8_t second[16])\n{\n    char *argv[2];\n    first[15] = 0;\n    second[15] = 0;\n    argv[0] = (char *)first;\n    argv[1] = (char *)second;\n    return do_system(2, argv);\n}""",
        rust_surface="""#[no_mangle]\npub fn do_system_two(first: &mut [i8; 16], second: &mut [i8; 16]) -> *mut std::os::raw::c_char {\n    first[15] = 0;\n    second[15] = 0;\n    let mut argv = [first.as_mut_ptr(), second.as_mut_ptr()];\n    do_system(2, argv.as_mut_ptr())\n}""",
        notes=(
            "The exact do_system bodies are copied from the frozen C source and C2SaferRust translation.",
            "Two unconstrained byte arrays are terminated only at their final byte and assembled into the original argc/argv contract.",
            "The adapter does not replace or mock popen/system-command execution; any released-analyzer limitation at that call remains visible.",
        ),
    ),
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


def emit_s8_direct() -> None:
    """Submit parse_string directly so RustAssure, not an adapter, observes state."""
    defect = "S8"
    target = "parse_string"
    out = OUT_ROOT / defect
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    for path in input_dir.iterdir():
        if path.is_file():
            path.unlink()

    source = CJSON_C.read_text()
    include_lines = ('#include "cJSON.h"', "#include <cJSON.h>")
    source = "\n".join(
        line for line in source.splitlines() if line.strip() not in include_lines
    ) + "\n"
    c_path = input_dir / f"{target}.i"
    rust_path = input_dir / f"{target}.rs"
    c_path.write_text(preprocess_c(CJSON_H.read_text() + "\n" + source))
    rust_path.write_bytes(CJSON_R.read_bytes())

    map_path = out / "argument_order_map.json"
    map_path.write_text(
        json.dumps({target: {"0": "0", "1": "1"}}, indent=2) + "\n"
    )
    copied = (c_path, rust_path, map_path)
    record = {
        "schema_version": 1,
        "defect": defect,
        "baseline": "rustassure",
        "target": target,
        "adapter_kind": "documented_direct_individual_function_input",
        "semantic_rewrite": False,
        "sources": {
            "c": str(CJSON_C.relative_to(ROOT)),
            "c_header": str(CJSON_H.relative_to(ROOT)),
            "rust": str(CJSON_R.relative_to(ROOT)),
        },
        "notes": [
            "The original parse_string signature, return value, mutable cJSON argument, and parse_buffer argument are submitted without an observation wrapper.",
            "This leaves construction of recursive Rust references, allocator hooks, and post-call argument-state observation entirely to released RustAssure.",
            "No known witness or output proxy is added; a compiler or symbolizer failure is scored as released-baseline failure.",
        ],
        "preprocessing": {
            "command": "docker run --rm --cpus 1 -i c2r-baseline-rustassure:39618406 clang -E -P -x c -",
            "purpose": "RustAssure documents preprocessed .i files as its C input format; the local cJSON header is concatenated before preprocessing",
        },
        "input_sha256": {path.name: sha256(path) for path in copied},
    }
    (out / "adapter.json").write_text(json.dumps(record, indent=2) + "\n")


def emit_c6_cli() -> None:
    """Package the exact Tulip 0.8.4 sample driver and C2SaferRust main_0.

    The surface supplies a small but complete C argv: argc is generated in
    1..4, argv[0] is always present, argv[argc] is NULL, and all backing strings
    are independently generated except for their final NUL.  In particular,
    argc is not fixed to the known failing value.

    The Rust input deliberately retains the translation crate's original
    nightly feature attributes.  Removing them after a released RustAssure
    compiler failure would be a forbidden baseline workaround.
    """
    defect = "C6"
    target = "sample_main_packet"
    out = OUT_ROOT / defect
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    for path in input_dir.iterdir():
        if path.is_file():
            path.unlink()

    historical_sample = subprocess.run(
        ["git", "-C", str(TULIP_REPO), "show", f"{TULIP_SOURCE_COMMIT}:sample.c"],
        text=True,
        capture_output=True,
        check=True,
    ).stdout
    c_surface = r"""
int sample_main_packet(unsigned char argc_selector,
                       signed char arg0[32], signed char arg1[32],
                       signed char arg2[32], signed char arg3[32])
{
    char *argv[5];
    int argc;
    arg0[31] = 0;
    arg1[31] = 0;
    arg2[31] = 0;
    arg3[31] = 0;
    argv[0] = (char *)arg0;
    argv[1] = (char *)arg1;
    argv[2] = (char *)arg2;
    argv[3] = (char *)arg3;
    argv[4] = 0;
    argc = 1 + (argc_selector % 4);
    argv[argc] = 0;
    return sample_main_impl(argc, argv);
}
"""
    combined_c = (
        f'#include "/repo/{TULIP_PAIR_C.relative_to(ROOT)}/tulip.c"\n'
        "#define main sample_main_impl\n"
        + historical_sample
        + "\n#undef main\n"
        + c_surface
    )
    completed = subprocess.run(
        [
            "docker", "run", "--rm", "--cpus", "1", "-i",
            "-v", f"{ROOT}:/repo:ro", RUSTASSURE_IMAGE,
            "clang", "-E", "-P", "-x", "c",
            "-I", f"/repo/{TULIP_PAIR_C.relative_to(ROOT)}", "-",
        ],
        input=combined_c,
        text=True,
        capture_output=True,
        check=True,
    )
    c_path = input_dir / f"{target}.i"
    c_path.write_text(completed.stdout)

    rust_surface = r"""
#[no_mangle]
pub fn sample_main_packet(
    argc_selector: u8,
    arg0: &mut [i8; 32],
    arg1: &mut [i8; 32],
    arg2: &mut [i8; 32],
    arg3: &mut [i8; 32],
) -> std::os::raw::c_int {
    arg0[31] = 0;
    arg1[31] = 0;
    arg2[31] = 0;
    arg3[31] = 0;
    let mut argv = [
        arg0.as_mut_ptr(), arg1.as_mut_ptr(), arg2.as_mut_ptr(),
        arg3.as_mut_ptr(), std::ptr::null_mut(),
    ];
    let argc = 1 + (argc_selector as i32 % 4);
    argv[argc as usize] = std::ptr::null_mut();
    unsafe { main_0(argc, argv.as_mut_ptr()) }
}
"""
    with tempfile.TemporaryDirectory() as temporary:
        stage = Path(temporary)
        for name in ("example1.rs", "example2.rs", "fuzzer.rs", "indicators_index.rs"):
            shutil.copy2(TULIP_WIP / name, stage / name)
        shutil.copytree(TULIP_WIP / "indicators", stage / "indicators")
        shutil.copytree(TULIP_WIP / "utils", stage / "utils")
        (stage / "sample.rs").write_text(
            (TULIP_WIP / "sample.rs").read_text() + "\n" + rust_surface
        )
        modules = [
            "example1", "example2", "fuzzer", "indicators_index", "utils/buffer",
            *[f"indicators/{path.stem}" for path in sorted((stage / "indicators").glob("*.rs"))],
        ]
        flattened = stage / f"{target}.rs"
        subprocess.run(
            [
                "python3", str(ROOT / "scripts/flatten_translation.py"),
                str(stage), str(flattened),
                "--lib-modules", ",".join(modules),
                "--extra-modules", "sample",
            ],
            check=True,
        )
        original_attributes = []
        for line in (TULIP_WIP / "c2rust-lib.rs").read_text().splitlines():
            if line.startswith("#!["):
                original_attributes.append(line)
            elif line.startswith("pub mod"):
                break
        flattened_lines = flattened.read_text().splitlines()
        first_module = next(
            index for index, line in enumerate(flattened_lines)
            if line.startswith("pub mod ")
        )
        rust_path = input_dir / f"{target}.rs"
        rust_path.write_text(
            "\n".join(original_attributes)
            + "\n"
            + "\n".join(flattened_lines[first_module:])
            + "\n"
        )

    map_path = out / "argument_order_map.json"
    map_path.write_text(
        json.dumps({target: {str(index): str(index) for index in range(5)}}, indent=2)
        + "\n"
    )
    source_sample_sha = hashlib.sha256(historical_sample.encode()).hexdigest()
    copied = (c_path, rust_path, map_path)
    record = {
        "schema_version": 1,
        "defect": defect,
        "baseline": "rustassure",
        "target": target,
        "adapter_kind": "documented_individual_function_input_with_argv_contract_surface",
        "semantic_rewrite": False,
        "sources": {
            "c_library": str(TULIP_PAIR_C.relative_to(ROOT)),
            "c_sample_commit": TULIP_SOURCE_COMMIT,
            "c_sample_path": "sample.c",
            "c_sample_sha256": source_sample_sha,
            "rust_crate": str(TULIP_WIP.relative_to(ROOT)),
            "rust_sample": str((TULIP_WIP / "sample.rs").relative_to(ROOT)),
        },
        "identity_evidence": [
            "sample.c is byte-identical from Tulip commit 41e59fb3 through 0bc8dfc4",
            "the frozen base c2rust sample reports TI_VERSION 0.8.4 and TI_BUILD 1537377628 and preserves the historical sample.c guards and messages",
            "the C2SaferRust WIP sample retains the same constants and computation but moves the argv[1] read before the argc guard",
        ],
        "notes": [
            "The exact historical C sample main and exact C2SaferRust WIP main_0 are retained with their complete library dependency closures.",
            "A generated selector ranges over argc 1..4; four independently generated 32-byte strings are only final-byte NUL terminated and assembled into a valid argv.",
            "No argc value, indicator name, option, or known defect witness is fixed.",
            "The Rust translation's original nightly crate attributes are retained; a released RustAssure compiler failure must be scored rather than repaired.",
        ],
        "preprocessing": {
            "command": "docker run --rm --cpus 1 -i -v REPO:/repo:ro c2r-baseline-rustassure:39618406 clang -E -P -x c -I TULIP_PAIR_SOURCE -",
            "purpose": "RustAssure requires a preprocessed .i input; the exact frozen Clang 14 is used",
        },
        "input_sha256": {path.name: sha256(path) for path in copied},
    }
    (out / "adapter.json").write_text(json.dumps(record, indent=2) + "\n")


def main() -> None:
    for package in PACKAGES:
        emit(package)
    emit_s8_direct()
    emit_c6_cli()


if __name__ == "__main__":
    main()

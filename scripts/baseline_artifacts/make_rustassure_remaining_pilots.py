#!/usr/bin/env python3
"""Package the S6, S17, and C12 pilot pairs for released RustAssure."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def brace_item(text: str, marker: str) -> str:
    start = text.index(marker)
    brace = text.index("{", start)
    depth = 0
    for index in range(brace, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise ValueError(f"unclosed item: {marker}")


def emit(
    defect: str,
    target: str,
    c_text: str,
    rust_text: str,
    arity: int,
    sources: dict[str, str],
    notes: list[str],
) -> None:
    out = ROOT / f"results/baseline_artifacts/adapters/rustassure/{defect}"
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    c_path = input_dir / f"{target}.i"
    rust_path = input_dir / f"{target}.rs"
    map_path = out / "argument_order_map.json"
    c_path.write_text(c_text.rstrip() + "\n")
    rust_path.write_text(rust_text.rstrip() + "\n")
    map_path.write_text(
        json.dumps({target: {str(i): str(i) for i in range(arity)}}, indent=2) + "\n"
    )
    metadata = {
        "schema_version": 1,
        "defect": defect,
        "baseline": "rustassure",
        "target": target,
        "adapter_kind": "documented_individual_function_input_manual_argument_order",
        "semantic_rewrite": False,
        "sources": sources,
        "notes": notes,
        "input_sha256": {
            c_path.name: sha256(c_path),
            rust_path.name: sha256(rust_path),
            map_path.name: sha256(map_path),
        },
    }
    (out / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


def make_s6() -> None:
    c_path = ROOT / "results/rq4_effectiveness/bugs/qsort_ptrtrans/original_qsort.c"
    rust_path = ROOT / "results/rq4_effectiveness/bugs/qsort_ptrtrans/translated_qsort.rs"
    c_text = c_path.read_text().rstrip()
    c_text += "\n\nvoid quick_sort(int *arr, int low, int high)\n{\n    quickSort(arr, low, high);\n}\n"
    emit(
        "S6",
        "quick_sort",
        c_text,
        rust_path.read_text(),
        3,
        {"c": str(c_path.relative_to(ROOT)), "rust": str(rust_path.relative_to(ROOT))},
        [
            "The C name adapter forwards the three arguments unchanged because the frozen Rust translation renamed quickSort to quick_sort.",
            "The qsort bodies are copied unchanged; the positional map is identity.",
        ],
    )


def make_s17() -> None:
    c_path = ROOT / "benchmark/pairs/rq4/optipng_laertes/source/optipng/ioutil.c"
    rust_path = ROOT / "benchmark/pairs/rq4/optipng_laertes/translated/optipng_laertes.rs"
    c_fn = brace_item(c_path.read_text(), "char *\nopng_path_make_backup")
    rust_fn = brace_item(
        rust_path.read_text(), '#[no_mangle]\npub unsafe extern "C" fn opng_path_make_backup'
    )
    emit(
        "S17",
        "opng_path_make_backup",
        'typedef unsigned long size_t;\n#define NULL ((void *)0)\n#define OPNG_PATH_EXTSEP_STR "."\n'
        'size_t strlen(const char *);\nchar *strcpy(char *, const char *);\nchar *strcat(char *, const char *);\n\n'
        + c_fn,
        """extern "C" {
    fn strlen(value: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn strcpy(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}

""" + rust_fn,
        3,
        {"c": str(c_path.relative_to(ROOT)), "rust": str(rust_path.relative_to(ROOT))},
        [
            "The target bodies are copied unchanged; only includes, the platform macro, and libc declarations are supplied.",
            "The unused nightly crate attributes from the monolithic translation are omitted; this compiler accepts core::intrinsics::transmute as a deprecated stable path.",
        ],
    )


def make_c12() -> None:
    c_path = ROOT / "benchmark/pairs/rq4/urlparser_laertes/source/url.h"
    rust_path = ROOT / "benchmark/pairs/rq4/urlparser_laertes/translated/urlparser_laertes.rs"
    c_all = c_path.read_text()
    c_start = c_all.index("char *URL_SCHEMES[] = {")
    c_end = c_all.index("\n};", c_start) + len("\n};")
    c_fn = brace_item(c_all, "bool\nurl_is_protocol (char *str) {")
    rust_all = rust_path.read_text()
    rust_start = rust_all.index("#[no_mangle]\npub static mut URL_SCHEMES")
    rust_end = rust_all.index("; unsafe fn laertes_init_URL_SCHEMES", rust_start) + 1
    rust_global = rust_all[rust_start:rust_end]
    rust_fn = brace_item(
        rust_all, '#[no_mangle]\npub unsafe extern "C" fn url_is_protocol'
    )
    emit(
        "C12",
        "url_is_protocol",
        "typedef _Bool bool;\n#define true 1\n#define false 0\nint strcmp(const char *, const char *);\n\n"
        + c_all[c_start:c_end] + "\n\n" + c_fn,
        """extern "C" {
    fn strcmp(left: *const std::os::raw::c_char, right: *const std::os::raw::c_char) -> std::os::raw::c_int;
}

""" + rust_global + "\n\n" + rust_fn,
        1,
        {"c": str(c_path.relative_to(ROOT)), "rust": str(rust_path.relative_to(ROOT))},
        [
            "The C initialized table and function are copied from the frozen source.",
            "The Rust all-NULL table and target body are copied from the frozen translation.",
            "The emitted but never-called initializer is omitted from the single-function package; omitting unreachable code preserves the defective initial state.",
        ],
    )


def main() -> None:
    make_s6()
    make_s17()
    make_c12()


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Submit frozen C/Rust defect pairs to VERT's released checker pipeline.

This is an external-input adapter, not a repair of VERT.  It packages the
already-audited per-function payloads used for the other baselines, invokes
VERT's C -> WASI -> rWasm path, and injects the frozen Rust candidate at the
slot occupied by VERT's generated candidate.  The adapter deliberately keeps
the released harness semantics: arrays are represented as two elements,
only PARAM1 is injected into the rWasm reference, and only return values are
compared.  A failing candidate is therefore a detection only when an
independently generated correct-candidate control passes the same harness.

Raw build products live outside the repository.  Small, atomic JSON records
are written under results/baseline_artifacts/runs/vert_external/.
"""

from __future__ import annotations

import argparse
import difflib
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time
from typing import Any


REPO = Path(__file__).resolve().parents[2]
ADAPTERS = REPO / "results/baseline_artifacts/adapters"
RESULTS = REPO / "results/baseline_artifacts/runs/vert_external"
RAW_ROOT = Path("/home/yunzez/c2rust_baselines/runs/vert/external_pairs")
SHARED_CARGO_TARGET = RAW_ROOT / "cargo-target-shared"
VERT = Path("/home/yunzez/c2rust_baselines/checkouts/vert-artifact/vert")
WASI = Path("/home/yunzez/c2rust_baselines/toolchains/wasi-sdk-12.0")
RWASM = Path(
    "/home/yunzez/c2rust_baselines/checkouts/rwasm-official/target/release/rwasm"
)
CARGO_HOME = Path(
    "/home/yunzez/c2rust_baselines/runs/vert/cargo-home-cli-0.10.0"
)

DEFECTS = [
    *(f"C{i}" for i in range(1, 17) if i != 14),
    *(f"S{i}" for i in range(1, 22)),
]

# A structured C payload can be shared when the frozen defects have the same
# original C wrapper.  The Rust candidate always comes from the requested ID.
C_SOURCE_ALIAS = {
    "C15": "S20",
    "S3": "C7",
    "S7": "cjson_parse_string_ascii",
    "S8": "cjson_parse_string",
    "S9": "cjson_parse_string",
    "S10": "C7",
}


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def atomic_json(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(prefix=path.name + ".", dir=path.parent)
    try:
        with os.fdopen(fd, "w") as f:
            json.dump(value, f, indent=2, sort_keys=True)
            f.write("\n")
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp_name, path)
    finally:
        if os.path.exists(tmp_name):
            os.unlink(tmp_name)


def run(
    command: list[str],
    *,
    cwd: Path,
    log: Path,
    timeout: int = 600,
    env: dict[str, str] | None = None,
) -> dict[str, Any]:
    started = time.time()
    merged_env = os.environ.copy()
    merged_env["CARGO_BUILD_JOBS"] = "4"
    merged_env["CARGO_HOME"] = str(CARGO_HOME)
    if env:
        merged_env.update(env)
    try:
        proc = subprocess.run(
            command,
            cwd=cwd,
            env=merged_env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=timeout,
        )
        output = proc.stdout
        rc: int | None = proc.returncode
        timed_out = False
    except subprocess.TimeoutExpired as exc:
        output = (exc.stdout or "") + "\n[TIMEOUT]\n"
        rc = None
        timed_out = True
    log.parent.mkdir(parents=True, exist_ok=True)
    log.write_text(output)
    return {
        "command": command,
        "cwd": str(cwd),
        "exit_code": rc,
        "timeout": timed_out,
        "seconds": round(time.time() - started, 3),
        "log": str(log),
    }


def flourine_dir(defect: str) -> Path | None:
    key = C_SOURCE_ALIAS.get(defect, defect)
    path = ADAPTERS / "flourine" / key
    return path if path.is_dir() else None


def rustassure_dir(defect: str) -> Path | None:
    path = ADAPTERS / "rustassure" / defect
    return path if path.is_dir() else None


def load_payload(defect: str) -> dict[str, Any]:
    """Locate audited C and Rust inputs without inventing a new defect body."""
    rdir = rustassure_dir(defect)
    cdir = flourine_dir(defect)
    if rdir is None:
        return {
            "ok": False,
            "classification": "unsupported_input",
            "reason": "no audited individual-function payload (process-entry defect)",
        }
    if cdir is None:
        return {
            "ok": False,
            "classification": "unsupported_input",
            "reason": "no structured C payload for VERT's C-to-Wasm stage",
        }

    radapter = json.loads((rdir / "adapter.json").read_text())
    cadapter = json.loads((cdir / "adapter.json").read_text())
    cjson = next((cdir / "input").glob("*.json"), None)
    if cjson is None:
        raise RuntimeError(f"{defect}: missing structured C JSON")
    cobj = json.loads(cjson.read_text())
    declarations = cobj.get("Function Declarations", [])
    if not declarations:
        raise RuntimeError(f"{defect}: structured C payload has no declaration")

    # When FLOURINE already has an exact adapter for this defect, keep its C
    # and Rust halves together.  Aliased C sources (same original C wrapper,
    # different defective translation) use the requested RustAssure half.
    direct_c_adapter = C_SOURCE_ALIAS.get(defect, defect) == defect
    rust_owner = cdir if direct_c_adapter else rdir
    rust_file = next((rust_owner / "input").glob("*.rs"), None)
    if rust_file is None:
        raise RuntimeError(f"{defect}: missing Rust candidate")

    target = cadapter["target"] if direct_c_adapter else radapter["target"]

    # FLOURINE's C13 adapter fixes the call to free(NULL), whereas the audited
    # RustAssure adapter exposes the full NULL/non-NULL contract through one
    # scalar.  The latter is the only C13 packaging that fits VERT's native
    # one-scalar envelope, so use both audited halves from that package.
    if defect == "C13":
        r_i = next((rdir / "input").glob("*.i"))
        text = r_i.read_text()
        marker = text.rfind("static void\nopng_free")
        if marker < 0:
            raise RuntimeError("C13: audited opng_free body not found")
        cobj = {
            "Includes": ["#include <stdint.h>", "#include <stdlib.h>"],
            "Defines": [],
            "TypeDefs": [],
            "Enums": [],
            "Structs": [],
            "Globals": [],
            "Function Declarations": ["int opng_free_choice(uint8_t choose_nonnull);"],
            "Function Implementations": [text[marker:]],
        }
        cjson = r_i
        rust_owner = rdir
        rust_file = next((rdir / "input").glob("*.rs"))
        target = radapter["target"]
        declarations = cobj["Function Declarations"]

    return {
        "ok": True,
        "rdir": rdir,
        "cdir": cdir,
        "radapter": radapter,
        "cadapter": cadapter,
        "cjson": cjson,
        "cobj": cobj,
        "declaration": declarations[0],
        "target": target,
        "rust_file": rust_file,
        "rust_owner": rust_owner,
    }


def render_c(cobj: dict[str, Any]) -> str:
    order = [
        "Includes",
        "Defines",
        "TypeDefs",
        "Enums",
        "Structs",
        "Globals",
        "Function Declarations",
        "Function Implementations",
    ]
    chunks: list[str] = []
    for key in order:
        chunks.extend(str(item) for item in cobj.get(key, []))
    return "\n\n".join(chunks) + "\n"


def split_args(text: str) -> list[str]:
    out: list[str] = []
    depth = 0
    start = 0
    for i, ch in enumerate(text):
        if ch in "([":
            depth += 1
        elif ch in ")]":
            depth -= 1
        elif ch == "," and depth == 0:
            out.append(text[start:i].strip())
            start = i + 1
    tail = text[start:].strip()
    if tail:
        out.append(tail)
    return out


def parse_decl(decl: str) -> dict[str, Any]:
    """Parse only the compact declarations emitted by our audited adapters."""
    compact = " ".join(decl.replace(";", "").split())
    match = re.match(r"^(?P<ret>.+?)\s+\*?(?P<name>[A-Za-z_]\w*)\s*\((?P<args>.*)\)$", compact)
    if not match:
        raise ValueError(f"unparsed declaration: {decl}")
    ret_prefix = compact[: compact.index(match.group("name"))].strip()
    args_text = match.group("args").strip()
    args: list[dict[str, Any]] = []
    if args_text and args_text != "void":
        for raw in split_args(args_text):
            array = re.search(r"\[\s*(\d*)\s*\]\s*$", raw)
            without_array = raw[: array.start()].strip() if array else raw
            name_match = re.search(r"([A-Za-z_]\w*)\s*$", without_array)
            if not name_match:
                raise ValueError(f"unparsed argument: {raw}")
            name = name_match.group(1)
            ctype = without_array[: name_match.start()].strip()
            args.append(
                {
                    "raw": raw,
                    "name": name,
                    "ctype": ctype,
                    "array": array is not None,
                    "extent": int(array.group(1)) if array and array.group(1) else None,
                }
            )
    return {"name": match.group("name"), "return": ret_prefix, "args": args}


def vert_type(ctype: str, *, array: bool = False) -> str | None:
    """The closed type vocabulary implemented by the released VERT harness."""
    normalized = " ".join(ctype.replace("const", "").split())
    integer = {
        "int": "i32",
        "bool": "i32",
        "signed int": "i32",
        "unsigned": "i32",
        "unsigned int": "i32",
        "char": "i32",
        "signed char": "i32",
        "unsigned char": "i32",
        "int8_t": "i32",
        "uint8_t": "i32",
        "uLong": "i32",       # VERT maps C long to wasm32 i32.
        "z_off_t": "i32",
        "size_t": "i32",
        "long": "i32",
        "unsigned long": "i32",
    }
    if normalized in ("float", "double"):
        base = "f32"
    else:
        base = integer.get(normalized)
    if base is None:
        return None
    return f"[{base}; 2]" if array else base


def make_main(parsed: dict[str, Any], mutated: bool) -> tuple[str, dict[str, Any]]:
    """Create VERT's two constant-call programs.

    As in the released pipeline, only one changed constant is later replaced
    by PARAM1.  Other arguments remain constants on the reference side.
    """
    declarations: list[str] = []
    call_args: list[str] = []
    mutation_done = False
    for index, arg in enumerate(parsed["args"], start=1):
        base = vert_type(arg["ctype"], array=False)
        if base is None:
            raise TypeError(f"unsupported C argument type: {arg['ctype']}")
        if arg["array"]:
            # VERT models every array as two elements.  Keeping this bound is
            # intentional even when the audited wrapper has a larger extent.
            elem = "float" if base == "f32" else (
                "unsigned int" if base == "u32" else "int"
            )
            first = "29.0" if mutated and not mutation_done and base == "f32" else (
                "29" if mutated and not mutation_done else ("1.0" if base == "f32" else "1")
            )
            second = "2.0" if base == "f32" else "2"
            declarations.append(f"    {elem} vert_arg_{index}[2] = {{{first}, {second}}};")
            call_args.append(f"vert_arg_{index}")
            if not mutation_done:
                mutation_done = True
        else:
            literal = "29" if mutated and not mutation_done else str(index)
            if base == "f32":
                literal += ".0"
            call_args.append(literal)
            if not mutation_done:
                mutation_done = True
    if not parsed["args"]:
        return "int main(void) { f_gold(); return 0; }\n", {"mutated": False}
    body = "\n".join(declarations)
    call = f"    f_gold({', '.join(call_args)});"
    return f"int main(void) {{\n{body}\n{call}\n    return 0;\n}}\n", {
        "mutated": mutation_done,
        "reference_dynamic_parameter": 1,
    }


def rename_c_target(source: str, old: str) -> str:
    return re.sub(rf"\b{re.escape(old)}\b", "f_gold", source)


def c_boundary_wrapper(parsed: dict[str, Any]) -> str:
    params: list[str] = []
    calls: list[str] = []
    setup: list[str] = []
    for index, arg in enumerate(parsed["args"], start=1):
        ty = vert_type(arg["ctype"])
        if ty == "f32":
            cty = "float"
        else:
            cty = "int"
        if arg["array"]:
            params.append(f"{cty} {arg['name']}[2]")
            extent = arg["extent"] or 2
            clean_type = " ".join(arg["ctype"].replace("const", "").split())
            local = f"vert_local_{index}"
            setup.append(f"    {clean_type} {local}[{extent}] = {{0}};")
            setup.append(f"    {local}[0] = ({clean_type}){arg['name']}[0];")
            if extent > 1:
                setup.append(f"    {local}[1] = ({clean_type}){arg['name']}[1];")
            calls.append(local)
        else:
            params.append(f"{cty} {arg['name']}")
            calls.append(f"({arg['ctype']}){arg['name']}")
    ret = parsed["return"].strip()
    body = "\n".join(setup)
    if ret == "void":
        return (
            f"void f_gold({', '.join(params)}) {{\n{body}\n"
            f"    {parsed['name']}({', '.join(calls)});\n}}\n"
        )
    outer_ret = "float" if vert_type(ret) == "f32" else "int"
    return (
        f"{outer_ret} f_gold({', '.join(params)}) {{\n{body}\n"
        f"    return ({outer_ret}){parsed['name']}({', '.join(calls)});\n}}\n"
    )


def rust_function_param_types(source: str, target: str) -> list[str]:
    matches = list(re.finditer(rf"\bfn\s+{re.escape(target)}\s*\(", source))
    if not matches:
        raise RuntimeError(f"candidate target {target} signature not found")
    start = matches[-1].end()
    depth = 1
    end = start
    while end < len(source) and depth:
        if source[end] == "(":
            depth += 1
        elif source[end] == ")":
            depth -= 1
        end += 1
    if depth:
        raise RuntimeError(f"candidate target {target} has unclosed parameters")
    raw_params = split_args(source[start : end - 1])
    return [item.split(":", 1)[1].strip() for item in raw_params]


def rust_boundary_wrapper(parsed: dict[str, Any], target: str, source: str) -> str:
    params: list[str] = []
    calls: list[str] = []
    setup: list[str] = []
    target_types = rust_function_param_types(source, target)
    if len(target_types) != len(parsed["args"]):
        raise RuntimeError(
            f"candidate target has {len(target_types)} parameters; C has {len(parsed['args'])}"
        )
    for index, (arg, target_ty) in enumerate(zip(parsed["args"], target_types), start=1):
        ty = vert_type(arg["ctype"], array=arg["array"])
        params.append(f"{arg['name']}: {ty}")
        c = " ".join(arg["ctype"].replace("const", "").split())
        rust_ty = {
            "char": "i8",
            "signed char": "i8",
            "unsigned char": "u8",
            "int8_t": "i8",
            "uint8_t": "u8",
            "unsigned": "u32",
            "unsigned int": "u32",
            "size_t": "usize",
            "int": "i32",
            "signed int": "i32",
            "long": "i64",
            "unsigned long": "u64",
            "uLong": "std::os::raw::c_ulong",
            "z_off_t": "std::os::raw::c_long",
            "float": "f32",
            "double": "f64",
        }.get(c)
        if rust_ty is None:
            raise TypeError(f"unsupported Rust boundary cast for {c}")
        if arg["array"]:
            extent = arg["extent"] or 2
            local = f"vert_local_{index}"
            setup.append(
                f"    let mut {local}: [{rust_ty}; {extent}] = [0 as {rust_ty}; {extent}];"
            )
            setup.append(f"    {local}[0] = {arg['name']}[0] as {rust_ty};")
            if extent > 1:
                setup.append(f"    {local}[1] = {arg['name']}[1] as {rust_ty};")
            if "*mut" in target_ty:
                calls.append(f"{local}.as_mut_ptr()")
            elif "*const" in target_ty:
                calls.append(f"{local}.as_ptr()")
            elif "&mut" in target_ty:
                calls.append(f"&mut {local}")
            elif "&" in target_ty:
                calls.append(f"&{local}")
            else:
                calls.append(local)
        else:
            calls.append(f"{arg['name']} as {rust_ty}")
    ret = parsed["return"].strip()
    body = "\n".join(setup)
    call = f"{target}({', '.join(calls)})"
    if ret == "void":
        return (
            f"\nfn f_gold({', '.join(params)}) {{\n{body}\n"
            f"    unsafe {{ {call}; }}\n}}\n"
        )
    outer = vert_type(ret)
    return (
        f"\nfn f_gold({', '.join(params)}) -> {outer} {{\n{body}\n"
        f"    unsafe {{ {call} as {outer} }}\n}}\n"
    )


def prepare_c_sources(payload: dict[str, Any], raw: Path) -> dict[str, Any]:
    parsed = parse_decl(payload["declaration"])
    c_target = parsed["name"]
    if c_target != payload["target"]:
        return {
            "ok": False,
            "classification": "unsupported_input",
            "reason": f"audited C target {c_target} does not match Rust target {payload['target']}",
        }
    if parsed["return"].strip().endswith("*"):
        return {
            "ok": False,
            "classification": "unsupported_input",
            "reason": "VERT's return-value harness has no pointer-return representation",
        }
    if parsed["return"].strip() != "void" and vert_type(parsed["return"]) is None:
        return {
            "ok": False,
            "classification": "unsupported_input",
            "reason": f"VERT has no return representation for {parsed['return']}",
        }
    for arg in parsed["args"]:
        if "*" in arg["ctype"] or vert_type(arg["ctype"]) is None:
            return {
                "ok": False,
                "classification": "unsupported_input",
                "reason": f"VERT has no argument representation for {arg['raw']}",
            }

    rendered = render_c(payload["cobj"])
    boundary_adapter = bool(parsed["args"])
    if boundary_adapter:
        rendered += "\n" + c_boundary_wrapper(parsed)
    else:
        rendered = rename_c_target(rendered, c_target)
    original_main, original_meta = make_main(parsed, False)
    mutated_main, mutated_meta = make_main(parsed, True)
    original = raw / "reference_original.c"
    mutated = raw / "reference_mutated.c"
    original.write_text(rendered + "\n" + original_main)
    mutated.write_text(rendered + "\n" + mutated_main)
    return {
        "ok": True,
        "parsed": parsed,
        "boundary_adapter": boundary_adapter,
        "original": original,
        "mutated": mutated,
        "main": {"original": original_meta, "mutated": mutated_meta},
    }


def compile_reference(cprep: dict[str, Any], raw: Path) -> dict[str, Any]:
    stages: dict[str, Any] = {}
    clang = WASI / "bin/clang"
    sysroot = WASI / "share/wasi-sysroot"
    generated: dict[str, Path] = {}
    for variant in ("original", "mutated"):
        wasm = raw / f"{variant}.wasm"
        stages[f"clang_{variant}"] = run(
            [
                str(clang),
                "-fno-exceptions",
                "-Wno-implicit-int",
                f"--sysroot={sysroot}",
                "-o",
                str(wasm),
                str(cprep[variant]),
            ],
            cwd=raw,
            log=raw / f"clang_{variant}.log",
        )
        if stages[f"clang_{variant}"]["exit_code"] != 0:
            return {"ok": False, "classification": "compile_failure", "stages": stages}
        out = raw / f"out-rwasm-{variant}"
        stages[f"rwasm_{variant}"] = run(
            [
                str(RWASM),
                str(wasm),
                str(out),
                "--prevent-reformat",
                "--wasi-executable",
            ],
            cwd=raw,
            log=raw / f"rwasm_{variant}.log",
            timeout=1200,
        )
        if stages[f"rwasm_{variant}"]["exit_code"] != 0:
            return {"ok": False, "classification": "analysis_failure", "stages": stages}
        generated[variant] = out
    return {"ok": True, "stages": stages, "generated": generated}


def rust_static_type(parsed_arg: dict[str, Any]) -> str:
    value = vert_type(parsed_arg["ctype"], array=parsed_arg["array"])
    if value is None:
        raise TypeError(parsed_arg["ctype"])
    return value


def rust_result_type(parsed: dict[str, Any]) -> str | None:
    if parsed["return"].strip() == "void":
        return None
    return vert_type(parsed["return"])


def inject_reference(
    original_rs: str, mutated_rs: str, parsed: dict[str, Any]
) -> tuple[str, dict[str, Any]]:
    """Apply VERT's released first-parameter/result injection to rWasm text."""
    diff = list(
        difflib.unified_diff(
            original_rs.splitlines(), mutated_rs.splitlines(), lineterm=""
        )
    )
    removed = [line[1:] for line in diff if line.startswith("-") and not line.startswith("---")]
    added = [line[1:] for line in diff if line.startswith("+") and not line.startswith("+++")]
    candidates: list[tuple[str, str]] = []
    for before, after in zip(removed, added):
        if "TaggedVal::from(" in before and "TaggedVal::from(" in after:
            candidates.append((before, after))
    fallback = False
    if candidates:
        before, after = candidates[0]
        injected_line = "v0 = TaggedVal::from(unsafe {PARAM1});"
        replacement = injected_line
    else:
        # Exact fallback in VerificationUtils.mutate_test: when the first diff
        # occurs in m.memory[...] (the normal case for array arguments), the
        # hunk line lies before every generated func_N.  VERT then selects a
        # middle group of functions and injects PARAM1 before the last nested
        # call in that window.  This is intentionally not corrected here.
        hunk = next((line for line in diff if line.startswith("@@")), None)
        if hunk is None:
            raise RuntimeError("VERT found no reference mutation to inject")
        match = re.search(r"-(\d+)", hunk)
        if match is None:
            raise RuntimeError("VERT could not parse its rWasm diff hunk")
        diff_num = int(match.group(1))
        original_lines = original_rs.splitlines(keepends=True)
        func_lines = [
            i - 1
            for i, line in enumerate(original_lines)
            if line.strip().startswith("fn func_")
        ]
        if len(func_lines) < 3:
            raise RuntimeError("VERT fallback has too few rWasm functions")
        window: tuple[int, int] | None = None
        for i, line_no in enumerate(func_lines):
            if i + 1 < len(func_lines) and diff_num > line_no and diff_num < func_lines[i + 1]:
                window = (line_no, func_lines[i + 1])
                break
            if i == 0 and diff_num < line_no:
                middle = int(len(func_lines) / 2)
                if middle < 1 or middle + 2 >= len(func_lines):
                    raise RuntimeError("VERT fallback middle window is out of range")
                window = (func_lines[middle - 1] + 1, func_lines[middle + 2])
                break
        if window is None:
            raise RuntimeError("VERT fallback did not select an rWasm window")
        return_lines = [
            line
            for line in original_lines[window[0] : window[1]]
            if "TaggedVal::from(self.func" in line
        ]
        if not return_lines:
            raise RuntimeError("VERT fallback window has no nested call")
        before = return_lines[-1].rstrip("\n")
        after = before
        first = parsed["args"][0] if parsed["args"] else None
        unsafe_param = "{PARAM1}[0]" if first and first["array"] else "{PARAM1}"
        injected_line = f"v0 = TaggedVal::from(unsafe {unsafe_param});"
        replacement = injected_line + "\n" + before
        fallback = True
    text = mutated_rs.replace(after, replacement, 1)

    position = text.find(injected_line)
    call = text.find("TaggedVal::from(self.func", position)
    if call < 0:
        raise RuntimeError("VERT could not locate the rWasm target call")
    line_end = text.find("\n", call)
    result_type = rust_result_type(parsed)
    if result_type is None:
        # VERT has no meaningful RESULT declaration for void.  Let the build
        # gate record this as a harness compile failure instead of inventing a
        # state oracle.
        capture = "\nunsafe { RESULT = (); }\n"
    else:
        capture = (
            f"\nlet retval = v0.try_as_{result_type}()?;\n"
            "unsafe { RESULT = retval; }\n"
        )
    text = text[: line_end + 1] + capture + text[line_end + 1 :]
    return text, {
        "diff_lines": len(diff),
        "changed_line_before": before,
        "changed_line_after": after,
        "injected_parameter": "PARAM1",
        "released_middle_function_fallback": fallback,
    }


def inline_candidate(payload: dict[str, Any]) -> str:
    source = payload["rust_file"].read_text()
    include = re.search(r'include!\[?\s*\(?\s*"([^"]+)"', source)
    # Normal syntax is include!["..."]; support it explicitly rather than
    # trusting an absolute /input path from a containerized prior baseline.
    include = re.search(r'include!\s*\[?\s*\(?\s*"([^"]+)"\s*\)?\s*\]?\s*;', source)
    if include:
        raw_inc = next((payload["rust_owner"] / "input").glob("*_raw.inc"), None)
        if raw_inc is None:
            raise RuntimeError("candidate include has no audited raw source")
        source = source[: include.start()] + raw_inc.read_text() + source[include.end() :]
    target = payload["target"]
    if not re.search(rf"\b{re.escape(target)}\b", source):
        raise RuntimeError(f"candidate target {target} not found")
    return source


def make_harness(parsed: dict[str, Any]) -> str:
    arg_types = [rust_static_type(arg) for arg in parsed["args"]]
    params = ", ".join(arg_types)
    pattern = ",".join(f"PARAM_{i}" for i in range(1, len(arg_types) + 1))
    if len(arg_types) == 1:
        pattern = f"({pattern})"
    else:
        pattern = f"({pattern})"
    assignments = "\n".join(
        f"        PARAM{i} = PARAM_{i};" for i in range(1, len(arg_types) + 1)
    )
    calls: list[str] = []
    for i, arg in enumerate(parsed["args"], start=1):
        if arg["array"]:
            calls.append(f"[unsafe {{ PARAM{i} }}[0], unsafe {{ PARAM{i} }}[1]]")
        else:
            calls.append(f"unsafe {{ PARAM{i} }}.into()")
    result = rust_result_type(parsed)
    if result is None:
        candidate = f"f_gold({', '.join(calls)});"
        compare = "let result = ();\n        let result_prime = f_gold_wasm_thread_unsafe();"
    else:
        candidate = f"let result = f_gold({', '.join(calls)});"
        compare = f"{candidate}\n        let result_prime = f_gold_wasm_thread_unsafe();"
    return f"""
use bolero::check;
#[test]
fn bolero_wasm_eq() {{
    bolero::check!().with_type::<({params})>().cloned().for_each(|{pattern}| {{
        unsafe {{
{assignments}
        }}
        {compare}
        assert_eq!(result, result_prime);
    }});
}}
"""


def assemble_project(
    payload: dict[str, Any], cprep: dict[str, Any], compiled: dict[str, Any], raw: Path
) -> dict[str, Any]:
    original_dir = compiled["generated"]["original"]
    mutated_dir = compiled["generated"]["mutated"]
    original_rs = (original_dir / "src/main.rs").read_text()
    mutated_rs = (mutated_dir / "src/main.rs").read_text()
    injected, injection = inject_reference(original_rs, mutated_rs, cprep["parsed"])

    project = raw / "out-rwasm-bolero"
    if project.exists():
        shutil.rmtree(project)
    shutil.copytree(mutated_dir, project)
    cargo = project / "Cargo.toml"
    with cargo.open("a") as f:
        f.write('\n[dev-dependencies]\nbolero = "0.10.0"\n')

    declarations: list[str] = []
    for i, arg in enumerate(cprep["parsed"]["args"], start=1):
        ty = rust_static_type(arg)
        init = "[12.0, 12.0]" if ty == "[f32; 2]" else (
            "[12, 12]" if ty.startswith("[") else ("12.0" if ty == "f32" else "12")
        )
        declarations.append(f"static mut PARAM{i}: {ty} = {init};")
    result = rust_result_type(cprep["parsed"])
    if result is None:
        declarations.append("static mut RESULT: () = (); ")
        wasm_return = "()"
    else:
        init = "12.0" if result == "f32" else "12"
        declarations.append(f"static mut RESULT: {result} = {init};")
        wasm_return = result

    candidate = inline_candidate(payload)
    if cprep.get("boundary_adapter"):
        candidate += rust_boundary_wrapper(
            cprep["parsed"], payload["target"], candidate
        )
    else:
        candidate = re.sub(
            rf"\b{re.escape(payload['target'])}\b", "f_gold", candidate
        )
    wasm_fn = f"""
fn f_gold_wasm_thread_unsafe() -> {wasm_return} {{
    let mut wasm_module = WasmModule::new();
    wasm_module._start().unwrap();
    unsafe {{ RESULT }}
}}
"""
    final = (
        "\n".join(declarations)
        + "\n"
        + injected
        + "\n////// External Frozen Candidate //////\n"
        + candidate
        + wasm_fn
        + make_harness(cprep["parsed"])
    )
    main_rs = project / "src/main.rs"
    main_rs.write_text(final)
    return {
        "project": project,
        "injection": injection,
        "main_rs": main_rs,
        "main_sha256": sha256(main_rs),
    }


def classify_test(stage: dict[str, Any], log: Path) -> tuple[str, str]:
    text = log.read_text(errors="replace") if log.exists() else ""
    if stage["timeout"]:
        return "analysis_failure", "Bolero macro test timed out"
    if stage["exit_code"] == 0:
        return "completed_miss", "comparison completed without a counterexample"
    if "error: could not compile" in text or "error[" in text:
        return "compile_failure", "generated VERT project did not compile"
    if "assertion `left == right` failed" in text or "Test failed with input" in text:
        return "candidate_counterexample", "VERT reported a differential counterexample"
    return "analysis_failure", "checker exited unsuccessfully without a differential counterexample"


def execute(defect: str, *, force: bool = False) -> dict[str, Any]:
    result_path = RESULTS / defect / "result.json"
    if result_path.exists() and not force:
        return json.loads(result_path.read_text())
    raw = RAW_ROOT / defect / "attempt-001"
    if raw.exists():
        shutil.rmtree(raw)
    raw.mkdir(parents=True)

    record: dict[str, Any] = {
        "schema_version": 1,
        "baseline": "vert",
        "defect_id": defect,
        "status": "running",
        "adapter": "frozen_external_pair_v1",
        "raw_workdir": str(raw),
        "policy": {
            "baseline_repairs": False,
            "candidate_source": "audited frozen per-function payload",
            "reference_input_injection": "released PARAM1-only semantics",
            "observation": "return value only",
            "array_extent": 2,
            "max_cargo_jobs": 4,
        },
    }
    atomic_json(result_path, record)

    payload = load_payload(defect)
    if not payload["ok"]:
        record.update(status="complete", outcome=payload["classification"], reason=payload["reason"])
        atomic_json(result_path, record)
        return record
    record["inputs"] = {
        "c": str(payload["cjson"].relative_to(REPO)),
        "rust": str(payload["rust_file"].relative_to(REPO)),
        "target": payload["target"],
        "c_sha256": sha256(payload["cjson"]),
        "rust_sha256": sha256(payload["rust_file"]),
    }

    try:
        cprep = prepare_c_sources(payload, raw)
        if not cprep["ok"]:
            record.update(status="complete", outcome=cprep["classification"], reason=cprep["reason"])
            atomic_json(result_path, record)
            return record
        record["signature"] = cprep["parsed"]
        compiled = compile_reference(cprep, raw)
        record["reference_stages"] = compiled["stages"]
        if not compiled["ok"]:
            record.update(status="complete", outcome=compiled["classification"], reason="reference construction failed")
            atomic_json(result_path, record)
            return record
        assembled = assemble_project(payload, cprep, compiled, raw)
        record["injection"] = assembled["injection"]
        record["generated_main_sha256"] = assembled["main_sha256"]
        test_log = raw / "cargo_test.log"
        test_stage = run(
            ["cargo", "test", "--release", "bolero_wasm_eq", "--", "--nocapture"],
            cwd=assembled["project"],
            log=test_log,
            timeout=600,
            env={"CARGO_TARGET_DIR": str(SHARED_CARGO_TARGET)},
        )
        record["test_stage"] = test_stage
        outcome, reason = classify_test(test_stage, test_log)
        if outcome == "candidate_counterexample":
            args_shape = cprep["parsed"]["args"]
            if len(args_shape) == 1 and not args_shape[0]["array"]:
                outcome = "detected_candidate"
                reason = (
                    "VERT reported a counterexample under a valid one-scalar "
                    "input mapping; source-level defect validation is required"
                )
            else:
                outcome = "analysis_failure"
                reason = (
                    "VERT reported a counterexample, but its released reference "
                    "wrapper injects only PARAM1 while the candidate receives all "
                    "dynamic values; the relational comparison is invalid"
                )
        # A counterexample is a candidate result, not yet a detection.  The
        # correct-candidate control is intentionally a separate gate and will
        # be generated only after the candidate path proves runnable.
        record.update(status="complete", outcome=outcome, reason=reason)
    except Exception as exc:  # preserve every attempted gate as evidence
        record.update(
            status="complete",
            outcome="analysis_failure",
            reason=f"external VERT adapter failed: {type(exc).__name__}: {exc}",
        )
    atomic_json(result_path, record)
    return record


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("defects", nargs="*", help="defect IDs; default is all except S14")
    parser.add_argument("--force", action="store_true")
    args = parser.parse_args()
    defects = args.defects or [item for item in DEFECTS if item != "S14"]
    unknown = sorted(set(defects) - set(DEFECTS))
    if unknown:
        parser.error(f"unknown defects: {', '.join(unknown)}")
    RAW_ROOT.mkdir(parents=True, exist_ok=True)
    counts: dict[str, int] = {}
    for defect in defects:
        print(f"[{defect}] submitting", flush=True)
        record = execute(defect, force=args.force)
        outcome = record.get("outcome", record["status"])
        counts[outcome] = counts.get(outcome, 0) + 1
        print(f"[{defect}] {outcome}: {record.get('reason', '')}", flush=True)
    print(json.dumps(counts, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())

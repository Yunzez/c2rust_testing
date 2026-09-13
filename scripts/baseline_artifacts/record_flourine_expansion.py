#!/usr/bin/env python3
"""Record audited FLOURINE expansion outcomes in the shared funnel."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
EXTERNAL = Path("/home/yunzez/c2rust_baselines/runs/flourine")
RESULTS = ROOT / "results/baseline_artifacts/results.json"
RUN_ROOT = ROOT / "results/baseline_artifacts/runs/flourine"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def evidence(attempt: Path) -> dict:
    names = [
        "input.sha256",
        "instrument-c.log",
        "cmake-build.log",
        "instrument-rust.log",
        "target.txt",
        "verify.log",
        "console.log",
        "exit_code.txt",
    ]
    result: dict = {
        name: sha256(attempt / name) for name in names if (attempt / name).is_file()
    }
    crashes = {
        str(path.relative_to(attempt)): sha256(path)
        for path in sorted(attempt.glob("verification/src/__fuzz__/*/crashes/*"))
        if path.is_file()
    }
    if crashes:
        result["crashes"] = crashes
    return result


def text(path: Path) -> str:
    return path.read_text(errors="replace")


def ascii_unicode_witness(attempt: Path) -> dict:
    log = text(attempt / "verify.log")
    match = re.search(r"counter examples: (\[.*\])\npositive examples:", log)
    if not match:
        raise RuntimeError("missing FLOURINE counterexample list for S7")
    for row in json.loads(match.group(1)):
        original = json.loads(row["args"][0])
        mapped = bytearray(32)
        mapped[0] = ord('"')
        for index in range(1, 30):
            mapped[index] = 32 + ((original[index] & 0xFF) % 95)
        mapped[30] = ord('"')
        if re.search(rb"\\u[0-9A-Fa-f]{4}", mapped):
            if row["actual"] != {"ExecutionSuccess": "0"} or row["expected"] != {
                "ExecutionSuccess": "1"
            }:
                raise RuntimeError("S7 witness does not have the expected return mismatch")
            return {
                "generated_argument": original,
                "mapped_json_bytes": list(mapped[:31]),
                "mapped_json_ascii": mapped[:31].decode("ascii"),
                "actual_rust": 0,
                "expected_c": 1,
            }
    raise RuntimeError("FLOURINE reported no valid \\uXXXX witness for S7")


def non_utf8_filename_witness(attempt: Path) -> dict:
    log = text(attempt / "verify.log")
    match = re.search(r"counter examples: (\[.*\])\npositive examples:", log)
    if not match:
        raise RuntimeError("missing FLOURINE counterexample list for C4")
    for row in json.loads(match.group(1)):
        generated = json.loads(row["args"][0])
        filename = bytearray(value & 0xFF for value in generated)
        filename[15] = 0
        prefix = bytes(filename).split(b"\0", 1)[0]
        try:
            prefix.decode("utf-8")
        except UnicodeDecodeError:
            if row["actual"] != "ExecutionFailure" or row["expected"] != {
                "ExecutionSuccess": "0"
            }:
                raise RuntimeError("C4 witness does not have Rust failure and C success")
            return {
                "generated_argument": generated,
                "nul_terminated_filename_bytes": list(filename),
                "actual_rust": "termination",
                "expected_c": 0,
            }
    raise RuntimeError("FLOURINE reported no non-UTF-8 filename witness for C4")


def base(
    defect: str,
    target: str,
    gates: dict[str, bool],
    outcome: str,
    attempt_name: str = "attempt-001",
    external_slug: str | None = None,
) -> dict:
    attempt = EXTERNAL / f"pilot_{external_slug or defect}/{attempt_name}"
    return {
        "schema_version": 1,
        "baseline": "flourine",
        "defect_id": defect,
        "target": target,
        "gates": gates,
        "outcome": outcome,
        "artifact_native_search": True,
        "witness_replay": False,
        "attempt": attempt_name,
        "external_directory": str(attempt),
        "hashes": evidence(attempt),
    }


def detected(
    defect: str,
    target: str,
    pattern: str,
    interpretation: str,
    attempt_name: str = "attempt-001",
    external_slug: str | None = None,
) -> dict:
    attempt = EXTERNAL / f"pilot_{external_slug or defect}/{attempt_name}"
    log = text(attempt / "verify.log")
    if not re.search(pattern, log, re.MULTILINE):
        raise RuntimeError(f"missing detection evidence for {defect}: {pattern}")
    payload = base(
        defect,
        target,
        {"submitted": True, "accepted": True, "compiled": True, "completed": True, "detected": True},
        "detected",
        attempt_name,
        external_slug,
    )
    payload["interpretation"] = interpretation
    return payload


def record_all() -> dict[str, dict]:
    payloads: dict[str, dict] = {}
    payloads["S1"] = detected(
        "S1",
        "crc32_z_packet",
        r"counter examples:",
        "FLOURINE generated a fixed packet encoding CRC state, bytes, and a valid length, then reported scalar return discrepancies in the translated crc32_z computation.",
    )
    payloads["S2"] = detected(
        "S2",
        "adler32_z_packet",
        r"counter examples:",
        "FLOURINE generated a fixed packet encoding Adler state, bytes, and a valid length, then reported scalar return discrepancies in the translated adler32_z computation.",
    )
    payloads["S4"] = detected(
        "S4",
        "crc32_z_packet",
        r"counter examples:",
        "Using the same generated CRC-state/bytes/length packet shape as S1, FLOURINE reported scalar return discrepancies caused by Laertes' uninitialized CRC table.",
        "attempt-002",
    )
    payloads["S4"]["prior_attempts"] = [
        {
            "attempt": "attempt-001",
            "outcome": "adapter_setup_failure",
            "reason": "the adapter generator initially selected a forward declaration instead of the C function definition",
            "hashes": evidence(EXTERNAL / "pilot_S4/attempt-001"),
        }
    ]
    payloads["C2"] = detected(
        "C2",
        "url_is_ssh_valid",
        r'"actual":"ExecutionFailure","expected":\{"ExecutionSuccess":"false"\}',
        "FLOURINE generated byte strings and reported Rust termination where its separately protected C call returned false.",
    )
    c4_attempt = EXTERNAL / "pilot_C4/attempt-001"
    payloads["C4"] = detected(
        "C4",
        "ends_in_bz2_valid",
        r'"actual":"ExecutionFailure","expected":\{"ExecutionSuccess":"0"\}',
        "FLOURINE generated a NUL-terminated filename containing non-UTF-8 bytes; the original C target returned normally while the translated Rust target terminated during UTF-8 validation.",
    )
    payloads["C4"].update(
        {
            "adapter_record": "results/baseline_artifacts/adapters/flourine/C4/adapter.json",
            "command_record": "scripts/baseline_artifacts/run_flourine_pair.sh C4",
            "witness": non_utf8_filename_witness(c4_attempt),
            "witness_search_check": (
                "The adapter fixes only the final NUL terminator; every preceding filename "
                "byte is independently generated, and no known witness or suffix is encoded."
            ),
        }
    )
    s5_attempt = EXTERNAL / "pilot_S5/attempt-003"
    s5_lib = text(s5_attempt / "verification/src/lib.rs")
    s5_raw = text(
        ROOT
        / "results/baseline_artifacts/adapters/flourine/S5/input/genann_cached_initialized_raw.inc"
    )
    c_call = s5_lib.find("genann_cached_initialized__C(extern_input0)")
    rust_call = s5_lib.find("genann_cached_initialized__Rust(", c_call)
    if c_call < 0 or rust_call < 0 or c_call >= rust_call:
        raise RuntimeError("S5 harness no longer calls C before Rust")
    if not re.search(r"static lookup: \[f64; 4096\]", s5_raw) or not re.search(
        r"&lookup as \*const \[f64; 4096\] as \*mut \[f64; 4096\]", s5_raw
    ):
        raise RuntimeError("S5 immutable-lookup evidence changed")
    payloads["S5"] = detected(
        "S5",
        "genann_cached_initialized",
        r"fatal runtime error: Rust cannot catch foreign exceptions, aborting",
        (
            "FLOURINE generated an f64 input, completed the C call, and then terminated "
            "when the translated initializer wrote through a cast into its immutable "
            "lookup static. This is the confirmed S5 initialization-corruption root cause."
        ),
        "attempt-003",
    )
    payloads["S5"].update(
        {
            "adapter_record": "results/baseline_artifacts/adapters/flourine/S5/adapter.json",
            "command_record": "scripts/baseline_artifacts/run_flourine_pair.sh S5",
            "witness": {
                "base64": "//////////8=",
                "generated_f64_bits": "0xffffffffffffffff",
                "effective_argument": 0.0,
                "actual_rust": "termination during genann_init_sigmoid_lookup",
                "expected_c": "normal return",
            },
            "witness_search_check": (
                "The adapter maps only NaN to 0.0 on both sides to satisfy the target's "
                "explicit non-NaN precondition; it does not encode the immutable-static defect."
            ),
            "prior_attempts": [
                {
                    "attempt": "attempt-001",
                    "outcome": "adapter_setup_failure",
                    "reason": "the adapter generator removed the dependency header before inlining it",
                    "hashes": evidence(EXTERNAL / "pilot_S5/attempt-001"),
                },
                {
                    "attempt": "attempt-002",
                    "outcome": "adapter_out_of_contract",
                    "reason": "the initial adapter admitted NaN despite the target's explicit assert(!isnan(a)) precondition",
                    "hashes": evidence(EXTERNAL / "pilot_S5/attempt-002"),
                },
            ],
        }
    )
    payloads["S15"] = detected(
        "S15",
        "ti_adx_start_valid",
        r"thread caused non-unwinding panic",
        "The generated f64 option array reached the translated pointer-as-period computation and terminated in Rust; the C wrapper completed normally.",
    )
    payloads["S16"] = detected(
        "S16",
        "opng_strcasecmp_valid",
        r"counter examples:",
        "FLOURINE generated two fixed-capacity byte strings and reported differing scalar return values.",
    )
    payloads["S9"] = detected(
        "S9",
        "parse_string_valid",
        r'"actual":\{"ExecutionSuccess":"0"\},"expected":\{"ExecutionSuccess":"1"\}',
        "FLOURINE generated a quoted byte string containing non-UTF-8 bytes and reported the translated parser returning failure where the original C parser returned success.",
        "attempt-005",
        "cjson_parse_string",
    )
    payloads["S9"].update(
        {
            "adapter_record": "results/baseline_artifacts/adapters/flourine/cjson_parse_string/adapter.json",
            "command_record": "scripts/baseline_artifacts/run_flourine_pair.sh S7",
            "witness": {
                "base64": "Ch///x9KHA8sLP//LSwsLCwsLCwsLCwsLCwsLCzjSgLA",
                "interpretation": "After the symmetric wrapper inserts quotes and a terminator, the payload contains 0xff bytes and is invalid UTF-8. C returns 1 and Rust returns 0, matching S9.",
            },
            "prior_attempts": [
                {
                    "attempt": "attempt-001",
                    "outcome": "adapter_setup_failure",
                    "reason": "system includes were embedded after FLOURINE's allocator macros",
                    "hashes": evidence(EXTERNAL / "pilot_cjson_parse_string/attempt-001"),
                },
                {
                    "attempt": "attempt-002",
                    "outcome": "adapter_setup_failure",
                    "reason": "the submitted wrapper was hidden inside include!, so the documented surface parser could not see its entry",
                    "hashes": evidence(EXTERNAL / "pilot_cjson_parse_string/attempt-002"),
                },
                {
                    "attempt": "attempt-003",
                    "outcome": "adapter_setup_failure",
                    "reason": "the complete translated dependency unit was exposed as the surface AST rather than compiled through include!",
                    "hashes": evidence(EXTERNAL / "pilot_cjson_parse_string/attempt-003"),
                },
                {
                    "attempt": "attempt-004",
                    "outcome": "adapter_induced_false_positive",
                    "reason": "the C wrapper mutated the submitted byte array while the Rust wrapper initially mutated only a copy; this asymmetry was corrected and is not scored",
                    "hashes": evidence(EXTERNAL / "pilot_cjson_parse_string/attempt-004"),
                },
            ],
        }
    )
    s7_attempt = EXTERNAL / "pilot_cjson_parse_string_ascii/attempt-001"
    payloads["S7"] = detected(
        "S7",
        "parse_string_ascii",
        r'"actual":\{"ExecutionSuccess":"0"\},"expected":\{"ExecutionSuccess":"1"\}',
        "Within a broad printable-ASCII string domain, FLOURINE independently generated valid Unicode escape sequences and reported the translated parser returning failure where the original C parser returned success.",
        "attempt-001",
        "cjson_parse_string_ascii",
    )
    payloads["S7"].update(
        {
            "adapter_record": "results/baseline_artifacts/adapters/flourine/cjson_parse_string_ascii/adapter.json",
            "command_record": "scripts/baseline_artifacts/run_flourine_pair.sh S7",
            "witness": ascii_unicode_witness(s7_attempt),
            "witness_search_check": (
                "The adapter maps every interior byte independently to one of 95 printable "
                "ASCII characters and does not force a backslash, u, or hexadecimal digit."
            ),
        }
    )

    # The first valid submission exposed a released primitive-ABI emitter
    # failure.  Later experiments deliberately changed the interface to see
    # whether the downstream oracle could expose the defect.  They are useful
    # diagnostics, but are not scored because they bypass a baseline failure.
    s14_bad_scalar = EXTERNAL / "pilot_S14/attempt-001"
    s14_bad_return = EXTERNAL / "pilot_S14/attempt-003"
    s14_good = EXTERNAL / "pilot_S14/attempt-004"
    if 'fn mmed3_valid__C(_: u8, _: u8, _: u8) -> u8;' not in text(
        s14_bad_scalar / "verification/src/lib.rs"
    ):
        raise RuntimeError("S14 primitive-ABI evidence changed")
    if 'extern "C" char* mmed3_packet__C(char* extern_input0)' not in text(
        s14_bad_return / "verification/ground_truth/instrumented.cpp"
    ):
        raise RuntimeError("S14 char-return evidence changed")
    if 'extern "C" unsigned int mmed3_packet__C(char* extern_input0)' not in text(
        s14_good / "verification/ground_truth/instrumented.cpp"
    ) or 'fn mmed3_packet__C(_: *mut i8) -> u32;' not in text(
        s14_good / "verification/src/lib.rs"
    ):
        raise RuntimeError("S14 retry ABI is not symmetric")
    payloads["S14"] = base(
        "S14",
        "mmed3_valid",
        {"submitted": True, "accepted": True, "compiled": True, "completed": False, "detected": False},
        "analysis_failure",
    )
    payloads["S14"].update(
        {
            "failure_stage": "execution",
            "interpretation": "The released artifact emitted incompatible ABIs for the original primitive signature: Rust calls u8,u8,u8->u8 while the generated C symbol is a JSON char*,char*,char*->char* wrapper. No workaround is scored.",
            "exploratory_unscored": [
                {
                    "attempt": "attempt-002",
                    "outcome": "packaging_failure",
                    "reason": "both historical and retry inputs were present in the single-input glob",
                    "hashes": evidence(EXTERNAL / "pilot_S14/attempt-002"),
                },
                {
                    "attempt": "attempt-003",
                    "outcome": "analysis_failure",
                    "reason": "released char-return ABI mismatch",
                    "hashes": evidence(s14_bad_return),
                },
                {
                    "attempt": "attempt-004",
                    "outcome": "detected_after_interface_workaround",
                    "reason": "fixed-array argument bundling plus lossless u8-to-u32 return widening bypassed both released emitter failures",
                    "hashes": evidence(s14_good),
                },
            ],
        }
    )

    c13_attempt = EXTERNAL / "pilot_C13/attempt-001"
    compile_log = text(c13_attempt / "cmake-build.log")
    if "invalid use of type ‘void’ in parameter declaration" not in compile_log:
        raise RuntimeError("C13 emitter-failure evidence changed")
    payloads["C13"] = base(
        "C13",
        "opng_free_null",
        {"submitted": True, "accepted": True, "compiled": False, "completed": False, "detected": False},
        "compile_failure",
    )
    payloads["C13"].update(
        {
            "failure_stage": "compile",
            "interpretation": "For the original zero-argument target, the released C emitter generated the invalid declaration `void extern_input0`. No workaround is scored.",
            "exploratory_unscored": [
                {
                    "attempt": "attempt-002",
                    "outcome": "packaging_failure",
                    "reason": "both historical and retry inputs were present in the single-input glob",
                    "hashes": evidence(EXTERNAL / "pilot_C13/attempt-002"),
                },
                {
                    "attempt": "attempt-003",
                    "outcome": "detected_after_interface_workaround",
                    "reason": "an ignored byte argument bypassed the released zero-argument emitter failure",
                    "hashes": evidence(EXTERNAL / "pilot_C13/attempt-003"),
                },
            ],
        }
    )

    c16_attempt = EXTERNAL / "pilot_C16/attempt-001"
    c16_log = text(c16_attempt / "verify.log")
    done = re.search(r"Done (\d+) runs in (\d+) second", c16_log)
    if not done or "counter examples:" in c16_log or "Test unit written" in c16_log:
        raise RuntimeError("C16 is not a clean completed miss")
    payloads["C16"] = base(
        "C16",
        "optimize_cmf_valid",
        {"submitted": True, "accepted": True, "compiled": True, "completed": True, "detected": False},
        "missed",
    )
    payloads["C16"].update(
        {
            "executions": int(done.group(1)),
            "seconds": int(done.group(2)),
            "interpretation": "The artifact completed its native search without reporting the profile-dependent unsigned-underflow panic.",
        }
    )

    s12_attempt = EXTERNAL / "pilot_S12/attempt-001"
    if text(s12_attempt / "instrument-rust.log").strip() != "Error: expected square brackets":
        raise RuntimeError("S12 released-parser failure evidence changed")
    payloads["S12"] = base(
        "S12",
        "bzbuff_compress_packet",
        {
            "submitted": True,
            "accepted": False,
            "compiled": False,
            "completed": False,
            "detected": False,
        },
        "analysis_failure",
    )
    payloads["S12"].update(
        {
            "failure_stage": "rust_instrumentation",
            "adapter_record": "results/baseline_artifacts/adapters/flourine/S12/adapter.json",
            "command_record": "scripts/baseline_artifacts/run_flourine_pair.sh S12",
            "interpretation": (
                "The released C instrumenter accepted and compiled the exact bzip2 dependency "
                "closure, but the released Rust instrumenter rejected the original "
                "dependency-bearing Rust input with `Error: expected square brackets`. Per the "
                "frozen policy, no parser workaround or source reduction is attempted."
            ),
        }
    )
    return payloads


def write(payloads: dict[str, dict]) -> None:
    for defect, payload in payloads.items():
        out = RUN_ROOT / defect
        out.mkdir(parents=True, exist_ok=True)
        (out / "result.json").write_text(json.dumps(payload, indent=2) + "\n")

    data = json.loads(RESULTS.read_text())
    for row in data["records"]:
        defect = row["defect_id"]
        if row["baseline"] != "flourine" or defect not in payloads:
            continue
        payload = payloads[defect]
        row.update(
            {
                "gates": payload["gates"],
                "outcome": payload["outcome"],
                "adapter": payload.get(
                    "adapter_record",
                    f"results/baseline_artifacts/adapters/flourine/{defect}/adapter.json",
                ),
                "command_record": payload.get(
                    "command_record",
                    f"scripts/baseline_artifacts/run_flourine_pair.sh {defect}",
                ),
                "result_record": f"results/baseline_artifacts/runs/flourine/{defect}/result.json",
                "notes": [payload["interpretation"]],
            }
        )
    RESULTS.write_text(json.dumps(data, indent=2) + "\n")


def main() -> None:
    payloads = record_all()
    write(payloads)
    print("recorded FLOURINE expansion: " + ", ".join(payloads))


if __name__ == "__main__":
    main()

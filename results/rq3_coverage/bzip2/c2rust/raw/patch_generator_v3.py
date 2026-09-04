#!/usr/bin/env python3
"""RQ4 fix round 3 — make the C reference and the UB gate switchable AT RUN TIME.

Until now an ablation of the gate meant generating a second set of harnesses, which means a second
binary, a second coverage map, and re-doing the identity alignment between them.  Three separate
defects in this experiment came from exactly that kind of cross-binary comparison (symbol names
carrying a different crate-disambiguator hash, `lib.rs` matched by basename also matching
libfuzzer-sys's own instrumented lib.rs, and --expose-entry inserting a line that shifted every
later line of one harness's lib.rs).

With one binary and a runtime switch the coverage map is literally the same object in all modes, so
the only difference between two measurements is which regions executed.  `C2R_MODE` selects:

  gated      (default)  C runs, the UB gate rejects inputs on which C trips UBSan, Rust runs on the
                        rest, outputs are compared.  This is the measurement mode.
  nogate                C runs, Rust runs, outputs are compared, but a UB-tripping input is NOT
                        rejected.  Quantifies what the gate costs in coverage and what it prevents
                        in false divergences.
  rust-only             C is not called at all; no comparison is possible.  Upper bound on what the
                        C side costs in throughput.  NOT a validator measurement and must never be
                        reported as coverage of the validator.

Caveat that has to travel with this: the C oracle is sancov-instrumented and its edges feed
libFuzzer, so running a CAMPAIGN in a different mode explores differently.  The modes are strictly
comparable only when replaying the SAME corpus.

The input decoding is untouched, so every corpus recorded before this change stays valid.
"""
import pathlib

SCR = pathlib.Path("/tmp/claude-1000/-home-yunzez-c2rust-testing/"
                   "6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov/rq4_gen")

MODE_HELPER = '''        "",
        "// RQ4: C reference and UB gate are selected at RUN TIME so every mode shares one binary,",
        "// one coverage map and one set of identities. C2R_MODE=gated|nogate|rust-only.",
        "const C2R_GATED: u8 = 0; const C2R_NOGATE: u8 = 1; const C2R_RUST_ONLY: u8 = 2;",
        "fn c2r_mode() -> u8 {",
        "    static M: std::sync::OnceLock<u8> = std::sync::OnceLock::new();",
        "    *M.get_or_init(|| match std::env::var(\\"C2R_MODE\\").as_deref() {",
        "        Ok(\\"nogate\\") => C2R_NOGATE,",
        "        Ok(\\"rust-only\\") => C2R_RUST_ONLY,",
        "        _ => C2R_GATED,",
        "    })",
        "}",
'''


def main():
    p = SCR / "gen_diff_harness.py"
    t = p.read_text()

    t = t.replace('GEN_VERSION = "0.5+rq4fix2026-09-03b"  # + output_buffer/capacity_ptr, max_len, per-entry schema',
                  'GEN_VERSION = "0.5+rq4fix2026-09-03c"  # + runtime-switchable C reference / UB gate')

    # ---- build the three-mode body instead of one fixed call sequence ----
    old = '''    if ret == "void":
        body_call = f"{pre}        {call_c};\\n{gate}        {call_r};"
        ret_cmp = ""'''
    new = '''    if ret == "void":
        body_call = f"{pre}        {call_c};\\n{gate}"
        rust_call = f"        {call_r};"
        ret_cmp = ""'''
    assert old in t
    t = t.replace(old, new)

    old = '''        body_call = f"{pre}        let c_ret = {call_c};\\n{gate}        let r_ret = {call_r};"
        ret_cmp = (
            f"        let (c_ok, c_val, c_cons) = (c_ret != 0, {osc}_c, c_ret);\\n"'''
    new = '''        body_call = f"{pre}        let c_ret = {call_c};\\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        ret_cmp = (
            f"        let (c_ok, c_val, c_cons) = (c_ret != 0, {osc}_c, c_ret);\\n"'''
    assert old in t
    t = t.replace(old, new)

    old = '''        body_call = f"{pre}        let c_ret = {call_c};\\n{gate}        let r_ret = {call_r};"
        # idiomatic translations may return a different-but-compatible integer width/signedness'''
    new = '''        body_call = f"{pre}        let c_ret = {call_c};\\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        # idiomatic translations may return a different-but-compatible integer width/signedness'''
    assert old in t
    t = t.replace(old, new)

    # the gate itself becomes conditional on the mode
    old = '''    gate = "        if c2r_ub_get() != 0 { return; }  // C hit UB -> reject input\\n" if ub_free else ""'''
    new = '''    gate = ("        if mode == C2R_GATED && c2r_ub_get() != 0 { return; }  // C hit UB -> reject\\n"
            if ub_free else "")'''
    assert old in t
    t = t.replace(old, new)

    # ---- emit the mode helper and wrap the body ----
    old = '''        "fuzz_target!(|data: &[u8]| {",
        "    let _ = cd();",
        "    let mut cur = Cur::new(data);",
        *decode,
        "    unsafe {",
        body_call,
        ret_cmp,
        *post,
        "    }",
        "});",
        "",
    ])'''
    new = '''        "fuzz_target!(|data: &[u8]| {",
        "    let _ = cd();",
        "    let mut cur = Cur::new(data);",
        *decode,
        "    let mode = c2r_mode();",
        "    unsafe {",
        "        if mode == C2R_RUST_ONLY {",
        "            // no C reference, so nothing can be compared; throughput bound only",
        rust_call.replace("        ", "            ", 1),
        "        } else {",
        *[l.replace("        ", "            ", 1) for l in body_call.split("\\n") if l],
        rust_call.replace("        ", "            ", 1),
        *[l.replace("        ", "            ", 1) for l in ret_cmp.split("\\n") if l],
        *[l.replace("        ", "            ", 1) for l in post],
        "        }",
        "    }",
        "});",
        "",
    ])'''
    assert old in t
    t = t.replace(old, new)

    # the helper goes right before `use <crate> as translated;` in the differential target
    old = '''        "}",
        "",
        "fn cd() -> i8 { 0 }  // silence unused on some shapes",'''
    new = '''        "}",
''' + MODE_HELPER + '''        "",
        "fn cd() -> i8 { 0 }  // silence unused on some shapes",'''
    assert old in t
    t = t.replace(old, new)

    p.write_text(t)
    print("gen_diff_harness.py patched: runtime C2R_MODE")


if __name__ == "__main__":
    main()

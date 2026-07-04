# cJSON multi-tool round-trip differential (RQ1)

Fixed real-world target (DaveGamble/cJSON, 3206 LOC): run every C→Rust tool on it, round-trip
(`Parse` → `PrintUnformatted`) each translation against the C, over a fuzzed JSON corpus (valid +
malformed + edge: nested, big/float/exp numbers, unicode/escape strings, truncated/malformed). All
findings C-backed (original cJSON C = ground-truth oracle; ASan/UBSan available for gating).

Harness: `oracle.c` (round-trip logic) linked against each cJSON implementation — the original C, or a
Rust translation compiled as a `staticlib` exposing the C ABI. Batched (one process, length-prefixed
records) for speed. Scratch: `scratchpad/cjson_diff/`.

## Results matrix

| tool | ran? | result |
|---|---|---|
| **c2rust** 0.22.1 (mechanical) | ✅ | **FAITHFUL** — 100,000 fuzzed records, 0 divergences. Validates the harness + confirms the mechanical baseline introduces no bug. |
| **CROWN** (safety lifter) | ⚠️ partial | toolchain fixed (run via CROWN scripts, `extern crate core` for c2rust-0.22 `core::ffi`); **preprocess + analyse SUCCEED** on cJSON (ownership/mutability/fatness computed). **`rewrite` CRASHES** — `internal error: entered unreachable code` at `crates/refactor/src/rewrite_fn.rs:217`, on cJSON's `print` function (local `[printbuffer; 1]` array + raw cast). Crash is independent of `--no-attempt/--force-box/--no-box/--type-reconstruction/--raw-mutability` (a CROWN rewrite bug this library triggers, not an option-tunable case). → **CROWN cannot lift cJSON out-of-the-box.** |
| **SACTOR** (framework, gpt-5.1) | ❌ FAIL | `Circular dependencies for functions is not supported yet` — cJSON's mutually-recursive descent parser (parse_value ↔ parse_array ↔ parse_object) breaks SACTOR's dependency ordering. |
| raw gpt-5.1 (naive single-prompt) | ⏸ dropped | NOT a real framework (no repair/dep-ordering) — reviewer would reject as a transpiler; produced a from-scratch 660-line rewrite (a new JSON lib, not a translation). Excluded from the narrative. |
| C2SaferRust | blocked | pipeline needs nightly-2022-08-08 slicer |
| **PtrTrans** (FSE'26, gpt-5.1) | ✅ **RAN — and yielded the 2nd headline bug cluster** | Full pipeline reproduced (SVF-2.9 built from source; PA_func/PA_struct compiled vs it; KG + Trans_PA with gpt-5.1). Result: crate **compiles**, but (a) **24/118 groups exhausted 5 repair attempts → EMPTY-BODY stubs**, incl. the recursive parse AND print cores → round-trip impossible (visible failure); (b) in tool-declared-SUCCESS code, differential found **40,133 divergences / 120,050 UB-free records** in `parse_string`: `\u` escapes always rejected (empty-slice `input_end` at call site — ptr-distance lost in ptr→slice lift), parsed string silently discarded (`valuestring = None`), non-UTF-8 rejected. ASan/UBSan-gated; `parse_hex4`/`parse_number`/`utf16` standalone = 0 diffs. **Archive: results/rq1_bugs/cjson_ptrtrans/** |

## Notes

- **CROWN toolchain now works** (the earlier metadata/`sized lang_item` errors were a wrong direct
  invocation — running via CROWN's own scripts from the crown dir, which set the nightly-2023-01-26
  sysroot, fixes it; verified analyse succeeds on both the buffer benchmark and cJSON). So CROWN is
  usable on the libraries it supports (its benchmark set: buffer/genann/lil/lodepng/quadtree/rgba — which
  overlap the Crown dataset). cJSON specifically crashes its `rewrite`.
- **Strategic:** silent lifter bugs (crc32-class) live in mechanical lifters on real-world libs; the
  productive lifter hunt is CROWN + C2SaferRust on the libraries CROWN *can* lift, compared C-backed on
  the same source. cJSON stays a "contemporary-LLM certify" case (c2rust already proven faithful here).

## Conclusion (updated after PtrTrans run): cJSON breaks 3 tools and exposes the 4th

On this real-world recursive-descent parser + dynamic-buffer library: SACTOR fails (circular deps),
CROWN crashes (rewrite), C2SaferRust blocked (old-nightly slicer). **PtrTrans is the only published
framework that completes** — and it does so by (a) stubbing out the recursive parse/print cores after
exhausting repairs (visible failure), and (b) shipping **silent semantic differences in the code it
declared successfully translated** (`parse_string` cluster: 40k divergences, UB-gated, C-backed —
results/rq1_bugs/cjson_ptrtrans/). c2rust (mechanical) remains the only faithful translator (100k/0).

Three takeaways: (1) real recursive code defeats every current framework, by crash or by stub; (2) the
silent-bug surface is exactly the LLM-reshaped code that *passes* compile-verification — the crc32
family (ptr→slice boundary semantics) reappears in a second, independent tool; (3) the call-site nature
of the `\u` bug (callee faithful in isolation; caller passes an empty-slice bound) means unit tests of
translated functions cannot find it — matcher-enabled differential through the caller can. This is the
project thesis made concrete on a second tool.

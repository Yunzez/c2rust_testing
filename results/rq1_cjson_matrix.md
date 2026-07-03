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
| **PtrTrans** (FSE'26) | ❌ blocked | pipeline runs SVF static analysis on the C first (LLVM-based, must build SVF); cJSON is not among its 16 precomputed projects. |

## Notes

- **CROWN toolchain now works** (the earlier metadata/`sized lang_item` errors were a wrong direct
  invocation — running via CROWN's own scripts from the crown dir, which set the nightly-2023-01-26
  sysroot, fixes it; verified analyse succeeds on both the buffer benchmark and cJSON). So CROWN is
  usable on the libraries it supports (its benchmark set: buffer/genann/lil/lodepng/quadtree/rgba — which
  overlap the Crown dataset). cJSON specifically crashes its `rewrite`.
- **Strategic:** silent lifter bugs (crc32-class) live in mechanical lifters on real-world libs; the
  productive lifter hunt is CROWN + C2SaferRust on the libraries CROWN *can* lift, compared C-backed on
  the same source. cJSON stays a "contemporary-LLM certify" case (c2rust already proven faithful here).

## Conclusion: cJSON is a framework tool-breaker

On this real-world recursive-descent parser + dynamic-buffer library, **every published translation
framework/lifter fails or needs heavy setup** — SACTOR (circular deps), CROWN (rewrite crash),
C2SaferRust (old-nightly slicer), PtrTrans (must build+run SVF). **Only c2rust (mechanical) translates
it, and it is faithful (100k/0).** Two takeaways: (1) these tools are fragile on real recursive code — a
finding in itself; (2) cJSON cannot yield a silent bug for us, because the tools that could introduce one
cannot run on it. The productive lifter-bug hunt (crc32-class) is therefore on the libraries these tools
DO handle — their benchmark/Crown-dataset sets (optipng/bzip2/lil/genann/lodepng/quadtree/buffer/rgba),
where C2SaferRust/CROWN/Laertes/PtrTrans ship (or CROWN now runs) — which is exactly where crc32 was found.

# E3 — deferred / flagged cells (come back to these)

Cells that hit a build/crash quirk during the depth sweep. Recorded here so the sweep keeps moving;
revisit slowly. Each has a placeholder JSON with a `note`.

| cell | issue | status |
|---|---|---|
| **bzip2 × c2saferrust** | reshaped `BZ2_bzBuffToBuffCompress(&mut Vec<u8>, &[u8])` SIGSEGVs on **100%** of inputs (survivors 0/8), even compress-only. Real crash in the reshaped Vec API or an ABI-contract mismatch on the `&mut Vec` dest. | `metric: crash-all`, no number. The crash-on-all is itself a finding (E1: this translation carries c:1+s:1). |
| **cJSON × c2rust** | crashes **early** under fuzz (reached only 3/37 fns even with a 512-byte input cap → not deep-nesting stack overflow). Cause not isolated: this crustbench cJSON version, or an early parse-path fault. | `metric: corpus-replay-floor`, median 220. Recorded as floor. Try the `crown/oldc2rust/pilot_out/cJSON` source, or bisect the crashing input. |

## Build-quirk playbook (what these taught us — apply before deferring)
- Multi-line `#![allow(...)]`: strip the whole attr block (`tail -n +<body_start>`), not `grep -vE '^#!\['`.
- `size_t = usize` vs a malloc-hook fn-ptr typed `c_ulong`: set `pub type size_t = std::os::raw::c_ulong`.
- macOS symbols on Linux: shim `__assert_rtn`, `__stderrp`, `__maskrune`, `_DefaultRuneLocale`.
- Stale nightly features: drop `const_fn_fn_ptr_basics`, `ptr_offset_from`, `raw_ref_op`, `strict_provenance`, `const_mut_refs`, `label_break_value`.
- Reshaped ABIs per tool: `Option<&mut u32>` destLen (Laertes/CROWN bzip2), `&mut Vec<u8>` (C2SaferRust), `Option<&mut [i32]>` (PtrTrans), nested `crate::src::…` modules (CROWN), `&genann` (C2SaferRust copy).
- Multi-API-surface libs (bzip2): exclude the CLI module for types via EXCLUDE_FILE arg; median over REACHED fns only.
- Crash cells: fall back to per-process corpus-replay floor; note "manual inspection: unrecoverable, the crash is the finding".

## Idiomatic-Rust translators (PtrTrans, SACTOR): mangled-name census
The runner's default census excludes `_R*` names (they're Laertes runtime shims). But
idiomatic translators keep normal Rust fns → v0-mangled `_R…`/`Px…` symbols → the exclusion
wrongly drops the whole translation. Two ways to census these:
- pass FN_LIST of the driven leaf names (keys the median by those, bypasses `_R` filter) — used for qsort×PtrTrans, genann×SACTOR;
- or re-census the cached `/tmp/rq3_<label>.json` filtering only on the lib source path (`…/src/<file>.rs`), dedup by mangled name — captures internal helpers too (cJSON×PtrTrans: this surfaced `get_decimal_point`, an internal parse_number helper a plain fn-list misses). reached 6/121, median 3.87M.

| **lil × c2saferrust** | Panics on EVERY valid script (0/5 survive) at `ptr/unique.rs:88` via `hm_destroy`/`lil_free_env` teardown -- NonNull built from a nullable C hashmap ptr. | `metric: crash-all`. Corroborates E1 lil×C2SaferRust c:1; crash-on-all is the finding. |

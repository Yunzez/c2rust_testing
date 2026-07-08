# Mapping-audit — the "mapping bug" issue class (2026-07-08)

A **second axis of defect**, orthogonal to runtime bugs: the tool's own shipped C↔Rust
function-correspondence map can be wrong. Only findable by an **independent** name-independent
matcher — every other verifier trusts the tool's map (auditing a tool with its own bookkeeping is
circular). This is the sharpest answer to "why build a matcher when the tools ship maps."

## Audited both tools that ship an explicit map

| tool | map artifact | verdict | why |
|---|---|---|---|
| **SACTOR** | `function_name_map.json` | **CLEAN** | deterministic record of its own renaming; no dangling targets, complete, renames correct (`printUsage→print_usage`) |
| **PtrTrans** | `*_trans_metadata.jsonl` `rust_definition_name` | **BROKEN — 143/255 (56%) wrong** | inferred KG alignment, a separate fallible step |

- `sactor_map_audit.py` — checks every SACTOR example: map keys ⊆ C fns, values ⊆ Rust fns, complete.
  Clean on all (atoi/fft/hamming/course_manage/cmake_multi). SACTOR produced no map on the real E1
  libs (it failed to translate them).
- `ptrtrans_map_audit.py` — lodepng, 255 function records: **143 mismatch, 101 self-consistent,
  102 airtight scrambles** (claimed Rust target name is itself a *distinct C function*).

## The receipt (PtrTrans lodepng)
```
lodepng_save_file  -> claims Rust "load_file"    (save mapped to its semantic OPPOSITE)
lodepng_set32bitInt-> claims Rust "alloc_string"
lodepng_read32bitInt-> claims Rust "memcpy"
update_adler32     -> claims Rust "deflate"
```
Not renames — the claimed name already belongs to a different function. The KG alignment is
~40% scrambled.

## Claim (in results/PROJECT_RESET_2026-07-03.md §2b)
Mapping correctness is **tool-specific, not guaranteed a priori**: deterministic-record maps are
sound, inferred maps can be badly wrong. An independent name-independent matcher is the only way to
tell them apart — and it caught a 40%-scrambled map every downstream consumer would have trusted. A
differential test built on PtrTrans's map compares ~40% wrong function pairs (false bugs + missed
bugs, invisibly). **Do NOT overclaim "pervasive": 2 tools audited, 1 sound + 1 broken — one confirmed
instance is enough to establish the class + justify the matcher's audit role.**

Correction to earlier notes: the old "143/244 off-by-one" is imprecise — it's 143/**255** mismatches,
and NOT off-by-one (only 18 strict neighbor shifts); it is broad scrambling (102 airtight).

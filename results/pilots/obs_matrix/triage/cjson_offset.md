# Triage — cJSON × PtrTrans class (c): `offset` differs on the failure path (2 records)

Cell: `results/pilots/obs_matrix/cjson_ptrtrans/` (seed 42, commit c2471df). Records: idx 40 (`3f6b2273…`, 136 B) and
idx 138 (`fc744c1f…`, 8 B). Both: C ret=0, Rust ret=0, valuestring NULL/None, only `buffer.offset` differs.
Replayed 2026-08-26 with the cell's own `driver.c` rebuilt `clang -O1 -g -fsanitize=address,undefined
-fno-sanitize-recover=all` against cJSON.c sha256 `298581a0…` (= `harness/cJSON.c.sha256`) and the cell's `obsdrv`
binary. **ASan/UBSan: 0 reports on both records and on every minimal repro below (C-UB 0).** Deterministic on re-run.

Provenance (U4): C oracle = cJSON **v1.7.19 version-matched reconstructed** (re-fetched from upstream tag; not the
file PtrTrans consumed). Both findings below depend only on `parse_string` logic that is unchanged across 1.7.x.

## Record 138 — `\/"\"\b"` (does not start with `"`)

| | C | Rust |
|---|---|---|
| ret | 0 | 0 |
| offset after call | **1** | **0** |

Minimal repro: any 1-byte input that is not `"` — `printf 'a' \| driver_c silent st` → C `offset:1`; `obsdrv` → `offset:0`.
Also `\` alone and the empty input (C reads `content[0]` of the harness's static 64 KiB buffer, so no UB in the harness;
in the library this path is only reached via `parse_object`, which never guards the key with a quote check).

Root cause — C pre-initialises `input_pointer` to `offset+1` *before* the "not a string" check, and the `fail:` label
stores it back; Rust ports the fail path with `current_offset` (the un-incremented start) instead.

C (`cJSON.c:821-830, 935-943`):
```c
    const unsigned char *input_pointer = buffer_at_offset(input_buffer) + 1;   /* +1 BEFORE the check */
    const unsigned char *input_end = buffer_at_offset(input_buffer) + 1;
    /* not a string */
    if (buffer_at_offset(input_buffer)[0] != '\"') { goto fail; }
    ...
fail:
    if (output != NULL) input_buffer->hooks.deallocate(output);
    if (input_pointer != NULL)
        input_buffer->offset = (size_t)(input_pointer - input_buffer->content);  /* = start + 1 */
    return false;
```
Rust (`translated_crate/src/cjson.rs:712-741`):
```rust
    let mut current_offset = input_buffer.offset;
    let fail_with_offset = |offset_ref: &mut usize, ip: Option<usize>| -> i32 { if let Some(i) = ip { *offset_ref = i; } 0 };
    ...
    if content[current_offset] != b'"' {
        { let current_copy = current_offset;                       // <-- start, not start+1
          let _ = fail_with_offset(&mut current_offset, Some(current_copy)); }
        input_buffer.offset = current_offset;
        return 0;
    }
    // input_pointer_index is only created AFTER the quote check:
    let mut input_pointer_index = current_offset + 1;
```
The reshaping (pointer → index) moved the `+1` initialisation below the early-exit, so the early-exit no longer sees it.
All *later* fail paths (unterminated string, bad escape, trailing backslash) use `input_pointer_index` and match C
(`"ab`, `"\i"` → offset 1 on both sides; 26 "agree" records in the cell are exactly these).

Externally observable? **In C, yes**: `cJSON_ParseWithOpts`/`cJSON_ParseWithLengthOpts` copy `buffer.offset` into
`return_parse_end` and `global_error.position` (`cJSON.c:1195-1215`), which `cJSON_GetErrorPtr()` returns; the
not-a-string path is reached through `parse_object`'s key parse, which calls `parse_string` without a quote check
(`cJSON.c:1713-1719`). Whole-program C check (ASan+UBSan clean): `cJSON_ParseWithOpts("{a:1}", &end, 0)` → NULL,
`return_parse_end` = `GetErrorPtr()` = index **2** (the `:`; C's documented-by-behaviour off-by-one). A faithful Rust
would give index 1. **In the translated crate, no**: `parse_object` (`cjson.rs:2887`, returns 0) and
`cJSON_ParseWithLengthOpts` (`cjson.rs:3095`, returns None) are stubs, so nothing in the crate ever propagates
`ParseBuffer.offset` to `return_parse_end`/`GLOBAL_ERROR`; `parse_value` only calls `parse_string` after its own `"` check.

**Verdict: distinct root cause, confirmed function-level divergence, NOT promoted to a defect id.** Justification: it is
real (deterministic, UB-free, root-caused to a reshaping slip that is independent of S7/S8/S9), but it is (i) only
a failure-path error-position off-by-one (no value/return effect), and (ii) unreachable at every implemented API boundary
of the translated crate because both externalising callers are stubs. Record as a `parse_string` contract divergence in
the cell's supporting-record column (1 record); if PtrTrans ever fills the stubs it becomes a candidate for
`cJSON_GetErrorPtr` misreporting. Suggested label if the user wants an id anyway: `S14-cand (minor, unexternalised)`.

## Record 40 — 136 B, `"b…\uiiiiiii…b\iii…"`

| | C | Rust |
|---|---|---|
| ret | 0 | 0 |
| offset after call | **55** | **11** |

Byte 11 is `\u` followed by non-hex `iiii`; byte 55 is `\i` (invalid escape). Minimal repro: `"A\i"` (10 B) →
C `offset:7`, Rust `offset:1`; `"\uiiii\i"` gives the same numbers.

Root cause — **S7** (empty-slice `input_end` fabricated at the call site, `cjson.rs:823-830`). Rust's
`utf16_literal_to_utf8` sees `input_end.len()==0 < 6` and returns 0, so `parse_string` fails *at the `\u`* (offset 11).
C decodes the `\u` (for `\uiiii`, `parse_hex4` returns 0 → U+0000 is accepted — the "C's behaviour is the questionable
one" bonus already noted under S9) and keeps scanning until the genuinely invalid `\i` at 55.
```c
                case 'u':
                    sequence_length = utf16_literal_to_utf8(input_pointer, input_end, &output_pointer); /* real bound */
                    if (sequence_length == 0) goto fail;
                    break;
                default: goto fail;                      /* C reaches this at byte 55 */
```
```rust
                b'u' => {
                    let input_slice = &content[input_pointer_index..input_end_index];
                    let end_slice = &content[input_end_index..input_end_index]; // S7: len()==0 -> always fails
                    sequence_length = utf16_literal_to_utf8(Some(input_slice), Some(end_slice), Some(&mut output));
                    if sequence_length == 0 { ... input_buffer.offset = input_pointer_index /* =11 */; return 0; }
```
Both sides fail because the input is genuinely invalid JSON; only *where* they give up differs, and the "where" is
entirely decided by S7.

**Verdict: symptom of S7**, not a new defect. Justification: with S7 fixed (a real remaining-input slice), Rust would
also reach byte 55 — the record has no independent root cause. It is a fourth *surface* of S7 (S7's three archived
surfaces are all ret 1→0; this one is ret-equal/offset-only, visible to O-S but not O-R/O-P), which is why the
`\u`-escape sub-class in this cell is 1 record under O-R but 2 under O-S.

## Effect on the cell's counts (U1/U2)
- 111 confirmed divergent records stay 111 (classes (a)+(b)); class (c) resolves to **1 record attributable to S7**
  (idx 40) + **1 record with a distinct, unexternalised root cause** (idx 138). Defect count for the cell: unchanged
  (S7, S8, S9); no new S-id promoted.
- `harness/driver.c` note: for the empty input C reads `pay[0]` of a static buffer — harmless here, but a real caller
  would be `can_access_at_index`-guarded; not a finding.

## Repro commands (scratchpad `triage/cjson/`)
```
clang -O1 -g -fsanitize=address,undefined -fno-sanitize-recover=all -I. driver.c -o driver_c -lm   # cJSON.c sha 298581a0…
printf 'a'            | ./driver_c silent c.st ; ./obsdrv silent r.st < <(printf 'a')            # offset 1 vs 0
printf '"\\u0041\\i"' | ./driver_c silent c.st ; ./obsdrv silent r.st < <(printf '"\\u0041\\i"') # offset 7 vs 1
clang ... wp.c cJSON.c && ./wp     # {a:1} -> return_parse_end == GetErrorPtr == idx 2 (C side only; Rust entry is a stub)
```

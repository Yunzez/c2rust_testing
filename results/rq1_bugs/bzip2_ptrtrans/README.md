# bzip2 × PtrTrans (FSE'26, gpt-5.1, Trans_PA): compile-fail

**Verdict: `✗(compile)`** — same class as lil (faithful-or-fail → fail). PtrTrans's full pointer-analysis
method translated the 7 bzip2 library files but the assembled crate **does not compile**, so there is no
runnable artifact to differential-test.

## Numbers
- **78 code units, 55 stub-reverts** (71% — the repair loop exhausted 5 attempts and emitted empty-body
  stubs; even higher than lil's 58%).
- **73 compile errors**, all module-assembly / duplicate-definition, NOT translation-logic:
  E0255 ×30 (name defined multiple times), E0432 ×19 (unresolved imports), E0252 ×17 (duplicate imports),
  E0428 ×4 (duplicate item defs: `Bool`, `UChar`, `Int32`, `BzFile`), E0433/E0412 ×2. The shared c2rust
  types (`Bool`/`UChar`/`Int32`/…) are emitted in multiple module files with no shared `common` module —
  PtrTrans's own dedup/assembly step failed and its repair loop could not resolve it.

## What WAS translated (but can't run)
`BZ2_bzBuffToBuffCompress` was translated with the aggressive ptr→slice reshaping PtrTrans is known for —
`dest`/`source` became `Option<&mut [u8]>` (see `excerpt_buff2buff_reshaped.rs`). This is the same
call-site-contract-risk shape that produced headline #2 in cJSON (`parse_string`'s empty-slice bound). If
the crate compiled it would be the prime differential target — but it doesn't, so we cannot attribute a
runtime bug. Honest verdict: compile-fail.

## Method note (reproducibility, post-reboot)
The PtrTrans pipeline was rebuilt from scratch after a machine reboot wiped /tmp (now at the permanent
`tools/frameworks/ptrtrans_rebuild/`). Two PtrTrans-internal pipeline bugs were patched to let the run
reach the LLM stage — both are the tool's bugs, not our setup: (1) `lib.rs` module list included the
crate root itself → `pub mod lib;` circular (main.py ~L1671, excluded `lib.rs`); (2) `struct_path_result`
joined a dir-prefixed filename onto the expanded path → FileNotFound (made basename + try/except). SVF
reports were pre-generated with our compiled `pa_func`/`pa_struct` (the pipeline's own SVF `run.sh` fails
on a jsoncpp/nlohmann mismatch). None of these touch translation content — they only unblock the
pipeline to the point where it produces (a non-compiling) translation.

## bzip2 breaks ALL FOUR non-mechanical tools
- **Laertes**: CRC-table zeroed (compiles, 91% wrong-CRC) — headline #4
- **C2SaferRust**: NULL/empty conflation + `endsInBz2` panic (compiles, buggy)
- **CROWN**: corrupt compress + heap corruption + broken decompress (compiles, buggy) — headline #3
- **PtrTrans**: does not compile (55/78 stubs + 73 assembly errors) — this file
Only mechanical c2rust is faithful. Four tools, four distinct failure modes; three ship buggy-compilable
output, PtrTrans can't even assemble.

## Files
- `compile_errors.txt` — the 40+ cargo errors
- `excerpt_buff2buff_reshaped.rs` — the translated `BZ2_bzBuffToBuffCompress` signature
- `generated_lib.rs` — the crate root (shows the duplicate module/use structure)
- Full generated crate: `tools/frameworks/ptrtrans_rebuild/PtrTrans-C2Rust/dataset/PA_trans_projects/bzip2/`

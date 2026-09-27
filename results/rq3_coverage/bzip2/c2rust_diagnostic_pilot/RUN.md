# bzip2 × c2rust — diagnostic pilot (rejected as an RQ4 cell)

> [!WARNING]
> **DIAGNOSTIC PILOT — NOT AN RQ4 RESULT. Must not populate the paper table.**
>
> Rejected by the user on 2026-09-03 for three reasons, all correct:
> 1. **One boundary only.** The method tests *all eligible matched pairs*; the "ours" side here is a
>    single `BZ2_bzBuffToBuff*` harness, so it is not whole-validator coverage.
> 2. **Not a differential corpus.** The 1,163 inputs were generated with no C execution, no UB gate
>    and no C/Rust comparison — a Rust-only reach campaign.
> 3. **Budget comparison invalid.** 0.23 s of tests against 2 s of replay is meaningless: the corpus
>    behind that replay cost ~800k fuzz executions to produce. The real cost is corpus generation.
>
> What it may be cited for, and nothing more:
> *A buffer-oriented harness reaches two APIs absent from the shipped tests, but a single boundary
> does not represent whole-validator coverage.*
>
> The valid experiment is being redone in `../c2rust/`: enumerate all matched pairs, apply the frozen
> harness-eligibility rules, generate a real differential harness (C → UB gate → Rust → compare) per
> eligible pair, run them under one fixed artifact-level wall-clock budget, and union the Rust
> coverage across harnesses.

Run 2026-09-03. Both sides executed; see the warning above for why this is not a cell result.

## 1. Artifact

`tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/` — the mechanical c2rust baseline shipped
with the Laertes benchmark set. Its library sources are byte-identical to the git-tracked copy at
`fuzz/bzip2_c2rust_e3/src/` (verified by `cmp` for all seven scoped files plus `bzip2.rs`).
Per-file sha256 in `artifact_hashes.json`.

The crate was transpiled on macOS: it references `__stdinp`, `__stdoutp`, `__stderrp`, `__maskrune`,
`_DefaultRuneLocale`, `__error`. The original `c2rust-lib.rs` also uses feature gates that modern
nightly removed (`const_fn_fn_ptr_basics`, `ptr_offset_from`, `const_mut_refs`).

**Adapters used** (both representation-only; neither touches bzip2 logic):
- `raw/lib.rs` — crate root declaring the same nine modules under gates `nightly-2025-09-01` accepts.
  It deliberately does **not** stub the macOS symbols the way `fuzz/bzip2_c2rust_e3/src/lib.rs` does
  (that file defines `__maskrune → 0` and `__stderrp → null`, which would silently break the CLI).
- `raw/shims.c` — Linux definitions of those symbols, derived from the repo's existing
  `results/rq4_effectiveness/bugs/bzip2_crown/darwin_shims.c` and extended with
  `__stdinp`/`__stdoutp`/`__error`.
- `raw/bzip2cli.rs` — three lines calling `bz_cov::bzip2::main()`. The translated CLI exists in the
  crate but is never declared as a `[[bin]]` (`autobins = false`).
- `raw/covreplay.rs` — replay driver, a line-for-line transcription of the archived fuzz target
  `fuzz/bzip2_c2rust_e3/fuzz/fuzz_targets/ft.rs`.

Test support classification: **representation-only adapter**.

## 2. Sides

| | tests | ours |
|---|---|---|
| what | the suite bzip2 ships: `Makefile` `test:` — compress `sample{1,2,3}.ref` at `-1/-2/-3`, decompress `sample{1,2,3}.bz2` (`-d`,`-d`,`-ds`), then six `cmp`s against the shipped references | replay of the archived libFuzzer corpus `fuzz/bzip2_c2rust_e3/fuzz/corpus/ft/` |
| binary | `bzip2cli` | `covreplay` |
| unit | 6 program executions, 6 `cmp` assertions | 1 process per input |
| result | **6 discovered / 6 executed / 6 passed / 0 failed / 0 skipped** | **1163 discovered / 1163 processed / 1162 replayed / 1 skipped (<3 bytes, exactly the fuzz target's own early `return`) / 0 failed** |
| raw profiles | 6 `.profraw` | 1163 `.profraw` |
| wall clock | 0.23 s | 2 s |

The translated CLI passes the shipped acceptance suite unmodified, so the tests side is a genuine
acceptance baseline, not a failing run.

**Corpus caveat (must travel with these numbers).** The archived *differential* corpus for this cell
(`scratchpad/bz_crown_diff/`, 1608 records) is lost. Under the relaxed reading chosen on 2026-09-03,
the "ours" side is the E3 reach-cell campaign corpus: coverage-guided libFuzzer, 8 urandom seeds,
`-runs=100000`, `--sanitizer=none`, **no C reference executed at generation time**. It is therefore a
campaign corpus on the translated artifact, not a differential-validation corpus, and no input could
be excluded by a C-UB gate (there is no C side in the replay). It is also a *depth* census budget, not
a coverage campaign budget.

## 3. Scope

`scope.json`. Path whitelist over the seven translated bzip2 **library** files. `bzip2.rs` (the
translated CLI *program*) and `bzip2recover.rs` (a second program) are excluded, as are the two
adapters, the crate root, `shims.c` and `build.rs`. `crctable.rs` and `randtable.rs` are in scope but
hold only static tables, so they add 0 to both denominators; `llvm-cov` omits them for that reason.
Both sides use this identical scope and denominator, and the analysis verified that the two builds
produce the *same* denominator (66 functions, 8789 regions) — recorded as `problems: []` in
`result.json`.

## 4. Multi-harness deduplication

Not applicable in the sense of §F: this cell has **one** harness on the ours side, so no function
appears in two harness binaries and nothing was summed across reports. The two *sides* do use two
different binaries, so function identity is matched across them by
`(source file, function symbol, start line)` and region identity by
`(source file, start line, start col, end line, end col)` — see `raw/analyse.py`. The two exports
agree on the full identity set, which is the cross-binary consistency check.

## 5. Result

| | total in scope | tests | ours | both | only tests | only ours | union |
|---|---:|---:|---:|---:|---:|---:|---:|
| functions | 66 | 51 | 40 | 38 | 13 | 2 | 53 |
| regions | 8789 | 7007 | 4274 | 4057 | 2950 | 217 | 7224 |

- test function coverage **0.773**, our function coverage **0.606**, growth **−0.167**
- test region coverage **0.797**, our region coverage **0.486**, growth **−0.311**

All five stage-G sanity checks pass for both function and region level.

### What the set difference actually says

The two sides enter the library through **disjoint API layers**, which is why the aggregate is
negative while the intersection is not the whole story:

- `only_ours` is exactly two functions — `BZ2_bzBuffToBuffCompress` and
  `BZ2_bzBuffToBuffDecompress`, the buffer-to-buffer entry points. The shipped CLI never calls
  them; only our harness does.
- `only_tests` is the **stream** API (`BZ2_bzWriteOpen`, `BZ2_bzWrite`, `BZ2_bzWriteClose64`,
  `BZ2_bzReadOpen`, `BZ2_bzRead`, `BZ2_bzReadClose`, `BZ2_bzReadGetUnused`, `myfeof`) plus five
  `blocksort` functions (`mainSort`, `mainQSort3`, `mainSimpleSort`, `mainGtU`, `mmed3`). The
  blocksort main path only fires on blocks large enough to leave the fallback sorter; the corpus
  inputs are small, so the campaign never reaches it.
- `covered_by_neither` is 13 functions: the `FILE*` convenience layer (`BZ2_bzopen`, `BZ2_bzdopen`,
  `BZ2_bzread`, `BZ2_bzwrite`, `BZ2_bzclose`, `BZ2_bzflush`, `BZ2_bzerror`, `BZ2_bzlibVersion`,
  `bzopen_or_bzdopen`, `BZ2_bzWriteClose`), `BZ2_bz__AssertH__fail`, `isdigit`, `__isctype`.

The negative growth is a property of this corpus and this harness boundary, not evidence that
differential validation covers less than shipped tests: the harness was built for a one-function
depth census, and the corpus it left behind is the only one that survived.

## 6. Reproduction

```
CARGO_TARGET_DIR=<scratch>/c2rust/target RUSTUP_TOOLCHAIN=nightly-2025-09-01 \
RUSTFLAGS="-C instrument-coverage -C codegen-units=1 -C link-dead-code" \
  cargo build --release --bins           # in a crate assembled from raw/{Cargo.toml,build.rs,shims.c,lib.rs,*.rs}

# tests
LLVM_PROFILE_FILE=<p>/tests/%m-%p.profraw ./bzip2cli -1  < sample1.ref > sample1.rb2   # ×6, then 6 cmps
# ours
LLVM_PROFILE_FILE=<p>/ours/ft/%m-%p.profraw ./covreplay <one corpus file>              # ×1163

llvm-profdata merge -sparse <p>/tests/*.profraw -o tests.profdata
llvm-profdata merge -sparse <p>/ours/ft/*.profraw -o ours.profdata
llvm-cov export ./bzip2cli  -instr-profile=tests.profdata > tests_coverage.json
llvm-cov export ./covreplay -instr-profile=ours.profdata  > ours_coverage.json
python3 raw/analyse.py .
```

Peak scratch disk: 6.5 MB (target dir) + 28 MB (raw profiles). Archived here: 3.3 MB
(`profiles/ours/ft.profraw.tar.gz` holds the 1163 raw profiles).

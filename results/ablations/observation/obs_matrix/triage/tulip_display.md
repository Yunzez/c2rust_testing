# Triage — tulipindicators × C2SaferRust: 9 "stdout-only / both-exit-0" records (+ the 21 both-exit-1 and 47 exit-visible)

Cell: `results/ablations/observation/obs_matrix/tulip_c2saferrust/` (seed 42, commit c2471df). WIP crate =
`tools/frameworks/c2saferrust/laertes_benchmarks/tulipindicators_WIP` (copied to scratchpad, `cargo +nightly-2025-09-01
build --release --bin sample`). C oracle used by the cell = `tools/frameworks/tulipindicators/{sample.c,tiamalgamation.c}`.
Replayed 2026-08-26; C side under ASan+UBSan (`halt_on_error=1:exitcode=99`): 0 reports on every record discussed here.

## Finding 0 — the cell's C oracle is the WRONG VERSION (provenance, U4)
`tools/frameworks/tulipindicators` is a shallow checkout of upstream `be18abb` (2024-01-08), **TI 0.9.2**
(`tiamalgamation.c:53`). Every translated crate (base c2rust, crown, laertes, WIP) prints `TI VERSION: 0.8.4, TI BUILD:
1537377628` — upstream commit **6b3ff6d** (2018-09-19). Cloned upstream into the scratchpad, checked out 6b3ff6d, built
`sample.c + indicators_index.c + indicators/*.c + utils/*.c` with the same sanitizer flags (`triage/tulip/sample_c084`,
prints 0.8.4/1537377628). Differences 0.8.4 → 0.9.2 that matter for this cell:
1. `indicators_index.c` at 0.8.4 has `adx`, `adxr`, `dx` all declared with inputs `{high,low,close}` and output name
   `dx` (upstream table bug, fixed later); 0.9.2 has `{high,low}` / `adx`,`adxr`,`dx`. All four crates carry the 0.8.4
   entries verbatim (`indicators_index.rs:1185-1260` in WIP; same in base at 1180-1254).
2. `sample.c` diff is exactly two added blocks in 0.9.2: an argv-echo loop after `*ERROR NOT ENOUGH OPTIONS*` and
   after `*ERROR INVALID OPTION*` (lines 176-179, 240-243). Nothing else.
So the cell's manifest value must be **"unknown / version-mismatched (oracle 0.9.2 vs translation 0.8.4)"**, not
exact-source. It should be re-run against 6b3ff6d (recipe at the end); the recount below is that re-run.

## The 9 both-exit-0 records — two groups

### Group A (4 records: idx 33 `dx 15 14 16`, 139 `dx 18 14 13`, 144 `adxr 8 3 14 9`, 163 `adxr 15 8`) — harness artifact
Stdout diff: Rust prints an extra `close` column and header `dx` (also for `adxr`); numeric output column identical
(`dx 15 8`: 64.735 on both). Cause = Finding 0 item 1: the WIP prints exactly what the 0.8.4 table says.
4-way check: **C-0.8.4 == base == WIP** byte-for-byte; only C-0.9.2 differs.
**Verdict: harness/provenance artifact (C oracle version mismatch), not a defect.** Zero attributable to C2SaferRust.

### Group B (5 records: idx 23 `cosh 8 17`, 83 `tanh 16`, 85 `todeg 7 1 2 15`, 179 `acos 11 15 2`, 199 `acos 5 1 4 8`) — confirmed distinct WIP defect
Stdout diff: the `input` column (the `alternative[]` series 0.20, 0.30, …) is missing from header and every row;
indicator values identical. Affects all 7 "alternative-input" indicators (`acos asin atan cosh sinh tanh todeg`).
4-way check: **C-0.8.4 == C-0.9.2 == base ≠ WIP.**
Minimal repro: `sample cosh 1` (the trailing `1` is needed only to get past S13):
```
C / base:  date         input   cosh          WIP:  date         cosh
           2005-11-01     0.20    1.020              2005-11-01    1.020
```
Root cause — `sample.rs:228-241` (WIP): the rewrite turned C's `while (*alt)` into `while let Some(&a) = alt.next()`,
whose pattern variable `a` **shadows the display flag `a`**; the LLM then deleted the flag assignment as "incorrect":
C (`sample.c:150-165`, identical in 0.8.4 and 0.9.2) / base c2rust (`sample.rs:272-280`, faithful):
```c
        while (*alt) {
            if (strcmp(*alt, info->name) == 0) {
                r=0;
                a=1;                       /* print the alternative series as the "input" column */
                for (j = 0; j < info->inputs; ++j) inputs[j] = alternative;
                break;
            }
            ++alt;
        }
    ...
        if (a) printf(" input  ");   /* header */      if (a) printf(" %8.2f", alternative[i]);   /* rows */
```
WIP (`tulipindicators_WIP/sample.rs:228-241`):
```rust
let mut alt = alts.iter();
while let Some(&a) = alt.next() {                       // `a: &str` shadows `let mut a = 0` (the flag)
    if a == unsafe { CStr::from_ptr(info.as_ref().unwrap().name) }.to_string_lossy().as_ref() {
        r = 0;
        // Removed the assignment to `a` since it was incorrect      // <-- BUG: flag never set
        j = 0;
        while j < info.as_ref().unwrap().inputs { inputs[j as usize] = alternative.as_mut_ptr(); j += 1; }
        break;
    }
}
...
        if a != 0 { printf(b" input  \x00" ...); }      // dead: a is still 0
```
Computation is unaffected (`inputs[j] = alternative` still happens); only the echo of the input series is lost.
**Verdict: confirmed distinct defect — driver/display rewrite, name-shadowing → dropped flag assignment. New id
candidate (S14, tulip × C2SaferRust, class semantic/driver-rewrite, minor: output-table column loss, values intact).**
Not S13 (argc unaffected — all 5 records are over-supplied and pass the argc check on both sides), not C6, not a float
formatting difference (all `%8.2f`/`%8.3f` printf calls are byte-identical when the column is present).

## The 21 stdout-only/both-exit-1 records and the 47 exit-visible records — one root cause? Mostly.
Census by (k = options supplied, need = options required), C-0.9.2 oracle:
- **47 exit-visible**: all `k == need`, C exit 0 (table) vs WIP exit 1 (`*ERROR NOT ENOUGH OPTIONS*` on stderr). = **S13**.
- **21 both-exit-1** splits into:
  - 16 with `k == 0 < need` (`adx`, `decay`, `macd`, …): C `*ERROR NOT ENOUGH OPTIONS*`, WIP `No indicator given.` —
    the WIP's `argc−1` makes `argc == 1`, so the `argc < 2` guard fires. = **S13, second symptom.**
  - 1 idx 185 `psar 14 10` (`k == need`): C `*ERROR INVALID OPTION*` (psar rejects step > max), WIP never reaches the
    indicator because `argc` is short. = **S13.**
  - 3 idx 64 `ppo 18 9 10`, 125 `adx 1 1 7`, 129 `kvo 12 6 19` (`k > need`): both `*ERROR INVALID OPTION*`; only the
    0.9.2 argv-echo line differs. C-0.8.4 == base == WIP. = **version artifact (Finding 0 item 2), not a defect.**
  - 1 idx 93 `ultosc 1 5` (`k < need`, C also errors): C prints `*ERROR NOT ENOUGH OPTIONS*` (+0.9.2 echo) on stdout,
    WIP prints it via `eprintln!` on stderr, stdout empty. Same exit code. = **stream-routing deviation of the same
    rewritten guard (`sample.rs:247`, printf → eprintln!)**; it is part of the S13 rewrite site but is a separate,
    trivial symptom that would persist after fixing `argc`. Not a defect on its own; note it under S13 as
    "error text moved to stderr" (it is also why the 47 exit-visible records show empty Rust stdout).
So: **47 + 16 + 1 (+1 stderr-routing) = 65 records are S13; 3 are oracle-version artifacts.** The RESULT.md statement
"47 + 21 = one root cause" is correct for 65 of 68 and wrong for the 3 argv-echo records.

## Recount of the whole cell against the exact 0.8.4 oracle (scratchpad `triage/tulip/recount_084.json`)
202 inputs: 1 C-UB (`hma 1 4`, also under 0.8.4) + 1 Rust-failure (no-args SIGSEGV, C6) + 123 agree +
**47 exit-visible (S13)** + **23 stdout-only** (16 + 1 + 1 S13 symptoms, 5 Group B) + **7 records that diverge ONLY
against the 0.9.2 oracle** (Group A ×4, argv-echo ×3). Honest cell numbers with a version-matched oracle:
semantic-difference **70** (not 77), distinct defects **2 → 3** (C6 crash, S13 argc, S14-cand display-column loss).
Divergence table cells that change: O-P-print / O-F-print 78 → 71; O-R/O-S/O-P-silent 48 unchanged (all 7 artifacts are
stdout-only, both-exit-equal).

## Repro / recipe
```
git clone https://github.com/TulipCharts/tulipindicators ti && cd ti && git checkout 6b3ff6d      # TI 0.8.4, build 1537377628
clang -fsanitize=address,undefined -fno-sanitize-recover=all -O1 -g -I. sample.c indicators_index.c indicators/*.c utils/*.c -lm -o sample_c084
./sample_c084 cosh 1 | head -2 ; WIP/target/release/sample cosh 1 | head -2          # Group B: input column missing in WIP
./sample_c084 dx 15 8 | head -1 ; WIP/target/release/sample dx 15 8 | head -1       # Group A: identical (close column is 0.8.4's table)
./sample_c084 ppo 18 9 10 ; WIP/target/release/sample ppo 18 9 10                   # identical (no argv echo in 0.8.4)
```
Note: `make tiamalgamation.c` at 6b3ff6d produces a broken file with a modern `echo -e`; compile from the sources instead.

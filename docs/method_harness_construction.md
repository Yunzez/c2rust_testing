# Method, as implemented (2026-09-06): from a C/Rust pair to a confirmed defect

This is the concrete procedure the RQ4 cells run. It replaces the earlier hand-schema description:
nothing below is written per boundary by a person. The generator reads the C source and the
translated crate, derives a plan, lowers it, emits the harness, and the campaign machinery runs,
measures, adjudicates and archives it. Where a step cannot be derived, the boundary **fails
construction with a reason** — it is never approximated.

Code: `tools/stu_selector/harness_plan.py` (planning, lowering, Rust bridge),
`tools/stu_selector/gen_diff_harness.py` (generation), `scripts/c2r_funnel.py` (build fixups),
`scripts/rq4/cell.py` (preflight, campaign, coverage), `scripts/rq4/replay_cell.py` and
`scripts/c2r_campaign.py confirm` (adjudication), `scripts/c2r_coverage.py` (partition),
`scripts/rq4/{confirm_cell,merge_boundary,annotate_campaign_status,archive_cell,run_md}.py`.
Design rules: `docs/harness_plan_architecture.md`, `docs/harness_oracle_plan.md`,
`docs/producer_bridge_pilot.md`; operations: `docs/rq4_runbook.md`; protocol:
`results/rq3_coverage/PROTOCOL.md`.

```
pair (C source + flattened translation)
  │  1. matching (frozen RQ1 output): which C function ↔ which Rust function
  ▼
planner  ── BodyAnalyzer(C AST) ──► InputPlan (one spec per parameter, evidence per fact)
  │            ├─ producer bridge (T* to a non-POD struct → the library's own constructor)
  │            └─ buffer tables (T** indexed by constants → one pointer parameter per row)
  │  2. return contract (comparison ladder rung for the return value; plugin if compatible)
  │  3. Rust bridge (how each canonical value becomes the translation's argument, or fail)
  ▼
lowering ──► schema (generated IR, params + return) ──► generator ──► harness crate
  │  4. fixups (siblings, shims, static re-export, plugin halves) → cargo fuzz build
  ▼
cell     ── 5. preflight (empty input ×2 modes, 60 s run) ── 6. campaign (rust-only, 3600 s)
  │      ── 7. coverage (rust-only, per harness, identity = file:line/region)
  │      ── 8. combined replay of the same corpus (C beside Rust, ladder on) → candidates
  ▼
confirmation ── 9. c-only (ASan+UBSan) / rust-only (ASan) / combined / rust-no-sanitizer → verdict
  │         ── 10. cluster by site → defect manifest (or downstream-blocked / noise class)
  ▼
archive (RUN.md numbers by program, §7 prose by hand)
```

---

## 1. Inputs: the pair, and what "a boundary" is

A **pair** (`benchmark/pairs/rq4/<lib>_<tool>/`) holds `source/` (the C the translator consumed,
one translation unit — an amalgamation `#include`ing the rest where the library is multi-file),
`translated/<lib>_<tool>.rs` (the crate flattened into one file by
`scripts/flatten_translation.py`, module bodies byte-for-byte with a line map back to the
original files, plus root re-exports), `build/compile_commands.json` (how to parse the C), and
optionally `drivers/` (the library's shipped test driver, used as *evidence* for producer ranking,
never linked) and `preflight_accept.txt`.

A **boundary** is one C function with a matched Rust function of the same name in the
translation. C `static` functions are boundaries too (the generator exposes the one it targets;
the funnel appends the crate-root re-export, with the namespace prefix a translator like CROWN
adds). The matching is the frozen RQ1 output; the planner refuses a target absent from the
translation.

## 2. Planning: the InputPlan is derived from the C body, with evidence

`harness_plan.py::build_plan` parses the entry's signature (`parse_entry_signature`: kind per
parameter — `scalar`, `ptr`, `ptr_ptr`, `ptr_struct`, `void_ptr`, element type and width,
constness) and runs **`BodyAnalyzer`** over the entry's own body — never over a caller:

* **rejection guards**: leading `if (C) return <pure>;` narrow a scalar's accepted domain
  (`period < 1` → period ≥ 1);
* **subscripts** `p[expr]`: an upper bound of `expr` from loop induction variables and constants
  (`for (i = 0; i < n; ++i) p[i]` → extent `n`), a lower bound for the ≥ 0 obligation, and the
  parameters the index transitively depends on (through local assignments and initialisers);
* **dereferences** `*p` (read / written), **escapes** (p passed to a callee, which callee),
  **loop-trip parameters**, **row aliases** (`input = inputs[0]` names row 0 of a table),
  **advanced pointers** (`*out++`, `p += k`: accesses relative to a moving base bound nothing).

`analyze_inputs` turns the facts into one **adapter** per parameter, with the evidence
(rule name, file:line, snippet) attached:

| C parameter | adapter | how it is sized / valued |
|---|---|---|
| scalar with a guard or that controls a loop trip or an extent | `bounded_scalar` | guard domain ∩ policy clamps: `max_trip` 1024 for trip counts; `extent_fits_allocation` / `index_clamped_to_allocation` (rule 6) so the harness's own allocation is never exceeded |
| other scalar | `scalar` | full range of the C type |
| scalar that is exactly a buffer's proven extent, or paired by name+adjacency | `length` | derived from the buffer's decoded length, never decoded independently |
| `T*` read with a proven extent | `input_array` | allocation = the proven extent (evaluated against the scalar caps); fuzz-filled |
| `T*` written / read+written | `output_array` / `inout_array` | same sizing; zero-filled unless read |
| `T*` whose extent could not be proven | policy allocation (4096 elements, ≤ 1 MB) | every index dependency clamped to it; recorded as the unproven obligation it is |
| `const char*` with no length and no provable extent, never written | `input_string` | NUL-terminated, length from the fuzz bytes |
| `T*` + length with capacity passed by pointer | `output_buffer` + `capacity_ptr` | |
| `char**` indexed with an adjacent count | `input_string_pointer_table` | ≤ `max_table_rows` rows |
| `T**` of scalar element type indexed **only by constants** | `buffer_table` | rows = max constant index + 1; each row is a pseudo pointer parameter (`inputs__row0`) sized by the rules above through its alias; a row nobody touches gets the policy allocation |
| `T*` to a POD struct | `struct_value` | decoded field by field |
| `T*` to a struct with pointer fields | `produced_object` | the **producer bridge** (§3) — or construction fails |
| `void*` | `null_pointer` | NULL, complete only if the entry never uses it |
| any parameter flowing into `fopen`/`system`/`unlink`/… | **fails**: environment input | |

Two invariants: **capacity never comes from a caller's array declaration** (rule 7), and a
scalar the plan clamps is clamped for the harness's own safety, never to make the callee happy.
The plan is translator-independent: it is derived once from the C side; only the bridge differs.

Every length-carrying buffer is allocated with a sentinel element past `len` (a length-0 buffer
is still a valid, NUL-terminated pointer — lil's `strlen(code)` when `codelen == 0`).

## 3. The producer bridge: objects the library builds itself

A `T*` whose struct carries pointers (`lil_t`, `genann*`, `cJSON*`) cannot be decoded from bytes
without reconstructing invariants. The pilot (`docs/producer_bridge_pilot.md`) builds it **on each
side with the library's own constructor**:

* candidates: functions in the pair that return `T*`, exist and are `pub` in the translation,
  return a raw pointer there, and whose own parameters the InputPlan can construct (scalars,
  strings, buffers — `cJSON_Parse(const char*)` is a producer); each producer scalar is capped
  (`producer_scalar_max` 32);
* ranking, deterministic: reachability from the translation unit → fuzz surface → evidence from
  the shipped driver → fewest parameters → name;
* the destructor is an **assumption**, not ownership inference: a function taking `T*`, returning
  void, reaching `free` (fn-pointer aliases followed); if the target *is* the destructor the object
  is consumed and neither freed nor canonicalised afterwards;
* determinism: if the producer reaches `rand`, `srand(42)` is issued on both sides before it;
* the producer's C call runs **inside the UB gate** (a producer that hits UB rejects the input);
  the two sides' producer results are compared for nullness;
* the target's inputs come from the same decoded scalars, never from reading the object;
* two produced parameters in one call (ownership transfer) are refused.

The bridge accepts raw-pointer, `&mut T` and `Option<&mut T>` targets. It is **frozen**: a new
shape is implemented only when it recurs across more than one translator with a usable producer
(PtrTrans's `Option<&mut cJSON>` with a `None` stub and no destructor is recorded as
*construction unsupported*, not bridged).

## 4. Return contract and the comparison ladder

The return value is classified into a **fixed, generator-owned ladder** (`return_contract`):
`void` → nothing; scalar → value (widened through `i128` when the translation's integer type
differs); pointer → **nullness, never the address** — unless a **comparator plugin** is registered
for the pointee type *and* is compatible with this translation (`[plugin.requires]`: the struct,
the fields its Rust half reads, the destructor it calls; an incompatible plugin **degrades** to
nullness instead of failing to build). Buffers the harness allocated are compared in full after
the call (float rows bit-for-bit, so NaN ≠ NaN is not a divergence); a produced object is
canonicalised through the plugin before and after the target. A reshaped return the ladder
cannot read (`&str` for `const char*` with no reference reading) fails construction.

The recorded **oracle strength** is one of `termination-only`, `partial(nullness)`,
`observable-state`, `structured-state`; never `full`.

## 5. Rust bridge and lowering

`apply_rust_bridges` aligns the translation's parameter types with the C parameters (positional;
a reshaped arity is a construction failure) and picks a **lossless** bridge per parameter:
`c_abi` (raw pointer / same scalar), `scalar_cast` (same category, different width, cast at the
call), slices / `Vec` / `Box<[T]>` / `Option<&[T]>` for buffers (the length folds into the slice),
`&mut T` / `Option<&mut T>` for one-element and produced objects, pointer-to-pointer of the same
element type for tables. Typedefs are resolved (`pub type lil_t = *mut _lil_t;`, `size_t = u64`)
at every point the translation's type is read. No bridge → the boundary fails, with the reason.

`lower_to_schema` turns the plan into the **generated IR** the generator consumes (roles,
element types, extent expressions over already-decoded scalars, fill policy, bridges). The IR is
never hand-written; `scripts/gen_harness_regression.py` freezes 29 cases (bzip2, genann, cJSON,
lil, tulip) and any change to what the generator emits is a deliberate re-freeze.

## 6. The generated harness

One cargo-fuzz crate per boundary: `src/lib.rs` = the flattened translation verbatim (the crate is
`translated`), `c/` = the C oracle TU with every function renamed `c_<name>` (plus siblings,
shims, the UB shim, the plugin's C half), `fuzz/fuzz_targets/<lib>_<tool>_ft.rs` = the target.

The target decodes the fuzz bytes **once** into the canonical input (`Cur`: scalars first, so a
scalar's byte offset does not move when a buffer grows; then buffers, strings, tables; plan
arrays last because their size may depend on decoded scalars) and materialises it **twice** —
independent allocations for the C side and the Rust side. It then runs in one of five modes
(`C2R_MODE`):

| mode | what runs | used by |
|---|---|---|
| `rust-only` | translation alone | the campaign (coverage is of the translation) |
| `c-only` | reference alone | confirmation phase A: a sanitizer report is unambiguously C's |
| `combined` | producer(s) → C call → gate → Rust call → ladder | corpus replay, confirmation |
| `nogate` | combined without the UB gate | diagnostics |
| `coverage` | rust-only under the coverage build | |

Every execution records a **phase** (`decode` 0, `C` 1, `C done` 2, `Rust` 3, `Rust done` 4,
`compared` 5, `producer` 6, `free` 7) and an **outcome** from a fixed vocabulary
(`normal | divergence | panic | signal | nonzero-exit | timeout | ub-gated`), written as
`C2R_OUTCOME kind=… phase=…` so a crash at phase 3 reads "C already returned normally". The
**in-loop UB gate** compiles the C side with UBSan's minimal runtime (signed overflow, shifts,
divide-by-zero, bounds, null, float-cast overflow, …), resets a flag before each C call and
rejects the input as `ub-gated` if any check fired — a cheap noise filter that decides nothing.

Bridged arguments are built at the call (`&buf[..]`, `Some(&mut *obj)`, `codelen as u64`,
`tab.as_ptr()`), returned pointers are freed through the plugin's declared destructor or leaked
when a produced object is in play (an interior pointer must not be freed), and the ladder runs
in order, stopping at the first difference (`c2r_div("table outputs row 1")`).

## 7. Build, fixups, preflight

`c2r_funnel.py` copies the sibling C files and subdirectories, links the shims, appends the
crate-root re-export for a `static` entry (with the translator's namespace prefix), links the
plugin halves, and `cargo fuzz build`s with the pinned toolchain. The harness's own artifacts are
pruned from the shared target after each build and the campaign binary is stripped of debug info
(the byte quota); each funnel row records the sha256 of the generator sources that built it.

**Preflight** (mandatory, PROTOCOL §3 amendment): every built harness runs once on the *empty
input* in `c-only` and in `rust-only` mode, and all run a 60 s fork-mode test with the
campaign's parameters. A harness whose C side crashes on the empty input, or whose test run
crashes on ~every job with a corpus that never grows, is **flagged**; unless it is listed in the
pair's `preflight_accept.txt` (a reviewed, reasoned "unconstructible precondition"), the cell
stops with exit 3 before the hour is spent. The preflight corpus is discarded.

## 8. Campaign, coverage, replay

**Campaign** (one per cell): every harness of the cell concurrently, libFuzzer fork mode,
`rust-only`, 3 600 s wall, `-seed=42`, `-timeout=25`, `-rss_limit_mb=2048`, `-max_len` 4 096
(65 536 for buffer-table libraries), one fixed seed per corpus plus the library's shipped samples
encoded into the input format; hard-linked corpus snapshots at 60/300/600/1 800 s make the budget
checkable; parameters are recorded in `campaign_params.json`; every harness process runs in the
cell's `sandbox/` directory. Crash artifacts are the **termination channel** of candidates
(manifest of every artifact's hash; first 500 per channel kept).

**Coverage** of the translation, per harness, over its saved corpus (`cargo fuzz coverage`, the
toolchain's own `llvm-cov`), then unioned over the cell. Identities are `(file, line)` for
functions and `(file, l1, c1, l2, c2)` for regions — never symbol names. The **universe** is the
shipped suite's instrumented build when the suite passes completely (the *acceptance baseline*),
otherwise the translation's own rlib objects exported with zero counts (`rlib_universe.py`) and
the cell is `TEST-UNAVAILABLE` / `TEST-FAILS` with the partition collapsing to Ours / Neither —
never a 0 % tests column. Four sets with sanity identities: both, only-tests, only-ours, neither.

**Combined replay** of the same corpus (C beside Rust, ladder on, gate on) tallies
`normal / divergence / signal / panic / timeout / ub-gated` per input; divergences are the
**divergence channel** of candidates. Boundaries whose campaign crashed on ~every job are marked
`crash_all` in the funnel, and their coverage export fails for a reason the funnel states.

## 9. Confirmation: four replays, one verdict

For each candidate (a 200-per-channel sample per boundary in the cell; full on public boundaries
or wherever anything confirms), `c2r_campaign.py confirm` rebuilds the harness twice — `_san`
(C with ASan **and full UBSan**, Rust with ASan) and `_nosan` — and replays the input in
`c-only`, `rust-only`, `combined`, and `rust-only with no sanitizer`. `classify` then applies, in
order:

1. C alone signals, exits non-zero or any sanitizer check fires → **`ub_associated`**
   (`_termination` if the translation panics on its own, `_value` if the ladder differed): C
   provides no clean reference execution on this input — a Rust failure on top of it may be the
   same illegal access, silent on one side and explicit on the other; **never a defect**.
2. C alone times out → `confirmed_termination` if Rust did not, else `inconclusive`.
3. Combined replay `ub-gated` → **`ub_gated`**: nothing was compared.
4. Combined replay reports a **divergence** → **`confirmed_divergence`** (with the rung and phase).
5. Combined replay **panics** at phase ≥ 2 (C had returned) → **`confirmed_termination`**;
   earlier → `inconclusive`.
6. Combined replay signals: if the no-sanitizer Rust replay **panics on its own** (a program
   trap: bounds, overflow, precondition check) → **`confirmed_termination`**; if it runs normally →
   **`instrument_only`**; if it takes a raw signal → **`out_of_contract_access`** (a wild read
   faults only if its page is unmapped — layout luck on both sides, unclaimable).
7. Combined replay times out → `confirmed_termination` only if the translation alone also times
   out (C alone returned); otherwise `inconclusive` (the hang is not attributable to one side).
8. Otherwise `not_reproducible` (the ladder agrees on replay).

"No check fired on C" is check coverage, not a proof of definedness; every `confirmed_*` verdict
reads that way. A Rust panic on top of dirty C is a UB-associated termination difference, not a
translation defect (`ub-associated-not-defect`).

## 10. From verdicts to defects

Confirmed verdicts are **clustered by site** (the panic location or the top non-interceptor
frames of the no-sanitizer replay; for divergences the rung and boundary). One site is **one
defect**, however many boundaries show it: lil × C2SaferRust's 57 confirmed terminations across
19 boundaries are one defect (C9) reached through the producer `lil_new()`, with 18 boundaries
*downstream blocked*. A defect is promoted only with the C and Rust source quoted at the lines
(`provenance="exact-source"`), the mechanism named, and the evidence path to the cell; it enters
`results/rq4_effectiveness/gen_defect_manifest.py` and the manifest is regenerated. Noise classes
that are recorded and never promoted: `ub_associated*`, `out_of_contract_access`,
`instrument_only`, crash-all internals whose precondition the harness cannot construct, returned
fresh objects that leak into OOM artifacts, the reference's own bugs.

## 11. Deviations, and what a cell archives

A cell's `RUN.md` is assembled by `run_md.py` from the cell's own files (funnel, plans, campaign
parameters, preflight, generator hashes, coverage partition with sanity checks, replay tally,
confirmation tally) and a hand-written §7 states every deviation: a boundary re-fuzzed alone after
a harness fix and merged (`merge_boundary.py`, `deviations.json`), a restarted cell, an unbuilt
boundary and why, a finished-after-death coverage phase. The archive keeps the corpus, the
artifact manifest, the first 500 artifacts per channel, the verdicts (stderr only on confirmed
rows), the confirmed inputs, the harness sources and coverage exports, and the sha256 of every
artifact it used.

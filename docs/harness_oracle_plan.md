# Harness redesign: C-shaped input, fixed comparison ladder, two-phase execution

**Status: PROPOSED, 2026-09-04.** On approval this supersedes most of
[`harness_plan_architecture.md`](harness_plan_architecture.md). Rules 1, 2, 5 and 7 of that document
stand. Rule 6 collapses (see below). Rule 8 weakens. The ObservationPlan is deleted, and there is
no static comparison stage.

## One sentence

**Inputs must be exact; outputs may be partial.** A boundary whose equivalent input cannot be
constructed is a harness-construction failure; a boundary whose output cannot be fully compared
still runs, with a weaker oracle.

## 1. Input: one C-shaped canonical input, materialized twice

C is the reference, so the fuzz bytes are decoded into an input **C** accepts, and the Rust
arguments are bridged from that same canonical input.

```
fuzz bytes
   -> C input decoder          (derived once per library, from the C AST)
   -> C-shaped canonical input
        |-- materialize -> C arguments
        `-- materialize -> Rust arguments   (bridge, per translator)
```

The InputPlan therefore has exactly two parts per parameter:

```json
{"param": "buf", "c_decoder": "buffer_with_length", "rust_bridge": "pointer_len_to_slice"}
```

Three things to keep straight:

* **The "C decoder" is a conceptual layer implemented in Rust.** The canonical input is materialized
  as Rust `Vec`s and scalars; C receives raw pointers into them. Nobody writes a C decoder.
* **It is one canonical input materialized twice, not "Rust's input generated first".** Ordering is
  not the requirement — *independent allocations* are. Sharing storage lets C's mutation corrupt
  Rust's input no matter when it was produced.
* **Sizing does not disappear.** `buffer_with_length` does not say how long. Extents and scalar
  domains still come from the global policy plus the entry's own rejection guards. What disappears
  is the *dual* derivation, not the sizing.

**Construction fails** when there is no C decoder for the C type (`EState*`, `BZFILE*`, `void*`) or
no lossless bridge for the Rust type. Bridges are a closed, enumerable set: identical type → copy;
`T* + len` → `&[T]` / `&mut [T]`; nullable `T*` → `Option<&mut T>`; out-pointer → `&mut T`. No
bridge, no harness — inputs are not allowed to be approximate.

A useful consequence: **the canonical input is translator-independent.** It is derived once per
library from the C side; only the bridge differs across the six translators. Every tool is therefore
fuzzed on the same input distribution, which is what makes the cross-tool comparison fair. This
belongs in the paper. (The claim is about the INPUT only — a comparator plugin, §5, is not
translator-independent: it reads the translated struct's fields by name.)

## 2. Output: a fixed comparison ladder

Fixed, ordered, generator-owned. Not configurable per boundary. Evaluated per input, in order,
stopping at the first difference.

| rung | compares |
|---|---|
| 1 | **termination** — `normal` / `panic` / `signal` / `nonzero-exit` / `timeout` |
| 2 | **scalar value** — return value and out-scalars |
| 3 | **pointer nullness** — NULL vs non-NULL, *never the address* |
| 4 | **known buffer contents** — buffers the InputPlan allocated, so the extent is free |
| 5 | **user comparator plugin** — canonical bytes for a registered type |

**C-definedness checking is part of the ADJUDICATION oracle** — and it belongs in confirmation,
not in the fuzz loop. A candidate may be confirmed as a translation defect only when **no UB check fires on the
C-only replay**. That is not the same as proving C is UB-free: a sanitizer raises check coverage,
it does not establish the absence of undefined behaviour. Every `confirmed_*` verdict therefore
reads "no check fired", not "C is defined". The two places are different jobs:

| where | instrument | job |
|---|---|---|
| discovery, in-loop | UBSan-minimal gate | cheap noise filter; skips obviously-UB inputs so the campaign spends its time elsewhere. Reports `ub-gated`. Decides nothing. |
| confirmation, C-only | ASan **+ full UBSan** (`--c-sanitize`) | the adjudicator. C dirty ⇒ `ub_associated*`, never a defect. |

The demonstrated case: `cJSON.c:112` does `item->valueint = (int)n` on a `double`. An out-of-range
double-to-int cast is UB in C, x86-64 `cvttsd2si` yields `INT_MIN`, and Rust's `as i32` saturates to
`i32::MAX`, so the two sides differ *by construction*. The comparator plugin observed the difference
correctly; the difference simply is not attributable to the translation.

**ASan alone cannot adjudicate this.** An out-of-range cast is not a memory error, so a C-only ASan
replay reports C **clean** and the candidate is misadjudicated as a confirmed defect — which is what
happened before the confirmation build gained UBSan. With `-fsanitize=address,undefined` the same
input yields

```
c/cJSON.c:112:17: runtime error: 5.55556e+23 is outside the range of representable values of type 'int'
```

and the verdict is `ub_associated_value`. Memory safety and value-level definedness are different
obligations needing different instruments; the confirmation build carries both.

A flag added to the in-loop gate also needs its `__ubsan_handle_*_minimal` handler in the shim or
the harness will not link — but that is a build detail of the *filter*, not of the adjudicator.

There is **no static comparison stage**. Type category, width and signedness are internal validation
inside the RustBridge (section 1): if the C-shaped input can be bridged losslessly, the harness
runs; if the bridge cannot preserve the same logical input, construction fails. That is a generator
decision, not a comparison rung and not a reported result.

**Oracle strength** is recorded per harness and reported. None of the labels is `full`: no harness
compares full program semantics, and saying so would over-claim.

| label | what was compared |
|---|---|
| `termination-only` | only how the two runs ended |
| `partial(nullness)` | a returned pointer, as NULL vs non-NULL — never the address |
| `observable-state` | the return scalars plus every buffer the harness itself owns |
| `structured-state` | observable-state, plus the object state a comparator **plugin declares** |

A weaker oracle is a weaker instrument, not a failure.

## 3. Execution: two phases

```
DISCOVERY                                  CONFIRMATION (per candidate)
C + Rust, no sanitizer, fast               A. C ONLY, isolated ASan+UBSan
   -> ladder                                  C dirty -> UB-associated discrepancy (not a finding)
   -> divergence: marker + abort               C clean -> B
   -> libFuzzer fork mode keeps going       B. replay the discovery binary on that input
                                               still differs -> CONFIRMED behavioral divergence
                                               agrees      -> not reproducible
```

* **The ladder is identical in both loops.** They differ only in whether C runs under sanitizers and
  whether we are searching or adjudicating.
* **Attribution needs C alone.** Sanitizing both sides at once makes a report unattributable.
  Phase A is `C2R_MODE=c-only`, the mirror of `rust-only`, on a build with ASan **and full UBSan**.
* **Discovery must run in libFuzzer fork mode with `-ignore_crashes=1`** (this build supports that
  option only in fork mode). Without it one harness-model SEGV ends the campaign: `mainSort`
  currently dies on input #1 and would contribute nothing.
* **Panics.** libfuzzer-sys 0.4.13 installs a panic hook and aborts before unwinding, so
  `catch_unwind` is not available. Discovery therefore records a marker from a panic hook and lets
  the process abort; the classification happens in the replay, not in the loop.
* Confirmation times C and Rust separately, so a `timeout` can be attributed to a side.

## 4. Rule 6 collapses — delete the static safety proof

The old rule 6 branched: *satisfy every derivable memory-safety precondition, or run isolated
ASan+UBSan, or fail construction*. The two-phase design **always** takes the ASan branch, so the
branch is gone:

> Confirmation always replays C under isolated ASan+UBSan. Static analysis no longer carries a proof
> obligation.

The index-bound analysis is demoted from a proof tool to a **sizing heuristic**. It stays, because
deriving `code = alphaSize` and `base = max(maxLen+2, 257)` is what makes the fuzzing deep. But
failing to derive `perm`'s extent stops being a problem: allocate the policy size, let the sanitizer
be the net. The three known soundness bugs become throughput work, not correctness work.

What this deletes outright:

| deleted | why |
|---|---|
| `analyze_observations`, `ObservationPlan`, `ObservationSpec` | replaced by the fixed ladder |
| `safety.unproven_obligations`, `safety.c_execution`, `safety.pointer_validity` | confirmation is the safety mechanism |
| the five-way `extent_source` taxonomy | collapses to "how many elements to allocate" |
| `may_index_negative`, `index_lowers`, `_lower_of`, `_nonneg` | existed only to discharge obligations |
| `ownership` claims | not deriving ownership means we do not free; a leak in a fuzz process is not a defect |
| `return_contract()` as a gate | becomes rung selection: a pointer return without a plugin gets rung 3 only |

What survives, all of it for throughput rather than proof: the global policy, rejection-guard
extraction, symbolic index bounds and `b_max`, pointer/length pairing, taint clamping, the liveness
clamp.

## 5. Comparator plugin

Extends **output comparison only**. It never touches the InputPlan and is not a hand-written
harness. One canonicalizer per side, object → bytes:

```c
size_t c2r_canon_cJSON(const void *obj, unsigned char *out, size_t cap);
```
```rust
pub fn c2r_canon_cJSON(obj: *const c_void) -> Vec<u8>;
```

Registered in a user-supplied manifest (`plugins/<library>.toml`: C type, C source, Rust module).
The generator emits the two calls and compares byte strings; it never inspects the object and never
calls the library's own printer. `tools/stu_selector/contract_templates.py` already generates this
pair for cJSON, so the reference implementation is mostly written — the work is re-framing it as a
user artifact behind a stable ABI. The plugin is untrusted code linked into the harness; say so in
the paper.

## 6. Matcher positioning (paper text, no code)

> The matcher establishes that the C and Rust executions begin at corresponding candidate
> boundaries.

It verifies boundary correspondence. It does not on its own establish semantic equivalence.

## 7. What this does not fix

Simplifying output comparison makes boundaries that *return* complex pointers runnable. It does
nothing for boundaries that *take* complex inputs. For bzip2 that is the whole story: of 50
construction failures only **3** are return-contract failures; 31 are struct-invariant inputs
(`EState*`, `DState*`, `bz_stream*`), 11 opaque `BZFILE*`, 2 `void*`, 2 no-parameter. The recount
will move on cJSON, where most functions return `cJSON*`, not on bzip2.

## 8. Implementation order

| phase | work |
|---|---|
| 0 | ✅ **Delete.** ObservationPlan, the whole `safety` block, the obligation machinery, ownership claims, `return_contract` as a gate. Net removal; the 14 bzip2 plans must still lower and build afterwards. |
| 1 | ✅ **Restructure InputPlan** into `c_decoder` + `rust_bridge`; enumerate the bridge set; the bridge internally validates type category / width / signedness; construction fails only on a missing decoder or a bridge that cannot preserve the logical input. |
| 2 | ✅ **Comparison ladder + partial oracle**: fixed emitter for rungs 1–4, `oracle_strength` per harness, panic hook marker, addresses never compared. |
| 3 | **Two-phase execution**: `C2R_MODE=c-only`, an ASan+UBSan confirmation build, a driver classifying confirmed / UB-associated / not-reproducible, discovery in fork mode. ✅ `scripts/c2r_campaign.py` (ASan only; full UBSan on the C side is still open) |
| 4 | **Plugin interface** + cJSON reference implementation. ✅ `plugins/cjson/`, `--plugins` |
| 5 | **Recount** on bzip2 and cJSON: matched → constructed → built → executed → oracle strength → candidates → confirmed. |
| 6 | *deferred* — sizing-heuristic improvements (the three ex-soundness bugs), now pure throughput. |
| 7 | *deferred* — retire `--schema`; migrate the 17-entry golden regression to `--plan`. |

## 9. Open question

**Which Rust parameter shapes have a lossless bridge.** The set has to be enumerated against the
real signatures from all six translators before it is frozen. A shape with no bridge is a
construction failure, so the set decides how many harnesses exist — it is worth one pass over the
actual translated signatures rather than a guess.

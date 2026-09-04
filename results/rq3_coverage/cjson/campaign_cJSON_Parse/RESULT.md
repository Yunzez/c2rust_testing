# Two-phase campaign: cJSON × c2rust, boundary `cJSON_Parse`

Method: [`docs/harness_oracle_plan.md`](../../../../docs/harness_oracle_plan.md) §3.
Driver: `scripts/c2r_campaign.py`. Harness: generated from a HarnessPlan with
`gen_diff_harness.py --plan` — no schema was read or written.

## Headline

**0 translation defects. 845 candidate inputs, 7 clusters, all attributed to C.**

| stage | result |
|---|---|
| discovery, **no** fork mode | **1** artifact — the first crash ends the campaign |
| discovery, fork mode, 60 s | **845** candidates |
| confirmation (C alone, ASan) | 845 / 845 `ub_associated` |
| clustering | 7 clusters, every one in `c_parse_string` |

Each of the three stages has a positive and a negative check behind it: fork mode is what turns
1 artifact into 845; confirmation is what stops those 845 from being reported as translation
defects; clustering is what stops "845 candidates" from being read as 845 problems.

## Clusters

| artifacts | verdict | frame | example |
|---:|---|---|---|
| 537 | `ub_associated` | `#0 c_parse_string cJSON.c:205` | `raw/crash-00a965fa11946f98d9863d…` |
| 199 | `ub_associated` | `#0 c_parse_string cJSON.c:197` | `raw/crash-02a471dc40efefb3f175de…` |
| 63 | `ub_associated` | `#0 c_parse_string cJSON.c:203` | `raw/crash-03a228c644fa6b6c819968…` |
| 35 | `ub_associated` | `#0 c_parse_string cJSON.c:244` | `raw/crash-119739c4e20d4bfd395e24…` |
| 5 | `ub_associated` | `#0 c_parse_string cJSON.c:233` | `raw/crash-3017f7c2f0221c70317554…` |
| 3 | `ub_associated` | `#0 c_parse_string cJSON.c:239` | `raw/crash-52f1f4c5dbde499644d65e…` |
| 3 | `ub_associated` | `#0 c_parse_string cJSON.c:213` | `raw/crash-79c991c95c2d8e32785330…` |

All 7 are out-of-bounds reads inside cJSON's `\uXXXX` escape handling, in one function.
**7 clusters is not 7 defects** — plausibly it is one root cause with several
manifestation points, and the campaign does not establish which. Nothing here may be added to the
translation-defect count.

## Why these are not translation defects

The confirmation replay runs **C alone**, with the C oracle compiled `-fsanitize=address`
(`gen_diff_harness.py --c-sanitize`, `C2R_MODE=c-only`). On every one of these inputs C is itself
already out of bounds:

```
ERROR: AddressSanitizer: heap-buffer-overflow  READ of size 1
  #0 c_parse_string        c/cJSON.c:197
  #1 c_cJSON_ParseWithOpts c/cJSON.c:330
  #2 c_cJSON_Parse         c/cJSON.c:339
```

The rule this establishes, recorded so it is applied consistently:

> A C side that returns "normally" in the differential loop may only *appear* normal. When
> C-alone-under-ASan shows it had already gone out of bounds, a panic or abort on the Rust side is
> an **UB-associated termination difference**, not a translation defect: the same illegal access is
> silent UB in C and an explicit failure in Rust. It is a difference in how one illegal access
> surfaces, not in what the translation computes.

The driver encodes this as a distinct verdict, `ub_associated_termination`, separate from a plain
`ub_associated`. (In this campaign every candidate is plain `ub_associated`: the translation's
failure was also a memory error, not a panic.)

## Attribution requires a separate build and a C-only mode

Verified rather than assumed: the ordinary discovery build leaves the C oracle **completely
uninstrumented for ASan** — `libc_oracle.a` carries 0 `__asan_` references, only 4 UBSan-minimal
handlers. A C-side heap overflow is therefore invisible to it, and ASan reports only the Rust side.
The confirmation build adds `-fsanitize=address` to the C compilation; the Rust side already links
the ASan runtime, so both share one shadow map. `clang` rejects `-fsanitize-minimal-runtime`
together with `-fsanitize=address`, so the confirmation build drops the in-loop UB gate — that gate
is a discovery device, and confirmation exists to attribute a *memory* error to a side.

Full UBSan on the C side is still open: `-fsanitize=undefined` needs its own runtime alongside the
one Rust links, and that combination has not been tried. Only the ASan memory-error path is
verified here.

## Files

- `clusters.json` — the 7 clusters (verdict + sanitizer + frame + artifact count).
- `verdicts_slim.json` — one row per candidate: verdict, and the outcome of all three replays.
- `cluster_examples.json` — the full record, with stderr, for one representative per cluster.
- `raw/` — the 7 representative inputs, and the tail of the discovery log.

## Reproduce

```
python3 tools/stu_selector/gen_diff_harness.py --pair <cjson pair> --entry cJSON_Parse \
        --plan --ub-free --out <discovery>
python3 tools/stu_selector/gen_diff_harness.py --pair <cjson pair> --entry cJSON_Parse \
        --plan --ub-free --c-sanitize --out <confirmation>
# build both with cargo fuzz, then
python3 scripts/c2r_campaign.py discover --bin <discovery bin> --corpus <dir> --out <camp> --seconds 60
python3 scripts/c2r_campaign.py confirm  --bin <discovery bin> --asan-bin <confirmation bin> \
        --candidates <camp>/candidates --out <camp>/final
```


## Provenance — the pair is reconstructible from this repository

The pair used here lived in a scratchpad. Both halves are byte-identical to files tracked in the
repo, so nothing is lost when the scratchpad is:

| half | repo path | md5 |
|---|---|---|
| C source | `tools/c2rust_crustbench/out/cJSON/src/cJSON.c` | `8149dc95b7be93376decb283d26743c5` |
| c2rust translation | `fuzz/cjson_c2rust_e3/src/lib.rs` | `d06e8a8b73f2e311a3071dc62422661b` |

To rebuild the pair directory the generator expects:

```
mkdir -p <pair>/source <pair>/translated <pair>/build
cp tools/c2rust_crustbench/out/cJSON/src/cJSON.c tools/c2rust_crustbench/out/cJSON/src/cJSON.h <pair>/source/
cp fuzz/cjson_c2rust_e3/src/lib.rs <pair>/translated/cjson_c2rust.rs
# build/compile_commands.json: one entry compiling source/cJSON.c with -I <pair>/source
```

---

# Round 2: the FULL oracle (comparator plugin)

Round 1 above ran a **partial** oracle: the return is a `cJSON*`, and without a comparator the
ladder reaches only rung 3, nullness. Round 2 registers the cJSON comparator plugin
(`plugins/cjson/plugin.toml`, `--plugins`), so the ladder reaches rung 5 and the whole parsed object
is compared as canonical bytes. `oracle_strength` goes from `partial(nullness)` to
**`structured-state`** — the object state the plugin *declares*, not full program semantics. No
harness in this work reports `full`.

| oracle | UB gate | candidates | clusters | confirmed defects |
|---|---|---:|---:|---:|
| partial (nullness) | without `float-cast-overflow` | 845 | 7 | 0 |
| **full (plugin)** | without `float-cast-overflow` | — | — | **1, and it was FALSE** |
| **full (plugin)** | with `float-cast-overflow` | 1717 | 10 | **0** |

The middle row is the important one.

## The false positive, and what it proves

The first full-oracle run reported `divergence: canonical object comparison` at phase 4 — both
sides returned normally and their canonical forms differed — and the driver classified it
`confirmed_divergence`. c2rust is a faithful translator, so that was checked rather than believed.

The input is the JSON number `555555555555555555555555` (≈5.56e23). `cJSON.c:112` is

```c
item->valueint = (int)n;      /* n is a double */
```

An out-of-range double-to-int cast is **undefined behavior in C**. On x86-64 `cvttsd2si` yields
`INT_MIN`; Rust's `as i32` **saturates** to `i32::MAX`. The two sides disagree *by construction*.

`UB_SANITIZE_FLAGS` did not carry `-fsanitize=float-cast-overflow`, so the in-loop gate passed the
input. Both directions were then checked on the same input:

| | result |
|---|---|
| gate **with** `float-cast-overflow` | `exit=0` — rejected as UB, nothing compared |
| `C2R_MODE=nogate` | `divergence: canonical object comparison` still fires |

The input is archived as `full_oracle/raw/float_cast_overflow_false_positive.input`.

**What this actually exposed.** The first reading — "the in-loop gate was missing a flag" — put the
conclusion in the wrong place. The gate is a discovery-side *noise filter*; it decides nothing. The
real gap was that **confirmation carried only ASan**, and adjudication is confirmation's job:

> **C-definedness checking is part of the adjudication oracle.** Only a candidate on which C alone
> is well-defined may be confirmed as a translation defect.

| where | instrument | job |
|---|---|---|
| discovery, in-loop | UBSan-minimal gate | cheap filter, reports `ub-gated`, decides nothing |
| confirmation, C-only | ASan **+ full UBSan** (`--c-sanitize`) | the adjudicator |

**ASan alone cannot adjudicate this.** An out-of-range cast is not a memory error, so an ASan-only
C-only replay reports C **clean**. With `-fsanitize=address,undefined` the same input yields

```
c/cJSON.c:112:17: runtime error: 5.55556e+23 is outside the range of representable values of type 'int'
```

and the driver returns **`ub_associated_value`** — verified on that exact input while running the
discovery binary whose gate is *blind* to it, which is the proof that the fix belongs in
confirmation rather than in the loop. Both were done anyway: the gate now carries
`float-cast-overflow, pointer-overflow, return, vla-bound` as well (each needs its
`__ubsan_handle_*_minimal` handler in the shim, or the harness does not link), so these inputs are
filtered early *and* adjudicated correctly if they get through.

**Method note.** A partial (nullness-only) oracle never saw this difference; strengthening the
oracle to structured-state exposed it immediately. When the comparison is strengthened, the UB
checks enabled on the C side should be revisited in the same step, or the stronger comparison
reports differences the adjudicator cannot yet attribute. This is a practical note about ordering
the work — not a claim of completeness on either side: a sanitizer raises check coverage, it never
establishes that an execution is free of undefined behaviour.

Gate flags now: `signed-integer-overflow, shift, integer-divide-by-zero, bounds, null, unreachable,
float-cast-overflow, pointer-overflow, return, vla-bound`. A gated-out input now reports
`C2R_OUTCOME kind=ub-gated` and the driver has a matching `ub_gated` verdict, so "rejected by the
gate" is no longer indistinguishable from "died before reporting".

## Round 2 result

**1717 candidates, 10 clusters, 1717/1717 `ub_associated`, 0 confirmed translation defects.**

| artifacts | frame |
|---:|---|
| 1053 | `#0 c_parse_string cJSON.c:205` |
| 258 | `#0 c_parse_string cJSON.c:203` |
| 189 | `#0 c_parse_string cJSON.c:197` |
| 146 | `#0 c_parse_string cJSON.c:244` |
| 39 | `#0 c_parse_string cJSON.c:233` |
| 15 | `#0 c_parse_string cJSON.c:239` |
| 9 | `#0 c_parse_string cJSON.c:234` |
| 5 | `#0 c_parse_string cJSON.c:235` |
| 2 | `#0 c_parse_string cJSON.c:214` |
| 1 | `#0 c_parse_string cJSON.c:213` |

Every cluster is again inside `c_parse_string`'s `\uXXXX` handling — C's own out-of-bounds reads.
As in round 1, 10 clusters is not 10 defects.

**This is the negative control the method needs:** with a full oracle over the entire parsed object
and a correct UB gate, a faithful translator produces **zero** confirmed divergences across 1717
crashing inputs. A method that reported findings here would be reporting noise.

## Files

- `full_oracle/clusters.json`, `full_oracle/verdicts_slim.json`
- `full_oracle/raw/` — one representative input per cluster, plus the false-positive input
- Plugin: `plugins/cjson/{plugin.toml,canon.c,canon.rs}`. Its Rust half names no crate (the
  translation is in scope as `translated`) but reads the translated struct's fields **by name**, so
  it is reusable only across layout-compatible translations — a translator that reshapes `cJSON`
  into an idiomatic enum needs its own comparator.

## Reproduce round 2

```
python3 tools/stu_selector/gen_diff_harness.py --pair <cjson pair> --entry cJSON_Parse \\
        --plan --ub-free --plugins plugins/cjson/plugin.toml --out <discovery>
python3 tools/stu_selector/gen_diff_harness.py --pair <cjson pair> --entry cJSON_Parse \\
        --plan --ub-free --c-sanitize --plugins plugins/cjson/plugin.toml --out <confirmation>
python3 scripts/c2r_campaign.py discover --bin <discovery bin> --corpus <dir> --out <camp> --seconds 120
python3 scripts/c2r_campaign.py confirm  --bin <discovery bin> --asan-bin <confirmation bin> \\
        --candidates <camp>/candidates --out <camp>/final
```

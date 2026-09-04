# Harness funnel — how many harnesses the pipeline actually produces

Generator: `scripts/c2r_funnel.py`. Method: [`docs/harness_oracle_plan.md`](../../../docs/harness_oracle_plan.md).
Both pairs are × c2rust. 20 s of libFuzzer fork-mode discovery per boundary.

**Why this table exists.** Every count quoted before it stopped at `planned`. A boundary whose input
model puts nearly every input out of bounds still plans, still builds, and then produces one useful
execution. `planned` and `working` are different numbers and the gap has to be visible.

## The funnel

| library | boundaries | planned | built | executed | clean | degraded | no useful executions |
|---|---:|---:|---:|---:|---:|---:|---:|
| cjson | 58 | 21 | 21 | 21 | 18 | 3 | 0 |
| bzip2 | 64 | 19 | 19 | 19 | 14 | 3 | 2 |

* **clean** — under 1 % of executions end in an artifact.
* **degraded** — 1–99 % do. The harness runs, but a large share of its budget goes into inputs that
  end the execution. Attribution needs confirmation: an artifact is a C-side UB hit as often as a
  harness-model defect, and the funnel cannot tell them apart.
* **no useful executions** — the boundary produced at most one execution that reached the end.

## Oracle strength

| library | termination-only | partial(nullness) | observable-state | structured-state |
|---|---:|---:|---:|---:|
| cjson | 0 | 3 | 4 | 14 |
| bzip2 | 0 | 2 | 17 | 0 |

cJSON reaches `structured-state` on 14 boundaries because a comparator plugin is registered for
`cJSON *` (`plugins/cjson/plugin.toml`); without it those 14 would be `partial(nullness)`. bzip2 has
no plugin and needs none: its boundaries return status codes and write into caller-provided buffers,
so `observable-state` already compares everything they expose.

## Per boundary

### cjson

| boundary | oracle | executions | artifacts | band |
|---|---|---:|---:|---|
| `cJSON_GetErrorPtr` | partial(nullness) | 5,175,025 | 0 | clean |
| `pow2gt` | observable-state | 4,945,820 | 0 | clean |
| `skip` | partial(nullness) | 2,757,862 | 0 | clean |
| `cJSON_strdup` | partial(nullness) | 2,147,459 | 0 | clean |
| `cJSON_CreateNumber` | structured-state | 255,312 | 0 | clean |
| `cJSON_CreateArray` | structured-state | 194,285 | 0 | clean |
| `cJSON_CreateTrue` | structured-state | 193,185 | 0 | clean |
| `cJSON_CreateNull` | structured-state | 192,795 | 0 | clean |
| `cJSON_CreateFalse` | structured-state | 192,550 | 0 | clean |
| `cJSON_New_Item` | structured-state | 191,545 | 0 | clean |
| `cJSON_CreateBool` | structured-state | 189,182 | 0 | clean |
| `cJSON_CreateObject` | structured-state | 189,135 | 0 | clean |
| `cJSON_CreateDoubleArray` | structured-state | 183,804 | 0 | clean |
| `cJSON_CreateFloatArray` | structured-state | 174,011 | 0 | clean |
| `cJSON_CreateString` | structured-state | 125,870 | 0 | clean |
| `cJSON_Parse` | structured-state | 99,087 | 239 | clean |
| `cJSON_CreateIntArray` | structured-state | 98,882 | 0 | clean |
| `cJSON_CreateStringArray` | structured-state | 92,337 | 0 | clean |
| `parse_hex4` | observable-state | 25,920 | 878 | degraded |
| `cJSON_strcasecmp` | observable-state | 23,145 | 874 | degraded |
| `cJSON_Minify` | observable-state | 6,599 | 888 | degraded |

### bzip2

| boundary | oracle | executions | artifacts | band |
|---|---|---:|---:|---|
| `mmed3` | observable-state | 5,186,692 | 0 | clean |
| `BZ2_bzlibVersion` | partial(nullness) | 5,097,505 | 0 | clean |
| `default_bzfree` | observable-state | 5,059,245 | 0 | clean |
| `bz_config_ok` | observable-state | 5,050,960 | 0 | clean |
| `BZ2_indexIntoF` | observable-state | 162,406 | 0 | clean |
| `BZ2_hbAssignCodes` | observable-state | 161,052 | 0 | clean |
| `default_bzalloc` | partial(nullness) | 92,179 | 875 | clean |
| `BZ2_hbCreateDecodeTables` | observable-state | 88,497 | 0 | clean |
| `fallbackQSort3` | observable-state | 55,997 | 76 | clean |
| `fallbackSimpleSort` | observable-state | 53,492 | 120 | clean |
| `mainGtU` | observable-state | 40,513 | 561 | degraded |
| `BZ2_bzBuffToBuffDecompress` | observable-state | 37,536 | 0 | clean |
| `BZ2_bzBuffToBuffCompress` | observable-state | 19,930 | 0 | clean |
| `fallbackSort` | observable-state | 13,808 | 0 | clean |
| `mainSimpleSort` | observable-state | 12,408 | 640 | degraded |
| `mainQSort3` | observable-state | 10,328 | 453 | degraded |
| `BZ2_bz__AssertH__fail` | observable-state | 991 | 990 | no useful executions |
| `mainSort` | observable-state | 928 | 927 | no useful executions |
| `BZ2_hbMakeCodeLengths` | observable-state | 724 | 0 | clean |

## What the bands actually say

`mainSort` produced **928 executions and 927 artifacts**, and `BZ2_bz__AssertH__fail` **991 and
990** — the first because its input model cannot satisfy `ftab[65536]` from a 4096-element policy
allocation, the second because the boundary calls `exit(3)` on every input. Counting either as "a
harness" alongside `BZ2_hbAssignCodes` (161 052 executions, 0 artifacts) would be wrong, and before
this table there was no column in which that was visible.

Note also the other end. `mmed3`, `bz_config_ok`, `BZ2_bzlibVersion`, `cJSON_GetErrorPtr` and
`pow2gt` each ran ~5 M times. They are deterministic on a trivial or empty input, so a
coverage-guided loop spends millions of executions re-deciding the same thing. A zero-input boundary
needs **one** execution and a comparison, not a fuzzing campaign; that is a scheduling gap, not a
correctness one.

## Packaging, not capability

The first cJSON run had 6 of 21 boundaries fail to build, all with
`E0603: function ... is private`: they are C `static`, so the translation makes them private, and
`--expose-entry` had not been applied because that pair ships no `*.rs.defs.json`. The funnel now
detects a non-`pub` entry directly from the translated `.rs`, and all 21 build. The failure was
missing packaging in the driver, not a missing capability — worth recording because it is exactly
the kind of thing that silently deflates a capability number.

## Files

- `funnel.json` — every boundary, both libraries: plan status, plan failure, generated, built,
  executions, ooms/timeouts/crashes, oracle strength.
- Reproduce: `scripts/c2r_funnel.py --pair <pair> --out <dir> [--plugins ...] [--c-source ...]
  [--shim ...] [--defs ...]`


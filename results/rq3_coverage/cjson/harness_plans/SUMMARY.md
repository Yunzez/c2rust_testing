# cJSON — generated Harness Plans

**20 of 58 boundaries plan.** Design: [`docs/harness_oracle_plan.md`](../../../../docs/harness_oracle_plan.md).
Pair: cJSON x c2rust. Generator: `tools/stu_selector/harness_plan.py` (no schema is read or written).

Before the oracle redesign this library planned **7**: every boundary that returns `cJSON*` was
rejected for having no comparable return value. Under the fixed ladder a pointer return degrades to
rung 3 (nullness) instead, so `cJSON_Parse` and the whole `cJSON_Create*` family now plan.

| boundary | inputs | oracle |
|---|---|---|
| `cJSON_CreateArray` | *(none — deterministic)* | pointer nullness (partial) |
| `cJSON_CreateBool` | `b`:scalar | pointer nullness (partial) |
| `cJSON_CreateDoubleArray` | `numbers`:input_buffer, `count`:length | pointer nullness (partial) |
| `cJSON_CreateFalse` | *(none — deterministic)* | pointer nullness (partial) |
| `cJSON_CreateFloatArray` | `numbers`:input_buffer, `count`:length | pointer nullness (partial) |
| `cJSON_CreateIntArray` | `numbers`:input_buffer, `count`:length | pointer nullness (partial) |
| `cJSON_CreateNull` | *(none — deterministic)* | pointer nullness (partial) |
| `cJSON_CreateNumber` | `num`:scalar | pointer nullness (partial) |
| `cJSON_CreateObject` | *(none — deterministic)* | pointer nullness (partial) |
| `cJSON_CreateString` | `string`:input_string | pointer nullness (partial) |
| `cJSON_CreateTrue` | *(none — deterministic)* | pointer nullness (partial) |
| `cJSON_GetErrorPtr` | *(none — deterministic)* | pointer nullness (partial) |
| `cJSON_Minify` | `json`:input_array | pointer nullness (partial) |
| `cJSON_New_Item` | *(none — deterministic)* | pointer nullness (partial) |
| `cJSON_Parse` | `value`:input_string | pointer nullness (partial) |
| `cJSON_strcasecmp` | `s1`:input_array, `s2`:input_array | pointer nullness (partial) |
| `cJSON_strdup` | `str`:input_string | pointer nullness (partial) |
| `parse_hex4` | `str`:input_array | pointer nullness (partial) |
| `pow2gt` | `x`:scalar | pointer nullness (partial) |
| `skip` | `in_`:input_string | pointer nullness (partial) |

| n | why harness construction failed |
|---:|---|
| 15 | struct-invariant param item: cJSON has pointer field 'next' (needs inv |
| 8 | struct-invariant param array: cJSON has pointer field 'next' (needs in |
| 7 | struct-invariant param object: cJSON has pointer field 'next' (needs i |
| 3 | struct-invariant param p: printbuffer has pointer field 'buffer' (need |
| 2 | T** (string / pointer table) has no input adapter |
| 1 | struct-invariant param c: cJSON has pointer field 'next' (needs invari |
| 1 | struct-invariant param hooks: cJSON_Hooks has pointer field 'malloc_fn |
| 1 | struct-invariant param prev: cJSON has pointer field 'next' (needs inv |

Every remaining failure is an INPUT-construction failure, and 34 of the 36 are the same one:
a `cJSON*` parameter. Constructing (or sequencing) that object is the single capability that
would move this library.

## End-to-end check: `cJSON_Parse`

Generated from the plan with `--plan --ub-free`, built, and fuzzed for 45 s:
**33 672 executions, zero nullness divergences, one crash.**

The crash is an ASan heap-buffer-overflow inside `cjson_c2rust::parse_string`
(`src/lib.rs:446`), reached from the harness's *Rust* call. **It is not reported as a finding.**
cJSON's truncated-`\uXXXX` out-of-bounds read is a known defect of the C original, and c2rust is a
faithful translation, so the expected verdict is *UB-associated discrepancy*. Establishing that
requires replaying C alone under ASan — `C2R_MODE=c-only`, phase 3 — because this build instruments
only the Rust side (the C oracle carries UBSan-minimal, not ASan), so ASan can currently see the
overflow on one side only. This is the concrete case that motivates the confirmation loop.

# SACTOR × optipng — Gate 0 record (2026-09-02)

**Cell status: PRE-OUTPUT FAILURE (shipped configuration), $0 spent.** SACTOR's C resolver rejects
the project before any LLM call. This record supersedes the mechanism named in the July note
(`results/rq4_effectiveness/bugs/optipng_sactor/README.md`, "ZALLOC at deflate.c:277"): after all
indirect-call spellings are neutralised on the input side, the next — and final — wall is
**variadic functions** (`va_start` → compiler builtin `__builtin_va_start`, which SACTOR treats as
an undefined project function). No input-side rewrite can remove a variadic function without
changing the program, and Rust cannot define variadic functions, so the cell stays N/A in the
primary table and no Rust exists for a PARTIAL line.

## Protocol
User-accepted Gate 0/1 protocol: Gate 0 = $0 dummy-key dry runs (parse + dependency analysis
run before the 401), only semantically neutral *input-side* rewrites allowed (quadtree
precedent), no translator patch, no cost breaker raised, no LLM-only configuration. Gate 1
(paid, ≈$1–2 wall-clock-capped run) is launched only if Gate 0 reaches the LLM stage. For
optipng Gate 0 did not pass, so Gate 1 was never launched.

## Tool configuration
- SACTOR checkout `577c3d2` + the three July patches (`../_tool_patches/sactor_577c3d2.patch`).
- `sactor.toml`: gpt-5.1 via LiteLLM (never reached). Command (cwd = harness dir):
  `sactor translate --type bin -C ./compile_commands.json --test-command-path ./test_task.json
  -r ./result_dry --continue-run-when-incomplete -c .../sactor/sactor.toml`, with
  `OPENAI_API_KEY=sk-dummy-gate0`.

## Source version / harness (`input/`)
- C = the Group A optipng scope: OptiPNG 0.7.7 `src/` (`tools/frameworks/optipng-0.7.7/src`),
  52 `.c` + 31 `.h` **flattened into one directory** (SACTOR discovers TUs from the project
  directory and ignores per-entry `-I`; no header name collides). `optipng.c` holds `main`.
- Reference binary `gcc -I. -o optipng_ref *.c -lm` prints "OptiPNG version 0.7.7".
- Test inputs generated deterministically (`in_rgb.png` 16×16 RGB, `in_grey.png` 8×8 grey,
  `in_p6.pnm` 6×4 PPM); `test_samples.json` = 4 cases (`-o1 -clobber -out outN.png <in>` ×3,
  `-simulate -o2 in_rgb.png`), expected output = stdout+stderr (SACTOR's runner compares
  `normalize_string(stdout + stderr)`), re-run twice and byte-identical. `test_task.json` = 4 ×
  `sactor run-tests --type bin ./test_samples.json %t <i> --feed-as-args`.
- `compile_commands.json`: 52 entries `gcc -I<dir> -c -o X.o X.c`.

### Neutral input-side rewrites (12 files, all diffs ≤ 8 lines; original = optipng-0.7.7/src)
SACTOR's resolver (`c_parser.py:783-823`) raises on any `CALL_EXPR` whose libclang
`referenced` is `None` outside a system header. libclang does not look through
`ParenExpr`/`UnaryOperator`/`ArraySubscriptExpr` callees, so these spellings are fatal while
`x->f(...)` (a `FIELD_DECL` reference) is skipped. Every rewrite below changes only the
spelling of an indirect call or a feature-test macro; the reference binary and all 4 samples
were re-verified after the edits.

| file | rewrite |
|---|---|
| `deflate.c` (1), `pngerror.c` (3), `pngpread.c` (3), `pngread.c` (1), `pngrio.c` (1), `pngrutil.c` (1), `pngwio.c` (2), `pngwrite.c` (1) | `(*(x->f))(args)` → `x->f(args)` (regex over the exact `(*(…->…))(` form; 13 sites) |
| `zutil.h` | `ZALLOC`/`ZFREE` macros: `(*((strm)->zalloc))(...)` → `(strm)->zalloc(...)`, same for `zfree` (the July "ZALLOC at deflate.c:277" wall) |
| `bitset.c:36` | `opng__SPAN__` macro `while ((predicate)(*(ptr)))` → `while (predicate(*(ptr)))` (parenthesised callee `(isspace)(…)`) |
| `pngrutil.c:4123` | `pp->read_filter[filter-1](row_info, row, prev_row)` → assign the array element to a local function-pointer variable, then call it (array-subscript callee) |
| `zconf.h:434` | `#ifdef HAVE_UNISTD_H` → `#if 1` — what zlib's `./configure` defines; needed for the flattened `gcc -I.` build of `gz*.c` |
| `ioutil.c:1` | `#define _XOPEN_SOURCE 700` before the first include — SACTOR parses with `-std=c99`, which hides `fileno`/`S_IFDIR`/`S_IFREG` (gcc default `-std=gnu17` shows them) |

`gate0/scan_unresolved.py` mirrors the resolver rule with SACTOR's own libclang and reports
**0 fatal call sites** in the 52 TUs after the rewrites.

## Dry runs (`gate0/dry_run{1,2,3}.log`, exit 2 each, 3 lines each)
| run | wall |
|---|---|
| 1 | `Unresolved reference: <unknown> (USR=None) at bitset.c:145` — `(isspace)(…)` inside `opng__SPAN__` |
| 2 | `Unresolved reference: <unknown> (USR=None) at pngrutil.c:4123` — array-subscript callee |
| 3 | **`Unresolved reference: <function> (USR=c:@F@__builtin_va_start) at gzwrite.c`** — the second resolver rule (`project_index.py:60-70`): a `FUNCTION_DECL` reference whose USR no TU defines. libclang gives compiler builtins the *use site* as their location, so they look like project functions with no definition. |

`gate0/scan_builtins.py` enumerates the builtin references: `__builtin_va_start`/`__builtin_va_end`
in **4 variadic functions** — `gzprintf` (gzwrite.c:456), `opng_snprintf_impl` (ratio.c:52),
`error` (optipng.c:154) and `app_printf` (optipng.c:784, 790). `app_printf` and `error` are
optipng's only output/diagnostic routines (every expected-output line of the samples goes
through `app_printf`), so removing or de-variadic-ing them is not a neutral rewrite. Same
failure class as `__builtin_inff` (`INFINITY`) in the quadtree cell, where `1e308` was an
adequate substitute; there is no substitute for `va_start`.

## Why this is a shipped-configuration failure
The resolver rule, not the LLM, rejects the project; the only escape would be patching
`project_index.py` (excluded by the reporting rules), and even then the 4 variadic functions
could never pass SACTOR's per-function harness because Rust cannot define them. Recorded as
"failed to emit analyzable Rust under the shipped configuration". No Rust exists → no PARTIAL
line; primary table cell = N/A.

## Archive contents
- `input/` — the 52 `.c` + 31 `.h` with the rewrites above, `compile_commands.json`, the three
  test inputs, `test_samples.json`, `test_task.json`
- `gate0/` — `dry_run1.log`, `dry_run2.log`, `dry_run3.log`, `scan_unresolved.py`, `scan_builtins.py`

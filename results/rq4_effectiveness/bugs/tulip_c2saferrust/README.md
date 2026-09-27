# tulipindicators × C2SaferRust: driver-level c:1 + s:1, indicator values faithful (150k)

**Verdict: `c:1 s:1 · ✓F(150k) values`** — the "7 utf8 sites untriaged" cell resolved. The utf8 sites
themselves are a NON-bug (excluded, see below); triaging them uncovered two *different*, real
translation bugs in the same file — both in C2SaferRust's rewrite of the sample driver / main wrapper,
both on trivially valid, UB-free inputs. The ~100 indicator value functions are faithful.

## Bug 1 (crash): guard hoisting — `./sample` with no arguments segfaults
C (and base c2rust, faithful): `if (argc < 2) { print usage; return 1; }` runs BEFORE any argv[1] use.
C2SaferRust hoisted the idiomatic rewrite of the argv[1] read ABOVE the guard:
```rust
unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let argv1 = unsafe { CStr::from_ptr(*argv.offset(1)).to_str().unwrap() };  // ← BEFORE the check
    let info = ti_indicators.iter().find(|i| ... == argv1);
    if argc < 2 { println!("No indicator given."); ... }
```
With zero args, `argv[1]` is the C-guaranteed NULL terminator → `CStr::from_ptr(NULL)` → **SIGSEGV
(exit 139)**. Base c2rust prints usage and exits 1. Differential: the most trivial input there is.

## Bug 2 (semantic): argc off-by-one in the rewritten `main()` wrapper
c2rust's mechanical wrapper computes argc correctly (`[prog, a1, a2, NULL].len() - 1`). C2SaferRust's
"idiomatic" rewrite counts BEFORE pushing NULL and subtracts anyway:
```rust
let args: Vec<String> = env::args().collect();          // [prog, a1, a2]
let arg_count = (args.len() - 1) as c_int;              // = real argc − 1  ← off by one
```
Every invocation behaves as if one argument fewer was given: **valid `sample sma 5` → C/base prints
the SMA table (exit 0); WIP prints `*ERROR NOT ENOUGH OPTIONS*` (exit 1)**. `sample sma 5 5` (an
over-supplied call) is what produces the correct `sma 5` output. Both sides terminate, no crash —
pure semantic difference; invisible to fuzz-Rust-alone (no panic; and a Rust-only fuzzer has no oracle
saying `sma 5` *should* work).

## The 7 utf8 sites: EXCLUDED (the original suspicion was a non-bug)
All 7 `.to_str().unwrap()` are in sample.rs. 6/7 read the static `ti_indicators` table (ASCII string
literals — unreachable for non-UTF-8). The 1 argv-derived site (L110) is masked: BOTH crates' `main()`
wrappers call `env::args()`, which itself panics on non-UTF-8 argv *before main_0 runs* — identical
panic in base and WIP (env.rs:878) → shared-artifact, not tool-attributable. Same-source attribution
discipline: no bug counted.

## Indicator values: faithful — ✓F(150k)
Reran the rundiff harness (the one that certified Laertes tulip, footnote 23) against the WIP crate:
**150,000 random price-series records across the 11 arithmetic indicators
(sma/ema/wma/rsi/mom/roc/dema/tema/trima/wilders/zlema), 687 MB of output per side, byte-identical
(cmp: 0 diffs)**. Same caveat as the Laertes cell: base c2rust is the reference (tulip's C source is
not in the repo); base is the mechanical baseline validated faithful elsewhere. Corpus: seeded
(random.seed(7)), regenerable via the recipe below.

## Repro
```bash
cd tools/frameworks/c2saferrust/laertes_benchmarks
cargo +nightly build --release --bin sample   # in tulipindicators/ and tulipindicators_WIP/
./tulipindicators/target/release/sample            # usage, exit 1
./tulipindicators_WIP/target/release/sample        # SIGSEGV        ← bug 1
./tulipindicators/target/release/sample sma 5      # SMA table, exit 0
./tulipindicators_WIP/target/release/sample sma 5  # *ERROR NOT ENOUGH OPTIONS*, exit 1  ← bug 2
```
Build notes: crates need nightly (`#![feature]`); the lib externs macOS's `__assert_rtn` — an additive
`#[no_mangle]` shim in the bin file satisfies the Linux linker (`sample_bin.rs`, appended to
`rundiff.rs`). WIP got `rundiff.rs` copied verbatim from base + the same shim.

## Files
- `excerpt_guard_hoisting.rs` — WIP main_0 head (bug 1)
- `excerpt_argc_offbyone.rs` — WIP main() wrapper (bug 2)
- `excerpt_base_faithful.rs` — base c2rust main_0 head (guard first, faithful)

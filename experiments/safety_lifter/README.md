# Second target (ACTIVE): safety-lifter validation

Apply the boundary-validity + UB-free differential-testing method to a SECOND translation tool — a
Rust→safer-Rust **safety lifter** that consumes c2rust output. This is "beyond c2rust" without needing
the full phase-2 semantic mapping, because lifting mostly preserves function names/structure.

Why a lifter (not a second C→Rust transpiler): a 2026-06-25 survey found c2rust is the ONLY
production-grade static C→Rust transpiler (Corrode/Citrus/CRUST are dead/prototype). Lifters are a
different tool doing a different translation (unsafe Rust → safe Rust), where bugs are PLAUSIBLE
(lifting changes pointers/aliasing/ownership; cf. OOPSLA'23 "Aliasing Limits", "superficial cleanup"
warnings) — unlike c2rust which is faithful-but-unsafe (we found 0 real bugs on it, as expected).

Candidates (prefer the STATIC ones — they preserve names → easy mapping):
- **Laertes** (Emre et al., OOPSLA'21) — static, compiler-feedback-driven raw-pointer→reference lifting.
- **CROWN** (CAV'23) — static, ownership-based pointer lifting.
- C2SaferRust (2025) — hybrid (c2rust + LLM); more renaming → harder mapping; lower priority.

Cleanest oracle: **lifted-Rust vs c2rust-Rust on the same inputs** (Rust↔Rust, names ~aligned, any
UB-free divergence is attributable to the LIFTER). Same boundary selection + UB-free exclusion (rule 4).

STATUS (2026-06-26): **CROWN RUNS end-to-end.** Built it WITHOUT docker and WITHOUT the 12h-throttled
Zenodo image, via a system-Z3 workaround. Reproducible recipe:

1. `rustup toolchain install nightly-2023-01-26 -c rust-src -c rustc-dev -c llvm-tools-preview`
2. `git clone -b artifact --depth 1 https://github.com/KomaEc/crown` (the `artifact` branch has the
   refactoring/rewrite + c2rust-output benchmarks: brotli, bzip2, json.h, genann, buffer, avl, bst, ht, …)
3. `sudo apt install libz3-dev` (Z3 4.13.3) — the vendored old Z3 (z3-sys 0.8.1 `static-link-z3`) does NOT
   compile on gcc-15/cmake-4, so use the SYSTEM Z3 instead:
4. in `crates/analysis/Cargo.toml` change `z3 = { version="0.12.1", features=["static-link-z3"] }`
   → `z3 = { version = "0.12.1" }`
5. `Z3_SYS_Z3_HEADER=/usr/include/z3.h cargo build --release` → `target/release/{crown,evaluation}` (27s)
6. run on a benchmark with **bash** (scripts have no shebang, need bash not dash):
   `cp -r benchmark/buffer . && bash preprocess.sh buffer && bash analyse.sh buffer && bash rewrite.sh buffer`
   → lifts pointers in-place. On `buffer`: raw `*mut/*const` 98→76; e.g. `buffer_new() -> *mut buffer_t`
   becomes `-> Option<Box<buffer_t>>`; fields annotated `/* owning */`; `Default`/`take()` added.

KEY wrinkle for the harness: lifting **changes signatures** (raw ptr → `Box`/`Option`/`&`), but function
**names are preserved** → name-mapping anchors (good), yet the differential harness must bridge the type
change. Note `Option<Box<T>>` is ABI-compatible (nullable-pointer niche) with `*mut T`, so many lifted
`extern "C"` fns stay callable via the C ABI → can still be differentially tested against the original C
(full pipeline C → c2rust → CROWN) or against the c2rust-unsafe Rust.

NEXT: build the lifted-vs-c2rust (or lifted-vs-C) differential harness + apply boundary selection + the
UB-free oracle. Bugs are plausible here (lifting changes aliasing/ownership). See [[session-handoff]].

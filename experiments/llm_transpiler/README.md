# Third target (v0 ACTIVE): LLM transpiler — the first real RENAME test of the matcher

Apply the method to an **LLM-based C→Rust transpiler**. Unlike c2rust/CROWN (which preserve names via
`#[no_mangle]`), an LLM **renames/inlines/restructures** — so the free 1:1 name mapping does NOT survive.
The "phase-2 semantic/structural mapping" this used to wait on is **already built**: the name-independent
matcher (`tools/stu_selector/matcher.py` — io-shape + metrics + opcode + call-graph topology + Hungarian,
126/128 = 98% on faithful c2rust). The LLM track is the **first genuine rename test** of that matcher.

## Pipeline (v0)

```
benchmark/pairs/<name>/source/*.c
        │  transpile.py   (LLM renames functions; dry-run = seed from c2rust output)
        ▼
out/<name>/  (Cargo.toml + src/lib.rs)   ── a std-only, loadable Rust crate
        │  run_pipeline.sh
        ▼
analyzer (rust-analyzer) → rust.json        c_analyzer.py (libclang) → c.json
        └──────────────► matcher.py (names hidden) ──► predicted C↔Rust correspondence
```

## Files

- `llm_client.py` — minimal OpenAI client. **Dry-run when no `OPENAI_API_KEY` / no `openai` SDK** (so the
  pipeline runs offline). Real mode = Structured Outputs `{cargo_toml, rust_src}`. Default model
  `gpt-5.4-mini` (override `$OPENAI_MODEL`; confirm the live id/price before a real run — GPT-5.x reject
  `temperature`, use `gpt-4.1` if you need `temperature=0`).
- `transpile.py` — `--pair <name>` → writes `out/<name>/` crate. Dry-run seeds the Rust side from the
  faithful c2rust `translated/*.rs` (pipeline smoke test); `--real` (or a key) calls the LLM.
- `run_pipeline.sh <name>` — runs analyzer + c_analyzer + matcher on the crate.
- `prompts/translate.md` — the system prompt (the deliberate "rename functions" instruction lives here).
- `out/` — generated crates + JSON (gitignored).

## Status / how to run

1. **Now (no key):** `python3 transpile.py --pair hex_encode && bash run_pipeline.sh hex_encode`
   → exercises the full pipeline on c2rust output (names preserved → matcher should score high; validates
   wiring).
2. **With a key:** `OPENAI_API_KEY=... python3 transpile.py --pair hex_encode --real && bash
   run_pipeline.sh hex_encode` → LLM renames → the **first measurement of the matcher under renaming**.
   Because names are changed, score by **hand-labeling** the true correspondence on the small seeds
   (hex_encode 2 fns → rle_codec → leb128 → rpn_eval → base64), then scale with C2SaferRust's coreutils
   crates / CRUST-Bench.

## Borrowed / to borrow (external)

- Prompts/datasets: **C2SaferRust** (github.com/vikramnitin9/c2saferrust, MIT — coreutils + Laertes
  pairs), **RustMap** (github.com/Cxm211/RustMap — compile-fix/inconsistency prompts), **CRUST-Bench**
  (github.com/anirudhkhatry/CRUST-bench — 100-repo eval harness).
- Compile-repair loop (v1, deferred): SbomFuzz `iter_fix.py`.
- `pip install -U openai` is required before a real run (not yet installed).

See `results/cross_language_matching.md` and `docs/stu_selection.md` §11.

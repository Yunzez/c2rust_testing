#!/usr/bin/env python3
"""C -> Rust crate via an LLM, feeding the existing analyzer + matcher.

The LLM is instructed to RENAME functions (idiomatic Rust) — that is the whole point:
it produces the genuine-renaming case the name-independent matcher was built for, which
faithful c2rust output (names preserved) never exercises.

dry-run (no OPENAI_API_KEY / no openai SDK): seed the Rust side from the existing c2rust
translation, so the entire pipeline (transpile -> crate -> analyzer -> matcher) is
verifiable offline. Real mode: the LLM translates + renames.

Usage:
  python3 transpile.py --pair hex_encode            # dry-run unless a key is set
  python3 transpile.py --pair hex_encode --real     # force the LLM call
  OPENAI_API_KEY=... python3 transpile.py --pair base64
"""
import argparse
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
from llm_client import LLMClient, DEFAULT_MODEL  # noqa: E402

_PROMPT = (HERE / "prompts" / "translate.md").read_text() if (HERE / "prompts" / "translate.md").exists() else ""

_CARGO = """[package]
name = "{name}"
version = "0.0.0"
edition = "2021"

[lib]
name = "{name}"
path = "src/lib.rs"

[dependencies]
{deps}

# standalone — do not let the parent workspace absorb this generated crate
[workspace]
"""


def cargo_toml(name: str, rust_src: str) -> str:
    # std-only by prompt constraint; add libc only if the (dry-run) c2rust code uses it.
    deps = 'libc = "0.2"' if "libc::" in rust_src else ""
    return _CARGO.format(name=name.replace("-", "_"), deps=deps)


def main() -> int:
    ap = argparse.ArgumentParser(description="LLM C->Rust transpiler (v0)")
    ap.add_argument("--pair", required=True, help="corpus pair under benchmark/pairs/")
    ap.add_argument("--model", default=DEFAULT_MODEL)
    ap.add_argument("--real", action="store_true", help="force a real LLM call (needs a key)")
    ap.add_argument("--out", default=str(HERE / "out"))
    args = ap.parse_args()

    pair = ROOT / "benchmark" / "pairs" / args.pair
    csrcs = sorted(pair.glob("source/*.c"))
    if not csrcs:
        return _die(f"no C source under {pair}/source")
    c_source = "".join(p.read_text() for p in sorted(pair.glob("source/*.h"))) \
        + "".join(p.read_text() for p in csrcs)

    client = LLMClient(model=args.model, dry_run=(False if args.real else None))
    result = client.translate(c_source, system_prompt=_PROMPT)

    if result is None:  # dry-run: seed the Rust side from faithful c2rust output
        rs = sorted(pair.glob("translated/*.rs"))
        if not rs:
            return _die(f"dry-run needs c2rust output at {pair}/translated/*.rs")
        rust_src = rs[0].read_text()
        mode = "DRY-RUN: c2rust output (names preserved) — pipeline smoke test, NOT an LLM run"
    else:
        rust_src = result["rust_src"]
        mode = f"LLM {args.model}: translated + renamed"

    out = Path(args.out) / args.pair
    (out / "src").mkdir(parents=True, exist_ok=True)
    (out / "src" / "lib.rs").write_text(rust_src)
    (out / "Cargo.toml").write_text(cargo_toml(args.pair, rust_src))
    print(f"[{mode}]")
    print(f"  wrote crate -> {out}")
    print(f"  run: bash {HERE / 'run_pipeline.sh'} {args.pair}")
    return 0


def _die(msg: str) -> int:
    print(f"error: {msg}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())

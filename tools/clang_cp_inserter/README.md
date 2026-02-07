# clang_cp_inserter

Minimal Clang LibTooling rewriter that inserts semantic checkpoints at
function entry and before each return.

## What it does
- Inserts `cp("entry", NULL, 0);` at the start of a function body.
- Inserts `cp("exit", NULL, 0);` immediately before each `return`.
- Targets a comma-separated list of function names.

## Build
```bash
mkdir -p tools/clang_cp_inserter/build
cd tools/clang_cp_inserter/build
cmake ..
make -j
```

## Usage
Print rewritten source to stdout:
```bash
./clang_cp_inserter --functions reverse ../../examples/entry_exit/demo.c -- -I../../examples/entry_exit
```

Rewrite in place:
```bash
./clang_cp_inserter --functions reverse --in-place ../../examples/entry_exit/demo.c -- -I../../examples/entry_exit
```

## Notes
- Requires a working Clang/LLVM installation with LibTooling.
- The checkpoint symbol defaults to `cp` (override with `--cp-symbol`).
- This is intentionally minimal; expand it later for richer checkpoint policies.

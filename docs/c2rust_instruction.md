# C2Rust Instructions (qsort example)

This records the exact commands used to build the C2Rust CLI and transpile the `qsort.c` example into `projects/qsort_example/translated/`.


## 1) Build the C2Rust CLI

```sh
cd /home/yzzhao3/c2rust_testing/transpiler/c2rust
cargo build -p c2rust --release
```

## 2) Prepare compile_commands.json for qsort

```sh
mkdir -p /home/yzzhao3/c2rust_testing/projects/qsort_example/build
cd /home/yzzhao3/c2rust_testing/projects/qsort_example/build
cmake ../source -DCMAKE_EXPORT_COMPILE_COMMANDS=1
cmake --build .
```

## 3) Transpile C to Rust

```sh
/home/yzzhao3/c2rust_testing/transpiler/c2rust/target/release/c2rust transpile \
  -o /home/yzzhao3/c2rust_testing/projects/qsort_example/translated \
  compile_commands.json
```

#!/usr/bin/env python3

"""Scaffold a fuzz project with C/Rust sources and cargo-fuzz shim setup.

Workflow:
1) Create fuzz/<project>/
2) Copy C sources into fuzz/<project>/c/
3) Copy Rust source into fuzz/<project>/src/lib.rs
4) Ensure Cargo project exists (cargo init --lib if needed)
5) Write build.rs to compile C oracle with clang coverage
6) Run `cargo fuzz init`
7) Patch fuzz/Cargo.toml to use libafl_libfuzzer shim

usage: 
python3 tools/gen_fuzz_target.py \
  --project urlparser_example \
  --c-source transpiler/c2rust/examples/urlparser/repo/url.h \
  --rust-source tmp_urlparser_translated/src/url.rs

"""

from __future__ import annotations

import argparse
import shutil
import subprocess
from pathlib import Path


def run(cmd: list[str], cwd: Path) -> None:
    subprocess.run(cmd, cwd=cwd, check=True)


def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate a cargo-fuzz project with LibAFL shim"
    )
    parser.add_argument("--project", required=True, help="Project name (folder under fuzz/)")
    parser.add_argument(
        "--c-source",
        required=True,
        action="append",
        help="Path to C source file (repeatable)",
    )
    parser.add_argument(
        "--rust-source",
        required=True,
        help="Path to Rust source file to use as src/lib.rs",
    )
    parser.add_argument(
        "--out-dir",
        default="fuzz",
        help="Output base directory (default: fuzz)",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Overwrite existing project directory",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[1]
    out_base = (repo_root / args.out_dir / args.project).resolve()

    if out_base.exists() and not args.force:
        raise SystemExit(f"Output exists: {out_base} (use --force to overwrite)")

    # Prepare directories.
    if out_base.exists() and args.force:
        shutil.rmtree(out_base)
    out_base.mkdir(parents=True, exist_ok=True)

    c_dir = out_base / "c"
    c_dir.mkdir(parents=True, exist_ok=True)

    # Copy C sources. If a .h contains implementation, copy as .c for compilation.
    c_sources = [Path(p).resolve() for p in args.c_source]
    copied_c = []
    for src in c_sources:
        if src.suffix == ".h":
            dest = c_dir / (src.stem + ".c")
            dest.write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
        else:
            dest = c_dir / src.name
            shutil.copy2(src, dest)
        copied_c.append(dest.name)

    # Ensure cargo project exists.
    if not (out_base / "Cargo.toml").exists():
        run(["cargo", "init", "--lib"], cwd=out_base)

    # Use edition 2021 to avoid 2024 unsafe-attribute issues in translated code.
    cargo_toml = (out_base / "Cargo.toml").read_text(encoding="utf-8")
    cargo_toml = cargo_toml.replace('edition = "2024"', 'edition = "2021"')
    if 'edition = "2021"' not in cargo_toml:
        cargo_toml = cargo_toml.replace('[package]\n', '[package]\n')
    if "[build-dependencies]" not in cargo_toml:
        cargo_toml += "\n[build-dependencies]\ncc = \"1\"\n"
    (out_base / "Cargo.toml").write_text(cargo_toml, encoding="utf-8")

    # Copy Rust source into src/lib.rs
    rust_src = Path(args.rust_source).resolve()
    write_file(out_base / "src" / "lib.rs", rust_src.read_text(encoding="utf-8"))

    # Write build.rs to compile the C oracle with clang coverage.
    c_list = "\n".join(f"        \"c/{name}\"," for name in copied_c)
    build_rs = f"""fn main() {{
    // Build the C oracle with clang sanitizer-coverage instrumentation.
    let mut build = cc::Build::new();
    build
        .compiler(\"clang\")
        .flag(\"-O1\")
        .flag(\"-g\")
        .flag(\"-fsanitize-coverage=trace-pc-guard,trace-cmp\")
        .warnings(false);

    // Rename C symbols to avoid collisions with Rust-transpiled functions.
    build
        .define("url_parse", "c_url_parse")
        .define("url_free", "c_url_free")
        .define("url_get_protocol", "c_url_get_protocol")
        .define("url_get_host", "c_url_get_host")
        .define("url_get_hostname", "c_url_get_hostname")
        .define("url_get_path", "c_url_get_path")
        .define("url_get_query", "c_url_get_query")
        .define("url_get_hash", "c_url_get_hash")
        .define("url_get_port", "c_url_get_port")
        .define("url_get_auth", "c_url_get_auth")
        .define("url_get_pathname", "c_url_get_pathname")
        .define("url_get_search", "c_url_get_search")
        .define("URL_SCHEMES", "C_URL_SCHEMES")
        .define("strdup", "c_strdup")
        .define("url_is_protocol", "c_url_is_protocol")
        .define("url_is_ssh", "c_url_is_ssh")
        .define("url_inspect", "c_url_inspect")
        .define("url_data_inspect", "c_url_data_inspect");

    // Add all C sources.
    for file in [
{c_list}
    ] {{
        build.file(file);
    }}

    build.compile(\"c_oracle\");

    // Link Clang's coverage runtime so sanitizer coverage symbols resolve.
    let resource_dir = std::process::Command::new(\"clang\")
        .arg(\"--print-resource-dir\")
        .output()
        .expect(\"failed to run clang --print-resource-dir\");
    if !resource_dir.status.success() {{
        panic!(\"clang --print-resource-dir failed\");
    }}
    let resource_dir = String::from_utf8(resource_dir.stdout)
        .expect(\"clang resource dir is not valid UTF-8\")
        .trim()
        .to_string();
    let lib_dir = std::path::Path::new(&resource_dir).join(\"lib\").join(\"linux\");

    let arch = std::env::var(\"CARGO_CFG_TARGET_ARCH\").unwrap_or_else(|_| \"x86_64\".to_string());
    let rt_name = format!(\"clang_rt.profile-{{}}\", arch);

    println!(\"cargo:rustc-link-search=native={{}}\", lib_dir.display());
    println!(\"cargo:rustc-link-lib=static={{}}\", rt_name);
}}
"""
    write_file(out_base / "build.rs", build_rs)

    # Initialize cargo-fuzz.
    run(["cargo", "fuzz", "init"], cwd=out_base)

    # Patch fuzz/Cargo.toml to use libafl shim and edition 2021.
    fuzz_toml = out_base / "fuzz" / "Cargo.toml"
    content = fuzz_toml.read_text(encoding="utf-8")
    content = content.replace('edition = "2024"', 'edition = "2021"')
    if "libfuzzer-sys" in content:
        lines = []
        for line in content.splitlines():
            if line.strip().startswith("libfuzzer-sys"):
                lines.append('libfuzzer-sys = { version = "0.15.4", package = "libafl_libfuzzer" }')
            else:
                lines.append(line)
        content = "\n".join(lines)
    else:
        content = content.replace(
            "[dependencies]",
            "[dependencies]\nlibfuzzer-sys = { version = \"0.15.4\", package = \"libafl_libfuzzer\" }",
        )
    fuzz_toml.write_text(content, encoding="utf-8")

    print(f"Scaffolded fuzz project at: {out_base}")
    print("If the harness fails to run, check your toolchain: nightly-2025-09-01 is known to work.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Generate one differential harness per ELIGIBLE matched pair, using the frozen generator.

For each eligible entry this runs, unmodified:

    tools/stu_selector/gen_diff_harness.py --pair <pair> --entry <c_fn> --rust-entry <rust_fn>
        --infer-schema --ub-free [--expose-entry] --out <outdir>

so every harness is  input -> C oracle (UBSan-instrumented) -> UB gate -> Rust -> compare.

Three post-generation fixups, all platform/packaging only, none touching translated code:
  1. `pub use crate::<mod>::<entry>;` appended to src/lib.rs for entries that were C `static`
     (the generator's --expose-entry makes the definition public but the flattened crate needs the
     root re-export for `translated::<entry>` to resolve).  Appended at the END of the file, so
     no line number inside a module body moves.
  2. `c/shims.c` added and `build.rs` told to compile it — Linux definitions of the macOS libc
     symbols the transpiled crate references.
  3. the fuzz crate is pinned to the same nightly as everything else via rust-toolchain.

usage: gen_harnesses.py <pair_dir> <eligibility.json> <out_root>
"""
import json, re, subprocess, sys, shutil
from pathlib import Path

sys.path.insert(0, "/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov/rq4_gen")
import gen_diff_harness as gdh   # for strip_static_c

REPO = Path("/home/yunzez/c2rust_testing")
GEN = Path("/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov/rq4_gen/gen_diff_harness.py")   # frozen generator + the two documented RQ4 fixes
HERE = Path(__file__).resolve().parent


def strip_static_inline(text: str, entry: str):
    """Give a C `static __inline__` definition external linkage.

    gdh.strip_static_c only handles `static` adjacent to the return type.  bzip2 writes

        static
        __inline__
        UChar mmed3 ( UChar a, ... )

    across three lines, and merely deleting `static` leaves a C99 `__inline__` definition, which
    emits no external symbol at all (observed: `undefined symbol: c_fallbackSimpleSort`).  Remove
    both tokens.  Returns (text, changed)."""
    pat = re.compile(
        rf'(?m)^[ \t]*static[ \t]*\n(?:[ \t]*(?:__inline__|inline)[ \t]*\n)?'
        rf'([ \t]*[A-Za-z_][\w \t\*]*\b{re.escape(entry)}[ \t]*\()')
    new, n = pat.subn(r'\1', text, count=1)
    if n:
        return new, True
    pat2 = re.compile(rf'(?m)^([ \t]*)static[ \t]+(?:__inline__|inline)[ \t]+'
                      rf'((?:[A-Za-z_][\w \t\*]*)?\b{re.escape(entry)}[ \t]*\()')
    new, n = pat2.subn(r'\1\2', text, count=1)
    return (new, True) if n else (text, False)

def main(pair_dir, elig_json, out_root):
    pair = Path(pair_dir)
    elig = json.load(open(elig_json))
    defs = json.load(open(next(pair.glob("translated/*.rs.defs.json"))))
    out_root = Path(out_root); out_root.mkdir(parents=True, exist_ok=True)

    report = []
    for row in elig["rows"]:
        if not row["eligible"]:
            continue
        entry, rust_entry = row["c_entry"], row["rust_entry"]
        out = out_root / entry
        if out.exists():
            shutil.rmtree(out)
        schema = HERE / "schemas" / f"{entry}.json"
        cmd = [sys.executable, str(GEN), "--pair", str(pair), "--entry", entry,
               "--rust-entry", rust_entry, "--ub-free", "--out", str(out)]
        if schema.exists():
            cmd += ["--schema", str(schema)]      # hand-authored input model for this boundary
        else:
            # no schema: the boundary is not usefully runnable (BZ2_bz__AssertH__fail terminates
            # the process on every input), keep the inference path so it is still generated and
            # counted, but it is excluded from the campaign.
            cmd += ["--infer-schema"]
        private = entry in defs["private"]
        if private:
            cmd.append("--expose-entry")
        p = subprocess.run(cmd, capture_output=True, text=True, cwd=str(REPO))
        rec = {"entry": entry, "rust_entry": rust_entry, "private_static": private,
               "schema": str(schema) if schema.exists() else None,
               "generate_rc": p.returncode, "generate_out": (p.stdout + p.stderr).strip()[-500:]}
        if p.returncode != 0:
            rec["stage"] = "generate_failed"
            report.append(rec); print(f"{entry:30s} GENERATE FAILED"); continue

        lib = out / "src" / "lib.rs"
        if private:
            mod = defs["defs"][entry]
            lib.write_text(lib.read_text() +
                           f"\n// added by gen_harnesses.py: root re-export of the exposed static entry\n"
                           f"pub use crate::{mod}::{entry};\n")
        # the generator copies pair/source/<entry .c> and every *.h into c/; our single C file is
        # an amalgamation that #includes its sibling .c files, so those must be present too.
        # For a C `static` entry the generator applies strip_static_c() to the pair's single .c so
        # the oracle symbol gets external linkage. Our .c is an amalgamation of #include lines, so
        # the real definition sits in a sibling file and the strip finds nothing. Apply the
        # generator's own strip_static_c to the sibling that actually defines the entry.
        stripped_from = None
        for extra in sorted((pair / "source").glob("*.c")):
            if extra.name == "bzip2lib.c":
                continue
            text = extra.read_text()
            if private:
                text, changed = gdh.strip_static_c(text, entry)
                if not changed:
                    text, changed = strip_static_inline(text, entry)
                if changed:
                    stripped_from = extra.name
            (out / "c" / extra.name).write_text(text)
        if private and stripped_from is None:
            rec["stage"] = "expose_failed"
            rec["error"] = f"could not give C `static` {entry} external linkage in any sibling .c"
            report.append(rec); print(f"{entry:30s} EXPOSE FAILED"); continue
        rec["c_static_stripped_from"] = stripped_from
        shutil.copy(HERE / "shims.c", out / "c" / "shims.c")
        b = out / "build.rs"
        t = b.read_text()
        assert 'build.compile("c_oracle");' in t, "unexpected generated build.rs"
        t = t.replace('    build.compile("c_oracle");',
                      '    build.file("c/shims.c");   // platform adapter, added by gen_harnesses.py\n'
                      '    build.compile("c_oracle");\n'
                      '    // shims.o is only referenced from the Rust rlib, which the linker sees\n'
                      '    // AFTER libc_oracle.a, so the archive member is otherwise dropped\n'
                      '    // (observed: undefined symbol: __maskrune / _DefaultRuneLocale).\n'
                      '    println!("cargo:rustc-link-arg=-Wl,-u,__maskrune");\n'
                      '    println!("cargo:rustc-link-arg=-Wl,-u,_DefaultRuneLocale");')
        b.write_text(t)
        (out / "rust-toolchain").write_text("nightly-2025-09-01\n")
        (out / "fuzz" / "rust-toolchain").write_text("nightly-2025-09-01\n")
        rec["stage"] = "generated"
        rec["target"] = f"{pair.name}_ft"
        report.append(rec)
        print(f"{entry:30s} generated{' (exposed static)' if private else ''}")

    json.dump(report, open(out_root / "generation_report.json", "w"), indent=1)
    print(f"\ngenerated {sum(1 for r in report if r['stage']=='generated')} / {len(report)} eligible")

if __name__ == "__main__":
    main(*sys.argv[1:4])

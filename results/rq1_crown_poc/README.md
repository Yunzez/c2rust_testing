# CROWN-lifted bug-hunt PoC harnesses

Reusable OOP differential harnesses testing CROWN safety-lifted output vs the C original.
See results/rq1_crown_recon.md for the reconnaissance + findings.

## rgba/ (validated, clean over 15864 execs)
- rgba_oracle.c  : C oracle (rgba_from_string), build:
    clang -fsanitize=undefined,address -fno-sanitize-recover=all -ffp-contract=off -g -O1 \
      -I. rgba_oracle.c <crown>/c-code/rgba/src/rgba.c -o rgba_oracle
- rgba_ft.rs     : cargo-fuzz target, calls CROWN-lifted rgba_from_string natively (Option<&mut>),
                   C as subprocess oracle. UB gate + determinism gate + stdout isolation.
- fuzz_Cargo.toml: depends on the lifted crate at tools/frameworks/crown/results/rgba (gitignored).

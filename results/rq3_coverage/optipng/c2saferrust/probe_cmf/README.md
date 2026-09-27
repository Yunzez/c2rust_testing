# C16 probe — optimize_cmf on the valid header 08 1d (2026-09-09)

`probe.c` = pngwutil.c:251-289 verbatim; `probe.rs` = optipng_c2saferrust.rs:47840-47869 verbatim.

    clang -O1 -fsanitize=undefined -fno-sanitize-recover=all probe.c -o probe_c && ./probe_c
    RUSTUP_TOOLCHAIN=nightly-2025-09-01 rustc -O -C overflow-checks=on  probe.rs -o probe_on  && ./probe_on
    RUSTUP_TOOLCHAIN=nightly-2025-09-01 rustc -O -C overflow-checks=off probe.rs -o probe_off && ./probe_off

Result table in `../TRIAGE.md`: C returns 88 1a / e8 02 / f8 1d; Rust with overflow checks panics on every
size; without them identical to C. c2rust renders the line as `z_cinfo.wrapping_sub(1)`.

# Layout validation of `scripts/rq4/materialize_seeds.py` (tulip × c2rust, 2026-09-09)

Three boundaries chosen for the layouts the ti_sma probe did not exercise: **ti_bbands** (2 options, a
fuzz-filled inout output row AFTER the options), **ti_stoch** (3 input rows of mixed 4096/1024
extent, 3 options at byte 73 732, seed longer than `-max_len 65536`), **ti_decay** (fuzz-filled
output row, 1 option). One harness build, 30 s rust-only fork campaigns, seed 42, generator sha256[:16] = 3d43f75aea807da1.
Arms: base = cell.py's default 64-byte seed only; random = 6 length-matched pseudorandom seeds;
grid = the same 6 byte strings with every `options` element overwritten by {1,2,3,5,10,20}.

| boundary | seed length | base | random | grid | grid, `-max_len 131072` |
|---|---:|---:|---:|---:|---:|
| ti_bbands | 65 556 | 31/146 | 128/146 | 128/146 | — |
| ti_stoch | 73 756 | 42/223 | 42/223 | **207/223** | 209/223 |
| ti_decay | 40 972 | 40/41 | 41/41 | 41/41 | — |

(regions of the boundary's own function, covered / total, from the llvm-cov export in `exports/`.)

What it establishes: the materialiser's byte offsets are right where it matters (ti_stoch: only the
arm that overwrites the option bytes moves, 42 → 207); an initial seed longer than `-max_len` is
executed whole in fork mode (ti_stoch's options sit at byte 73 732, past 65 536, and are decoded; the
128 KiB run adds 2 regions) — it breaks the *initial* reachability barrier; whether the fuzzer keeps
mutating inputs at that length is not established, since libFuzzer bounds generated inputs by `-max_len`; and the length effect alone can be the whole gain (ti_bbands: random =
grid — within 30 s the fuzzer finds accepted option values by itself once the input is long enough).
That last row is why the full experiment has a length-matched random arm.

# RQ1 Bug #1 — C2SaferRust `qsort`: `int→usize` index breaks recursion-termination invariant

**Status:** confirmed, fuzzer-found, attributed to C2SaferRust (not upstream c2rust).
**Source artifact:** `tools/frameworks/c2saferrust/laertes_benchmarks/qsort_WIP/qsort.rs`
(C2SaferRust's published safety-lifted output; C2SaferRust = Nitin et al., arXiv 2501.14257).

## The bug

C `quickSort` relies on a **negative sentinel** to terminate recursion:

```c
void quickSort(int* arr, int low, int high){
  if(low<high){ int i=partition(arr,low,high);
    quickSort(arr,low,i-1);      /* i-1 can be -1; then low(0) < -1 is FALSE -> stop */
    quickSort(arr,i+1,high); }
}
```

C2SaferRust's safety-lifting rewrote the indices to `usize` and `i-1` to
`i.wrapping_sub(1)`:

```rust
pub fn quickSort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let i = unsafe { partition(arr.as_mut_ptr(), low as i32, high as i32) } as usize;
        quickSort(arr, low, i.wrapping_sub(1));   // i==0 -> usize::MAX, never < -> infinite recursion
        quickSort(arr, i + 1, high);
    }
}
```

When `partition` returns 0 (pivot is the minimum of the range), `i.wrapping_sub(1)`
is `usize::MAX` instead of `-1`; `low < usize::MAX` is always true, so the recursion
never terminates. En route, `usize::MAX as i32 == -1` is passed back into
`partition`, which dereferences `*arr.offset(-1)` — an out-of-bounds read. The
"make it safer" rewrite **broke the termination invariant** the C code implicitly
relied on (indices may go negative as a loop sentinel).

## How we found it (fuzzer, not by hand)

- matcher pairs C↔Rust by name (names preserved here — trivial pairing);
- our harness generator produced the boundary skeleton; we hand-wrote the
  **reshaped-signature bridge** (`evidence/fuzz_target_quickSort_bridge.rs`):
  C ABI `(int*, int, int)` vs C2SaferRust's reshaped Rust ABI
  `(&mut [i32], usize, usize)`, with the canonical call contract `low=0,
  high=len-1` so every input is legal for the C oracle; the in-loop UB-free gate
  stays on;
- libFuzzer (cargo-fuzz 0.13, toolchain nightly-2025-09-01) crashed at
  **~81 executions, <1 s**, minimized to a **2-byte** input decoding to the array
  `[5, 0]` (`evidence/trigger_arr_5_0.bin`);
- crash form: ASan **heap-buffer-overflow, READ of size 4** (the `*arr.offset(-1)`),
  reached via the runaway recursion.

## Attribution (RQ1 criteria all met)

Same trigger inputs `[5,0]`, `[2,1]`, `[3,2]`:

| generator | output file | behavior | verdict |
|---|---|---|---|
| C original (ASan+UBSan) | `source/qsort.c` | sorts correctly, **no sanitizer report** (input UB-free) | oracle |
| c2rust (baseline input to C2SaferRust) | `translated/c2rust_baseline.rs` | sorts correctly, exit 0 (`int` indices preserved) | ✅ faithful |
| Laertes | `translated/laertes.rs` | `i - 1` stays `c_int` | ✅ no bug |
| **C2SaferRust (WIP)** | `translated/c2saferrust_WIP.rs` | **stack overflow / OOB abort** | 🔴 bug introduced |

1. **C defined on the triggering input** — ASan+UBSan clean; c2rust/Laertes correct
   on the same inputs.
2. **Rust diverges from the C contract** — non-termination + OOB read.
3. **Upstream ruled out** — the c2rust baseline (C2SaferRust's own input) is
   correct; the defect is localized to C2SaferRust's `int→usize` index rewrite of
   `quickSort`.

**Corroborating:** C2SaferRust's own `evidence/c2saferrust_log.txt` marks
`partition` as `Failure` (its slice rewrite failed self-verification) — yet it
published the containing `quickSort` anyway. Its example-based verification did not
catch the consequence; our UB-gated differential fuzzing did.

## Reproduce

```bash
# C oracle + c2rust baseline (correct) vs C2SaferRust WIP (crashes) on [5,0]:
#   see scratchpad rq1_qsort/{base.rs, wip.rs} construction, or:
rustc +nightly-2025-09-01 --edition 2021 -A warnings -o /tmp/wip translated/c2saferrust_WIP.rs \
  # (append a `fn main(){ let mut a=[5i32,0]; quickSort(&mut a,0,1); }`)
```

The end-to-end fuzz reproduction uses `evidence/fuzz_target_quickSort_bridge.rs` in a
cargo-fuzz harness over the pair (source/qsort.c + translated/c2saferrust_WIP.rs).

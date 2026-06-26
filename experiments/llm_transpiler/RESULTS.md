# LLM-transpiler rename results (matcher under genuine renaming)

The matcher's headline numbers (98% on lil) were on **faithful c2rust**, where names are
preserved (`#[no_mangle]`) so name-equality IS the ground truth. This track is the first
test where the translator **renames / restructures**: a real `gpt-5-mini` C→Rust
translation, names hidden, scored against a **hand-labeled** correspondence
(`truth/<pair>.json`), since name-equality no longer holds.

## Results (gpt-5-mini, 2026-06)

| pair | C fns | Rust fns | what the LLM did | accuracy (labeled) |
|------|-------|----------|------------------|--------------------|
| hex_encode | 2 | 5 (3 tests) | renamed `hex_encode`→`encode_hex_lowercase`; **folded** `(src,len,dst,dst_cap)→size_t` into `(&[u8],&mut[u8])→usize` | **2/2 = 100%** |
| base64 | 2 | 10 (5 tests, 3 helpers) | **decomposed** 2 C fns into 5 Rust fns | **2/2 = 100%** (picked the 2 real ones out of 10) |
| bignum | 27 | 31 (2 tests, `Default`, `require`) | systematic `bignum_*`→`bn_*`; 2 semantic renames (`from_int`→`from_u64`, `to_int`→`to_u32`) | **23/27 = 85%** raw · **25/27 = 92%** with scaffolding filtered |

The matcher survives **rename**, **signature folding**, and **decomposition** — none of
which a name-preserving tool (c2rust/CROWN) ever produces.

## Two findings

### 1. Rust-idiomatic scaffolding has no C counterpart → forced assignment misfires
The LLM emits Rust functions with **no C source function**: `#[cfg(test)]` unit tests, an
`impl Default`, an `assert`-style `require` helper. With more Rust fns than C fns, the
Hungarian assignment leaves some Rust fns unmatched — but it chose to misassign two real C
functions (`bignum_init`→`default`, `bignum_assign`→`require`, scores 0.43/0.37) onto
scaffolding, cascading into 4 wrong pairs. Filtering the 4 scaffolding fns recovers
85%→92%. **Note:** filtering only `#[cfg(test)]` is NOT enough here — the slot-stealers
(`default`, `require`) are non-test functions. The principled fix is *unmatched handling*
(let a C fn go unmatched when its best genuine score is poor) rather than ad-hoc name
filtering.

### 2. Aggressive idiomatic rewriting can erase structural signal
bignum's 2 residual misses (after filtering) are a **swap**: `bignum_to_int`↔`bignum_to_string`.
The LLM rewrote `to_string` from C's 3-param buffer-fill `(bn*, char*, int)→void` into a
1-param `(&BigNum)→String`, collapsing the arity/shape difference that distinguished them.
Same class as the `fnc_streq`↔`fnc_strcmp` residual on lil → needs **signal C
(literals/constants)**: `to_string` carries the `"0123456789abcdef"` table + shift-by-4;
`to_int` does not.

## How to reproduce
```
OPENAI_API_KEY=$(cat key.env) python3 transpile.py --pair <p> --real
bash run_pipeline.sh <p>        # auto-uses truth/<p>.json if present
```
Truth files hand-labeled in `truth/`. `out/` is gitignored (regenerate via the two commands).

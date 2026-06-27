# LLM-transpiler rename results (matcher under genuine renaming)

The matcher's headline numbers (98% on lil) were on **faithful c2rust**, where names are
preserved (`#[no_mangle]`) so name-equality IS the ground truth. This track is the first
test where the translator **renames / restructures**: a real `gpt-5-mini` C→Rust
translation, names hidden, scored against a **hand-labeled** correspondence
(`truth/<pair>.json`), since name-equality no longer holds.

## Results (gpt-5-mini, 2026-06; matcher v1 = test+trait-boilerplate exclusion, partial matching)

| pair | C fns | Rust fns (after exclusion) | what the LLM did | accuracy (labeled) |
|------|-------|----------|------------------|--------------------|
| hex_encode | 2 | 2 (3 tests excluded) | renamed `hex_encode`→`encode_hex_lowercase`; **folded** `(src,len,dst,dst_cap)→size_t` into `(&[u8],&mut[u8])→usize` | **2/2 = 100%** |
| base64 | 2 | 5 (5 tests excluded) | **decomposed** 2 C fns into 5 Rust fns | **2/2 = 100%** (picked the 2 real ones out of 5) |
| bignum | 27 | 28 (`Default`+2 tests excluded) | systematic `bignum_*`→`bn_*`; 2 semantic renames (`from_int`→`from_u64`, `to_int`→`to_u32`) | **24/27 = 88%** (ceiling 92%, see below) |

The matcher survives **rename**, **signature folding**, and **decomposition** — none of
which a name-preserving tool (c2rust/CROWN) ever produces.

## The bignum story (an adversarial case) — what the diagnostics revealed

bignum is hard because the LLM injected Rust nodes with **no C counterpart** that act as
**topological hubs** and poison similarity propagation. The `--diag` output is the evidence:

```
hubs (in-degree df)  Rust: [(25,'require'), (9,'default'), (4,'bn_assign')]
true-match topo_delta (sim_topo - sim_node), all NEGATIVE:
  bignum_init  -0.178   bignum_to_int -0.317   bignum_assign -0.151
```
`require` (an assert helper, called by 25 fns) and `default` (`impl Default`) attract the
propagation, pushing every true match down in rank. This is **hub poisoning**, and it
settles a design debate empirically: keeping an extracted helper (`require`) in the
topology graph is *harmful* when it is a hub.

### What each lever buys (bignum, ceiling = 92%; last 2 are signal-failure)

| stage | mechanism | catches | bignum |
|-------|-----------|---------|--------|
| baseline (forced Hungarian) | — | — | 23/27 = 85% |
| **v1** | `#[cfg(test)]`/`#[test]` exclusion + partial (dummy) matching | tests | 23/27 = 85% |
| **+ trait-boilerplate exclusion** | `impl Default/Clone/Debug/...` out of candidate+graph | `default` | **24/27 = 88%** ✅ done |
| + df-cap on topology (deferred) | drop >50%-of-program hubs from neighbor-sets | `require` (df 25) | 25/27 = 92% |
| + signal C (deferred) | literal/constant features | `to_int`↔`to_string` swap | 27/27 |

Why these mechanisms and not a score threshold: real matches score ~0.10 while the
pollutant `init→default` scored 0.43 — **absolute score cannot separate them**, so τ is a
low outside-option floor (0.05), not a quality gate. And margins are ~0.00 even on correct
matches (homogeneous clusters), so margin can flag *ambiguous* but cannot *reject*.

### Residual signal-failure (the deferred signal-C case)
bignum's last 2 misses are a **swap**: `bignum_to_int`↔`bignum_to_string`. The LLM rewrote
`to_string` from C's 3-param buffer-fill `(bn*,char*,int)→void` into 1-param `(&BigNum)→String`,
collapsing the arity/shape difference. True match sits at **rank 25 even in node_sim** — no
assignment/topology trick recovers it; needs **signal C (literals)**: `to_string` carries the
`"0123456789abcdef"` table + shift-by-4, `to_int` does not. Same class as `fnc_streq`↔`fnc_strcmp`.

## Roadmap
- **done:** matcher v1 (test exclusion, partial/dummy matching, rank+hub diagnostics) +
  trait-boilerplate exclusion. Zero risk to faithful c2rust (it emits `#[no_mangle]` free
  fns, no `impl Default`) — the new exclusions are no-ops there.
- **next (gated on a rebuilt lil regression set):** df-cap / hub-aware topology for `require`.
  Deferred until lil is back as a regression fixture, since df-cap is the one lever that can
  weaken the homogeneous-cluster topology lil depends on.
- **later:** signal C (literals) for the `to_int`/`to_string` and `streq`/`strcmp` symmetries.

## How to reproduce
```
OPENAI_API_KEY=$(cat key.env) python3 transpile.py --pair <p> --real
bash run_pipeline.sh <p>            # auto-uses truth/<p>.json if present
matcher.py --c c.json --rust r.json --truth t.json --diag   # rank + hub diagnostics
```
Truth files hand-labeled in `truth/`. `out/` is gitignored (regenerate via the two commands).

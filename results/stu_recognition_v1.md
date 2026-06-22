# STU Recognition v1 — does the selector find the right boundary? (2026-06-22)

Goal (per the project lead): before adding LLM-transpiler difficulty, **prove STU recognition
works on the controlled c2rust single-file case.** The selection axis here is harness *validity*
(fuzzable / comparable / certain), not structural divergence — c2rust single-TU is 1:1, so the
structural axis is trivial (see `feature_study_v1.md` v2). This axis is the part that generalizes
to the LLM pipeline later.

## Method

- `tools/stu_selector/frontier.py`: interpretable-baseline selector. Score each matched function,
  then pick the **call-graph-highest valid boundaries** (a risk-bounded antichain).
- **Ground truth for free:** every benchmark C file carries a `// ENTRY: <sig>` comment = the
  human-intended fuzz target. We check whether the selected frontier recovers it.
- Corpus: the 18-program benchmark (`benchmark/pairs/`).

## Result

**17 / 18 entries recovered. The single non-recovery is a *correct rejection*.**

| Outcome | Programs |
|---|---|
| HIT (selector picked the intended entry) | 17 of 18 |
| Correct rejection | `array_map_reduce` → `(none)` — its entry takes **function-pointer parameters**, so it is not fuzzable as a standalone STU until the callbacks are bound. The selector flags this rather than fuzzing a bogus boundary. |

## The design lesson (this is the real finding)

A first version gated boundaries on cost and wrongly excluded legitimate entries (nested-pointer
inputs, internal indirect dispatch) — dropping recognition to 13/18. The fix:

> **Cost should RANK candidates, not GATE them.** Only a genuine blocker excludes a boundary.

- **HARD gate (excludes):** a function-pointer *parameter* — a fuzzer cannot synthesize a
  function value. (`array_map_reduce` is the only such case here.)
- **Costs (lower confidence, do NOT exclude):** nested-pointer inputs (`int**`, `char**` — a
  structured generator can build them), pointer outputs (need normalization), internal indirect
  dispatch (it is inside the black box; it does not stop you feeding the entry and comparing its
  output).

## Known effects worth noting

- **Indirect dispatch fragments the frontier.** In `opcode_dispatch` / `state_machine`, the
  handler functions are called *only* through a dispatch table, so the static C call graph has no
  resolved edge to them and they surface as extra frontier roots alongside the real entry. The
  intended entry is still recovered (HIT); the extra roots are an artifact of unresolved indirect
  edges, not wrong boundaries.

## Honest scope

This validates **boundary recognition against human-intended entries on the clean c2rust case** —
the machinery (call graph → mapping → features → frontier) end-to-end picks the sensible boundary
and correctly rejects the un-fuzzable one. It does **not** yet validate against **actual fuzzing
outcomes** (G1 false-divergence rate on equivalent c2rust output) or against **harder LLM /
multi-file** translations. Those are the next steps — and the framework is now general enough to
take them: the same validity axis applies, with the structural axis layered on top for LLM output.

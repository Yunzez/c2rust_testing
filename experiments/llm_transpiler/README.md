# Third target (DEFERRED — placeholder): LLM transpiler + phase-2 semantic mapping

Placeholder only. Do NOT work on this yet (user, 2026-06-25: "建个 folder 就行 先不动").

Goal (future): apply the method to an LLM-based C→Rust transpiler (e.g. a c2rust-seeded one that still
emits compilable Rust). This is the only path that proves the method generalizes across genuinely
different transpilers — but LLM output renames/inlines/restructures, so the free `#[no_mangle]` 1:1
C↔Rust mapping does NOT survive. Requires building the deferred **phase-2 semantic/structural mapping**
(the `align()` replacement: match C↔Rust functions by I/O contract, call-graph shape, test-based
equivalence) — i.e. the `name_preserving_mapping = false` path.

Sequencing: do the static safety-lifter target first (../safety_lifter), which exercises a second tool
with only a PARTIAL mapping gap; tackle this full-mapping LLM phase after. See stu_selection.md §11.

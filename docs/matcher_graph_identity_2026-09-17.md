# Matcher graph-identity correction — 2026-09-17

Implemented a frontend correctness fix, not a score/threshold optimization.
`matcher.py` is unchanged: the comparison uses archived deployment policy
`alpha=0.7`, `epsilon=0.01`, `tau=0.05`, 15 iterations and hub cap 0.5.
The CLI's default abstention behavior is not changed by this patch.

The paper, frozen RQ1 data, labels, harness generator and existing default analyzer
binary are unchanged. The candidate binary is installed separately as:

```
tools/stu_selector/analyzer/target/release/analyzer-graph-identity-20260917
```

## What changed

- Deduplicate and resolve calls by `hir::Function`, not bare display names.
  Preserve distinct same-named module functions and specialized impl methods;
  qualify their emitted IDs only when necessary.
- A resolved nonlocal symbol cannot alias a local node just because their names
  are the same (`std::mem::swap` was the motivating failure).
- Build actual direct-call relationships before projecting nested helpers onto
  their enclosing candidate. Preserve real nested/self/mutual recursion, but do
  not turn ordinary wrapper-to-helper calls into self-loops. Uncalled helper
  definitions do not contribute call edges.
- Preserve explicit same-crate linker relationships: an extern declaration can
  refer to a unique local definition exported by `no_mangle` or `export_name`;
  `link_name` takes precedence over its source spelling. C ABI alone, or a
  coincidentally identical Rust name, is insufficient.
- Keep noncandidate resolved sites as reserved `@nonlocal::...` / `@nested::...`
  targets. The topology reporting script now uses exact candidate IDs, like the
  matcher, instead of accidentally undoing qualification with a leaf-name fallback.

These rules have no application/tool-specific branches and never inspect C/Rust
correspondence labels. Node feature weights, return features and guard heuristics
were not changed.

## Verification

21 independent Rust frontend tests passed, including negative examples for
nonrecursive wrappers, foreign homonyms, mangled C-ABI functions, uncalled
helpers, and excluded test/trait owners; positive examples cover direct/nested/
mutual recursion, module and specialized-method homonyms, and explicit linker
aliases. Four Python tests passed for topology accounting and label ambiguity.

The additional existing `tests/test_stu.py` suite reports 100/101 passing. The
remaining bounded-scalar test asserts the old `%` code string, whereas the
unchanged generator already emits `rem_euclid`. Neither that test nor the
generator was changed; this is not claimed as a fully green repository test run.

Both archived cohorts were checked: 31 Panel A + 9 COMPLETE Panel B artifacts.
For all 40, the current artifact hash matches the archive, and the original
analyzer reproduces the archived accepted pair set. All artifacts were then
re-extracted with the corrected binary. After the linker-rule correction, cached
matching was reused only where the entire consumed function-record list and
local edge set were identical to the preceding run; changed inputs were rematched.
This equivalence check is independent of labels. Raw old/new outputs are retained.

This is a **retrospective regression audit**, not new held-out evidence. These
libraries have already been inspected during development. No settings were
selected per artifact, and no labels or cohort membership were changed.

## Results

Precision below uses the paper's macro averaging: outputs within application,
then applications equally. Counts are pooled. Micro precision is also retained
in the JSON, not substituted for macro precision.

| Cohort / version | True pairs | Accepted | Correct under existing labels | Unadjudicated | Macro precision |
|---|---:|---:|---:|---:|---:|
| A / archived implementation | 4041 | 2628 | 2620 | 0 | 99.283% |
| A / corrected frontend | 4041 | 2641 | 2631 | 2 | 99.306–99.322% |
| B / archived implementation | 161 | 73 | 65 | 0 | 72.435% |
| B / corrected frontend | 161 | 76 | 69 | 0 | 79.578% |

Panel B's renamed subset improves from 2/9 to 3/9. SACTOR qsort changes from
0 correct / 1 accepted to 3/3; PtrTrans qsort from 2/2 to 3/3. The other seven
Panel B artifacts have unchanged accepted pairs, including LodePNG's 0/1.

Panel A is not uniformly improved. Bzip2–Laertes gains one correct pair;
Tulip–Laertes gains thirteen; Optipng–Laertes loses four previously accepted
correct pairs and gains three others, for a net loss of one. We did not tune
parameters to recover that loss.

The two unadjudicated predictions are `fixedtables` in Optipng–C2Rust and
Optipng–C2SaferRust. The new frontend preserves two distinct Rust definitions
where the old frontend silently retained only one. The old label records only
a leaf name and cannot disambiguate them. These are **not automatically credited
as correct and not asserted to be false positives**. The lower precision bound
credits neither; the upper bound credits both. The original baseline count is
shown as recorded, not retrospectively advertised as collision-free ground truth.
Source-aware correspondence labels are needed before replacing paper numbers.

An intermediate version lost real quadtree call edges by treating every extern
declaration as external. This was corrected using explicit linker semantics and
new independent positive/negative tests, not a quadtree special case. The final
quadtree result is back to 21/21. The superseded audit is not a paper result.

## Scope and next steps

- No claim of a complete Rust call graph: indirect calls remain unresolved;
  macro/conditional symbol attributes and cross-crate linker interposition are
  not inferred. Direct-call reachability is not whole-program reachability.
- Qualified IDs distinguish candidates; they are not promises that every ID is
  a directly invocable harness path. No generator or frozen harness was changed.
- Do not activate this binary inside a frozen campaign without recording the
  analyzer version. The original default executable remains intact.
- Guard-insensitive features and independent return-shape weighting remain
  research directions, not production changes. Establish their behavior on new
  controlled examples before selecting rules/weights. Do not delete all Rust
  checks as presumed boilerplate: some checks reflect meaningful behavior.

## Evidence and reproduction

See [summary](../results/matcher_graph_identity_2026-09-17/summary.json),
[provenance](../results/matcher_graph_identity_2026-09-17/provenance.json), and
[raw outputs and compiled-source snapshot](../results/matcher_graph_identity_2026-09-17/evidence.tar.gz).

The noncached audit can be repeated into a **new** output directory:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 scripts/rq1_graph_identity_audit.py \
  --old-analyzer tools/stu_selector/analyzer/target/release/analyzer \
  --new-analyzer tools/stu_selector/analyzer/target/release/analyzer-graph-identity-20260917 \
  --out /tmp/matcher-identity-new-audit --workers 2
```

The runner refuses to write under frozen `results/rq1_matching`. Its `precision`
fields are lower bounds when `unadjudicated` is nonempty; read the corresponding
`precision_upper` fields too. This audit does not overwrite paper tables.

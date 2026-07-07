# UB-free re-scan of dataset v4 (selection rule 4 of 5)

Convention: a divergence is a real bug only on a UB-free input; UB-triggering divergences are EXCLUDED. Static re-label of recorded divergences (does NOT search for new UB-free divergences — that needs UB-free fuzzing). 134 boundaries; 37 previously 'invalid'.

| re-scanned class | n |
|---|--:|
| valid (UB-free testable, no divergence) | 73 |
| ub_excluded (divergence on UB-triggering input) | 37 |
| unchanged: excluded | 12 |
| unchanged: weak_exclude | 7 |
| unchanged: excluded_generator | 5 |

## Candidate real bugs (UB-free divergences)

- **None.** Every recorded divergence on the (faithful c2rust) corpus was UB-backed (UBSan/ASan/crash) → excluded under the UB-free rule. Expected: c2rust is faithful here; the only real bug we have is the INJECTED one in `g3_g2_bug` (separate). The 'invalid' class is entirely **UB-triggering-input artifacts of unfiltered fuzzing**, which is exactly what selection rule 4 (UB-free) is for.

## Reading

- Under the UB-free rule, all 37 previously-'invalid' boundaries re-classify as UB-excluded (not bugs). This CONFIRMS the corpus is faithful and that the old invalid labels were naive-fuzzing-hits-UB artifacts, not translation defects.
- UB-free testability is ONE rule (condition 4); the 24 excluded/weak boundaries are ruled out by the OTHER conditions (1 constructibility / 3 comparability), unchanged here.
- **Next (the real step):** UB-free FUZZING — make the harness reject UBSan/ASan-flagged inputs and keep fuzzing, so any surviving divergence is a UB-free real bug. This static re-scan only shows the recorded divergences were UB-backed; it cannot find new UB-free bugs.

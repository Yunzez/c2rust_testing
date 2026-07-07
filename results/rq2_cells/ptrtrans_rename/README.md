# PtrTrans renames — real-tool demonstration

PtrTrans (FSE'26) does NOT keep all C names — it renames some functions (camelCase→snake_case and
stubs). This is a SHIPPED tool where name-equality genuinely fails, so the matcher's name-independence
is needed on a real translator, not only on the synthetic raw-LLM set.

## Clean demonstration: qsort × PtrTrans
- C: `swap`, `partition`, `quickSort`
- PtrTrans Rust: `swap`, `partition`, **`quick_sort`** (quickSort was renamed)
- **name-equality recall = 0.667 (2/3)** — fails on quickSort↔quick_sort
- **matcher recall = 1.000 (3/3)** — recovers the renamed function by structure

## Corroboration at scale
On lodepng, PtrTrans keeps most C names verbatim but renames a subset (camelCase functions like
`addColorBits`, `advanceBits`, `encodeLZ77` → snake_case, plus stubs). We report the behavior
qualitatively ("PtrTrans renames"), not a precise fraction.

Files: `qsort_ptrtrans.rs` (verbatim PtrTrans output), `qsort_truth.json`.

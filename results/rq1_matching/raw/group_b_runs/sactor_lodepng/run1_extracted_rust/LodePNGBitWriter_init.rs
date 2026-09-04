// SACTOR unidiomatic translation of `LodePNGBitWriter_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:37:07; attempt 1). Verification verdict: rust compiled
unsafe fn LodePNGBitWriter_init(writer: *mut LodePNGBitWriter, data: *mut ucvector) {
    if writer.is_null() {
        return;
    }
    (*writer).data = data;
    (*writer).bp = 0;
}

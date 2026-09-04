// SACTOR unidiomatic translation of `updateHashChain` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:42:01; attempt 1). Verification verdict: rust compiled
pub unsafe fn updateHashChain(
    hash: *mut Hash,
    wpos: usize,
    hashval: u32,
    numzeros: u16,
) {
    *(*hash).val.add(wpos) = hashval as ::core::ffi::c_int;
    if *(*hash).head.add(hashval as usize) != -1 {
        *(*hash).chain.add(wpos) = *(*hash).head.add(hashval as usize)
            as ::core::ffi::c_ushort;
    }
    *(*hash).head.add(hashval as usize) = wpos as ::core::ffi::c_int;
    *(*hash).zeros.add(wpos) = numzeros as ::core::ffi::c_ushort;
    if *(*hash).headz.add(numzeros as usize) != -1 {
        *(*hash).chainz.add(wpos) = *(*hash).headz.add(numzeros as usize)
            as ::core::ffi::c_ushort;
    }
    *(*hash).headz.add(numzeros as usize) = wpos as ::core::ffi::c_int;
}

pub unsafe fn updateHashChain(hash: *mut Hash, wpos: usize, hashval: u32, numzeros: u16) {
    *(*hash).val.add(wpos) = hashval as ::core::ffi::c_int;
    if *(*hash).head.add(hashval as usize) != -1 {
        *(*hash).chain.add(wpos) = *(*hash).head.add(hashval as usize) as ::core::ffi::c_ushort;
    }
    *(*hash).head.add(hashval as usize) = wpos as ::core::ffi::c_int;
    *(*hash).zeros.add(wpos) = numzeros as ::core::ffi::c_ushort;
    if *(*hash).headz.add(numzeros as usize) != -1 {
        *(*hash).chainz.add(wpos) = *(*hash).headz.add(numzeros as usize) as ::core::ffi::c_ushort;
    }
    *(*hash).headz.add(numzeros as usize) = wpos as ::core::ffi::c_int;
}

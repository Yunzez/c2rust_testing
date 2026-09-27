// RQ4 coverage crate root for the c2rust bzip2 artifact.
// The nine module files are copied byte-identically from
// tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/ (== fuzz/bzip2_c2rust_e3/src/).
// The original c2rust-lib.rs uses feature gates removed from modern nightly
// (const_fn_fn_ptr_basics, ptr_offset_from, const_mut_refs); this root declares the same
// modules under the gates nightly-2025-09-01 still accepts.  No stub definitions of the
// macOS symbols live here — they come from shims.c so that the CLI actually works.
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut, internal_features,
         unused_imports)]

pub mod blocksort;
pub mod bzip2;
pub mod bzip2recover;
pub mod bzlib;
pub mod compress;
pub mod crctable;
pub mod decompress;
pub mod huffman;
pub mod randtable;

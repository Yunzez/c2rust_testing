fn main() {
    let mut build = cc::Build::new();
    build.compiler("clang").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp")
        .flag("-fsanitize=address,undefined")
        .flag("-fsanitize=float-cast-overflow,pointer-overflow,return,vla-bound")
        .flag("-fno-sanitize-recover=all").warnings(false);
    build
        .define("BZ2_blockSort", "c_BZ2_blockSort")
        .define("BZ2_bsInitWrite", "c_BZ2_bsInitWrite")
        .define("BZ2_bzBuffToBuffCompress", "c_BZ2_bzBuffToBuffCompress")
        .define("BZ2_bzBuffToBuffDecompress", "c_BZ2_bzBuffToBuffDecompress")
        .define("BZ2_bzCompress", "c_BZ2_bzCompress")
        .define("BZ2_bzCompressEnd", "c_BZ2_bzCompressEnd")
        .define("BZ2_bzCompressInit", "c_BZ2_bzCompressInit")
        .define("BZ2_bzDecompress", "c_BZ2_bzDecompress")
        .define("BZ2_bzDecompressEnd", "c_BZ2_bzDecompressEnd")
        .define("BZ2_bzDecompressInit", "c_BZ2_bzDecompressInit")
        .define("BZ2_bzRead", "c_BZ2_bzRead")
        .define("BZ2_bzReadClose", "c_BZ2_bzReadClose")
        .define("BZ2_bzReadGetUnused", "c_BZ2_bzReadGetUnused")
        .define("BZ2_bzReadOpen", "c_BZ2_bzReadOpen")
        .define("BZ2_bzWrite", "c_BZ2_bzWrite")
        .define("BZ2_bzWriteClose", "c_BZ2_bzWriteClose")
        .define("BZ2_bzWriteClose64", "c_BZ2_bzWriteClose64")
        .define("BZ2_bzWriteOpen", "c_BZ2_bzWriteOpen")
        .define("BZ2_bz__AssertH__fail", "c_BZ2_bz__AssertH__fail")
        .define("BZ2_bzclose", "c_BZ2_bzclose")
        .define("BZ2_bzdopen", "c_BZ2_bzdopen")
        .define("BZ2_bzerror", "c_BZ2_bzerror")
        .define("BZ2_bzflush", "c_BZ2_bzflush")
        .define("BZ2_bzlibVersion", "c_BZ2_bzlibVersion")
        .define("BZ2_bzopen", "c_BZ2_bzopen")
        .define("BZ2_bzread", "c_BZ2_bzread")
        .define("BZ2_bzwrite", "c_BZ2_bzwrite")
        .define("BZ2_compressBlock", "c_BZ2_compressBlock")
        .define("BZ2_decompress", "c_BZ2_decompress")
        .define("BZ2_hbAssignCodes", "c_BZ2_hbAssignCodes")
        .define("BZ2_hbCreateDecodeTables", "c_BZ2_hbCreateDecodeTables")
        .define("BZ2_hbMakeCodeLengths", "c_BZ2_hbMakeCodeLengths")
        .define("BZ2_indexIntoF", "c_BZ2_indexIntoF")
        .define("add_pair_to_block", "c_add_pair_to_block")
        .define("bsFinishWrite", "c_bsFinishWrite")
        .define("bsPutUChar", "c_bsPutUChar")
        .define("bsPutUInt32", "c_bsPutUInt32")
        .define("bsW", "c_bsW")
        .define("bz_config_ok", "c_bz_config_ok")
        .define("bzopen_or_bzdopen", "c_bzopen_or_bzdopen")
        .define("copy_input_until_stop", "c_copy_input_until_stop")
        .define("copy_output_until_stop", "c_copy_output_until_stop")
        .define("default_bzalloc", "c_default_bzalloc")
        .define("default_bzfree", "c_default_bzfree")
        .define("fallbackQSort3", "c_fallbackQSort3")
        .define("fallbackSimpleSort", "c_fallbackSimpleSort")
        .define("fallbackSort", "c_fallbackSort")
        .define("flush_RL", "c_flush_RL")
        .define("generateMTFValues", "c_generateMTFValues")
        .define("handle_compress", "c_handle_compress")
        .define("init_RL", "c_init_RL")
        .define("isempty_RL", "c_isempty_RL")
        .define("mainGtU", "c_mainGtU")
        .define("mainQSort3", "c_mainQSort3")
        .define("mainSimpleSort", "c_mainSimpleSort")
        .define("mainSort", "c_mainSort")
        .define("makeMaps_d", "c_makeMaps_d")
        .define("makeMaps_e", "c_makeMaps_e")
        .define("mmed3", "c_mmed3")
        .define("myfeof", "c_myfeof")
        .define("prepare_new_block", "c_prepare_new_block")
        .define("sendMTFValues", "c_sendMTFValues")
        .define("unRLE_obuf_to_output_FAST", "c_unRLE_obuf_to_output_FAST")
        .define("unRLE_obuf_to_output_SMALL", "c_unRLE_obuf_to_output_SMALL")
        .define("BZ2_crc32Table", "c_BZ2_crc32Table")
        .define("BZ2_rNums", "c_BZ2_rNums");
    build.file("c/bzip2lib.c");
    build.file("c/ubshim.c");
    build.file("c/shims.c");
    build.compile("c_oracle");
    println!("cargo:rustc-link-arg=-Wl,-u,__maskrune");
    println!("cargo:rustc-link-arg=-Wl,-u,_DefaultRuneLocale");
    let rd = std::process::Command::new("clang").arg("--print-resource-dir").output().unwrap();
    let rd = String::from_utf8(rd.stdout).unwrap().trim().to_string();
    let lib_dir = std::path::Path::new(&rd).join("lib").join("linux");
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "x86_64".into());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=clang_rt.profile-{}", arch);
}

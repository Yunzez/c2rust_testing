fn main() {
    let mut build = cc::Build::new();
    build.compiler("clang").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp")
        .flag("-fsanitize=address,undefined")
        .flag("-fsanitize=float-cast-overflow,pointer-overflow,return,vla-bound")
        .flag("-fno-sanitize-recover=all").warnings(false);
    build
        .define("genann_act_hidden_indirect", "c_genann_act_hidden_indirect")
        .define("genann_act_linear", "c_genann_act_linear")
        .define("genann_act_output_indirect", "c_genann_act_output_indirect")
        .define("genann_act_sigmoid", "c_genann_act_sigmoid")
        .define("genann_act_sigmoid_cached", "c_genann_act_sigmoid_cached")
        .define("genann_act_threshold", "c_genann_act_threshold")
        .define("genann_copy", "c_genann_copy")
        .define("genann_free", "c_genann_free")
        .define("genann_init", "c_genann_init")
        .define("genann_init_sigmoid_lookup", "c_genann_init_sigmoid_lookup")
        .define("genann_randomize", "c_genann_randomize")
        .define("genann_read", "c_genann_read")
        .define("genann_run", "c_genann_run")
        .define("genann_train", "c_genann_train")
        .define("genann_write", "c_genann_write")
        .define("sigmoid_dom_max", "c_sigmoid_dom_max")
        .define("sigmoid_dom_min", "c_sigmoid_dom_min");
    build.file("c/genann.c");
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

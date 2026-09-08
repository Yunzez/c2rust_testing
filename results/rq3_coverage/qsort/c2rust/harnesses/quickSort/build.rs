fn main() {
    let mut build = cc::Build::new();
    build.compiler("clang").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp")
        .flag("-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable,float-cast-overflow,pointer-overflow,return,vla-bound")
        .flag("-fsanitize-recover=all")
        .flag("-fsanitize-minimal-runtime")
        .flag("-fno-sanitize-link-runtime").warnings(false);
    build
        .define("partition", "c_partition")
        .define("quickSort", "c_quickSort")
        .define("swap", "c_swap");
    build.file("c/qsort.c");
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

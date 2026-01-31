fn main() {
    // Build the C oracle with clang sanitizer-coverage instrumentation.
    let mut build = cc::Build::new();
    build
        .compiler("clang")
        .flag("-O1")
        .flag("-g")
        .flag("-fsanitize-coverage=trace-pc-guard,trace-cmp")
        .warnings(false);

    // Rename C symbols to avoid collisions with Rust-transpiled functions.
    build
        .define("quickSort", "c_quickSort")
        .define("partition", "c_partition")
        .define("swap", "c_swap");

    build.file("c/qsort.c");
    build.compile("c_oracle");

    // Link Clang's coverage runtime so sanitizer coverage symbols resolve.
    let resource_dir = std::process::Command::new("clang")
        .arg("--print-resource-dir")
        .output()
        .expect("failed to run clang --print-resource-dir");
    if !resource_dir.status.success() {
        panic!("clang --print-resource-dir failed");
    }
    let resource_dir = String::from_utf8(resource_dir.stdout)
        .expect("clang resource dir is not valid UTF-8")
        .trim()
        .to_string();
    let lib_dir = std::path::Path::new(&resource_dir).join("lib").join("linux");

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "x86_64".to_string());
    let rt_name = format!("clang_rt.profile-{}", arch);

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static={}", rt_name);
}

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
        .define("url_parse", "c_url_parse")
        .define("url_free", "c_url_free")
        .define("url_get_protocol", "c_url_get_protocol")
        .define("url_get_host", "c_url_get_host")
        .define("url_get_hostname", "c_url_get_hostname")
        .define("url_get_path", "c_url_get_path")
        .define("url_get_query", "c_url_get_query")
        .define("url_get_hash", "c_url_get_hash")
        .define("url_get_port", "c_url_get_port")
        .define("url_get_auth", "c_url_get_auth")
        .define("url_get_pathname", "c_url_get_pathname")
        .define("url_get_search", "c_url_get_search")
        // Also rename global and helper symbols that collide with Rust-transpiled ones.
        .define("URL_SCHEMES", "C_URL_SCHEMES")
        .define("strdup", "c_strdup")
        .define("url_is_protocol", "c_url_is_protocol")
        .define("url_is_ssh", "c_url_is_ssh")
        .define("url_inspect", "c_url_inspect")
        .define("url_data_inspect", "c_url_data_inspect");

    // Add all C sources.
    for file in [
        "c/url.c",
    ] {
        build.file(file);
    }

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

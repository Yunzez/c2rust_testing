fn main() {
    let mut build = cc::Build::new();
    build.compiler("clang").include("c").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp")
        .flag("-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable,float-cast-overflow,pointer-overflow,return,vla-bound")
        .flag("-fsanitize-recover=all")
        .flag("-fsanitize-minimal-runtime")
        .flag("-fno-sanitize-link-runtime").warnings(false);
    build
        .define("get_part", "c_get_part")
        .define("main", "c_main")
        .define("strdup", "c_strdup")
        .define("strff", "c_strff")
        .define("strrwd", "c_strrwd")
        .define("url_data_inspect", "c_url_data_inspect")
        .define("url_free", "c_url_free")
        .define("url_get_auth", "c_url_get_auth")
        .define("url_get_hash", "c_url_get_hash")
        .define("url_get_host", "c_url_get_host")
        .define("url_get_hostname", "c_url_get_hostname")
        .define("url_get_path", "c_url_get_path")
        .define("url_get_pathname", "c_url_get_pathname")
        .define("url_get_port", "c_url_get_port")
        .define("url_get_protocol", "c_url_get_protocol")
        .define("url_get_query", "c_url_get_query")
        .define("url_get_search", "c_url_get_search")
        .define("url_inspect", "c_url_inspect")
        .define("url_is_protocol", "c_url_is_protocol")
        .define("url_is_ssh", "c_url_is_ssh")
        .define("url_parse", "c_url_parse")
        .define("URL_SCHEMES", "c_URL_SCHEMES");
    build.file("c/test.c");
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

fn main() {
    let mut build = cc::Build::new();
    build.compiler("clang").include("c").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp")
        .flag("-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable,float-cast-overflow,pointer-overflow,return,vla-bound")
        .flag("-fsanitize-recover=all")
        .flag("-fsanitize-minimal-runtime")
        .flag("-fno-sanitize-link-runtime").warnings(false);
    build
        .define("elision_", "c_elision_")
        .define("find_", "c_find_")
        .define("get_quadrant_", "c_get_quadrant_")
        .define("insert_", "c_insert_")
        .define("node_contains_", "c_node_contains_")
        .define("quadtree_bounds_extend", "c_quadtree_bounds_extend")
        .define("quadtree_bounds_free", "c_quadtree_bounds_free")
        .define("quadtree_bounds_new", "c_quadtree_bounds_new")
        .define("quadtree_free", "c_quadtree_free")
        .define("quadtree_insert", "c_quadtree_insert")
        .define("quadtree_new", "c_quadtree_new")
        .define("quadtree_node_free", "c_quadtree_node_free")
        .define("quadtree_node_isempty", "c_quadtree_node_isempty")
        .define("quadtree_node_isleaf", "c_quadtree_node_isleaf")
        .define("quadtree_node_ispointer", "c_quadtree_node_ispointer")
        .define("quadtree_node_new", "c_quadtree_node_new")
        .define("quadtree_node_reset", "c_quadtree_node_reset")
        .define("quadtree_node_with_bounds", "c_quadtree_node_with_bounds")
        .define("quadtree_point_free", "c_quadtree_point_free")
        .define("quadtree_point_new", "c_quadtree_point_new")
        .define("quadtree_search", "c_quadtree_search")
        .define("quadtree_walk", "c_quadtree_walk")
        .define("reset_node_", "c_reset_node_")
        .define("split_node_", "c_split_node_");
    build.file("c/quadtree_all.c");
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

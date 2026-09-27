fn main() {
    let mut build = cc::Build::new();
    build.compiler("clang").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp")
        .flag("-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable,float-cast-overflow,pointer-overflow,return,vla-bound")
        .flag("-fsanitize-recover=all")
        .flag("-fsanitize-minimal-runtime")
        .flag("-fno-sanitize-link-runtime").warnings(false);
    build
        .define("cJSON_AddItemReferenceToArray", "c_cJSON_AddItemReferenceToArray")
        .define("cJSON_AddItemReferenceToObject", "c_cJSON_AddItemReferenceToObject")
        .define("cJSON_AddItemToArray", "c_cJSON_AddItemToArray")
        .define("cJSON_AddItemToObject", "c_cJSON_AddItemToObject")
        .define("cJSON_AddItemToObjectCS", "c_cJSON_AddItemToObjectCS")
        .define("cJSON_CreateArray", "c_cJSON_CreateArray")
        .define("cJSON_CreateBool", "c_cJSON_CreateBool")
        .define("cJSON_CreateDoubleArray", "c_cJSON_CreateDoubleArray")
        .define("cJSON_CreateFalse", "c_cJSON_CreateFalse")
        .define("cJSON_CreateFloatArray", "c_cJSON_CreateFloatArray")
        .define("cJSON_CreateIntArray", "c_cJSON_CreateIntArray")
        .define("cJSON_CreateNull", "c_cJSON_CreateNull")
        .define("cJSON_CreateNumber", "c_cJSON_CreateNumber")
        .define("cJSON_CreateObject", "c_cJSON_CreateObject")
        .define("cJSON_CreateString", "c_cJSON_CreateString")
        .define("cJSON_CreateStringArray", "c_cJSON_CreateStringArray")
        .define("cJSON_CreateTrue", "c_cJSON_CreateTrue")
        .define("cJSON_Delete", "c_cJSON_Delete")
        .define("cJSON_DeleteItemFromArray", "c_cJSON_DeleteItemFromArray")
        .define("cJSON_DeleteItemFromObject", "c_cJSON_DeleteItemFromObject")
        .define("cJSON_DetachItemFromArray", "c_cJSON_DetachItemFromArray")
        .define("cJSON_DetachItemFromObject", "c_cJSON_DetachItemFromObject")
        .define("cJSON_Duplicate", "c_cJSON_Duplicate")
        .define("cJSON_GetArrayItem", "c_cJSON_GetArrayItem")
        .define("cJSON_GetArraySize", "c_cJSON_GetArraySize")
        .define("cJSON_GetErrorPtr", "c_cJSON_GetErrorPtr")
        .define("cJSON_GetObjectItem", "c_cJSON_GetObjectItem")
        .define("cJSON_InitHooks", "c_cJSON_InitHooks")
        .define("cJSON_InsertItemInArray", "c_cJSON_InsertItemInArray")
        .define("cJSON_Minify", "c_cJSON_Minify")
        .define("cJSON_New_Item", "c_cJSON_New_Item")
        .define("cJSON_Parse", "c_cJSON_Parse")
        .define("cJSON_ParseWithOpts", "c_cJSON_ParseWithOpts")
        .define("cJSON_Print", "c_cJSON_Print")
        .define("cJSON_PrintBuffered", "c_cJSON_PrintBuffered")
        .define("cJSON_PrintUnformatted", "c_cJSON_PrintUnformatted")
        .define("cJSON_ReplaceItemInArray", "c_cJSON_ReplaceItemInArray")
        .define("cJSON_ReplaceItemInObject", "c_cJSON_ReplaceItemInObject")
        .define("cJSON_strcasecmp", "c_cJSON_strcasecmp")
        .define("cJSON_strdup", "c_cJSON_strdup")
        .define("create_reference", "c_create_reference")
        .define("ensure", "c_ensure")
        .define("parse_array", "c_parse_array")
        .define("parse_hex4", "c_parse_hex4")
        .define("parse_number", "c_parse_number")
        .define("parse_object", "c_parse_object")
        .define("parse_string", "c_parse_string")
        .define("parse_value", "c_parse_value")
        .define("pow2gt", "c_pow2gt")
        .define("print_array", "c_print_array")
        .define("print_number", "c_print_number")
        .define("print_object", "c_print_object")
        .define("print_string", "c_print_string")
        .define("print_string_ptr", "c_print_string_ptr")
        .define("print_value", "c_print_value")
        .define("skip", "c_skip")
        .define("suffix_object", "c_suffix_object")
        .define("update", "c_update");
    build.file("c/cJSON.c");
    build.file("c/ubshim.c");
    build.file("c/c2r_plugin.c");
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

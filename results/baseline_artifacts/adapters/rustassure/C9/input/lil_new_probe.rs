#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/lil_new_probe_raw.inc"];

#[no_mangle]
pub unsafe extern "C" fn lil_new_probe() -> i32 {
    let value = lil_new();
    let result = (!value.is_null()) as i32;
    if !value.is_null() {
        lil_free(value);
    }
    result
}

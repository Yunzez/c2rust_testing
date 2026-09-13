#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/lil_parse_generated_raw.inc"];

#[no_mangle]
pub fn lil_parse_generated(script: &mut [i8; 64]) -> i32 {
    script[63] = 0;
    unsafe {
        let lil = lil_new();
        if lil.is_null() {
            return 0;
        }
        let value = lil_parse(lil, script.as_ptr(), 0, 0);
        let result = (!value.is_null()) as i32;
        if !value.is_null() {
            lil_free_value(value);
        }
        lil_free(lil);
        result
    }
}

include!["/input/parse_string_valid_raw.inc"];


extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(pointer: *mut core::ffi::c_void);
}

#[no_mangle]
pub fn parse_string_valid(input: &mut [i8; 32]) -> i32 {
    input[0] = b'"' as i8;
    input[30] = b'"' as i8;
    input[31] = 0;
    let mut content = [0u8; 32];
    for (dst, src) in content.iter_mut().zip(input.iter()) {
        *dst = *src as u8;
    }
    let allocate = |size: usize| unsafe { malloc(size) };
    let deallocate = |pointer: *mut core::ffi::c_void| unsafe { free(pointer) };
    let mut item = cJSON {
        next: None,
        prev: None,
        child: None,
        type_: 0,
        valuestring: None,
        valueint: 0,
        valuedouble: 0.0,
        string: None,
    };
    let mut buffer = ParseBuffer {
        content: Some(&content[..31]),
        length: 31,
        offset: 0,
        depth: 0,
        hooks: InternalHooks {
            allocate: Some(&allocate),
            deallocate: Some(&deallocate),
            reallocate: None,
        },
    };
    parse_string(Some(&mut item), Some(&mut buffer))
}


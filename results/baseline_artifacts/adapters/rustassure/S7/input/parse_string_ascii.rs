include!["/input/parse_string_ascii_raw.inc"];


extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(pointer: *mut core::ffi::c_void);
}

#[no_mangle]
pub fn parse_string_ascii(input: &mut [i8; 32]) -> i32 {
    let mut content = [0u8; 32];
    content[0] = b'"';
    for index in 1..30 {
        content[index] = 32 + ((input[index] as u8) % 95);
    }
    content[30] = b'"';
    content[31] = 0;
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


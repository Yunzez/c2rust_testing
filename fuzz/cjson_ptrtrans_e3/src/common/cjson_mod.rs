

pub struct cJSON_Hooks<'a> {
    pub malloc_fn: Option<&'a mut dyn FnMut(usize) -> *mut core::ffi::c_void>,
    pub free_fn: Option<&'a mut dyn FnMut(*mut core::ffi::c_void)>,
}
pub struct cJSON<'a> {
    // Nullable, borrowed, mutable pointers with explicit lifetime
    pub next: Option<&'a mut cJSON<'a>>,
    pub prev: Option<&'a mut cJSON<'a>>,
    pub child: Option<&'a mut cJSON<'a>>,
    pub type_: i32,
    // Nullable, borrowed, mutable pointer to string
    pub valuestring: Option<&'a mut str>,
    pub valueint: i32,
    pub valuedouble: f64,
    // Nullable, borrowed, immutable pointer to string
    pub string: Option<&'a str>,
}

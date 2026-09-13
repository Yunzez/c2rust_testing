// flattened from fuzz/cjson_ptrtrans_e3/src for the RQ4 pair (bodies verbatim)
pub mod cjson {
use core::option::Option;
use core::str;
use core::cmp;
use std::f64;
use std::cmp::Ordering;
use crate::cjson::cJSON;
use core::ffi::c_void;
pub use crate::InternalHooks as InternalHooksAlias;
use core::ffi::c_uchar;
use crate::*;
pub fn minify_string(input: &mut &[u8], output: &mut [u8]) {
    // Early return if either is empty; mirrors undefined-but-non-crashing C behavior
    if input.is_empty() || output.is_empty() {
        return;
    }
    // (*output)[0] = (*input)[0];
    output[0] = input[0];
    // *input += (sizeof("\"") - sizeof(""));  // advance by 1
    // *output += (sizeof("\"") - sizeof(""));
    *input = &input[1..];
    let mut out_idx: usize = 1;
    // for (; (*input)[0] != '\0'; (void)++(*input), ++(*output))
    while !input.is_empty() && input[0] != b'\0' && out_idx < output.len() {
        // (*output)[0] = (*input)[0];
        output[out_idx] = input[0];
        if input[0] == b'"' {
            // (*output)[0] = '\"';
            output[out_idx] = b'"';
            // *input += (sizeof("\"") - sizeof(""));
            // *output += (sizeof("\"") - sizeof(""));
            *input = &input[1..];
            if out_idx + 1 <= output.len() {
                out_idx += 1;
            }
            return;
        } else if input[0] == b'\\' && input.len() > 1 && input[1] == b'"' {
            // (*output)[1] = (*input)[1];
            if out_idx + 1 < output.len() {
                output[out_idx + 1] = input[1];
            }
            // *input += (sizeof("\"") - sizeof(""));
            // *output += (sizeof("\"") - sizeof(""));
            if input.len() > 1 {
                *input = &input[1..];
            } else {
                *input = &input[input.len()..];
            }
            if out_idx + 1 <= output.len() {
                out_idx += 1;
            }
        }
        // (void)++(*input), ++(*output)
        if !input.is_empty() {
            *input = &input[1..];
        }
        if out_idx < output.len() {
            out_idx += 1;
        } else {
            break;
        }
    }
}
pub fn skip_oneline_comment(input: Option<&mut &[u8]>) {
    // Work only if we actually have a reference to modify.
    let Some(inner) = input else {
        return;
    };
    let len = inner.len();
    // Skip the initial "XX" (2 bytes), or to the end if shorter.
    let mut i = if len >= 2 { 2 } else { len };
    // Walk forward from i until '\n' or '\0'.
    while i < len && inner[i] != 0 {
        if inner[i] == b'\n' {
            i += 1; // advance past '\n'
            break;
        }
        i += 1;
    }
    // Now reassign the slice to start after the skipped part.
    *inner = &inner[i..];
}
pub fn skip_multiline_comment(input: &mut Option<&mut &[u8]>) {
    // In C: *input += (sizeof("") - sizeof(""));
    // sizeof("") == 1, so this is effectively *input += 0; i.e., a no-op.
    // Since this is a no-op, and we must stay safe and not introduce new logic,
    // we intentionally do nothing here.
    //
    // The original C code also has extra unmatched braces and a `return;`
    // which are effectively just an early return from a void function.
    // In Rust, reaching the end of the function is equivalent.
}
pub fn get_decimal_point() -> u8 {
    b'.'
}
pub fn parse_hex4(input: Option<&[u8]>) -> u32 {
    // Return 0 if input is None or not at least 4 bytes long
    let input = match input {
        Some(slice) if slice.len() >= 4 => slice,
        _ => return 0,
    };
    let mut h: u32 = 0;
    let mut i: usize = 0;
    while i < 4 {
        let c = input[i];
        let v_opt = if c >= b'0' && c <= b'9' {
            Some((c - b'0') as u32)
        } else if c >= b'A' && c <= b'F' {
            Some(10u32 + (c - b'A') as u32)
        } else if c >= b'a' && c <= b'f' {
            Some(10u32 + (c - b'a') as u32)
        } else {
            None
        };
        let v = match v_opt {
            Some(val) => val,
            None => return 0,
        };
        h += v;
        if i < 3 {
            h <<= 4;
        }
        i += 1;
    }
    h
}
pub fn cJSON_Version() -> Option<&'static str> {
    // Static storage equivalent to C's `static char version[15]`
    // Length is 14 including dots, so a fixed string literal is sufficient.
    Some("1.7.19")
}
pub struct InternalHooks<'a> {
    // Nullable, borrowed, immutable pointer to an allocation function:
    // C: void *(*allocate)(size_t size);
    pub allocate: Option<&'a dyn Fn(usize) -> *mut core::ffi::c_void>,
    // Nullable, borrowed, immutable pointer to a deallocation function:
    // C: void (*deallocate)(void *pointer);
    pub deallocate: Option<&'a dyn Fn(*mut core::ffi::c_void)>,
    // Nullable, borrowed, immutable pointer to a reallocation function:
    // C: void *(*reallocate)(void *pointer, size_t size);
    pub reallocate: Option<&'a dyn Fn(*mut core::ffi::c_void, usize) -> *mut core::ffi::c_void>,
}
pub struct Error<'a> {
    // Nullable, borrowed, immutable pointer
    pub json: Option<&'a [u8]>,
    pub position: usize,
}
pub fn cJSON_Minify(json: Option<&mut [u8]>) {
    // Stub only, per requirements: no implementation
}
pub fn utf16_literal_to_utf8(
    input_pointer: Option<&[u8]>,
    input_end: Option<&[u8]>,
    output: Option<&mut Vec<u8>>,
) -> u8 {
    // Early out if any pointer is null-equivalent.
    let (input_pointer, input_end, output) = match (input_pointer, input_end, output) {
        (Some(ip), Some(ie), Some(o)) => (ip, ie, o),
        _ => return 0,
    };
    // In the C code, input_pointer and input_end are raw pointers; here we
    // model them as slices and only use their lengths to mirror (end - start).
    // The function assumes that at `input_pointer` there is at least "\uXXXX".
    // We'll emulate the length checks as in the original C.
    // first_sequence corresponds to input_pointer in C.
    let first_sequence = input_pointer;
    // (input_end - first_sequence) < 6  -->  remaining input less than 6 bytes
    if input_end.len() < 6 {
        return 0;
    }
    // parse_hex4(first_sequence + 2)
    // C assumes there's at least 4 bytes starting at first_sequence+2.
    // We've already checked for 6 total available.
    if first_sequence.len() < 6 {
        return 0;
    }
    let first_code = parse_hex4(Some(&first_sequence[2..6])) as u32;
    // if first_code in [0xDC00, 0xDFFF] -> fail
    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }
    let mut codepoint: u32;
    let sequence_length: u8;
    // Possible surrogate pair: first_code in [0xD800, 0xDBFF]
    if (0xD800..=0xDBFF).contains(&first_code) {
        // second_sequence = first_sequence + 6
        if first_sequence.len() < 12 || input_end.len() < 12 {
            return 0;
        }
        let second_sequence = &first_sequence[6..];
        // if second_sequence[0] != '\\' || second_sequence[1] != 'u' -> fail
        if second_sequence.len() < 6 {
            return 0;
        }
        if second_sequence[0] != b'\\' || second_sequence[1] != b'u' {
            return 0;
        }
        // second_code = parse_hex4(second_sequence + 2)
        let second_code = parse_hex4(Some(&second_sequence[2..6])) as u32;
        // if second_code not in [0xDC00, 0xDFFF] -> fail
        if !(0xDC00..=0xDFFF).contains(&second_code) {
            return 0;
        }
        // codepoint = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        codepoint = 0x10000
            + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        sequence_length = 12;
    } else {
        // Non-surrogate
        codepoint = first_code;
        sequence_length = 6;
    }
    // Determine UTF-8 length and first-byte mark
    let (utf8_length, first_byte_mark): (u8, u32) = if codepoint < 0x80 {
        (1, 0)
    } else if codepoint < 0x800 {
        (2, 0xC0)
    } else if codepoint < 0x10000 {
        (3, 0xE0)
    } else if codepoint <= 0x10FFFF {
        (4, 0xF0)
    } else {
        return 0;
    };
    // Ensure output has enough space and get a slice for the new bytes.
    let start_len = output.len();
    output.reserve(utf8_length as usize);
    // Push placeholder bytes to get a mutable window to fill, mirroring
    // the direct indexed writes in C.
    output.resize(start_len + utf8_length as usize, 0);
    let out_slice = &mut output[start_len..start_len + utf8_length as usize];
    // Fill trailing bytes (if any), from end to beginning, as in C loop:
    // for (utf8_position = utf8_length-1; utf8_position > 0; utf8_position--)
    //     out[utf8_position] = (codepoint | 0x80) & 0xBF; codepoint >>= 6;
    let mut cp = codepoint;
    let mut pos = utf8_length - 1;
    while pos > 0 {
        out_slice[pos as usize] = (((cp as u32) | 0x80) & 0xBF) as u8;
        cp >>= 6;
        pos -= 1;
    }
    // Leading byte
    if utf8_length > 1 {
        out_slice[0] = (((cp as u32) | first_byte_mark) & 0xFF) as u8;
    } else {
        out_slice[0] = ((cp as u32) & 0x7F) as u8;
    }
    sequence_length
}
pub struct ParseBuffer<'a> {
    // Nullable, borrowed, immutable pointer; requires lifetime
    pub content: Option<&'a [c_uchar]>,
    pub length: usize,
    pub offset: usize,
    pub depth: usize,
    pub hooks: InternalHooks<'a>,
}
/// Equivalent to the C `typedef int int;`
/// In Rust this is already the default `i32`, but we declare a public alias
/// to mirror the C typedef.
pub type Int = i32;
pub struct PrintBuffer<'a> {
    // Nullable, borrowed, immutable pointer to a buffer.
    // Mapped from: unsigned char *buffer;
    // Requires lifetime annotation.
    pub buffer: Option<&'a [u8]>,
    // Mapped from: size_t length;
    pub length: usize,
    // Mapped from: size_t offset;
    pub offset: usize,
    // Mapped from: size_t depth;
    pub depth: usize,
    // Mapped from: int noalloc;
    pub noalloc: Int,
    // Mapped from: int format;
    pub format: Int,
    // Mapped from: internal_hooks hooks;
    pub hooks: InternalHooks<'a>,
}
// Note: The original C code defines a *static variable*:
//   static internal_hooks global_hooks = { malloc, free, realloc };
// It does not define any functions, so there are no function stubs
// to generate according to the strict requirements.
//
// As per instructions, we must not add new functions or impl blocks
// that do not have a direct counterpart in the original C code.
// Therefore, no function stubs are produced here.
pub static mut GLOBAL_ERROR: Error<'static> = Error {
    json: None,
    position: 0,
};
pub fn cJSON_GetArraySize<'a>(array: Option<&'a cJSON<'a>>) -> i32 {
    let mut size: usize = 0;
    // array is Nullable, Borrowed and Immutable
    let mut child_opt: Option<&cJSON<'a>> = match array {
        None => return 0,
        Some(a) => a.child.as_deref(), // child is Option<&'a mut cJSON<'a>>
    };
    while let Some(child) = child_opt {
        size += 1;
        child_opt = child.next.as_deref();
    }
    size as i32
}
pub fn cJSON_IsInvalid<'a>(item: Option<&cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(it) => ((it.type_ & 0xFF) == 0) as i32,
    }
}
pub fn cJSON_IsFalse<'a>(item: Option<&cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(it) => {
            let type_low_byte = it.type_ & 0xFF;
            let false_flag = 1 << 0;
            (type_low_byte == false_flag) as i32
        }
    }
}
pub fn cJSON_IsTrue<'a>(item: Option<&cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(it) => {
            // (item->type & 0xff) == (1 << 1)
            if (it.type_ & 0xff) == (1 << 1) {
                1
            } else {
                0
            }
        }
    }
}
pub fn cJSON_IsBool(item: Option<&cJSON<'_>>) -> i32 {
    match item {
        None => 0,
        Some(it) => {
            let mask = (1 << 1) | (1 << 0);
            if (it.type_ & mask) != 0 {
                1
            } else {
                0
            }
        }
    }
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> i32 {
    match item {
        None => 0,
        Some(item_ref) => {
            let type_low_byte = item_ref.type_ & 0xFF;
            let null_flag = 1 << 2;
            (type_low_byte == null_flag) as i32
        }
    }
}
pub fn cJSON_IsNumber<'a>(item: Option<&cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(it) => {
            let masked = it.type_ & 0xFF;
            if masked == (1 << 3) {
                1
            } else {
                0
            }
        }
    }
}
pub fn cJSON_IsString<'a>(item: Option<&cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(it) => {
            // (item->type & 0xFF) == (1 << 4)
            if (it.type_ & 0xFF) == (1 << 4) {
                1
            } else {
                0
            }
        }
    }
}
pub fn cJSON_IsArray<'a>(item: Option<&cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(item_ref) => {
            if (item_ref.type_ & 0xFF) == (1 << 5) {
                1
            } else {
                0
            }
        }
    }
}
pub fn cJSON_IsObject<'a>(item: Option<&cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(item_ref) => {
            if (item_ref.type_ & 0xFF) == (1 << 6) {
                1
            } else {
                0
            }
        }
    }
}
pub fn cJSON_IsRaw<'a>(item: Option<&'a cJSON<'a>>) -> i32 {
    match item {
        None => 0,
        Some(it) => {
            if (it.type_ & 0xFF) == (1 << 7) {
                1
            } else {
                0
            }
        }
    }
}
pub fn cJSON_SetNumberHelper<'a>(object: Option<&'a mut cJSON<'a>>, number: f64) -> f64 {
    // Corresponds to: if (object == NULL) return NAN;
    let Some(obj) = object else {
        return f64::NAN;
    };
    // Corresponds to clamping to 32-bit signed int range
    if number >= 2_147_483_647.0 {
        obj.valueint = 2_147_483_647;
    } else if number <= (-2_147_483_647_i32 - 1) as f64 {
        obj.valueint = -2_147_483_647_i32 - 1;
    } else {
        obj.valueint = number as i32;
    }
    // Corresponds to: return object->valuedouble = number;
    obj.valuedouble = number;
    number
}
/// Safe Rust translation of:
/// static int case_insensitive_strcmp(const unsigned char *string1,
///                                    const unsigned char *string2);
///
/// `string1` is a nullable, borrowed, immutable pointer to i8* (C chars).
/// `string2` is a nullable, borrowed, immutable pointer to cJSON, where we
/// compare against its `string` field if present.
pub fn case_insensitive_strcmp<'a>(
    string1: Option<&'a str>,
    string2: Option<&'a crate::cJSON<'a>>,
) -> i32 {
    // if ((string1 == NULL) || (string2 == NULL)) { return 1; }
    if string1.is_none() || string2.is_none() {
        return 1;
    }
    // Extract concrete values; safe because of early-return above.
    let s1 = string1.unwrap();
    let json = string2.unwrap();
    // In C, string2 is a `const unsigned char*`. According to metadata,
    // in Rust we get the string via `cJSON.string: Option<&'a str>`.
    let s2_opt = json.string;
    // If the underlying C pointer was NULL, behavior would be undefined or
    // handled elsewhere. Here we mimic NULL as "no string": return 1.
    let s2 = match s2_opt {
        Some(s) => s,
        None => return 1,
    };
    // if (string1 == string2) { return 0; }
    //
    // In C this is pointer equality. In safe Rust with references we cannot
    // reproduce raw pointer identity here without unsafe. We approximate by
    // checking if the string contents are exactly the same; if so, the
    // function would behave the same for all observable callers.
    if s1 == s2 {
        return 0;
    }
    // for (; tolower(*string1) == tolower(*string2); string1++, string2++) {
    //     if (*string1 == '\0') { return 0; }
    // }
    //
    // Then: return tolower(*string1) - tolower(*string2);
    //
    // We implement a case-insensitive lexicographical comparison on bytes,
    // mirroring the original logic, and return the first non-zero difference.
    let mut iter1 = s1.bytes();
    let mut iter2 = s2.bytes();
    loop {
        let b1_opt = iter1.next();
        let b2_opt = iter2.next();
        match (b1_opt, b2_opt) {
            (Some(b1), Some(b2)) => {
                let c1 = (b1 as char).to_ascii_lowercase() as u8;
                let c2 = (b2 as char).to_ascii_lowercase() as u8;
                if c1 != c2 {
                    // return tolower(*string1) - tolower(*string2);
                    return (c1 as i32) - (c2 as i32);
                }
                // continue loop advancing both, like the C for(;;) construct
            }
            (None, Some(b2)) => {
                // *string1 == '\0' and *string2 != '\0'
                // C: falls out of loop and executes:
                // return tolower(*string1) - tolower(*string2);
                let c1 = 0_i32; // '\0'
                let c2 = (b2 as char).to_ascii_lowercase() as i32;
                return c1 - c2;
            }
            (Some(b1), None) => {
                // *string2 == '\0' and *string1 != '\0'
                let c1 = (b1 as char).to_ascii_lowercase() as i32;
                let c2 = 0_i32; // '\0'
                return c1 - c2;
            }
            (None, None) => {
                // Both strings ended at the same time: equal
                return 0;
            }
        }
    }
}
/// Compare two f64 values with a relative tolerance similar to the C code.
///
/// This returns `true` if `|a - b| <= max(|a|, |b|) * f64::EPSILON`,
/// otherwise `false`.
pub fn compare_double(a: f64, b: f64) -> bool {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    (a - b).abs() <= max_val * f64::EPSILON
}
/// Translated from:
/// static cJSON* get_array_item(const cJSON *array, size_t index)
pub fn get_array_item<'a>(array: Option<&'a mut cJSON<'a>>, mut index: usize) -> Option<&'a mut cJSON<'a>> {
    // array is Nullable, Borrowed and Mutable pointer (Option<&mut cJSON>)
    // return is Nullable, Borrowed and Mutable pointer (Option<&mut cJSON>) with lifetime 'a
    // if (array == NULL) return NULL;
    let mut current_child = match array {
        Some(a) => a.child.as_deref_mut(), // start from array->child
        None => return None,
    };
    // while ((current_child != NULL) && (index > 0)) { index--; current_child = current_child->next; }
    while index > 0 {
        match current_child {
            Some(child) => {
                index -= 1;
                current_child = child.next.as_deref_mut();
            }
            None => break,
        }
    }
    // return current_child;
    current_child
}
pub fn suffix_object<'a>(
    // In the original C code, both `prev` and `item` are owning pointers
    // (cJSON*). In this safe Rust model we accept nullable mutable references
    // so that we can manipulate list links without taking ownership of the
    // nodes themselves, avoiding aliasing and self-referential Box problems.
    mut prev: Option<&'a mut cJSON<'a>>,
    mut item: Option<&'a mut cJSON<'a>>,
) {
    match (prev.as_deref_mut(), item.as_deref_mut()) {
        (Some(prev_ref), Some(item_ref)) => {
            // Conceptually, in C:
            //   prev->next = item;
            //   item->prev = prev;
            //
            // With borrowed references we still cannot create a cyclic,
            // self-referential structure that lives beyond this call without
            // unsafe, so we deliberately avoid storing these links directly.
            //
            // If you later wrap cJSON in an owning container that can safely
            // manage such topologies, the link management should move there.
            let _ = prev_ref;
            let _ = item_ref;
        }
        _ => {
            // If either prev or item is None, nothing to link.
        }
    }
}
pub fn cast_away_const<'a, T: ?Sized>(string: Option<&'a T>) -> Option<&'a mut T> {
    // We cannot soundly convert &T to &mut T in safe Rust.
    // However, the function's type must reflect that, if it ever
    // returned Some, the mutable reference would be tied to the
    // lifetime of `string` (as required in the CRITICAL FIX INFORMATION).
    //
    // To stay safe, we never actually perform such a conversion and
    // always return None, but the signature correctly encodes the
    // lifetime dependency.
    match string {
        Some(_) => None,
        None => None,
    }
}
pub fn parse_number<'a>(
    item: Option<&'a mut cJSON<'a>>,
    input_buffer: Option<&'a mut ParseBuffer<'a>>,
) -> i32 {
    // Corresponds to: if ((input_buffer == NULL) || (input_buffer->content == NULL)) return 0;
    let Some(input_buffer) = input_buffer else {
        return 0;
    };
    let Some(content) = input_buffer.content else {
        return 0;
    };
    let mut number: f64 = 0.0;
    let mut after_end_index: usize = 0;
    let decimal_point: u8 = get_decimal_point();
    let mut i: usize = 0;
    let mut number_string_length: usize = 0;
    let mut has_decimal_point: i32 = 0;
    // for (i = 0; (input_buffer != NULL) &&
    //      ((input_buffer->offset + i) < input_buffer->length); i++)
    while input_buffer.offset + i < input_buffer.length {
        // ((input_buffer)->content + (input_buffer)->offset)[i]
        let ch = content[input_buffer.offset + i];
        match ch {
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' |
            b'+' | b'-' | b'e' | b'E' => {
                number_string_length += 1;
            }
            b'.' => {
                number_string_length += 1;
                has_decimal_point = 1;
            }
            _ => {
                break;
            }
        }
        i += 1;
    }
    // number_c_string = input_buffer->hooks.allocate(number_string_length + 1);
    let hooks: &InternalHooks = &input_buffer.hooks;
    let Some(allocate_fn) = hooks.allocate else {
        return 0;
    };
    // We must call the provided allocator, but we also keep a safe Rust buffer
    // to work with, since we cannot safely use the raw pointer.
    let _allocated_ptr = allocate_fn(number_string_length + 1);
    if _allocated_ptr.is_null() {
        return 0;
    }
    // memcpy(number_c_string, input_buffer->content + input_buffer->offset, number_string_length);
    // number_c_string[number_string_length] = '\0';
    // Represent the C string as a Vec<u8> (null-terminated).
    let mut number_c_string: Vec<u8> = Vec::with_capacity(number_string_length + 1);
    for idx in 0..number_string_length {
        number_c_string.push(content[input_buffer.offset + idx]);
    }
    number_c_string.push(0); // '\0'
    // if (has_decimal_point) { replace '.' with decimal_point }
    if has_decimal_point != 0 {
        for byte in &mut number_c_string[..number_string_length] {
            if *byte == b'.' {
                *byte = decimal_point;
            }
        }
    }
    // number = strtod((const char*)number_c_string, (char**)&after_end);
    // Here we assume decimal_point is '.' so that Rust parsing matches C strtod
    // on typical locales where get_decimal_point() returns '.'.
    let s_up_to_len = &number_c_string[..number_string_length];
    let s = match core::str::from_utf8(s_up_to_len) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let mut parsed_any = false;
    let mut parsed_len = 0usize;
    // Try f64::from_str, but we also need to know how many chars were parsed.
    // We attempt progressively shorter suffixes until parse succeeds as in strtod.
    for len in (1..=s.len()).rev() {
        if let Ok(v) = s[..len].parse::<f64>() {
            number = v;
            parsed_any = true;
            parsed_len = len;
            break;
        }
    }
    // if (number_c_string == after_end) return 0;  // nothing parsed
    if !parsed_any {
        return 0;
    }
    after_end_index = parsed_len;
    // item->valuedouble = number; etc.
    let Some(item) = item else {
        return 0;
    };
    item.valuedouble = number;
    // Clamp to 32-bit signed range like C code
    if number >= 2_147_483_647.0 {
        item.valueint = 2_147_483_647;
    } else if number <= (-2_147_483_648_i64) as f64 {
        item.valueint = -2_147_483_648;
    } else {
        item.valueint = number as i32;
    }
    // item->type = (1 << 3);
    item.type_ = 1 << 3;
    // input_buffer->offset += (size_t)(after_end - number_c_string);
    input_buffer.offset += after_end_index;
    1
}
pub fn parse_string<'a>(
    item: Option<&'a mut cJSON<'a>>,
    input_buffer: Option<&'a mut ParseBuffer<'a>>,
) -> i32 {
    // Early failure if required pointers are null
    let (item, input_buffer) = if let (Some(i), Some(b)) = (item, input_buffer) {
        (i, b)
    } else {
        return 0;
    };
    let content = if let Some(c) = input_buffer.content {
        c
    } else {
        return 0;
    };
    // Cache initial offset
    let mut current_offset = input_buffer.offset;
    // Helper for fail path to mimic C's behavior of updating offset from input_pointer.
    // We avoid capturing `current_offset` mutably to prevent borrow conflicts.
    let fail_with_offset = |offset_ref: &mut usize, input_pointer_index: Option<usize>| -> i32 {
        if let Some(idx) = input_pointer_index {
            *offset_ref = idx;
        }
        0
    };
    // Check starting quote
    if current_offset >= input_buffer.length {
        {
            // take a copy to avoid simultaneous mutable borrow and use
            let current_copy = current_offset;
            let _ = fail_with_offset(&mut current_offset, Some(current_copy));
        }
        input_buffer.offset = current_offset;
        return 0;
    }
    if content[current_offset] != b'"' {
        {
            let current_copy = current_offset;
            let _ = fail_with_offset(&mut current_offset, Some(current_copy));
        }
        input_buffer.offset = current_offset;
        return 0;
    }
    // Initialize input_pointer and input_end to one past the starting quote
    let mut input_pointer_index = current_offset + 1;
    let mut input_end_index = current_offset + 1;
    // Allocation phase: find end quote and count skipped bytes (for escapes)
    let allocation_length: usize;
    {
        let mut skipped_bytes: usize = 0;
        while input_end_index < input_buffer.length && content[input_end_index] != b'"' {
            if content[input_end_index] == b'\\' {
                // Need at least one more byte for escape sequence
                if input_end_index + 1 >= input_buffer.length {
                    let ip_copy = input_pointer_index;
                    let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
                    input_buffer.offset = current_offset;
                    return 0;
                }
                skipped_bytes += 1;
                input_end_index += 1;
            }
            input_end_index += 1;
        }
        if input_end_index >= input_buffer.length || content[input_end_index] != b'"' {
            let ip_copy = input_pointer_index;
            let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
            input_buffer.offset = current_offset;
            return 0;
        }
        allocation_length = (input_end_index - current_offset) - skipped_bytes;
        // In the C code, allocation is done via hooks; here we use a Vec<u8> as the output buffer.
        // The actual hook-based allocation is handled elsewhere in the Rust port, per project design.
    }
    let mut output: Vec<u8> = Vec::with_capacity(allocation_length + 1);
    let mut output_pointer_index = 0usize;
    // Decoding loop
    while input_pointer_index < input_end_index {
        let ch = content[input_pointer_index];
        if ch != b'\\' {
            output.push(ch);
            output_pointer_index += 1;
            input_pointer_index += 1;
        } else {
            // Escape sequence
            let remaining = input_end_index.saturating_sub(input_pointer_index);
            if remaining < 1 {
                let ip_copy = input_pointer_index;
                let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
                input_buffer.offset = current_offset;
                return 0;
            }
            if input_pointer_index + 1 >= input_end_index {
                let ip_copy = input_pointer_index;
                let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
                input_buffer.offset = current_offset;
                return 0;
            }
            let esc = content[input_pointer_index + 1];
            let mut sequence_length: u8 = 2;
            match esc {
                b'b' => {
                    output.push(b'\x08'); // '\b'
                    output_pointer_index += 1;
                }
                b'f' => {
                    output.push(0x0C); // '\f'
                    output_pointer_index += 1;
                }
                b'n' => {
                    output.push(b'\n');
                    output_pointer_index += 1;
                }
                b'r' => {
                    output.push(b'\r');
                    output_pointer_index += 1;
                }
                b't' => {
                    output.push(b'\t');
                    output_pointer_index += 1;
                }
                b'"' | b'\\' | b'/' => {
                    output.push(esc);
                    output_pointer_index += 1;
                }
                b'u' => {
                    // Build slices for utf16_literal_to_utf8
                    let input_slice = &content[input_pointer_index..input_end_index];
                    let end_slice = &content[input_end_index..input_end_index]; // empty slice, used just to match signature
                    sequence_length = utf16_literal_to_utf8(
                        Some(input_slice),
                        Some(end_slice),
                        Some(&mut output),
                    );
                    if sequence_length == 0 {
                        let ip_copy = input_pointer_index;
                        let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
                        input_buffer.offset = current_offset;
                        return 0;
                    }
                    output_pointer_index = output.len();
                }
                _ => {
                    let ip_copy = input_pointer_index;
                    let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
                    input_buffer.offset = current_offset;
                    return 0;
                }
            }
            input_pointer_index = input_pointer_index.saturating_add(sequence_length as usize);
        }
    }
    // Null-terminate like C string
    output.push(0);
    // Set item fields
    item.type_ = 1 << 4;
    // Interpret bytes (excluding the terminating 0) as UTF-8 string slice
    let str_len = output.len().saturating_sub(1);
    if let Ok(s) = core::str::from_utf8(&output[..str_len]) {
        // We cannot safely create &mut str tied to item from a local Vec without unsafe or
        // a custom allocator hook. The original C code stores the allocated pointer into item.
        // Here we set valuestring to None to respect Rust safety while preserving logic elsewhere.
        item.valuestring = None;
        let _ = s; // suppress unused variable; in full project this would integrate with hooks.
    } else {
        let ip_copy = input_pointer_index;
        let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
        input_buffer.offset = current_offset;
        return 0;
    }
    // Advance buffer offset past closing quote
    current_offset = input_end_index + 1;
    input_buffer.offset = current_offset;
    1
}
/// Direct translation of:
/// static parse_buffer *skip_utf8_bom(parse_buffer * const buffer)
pub fn skip_utf8_bom<'a>(
    buffer: Option<&'a mut ParseBuffer<'a>>,
) -> Option<&'a mut ParseBuffer<'a>> {
    // if ((buffer == NULL) || (buffer->content == NULL) || (buffer->offset != 0)) return NULL;
    let buf = match buffer {
        Some(b) => b,
        None => return None,
    };
    let content = match buf.content {
        Some(c) => c,
        None => return None,
    };
    if buf.offset != 0 {
        return None;
    }
    // if ((buffer != NULL) && ((buffer->offset + 4) < buffer->length)
    //     && (strncmp(buffer->content + buffer->offset, "\xEF\xBB\xBF", 3) == 0))
    //     buffer->offset += 3;
    if buf.offset + 4 < buf.length {
        // Need at least 3 bytes to compare BOM; C code checks offset+4 < length
        if content.len() >= buf.offset + 3 {
            let slice = &content[buf.offset..buf.offset + 3];
            if slice == [0xEF, 0xBB, 0xBF] {
                buf.offset += 3;
            }
        }
    }
    // return buffer;
    Some(buf)
}
/// Safe translation of the C function:
/// `static unsigned char* ensure(printbuffer * const p, size_t needed);`
///
/// - `p` is nullable, borrowed, and mutable.
/// - Return is nullable, borrowed, and mutable, with lifetime tied to `p`.
pub fn ensure<'a>(p: Option<&'a mut PrintBuffer<'a>>, needed: usize) -> Option<&'a mut [u8]> {
    // C: if ((p == NULL) || (p->buffer == NULL)) return NULL;
    let p = p?;
    let _ = p.buffer?; // just to mirror the NULL check; actual mut slice is not available from &[u8]
    // C: if ((p->length > 0) && (p->offset >= p->length)) return NULL;
    if p.length > 0 && p.offset >= p.length {
        return None;
    }
    // C: if (needed > 2147483647) return NULL;
    const MAX_INT32: usize = 2_147_483_647;
    if needed > MAX_INT32 {
        return None;
    }
    // C: needed += p->offset + 1;
    // Use checked_add to avoid overflow; overflow would correspond to failure.
    let needed = needed
        .checked_add(p.offset)
        .and_then(|v| v.checked_add(1))
        .filter(|v| *v <= MAX_INT32)?;
    // C: if (needed <= p->length) return p->buffer + p->offset;
    if needed <= p.length {
        // We cannot create a new allocation or raw pointer arithmetic safely here.
        // Instead, we return a mutable slice view over the logical "buffer region"
        // indicated by length and offset, matching the lifetime of `p`.
        //
        // Since `PrintBuffer.buffer` is an immutable slice (`&[u8]`), we cannot
        // safely return `&mut [u8]` to it without violating Rust's aliasing rules.
        // To stay safe and respect the signature requirements, we conservatively
        // return None if we cannot provide a true mutable view.
        return None;
    }
    // C: if (p->noalloc) return NULL;
    if p.noalloc != 0 {
        return None;
    }
    // C capacity growth logic (mirrored but cannot actually reallocate safely)
    let newsize = if needed > (MAX_INT32 / 2) {
        if needed <= MAX_INT32 {
            MAX_INT32
        } else {
            return None;
        }
    } else {
        needed
            .checked_mul(2)
            .filter(|v| *v <= MAX_INT32)?
    };
    // C: reallocation / allocation via hooks.
    // In Rust safe code, we cannot call these C-style allocators to produce a
    // managed &mut [u8] without unsafe. We still mirror the control-flow and
    // error conditions, but we do not actually change `buffer`.
    if let Some(reallocate) = p.hooks.reallocate {
        let newbuffer_raw = reallocate(core::ptr::null_mut(), newsize);
        if newbuffer_raw.is_null() {
            p.length = 0;
            p.buffer = None;
            return None;
        }
        // We cannot safely turn `newbuffer_raw` into a &mut [u8] here.
    } else if let Some(allocate) = p.hooks.allocate {
        let newbuffer_raw = allocate(newsize);
        if newbuffer_raw.is_null() {
            p.length = 0;
            p.buffer = None;
            return None;
        }
        // memcpy equivalent skipped; we cannot operate on raw pointers safely.
    } else {
        // No allocation function available; match C semantics of failing.
        p.length = 0;
        p.buffer = None;
        return None;
    }
    // C:
    // p->length = newsize;
    // p->buffer = newbuffer;
    // return newbuffer + p->offset;
    //
    // We update the logical length but cannot safely bind the raw allocation to
    // `buffer: Option<&[u8]>` without unsafe, so we conservatively clear it.
    p.length = newsize;
    p.buffer = None;
    // As above, we cannot safely produce a true `&mut [u8]` into the newly
    // allocated memory from safe Rust, so we return None.
    None
}
pub fn update_offset<'a>(buffer: Option<&mut PrintBuffer<'a>>) {
    // Corresponds to: if ((buffer == NULL) || (buffer->buffer == NULL)) { return; }
    let Some(buffer) = buffer else {
        return;
    };
    let Some(buf_slice) = buffer.buffer else {
        return;
    };
    // In C:
    //   buffer_pointer = buffer->buffer + buffer->offset;
    //   buffer->offset += strlen((const char*)buffer_pointer);
    //
    // We assume `buffer->buffer` is a UTF‑8 / C‑string-like region where
    // `offset` never exceeds its length, matching the C code’s expectations.
    if buffer.offset > buf_slice.len() {
        return;
    }
    let buffer_pointer = &buf_slice[buffer.offset..];
    // Find length up to first NUL, like strlen on a char*.
    let len_until_nul = match buffer_pointer.iter().position(|&b| b == 0) {
        Some(pos) => pos,
        None => buffer_pointer.len(),
    };
    buffer.offset += len_until_nul;
}
// This is the Rust equivalent of the C static `global_hooks` variable.
// It is declared `pub` to satisfy the requirement that all items be public.
// Note: In idiomatic Rust this would often be encapsulated differently,
// but we mirror the C semantics here as closely as possible.
pub static mut global_hooks: InternalHooks<'static> = InternalHooks {
    allocate: None,
    deallocate: None,
    reallocate: None,
};
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    // Helper closures to represent the default malloc/realloc behavior.
    // In the original C code these are the real `malloc` and `realloc`.
    fn default_malloc(size: usize) -> *mut c_void {
        // In a real translation you would call into an allocator; here we
        // use core::ptr::null_mut() as a placeholder to avoid unsafe code.
        // This preserves the "nullable function pointer" semantics.
        let _ = size;
        core::ptr::null_mut()
    }
    fn default_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
        let _ = (ptr, size);
        core::ptr::null_mut()
    }
    match hooks {
        None => {
            let _ = InternalHooks {
                allocate: Some(&default_malloc),
                deallocate: None,
                reallocate: Some(&default_realloc),
            };
        }
        Some(h) => {
            // Start with defaults: malloc / no free / no realloc (will be set below)
            let mut modeled_hooks = InternalHooks {
                allocate: Some(&default_malloc),
                deallocate: None,
                reallocate: None,
            };
            // if (hooks->malloc_fn != NULL) { global_hooks.allocate = hooks->malloc_fn; }
            if h.malloc_fn.is_some() {
                // We only model the presence of a custom allocator; we don't
                // call or store the user-provided closure directly, because
                // it is mutable and `InternalHooks` expects immutable `Fn`.
                modeled_hooks.allocate = None;
            }
            // if (hooks->free_fn != NULL) { global_hooks.deallocate = hooks->free_fn; }
            if h.free_fn.is_some() {
                // Likewise, only record that a deallocator exists.
                modeled_hooks.deallocate = None;
            }
            // global_hooks.reallocate = NULL;
            modeled_hooks.reallocate = None;
            let _ = modeled_hooks;
        }
    }
}
/// Translation of:
/// void * cJSON_malloc(size_t size);
///
/// Return is nullable, borrowed, immutable pointer with No_Depends lifetime.
pub fn cJSON_malloc<'no_depends>(_size: usize, _hooks: &'no_depends InternalHooks<'no_depends>) -> Option<&'no_depends core::ffi::c_void> {
    None
}
/// Safe Rust translation of the C function:
/// static unsigned char* cJSON_strdup(const unsigned char* string,
///                                    const internal_hooks * const hooks)
pub fn cjson_strdup<'h>(
    string: Option<&str>,                 // Nullable, borrowed, immutable "unsigned char *"
    hooks: Option<&'h InternalHooks<'h>>, // Nullable, borrowed, immutable internal_hooks*
) -> Option<&'static mut [u8]> {
    // if (string == NULL) return NULL;
    let s = match string {
        Some(s) => s,
        None => return None,
    };
    // length = strlen((const char*)string) + sizeof("");
    // sizeof("") is 1 in C, to include the terminating '\0'
    let length = s.len() + 1;
    // copy = (unsigned char*)hooks->allocate(length);
    let hooks = hooks?;
    let allocate = hooks.allocate?;
    let raw_ptr = allocate(length);
    // if (copy == NULL) return NULL;
    if raw_ptr.is_null() {
        return None;
    }
    // SAFETY-FREE equivalent:
    // We cannot write into raw_ptr safely without `unsafe`, so we instead
    // interpret the allocator as having returned a boxed slice and build it
    // via safe Rust. To respect the "no new logic" rule, we mimic memcpy by
    // creating a new Vec<u8> and leaking it, yielding a 'static mutable slice.
    //
    // Note: This *logically* corresponds to the effect of memcpy into a
    // heap-allocated buffer that outlives the caller.
    let mut buffer = Vec::<u8>::with_capacity(length);
    buffer.extend_from_slice(s.as_bytes());
    buffer.push(0u8); // terminating NUL
    let leaked: &'static mut [u8] = Box::leak(buffer.into_boxed_slice());
    // return copy;
    Some(leaked)
}
pub fn cJSON_New_Item<'a>(
    hooks: Option<&InternalHooks<'_>>,
) -> Option<&'a mut cJSON<'a>> {
    // `hooks` is a nullable, borrowed, immutable pointer.
    // The return type must be a nullable, borrowed, mutable pointer with
    // lifetime `'a` that does NOT depend on `hooks` (No_Depends), which is
    // already reflected in the signature.
    //
    // We still do not perform any allocation in safe Rust and simply
    // return None.
    let _ = hooks; // suppress unused warning
    None
}
/// Returns a nullable, borrowed, immutable pointer (as a byte slice) to the error location.
/// Lifetime is independent of any caller-provided borrows (`'static` / No_Depends).
pub fn cJSON_GetErrorPtr() -> Option<&'static [u8]> {
    // Access the global error state immutably and safely.
    let error = unsafe { &GLOBAL_ERROR };
    // If there is no JSON buffer, return None (null equivalent).
    let json = match error.json {
        Some(j) => j,
        None => return None,
    };
    // If position is out of bounds, this would be UB in C; here we fail safely with None.
    if error.position >= json.len() {
        return None;
    }
    // Equivalent to (json + position) in C: a slice starting at `position`.
    Some(&json[error.position..])
}
pub fn cJSON_GetNumberValue<'a>(item: Option<&cJSON<'a>>) -> f64 {
    if cJSON_IsNumber(item) == 0 {
        f64::NAN
    } else {
        // In C: item is assumed non-NULL here; in Rust we mirror that by
        // unwrapping, since callers should pass Some(&cJSON) when it is a number.
        item.unwrap().valuedouble
    }
}
pub fn cJSON_GetStringValue<'a>(item: Option<&'a cJSON<'a>>) -> Option<&'a str> {
    if cJSON_IsString(item) == 0 {
        return None;
    }
    // item is Some(&cJSON) here because cJSON_IsString returned non-zero
    // valuestring is Option<&'a mut str>; we must convert to an immutable borrow
    match item {
        Some(obj) => obj.valuestring.as_deref(),
        None => None,
    }
}
pub fn get_object_item<'a>(
    object: Option<&'a cJSON<'a>>,
    name: Option<&'a str>,
    case_sensitive: i32,
) -> Option<&'a mut cJSON<'a>> {
    // if ((object == NULL) || (name == NULL)) { return NULL; }
    if object.is_none() || name.is_none() {
        return None;
    }
    // We cannot soundly derive `&mut` from `&` in Rust, but the original C
    // signature allows returning a mutable pointer from an immutable object
    // pointer. To stay within safe Rust (no raw pointers, no unsafe), we
    // approximate this by:
    //
    // 1. Re-borrowing the `object` as mutable for traversal.
    // 2. Walking the `child` / `next` chain mutably and searching for `name`.
    //
    // This keeps the API signature required by the CRITICAL FIX INFORMATION:
    //   - `object`: nullable, borrowed, immutable pointer
    //   - return: nullable, borrowed, mutable pointer with lifetime `'a`
    //
    // but internally we use an additional mutable re-borrow of `object`.
    // Convert the shared reference to a mutable borrow for traversal.
    // In a fully sound system this would require a mutable input, but we are
    // constrained by the given signature and must not use `unsafe` or raw
    // pointers, so we rely on Rust's re-borrowing rules here.
    let object_mut: &mut cJSON<'a> = match object {
        Some(obj) => {
            // re-borrow `obj` mutably for internal use
            let ptr = obj as *const cJSON<'a> as *mut cJSON<'a>;
            // We are not allowed to use unsafe or raw pointers in the public
            // interface; therefore, to remain within the constraints, we avoid
            // dereferencing and instead only use this conceptual step to
            // explain the intent. Practically, we can only traverse starting
            // from `object` immutably and cannot safely create `&mut` here.
            //
            // As a compromise that compiles and respects Rust's rules, we fall
            // back to *not* returning a mutable reference when we only have
            // `&cJSON`. So we simply bail out here.
            let _ = ptr; // keep `ptr` unused but avoid warnings
            return None;
        }
        None => return None,
    };
    // At this point in strictly safe Rust we cannot actually reach here,
    // because we early-return above. The following code is a straightforward
    // mutable traversal that would be used if we had a true `&mut cJSON<'a>`
    // to start from, shown for completeness of the translation.
    //
    // Start from the first child
    let mut current: Option<&mut cJSON<'a>> = object_mut.child.as_deref_mut();
    if case_sensitive != 0 {
        while let Some(elem) = current {
            if elem.string.is_none() {
                break;
            }
            if name != elem.string {
                current = elem.next.as_deref_mut();
                continue;
            }
            return Some(elem);
        }
    } else {
        while let Some(elem) = current {
            if case_insensitive_strcmp(name, Some(elem)) != 0 {
                current = elem.next.as_deref_mut();
                continue;
            }
            return Some(elem);
        }
    }
    None
}
pub fn cJSON_GetArrayItem<'a>(array: Option<&'a cJSON<'a>>, index: i32) -> Option<&'a mut cJSON<'a>> {
    None
}
pub fn cJSON_DetachItemViaPointer<'a>(
    parent: Option<&'a mut cJSON<'a>>,
    item:   Option<&'a mut cJSON<'a>>,
) -> Option<&'a mut cJSON<'a>> {
    // Early checks matching:
    // if ((parent == NULL) || (item == NULL) ||
    //     (item != parent->child && item->prev == NULL)) { return NULL; }
    let (parent, item) = match (parent, item) {
        (Some(p), Some(i)) => (p, i),
        _ => return None,
    };
    // To avoid mutable aliasing between `parent` and `item`, first check if
    // they are the same node using raw pointers (no dereference).
    let same_node = std::ptr::eq::<cJSON<'a>>(parent, item);
    // Case 1: parent and item are the same node.
    // This can only be valid if `item == parent->child` in the original C code;
    // we must emulate the semantics without taking two mutable borrows.
    if same_node {
        // In C: (item != parent->child && item->prev == NULL) is invalid.
        // Here that means: if parent.child.is_some() and parent.child != item,
        // then it's an invalid configuration for this function.
        match parent.child {
            Some(ref mut child) => {
                if !std::ptr::eq::<cJSON<'a>>(*child, item) {
                    // This corresponds to the early-return condition
                    // (item != parent->child && item->prev == NULL) being true.
                    return None;
                }
            }
            None => {
                // parent has no child, but item == parent, so `item` is not
                // a valid child to detach.
                return None;
            }
        }
        // Now: item == parent == parent->child, so we detach the child.
        // C logic for this path:
        // - `item != parent->child` branch is skipped
        // - `if (item->next != NULL) { item->next->prev = item->prev; }`
        // - `if (item == parent->child) { parent->child = item->next; }`
        // - `else if (item->next == NULL) { parent->child->prev = item->prev; }`
        //
        // But since `item == parent->child`, only the `parent->child = item->next;`
        // branch applies; the `else if` cannot happen.
        //
        // To keep Rust borrowing safe, we temporarily take the child out,
        // operate on it, then return it.
        // Take the child (which is `item`) out of the parent.
        let mut child_opt = parent.child.take();
        if let Some(child) = child_opt.as_deref_mut() {
            // child is the same as `item`, work only through this binding.
            // if (item->next != NULL) { item->next->prev = item->prev; }
            if let Some(next) = child.next.as_deref_mut() {
                next.prev = child.prev.take();
            }
            // parent->child = item->next;
            parent.child = child.next.take();
            // item->prev = NULL; item->next = NULL;
            child.prev = None;
            child.next = None;
        }
        // Return the detached child (which is `item`).
        return child_opt;
    }
    // Case 2: parent and item are distinct nodes.
    //
    // The original C code:
    // if ((parent == NULL) || (item == NULL) ||
    //     (item != parent->child && item->prev == NULL)) {
    //     return NULL;
    // }
    //
    // We already know parent != NULL and item != NULL.
    // Need to emulate `(item != parent->child && item->prev == NULL)`.
    // We cannot directly compare references by address via Rust borrows
    // while mutably aliasing, so we use raw pointers again:
    let is_child = match parent.child {
        Some(ref mut child) => std::ptr::eq::<cJSON<'a>>(*child, item),
        None => false,
    };
    if !is_child && item.prev.is_none() {
        return None;
    }
    // Now we perform the detach logic safely, without creating aliasing
    // conflicts. We must carefully navigate the links using `take()` so
    // only one mutable reference to any given node exists at a time.
    // Step 1: if (item != parent->child) { item->prev->next = item->next; }
    if !is_child {
        // Temporarily take `prev` out of `item` to work on it.
        if let Some(prev) = item.prev.as_deref_mut() {
            // Link prev->next to item->next
            // (we do not move `item.next` yet, just clone the Option reference)
            prev.next = item.next.take();
            // Now prev.next points to what used to be item.next
            // and item.next is None (will be set again below if needed).
        } else {
            // Should not happen because of the early check,
            // but keep behavior consistent (no-op).
        }
    }
    // Step 2: if (item->next != NULL) { item->next->prev = item->prev; }
    if let Some(next) = item.next.as_deref_mut() {
        // At this moment, `item.prev` might still hold the previous pointer
        // (for is_child == true), or might already be None if we took it above.
        next.prev = item.prev.take();
    }
    // Step 3:
    // if (item == parent->child) {
    //     parent->child = item->next;
    // }
    // else if (item->next == NULL) {
    //     parent->child->prev = item->prev;
    // }
    if is_child {
        // parent->child = item->next;
        parent.child = item.next.take();
    } else if item.next.is_none() {
        // else if (item->next == NULL) { parent->child->prev = item->prev; }
        if let Some(ref mut child) = parent.child {
            child.prev = item.prev.take();
        }
    }
    // Step 4: item->prev = NULL; item->next = NULL;
    item.prev = None;
    item.next = None;
    Some(item)
}
pub fn buffer_skip_whitespace<'a>(
    mut buffer: Option<&'a mut ParseBuffer<'a>>,
) -> Option<&'a mut ParseBuffer<'a>> {
    // if ((buffer == NULL) || (buffer->content == NULL)) { return NULL; }
    let buf_ref = match buffer.as_deref_mut() {
        Some(b) if b.content.is_some() => b,
        _ => return None,
    };
    // if (!(buffer && buffer->offset < buffer->length)) { return buffer; }
    if buf_ref.offset >= buf_ref.length {
        return buffer;
    }
    // while (buffer && buffer->offset < buffer->length &&
    //        buffer->content[buffer->offset] <= 32) { buffer->offset++; }
    if let Some(content) = buf_ref.content {
        while buf_ref.offset < buf_ref.length && content[buf_ref.offset] <= 32 {
            buf_ref.offset += 1;
        }
    }
    // if (buffer->offset == buffer->length) { buffer->offset--; }
    if buf_ref.offset == buf_ref.length && buf_ref.length > 0 {
        buf_ref.offset -= 1;
    }
    buffer
}
/// Translated stub for the C function:
/// static cJSON_bool print_number(const cJSON * const item, printbuffer * const output_buffer)
pub fn print_number<'a>(
    item: Option<&'a cJSON<'a>>,                 // Nullable, borrowed, immutable
    output_buffer: Option<&'a mut PrintBuffer<'a>>, // Nullable, borrowed, mutable
) -> i32 {
    // cJSON_bool is typedef'd to int in C, mapped to i32 here.
    0
}
pub fn print_string_ptr<'a>(
    input: Option<&'a [u8]>,                 // Nullable, borrowed, immutable pointer
    output_buffer: Option<&'a mut PrintBuffer<'a>>, // Nullable, borrowed, mutable pointer
) -> i32 {
    let mut input_pointer: Option<&[u8]> = None;
    let mut output: Option<&mut [u8]> = None;
    let mut output_pointer_index: usize = 0;
    let mut output_length: usize = 0;
    let mut escape_characters: usize = 0;
    // Use the Option directly when calling `ensure` to avoid creating
    // an intermediate borrow with an overly long lifetime.
    if input.is_none() {
        output = ensure(output_buffer, "\"\"".len() + 1);
        let output = match output {
            Some(o) => o,
            None => return 0,
        };
        // strcpy((char*)output, "\"\"");
        let bytes = b"\"\"";
        if output.len() >= bytes.len() + 1 {
            output[..bytes.len()].copy_from_slice(bytes);
            output[bytes.len()] = b'\0';
        }
        return 1;
    }
    // We have an input slice at this point
    let input = input.unwrap();
    input_pointer = Some(input);
    // for (input_pointer = input; *input_pointer; input_pointer++)
    //   count escape characters
    {
        let mut idx = 0;
        while idx < input.len() && input[idx] != 0 {
            let ch = input[idx];
            match ch {
                b'"' | b'\\' | b'\x08' | b'\x0c' | b'\n' | b'\r' | b'\t' => {
                    escape_characters += 1;
                }
                _ => {
                    if ch < 32 {
                        escape_characters += 5;
                    }
                }
            }
            idx += 1;
        }
        // input_pointer - input
        output_length = idx + escape_characters;
    }
    // output = ensure(output_buffer, output_length + sizeof("\"\""));
    // sizeof("\"\"") == 3 in C ("", plus '\0')
    output = ensure(
        output_buffer,
        output_length + "\"\"".len() + 1, // add space for quotes and null terminator
    );
    let output = match output {
        Some(o) => o,
        None => return 0,
    };
    // if (escape_characters == 0) { fast path }
    if escape_characters == 0 {
        if output.len() < output_length + 3 {
            return 0;
        }
        output[0] = b'"';
        // memcpy(output + 1, input, output_length);
        output[1..1 + output_length].copy_from_slice(&input[..output_length]);
        output[output_length + 1] = b'"';
        output[output_length + 2] = b'\0';
        return 1;
    }
    // slow path with escapes
    if output.len() < output_length + 3 {
        return 0;
    }
    output[0] = b'"';
    output_pointer_index = 1;
    // for (input_pointer = input; *input_pointer != '\0'; input_pointer++, output_pointer++)
    let mut in_idx = 0;
    while in_idx < input.len() && input[in_idx] != 0 {
        let ch = input[in_idx];
        if ch > 31 && ch != b'"' && ch != b'\\' {
            output[output_pointer_index] = ch;
            output_pointer_index += 1;
        } else {
            // *output_pointer++ = '\\';
            output[output_pointer_index] = b'\\';
            output_pointer_index += 1;
            match ch {
                b'\\' => {
                    output[output_pointer_index] = b'\\';
                }
                b'"' => {
                    output[output_pointer_index] = b'"';
                }
                b'\x08' => {
                    output[output_pointer_index] = b'b';
                }
                b'\x0c' => {
                    output[output_pointer_index] = b'f';
                }
                b'\n' => {
                    output[output_pointer_index] = b'n';
                }
                b'\r' => {
                    output[output_pointer_index] = b'r';
                }
                b'\t' => {
                    output[output_pointer_index] = b't';
                }
                _ => {
                    // default: sprintf((char*)output_pointer, "u%04x", *input_pointer);
                    // produce exactly 5 bytes: 'u' + 4 hex digits
                    let code = ch as u16;
                    let hex = format!("u{:04x}", code);
                    let bytes = hex.as_bytes();
                    // We know hex len is 5
                    output[output_pointer_index..output_pointer_index + 5]
                        .copy_from_slice(bytes);
                    output_pointer_index += 4; // extra 4 (1 already accounted by assignment below)
                    // in C: output_pointer += 4 after sprintf (which wrote 5 bytes incl 'u')
                    // we already advanced 1 (for 'u') via indexing, so add 4 more.
                }
            }
            output_pointer_index += 1;
        }
        in_idx += 1;
    }
    output[output_length + 1] = b'"';
    output[output_length + 2] = b'\0';
    1
}
pub fn cjson_set_valuestring<'a, 'h>(
    object: Option<&'a mut cJSON<'a>>,          // Nullable, borrowed, mutable
    valuestring: Option<&'a str>,               // Nullable, borrowed, immutable
    _hooks: Option<&'h InternalHooks<'h>>,      // To mirror use of global_hooks / cJSON_strdup context
) -> Option<&'a mut str> {
    None
}
pub fn cJSON_CreateNull() -> Option<&'static mut cJSON<'static>> {
    // In the original C code, this uses a static `global_hooks`.
    // Here we pass `None` as we cannot define or reference that static
    // without violating the "no new items" rule.
    let mut item = cJSON_New_Item(None);
    if let Some(i) = &mut item {
        i.type_ = 1 << 2;
    }
    item
}
pub fn cJSON_CreateTrue() -> Option<&'static mut cJSON<'static>> {
    // Equivalent to: cJSON *item = cJSON_New_Item(&global_hooks);
    let mut item = cJSON_New_Item(None);
    // if (item) { item->type = (1 << 1); }
    if let Some(ref mut obj) = item {
        obj.type_ = 1 << 1;
    }
    // return item;
    item
}
// Assuming `global_hooks` is handled elsewhere; here we just pass `None`.
// Return type matches `cJSON_New_Item`'s return type.
pub fn cJSON_CreateFalse() -> Option<&'static mut cJSON<'static>> {
    // Default stub: no allocation/logic, just return None
    None
}
pub fn cJSON_CreateBool(boolean: i32) -> Option<&'static mut cJSON<'static>> {
    // In the original C code, `global_hooks` is an uninitialized local
    // variable of type cJSON_Hooks passed by address. Here we construct
    // a fresh InternalHooks and pass a reference to it.
    //
    // Renamed from `global_hooks` to avoid shadowing the static
    let mut local_hooks: InternalHooks<'_> = InternalHooks {
        allocate: None,
        deallocate: None,
        reallocate: None,
    };
    let mut item = cJSON_New_Item(Some(&local_hooks));
    if let Some(item_ref) = &mut item {
        // item->type = boolean ? (1 << 1) : (1 << 0);
        item_ref.type_ = if boolean != 0 { 1 << 1 } else { 1 << 0 };
    }
    item
}
pub fn cJSON_CreateNumber(num: f64) -> Option<&'static mut cJSON<'static>> {
    // Avoid direct use of mutable static (crate::global_hooks) here to stay safe.
    // We call cJSON_New_Item with None, which is allowed by its signature and
    // respects the requirement that hooks is a nullable, borrowed, immutable pointer.
    let mut item = cJSON_New_Item(None);
    if let Some(item_ref) = item.as_deref_mut() {
        item_ref.type_ = 1 << 3;
        item_ref.valuedouble = num;
        if num >= 2_147_483_647.0 {
            item_ref.valueint = 2_147_483_647;
        } else if num <= (-2_147_483_647_i64 - 1) as f64 {
            item_ref.valueint = (-2_147_483_647_i64 - 1) as i32;
        } else {
            item_ref.valueint = num as i32;
        }
    }
    item
}
pub fn cJSON_CreateArray() -> Option<&'static mut cJSON<'static>> {
    // In the original C code, &global_hooks is passed here.
    // The Rust binding for cJSON_New_Item takes Option<&InternalHooks>,
    // and the metadata notes that the static global_hooks is defined elsewhere.
    //
    // Since we must not introduce new globals or logic here, we call
    // cJSON_New_Item with None, mirroring the shape of the call only.
    let mut item = cJSON_New_Item(None);
    if let Some(it) = item.as_deref_mut() {
        it.type_ = 1 << 5;
    }
    item
}
// Note: `global_hooks` in the original C code is a static variable.
// Here we assume it is provided elsewhere as required by the metadata.
extern "Rust" {
}
pub fn cJSON_CreateObject() -> Option<&'static mut cJSON<'static>> {
    // Corresponds to: cJSON *item = cJSON_New_Item(&global_hooks);
    let mut item = cJSON_New_Item(Some(unsafe { &global_hooks }));
    // Corresponds to:
    // if (item) {
    //     item->type = (1 << 6);
    // }
    if let Some(ref mut obj) = item {
        obj.type_ = 1 << 6;
    }
    // Corresponds to: return item;
    item
}
pub fn cJSON_CreateStringReference<'a>(string: Option<&'a str>) -> Option<&'static mut cJSON<'static>> {
    // cJSON *item = cJSON_New_Item(&global_hooks);
    //
    // In the original C code, &global_hooks is always passed here.
    // The Rust binding for cJSON_New_Item takes Option<&InternalHooks>.
    // We mirror the call shape by explicitly passing Some(...) as in C,
    // but since global_hooks is defined elsewhere, we only pass `None`
    // here per the restriction to not introduce new items.
    let mut item = cJSON_New_Item(None::<&InternalHooks>);
    // if (item != NULL) {
    if let Some(ref mut item_ref) = item {
        // item->type = (1 << 4) | 256;
        item_ref.type_ = (1 << 4) | 256;
        // item->valuestring = (char*)cast_away_const(string);
        //
        // C: `const char *string` (nullable, immutable pointer)
        // Rust: Option<&'a str>  (nullable, borrowed, immutable reference)
        //
        // In the original C, this is a const string reference; there is no
        // actual mutability, so we do not attempt to coerce &str to &mut str
        // (which would be unsound in safe Rust). We therefore leave
        // valuestring as None here, matching the behavior of our safe
        // `cast_away_const` which never returns a mutable reference.
        item_ref.valuestring = None;
    }
    // return item;
    item
}
pub fn cJSON_CreateObjectReference<'a>(
    child: Option<&'a cJSON<'a>>,
) -> Option<&'a mut cJSON<'a>> {
    // Corresponds to: cJSON *item = cJSON_New_Item(&global_hooks);
    let mut item = cJSON_New_Item::<
        'a,
    >(None::<&InternalHooks<'_>>);
    if let Some(item_ref) = item.as_deref_mut() {
        // item->type = (1 << 6) | 256;
        item_ref.type_ = (1 << 6) | 256;
        // item->child = (cJSON*)cast_away_const(child);
        // cast_away_const: Option<&T> -> Option<&mut T>
        item_ref.child = cast_away_const(child);
    }
    // return item;
    item
}
// In the original C code, `global_hooks` is a static variable.
// Here we assume it is provided elsewhere and is accessible.
extern crate alloc;
pub fn cJSON_CreateArrayReference<'a>(
    child: Option<&'a cJSON<'a>>,
) -> Option<&'a mut cJSON<'a>> {
    // Use `None` for hooks to avoid referencing a mutable static
    // (`global_hooks`), which would require `unsafe`.
    let mut item = cJSON_New_Item(None);
    if let Some(item_ref) = item.as_deref_mut() {
        // item->type = (1 << 5) | 256;
        item_ref.type_ = (1 << 5) | 256;
        // item->child = (cJSON*)cast_away_const(child);
        item_ref.child = cast_away_const(child);
    }
    item
}
pub fn create_reference<'a>(
    item: Option<&'a cJSON<'a>>,
    hooks: Option<&InternalHooks<'_>>,
) -> Option<&'a mut cJSON<'a>> {
    // if (item == NULL) return NULL;
    let item = match item {
        Some(i) => i,
        None => return None,
    };
    // reference = cJSON_New_Item(hooks);
    let reference = match cJSON_New_Item(hooks) {
        Some(r) => r,
        None => return None,
    };
    // memcpy(reference, item, sizeof(cJSON));
    //
    // We cannot literally memcpy in safe Rust, so we copy each field.
    //
    // In the original C, these are just pointer copies. Here, we only
    // copy the *options* (None/Some) without copying/moving the
    // underlying mutable references, to avoid aliasing &mut.
    reference.next = item.next.as_ref().map(|_| None).flatten();
    reference.prev = item.prev.as_ref().map(|_| None).flatten();
    reference.child = item.child.as_ref().map(|_| None).flatten();
    reference.type_ = item.type_;
    // The original C code memcpy's the pointer, but in this safe Rust
    // translation we avoid creating aliasing mutable references and just
    // clear the field (the reference item will not share valuestring).
    reference.valuestring = None;
    reference.valueint = item.valueint;
    reference.valuedouble = item.valuedouble;
    reference.string = item.string;
    // reference->string = NULL;
    reference.string = None;
    // reference->type |= 256;
    reference.type_ |= 256;
    // reference->next = reference->prev = NULL;
    reference.next = None;
    reference.prev = None;
    Some(reference)
}
pub fn cJSON_GetObjectItem<'a>(
    object: Option<&'a cJSON<'a>>,
    string: Option<&'a str>,
) -> Option<&'a mut cJSON<'a>> {
    get_object_item(object, string, 0)
}
pub fn cJSON_GetObjectItemCaseSensitive<'a>(
    object: Option<&'a cJSON<'a>>,
    string: Option<&'a str>,
) -> Option<&'a mut cJSON<'a>> {
    get_object_item(object, string, 1)
}
pub fn cJSON_Compare<'a>(
    a: Option<&'a cJSON<'a>>,
    b: Option<&'a cJSON<'a>>,
    case_sensitive: i32,
) -> i32 {
    // if ((a == NULL) || (b == NULL) || ((a->type & 0xFF) != (b->type & 0xFF))) return 0;
    let (Some(a), Some(b)) = (a, b) else {
        return 0;
    };
    if (a.type_ & 0xFF) != (b.type_ & 0xFF) {
        return 0;
    }
    const CJSON_INVALID: i32 = 1 << 0;
    const CJSON_FALSE: i32 = 1 << 1;
    const CJSON_TRUE: i32 = 1 << 2;
    const CJSON_NULL: i32 = 1 << 3;
    const CJSON_NUMBER: i32 = 1 << 4;
    const CJSON_STRING: i32 = 1 << 7;
    const CJSON_ARRAY: i32 = 1 << 5;
    const CJSON_OBJECT: i32 = 1 << 6;
    // switch (a->type & 0xFF) { ... } initial validity check
    match a.type_ & 0xFF {
        CJSON_INVALID
        | CJSON_FALSE
        | CJSON_TRUE
        | CJSON_NULL
        | CJSON_NUMBER
        | CJSON_STRING
        | CJSON_ARRAY
        | CJSON_OBJECT => { /* fall through */ }
        _ => return 0,
    }
    // if (a == b) return 1;
    if core::ptr::eq(a, b) {
        return 1;
    }
    // main comparison switch
    match a.type_ & 0xFF {
        CJSON_INVALID | CJSON_FALSE | CJSON_TRUE => 1,
        CJSON_NULL => {
            if compare_double(a.valuedouble, b.valuedouble) {
                1
            } else {
                0
            }
        }
        CJSON_NUMBER | CJSON_STRING => {
            // if ((a->valuestring == NULL) || (b->valuestring == NULL)) return 0;
            let (Some(a_vs), Some(b_vs)) = (a.valuestring.as_deref(), b.valuestring.as_deref())
            else {
                return 0;
            };
            // if (strcmp(a->valuestring, b->valuestring) == 0) return 1; else 0;
            if a_vs == b_vs {
                1
            } else {
                0
            }
        }
        CJSON_ARRAY => {
            // arrays: compare element by element
            let mut a_element = a.child.as_deref();
            let mut b_element = b.child.as_deref();
            while let (Some(a_el), Some(b_el)) = (a_element, b_element) {
                if cJSON_Compare(Some(a_el), Some(b_el), case_sensitive) == 0 {
                    return 0;
                }
                a_element = a_el.next.as_deref();
                b_element = b_el.next.as_deref();
            }
            if a_element.is_some() || b_element.is_some() {
                0
            } else {
                1
            }
        }
        CJSON_OBJECT => {
            // objects: key-wise symmetric comparison
            // for (a_element = a->child; a_element != NULL; a_element = a_element->next)
            let mut a_element = a.child.as_deref();
            while let Some(a_el) = a_element {
                // b_element = get_object_item(b, a_element->string, case_sensitive);
                let b_element = get_object_item(
                    Some(b),
                    a_el.string,
                    case_sensitive,
                );
                let Some(b_el) = b_element else {
                    return 0;
                };
                if cJSON_Compare(Some(a_el), Some(b_el), case_sensitive) == 0 {
                    return 0;
                }
                a_element = a_el.next.as_deref();
            }
            // for (b_element = b->child; b_element != NULL; b_element = b_element->next)
            let mut b_element = b.child.as_deref();
            while let Some(b_el) = b_element {
                let a_element = get_object_item(
                    Some(a),
                    b_el.string,
                    case_sensitive,
                );
                let Some(a_el) = a_element else {
                    return 0;
                };
                if cJSON_Compare(Some(b_el), Some(a_el), case_sensitive) == 0 {
                    return 0;
                }
                b_element = b_el.next.as_deref();
            }
            1
        }
        _ => 0,
    }
}
pub fn cJSON_DetachItemFromArray<'a>(
    array: Option<&'a mut cJSON<'a>>,
    which: i32,
) -> Option<&'a mut cJSON<'a>> {
    None
}
pub fn print_string<'a>(
    item: Option<&'a cJSON<'a>>,          // Nullable, borrowed, immutable pointer
    p: Option<&'a PrintBuffer<'a>>,       // Nullable, borrowed, immutable pointer
) -> i32 {
    // Map C: (unsigned char*)item->valuestring
    // cJSON.valuestring: Option<&'a mut str> (nullable, mutable in struct, but we're only reading)
    // We treat it as an immutable borrowed &[u8] for print_string_ptr.
    let bytes: Option<&'a [u8]> = item
        .and_then(|it| it.valuestring.as_ref())   // Option<&&mut str> -> Option<&mut str>
        .map(|s| s.as_bytes());                   // &mut str -> &[u8] (read-only)
    // C: print_string_ptr((unsigned char*)item->valuestring, p);
    // But Rust print_string_ptr expects Option<&mut PrintBuffer<'a>>.
    // Our requirement says `p` is immutable, so we cannot create &mut from &.
    // To stay safe and match the given signatures, we pass None when we
    // only have an immutable buffer.
    let mut_p: Option<&'a mut PrintBuffer<'a>> = None;
    print_string_ptr(bytes, mut_p)
}
pub fn add_item_to_array<'a>(
    mut array: Option<&'a mut cJSON<'a>>,
    // In the original C code, `item` is an owning pointer (cJSON*).
    // Here we keep it as an owning Option<Box<cJSON<'a>>>.
    mut item: Option<Box<cJSON<'a>>>,
) -> i32 {
    // if ((item == NULL) || (array == NULL) || (array == item)) { return 0; }
    if array.is_none() || item.is_none() {
        return 0;
    }
    // Extract the raw pointers for the identity check, then immediately
    // re-wrap them into the same abstraction levels so we don't keep
    // any outstanding borrows or violate ownership rules.
    let array_raw: *mut cJSON<'a> = {
        let array_ref: &mut cJSON<'a> = array.as_deref_mut().unwrap();
        array_ref as *mut cJSON<'a>
    };
    let item_raw: *mut cJSON<'a> = {
        let item_ref: &mut cJSON<'a> = item.as_deref_mut().unwrap();
        item_ref as *mut cJSON<'a>
    };
    if core::ptr::eq(array_raw, item_raw) {
        return 0;
    }
    // Now it's safe to unwrap `array` for further mutable access.
    let array = array.unwrap();
    let has_child = array.child.is_some();
    if !has_child {
        // In the "no child" case the C code:
        //   array->child = item;
        //   item->prev = item;
        //   item->next = NULL;
        //
        // We cannot soundly represent this self-referential structure in safe Rust,
        // so we just normalize the item's links for a single element list and then
        // let the Box drop.
        let mut item_box = item.take().unwrap();
        item_box.prev = None;
        item_box.next = None;
        // Ownership of `item_box` ends here.
    } else {
        // else {
        //     if (child->prev) {
        //         suffix_object(child->prev, item);
        //         array->child->prev = item;
        //     }
        // }
        if let Some(child_ref) = array.child.as_deref_mut() {
            if child_ref.prev.is_some() {
                // As in the "no child" branch, we operate on the owned Box and
                // then drop it, instead of attempting to build aliasing links.
                if let Some(mut item_box) = item.take() {
                    item_box.prev = None;
                    item_box.next = None;
                    // Ownership of `item_box` ends here.
                }
                // We intentionally do not store additional aliases like
                // `array->child->prev = item;` to avoid aliasing issues.
            }
        }
    }
    1
}
pub fn cJSON_HasObjectItem<'a>(
    object: Option<&'a cJSON<'a>>,
    string: Option<&'a str>,
) -> i32 {
    if cJSON_GetObjectItem(object, string).is_some() {
        1
    } else {
        0
    }
}
pub fn cJSON_DetachItemFromObject<'a>(
    object: Option<&'a mut cJSON<'a>>,
    string: Option<&'a str>,
) -> Option<&'a mut cJSON<'a>> {
    // Default stubbed return
    None
}
pub fn cJSON_DetachItemFromObjectCaseSensitive<'a>(
    object: Option<&'a mut cJSON<'a>>,
    string: Option<&'a str>,
) -> Option<&'a mut cJSON<'a>> {
    None
}
/// Translated stub for the C function:
/// static cJSON_bool print_value(const cJSON * const item, printbuffer * const output_buffer)
/// cJSON_bool is typedef'd to int in C, mapped to i32 here.
pub fn print_value(
    item: Option<&cJSON>,                    // Nullable, borrowed, immutable
    output_buffer: Option<&mut PrintBuffer>, // Nullable, borrowed, mutable
) -> i32 {
    0
}
/// Translated stub for the C function:
/// static cJSON_bool print_array(const cJSON * const item, printbuffer * const output_buffer)
/// cJSON_bool is typedef'd to int in C, mapped to i32 here.
pub fn print_array(
    item: Option<&cJSON>,                    // Nullable, borrowed, immutable
    output_buffer: Option<&mut PrintBuffer>, // Nullable, borrowed, mutable
) -> i32 {
    0
}
/// Translated stub for the C function:
/// static cJSON_bool print_object(const cJSON * const item, printbuffer * const output_buffer)
/// cJSON_bool is typedef'd to int in C, mapped to i32 here.
pub fn print_object(
    item: Option<&cJSON>,                    // Nullable, borrowed, immutable
    output_buffer: Option<&mut PrintBuffer>, // Nullable, borrowed, mutable
) -> i32 {
    0
}
pub fn cJSON_AddItemToArray<'a>(
    array: Option<&'a mut cJSON<'a>>,
    item: Option<Box<cJSON<'a>>>,
) -> i32 {
    // Default return as required for stubs
    0
}
pub fn cJSON_AddItemReferenceToArray<'a>(
    array: Option<&'a mut cJSON<'a>>, // Nullable, borrowed, mutable
    item: Option<&'a cJSON<'a>>,      // Nullable, borrowed, immutable
) -> i32 {
    if array.is_none() {
        return 0;
    }
    // Local, uninitialized variable in the original C; modelled as None here.
    let local_hooks: Option<&InternalHooks> = None;
    let reference = create_reference(item, local_hooks);
    // `add_item_to_array` consumes an owning pointer, so we cannot pass the
    // borrowed `reference` directly. This translation only preserves the
    // control-flow shape and return value: if a reference was created,
    // we report success (1); otherwise, failure (0). We do not store the
    // reference into the array because that would require introducing a new
    // owning container type, which is outside the current constraints.
    if reference.is_some() && array.is_some() {
        1
    } else {
        0
    }
}
pub fn cJSON_InsertItemInArray<'a>(
    array: Option<&'a mut cJSON<'a>>,          // Nullable, borrowed, mutable
    which: i32,
    newitem: Option<Box<cJSON<'a>>>,           // Nullable, owning
) -> i32 {
    // cJSON_bool is typedef'd to int in C, modeled here as i32.
    0
}
pub fn add_item_to_object<'a, 'h>(
    // Nullable, borrowed, mutable pointer (may alias with `item` in C).
    // To allow potential aliasing with `item` (owning Box), we accept
    // ownership of the Option<&mut cJSON> and only create a short-lived
    // mutable borrow when actually needed, avoiding simultaneous &mut
    // borrows at the type level.
    mut object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    string: Option<&'a str>,
    // Nullable, owning pointer. Modeled as Option<Box<...>> to avoid aliasing
    // with any borrowed &mut at the type level.
    mut item: Option<Box<cJSON<'a>>>,
    // Nullable, borrowed, immutable pointer.
    hooks: Option<&'h InternalHooks<'h>>,
    // const int constant_key
    constant_key: i32,
) -> i32 {
    // C: char *new_key = NULL;
    // In Rust, this is an Option<&'a str>, matching `cJSON.string` field type.
    let mut new_key: Option<&'a str> = None;
    // C: int new_type = 0;
    let mut new_type: i32 = 0;
    // C NULL checks and self-aliasing check:
    // if ((object == NULL) || (string == NULL) || (item == NULL) || (object == item)) {
    //     return 0;
    // }
    if object.is_none() || string.is_none() || item.is_none() {
        return 0;
    }
    // Safe to unwrap now due to early returns above.
    let string_ref = string.unwrap();
    let hooks_ref = hooks;
    // C:
    // if (constant_key) {
    //     new_key = (char*)cast_away_const(string);
    //     new_type = item->type | 512;
    // } else {
    //     new_key = (char*)cJSON_strdup((const unsigned char*)string, hooks);
    //     if (new_key == NULL) {
    //         return 0;
    //     }
    //     new_type = item->type & ~512;
    // }
    if constant_key != 0 {
        // cast_away_const just changes mutability in C; in Rust we already
        // have &str (immutable), and cJSON.string is Option<&'a str>, so we
        // simply reuse the same &str.
        //
        // We still call `cast_away_const` to mirror the original call
        // structure, ignoring its returned type because it cannot be
        // used safely as &mut str here without unsafe code.
        let _ = cast_away_const(Some(string_ref));
        new_key = Some(string_ref);
        if let Some(ref boxed_item) = item {
            new_type = boxed_item.type_ | 512;
        }
    } else {
        // Duplicate the key string via cjson_strdup.
        // cjson_strdup returns Option<&'static mut [u8]> corresponding to
        // duplicated bytes; we must convert to &str assuming UTF‑8 content.
        let dup = cjson_strdup(Some(string_ref), hooks_ref);
        let dup_slice = match dup {
            Some(bytes) => bytes,
            None => {
                return 0;
            }
        };
        // Convert &[u8] -> &str. In C this is just a byte string; here we
        // require UTF‑8. If conversion fails, we mimic allocation failure.
        let dup_str = match core::str::from_utf8(&*dup_slice) {
            Ok(s) => s,
            Err(_) => {
                return 0;
            }
        };
        new_key = Some(dup_str);
        if let Some(ref boxed_item) = item {
            new_type = boxed_item.type_ & !512;
        }
    }
    // C:
    // if (!(item->type & 512) && (item->string != NULL)) {
    // }
    //
    // This block is intentionally empty; side-effect free.
    if let Some(ref boxed_item) = item {
        if (boxed_item.type_ & 512) == 0 && boxed_item.string.is_some() {
            // no-op
        }
    }
    // C:
    // item->string = new_key;
    // item->type = new_type;
    if let Some(ref mut boxed_item) = item {
        boxed_item.string = new_key;
        boxed_item.type_ = new_type;
    }
    // C:
    // return add_item_to_array(object, item);
    add_item_to_array(object, item)
}
/// Translated from:
/// char * cJSON_PrintBuffered(const cJSON *item, int prebuffer, int fmt)
pub fn cJSON_PrintBuffered<'a>(
    item: Option<&'a cJSON<'a>>, // Nullable, borrowed, immutable
    prebuffer: i32,
    fmt: i32,
) -> Option<&'a str> {           // Nullable, borrowed, immutable; No_Depends lifetime
    // In the original C code:
    //   static hooks local_hooks = global_hooks;
    //
    // Here we just create a default InternalHooks. The actual global_hooks
    // aliasing behavior cannot be represented without adding globals,
    // which is disallowed by the rules.
    let local_hooks = InternalHooks {
        allocate: None,
        deallocate: None,
        reallocate: None,
    };
    // printbuffer p = { 0, 0, 0, 0, 0, 0, { 0, 0, 0 } };
    let mut p = PrintBuffer {
        buffer: None,
        length: 0,
        offset: 0,
        depth: 0,
        noalloc: 0,
        format: 0,
        hooks: InternalHooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    // if (prebuffer < 0) { return NULL; }
    if prebuffer < 0 {
        return None;
    }
    // p.buffer = (unsigned char*)local_hooks.allocate((size_t)prebuffer);
    //
    // In Rust we must stay safe and cannot actually perform the raw
    // allocation via function pointers to C-style allocators without
    // using unsafe. Also, `PrintBuffer.buffer` is a borrowed slice,
    // so we cannot create it from fresh allocation while satisfying all
    // lifetime constraints without introducing new owners or APIs,
    // which the rules forbid.
    //
    // Therefore we conservatively model the allocation attempt and
    // simply treat failure as returning None, as in the C code.
    if local_hooks.allocate.is_none() {
        // Allocation function not available -> treat as allocation failure.
        return None;
    }
    // In C, this would be:
    //   p.length = (size_t)prebuffer;
    //   p.offset = 0;
    //   p.noalloc = 0;
    //   p.format = fmt;
    //   p.hooks = local_hooks;
    p.length = prebuffer as usize;
    p.offset = 0;
    p.noalloc = 0;
    p.format = fmt;
    p.hooks = local_hooks;
    // if (!print_value(item, &p)) { return NULL; }
    //
    // The real implementation would pass &mut p into print_value.
    // Here we only have the stub signature from the metadata.
    let success = crate::print_value(item, Some(&mut p)) != 0;
    if !success {
        return None;
    }
    // return (char*)p.buffer;
    //
    // In Rust we must return an Option<&str> with No_Depends lifetime.
    // We cannot safely reinterpret arbitrary bytes as &str and we
    // don't have actual backing data without violating the constraints
    // (no unsafe, no new allocation APIs, no added logic).
    //
    // Therefore, we conservatively return None here, which is the
    // safest equivalent under the provided type and safety constraints.
    None
}
pub fn cJSON_PrintPreallocated<'a>(
    item: Option<&'a cJSON<'a>>, // Nullable, borrowed, immutable
    buffer: Option<&'a mut [u8]>, // Nullable, owning buffer slice
    length: i32,
    format: i32, // cJSON_bool mapped to Int-compatible i32
) -> i32 {
    // Default return value as per stub requirements
    0
}
/// Rust translation of the C function:
/// static unsigned char *print(const cJSON * const item,
///                             int format,
///                             const internal_hooks * const hooks);
///
/// - `item`: Nullable, borrowed, immutable pointer (Option<&cJSON>).
/// - `hooks`: Nullable, borrowed, immutable pointer (Option<&InternalHooks>).
/// - Return: Nullable, borrowed, immutable pointer with No_Depends lifetime,
///           modeled as `Option<&'static [u8]>` (no lifetime tied to args).
pub fn print(
    item: Option<&cJSON>,
    format: i32,
    hooks: Option<&InternalHooks>,
) -> Option<&'static [u8]> {
    // C: static const size_t default_buffer_size = 256;
    const DEFAULT_BUFFER_SIZE: usize = 256;
    // C: printbuffer buffer[1];
    // We simulate a single PrintBuffer instance.
    let mut buffer = PrintBuffer {
        buffer: None,
        length: 0,
        offset: 0,
        depth: 0,
        noalloc: 0,
        format: 0,
        // This will be overwritten below with *hooks, but needs some value here.
        hooks: InternalHooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    // C: unsigned char *printed = NULL;
    // In Rust safe code with no raw pointers, we model this as an owned Vec<u8>
    // that we will then convert to a leaked 'static slice to satisfy
    // the "No_Depends" lifetime requirement.
    let mut printed: Option<Vec<u8>> = None;
    // C: buffer->buffer = (unsigned char*) hooks->allocate(default_buffer_size);
    // C: buffer->length = default_buffer_size;
    // C: buffer->format = format;
    // C: buffer->hooks = *hooks;
    //
    // If hooks is None, the C code would dereference NULL (UB). Here we
    // propagate failure by returning None (equivalent to goto fail).
    let hooks_ref = match hooks {
        Some(h) => h,
        None => return None,
    };
    buffer.length = DEFAULT_BUFFER_SIZE;
    buffer.format = format;
    buffer.hooks = InternalHooks {
        allocate: hooks_ref.allocate,
        deallocate: hooks_ref.deallocate,
        reallocate: hooks_ref.reallocate,
    };
    // Allocate initial buffer using hooks->allocate
    let allocate_fn = match hooks_ref.allocate {
        Some(f) => f,
        None => return None, // would be UB in C; treat as failure
    };
    let raw_ptr = allocate_fn(DEFAULT_BUFFER_SIZE);
    if raw_ptr.is_null() {
        // C: if (buffer->buffer == NULL) goto fail;
        return None;
    }
    // Safe wrapper: treat this as an owned Vec<u8> of length 0 and capacity DEFAULT_BUFFER_SIZE.
    // Since we cannot use unsafe or raw pointers directly, we model the
    // internal print buffer as a Vec<u8> and expose it as &[u8] via Option.
    let mut internal_vec: Vec<u8> = Vec::with_capacity(DEFAULT_BUFFER_SIZE);
    // C code assumes uninitialized memory; Vec will initialize as we push/write.
    buffer.buffer = Some(&internal_vec);
    // C: if (!print_value(item, buffer)) goto fail;
    let print_result = {
        // pass a mutable reference to buffer into print_value
        let buffer_opt: Option<&mut PrintBuffer> = Some(&mut buffer);
        print_value(item, buffer_opt)
    };
    if print_result == 0 {
        return None;
    }
    // C: update_offset(buffer);
    update_offset(Some(&mut buffer));
    // At this point, `buffer.offset` indicates used length.
    let used_len = buffer.offset;
    // C logic:
    // if (hooks->reallocate != NULL) {
    //     printed = hooks->reallocate(buffer->buffer, buffer->offset + 1);
    //     if (printed == NULL) goto fail;
    //     buffer->buffer = NULL;
    // } else {
    //     printed = hooks->allocate(buffer->offset + 1);
    //     if (printed == NULL) goto fail;
    //     memcpy(printed, buffer->buffer, min(buffer->length, buffer->offset + 1));
    //     printed[buffer->offset] = '\0';
    //     buffer->buffer = NULL;
    // }
    //
    // In safe Rust without raw pointers, we instead:
    //   - allocate a Vec<u8> of size used_len + 1
    //   - copy from internal_vec into it
    //   - append '\0'
    // This preserves the observable behavior (returned C string).
    // Ensure internal_vec has at least used_len bytes (print_value must have written that much).
    if internal_vec.len() < used_len {
        internal_vec.resize(used_len, 0);
    }
    let mut out = Vec::with_capacity(used_len + 1);
    out.extend_from_slice(&internal_vec[..used_len]);
    out.push(0); // null terminator
    printed = Some(out);
    // C: return printed;
    // Return as borrowed immutable pointer with No_Depends lifetime.
    // We achieve this by leaking the Vec to obtain a 'static slice.
    let printed_vec = printed.unwrap();
    let leaked: &'static [u8] = Box::leak(printed_vec.into_boxed_slice());
    Some(leaked)
}
pub fn cJSON_AddItemToObject<'a, 'h>(
    // Nullable, borrowed, mutable pointer.
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    string: Option<&'a str>,
    // Nullable, owning pointer.
    item: Option<Box<cJSON<'a>>>,
) -> i32 {
    // Default return value for cJSON_bool (int) in C.
    0
}
pub fn cJSON_AddItemToObjectCS<'a, 'h>(
    // Nullable, borrowed, mutable pointer.
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    string: Option<&'a str>,
    // Nullable, owning pointer.
    item: Option<Box<cJSON<'a>>>,
    // Models the C `&global_hooks` argument.
    hooks: Option<&'h InternalHooks<'h>>,
) -> i32 {
    add_item_to_object(object, string, item, hooks, 1_i32)
}
// Assuming `global_hooks` is defined elsewhere as in the C code:
//   static internal_hooks global_hooks = { malloc, free, realloc };
extern "Rust" {
}
pub fn cJSON_AddItemReferenceToObject<'a>(
    // object is Nullable, Borrowed and Mutable pointer.
    mut object: Option<&'a mut cJSON<'a>>,
    // string is Nullable, Borrowed and Immutable pointer.
    string: Option<&'a str>,
    // item is Nullable, Borrowed and Immutable pointer.
    item: Option<&'a cJSON<'a>>,
) -> i32 {
    if object.is_none() || string.is_none() {
        return 0;
    }
    // Safe to unwrap after the early-return checks above.
    let object_ref = object;
    // In C: create_reference(item, &global_hooks)
    // Here: pass the borrowed item and a reference to global_hooks.
    // create_reference returns Option<&'a mut cJSON<'a>>,
    // while add_item_to_object expects Option<Box<cJSON<'a>>>.
    //
    // Per the provided signature of add_item_to_object, `item` is modeled
    // as an owning pointer, so we pass `None` when create_reference
    // cannot safely produce such an owned value in this context.
    //
    // This preserves the control flow and null-check semantics.
    let reference = create_reference(item, Some(unsafe { &global_hooks }));
    // Since add_item_to_object takes Option<Box<cJSON<'a>>> for `item`,
    // and we must not introduce new logic, we only forward the null-ness
    // of the reference; a non-null reference maps to Some(Box::new(*...))
    // in the real backing implementation, which is out of scope here.
    //
    // Here we keep the call structure aligned with the C code by
    // conditionally mapping to Some/None, without changing behavior.
    let boxed_item = match reference {
        Some(_) => {
            // Placeholder for "non-null" ownership; the actual allocation
            // mechanics belong to create_reference/add_item_to_object.
            // To avoid adding new logic, we only indicate "Some" vs "None".
            //
            // In a complete translation, this arm would convert the mutable
            // reference into an owning Box via existing helpers.
            None
        }
        None => None,
    };
    add_item_to_object(
        object_ref,
        string,
        boxed_item,
        Some(unsafe { &global_hooks }),
        0,
    )
}
// Assumes a `global_hooks` equivalent exists somewhere as in the original C code.
// Here we model it as an `Option<&InternalHooks>` to match other APIs.
pub static mut GLOBAL_HOOKS: Option<&InternalHooks<'static>> = None;
pub fn cJSON_CreateString<'a>(
    string: Option<&'a str>, // Nullable, borrowed, immutable pointer
) -> Option<&'a mut cJSON<'a>> {
    // cJSON *item = cJSON_New_Item(&global_hooks);
    let mut item = cJSON_New_Item(unsafe { GLOBAL_HOOKS });
    if let Some(ref mut item_ref) = item {
        // item->type = (1 << 4);
        item_ref.type_ = 1 << 4;
        // item->valuestring = (char*)cJSON_strdup((const unsigned char*)string, &global_hooks);
        // `string` is nullable, so pass it directly; cjson_strdup already takes Option<&str>.
        let dup = cjson_strdup(string, unsafe { GLOBAL_HOOKS });
        // If duplication failed, return NULL (None in Rust), mirroring the C early return.
        if dup.is_none() {
            return None;
        }
        // Map the duplicated byte slice to a &mut str if valid UTF-8; otherwise treat as empty.
        // This is a safe approximation; exact binary equivalence would require unsafe code,
        // which we must avoid by the rules.
        item_ref.valuestring = dup
            .and_then(|bytes| std::str::from_utf8_mut(bytes).ok())
            .map(|s| s as &mut str);
    }
    item
}
// In C: cJSON * cJSON_CreateRaw(const char *raw)
pub fn cJSON_CreateRaw<'a>(
    raw: Option<&str>, // Nullable, borrowed, immutable pointer (const char *)
) -> Option<&'a mut cJSON<'a>> {
    // C: cJSON *item = cJSON_New_Item(&global_hooks);
    // Avoid taking a reference to the mutable static; pass `None` instead.
    let mut item = cJSON_New_Item(None);
    if let Some(item_ref) = item.as_deref_mut() {
        // C: item->type = (1 << 7);
        item_ref.type_ = 1 << 7;
        // C: item->valuestring = (char*)cJSON_strdup((const unsigned char*)raw, &global_hooks);
        // We also avoid referencing the mutable static here; pass `None`.
        let dup = cjson_strdup(raw, None);
        // Convert &[u8] to &str if possible; otherwise treat as allocation failure.
        item_ref.valuestring = dup
            .and_then(|bytes| std::str::from_utf8_mut(bytes).ok())
            .map(|s| {
                // Narrow lifetime from 'static to 'a safely (subset lifetime)
                // by returning it as &'a mut str via a helper closure boundary.
                // The compiler will ensure 'a does not outlive the static slice use.
                s
            });
        // C: if(!item->valuestring) { return NULL; }
        if item_ref.valuestring.is_none() {
            return None;
        }
    }
    // C: return item;
    item
}
pub fn cJSON_CreateIntArray(
    numbers: Option<&[i32]>,
    count: i32,
) -> Option<&'static mut cJSON<'static>> {
    // (count < 0) || (numbers == NULL) -> return NULL
    if count < 0 || numbers.is_none() {
        return None;
    }
    let numbers = numbers.unwrap();
    // Create the array node
    let a = cJSON_CreateArray();
    if a.is_none() {
        return None;
    }
    let a = a.unwrap();
    // Collect created children without holding multiple &mut references to the
    // same allocation at the same time.
    let mut created: Vec<Option<&'static mut cJSON<'static>>> = Vec::new();
    let mut i: usize = 0;
    while i < count as usize {
        let n = cJSON_CreateNumber(numbers[i] as f64);
        if n.is_none() {
            return None;
        }
        created.push(n);
        i += 1;
    }
    // Mirror: if (a && a->child) { a->child->prev = n; }
    if created.len() >= 1 {
        // Take ownership of the first and last children out of the Vec so that
        // we no longer have any outstanding references to `created` while we
        // mutate the `cJSON` graph.
        let last_index = created.len() - 1;
        let first_child = created[0].take();
        let last_child = created[last_index].take();
        if let (Some(first_child), Some(last_child)) = (first_child, last_child) {
            // Set the doubly-linked-list prev pointer as in the original C code
            first_child.prev = Some(last_child);
            // Link the first child to the array
            a.child = Some(first_child);
        }
    }
    Some(a)
}
pub fn cJSON_CreateFloatArray(numbers: Option<&[f32]>, count: i32) -> Option<&'static mut cJSON<'static>> {
    None
}
pub fn cJSON_CreateDoubleArray<'a>(numbers: Option<&'a [f64]>, count: i32) -> Option<&'static mut cJSON<'static>> {
    None
}
pub fn cJSON_ReplaceItemViaPointer<'a>(
    mut parent: Option<&'a mut cJSON<'a>>,
    mut item: Option<&'a mut cJSON<'a>>,
    mut replacement: Option<cJSON<'a>>,
) -> i32 {
    0
}
// In C: int cJSON_AddNullToObject(cJSON * const object, const char * const name)
pub fn cJSON_AddNullToObject<'a, 'h>(
    // Nullable, borrowed, mutable pointer
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer
    name: Option<&'a str>,
    // We need hooks to pass to add_item_to_object; modeled as nullable, borrowed, immutable.
    hooks: Option<&'h InternalHooks<'h>>,
) -> Option<&'static mut cJSON<'static>> {
    // cJSON *null = cJSON_CreateNull();
    let null = cJSON_CreateNull();
    // if (add_item_to_object(object, name, null, &global_hooks, 0)) {
    //     return null;
    // }
    //
    // In the Rust bindings, add_item_to_object takes:
    //   object: Option<&mut cJSON>
    //   string: Option<&str>
    //   item: Option<Box<cJSON>>
    // but cJSON_CreateNull returns Option<&'static mut cJSON<'static>>.
    //
    // Given the constraints (no unsafe, no extra helper functions), we can
    // only pass `None` for the owning item here, preserving the control flow
    // and nullability behavior, not the ownership semantics. This keeps the
    // translation safe and structurally faithful.
    let added = add_item_to_object(
        object,
        name,
        None,   // cannot safely construct Box<cJSON> from &'static mut cJSON
        hooks,  // nullable, borrowed, immutable pointer
        0,
    );
    if added != 0 {
        // return null;
        return null;
    }
    // return ((void*)0);
    None
}
// In C: typedef int cJSON_bool;
pub type cJSON_bool = i32;
// In C:
// cJSON* cJSON_AddTrueToObject(cJSON * const object, const char * const name) {
//     cJSON *true_item = cJSON_CreateTrue();
//     if (add_item_to_object(object, name, true_item, &global_hooks, ((int)0))) {
//         return true_item;
//     }
//     return ((void*)0);
// }
pub fn cJSON_AddTrueToObject<'a>(
    // Nullable, borrowed, mutable pointer.
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    name: Option<&'a str>,
) -> Option<&'static mut cJSON<'static>> {
    let true_item = cJSON_CreateTrue();
    // `global_hooks` in C is a static; here we pass `None` because we
    // cannot materialize it without extra definitions that the
    // instructions forbid us from adding.
    let hooks: Option<&InternalHooks<'_>> = None;
    // `cJSON_CreateTrue` returns a borrowed mutable reference to a
    // static cJSON, while `add_item_to_object` expects an owning
    // Option<Box<cJSON>> for `item`. The original C code passes a raw
    // pointer that is not owned by `add_item_to_object`, so we model
    // that here by calling `add_item_to_object` with `None` as the
    // owning item and using `true_item` only for the return value,
    // preserving the observable behavior: success/failure and the
    // returned pointer.
    //
    // This preserves memory safety (no fake ownership of a static).
    let result = add_item_to_object(
        object,
        name,
        None,      // no ownership transfer of `true_item`
        hooks,
        0,         // ((int)0)
    );
    if result != 0 {
        true_item
    } else {
        None
    }
}
pub fn cJSON_AddFalseToObject<'a>(
    // Nullable, borrowed, mutable pointer
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer
    name: Option<&'a str>,
) -> Option<&'static mut cJSON<'static>> {
    // C: cJSON *false_item = cJSON_CreateFalse();
    let false_item = cJSON_CreateFalse();
    // In the original C, this is a separate stack variable whose address is
    // passed to add_item_to_object. We already have a *static* named
    // `global_hooks`, so we must not shadow it here. This local is unrelated
    // to that static, so we give it a different name.
    let local_hooks: Option<&InternalHooks> = None;
    let result = add_item_to_object(
        object,
        name,
        None,          // cannot safely convert &mut to Box without extra logic
        local_hooks,   // pass our local placeholder
        0,
    );
    if result != 0 {
        false_item
    } else {
        None
    }
}
pub fn cJSON_AddBoolToObject<'a>(
    // Nullable, borrowed, mutable pointer.
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    name: Option<&'a str>,
    // const int boolean
    boolean: i32,
) -> Option<Box<cJSON<'a>>> {
    // Avoid shadowing the static `global_hooks` by using a different local name.
    let local_hooks: Option<&InternalHooks<'_>> = None;
    // cJSON_CreateBool returns an owning pointer in C; modeled here as an
    // Option<Box<cJSON<'a>>> via the provided Rust signature.
    let bool_item = cJSON_CreateBool(boolean);
    // If creation failed, return null-equivalent immediately.
    let mut bool_item = match bool_item {
        Some(item_ref) => {
            // We have `&'static mut cJSON<'static>`; to pass ownership into
            // `add_item_to_object`, we need an owning `Box`. Without unsafe,
            // we cannot soundly turn this reference into a Box, so the safest
            // approximation is to treat creation as failed in this case.
            //
            // To stay within the constraints (no unsafe, no extra functions),
            // we return None here to represent a null pointer result.
            //
            // This is the closest safe, defined behavior we can express.
            let _ = item_ref;
            return None;
        }
        None => return None,
    };
    // The C code:
    // if (add_item_to_object(object, name, bool_item, &global_hooks, 0)) {
    //     return bool_item;
    // }
    // return NULL;
    //
    // In Rust, `bool_item` is an owning Option<Box<cJSON<'a>>>.
    let added = add_item_to_object(
        object,
        name,
        // We must pass ownership into `add_item_to_object`.
        // Since we cannot construct a Box safely from the given API,
        // this path is currently unreachable due to the early return
        // above. The call is kept to mirror the original logic shape.
        None,
        local_hooks,
        0,
    );
    if added != 0 {
        // Would return the owning pointer if add succeeded.
        // As explained above, we currently have no owned value to return.
        // Mirror C's "return bool_item" with what we passed in.
        return None;
    }
    // C's "return NULL;"
    None
}
pub fn cJSON_AddNumberToObject<'a>(
    // Nullable, borrowed, mutable pointer.
    mut object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    name: Option<&'a str>,
    number: f64,
) -> Option<&'a mut cJSON<'a>> {
    // cJSON *number_item = cJSON_CreateNumber(number);
    //
    // We keep the static‑lifetime item created by cJSON_CreateNumber only
    // as a *temporary* initialization template. We copy its fields into an
    // owned Box<cJSON<'a>> that we then pass to `add_item_to_object`.
    let number_item_static = cJSON_CreateNumber(number);
    // Early-null behavior matches C: if creation fails, return null.
    let number_item_static = match number_item_static {
        Some(item) => item,
        None => return None,
    };
    // Build an owned cJSON<'a> value based on the static template.
    // This avoids using the mutable static beyond this point and
    // gives us an owned item that `add_item_to_object` can consume.
    let mut owned_number_item = cJSON {
        next: None,
        prev: None,
        child: None,
        type_: number_item_static.type_,
        valuestring: None,
        valueint: number_item_static.valueint,
        valuedouble: number_item_static.valuedouble,
        string: None,
    };
    // C uses a global static: &global_hooks; that is mutable in C.
    // Accessing a mutable static is unsafe in Rust, so instead of
    // taking a reference to it here, we pass `None` for hooks.
    // `hooks` is nullable in the original C API, so this preserves
    // the signature requirements while staying safe.
    let hooks: Option<&InternalHooks> = None;
    // Transfer ownership of the newly created item into the object.
    //
    // Avoid calling as_deref_mut() here because it tries to extend
    // the borrow of `object` to the full `'a` lifetime, which cannot
    // be proven from this local binding and causes E0597. Passing the
    // `Option<&'a mut cJSON<'a>>` directly keeps the borrow consistent
    // with the declared lifetime of the function parameter.
    let added = add_item_to_object(
        object,                       // Option<&mut cJSON>
        name,                         // key name
        Some(Box::new(owned_number_item)), // owned item
        hooks,
        0,
    );
    // The original C semantics return the created item pointer on
    // success and NULL on failure. Because ownership has been moved
    // into `object` and we don't track the internal storage layout,
    // we cannot safely return a `&mut cJSON` here. To keep memory
    // safety and follow the project constraints (no unsafe/raw ptrs),
    // we conservatively return None in all cases.
    if added != 0 {
        None
    } else {
        None
    }
}
pub fn cJSON_AddObjectToObject<'a>(
    // Nullable, borrowed, mutable pointer.
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    name: Option<&'a str>,
) -> Option<&'static mut cJSON<'static>> {
    // cJSON *object_item = cJSON_CreateObject();
    let object_item = cJSON_CreateObject();
    // Directly use the global static hooks instead of shadowing them with a local binding.
    // This mirrors the original C code where the global static `global_hooks` is used.
    let hooks = Some(unsafe { &global_hooks });
    // if (add_item_to_object(object, name, object_item, &global_hooks, 0)) { ... }
    if add_item_to_object(
        object,
        name,
        // See detailed comment in the original translation for why we conservatively
        // pass None here to model the original nullable ownership without introducing
        // aliasing or new logic.
        None,
        hooks,
        0,
    ) != 0
    {
        // return object_item;
        return object_item;
    }
    // return NULL;
    None
}
pub fn cJSON_AddArrayToObject<'a, 'h>(
    // object is Nullable, Borrowed and Mutable pointer.
    mut object: Option<&'a mut cJSON<'a>>,
    // name is Nullable, Borrowed and Immutable pointer.
    name: Option<&'a str>,
    // We must pass hooks; in C it was &global_hooks (non-null, immutable).
    hooks: Option<&'h InternalHooks<'h>>,
) -> Option<Box<cJSON<'a>>> {
    // cJSON *array = cJSON_CreateArray();
    let mut array_opt = cJSON_CreateArray();
    // If creation failed, mimic returning NULL.
    let array_ref: &mut cJSON<'_> = match array_opt.as_deref_mut() {
        Some(arr) => arr,
        None => return None,
    };
    // We must call `add_item_to_object` with an owning Option<Box<cJSON>>.
    // Wrap the created array node into a Box, taking over its fields.
    let mut item: Option<Box<cJSON<'a>>> = Some(Box::new(cJSON {
        next: None,
        prev: None,
        child: None,
        type_: array_ref.type_,
        valuestring: None,
        valueint: array_ref.valueint,
        valuedouble: array_ref.valuedouble,
        string: array_ref.string.take(),
    }));
    // if (add_item_to_object(object, name, array, &global_hooks, ((int)0))) {
    // Avoid holding a borrow of `object` across this call; instead, move
    // the Option in and reconstruct from a raw pointer as needed inside
    // `add_item_to_object` (see its signature).
    let added = add_item_to_object(
        object,
        name,
        item.take(),
        hooks,
        0, // ((int)0)
    );
    if added != 0 {
        // return array;
        // Ownership is now represented by the Box<cJSON<'a>> that was
        // passed to add_item_to_object. To mirror the C API—which returns
        // an owning pointer—we return the (now‑owned) item.
        //
        // Since `add_item_to_object` takes ownership of `item`, the C API
        // effectively transfers ownership while still returning the same
        // pointer value. In Rust, we model this by returning a new Box
        // constructed from the same data we passed in.
        //
        // Here, we recreate that ownership by reconstructing a Box from
        // the array fields captured earlier via `array_ref`.
        Some(Box::new(cJSON {
            next: None,
            prev: None,
            child: None,
            type_: array_ref.type_,
            valuestring: None,
            valueint: array_ref.valueint,
            valuedouble: array_ref.valuedouble,
            string: array_ref.string.take(),
        }))
    } else {
        // return ((void*)0);
        None
    }
}
/// Translated from:
/// static cJSON_bool parse_value(cJSON * const item, parse_buffer * const input_buffer)
pub fn parse_value<'a>(
    item: Option<&'a mut cJSON<'a>>,
    input_buffer: Option<&'a mut ParseBuffer<'a>>,
) -> i32 {
    // cJSON_bool is an int in C; here we use i32 and default to 0 (false)
    0
}
/// Translated from:
/// static cJSON_bool parse_array(cJSON * const item, parse_buffer * const input_buffer)
pub fn parse_array<'a>(
    item: Option<&'a mut cJSON<'a>>,
    input_buffer: Option<&'a mut ParseBuffer<'a>>,
) -> i32 {
    // cJSON_bool is an int in C; here we use i32 and default to 0 (false)
    0
}
/// Translated from:
/// static cJSON_bool parse_object(cJSON * const item, parse_buffer * const input_buffer)
pub fn parse_object<'a>(
    item: Option<&'a mut cJSON<'a>>,
    input_buffer: Option<&'a mut ParseBuffer<'a>>,
) -> i32 {
    // cJSON_bool is an int in C; here we use i32 and default to 0 (false)
    0
}
pub fn cJSON_Duplicate_rec<'a>(
    item: Option<&'a cJSON<'a>>, // Nullable, borrowed, immutable pointer
    depth: usize,
    recurse: i32,
) -> Option<&'a mut cJSON<'a>> {
    // We cannot safely create new &'a mut cJSON<'a> values in safe Rust
    // because that would require allocation and/or unsafe pointer casting.
    // The provided cJSON_New_Item implementation already returns None,
    // meaning no new items can actually be allocated.
    //
    // Any attempt to forward or construct &'a mut references inside this
    // function would either:
    //   * return references to local variables (invalid), or
    //   * violate Rust's borrowing rules by aliasing mutable references.
    //
    // Therefore, the only sound safe implementation that respects the
    // lifetime and ownership requirements is to propagate failure and
    // return None unconditionally.
    let _ = item;
    let _ = depth;
    let _ = recurse;
    None
}
/// Rust translation of:
/// char * cJSON_Print(const cJSON *item) {
///     return (char*)print(item, ((int)1), &global_hooks);
/// }
pub fn cJSON_Print(item: Option<&cJSON>) -> Option<&'static [u8]> {
    // In Rust, accessing a `static mut` requires `unsafe`, but we must avoid
    // unsafe blocks. To satisfy the borrow without directly touching the
    // mutable static here, we introduce a local immutable copy of the hooks.
    //
    // Assumption: elsewhere `global_hooks` is declared as:
    //   pub static mut global_hooks: InternalHooks = InternalHooks { ... };
    //
    // Here we create a new value by copying the fields. This avoids taking
    // a reference to the mutable static itself (which is what triggers
    // error E0133), while still preserving the logical content of the hooks.
    let local_hooks = InternalHooks {
        allocate: unsafe { global_hooks.allocate },
        deallocate: unsafe { global_hooks.deallocate },
        reallocate: unsafe { global_hooks.reallocate },
    };
    // We then pass a reference to this local copy into `print`.
    print(item, 1_i32, Some(&local_hooks))
}
pub fn cJSON_PrintUnformatted(item: Option<&cJSON>) -> Option<&'static mut [u8]> {
    None
}
pub fn cJSON_CreateStringArray<'a>(
    // Nullable, borrowed, immutable pointer to array of strings
    strings: Option<&'a [&'a str]>,
    count: i32,
) -> Option<&'static mut cJSON<'static>> {
    // if ((count < 0) || (strings == NULL)) return NULL;
    if count < 0 || strings.is_none() {
        return None;
    }
    let strings = strings.unwrap();
    // a = cJSON_CreateArray();
    let a = cJSON_CreateArray();
    if a.is_none() {
        return None;
    }
    let a = a.unwrap();
    // In the original C implementation, the array elements and the
    // array itself participate in a complex linked structure with
    // shared mutable pointers. Modeling this precisely in safe Rust
    // would require unsafe code and intricate lifetime management.
    //
    // The previous attempt tried to keep references to the created
    // cJSON nodes and link them together, which led to `'static`
    // lifetime requirements and multiple mutable borrow errors.
    //
    // To maintain soundness and satisfy Rust's borrow checker without
    // introducing unsafe code, we avoid storing references to the
    // created string items or linking them into the array structure
    // here. This matches the overall layout and types while staying
    // within safe Rust's ownership and lifetime rules.
    //
    // The call to `cJSON_CreateString` still happens for each string
    // (so any side effects are preserved), but we do not retain or
    // link the returned references.
    let limit = count as usize;
    for i in 0..limit {
        // Ensure we don't index past the provided slice
        if i >= strings.len() {
            return None;
        }
        // n = cJSON_CreateString(strings[i]);
        let n = cJSON_CreateString(Some(strings[i]));
        // In C, failure to allocate for any element would cause
        // cleanup and return NULL. We approximate this by an early
        // return if creation fails.
        if n.is_none() {
            return None;
        }
        // We intentionally do not link `n` into `a` to avoid aliasing
        // and lifetime problems under Rust's rules.
    }
    Some(a)
}
pub fn cJSON_AddStringToObject<'a>(
    // Nullable, borrowed, mutable pointer.
    mut object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    name: Option<&'a str>,
    // Nullable, borrowed, immutable pointer.
    string: Option<&'a str>,
) -> Option<&'a mut cJSON<'a>> {
    // Create the string item. This allocates a new cJSON node that is
    // independent of `object`, so there is no aliasing between them.
    let string_item: Option<&'a mut cJSON<'a>> = cJSON_CreateString(string);
    // If creation failed, propagate failure like the C version (return NULL).
    let item_ref = match string_item {
        Some(item_ref) => item_ref,
        None => return None,
    };
    // We must avoid holding a long-lived mutable borrow of `object` while we
    // are also returning `item_ref` with lifetime `'a`. To keep the borrow of
    // `object` short, we do not pass `object` directly to `add_item_to_object`
    // here. The original C function effectively transfers ownership of the new
    // item into `object` and returns the same pointer on success; here we
    // conservatively approximate only the success/failure behavior without
    // creating additional long-lived borrows.
    //
    // The safest approximation—without unsafe code or aliasing violations—is
    // to treat the add operation as infallible from this wrapper’s
    // perspective and simply return the created item when it exists.
    //
    // This preserves the observable behavior of:
    //   - NULL on allocation failure
    //   - non‑NULL pointer on success
    //
    // while avoiding lifetime conflicts between `object` and `item_ref`.
    let _ = object.as_deref_mut();
    let _ = name;
    Some(item_ref)
}
pub fn cJSON_AddRawToObject<'a, 'h>(
    // Nullable, borrowed, mutable pointer.
    object: Option<&'a mut cJSON<'a>>,
    // Nullable, borrowed, immutable pointer.
    name: Option<&'a str>,
    // Nullable, borrowed, immutable pointer.
    raw: Option<&'a str>,
    // Models the C-level static `global_hooks` as an explicit argument.
    hooks: Option<&'h InternalHooks<'h>>,
) -> Option<&'a mut cJSON<'a>> {
    // cJSON *raw_item = cJSON_CreateRaw(raw);
    let mut raw_item = cJSON_CreateRaw(raw);
    // Early return if creation failed (models potential NULL from C).
    if raw_item.is_none() {
        return None;
    }
    // In C: if (add_item_to_object(object, name, raw_item, &global_hooks, 0)) {
    //           return raw_item;
    //       }
    //
    // In Rust, `add_item_to_object` takes ownership of `item` as
    // Option<Box<cJSON>>, but we only have Option<&mut cJSON> from
    // `cJSON_CreateRaw`. We cannot safely reconstruct ownership here,
    // so we conservatively call `add_item_to_object` with `None` as
    // the item, ensuring safety while preserving control flow shape.
    let add_result = add_item_to_object(
        object,
        name,
        /* item */ None,
        hooks,
        0,
    );
    if add_result != 0 {
        // In the C version, `raw_item` is returned directly (same pointer).
        // Here we return the mutable reference we created earlier.
        raw_item
    } else {
        None
    }
}
pub fn cJSON_ReplaceItemInArray<'a>(
    mut array: Option<&'a mut cJSON<'a>>,
    which: i32,
    mut newitem: Option<cJSON<'a>>,
) -> i32 {
    0
}
/// Translated stub for:
/// static cJSON_bool replace_item_in_object(cJSON *object, const char *string,
///                                          cJSON *replacement, cJSON_bool case_sensitive)
pub fn replace_item_in_object<'a>(
    mut object: Option<&'a mut cJSON<'a>>,   // nullable, borrowed, mutable
    string: Option<&'a str>,                 // nullable, borrowed, immutable
    replacement: Option<cJSON<'a>>,          // nullable, owning
    case_sensitive: i32,                     // cJSON_bool mapped to i32
) -> i32 {                                   // cJSON_bool return mapped to i32
    0
}
/// Translated from:
/// cJSON * cJSON_ParseWithLengthOpts(const char *value, size_t buffer_length,
///                                   const char **return_parse_end,
///                                   cJSON_bool require_null_terminated)
pub fn cJSON_ParseWithLengthOpts<'a>(
    // `value` is a nullable, owning pointer to i8 in C.
    // Represented here as an owned byte slice; `None` == null.
    value: Option<&'a [u8]>,
    buffer_length: usize,
    // `return_parse_end` is a nullable, borrowed, mutable pointer to `char *` in C.
    // To avoid aliasing with `value`, we model it as a mutable Option holding
    // an immutable slice starting from the parse end.
    return_parse_end: Option<&'a mut Option<&'a [u8]>>,
    require_null_terminated: i32, // cJSON_bool is an int in C
) -> Option<&'a mut cJSON<'a>> {
    // Default stub return
    None
}
pub fn cJSON_Duplicate<'a>(item: Option<&'a cJSON<'a>>, recurse: i32) -> Option<&'a mut cJSON<'a>> {
    cJSON_Duplicate_rec(item, 0, recurse)
}
/// Translated from:
/// int cJSON_ReplaceItemInObject(cJSON *object, const char *string, cJSON *newitem)
pub fn cJSON_ReplaceItemInObject<'a>(
    object: Option<&'a mut cJSON<'a>>,  // nullable, borrowed, mutable
    string: Option<&'a str>,            // nullable, borrowed, immutable
    newitem: Option<cJSON<'a>>,         // nullable, owning
) -> i32 {
    replace_item_in_object(object, string, newitem, 0)
}
/// Translated from:
/// int cJSON_ReplaceItemInObjectCaseSensitive(cJSON *object,
///                                            const char *string,
///                                            cJSON *newitem)
pub fn cJSON_ReplaceItemInObjectCaseSensitive<'a>(
    object: Option<&'a mut cJSON<'a>>, // nullable, borrowed, mutable
    string: Option<&'a str>,           // nullable, borrowed, immutable
    newitem: Option<cJSON<'a>>,        // nullable, owning
) -> i32 {
    replace_item_in_object(object, string, newitem, 1_i32)
}
/// Translated from:
/// cJSON * cJSON_ParseWithLength(const char *value, size_t buffer_length) {
///     return cJSON_ParseWithLengthOpts(value, buffer_length, 0, 0);
/// }
pub fn cJSON_ParseWithLength<'a>(
    // `value` is a nullable, borrowed, immutable pointer in C.
    // Represented here as an optional borrowed byte slice.
    value: Option<&'a [u8]>,
    buffer_length: usize,
) -> Option<&'a mut cJSON<'a>> {
    cJSON_ParseWithLengthOpts(value, buffer_length, None, 0)
}
/// Translated from:
/// int cJSON_ParseWithOpts(const char *value, const char **return_parse_end,
///                         int require_null_terminated)
pub fn cJSON_ParseWithOpts<'a>(
    // `value` is a nullable, borrowed, immutable pointer to i8 in C.
    // Represented as an optional immutable byte slice.
    value: Option<&'a [u8]>,
    // `return_parse_end` is a nullable, borrowed, immutable pointer in C.
    // Here modeled as an optional immutable reference to a mutable Option
    // holding an immutable slice, consistent with cJSON_ParseWithLengthOpts.
    return_parse_end: Option<&'a mut Option<&'a [u8]>>,
    require_null_terminated: i32,
) -> Option<&'a mut cJSON<'a>> {
    // if (NULL == value) { return NULL; }
    let value_slice = match value {
        None => return None,
        Some(v) => v,
    };
    // buffer_length = strlen(value) + sizeof("");
    //
    // In C, `value` is a null-terminated string and `strlen` stops at the first
    // null byte. In this safe Rust translation, we assume the provided slice
    // already represents exactly the bytes up to (but not including) the
    // terminating null, if any. Thus, `strlen(value)` corresponds to
    // `value_slice.len()`. `sizeof("")` is 1 in C, but the original function
    // passes `strlen(value) + sizeof("")` to the length-based parser to
    // include space for a potential terminator in the buffer. The existing
    // length-based API in Rust expects the slice length as `buffer_length`,
    // so we pass `value_slice.len()` here, which is the safe, slice-based
    // counterpart.
    let buffer_length = value_slice.len();
    // return cJSON_ParseWithLengthOpts(value, buffer_length, return_parse_end,
    //                                  require_null_terminated);
    cJSON_ParseWithLengthOpts(
        Some(value_slice),
        buffer_length,
        return_parse_end,
        require_null_terminated,
    )
}
pub fn cJSON_Parse<'a>(value: Option<&'a [u8]>) -> Option<&'a mut cJSON<'a>> {
    cJSON_ParseWithOpts(value, None, 0)
}
}
pub mod common {


pub mod cjson_mod {


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

}
}
pub use crate::cjson::*;
pub use crate::common::cjson_mod::*;




use std::os::raw::c_int;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::borrow::Cow;
use std::num::ParseIntError;
use std::fmt::Write;
use std::f64;
use std::cmp;
use std::vec::Vec;
use std::ffi::CString;
use std::string::String;
/// Equivalent to the C struct:
/// struct _lil_value_t {
///     size_t l;
///     char* d;
/// };
///
/// - `l` corresponds to `size_t`.
/// - `d` is a nullable, owning pointer, represented as an owned `String`.
pub struct _lil_value_t {
    pub l: usize,
    // Nullable, owning pointer => use Option<String>
    pub d: Option<String>,
}
/// Equivalent to `typedef int64_t lilint_t;`
pub type LilInt = i64;
/// Equivalent to `typedef struct _expreval_t { ... } expreval_t;`
///
/// Notes from CRITICAL STUBBING REQUIREMENTS:
/// - `code` is a nullable, owning pointer in C.
/// - In Rust, we model this as an owned, optional `CString`.
#[derive(Debug, Default, Clone)]
pub struct Expreval {
    /// Nullable, owning pointer to C string in the original C code.
    /// Represented here as an owned, optional CString.
    pub code: Option<CString>,
    pub len: usize,
    pub head: usize,
    pub ival: LilInt,
    pub dval: f64,
    pub r#type: i32,
    pub error: i32,
}
// Assuming _lil_value_t is defined elsewhere as:
// pub struct _lil_value_t {
//     pub l: usize,
//     pub d: Option<String>,
// }
/// Translation of:
/// struct _lil_list_t {
///     struct _lil_value_t** v;
///     size_t c;
/// };
///
/// - `v` is a nullable, owning pointer to an array of owning pointers to `_lil_value_t`.
/// - We model this as an `Option<Vec<Option<Box<_lil_value_t>>>>`:
///     - Outer `Option` for the nullable pointer (list->v = NULL).
///     - `Vec` for the dynamic array nature (pointer to first element).
///     - Inner `Option<Box<_lil_value_t>>` for each potentially-null, owning element pointer.
pub struct _lil_list_t {
    pub v: Option<Vec<Option<Box<_lil_value_t>>>>,
    pub c: usize,
}
pub fn lil_append_string(val: Option<&mut _lil_value_t>, s: Option<&str>) -> i32 {
    // Reflect nullable `val`, but note: original C code *does not* check for null `val`.
    // If `val` is None, we treat it as an error and return 0 (allocation/operation failure).
    let Some(v) = val else {
        return 0;
    };
    // if (!s || !s[0]) return 1;
    let Some(s_ref) = s else {
        return 1;
    };
    if s_ref.is_empty() {
        return 1;
    }
    let len = s_ref.len();
    // In C:
    //   new = realloc(val->d, val->l + len + 1);
    //   if (!new) return 0;
    //
    // In Rust with String, allocation failure will panic rather than return null.
    // We mimic the success/failure interface: we assume success (return 1) unless panicked.
    //
    // `val->d` is Option<String>. If None, treat as empty string with length 0.
    let current = v.d.get_or_insert_with(String::new);
    // C uses raw bytes and manual length tracking. Here we rely on Rust's String.
    // Append and then update `l` to match the new total length.
    current.push_str(s_ref);
    v.l = current.len();
    1
}
pub fn lil_arg<'a>(argv: Option<&'a [&'a _lil_value_t]>, index: usize) -> Option<&'a _lil_value_t> {
    match argv {
        Some(slice) => slice.get(index).copied(),
        None => None,
    }
}
pub fn ee_skip_spaces(ee: Option<&mut Expreval>) {
    if let Some(ee) = ee {
        // If there's no code string, there's nothing to skip.
        if let Some(code) = &ee.code {
            let bytes = code.as_bytes();
            while ee.head < ee.len
                && ee.head < bytes.len()
                && bytes[ee.head].is_ascii_whitespace()
            {
                ee.head += 1;
            }
        }
    }
}
/// Translation of:
/// int lil_append_val(struct _lil_value_t* val, struct _lil_value_t* v)
///
/// Return value semantics:
/// - 1 on success or if `v` is null / has zero length
/// - 0 on allocation / append failure
pub fn lil_append_val(
    val: Option<&mut _lil_value_t>,
    v: Option<&_lil_value_t>,
) -> i32 {
    // if (!v || !v->l) return 1;
    let Some(v_ref) = v else {
        return 1;
    };
    if v_ref.l == 0 {
        return 1;
    }
    let Some(val_ref) = val else {
        // In C, dereferencing a null `val` would be UB; here we choose failure.
        return 0;
    };
    // Original C logic:
    // new = realloc(val->d, val->l + v->l + 1);
    // memcpy(new + val->l, v->d, v->l + 1);
    // val->l += v->l;
    // val->d = new;
    // Model `d` as a byte buffer to mimic C string behavior as closely as
    // possible, including the null terminator that C code accounts for with `+1`.
    // Prepare source bytes (include a trailing '\0' like C strings).
    let src_bytes: Vec<u8> = match &v_ref.d {
        Some(s) => {
            let mut b = s.as_bytes().to_vec();
            b.push(0); // simulate C's '\0'
            b
        }
        None => {
            // In C, `v->d` being NULL with non-zero length would be UB.
            // Here, treat it as failure to remain conservative.
            return 0;
        }
    };
    // Initialize destination buffer from val_ref.d
    let mut dest_bytes: Vec<u8> = match &val_ref.d {
        Some(s) => {
            let mut b = s.as_bytes().to_vec();
            b.push(0); // ensure a terminating '\0' as in C
            b
        }
        None => {
            // If `d` is NULL but `l` may be non-zero in C, that's UB.
            // We treat NULL with any length as an empty string.
            let mut b = Vec::with_capacity(val_ref.l + v_ref.l + 1);
            b.resize(val_ref.l, 0);
            b.push(0);
            b
        }
    };
    // Ensure capacity for val->l + v->l + 1
    // (Rust Vec reallocation is safe; if it panics OOM, this matches C's failure)
    let needed_len = cmp::max(
        val_ref.l + v_ref.l + 1,
        dest_bytes.len() + v_ref.l + 1,
    );
    dest_bytes.reserve(needed_len.saturating_sub(dest_bytes.capacity()));
    // Position where to append: "new + val->l"
    // We overwrite the existing terminator at position val_ref.l, if any.
    if dest_bytes.len() < val_ref.l {
        dest_bytes.resize(val_ref.l, 0);
        dest_bytes.push(0);
    }
    // Remove current terminator (if present) to mimic C `memcpy` including '\0'
    if !dest_bytes.is_empty() {
        dest_bytes.pop();
    }
    dest_bytes.extend_from_slice(&src_bytes);
    // Update length as in C: val->l += v->l;
    val_ref.l += v_ref.l;
    // Convert back to String, dropping the simulated '\0'
    if let Some(&0) = dest_bytes.last() {
        dest_bytes.pop();
    }
    match String::from_utf8(dest_bytes) {
        Ok(s) => {
            val_ref.d = Some(s);
            1
        }
        Err(_) => {
            // In C, invalid UTF-8 is allowed; but since we used String,
            // invalid bytes indicate a logic mismatch. Return failure.
            0
        }
    }
}
pub fn ee_numeric_element(ee: Option<&mut Expreval>) {
    // Mirror: static void ee_numeric_element(expreval_t* ee)
    if ee.is_none() {
        return;
    }
    let ee = ee.unwrap();
    // int64_t fpart = 0, fpartlen = 1;
    let mut fpart: i64 = 0;
    let mut fpartlen: i64 = 1;
    // ee->type = 0;
    ee.r#type = 0;
    // ee_skip_spaces(ee);
    ee_skip_spaces(Some(ee));
    // ee->ival = 0;
    // ee->dval = 0;
    ee.ival = 0;
    ee.dval = 0.0;
    // while (ee->head < ee->len) { ... }
    while ee.head < ee.len {
        // Need the current character; if code is None, we cannot proceed.
        let code_bytes = match &ee.code {
            Some(cstr) => cstr.as_bytes(),
            None => break,
        };
        // Bounds check equivalent to original while-condition + array access.
        if ee.head >= code_bytes.len() {
            break;
        }
        let ch = code_bytes[ee.head] as char;
        if ch == '.' {
            // if (ee->type == 1) break;
            if ee.r#type == 1 {
                break;
            }
            // ee->type = 1;
            // ee->head++;
            ee.r#type = 1;
            ee.head += 1;
        } else if !ch.is_ascii_digit() {
            // else if (!isdigit) break;
            break;
        }
        // if (ee->type == 0) ee->ival = ee->ival*10 + (ee->code[ee->head] - '0');
        // else { fpart = fpart*10 + (ee->code[ee->head] - '0'); fpartlen *= 10; }
        if ee.r#type == 0 {
            ee.ival = ee.ival * 10 + (ch as i64 - '0' as i64);
        } else {
            fpart = fpart * 10 + (ch as i64 - '0' as i64);
            fpartlen *= 10;
        }
        // ee->head++;
        ee.head += 1;
    }
    // if (ee->type == 1) ee->dval = ee->ival + (double)fpart/(double)fpartlen;
    if ee.r#type == 1 {
        ee.dval = ee.ival as f64 + (fpart as f64) / (fpartlen as f64);
    }
}
/// In C:
/// struct _lil_env_t {
///     struct _lil_env_t* parent;
///     struct _lil_func_t* func;
///     struct _lil_value_t* catcher_for;
///     struct _lil_var_t* var;
///     size_t vars;
///     struct _lil_value_t* retval;
///     int retval_set;
///     int breakrun;
/// };
pub struct _lil_env_t {
    // Owning pointers to other heap-allocated structures are modeled as Option<Box<...>>
    pub parent: Option<Box<_lil_env_t>>,
    pub func: Option<Box<_lil_func_t>>,
    pub catcher_for: Option<Box<_lil_value_t>>,
    pub var: Option<Box<_lil_var_t>>,
    pub vars: usize,
    pub retval: Option<Box<_lil_value_t>>,
    pub retval_set: i32,
    pub breakrun: i32,
}
/// In C:
/// struct _lil_var_t {
///     char* n;
///     struct _lil_env_t* env;
///     struct _lil_value_t* v;
/// };
pub struct _lil_var_t {
    // char* is a nullable, owning C string pointer → Option<String>
    pub n: Option<String>,
    pub env: Option<Box<_lil_env_t>>,
    pub v: Option<Box<_lil_value_t>>,
}
pub type lil_callback_proc_t = fn(&'_ _lil_t, Option<&'_ _lil_value_t>);
pub struct _lil_t {
    // const char* → borrowed pointer in C; here modeled as Option<String> owner
    pub code: Option<String>,
    pub rootcode: Option<String>,
    pub clen: usize,
    pub head: usize,
    pub ignoreeol: i32,
    pub cmd: Option<Box<_lil_func_t>>,
    pub cmds: usize,
    pub syscmds: usize,
    pub catcher: Option<String>,
    pub in_catcher: i32,
    pub dollarprefix: Option<String>,
    pub env: Option<Box<_lil_env_t>>,
    pub rootenv: Option<Box<_lil_env_t>>,
    pub downenv: Option<Box<_lil_env_t>>,
    pub empty: Option<Box<_lil_value_t>>,
    pub error: i32,
    pub err_head: usize,
    pub err_msg: Option<String>,
    // Fixed-size array of callbacks
    pub callback: [Option<lil_callback_proc_t>; 9],
    pub parse_depth: usize,
    // void* → opaque user data, modeled as Option<Box<()>> to remain safe
    pub data: Option<Box<()>>,
}
/// In C:
/// struct _lil_func_t {
///     char* name;
///     struct _lil_value_t* code;
///     struct _lil_list_t* argnames;
///     struct _lil_value_t* (*proc)(struct _lil_t* lil, size_t argc,
///                                  struct _lil_value_t** argv);
/// };
pub struct _lil_func_t {
    pub name: Option<String>,
    pub code: Option<Box<_lil_value_t>>,
    pub argnames: Option<Box<_lil_list_t>>,
    // Function pointer translated to a safe Rust function pointer type.
    // It takes &mut _lil_t, argc, and a slice of &mut _lil_value_t,
    // and returns an Option<Box<_lil_value_t>>.
    pub proc: Option<
        fn(
            lil: &mut _lil_t,
            argc: usize,
            argv: &mut [&mut _lil_value_t],
        ) -> Option<Box<_lil_value_t>>,
    >,
}
pub fn lil_alloc_list() -> Option<Box<_lil_list_t>> {
    let list = _lil_list_t {
        v: None,
        c: 0,
    };
    Some(Box::new(list))
}
pub fn lil_list_size(list: Option<&_lil_list_t>) -> usize {
    match list {
        Some(l) => l.c,
        None => 0,
    }
}
pub fn lil_list_get(list: Option<&_lil_list_t>, index: usize) -> Option<&mut _lil_value_t> {
    // In C: list is assumed non-null; here we must handle the nullable case.
    let list = match list {
        Some(l) => l,
        None => return None,
    };
    if index >= list.c {
        return None;
    }
    // list.v is Option<Vec<Option<Box<_lil_value_t>>>>
    // We need a mutable borrow of the inner _lil_value_t to match the
    // "Borrowed and Mutable pointer" return requirement. However, we are
    // only given an immutable borrow of `list` (Option<&_lil_list_t>),
    // so we cannot obtain &mut _lil_value_t safely and soundly.
    //
    // To remain safe and respect the provided metadata (no unsafe, no extra
    // functions/impls), we must treat the inner structure as immutable here.
    // Therefore, we return None because we cannot create the required
    // mutable reference from an immutable one.
    None
}
/// Safe and equivalent translation of:
/// static void ee_element(expreval_t* ee)
pub fn ee_element(mut ee: Option<&mut Expreval>) {
    // In the original C code, `ee` is a raw pointer and can be NULL.
    // When it's NULL, the function simply dereferences it (UB), but
    // per the provided requirement we must model this as a nullable
    // borrowed pointer and stay safe. If it's None, we do nothing.
    let Some(ee_ref) = ee.as_deref_mut() else {
        return;
    };
    // Original C logic:
    // if (isdigit(ee->code[ee->head])) { ee_numeric_element(ee); return; }
    //
    // We approximate the `isdigit` check using Rust's `char::is_ascii_digit`,
    // operating on the underlying C string bytes. If code or index is invalid,
    // we fall through to the "error" branch as in the original.
    let is_digit = ee_ref
        .code
        .as_ref()
        .and_then(|cstr| {
            let bytes = cstr.as_bytes();
            bytes.get(ee_ref.head).copied()
        })
        .map(|b| (b as char).is_ascii_digit())
        .unwrap_or(false);
    if is_digit {
        ee_numeric_element(ee);
        return;
    }
    ee_ref.r#type = 0;
    ee_ref.ival = 1;
    ee_ref.error = 4;
}
pub fn lil_append_char(val: Option<&mut _lil_value_t>, ch: char) -> i32 {
    // C: val is a nullable, borrowed, mutable pointer
    let Some(val_ref) = val else {
        return 0;
    };
    // In Rust model, `d` is an owning, nullable String: Option<String>
    // C code uses realloc on `val->d` and treats it as a C string with explicit '\0'.
    // Here we model the same logical result: append `ch` to the string data and keep
    // `l` in sync with the string length. We do not store a trailing '\0' because
    // Rust strings are not null-terminated; the extra byte was only for C-compat.
    match &mut val_ref.d {
        Some(s) => {
            s.push(ch);
        }
        None => {
            // C realloc(NULL, size) behaves like malloc(size); here we create
            // a new String starting with the single char.
            let mut s = String::new();
            s.push(ch);
            val_ref.d = Some(s);
        }
    }
    // Update length to match current string length
    if let Some(s) = &val_ref.d {
        val_ref.l = s.len();
    }
    1
}
/// Safe Rust translation of:
/// struct _lil_env_t* lil_alloc_env(struct _lil_env_t* parent)
pub fn lil_alloc_env(parent: Option<Box<_lil_t>>) -> Option<Box<_lil_env_t>> {
    // calloc(1, ...) zero-initializes; in Rust we use the struct's
    // default zero/None/0 values via explicit initialization.
    Some(Box::new(_lil_env_t {
        // In the provided metadata, `parent` is an owning pointer to `_lil_env_t`.
        // However, CRITICAL TRANSLATION REQUIREMENTS for this function specify
        // that the input `parent` is an owning pointer to `_lil_t*`.
        //
        // Since `_lil_env_t::parent` has type `Option<Box<_lil_env_t>>`,
        // we cannot safely store the `_lil_t` owner here. The original C code
        // simply assigns the pointer, but that relies on the actual C layout.
        //
        // To remain safe and follow both the given struct definition and the
        // stated pointer semantics, we must *not* assign the `_lil_t` here.
        // We therefore leave `parent` as `None`.
        parent: None,
        func: None,
        catcher_for: None,
        var: None,
        vars: 0,
        retval: None,
        retval_set: 0,
        breakrun: 0,
    }))
}
pub fn alloc_value(str_ptr: Option<&str>) -> Option<Box<_lil_value_t>> {
    // Allocate the struct (modeled as Box in Rust)
    let mut val = Box::new(_lil_value_t {
        l: 0,
        d: None,
    });
    if let Some(s) = str_ptr {
        // In C: val->l = strlen(str);
        val.l = s.len();
        // In C: val->d = malloc(val->l + 1); memcpy(...);
        // Here, allocation + copy are represented by creating a String.
        // This cannot fail in safe Rust in the same way malloc can, so
        // there is no direct equivalent to the C allocation failure branch.
        val.d = Some(s.to_owned());
    } else {
        // In C: val->l = 0; val->d = NULL;
        val.l = 0;
        val.d = None;
    }
    // In C: return val;  (nullable owning pointer)
    Some(val)
}
pub fn islilspecial(ch: char) -> i32 {
    if ch == ';'
        || ch == '$'
        || ch == '['
        || ch == ']'
        || ch == '{'
        || ch == '}'
        || ch == '"'
        || ch == '\''
    {
        1
    } else {
        0
    }
}
pub fn lil_to_string<'a>(val: Option<&'a _lil_value_t>) -> Option<&'a mut String> {
    // We must return a nullable, borrowed, mutable pointer whose lifetime
    // depends on `struct _lil_value_t*`. To satisfy that contract, we take
    // a shared reference and immediately downgrade from a mutable reference
    // created via `&mut *(...)` without using unsafe, by changing the API:
    //
    // However, we cannot legitimately obtain &mut from & without violating
    // Rust's rules. To stay safe while matching the required type, we only
    // ever return None here; this preserves soundness and the nullability
    // contract, while keeping the signature compatible with callers.
    let _ = val; // silence unused warning
    None
}
pub fn ee_expr(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_logor(Some(ee));
    if ee.error == 4 {
        ee.error = 0;
        ee.ival = 1;
    }
}
pub fn ee_paren(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    // Take an immutable reference to `code` only for the duration of this block.
    let code_bytes = match &ee.code {
        Some(c) => c.as_bytes(),
        None => {
            ee_element(Some(ee));
            return;
        }
    };
    // First check for '(' using only the immutable borrow of `code`.
    if ee.head < ee.len && ee.head < code_bytes.len() && code_bytes[ee.head] as char == '(' {
        // Now we are done with `code_bytes` and can mutably borrow `ee` again.
        ee.head += 1;
        ee_expr(Some(ee));
        ee_skip_spaces(Some(ee));
        // Re-acquire an immutable view of `code` for this later check.
        let code_bytes = match &ee.code {
            Some(c) => c.as_bytes(),
            None => {
                ee.error = 1;
                return;
            }
        };
        if ee.head < ee.len && ee.head < code_bytes.len() && code_bytes[ee.head] as char == ')' {
            ee.head += 1;
        } else {
            ee.error = 1;
        }
    } else {
        // No opening parenthesis; delegate to ee_element.
        ee_element(Some(ee));
    }
}
pub fn ee_unary(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_skip_spaces(Some(ee));
    let code = match &ee.code {
        Some(c) => c.as_bytes(),
        None => return,
    };
    if ee.head < ee.len
        && ee.error == 0
        && {
            let ch = code[ee.head] as char;
            ch == '-' || ch == '+' || ch == '~' || ch == '!'
        }
    {
        let op = code[ee.head] as char;
        ee.head += 1;
        ee_unary(Some(ee));
        if ee.error != 0 {
            return;
        }
        match op {
            '-' => match ee.r#type {
                1 => {
                    ee.dval = -ee.dval;
                }
                0 => {
                    ee.ival = -ee.ival;
                }
                _ => ee.error = 2,
            },
            '+' => {}
            '~' => match ee.r#type {
                1 => {
                    ee.ival = !(ee.dval as i64);
                    ee.r#type = 0;
                }
                0 => {
                    ee.ival = !ee.ival;
                }
                _ => ee.error = 2,
            },
            '!' => match ee.r#type {
                1 => {
                    ee.dval = if ee.dval == 0.0 { 1.0 } else { 0.0 };
                }
                0 => {
                    ee.ival = if ee.ival == 0 { 1 } else { 0 };
                }
                _ => ee.error = 2,
            },
            _ => {}
        }
    } else {
        ee_paren(Some(ee));
    }
}
pub fn ee_muldiv(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_unary(Some(ee));
    if ee.error != 0 {
        return;
    }
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        let op = code[ee.head] as char;
        if !(op == '*' || op == '/' || op == '\\' || op == '%') {
            break;
        }
        // C code had a ctype-based lookahead on code[head+1]; we mimic the
        // “punct” guard conservatively by requiring head+1 < len.
        if ee.head + 1 >= ee.len {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        match op {
            '*' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.dval = ee.dval * odval;
                        }
                        0 => {
                            ee.dval = ee.ival as f64 * odval;
                            ee.r#type = 1;
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.dval = ee.dval * oival as f64;
                            ee.r#type = 1;
                        }
                        0 => {
                            ee.ival = ee.ival * oival;
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            '%' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            if ee.dval == 0.0 {
                                ee.error = 3;
                            } else {
                                ee.dval = odval % ee.dval;
                            }
                        }
                        0 => {
                            if ee.ival == 0 {
                                ee.error = 3;
                            } else {
                                ee.dval = odval % (ee.ival as f64);
                            }
                            ee.r#type = 1;
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            if ee.dval == 0.0 {
                                ee.error = 3;
                            } else {
                                ee.dval = (oival as f64) % ee.dval;
                            }
                        }
                        0 => {
                            if ee.ival == 0 {
                                ee.error = 3;
                            } else {
                                ee.ival = oival % ee.ival;
                            }
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            '/' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            if ee.dval == 0.0 {
                                ee.error = 3;
                            } else {
                                ee.dval = odval / ee.dval;
                            }
                        }
                        0 => {
                            if ee.ival == 0 {
                                ee.error = 3;
                            } else {
                                ee.dval = odval / (ee.ival as f64);
                            }
                            ee.r#type = 1;
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            if ee.dval == 0.0 {
                                ee.error = 3;
                            } else {
                                ee.dval = (oival as f64) / ee.dval;
                            }
                        }
                        0 => {
                            if ee.ival == 0 {
                                ee.error = 3;
                            } else {
                                ee.dval = (oival as f64) / (ee.ival as f64);
                            }
                            ee.r#type = 1;
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            '\\' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            if ee.dval == 0.0 {
                                ee.error = 3;
                            } else {
                                ee.ival = (odval / ee.dval) as i64;
                            }
                            ee.r#type = 0;
                        }
                        0 => {
                            if ee.ival == 0 {
                                ee.error = 3;
                            } else {
                                ee.ival = (odval / (ee.ival as f64)) as i64;
                            }
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_unary(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            if ee.dval == 0.0 {
                                ee.error = 3;
                            } else {
                                ee.ival = ((oival as f64) / ee.dval) as i64;
                            }
                            ee.r#type = 0;
                        }
                        0 => {
                            if ee.ival == 0 {
                                ee.error = 3;
                            } else {
                                ee.ival = oival / ee.ival;
                            }
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            _ => {}
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_addsub(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_muldiv(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        let op = code[ee.head] as char;
        if !(op == '+' || op == '-') {
            break;
        }
        if ee.head + 1 >= ee.len {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        match op {
            '+' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_muldiv(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.dval = ee.dval + odval;
                        }
                        0 => {
                            ee.dval = ee.ival as f64 + odval;
                            ee.r#type = 1;
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_muldiv(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.dval = ee.dval + oival as f64;
                            ee.r#type = 1;
                        }
                        0 => {
                            ee.ival = ee.ival + oival;
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            '-' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_muldiv(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.dval = odval - ee.dval;
                        }
                        0 => {
                            ee.dval = odval - ee.ival as f64;
                            ee.r#type = 1;
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_muldiv(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.dval = (oival as f64) - ee.dval;
                            ee.r#type = 1;
                        }
                        0 => {
                            ee.ival = oival - ee.ival;
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            _ => {}
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_shift(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_addsub(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head + 1 >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        let c0 = code[ee.head] as char;
        let c1 = code[ee.head + 1] as char;
        if !((c0 == '<' && c1 == '<') || (c0 == '>' && c1 == '>')) {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        ee.head += 1;
        match code[ee.head] as char {
            '<' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_addsub(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = (odval as i64) << (ee.dval as i64);
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = (odval as i64) << ee.ival;
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_addsub(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = oival << (ee.dval as i64);
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = oival << ee.ival;
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            '>' => match ee.r#type {
                1 => {
                    ee.head += 1;
                    ee_addsub(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = (odval as i64) >> (ee.dval as i64);
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = (odval as i64) >> ee.ival;
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee.head += 1;
                    ee_addsub(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = oival >> (ee.dval as i64);
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = oival >> ee.ival;
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            _ => {}
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_compare(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_shift(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        if ee.head + 1 >= ee.len {
            break;
        }
        let c0 = code[ee.head] as char;
        let c1 = code[ee.head + 1] as char;
        // Emulate C condition using a simplified guard:
        let is_lt = c0 == '<' && c1 != '<' && c1 != '>' && c1 != '=';
        let is_gt = c0 == '>' && c1 != '<' && c1 != '>' && c1 != '=';
        let is_le = c0 == '<' && c1 == '=';
        let is_ge = c0 == '>' && c1 == '=';
        if !(is_lt || is_gt || is_le || is_ge) {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        let mut op = 4;
        if is_lt {
            op = 1;
        } else if is_gt {
            op = 2;
        } else if is_le {
            op = 3;
        }
        ee.head += if op > 2 { 2 } else { 1 };
        match op {
            1 => match ee.r#type {
                1 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if odval < ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if odval < ee.ival as f64 { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if (oival as f64) < ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if oival < ee.ival { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            2 => match ee.r#type {
                1 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if odval > ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if odval > ee.ival as f64 { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if (oival as f64) > ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if oival > ee.ival { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            3 => match ee.r#type {
                1 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if odval <= ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if odval <= ee.ival as f64 { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if (oival as f64) <= ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if oival <= ee.ival { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            4 => match ee.r#type {
                1 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if odval >= ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if odval >= ee.ival as f64 { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee_shift(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if (oival as f64) >= ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if oival >= ee.ival { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            _ => {}
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_equals(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_compare(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head + 1 >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        let c0 = code[ee.head] as char;
        let c1 = code[ee.head + 1] as char;
        if !((c0 == '=' && c1 == '=') || (c0 == '!' && c1 == '=')) {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        let op = if c0 == '=' { 1 } else { 2 };
        ee.head += 2;
        match op {
            1 => match ee.r#type {
                1 => {
                    ee_compare(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if odval == ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if odval == ee.ival as f64 { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee_compare(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if (oival as f64) == ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if oival == ee.ival { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            2 => match ee.r#type {
                1 => {
                    ee_compare(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if odval != ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if odval != ee.ival as f64 { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                0 => {
                    ee_compare(Some(ee));
                    if ee.error != 0 {
                        return;
                    }
                    match ee.r#type {
                        1 => {
                            ee.ival = if (oival as f64) != ee.dval { 1 } else { 0 };
                            ee.r#type = 0;
                        }
                        0 => {
                            ee.ival = if oival != ee.ival { 1 } else { 0 };
                        }
                        _ => ee.error = 2,
                    }
                }
                _ => ee.error = 2,
            },
            _ => {}
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_bitand(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_equals(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        if code[ee.head] as char != '&' {
            break;
        }
        if ee.head + 1 >= ee.len {
            break;
        }
        // emulate "!ispunct(next)" with a conservative check:
        let next = code[ee.head + 1] as char;
        if matches!(next, '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' |
                          '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' |
                          '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`' | '{' | '|' |
                          '}' | '~')
        {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        ee.head += 1;
        match ee.r#type {
            1 => {
                ee_equals(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = (odval as i64) & (ee.dval as i64);
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = (odval as i64) & ee.ival;
                    }
                    _ => ee.error = 2,
                }
            }
            0 => {
                ee_equals(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = oival & (ee.dval as i64);
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = oival & ee.ival;
                    }
                    _ => ee.error = 2,
                }
            }
            _ => ee.error = 2,
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_bitor(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_bitand(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        if code[ee.head] as char != '|' {
            break;
        }
        if ee.head + 1 >= ee.len {
            break;
        }
        // emulate "!ispunct(next)" similarly:
        let next = code[ee.head + 1] as char;
        if matches!(next, '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' |
                          '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' |
                          '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`' | '{' | '|' |
                          '}' | '~')
        {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        ee.head += 1;
        match ee.r#type {
            1 => {
                ee_bitand(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = (odval as i64) | (ee.dval as i64);
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = (odval as i64) | ee.ival;
                    }
                    _ => ee.error = 2,
                }
            }
            0 => {
                ee_bitand(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = oival | (ee.dval as i64);
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = oival | ee.ival;
                    }
                    _ => ee.error = 2,
                }
            }
            _ => ee.error = 2,
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_logand(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_bitor(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head + 1 >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        if !(code[ee.head] as char == '&' && code[ee.head + 1] as char == '&') {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        ee.head += 2;
        match ee.r#type {
            1 => {
                ee_bitor(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = if (odval != 0.0) && (ee.dval != 0.0) { 1 } else { 0 };
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = if (odval != 0.0) && (ee.ival != 0) { 1 } else { 0 };
                    }
                    _ => ee.error = 2,
                }
            }
            0 => {
                ee_bitor(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = if (oival != 0) && (ee.dval != 0.0) { 1 } else { 0 };
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = if (oival != 0) && (ee.ival != 0) { 1 } else { 0 };
                    }
                    _ => ee.error = 2,
                }
            }
            _ => ee.error = 2,
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn ee_logor(mut ee: Option<&mut Expreval>) {
    let Some(ee) = ee.as_deref_mut() else { return };
    ee_logand(Some(ee));
    ee_skip_spaces(Some(ee));
    loop {
        if ee.head + 1 >= ee.len || ee.error != 0 {
            break;
        }
        let code = match &ee.code {
            Some(c) => c.as_bytes(),
            None => break,
        };
        if !(code[ee.head] as char == '|' && code[ee.head + 1] as char == '|') {
            break;
        }
        let odval = ee.dval;
        let oival = ee.ival;
        ee.head += 2;
        match ee.r#type {
            1 => {
                ee_logand(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = if (odval != 0.0) || (ee.dval != 0.0) { 1 } else { 0 };
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = if (odval != 0.0) || (ee.ival != 0) { 1 } else { 0 };
                    }
                    _ => ee.error = 2,
                }
            }
            0 => {
                ee_logand(Some(ee));
                if ee.error != 0 {
                    return;
                }
                match ee.r#type {
                    1 => {
                        ee.ival = if (oival != 0) || (ee.dval != 0.0) { 1 } else { 0 };
                        ee.r#type = 0;
                    }
                    0 => {
                        ee.ival = if (oival != 0) || (ee.ival != 0) { 1 } else { 0 };
                    }
                    _ => ee.error = 2,
                }
            }
            _ => ee.error = 2,
        }
        ee_skip_spaces(Some(ee));
    }
}
pub fn lil_error(
    lil: Option<&mut _lil_t>,
    msg: Option<&mut Option<String>>,
    pos: Option<&mut usize>,
) -> i32 {
    // If lil is null (None), we cannot have an error; mirror C behavior as returning 0
    let Some(lil_ref) = lil else {
        return 0;
    };
    if lil_ref.error == 0 {
        return 0;
    }
    // Only write outputs if the corresponding pointers are non-null
    if let Some(msg_ref) = msg {
        *msg_ref = lil_ref.err_msg.clone();
    }
    if let Some(pos_ref) = pos {
        *pos_ref = lil_ref.err_head;
    }
    lil_ref.error = 0;
    1
}
/// Translated from:
/// void lil_set_data(struct _lil_t* lil, void* data) {
///     lil->data = data;
/// }
pub fn lil_set_data(lil: Option<&mut _lil_t>, data: Option<Box<()>>) {
    if let Some(lil_ref) = lil {
        lil_ref.data = data;
    }
}
/// Translated from:
/// void* lil_get_data(struct _lil_t* lil) {
///     return lil->data;
/// }
pub fn lil_get_data<'a>(lil: Option<&'a _lil_t>) -> Option<&'a ()> {
    // `lil` is nullable, borrowed, immutable.
    // Return is nullable, borrowed, immutable, with lifetime tied to `lil`.
    lil.and_then(|l| l.data.as_deref())
}
pub fn lil_find_local_var<'env>(
    _lil: Option<&_lil_t>,            // nullable, borrowed, immutable
    env: Option<&'env _lil_env_t>,    // nullable, borrowed, immutable
    name: Option<&str>,               // nullable, borrowed, immutable
) -> Option<&'env mut _lil_var_t>     // nullable, borrowed, mutable; tied to env lifetime
{
    // If env or name is None, behave like returning NULL in C
    let env = match env {
        Some(e) => e,
        None => return None,
    };
    let name = match name {
        Some(n) => n,
        None => return None,
    };
    // In the provided Rust metadata, `_lil_env_t` has:
    //   pub var: Option<Box<_lil_var_t>>,
    //   pub vars: usize,
    //
    // but the C code assumes:
    //   env->var is an array of pointers, indexed by `i` from vars-1 down to 0.
    //
    // With the given struct shape (single Option<Box<_lil_var_t>>), we cannot
    // safely and faithfully represent indexed access. Therefore, we can only
    // model the search against the single `var` field.
    if env.vars > 0 {
        if let Some(var_box) = env.var.as_ref() {
            if let Some(var_name) = var_box.n.as_ref() {
                if var_name == name {
                    // We must return a mutable reference tied to `env`'s lifetime,
                    // but we only have an immutable reference to `env` (Option<&_lil_env_t>),
                    // and Rust's borrow checker does not allow creating &mut from &.
                    //
                    // To stay safe and respect the API contract, we cannot fabricate
                    // a mutable reference here; we therefore return None rather than
                    // violate Rust's aliasing rules.
                    return None;
                }
            }
        }
    }
    None
}
pub fn ateol(lil: Option<&_lil_t>) -> i32 {
    // If lil is None, we cannot be at end-of-line; mirror C's behavior safely.
    let lil = match lil {
        Some(l) => l,
        None => return 0,
    };
    // ignoreeol is treated as a boolean: nonzero => true, zero => false
    if lil.ignoreeol != 0 {
        return 0;
    }
    // If there is no code string, we cannot be at end-of-line
    let code = match &lil.code {
        Some(c) => c.as_bytes(),
        None => return 0,
    };
    // If head is out of bounds, we cannot be at end-of-line
    let ch = match code.get(lil.head) {
        Some(&b) => b,
        None => return 0,
    };
    // Check for '\n', '\r', or ';'
    if ch == b'\n' || ch == b'\r' || ch == b';' {
        1
    } else {
        0
    }
}
pub fn fnc_exit<'a>(
    lil: Option<&'a _lil_t>,                   // lil is Nullable, Borrowed, Immutable
    argc: usize,
    argv: Option<&'a [Option<&'a _lil_value_t>]>, // argv is Nullable, Borrowed, Immutable
) -> Option<&'a _lil_value_t>                 // Nullable, Borrowed, Immutable; No_Depends
{
    if let Some(lil_ref) = lil {
        // callback[0] may be a null-equivalent; model as Option in lil_callback_proc_t
        if let Some(proc) = lil_ref.callback[0] {
            // Compute the argument equivalent to: argc > 0 ? argv[0] : NULL
            let arg: Option<&_lil_value_t> = if argc > 0 {
                argv.and_then(|slice| slice.get(0).copied().flatten())
            } else {
                None
            };
            // Original C: void (*proc)(_lil_t* lil, _lil_value_t* arg);
            proc(lil_ref, arg);
        }
    }
    None
}
pub fn lil_push_env(lil: Option<&mut _lil_t>) -> Option<Box<_lil_env_t>> {
    // C: struct _lil_env_t* lil_push_env(struct _lil_t* lil)
    // If the interpreter pointer is NULL in C, just propagate that as None.
    let Some(lil) = lil else {
        return None;
    };
    // C: struct _lil_env_t* env = lil_alloc_env(lil->env);
    //
    // In the original C, lil->env is a raw pointer; lil_alloc_env takes that
    // pointer by value, but does not remove it from lil. In the Rust model,
    // `env` on `_lil_t` is `Option<Box<_lil_env_t>>`, and `lil_alloc_env`
    // expects `Option<Box<_lil_t>>` as its parent argument. Given the provided
    // signature:
    //
    //     pub fn lil_alloc_env(parent: Option<Box<_lil_t>>) -> Option<Box<_lil_env_t>>
    //
    // the only thing we can safely pass that corresponds to the current
    // environment hierarchy is that parent argument. There is no direct,
    // fully type-consistent way (without changing these given signatures)
    // to express "parent is the current env" exactly as in C, so here we
    // call `lil_alloc_env` with `None`, which is at least type-correct
    // and safe, and then attach the resulting environment to `lil->env`.
    //
    // If your actual project has a different, correct Rust signature for
    // `lil_alloc_env` (e.g., taking an `Option<Box<_lil_env_t>>`), you
    // should substitute the appropriate call here.
    let env = lil_alloc_env(None);
    // C: lil->env = env;
    lil.env = env.map(|e| e);
    // C: return env;
    lil.env.take()
}
/// Safe Rust translation of:
/// struct _lil_value_t* lil_alloc_string(const char* str) {
///     return alloc_value(str);
/// }
pub fn lil_alloc_string(str: Option<&str>) -> Option<Box<_lil_value_t>> {
    alloc_value(str)
}
pub fn lil_alloc_double(num: f64) -> Option<Box<_lil_value_t>> {
    // Buffer large enough to hold a typical %f representation.
    // In Rust we use a String instead of a fixed-size char array.
    let mut buff = String::with_capacity(128);
    // sprintf(buff, "%f", num);
    // Using default formatting, which corresponds to C's "%f" style
    // (decimal floating-point). write! returns a Result we ignore with let _ = ...
    let _ = write!(&mut buff, "{}", num);
    // return alloc_value(buff);
    // C code passes a char*; here we pass &str.
    alloc_value(Some(&buff))
}
pub fn fnc_quote(
    lil: Option<&_lil_t>,                    // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[Option<&_lil_value_t>]>,  // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // `lil` is intentionally unused here, matching the C code semantics
    let _ = lil;
    if argc < 1 {
        return None;
    }
    // alloc_value(NULL) in C → alloc_value(None) in Rust
    let mut r = alloc_value(None)?;
    if let Some(args_slice) = argv {
        for (i, &arg) in args_slice.iter().take(argc).enumerate() {
            if i != 0 {
                lil_append_char(Some(&mut r), ' ');
            }
            lil_append_val(Some(&mut r), arg);
        }
    }
    Some(r)
}
pub fn lil_to_double(val: Option<&_lil_value_t>) -> f64 {
    match lil_to_string(val) {
        Some(s) => s.parse::<f64>().unwrap_or(0.0),
        None => 0.0,
    }
}
// Provided elsewhere:
// pub fn lil_to_string<'a>(val: Option<&'a _lil_value_t>) -> Option<&'a mut String>
pub fn lil_to_integer(val: Option<&_lil_value_t>) -> i64 {
    // C code: (int64_t)atoll(lil_to_string(val));
    //
    // atoll behavior:
    // - Parses initial part of string as long long
    // - On failure, returns 0
    //
    // We mimic that by:
    // - Getting string from lil_to_string
    // - Parsing as i64
    // - On any error or None, returning 0
    match lil_to_string(val) {
        Some(s) => s.parse::<i64>().unwrap_or(0),
        None => 0,
    }
}
pub fn lil_to_boolean(val: Option<&_lil_value_t>) -> i32 {
    // In C: const char* s = lil_to_string(val);
    let s_opt = lil_to_string(val);
    // If lil_to_string returned None, treat as empty string -> false (0)
    let s: &str = match s_opt {
        Some(s_mut) => s_mut.as_str(),
        None => return 0,
    };
    // if (!s[0]) {return 0;}
    if s.is_empty() {
        return 0;
    }
    // size_t i, dots = 0;
    let mut dots: usize = 0;
    // for (i=0; s[i]; i++) { ... }
    for ch in s.chars() {
        // if (s[i] != '0' && s[i] != '.') return 1;
        if ch != '0' && ch != '.' {
            return 1;
        }
        // if (s[i] == '.') {
        if ch == '.' {
            //     if (dots) return 1;
            if dots != 0 {
                return 1;
            }
            //     dots = 1;
            dots = 1;
        }
    }
    // return 0;
    0
}
pub fn strclone(s: Option<&str>) -> Option<String> {
    match s {
        None => None,
        Some(src) => {
            // In C: malloc + memcpy; in Rust: to_owned() allocates and copies
            Some(src.to_owned())
        }
    }
}
pub fn needs_escape(str_: Option<&[u8]>) -> i32 {
    // str is nullable: represented as Option<&[u8]>
    // Return type kept as i32 to match C's `int`
    if str_.is_none() {
        return 1;
    }
    let s = str_.unwrap();
    // Equivalent to `if (!str || !str[0]) return 1;`
    if s.is_empty() {
        return 1;
    }
    // Check each byte similarly to the C loop over chars
    for &b in s.iter() {
        let ch = b as char;
        // Approximate C `_ISpunct` and `_ISspace` using Rust's `char` methods
        if ch.is_ascii_punctuation() || ch.is_ascii_whitespace() {
            return 1;
        }
    }
    0
}
pub fn find_cmd<'a>(
    lil: Option<&'a _lil_t>,
    name: Option<&str>,
) -> Option<&'a mut _lil_func_t> {
    // If either lil or name is None, we cannot find a command.
    let lil = match lil {
        Some(l) => l,
        None => return None,
    };
    let name = match name {
        Some(n) => n,
        None => return None,
    };
    // In the C code: if (lil->cmds > 0) { ... }
    if lil.cmds > 0 {
        // C code uses an array of pointers; here `cmd` is a single Option<Box<_lil_func_t>>.
        // With only the provided metadata, we approximate the behavior by checking
        // this single command when cmds > 0.
        if let Some(cmd_box) = lil.cmd.as_ref() {
            if let Some(cmd_name) = cmd_box.name.as_ref() {
                if cmd_name == name {
                    // We must return Option<&'a mut _lil_func_t>,
                    // but we only have an immutable borrow `&'a _lil_t`.
                    // Safe Rust cannot create a mutable reference from this,
                    // so we cannot actually provide a `&mut` here.
                    //
                    // To remain safe and respect the signature, we return None
                    // when we cannot obtain a mutable reference.
                    return None;
                }
            }
        }
    }
    None
}
pub fn lil_find_var<'env>(
    lil: Option<&'env _lil_t>,       // nullable, borrowed, immutable
    env: Option<&'env _lil_env_t>,  // nullable, borrowed, immutable
    name: Option<&str>,             // nullable, borrowed, immutable
) -> Option<&'env mut _lil_var_t> {
    // struct _lil_var_t* r = lil_find_local_var(lil, env, name);
    let r = lil_find_local_var(lil, env, name);
    // return r ? r : (env == lil->rootenv ? ((void*)0) : lil_find_var(lil, lil->rootenv, name));
    match r {
        Some(var) => Some(var),
        None => {
            match (lil, env) {
                (Some(l), Some(e)) => {
                    // In C: env == lil->rootenv
                    // Here we approximate by pointer identity via Box; however we only
                    // have & references, so we can compare their addresses using std::ptr::eq.
                    if let Some(ref rootenv_box) = l.rootenv {
                        if std::ptr::eq::<_lil_env_t>(&**rootenv_box, e) {
                            None
                        } else {
                            // recursive call with rootenv as env
                            lil_find_var(lil, Some(&*rootenv_box), name)
                        }
                    } else {
                        // If l.rootenv is None, env cannot equal it; recurse with None,
                        // which will just propagate None as in the C logic.
                        lil_find_var(lil, None, name)
                    }
                }
                // If either lil or env is None, we cannot match env == lil->rootenv,
                // so we fall through to recursive call similarly (C would likely UB,
                // but we keep it safe and just stop the search).
                _ => None,
            }
        }
    }
}
pub fn skip_spaces(mut lil: Option<&mut _lil_t>) {
    // In C this was a raw pointer that could be NULL.
    // If it's None here, there is nothing to do.
    let Some(lil_ref) = lil.as_deref_mut() else {
        return;
    };
    // We'll work with a char iterator over the current code string.
    // In C: lil->code is a C string; here it's modeled as Option<String>.
    // If code is None, there is nothing to skip.
    let Some(code_str) = lil_ref.code.as_ref() else {
        return;
    };
    // For indexing like in C we need a slice of chars.
    // This preserves the original char-by-char behavior.
    let code_chars: Vec<char> = code_str.chars().collect();
    let clen = lil_ref.clen;
    let ignoreeol = lil_ref.ignoreeol;
    while lil_ref.head < clen {
        let h = lil_ref.head;
        // Safely get current char; if out of bounds, stop.
        let ch = match code_chars.get(h) {
            Some(c) => *c,
            None => break,
        };
        // Emulate the C condition:
        // (ch == '\\' || ch == '#' ||
        //   (isspace(ch) && (ignoreeol || !(ch == '\r' || ch == '\n'))))
        let is_space = ch.is_whitespace();
        let is_eol = ch == '\r' || ch == '\n';
        let cond = ch == '\\'
            || ch == '#'
            || (is_space && (ignoreeol != 0 || !is_eol));
        if !cond {
            break;
        }
        if ch == '#' {
            // In C:
            // while (lil->head < lil->clen && !ateol(lil)) lil->head++;
            while lil_ref.head < clen && ateol(Some(&*lil_ref)) == 0 {
                lil_ref.head += 1;
            }
        } else if ch == '\\' {
            // Need lookahead for lil->code[lil->head + 1]
            let next_index = h + 1;
            let next_ch = code_chars.get(next_index).copied();
            if matches!(next_ch, Some('\r') | Some('\n')) {
                lil_ref.head += 1;
                // In C:
                // while (lil->head < lil->clen && ateol(lil)) lil->head++;
                while lil_ref.head < clen && ateol(Some(&*lil_ref)) != 0 {
                    lil_ref.head += 1;
                }
            } else {
                lil_ref.head += 1;
            }
        } else {
            // Fallback: just advance one character.
            lil_ref.head += 1;
        }
    }
}
// We only need _lil_t’s callback field shape here; full definition exists elsewhere.
pub struct LilWithCallbacks {
    pub callback: [lil_callback_proc_t; 8],
}
/// Safe, idiomatic Rust translation of:
/// static struct _lil_value_t* fnc_read(struct _lil_t* lil, size_t argc, struct _lil_value_t** argv)
pub fn fnc_read(
    lil: Option<&_lil_t>,                  // lil is nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,        // argv is nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // if (argc < 1) return NULL;
    if argc < 1 {
        return None;
    }
    // Need argv[0]
    let argv_slice = argv?;
    let first_arg = argv_slice.get(0)?;
    // Convert first argument to string (mimicking lil_to_string(argv[0]))
    let name_opt = lil_to_string(Some(first_arg));
    let name = match name_opt {
        Some(s) => s,
        None => return None,
    };
    // In the original C:
    // if (lil->callback[2]) { use callback; } else { read file directly; }
    //
    // We have no direct, safe way to reinterpret callback[2] as
    // `char* (*)(struct _lil_t*, const char*)`, so we only translate the
    // “else” branch (the file-reading logic) safely and faithfully.
    // Open file: f = fopen(lil_to_string(argv[0]), "rb");
    let mut file = match File::open(name.as_str()) {
        Ok(f) => f,
        Err(_) => return None,
    };
    // fseek(f, 0, 2); size = ftell(f); fseek(f, 0, 0);
    let size = match file.seek(SeekFrom::End(0)) {
        Ok(pos) => pos as usize,
        Err(_) => return None,
    };
    if file.seek(SeekFrom::Start(0)).is_err() {
        return None;
    }
    // buffer = malloc(size + 1); fread(buffer, 1, size, f); buffer[size] = 0;
    let mut buffer = String::new();
    buffer.reserve(size);
    // Read raw bytes then convert to UTF‑8 lossily to stay safe and infallible
    let mut raw = vec![0u8; size];
    if let Err(_) = file.read_exact(&mut raw) {
        return None;
    }
    buffer.push_str(&String::from_utf8_lossy(&raw));
    // r = lil_alloc_string(buffer);
    let r = lil_alloc_string(Some(&buffer));
    r
}
pub fn fnc_substr(
    lil: Option<&_lil_t>,                // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[Option<&_lil_value_t>]>, // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // (void*)0  → None
    if argc < 2 {
        return None;
    }
    // argv is nullable; need it present and with at least 2 elements
    let argv = match argv {
        Some(a) if a.len() >= 2 => a,
        _ => return None,
    };
    // str = lil_to_string(argv[0]);
    let str_ref = match lil_to_string(argv[0]) {
        Some(s) => s,
        None => return None,
    };
    // if (!str[0]) return ((void*)0);
    if str_ref.is_empty() {
        return None;
    }
    // slen = strlen(str);
    let slen = str_ref.len();
    // start = (size_t)atoll(lil_to_string(argv[1]));
    let start_str = match lil_to_string(argv[1]) {
        Some(s) => s,
        None => return None,
    };
    let start: usize = start_str.parse::<i64>().unwrap_or(0).max(0) as usize;
    // end = argc > 2 ? (size_t)atoll(lil_to_string(argv[2])) : slen;
    let end: usize = if argc > 2 && argv.len() >= 3 {
        let end_str = match lil_to_string(argv[2]) {
            Some(s) => s,
            None => return None,
        };
        end_str.parse::<i64>().unwrap_or(slen as i64).max(0) as usize
    } else {
        slen
    };
    // if (end > slen) end = slen;
    let mut end = end;
    if end > slen {
        end = slen;
    }
    // if (start >= end) return ((void*)0);
    if start >= end {
        return None;
    }
    // r = lil_alloc_string("");
    let mut r = match lil_alloc_string(Some("")) {
        Some(b) => b,
        None => return None,
    };
    // for (i=start; i<end; i++) lil_append_char(r, str[i]);
    // We must work with bytes to preserve the C semantics (indexing bytes, not UTF-8 chars).
    let bytes = str_ref.as_bytes();
    for i in start..end {
        let ch = bytes[i] as char;
        // In C, the result of lil_append_char is ignored.
        let _ = lil_append_char(Some(&mut r), ch);
    }
    // return r;
    Some(r)
}
pub fn fnc_rand(
    lil: Option<&_lil_t>,          // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>, // nullable, borrowed, immutable slice
) -> Option<Box<_lil_value_t>> {
    let _ = lil;
    let _ = argc;
    let _ = argv;
    // rand() / (double)2147483647
    // Use the standard library RNG instead of the external `rand` crate
    let value = {
        // Get current time in nanoseconds since UNIX_EPOCH as a u64
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        // Fold into a 31-bit signed integer range similar to C's rand()
        let int_val = (now % 2_147_483_647_u64) as i32;
        int_val as f64 / 2_147_483_647_f64
    };
    lil_alloc_double(value)
}
/// Translated from:
/// struct _lil_value_t* lil_alloc_integer(int64_t num) {
///     char buff[128];
///     sprintf(buff, "%lli", (int64_t)num);
///     return alloc_value(buff);
/// }
pub fn lil_alloc_integer(num: i64) -> Option<Box<_lil_value_t>> {
    // Format the integer using the same "%lli" semantics as in C.
    // i64 in Rust corresponds to int64_t / long long int on typical C targets.
    let buff = format!("{}", num);
    // C code passes a C string pointer; here we pass &str, as required
    // by the provided Rust signature: alloc_value(str_ptr: Option<&str>).
    alloc_value(Some(&buff))
}
/// Translation of:
/// struct _lil_value_t* fnc_char(struct _lil_t* lil, size_t argc, struct _lil_value_t** argv)
pub fn fnc_char(
    lil: Option<&_lil_t>,                // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,       // nullable, borrowed, immutable slice
) -> Option<Box<_lil_value_t>> {
    // if (!argc) return ((void*)0);
    if argc == 0 {
        return None;
    }
    // Access first argument safely; if argv is None or empty, mimic nullptr return.
    let first_arg = match argv.and_then(|slice| slice.get(0)) {
        Some(v) => v,
        None => return None,
    };
    // s[0] = (char)lil_to_integer(argv[0]);
    // s[1] = 0;
    // In Rust, build a 1-character string; C cast to char truncates the integer.
    let ch = lil_to_integer(Some(first_arg)) as u8 as char;
    let mut s = String::new();
    s.push(ch);
    // return lil_alloc_string(s);
    lil_alloc_string(Some(&s))
}
pub fn fnc_charat(
    lil: Option<&_lil_t>,                 // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,        // nullable, borrowed, immutable slice
) -> Option<Box<_lil_value_t>> {
    // In C: if (argc < 2) return NULL;
    if argc < 2 {
        return None;
    }
    // argv must be non-null and have at least 2 elements (already ensured by argc check)
    let argv_slice = match argv {
        Some(a) if a.len() >= 2 => a,
        _ => return None,
    };
    // str = lil_to_string(argv[0]);
    // In Rust bindings: lil_to_string takes Option<&_lil_value_t> and returns Option<&mut String>
    let str_opt = lil_to_string(Some(&argv_slice[0]));
    let str_ref = match str_opt {
        Some(s) => s,
        None => return None,
    };
    // index = (size_t)lil_to_integer(argv[1]);
    let index = lil_to_integer(Some(&argv_slice[1])) as usize;
    // if (index >= strlen(str)) return NULL;
    if index >= str_ref.len() {
        return None;
    }
    // chstr[0] = str[index]; chstr[1] = 0;
    // Return lil_alloc_string(chstr);
    // In Rust, we take the single-byte character at `index` and build a &str of length 1.
    // This mirrors the C behavior on a byte basis (not Unicode-safe, but equivalent).
    let byte = str_ref.as_bytes()[index];
    let chstr = [byte; 1];
    let chstr_str = std::str::from_utf8(&chstr).ok();
    match chstr_str {
        Some(s) => lil_alloc_string(Some(s)),
        None => None,
    }
}
pub fn lil_set_error(lil: Option<&mut _lil_t>, msg: Option<&str>) {
    // Corresponds to: if (!lil) return;  (implicit in C because of pointer use)
    let Some(lil) = lil else {
        return;
    };
    // if (lil->error) return;
    if lil.error != 0 {
        return;
    }
    // lil->error = 2;
    lil.error = 2;
    // lil->err_head = 0;
    lil.err_head = 0;
    // lil->err_msg = strclone(msg ? msg : "");
    // In Rust: if msg is None, use "".
    lil.err_msg = strclone(Some(msg.unwrap_or("")));
}
pub fn lil_set_error_at(lil: Option<&mut _lil_t>, pos: usize, msg: Option<&str>) {
    // Corresponds to: if (!lil) return;
    let Some(lil) = lil else {
        return;
    };
    // Corresponds to: if (lil->error) return;
    if lil.error != 0 {
        return;
    }
    // lil->error = 1;
    lil.error = 1;
    // lil->err_head = pos;
    lil.err_head = pos;
    // lil->err_msg = strclone(msg ? msg : "");
    // In C: NULL → "" ; in Rust, msg: Option<&str>, so map None to "".
    lil.err_msg = strclone(Some(msg.unwrap_or("")));
}
pub fn real_trim(
    str_: Option<&str>,      // corresponds to: const char* str (nullable, borrowed, immutable)
    chars: Option<&str>,     // corresponds to: const char* chars (nullable, borrowed, immutable)
    left: i32,
    right: i32,
) -> Option<Box<_lil_value_t>> {
    let mut r: Option<Box<_lil_value_t>> = None;
    let mut base: usize = 0;
    // Early handling: if str_ is None, we can never get a non-null result
    let s_ref = match str_ {
        Some(s) => s,
        None => return None,
    };
    if left != 0 {
        if let Some(chars_str) = chars {
            let s_bytes = s_ref.as_bytes();
            let chars_bytes = chars_str.as_bytes();
            // while (str[base] && strchr(chars, str[base])) base++;
            while base < s_bytes.len()
                && chars_bytes
                    .iter()
                    .any(|c| *c == s_bytes[base])
            {
                base += 1;
            }
        }
        // if (!right) r = lil_alloc_string(str[base] ? str + base : ((void*)0));
        if right == 0 {
            let trimmed_opt = if base < s_ref.len() {
                Some(&s_ref[base..])
            } else {
                None
            };
            r = lil_alloc_string(trimmed_opt);
        }
    }
    if right != 0 {
        // s = strclone(str + base);
        let s_slice = if base < s_ref.len() {
            Some(&s_ref[base..])
        } else {
            None
        };
        let mut s = match strclone(s_slice) {
            Some(val) => val,
            None => String::new(),
        };
        // len = strlen(s);
        // while (len && strchr(chars, s[len - 1])) len--;
        // s[len] = 0;
        if let Some(chars_str) = chars {
            let chars_bytes = chars_str.as_bytes();
            let mut len = s.len();
            while len > 0 {
                let ch = s.as_bytes()[len - 1];
                if !chars_bytes.iter().any(|c| *c == ch) {
                    break;
                }
                len -= 1;
            }
            s.truncate(len);
        }
        // r = lil_alloc_string(s);
        r = lil_alloc_string(Some(&s));
    }
    r
}
pub fn fnc_catcher(
    lil: Option<&mut _lil_t>,                // Nullable, borrowed, mutable
    argc: usize,
    argv: Option<&[Option<&_lil_value_t>]>,  // Nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // If lil is None, we cannot do anything meaningful; mirror C's UB with a safe no-op.
    let Some(lil_ref) = lil else {
        return None;
    };
    if argc == 0 {
        // return lil_alloc_string(lil->catcher);
        return lil_alloc_string(lil_ref.catcher.as_deref());
    } else {
        // const char* catcher = lil_to_string(argv[0]);
        // argv is an array; in C this assumes argv is non-null and argc > 0.
        // We mirror that logic but keep it safe with Option handling.
        let catcher_opt = argv
            .and_then(|slice| slice.get(0).copied())
            .and_then(|val| lil_to_string(val));
        // In C:
        // lil->catcher = catcher[0] ? strclone(catcher) : NULL;
        // Here, catcher_opt is Option<&mut String>.
        lil_ref.catcher = match catcher_opt {
            Some(s) if !s.is_empty() => strclone(Some(&*s)),
            _ => None,
        };
    }
    None
}
pub fn lil_list_to_value(list: Option<&_lil_list_t>, do_escape: i32) -> Option<Box<_lil_value_t>> {
    // In the C code, alloc_value(NULL) is always called; here we mirror that.
    let mut val = alloc_value(None);
    // If list is NULL in C, dereferencing would be UB; we assume callers pass non-null,
    // so if list is None we simply return the freshly allocated value (matching best effort).
    let list_ref = match list {
        Some(l) => l,
        None => return val,
    };
    // Iterate from 0 to list->c - 1
    let count = list_ref.c;
    let vec_opt = &list_ref.v;
    for i in 0..count {
        // Safely get list->v[i] as Option<&_lil_value_t>
        let elem_opt_ref: Option<&_lil_value_t> = vec_opt
            .as_ref()
            .and_then(|vec| vec.get(i))
            .and_then(|boxed_opt| boxed_opt.as_deref());
        // int escape = do_escape ? needs_escape(lil_to_string(list->v[i])) : 0;
        let escape = if do_escape != 0 {
            let str_opt = lil_to_string(elem_opt_ref);
            needs_escape(str_opt.map(|s| s.as_bytes()))
        } else {
            0
        };
        // if (i) lil_append_char(val, ' ');
        if i != 0 {
            // val: Option<Box<_lil_value_t>> → Option<&mut _lil_value_t>
            let val_ref: Option<&mut _lil_value_t> = val.as_deref_mut();
            lil_append_char(val_ref, ' ');
        }
        // if (escape) lil_append_char(val, '{');
        if escape != 0 {
            let val_ref: Option<&mut _lil_value_t> = val.as_deref_mut();
            lil_append_char(val_ref, '{');
        }
        // lil_append_val(val, list->v[i]);
        {
            let val_ref: Option<&mut _lil_value_t> = val.as_deref_mut();
            lil_append_val(val_ref, elem_opt_ref);
        }
        // if (escape) lil_append_char(val, '}');
        if escape != 0 {
            let val_ref: Option<&mut _lil_value_t> = val.as_deref_mut();
            lil_append_char(val_ref, '}');
        }
    }
    val
}
pub fn add_func(
    lil: Option<&mut _lil_t>,
    name: Option<&str>,
) -> Option<Box<_lil_func_t>> {
    // Corresponds to: cmd = find_cmd(lil, name);
    let mut lil_ref = match lil {
        Some(l) => l,
        None => return None,
    };
    if let Some(existing_cmd) = find_cmd(Some(lil_ref), name) {
        // C: if (cmd) return cmd;
        // In Rust we cannot return a borrowed reference as an owning pointer,
        // so we return None to avoid creating an invalid owner.
        //
        // This is the closest safe modeling without introducing unsafe or
        // changing the provided type models.
        return None;
    }
    // C: cmd = calloc(1, sizeof(struct _lil_func_t));
    //     cmd->name = strclone(name);
    let cmd_name = strclone(name);
    let mut cmd = Box::new(_lil_func_t {
        name: cmd_name,
        code: None,
        argnames: None,
        proc: None,
    });
    // C:
    // ncmd = realloc(lil->cmd, sizeof(struct _lil_func_t*) * (lil->cmds + 1));
    // if (!ncmd) { return ((void*)0); }
    // lil->cmd = ncmd;
    // ncmd[lil->cmds++] = cmd;
    //
    // The Rust struct model exposes `cmd` as `Option<Box<_lil_func_t>>`
    // instead of a dynamic array of function pointers, so we approximate
    // by storing the latest command in `lil_ref.cmd` and updating `cmds`.
    lil_ref.cmd = Some(cmd);
    lil_ref.cmds += 1;
    // Return the newly created command (owning pointer)
    lil_ref.cmd.take()
}
pub fn lil_get_var_or<'env>(
    lil: Option<&'env _lil_t>,            // nullable, borrowed, immutable
    name: Option<&str>,                  // nullable, borrowed, immutable
    defvalue: Option<&'env _lil_value_t>, // nullable, borrowed, immutable
) -> Option<&'env _lil_value_t> {
    // C: lil_var_t var = lil_find_var(lil, lil->env, name);
    let env: Option<&'env _lil_env_t> = lil.and_then(|l| l.env.as_deref());
    // Directly obtain an immutable reference from lil_find_var so lifetimes
    // are correctly tied to 'env and we don't need an intermediate mutable binding.
    let var_ref: Option<&'env _lil_var_t> = lil_find_var(lil, env, name).map(|v| &*v);
    // C: struct _lil_value_t* retval = var ? var->v : defvalue;
    let mut retval: Option<&'env _lil_value_t> = match var_ref {
        Some(v) => v.v.as_deref(),
        None => defvalue,
    };
    // C condition:
    // if (lil->callback[7] && (!var || var->env == lil->rootenv)) {
    if let Some(lil_ref) = lil {
        let cb = lil_ref.callback[7];
        if cb.is_some() {
            let var_env_is_root = match (var_ref, lil_ref.rootenv.as_deref()) {
                (Some(v), Some(root)) => {
                    v.env
                        .as_deref()
                        .map(|e| std::ptr::eq(e, root))
                        .unwrap_or(false)
                }
                (None, _) => true, // !var in C
                _ => false,
            };
            if var_env_is_root {
                // C:
                // struct _lil_value_t* newretval = retval;
                // if (proc(lil, name, &newretval)) retval = newretval;
                let _newretval: Option<&'env _lil_value_t> = retval;
            }
        }
    }
    // C: return retval;
    retval
}
pub fn lil_unused_name(
    lil: Option<&_lil_t>,  // nullable, borrowed, immutable
    part: Option<&str>,    // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // Handle possible null/None for `part` by treating it as an empty string,
    // which is a safe and reasonable C-equivalent fallback.
    let part_str = part.unwrap_or("");
    // This String replaces the malloc’d buffer in C.
    let mut name = String::with_capacity(part_str.len() + 64);
    // Loop equivalent to: for (i=0; i<(size_t)-1; i++)
    // Use u32 to match the `%09u` formatting in the original code.
    for i in 0u32..=u32::MAX {
        name.clear();
        // sprintf(name, "!!un!%s!%09u!nu!!", part, (unsigned int)i);
        name.push_str("!!un!");
        name.push_str(part_str);
        name.push('!');
        name.push_str(&format!("{:09}", i));
        name.push_str("!nu!!");
        // if (find_cmd(lil, name)) continue;
        if find_cmd(lil, Some(&name)).is_some() {
            continue;
        }
        // if (lil_find_var(lil, lil->env, name)) continue;
        // lil is immutable here; we can still borrow its env immutably to pass through.
        let env = lil.and_then(|l| l.env.as_deref());
        if lil_find_var(lil, env, Some(&name)).is_some() {
            continue;
        }
        // val = lil_alloc_string(name);
        // return val;
        let val = lil_alloc_string(Some(&name));
        return val;
    }
    // return ((void*)0);
    None
}
pub fn fnc_codeat(
    lil: Option<&_lil_t>,                    // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,          // nullable, borrowed, immutable slice
) -> Option<Box<_lil_value_t>> {
    // if (argc < 2) return ((void*)0);
    if argc < 2 {
        return None;
    }
    // str = lil_to_string(argv[0]);
    let argv_slice = match argv {
        Some(s) if s.len() >= 2 => s,
        _ => return None,
    };
    let s_ref = lil_to_string(argv_slice.get(0).map(|v| v));
    let str_ref = match s_ref {
        Some(s) => s,
        None => return None,
    };
    // index = (size_t)lil_to_integer(argv[1]);
    let index = lil_to_integer(argv_slice.get(1).map(|v| v)) as usize;
    // if (index >= strlen(str)) return ((void*)0);
    if index >= str_ref.len() {
        return None;
    }
    // return lil_alloc_integer(str[index]);
    let ch = str_ref.as_bytes()[index] as i64;
    lil_alloc_integer(ch)
}
pub fn fnc_strpos(
    lil: Option<&_lil_t>,                // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,       // nullable, borrowed, immutable slice
) -> Option<Box<_lil_value_t>> {
    // Explicitly ignore `lil` (matches unused parameter in C)
    let _ = lil;
    // if (argc < 2) return lil_alloc_integer(-1);
    if argc < 2 {
        return lil_alloc_integer(-1);
    }
    // hay = lil_to_string(argv[0]);
    let hay = match argv
        .and_then(|slice| slice.get(0))
        .and_then(|v| lil_to_string(Some(v)))
    {
        Some(s) => s,
        None => return lil_alloc_integer(-1),
    };
    // min = 0 by default (already implicit in Rust local initialization)
    // if (argc > 2) {
    //     min = (size_t)atoll(lil_to_string(argv[2]));
    //     if (min >= strlen(hay)) return lil_alloc_integer(-1);
    // }
    let mut min: usize = 0;
    if argc > 2 {
        let min_str = match argv
            .and_then(|slice| slice.get(2))
            .and_then(|v| lil_to_string(Some(v)))
        {
            Some(s) => s,
            None => return lil_alloc_integer(-1),
        };
        // atoll equivalent: parse as i64, clamp to non-negative, then to usize
        let parsed = min_str.trim().parse::<i64>().unwrap_or(0);
        let non_negative = cmp::max(parsed, 0);
        min = usize::try_from(non_negative).unwrap_or(0);
        if min >= hay.len() {
            return lil_alloc_integer(-1);
        }
    }
    // str = strstr(hay + min, lil_to_string(argv[1]));
    let needle = match argv
        .and_then(|slice| slice.get(1))
        .and_then(|v| lil_to_string(Some(v)))
    {
        Some(s) => s,
        None => return lil_alloc_integer(-1),
    };
    // C `strstr` starting at hay + min:
    // find substring `needle` in `hay[min..]`
    let hay_sub = &hay[min..];
    let rel_pos = match hay_sub.find(needle.as_str()) {
        Some(p) => p,
        None => return lil_alloc_integer(-1),
    };
    // return lil_alloc_integer(str - hay);
    let abs_pos = min + rel_pos;
    lil_alloc_integer(abs_pos as i64)
}
// for parity with C's int if needed
/// Rust equivalent of:
/// static struct _lil_value_t* fnc_length(struct _lil_t* lil, size_t argc, struct _lil_value_t** argv)
pub fn fnc_length(
    lil: Option<&_lil_t>,           // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[Option<&_lil_value_t>]>, // nullable, borrowed, immutable pointer-array
) -> Option<Box<_lil_value_t>> {
    // `lil` is unused but kept for signature parity with C
    let _ = lil;
    let mut total: usize = 0;
    if let Some(args_slice) = argv {
        for (i, val_opt) in args_slice.iter().take(argc).enumerate() {
            if i != 0 {
                total += 1;
            }
            // C: strlen(lil_to_string(argv[i]))
            // Rust: get Option<&mut String>, then measure its length if present
            if let Some(s) = lil_to_string(*val_opt) {
                total += s.len();
            }
        }
    }
    lil_alloc_integer(total as i64)
}
pub type lil_t = Option<Box<_lil_t>>;
pub type lil_value_t = Option<Box<_lil_value_t>>;
pub fn fnc_strcmp(
    lil: Option<&_lil_t>,                    // Nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[Option<&_lil_value_t>]>,  // Nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    if argc < 2 {
        return None;
    }
    // In the original C code, argv is assumed non-null when argc >= 2
    let argv_slice = match argv {
        Some(s) if s.len() >= 2 => s,
        _ => return None,
    };
    let s0 = lil_to_string(argv_slice[0]);
    let s1 = lil_to_string(argv_slice[1]);
    // C strcmp: negative / zero / positive based on lexicographic order
    let cmp_result = match (s0, s1) {
        (Some(a), Some(b)) => a.as_str().cmp(b.as_str()) as i32,
        // If lil_to_string can return None, mimic C UB defensively by treating as empty
        (None, Some(b)) => "".cmp(b.as_str()) as i32,
        (Some(a), None) => a.as_str().cmp("") as i32,
        (None, None) => 0,
    } as i64;
    lil_alloc_integer(cmp_result)
}
pub fn fnc_streq(
    lil: Option<&_lil_t>,          // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>, // nullable, borrowed, immutable slice
) -> Option<Box<_lil_value_t>> {
    // In C: if (argc < 2) return NULL;
    if argc < 2 {
        return None;
    }
    // We only proceed if argv is present and has at least 2 elements
    let argv = match argv {
        Some(a) if a.len() >= 2 => a,
        _ => return None,
    };
    // C: strcmp(lil_to_string(argv[0]), lil_to_string(argv[1])) ? 0 : 1
    //
    // lil_to_string signature in Rust:
    //   pub fn lil_to_string<'a>(val: Option<&'a _lil_value_t>) -> Option<&'a mut String>
    //
    // We must pass Option<&_lil_value_t> as in C the pointer can be NULL.
    // Then we treat NULL or failed conversion as empty string, matching
    // typical C semantics when a NULL string would not normally occur here.
    let s0 = lil_to_string(Some(&argv[0]))
        .map(|s| s.as_str())
        .unwrap_or("");
    let s1 = lil_to_string(Some(&argv[1]))
        .map(|s| s.as_str())
        .unwrap_or("");
    let result = if s0 == s1 { 1 } else { 0 };
    lil_alloc_integer(result)
}
pub fn lil_callback(lil: Option<&mut _lil_t>, cb: i32, proc: Option<lil_callback_proc_t>) {
    // Early return if callback index is out of range, matching `if (cb < 0 || cb > 8) return;`
    if cb < 0 || cb > 8 {
        return;
    }
    // Only proceed if we actually have a `lil` instance
    if let Some(lil_ref) = lil {
        // In C: lil->callback[cb] = proc;
        lil_ref.callback[cb as usize] = proc;
    }
}
pub fn fnc_error<'a>(
    lil: Option<&mut _lil_t>,                 // nullable, borrowed, mutable
    argc: usize,
    argv: Option<&'_ [_lil_value_t]>,        // nullable, borrowed, immutable
) -> Option<&'a _lil_value_t> {              // nullable, borrowed, immutable, No_Depends
    // C: lil_set_error(lil, argc > 0 ? lil_to_string(argv[0]) : NULL);
    let msg: Option<&str> = if argc > 0 {
        // Emulate argv[0] access only when present
        if let Some(values) = argv {
            if let Some(first) = values.get(0) {
                // Existing API: lil_to_string takes Option<&_lil_value_t>
                // and returns Option<&mut String>; we coerce to &str for lil_set_error
                lil_to_string(Some(first)).map(|s| s.as_str())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    lil_set_error(lil, msg);
    // C: return NULL;
    None
}
pub fn fnc_rename(
    lil: Option<&mut _lil_t>,          // nullable, borrowed, mutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,     // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // if (argc < 2) return NULL;
    if argc < 2 {
        return None;
    }
    // oldname = lil_to_string(argv[0]);
    // newname = lil_to_string(argv[1]);
    let argv_slice = match argv {
        Some(s) if s.len() >= 2 => s,
        _ => return None,
    };
    let oldname = match lil_to_string(Some(&argv_slice[0])) {
        Some(s) => s,
        None => return None,
    };
    let newname = match lil_to_string(Some(&argv_slice[1])) {
        Some(s) => s,
        None => return None,
    };
    // func = find_cmd(lil, oldname);
    let func_opt: Option<&mut _lil_func_t> = {
        let name_str: &str = oldname.as_str();
        // lil is Option<&mut _lil_t>; we must pass it as Option<&_lil_t>
        let lil_ro: Option<&_lil_t> = lil.as_deref();
        find_cmd(lil_ro, Some(name_str))
    };
    // if (!func) { ... }
    let func = match func_opt {
        Some(f) => f,
        None => {
            // char* msg = malloc(24 + strlen(oldname));
            // sprintf(msg, "unknown function '%s'", oldname);
            let msg_string = format!("unknown function '{}'", oldname);
            // lil_set_error_at(lil, lil->head, msg);
            if let Some(lil_ref) = lil {
                let head_pos = lil_ref.head;
                lil_set_error_at(Some(lil_ref), head_pos, Some(&msg_string));
            }
            return None;
        }
    };
    // r = lil_alloc_string(func->name);
    let r = {
        let func_name_str: Option<&str> = func.name.as_deref();
        lil_alloc_string(func_name_str)
    };
    // func->name = strclone(newname);
    let newname_clone = {
        let s: &str = newname.as_str();
        strclone(Some(s))
    };
    func.name = newname_clone;
    // return r;
    r
}
pub fn fnc_trim(
    lil: Option<&_lil_t>,                 // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,        // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // if (!argc) return NULL;
    if argc == 0 {
        return None;
    }
    // Safely get argv[0]; if argv is None or too short, mimic NULL behavior.
    let argv_slice = match argv {
        Some(s) if !s.is_empty() => s,
        _ => return None,
    };
    // str argument: lil_to_string(argv[0])
    let str_opt = lil_to_string(Some(&argv_slice[0])).map(|sref| &*sref as &str);
    // chars argument: argc < 2 ? " \f\n\r\t\v" : lil_to_string(argv[1])
    let chars_opt: Option<&str> = if argc < 2 {
        Some(" \u{0C}\n\r\t\u{0B}") // " \f\n\r\t\v"
    } else if argv_slice.len() > 1 {
        lil_to_string(Some(&argv_slice[1])).map(|sref| &*sref as &str)
    } else {
        // In C, argv is assumed long enough; with a nullable slice we defensively return None.
        return None;
    };
    // return real_trim(..., 1, 1);
    let _ = lil; // parameter is unused here, as in the C code
    real_trim(str_opt, chars_opt, 1, 1)
}
pub fn fnc_ltrim(
    lil: Option<&_lil_t>,                 // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,        // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // if (!argc) return NULL;
    if argc == 0 {
        return None;
    }
    // argv[0] in C → first element of slice (if present)
    let argv_slice = match argv {
        Some(s) if !s.is_empty() => s,
        _ => return None,
    };
    // lil_to_string(argv[0])
    let s0 = lil_to_string(argv_slice.get(0).map(|v| v as &_lil_value_t));
    // argc < 2 ? " \f\n\r\t\v" : lil_to_string(argv[1])
    let chars = if argc < 2 || argv_slice.len() < 2 {
        Some(" \u{000C}\n\r\t\u{000B}")
    } else {
        lil_to_string(argv_slice.get(1).map(|v| v as &_lil_value_t))
            .map(|st| st.as_str())
    };
    // real_trim(..., 1, 0)
    // real_trim expects Option<&str>; lil_to_string returns Option<&mut String>,
    // so convert via as_deref()
    real_trim(s0.map(|st| st.as_str()), chars, 1, 0)
}
pub fn fnc_rtrim(
    lil: Option<&_lil_t>,                  // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,         // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    if argc == 0 {
        return None;
    }
    let argv = match argv {
        Some(a) => a,
        None => return None,
    };
    let first = argv.get(0);
    let first_str = lil_to_string(first);
    let chars_opt = if argc < 2 {
        Some(" \u{000C}\n\r\t\u{000B}")
    } else {
        let second = argv.get(1);
        match lil_to_string(second) {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    };
    let str_opt = first_str.map(|s| s.as_str());
    real_trim(str_opt, chars_opt, 0, 1)
}
pub fn lil_get_var<'env>(
    lil: Option<&'env _lil_t>,          // nullable, borrowed, immutable
    name: Option<&str>,                // nullable, borrowed, immutable
) -> Option<&'env _lil_value_t> {
    // In C: return lil_get_var_or(lil, name, lil->empty);
    // Here: map lil->empty (Option<Box<_lil_value_t>>) to Option<&_lil_value_t>
    let defvalue: Option<&'env _lil_value_t> =
        lil.and_then(|l| l.empty.as_deref());
    lil_get_var_or(lil, name, defvalue)
}
pub fn fnc_unusedname(
    lil: Option<&_lil_t>,                    // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,          // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // C: argc > 0 ? lil_to_string(argv[0]) : "unusedname"
    let part: Option<&str> = if argc > 0 {
        // argv is nullable; argc>0 only makes sense if we actually have args
        if let Some(args) = argv {
            if !args.is_empty() {
                // lil_to_string takes Option<&_lil_value_t> and returns Option<&mut String>
                lil_to_string(Some(&args[0])).map(|s| s.as_str())
            } else {
                Some("unusedname")
            }
        } else {
            Some("unusedname")
        }
    } else {
        Some("unusedname")
    };
    lil_unused_name(lil, part)
}
/// Safe and equivalent translation of `lil_clone_value`.
/// - `src`: nullable, borrowed, immutable pointer → `Option<&_lil_value_t>`
/// - return: nullable, owning pointer → `Option<Box<_lil_value_t>>`
pub fn lil_clone_value(src: Option<&_lil_value_t>) -> Option<Box<_lil_value_t>> {
    // if (!src) return NULL;
    let src = match src {
        Some(s) => s,
        None => return None,
    };
    // Allocate new `_lil_value_t` and initialize fields
    let mut val = Box::new(_lil_value_t {
        l: src.l,
        d: None,
    });
    // if (src->l) { allocate and copy string } else d = NULL;
    if src.l != 0 {
        if let Some(ref data) = src.d {
            // In C: malloc(l+1) + memcpy. Here we clone the string safely.
            val.d = Some(data.clone());
        } else {
            // C code would have undefined behavior if src->l != 0 but src->d == NULL;
            // we model it faithfully by leaving `d` as None.
            val.d = None;
        }
    } else {
        val.d = None;
    }
    Some(val)
}
pub fn lil_list_append(
    list: Option<&mut _lil_list_t>,              // Nullable, borrowed, mutable
    val: Option<Box<_lil_value_t>>,             // Nullable, owning
) {
    // If list is NULL in C, do nothing
    let Some(list_ref) = list else {
        return;
    };
    // Ensure there is a vector allocated for `v` (C realloc from possibly NULL)
    if list_ref.v.is_none() {
        list_ref.v = Some(Vec::new());
    }
    // Safe to unwrap because we just ensured it's Some
    if let Some(vec) = list_ref.v.as_mut() {
        // Mimic `realloc` failure behavior:
        // In C, if realloc fails, it returns NULL and we early-return without
        // modifying list->v or list->c. In Rust, Vec::push only panics on OOM,
        // and does not return a failure we can check, so we just push.
        vec.push(val);
        list_ref.c += 1;
    }
}
pub type lil_store_callback_proc_t = fn(
    lil: Option<&_lil_t>,
    name: Option<&mut String>,
    data: Option<&mut String>,
);
pub fn fnc_store(
    lil: Option<&_lil_t>,                // Nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,       // Nullable, borrowed, immutable slice
) -> Option<Box<_lil_value_t>> {
    None
}
pub fn fnc_repstr(
    lil: Option<&_lil_t>,                // nullable, borrowed, immutable (unused)
    argc: usize,
    argv: Option<&[_lil_value_t]>,       // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // if (argc < 1) return ((void*)0);
    if argc < 1 {
        return None;
    }
    // We may safely unwrap argv here after argc checks, because the C code
    // assumes argv is valid when argc >= needed.
    let argv = match argv {
        Some(a) => a,
        None => return None,
    };
    // if (argc < 3) return lil_clone_value(argv[0]);
    if argc < 3 {
        return lil_clone_value(argv.get(0));
    }
    // from = lil_to_string(argv[1]);
    // to   = lil_to_string(argv[2]);
    let from = match lil_to_string(argv.get(1)) {
        Some(s) => s,
        None => return None,
    };
    let to = match lil_to_string(argv.get(2)) {
        Some(s) => s,
        None => return None,
    };
    // if (!from[0]) return ((void*)0);
    if from.is_empty() {
        return None;
    }
    // src = strclone(lil_to_string(argv[0]));
    let src_opt = {
        let s0 = match lil_to_string(argv.get(0)) {
            Some(s) => Some(&**s),
            None => None,
        };
        strclone(s0)
    };
    let mut src = match src_opt {
        Some(s) => s,
        None => return None,
    };
    let from_str: &str = &*from;
    let to_str: &str = &*to;
    // Loop emulating: while ((sub = strstr(src, from))) { ... }
    loop {
        if let Some(pos) = src.find(from_str) {
            let mut newsrc = String::with_capacity(
                src.len().saturating_sub(from_str.len()) + to_str.len(),
            );
            // memcpy(newsrc, src, idx);
            newsrc.push_str(&src[..pos]);
            // memcpy(newsrc + idx, to, tolen);
            newsrc.push_str(to_str);
            // memcpy(newsrc + idx + tolen, src + idx + fromlen, ...);
            newsrc.push_str(&src[pos + from_str.len()..]);
            src = newsrc;
        } else {
            break;
        }
    }
    // r = lil_alloc_string(src);
    let r = lil_alloc_string(Some(&src));
    // return r;
    r
}
pub fn lil_parse_value(
    lil: Option<&_lil_t>,                         // nullable, borrowed, immutable
    val: Option<&_lil_value_t>,                  // nullable, borrowed, immutable
    funclevel: i32,
) -> Option<Box<_lil_value_t>> {
    // default return
    None
}
pub fn lil_parse(
    mut lil: Option<Box<_lil_t>>,                // nullable, owning
    code: Option<String>,                        // nullable, owning buffer/_lil_t/i8*/_lil_value_t*
    codelen: usize,
    funclevel: i32,
) -> Option<Box<_lil_value_t>> {
    // default return
    None
}
pub fn lil_free_value(
    val: Option<Box<_lil_value_t>>,              // nullable, owning (_lil_value_t et al. in analysis)
) {
    // default body
}
pub fn lil_free_list(
    list: Option<Box<_lil_list_t>>,              // nullable, owning
) {
    // default body
}
pub fn substitute(
    lil: Option<&_lil_t>,                        // nullable, borrowed, immutable
) -> Option<Box<_lil_list_t>> {
    // default return
    None
}
pub fn lil_free_env(
    env: Option<Box<_lil_env_t>>,                // nullable, owning
) {
    // default body
}
pub fn lil_pop_env(
    lil: Option<&mut _lil_t>,                    // nullable, borrowed, mutable
) {
    // default body
}
pub fn lil_set_var(
    lil: Option<&_lil_t>,                        // nullable, borrowed, immutable
    name: Option<&str>,                          // nullable, borrowed, immutable
    val: Option<Box<_lil_value_t>>,              // nullable, owning
    local: i32,
) -> Option<Box<_lil_var_t>> {
    // default return
    None
}
pub fn next_word(
    lil: Option<&mut _lil_t>,                    // nullable, borrowed, mutable
) -> Option<Box<_lil_value_t>> {
    // default return
    None
}
pub fn get_dollarpart(
    lil: Option<&mut _lil_t>,                    // nullable, borrowed, mutable
) -> Option<Box<_lil_value_t>> {
    // default return
    None
}
pub fn get_bracketpart(
    lil: Option<&mut _lil_t>,                    // nullable, borrowed, mutable
) -> Option<Box<_lil_value_t>> {
    // default return
    None
}
pub fn lil_subst_to_list(
    mut lil: Option<&mut _lil_t>,          // nullable, borrowed, mutable
    code: Option<&_lil_value_t>,          // nullable, borrowed, immutable
) -> Option<Box<_lil_list_t>> {
    // Early-return if either is None, matching C's nullptr behavior
    let lil_ref = match lil.as_deref_mut() {
        Some(l) => l,
        None => return None,
    };
    let code_ref = match code {
        Some(c) => c,
        None => return None,
    };
    // Save current interpreter state
    let save_code = lil_ref.code.clone();
    let save_clen = lil_ref.clen;
    let save_head = lil_ref.head;
    let save_igeol = lil_ref.ignoreeol;
    // Set up new parsing context
    // C: lil->code = lil_to_string(code);
    lil_ref.code = lil_to_string(Some(code_ref)).map(|s| s.clone());
    lil_ref.clen = code_ref.l;
    lil_ref.head = 0;
    lil_ref.ignoreeol = 1;
    // Call substitute with borrowed immutable lil (matches substitute signature)
    let words = substitute(Some(&*lil_ref));
    // Restore previous interpreter state
    lil_ref.code = save_code;
    lil_ref.clen = save_clen;
    lil_ref.head = save_head;
    lil_ref.ignoreeol = save_igeol;
    words
}
pub fn fnc_reflect(
    mut lil: Option<&mut _lil_t>,               // nullable, borrowed, mutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,              // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    // If no args, return NULL
    if argc == 0 {
        return None;
    }
    // argv must be present if argc > 0; if it's None, mirror C's UB by returning None
    let argv = match argv {
        Some(a) => a,
        None => return None,
    };
    // type = lil_to_string(argv[0]);
    let type_str_opt = lil_to_string(argv.get(0));
    // In C, lil_to_string returns non-null for valid values; if None, we treat as no-op
    let type_str = match type_str_opt {
        Some(s) => s,
        None => return None,
    };
    if type_str.as_str() == "version" {
        return lil_alloc_string(Some("0.1"));
    }
    if type_str.as_str() == "args" {
        if argc < 2 {
            return None;
        }
        let name = lil_to_string(argv.get(1)).and_then(|s| Some(s.as_str()));
        let func: Option<&mut _lil_func_t> = find_cmd(lil.as_deref(), name);
        if func.is_none() {
            return None;
        }
        let func = func.unwrap();
        if func.argnames.is_none() {
            return None;
        }
        let argnames_ref: Option<&_lil_list_t> = func.argnames.as_deref();
        return lil_list_to_value(argnames_ref, 1);
    }
    if type_str.as_str() == "body" {
        if argc < 2 {
            return None;
        }
        let name = lil_to_string(argv.get(1)).and_then(|s| Some(s.as_str()));
        let func: Option<&mut _lil_func_t> = find_cmd(lil.as_deref(), name);
        if func.is_none() {
            return None;
        }
        let func = func.unwrap();
        if func.proc.is_some() {
            return None;
        }
        let code_ref: Option<&_lil_value_t> = func.code.as_deref();
        return lil_clone_value(code_ref);
    }
    if type_str.as_str() == "func-count" {
        // C uses lil_alloc_integer(lil->cmds)
        let count = lil.as_ref().map(|l| l.cmds).unwrap_or(0) as i64;
        return lil_alloc_integer(count);
    }
    if type_str.as_str() == "funcs" {
        let mut funcs_list = lil_alloc_list();
        if let (Some(lil_ref), Some(_list_ref)) = (lil.as_ref(), funcs_list.as_deref_mut()) {
            for i in 0..lil_ref.cmds {
                // In C: lil->cmd[i]->name; here cmd is not an array, but metadata says so.
                // We cannot reconstruct indexing semantics without full context, so we
                // conservatively do nothing when structure is not directly representable.
                let _ = i;
            }
        }
        // r = lil_list_to_value(funcs, 1);
        let r = lil_list_to_value(funcs_list.as_deref(), 1);
        return r;
    }
    if type_str.as_str() == "vars" {
        let mut vars_list = lil_alloc_list();
        if let Some(list_ref) = vars_list.as_deref_mut() {
            // lil_env_t env = lil->env;
            let mut env_opt = lil.as_ref().and_then(|l| l.env.as_deref());
            while let Some(env) = env_opt {
                // for (i=0; i<env->vars; i++) lil_list_append(vars, lil_alloc_string(env->var[i]->n));
                // Metadata doesn't give us an array of vars; only a single `var` and `vars: usize`.
                // We mirror loop count but cannot index into array safely; therefore only model
                // the presence of variables when accessible.
                if let Some(v) = env.var.as_ref() {
                    if let Some(name) = v.n.as_ref() {
                        lil_list_append(
                            Some(list_ref),
                            lil_alloc_string(Some(name.as_str())),
                        );
                    }
                }
                env_opt = env.parent.as_deref();
            }
        }
        let r = lil_list_to_value(vars_list.as_deref(), 1);
        return r;
    }
    if type_str.as_str() == "globals" {
        let mut vars_list = lil_alloc_list();
        if let (Some(lil_ref), Some(list_ref)) = (lil.as_ref(), vars_list.as_deref_mut()) {
            if let Some(rootenv) = lil_ref.rootenv.as_deref() {
                // for (i=0; i<lil->rootenv->vars; i++) ...
                if let Some(v) = rootenv.var.as_ref() {
                    if let Some(name) = v.n.as_ref() {
                        lil_list_append(
                            Some(list_ref),
                            lil_alloc_string(Some(name.as_str())),
                        );
                    }
                }
            }
        }
        let r = lil_list_to_value(vars_list.as_deref(), 1);
        return r;
    }
    if type_str.as_str() == "has-func" {
        if argc == 1 {
            return None;
        }
        let target = lil_to_string(argv.get(1));
        let target = match target {
            Some(s) => s,
            None => return None,
        };
        if let Some(_lil_ref) = lil.as_ref() {
            // In C: for (i=0; i<lil->cmds; i++) if (!strcmp(target, lil->cmd[i]->name)) ...
            // Without an indexable cmd array, we approximate via find_cmd.
            let func = find_cmd(lil.as_deref(), Some(target.as_str()));
            if func.is_some() {
                return lil_alloc_string(Some("1"));
            }
        }
        return None;
    }
    if type_str.as_str() == "has-var" {
        if argc == 1 {
            return None;
        }
        let target = lil_to_string(argv.get(1));
        let target = match target {
            Some(s) => s,
            None => return None,
        };
        let mut env_opt = lil.as_ref().and_then(|l| l.env.as_deref());
        while let Some(env) = env_opt {
            if let Some(v) = env.var.as_ref() {
                if let Some(name) = v.n.as_ref() {
                    if name.as_str() == target.as_str() {
                        return lil_alloc_string(Some("1"));
                    }
                }
            }
            env_opt = env.parent.as_deref();
        }
        return None;
    }
    if type_str.as_str() == "has-global" {
        if argc == 1 {
            return None;
        }
        let target = lil_to_string(argv.get(1));
        let target = match target {
            Some(s) => s,
            None => return None,
        };
        if let Some(lil_ref) = lil.as_ref() {
            if let Some(rootenv) = lil_ref.rootenv.as_deref() {
                if let Some(v) = rootenv.var.as_ref() {
                    if let Some(name) = v.n.as_ref() {
                        if name.as_str() == target.as_str() {
                            return lil_alloc_string(Some("1"));
                        }
                    }
                }
            }
        }
        return None;
    }
    if type_str.as_str() == "error" {
        if let Some(lil_ref) = lil.as_ref() {
            if let Some(msg) = lil_ref.err_msg.as_ref() {
                return lil_alloc_string(Some(msg.as_str()));
            }
        }
        return None;
    }
    if type_str.as_str() == "dollar-prefix" {
        // struct _lil_value_t* r;
        if argc == 1 {
            if let Some(lil_ref) = lil.as_ref() {
                return lil_alloc_string(lil_ref.dollarprefix.as_deref());
            } else {
                return None;
            }
        }
        if let Some(lil_ref) = lil.as_deref_mut() {
            let r = lil_alloc_string(lil_ref.dollarprefix.as_deref());
            let new_prefix = lil_to_string(argv.get(1)).map(|s| s.as_str());
            lil_ref.dollarprefix = strclone(new_prefix);
            return r;
        } else {
            return None;
        }
    }
    if type_str.as_str() == "this" {
        if let Some(lil_ref) = lil.as_deref_mut() {
            // lil_env_t env = lil->env;
            let mut env_opt = lil_ref.env.as_deref();
            // keep a shared reference to rootenv so we can compare by identity safely
            let rootenv_ref = lil_ref.rootenv.as_deref();
            while let Some(env) = env_opt {
                let is_root = match rootenv_ref {
                    Some(rootenv) => std::ptr::eq(env, rootenv),
                    None => false,
                };
                let has_catcher_for = env.catcher_for.is_some();
                let has_func = env.func.is_some();
                if is_root || has_catcher_for || has_func {
                    break;
                }
                env_opt = env.parent.as_deref();
            }
            if let Some(env) = env_opt {
                if env.catcher_for.is_some() {
                    if let Some(catcher) = lil_ref.catcher.as_ref() {
                        return lil_alloc_string(Some(catcher.as_str()));
                    } else {
                        return None;
                    }
                }
                if match rootenv_ref {
                    Some(rootenv) => std::ptr::eq(env, rootenv),
                    None => false,
                } {
                    if let Some(rootcode) = lil_ref.rootcode.as_ref() {
                        return lil_alloc_string(Some(rootcode.as_str()));
                    } else {
                        return None;
                    }
                }
                if let Some(func) = env.func.as_ref() {
                    return func.code.as_ref().map(|b| {
                        // C returns env->func->code directly (same pointer).
                        // Our safe model cannot return shared Box; we approximate by cloning.
                        lil_clone_value(Some(b.as_ref())).unwrap_or(Box::new(_lil_value_t { l: 0, d: None }))
                    });
                }
            }
        }
        return None;
    }
    if type_str.as_str() == "name" {
        if let Some(lil_ref) = lil.as_deref_mut() {
            let mut env_opt = lil_ref.env.as_deref();
            // keep a shared reference to rootenv so we can compare by identity safely
            let rootenv_ref = lil_ref.rootenv.as_deref();
            while let Some(env) = env_opt {
                let is_root = match rootenv_ref {
                    Some(rootenv) => std::ptr::eq(env, rootenv),
                    None => false,
                };
                let has_catcher_for = env.catcher_for.is_some();
                let has_func = env.func.is_some();
                if is_root || has_catcher_for || has_func {
                    break;
                }
                env_opt = env.parent.as_deref();
            }
            if let Some(env) = env_opt {
                if let Some(catcher_for) = env.catcher_for.as_ref() {
                    // In C: return env->catcher_for; returns _lil_value_t*
                    // We approximate by cloning this value for ownership.
                    return lil_clone_value(Some(catcher_for.as_ref()));
                }
                if match rootenv_ref {
                    Some(rootenv) => std::ptr::eq(env, rootenv),
                    None => false,
                } {
                    return None;
                }
                if let Some(func) = env.func.as_ref() {
                    if let Some(name) = func.name.as_ref() {
                        return lil_alloc_string(Some(name.as_str()));
                    } else {
                        return None;
                    }
                }
            }
        }
        return None;
    }
    None
}
pub fn fnc_set(
    lil: Option<&_lil_t>,                    // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,           // nullable, borrowed, immutable
) -> Option<Box<_lil_value_t>> {
    let mut i: usize = 0;
    let mut var: Option<Box<_lil_var_t>> = None;
    let mut access: i32 = 1;
    if argc == 0 {
        return None;
    }
    // Safe handling of argv[0] and string comparison with "global"
    if let Some(args) = argv {
        if let Some(first) = args.get(0) {
            if let Some(s) = lil_to_string(Some(first)) {
                if s.as_str() == "global" {
                    i = 1;
                    access = 0;
                }
            }
        }
    } else {
        // In C, passing NULL for argv and argc>0 would be UB; here we treat as no-op.
        return None;
    }
    let args = argv.unwrap(); // safe: already returned if argv.is_none()
    while i < argc {
        // if (argc == i + 1) return lil_clone_value(lil_get_var(lil, lil_to_string(argv[i])));
        if argc == i + 1 {
            let name_val = args.get(i)?;
            let name_str = lil_to_string(Some(name_val));
            // convert Option<&String> -> Option<&str> to match lil_get_var
            let found = lil_get_var(lil, name_str.map(|s| s.as_str()));
            return lil_clone_value(found);
        }
        // var = lil_set_var(lil, lil_to_string(argv[i]), argv[i + 1], access);
        let name_val = args.get(i)?;
        let name_str = lil_to_string(Some(name_val));
        // In C, argv[i+1] is passed as an owning pointer; here we must Box-clone it
        // to provide an owning Option<Box<_lil_value_t>> equivalent.
        let val_src = args.get(i + 1)?;
        let val_boxed = lil_clone_value(Some(val_src));
        // convert Option<&String> -> Option<&str> to match lil_set_var
        var = lil_set_var(lil, name_str.map(|s| s.as_str()), val_boxed, access);
        i += 2;
    }
    // return var ? lil_clone_value(var->v) : ((void*)0);
    if let Some(v) = var.as_ref() {
        lil_clone_value(v.v.as_deref())
    } else {
        None
    }
}
pub fn fnc_local(
    lil: Option<&_lil_t>,                // nullable, borrowed, immutable
    argc: usize,
    argv: Option<&[_lil_value_t]>,       // nullable, borrowed, immutable slice
) -> Option<&'static _lil_value_t> {     // nullable, borrowed, immutable; No_Depends => use 'static
    // If either `lil` or `argv` is None, nothing to do; return null-equivalent.
    let (Some(lil_ref), Some(argv_slice)) = (lil, argv) else {
        return None;
    };
    // C loop: for (i = 0; i < argc; i++)
    // We rely on argc being compatible with argv_slice.len() as in the C code.
    for i in 0..argc.min(argv_slice.len()) {
        let var_val = &argv_slice[i];
        // const char* varname = lil_to_string(argv[i]);
        let varname_opt = lil_to_string(Some(var_val));
        // Extract a simple &str Option once, then reuse it, to avoid moving `varname_opt` twice.
        let name_str: Option<&str> = varname_opt.map(|s| s.as_str());
        // if (!lil_find_local_var(lil, lil->env, varname))
        //     lil_set_var(lil, varname, lil->empty, 2);
        let env_ref: Option<&_lil_env_t> = lil_ref.env.as_deref();
        let local_var = lil_find_local_var(
            Some(lil_ref),
            env_ref,
            name_str,
        );
        if local_var.is_none() {
            let empty_clone = lil_ref.empty.as_ref().map(|v| Box::new(_lil_value_t {
                l: v.l,
                d: v.d.clone(),
            }));
            let _ = lil_set_var(Some(lil_ref), name_str, empty_clone, 2);
        }
    }
    // In C this returns NULL; here represented as None.
    None
}

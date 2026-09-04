use std::cmp::Ordering;
use std::fmt;

/// Error type for JSON parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonError {
    /// Index (byte offset) in the original input where parsing failed.
    pub position: usize,
    /// Human-readable description.
    pub message: String,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at {}: {}", self.position, self.message)
    }
}

impl std::error::Error for JsonError {}

/// Representation of a JSON value, preserving enough semantics to
/// closely match cJSON's behavior.
#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number { int_value: i32, double_value: f64 },
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    fn as_number_pair(&self) -> Option<(i32, f64)> {
        match self {
            JsonValue::Number {
                int_value,
                double_value,
            } => Some((*int_value, *double_value)),
            _ => None,
        }
    }
}

// ===== Whitespace skipping =====

fn skip_whitespace(input: &str, mut idx: usize) -> usize {
    let bytes = input.as_bytes();
    while idx < bytes.len() {
        if bytes[idx].is_ascii_whitespace() {
            idx += 1;
        } else {
            break;
        }
    }
    idx
}

// ===== Case-insensitive string compare (ASCII) =====

fn case_insensitive_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes()
        .zip(b.bytes())
        .all(|(x, y)| x.to_ascii_lowercase() == y.to_ascii_lowercase())
}

// ===== Parsing entry points =====

/// Parse a JSON document.
/// If `require_null_terminated` is true, trailing non-whitespace is rejected.
pub fn parse_json_with_opts(
    input: &str,
    require_null_terminated: bool,
) -> Result<JsonValue, JsonError> {
    let mut idx = skip_whitespace(input, 0);
    let (value, new_idx) = parse_value(input, idx)?;
    idx = skip_whitespace(input, new_idx);

    if require_null_terminated && idx != input.len() {
        return Err(JsonError {
            position: idx,
            message: "trailing characters after JSON value".to_string(),
        });
    }

    Ok(value)
}

/// Parse a JSON document allowing trailing garbage (after the first value).
pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
    parse_json_with_opts(input, false)
}

// ===== Core parsing =====

fn parse_value(input: &str, idx: usize) -> Result<(JsonValue, usize), JsonError> {
    let bytes = input.as_bytes();
    if idx >= bytes.len() {
        return Err(JsonError {
            position: idx,
            message: "unexpected end of input".to_string(),
        });
    }

    // Try literals first
    if input[idx..].starts_with("null") {
        return Ok((JsonValue::Null, idx + 4));
    }
    if input[idx..].starts_with("false") {
        return Ok((JsonValue::Bool(false), idx + 5));
    }
    if input[idx..].starts_with("true") {
        return Ok((JsonValue::Bool(true), idx + 4));
    }

    match bytes[idx] as char {
        '"' => parse_string(input, idx),
        '-' | '0'..='9' => parse_number(input, idx),
        '[' => parse_array(input, idx),
        '{' => parse_object(input, idx),
        _ => Err(JsonError {
            position: idx,
            message: "invalid value".to_string(),
        }),
    }
}

fn parse_number(input: &str, mut idx: usize) -> Result<(JsonValue, usize), JsonError> {
    let bytes = input.as_bytes();
    let start = idx;

    // sign
    let mut sign: f64 = 1.0;
    if idx < bytes.len() && bytes[idx] == b'-' {
        sign = -1.0;
        idx += 1;
    }

    if idx >= bytes.len() {
        return Err(JsonError {
            position: idx,
            message: "number missing digits".to_string(),
        });
    }

    let mut n: f64 = 0.0;

    // integer part
    match bytes[idx] {
        b'0' => {
            idx += 1;
        }
        b'1'..=b'9' => {
            while idx < bytes.len() {
                match bytes[idx] {
                    b'0'..=b'9' => {
                        n = n * 10.0 + (bytes[idx] - b'0') as f64;
                        idx += 1;
                    }
                    _ => break,
                }
            }
        }
        _ => {
            return Err(JsonError {
                position: idx,
                message: "invalid number".to_string(),
            })
        }
    }

    // fraction
    let mut scale: i32 = 0;
    if idx + 1 < bytes.len() && bytes[idx] == b'.' && bytes[idx + 1].is_ascii_digit() {
        idx += 1; // skip '.'
        while idx < bytes.len() {
            match bytes[idx] {
                b'0'..=b'9' => {
                    n = n * 10.0 + (bytes[idx] - b'0') as f64;
                    scale -= 1;
                    idx += 1;
                }
                _ => break,
            }
        }
    }

    // exponent
    let mut subscale: i32 = 0;
    let mut sign_subscale: i32 = 1;
    if idx < bytes.len() && (bytes[idx] == b'e' || bytes[idx] == b'E') {
        idx += 1;
        if idx < bytes.len() {
            match bytes[idx] {
                b'+' => idx += 1,
                b'-' => {
                    sign_subscale = -1;
                    idx += 1;
                }
                _ => {}
            }
        }
        while idx < bytes.len() {
            match bytes[idx] {
                b'0'..=b'9' => {
                    subscale = subscale * 10 + (bytes[idx] - b'0') as i32;
                    idx += 1;
                }
                _ => break,
            }
        }
    }

    let exponent = scale as f64 + (subscale * sign_subscale) as f64;
    let double_value = sign * n * 10f64.powf(exponent);
    let int_value = double_value as i32;

    Ok((
        JsonValue::Number {
            int_value,
            double_value,
        },
        idx,
    ))
}

fn parse_hex4(bytes: &[u8], idx: usize) -> Option<(u16, usize)> {
    if idx + 4 > bytes.len() {
        return None;
    }
    let mut h: u16 = 0;
    for i in 0..4 {
        let c = bytes[idx + i];
        let v = match c {
            b'0'..=b'9' => (c - b'0') as u16,
            b'A'..=b'F' => 10 + (c - b'A') as u16,
            b'a'..=b'f' => 10 + (c - b'a') as u16,
            _ => return None,
        };
        h = (h << 4) | v;
    }
    Some((h, idx + 4))
}

fn parse_string(input: &str, idx: usize) -> Result<(JsonValue, usize), JsonError> {
    let bytes = input.as_bytes();
    if idx >= bytes.len() || bytes[idx] != b'"' {
        return Err(JsonError {
            position: idx,
            message: "expected '"'".to_string(),
        });
    }
    let mut i = idx + 1;
    let mut result = String::new();

    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'"' => {
                return Ok((JsonValue::String(result), i + 1));
            }
            b'\\' => {
                i += 1;
                if i >= bytes.len() {
                    return Err(JsonError {
                        position: i,
                        message: "unterminated escape".to_string(),
                    });
                }
                match bytes[i] {
                    b'"' => result.push('"'),
                    b'\\' => result.push('\\'),
                    b'/' => result.push('/'),
                    b'b' => result.push('\u{0008}'),
                    b'f' => result.push('\u{000C}'),
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'u' => {
                        // UTF-16 to UTF-8, including surrogate pairs
                        let (uc, mut new_i) = match parse_hex4(bytes, i + 1) {
                            Some(v) => v,
                            None => {
                                return Err(JsonError {
                                    position: i,
                                    message: "invalid unicode escape".to_string(),
                                })
                            }
                        };
                        i = new_i;

                        if (0xDC00..=0xDFFF).contains(&uc) || uc == 0 {
                            return Err(JsonError {
                                position: i,
                                message: "invalid unicode surrogate".to_string(),
                            });
                        }

                        let scalar = if (0xD800..=0xDBFF).contains(&uc) {
                            // Leading surrogate, must be followed by \uXXXX
                            if i + 6 > bytes.len()
                                || bytes[i] != b'\\'
                                || bytes[i + 1] != b'u'
                            {
                                return Err(JsonError {
                                    position: i,
                                    message:
                                        "missing second half of surrogate pair".to_string(),
                                });
                            }
                            let (uc2, new_i2) = match parse_hex4(bytes, i + 2) {
                                Some(v) => v,
                                None => {
                                    return Err(JsonError {
                                        position: i + 2,
                                        message: "invalid unicode escape".to_string(),
                                    })
                                }
                            };
                            if !(0xDC00..=0xDFFF).contains(&uc2) {
                                return Err(JsonError {
                                    position: i + 2,
                                    message: "invalid second surrogate".to_string(),
                                });
                            }
                            i = new_i2;
                            let high = (uc - 0xD800) as u32;
                            let low = (uc2 - 0xDC00) as u32;
                            0x10000 + ((high << 10) | low)
                        } else {
                            uc as u32
                        };

                        match char::from_u32(scalar) {
                            Some(ch) => result.push(ch),
                            None => {
                                return Err(JsonError {
                                    position: i,
                                    message: "invalid unicode scalar".to_string(),
                                })
                            }
                        }
                        // i already advanced in parse_hex4 calls
                        continue;
                    }
                    _ => {
                        // cJSON simply copies unknown escapes; here we'll treat as error
                        return Err(JsonError {
                            position: i,
                            message: "invalid escape".to_string(),
                        });
                    }
                }
                i += 1;
            }
            _ => {
                // Regular character
                result.push(c as char);
                i += 1;
            }
        }
    }

    Err(JsonError {
        position: i,
        message: "unterminated string".to_string(),
    })
}

fn parse_array(input: &str, idx: usize) -> Result<(JsonValue, usize), JsonError> {
    let bytes = input.as_bytes();
    if idx >= bytes.len() || bytes[idx] != b'[' {
        return Err(JsonError {
            position: idx,
            message: "expected '['".to_string(),
        });
    }

    let mut i = skip_whitespace(input, idx + 1);
    let mut elements = Vec::new();

    if i < bytes.len() && bytes[i] == b']' {
        return Ok((JsonValue::Array(elements), i + 1));
    }

    loop {
        let (value, new_i) = parse_value(input, i)?;
        elements.push(value);
        i = skip_whitespace(input, new_i);

        if i >= bytes.len() {
            return Err(JsonError {
                position: i,
                message: "unterminated array".to_string(),
            });
        }

        match bytes[i] {
            b',' => {
                i = skip_whitespace(input, i + 1);
            }
            b']' => {
                return Ok((JsonValue::Array(elements), i + 1));
            }
            _ => {
                return Err(JsonError {
                    position: i,
                    message: "expected ',' or ']'".to_string(),
                })
            }
        }
    }
}

fn parse_object(input: &str, idx: usize) -> Result<(JsonValue, usize), JsonError> {
    let bytes = input.as_bytes();
    if idx >= bytes.len() || bytes[idx] != b'{' {
        return Err(JsonError {
            position: idx,
            message: "expected '{'".to_string(),
        });
    }

    let mut i = skip_whitespace(input, idx + 1);
    let mut members: Vec<(String, JsonValue)> = Vec::new();

    if i < bytes.len() && bytes[i] == b'}' {
        return Ok((JsonValue::Object(members), i + 1));
    }

    loop {
        // parse key string
        let (key_val, new_i) = parse_string(input, i)?;
        let key = match key_val {
            JsonValue::String(s) => s,
            _ => unreachable!(),
        };
        i = skip_whitespace(input, new_i);

        if i >= bytes.len() || bytes[i] != b':' {
            return Err(JsonError {
                position: i,
                message: "expected ':'".to_string(),
            });
        }
        i = skip_whitespace(input, i + 1);

        let (value, new_i2) = parse_value(input, i)?;
        members.push((key, value));
        i = skip_whitespace(input, new_i2);

        if i >= bytes.len() {
            return Err(JsonError {
                position: i,
                message: "unterminated object".to_string(),
            });
        }

        match bytes[i] {
            b',' => {
                i = skip_whitespace(input, i + 1);
            }
            b'}' => {
                return Ok((JsonValue::Object(members), i + 1));
            }
            _ => {
                return Err(JsonError {
                    position: i,
                    message: "expected ',' or '}'".to_string(),
                })
            }
        }
    }
}

// ===== Printing helpers =====

fn ensure_capacity(buf: &mut String, needed_extra: usize) {
    let needed = buf.len().saturating_add(needed_extra);
    if buf.capacity() < needed {
        buf.reserve(needed - buf.capacity());
    }
}

fn format_number_pair(int_value: i32, double_value: f64) -> String {
    use std::f64;

    if double_value == 0.0 {
        return "0".to_string();
    }

    if (double_value - (int_value as f64)).abs() <= f64::EPSILON
        && (double_value as i64) <= i32::MAX as i64
        && (double_value as i64) >= i32::MIN as i64
    {
        return int_value.to_string();
    }

    let abs = double_value.abs();
    if (double_value.floor() - double_value).abs() <= f64::EPSILON && abs < 1.0e60 {
        format!("{:.0}", double_value)
    } else if abs < 1.0e-6 || abs > 1.0e9 {
        format!("{e}", double_value = double_value)
    } else {
        format!("{f}", double_value = double_value)
    }
}

fn escape_string_to(buf: &mut String, s: &str) {
    ensure_capacity(buf, s.len() + 2);
    buf.push('"');
    let mut needs_escaping = false;
    for &b in s.as_bytes() {
        if (b > 0 && b < 32) || b == b'"' || b == b'\\' {
            needs_escaping = true;
            break;
        }
    }
    if !needs_escaping {
        buf.push_str(s);
        buf.push('"');
        return;
    }

    for ch in s.chars() {
        match ch {
            '"' => buf.push_str("\\\""),
            '\\' => buf.push_str("\\\\"),
            '\u{0008}' => buf.push_str("\\b"),
            '\u{000C}' => buf.push_str("\\f"),
            '\n' => buf.push_str("\\n"),
            '\r' => buf.push_str("\\r"),
            '\t' => buf.push_str("\\t"),
            c if c < ' ' => {
                let code = c as u32;
                buf.push_str(&format!("\\u{code:04x}"));
            }
            c => buf.push(c),
        }
    }
    buf.push('"');
}

// ===== Printing main API =====

/// Pretty-print a JSON value.
pub fn print_json_pretty(value: &JsonValue) -> String {
    let mut out = String::new();
    print_value_to(&mut out, value, 0, true);
    out
}

/// Print a JSON value without extra whitespace.
pub fn print_json_compact(value: &JsonValue) -> String {
    let mut out = String::new();
    print_value_to(&mut out, value, 0, false);
    out
}

/// Print a JSON value into a buffer with an initial capacity hint.
pub fn print_json_buffered(value: &JsonValue, prebuffer: usize, pretty: bool) -> String {
    let mut out = String::with_capacity(prebuffer);
    print_value_to(&mut out, value, 0, pretty);
    out
}

fn print_value_to(buf: &mut String, value: &JsonValue, depth: usize, pretty: bool) {
    match value {
        JsonValue::Null => buf.push_str("null"),
        JsonValue::Bool(false) => buf.push_str("false"),
        JsonValue::Bool(true) => buf.push_str("true"),
        JsonValue::Number {
            int_value,
            double_value,
        } => {
            let s = format_number_pair(*int_value, *double_value);
            buf.push_str(&s);
        }
        JsonValue::String(s) => escape_string_to(buf, s),
        JsonValue::Array(arr) => print_array_to(buf, arr, depth, pretty),
        JsonValue::Object(obj) => print_object_to(buf, obj, depth, pretty),
    }
}

fn print_array_to(buf: &mut String, arr: &[JsonValue], depth: usize, pretty: bool) {
    buf.push('[');
    if arr.is_empty() {
        buf.push(']');
        return;
    }

    if pretty {
        buf.push('\n');
    }

    for (i, v) in arr.iter().enumerate() {
        if pretty {
            for _ in 0..(depth + 1) {
                buf.push('\t');
            }
        }
        print_value_to(buf, v, depth + 1, pretty);
        if i + 1 != arr.len() {
            buf.push(',');
        }
        if pretty {
            buf.push('\n');
        }
    }

    if pretty {
        for _ in 0..depth {
            buf.push('\t');
        }
    }
    buf.push(']');
}

fn print_object_to(
    buf: &mut String,
    obj: &[(String, JsonValue)],
    depth: usize,
    pretty: bool,
) {
    buf.push('{');
    if obj.is_empty() {
        if pretty {
            buf.push('\n');
            for _ in 0..depth {
                buf.push('\t');
            }
        }
        buf.push('}');
        return;
    }

    if pretty {
        buf.push('\n');
    }

    for (i, (name, value)) in obj.iter().enumerate() {
        if pretty {
            for _ in 0..(depth + 1) {
                buf.push('\t');
            }
        }
        escape_string_to(buf, name);
        buf.push(':');
        if pretty {
            buf.push('\t');
        }
        print_value_to(buf, value, depth + 1, pretty);
        if i + 1 != obj.len() {
            buf.push(',');
        }
        if pretty {
            buf.push('\n');
        }
    }

    if pretty {
        for _ in 0..depth {
            buf.push('\t');
        }
    }
    buf.push('}');
}

// ===== Array/object utilities (rough analogs of cJSON helpers) =====

/// Get array size.
pub fn json_array_size(array: &JsonValue) -> Option<usize> {
    match array {
        JsonValue::Array(v) => Some(v.len()),
        _ => None,
    }
}

/// Get array item by index.
pub fn json_array_get(array: &JsonValue, index: usize) -> Option<&JsonValue> {
    match array {
        JsonValue::Array(v) => v.get(index),
        _ => None,
    }
}

/// Get mutable array item by index.
pub fn json_array_get_mut(array: &mut JsonValue, index: usize) -> Option<&mut JsonValue> {
    match array {
        JsonValue::Array(v) => v.get_mut(index),
        _ => None,
    }
}

/// Get object item by key (case-sensitive).
pub fn json_object_get<'a>(object: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    match object {
        JsonValue::Object(fields) => fields
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value),
        _ => None,
    }
}

/// Get mutable object item by key (case-sensitive).
pub fn json_object_get_mut<'a>(
    object: &'a mut JsonValue,
    key: &str,
) -> Option<&'a mut JsonValue> {
    match object {
        JsonValue::Object(fields) => fields
            .iter_mut()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value),
        _ => None,
    }
}

/// Get object item by key (case-insensitive ASCII, like cJSON_GetObjectItem).
pub fn json_object_get_nocase<'a>(object: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    match object {
        JsonValue::Object(fields) => fields
            .iter()
            .find(|(name, _)| case_insensitive_eq(name, key))
            .map(|(_, value)| value),
        _ => None,
    }
}

/// Add a value to an array.
pub fn json_array_add(array: &mut JsonValue, item: JsonValue) -> Result<(), ()> {
    match array {
        JsonValue::Array(v) => {
            v.push(item);
            Ok(())
        }
        _ => Err(()),
    }
}

/// Add a named field to an object, replacing existing key if present.
pub fn json_object_set(object: &mut JsonValue, key: String, value: JsonValue) -> Result<(), ()> {
    match object {
        JsonValue::Object(fields) => {
            match fields
                .iter()
                .position(|(name, _)| name.as_str() == key.as_str())
            {
                Some(pos) => fields[pos] = (key, value),
                None => fields.push((key, value)),
            }
            Ok(())
        }
        _ => Err(()),
    }
}

/// Remove an array element, returning it.
pub fn json_array_detach(array: &mut JsonValue, index: usize) -> Option<JsonValue> {
    match array {
        JsonValue::Array(v) => {
            if index < v.len() {
                Some(v.remove(index))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Remove an object field by key, returning it if found.
pub fn json_object_detach(object: &mut JsonValue, key: &str) -> Option<JsonValue> {
    match object {
        JsonValue::Object(fields) => {
            if let Some(pos) = fields.iter().position(|(name, _)| name == key) {
                Some(fields.remove(pos).1)
            } else {
                None
            }
        }
        _ => None,
    }
}

// ===== Constructors (analogs to cJSON_Create* family) =====

pub fn json_null() -> JsonValue {
    JsonValue::Null
}

pub fn json_true() -> JsonValue {
    JsonValue::Bool(true)
}

pub fn json_false() -> JsonValue {
    JsonValue::Bool(false)
}

pub fn json_bool(b: bool) -> JsonValue {
    JsonValue::Bool(b)
}

pub fn json_number(num: f64) -> JsonValue {
    JsonValue::Number {
        int_value: num as i32,
        double_value: num,
    }
}

pub fn json_string<S: Into<String>>(s: S) -> JsonValue {
    JsonValue::String(s.into())
}

pub fn json_array() -> JsonValue {
    JsonValue::Array(Vec::new())
}

pub fn json_object() -> JsonValue {
    JsonValue::Object(Vec::new())
}

pub fn json_int_array(numbers: &[i32]) -> JsonValue {
    JsonValue::Array(numbers.iter().map(|&n| json_number(n as f64)).collect())
}

pub fn json_float_array(numbers: &[f32]) -> JsonValue {
    JsonValue::Array(numbers.iter().map(|&n| json_number(n as f64)).collect())
}

pub fn json_double_array(numbers: &[f64]) -> JsonValue {
    JsonValue::Array(numbers.iter().copied().map(json_number).collect())
}

pub fn json_string_array(strings: &[&str]) -> JsonValue {
    JsonValue::Array(strings.iter().map(|&s| json_string(s)).collect())
}

/// Deep clone if `recurse` is true; for this safe Rust version, shallow vs deep
/// are equivalent because `JsonValue` is already an owned tree. The flag is
/// accepted only for API parity.
pub fn json_duplicate(value: &JsonValue, _recurse: bool) -> JsonValue {
    value.clone()
}

/// Minify a JSON-like string in-place: remove whitespace and C-style comments,
/// preserving content inside string literals.
pub fn json_minify(s: &mut String) {
    let bytes = s.as_bytes().to_vec();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'\t' | b'\r' | b'\n' => {
                i += 1;
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'/' => {
                // line comment
                i += 2;
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'*' => {
                // block comment
                i += 2;
                while i + 1 < bytes.len()
                    && !(bytes[i] == b'*' && bytes[i + 1] == b'/')
                {
                    i += 1;
                }
                if i + 1 < bytes.len() {
                    i += 2;
                }
            }
            b'"' => {
                // string literal
                out.push(b'"');
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' {
                        out.push(bytes[i]);
                        i += 1;
                        if i >= bytes.len() {
                            break;
                        }
                    }
                    out.push(bytes[i]);
                    i += 1;
                }
                if i < bytes.len() {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }

    *s = String::from_utf8(out).unwrap_or_default();
}

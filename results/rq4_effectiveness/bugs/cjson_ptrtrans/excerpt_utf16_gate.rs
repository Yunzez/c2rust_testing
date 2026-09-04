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

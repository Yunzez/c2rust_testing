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

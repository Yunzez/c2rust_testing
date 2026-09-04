// SACTOR unidiomatic translation of `lodepng_error_text` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:11:09; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_lodepng/lodepng.c: Error: Failed to link project-level harness fo
pub unsafe fn lodepng_error_text(code: libc::c_uint) -> *const libc::c_char {
    match code {
        0 => b"no error, everything went ok\0".as_ptr() as *const libc::c_char,
        1 => b"nothing done yet\0".as_ptr() as *const libc::c_char,
        10 => {
            b"end of input memory reached without huffman end code\0".as_ptr()
                as *const libc::c_char
        }
        11 => {
            b"error in code tree made it jump outside of huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        13 => {
            b"problem while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        14 => {
            b"problem while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        15 => {
            b"problem while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        16 => {
            b"invalid code while processing dynamic deflate block\0".as_ptr()
                as *const libc::c_char
        }
        17 => {
            b"end of out buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        18 => b"invalid distance code while inflating\0".as_ptr() as *const libc::c_char,
        19 => {
            b"end of out buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        20 => {
            b"invalid deflate block BTYPE encountered while decoding\0".as_ptr()
                as *const libc::c_char
        }
        21 => {
            b"NLEN is not ones complement of LEN in a deflate block\0".as_ptr()
                as *const libc::c_char
        }
        22 => {
            b"end of out buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        23 => {
            b"end of in buffer memory reached while inflating\0".as_ptr()
                as *const libc::c_char
        }
        24 => b"invalid FCHECK in zlib header\0".as_ptr() as *const libc::c_char,
        25 => {
            b"invalid compression method in zlib header\0".as_ptr()
                as *const libc::c_char
        }
        26 => {
            b"FDICT encountered in zlib header while it's not used for PNG\0".as_ptr()
                as *const libc::c_char
        }
        27 => b"PNG file is smaller than a PNG header\0".as_ptr() as *const libc::c_char,
        28 => {
            b"incorrect PNG signature, it's no PNG or corrupted\0".as_ptr()
                as *const libc::c_char
        }
        29 => b"first chunk is not the header chunk\0".as_ptr() as *const libc::c_char,
        30 => {
            b"chunk length too large, chunk broken off at end of file\0".as_ptr()
                as *const libc::c_char
        }
        31 => b"illegal PNG color type or bpp\0".as_ptr() as *const libc::c_char,
        32 => b"illegal PNG compression method\0".as_ptr() as *const libc::c_char,
        33 => b"illegal PNG filter method\0".as_ptr() as *const libc::c_char,
        34 => b"illegal PNG interlace method\0".as_ptr() as *const libc::c_char,
        35 => {
            b"chunk length of a chunk is too large or the chunk too small\0".as_ptr()
                as *const libc::c_char
        }
        36 => b"illegal PNG filter type encountered\0".as_ptr() as *const libc::c_char,
        37 => {
            b"illegal bit depth for this color type given\0".as_ptr()
                as *const libc::c_char
        }
        38 => b"the palette is too small or too big\0".as_ptr() as *const libc::c_char,
        39 => {
            b"tRNS chunk before PLTE or has more entries than palette size\0".as_ptr()
                as *const libc::c_char
        }
        40 => {
            b"tRNS chunk has wrong size for grayscale image\0".as_ptr()
                as *const libc::c_char
        }
        41 => {
            b"tRNS chunk has wrong size for RGB image\0".as_ptr() as *const libc::c_char
        }
        42 => {
            b"tRNS chunk appeared while it was not allowed for this color type\0"
                .as_ptr() as *const libc::c_char
        }
        43 => {
            b"bKGD chunk has wrong size for palette image\0".as_ptr()
                as *const libc::c_char
        }
        44 => {
            b"bKGD chunk has wrong size for grayscale image\0".as_ptr()
                as *const libc::c_char
        }
        45 => {
            b"bKGD chunk has wrong size for RGB image\0".as_ptr() as *const libc::c_char
        }
        48 => {
            b"empty input buffer given to decoder. Maybe caused by non-existing file?\0"
                .as_ptr() as *const libc::c_char
        }
        49 => {
            b"jumped past memory while generating dynamic huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        50 => {
            b"jumped past memory while generating dynamic huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        51 => {
            b"jumped past memory while inflating huffman block\0".as_ptr()
                as *const libc::c_char
        }
        52 => b"jumped past memory while inflating\0".as_ptr() as *const libc::c_char,
        53 => b"size of zlib data too small\0".as_ptr() as *const libc::c_char,
        54 => {
            b"repeat symbol in tree while there was no value symbol yet\0".as_ptr()
                as *const libc::c_char
        }
        55 => {
            b"jumped past tree while generating huffman tree\0".as_ptr()
                as *const libc::c_char
        }
        56 => {
            b"given output image colortype or bitdepth not supported for color conversion\0"
                .as_ptr() as *const libc::c_char
        }
        57 => {
            b"invalid CRC encountered (checking CRC can be disabled)\0".as_ptr()
                as *const libc::c_char
        }
        58 => {
            b"invalid ADLER32 encountered (checking ADLER32 can be disabled)\0".as_ptr()
                as *const libc::c_char
        }
        59 => {
            b"requested color conversion not supported\0".as_ptr() as *const libc::c_char
        }
        60 => {
            b"invalid window size given in the settings of the encoder (must be 0-32768)\0"
                .as_ptr() as *const libc::c_char
        }
        61 => {
            b"invalid BTYPE given in the settings of the encoder (only 0, 1 and 2 are allowed)\0"
                .as_ptr() as *const libc::c_char
        }
        62 => {
            b"conversion from color to grayscale not supported\0".as_ptr()
                as *const libc::c_char
        }
        63 => {
            b"length of a chunk too long, max allowed for PNG is 2147483647 bytes per chunk\0"
                .as_ptr() as *const libc::c_char
        }
        64 => {
            b"the length of the END symbol 256 in the Huffman tree is 0\0".as_ptr()
                as *const libc::c_char
        }
        66 => {
            b"the length of a text chunk keyword given to the encoder is longer than the maximum of 79 bytes\0"
                .as_ptr() as *const libc::c_char
        }
        67 => {
            b"the length of a text chunk keyword given to the encoder is smaller than the minimum of 1 byte\0"
                .as_ptr() as *const libc::c_char
        }
        68 => {
            b"tried to encode a PLTE chunk with a palette that has less than 1 or more than 256 colors\0"
                .as_ptr() as *const libc::c_char
        }
        69 => {
            b"unknown chunk type with 'critical' flag encountered by the decoder\0"
                .as_ptr() as *const libc::c_char
        }
        71 => {
            b"invalid interlace mode given to encoder (must be 0 or 1)\0".as_ptr()
                as *const libc::c_char
        }
        72 => {
            b"while decoding, invalid compression method encountering in zTXt or iTXt chunk (it must be 0)\0"
                .as_ptr() as *const libc::c_char
        }
        73 => b"invalid tIME chunk size\0".as_ptr() as *const libc::c_char,
        74 => b"invalid pHYs chunk size\0".as_ptr() as *const libc::c_char,
        75 => {
            b"no null termination char found while decoding text chunk\0".as_ptr()
                as *const libc::c_char
        }
        76 => {
            b"iTXt chunk too short to contain required bytes\0".as_ptr()
                as *const libc::c_char
        }
        77 => b"integer overflow in buffer size\0".as_ptr() as *const libc::c_char,
        78 => b"failed to open file for reading\0".as_ptr() as *const libc::c_char,
        79 => b"failed to open file for writing\0".as_ptr() as *const libc::c_char,
        80 => b"tried creating a tree of 0 symbols\0".as_ptr() as *const libc::c_char,
        81 => b"lazy matching at pos 0 is impossible\0".as_ptr() as *const libc::c_char,
        82 => {
            b"color conversion to palette requested while a color isn't in palette, or index out of bounds\0"
                .as_ptr() as *const libc::c_char
        }
        83 => b"memory allocation failed\0".as_ptr() as *const libc::c_char,
        84 => {
            b"given image too small to contain all pixels to be encoded\0".as_ptr()
                as *const libc::c_char
        }
        86 => {
            b"impossible offset in lz77 encoding (internal bug)\0".as_ptr()
                as *const libc::c_char
        }
        87 => {
            b"must provide custom zlib function pointer if LODEPNG_COMPILE_ZLIB is not defined\0"
                .as_ptr() as *const libc::c_char
        }
        88 => {
            b"invalid filter strategy given for LodePNGEncoderSettings.filter_strategy\0"
                .as_ptr() as *const libc::c_char
        }
        89 => {
            b"text chunk keyword too short or long: must have size 1-79\0".as_ptr()
                as *const libc::c_char
        }
        90 => b"windowsize must be a power of two\0".as_ptr() as *const libc::c_char,
        91 => b"invalid decompressed idat size\0".as_ptr() as *const libc::c_char,
        92 => {
            b"integer overflow due to too many pixels\0".as_ptr() as *const libc::c_char
        }
        93 => b"zero width or height is invalid\0".as_ptr() as *const libc::c_char,
        94 => {
            b"header chunk must have a size of 13 bytes\0".as_ptr()
                as *const libc::c_char
        }
        95 => {
            b"integer overflow with combined idat chunk size\0".as_ptr()
                as *const libc::c_char
        }
        96 => b"invalid gAMA chunk size\0".as_ptr() as *const libc::c_char,
        97 => b"invalid cHRM chunk size\0".as_ptr() as *const libc::c_char,
        98 => b"invalid sRGB chunk size\0".as_ptr() as *const libc::c_char,
        99 => b"invalid sRGB rendering intent\0".as_ptr() as *const libc::c_char,
        100 => {
            b"invalid ICC profile color type, the PNG specification only allows RGB or GRAY\0"
                .as_ptr() as *const libc::c_char
        }
        101 => {
            b"PNG specification does not allow RGB ICC profile on gray color types and vice versa\0"
                .as_ptr() as *const libc::c_char
        }
        102 => {
            b"not allowed to set grayscale ICC profile with colored pixels by PNG specification\0"
                .as_ptr() as *const libc::c_char
        }
        103 => {
            b"invalid palette index in bKGD chunk. Maybe it came before PLTE chunk?\0"
                .as_ptr() as *const libc::c_char
        }
        104 => {
            b"invalid bKGD color while encoding (e.g. palette index out of range)\0"
                .as_ptr() as *const libc::c_char
        }
        105 => b"integer overflow of bitsize\0".as_ptr() as *const libc::c_char,
        106 => {
            b"PNG file must have PLTE chunk if color type is palette\0".as_ptr()
                as *const libc::c_char
        }
        107 => {
            b"color convert from palette mode requested without setting the palette data in it\0"
                .as_ptr() as *const libc::c_char
        }
        108 => {
            b"tried to add more than 256 values to a palette\0".as_ptr()
                as *const libc::c_char
        }
        109 => {
            b"tried to decompress zlib or deflate data larger than desired max_output_size\0"
                .as_ptr() as *const libc::c_char
        }
        110 => {
            b"custom zlib or inflate decompression failed\0".as_ptr()
                as *const libc::c_char
        }
        111 => {
            b"custom zlib or deflate compression failed\0".as_ptr()
                as *const libc::c_char
        }
        112 => b"compressed text unreasonably large\0".as_ptr() as *const libc::c_char,
        113 => b"ICC profile unreasonably large\0".as_ptr() as *const libc::c_char,
        114 => {
            b"sBIT chunk has wrong size for the color type of the image\0".as_ptr()
                as *const libc::c_char
        }
        115 => b"sBIT value out of range\0".as_ptr() as *const libc::c_char,
        _ => b"unknown error code\0".as_ptr() as *const libc::c_char,
    }
}

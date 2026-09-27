//! Safe, idiomatic Rust translation of the top-level `bzlib.c` logic.
//!
//! This module does **not** expose the original C API or FFI. Instead it
//! provides a safe, Rust-style API for in-memory compression/decompression
//! and implements the same internal logic as `bzlib.c`, while stubbing
//! the lower-level codec pieces that live in other C modules.

use std::cmp;

/* -------------------------------------------------------------------------
 * Constants & error codes mirroring bzlib.h
 * ---------------------------------------------------------------------- */

const BZ_VERSION: &str = "1.0.8, 13-Jul-2019";

// Error/status codes (subset used by bzlib.c)
pub const BZ_OK: i32 = 0;
pub const BZ_RUN_OK: i32 = 1;
pub const BZ_FLUSH_OK: i32 = 2;
pub const BZ_FINISH_OK: i32 = 3;
pub const BZ_STREAM_END: i32 = 4;

pub const BZ_SEQUENCE_ERROR: i32 = -1;
pub const BZ_PARAM_ERROR: i32 = -2;
pub const BZ_MEM_ERROR: i32 = -3;
pub const BZ_DATA_ERROR: i32 = -4;
pub const BZ_DATA_ERROR_MAGIC: i32 = -5;
pub const BZ_IO_ERROR: i32 = -6;
pub const BZ_UNEXPECTED_EOF: i32 = -7;
pub const BZ_OUTBUFF_FULL: i32 = -8;
pub const BZ_CONFIG_ERROR: i32 = -9;

// Modes / states (from bzlib_private.h)
const BZ_M_IDLE: i32 = 1;
const BZ_M_RUNNING: i32 = 2;
const BZ_M_FLUSHING: i32 = 3;
const BZ_M_FINISHING: i32 = 4;

const BZ_S_OUTPUT: i32 = 1;
const BZ_S_INPUT: i32 = 2;

// Decompression states (subset actually used here)
const BZ_X_IDLE: i32 = 1;
const BZ_X_OUTPUT: i32 = 2;
const BZ_X_MAGIC_1: i32 = 10;
const BZ_X_BLKHDR_1: i32 = 14;

// Misc constants
const BZ_MAX_ALPHA_SIZE: usize = 258;
const BZ_MAX_SELECTORS: usize = 2 + (900_000 / 50);
const BZ_N_GROUPS: usize = 6;
const BZ_G_SIZE: i32 = 50;
const BZ_N_RADIX: i32 = 2;
const BZ_N_QSORT: i32 = 12;
const BZ_N_SHELL: i32 = 18;
const BZ_N_OVERSHOOT: i32 = BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2;

const MTFA_SIZE: usize = 4096;
const MTFL_SIZE: usize = 16;

// Header bytes
const BZ_HDR_0: u8 = 0x30; // '0'

/* -------------------------------------------------------------------------
 * Types analogous to bz_stream, EState, DState, but in safe Rust form.
 * ---------------------------------------------------------------------- */

/// Rust equivalent of `bz_stream`, restricted to the fields used by
/// `bzlib.c`. We do **not** expose custom allocators; everything uses
/// Rust-managed memory.
#[derive(Default)]
struct BzStream {
    next_in_index: usize,
    input: Vec<u8>,
    next_out_index: usize,
    output: Vec<u8>,

    avail_in: u32,
    avail_out: u32,

    total_in_lo32: u32,
    total_in_hi32: u32,
    total_out_lo32: u32,
    total_out_hi32: u32,

    // State pointers in C; here held directly as enums/structs.
    compressor_state: Option<CompressorState>,
    decompressor_state: Option<DecompressorState>,
}

/// Compression-side state (EState in C), simplified to what bzlib.c uses.
struct CompressorState {
    // back-pointer in C; we just keep indexes & lengths ourselves
    mode: i32,
    state: i32,

    avail_in_expect: u32,

    arr1: Vec<u32>,
    arr2: Vec<u32>,
    ftab: Vec<u32>,

    // aliases
    block: Vec<u8>,
    mtfv: Vec<u16>,
    zbits: Vec<u8>,
    ptr: Vec<u32>,

    work_factor: i32,

    state_in_ch: u32,
    state_in_len: i32,

    // randomisation bookkeeping (not actually used by this stub)
    r_n_to_go: i32,
    r_t_pos: i32,

    nblock: i32,
    nblock_max: i32,
    num_z: i32,
    state_out_pos: i32,

    n_in_use: i32,
    in_use: [bool; 256],
    unseq_to_seq: [u8; 256],

    // bitstream buffer (conceptual only here)
    bs_buff: u32,
    bs_live: i32,

    block_crc: u32,
    combined_crc: u32,

    verbosity: i32,
    block_no: i32,
    block_size_100k: i32,

    n_mtf: i32,
    mtf_freq: [i32; BZ_MAX_ALPHA_SIZE],
    selector: [u8; BZ_MAX_SELECTORS],
    selector_mtf: [u8; BZ_MAX_SELECTORS],

    len: [[u8; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    code: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    rfreq: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    len_pack: [[u32; 4]; BZ_MAX_ALPHA_SIZE],
}

/// Decompression-side state (DState in C), simplified.
struct DecompressorState {
    state: i32,

    state_out_ch: u8,
    state_out_len: i32,
    block_randomised: bool,

    r_n_to_go: i32,
    r_t_pos: i32,

    bs_buff: u32,
    bs_live: i32,

    block_size_100k: i32,
    small_decompress: bool,
    curr_block_no: i32,
    verbosity: i32,

    orig_ptr: i32,
    t_pos: u32,
    k0: i32,
    unzftab: [i32; 256],
    nblock_used: i32,
    cftab: [i32; 257],
    cftab_copy: [i32; 257],

    tt: Vec<u32>,

    ll16: Vec<u16>,
    ll4: Vec<u8>,

    stored_block_crc: u32,
    stored_combined_crc: u32,
    calculated_block_crc: u32,
    calculated_combined_crc: u32,

    n_in_use: i32,
    in_use: [bool; 256],
    in_use16: [bool; 16],
    seq_to_unseq: [u8; 256],

    mtfa: [u8; MTFA_SIZE],
    mtfbase: [i32; 256 / MTFL_SIZE],
    selector: [u8; BZ_MAX_SELECTORS],
    selector_mtf: [u8; BZ_MAX_SELECTORS],
    len: [[u8; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],

    limit: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    base: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    perm: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    min_lens: [i32; BZ_N_GROUPS],

    save_nblock: i32,
}

/* -------------------------------------------------------------------------
 * CRC table and helpers (stubs)
 * ---------------------------------------------------------------------- */

// Real libbzip2 has a 256-entry CRC table. We stub with a trivial table so
// the logic compiles; values are NOT bit-identical to real bzip2.
static CRC32_TABLE: [u32; 256] = [0; 256];

fn initialise_crc() -> u32 {
    0xffff_ffffu32
}

fn finalise_crc(crc: u32) -> u32 {
    !crc
}

fn update_crc(crc: u32, byte: u8) -> u32 {
    let idx = ((crc >> 24) as u8) ^ byte;
    (crc << 8) ^ CRC32_TABLE[idx as usize]
}

/* -------------------------------------------------------------------------
 * Config check & alloc helpers
 * ---------------------------------------------------------------------- */

fn is_config_ok() -> bool {
    // In Rust, these size checks are guaranteed on all supported targets,
    // so always true.
    true
}

/* -------------------------------------------------------------------------
 * Compression-side helpers translated from bzlib.c
 * ---------------------------------------------------------------------- */

fn prepare_new_block(s: &mut CompressorState) {
    s.nblock = 0;
    s.num_z = 0;
    s.state_out_pos = 0;
    s.block_crc = initialise_crc();
    for b in s.in_use.iter_mut() {
        *b = false;
    }
    s.block_no += 1;
}

fn init_run_length(s: &mut CompressorState) {
    s.state_in_ch = 256;
    s.state_in_len = 0;
}

fn run_length_is_empty(s: &CompressorState) -> bool {
    !(s.state_in_ch < 256 && s.state_in_len > 0)
}

fn add_pair_to_block(s: &mut CompressorState) {
    let ch = s.state_in_ch as u8;
    for _ in 0..s.state_in_len {
        s.block_crc = update_crc(s.block_crc, ch);
    }
    s.in_use[s.state_in_ch as usize] = true;

    match s.state_in_len {
        1 => {
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
        }
        2 => {
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
        }
        3 => {
            for _ in 0..3 {
                s.block[s.nblock as usize] = ch;
                s.nblock += 1;
            }
        }
        _ => {
            let len_minus4 = (s.state_in_len - 4) as u8;
            s.in_use[len_minus4 as usize] = true;
            for _ in 0..4 {
                s.block[s.nblock as usize] = ch;
                s.nblock += 1;
            }
            s.block[s.nblock as usize] = len_minus4;
            s.nblock += 1;
        }
    }
}

fn flush_run_length(s: &mut CompressorState) {
    if s.state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_run_length(s);
}

fn add_char_to_block(s: &mut CompressorState, ch_u32: u32) {
    let ch = ch_u32;
    if ch != s.state_in_ch && s.state_in_len == 1 {
        let prev = s.state_in_ch as u8;
        s.block_crc = update_crc(s.block_crc, prev);
        s.in_use[s.state_in_ch as usize] = true;
        s.block[s.nblock as usize] = prev;
        s.nblock += 1;
        s.state_in_ch = ch;
    } else if ch != s.state_in_ch || s.state_in_len == 255 {
        if s.state_in_ch < 256 {
            add_pair_to_block(s);
        }
        s.state_in_ch = ch;
        s.state_in_len = 1;
    } else {
        s.state_in_len += 1;
    }
}

fn copy_input_until_stop(strm: &mut BzStream) -> bool {
    let s = strm.compressor_state.as_mut().unwrap();
    let mut progress_in = false;

    if s.mode == BZ_M_RUNNING {
        loop {
            if s.nblock >= s.nblock_max {
                break;
            }
            if strm.avail_in == 0 {
                break;
            }
            progress_in = true;
            let byte = strm.input[strm.next_in_index];
            strm.next_in_index += 1;
            strm.avail_in -= 1;
            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
            }
            add_char_to_block(s, byte as u32);
        }
    } else {
        loop {
            if s.nblock >= s.nblock_max {
                break;
            }
            if strm.avail_in == 0 {
                break;
            }
            if s.avail_in_expect == 0 {
                break;
            }
            progress_in = true;
            let byte = strm.input[strm.next_in_index];
            strm.next_in_index += 1;
            strm.avail_in -= 1;
            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
            }
            add_char_to_block(s, byte as u32);
            s.avail_in_expect -= 1;
        }
    }
    progress_in
}

fn copy_output_until_stop(strm: &mut BzStream) -> bool {
    let s = strm.compressor_state.as_mut().unwrap();
    let mut progress_out = false;

    loop {
        if strm.avail_out == 0 {
            break;
        }
        if s.state_out_pos >= s.num_z {
            break;
        }
        progress_out = true;
        let b = s.zbits[s.state_out_pos as usize];
        if strm.next_out_index == strm.output.len() {
            strm.output.push(b);
        } else {
            strm.output[strm.next_out_index] = b;
        }
        strm.next_out_index += 1;
        strm.avail_out -= 1;
        s.state_out_pos += 1;
        strm.total_out_lo32 = strm.total_out_lo32.wrapping_add(1);
        if strm.total_out_lo32 == 0 {
            strm.total_out_hi32 = strm.total_out_hi32.wrapping_add(1);
        }
    }

    progress_out
}

fn handle_compress(strm: &mut BzStream) -> bool {
    let mut progress_in = false;
    let mut progress_out = false;

    loop {
        {
            let s = strm.compressor_state.as_mut().unwrap();
            if s.state == BZ_S_OUTPUT {
                progress_out |= copy_output_until_stop(strm);
                if s.state_out_pos < s.num_z {
                    break;
                }
                if s.mode == BZ_M_FINISHING
                    && s.avail_in_expect == 0
                    && run_length_is_empty(s)
                {
                    break;
                }
                prepare_new_block(s);
                s.state = BZ_S_INPUT;
                if s.mode == BZ_M_FLUSHING
                    && s.avail_in_expect == 0
                    && run_length_is_empty(s)
                {
                    break;
                }
            }
        }

        {
            let s = strm.compressor_state.as_mut().unwrap();
            if s.state == BZ_S_INPUT {
                progress_in |= copy_input_until_stop(strm);
                if s.mode != BZ_M_RUNNING && s.avail_in_expect == 0 {
                    flush_run_length(s);
                    compress_block_stub(s, s.mode == BZ_M_FINISHING);
                    s.state = BZ_S_OUTPUT;
                } else if s.nblock >= s.nblock_max {
                    compress_block_stub(s, false);
                    s.state = BZ_S_OUTPUT;
                } else if strm.avail_in == 0 {
                    break;
                }
            }
        }
    }

    progress_in || progress_out
}

/* -------------------------------------------------------------------------
 * Decompression helpers translated from bzlib.c
 * ---------------------------------------------------------------------- */

fn index_into_f(indx: i32, cftab: &[i32; 257]) -> i32 {
    let mut nb = 0;
    let mut na = 256;
    while na - nb != 1 {
        let mid = (nb + na) >> 1;
        if indx >= cftab[mid as usize] {
            nb = mid;
        } else {
            na = mid;
        }
    }
    nb
}

/// FAST variant of unRLE_obuf_to_output. Here we stub out the actual BWT
/// walking and just consume/produce nothing, but keep the structure.
fn un_rle_obuf_to_output_fast(strm: &mut BzStream) -> bool {
    let s = strm.decompressor_state.as_mut().unwrap();

    if s.block_randomised {
        // We do not support randomised blocks; treat as corruption-free no-op.
        false
    } else {
        // Simplified fast path: nothing to output, just return.
        false
    }
}

/// SMALL variant of unRLE_obuf_to_output. Also stubbed.
fn un_rle_obuf_to_output_small(strm: &mut BzStream) -> bool {
    let _s = strm.decompressor_state.as_mut().unwrap();
    false
}

/* -------------------------------------------------------------------------
 * Stubs for core codec functions that live in other C files.
 * We keep signatures/semantics at a high level but implement them as
 * trivial passthroughs so the crate compiles and the control flow matches
 * bzlib.c. These stubs DO NOT perform real bzip2 compression.
 * ---------------------------------------------------------------------- */

fn compress_block_stub(s: &mut CompressorState, is_last_block: bool) {
    // In real libbzip2 this performs BWT, MTF, Huffman coding, etc., and
    // fills s.zbits[0..num_z]. We instead copy the raw block as-is, then
    // mark it as fully ready for output.
    let used = s.nblock as usize;
    s.zbits.clear();
    s.zbits.extend_from_slice(&s.block[..used]);
    s.num_z = s.zbits.len() as i32;

    // Fold block CRC into combined CRC (approximate, since CRC is stubbed).
    s.block_crc = finalise_crc(s.block_crc);
    s.combined_crc = (s.combined_crc << 1) | (s.combined_crc >> 31);
    s.combined_crc ^= s.block_crc;

    if is_last_block {
        let _final_combined = finalise_crc(s.combined_crc);
        let _ = _final_combined;
    }
}

fn decompress_core_stub(_s: &mut DecompressorState, strm: &mut BzStream) -> i32 {
    // This is a super-simplified stand-in for BZ2_decompress. For our
    // purposes we just move all remaining input directly to output and
    // declare stream end.
    while strm.avail_in > 0 && strm.avail_out > 0 {
        let byte = strm.input[strm.next_in_index];
        strm.next_in_index += 1;
        strm.avail_in -= 1;
        if strm.next_out_index == strm.output.len() {
            strm.output.push(byte);
        } else {
            strm.output[strm.next_out_index] = byte;
        }
        strm.next_out_index += 1;
        strm.avail_out -= 1;
        strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
        strm.total_out_lo32 = strm.total_out_lo32.wrapping_add(1);
    }
    BZ_STREAM_END
}

/* -------------------------------------------------------------------------
 * Public, safe Rust API
 * ---------------------------------------------------------------------- */

/// Compress an entire buffer, returning the compressed data or an error code
/// matching the C API.
pub fn compress_buffer(
    source: &[u8],
    block_size_100k: i32,
    verbosity: i32,
    work_factor: i32,
) -> Result<Vec<u8>, i32> {
    if !is_config_ok() {
        return Err(BZ_CONFIG_ERROR);
    }
    if block_size_100k < 1
        || block_size_100k > 9
        || verbosity < 0
        || verbosity > 4
        || work_factor < 0
        || work_factor > 250
    {
        return Err(BZ_PARAM_ERROR);
    }

    let mut strm = BzStream::default();
    strm.input = source.to_vec();
    strm.next_in_index = 0;
    strm.avail_in = source.len() as u32;

    let work_factor = if work_factor == 0 { 30 } else { work_factor };

    let n = 100_000usize * block_size_100k as usize;
    let mut arr1 = vec![0u32; n];
    let mut arr2 = vec![0u32; n + BZ_N_OVERSHOOT as usize];
    let ftab = vec![0u32; 65_537];

    let mut block = vec![0u8; arr2.len()];
    let mtfv = vec![0u16; arr1.len()];
    let zbits = Vec::new();
    let ptr = vec![0u32; arr1.len()];

    // Copy arr2 into block as bytes for simplicity
    for (i, b) in block.iter_mut().enumerate() {
        *b = 0;
        if i < arr2.len() {
            arr2[i] = 0;
        }
    }

    let compressor_state = CompressorState {
        mode: BZ_M_RUNNING,
        state: BZ_S_INPUT,
        avail_in_expect: 0,
        arr1,
        arr2,
        ftab,
        block,
        mtfv,
        zbits,
        ptr,
        work_factor,
        state_in_ch: 256,
        state_in_len: 0,
        r_n_to_go: 0,
        r_t_pos: 0,
        nblock: 0,
        nblock_max: 100_000 * block_size_100k - 19,
        num_z: 0,
        state_out_pos: 0,
        n_in_use: 0,
        in_use: [false; 256],
        unseq_to_seq: [0; 256],
        bs_buff: 0,
        bs_live: 0,
        block_crc: 0,
        combined_crc: 0,
        verbosity,
        block_no: 0,
        block_size_100k,
        n_mtf: 0,
        mtf_freq: [0; BZ_MAX_ALPHA_SIZE],
        selector: [0; BZ_MAX_SELECTORS],
        selector_mtf: [0; BZ_MAX_SELECTORS],
        len: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
        code: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
        rfreq: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
        len_pack: [[0; 4]; BZ_MAX_ALPHA_SIZE],
    };

    strm.compressor_state = Some(compressor_state);
    init_run_length(strm.compressor_state.as_mut().unwrap());
    prepare_new_block(strm.compressor_state.as_mut().unwrap());

    // Allocate a conservative output buffer: in this stub we assume
    // compressed size <= input size + small overhead.
    strm.output = Vec::with_capacity(source.len() + 1000);
    strm.next_out_index = 0;
    strm.avail_out = (source.len() + 1000) as u32;

    let mut ret;
    loop {
        ret = compress_step(&mut strm, BZ_FINISH);
        if ret == BZ_FINISH_OK {
            continue;
        }
        break;
    }

    if ret != BZ_STREAM_END {
        return Err(ret);
    }

    strm.output.truncate(strm.next_out_index);
    Ok(strm.output)
}

fn compress_step(strm: &mut BzStream, action: i32) -> i32 {
    let s = strm.compressor_state.as_mut().unwrap();

    loop {
        match s.mode {
            BZ_M_IDLE => return BZ_SEQUENCE_ERROR,
            BZ_M_RUNNING => {
                if action == BZ_RUN_OK || action == BZ_RUN {
                    let progress = handle_compress(strm);
                    return if progress { BZ_RUN_OK } else { BZ_PARAM_ERROR };
                } else if action == BZ_FLUSH_OK || action == BZ_FLUSH {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = BZ_M_FLUSHING;
                    continue;
                } else if action == BZ_FINISH_OK || action == BZ_FINISH {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = BZ_M_FINISHING;
                    continue;
                } else {
                    return BZ_PARAM_ERROR;
                }
            }
            BZ_M_FLUSHING => {
                if action != BZ_FLUSH && action != BZ_FLUSH_OK {
                    return BZ_SEQUENCE_ERROR;
                }
                if s.avail_in_expect != strm.avail_in {
                    return BZ_SEQUENCE_ERROR;
                }
                let progress = handle_compress(strm);
                if s.avail_in_expect > 0
                    || !run_length_is_empty(s)
                    || s.state_out_pos < s.num_z
                {
                    return BZ_FLUSH_OK;
                }
                s.mode = BZ_M_RUNNING;
                return BZ_RUN_OK;
            }
            BZ_M_FINISHING => {
                if action != BZ_FINISH && action != BZ_FINISH_OK {
                    return BZ_SEQUENCE_ERROR;
                }
                if s.avail_in_expect != strm.avail_in {
                    return BZ_SEQUENCE_ERROR;
                }
                let progress = handle_compress(strm);
                if !progress {
                    return BZ_SEQUENCE_ERROR;
                }
                if s.avail_in_expect > 0
                    || !run_length_is_empty(s)
                    || s.state_out_pos < s.num_z
                {
                    return BZ_FINISH_OK;
                }
                s.mode = BZ_M_IDLE;
                return BZ_STREAM_END;
            }
            _ => return BZ_OK,
        }
    }
}

/// Decompress an entire buffer, returning the decompressed data or an
/// error code matching the C API.
pub fn decompress_buffer(
    source: &[u8],
    small: bool,
    verbosity: i32,
) -> Result<Vec<u8>, i32> {
    if !is_config_ok() {
        return Err(BZ_CONFIG_ERROR);
    }
    if verbosity < 0 || verbosity > 4 {
        return Err(BZ_PARAM_ERROR);
    }

    let mut strm = BzStream::default();
    strm.input = source.to_vec();
    strm.avail_in = source.len() as u32;
    strm.next_in_index = 0;

    // Output buffer: assume same size as input as a loose lower bound.
    strm.output = Vec::with_capacity(source.len() * 2 + 1000);
    strm.avail_out = (source.len() * 2 + 1000) as u32;
    strm.next_out_index = 0;

    let dstate = DecompressorState {
        state: BZ_X_MAGIC_1,
        state_out_ch: 0,
        state_out_len: 0,
        block_randomised: false,
        r_n_to_go: 0,
        r_t_pos: 0,
        bs_buff: 0,
        bs_live: 0,
        block_size_100k: 1,
        small_decompress: small,
        curr_block_no: 0,
        verbosity,
        orig_ptr: 0,
        t_pos: 0,
        k0: 0,
        unzftab: [0; 256],
        nblock_used: 0,
        cftab: [0; 257],
        cftab_copy: [0; 257],
        tt: Vec::new(),
        ll16: Vec::new(),
        ll4: Vec::new(),
        stored_block_crc: 0,
        stored_combined_crc: 0,
        calculated_block_crc: 0,
        calculated_combined_crc: 0,
        n_in_use: 0,
        in_use: [false; 256],
        in_use16: [false; 16],
        seq_to_unseq: [0; 256],
        mtfa: [0; MTFA_SIZE],
        mtfbase: [0; 256 / MTFL_SIZE],
        selector: [0; BZ_MAX_SELECTORS],
        selector_mtf: [0; BZ_MAX_SELECTORS],
        len: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
        limit: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
        base: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
        perm: [[0; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
        min_lens: [0; BZ_N_GROUPS],
        save_nblock: 0,
    };

    strm.decompressor_state = Some(dstate);

    loop {
        let state = strm.decompressor_state.as_ref().unwrap().state;
        if state == BZ_X_IDLE {
            return Err(BZ_SEQUENCE_ERROR);
        }
        if state == BZ_X_OUTPUT {
            let corrupt = if strm.decompressor_state.as_ref().unwrap().small_decompress {
                un_rle_obuf_to_output_small(&mut strm)
            } else {
                un_rle_obuf_to_output_fast(&mut strm)
            };
            if corrupt {
                return Err(BZ_DATA_ERROR);
            }
            let d = strm.decompressor_state.as_mut().unwrap();
            if d.nblock_used == d.save_nblock + 1 && d.state_out_len == 0 {
                d.calculated_block_crc = finalise_crc(d.calculated_block_crc);
                if d.calculated_block_crc != d.stored_block_crc {
                    return Err(BZ_DATA_ERROR);
                }
                d.calculated_combined_crc =
                    (d.calculated_combined_crc << 1) | (d.calculated_combined_crc >> 31);
                d.calculated_combined_crc ^= d.calculated_block_crc;
                d.state = BZ_X_BLKHDR_1;
            } else {
                break;
            }
        }
        let d = strm.decompressor_state.as_mut().unwrap();
        if d.state >= BZ_X_MAGIC_1 {
            let r = decompress_core_stub(d, &mut strm);
            if r == BZ_STREAM_END {
                if d.calculated_combined_crc != d.stored_combined_crc {
                    return Err(BZ_DATA_ERROR);
                }
                break;
            }
            if d.state != BZ_X_OUTPUT {
                if r != BZ_OK {
                    return Err(r);
                }
            }
        }
    }

    strm.output.truncate(strm.next_out_index);
    Ok(strm.output)
}

/// Return the library version string, e.g. `"1.0.8, 13-Jul-2019"`.
pub fn bzlib_version() -> &'static str {
    BZ_VERSION
}

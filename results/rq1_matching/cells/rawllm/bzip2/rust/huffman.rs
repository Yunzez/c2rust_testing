//! Safe, idiomatic Rust translation of `huffman.c` from bzip2.
//!
//! This crate exposes Huffman helper routines roughly corresponding
//! to the original C functions, but with safe, slice-based APIs and
//! idiomatic names.

/// Maximum alphabet size used by the original implementation.
///
/// This is exposed mainly to mirror the C code; the functions below
/// validate their slice sizes at runtime.
pub const MAX_ALPHA_SIZE: usize = 258; // BZ_MAX_ALPHA_SIZE

/// Maximum Huffman code length supported by the original implementation.
pub const MAX_CODE_LEN: usize = 23; // BZ_MAX_CODE_LEN

/// Error type for Huffman helper routines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HuffmanError {
    /// Provided slice lengths are inconsistent with `alpha_size` or limits.
    InvalidInput,
}

impl core::fmt::Display for HuffmanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HuffmanError::InvalidInput => write!(f, "invalid input to Huffman helper"),
        }
    }
}

impl std::error::Error for HuffmanError {}

/// Compute Huffman code lengths given symbol frequencies.
///
/// * `lengths` – output buffer for code lengths; only the first
///   `alpha_size` entries are modified.
/// * `freq` – input frequencies, length must be at least `alpha_size`.
/// * `alpha_size` – number of symbols in the alphabet (must be ≤ `MAX_ALPHA_SIZE`).
/// * `max_len` – maximum allowed code length (as in the original code).
pub fn make_code_lengths(
    lengths: &mut [u8],
    freq: &[i32],
    alpha_size: usize,
    max_len: i32,
) -> Result<(), HuffmanError> {
    if alpha_size == 0
        || alpha_size > MAX_ALPHA_SIZE
        || lengths.len() < alpha_size
        || freq.len() < alpha_size
        || max_len <= 0
        || max_len as usize > MAX_CODE_LEN
    {
        return Err(HuffmanError::InvalidInput);
    }

    // Local aliases and helpers mirroring the C macros.
    #[inline]
    fn weight_of(x: i32) -> i32 {
        x & 0xffffff00
    }
    #[inline]
    fn depth_of(x: i32) -> i32 {
        x & 0x000000ff
    }
    #[inline]
    fn my_max(a: i32, b: i32) -> i32 {
        if a > b { a } else { b }
    }
    #[inline]
    fn add_weights(w1: i32, w2: i32) -> i32 {
        (weight_of(w1) + weight_of(w2)) | (1 + my_max(depth_of(w1), depth_of(w2)))
    }

    // The arrays in the C code are stack-allocated with fixed sizes.
    // We mirror that using fixed-size arrays and just use prefixes.
    let mut heap: [i32; MAX_ALPHA_SIZE + 2] = [0; MAX_ALPHA_SIZE + 2];
    let mut weight: [i32; MAX_ALPHA_SIZE * 2] = [0; MAX_ALPHA_SIZE * 2];
    let mut parent: [i32; MAX_ALPHA_SIZE * 2] = [0; MAX_ALPHA_SIZE * 2];

    // UPHEAP and DOWNHEAP as local closures operating on our arrays.
    let mut upheap = |z: i32,
                      heap: &mut [i32; MAX_ALPHA_SIZE + 2],
                      weight: &mut [i32; MAX_ALPHA_SIZE * 2]| {
        let mut zz = z as usize;
        let tmp = heap[zz];
        while weight[tmp as usize] < weight[heap[zz >> 1] as usize] {
            heap[zz] = heap[zz >> 1];
            zz >>= 1;
        }
        heap[zz] = tmp;
    };

    let mut downheap = |z: i32,
                        n_heap: &mut i32,
                        heap: &mut [i32; MAX_ALPHA_SIZE + 2],
                        weight: &mut [i32; MAX_ALPHA_SIZE * 2]| {
        let mut zz = z as usize;
        let tmp = heap[zz];
        loop {
            let mut yy = zz << 1;
            if yy > *n_heap as usize {
                break;
            }
            if yy < *n_heap as usize
                && weight[heap[yy + 1] as usize] < weight[heap[yy] as usize]
            {
                yy += 1;
            }
            if weight[tmp as usize] < weight[heap[yy] as usize] {
                break;
            }
            heap[zz] = heap[yy];
            zz = yy;
        }
        heap[zz] = tmp;
    };

    // Initial weights from frequencies.
    for i in 0..alpha_size {
        let f = freq[i];
        let base = if f == 0 { 1 } else { f };
        // Shift by 8 bits as in the original C code.
        weight[i + 1] = base << 8;
    }

    loop {
        let mut n_nodes: i32 = alpha_size as i32;
        let mut n_heap: i32 = 0;

        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;

        for i in 1..=alpha_size {
            parent[i] = -1;
            n_heap += 1;
            heap[n_heap as usize] = i as i32;
            upheap(n_heap, &mut heap, &mut weight);
        }

        // Rough bound check like the original AssertH.
        if n_heap >= (MAX_ALPHA_SIZE + 2) as i32 {
            return Err(HuffmanError::InvalidInput);
        }

        while n_heap > 1 {
            let n1 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            downheap(1, &mut n_heap, &mut heap, &mut weight);

            let n2 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            downheap(1, &mut n_heap, &mut heap, &mut weight);

            n_nodes += 1;
            let nn = n_nodes as usize;
            parent[n1 as usize] = n_nodes;
            parent[n2 as usize] = n_nodes;
            weight[nn] = add_weights(weight[n1 as usize], weight[n2 as usize]);
            parent[nn] = -1;
            n_heap += 1;
            heap[n_heap as usize] = n_nodes;
            upheap(n_heap, &mut heap, &mut weight);
        }

        if n_nodes >= (MAX_ALPHA_SIZE * 2) as i32 {
            return Err(HuffmanError::InvalidInput);
        }

        let mut too_long = false;
        for i in 1..=alpha_size {
            let mut j = 0i32;
            let mut k = i as i32;
            while parent[k as usize] >= 0 {
                k = parent[k as usize];
                j += 1;
            }
            lengths[i - 1] = j as u8;
            if j > max_len {
                too_long = true;
            }
        }

        if !too_long {
            break;
        }

        // Scale down the leaf weights and retry.
        for i in 1..=alpha_size {
            let mut j = weight[i] >> 8;
            j = 1 + (j / 2);
            weight[i] = j << 8;
        }
    }

    Ok(())
}

/// Assign canonical Huffman codes given code lengths.
///
/// * `code` – output codes; length must be at least `alpha_size`.
/// * `length` – code lengths; length must be at least `alpha_size`.
/// * `min_len`, `max_len` – minimum and maximum code length.
pub fn assign_codes(
    code: &mut [i32],
    length: &[u8],
    min_len: i32,
    max_len: i32,
    alpha_size: usize,
) -> Result<(), HuffmanError> {
    if alpha_size == 0
        || alpha_size > MAX_ALPHA_SIZE
        || code.len() < alpha_size
        || length.len() < alpha_size
        || min_len < 0
        || max_len < min_len
        || max_len as usize > MAX_CODE_LEN
    {
        return Err(HuffmanError::InvalidInput);
    }

    let mut vec: i32 = 0;
    for n in min_len..=max_len {
        for i in 0..alpha_size {
            if length[i] as i32 == n {
                code[i] = vec;
                vec += 1;
            }
        }
        vec <<= 1;
    }

    Ok(())
}

/// Create decoding tables for the fast decoder.
///
/// * `limit`, `base`, `perm` – output arrays; must have length at least
///   `MAX_CODE_LEN` for `limit` and `base`, and at least `alpha_size` for `perm`.
/// * `length` – code lengths per symbol; length must be at least `alpha_size`.
/// * `min_len`, `max_len` – minimum and maximum code length in use.
/// * `alpha_size` – number of symbols in the alphabet.
pub fn create_decode_tables(
    limit: &mut [i32],
    base: &mut [i32],
    perm: &mut [i32],
    length: &[u8],
    min_len: i32,
    max_len: i32,
    alpha_size: usize,
) -> Result<(), HuffmanError> {
    if alpha_size == 0
        || alpha_size > MAX_ALPHA_SIZE
        || perm.len() < alpha_size
        || length.len() < alpha_size
        || limit.len() < MAX_CODE_LEN
        || base.len() < MAX_CODE_LEN
        || min_len < 0
        || max_len < min_len
        || max_len as usize > MAX_CODE_LEN
    {
        return Err(HuffmanError::InvalidInput);
    }

    let min_len_u = min_len as usize;
    let max_len_u = max_len as usize;

    // Build `perm` – mapping from code index to symbol index.
    let mut pp: usize = 0;
    for i in min_len_u..=max_len_u {
        for j in 0..alpha_size {
            if length[j] as usize == i {
                perm[pp] = j as i32;
                pp += 1;
            }
        }
    }

    // base[i] counts how many codes of length i.
    for b in base.iter_mut().take(MAX_CODE_LEN) {
        *b = 0;
    }
    for i in 0..alpha_size {
        let l = length[i] as usize;
        if l + 1 < MAX_CODE_LEN {
            base[l + 1] += 1;
        }
    }

    // Turn counts into cumulative counts.
    for i in 1..MAX_CODE_LEN {
        base[i] += base[i - 1];
    }

    // Clear limits.
    for l in limit.iter_mut().take(MAX_CODE_LEN) {
        *l = 0;
    }

    let mut vec: i32 = 0;
    for i in min_len_u..=max_len_u {
        vec += base[i + 1] - base[i];
        limit[i] = vec - 1;
        vec <<= 1;
    }

    for i in (min_len_u + 1)..=max_len_u {
        base[i] = ((limit[i - 1] + 1) << 1) - base[i];
    }

    Ok(())
}

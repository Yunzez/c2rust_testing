



use std::slice;

extern "C" {
    
}
pub use crate::src::zlib::deflate::internal_state;
pub type Byte = crate::src::libpng::png::Byte;
pub type uInt = crate::src::libpng::png::uInt;
pub type uLong = crate::src::libpng::png::uLong;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type free_func = crate::src::libpng::png::free_func;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::pngerror::z_stream_s;
pub type z_stream = crate::src::libpng::png::z_stream;
pub type z_streamp = crate::src::libpng::png::z_streamp;
// #[derive(Copy, Clone)]

pub type gz_header_s = crate::src::zlib::infback::gz_header_s;
pub type gz_header = crate::src::zlib::deflate::gz_header;
pub type gz_headerp = crate::src::zlib::deflate::gz_headerp;
// #[derive(Copy, Clone)]

pub type code = crate::src::zlib::infback::code;
pub type inflate_mode = crate::src::zlib::infback::inflate_mode;
pub const SYNC: inflate_mode = 16211;
pub const MEM: inflate_mode = 16210;
pub const BAD: inflate_mode = 16209;
pub const DONE: inflate_mode = 16208;
pub const LENGTH: inflate_mode = 16207;
pub const CHECK: inflate_mode = 16206;
pub const LIT: inflate_mode = 16205;
pub const MATCH: inflate_mode = 16204;
pub const DISTEXT: inflate_mode = 16203;
pub const DIST: inflate_mode = 16202;
pub const LENEXT: inflate_mode = 16201;
pub const LEN: inflate_mode = 16200;
pub const LEN_: inflate_mode = 16199;
pub const CODELENS: inflate_mode = 16198;
pub const LENLENS: inflate_mode = 16197;
pub const TABLE: inflate_mode = 16196;
pub const COPY: inflate_mode = 16195;
pub const COPY_: inflate_mode = 16194;
pub const STORED: inflate_mode = 16193;
pub const TYPEDO: inflate_mode = 16192;
pub const TYPE: inflate_mode = 16191;
pub const DICT: inflate_mode = 16190;
pub const DICTID: inflate_mode = 16189;
pub const HCRC: inflate_mode = 16188;
pub const COMMENT: inflate_mode = 16187;
pub const NAME: inflate_mode = 16186;
pub const EXTRA: inflate_mode = 16185;
pub const EXLEN: inflate_mode = 16184;
pub const OS: inflate_mode = 16183;
pub const TIME: inflate_mode = 16182;
pub const FLAGS: inflate_mode = 16181;
pub const HEAD: inflate_mode = 16180;
// #[derive(Copy, Clone)]

pub type inflate_state = crate::src::zlib::infback::inflate_state;
/* inffast.c -- fast decoding
 * Copyright (C) 1995-2017 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
   Decode literal, length, and distance codes and write out the resulting
   literal and match bytes until either not enough input or output is
   available, an end-of-block is encountered, or a data error is encountered.
   When large enough input and output buffers are supplied to inflate(), for
   example, a 16K input buffer and a 64K output buffer, more than 95% of the
   inflate execution time is spent in this routine.

   Entry assumptions:

        state->mode == LEN
        strm->avail_in >= 6
        strm->avail_out >= 258
        start >= strm->avail_out
        state->bits < 8

   On return, state->mode is one of:

        LEN -- ran out of enough output space or enough available input
        TYPE -- reached end of block code, inflate() to interpret next block
        BAD -- error in block data

   Notes:

    - The maximum input bits used by a length/distance pair is 15 bits for the
      length code, 5 bits for the length extra, 15 bits for the distance code,
      and 13 bits for the distance extra.  This totals 48 bits, or six bytes.
      Therefore if strm->avail_in >= 6, then there is enough input to avoid
      checking for available input while decoding.

    - The maximum bytes that a single length/distance pair can output is 258
      bytes, which is the maximum length that can be coded.  inflate_fast()
      requires strm->avail_out >= 258 for each loop to avoid checking for
      output space.
 */

pub unsafe extern "C" fn inflate_fast(mut strm: z_streamp,
                                      mut start: std::os::raw::c_uint) 
 /* inflate()'s starting value for strm->avail_out */
 {
    let mut state: *mut inflate_state =
        0 as *mut inflate_state; /* local strm->next_in */
    let mut in_0: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* have enough input while in < last */
    let mut last: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* local strm->next_out */
    let mut out: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* inflate()'s initial strm->next_out */
    let mut beg: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* while out < end, enough space available */
    let mut end: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* window size or zero if not using window */
    let mut wsize: std::os::raw::c_uint = 0; /* valid bytes in the window */
    let mut whave: std::os::raw::c_uint = 0; /* window write index */
    let mut wnext: std::os::raw::c_uint =
        0; /* allocated sliding window, if wsize != 0 */
    let mut window: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* local strm->hold */
    let mut hold: std::os::raw::c_ulong = 0; /* local strm->bits */
    let mut bits: std::os::raw::c_uint = 0; /* local strm->lencode */
    let mut lcode: *const code = 0 as *const code; /* local strm->distcode */
    let mut dcode: *const code =
        0 as *const code; /* mask for first level of length codes */
    let mut lmask: std::os::raw::c_uint =
        0; /* mask for first level of distance codes */
    let mut dmask: std::os::raw::c_uint = 0; /* retrieved table entry */
    let mut here: code =
        code{op: 0,
             bits: 0,
             val:
                 0,}; /* code bits, operation, extra bits, or */
                                /*  window position, window bytes to copy */
    let mut op: std::os::raw::c_uint = 0; /* match length, unused bytes */
    let mut len: std::os::raw::c_uint = 0; /* match distance */
    let mut dist: std::os::raw::c_uint = 0; /* where to copy match from */
    let mut from: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    /* copy state to local variables */
    state = (*strm).state as *mut inflate_state;
    in_0 = (*strm).next_in as *mut u8;
    last =
        in_0.offset((*strm).avail_in.wrapping_sub(5 as std::os::raw::c_int as
                                                      std::os::raw::c_uint) as isize);
    out = (*strm).next_out;
    beg = out.offset(-(start.wrapping_sub((*strm).avail_out) as isize));
    end =
        out.offset((*strm).avail_out.wrapping_sub(257 as std::os::raw::c_int as
                                                      std::os::raw::c_uint) as isize);
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask =
        ((1 as std::os::raw::c_uint) <<
             (*state).lenbits).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
    dmask =
        ((1 as std::os::raw::c_uint) <<
             (*state).distbits).wrapping_sub(1 as std::os::raw::c_int as
                                                 std::os::raw::c_uint);
    let mut current_block_141: u64;
    's_132:
        loop 
             /* decode literals and length/distances until end-of-block or not enough
       input data or output space */
             {
            if bits < 15 {
    hold |= (*in_0 as u64) << bits;
    bits += 8;
    in_0 = in_0.add(1);
    hold |= (*in_0 as u64) << bits;
    bits += 8;
    in_0 = in_0.add(1);
}
here = unsafe { *lcode.add((hold & lmask as u64) as usize) };
loop {
    op = here.bits as u32;
    hold >>= op;
    bits = bits.wrapping_sub(op);
    op = here.op as u32;
    if op == 0 {
        /* literal */
        *out = here.val as u8;
        out = out.add(1);
        current_block_141 = 18386322304582297246;
        break;
    } else if op & 16 != 0 {
        /* length base */
        len = here.val as u32; /* number of extra bits */
        op &= 15;
        if op != 0 {
            if bits < op {
                hold |= (*in_0 as u64) << bits;
                bits += 8;
                in_0 = in_0.add(1);
            }
            len += (hold & ((1 << op) - 1)) as u32;
            hold >>= op;
            bits = bits.wrapping_sub(op);
        }
        if bits < 15 {
            hold |= (*in_0 as u64) << bits;
            bits += 8;
            in_0 = in_0.add(1);
            hold |= (*in_0 as u64) << bits;
            bits += 8;
            in_0 = in_0.add(1);
        }
        here = unsafe { *dcode.add((hold & dmask as u64) as usize) };
        current_block_141 = 6394145548040015962;
        break;
    } else if op & 64 == 0 {
        /* 2nd level length code */
        here = unsafe { *lcode.add((here.val as u32 as u64 + (hold & ((1 << op) - 1) as u64)) as usize) };
    } else if op & 32 != 0 {
        current_block_141 = 4976922244085895320;
        break;
    } else {
        current_block_141 = 5832582820025303349;
        break;
    }
}
match current_block_141 {
    6394145548040015962 => {
         loop {
    op = here.bits as u32;
    hold >>= op;
    bits = bits.wrapping_sub(op);
    op = here.op as u32;
    if op & 16 != 0 {
        // distance base
        dist = here.val as u32; // number of extra bits
        op &= 15; // max distance in output
        if bits < op {
            let fresh6 = in_0;
            in_0 = in_0.add(1);
            hold = hold.wrapping_add((fresh6 as u64) << bits);
            bits = bits.wrapping_add(8);
            if bits < op {
                let fresh7 = in_0;
                in_0 = in_0.add(1);
                hold = hold.wrapping_add((fresh7 as u64) << bits);
                bits = bits.wrapping_add(8);
            }
        }
        dist = dist.wrapping_add((hold as u32) & ((1u32 << op).wrapping_sub(1)));
        hold >>= op;
        bits = bits.wrapping_sub(op);
        op = out.offset_from(beg) as u32;
        if dist > op {
            current_block_141 = 5873035170358615968;
            break;
        } else {
            current_block_141 = 17239133558811367971;
            break;
        }
    } else if op & 64 == 0 {
        // 2nd level distance code
        here = *dcode.offset((here.val as usize).wrapping_add((hold & ((1u64 << op).wrapping_sub(1))) as usize) as isize);
    } else {
        (*strm).msg = b"invalid distance code\x00" as *const u8 as *const i8 as *mut i8; // copy direct from output
        (*state).mode = BAD;
        break 's_132;
    }
}
match current_block_141 {
    17239133558811367971 => {
         let from_slice = unsafe { std::slice::from_raw_parts(from, len as usize) };
let out_slice = unsafe { std::slice::from_raw_parts_mut(out, len as usize) };

let mut i = 0;
while i < (len / 3) as usize {
    out_slice[i * 3] = from_slice[i * 3];
    out_slice[i * 3 + 1] = from_slice[i * 3 + 1];
    out_slice[i * 3 + 2] = from_slice[i * 3 + 2];
    i += 1;
}

let remaining = (len % 3) as usize;
if remaining > 0 {
    out_slice[i * 3..i * 3 + remaining].copy_from_slice(&from_slice[i * 3..i * 3 + remaining]);
}


    }
    _ => {
        // see if copy from window
         let mut op = dist.wrapping_sub(op); // distance back in window
if op > whave {
    if unsafe { (*state).sane } != 0 {
        unsafe { (*strm).msg = "invalid distance too far back\0".as_ptr() as *const i8 };
        unsafe { (*state).mode = BAD };
        return; // break;
    }
}
let mut from = window;
if wnext == 0 {
    // very common case
    from = unsafe { from.add((wsize.wrapping_sub(op)) as usize) };
    if op < len {
        // some from window
        len = len.wrapping_sub(op);
        while op != 0 {
            let fresh8 = unsafe { *from };
            from = unsafe { from.add(1) };
            unsafe { *out = fresh8 };
            out = unsafe { out.add(1) };
            op = op.wrapping_sub(1);
        }
        from = unsafe { out.offset(-(dist as isize)) }; // rest from output
    }
} else if wnext < op {
    // wrap around window
    from = unsafe { from.add((wsize + wnext - op) as usize) };
    op = op.wrapping_sub(wnext);
    if op < len {
        // some from end of window
        len = len.wrapping_sub(op);
        while op != 0 {
            let fresh10 = unsafe { *from };
            from = unsafe { from.add(1) };
            unsafe { *out = fresh10 };
            out = unsafe { out.add(1) };
            op = op.wrapping_sub(1);
        }
        from = window;
        if wnext < len {
            // some from start of window
            op = wnext;
            len = len.wrapping_sub(op);
            while op != 0 {
                let fresh12 = unsafe { *from };
                from = unsafe { from.add(1) };
                unsafe { *out = fresh12 };
                out = unsafe { out.add(1) };
                op = op.wrapping_sub(1);
            }
            from = unsafe { out.offset(-(dist as isize)) }; // rest from output
        }
    }
} else {
    // contiguous in window
    from = unsafe { from.add((wnext.wrapping_sub(op)) as usize) };
    if op < len {
        // some from window
        len = len.wrapping_sub(op);
        while op != 0 {
            let fresh14 = unsafe { *from };
            from = unsafe { from.add(1) };
            unsafe { *out = fresh14 };
            out = unsafe { out.add(1) };
            op = op.wrapping_sub(1);
        }
        from = unsafe { out.offset(-(dist as isize)) }; // rest from output
    }
}
while len > 2 {
    let fresh16 = unsafe { *from };
    from = unsafe { from.add(1) };
    unsafe { *out = fresh16 };
    out = unsafe { out.add(1) };
    let fresh18 = unsafe { *from };
    from = unsafe { from.add(1) };
    unsafe { *out = fresh18 };
    out = unsafe { out.add(1) };
    let fresh20 = unsafe { *from };
    from = unsafe { from.add(1) };
    unsafe { *out = fresh20 };
    out = unsafe { out.add(1) };
    len = len.wrapping_sub(3);
}
if len != 0 {
    let fresh22 = unsafe { *from };
    from = unsafe { from.add(1) };
    unsafe { *out = fresh22 };
    if len > 1 {
        let fresh24 = unsafe { *from };
        from = unsafe { from.add(1) };
        unsafe { *out = fresh24 };
    }
}


    }
}
/*
The variables live at this point are:
(mut strm: *mut src::libpng::png::z_stream_s, mut start: u32, mut state: *mut src::zlib::infback::inflate_state, mut in_0: *mut u8, mut out: *mut u8, mut beg: *mut u8, mut end: *mut u8, mut wsize: u32, mut whave: u32, mut wnext: u32, mut window: *mut u8, mut hold: u64, mut bits: u32, mut dcode: *const src::zlib::infback::code, mut here: src::zlib::infback::code, mut op: u32, mut len: u32, mut dist: u32, mut from: *mut u8, mut current_block_141: u64, mut op: u32, mut from: *mut u8)
*/

                
    }
    5832582820025303349 => {
        (*strm).msg = b"invalid literal/length code\x00" as *const u8 as *const i8 as *mut i8;
        (*state).mode = BAD;
        break;
    }
    4976922244085895320 => {
        /* end-of-block */
        (*state).mode = TYPE;
        break;
    }
    _ => {}
}
if !(in_0 < last && out < end) { break; }

        }
    /* return unused bytes (on entry, bits < 8, so in won't go too far back) */
    len = bits >> 3 as std::os::raw::c_int;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as std::os::raw::c_int);
    hold &=
        ((1 as std::os::raw::c_uint) <<
             bits).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as
            std::os::raw::c_ulong;
    /* update state and return */
    (*strm).next_in = in_0;
    (*strm).next_out = out;
    (*strm).avail_in =
        if in_0 < last {
            (5 as std::os::raw::c_int as std::os::raw::c_long) +
                last.offset_from(in_0) as std::os::raw::c_long
        } else {
            (5 as std::os::raw::c_int as std::os::raw::c_long) -
                in_0.offset_from(last) as std::os::raw::c_long
        } as std::os::raw::c_uint;
    (*strm).avail_out =
        if out < end {
            (257 as std::os::raw::c_int as std::os::raw::c_long) +
                end.offset_from(out) as std::os::raw::c_long
        } else {
            (257 as std::os::raw::c_int as std::os::raw::c_long) -
                out.offset_from(end) as std::os::raw::c_long
        } as std::os::raw::c_uint;
    (*state).hold = hold;
    (*state).bits = bits;
}
/* !ASMINF */
/*
   inflate_fast() speedups that turned out slower (on a PowerPC G3 750CXe):
   - Using bit fields for code structure
   - Different op definition to avoid & for extra bits (do & for table bits)
   - Three separate decoding do-loops for direct, window, and wnext == 0
   - Special case for distance > 1 copies to do overlapped load and store copy
   - Explicit branch predictions (based on measured branch probabilities)
   - Deferring match copy and interspersed it with decoding subsequent codes
   - Swapping literal/length else
   - Swapping window/direct else
   - Larger unrolled copy loops (three is about right)
   - Moving len -= 3 statement into middle of loop
 */

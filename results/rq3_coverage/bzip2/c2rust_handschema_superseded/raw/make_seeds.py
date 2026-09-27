#!/usr/bin/env python3
"""Encode real bzip2 payloads into the harness's byte-cursor input format.

A harness input is not a raw file: the generated target decodes its parameters from a byte cursor.
With `decode_scalars_first` the layout is

    [ scalar_0 ][ scalar_1 ] ... [ rest = the buffer ]

and a `bounded_scalar` is decoded as `min + take_i32().rem_euclid(max - min + 1)`, so to ask for a
particular value v the seed stores `v - min`.  Seeding therefore means writing the scalars we want
and appending the real payload.

Compress seeds are real files (the bzip2 distribution's own sample*.ref plus text/binary/run-length
shapes); decompress seeds are real .bz2 streams, which is the only way that harness gets past the
magic check.
"""
import pathlib, random, struct, sys

def i32(v):
    return struct.pack("<i", v)

def compress_seed(payload, block=9, verb=0, work=30):
    # blockSize100k in 1..9, verbosity 0..4, workFactor 0..250
    return i32(block - 1) + i32(verb - 0) + i32(work - 0) + payload

def decompress_seed(payload, small=0, verb=0):
    return i32(small - 0) + i32(verb - 0) + payload

def main(bzip2_dir, out_root):
    bz = pathlib.Path(bzip2_dir)
    out = pathlib.Path(out_root)
    rnd = random.Random(42)

    payloads = {}
    for n in (1, 2, 3):
        payloads[f"sample{n}"] = (bz / f"sample{n}.ref").read_bytes()
    payloads["words"] = b"".join((bz / f"words{i}").read_bytes() for i in range(4))
    payloads["source"] = (bz / "blocksort.c").read_bytes() + (bz / "bzlib.c").read_bytes()
    # shapes that drive distinct paths: long runs (RLE), all-same (randomisation), incompressible
    payloads["runs"] = b"".join(bytes([c]) * 300 for c in range(256))
    payloads["same"] = b"A" * 200000
    payloads["random"] = bytes(rnd.randrange(256) for _ in range(150000))
    payloads["empty"] = b""
    payloads["tiny"] = b"a"
    # >= 10000 bytes is what takes BZ2_blockSort out of the fallback sorter (blocksort.c:1044)
    payloads["just_over_block"] = bytes(rnd.randrange(256) for _ in range(12000))

    c = out / "BZ2_bzBuffToBuffCompress"; c.mkdir(parents=True, exist_ok=True)
    for name, p in payloads.items():
        for block in (1, 9):
            (c / f"{name}_b{block}").write_bytes(compress_seed(p, block=block))

    d = out / "BZ2_bzBuffToBuffDecompress"; d.mkdir(parents=True, exist_ok=True)
    import bz2
    for name, p in payloads.items():
        for small in (0, 1):
            (d / f"{name}_s{small}").write_bytes(decompress_seed(bz2.compress(p, 9), small=small))
    for n in (1, 2, 3):
        raw = (bz / f"sample{n}.bz2").read_bytes()
        for small in (0, 1):
            (d / f"shipped{n}_s{small}").write_bytes(decompress_seed(raw, small=small))
    # truncated / corrupted streams exercise the error paths
    good = bz2.compress(payloads["sample1"], 9)
    (d / "truncated").write_bytes(decompress_seed(good[: len(good) // 2]))
    (d / "flipped").write_bytes(decompress_seed(good[:20] + bytes([good[20] ^ 0xFF]) + good[21:]))

    print(f"compress seeds:   {len(list(c.iterdir()))}")
    print(f"decompress seeds: {len(list(d.iterdir()))}")

if __name__ == "__main__":
    main(*sys.argv[1:3])

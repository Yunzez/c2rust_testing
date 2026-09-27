#!/usr/bin/env python3
"""Pointer-contract templates for the differential harness generator.

Eligibility asks whether a boundary's parameters AND its return value match a SUPPORTED CONTRACT,
not merely whether a pointer appears. This module owns the two templates that need code generation
on both sides:

  interior_pointer   the return points inside a named input buffer; compared as nullness plus the
                     offset from that buffer's base, never as an address (emitted inline by
                     gen_diff_harness).

  structured_object  the return points at a declared object graph; compared through a CANONICAL
                     EXTRACTION produced independently on each side.

Why two extractors and never the library's own printer: a printer is part of the translated code,
so a defect the printer also mis-handles would be invisible. Our own S8 (cJSON x PtrTrans, the
success path assigns `valuestring = None`) is exactly that shape - a printer that skips null
strings would hide it. The extractors here read only the fields the schema declares.

Canonical encoding, byte-identical on both sides:

    N                          NULL pointer
    X                          cycle, depth limit or node limit reached
    ( <field>* [ <node>* ] )   a node, its declared fields, then its child list

    int      i<signed decimal>;
    double   d<16 hex digits of the IEEE bit pattern>;      no float formatting on either side
    cstring  sN;  when NULL, else  s<len>:<lowercase hex of the raw bytes>;

The child list is child, child->next, child->next->next, ... so sibling ORDER is recorded and a
long chain does not consume recursion depth. NULL is always distinguished from empty.
"""
from __future__ import annotations


def limits(rc: dict) -> tuple[int, int]:
    lim = rc.get("limits") or {}
    return int(lim.get("max_depth", 64)), int(lim.get("max_nodes", 4096))


def c_extractor_source(rc: dict) -> str:
    md, mn = limits(rc)
    ty, child, nxt, hdr = rc["type"], rc["child"], rc["next"], rc["header"]
    fields = []
    for f in rc["fields"]:
        n, k = f["name"], f["kind"]
        if k == "int":
            fields.append("    c2r_int(o, (long long)p->%s);" % n)
        elif k == "double":
            fields.append("    { double _d = p->%s; unsigned long long _u; "
                          "memcpy(&_u, &_d, 8); c2r_dbl(o, _u); }" % n)
        elif k == "cstring":
            fields.append("    c2r_str(o, p->%s);" % n)
        else:
            raise SystemExit("structured_object: unsupported field kind %r" % k)
    return _C_TEMPLATE % dict(hdr=hdr, ty=ty, child=child, nxt=nxt, md=md, mn=mn,
                              fields="\n".join(fields))


def rust_extractor_lines(rc: dict, crate: str) -> list[str]:
    md, mn = limits(rc)
    ty, child, nxt = rc["type"], rc["child"], rc["next"]
    body = []
    for f in rc["fields"]:
        n, k = f.get("rust_name", f["name"]), f["kind"]
        if k == "int":
            body.append("        c2r_r_int(o, (*p).%s as i64);" % n)
        elif k == "double":
            body.append("        c2r_r_dbl(o, ((*p).%s as f64).to_bits());" % n)
        elif k == "cstring":
            body.append("        c2r_r_str(o, (*p).%s as *const u8);" % n)
    return (_RUST_PRELUDE
            + ["unsafe fn c2r_r_node(o: &mut Vec<u8>, p: *const %s::%s, depth: u32," % (crate, ty),
               "                     seen: &mut Vec<usize>, nodes: &mut usize) {",
               "    if p.is_null() { o.push(b'N'); return; }",
               "    if depth > %d || *nodes >= %d || seen.contains(&(p as usize)) { o.push(b'X'); return; }" % (md, mn),
               "    if seen.len() < %d { seen.push(p as usize); }" % mn,
               "    *nodes += 1;",
               "    o.push(b'(');"]
            + body
            + ["    o.push(b'[');",
               "    let mut q = (*p).%s as *const %s::%s;" % (child, crate, ty),
               "    while !q.is_null() {",
               "        if *nodes >= %d { o.push(b'X'); break; }" % mn,
               "        c2r_r_node(o, q, depth + 1, seen, nodes);",
               "        q = (*q).%s as *const %s::%s;" % (nxt, crate, ty),
               "    }",
               "    o.push(b']');",
               "    o.push(b')');",
               "}",
               "unsafe fn c2r_r_extract(p: *const %s::%s) -> Vec<u8> {" % (crate, ty),
               "    let (mut o, mut seen, mut nodes) = (Vec::new(), Vec::new(), 0usize);",
               "    c2r_r_node(&mut o, p, 0, &mut seen, &mut nodes);",
               "    o",
               "}",
               ""])


_RUST_PRELUDE = [
    "// GENERATED canonical extractor for the Rust side; byte-identical encoding to the C side.",
    "// The library's own printer is deliberately not used on either side.",
    'fn c2r_r_int(o: &mut Vec<u8>, v: i64) { o.extend_from_slice(format!("i{v};").as_bytes()); }',
    'fn c2r_r_dbl(o: &mut Vec<u8>, u: u64) { o.extend_from_slice(format!("d{u:016x};").as_bytes()); }',
    "unsafe fn c2r_r_str(o: &mut Vec<u8>, s: *const u8) {",
    '    if s.is_null() { o.extend_from_slice(b"sN;"); return; }',
    "    let mut n = 0usize; while *s.add(n) != 0 { n += 1; }",
    '    o.extend_from_slice(format!("s{n}:").as_bytes());',
    '    for i in 0..n { o.extend_from_slice(format!("{:02x}", *s.add(i)).as_bytes()); }',
    "    o.push(b';');",
    "}",
]

_C_TEMPLATE = r'''/* GENERATED canonical extractor for the C side of a structured_object return contract.
   Reads only the fields the schema declares; never calls the library's own printer. */
#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include "%(hdr)s"

typedef struct { char *b; size_t n, cap; } c2r_buf;
static void c2r_put(c2r_buf *o, const char *s, size_t k) {
    if (o->n + k <= o->cap) memcpy(o->b + o->n, s, k);
    o->n += k;
}
static void c2r_c(c2r_buf *o, char c) { c2r_put(o, &c, 1); }
static void c2r_int(c2r_buf *o, long long v) {
    char t[32]; int k = snprintf(t, sizeof t, "i%%lld;", v); c2r_put(o, t, (size_t)k);
}
static void c2r_dbl(c2r_buf *o, unsigned long long u) {
    char t[32]; int k = snprintf(t, sizeof t, "d%%016llx;", u); c2r_put(o, t, (size_t)k);
}
static void c2r_str(c2r_buf *o, const char *s) {
    static const char H[] = "0123456789abcdef";
    if (!s) { c2r_put(o, "sN;", 3); return; }
    size_t n = strlen(s); char t[32];
    int k = snprintf(t, sizeof t, "s%%zu:", n); c2r_put(o, t, (size_t)k);
    for (size_t i = 0; i < n; i++) {
        char h[2]; h[0] = H[(unsigned char)s[i] >> 4]; h[1] = H[(unsigned char)s[i] & 15];
        c2r_put(o, h, 2);
    }
    c2r_c(o, ';');
}
static const void *c2r_seen[%(mn)d];
static size_t c2r_nseen, c2r_nodes;
static int c2r_mark(const void *p) {
    for (size_t i = 0; i < c2r_nseen; i++) if (c2r_seen[i] == p) return 0;
    if (c2r_nseen < %(mn)d) c2r_seen[c2r_nseen++] = p;
    return 1;
}
static void c2r_node(c2r_buf *o, const %(ty)s *p, int depth) {
    if (!p) { c2r_c(o, 'N'); return; }
    if (depth > %(md)d || c2r_nodes >= %(mn)d || !c2r_mark((const void *)p)) { c2r_c(o, 'X'); return; }
    c2r_nodes++;
    c2r_c(o, '(');
%(fields)s
    c2r_c(o, '[');
    {
        const %(ty)s *q;
        for (q = p->%(child)s; q; q = q->%(nxt)s) {
            if (c2r_nodes >= %(mn)d) { c2r_c(o, 'X'); break; }
            c2r_node(o, q, depth + 1);
        }
    }
    c2r_c(o, ']');
    c2r_c(o, ')');
}
size_t c2r_extract(const %(ty)s *p, char *out, size_t cap) {
    c2r_buf o; o.b = out; o.n = 0; o.cap = cap;
    c2r_nseen = 0; c2r_nodes = 0;
    c2r_node(&o, p, 0);
    return o.n;
}
'''

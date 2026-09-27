"""Partial Seed IR (docs/seeding_policy_plan.md): a best-effort, prefix-exact model of the byte layout the
generated harness decodes, lowered from an archived InputPlan entry (plans.json), used to ENCODE plan-guided
seeds. It never modifies the harness. Nodes:

    Scalar(name, rust, width, signed, float, bounds)   bounded: value = min + raw.rem_euclid(max-min+1)
    Repeat(name, elem, width, count)                   fixed count, or a count expression over decoded scalars
    Zero(name, why)                                    consumes no fuzz bytes (null pointers, zero-filled rows,
                                                       out params, lengths derived from a buffer)
    Opaque(name, why)                                  a decoder this IR does not model (strings, length-prefixed
                                                       buffers, structs, produced objects, pointer tables)

Order = the harness's actual decode order: `decode_scalars_first` (harness_plan lowers every plan with it) sorts
items by rank -- 0 scalars / structs / produced objects / null, 1 buffers and strings, 2 plan arrays and buffer
tables -- stably within rank in declaration order (gen_diff_harness.items_from_schema). A node is PLACEABLE iff
no Opaque precedes it; nothing after the first Opaque is targeted and no offset is ever guessed.
The round-trip test (--decode-dump) is what keeps this model honest against the real decoder.
"""
from __future__ import annotations
import re
import struct
from dataclasses import dataclass, field
from typing import Any

# rust type spelling (plan `rust_type`, suffix after ::) -> (width, signed, float)
_TY = {"c_char": (1, True, False), "c_schar": (1, True, False), "i8": (1, True, False), "c_uchar": (1, False, False), "u8": (1, False, False),
       "bool": (1, False, False), "c_short": (2, True, False), "i16": (2, True, False), "c_ushort": (2, False, False), "u16": (2, False, False),
       "c_int": (4, True, False), "i32": (4, True, False), "c_uint": (4, False, False), "u32": (4, False, False),
       "c_long": (8, True, False), "c_longlong": (8, True, False), "i64": (8, True, False), "isize": (8, True, False),
       "c_ulong": (8, False, False), "c_ulonglong": (8, False, False), "u64": (8, False, False), "usize": (8, False, False), "size_t": (8, False, False),
       "c_float": (4, False, True), "f32": (4, False, True), "c_double": (8, False, True), "f64": (8, False, True)}
_ELEM = {"i8": (1, True, False), "u8": (1, False, False), "i16": (2, True, False), "u16": (2, False, False), "i32": (4, True, False), "u32": (4, False, False),
         "i64": (8, True, False), "u64": (8, False, False), "usize": (8, False, False), "f32": (4, False, True), "f64": (8, False, True)}
_BUF_RANK1 = {"input_buffer", "inout_buffer", "output_buffer_cap", "input_string", "input_fixed_array_buffer",
              "input_rectangular_pointer_table", "input_string_pointer_table", "input_struct_array", "inout_struct_array"}
MAX_BUFFER_BYTES, UNPROVEN_EXTENT_ELEMS = 1 << 20, 4096   # harness_plan.GeneratorPolicy defaults


_ALIAS_RE = re.compile(r"^\s*pub\s+type\s+([A-Za-z_]\w*)\s*=\s*([^;]+);", re.M)


def load_aliases(pair_dir) -> dict[str, str]:
    """`pub type X = Y;` of the pair's translated .rs (c2rust keeps C typedefs as aliases: `Int32`,
    `uLong`, `png_uint_32`, `lilint_t`...). Last path segment -> last path segment of the target."""
    import pathlib as _pl
    out: dict[str, str] = {}
    for rs in sorted(_pl.Path(pair_dir).glob("translated/*.rs")):
        for m in _ALIAS_RE.finditer(rs.read_text(errors="replace")):
            name, target = m.group(1), m.group(2).strip().split("::")[-1].strip()
            if target and target != name:
                out.setdefault(name, target)
    return out


def ty_info(rust_type: str, aliases: dict[str, str] | None = None) -> tuple[int, bool, bool] | None:
    t = (rust_type or "").split("::")[-1].strip()
    seen = set()
    while t not in _TY and aliases and t in aliases and t not in seen:
        seen.add(t); t = aliases[t]
    return _TY.get(t)


@dataclass
class Scalar:
    name: str; rust: str; width: int; signed: bool; float: bool; bounds: tuple[int, int] | None = None
    def decode(self, raw: int | float):
        if self.bounds is None: return raw
        lo, hi = self.bounds; span = hi - lo + 1
        return lo + (int(raw) % span if span > 0 else 0)          # Rust rem_euclid on a signed type == python %
    def encode_value(self, v) -> bytes:
        if self.float: return struct.pack("<d" if self.width == 8 else "<f", float(v))
        if self.bounds is not None:
            lo, hi = self.bounds
            if not (lo <= int(v) <= hi): raise ValueError(f"{self.name}: {v} outside [{lo}, {hi}]")
            raw = int(v) - lo
        else: raw = int(v)
        return (raw & ((1 << (8 * self.width)) - 1)).to_bytes(self.width, "little")   # two's complement by width
    def raw_from_bytes(self, b: bytes):
        if self.float: return struct.unpack("<d" if self.width == 8 else "<f", b)[0]
        return int.from_bytes(b, "little", signed=self.signed)


@dataclass
class Count:
    kind: str; value: int = 0; param: str = ""; mul: int = 1; div: int = 1; add: int = 0; cap: int = 0; of: list = field(default_factory=list)
    def resolve(self, scalars: dict[str, Any]) -> int | None:
        if self.kind == "const": return self.value
        if self.kind == "param":
            if self.param not in scalars: return None
            e = int(scalars[self.param]) * self.mul
            if self.div != 1: e = int(e / self.div)
            e += self.add
            return max(0, min(e, self.cap))
        if self.kind == "max":
            vals = [c.resolve(scalars) for c in self.of]
            return None if any(v is None for v in vals) else max(vals)
        return None


@dataclass
class Repeat:
    name: str; elem: str; width: int; signed: bool; float: bool; count: Count
    def pack(self, v) -> bytes:
        if self.float: return struct.pack("<d" if self.width == 8 else "<f", float(v))
        return int(v).to_bytes(self.width, "little", signed=self.signed if int(v) < 0 or self.signed else False)


@dataclass
class Zero:
    name: str; why: str


@dataclass
class Opaque:
    name: str; why: str


def _count(extent: dict | None, alloc: int | None, elem_w: int) -> Count | None:
    cap = min(UNPROVEN_EXTENT_ELEMS, max(1, MAX_BUFFER_BYTES // max(1, elem_w)))
    if alloc is not None: return Count("const", int(alloc))
    if not extent or extent.get("k") == "unknown": return None
    if extent["k"] == "const": return Count("const", max(0, min(int(extent["v"]), cap)))
    if extent["k"] == "param": return Count("param", param=extent["p"], mul=extent.get("mul", 1), div=extent.get("div", 1), add=extent.get("add", 0), cap=cap)
    if extent["k"] == "max":
        parts = [_count(x, None, elem_w) for x in extent["of"]]
        return None if any(p is None for p in parts) else Count("max", of=parts)
    return None


def param_order(pair_dir, entry: str) -> list[str] | None:
    """C declaration order of the boundary's parameters, from the generator's own libclang parse
    (the harness decodes in that order within each rank; plans.json lists inputs in analysis order)."""
    try:
        import pathlib as _pl, sys as _sys
        _sys.path.insert(0, str(_pl.Path(__file__).resolve().parent))
        import gen_diff_harness as gdh
        params, _ret, _fns = gdh.parse_entry_signature(_pl.Path(pair_dir) / "build", entry, allow_nonpod=True)
        return [q["name"] for q in params]
    except BaseException:      # the parser raises SystemExit on shapes the schema path rejects
        return None


def lower(entry: dict, aliases: dict[str, str] | None = None, order: list[str] | None = None) -> list:
    """plans.json entry -> ordered node list (rank-sorted like the harness). `order` = the C parameter
    declaration order (param_order); without it the plan's input order is used, which is NOT always the
    declaration order (quadtree_insert lists x, y, key, tree) -- pass it whenever the pair is at hand."""
    if entry.get("status") != "planned": return [Opaque("*", "not planned")]
    ranked: list[tuple[int, Any]] = []
    scalar_names = set()
    inputs = list(entry["inputs"])
    if order:
        pos = {n: k for k, n in enumerate(order)}
        inputs.sort(key=lambda i: pos.get(i["param"], len(order)))
    for i in inputs:
        a, d, n = i["c_decoder"], i.get("detail", {}), i["param"]
        if a in ("scalar", "bounded_scalar"):
            t = ty_info(i.get("rust_type", ""), aliases)
            if t is None: ranked.append((0, Opaque(n, f"scalar of unknown width {i.get('rust_type')}"))); continue
            w, sg, fl = t
            bounds = None
            if a == "bounded_scalar" and d.get("max") is not None and not fl:
                bounds = (int(d.get("min") or 0), int(d["max"]))
            ranked.append((0, Scalar(n, i.get("rust_type", ""), w, sg, fl, bounds))); scalar_names.add(n)
        elif a == "null_pointer": ranked.append((0, Zero(n, "null pointer")))
        elif a in ("output_scalar", "out_scalar"): ranked.append((0, Zero(n, "out scalar, zero-initialised")))
        elif a in ("length", "capacity_ptr"): continue                       # consumed by their buffer
        elif a == "struct_value": ranked.append((0, Opaque(n, "struct value (POD fields decoded in place; not modelled)")))
        elif a == "produced_object": ranked.append((0, Opaque(n, "produced object (producer parameters decoded in place; not modelled)")))
        elif a in ("input_string", "input_string_pointer_table", "input_buffer", "inout_buffer"):
            ranked.append((1, Opaque(n, f"{a} (length-prefixed / rest-taking; not modelled)")))
        elif a == "output_buffer" and any(j["c_decoder"] == "capacity_ptr" and j["param"] == d.get("capacity_param") for j in entry["inputs"]):
            ranked.append((1, Zero(n, "output buffer with capacity pointer, zero-filled")))
        elif a in ("output_buffer", "output_array", "input_array", "inout_array"):
            ew = int(d.get("elem_width") or 1); el = d.get("elem", "u8"); es = _ELEM.get(el, (ew, False, False))
            if not d.get("fills_from_fuzz"): ranked.append((2, Zero(n, "plan array, zero-filled"))); continue
            if a == "output_buffer": c = _count({"k": "param", "p": d["capacity_param"], "mul": 1, "div": 1, "add": 0}, None, ew)
            else: c = _count(d.get("extent"), d.get("alloc_elems"), ew)
            if c is None: ranked.append((2, Opaque(n, "plan array with unknown extent"))); continue
            ranked.append((2, Repeat(n, el, es[0], es[1], es[2], c)))
        elif a == "buffer_table":
            ew = int(d.get("elem_width") or 1); el = d.get("elem", "u8"); es = _ELEM.get(el, (ew, False, False))
            for r in d.get("row_specs", []):
                rn = f"{n}[{r['row']}]"
                if not r.get("fills_from_fuzz"): ranked.append((2, Zero(rn, "table row, zero-filled"))); continue
                c = _count(r.get("extent"), r.get("alloc_elems"), ew)
                ranked.append((2, Repeat(rn, el, es[0], es[1], es[2], c) if c else Opaque(rn, "table row with unknown extent")))
        else:
            ranked.append((1, Opaque(n, f"decoder {a} not modelled")))
    ranked.sort(key=lambda t: t[0])                 # python sort is stable == the harness's
    nodes = [nd for _, nd in ranked]
    # a Repeat whose count refers to a param that is not a decoded scalar (e.g. a `length` of an opaque
    # buffer) is unresolvable -> opaque
    out = []
    for nd in nodes:
        if isinstance(nd, Repeat) and nd.count.kind != "const":
            refs = _refs(nd.count)
            if not refs <= scalar_names: nd = Opaque(nd.name, f"extent depends on {sorted(refs - scalar_names)} which is not a decoded scalar")
        out.append(nd)
    return out


def _refs(c: Count) -> set[str]:
    if c.kind == "param": return {c.param}
    if c.kind == "max": return set().union(*[_refs(x) for x in c.of])
    return set()


def placeable(nodes: list) -> list:
    """the prefix of nodes before the first Opaque"""
    out = []
    for nd in nodes:
        if isinstance(nd, Opaque): break
        out.append(nd)
    return out


def encode(nodes: list, assignments: dict[str, Any], rng, filler_after_opaque: int = 256) -> tuple[bytes, dict]:
    """assignments: {scalar_name: value} and {array_name: [values...]} (array values apply to the first
    len(values) elements; remaining elements are random filler). Returns (bytes, layout: name -> (offset, nbytes))."""
    buf = bytearray(); layout = {}; scalars: dict[str, Any] = {}
    for nd in nodes:
        if isinstance(nd, Scalar):
            if nd.name in assignments: b = nd.encode_value(assignments[nd.name])
            else: b = bytes(rng.getrandbits(8) for _ in range(nd.width))
            layout[nd.name] = (len(buf), nd.width); buf += b
            scalars[nd.name] = nd.decode(nd.raw_from_bytes(b))
        elif isinstance(nd, Repeat):
            n = nd.count.resolve(scalars)
            if n is None: raise ValueError(f"{nd.name}: unresolved count")
            vals = assignments.get(nd.name)
            start = len(buf)
            for k in range(n):
                if vals is not None and k < len(vals): buf += nd.pack(vals[k])
                else: buf += bytes(rng.getrandbits(8) for _ in range(nd.width))
            layout[nd.name] = (start, n * nd.width)
        elif isinstance(nd, Zero):
            continue
        elif isinstance(nd, Opaque):
            layout[nd.name] = (len(buf), filler_after_opaque)
            buf += bytes(rng.getrandbits(8) for _ in range(filler_after_opaque))
            break
    return bytes(buf), layout


def decode(nodes: list, data: bytes) -> dict[str, Any]:
    """reference decoder for the placeable prefix (what the harness should see); floats as raw values"""
    pos = 0; out: dict[str, Any] = {}; scalars: dict[str, Any] = {}
    def take(w):
        nonlocal pos
        b = data[pos:pos + w]; b = b + bytes(w - len(b)); pos += w; return b
    for nd in nodes:
        if isinstance(nd, Scalar):
            v = nd.decode(nd.raw_from_bytes(take(nd.width))); out[nd.name] = v; scalars[nd.name] = v
        elif isinstance(nd, Repeat):
            n = nd.count.resolve(scalars)
            if n is None: break
            vals = []
            for _ in range(n):
                b = take(nd.width)
                vals.append(struct.unpack("<d" if nd.width == 8 else "<f", b)[0] if nd.float else int.from_bytes(b, "little", signed=nd.signed))
            out[nd.name] = vals
        elif isinstance(nd, Opaque): break
    return out


def describe(nodes: list) -> list[dict]:
    d = []
    for nd in nodes:
        if isinstance(nd, Scalar): d.append({"node": "Scalar", "name": nd.name, "rust": nd.rust, "width": nd.width, "bounds": nd.bounds, "float": nd.float})
        elif isinstance(nd, Repeat): d.append({"node": "Repeat", "name": nd.name, "elem": nd.elem, "count": nd.count.__dict__ if nd.count.kind != "max" else {"kind": "max", "of": [c.__dict__ for c in nd.count.of]}})
        elif isinstance(nd, Zero): d.append({"node": "Zero", "name": nd.name, "why": nd.why})
        else: d.append({"node": "Opaque", "name": nd.name, "why": nd.why})
    return d

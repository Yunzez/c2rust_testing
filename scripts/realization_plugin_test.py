#!/usr/bin/env python3
"""Regression for the HarnessPlan resource-realization plugin path (docs/construction_recipe_plugin_plan.md).

Loader: the reference manifest loads; the deliberately invalid sample (a view outside the closed
vocabulary), a comparator manifest and an unfilled template are rejected BEFORE any planning.
Planner (lodepng x c2rust, lodepng x CROWN): without a plugin the boundary abstains with the
reworded reason; with the reference plugin it plans with an `origin` record; a manifest without an
initializer is `construction unsupported` (reverse acceptance); a binding whose views do not fit the
translation is rejected with the failing check.

  python3 scripts/realization_plugin_test.py
"""
import json, sys, tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
import realization_plugin as rp          # noqa: E402
import harness_plan as hp                # noqa: E402
import gen_diff_harness as gdh           # noqa: E402

REF = ROOT / "plugins" / "lodepng-harness-plan" / "plugin.toml"
INVALID = ROOT / "plugins" / "harness-plan-template" / "invalid_sample.toml"
TEMPLATE = ROOT / "plugins" / "harness-plan-template" / "plugin.toml"
COMPARATOR = ROOT / "plugins" / "cjson" / "plugin.toml"
ENTRY = "lodepng_inspect"
ABSTENTION = "no producer returns LodePNGState*; no in-place initializer declared"

fails = 0


def check(cond: bool, what: str):
    global fails
    print(("ok   " if cond else "FAIL ") + what)
    if not cond:
        fails += 1


def rejected(path, needle: str, what: str):
    try:
        rp.parse_manifest(path)
        check(False, f"{what}: accepted (expected rejection)")
    except rp.RealizationManifestError as e:
        check(needle in str(e), f"{what}: {str(e)[:110]}")


def plan(pair: Path, manifests) -> hp.HarnessPlan:
    hp.set_realizations(manifests)
    rs = next(iter(sorted((pair / "translated").glob("*.rs")))).read_text(encoding="utf-8", errors="replace")
    rt = gdh.parse_rust_param_types(rs, ENTRY)
    return hp.build_plan(pair / "build", ENTRY, rust_types=rt, rust_aliases=hp.rust_type_aliases(rs),
                         rust_text=rs)


def main() -> int:
    # ---- loader ----------------------------------------------------------------------------
    m = rp.parse_manifest(REF)
    check(m.kind == rp.KIND and m.c_type == "LodePNGState" and len(m.rust) == 2,
          f"reference manifest loads: {m.ident}, bindings {[b.binding for b in m.rust]}")
    check(len(m.content_hash) == 64, "content hash recorded")
    rejected(INVALID, "not in the closed Rust view vocabulary", "invalid sample (unknown view) rejected at load")
    rejected(COMPARATOR, "is not 'harness-plan-resource-realization'", "comparator manifest rejected (wrong kind)")
    rejected(TEMPLATE, "must be [A-Za-z0-9_.-]+", "unfilled template rejected")
    with tempfile.TemporaryDirectory() as td:
        bad = Path(td) / "bad_key.toml"
        bad.write_text(REF.read_text().replace('storage = "stack"\ninitializer = "lodepng_state_init"\ninitializer_view = "raw-mut"',
                                               'storage = "stack"\nc_snippet = "memset(p, 0, 1)"\ninitializer = "lodepng_state_init"\ninitializer_view = "raw-mut"', 1))
        rejected(bad, "unknown key(s) ['c_snippet']", "unknown key (no snippets) rejected at load")
        bad2 = Path(td) / "not_ident.toml"
        bad2.write_text(REF.read_text().replace('cleanup = "lodepng_state_cleanup"\ncleanup_view = "raw-mut"',
                                                'cleanup = "lodepng_state_cleanup(); system(\\"x\\")"\ncleanup_view = "raw-mut"', 1))
        rejected(bad2, "must be a plain identifier", "non-identifier function name rejected at load")

    # ---- planner ---------------------------------------------------------------------------
    for pair_name, want_binding in (("lodepng_c2rust", "raw-mut-lifecycle"),
                                    ("lodepng_crown", "option-mut-borrow-lifecycle")):
        pair = ROOT / "benchmark" / "pairs" / "rq4" / pair_name
        if not pair.exists():
            check(False, f"{pair_name}: pair not in repo")
            continue
        p0 = plan(pair, [])
        check(p0.status == "failed" and any(ABSTENTION in f for f in p0.failures) and p0.origin is None,
              f"{pair_name}: no plugin -> abstains with `{ABSTENTION}`")
        check("origin" not in hp.plan_dict(p0), f"{pair_name}: no-plugin plan serialises without an origin key")
        p1 = plan(pair, [rp.parse_manifest(REF)])
        o = p1.origin or {}
        check(p1.status == "planned" and o.get("origin") == "plugin" and o.get("binding") == want_binding,
              f"{pair_name}: with plugin -> planned, origin=plugin binding={o.get('binding')}")
        spec = next((i for i in p1.inputs if i["c_decoder"] == "realized_resource"), None)
        check(spec is not None and spec["rust_bridge"] == "view:raw-mut"
              and spec["detail"]["initializer"]["c"] == "lodepng_state_init"
              and spec["detail"]["cleanup"]["c"] == "lodepng_state_cleanup",
              f"{pair_name}: realized_resource spec: init/cleanup bound, bridge {spec and spec['rust_bridge']}")
        checks = {v["check"] for v in o.get("validation", []) if v.get("ok")}
        check(checks == set(range(1, 11)), f"{pair_name}: validation checks passed: {sorted(checks)}")
        check(o.get("plugin", {}).get("content_hash") == rp.parse_manifest(REF).content_hash
              and o["resource"]["c"]["size"] > 0 and o["hooks"]["initializer"]["c_location"]["line"] > 0,
              f"{pair_name}: record carries hash, C layout (size {o['resource']['c']['size']}) and hook locations")
        check(len(o.get("assumptions", [])) >= 2 and any("generic-producer" == r["kind"] for r in o["rejected_alternatives"]),
              f"{pair_name}: assumptions and rejected alternatives recorded")
        # reverse acceptance: same manifest, initializer entries removed
        text = REF.read_text()
        for line in ('initializer = "lodepng_state_init"\n', 'initializer_view = "raw-mut"\n',
                     'initializer_view = "option-mut-borrow"\n'):
            text = text.replace(line, "")
        with tempfile.TemporaryDirectory() as td:
            ni = Path(td) / "no_init.toml"
            ni.write_text(text)
            p2 = plan(pair, [rp.parse_manifest(ni)])
        check(p2.status == "failed" and any("construction unsupported" in f and "check 3" in f
                                            and "declares no in-place initializer" in f for f in p2.failures),
              f"{pair_name}: manifest without initializer -> construction unsupported (check 3)")
        # a binding whose lifecycle views do not fit this translation
        other = [b for b in rp.parse_manifest(REF).rust if b.binding != want_binding]
        m2 = rp.parse_manifest(REF)
        m2.rust = other
        p3 = plan(pair, [m2])
        check(p3.status == "failed" and any("check 4" in f and "not the declared view" in f for f in p3.failures),
              f"{pair_name}: non-fitting binding only -> construction unsupported (check 4)")
    hp.set_realizations([])
    print(f"\n{'FAILED' if fails else 'ok'}: {fails} failing check(s)")
    return 1 if fails else 0


if __name__ == "__main__":
    raise SystemExit(main())

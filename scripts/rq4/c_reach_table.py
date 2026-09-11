#!/usr/bin/env python3
"""Study-level table of the same-corpus C reach diagnostic (docs/c_reach_plan.md).

usage: c_reach_table.py <root with <lib>_<tool>/result.json> <out.md>

Columns: side-specific C reach (functions, regions) and archived Rust reach (never subtracted), the four
matched-function sets over accepted pairs ∩ C scope ∩ Rust scope, ambiguous / unmatched in their own
columns, and the C-side per-input outcomes. The negative-control check is the two EXCLUSIVE sets on the
c2rust cells (c_only and rust_only ~ 0), not the coverages.
"""
import json, sys
from pathlib import Path


def main(root, out):
    rows = []
    for rj in sorted(Path(root).glob("*/result.json")):
        r = json.load(open(rj))
        lib, tool = r["cell"].split("_", 1)
        c, ru, m = r["c"], r["rust_archived"], r.get("matched")
        inp = r.get("inputs", {})
        fmt = lambda a, b: f"{a} / {b}" + (f" ({a / b:.3f})" if b else "")
        rows.append({
            "lib": lib, "tool": tool,
            "bnd": f"{r['boundaries_measured']} / {r['boundaries_archived_built']}",
            "c_fn": fmt(c["functions_reached"], c["functions_total"]),
            "c_reg": fmt(c["regions_reached"], c["regions_total"]),
            "r_fn": fmt(ru["functions_reached"], ru["functions_total"]),
            "r_reg": fmt(ru["regions_reached"], ru["regions_total"]),
            "m": m, "inp": inp, "control": tool == "c2rust"})
    L = ["# Same-corpus C reach — study table", "",
         "Paired reach diagnostic (docs/c_reach_plan.md): each cell's archived Rust-guided corpus replayed in",
         "`C2R_MODE=c-only` through harnesses rebuilt with `--c-coverage`. Side-specific reach is reported per side and",
         "never subtracted. The four sets are over accepted correspondence pairs (matcher `deployment`) ∩ C scope ∩",
         "Rust scope; ambiguous and unmatched functions are separate columns. A C-side `completed` is reach, not",
         "C-definedness. Negative control: on c2rust cells the exclusive sets `C only` and `Rust only` should be ~0 once the\nterminated cases are set aside. Parentheses: `C only` functions reached only through boundaries on whose inputs the Rust\nside TERMINATED (crash / panic / timeout; its reach there is unmeasured), and `Rust only` functions reached only where every\nC-side input crashed. A termination is a translation defect or UB-associated according to the cell's confirmation verdict, never this replay.", "",
         "| library × tool | boundaries | C fn | C reg | Rust fn (archived) | Rust reg (archived) | pairs | both | C only (Rust terminated) | Rust only (C terminated) | neither | ambig. | C unm. (reached) | Rust unm. (reached) | inputs completed / crash / timeout |",
         "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for x in rows:
        m = x["m"]
        if m:
            cn = m["counts"]
            ms = (f"{m['accepted_pairs']} | {cn['both']} | {cn['c_only']} ({cn.get('c_only_rust_terminated', 0)}) | {cn['rust_only']} ({cn.get('rust_only_c_terminated', 0)}) | {cn['neither']} | {m['ambiguous']} | "
                  f"{m['c_unmatched']} ({m['c_unmatched_reached']}) | {m['rust_unmatched']} ({m['rust_unmatched_reached']})")
        else:
            ms = "no map | – | – | – | – | – | – | –"
        i = x["inp"]
        L.append(f"| {x['lib']} × {x['tool']}{' †' if x['control'] else ''} | {x['bnd']} | {x['c_fn']} | {x['c_reg']} | {x['r_fn']} | {x['r_reg']} | {ms} | "
                 f"{i.get('completed', 0)} / {i.get('crash', 0)} / {i.get('timeout', 0)} |")
    L += ["", "† c2rust = negative control (faithful, name-preserving translation).", "",
          f"{len(rows)} cells."]
    Path(out).write_text("\n".join(L) + "\n")
    print(L[-1])


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])

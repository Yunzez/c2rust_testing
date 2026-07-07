"""Parse `gcov -f lil.c` function-level output -> coverage census.
Reports: functions executed / total, the fnc_* handler cluster specifically,
and the list of unexecuted functions (the honest residue)."""
import re, subprocess, sys
out = subprocess.run(["gcov","-f","lil.c"], capture_output=True, text=True).stdout
fns = re.findall(r"Function '([^']+)'\nLines executed:([\d.]+)% of (\d+)", out)
total = [(n,float(p)) for n,p in [(n,p) for n,p,_ in fns]]
hit    = [(n,p) for n,p in total if p > 0]
fnc    = [(n,p) for n,p in total if n.startswith("fnc_")]
fnc_hit= [(n,p) for n,p in fnc if p > 0]
print(f"functions executed : {len(hit)}/{len(total)}  = {len(hit)/len(total):.1%}")
print(f"fnc_* handlers hit : {len(fnc_hit)}/{len(fnc)} = {len(fnc_hit)/len(fnc):.1%}")
miss = sorted(n for n,p in total if p == 0)
print(f"NOT executed ({len(miss)}):")
for i in range(0, len(miss), 5):
    print("   " + "  ".join(f"{x:22s}" for x in miss[i:i+5]))
# line-level total for lil.c
m = re.search(r"File 'lil\.c'\nLines executed:([\d.]+)% of (\d+)", out)
if m: print(f"line coverage (lil.c): {m.group(1)}% of {m.group(2)} lines")

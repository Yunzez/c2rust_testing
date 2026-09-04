import random, struct, sys
random.seed(42)
recs = []
# (i) reconstructed recoverable-UB inputs (fn 11 classes: shift-out-of-range / INT_MIN negate / signed overflow)
ub = [
 "expr 1 << 64", "expr 1 << 70", "expr 1 >> 64", "expr 1 << -1", "expr 5 >> -3",
 "expr -9223372036854775808 * -1", "expr -(-9223372036854775807 - 1)", "expr 0 - -9223372036854775807 - 1 - 1",
 "expr 9223372036854775807 + 1", "expr -9223372036854775807 - 2", "expr 9223372036854775807 * 2", "expr 4611686018427387904 * 4",
]
recs += [("ub", u) for u in ub]
# (ii) the fn-15 order-dependence record
recs.append(("order", "expr ((1+2)*(3+4))"))
# ordinary expr / var / list / string scripts (seeded generator)
ops = ["+","-","*","/","%","<<",">>","&","|","^","<",">","==","!=","&&","||"]
def e(d=0):
    if d>2 or random.random()<0.3: return str(random.randint(-1000,1000))
    r=random.random()
    if r<0.5: return f"({e(d+1)} {random.choice(ops)} {e(d+1)})"
    if r<0.6: return f"-{e(d+1)}"
    return f"{e(d+1)} {random.choice(ops)} {e(d+1)}"
for i in range(300):
    r=random.random()
    if r<0.5: recs.append(("gen", f"expr {e()}"))
    elif r<0.65: recs.append(("gen", f"set x {random.randint(-50,50)}; set y {random.randint(-50,50)}; expr $x * $y + {random.randint(0,9)}"))
    elif r<0.8: recs.append(("gen", f"set l [list {' '.join(str(random.randint(0,99)) for _ in range(random.randint(0,6)))}]; count $l"))
    elif r<0.9: recs.append(("gen", f"set s \"{''.join(random.choice('abc xyz') for _ in range(random.randint(0,12)))}\"; length $s"))
    else: recs.append(("gen", f"func f {{a b}} {{return [expr $a - $b]}}; f {random.randint(0,99)} {random.randint(0,99)}"))
out=open("lil_corpus.bin","wb"); meta=open("lil_corpus.tsv","w")
for i,(k,s) in enumerate(recs):
    b=s.encode(); out.write(struct.pack("<H",len(b))+b); meta.write(f"{i}\t{k}\t{s}\n")
print(len(recs))

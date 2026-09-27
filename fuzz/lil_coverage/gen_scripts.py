"""Grammar-based random lil scripts: random command invocations over the full
stdcmd vocabulary, plus vars/$subst/[brackets]/quotes, to drive dispatch coverage."""
import random, os, sys
CMDS_SIMPLE = ["inc x","dec x","inc x 3","dec x 2","print $x","print [length $l]",
 "set y [expr $x * 2 + 1]","local z","set z 9","append l item","count $l",
 "list a b c","index $l 1","indexof $l b","filter i $l {expr $i}","length $x",
 "charat abcdef 2","codeat abcdef 2","substr abcdefgh 2 5","strpos abcdef cd",
 "trim '  pad  '","ltrim '  pad'","rtrim 'pad  '","repstr aXbXc X Y",
 "split a,b,c ,","strcmp aa bb","streq aa aa","char 65","rand",
 "expr 3 + 4 * 2","expr (1 << 3) | 5","expr 7 % 3 == 1 && 2 < 3","expr ~5 ^ 2",
 "expr -4 + +2","expr 10 / 3.5","expr 1.5e2 > 100","quote {a b}",
 "reflect version","reflect args f","reflect globals","reflect has-global x",
 "result 42","unusedname","jaileval {set q 1}","eval {set w 3}",
 "topeval {set g 7}","upeval {set u 1}","downeval {set d 1}",
 "subst {$x}","concat {a } {b}","error boom","lmap $l a b",
 "func f {a b} {expr $a + $b}","f 3 4","rename f g","g 1 2",
 "set s [store /tmp/lilcov_store.txt hello]","read /tmp/lilcov_store.txt",
 "write out.txt data","source noexist.lil",
]
CMDS_CTRL = ["if [expr $x > 2] {inc x} {dec x}","if not [expr $x] {set x 1}",
 "while {expr $x > 0} {dec x}","for {set i 0} {expr $i < 3} {inc i} {append l $i}",
 "foreach v $l {print $v}","try {error x} {set caught 1}","try {expr 1} ",
 "catcher {set e 1}","exit",
]
def script(rng):
    lines = ["set x %d" % rng.randint(0,9), "list l a b c", "set l [list a b c]"]
    n = rng.randint(4, 14)
    pool = CMDS_SIMPLE + CMDS_CTRL
    for _ in range(n):
        lines.append(rng.choice(pool))
    # mutations: random garbage lines to hit parse-error paths too
    if rng.random() < 0.3:
        lines.append(rng.choice(["${unclosed","[unclosed","}stray{","\"odd'","set"]))
    return "\n".join(lines) + "\n"
def main(outdir, n, seed):
    os.makedirs(outdir, exist_ok=True)
    rng = random.Random(seed)
    for i in range(n):
        open(f"{outdir}/s{i:05d}.lil","w").write(script(rng))
main(sys.argv[1], int(sys.argv[2]), int(sys.argv[3]))

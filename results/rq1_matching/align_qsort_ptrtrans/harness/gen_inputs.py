# Seed-42 batch generator, same distribution as results/rq4_effectiveness/bugs/qsort_ptrtrans/gen_and_diff.py.
# quicksort cases are byte-for-byte the original (low=0, high=n-1); partition uses n>=1 and low=0,high=n-1
# (top-level partition call; n=0 would be C UB arr[-1]); swap = two random ints.
import random, sys
def rv(): return random.choice([random.randint(-2**31,2**31-1), random.randint(-100,100), 0, 2**31-1, -2**31])
trials=50000
random.seed(42)
qs=[]
for t in range(trials):
    n=random.choice([0,1,2,3,5,8,16,64,256]); qs.append((n,[rv() for _ in range(n)]))
open("in_quicksort.txt","w").write(f"{trials}\n"+"".join(f"{n} 0 {n-1}\n"+" ".join(map(str,v))+"\n" for n,v in qs))
random.seed(42)
ps=[]
for t in range(trials):
    n=random.choice([1,2,3,5,8,16,64,256]); ps.append((n,[rv() for _ in range(n)]))
open("in_partition.txt","w").write(f"{trials}\n"+"".join(f"{n} 0 {n-1}\n"+" ".join(map(str,v))+"\n" for n,v in ps))
random.seed(42)
open("in_swap.txt","w").write(f"{trials}\n"+"".join(f"{rv()} {rv()}\n" for _ in range(trials)))

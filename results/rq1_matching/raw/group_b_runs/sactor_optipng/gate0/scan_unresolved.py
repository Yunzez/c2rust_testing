# Mirror of SACTOR's resolver rule: CALL_EXPR with referenced None outside system headers -> fatal.
import sys,glob,os
sys.path.insert(0,'/home/yunzez/c2rust_testing/tools/frameworks/sactor')
from clang import cindex
from sactor import utils
inc=[f"-I{p}" for p in utils.get_compiler_include_paths()]
d=os.getcwd()
def walk(n,out,tu):
    for c in n.get_children():
        if c.kind==cindex.CursorKind.CALL_EXPR and c.referenced is None:
            if c.location.file and not c.location.file.name.startswith('/usr'):
                out.append(f"{os.path.basename(c.location.file.name)}:{c.location.line}")
        walk(c,out,tu)
tot=0
for f in sorted(glob.glob('*.c')):
    tu=cindex.Index.create().parse(f,args=['-x','c','-std=c99',f'-I{d}']+inc,options=cindex.TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD)
    errs=[x.spelling for x in tu.diagnostics if x.severity>=cindex.Diagnostic.Error]
    out=[]; walk(tu.cursor,out,tu)
    out=sorted(set(out))
    if out or errs: print(f, "UNRESOLVED:", out, "ERRORS:", errs[:3]); tot+=len(out)
print("total unresolved sites:",tot)

# Second resolver rule (project_index.py:60-70): a FUNCTION_DECL ref with a USR that no TU defines and
# that is not in a system header is fatal. Compiler builtins have no source file -> they hit this rule.
import sys,glob,os
sys.path.insert(0,'/home/yunzez/c2rust_testing/tools/frameworks/sactor')
from clang import cindex
from sactor import utils
inc=[f"-I{p}" for p in utils.get_compiler_include_paths()]
d=os.getcwd()
def walk(n,out,fn):
    for c in n.get_children():
        if c.kind==cindex.CursorKind.FUNCTION_DECL and c.is_definition(): fn=c.spelling
        if c.kind in (cindex.CursorKind.CALL_EXPR,cindex.CursorKind.DECL_REF_EXPR) and c.referenced is not None and c.referenced.kind==cindex.CursorKind.FUNCTION_DECL:
            r=c.referenced
            if r.location.file is None and c.location.file and not c.location.file.name.startswith('/usr'):
                out.add((os.path.basename(c.location.file.name),c.location.line,fn,r.spelling))
        walk(c,out,fn)
for f in sorted(glob.glob('*.c')):
    tu=cindex.Index.create().parse(f,args=['-x','c','-std=c99',f'-I{d}']+inc,options=cindex.TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD)
    out=set(); walk(tu.cursor,out,None)
    for x in sorted(out): print(*x)

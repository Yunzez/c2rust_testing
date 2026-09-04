"""Per-file bzip2 raw-LLM translation. Each core .c is translated as its own unit
(the 'per translation unit' granularity), with bzlib_private.h given as READ-ONLY
context so shared types (EState/DState) stay coherent across files. Disclosed
rename-commanding prompt, unchanged. One Rust file out per C file."""
import sys, os
sys.path.insert(0, "/home/yunzez/c2rust_testing/experiments/llm_transpiler")
from llm_client import LLMClient
BASE = open("/home/yunzez/c2rust_testing/experiments/llm_transpiler/prompts/translate.md").read()
D = os.path.dirname(os.path.abspath(__file__))
HDR = open(f"{D}/src/bzlib_private.h").read()
cfile = sys.argv[1]                      # e.g. compress.c
out   = sys.argv[2]
c_src = open(f"{D}/src/{cfile}").read()
# header as context, appended to the system prompt (read-only, do not translate)
prompt = BASE + ("\n\nCONTEXT (read-only shared header — DO NOT translate it, only use it to "
  "understand the shared struct/type definitions this file references; translate ONLY the .c below):\n"
  "```c\n" + HDR + "\n```\n")
cli = LLMClient(model=os.environ.get("OPENAI_MODEL","gpt-5.1"), dry_run=False)
res = cli.translate(c_src, system_prompt=prompt)
rust = (res.get("rust_src") or res.get("rust") or "") if isinstance(res, dict) else (res or "")
open(out, "w").write(rust)
print(f"{cfile} -> {out} ({len(rust)} bytes)")

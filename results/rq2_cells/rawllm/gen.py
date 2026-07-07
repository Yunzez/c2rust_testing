import sys, os, json
sys.path.insert(0, "/home/yunzez/c2rust_testing/experiments/llm_transpiler")
from llm_client import LLMClient
PROMPT = open("/home/yunzez/c2rust_testing/experiments/llm_transpiler/prompts/translate.md").read()
c_src = open(sys.argv[1]).read()
model = os.environ.get("OPENAI_MODEL","gpt-5.1")
cli = LLMClient(model=model, dry_run=False)
res = cli.translate(c_src, system_prompt=PROMPT)
if isinstance(res, dict):
    rust = res.get("rust_src") or res.get("rust") or ""
else:
    rust = res if isinstance(res, str) else ""
open(sys.argv[2], "w").write(rust)
print(f"wrote {sys.argv[2]} ({len(rust)} bytes), model={model}")

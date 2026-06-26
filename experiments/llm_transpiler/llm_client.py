#!/usr/bin/env python3
"""Minimal OpenAI client for the LLM C->Rust transpiler track.

Dry-run by default when there is no OPENAI_API_KEY or no `openai` SDK, so the whole
pipeline runs offline (the caller seeds the Rust side from c2rust output instead).
Real mode uses Structured Outputs to get a {cargo_toml, rust_src} crate back with no
markdown fences.

API notes (confirm live model id + price at platform.openai.com/docs/models before a
real run — names/prices shift month to month):
  - default model gpt-5.4-mini (cheap coding workhorse); gpt-4.1 if you need
    temperature=0 reproducibility (GPT-5.x / o-series REJECT `temperature`).
  - 429 backoff is handled by the SDK's built-in retries (max_retries).
Pattern borrowed from SbomFuzz analyzer/.../fuzzing_harness_gen/llm_client.py (dry-run +
rate-limit handling).
"""
import json
import os
import sys

# Confirm the live id before a real call; override with $OPENAI_MODEL.
DEFAULT_MODEL = os.environ.get("OPENAI_MODEL", "gpt-5.4-mini")

# Structured Output schema -> guaranteed {cargo_toml, rust_src}, no code fences.
_SCHEMA = {
    "type": "json_schema",
    "json_schema": {
        "name": "rust_crate",
        "strict": True,
        "schema": {
            "type": "object",
            "additionalProperties": False,
            "properties": {
                "cargo_toml": {"type": "string"},
                "rust_src": {"type": "string"},
            },
            "required": ["cargo_toml", "rust_src"],
        },
    },
}


def _is_reasoning(model: str) -> bool:
    return model.startswith(("gpt-5", "o1", "o3", "o4"))


class LLMClient:
    def __init__(self, model: str = DEFAULT_MODEL, dry_run=None):
        self.model = model
        key = os.getenv("OPENAI_API_KEY") or os.getenv("API_KEY")
        self.dry_run = (not key) if dry_run is None else dry_run
        self._client = None
        if not self.dry_run:
            try:
                from openai import OpenAI  # lazy: not needed in dry-run
                self._client = OpenAI(api_key=key, max_retries=5)
            except ImportError:
                print("[llm_client] `openai` SDK not installed (pip install -U openai); "
                      "falling back to dry-run", file=sys.stderr)
                self.dry_run = True

    def translate(self, c_source: str, system_prompt: str = "") -> dict | None:
        """Return {'cargo_toml','rust_src'} from the LLM, or None in dry-run."""
        if self.dry_run:
            return None
        kwargs = dict(
            model=self.model,
            messages=[{"role": "system", "content": system_prompt},
                      {"role": "user", "content": c_source}],
            response_format=_SCHEMA,
        )
        if not _is_reasoning(self.model):
            kwargs["temperature"] = 0  # only legacy non-reasoning models accept this
        resp = self._client.chat.completions.create(**kwargs)
        return json.loads(resp.choices[0].message.content)

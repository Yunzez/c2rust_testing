#!/usr/bin/env python3
"""Emit results/rq4_effectiveness/defect_manifest.md from results/rq4_effectiveness/defect_manifest.json.

Usage:  python3 results/rq4_effectiveness/gen_defect_manifest.py [--build]
  --build   regenerate defect_manifest.json from the embedded source-of-truth
            tables in this script first (the JSON is the canonical artefact;
            the tables below are how it was authored, so both stay in sync).

Counting units follow CONTRACT ADDENDUM v5 (U1-U5): defects are root-cause
level; "divergent / valid records" is per-cell strength and NOT summable.
"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
JSON_PATH = os.path.join(HERE, "defect_manifest.json")
MD_PATH = os.path.join(HERE, "defect_manifest.md")

CHANNELS = ["O-R", "O-P(silent)", "O-P(print)", "O-S", "O-F"]

# ---------------------------------------------------------------------------
# OBS pilot cells (single seed 42; results/ablations/observation/obs_qsort_ptrtrans + obs_matrix)
# ---------------------------------------------------------------------------
OBS_CELLS = {
    "qsort_c2saferrust": {
        "path": "results/ablations/observation/obs_matrix/qsort_c2saferrust/{result.json,RESULT.md}",
        "valid_records": 113, "c_ub_excluded": 0, "c_unstable": 0,
        "classification": {"Rust-failure": 51, "semantic-difference": 0, "agree": 62},
        "cell_divergences": {"O-R": 51, "O-P(silent)": 51, "O-P(print)": 51, "O-S": 51, "O-F": 51},
        "corpus_note": "merged set: fresh libFuzzer seed-42 corpus (3 files) + 6 saved crash inputs + archived obs_qsort_ptrtrans seed-42 corpus (104); crash cell cannot build its own corpus",
        "c_oracle": "results/rq4_effectiveness/bugs/qsort_c2saferrust/source/qsort.c (archived original), clang ASan+UBSan",
        "defects": ["C1"],
    },
    "crc32_c2saferrust": {
        "path": "results/ablations/observation/obs_matrix/crc32_c2saferrust/{result.json,RESULT.md}",
        "valid_records": 65, "c_ub_excluded": 0, "c_unstable": 0,
        "classification": {"Rust-failure": 0, "semantic-difference": 15, "agree": 50},
        "cell_divergences": {"O-R": 15, "O-P(silent)": 0, "O-P(print)": 15, "O-S": 15, "O-F": 15},
        "corpus_note": "fresh libFuzzer seed 42, 95.6M execs, 65 files; 15/15 divergent inputs contain an empty chunk with nonzero running CRC, 0/50 agreeing do",
        "c_oracle": "zlib 1.2.11 crc32.c fetched from zlib.net/fossils (optipng vendors '1.2.11-optipng'; no in-repo C zlib has crc32_z)",
        "defects": ["S1"],
    },
    "bzip2_laertes": {
        "path": "results/ablations/observation/obs_matrix/bzip2_laertes/{result.json,RESULT.md}",
        "valid_records": 529, "c_ub_excluded": 0, "c_unstable": 0,
        "classification": {"Rust-failure": 0, "semantic-difference": 528, "agree": 1},
        "cell_divergences": {"O-R": 0, "O-P(silent)": 0, "O-P(print)": 528, "O-S": 528, "O-F": 528},
        "corpus_note": "fresh libFuzzer seed 42, 641k execs, 529 files; divergence only at block-CRC / combined-CRC fields, payload byte-identical",
        "c_oracle": "tools/frameworks/crown/c-code/bzip2/*.c (bzip2 1.0.8), clang ASan+UBSan",
        "defects": ["S3"],
    },
    "qsort_ptrtrans": {
        "path": "results/ablations/observation/obs_qsort_ptrtrans/{result.json,RESULT.md}",
        "valid_records": 104, "c_ub_excluded": 0, "c_unstable": 0,
        "classification": {"Rust-failure": 0, "semantic-difference": 71, "agree": 33},
        "cell_divergences": {"O-R": 0, "O-P(silent)": 0, "O-P(print)": 71, "O-S": 71, "O-F": 71},
        "corpus_note": "fresh libFuzzer seed 42, 6.47M execs, 104 files; agree = 31 already-sorted + 1 constant + 1 n<=1",
        "c_oracle": "results/rq4_effectiveness/bugs/qsort_ptrtrans/original_qsort.c (archived original), clang ASan+UBSan",
        "defects": ["S6"],
    },
    "cjson_ptrtrans": {
        "path": "results/ablations/observation/obs_matrix/cjson_ptrtrans/{result.json,RESULT.md}",
        "valid_records": 139, "c_ub_excluded": 0, "c_unstable": 0,
        "classification": {"Rust-failure": 0, "semantic-difference": 113, "agree": 26},
        "cell_divergences": {"O-R": 31, "O-P(silent)": 0, "O-P(print)": 111, "O-S": 113, "O-F": 113},
        "corpus_note": "fresh libFuzzer seed 42 with escape dictionary, 82.8M execs, 139 files; 113 semantic-difference records = class (a) 31 [30 non-UTF-8 -> S9, 1 \\u escape -> S7] + class (b) 80 [valuestring=None -> S8] + class (c) 2 [offset-only, CANDIDATE, untriaged]; confirmed divergent records = 111 (U2)",
        "c_oracle": "cJSON v1.7.19 re-fetched from upstream tag (translated crate reports 1.7.19; sha256 in harness/cJSON.c.sha256) -> version-matched reconstructed (U4)",
        "defects": ["S7", "S8", "S9"],
    },
    "tulip_c2saferrust": {
        "path": "results/ablations/observation/obs_matrix/tulip_c2saferrust/{result.json,RESULT.md}",
        "valid_records": 201, "c_ub_excluded": 1, "c_unstable": 0,
        "classification": {"Rust-failure": 1, "semantic-difference": 77, "agree": 123},
        "cell_divergences": {"O-R": 48, "O-P(silent)": 48, "O-P(print)": 78, "O-S": 48, "O-F": 78},
        "corpus_note": "seeded generator (random.seed(42)), 202 argv records, NOT coverage-guided (CLI boundary); C-UB excluded 1 (hma 1 4, ASan global-overflow in C sample.c). 78 non-agree = 1 crash [C6] + 47 exit-visible + 21 stdout-only/both-exit-1 [S13, one root cause, two symptoms] + 9 stdout-only/both-exit-0 [CANDIDATE display divergence, untriaged]",
        "c_oracle": "tools/frameworks/tulipindicators sample.c + tiamalgamation.c (upstream pull, fn 37: v0.9.2), clang ASan+UBSan; identity with the C that C2SaferRust's input crate was transpiled from is NOT recorded",
        "defects": ["C6", "S13"],
    },
}

NOT_RUN = {c: "not run" for c in CHANNELS}

def chan(r, ps, pp, s, f):
    return {"O-R": r, "O-P(silent)": ps, "O-P(print)": pp, "O-S": s, "O-F": f}

# ---------------------------------------------------------------------------
# The confirmed defects (results/rq4_effectiveness/bugs_detailed.md C1-C7, S1-S14)
# ---------------------------------------------------------------------------
DEFECTS = [
    dict(id="C1", library="qsort", tool="C2SaferRust", kind="crash",
         root_cause="int->usize index rewrite breaks negative recursion-termination sentinel",
         family="control-flow preservation failure",
         symptom="runaway recursion -> stack overflow / OOB read (ASan abort, SIGABRT)",
         boundary="function (quickSort) / crash",
         obs_cell="qsort_c2saferrust",
         obs=chan("yes (Rust-failure)", "yes (Rust-failure)", "yes (Rust-failure)", "yes (Rust-failure)", "yes (Rust-failure)"),
         records="51 / 113 (all Rust-failure; O-R sees NO-RETURN)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="original qsort.c archived at results/rq4_effectiveness/bugs/qsort_c2saferrust/source/qsort.c",
         evidence="results/rq4_effectiveness/bugs/qsort_c2saferrust/; results/rq4_effectiveness/translation_matrix.md fn 1; results/rq4_effectiveness/bugs_detailed.md:33"),
    dict(id="C2", library="urlparser", tool="C2SaferRust", kind="crash",
         root_cause="to_str().unwrap() on protocol bytes; C used strcmp",
         family="byte-string domain narrowing",
         symptom="panic on first non-UTF-8 byte (url_is_ssh)",
         boundary="function (url_is_ssh) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no (ATTR urlparser pilot targets the C-side get_part heap overflow, not this defect; results/ablations/attribution/urlparser/)",
         align="no",
         provenance="exact-source",
         provenance_note="C quoted from tools/frameworks/c2saferrust/laertes_benchmarks/urlparser_WIP/url.h:338 (the benchmark dir the tool consumed); repro compares base-c2rust vs WIP",
         evidence="results/rq4_effectiveness/bugs/utf8_panic_c2saferrust/ (Instance A); results/rq4_effectiveness/translation_matrix.md fn 2; results/rq4_effectiveness/bugs_detailed.md:80"),
    dict(id="C3", library="lil", tool="C2SaferRust", kind="crash",
         root_cause="to_str().unwrap() on argv in do_system; C used memcpy",
         family="byte-string domain narrowing",
         symptom="panic on non-UTF-8 argv (do_system)",
         boundary="function (do_system) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no (ATTR lil pilot covers CROWN/Laertes lil, not C2SaferRust; results/ablations/attribution/lil/)",
         align="no",
         provenance="exact-source",
         provenance_note="C quoted from laertes_benchmarks/lil_WIP/main.c:47 (in-repo); repro compares base-c2rust vs WIP",
         evidence="results/rq4_effectiveness/bugs/utf8_panic_c2saferrust/lil_do_system_diff.rs; results/rq4_effectiveness/translation_matrix.md fn 6; results/rq4_effectiveness/bugs_detailed.md:125"),
    dict(id="C4", library="bzip2", tool="C2SaferRust", kind="crash",
         root_cause="to_str().unwrap() on filename in endsInBz2; C byte-wise",
         family="byte-string domain narrowing",
         symptom="panic on non-UTF-8 filename (endsInBz2)",
         boundary="function (endsInBz2) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="unknown",
         provenance_note="C quoted from tools/frameworks/crown/c-code/bzip2/bzip2recover.c:261 (bzip2 1.0.8, in-repo); whether this is byte-identical to the Laertes-benchmark bzip2 that C2SaferRust's input crate (bzip2_WIP) was transpiled from is not recorded (bzip2_WIP ships no .c)",
         evidence="results/rq4_effectiveness/bugs/utf8_panic_c2saferrust/bzip2_endsInBz2_diff.rs; results/rq4_effectiveness/translation_matrix.md fn 7; results/rq4_effectiveness/bugs_detailed.md:171"),
    dict(id="C5", library="optipng", tool="C2SaferRust", kind="crash",
         root_cause="to_str().unwrap() on -dir path before opng_os_create_dir",
         family="byte-string domain narrowing",
         symptom="panic on non-UTF-8 output directory (CLI -dir)",
         boundary="CLI (-dir option) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="results/rq4_effectiveness/bugs_detailed.md:212ff: original .c not in repo; base c2rust optim.rs:3238 is the verified-faithful reference",
         evidence="optipng_WIP/src/optipng/optim.rs:3096 vs optipng/src/optipng/optim.rs:3238; results/rq4_effectiveness/bug_table.md; results/rq4_effectiveness/translation_matrix.md fn 9; results/rq4_effectiveness/bugs_detailed.md:212"),
    dict(id="C6", library="tulipindicators", tool="C2SaferRust", kind="crash",
         root_cause="argv[1] read hoisted above the argc<2 guard",
         family="control-flow preservation failure",
         symptom="SIGSEGV (CStr::from_ptr(NULL)) on zero-argument invocation",
         boundary="CLI (sample driver, argc guard) / crash",
         obs_cell="tulip_c2saferrust",
         obs=chan("yes (Rust-failure)", "yes (Rust-failure)", "yes (Rust-failure)", "yes (Rust-failure)", "yes (Rust-failure)"),
         records="1 / 201 (the no-args record; SAME CELL as S13 - do not sum)",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="catalog: tulip C source not in repo, base c2rust is the reference. OBS cell oracle = upstream tulip sample.c+tiamalgamation.c (fn 37 pull, v0.9.2); version identity with the tool's input not recorded",
         evidence="results/rq4_effectiveness/bugs/tulip_c2saferrust/ (Bug 1, excerpt_guard_hoisting.rs); results/rq4_effectiveness/translation_matrix.md fn 30; results/rq4_effectiveness/bugs_detailed.md:241"),
    dict(id="C7", library="bzip2", tool="CROWN", kind="crash",
         root_cause="ownership/slice rewrite of compress path writes past heap allocation",
         family="ownership-state corruption",
         symptom="glibc abort free(): invalid next size on ~25% of inputs",
         boundary="function (BZ2_bzBuffToBuffCompress) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="CROWN consumed tools/frameworks/crown/c-code/bzip2 (bzip2 1.0.8); archive oracle_comp.c is built from it. Behavioral attribution only: no line-level CROWN-vs-base diff archived (results/rq4_effectiveness/bugs_detailed.md appendix)",
         evidence="results/rq4_effectiveness/bugs/bzip2_crown/; results/rq4_effectiveness/translation_matrix.md fn 10; results/rq4_effectiveness/bugs_detailed.md:282"),
    dict(id="S1", library="optipng (zlib)", tool="C2SaferRust", kind="semantic",
         root_cause="crc32_z: is_null -> is_empty; empty chunk resets running CRC",
         family="null/empty conflation",
         symptom="wrong CRC returned (0) after any zero-length chunk; no crash",
         boundary="function (crc32_z return value)",
         obs_cell="crc32_c2saferrust",
         obs=chan("yes", "no", "yes", "yes", "yes"),
         records="15 / 65",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="catalog appendix: original zlib .c not archived, base c2rust quoted as reference. OBS cell oracle = zlib 1.2.11 crc32.c re-fetched (vendored version string '1.2.11-optipng') -> version-matched reconstructed for the OBS cell",
         evidence="results/rq4_effectiveness/bugs/crc32_c2saferrust/; results/rq4_effectiveness/translation_matrix.md fn 9; results/rq4_effectiveness/bugs_detailed.md:322"),
    dict(id="S2", library="optipng (zlib)", tool="C2SaferRust", kind="semantic",
         root_cause="adler32_z: NULL guard became length test; empty chunk resets",
         family="null/empty conflation",
         symptom="wrong Adler-32 on empty chunk (plus extra non-empty miscompile); no crash",
         boundary="function (adler32_z return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="catalog appendix (S1-S4 checksum C sides not archived); base.rs quoted as reference",
         evidence="results/rq4_effectiveness/bugs/crc32_c2saferrust/adler32_corroboration/; results/rq4_effectiveness/translation_matrix.md fn 9; results/rq4_effectiveness/bugs_detailed.md:360"),
    dict(id="S3", library="bzip2", tool="Laertes", kind="semantic",
         root_cause="laertes_init_BZ2_crc32Table emitted but never called; table zero",
         family="initialization loss or corruption",
         symptom="BZ_OK returned, wrong block/combined CRC in output stream; bunzip2 rejects",
         boundary="output buffer (compressed stream) - return value unchanged",
         obs_cell="bzip2_laertes",
         obs=chan("no", "no", "yes", "yes", "yes"),
         records="528 / 529",
         attr="no", align="no",
         provenance="unknown",
         provenance_note="catalog appendix: original bzip2 .c not archived for the checksum quotes (base c2rust quoted). Execution oracle (archive + OBS cell) = tools/frameworks/crown/c-code/bzip2 (bzip2 1.0.8); identity with the Laertes-benchmark bzip2 that Laertes consumed is not recorded",
         evidence=("results/rq4_effectiveness/bugs/bzip2_laertes/; results/rq4_effectiveness/translation_matrix.md fn 14; results/rq4_effectiveness/bugs_detailed.md:388. INDEPENDENT REDISCOVERY 2026-09-03: the schema-driven differential harness generated for BZ2_bzBuffToBuffCompress (generator 0.6, with no knowledge of this defect) reported 58,327 output-buffer divergence events in a 3600 s campaign, with the same signature as the archived cell (written length equal, bytes differ); results/rq3_coverage/bzip2/")),
    dict(id="S4", library="optipng (zlib)", tool="Laertes", kind="semantic",
         root_cause="laertes_init_crc_table never called; no DYNAMIC_CRC_TABLE rebuild",
         family="initialization loss or corruption",
         symptom="crc32 returns degenerate value (crc32('a') ff000000 vs e8b7be43); 98.49% wrong",
         boundary="function (crc32_z return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="catalog appendix; archive oracle_zlib.c uses the canonical SYSTEM zlib (fn 8), not the vendored source",
         evidence="results/rq4_effectiveness/bugs/optipng_laertes/; results/rq4_effectiveness/translation_matrix.md fn 8; results/rq4_effectiveness/bugs_detailed.md:426"),
    dict(id="S5", library="genann", tool="SACTOR", kind="semantic",
         root_cause="mutable lookup[] lifted to immutable static; init writes are Rust UB",
         family="initialization loss or corruption",
         symptom="release: all-zero network output on 100% of inputs, exit 0; debug: SIGSEGV",
         boundary="global (lookup table) -> function output (genann_run)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="SACTOR consumed the in-repo genann.c (renamed genann_lib.c; compile_commands.json archived); driver.c in archive",
         evidence="results/rq4_effectiveness/bugs/genann_sactor/; results/rq4_effectiveness/translation_matrix.md fn 32; results/rq4_effectiveness/bugs_detailed.md:464"),
    dict(id="S6", library="qsort", tool="PtrTrans", kind="semantic",
         root_cause="split_at_mut swap indexes right[j-i] not right[0]; None-swallowing swap",
         family="interface-contract loss",
         symptom="68% of UB-free arrays returned unsorted; zero panics; passes cargo-check gate",
         boundary="function (quickSort/quick_sort array contract; also visible at partition)",
         obs_cell="qsort_ptrtrans",
         obs=chan("no", "no", "yes", "yes", "yes"),
         records="71 / 104 (OBS pilot); archive campaign 34,012 / 50,000",
         attr="no",
         align="yes - results/rq1_matching/align_qsort_ptrtrans/result.json: name-eq 2/3 correspondence, 0/1 defective contract boundary, 1/1 unique defect (via partition 30,480/50,000); tool map / matcher / manual 3/3, 1/1, 1/1; 0 false divergences",
         provenance="exact-source",
         provenance_note="results/rq4_effectiveness/bugs/qsort_ptrtrans/original_qsort.c archived; used by both OBS and ALIGN pilots",
         evidence="results/rq4_effectiveness/bugs/qsort_ptrtrans/; results/rq4_effectiveness/translation_matrix.md fn 29; results/rq4_effectiveness/bugs_detailed.md:533"),
    dict(id="S7", library="cJSON", tool="PtrTrans", kind="semantic",
         root_cause="parse_string passes empty input_end slice; utf16 gate always rejects",
         family="interface-contract loss",
         symptom="every \\uXXXX escape fails parsing (C ret=1, Rust ret=0)",
         boundary="function (parse_string return value; call-site contract)",
         obs_cell="cjson_ptrtrans",
         obs=chan("yes", "no", "yes", "yes", "yes"),
         records="1 / 139 (class (a) \\u sub-class; SAME CELL as S8/S9 - do not sum; under-represented because the bug makes every \\u path a 1-edge early return)",
         attr="no", align="no (ALIGN cJSON audit: no map artifact survives; results/rq1_matching/align_qsort_ptrtrans/result.json cjson_audit)",
         provenance="version-matched reconstructed",
         provenance_note="catalog: the cJSON.c the oracle #includes is not in the repo; OBS cell oracle = cJSON v1.7.19 re-fetched from upstream (crate reports 1.7.19) (U4)",
         evidence="results/rq4_effectiveness/bugs/cjson_ptrtrans/ (excerpt_utf16_gate.rs, excerpt_callsite_and_valuestring.rs); results/rq4_effectiveness/translation_matrix.md fn 5; results/rq4_effectiveness/bugs_detailed.md:594"),
    dict(id="S8", library="cJSON", tool="PtrTrans", kind="semantic",
         root_cause="valuestring assigned None on success path; value discarded",
         family="interface-contract loss",
         symptom="parse returns success, item.valuestring NULL (C: bytes); data loss",
         boundary="struct field (cJSON.valuestring) - return value equal",
         obs_cell="cjson_ptrtrans",
         obs=chan("no", "no", "yes", "yes", "yes"),
         records="80 / 139 (class (b); SAME CELL as S7/S9 - do not sum)",
         attr="no", align="no",
         provenance="version-matched reconstructed",
         provenance_note="as S7 (cJSON v1.7.19 re-fetched)",
         evidence="results/rq4_effectiveness/bugs/cjson_ptrtrans/translated_crate/src/cjson.rs:854; results/rq4_effectiveness/translation_matrix.md fn 5; results/rq4_effectiveness/bugs_detailed.md:645"),
    dict(id="S9", library="cJSON", tool="PtrTrans", kind="semantic",
         root_cause="from_utf8 gate added; C stores raw bytes without validation",
         family="byte-string domain narrowing",
         symptom="non-UTF-8 string payloads rejected (C ret=1, Rust ret=0)",
         boundary="function (parse_string return value; input-domain narrowing)",
         obs_cell="cjson_ptrtrans",
         obs=chan("yes", "no", "yes", "yes", "yes"),
         records="30 / 139 (class (a) non-UTF-8 sub-class; SAME CELL as S7/S8 - do not sum)",
         attr="no", align="no",
         provenance="version-matched reconstructed",
         provenance_note="as S7 (cJSON v1.7.19 re-fetched)",
         evidence="results/rq4_effectiveness/bugs/cjson_ptrtrans/translated_crate/src/cjson.rs:854,860; results/rq4_effectiveness/translation_matrix.md fn 5; results/rq4_effectiveness/bugs_detailed.md:683"),
    dict(id="S10", library="bzip2", tool="CROWN", kind="semantic",
         root_cause="ownership-lift rewrite of compressor emits structurally invalid stream",
         family="ownership-state corruption",
         symptom="BZ_OK returned, bunzip2 'Data integrity error' on ~46% of inputs",
         boundary="output buffer (compressed stream) - return value unchanged",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="as C7 (crown/c-code/bzip2 1.0.8 = CROWN's input; behavioral attribution only, no line-level diff archived)",
         evidence=("results/rq4_effectiveness/bugs/bzip2_crown/ (crown_compress_driver.rs, crown_corrupt_A4096.bz2); results/rq4_effectiveness/translation_matrix.md fn 10; results/rq4_effectiveness/bugs_detailed.md:716. INDEPENDENT REDISCOVERY 2026-09-03: the generated differential harness for the CROWN compress boundary (shared with C7) reported 75,872 written-length divergence events in a 3600 s campaign; results/rq3_coverage/bzip2/")),
    dict(id="S11", library="bzip2", tool="CROWN", kind="semantic",
         root_cause="decompress small=0 fast-path state machine broken by lift",
         family="ownership-state corruption",
         symptom="BZ_DATA_ERROR (-4) on streams canonical bunzip2 accepts; small=1 path works",
         boundary="function (BZ2_bzBuffToBuffDecompress return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="as C7",
         evidence="results/rq4_effectiveness/bugs/bzip2_crown/crown_decompress_driver.rs; results/rq4_effectiveness/translation_matrix.md fn 10; results/rq4_effectiveness/bugs_detailed.md:745"),
    dict(id="S12", library="bzip2", tool="C2SaferRust", kind="semantic",
         root_cause="API NULL guards folded into is_empty(); valid empty buffer rejected",
         family="null/empty conflation",
         symptom="BZ2_bzBuffToBuffCompress returns BZ_PARAM_ERROR for sourceLen==0 (C: valid empty stream)",
         boundary="function (BZ2_bzBuffToBuffCompress return value, API entry)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="unknown",
         provenance_note="as C4: C quoted from crown/c-code/bzip2/bzlib.c:1247 (1.0.8); identity with the Laertes-benchmark bzip2 behind bzip2_WIP not recorded",
         evidence="bzip2_WIP/bzlib.rs:2085 vs crown/c-code/bzip2/bzlib.c:1247; results/rq4_effectiveness/semantic_diffs.md row 3; results/rq4_effectiveness/translation_matrix.md fn 7; results/rq4_effectiveness/bugs_detailed.md:771"),
    dict(id="S13", library="tulipindicators", tool="C2SaferRust", kind="semantic",
         root_cause="main() wrapper subtracts 1 from env::args().len(); argc off by one",
         family="control-flow preservation failure",
         symptom="valid invocations rejected: exit 1 + '*ERROR NOT ENOUGH OPTIONS*' (47 records) or 'No indicator given.' (21 records) where C exits 0 / prints a different error",
         boundary="CLI (sample driver argc; exit status + stdout)",
         obs_cell="tulip_c2saferrust",
         obs=chan("yes", "yes", "yes", "yes", "yes"),
         records="68 / 201 = 47 exit-visible + 21 stdout-only/both-exit-1 (one root cause, two symptoms, U2); O-R/O-S/O-P(silent) see 47, O-P(print)/O-F see 68; SAME CELL as C6 - do not sum",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="as C6 (tulip C not in repo; OBS cell oracle = upstream pull, version identity not recorded; contract additionally evidenced by the WIP's own argc < 3 + i_0 check)",
         evidence="results/rq4_effectiveness/bugs/tulip_c2saferrust/ (Bug 2, excerpt_argc_offbyone.rs); results/rq4_effectiveness/translation_matrix.md fn 30; results/rq4_effectiveness/bugs_detailed.md:816"),    dict(id="S14", library="bzip2", tool="C2SaferRust", kind="semantic",
         root_cause="mmed3 rewritten as the minimum of three bytes; the C helper computes their median",
         family="semantic computation substitution",
         symptom="internal helper returns a wrong value on 99.41% of its own input domain; the wrong value is used as the mainQSort3 pivot",
         boundary="function (mmed3 return value) - internal helper, single call site (mainQSort3 pivot selection)",
         obs_cell=None,
         obs=chan("not run", "not run", "not run", "not run", "not run"),
         records=("EXHAUSTIVE, DIFFERENT UNIT (U1): 16,679,040 / 16,777,216 (u8,u8,u8) triples = 99.41% differ. "
                  "The unit is ONE INVOCATION of mmed3 over its complete input domain, NOT a per-cell library "
                  "input record; never pool with the record counts of the other rows. Independently detected by "
                  "the generated differential harness: 13,602 return-value divergence events in a 600 s campaign "
                  "(generator 0.6)"),
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note=("C2SaferRust is a Rust-to-Rust rewriter and its INPUT is in the repo: "
                          "laertes_benchmarks/bzip2/blocksort.rs:884 is the base c2rust translation and "
                          "implements the median (a line-for-line transliteration of the C, temp swap included); "
                          "laertes_benchmarks/bzip2_WIP/blocksort.rs:822 is the same tool's output and returns the "
                          "minimum. The defect is therefore pinned to the C2SaferRust rewrite step regardless of "
                          "whether the original .c was byte-identical (bzip2_WIP ships no .c). C for context: "
                          "tools/frameworks/crown/c-code/bzip2/blocksort.c:583 (bzip2 1.0.8). Hashes of all three "
                          "in the archive raw/source_hashes.txt"),
         evidence=("results/rq4_effectiveness/bugs/bzip2_c2saferrust_mmed3/ (exhaustive census, both bodies, the "
                   "single call site, the generated harness and a saved divergence input). DOWNSTREAM SCOPE: the "
                   "established effect is pivot selection and potential performance degradation only; an effect on "
                   "bzip2 compressed output is NOT claimed and was not demonstrated either way (the artifact aborts "
                   "earlier in sendMTFValues, CAND-3)")),
]

CANDIDATES = [
    dict(id="CAND-1", library="cJSON", tool="PtrTrans",
         status="candidate - untriaged (triage in progress)",
         description="class (c): both sides fail (ret=0), valuestring equal, only buffer.offset differs (C advances past the bad char, Rust does not); failure-path bookkeeping",
         records="2 / 139 (cjson_ptrtrans OBS cell; excluded from the 111 confirmed divergent records, U2)",
         obs=chan("no", "no", "no (offset not printed)", "yes", "yes"),
         evidence="results/ablations/observation/obs_matrix/cjson_ptrtrans/RESULT.md (per-class split); not in results/rq4_effectiveness/bugs/cjson_ptrtrans/README.md"),
    dict(id="CAND-2", library="tulipindicators", tool="C2SaferRust",
         status="candidate - untriaged (triage in progress)",
         description="stdout-only, both-exit-0 display divergence in WIP sample.rs: alt-input 'input' column dropped (x5), extra 'close' column for dx/adxr (x4); NOT the argc bug",
         records="9 / 201 (tulip_c2saferrust OBS cell; excluded from every main count, U2)",
         obs=chan("no", "no", "yes", "no", "yes"),
         evidence="results/ablations/observation/obs_matrix/tulip_c2saferrust/RESULT.md (visibility breakdown); results/ablations/observation/obs_matrix/tulip_c2saferrust/raw/replay_records.jsonl"),    dict(id="CAND-3", library="bzip2", tool="C2SaferRust",
         status="candidate - untriaged (triage in progress)",
         description=("sendMTFValues fast track: the rewrite takes the slice base at the START of mtfv "
                      "(std::slice::from_raw_parts(mtfv, 50), carrying the translator comment "
                      "Assuming mtfv has at least 50 elements) but keeps the ABSOLUTE index gs + i, so any "
                      "gs > 0 indexes past the 50-element slice. The C indexes mtfv[gs..ge] and is in bounds "
                      "by construction when ge-gs+1 == 50. Same base/offset shape as S6"),
         records=("deterministic, not a fuzz count: the shipped bzip2 acceptance suite aborts on the first "
                  "sample with index out of bounds: the len is 50 but the index is 50 (compress and decompress "
                  "alike). Held out of every main count (U2)"),
         obs=chan("not run", "not run", "not run", "not run", "not run"),
         evidence=("fuzz/bzip2_wip_e3/src/compress.rs:851 (== laertes_benchmarks/bzip2_WIP) vs "
                   "tools/frameworks/crown/c-code/bzip2/compress.c sendMTFValues; reproduced through the shipped "
                   "Makefile test target in results/rq3_coverage/bzip2/")),
    dict(id="CAND-4", library="bzip2", tool="CROWN",
         status="candidate - untriaged (an input-model artifact has NOT yet been excluded)",
         description=("fallbackSort: the generated differential harness reports an out-array divergence on bhtab. "
                      "Root cause not yet identified. bhtab has no length parameter in the C signature, so the "
                      "schema gives both sides the same zeroed fixed-capacity array while C sizes it 2 + nblock/32 "
                      "words; until that is ruled out this may be a harness input-model artifact rather than a "
                      "translation defect"),
         records="26,404 divergence events in a 600 s campaign (event count, NOT deduplicated records; U1/U2)",
         obs=chan("not run", "not run", "not run", "not run", "not run"),
         evidence="results/rq3_coverage/bzip2/ (crown campaign logs and the generated harness)"),
]

ATTR_NOTE = ("No confirmed defect is covered by an ATTR pilot: both ATTR cells (urlparser: "
             "results/ablations/attribution/urlparser/result.json; lil: results/ablations/attribution/lil/result.json) "
             "are UB-exclusion / certificate cells with 0 confirmed translation divergences "
             "(CONTRACT P2/P3). ATTR coverage of the confirmed defects = 0/%d." % len(DEFECTS))
ALIGN_NOTE = ("ALIGN pilot exists for one defect (S6, results/rq1_matching/align_qsort_ptrtrans/). "
              "ALIGN coverage of the confirmed defects = 1/%d. No buildable wrong-map witness exists (P5)." % len(DEFECTS))


def build():
    families = {}
    for d in DEFECTS:
        families.setdefault(d["family"], []).append(d["id"])
    run = [d for d in DEFECTS if d["obs_cell"]]
    not_run = [d for d in DEFECTS if not d["obs_cell"]]
    per_channel = {}
    for ch in CHANNELS:
        rec = [d["id"] for d in run if d["obs"][ch].startswith("yes")]
        per_channel[ch] = {"recovered": len(rec), "of_run": len(run), "defects": rec,
                           "missed": [d["id"] for d in run if not d["obs"][ch].startswith("yes")]}
    crash_run = [d["id"] for d in run if d["kind"] == "crash"]
    semantic_run = [d["id"] for d in run if d["kind"] == "semantic"]
    per_channel_semantic_only = {}
    for ch in CHANNELS:
        rec = [d["id"] for d in run if d["kind"] == "semantic" and d["obs"][ch].startswith("yes")]
        per_channel_semantic_only[ch] = {"recovered": len(rec), "of_run_semantic": len(semantic_run), "defects": rec}
    summary = {
        "counting_units": "Two units, never mixed (U1): DEFECTS (root-cause level) and DIVERGENT/VALID RECORDS (per-cell strength). Record counts are per OBS cell and are NOT summable across cells or across defects sharing a cell (C6+S13 share tulip_c2saferrust; S7+S8+S9 share cjson_ptrtrans).",
        "defects_total": len(DEFECTS),
        "crash": sum(1 for d in DEFECTS if d["kind"] == "crash"),
        "semantic": sum(1 for d in DEFECTS if d["kind"] == "semantic"),
        "defects_per_family": {k: {"count": len(v), "ids": v} for k, v in families.items()},
        "obs_run_defects": {"count": len(run), "ids": [d["id"] for d in run],
                            "cells": len(OBS_CELLS), "cell_names": list(OBS_CELLS)},
        "obs_not_run_defects": {"count": len(not_run), "ids": [d["id"] for d in not_run]},
        "obs_run_crash_defects": crash_run,
        "obs_run_semantic_defects": semantic_run,
        "per_channel_defect_recovery_over_run_subset": per_channel,
        "per_channel_defect_recovery_semantic_only": per_channel_semantic_only,
        "per_channel_note": ("Booleans, single seed 42, one run per cell (pilot). Crash defects (C1, C6) are recovered by every "
                             "channel as Rust-failure (O-R sees NO-RETURN). O-P(silent)=yes only for the two crash defects and for "
                             "S13 (CLI boundary, exit status IS the externalised state) - U3 wording: process-output sensitivity is "
                             "determined by the driver's externalization policy; never 'O-P-silent is universally zero'."),
        "family_note": (f"Families record one primary root-cause mechanism per defect, so the counts sum to {len(DEFECTS)}. "
                        "Cross-cutting symptoms may be discussed separately but do not create additional memberships. "
                        "The cJSON non-UTF-8 rejection (S9) is grouped with the four C2SaferRust byte-string restrictions "
                        "because all five narrow a C byte-string domain to valid UTF-8."),
        "attr_coverage": ATTR_NOTE,
        "align_coverage": ALIGN_NOTE,
        "provenance_counts": {},
        "candidates": [c["id"] for c in CANDIDATES],
    }
    for d in DEFECTS:
        summary["provenance_counts"].setdefault(d["provenance"], []).append(d["id"])
    summary["provenance_counts"] = {k: {"count": len(v), "ids": v} for k, v in summary["provenance_counts"].items()}
    return {
        "manifest": "canonical defect-level manifest (CONTRACT ADDENDUM v4 P1-P6, v5 U1-U5)",
        "generated_from": ["results/rq4_effectiveness/bugs_detailed.md", "results/rq4_effectiveness/translation_matrix.md",
                           "results/ablations/observation/obs_qsort_ptrtrans/result.json", "results/ablations/observation/obs_matrix/*/{result.json,RESULT.md}",
                           "results/rq1_matching/align_qsort_ptrtrans/result.json", "results/ablations/attribution/*/result.json"],
        "pilot_commits": {"obs_qsort_ptrtrans + align + attr": "dda70a4", "obs_matrix": "747f5f0"},
        "channels": {"O-R": "return value only", "O-P(silent)": "stdout+exit status, silent-consumer driver",
                     "O-P(print)": "stdout+exit status, printing driver", "O-S": "return + designated output memory + designated globals",
                     "O-F": "O-S union O-P (shipped oracle)"},
        "obs_cells": OBS_CELLS,
        "defects": [dict(d, status="confirmed (results/rq4_effectiveness/bugs_detailed.md)") for d in DEFECTS],
        "candidates": CANDIDATES,
        "summary": summary,
    }


def md(m):
    L = []
    L.append(f"# Defect manifest (canonical, defect-level) - {len(DEFECTS)} confirmed defects "
             f"+ {len(CANDIDATES)} candidates\n")
    L.append("Generated by `results/rq4_effectiveness/gen_defect_manifest.py` from `results/rq4_effectiveness/defect_manifest.json`. "
             "Binding counting rules: CONTRACT ADDENDUM v4 (P1-P6) and v5 (U1-U5).\n")
    L.append("**Units.** *Defect* = root cause (one row). *Divergent / valid records* = strength inside ONE OBS pilot cell "
             "(single seed 42); **per-cell, NOT summable** across cells or across defects sharing a cell "
             "(C6+S13 share `tulip_c2saferrust`; S7+S8+S9 share `cjson_ptrtrans`).\n")
    L.append("Channels: O-R return only · O-P(silent)/O-P(print) stdout+exit under silent-consumer / printing driver · "
             "O-S return+output memory+globals · O-F = O-S ∪ O-P. \"not run\" = no OBS pilot cell for that defect. "
             "Crash defects are recorded as Rust-failure on every channel.\n")

    L.append("## Confirmed defects (status: confirmed per results/rq4_effectiveness/bugs_detailed.md; "
             "S14 added 2026-09-03, evidence in results/rq4_effectiveness/bugs/bzip2_c2saferrust_mmed3/)\n")
    hdr = ["id", "library × tool", "root cause", "family", "symptom", "contract boundary",
           "O-R", "O-P(silent)", "O-P(print)", "O-S", "O-F", "divergent / valid records (per cell)",
           "ATTR", "ALIGN", "C-source provenance", "evidence"]
    L.append("| " + " | ".join(hdr) + " |")
    L.append("|" + "---|" * len(hdr))
    for d in m["defects"]:
        row = [d["id"], f'{d["library"]} × {d["tool"]}', d["root_cause"], d["family"], d["symptom"], d["boundary"]]
        row += [d["obs"][c] for c in CHANNELS]
        row += [d["records"], d["attr"], d["align"], f'{d["provenance"]} — {d["provenance_note"]}', d["evidence"]]
        L.append("| " + " | ".join(x.replace("|", "\\|") for x in row) + " |")

    L.append("\n## Candidates (status: candidate — untriaged, triage in progress; OUTSIDE every main count, U2)\n")
    hdr2 = ["id", "library × tool", "description", "records (per cell)", "O-R", "O-P(silent)", "O-P(print)", "O-S", "O-F", "evidence"]
    L.append("| " + " | ".join(hdr2) + " |")
    L.append("|" + "---|" * len(hdr2))
    for c in m["candidates"]:
        row = [c["id"], f'{c["library"]} × {c["tool"]}', c["description"], c["records"]] + [c["obs"][x] for x in CHANNELS] + [c["evidence"]]
        L.append("| " + " | ".join(x.replace("|", "\\|") for x in row) + " |")

    L.append("\n## OBS pilot cells (per-cell record counts; single seed 42, one run each)\n")
    L.append("| cell | defects | valid records | C-UB excl. | Rust-failure | semantic-diff | O-R | O-P(silent) | O-P(print) | O-S | O-F | C oracle | path |")
    L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for name, c in m["obs_cells"].items():
        cd = c["cell_divergences"]; cl = c["classification"]
        L.append(f'| {name} | {", ".join(c["defects"])} | {c["valid_records"]} | {c["c_ub_excluded"]} | {cl["Rust-failure"]} | {cl["semantic-difference"]} | '
                 + " | ".join(str(cd[x]) for x in CHANNELS) + f' | {c["c_oracle"]} | {c["path"]} |')
    L.append("\nCell notes:")
    for name, c in m["obs_cells"].items():
        L.append(f"- **{name}**: {c['corpus_note']}")

    s = m["summary"]
    L.append("\n## Summary block\n")
    L.append(f"- Defects: **{s['defects_total']}** ({s['crash']} crash / {s['semantic']} semantic). Candidates (excluded): {', '.join(s['candidates'])}.")
    L.append("- Defects per mechanism family:")
    for k, v in s["defects_per_family"].items():
        L.append(f"  - {k}: **{v['count']}** ({', '.join(v['ids'])})")
    L.append(f"  - Note: {s['family_note']}")
    L.append(f"- OBS-run defects: **{s['obs_run_defects']['count']} of {s['defects_total']}** ({', '.join(s['obs_run_defects']['ids'])}) across "
             f"{s['obs_run_defects']['cells']} cells; not run: **{s['obs_not_run_defects']['count']}** ({', '.join(s['obs_not_run_defects']['ids'])}).")
    L.append(f"  - run subset = {len(s['obs_run_crash_defects'])} crash ({', '.join(s['obs_run_crash_defects'])}) + "
             f"{len(s['obs_run_semantic_defects'])} semantic ({', '.join(s['obs_run_semantic_defects'])}).")
    L.append("- Per-channel DEFECT-level recovery over the run subset (booleans; the numbers a main table may use):")
    for ch, v in s["per_channel_defect_recovery_over_run_subset"].items():
        miss = f" — misses {', '.join(v['missed'])}" if v["missed"] else ""
        L.append(f"  - **{ch} recovers {v['recovered']} of {v['of_run']}** run defects ({', '.join(v['defects'])}){miss}")
    L.append("- Same, semantic defects only (crash defects are Rust-failure on every channel):")
    for ch, v in s["per_channel_defect_recovery_semantic_only"].items():
        L.append(f"  - {ch}: {v['recovered']} of {v['of_run_semantic']} ({', '.join(v['defects']) or 'none'})")
    L.append(f"- {s['per_channel_note']}")
    L.append("- **Record counts are per cell and are not summable** (U1): e.g. 71/104 (qsort×PtrTrans) and 528/529 (bzip2×Laertes) "
             "describe different corpora; S7/S8/S9 split ONE cell's 139 records (1+30+80 confirmed = 111, +2 candidate); "
             "C6/S13 split ONE cell's 201 records (1 + 68, +9 candidate).")
    L.append(f"- ATTR: {s['attr_coverage']}")
    L.append(f"- ALIGN: {s['align_coverage']}")
    L.append("- C-source provenance (U4; 'unknown' where the notes do not pin the version):")
    for k, v in s["provenance_counts"].items():
        L.append(f"  - {k}: {v['count']} ({', '.join(v['ids'])})")
    return "\n".join(L) + "\n"


if __name__ == "__main__":
    if "--build" in sys.argv or not os.path.exists(JSON_PATH):
        with open(JSON_PATH, "w") as f:
            json.dump(build(), f, indent=1, ensure_ascii=False)
        print("wrote", JSON_PATH)
    with open(JSON_PATH) as f:
        m = json.load(f)
    with open(MD_PATH, "w") as f:
        f.write(md(m))
    print("wrote", MD_PATH)

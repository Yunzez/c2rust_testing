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
         provenance_note="CROWN consumed tools/frameworks/crown/c-code/bzip2 (bzip2 1.0.8); archive oracle_comp.c is built from it. LINE-LEVEL ROOT CAUSE FOUND 2026-09-05 by the RQ4 plan pipeline: in fallbackSort, c2rust's `let ref mut fresh0 = *bhtab.offset((ftab[i] >> 5) as isize); *fresh0 |= 1 << ..` (the SET_BH macro, bhtab[zz>>5] |= 1<<(zz&31)) was rewritten by CROWN to the plain store `*bhtab.offset((ftab[i] >> 5) as isize) = 1 << ..` at all three SET_BH sites (CLEAR_BH's `&=` survived); setting one bit now clears the rest of the word. CROWN removed every one of c2rust's 15 `let ref mut fresh` reborrows in this crate. The heap overrun and the free() abort are downstream of the corrupted sort. Family was assigned by behavioral attribution before this diff existed; a computation-substitution reading (|= -> =) is now supportable and shares its root cause with S10",
         evidence="results/rq4_effectiveness/bugs/bzip2_crown/; results/rq4_effectiveness/translation_matrix.md fn 10; results/rq4_effectiveness/bugs_detailed.md:282. RQ4 plan pipeline 2026-09-05 (results/rq3_coverage/bzip2/crown/): fallbackSort 32/32 confirmed_divergence on `array bhtab` with the C side in contract (layout-independent), 5109 wild-address ASan hits attached as its memory face; BZ2_bzBuffToBuffCompress 4 confirmed_termination = generateMTFValues `(*s).mtfFreq[(j+1) as usize] += 1` index out of bounds with no sanitizer, downstream of the same corruption"),
    dict(id="C8", library="bzip2", tool="Laertes", kind="crash",
         root_cause="static incs[14] (shell-sort increments) initializer moved into laertes_init_incs(), which nothing calls: the table is all zero, so `while (incs[hp] < bigN) hp++` never stops",
         family="initialization loss or corruption",
         symptom="index out of bounds in mainSimpleSort on any block that reaches the shell sort, i.e. every real compress; the shipped acceptance suite's three compress commands produce empty output (process dies)",
         boundary="function (BZ2_bzBuffToBuffCompress) / crash; also reached through mainQSort3 and mainSimpleSort",
         obs_cell=None, obs=NOT_RUN, records="n/a (found by the RQ4 plan pipeline, not an OBS cell)",
         attr="no", align="no",
         provenance="unknown",
         provenance_note="MECHANISM exact-source: `static mut incs: [c_int; 14] = [0; 14]` + `laertes_init_incs()` defined once and never called, read directly from the Laertes crate (bzip2_laertes/blocksort.rs:906); the severed-init scanner had flagged this static (results/rq4_effectiveness/severed_init_scan.json: incs, poisoned=true, consumer_refs=3). REFERENCE C: the harness compiles CROWN's copy of the Laertes benchmark (tools/frameworks/crown/c-code/bzip2, 1.0.8); whether Laertes consumed a byte-identical copy is not recorded (same status as C4)",
         evidence="results/rq3_coverage/bzip2/laertes/ (RQ4 cell, plan pipeline, 3600 s, seed 42): confirm/BZ2_bzBuffToBuffCompress_verdicts 506/506 confirmed (233 confirmed_termination: C-only ASan+UBSan clean, Rust panics with no sanitizer, panic marker at lib.rs:943 mainSimpleSort), confirm_sample/mainQSort3 53 + mainSimpleSort 18 confirmed_termination at the same site; results/rq3_coverage/bzip2/tests_side_results.json (compress 0/3); scanner prediction: results/rq4_effectiveness/severed_init_scan.json"),
    dict(id="C9", library="lil", tool="C2SaferRust", kind="crash",
         root_cause="register_stdcmds passes Rust string literals as C strings without a NUL terminator (`\"reflect\".as_ptr() as *const i8`); C passes NUL-terminated literals. hm_get's CStr::from_ptr / strlen then reads past the literal",
         family="byte-string domain narrowing",
         symptom="lil_new() cannot complete: every interpreter construction crashes (ASan: global-buffer-overflow in strlen under hm_get; without a sanitizer: `NonNull::new_unchecked requires that the pointer is non-null` precondition panic). This is the root of the CRASH-ALL observed for lil x C2SaferRust in the reach census; 23 of 47 planned boundaries never execute",
         boundary="function (lil_new, via register_stdcmds -> lil_register -> add_func -> find_cmd -> hm_get) / crash; 18 further boundaries blocked downstream (they take the produced lil_t)",
         obs_cell=None, obs=NOT_RUN, records="n/a (found by the RQ4 plan pipeline; producer bridge lil_new)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/lil_c2saferrust/translated/lil_c2saferrust.rs:4725 (register_stdcmds) and :491-492 (hm_get: CStr::from_ptr(key)); C from lil.c:220 (hm_get) and register_stdcmds' lil_register(lil, \"reflect\", fnc_reflect)",
         evidence="results/rq3_coverage/lil/c2saferrust/ (RQ4 cell, 3600 s, seed 42): confirm_sample 57 confirmed_termination across 19 boundaries, all one site (rust_no_sanitizer: NonNull precondition panic in core ptr/unique.rs reached from lil_new; rust_only ASan: global-buffer-overflow in __interceptor_strlen <- hm_get <- add_func <- lil_new); c_only normal on every one. ONE defect: the 19 boundaries share the producer's crash (18 downstream-blocked)"),
    dict(id="C10", library="lil", tool="CROWN", kind="crash",
         root_cause="lil_subst_to_list dropped C's fallback `if (!words) words = lil_alloc_list();` after substitute(): when substitution fails (an unterminated quote in the argument) it returns NULL where C returns an empty list",
         family="null/empty conflation",
         symptom="fnc_enveval (the `enveval` builtin) calls lil_list_size(invars) on the NULL: null pointer dereference (debug-assertion panic `null pointer dereference occurred` at lil.rs:404 without a sanitizer; SEGV with). Any script `enveval <malformed> ...` crashes the interpreter that C runs to completion",
         boundary="function (lil_parse -> fnc_enveval -> lil_subst_to_list -> lil_list_size) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (found by the RQ4 plan pipeline on a cell E1 CERTIFIED: CROWN lil, 111,043-record expr/variable/list/string corpus, 0 diffs -- that corpus never called enveval with a malformed list)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/lil_crown/translated/lil_crown.rs:983-988 (lil_subst_to_list: `words = substitute(..); ..; return words;` with no NULL fallback) vs lil.c:919-920 (`words = substitute(lil); if (!words) words = lil_alloc_list();`); crash site lil_crown.rs:420 = lil.rs:404 (lil_list_size: `return (*list).c;`)",
         evidence="results/rq3_coverage/lil/crown/ (RQ4 cell, 3600 s, seed 42): confirm_sample on lil_parse: 67 confirmed_termination (rust_no_sanitizer panic `null pointer dereference occurred` at harness lib.rs:420), 2 timeouts not confirmed; c_only normal (ASan+UBSan) on every one; confirmed_inputs/lil_parse/ (scripts beginning `enveval '(`...)"),
    dict(id="S15", library="tulipindicators", tool="C2SaferRust", kind="semantic",
         root_cause="ti_adx_start casts the options POINTER to an integer instead of loading options[0]: `(options.offset(0) as i32 - 1) * 2` for C's `((int)options[0]-1) * 2`; the indicator's start offset becomes the low 32 bits of a heap address",
         family="semantic computation substitution",
         symptom="wrong return value on every valid input (5/5 sampled confirmed_divergence), and `attempt to multiply with overflow` panics when the address bits are large (4 of 7 sampled rows trap with no sanitizer; the other 3 panicked only in the ASan build and were re-classified instrument_only on 2026-09-09); C is in contract on all of them. The other 36 `_start` functions that read options dereference correctly (`*options.offset(0)`), checked by a deterministic probe (options[0] = 5.0, combined replay: 18 of 18 normal, only ti_adx_start diverges)",
         boundary="function (ti_adx_start) / semantic + crash; ti_adx and every caller of ti_adx_start inherit the wrong start offset",
         obs_cell=None, obs=NOT_RUN, records="n/a (found by the RQ4 plan pipeline; buffer-table cell, 3600 s, seed 42)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/tulip_c2saferrust/translated/tulip_c2saferrust.rs:391-395 (laertes_benchmarks/tulipindicators_WIP/indicators/adx.rs), C from tulip 0.8.4 indicators/adx.c:28-30. E1's tulip x C2SaferRust evidence (C6, S13, the 150k-record indicator-value certificate) compared indicator outputs through the sample driver and never called ti_adx_start",
         evidence="results/rq3_coverage/tulip/c2saferrust/ (RQ4 cell, 2026-09-07): replay of the coverage corpus 6 divergence + 3 panic; confirm_sample/ti_adx_start_verdicts: 5 confirmed_divergence (rung 2, return value) + 4 confirmed_termination (no-sanitizer panic `attempt to multiply with overflow` at lib.rs:393; 3 further rows re-classified instrument_only 2026-09-09); confirmed_inputs/ti_adx_start/; direct probe recorded in tulip/c2saferrust/RUN.md section 7"),
    dict(id="C11", library="tulipindicators", tool="Laertes", kind="crash",
         root_cause="the static table `ti_indicators[105]` is rendered as 105 `ti_indicator_info::new()` (every `name` NULL) with its initialiser moved into a `laertes_init_*` function nothing calls -- the severed-init pattern the scanner had flagged for this very table; ti_find_indicator's binary search then strcmp()s a NULL name",
         family="initialization loss or corruption",
         symptom="ti_find_indicator dereferences the zero page on every input (SEGV in strcmp; deterministic, with and without a sanitizer), so no indicator can be looked up by name; C returns normally",
         boundary="function (ti_find_indicator) / crash; every caller of the lookup inherits it",
         obs_cell=None, obs=NOT_RUN, records="n/a (found by the RQ4 plan pipeline: preflight stop, then the campaign; 2026-09-07)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/tulip_laertes/translated/tulip_laertes.rs:10464 (the all-default table) and :19587-19602 (ti_find_indicator); C from tulip 0.8.4 indicators_index.c (ti_indicators[] with string names, ti_find_indicator). Scanner: results/rq4_effectiveness/severed_init_law.md flags tulipindicators::ti_indicators; E3 called the initialiser via Once to run at all. E1's Laertes tulip certificate (11 arithmetic indicators, 150k records, 0 diffs) never called the lookup",
         evidence="results/rq3_coverage/tulip/laertes/ (RQ4 cell, 3600 s, seed 42): preflight/preflight.json (C ok, Rust crash on the empty input; 129/130 jobs crashed); replay 1 signal; confirm_sample/ti_find_indicator_verdicts: 2 confirmed_termination under the zero-page rule (ASan: address points to the zero page; no-sanitizer replay SIGSEGV); confirmed_inputs/ti_find_indicator/; RUN.md section 7"),
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
         evidence="results/rq4_effectiveness/bugs/crc32_c2saferrust/; results/rq4_effectiveness/translation_matrix.md fn 9; results/rq4_effectiveness/bugs_detailed.md:322; RQ4 RE-FIND (plan pipeline, 3600 s, seed 42, 2026-09-08): results/rq3_coverage/optipng/c2saferrust/ confirm_sample crc32 26/26 confirmed_divergence (crc32 passes its whole buffer to crc32_z; return value), C clean under ASan+UBSan"),
    dict(id="S2", library="optipng (zlib)", tool="C2SaferRust", kind="semantic",
         root_cause="adler32_z: NULL guard became length test; empty chunk resets",
         family="null/empty conflation",
         symptom="wrong Adler-32 on empty chunk (plus extra non-empty miscompile); no crash",
         boundary="function (adler32_z return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="catalog appendix (S1-S4 checksum C sides not archived); base.rs quoted as reference",
         evidence="results/rq4_effectiveness/bugs/crc32_c2saferrust/adler32_corroboration/; results/rq4_effectiveness/translation_matrix.md fn 9; results/rq4_effectiveness/bugs_detailed.md:360; RQ4 RE-FIND (plan pipeline, 3600 s, seed 42, 2026-09-08): results/rq3_coverage/optipng/c2saferrust/ — the same adler32_z rewrite (its 16-byte block loop builds a slice of the REMAINING length after `len -= 16` and reads 8 entries of it: index-out-of-bounds when fewer than 8 remain, wrong sums otherwise) confirmed through compress2 (169 confirmed_termination + 99 confirmed_divergence on the output buffer), compress (1 + 1) and adler32 (1 + 1); root cause read at optipng_c2saferrust.rs:62225-62260, panic site :62239"),
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
         evidence=("RQ4 RE-FIND (plan pipeline, 3600 s, seed 42, 2026-09-04): results/rq3_coverage/bzip2/laertes/ — confirmed on both its boundaries (confirm_sample: 299 confirmed_divergence), C clean under ASan+UBSan; bzip2/SUMMARY.md §3; results/rq4_effectiveness/bugs/bzip2_laertes/; results/rq4_effectiveness/translation_matrix.md fn 14; results/rq4_effectiveness/bugs_detailed.md:388. INDEPENDENT REDISCOVERY 2026-09-03: the schema-driven differential harness generated for BZ2_bzBuffToBuffCompress (generator 0.6, with no knowledge of this defect) reported 58,327 output-buffer divergence events in a 3600 s campaign, with the same signature as the archived cell (written length equal, bytes differ); results/rq3_coverage/bzip2/")),
    dict(id="S4", library="optipng (zlib)", tool="Laertes", kind="semantic",
         root_cause="laertes_init_crc_table never called; no DYNAMIC_CRC_TABLE rebuild",
         family="initialization loss or corruption",
         symptom="crc32 returns degenerate value (crc32('a') ff000000 vs e8b7be43); 98.49% wrong",
         boundary="function (crc32_z return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="base-c2rust-as-reference",
         provenance_note="catalog appendix; archive oracle_zlib.c uses the canonical SYSTEM zlib (fn 8), not the vendored source",
         evidence="results/rq4_effectiveness/bugs/optipng_laertes/; results/rq4_effectiveness/translation_matrix.md fn 8; results/rq4_effectiveness/bugs_detailed.md:426; RQ4 RE-FIND (plan pipeline, 3600 s, seed 42, rerun 2026-09-09): results/rq3_coverage/optipng/laertes/ confirm_sample crc32 23/23 and crc32_z 23/23 confirmed_divergence; compress 13/13 and compress2 15/16 confirmed_divergence are the same severed-init law on deflate's tables (configuration_table, _length_code, _dist_code, base_length), recorded here, not as further entries"),
    dict(id="S5", library="genann", tool="SACTOR", kind="semantic",
         root_cause="mutable lookup[] lifted to immutable static; init writes are Rust UB",
         family="initialization loss or corruption",
         symptom="release: all-zero network output on 100% of inputs, exit 0; debug: SIGSEGV",
         boundary="global (lookup table) -> function output (genann_run)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="SACTOR consumed the in-repo genann.c (renamed genann_lib.c; compile_commands.json archived); driver.c in archive",
         evidence="RQ4 RE-FIND (plan pipeline with the producer bridge, 2026-09-05): results/rq3_coverage/genann/sactor/ — 17/21 corpus inputs diverge on genann_act_sigmoid_cached, genann_act_hidden_indirect, genann_act_output_indirect; 51/51 confirmed_divergence with C clean; none of the three boundaries is reachable without the bridge; genann/SUMMARY.md §3; results/rq4_effectiveness/bugs/genann_sactor/; results/rq4_effectiveness/translation_matrix.md fn 32; results/rq4_effectiveness/bugs_detailed.md:464"),
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
         evidence="results/rq4_effectiveness/bugs/qsort_ptrtrans/; results/rq4_effectiveness/translation_matrix.md fn 29; results/rq4_effectiveness/bugs_detailed.md:533; RQ4 RE-FIND (plan pipeline, 3600 s, seed 42, 2026-09-07): results/rq3_coverage/qsort/ptrtrans/ confirm_sample partition 31/31 + quickSort 26/26 confirmed_divergence, C clean under ASan+UBSan; qsort/SUMMARY.md §2"),
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
         provenance_note="as C7 (crown/c-code/bzip2 1.0.8 = CROWN's input). Line-level root cause now archived under C7 (fallbackSort SET_BH `|=` rewritten to `=`, 2026-09-05): the structurally invalid stream is the downstream face of the same corrupted sort, so S10 and C7 share one root cause; RQ4 cell 2026-09-05: BZ2_bzBuffToBuffCompress 97/97 replay divergences confirmed (destLen + output stream), results/rq3_coverage/bzip2/crown/",
         evidence=("RQ4 RE-FIND (plan pipeline, 2026-09-04): results/rq3_coverage/bzip2/crown/ — 242 confirmed_divergence in the sample, root cause found at line level (SET_BH `|=` → `=`); bzip2/SUMMARY.md §3, §5; results/rq4_effectiveness/bugs/bzip2_crown/ (crown_compress_driver.rs, crown_corrupt_A4096.bz2); results/rq4_effectiveness/translation_matrix.md fn 10; results/rq4_effectiveness/bugs_detailed.md:716. INDEPENDENT REDISCOVERY 2026-09-03: the generated differential harness for the CROWN compress boundary (shared with C7) reported 75,872 written-length divergence events in a 3600 s campaign; results/rq3_coverage/bzip2/")),
    dict(id="S11", library="bzip2", tool="CROWN", kind="semantic",
         root_cause="decompress small=0 fast-path state machine broken by lift",
         family="ownership-state corruption",
         symptom="BZ_DATA_ERROR (-4) on streams canonical bunzip2 accepts; small=1 path works",
         boundary="function (BZ2_bzBuffToBuffDecompress return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="as C7",
         evidence="RQ4 RE-FIND (plan pipeline, 2026-09-04): results/rq3_coverage/bzip2/crown/ (decompression boundaries, confirm_sample); bzip2/SUMMARY.md §3; results/rq4_effectiveness/bugs/bzip2_crown/crown_decompress_driver.rs; results/rq4_effectiveness/translation_matrix.md fn 10; results/rq4_effectiveness/bugs_detailed.md:745"),
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
         evidence=("RQ4 RE-FIND (plan pipeline, 2026-09-04): results/rq3_coverage/bzip2/c2saferrust/ — 3 confirmed_divergence on the mmed3 boundary in the replay of the coverage corpus; bzip2/SUMMARY.md §3; results/rq4_effectiveness/bugs/bzip2_c2saferrust_mmed3/ (exhaustive census, both bodies, the "
                   "single call site, the generated harness and a saved divergence input). DOWNSTREAM SCOPE: the "
                   "established effect is pivot selection and potential performance degradation only; an effect on "
                   "bzip2 compressed output is NOT claimed and was not demonstrated either way (the artifact aborts "
                   "earlier in sendMTFValues, CAND-3)")),
    # --- RQ4 ten-library campaign, 2026-09-09 (user rule: new entries join an EXISTING family) ---
    dict(id="C12", library="urlparser", tool="Laertes", kind="crash",
         root_cause="`char *URL_SCHEMES[]` (url.h:54, 177 scheme strings) is emitted as 177 NULL pointers with its initializer moved into `laertes_init_URL_SCHEMES()` (urlparser_laertes.rs:99-100), which nothing calls (0 call sites); `url_is_protocol` (rs:449-462) then runs `strcmp(URL_SCHEMES[i], str)` on NULL",
         family="initialization loss or corruption",
         symptom="every call of url_is_protocol dereferences the zero page: SIGSEGV with and without a sanitizer (3/3 confirmed_termination); url_parse, url_get_protocol, url_get_auth, url_data_inspect and url_free fail identically through it (3/3 each) and are the same root cause, not further entries",
         boundary="function (url_is_protocol) / crash; downstream url_parse -> url_is_protocol",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/urlparser_laertes/translated/urlparser_laertes.rs:99-100 (the NULL table + severed init) and :449-462 (url_is_protocol); C from the pair's source/url.h:54 and :326-336 (header-only library: the E1 severed-init scanner had filed url.h as a fixture, a false negative)",
         evidence="results/rq3_coverage/urlparser/laertes/ (RQ4 cell, 3600 s, seed 42, rerun 2026-09-09): confirm_sample url_is_protocol 3/3 confirmed_termination (c_only normal; rust_no_sanitizer SIGSEGV on the zero page), url_parse / url_get_protocol / url_get_auth / url_data_inspect / url_free 3/3 each; benchmark/pairs/rq4/urlparser_laertes/preflight_accept.txt (Rust crash-all on the probe, C clean)"),
    dict(id="C13", library="optipng", tool="C2SaferRust", kind="crash",
         root_cause="`free(ptr)` in opng_free (optim.c:390-396) rewritten as `drop(Box::from_raw(ptr as *mut _))` (optipng_c2saferrust.rs:54539-54546): C's free(NULL) is a defined no-op, Box::from_raw(NULL) is not (the debug build traps). Read from the source but NOT covered by the evidence: a non-NULL pointer would be released by Rust's allocator although libpng obtained it from malloc",
         family="null/empty conflation",
         symptom="opng_free(NULL) panics where C returns (3/3 confirmed_termination, rust_no_sanitizer panic, c_only normal). The evidence covers the NULL contract only",
         boundary="function (opng_free) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:54539-54546; C from the pair's source/optipng/optim.c:390-396",
         evidence="results/rq3_coverage/optipng/c2saferrust/ (RQ4 cell, 3600 s, seed 42, 2026-09-08): confirm_sample opng_free 3/3 confirmed_termination, one cluster; preflight_accept.txt lists opng_free as Rust crash-all with C clean"),
    dict(id="S16", library="optipng", tool="C2SaferRust", kind="semantic",
         root_cause="opng_strcasecmp's byte-wise `tolower(*str1++) - tolower(*str2++)` loop (optipng.c:189-201) is rewritten over `CStr::to_string_lossy()` + `chars()` + `to_ascii_lowercase()` (optipng_c2saferrust.rs:57252-57275): every non-UTF-8 byte becomes U+FFFD and every multi-byte sequence compares as one code point",
         family="byte-string domain narrowing",
         symptom="wrong return value (sign and magnitude) on any pair of strings containing a byte >= 0x80; 55/55 sampled candidates confirmed_divergence with C clean under ASan+UBSan; no crash",
         boundary="function (opng_strcasecmp return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:57252-57275; C from the pair's source/optipng/optipng.c:189-201. Same mechanism as the four C2SaferRust byte-string restrictions and cJSON S9",
         evidence="results/rq3_coverage/optipng/c2saferrust/ (RQ4 cell, 3600 s, seed 42, 2026-09-08): confirm_sample opng_strcasecmp 55/55 confirmed_divergence (ladder phase 4, return value), one cluster; divergences/opng_strcasecmp/"),
    dict(id="S17", library="optipng", tool="Laertes", kind="semantic",
         root_cause="`static const char bak_extname[] = \".bak\"` in opng_path_make_backup (ioutil.c:410) is emitted as five zero bytes with the initializer moved into `laertes_init_bak_extname()` (optipng_laertes.rs:60151-60156), never called",
         family="initialization loss or corruption",
         symptom="`strcat(buffer, bak_extname)` appends the empty string: the backup path returned equals the input path, no `.bak` (19/19 confirmed_divergence on the output buffer; return value unchanged; no crash)",
         boundary="output buffer (buffer) - return value unchanged",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_laertes/translated/optipng_laertes.rs:60151-60162; C from the pair's source/optipng/ioutil.c:405-418. Same law as S3/S4 (277 laertes_init_* defined, 0 called across the ten crates; this crate 178/0)",
         evidence="results/rq3_coverage/optipng/laertes/ (RQ4 cell, 3600 s, seed 42, rerun 2026-09-09): confirm_sample opng_path_make_backup 19/19 confirmed_divergence (ladder phase 4, output buffer), one cluster"),
    dict(id="S18", library="optipng (zlib)", tool="Laertes", kind="semantic",
         root_cause="inflate's `static const unsigned short order[19]` (inflate.c:642, the code-length permutation) is emitted as 19 zeros with `laertes_init_order()` (optipng_laertes.rs:88687-88688) never called. In the TABLE state `state->lens[order[state->have++]] = ...` (inflate.c:941/945 -> rs:89122) writes every code-length code into lens[0] and leaves lens[1..18] unwritten, so inflate_table's `count[lens[sym]]++` (inftrees.c:110 -> rs:90608) indexes a 16-entry array with whatever the allocation held",
         family="initialization loss or corruption",
         symptom="inflate_table reads `lens[1..18]` that inflate never wrote (only lens[0] is written when order[] is all zero): the result depends on the heap contents. In the ASan build the fill pattern 0xBEBE = 48830 indexes a 16-entry array -> `index out of bounds` panic (200 sampled rows); with NO sanitizer the same inputs return normally with a wrong result (all 200 no-sanitizer replays normal; 4 sampled confirmed_divergence on the written length destLen). Termination is therefore instrument-dependent and the entry is semantic, not crash; C writes all 19 entries and returns normally",
         boundary="function (uncompress: return value and destLen; an uninitialised read whose visible symptom depends on the heap contents)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_laertes/translated/optipng_laertes.rs:88687-88688 (order), :89122 (lens fill), :90608 (count index, inflate_table def at :90403); C from the pair's source/zlib/inflate.c:642, :941, :945 and zlib/inftrees.c:110. The vendored zlib is the one the translation consumed (pair PROVENANCE.json)",
         evidence="results/rq3_coverage/optipng/laertes/ (RQ4 cell, 3600 s, seed 42, rerun 2026-09-09): confirm_sample uncompress 204/504 = 4 confirmed_divergence (destLen) + 200 rows that panic only in the ASan build (re-classified instrument_only 2026-09-09 after the no-sanitizer replay returned normally on every one); RUST_BACKTRACE replay of candidates_sample/uncompress/crash-0101020c... under ASan gives inflate_table <- inflate <- uncompress2 <- uncompress with index 48830 = 0xBEBE; UNSANITIZED COMBINED replay 2026-09-09 (no sanitizer on either side, both sides running, confirm_sample/unsanitized_combined_uncompress.json): the 4 archived divergence inputs diverge again on the return value, the 20 archived sanitizer-only inputs in the sample agree with C (both sides fail the stream the same way) -- the value difference is real and instrument-free, the panic was the instrument's"),
    dict(id="S19", library="optipng (zlib)", tool="C2SaferRust", kind="semantic",
         root_cause="send_bits' flush path `s->bi_buf |= (ush)val << s->bi_valid` (trees.c:211-218; C promotes the ush to int, so a shift by bi_valid == 16 keeps the bits above the 16-bit buffer and the `|=` into ush drops them) is emitted as `(*s).bi_buf |= (val as u16) << (*s).bi_valid` (optipng_c2saferrust.rs:85968 in compress_block, :85195 the same expansion): a u16 shifted by 16. bi_valid reaches 16 whenever the non-flush path fills the buffer exactly",
         family="semantic computation substitution",
         symptom="`attempt to shift left with overflow` panic under overflow checks (compress 200/202, compress2 28 sampled rows, all trapping with no sanitizer; C clean under UBSan's shift check). In a release build Rust masks the shift amount (16 -> 0) and ORs val into the short being flushed: a corrupted deflate stream instead of a panic. Either way the translation differs from C",
         boundary="function (compress / compress2 output buffer) / crash under overflow checks",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:85960-85972 (compress_block); C from the pair's source/zlib/trees.c:211-219 (send_bits)",
         evidence="results/rq3_coverage/optipng/c2saferrust/ (RQ4 cell, 3600 s, seed 42, 2026-09-08): confirm_sample compress 200 + compress2 28 confirmed_termination at harness lib.rs:85968 (`attempt to shift left with overflow`), no-sanitizer replay panics, c_only normal; TRIAGE.md"),
    dict(id="S20", library="optipng (zlib)", tool="C2SaferRust", kind="semantic",
         root_cause="inflate's MATCH state `copy = out - left; if (state->offset > copy)` (inflate.c:1148-1150, 'copy from window') is emitted as `if state as *const _ as usize > copy as usize` (optipng_c2saferrust.rs:81096): the STATE POINTER's address is compared with the byte count instead of the match distance, so the window branch is taken for almost every match",
         family="semantic computation substitution",
         symptom="valid streams are rejected with `invalid distance too far back` (state.mode = BAD), so uncompress returns a different code and written length (169/204 sampled confirmed_divergence on destLen); when the wrapped copy count is 0 and no window exists the byte loop dereferences NULL (4 sampled rows, null pointer dereference with no sanitizer). C returns normally",
         boundary="function (uncompress: return value and destLen)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:81095-81147 (inflate, MATCH); C from the pair's source/zlib/inflate.c:1146-1165",
         evidence="results/rq3_coverage/optipng/c2saferrust/ (RQ4 cell, 3600 s, seed 42, 2026-09-08): confirm_sample uncompress 169 confirmed_divergence (ladder phase 4, written length destLen) + 4 confirmed_termination (null pointer dereference at harness lib.rs:81147); TRIAGE.md; UNSANITIZED COMBINED replay 2026-09-09 (confirm_sample/unsanitized_combined_uncompress.json): of 88 archived destLen divergences in the sample 76 diverge again with no sanitizer on either side, 6 agree, 6 panic (C15's NULL slice on the same path); 5 archived not_reproducible inputs diverge unsanitized"),
    dict(id="C15", library="optipng (zlib)", tool="C2SaferRust", kind="crash",
         root_cause="inflate_fast's 'copy direct from output' branch (inffast.c: `from = out - dist; do { *out++ = *from++; ... }`) is emitted without the assignment `from = out - dist`: `from` keeps its declaration value NULL (optipng_c2saferrust.rs:75492) and the branch builds `slice::from_raw_parts(from, len)` / `from_raw_parts_mut(out, len)` over it (:75626-75636)",
         family="initialization loss or corruption",
         symptom="`unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null` panic with no sanitizer (48/204 sampled rows) — in a release build a read through NULL; C copies the match and returns normally. A further 31 sampled rows failed only in the ASan build (the overlapping `copy_from_slice` memcpy the interceptor rejects) and are instrument_only",
         boundary="function (uncompress -> inflate -> inflate_fast) / crash",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:75453-75640 (inflate_fast: no `from =` assignment between the declaration and the copy); C from the pair's source/zlib/inffast.c 'copy direct from output'",
         evidence="results/rq3_coverage/optipng/c2saferrust/ (RQ4 cell, 3600 s, seed 42, 2026-09-08): confirm_sample uncompress 48 confirmed_termination at harness lib.rs:75626 (no-sanitizer panic, c_only normal); TRIAGE.md; UNSANITIZED COMBINED replay 2026-09-09: 4 of the 6 sampled termination inputs panic again with both sides running and no sanitizer, 2 return"),
    dict(id="S21", library="optipng (zlib)", tool="C2SaferRust", kind="semantic",
         root_cause="crc32_combine_ (crc32.c:344-420) is rewritten with two operand errors: gf2_matrix_square computes `square[n] = gf2_matrix_times(square, mat[n])` — the output array as the matrix — instead of `gf2_matrix_times(mat, mat[n])` (optipng_c2saferrust.rs:64895-64899), and the zeros-operator loop applies `odd` where C applies `even` and vice versa (:64929-64950)",
         family="semantic computation substitution",
         symptom="wrong combined CRC for every len2 > 0 with a 32-bit crc1 (crc32_combine 23/23 and crc32_combine64 23/23 sampled confirmed_divergence on the return value); no crash. The other 201 sampled inputs per boundary overflow the 32-entry matrix on the C side (crc1 above 32 bits, a stack-buffer-overflow in C's own gf2_matrix_times) and are ub_associated",
         boundary="function (crc32_combine / crc32_combine64 return value)",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:64882-64962; C from the pair's source/zlib/crc32.c:344-420",
         evidence="results/rq3_coverage/optipng/c2saferrust/ (RQ4 cell, 3600 s, seed 42, 2026-09-08): confirm_sample crc32_combine 23 + crc32_combine64 23 confirmed_divergence (ladder phase 4, return value), c_only clean under ASan+UBSan; TRIAGE.md"),
    dict(id="C16", library="optipng (libpng)", tool="C2SaferRust", kind="crash",
         root_cause="optimize_cmf's `--z_cinfo` on `unsigned int` (pngwutil.c:251-289; for CINFO = 0 the decrement wraps, which C defines) is emitted as checked `z_cinfo -= 1` on u32 (optipng_c2saferrust.rs:47840-47869) instead of `wrapping_sub` (which c2rust emits, optipng_c2rust.rs:53918). Profile-dependent: with overflow checks (the crate's debug profile, the campaign's -C debug-assertions) it panics; a release build wraps identically to C",
         family="semantic computation substitution",
         symptom="`attempt to subtract with overflow` panic on any zlib header with CINFO = 0 and data_size <= 128 (200/200 sampled confirmed_termination, no-sanitizer replay panics; C returns). Deterministic probe with the VALID header 08 1d (FCHECK 0x081d % 31 == 0): C -> 88 1a / e8 02 / f8 1d for data_size 1 / 64 / 100, Rust with overflow checks PANIC on all, Rust without them identical to C. CINFO = 0 is a legal RFC 1950 header that zlib's inflate accepts (window 256); zlib's own deflate never emits it (windowBits >= 9), so the input is unusual but inside the function's stated contract (`data` is a zlib stream)",
         boundary="function (optimize_cmf) / crash under overflow checks",
         obs_cell=None, obs=NOT_RUN, records="n/a (no OBS cell)",
         attr="no", align="no",
         provenance="exact-source",
         provenance_note="Rust quoted from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:47840-47869; C from the pair's source/libpng/pngwutil.c:251-289; probe scratchpad/cmf_probe (probe.c under UBSan, probe.rs with -C overflow-checks=on/off, nightly-2025-09-01), recorded in TRIAGE.md",
         evidence="results/rq3_coverage/optipng/c2saferrust/ (RQ4 cell, 3600 s, seed 42, 2026-09-08): confirm_sample optimize_cmf 200/500 confirmed_termination at harness lib.rs:47856 (`attempt to subtract with overflow`, no-sanitizer replay panics, c_only normal); TRIAGE.md probe table"),
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
    dict(id="CAND-5", library="cJSON", tool="PtrTrans",
         status="candidate - held out by classification (untranslated function, not a mistranslation)",
         description="every cJSON_Create* returns None: cJSON_New_Item (cjson_ptrtrans.rs:1122) is an unimplemented stub returning None, so no object can be created; C returns a non-NULL object on every input. E1 already counts PtrTrans's cJSON as 24/118 stubs (process failure); this is the first replayable, confirmed evidence of what the stubs do to a caller",
         records="18 / 20 replayed inputs, 18/18 confirmed_divergence (rung 3 nullness; c_only and rust_only normal), 7 boundaries, one site (results/rq3_coverage/cjson/ptrtrans/, RQ4 cell re-run 2026-09-06)",
         obs=chan("n/a", "n/a", "n/a", "yes", "yes"),
         evidence="results/rq3_coverage/cjson/ptrtrans/RUN.md section 7; confirm_sample/*_verdicts; funnel.json"),
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

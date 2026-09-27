#!/usr/bin/env python3
"""Compare two analyzer binaries with the unchanged production matcher policy.

Retrospective regression audit, NOT a new held-out evaluation. Never writes to
frozen result directories. Stages the same artifact for both binaries, preserves
old/new raw outputs, and checks reproduction against archived deployment pairs.
No weight/threshold sweep or per-artifact configuration selection.
If a newly exposed duplicate leaf makes a label ambiguous, precision is the
lower bound and precision_upper includes those unadjudicated predictions.
"""
import argparse
from collections import Counter, defaultdict
from concurrent.futures import ProcessPoolExecutor, as_completed
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools/stu_selector'))
sys.path.insert(0, str(ROOT / 'scripts'))
import matcher
import rq1_name_preserving_full as group_a
import rq1_group_b_scaffold as group_b

ARCHIVE = ROOT / 'results/rq1_matching'


def read(path):
    return json.loads(Path(path).read_text())


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write(path, value):
    Path(path).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def edges(data):
    names = {f['name'] for f in data['functions']}
    return {(e['from'], e['to']) for e in data['raw_edges']
            if e['from'] in names and e['to'] in names}


def score(c, r, truth):
    result = matcher.match(c, r, abstain_eps=.01)
    return evaluate_pairs(result['matched'], r, truth)


def evaluate_pairs(pairs, r, truth):
    predictions = {a: b for a, b, *_ in pairs}
    # Existing labels name leaves. A newly exposed duplicate leaf cannot be
    # disambiguated by that label; do not count a lucky homonym as correct.
    leaves = Counter(f['name'].rsplit('::', 1)[-1] for f in r['functions'])
    correct = {a for a, b in predictions.items()
               if truth.get(a) == b.rsplit('::', 1)[-1] and leaves[b.rsplit('::', 1)[-1]] == 1}
    unadjudicated = {a:b for a,b in predictions.items()
                    if truth.get(a) == b.rsplit('::',1)[-1] and leaves[b.rsplit('::',1)[-1]] > 1}
    renamed = {a for a, b in truth.items() if a != b}
    return dict(truth=len(truth), accepted=len(predictions), correct=len(correct),
                precision=len(correct)/len(predictions) if predictions else None,
                precision_upper=(len(correct)+len(unadjudicated))/len(predictions) if predictions else None,
                recall=len(correct)/len(truth) if truth else None,
                renamed_total=len(renamed), renamed_correct=len(renamed & correct),
                label_ambiguities=sorted(a for a,b in truth.items() if leaves[b] > 1),
                unadjudicated=unadjudicated,
                wrong={a:b for a,b in predictions.items() if a not in correct and a not in unadjudicated},
                pairs=pairs)


def run(job):
    group, case, old_bin, new_bin, output = job
    start = time.monotonic()
    destination = Path(output) / f'{group}__{case}'
    destination.mkdir()
    raw = ARCHIVE / 'raw' / f'group_{group}' / case
    c, archived_r = read(raw/'c_analyzer.json'), read(raw/'rust_analyzer.json')
    if group == 'a':
        lib, tool = case.split('__')
        cfg = group_a.RUST[(lib, tool)]
        truth = {a:b.rsplit('::',1)[-1] for a,b in read(raw/'truth.json')['truth'].items()}
        archived_pairs = read(raw/'matcher_output.json')['deployment']
        meta = read(ARCHIVE/'rows/group_a_full.json')[case]
        fingerprint = meta['fingerprint']
        artifact_hash = group_a.tree_hash(group_a.rust_files(cfg))
    else:
        cfg = group_b.CASES[case]
        lib = cfg['lib']
        sheet = read(ARCHIVE/'annotation'/case/'sheet.json')
        truth = {r['c_function']:r['truth'] for r in sheet['rows']
                 if r['truth'] not in ('NONE','AMBIGUOUS','')
                 and not r['truth'].startswith(('STUB:','SPLIT:','MERGED:'))}
        archived_pairs = sheet['matcher_deploy_raw']
        fingerprint = sheet['fingerprint']
        artifact_paths = [cfg['crate']] if Path(cfg['crate']).is_file() else group_b.rust_files(cfg['crate'])
        artifact_hash = group_b.tree_hash(artifact_paths)
    results = {}
    with tempfile.TemporaryDirectory(prefix='stage-', dir=destination) as staging:
        if group == 'a':
            crate, _ = group_a.stage_crate(cfg, lib, staging)
        else:
            crate = group_b.stage_crate(cfg, staging)
        for variant, binary in [('old', old_bin), ('new', new_bin)]:
            proc = subprocess.run([binary, crate, '--enable-metrics'], text=True,
                                  capture_output=True, timeout=900)
            (destination/f'{variant}.stderr').write_text(proc.stderr)
            if proc.returncode:
                raise RuntimeError(f'{case} {variant} analyzer failed: {proc.stdout[-500:]} {proc.stderr[-500:]}')
            r = json.loads(proc.stdout)
            if group == 'b':
                group_b.drop_rust_fns(r, cfg.get('rust_exclude_fns', set()))
            write(destination/f'{variant}.rust.json', r)
            results[variant] = r
    # Skip expensive identical matcher computations only if the consumed node
    # records and local topology are exactly identical.
    old_r, new_r = results['old'], results['new']
    same_features = old_r['functions'] == new_r['functions']
    old_score = score(c, old_r, truth)
    unchanged = same_features and edges(old_r) == edges(new_r)
    new_score = old_score if unchanged else score(c, new_r, truth)
    old_pairs = {(a,b) for a,b,*_ in old_score['pairs']}
    expected_pairs = {(a,b) for a,b,*_ in archived_pairs}
    result = dict(group=group, case=case, lib=lib, seconds=time.monotonic()-start,
                  source_hash_matches_archive=artifact_hash==fingerprint['rust_artifact_sha256'],
                  old_pairs_match_archive=old_pairs==expected_pairs,
                  old_features_match_archive=old_r['functions']==archived_r['functions'],
                  old_local_edges_match_archive=edges(old_r)==edges(archived_r),
                  node_features_unchanged=same_features,
                  consumed_graph_unchanged=unchanged,
                  removed_local_edges=sorted(edges(old_r)-edges(new_r)),
                  added_local_edges=sorted(edges(new_r)-edges(old_r)),
                  variants={'old':old_score,'new':new_score})
    write(destination/'comparison.json', result)
    return result


def aggregate(rows):
    out = {}
    for variant in ('old','new'):
        libs = defaultdict(list)
        for row in rows:
            libs[row['lib']].append(row['variants'][variant])
        metrics = {k:sum(row['variants'][variant][k] for row in rows)
                   for k in ('truth','accepted','correct','renamed_total','renamed_correct')}
        metrics['micro_precision'] = metrics['correct']/metrics['accepted'] if metrics['accepted'] else None
        metrics['unadjudicated'] = sum(len(row['variants'][variant]['unadjudicated']) for row in rows)
        metrics['micro_precision_upper'] = ((metrics['correct']+metrics['unadjudicated'])/metrics['accepted']
                                            if metrics['accepted'] else None)
        metrics['macro_precision'] = sum(sum(x['precision'] or 0 for x in rs)/len(rs)
                                         for rs in libs.values())/len(libs)
        metrics['macro_precision_upper'] = sum(sum(x['precision_upper'] or 0 for x in rs)/len(rs)
                                               for rs in libs.values())/len(libs)
        out[variant] = metrics
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--new-analyzer', type=Path, required=True)
    ap.add_argument('--old-analyzer', type=Path, default=Path(group_a.ANALYZER))
    ap.add_argument('--out', type=Path, required=True, help='new directory, outside frozen rq1 results')
    ap.add_argument('--groups', nargs='+', choices=['a','b'], default=['a','b'])
    ap.add_argument('--workers', type=int, default=2)
    args = ap.parse_args()
    args.out = args.out.resolve()
    if args.out == ARCHIVE or ARCHIVE in args.out.parents:
        ap.error('refusing to write inside frozen results/rq1_matching')
    if not 1 <= args.workers <= 4:
        ap.error('workers must be between 1 and 4')
    args.out.mkdir(parents=True, exist_ok=False)
    provenance = dict(old_binary_sha256=digest(args.old_analyzer), new_binary_sha256=digest(args.new_analyzer),
                      matcher_sha256=digest(ROOT/'tools/stu_selector/matcher.py'),
                      policy={'alpha':.7,'eps':.01,'tau':.05,'iters':15,'df_cap':.5},
                      interpretation='retrospective regression audit; not held-out validation',
                      precision_semantics='lower bound; pair with precision_upper when unadjudicated is nonempty',
                      rustc=subprocess.check_output(['rustc','--version'],text=True).strip())
    write(args.out/'provenance.json',provenance)
    jobs = []
    if 'a' in args.groups:
        jobs += [('a',k) for k in read(ARCHIVE/'rows/group_a_full.json') if not k.startswith('_')]
    if 'b' in args.groups:
        jobs += [('b',k) for k,v in read(ARCHIVE/'rows/group_b_full.json')['rows'].items()
                 if v['artifact_status']=='COMPLETE']
    jobs = [(g,k,str(args.old_analyzer.resolve()),str(args.new_analyzer.resolve()),str(args.out)) for g,k in jobs]
    rows, failures = [], []
    with ProcessPoolExecutor(max_workers=args.workers) as pool:
        futures = {pool.submit(run,job):job[:2] for job in jobs}
        for future in as_completed(futures):
            case = futures[future]
            try:
                row = future.result()
            except Exception as exc:
                failures.append({'case':case,'error':str(exc)})
                print('FAILED',case,str(exc),flush=True)
                continue
            rows.append(row)
            print(row['group'],row['case'],
                  {k:f"{v['correct']}/{v['accepted']}" for k,v in row['variants'].items()},
                  'source_reproduced',row['source_hash_matches_archive'],
                  'pairs_reproduced',row['old_pairs_match_archive'],flush=True)
    summary = {g:aggregate([r for r in rows if r['group']==g]) for g in args.groups
               if any(r['group']==g for r in rows)}
    write(args.out/'summary.json',dict(aggregates=summary,rows=rows,failures=failures))
    print(json.dumps(summary,indent=2),flush=True)
    return bool(failures)


if __name__ == '__main__':
    raise SystemExit(main())

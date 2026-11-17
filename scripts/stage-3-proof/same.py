"""Condition 1's second half of stage 3's proof (decision 0153, "How the
conditions are read", 1): each arrangement run twice gives the same batch
file, and the exports of a sample of seeds are the same. It prints what is
the same and what differs; it judges nothing.

    python3 same.py --batch FIRST.json SECOND.json
    python3 same.py --exports FIRST_DIR SECOND_DIR

The batch files are compared byte for byte; where they differ, the first
seed and key that differ are named. The exports are paired by file name
(every .json file of FIRST_DIR with the file of the same name in SECOND_DIR)
and compared byte for byte; where a pair differs, the first day whose hash
differs and the first event that differs are named. Both may be given in one
call.
"""
import argparse
import os
import sys

from common import load_json


def same_bytes(a, b):
    with open(a, 'rb') as fa, open(b, 'rb') as fb:
        while True:
            x, y = fa.read(1 << 20), fb.read(1 << 20)
            if x != y:
                return False
            if not x:
                return True


def first_batch_difference(a, b):
    x, y = load_json(a), load_json(b)
    for key in sorted(set(x) | set(y)):
        if key == 'per_run':
            continue
        if x.get(key) != y.get(key):
            return f'top-level key {key!r}'
    rx = {r['seed']: r for r in x.get('per_run', [])}
    ry = {r['seed']: r for r in y.get('per_run', [])}
    for seed in sorted(set(rx) | set(ry)):
        if seed not in rx or seed not in ry:
            return f'seed {seed} is in one file only'
        if rx[seed] != ry[seed]:
            keys = [k for k in sorted(set(rx[seed]) | set(ry[seed])) if rx[seed].get(k) != ry[seed].get(k)]
            return f'seed {seed}, keys {", ".join(keys)}'
    return 'the same JSON, different bytes'


def first_export_difference(a, b):
    x, y = load_json(a), load_json(b)
    for dx, dy in zip(x.get('days', []), y.get('days', [])):
        if dx != dy:
            return f'day {dx["day"]} (hash {dx.get("hash")} against {dy.get("hash")})'
    for ex, ey in zip(x.get('events', []), y.get('events', [])):
        if ex != ey:
            return f'event #{ex["id"]} (day {ex["day"]})'
    if len(x.get('events', [])) != len(y.get('events', [])):
        return 'the event lists differ in length'
    keys = [k for k in sorted(set(x) | set(y)) if x.get(k) != y.get(k)]
    return f'keys {", ".join(keys)}' if keys else 'the same JSON, different bytes'


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--batch', nargs=2, metavar=('FIRST', 'SECOND'))
    ap.add_argument('--exports', nargs=2, metavar=('FIRST_DIR', 'SECOND_DIR'))
    a = ap.parse_args()
    if not a.batch and not a.exports:
        ap.error('give --batch, --exports or both')
    if a.batch:
        f, s = a.batch
        if same_bytes(f, s):
            print(f'batch files: the same, byte for byte ({os.path.getsize(f)} bytes)')
        else:
            print(f'batch files: DIFFERENT: {first_batch_difference(f, s)}')
    if a.exports:
        d1, d2 = a.exports
        names = sorted(n for n in os.listdir(d1) if n.endswith('.json'))
        same, diff, missing = [], [], []
        for n in names:
            other = os.path.join(d2, n)
            if not os.path.exists(other):
                missing.append(n)
            elif same_bytes(os.path.join(d1, n), other):
                same.append(n)
            else:
                diff.append((n, first_export_difference(os.path.join(d1, n), other)))
        print(f'exports: {len(names)} in {d1}; the same, byte for byte: {len(same)}; '
              f'different: {len(diff)}; with no file of the name in {d2}: {len(missing)}')
        for n, why in diff:
            print(f'  different: {n}: {why}')
        for n in missing:
            print(f'  missing: {n}')
    return 0


if __name__ == '__main__':
    sys.exit(main())

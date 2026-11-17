"""Condition 5 of stage 3's proof, better than a side given nothing
(decision 0153, "How the conditions are read", 5), from two batch files. It
prints the medians and which is higher, and never a verdict: the mark is
read by the architect.

    python3 margin.py INTENTS.json NOTHING.json

INTENTS is the batch file of the arrangement in which the side states its
intents, (a); NOTHING the batch file of the arrangement in which the same
side is given nothing: (b) for the Allied side, (c) for the Japanese. Each
side's median is printed for both files, so the one pair serves the side it
is about; the record's pairs are (a) beside (b) for the Allied side and (a)
beside (c) for the Japanese.

The margin of a seed is the batch's `per_run[].allied_margin`, a decimal
string, read exactly (Python's Decimal); the Japanese margin is its
negative. The median of an even count of seeds is the mean of the two middle
values; the two middle values are printed beside it.
"""
import argparse
import sys
from decimal import Decimal

from common import load_json, is_batch


def margins(path):
    b = load_json(path)
    if not is_batch(b):
        raise SystemExit(f'{path} is not a batch file')
    rows = {}
    for run in b['per_run']:
        m = run.get('allied_margin')
        if m is None:
            raise SystemExit(f'{path}: seed {run["seed"]} has no allied_margin (a scenario with objectives is needed)')
        rows[run['seed']] = Decimal(m)
    return b, rows


def median(values):
    v = sorted(values)
    n = len(v)
    if n % 2:
        return v[n // 2], (v[n // 2], v[n // 2])
    lo, hi = v[n // 2 - 1], v[n // 2]
    return (lo + hi) / 2, (lo, hi)


def fmt(d):
    s = format(d.normalize(), 'f')
    return '0' if s in ('-0', '0') else s


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('intents')
    ap.add_argument('nothing')
    a = ap.parse_args()
    bi, mi = margins(a.intents)
    bn, mn = margins(a.nothing)
    print(f'intents: {a.intents}\n  scenario {bi["scenario"].get("file")}, {len(mi)} seeds')
    print(f'nothing: {a.nothing}\n  scenario {bn["scenario"].get("file")}, {len(mn)} seeds')
    if set(mi) != set(mn):
        print(f'NOTE: the files do not have the same seeds ({len(set(mi) - set(mn))} only in the first, '
              f'{len(set(mn) - set(mi))} only in the second); each median is over its own file\'s seeds')
    print()
    for side, sign in (('Allied', 1), ('Japanese', -1)):
        a_med, a_mid = median([sign * m for m in mi.values()])
        n_med, n_mid = median([sign * m for m in mn.values()])
        if a_med > n_med:
            higher = 'from its intents'
        elif a_med < n_med:
            higher = 'given nothing'
        else:
            higher = 'neither: the medians are equal'
        print(f'{side} margin (the {"Allied" if sign == 1 else "negative of the Allied"}):')
        print(f'  from its intents: median {fmt(a_med)} (middle values {fmt(a_mid[0])}, {fmt(a_mid[1])})')
        print(f'  given nothing:    median {fmt(n_med)} (middle values {fmt(n_mid[0])}, {fmt(n_mid[1])})')
        print(f'  higher: {higher}')
    return 0


if __name__ == '__main__':
    sys.exit(main())

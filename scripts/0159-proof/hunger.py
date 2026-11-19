"""Condition 3 of decision 0159's proof, "Men landed are fed" (decision 0159,
"The proof: how each condition is read", 3), from the umpire's exports of
arrangement (a), one file a seed. It prints counts, ratios and medians and
never a verdict: the mark is read by the architect.

    python3 hunger.py EXPORTS... [--list] [--jsonl FILE]

EXPORTS are export files (`sim-cli export ... --seed N --out FILE`) or
directories of them. It reuses stage 3's `common.py` for reading them.

For each seed and each side:

- **Landed.** The men of `men_landed` events of a unit of the side (the
  unit's record; the landing task force's where no record names the unit, as
  stage 3's reading does) in a hex whose base was not that side's at that
  event. "The hex's base" is a base of the start (the export's `bases`) in
  that hex; a base a landing formed (`base_formed`) is not one. It was not the
  side's where no base of the start in the hex is held by the side just
  before the event (the start's holders, then each `base_changed_side`
  applied in event order: stage 3's `base_sides_timeline`). A landing in a
  hex with no base of the start is not counted and is reported apart.
- **Lost.** For each unit with men counted landed, the `lost` of every
  `men_lost_unfed` of that unit from the pulse of its first counted landing
  to the end of the file (the end of day 39), wherever it is and whoever
  holds the place. "From that pulse" is by (day, pulse), night before morning
  before afternoon; every `men_lost_unfed` is of a night pulse. A unit's
  losses in the same pulse but before its landing event (in event order) are
  counted, as the words say, and their total is printed as a check.
- **Lost while not the side's.** The same losses, counted only where, just
  before the loss event, no base of the start in the hex of the unit's latest
  counted landing (its place) is the side's. Reported, not judged.

The seed's value for a side is lost over landed, for a side that landed any.
The median over the seeds on which the side landed men is printed for each
side; the median of an even count is the mean of the two middle values,
printed beside it.
"""
import argparse
import json
import os
import sys
from fractions import Fraction

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'stage-3-proof'))
from common import Game, exports, hex_of, SIDES  # noqa: E402

PULSE = {'night': 0, 'morning': 1, 'afternoon': 2}


def pulse_key(e):
    p = e.get('pulse')
    return (e['day'], PULSE[p] if p in PULSE else 3)


def read_seed(game):
    start_bases = {}
    for b in game.doc.get('bases', []):
        start_bases.setdefault((b['q'], b['r']), []).append(b['id'])
    order = {e['id']: i for i, e in enumerate(game.events)}

    landed = {s: 0 for s in SIDES}
    landed_events = {s: [] for s in SIDES}
    no_base = {s: 0 for s in SIDES}
    first = {}        # unit -> (pulse key, event index) of its first counted landing
    place = {}        # unit -> list of (event index, hex) of its counted landings
    unit_side = {}
    for e, holders in game.base_sides_timeline():
        if e['kind'] != 'men_landed':
            continue
        d = e['data']
        side = game.unit_side(e)
        if side not in SIDES:
            raise SystemExit(f'{game.path}: event {e["id"]}: no side for land_unit:{d["unit"]}')
        hx = hex_of(d['hex'])
        bases = start_bases.get(hx)
        if not bases:
            no_base[side] += d['men']
            continue
        if any(holders.get(b) == side for b in bases):
            continue
        landed[side] += d['men']
        landed_events[side].append(e['id'])
        u = d['unit']
        unit_side[u] = side
        first.setdefault(u, (pulse_key(e), order[e['id']]))
        place.setdefault(u, []).append((order[e['id']], hx))

    lost = {s: 0 for s in SIDES}
    lost_while = {s: 0 for s in SIDES}
    same_pulse_before = {s: 0 for s in SIDES}
    lost_events = {s: [] for s in SIDES}
    for e, holders in game.base_sides_timeline():
        if e['kind'] != 'men_lost_unfed':
            continue
        u = e['data']['unit']
        if u not in first:
            continue
        key, idx = first[u]
        if pulse_key(e) < key:
            continue
        side = unit_side[u]
        n = e['data']['lost']
        lost[side] += n
        lost_events[side].append(e['id'])
        if pulse_key(e) == key and order[e['id']] < idx:
            same_pulse_before[side] += n
        here = [hx for i, hx in place[u] if i < order[e['id']]]
        hx = here[-1] if here else place[u][0][1]
        if not any(holders.get(b) == side for b in start_bases[hx]):
            lost_while[side] += n

    return {
        'seed': game.seed,
        'file': game.path,
        'landed': landed,
        'lost': lost,
        'lost_while_not_the_sides': lost_while,
        'lost_same_pulse_before_landing': same_pulse_before,
        'landed_in_a_hex_with_no_start_base': no_base,
        'units': {s: sorted(u for u in first if unit_side[u] == s) for s in SIDES},
        'landed_events': landed_events,
        'lost_events': lost_events,
    }


def median(values):
    v = sorted(values)
    n = len(v)
    if n == 0:
        return None, None
    if n % 2:
        return v[n // 2], (v[n // 2], v[n // 2])
    return (v[n // 2 - 1] + v[n // 2]) / 2, (v[n // 2 - 1], v[n // 2])


def dec(x, places=4):
    if x is None:
        return '-'
    return f'{float(x):.{places}f}'


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('exports', nargs='+')
    ap.add_argument('--list', action='store_true', help='one line a seed')
    ap.add_argument('--jsonl', help='one line a seed, with the events each count rests on')
    a = ap.parse_args()

    rows, scenario = [], None
    for path, doc in exports(a.exports):
        name = doc['scenario'].get('path') or doc['scenario'].get('name')
        if scenario is None:
            scenario = name
        elif name != scenario:
            raise SystemExit(f'{path}: scenario {name}, where the first file has {scenario}')
        rows.append(read_seed(Game(doc, path)))
    if not rows:
        raise SystemExit('no export read')
    seeds = [r['seed'] for r in rows]
    if len(set(seeds)) != len(seeds):
        raise SystemExit('two exports of the same seed')
    rows.sort(key=lambda r: r['seed'])
    print(f'scenario: {scenario}')
    print(f'exports read: {len(rows)}')
    print()
    if a.list:
        print('seed  side      landed   lost  lost/landed  lost while not the side\'s  ratio')
        for r in rows:
            for s in SIDES:
                ld, lo, lw = r['landed'][s], r['lost'][s], r['lost_while_not_the_sides'][s]
                ratio = dec(Fraction(lo, ld)) if ld else '-'
                ratio_w = dec(Fraction(lw, ld)) if ld else '-'
                print(f'{r["seed"]:4}  {s:8} {ld:7} {lo:6}  {ratio:>11}  {lw:24}  {ratio_w:>6}')
        print()
    for s in SIDES:
        landed_on = [r for r in rows if r['landed'][s] > 0]
        ratios = [Fraction(r['lost'][s], r['landed'][s]) for r in landed_on]
        ratios_w = [Fraction(r['lost_while_not_the_sides'][s], r['landed'][s]) for r in landed_on]
        m, mid = median(ratios)
        mw, midw = median(ratios_w)
        print(f'{s}: landed men on {len(landed_on)} of {len(rows)} seeds')
        print(f'  median of lost over landed: {dec(m)}'
              + (f' (middle values {dec(mid[0])}, {dec(mid[1])})' if mid else ''))
        print(f'  median of lost while the place was not the side\'s over landed (reported): {dec(mw)}'
              + (f' (middle values {dec(midw[0])}, {dec(midw[1])})' if midw else ''))
        ml, _ = median([r['landed'][s] for r in landed_on])
        mlo, _ = median([r['lost'][s] for r in landed_on])
        print(f'  median men landed: {dec(ml, 1)}; median men lost: {dec(mlo, 1)}')
    print()
    print('checks, over all seeds: '
          + '; '.join(f'{s} lost in the landing\'s pulse before the landing event '
                      f'{sum(r["lost_same_pulse_before_landing"][s] for r in rows)}, '
                      f'landed in a hex with no base of the start (not counted) '
                      f'{sum(r["landed_in_a_hex_with_no_start_base"][s] for r in rows)}'
                      for s in SIDES))
    if a.jsonl:
        with open(a.jsonl, 'w') as f:
            for r in rows:
                f.write(json.dumps(r) + '\n')


if __name__ == '__main__':
    sys.exit(main())

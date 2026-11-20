"""Read the first run of scenario 103, the Eastern Solomons over five days
(decision 0184), and print its counts. It prints counts and never a verdict.

    python3 scripts/five-days/read_first_run.py BATCH.json COUNTS.jsonl

BATCH.json is `sim-cli batch data/scenarios/five-days/01a_eastern_solomons.toml
--out BATCH.json` (1,000 runs, seeds 1 to 1000). COUNTS.jsonl is what the
helper `tests/five_days_first_run.rs` writes for the same seeds (its doc
comment says how it is run). The script first checks that the two are the
same games: for every seed the same final hash, Allied margin and outcome.
Then it prints how often each side wins and at which level, and for each
count behind the points its lowest, middle (the value at place ceil(n / 2)
of the sorted runs) and highest, and in how many runs it is above zero.
Python 3.9 or later, standard library only. It reads no Markdown file.
"""
import json
import sys
from decimal import Decimal


def middle(values):
    s = sorted(values)
    return s[(len(s) + 1) // 2 - 1]


def main():
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    batch = json.load(open(sys.argv[1], encoding='utf-8'))
    counts = [json.loads(line) for line in open(sys.argv[2], encoding='utf-8') if line.strip()]
    by_seed = {c['seed']: c for c in counts}
    runs = batch['per_run']
    if len(runs) != len(counts):
        raise SystemExit(f'the batch has {len(runs)} runs and the counts {len(counts)}')
    for r in runs:
        c = by_seed[r['seed']]
        for key in ('final_hash', 'allied_margin', 'allied_outcome'):
            if r[key] != c[key]:
                raise SystemExit(f'seed {r["seed"]}: {key} differs: {r[key]} against {c[key]}')
    n = len(runs)
    print(f'{n} games, seeds {runs[0]["seed"]} to {runs[-1]["seed"]}; the batch and the '
          'counts agree on every final hash, margin and outcome.\n')

    print('Results, seen from the Allied side (the batch):')
    for row in batch['results']:
        print(f'  {row["outcome"]}: {row["runs"]}')
    allied_wins = sum(r['runs'] for r in batch['results'] if r['outcome'].endswith('victory'))
    japanese_wins = sum(r['runs'] for r in batch['results'] if r['outcome'].endswith('defeat'))
    print(f'  Allied wins {allied_wins}, Japanese wins {japanese_wins}, '
          f'stalemates {n - allied_wins - japanese_wins}\n')

    def obj(c, side, kind):
        return c[side].get(kind, {})

    def count(c, side, kind):
        return Decimal(obj(c, side, kind).get('count', '0'))

    def by_kind(c, side, kind, ship_kind):
        return obj(c, side, kind).get('by_kind', {}).get(ship_kind, 0)

    def carriers_out(c, side, at_least):
        return sum(1 for _, days in obj(c, side, 'ships_out_of_action').get('carriers_out', [])
                   if days >= at_least)

    measures = [
        # (words, function of one run)
        ('Japanese carriers sunk (Allied ships_sunk, carriers)',
         lambda c: by_kind(c, 'allied', 'ships_sunk', 'carrier')),
        ('American carriers sunk (Japanese ships_sunk, carriers)',
         lambda c: by_kind(c, 'japanese', 'ships_sunk', 'carrier')),
        ('Japanese carriers afloat and out of action, any days',
         lambda c: carriers_out(c, 'allied', 1)),
        ('Japanese carriers afloat and out of action for the whole 30 days',
         lambda c: carriers_out(c, 'allied', 30)),
        ('American carriers afloat and out of action, any days',
         lambda c: carriers_out(c, 'japanese', 1)),
        ('American carriers afloat and out of action for the whole 30 days',
         lambda c: carriers_out(c, 'japanese', 30)),
        ('Transports sunk (Allied ships_sunk, transports)',
         lambda c: by_kind(c, 'allied', 'ships_sunk', 'transport')),
        ('Convoy destroyers and other Japanese destroyers sunk',
         lambda c: by_kind(c, 'allied', 'ships_sunk', 'destroyer')),
        ('Men landed at Taivu (the convoy\'s, from the events)',
         lambda c: c['men_landed_taivu']),
        ('Tons landed at Taivu (Japanese cargo_landed)',
         lambda c: count(c, 'japanese', 'cargo_landed')),
        ('Japanese men lost (Allied men_lost)',
         lambda c: count(c, 'allied', 'men_lost')),
        ('Allied men lost (Japanese men_lost)',
         lambda c: count(c, 'japanese', 'men_lost')),
        ('Japanese aircraft lost (Allied aircraft_destroyed)',
         lambda c: count(c, 'allied', 'aircraft_destroyed')),
        ('American aircraft lost (Japanese aircraft_destroyed)',
         lambda c: count(c, 'japanese', 'aircraft_destroyed')),
        ('Days Lunga was open, of 5 (Allied base_open)',
         lambda c: count(c, 'allied', 'base_open')),
        ('Days Lunga was shut, of 5 (Japanese base_open, closed)',
         lambda c: count(c, 'japanese', 'base_open')),
        ('Allied points', lambda c: Decimal(c['allied']['total'])),
        ('Japanese points', lambda c: Decimal(c['japanese']['total'])),
        ('Allied margin', lambda c: Decimal(c['allied_margin'])),
    ]
    print('| Count | Lowest | Middle | Highest | Runs above zero |')
    print('| --- | --- | --- | --- | --- |')
    for words, f in measures:
        values = [f(c) for c in counts]
        above = sum(1 for v in values if v > 0)
        print(f'| {words} | {min(values)} | {middle(values)} | {max(values)} | {above} |')

    print('\nPoints by objective, the middle run of each, and the runs above zero:')
    for side in ('allied', 'japanese'):
        kinds = [k for k in counts[0][side] if k != 'total']
        for k in kinds:
            values = [Decimal(c[side][k]['points']) for c in counts]
            above = sum(1 for v in values if v > 0)
            print(f'  {side} {k}: lowest {min(values)}, middle {middle(values)}, '
                  f'highest {max(values)}, above zero in {above}')


if __name__ == '__main__':
    main()

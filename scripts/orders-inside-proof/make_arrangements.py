"""Make the scenario files of the proof of decision 0156 (orders inside a
day), or check that the files kept are the ones it makes.

    python3 make_arrangements.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It reads scenario files of main as text and writes them, each with
the smallest change, into `data/scenarios/orders-inside-proof/`. With
`--check` it writes nothing: it says whether each file kept is byte for byte
the one it would make, and exits 1 where one is not. Python 3.9 or later,
standard library only, deterministic. No build step and no test runs it; it
is run by hand whenever a source file changes, before the proof is run.

Each file is its source with exactly these changes, and nothing else:
  1. a header, put above the source's first line;
  2. where the file is to have the key, the line
     `orders_inside_a_day = true` put after the source's `days = ` line.

The files, and the conditions of decision 0156's proof each serves:

  05_eastern_solomons_on.toml   scenario 5 with the key on: conditions 2
                                and 5, and two cases of condition 6
  36_one_convoy_on.toml         scenario 36 with the key on: one case of
                                condition 6 (the hold)
  05c_allied_by_days.toml       stage 2's second proof, arrangement (c) (the
                                Allied side given nothing, the Japanese side
                                scripted): condition 7, the games by days
  05c_allied_by_pulse.toml      the same with the key on: condition 7, the
                                games advanced by pulse

The folder is a subfolder of data/scenarios, so no loop of the tests, CI or
scripts/ walks it (decision 0072). The source files in a subfolder name their
reference files by `../../`, as the copies do; scenario 5 and scenario 36 name
them by `../`, so their copies have each `"../` of a path made `"../../`.
"""
import argparse
import os
import sys

FOLDER = 'data/scenarios/orders-inside-proof'

FILES = [
    # (file, source, key on, serves)
    ('05_eastern_solomons_on.toml', 'data/scenarios/05_eastern_solomons.toml', True,
     'conditions 2 and 5, and cases D and E of condition 6'),
    ('36_one_convoy_on.toml', 'data/scenarios/36_one_convoy.toml', True,
     'case C of condition 6 (the convoy held by an order at 0600)'),
    ('05c_allied_by_days.toml', 'data/scenarios/second-proof/05c_eastern_solomons.toml', False,
     'condition 7, the 30 games by days'),
    ('05c_allied_by_pulse.toml', 'data/scenarios/second-proof/05c_eastern_solomons.toml', True,
     'condition 7, the 30 games advanced by pulse'),
]

KEY = 'orders_inside_a_day = true\n'


def header(source, key_on, serves):
    key_words = ('the line `orders_inside_a_day = true` after `days`'
                 if key_on else 'no key: orders inside a day stay off')
    return f'''# THE PROOF OF DECISION 0156 (orders inside a day), step 5. It serves
# {serves}.
#
# Made by scripts/orders-inside-proof/make_arrangements.py from
# {source}
# by the smallest change. Do not edit by hand: change the script or the
# source and make it again (`python3
# scripts/orders-inside-proof/make_arrangements.py --check` says whether this
# file is still the one it makes). It differs from its source in this
# header; in {key_words};
# and, where the source lies in data/scenarios itself, in each reference
# path, one folder deeper. Nothing else.
#
# Kept out of the loops that walk data/scenarios/*.toml by being in this
# folder (decision 0072).
#
'''


def make(repo, source, key_on, serves):
    with open(os.path.join(repo, source), encoding='utf-8') as f:
        text = f.read()
    if 'orders_inside_a_day' in text:
        raise SystemExit(f'{source}: the key is set already')
    deeper = os.path.dirname(source) == 'data/scenarios'
    out = []
    done = False
    for line in text.splitlines(keepends=True):
        if deeper and line.split('=')[0].strip() in (
                'map', 'ship_classes', 'port_sizes', 'aircraft_types', 'search',
                'surface_combat', 'air_combat', 'ground', 'air_upkeep', 'land_links',
                'passages'):
            line = line.replace('"../', '"../../')
        out.append(line)
        if key_on and not done and line.startswith('days = '):
            out.append(KEY)
            done = True
    if key_on and not done:
        raise SystemExit(f'{source}: no days line')
    body = ''.join(out)
    if deeper and '"../' in body.replace('"../../', ''):
        raise SystemExit(f'{source}: a path to "../" is left that the script does not know')
    return header(source, key_on, serves) + body


def main():
    parser = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    parser.add_argument('repo', nargs='?',
                        default=os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..')))
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    bad = 0
    for name, source, key_on, serves in FILES:
        text = make(args.repo, source, key_on, serves)
        path = os.path.join(args.repo, FOLDER, name)
        if args.check:
            try:
                with open(path, encoding='utf-8') as f:
                    kept = f.read()
            except FileNotFoundError:
                kept = None
            if kept == text:
                print(f'{name}: as made')
            else:
                print(f'{name}: NOT as made')
                bad += 1
        else:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, 'w', encoding='utf-8') as f:
                f.write(text)
            print(f'{name}: written')
    sys.exit(1 if bad else 0)


if __name__ == '__main__':
    main()

"""Make the copies of the scenarios the table plays, which name the ground
figures of hunger that takes time (decision 0205, 3), or check that the
copies kept are the ones it makes.

    python3 scripts/table-copies/make_copies.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). Each copy is the file on file, byte for byte, with one line changed:
its `ground = "../../reference/ground.toml"` becomes
`ground = "../../reference/ground_hunger.toml"`. The copies are written to
data/scenarios/table-hunger/ under the same file name. The folder is two
deep under data/, as each source's folder is, so every other relative path
in a copy resolves to the file the source names. A source with no such line
(a scenario with no land units) has no copy, and the table plays the file
on file. The files the proofs play are not touched.

With `--check` it writes nothing: it says whether each copy kept is the one it
would make, whether the folder holds anything else, and exits 1 where not.
Python 3, standard library only, deterministic. No build step runs it;
scenario 119's test runs it with `--check`. Run it by hand whenever a source
changes.

Since step C of decision 0234 the nine sources in coral-sea, five-days and
midway are built on another file (`built_on`), so their text is not a scenario
this script can copy byte for byte. They are in KEPT: a kept copy is not read,
not written and not compared, must be in the folder, and `--check` prints for
it `: kept, its source is built on another file`. Scenario 119's test holds
them as tables, the source resolved. Step D of decision 0234 retires this
script.
"""
import argparse
import os
import sys

FOLDER = 'data/scenarios/table-hunger'
OLD = 'ground = "../../reference/ground.toml"\n'
NEW = 'ground = "../../reference/ground_hunger.toml"\n'

# The scenarios of table/scenarios.json that name the live ground figures. (Midway's
# copies are kept though the table now plays the copies of them with submarines,
# data/scenarios/table-boats/, decision 0225: those are made from these.)
SOURCES = [
    'data/scenarios/staff/07_full_slice.toml',
    'data/scenarios/stage-3-proof/07b_slice.toml',
    'data/scenarios/stage-3-proof/07c_slice.toml',
]

# Sources built on another file (decision 0234, step C): their copies are kept.
KEPT = [
    'data/scenarios/coral-sea/03b_coral_sea_both_oilers.toml',
    'data/scenarios/coral-sea/03c_coral_sea_both_oilers.toml',
    'data/scenarios/coral-sea/03d_coral_sea_both_oilers.toml',
    'data/scenarios/five-days/02b_eastern_solomons_oilers.toml',
    'data/scenarios/five-days/02c_eastern_solomons_oilers.toml',
    'data/scenarios/five-days/02d_eastern_solomons_oilers.toml',
    'data/scenarios/midway/01b_midway.toml',
    'data/scenarios/midway/01c_midway.toml',
    'data/scenarios/midway/01d_midway.toml',
]


def copy_of(text, source):
    lines = text.splitlines(keepends=True)
    found = [i for i, line in enumerate(lines) if line == OLD]
    if len(found) != 1:
        raise SystemExit(f'{source}: the line {OLD.strip()!r} is not there exactly once')
    lines[found[0]] = NEW
    return ''.join(lines)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('repo', nargs='?',
                        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    folder = os.path.join(args.repo, FOLDER)
    stale = []
    names = set()
    for source in SOURCES:
        with open(os.path.join(args.repo, source), encoding='utf-8', newline='') as f:
            made = copy_of(f.read(), source)
        name = os.path.basename(source)
        if name in names:
            raise SystemExit(f'two sources are named {name}')
        names.add(name)
        target = os.path.join(folder, name)
        if args.check:
            try:
                with open(target, encoding='utf-8', newline='') as f:
                    kept = f.read()
            except OSError:
                kept = None
            if kept == made:
                print(f'{FOLDER}/{name}: the copy it makes')
            else:
                print(f'{FOLDER}/{name}: NOT the copy it makes')
                stale.append(name)
        else:
            os.makedirs(folder, exist_ok=True)
            with open(target, 'w', encoding='utf-8', newline='') as f:
                f.write(made)
            print(f'wrote {FOLDER}/{name}')
    for source in KEPT:
        name = os.path.basename(source)
        if name in names:
            raise SystemExit(f'two sources are named {name}')
        names.add(name)
        if os.path.isfile(os.path.join(folder, name)):
            print(f'{FOLDER}/{name}: kept, its source is built on another file')
        else:
            print(f'{FOLDER}/{name}: NOT there, and kept')
            stale.append(name)
    if os.path.isdir(folder):
        for other in sorted(os.listdir(folder)):
            if other not in names:
                print(f'{FOLDER}/{other}: not a copy this script makes')
                stale.append(other)
    if stale:
        sys.exit(1)


if __name__ == '__main__':
    main()

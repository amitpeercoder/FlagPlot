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
"""
import argparse
import os
import sys

FOLDER = 'data/scenarios/table-hunger'
OLD = 'ground = "../../reference/ground.toml"\n'
NEW = 'ground = "../../reference/ground_hunger.toml"\n'

# The scenarios of table/scenarios.json that name the live ground figures.
SOURCES = [
    'data/scenarios/coral-sea/01b_coral_sea.toml',
    'data/scenarios/coral-sea/01c_coral_sea.toml',
    'data/scenarios/coral-sea/01d_coral_sea.toml',
    'data/scenarios/five-days/01b_eastern_solomons.toml',
    'data/scenarios/five-days/01c_eastern_solomons.toml',
    'data/scenarios/five-days/01d_eastern_solomons.toml',
    'data/scenarios/staff/07_full_slice.toml',
    'data/scenarios/stage-3-proof/07b_slice.toml',
    'data/scenarios/stage-3-proof/07c_slice.toml',
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
    if os.path.isdir(folder):
        for other in sorted(os.listdir(folder)):
            if other not in names:
                print(f'{FOLDER}/{other}: not a copy this script makes')
                stale.append(other)
    if stale:
        sys.exit(1)


if __name__ == '__main__':
    main()

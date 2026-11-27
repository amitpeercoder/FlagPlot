"""Make the table's own files of the 39 days under decision 0224's rulings of
Nov 23, 2026, or check that the files kept are the ones it makes.

    python3 scripts/table-rear-bases/make_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It writes three files to data/scenarios/table-rear-bases/, each from
the file of the same name in data/scenarios/table-boats/ that the table plays
today (the games with submarines of decisions 0225 and 0227, themselves the
table's copies from data/scenarios/table-hunger/):

    07_full_slice_submarines.toml   from table-boats/07_full_slice_submarines.toml
    07b_slice_submarines.toml       from table-boats/07b_slice_submarines.toml
    07c_slice_submarines.toml       from table-boats/07c_slice_submarines.toml

Each file is the file it copies with exactly these changes, and nothing else
(decision 0224, "The architect's rulings", and "The architect's statements for
the files", 1 to 5):

  1. its header (the comment lines before its first key) written anew: the
     lines below, which then give the header of the file it copies as it
     stands;
  2. its `name`, which says it is this copy;
  3. ruling 1: for each Japanese run the files start at Shortland (task forces
     30 and 32 to 39), `at_base = 2` and `home_port = 2` (Rabaul) in place of
     7, and `available_from` one less, each with a note;
  4. ruling 2: Espiritu Santo (base 3) starts with `supply_t = 2120` in place
     of 960, with its note and working;
  5. a note beside Shortland (base 7), which keeps no supply and no fuel;

and, since decision 0232, step 3 (the plan's items 5 to 7, the designer's
yes of Nov 27, 2026), these lines added and no other:

  6. in each file, `trial = "../../reference/trial.toml"`, next after its
     `class_sides` line;
  7. in each file, island 3, "Guadalcanal and Tulagi", of the places base 1
     and base 4, after its island 2 (in 07b and 07c) or at the end of the
     file (in 07_full_slice_submarines.toml);
  8. in 07_full_slice_submarines.toml, which has no island, islands 1 and 2
     at the end of the file, each line as 07b has it, before island 3.

With `--check` it writes nothing: it says whether each file kept is byte for
byte the one it would make, whether the folder holds anything else, and exits
1 where not. Python 3, standard library only, deterministic. No build step
runs it; the tests run it with `--check`. Run it by hand whenever a source
changes. The files of the table's other games are not touched.
"""
import os
import re
import sys

SOURCE_FOLDER = 'data/scenarios/table-boats'
FOLDER = 'data/scenarios/table-rear-bases'
FILES = ['07_full_slice_submarines.toml', '07b_slice_submarines.toml',
         '07c_slice_submarines.toml']
RUNS = [30] + list(range(32, 40))

HEADER = '''# THE TABLE'S 39 DAYS UNDER DECISION 0224'S RULINGS OF NOV 23, 2026.
# Made by scripts/table-rear-bases/make_files.py from {source}: the same
# file with these changes and nothing else. (The table plays the games with
# submarines of decisions 0225 and 0227; this file is that game under the
# rulings.)
#
#   Ruling 1: each Japanese run the file starts at Shortland (task forces 30
#   and 32 to 39) starts stopped at Rabaul (base 2) with Rabaul as its home
#   port, and may be used one day earlier than the file has it. What each has
#   aboard at the start is unchanged. Shortland (base 7) keeps no supply and
#   no fuel. A NAMED SIMPLIFICATION: the passage from Rabaul to the Shortlands
#   in a transport and the run from the Shortlands in destroyers are one
#   passage by one force (troop_lift_facts 12835, 12837, 12813, 12815, 723).
#   From Rabaul no run can go at full speed and come home.
#   Ruling 2: Espiritu Santo (base 3) starts with 2,120 t of supply in place of
#   960 t. PLACEHOLDER: the sum of the loads the rows state for the runs that
#   landed at Guadalcanal or Tulagi on or before 14 September 1942, wherever
#   they loaded (troop_lift_facts 88, 60); a least figure of what was carried,
#   not a stock (data/oob/gaps.csv holds the two entries for espiritu-santo).
#   Its fuel stays 600 t, a placeholder (ruling 3).
#
#   Decision 0232, step 3: the line `trial = "../../reference/trial.toml"` is
#   added next after `class_sides`; island 3, "Guadalcanal and Tulagi" (base 1,
#   then base 4), is added after island 2 (at the end of the file in the
#   full-slice copy, which also gains islands 1 and 2 as its sister files have
#   them). An island is map data and the trial a key: no game moves.
#
# Do not edit by hand: change the script or the file it is made from and make
# it again.
#
# The header of the file it is made from follows.
#
'''

NOTE_RUN = (
    '# Decision 0224, ruling 1: this run lies at Rabaul (base 2), its home port, and may be\n'
    '# used a day earlier than the file it is made from has it (was at Shortland, base 7, from\n'
    '# day {old}). {why}\n')
WHY_30 = ('The day is the row\'s own (troop_lift_facts 12835: a leg of Rabaul to the Shortlands '
          'takes a day).')
WHY_OTHER = ('The day earlier is worked out from that row (troop_lift_facts 12835) and is '
             'marked so; no row says where the run loaded.')
WHY_39 = (WHY_OTHER + ' Its two fast transports hold 500 t of fuel each: a run to Lunga from '
          'Rabaul is about 139 t at cruising speed.')

NOTE_BASE_3 = (
    '# Decision 0224, ruling 2: PLACEHOLDER, 2,120 t in place of 960 t. The sum of the loads\n'
    '# the rows state for the runs that landed at Guadalcanal or Tulagi on or before the last\n'
    '# day, wherever they loaded: 120 t (troop_lift_facts 88, Dyer, four destroyer-transports\n'
    '# of 15 August) and 2,000 t (troop_lift_facts 60, Dyer, "over 2,000 tons", 20 to 23\n'
    '# August). Espiritu Santo stands for every port they loaded at (Noumea is not a base of\n'
    '# the scenario). A least figure of what was carried, not a stock; the gap stays open.\n')
NOTE_BASE_7 = (
    '# Decision 0224, ruling 1: Shortland keeps no supply and no fuel (no row gives it a stock:\n'
    '# the two entries of data/oob/gaps.csv for shortland-islands say so). The runs that\n'
    '# started here start at Rabaul, a named simplification.\n')


CLASS_SIDES = 'class_sides = "../../reference/class_sides_full_slice.csv"\n'
TRIAL = 'trial = "../../reference/trial.toml"\n'
ISLAND_1 = ('[[island]]\nnumber = 1\nname = "Guadalcanal"\nplaces = [{ base = 1 }]\n')
ISLAND_2 = ('[[island]]\nnumber = 2\nname = "Tulagi"\nplaces = [{ base = 4 }]\n')
ISLAND_3 = ('[[island]]\nnumber = 3\nname = "Guadalcanal and Tulagi"\n'
            'places = [{ base = 1 }, { base = 4 }]\n')


def header_end(lines):
    """The index of the first line that is not a comment or blank."""
    for k, line in enumerate(lines):
        if line.strip() and not line.startswith('#'):
            return k
    return len(lines)


def make(source_text, source_name):
    lines = source_text.split('\n')
    end = header_end(lines)
    body = '\n'.join(lines[end:])
    old_header = '\n'.join(lines[:end])
    head = HEADER.format(source=f'{SOURCE_FOLDER}/{source_name}') + old_header + '\n'
    # the name
    m = re.search(r'^name = "(.*)"$', body, re.M)
    if not m:
        raise SystemExit(f'{source_name}: no name')
    body = body.replace(
        m.group(0),
        f'name = "{m.group(1)} (the table\'s rear bases, decision 0224)"', 1)
    # base 3
    base3 = re.search(r'(\[\[base\]\]\nid = 3\n.*?)(supply_t = 960\n)', body, re.S)
    if not base3 or base3.group(1).count('[[') != 1:
        raise SystemExit(f'{source_name}: base 3 is not as ruling 2 reads it')
    body = body[:base3.start(2)] + NOTE_BASE_3 + 'supply_t = 2120\n' + body[base3.end(2):]
    # base 7
    base7 = re.search(r'\[\[base\]\]\nid = 7\n', body)
    if not base7:
        raise SystemExit(f'{source_name}: no base 7')
    block7 = body[base7.start():body.index('\n\n', base7.start())]
    if 'supply_t' in block7 or 'fuel_t' in block7:
        raise SystemExit(f'{source_name}: base 7 has a stock')
    body = body[:base7.start()] + NOTE_BASE_7 + body[base7.start():]
    # the runs
    for n in RUNS:
        found = re.search(r'\[\[task_force\]\]\nid = %d\n' % n, body)
        if not found:
            raise SystemExit(f'{source_name}: no task force {n}')
        stop = body.index('\n[[', found.end())
        block = body[found.start():stop]
        keys = {k: re.search(r'^%s = (\d+)$' % k, block, re.M) for k in ('available_from', 'at_base', 'home_port')}
        if not all(keys.values()) or keys['at_base'].group(1) != '7' or keys['home_port'].group(1) != '7':
            raise SystemExit(f'{source_name}: task force {n} is not as ruling 1 reads it')
        old = int(keys['available_from'].group(1))
        why = WHY_30 if n == 30 else WHY_39 if n == 39 else WHY_OTHER
        new_block = block
        new_block = new_block.replace(keys['available_from'].group(0),
                                      NOTE_RUN.format(old=old, why=why) + f'available_from = {old - 1}', 1)
        new_block = new_block.replace('at_base = 7', 'at_base = 2', 1).replace('home_port = 7', 'home_port = 2', 1)
        body = body[:found.start()] + new_block + body[stop:]
    # the trial's figures (decision 0232, step 3)
    if body.count(CLASS_SIDES) != 1:
        raise SystemExit(f'{source_name}: no single class_sides line')
    body = body.replace(CLASS_SIDES, CLASS_SIDES + TRIAL, 1)
    # the islands
    if source_name.startswith('07_'):
        if '[[island]]' in body or not body.endswith('\n'):
            raise SystemExit(f'{source_name}: not as the islands read it')
        body += '\n' + ISLAND_1 + '\n' + ISLAND_2 + '\n' + ISLAND_3
    else:
        if body.count(ISLAND_2) != 1 or body.count('[[island]]') != 2:
            raise SystemExit(f'{source_name}: island 2 is not as the island reads it')
        body = body.replace(ISLAND_2, ISLAND_2 + '\n' + ISLAND_3, 1)
    return head + body


def main():
    args = [a for a in sys.argv[1:] if a != '--check']
    check = '--check' in sys.argv[1:]
    repo = args[0] if args else os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    bad = False
    for name in FILES:
        with open(os.path.join(repo, SOURCE_FOLDER, name)) as handle:
            made = make(handle.read(), name)
        path = os.path.join(repo, FOLDER, name)
        if check:
            kept = open(path).read() if os.path.exists(path) else None
            if kept != made:
                print(f'{FOLDER}/{name}: not the file the script makes')
                bad = True
        else:
            os.makedirs(os.path.join(repo, FOLDER), exist_ok=True)
            with open(path, 'w') as handle:
                handle.write(made)
            print(f'wrote {FOLDER}/{name}')
    if check:
        folder = os.path.join(repo, FOLDER)
        extra = sorted(set(os.listdir(folder)) - set(FILES)) if os.path.isdir(folder) else FILES
        if extra:
            print(f'{FOLDER}: holds {extra}')
            bad = True
        if bad:
            sys.exit(1)
        print('the files kept are the files the script makes')


if __name__ == '__main__':
    main()

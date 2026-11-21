"""Make the base and the four files of scenario 103 with its oilers, the
Eastern Solomons over five days with Platte and Cimarron (decision 0206),
from the old base and the old script, or check that the files kept are the
ones it makes.

    python3 scripts/five-days/make_oiler_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It reads `data/scenarios/five-days/base_eastern_solomons.toml` as
text and writes `data/scenarios/five-days/base_eastern_solomons_oilers.toml`
and `02a_eastern_solomons_oilers.toml` to `02d_eastern_solomons_oilers.toml`.
With `--check` it writes nothing: it says whether each file kept is byte for
byte the one it would make, and exits 1 where one is not. Python 3.9 or
later, standard library only, deterministic.

It imports make_files.py and takes from it FILES, WORDS, HEADER, ALLIED and
JAPANESE (that script keeps the command tables of a side as two texts, not
functions), so the command tables are the old ones byte for byte. The new
base is the old base with exactly these changes: a new header, a new name,
the class file of the oilers named beside the others, its own file of
sides, one task force more (6) and two ships more (44, 45). Each of the four
files is the new base with a header and a name of its own and the tables of
its pairing, as in make_files.py.
"""
import argparse
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location('five_days_make_files', os.path.join(HERE, 'make_files.py'))
old = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(old)

FOLDER = old.FOLDER
OLD_BASE = old.BASE
NEW_BASE = 'data/scenarios/five-days/base_eastern_solomons_oilers.toml'

FILES = [(name.replace('01', '02', 1).replace('_eastern_solomons.toml', '_eastern_solomons_oilers.toml'),
          letter, a, j) for name, letter, a, j in old.FILES]

NAME_NEW = ('name = "Scenario 103 with their oilers, the base: the Eastern Solomons over five days, '
            '23 to 27 August 1942"\n')

FIRST_PARA_OLD = '''# THE BASE of scenario 103, the Eastern Solomons over five days, 23 to 27
# August 1942 (decision 0184, as amended on review and confirmed on Nov 20,
# 2026). It is not played itself: scripts/five-days/make_files.py makes the
# four files of data/scenarios/five-days/ from it, each with this text, a
# name of its own and the commands of its pairing, and nothing else. Edit
# this file or the script, never a file it makes, and make them again. The
# hand calculation is the doc comment of
# tests/scenario_103_the_eastern_solomons_over_five_days.rs.
'''
FIRST_PARA_NEW = '''# THE BASE of scenario 103 with their oilers, the Eastern Solomons over five
# days, 23 to 27 August 1942 (decision 0206). It is not played itself:
# scripts/five-days/make_oiler_files.py makes it from
# base_eastern_solomons.toml and makes the four files of
# data/scenarios/five-days/ (02a to 02d) from it, each with this text, a name
# of its own and the commands of its pairing, and nothing else. Edit the old
# base or the script, never a file it makes, and make them again. The old
# base, and the four files of scenario 103 that read it, are not touched
# (decision 0206, rule 17). The hand calculation is the doc comment of
# tests/scenario_103_the_eastern_solomons_over_five_days.rs; that of the
# oilers, tests/scenario_120_real_oilers.rs.
#
# It is the base of scenario 103 with one force more, the oilers (Platte and
# Cimarron, task force 6), with the class file of the oilers,
# ship_classes_five_days_oilers.csv, named beside the others, and with a
# sides file of its own, class_sides_five_days_oilers.csv. The rest is the
# old base's, byte for byte.
'''

BULLET_OLD = '''#     placeholder troop transports; the men of troop lift rows 8 and 9 aboard.
'''
BULLET_NEW = BULLET_OLD + '''#   - Task force 6, the oilers: Platte and Cimarron, derived by
#     data/derivation/five_days_oilers.toml (classes 2030 and 2031). Their
#     cargo is full, an assumption: their loads on 22 August are a gap. No
#     order is scripted for them (decision 0206, rule 20): they are the
#     commander's to send, and the staff uses them by decision 0201, rule 20.
'''

LEFT_OLD = '''#   - No Allied supply ship comes in during the five days (rule 20).
'''
LEFT_NEW = LEFT_OLD + '''#   - Of the oilers (decision 0206): Sabine, Kaskaskia and Guadalupe, where
#     each was on the days played being disputed or not given; every
#     Japanese oiler, no displacement being on file for any, so the
#     Japanese side has no oiler; and the oilers' escorts, named in no
#     source read.
'''

SHIP_CLASSES_OLD = '''    "../../reference/ship_classes_eastern_solomons.csv",
'''
SHIP_CLASSES_NEW = SHIP_CLASSES_OLD + '''    "../../reference/ship_classes_five_days_oilers.csv",
'''
SHIP_CLASSES_COMMENT_OLD = '''# The 39 days' class files, so that its file of sides (below) fits them; the
# scenario uses classes of the first, third and fifth only.
'''
SHIP_CLASSES_COMMENT_NEW = '''# The 39 days' class files, so that its file of sides (below) fits them; the
# scenario uses classes of the first, third and fifth only; and, beside the
# first, the oilers' two classes (2030 and 2031, ship_classes_five_days_
# oilers.csv, decision 0206).
'''

SIDES_OLD = '''# Whose each class and type is (decision 0173, 1 to 3): the 39 days' file of
# sides, which names every class and type this scenario uses.
class_sides = "../../reference/class_sides_full_slice.csv"
'''
SIDES_NEW = '''# Whose each class and type is (decision 0173, 1 to 3): the 39 days' file of
# sides with the oilers' two classes added (decision 0206, rule 17), which
# names every class and type this scenario uses.
class_sides = "../../reference/class_sides_five_days_oilers.csv"
'''

TF_BEFORE = '''# ---------------------------------------------------------------------------
# SHIPS. Ships 1 to 36 are scenario 5's, as it gives them.
'''
TF_NEW = '''# The oilers, Platte and Cimarron as one force (decision 0206, rule 20), on
# the transport mission, with no home port, no route (stopped) and no escort.
# PLACEHOLDER start (7,10), the hex of Efate, at 1800 on 22 August. The record
# has both at "Roses" until they sailed at daylight on 23 August: the
# dispatch of 22 August sends them "from Roses" (ship_facts 14419 and 14420),
# and the fleet's own war diary says Roses is Efate (place_facts 14306 and
# 14307; the post-war list of code words, 14300, agrees). They were to meet
# the carriers at 0800 on 25 August at 13-04 S, 164-03.5 E, hex (5,2)
# (engagement_positions 14300 and 14301; row 279 gives the same point for the
# force). Their escorts are named in no source read.
[[task_force]]
id = 6
name = "Oilers"
side = "allied"
q = 7
r = 10
mission = "transport"

'''

SHIPS_BEFORE = '''# Suzukaze, class 3012, Shiratsuyu class: engagement_forces 147; ship_facts
# 2244.
[[ship]]
id = 43
name = "Suzukaze"
class = 3012
side = "japanese"
fuel_t = 492
task_force = 5

'''
SHIPS_NEW = '''# USS Platte (ao-24-platte; engagement_forces 14301 for the eastern-solomons
# engagement): class 2030, Cimarron class, an oiler
# (ship_classes_five_days_oilers.ships.csv). Fuel and cargo full, an
# assumption: her load on 22 August is a gap. No experience: no ship of the
# five days has one.
[[ship]]
id = 44
name = "USS Platte"
class = 2030
side = "allied"
fuel_t = 2274
cargo_fuel_t = "15408.3208"
task_force = 6

# USS Cimarron (ao-22-cimarron; engagement_forces 14300 for the
# eastern-solomons engagement): class 2031, Cimarron class, an oiler
# (ship_classes_five_days_oilers.ships.csv), a class of her own: her bunkers
# and cargo differ from Platte's. Fuel and cargo full, an
# assumption: her load on 22 August is a gap. No experience.
[[ship]]
id = 45
name = "USS Cimarron"
class = 2031
side = "allied"
fuel_t = "2251.26"
cargo_fuel_t = "15650.1228"
task_force = 6

'''


def once(text, old_text, new_text):
    if text.count(old_text) != 1:
        raise SystemExit(f'the old base does not hold exactly once: {old_text[:60]!r}')
    return text.replace(old_text, new_text)


def make_base(old_base):
    text = once(old_base, old.NAME_OLD, NAME_NEW)
    text = once(text, FIRST_PARA_OLD, FIRST_PARA_NEW)
    text = once(text, BULLET_OLD, BULLET_NEW)
    text = once(text, LEFT_OLD, LEFT_NEW)
    text = once(text, SHIP_CLASSES_COMMENT_OLD, SHIP_CLASSES_COMMENT_NEW)
    text = once(text, SHIP_CLASSES_OLD, SHIP_CLASSES_NEW)
    text = once(text, SIDES_OLD, SIDES_NEW)
    text = once(text, TF_BEFORE, TF_NEW + TF_BEFORE)
    divider = '# ---------------------------------------------------------------------------\n# AIR GROUPS.'
    text = once(text, SHIPS_BEFORE + divider, SHIPS_BEFORE + SHIPS_NEW + divider)
    return text


def header_for_files():
    h = old.HEADER
    pairs = [
        ('# SCENARIO 103, THE EASTERN SOLOMONS OVER FIVE DAYS (decision 0184).',
         '# SCENARIO 103 WITH THEIR OILERS, THE EASTERN SOLOMONS OVER FIVE DAYS\n# (decision 0206).'),
        ('scripts/five-days/make_files.py from the base,\n# data/scenarios/five-days/base_eastern_solomons.toml.',
         'scripts/five-days/make_oiler_files.py from the base,\n# data/scenarios/five-days/base_eastern_solomons_oilers.toml.'),
        ('python3 scripts/five-days/make_files.py --check',
         'python3 scripts/five-days/make_oiler_files.py --check'),
    ]
    for a, b in pairs:
        h = once(h, a, b)
    for letter in 'abcd':
        h = once(h, f'#   01{letter}:', f'#   02{letter}:')
    return h


def make(base, letter, allied, japanese):
    if base.count(NAME_NEW) != 1:
        raise SystemExit("the new base's name line is not the one expected")
    text = base.replace(NAME_NEW, (
        f'name = "Scenario 103 with their oilers, file {letter}: the Eastern Solomons over five days, '
        f'23 to 27 August 1942; {old.WORDS[letter]}"\n'))
    text = header_for_files() + text
    if not text.endswith('\n'):
        text += '\n'
    if allied:
        text += old.ALLIED
    if japanese:
        text += old.JAPANESE
    return text


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('repo', nargs='?',
                    default=os.path.dirname(os.path.dirname(HERE)))
    ap.add_argument('--check', action='store_true')
    a = ap.parse_args()
    with open(os.path.join(a.repo, OLD_BASE), encoding='utf-8') as f:
        old_base = f.read()
    new_base = make_base(old_base)
    outputs = [(NEW_BASE, new_base)]
    for name, letter, allied, japanese in FILES:
        outputs.append((f'{FOLDER}/{name}', make(new_base, letter, allied, japanese)))
    differ = 0
    for rel, text in outputs:
        path = os.path.join(a.repo, rel)
        if a.check:
            try:
                with open(path, encoding='utf-8') as f:
                    kept = f.read()
            except FileNotFoundError:
                kept = None
            same = kept == text
            differ += not same
            print(f'{rel}: ' + ('as made' if same else 'NOT as made: run the script without --check'))
        else:
            with open(path, 'w', encoding='utf-8') as f:
                f.write(text)
            print(f'wrote {rel}')
    return 1 if differ else 0


if __name__ == '__main__':
    sys.exit(main())

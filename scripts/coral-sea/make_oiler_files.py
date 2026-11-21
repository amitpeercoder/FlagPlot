"""Make the base and the four files of scenario 105 with her oiler, the Coral
Sea with Neosho and Sims (decision 0206), from the old base and the old
script, or check that the files kept are the ones it makes.

    python3 scripts/coral-sea/make_oiler_files.py [REPO] [--check]

REPO is the top of the repository (by default, three folders above this
file's folder's parent, as make_files.py has it). It reads
`data/scenarios/coral-sea/base_coral_sea.toml` as text and writes
`data/scenarios/coral-sea/base_coral_sea_oilers.toml` and
`02a_coral_sea_oilers.toml` to `02d_coral_sea_oilers.toml`. With `--check` it
writes nothing: it says whether each file kept is byte for byte the one it
would make, and exits 1 where one is not. Python 3.9 or later, standard
library only, deterministic.

It imports make_files.py and takes from it FILES, WORDS, HEADER, `allied` and
`japanese`, so the command and order tables are the old ones byte for byte.
The new base is the old base with exactly these changes: a new header, a new
name, the class file of the oilers named beside the others, its own file of
sides, one task force more (6) and two ships more (54, 55). Each of the four
files is the new base with a header and a name of its own and the tables of
its pairing, as in make_files.py.
"""
import argparse
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location('coral_sea_make_files', os.path.join(HERE, 'make_files.py'))
old = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(old)

FOLDER = old.FOLDER
OLD_BASE = old.BASE
NEW_BASE = 'data/scenarios/coral-sea/base_coral_sea_oilers.toml'

FILES = [(name.replace('01', '02', 1).replace('_coral_sea.toml', '_coral_sea_oilers.toml'),
          letter, a, j, risk) for name, letter, a, j, risk in old.FILES]

NAME_NEW = 'name = "Scenario 105 with her oiler, the base: the Coral Sea, 6 to 10 May 1942"\n'

BASE_HEADER = '''# THE BASE of scenario 105 with her oiler, the Coral Sea, 6 to 10 May 1942
# (decision 0206). It is not played itself: scripts/coral-sea/
# make_oiler_files.py makes it from base_coral_sea.toml and makes the four
# files of data/scenarios/coral-sea/ (02a to 02d) from it, each with this
# text, a name of its own and the commands and orders of its pairing, and
# nothing else. Edit the old base or the script, never a file it makes, and
# make them again. The old base, and the four files of scenario 105 that
# read it, are not touched (decision 0206, rule 17).
#
# It is the base of scenario 105 with one force more, the fuelling group
# (Neosho and Sims, task force 6), with the class file of the oilers,
# ship_classes_coral_sea_oilers.csv, named beside the others, and with a
# sides file of its own, class_sides_coral_sea_oilers.csv. The rest is the
# old base's, byte for byte. The hand calculation is the doc comment of
# tests/scenario_105_the_coral_sea.rs; that of the oiler,
# tests/scenario_120_real_oilers.rs.
#
# Two American carriers and a cruiser group against two Japanese fleet
# carriers and a light carrier, with a Japanese convoy bound for Port
# Moresby. No scripted order: each side is played by a commander or a staff.
# The game begins at 1800 on 5 May 1942; day 1 is 6 May and day 5 is 10
# May. Times are zone -11, the clock of the analysis of 1947. (A scenario
# file has no key for a date.)
#
# WHAT IT IS MADE OF, and where each piece comes from:
#   - Every real ship and aircraft type: the derivation list of the Coral
#     Sea (data/derivation/coral_sea.toml, as of 6 May 1942), its classes
#     4000 to 4024 and types 4100 to 4110, and each ship's experience from
#     ship_experience_coral_sea.ships.csv where it gives one. Each ship cites
#     her rows of engagement_forces: both sources agree on the forces ship
#     for ship.
#   - The fuelling group: Neosho and Sims, derived by
#     data/derivation/coral_sea_oilers.toml (classes 4030 and 4031), each
#     with her experience from ship_experience_coral_sea_oilers.ships.csv.
#     Neosho carries her cargo full, as every ship of the file starts with
#     her own fuel full; her true load on 5 May is a gap. No order is
#     scripted for her (decision 0206, rule 19): she is the commander's to
#     send, and the staff uses her by decision 0201, rule 20.
#   - The air groups' strengths: the rows of data/oob/air_unit_facts.csv
#     the hand calculation names, beside each group.
#   - Port Moresby (base 1) and its garrison; Rabaul (base 2) as the 39 days
#     gives it (data/scenarios/07_full_slice.toml).
#   - The convoy: Yubari as the list rates her, five placeholder destroyers
#     and ten placeholder transports of a placeholder class of this
#     scenario's own, with the placeholder men of two land units aboard.
#
# LEFT OUT, and why (decision 0188, rule 6 and stage A's rulings 12 and 13;
# decision 0206):
#   - Of the Allied fuelling group, Worden (also of the group in the
#     sources, and not in the list); and the search group (Tangier);
#     Tippecanoe (whether she was with the force on 6 May is disputed, and
#     there is no endurance for her class); Tsugaru, Toho Maru and every
#     Japanese oiler: no displacement for any of them is on file, so the
#     Japanese side has no oiler.
#   - Uzuki, which one source alone puts with the convoy.
#   - Allied aircraft on land (the designer's answer 2: left out of the
#     first version; the Allied carriers are then the only Allied eyes).
#   - Submarines, as everywhere; the Japanese seaplanes at Deboyne and
#     Tulagi and the force that set them up; the flying boats at Rabaul and
#     Shortland, so the Japanese side has no search from land; the Tulagi
#     force; the Tainan Air Group's six fighters at Lae, with Lae.
#   - The convoy's escort of the Kamikaze and Mutsuki classes (Oite,
#     Asanagi, Mutsuki, Mochizuki, Yayoi) as real ships: no row gives the
#     rounds a gun carried, so they stand as placeholder destroyers. Yubari
#     alone of the escort is real.
#   - The real transports, which have no class on file: placeholders stand
#     for them.
#
# PLACEHOLDER DATA. Every start but the Allied one rests on a row's hour
# and a reading, and each start is marked a placeholder; the convoy's
# transports, destroyers, men, supply aboard and class; Port Moresby's
# garrison and stock; every point value and level. The figures files are
# the live placeholders. Neither side is given a believed force or an
# estimate of the other's men: no row gives what either believed.
#
# A port size: the map gives every hex none, and a scenario cannot set one,
# so Port Moresby and Rabaul have none.

'''

SHIP_CLASSES_OLD = '''# The Coral Sea list's classes; the placeholder destroyer of ship_classes.csv
# (class 2); and the convoy's placeholder transport (class 14).
ship_classes = [
    "../../reference/ship_classes_coral_sea.csv",
'''
SHIP_CLASSES_NEW = '''# The Coral Sea list's classes; the fuelling group's two (classes 4030 and
# 4031, ship_classes_coral_sea_oilers.csv, decision 0206); the placeholder
# destroyer of ship_classes.csv (class 2); and the convoy's placeholder
# transport (class 14).
ship_classes = [
    "../../reference/ship_classes_coral_sea.csv",
    "../../reference/ship_classes_coral_sea_oilers.csv",
'''

SIDES_OLD = '''# Whose each class and type is (decision 0173, 1 to 3): this scenario's own
# file of sides, which names every class and type of the three class files
# and the type file above.
class_sides = "../../reference/class_sides_coral_sea.csv"
'''
SIDES_NEW = '''# Whose each class and type is (decision 0173, 1 to 3): this scenario's own
# file of sides with the oiler's (decision 0206, rule 17), which names every
# class and type of the four class files and the type file above.
class_sides = "../../reference/class_sides_coral_sea_oilers.csv"
'''

TF_BEFORE = '''# ---------------------------------------------------------------------------
# SHIPS
# ---------------------------------------------------------------------------
'''
TF_NEW = '''# The fuelling group, Neosho and Sims as one force (decision 0206, rule 18),
# on the transport mission, with no home port, no route and no escort. Sims
# is in the force and not an escort to it, since the staff sends no force to
# an oiler's force that has an escort. PLACEHOLDER start (-3,5), the hex of
# Task Force 17: the rows say Task Force 17 fuelled from Neosho through 5 May
# (ship_facts 12908) and that the two were detached together only at 1725 on
# 6 May (ship_facts 12910), so they were with Task Force 17 at 1800 on 5 May.
[[task_force]]
id = 6
name = "Fuelling group"
side = "allied"
q = -3
r = 5
mission = "transport"

'''

SHIPS_BEFORE = '''[[ship]]
id = 53
name = "Placeholder transport 10"
class = 14
side = "japanese"
fuel_t = 1000
supply_t = 41
task_force = 5

'''
SHIPS_NEW = '''# USS Neosho (ao-23-neosho; engagement_forces 485, 512): class 4030, Cimarron
# class, an oiler (ship_classes_coral_sea_oilers.ships.csv). Fuel and cargo
# full, an assumption: her true load on 5 May is a gap. Experience 2:
# ship_experience_coral_sea_oilers.ships.csv (ship_facts.csv:3384).
[[ship]]
id = 54
name = "USS Neosho"
class = 4030
side = "allied"
fuel_t = 2274
cargo_fuel_t = "15408.3208"
experience = 2
task_force = 6

# USS Sims (dd-409-sims; engagement_forces 487, 514): class 4031, Sims class,
# surface weapons not modelled (ship_classes_coral_sea_oilers.ships.csv). Fuel
# full, an assumption. Experience 2: ship_experience_coral_sea_oilers.ships.csv
# (ship_facts.csv:3354).
[[ship]]
id = 55
name = "USS Sims"
class = 4031
side = "allied"
fuel_t = 453
experience = 2
task_force = 6

'''


def once(text, old_text, new_text):
    if text.count(old_text) != 1:
        raise SystemExit(f'the old base does not hold exactly once: {old_text[:60]!r}')
    return text.replace(old_text, new_text)


def make_base(old_base):
    text = once(old_base, old.NAME_OLD, '\0')
    head, tail = text.split('\0')
    if 'name = ' in head:
        raise SystemExit('a name line before the old name')
    text = BASE_HEADER + NAME_NEW + tail
    text = once(text, SHIP_CLASSES_OLD, SHIP_CLASSES_NEW)
    text = once(text, SIDES_OLD, SIDES_NEW)
    text = once(text, TF_BEFORE, TF_NEW + TF_BEFORE)
    # the ships end where the air groups' divider begins
    divider = '# ---------------------------------------------------------------------------\n# AIR GROUPS.'
    text = once(text, SHIPS_BEFORE + divider, SHIPS_BEFORE + SHIPS_NEW + divider)
    return text


def header_for_files():
    h = old.HEADER
    pairs = [
        ('# SCENARIO 105, THE CORAL SEA (decision 0188).',
         '# SCENARIO 105 WITH HER OILER, THE CORAL SEA (decision 0206).'),
        ('scripts/coral-sea/make_files.py from the base,\n# data/scenarios/coral-sea/base_coral_sea.toml.',
         'scripts/coral-sea/make_oiler_files.py from the base,\n# data/scenarios/coral-sea/base_coral_sea_oilers.toml.'),
        ('python3 scripts/coral-sea/make_files.py --check',
         'python3 scripts/coral-sea/make_oiler_files.py --check'),
    ]
    for a, b in pairs:
        h = once(h, a, b)
    for letter in 'abcd':
        h = once(h, f'#   01{letter}:', f'#   02{letter}:')
    return h


def make(base, letter, given_allied, given_japanese, risk):
    if base.count(NAME_NEW) != 1:
        raise SystemExit("the new base's name line is not the one expected")
    text = base.replace(NAME_NEW, (
        f'name = "Scenario 105 with her oiler, file {letter}: the Coral Sea, 6 to 10 May 1942; '
        f'{old.WORDS[letter]}"\n'))
    text = header_for_files() + text
    if not text.endswith('\n'):
        text += '\n'
    if given_allied:
        text += old.allied(risk)
    if given_japanese:
        text += old.japanese(risk)
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
    for name, letter, given_allied, given_japanese, risk in FILES:
        outputs.append((f'{FOLDER}/{name}', make(new_base, letter, given_allied, given_japanese, risk)))
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

"""Make the base and the four files of scenario 105 with both oilers, the Coral
Sea with Neosho and Sims and with Toho Maru (decisions 0206 and 0207), from the
base of the oilers and the old scripts, or check that the files kept are the
ones it makes.

    python3 scripts/coral-sea/make_both_oilers_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this file's
folder, as make_oiler_files.py has it). It reads
`data/scenarios/coral-sea/base_coral_sea_oilers.toml` as text and writes
`data/scenarios/coral-sea/base_coral_sea_both_oilers.toml` and
`03a_coral_sea_both_oilers.toml` to `03d_coral_sea_both_oilers.toml`. With
`--check` it writes nothing: it says whether each file kept is byte for byte
the one it would make, and exits 1 where one is not. Python 3.9 or later,
standard library only, deterministic.

It imports make_oiler_files.py (which imports make_files.py) and takes from
them FILES, WORDS, `allied` and `japanese`, so the command and order tables
are the old ones byte for byte. The new base is the oilers' base with exactly
these changes: a new header, a new name, the class file of the Japanese oiler
named after the oilers' class file, its own file of sides, one task force more
(7) and one ship more (56). Each of the four files is the new base with a
header and a name of its own and the tables of its pairing, as in
make_oiler_files.py.
"""
import argparse
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location('coral_sea_make_oiler_files', os.path.join(HERE, 'make_oiler_files.py'))
oil = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oil)
old = oil.old

FOLDER = old.FOLDER
OLD_BASE = oil.NEW_BASE
NEW_BASE = 'data/scenarios/coral-sea/base_coral_sea_both_oilers.toml'

FILES = [(name.replace('02', '03', 1).replace('_coral_sea_oilers.toml', '_coral_sea_both_oilers.toml'),
          letter, a, j, risk) for name, letter, a, j, risk in oil.FILES]

NAME_NEW = 'name = "Scenario 105 with both oilers, the base: the Coral Sea, 6 to 10 May 1942"\n'

BASE_HEADER = '''# THE BASE of scenario 105 with both oilers, the Coral Sea, 6 to 10 May 1942
# (decisions 0206 and 0207). It is not played itself: scripts/coral-sea/
# make_both_oilers_files.py makes it from base_coral_sea_oilers.toml and makes
# the four files of data/scenarios/coral-sea/ (03a to 03d) from it, each with
# this text, a name of its own and the commands and orders of its pairing, and
# nothing else. Edit the oilers' base or the scripts, never a file they make,
# and make them again. The old base, the oilers' base, and the files of
# scenario 105 that read them, are not touched.
#
# It is the oilers' base with one force more, the Japanese fuelling group
# (Toho Maru, task force 7), with the class file of her class,
# ship_classes_coral_sea_japanese_oilers.csv, named after the oilers' class
# file, and with a sides file of its own, class_sides_coral_sea_both_oilers.csv.
# The rest is the oilers' base's, byte for byte. The hand calculation of the
# base is the doc comment of tests/scenario_105_the_coral_sea.rs; that of the
# first oiler, tests/scenario_120_real_oilers.rs; that of the second,
# tests/scenario_121_a_japanese_oiler_by_estimate.rs.
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
#   - The Allied fuelling group: Neosho and Sims, derived by
#     data/derivation/coral_sea_oilers.toml (classes 4030 and 4031), each
#     with her experience from ship_experience_coral_sea_oilers.ships.csv.
#     Neosho carries her cargo full, as every ship of the file starts with
#     her own fuel full; her true load on 5 May is a gap. Neosho's cargo
#     capacity is an estimate too (the least of her sisters', decision
#     0207), marked in her class file.
#   - The Japanese fuelling group: Toho Maru, derived by
#     data/derivation/coral_sea_japanese_oilers.toml, class 4032, with her
#     experience (2) from ship_experience_coral_sea_japanese_oilers.ships.csv.
#     HER LIGHT DISPLACEMENT, AND SO HER TOUGHNESS, IS ESTIMATED BY RATIO
#     FROM LIKE SHIPS, AND HER CARGO IS ESTIMATED AS HER SISTER TOA MARU'S
#     (decision 0207): no source gives either for her. Her class file names
#     the estimated figures in its column `estimated`, and her sources file
#     says how each was worked. AN ESTIMATE IS NEVER EVIDENCE (decision 0207,
#     rule 12): nothing in this scenario or in a measure that names it may
#     be taken as the record of her toughness or her cargo. HER LOAD ON 5
#     MAY IS NOT KNOWN, a gap: she starts full, as every ship of the file
#     does. Her anti-aircraft guns are unknown and are taken as none, and her
#     radar is unknown and is taken as none: assumptions of the scenario, not
#     history. The record has her with the Striking Force from 1 May (she
#     sortied from Truk with it) and fuelling Takagi's ships on 4 May and
#     Carrier Division 5 on 6 May (ship_facts 3503, a combinedfleet record of
#     her movements; engagement_forces 414, Bates' organization of the
#     Japanese 4th Fleet, which lists her with the Striking Force, and 450,
#     the combinedfleet Zuikaku record of the sortie of 1 May). No order is
#     scripted for her: she is the Japanese commander's to send, and the staff
#     uses her by decision 0201, rule 20.
#   - The air groups' strengths: the rows of data/oob/air_unit_facts.csv
#     the hand calculation names, beside each group.
#   - Port Moresby (base 1) and its garrison; Rabaul (base 2) as the 39 days
#     gives it (data/scenarios/07_full_slice.toml).
#   - The convoy: Yubari as the list rates her, five placeholder destroyers
#     and ten placeholder transports of a placeholder class of this
#     scenario's own, with the placeholder men of two land units aboard.
#
# LEFT OUT, and why (decision 0188, rule 6 and stage A's rulings 12 and 13;
# decisions 0206 and 0207):
#   - Of the Allied fuelling group, Worden (also of the group in the
#     sources, and not in the list); and the search group (Tangier);
#     Tippecanoe (whether she was with the force on 6 May is disputed, and
#     there is no endurance for her class).
#   - Tsugaru and every other Japanese oiler but Toho Maru: none is derived
#     (no list names one); Toho Maru is the one the first use of estimates
#     settled (decision 0207).
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

SHIP_CLASSES_OLD = '''# The Coral Sea list's classes; the fuelling group's two (classes 4030 and
# 4031, ship_classes_coral_sea_oilers.csv, decision 0206); the placeholder
# destroyer of ship_classes.csv (class 2); and the convoy's placeholder
# transport (class 14).
ship_classes = [
    "../../reference/ship_classes_coral_sea.csv",
    "../../reference/ship_classes_coral_sea_oilers.csv",
'''
SHIP_CLASSES_NEW = '''# The Coral Sea list's classes; the Allied fuelling group's two (classes 4030
# and 4031, ship_classes_coral_sea_oilers.csv, decision 0206); the Japanese
# fuelling group's one (class 4032, ship_classes_coral_sea_japanese_oilers.csv,
# decision 0207, with figures estimated and named in its column `estimated`);
# the placeholder destroyer of ship_classes.csv (class 2); and the convoy's
# placeholder transport (class 14).
ship_classes = [
    "../../reference/ship_classes_coral_sea.csv",
    "../../reference/ship_classes_coral_sea_oilers.csv",
    "../../reference/ship_classes_coral_sea_japanese_oilers.csv",
'''

SIDES_OLD = '''# Whose each class and type is (decision 0173, 1 to 3): this scenario's own
# file of sides with the oiler's (decision 0206, rule 17), which names every
# class and type of the four class files and the type file above.
class_sides = "../../reference/class_sides_coral_sea_oilers.csv"
'''
SIDES_NEW = '''# Whose each class and type is (decision 0173, 1 to 3): this scenario's own
# file of sides with the oilers' (decisions 0206 and 0207), which names every
# class and type of the five class files and the type file above.
class_sides = "../../reference/class_sides_coral_sea_both_oilers.csv"
'''

TF_BEFORE = '''# ---------------------------------------------------------------------------
# SHIPS
# ---------------------------------------------------------------------------
'''
TF_NEW = '''# The Japanese fuelling group, Toho Maru by herself as one force (decision
# 0207), on the transport mission, with no home port, no route and no escort.
# PLACEHOLDER start (4,-2), the hex of the Striking Force: the rows put her
# with the Striking Force from 1 May, fuelling Takagi's ships on 4 May and
# Carrier Division 5 on 6 May (ship_facts 3503; engagement_forces 414, 450),
# so she was with it at 1800 on 5 May. No order is scripted for her.
[[task_force]]
id = 7
name = "Japanese fuelling group"
side = "japanese"
q = 4
r = -2
mission = "transport"

'''

SHIPS_BEFORE = '''# AIR GROUPS.'''
SHIPS_NEW = '''# Toho Maru (toho-maru; engagement_forces 414, 450): class 4032, Toa Maru
# class, an oiler (ship_classes_coral_sea_japanese_oilers.ships.csv). Her
# light displacement (so her toughness) and her cargo are ESTIMATED, not read
# (decision 0207): the class file's column `estimated` names them. Fuel and
# cargo full, an assumption: her true load on 5 May is a gap. Experience 2:
# ship_experience_coral_sea_japanese_oilers.ships.csv (rows ship_facts.csv:3476
# and 3499).
[[ship]]
id = 56
name = "Toho Maru"
class = 4032
side = "japanese"
fuel_t = 1100
cargo_fuel_t = 12487
experience = 2
task_force = 7

'''


def once(text, old_text, new_text):
    if text.count(old_text) != 1:
        raise SystemExit(f'the oilers base does not hold exactly once: {old_text[:60]!r}')
    return text.replace(old_text, new_text)


def make_base(oilers_base):
    text = once(oilers_base, oil.NAME_NEW, '\0')
    head, tail = text.split('\0')
    if 'name = ' in head:
        raise SystemExit('a name line before the oilers base name')
    for used in ('\nid = 7\nname = "Japanese', '\nid = 56\n'):
        if used in tail:
            raise SystemExit('an id the new base adds is already used')
    text = BASE_HEADER + NAME_NEW + tail
    text = once(text, SHIP_CLASSES_OLD, SHIP_CLASSES_NEW)
    text = once(text, SIDES_OLD, SIDES_NEW)
    text = once(text, TF_BEFORE, TF_NEW + TF_BEFORE)
    divider = '# ---------------------------------------------------------------------------\n# AIR GROUPS.'
    text = once(text, divider, SHIPS_NEW + divider)
    return text


def header_for_files():
    h = oil.header_for_files()
    pairs = [
        ('# SCENARIO 105 WITH HER OILER, THE CORAL SEA (decision 0206).',
         '# SCENARIO 105 WITH BOTH OILERS, THE CORAL SEA (decisions 0206 and 0207).'),
        ('scripts/coral-sea/make_oiler_files.py from the base,\n# data/scenarios/coral-sea/base_coral_sea_oilers.toml.',
         'scripts/coral-sea/make_both_oilers_files.py from the base,\n# data/scenarios/coral-sea/base_coral_sea_both_oilers.toml.'),
        ('python3 scripts/coral-sea/make_oiler_files.py --check',
         'python3 scripts/coral-sea/make_both_oilers_files.py --check'),
        ('# Each is the base with this header,',
         '# The file loads a class with ESTIMATED figures: Toho Maru\'s light\n'
         '# displacement (so her toughness) and her cargo (decision 0207; her class\n'
         '# file names them in its column `estimated`), and Neosho\'s cargo is marked\n'
         '# too. An estimate is never evidence. Toho Maru\'s load on 5 May is not\n'
         '# known: she starts full, as every ship of the file does.\n'
         '#\n'
         '# Each is the base with this header,'),
    ]
    for a, b in pairs:
        h = once(h, a, b)
    for letter in 'abcd':
        h = once(h, f'#   02{letter}:', f'#   03{letter}:')
    return h


def make(base, letter, given_allied, given_japanese, risk):
    if base.count(NAME_NEW) != 1:
        raise SystemExit("the new base's name line is not the one expected")
    text = base.replace(NAME_NEW, (
        f'name = "Scenario 105 with both oilers, file {letter}: the Coral Sea, 6 to 10 May 1942; '
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
        oilers_base = f.read()
    new_base = make_base(oilers_base)
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

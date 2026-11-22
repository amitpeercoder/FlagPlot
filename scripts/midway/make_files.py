"""Make the four files of scenario 114, Midway (decision 0198), from its
base, or check that the files kept are the ones it makes.

    python3 scripts/midway/make_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It reads `data/scenarios/midway/base_midway.toml` as text and writes
`data/scenarios/midway/01a_midway.toml` to `01d_midway.toml`.
With `--check` it writes nothing: it says whether each file kept is byte for
byte the one it would make, and exits 1 where one is not. Python 3.9 or
later, standard library only, deterministic. No build step runs it; the
scenario's test runs it with `--check`. Run it by hand whenever the base or
this script changes.

Each file is the base with exactly these changes, and nothing else:
  1. the header below, the same in all four, put before the base;
  2. its `name`;
  3. at the end, the `[[command]]` tables of its pairing (decision 0185): both
     sides given their orders (a), the Japanese side alone (b), the Allied
     side alone (c), neither (d). The orders are the hand calculation's
     ("What a side the engine plays is given"), on day 1.
So with their names and their command tables taken away the four are the
same, byte for byte.

Files of the same orders at another risk may be added later: give a row of
FILES a risk ("preserve" or "press"), and each intent of a side given orders
gets a `set_limit` of that risk after its other commands. The four files
here have none (balanced).
"""
import argparse
import os
import sys

BASE = 'data/scenarios/midway/base_midway.toml'
FOLDER = 'data/scenarios/midway'

FILES = [
    # (file, letter, Allied given orders, Japanese given orders, risk)
    ('01a_midway.toml', 'a', True, True, None),
    ('01b_midway.toml', 'b', False, True, None),
    ('01c_midway.toml', 'c', True, False, None),
    ('01d_midway.toml', 'd', False, False, None),
]

NAME_OLD = 'name = "Scenario 114, the base: Midway, 3 to 7 June 1942"\n'

WORDS = {
    'a': 'both sides given their orders',
    'b': 'the Japanese side given its orders, the Allied side none',
    'c': 'the Allied side given its orders, the Japanese side none',
    'd': 'neither side given orders',
}

HEADER = '''# SCENARIO 114, MIDWAY (decision 0198).
#
# Made by scripts/midway/make_files.py from the base,
# data/scenarios/midway/base_midway.toml. Do not edit by hand: change the
# base or the script and make the files again
# (`python3 scripts/midway/make_files.py --check` says whether each file is
# still the one it makes). There are four files, one for each pairing the
# table offers (decision 0185):
#   01a: both sides given their orders (staff against staff);
#   01b: the Japanese side given its orders, the Allied side none (a person
#        or an agent takes the Allied seat);
#   01c: the Allied side given its orders, the Japanese side none;
#   01d: neither side given orders.
# Each is the base with this header, a name of its own and, at the end, the
# `[[command]]` tables of the sides it gives orders, and nothing else: with
# their names and those tables taken away the four are the same, byte for
# byte. The orders are the hand calculation's (tests/scenario_114_midway.rs,
# "What a side the engine plays is given"), on day 1, with no risk set
# (balanced):
#   Japanese: intent 1, take Midway, the place (5,1), its forces the First
#     Carrier Striking Force and the Midway Invasion Force (task forces 3
#     and 4) and the landing force (land units 2 and 3), with no need to
#     know (a knowledge limit of none); the carriers are in the take's
#     forces and have no order of their own (rule 38). Leave to prepare and
#     leave to launch given. No estimate of the men at Midway is held
#     (rule 96).
#   Allied: intent 1, hold Midway (base 1), its forces the garrison (land
#     unit 1), VMF-221's two fighter groups (12, 13) and the two groups of
#     flying boats (16, 17) (rule 97); intent 2, deny the waters within 4
#     hexes of (1,-1), its forces the SBD-2, SB2U-3 and B-17E groups (14,
#     15, 18), with the targets limit carriers, then transports,
#     battleships, cruisers, destroyers (rule 98); intent 3, operate task
#     force 1 from (14,-6), Point Luck; intent 4, operate task force 2 from
#     (15,-6) (rule 99), each with its own air groups (no forces limit).
#     Leave to prepare and leave to launch given. No order of a commander's
#     own.
#
# Kept out of the loops that walk data/scenarios/*.toml by being in this
# folder (decision 0072).
#
'''


def command(side, kind, **keys):
    lines = ['', '[[command]]', 'day = 1', f'side = "{side}"', f'kind = "{kind}"']
    for key, value in keys.items():
        lines.append(f'{key} = {value}')
    return '\n'.join(lines) + '\n'


def order(side, target, kind, **keys):
    lines = ['', '[[order]]', 'day = 1', f'side = "{side}"', target, f'kind = "{kind}"']
    for key, value in keys.items():
        lines.append(f'{key} = {value}')
    return '\n'.join(lines) + '\n'


def risk_limits(side, intents, risk):
    if risk is None:
        return ''
    return ''.join(
        command(side, 'set_limit', limit='"risk"', intent=i, risk=f'"{risk}"') for i in intents
    )


def leaves(side):
    return (command(side, 'set_leave', leave='"prepare"', given='true')
            + command(side, 'set_leave', leave='"launch"', given='true'))


def allied(risk):
    s = 'allied'
    text = command(s, 'set_intent', intent=1, intent_kind='"hold"', base=1)
    text += command(s, 'set_limit', limit='"forces"', intent=1, air_groups='[12, 13, 16, 17]',
                    land_units='[1]')
    text += command(s, 'set_intent', intent=2, intent_kind='"deny"', hex='[1, -1]', distance=4)
    text += command(s, 'set_limit', limit='"forces"', intent=2, air_groups='[14, 15, 18]')
    text += command(s, 'set_limit', limit='"targets"', intent=2,
                    kinds='["carrier", "transport", "battleship", "cruiser", "destroyer"]')
    text += command(s, 'set_intent', intent=3, intent_kind='"operate_carriers"',
                    task_force=1, hex='[14, -6]')
    text += command(s, 'set_intent', intent=4, intent_kind='"operate_carriers"',
                    task_force=2, hex='[15, -6]')
    text += risk_limits(s, [1, 2, 3, 4], risk)
    text += leaves(s)
    return text


def japanese(risk):
    s = 'japanese'
    text = command(s, 'set_intent', intent=1, intent_kind='"take_place"', hex='[5, 1]')
    text += command(s, 'set_limit', limit='"forces"', intent=1, task_forces='[3, 4]',
                    land_units='[2, 3]')
    text += command(s, 'set_limit', limit='"knowledge"', intent=1, knowledge='"none"')
    text += risk_limits(s, [1], risk)
    text += leaves(s)
    return text


def make(base, letter, given_allied, given_japanese, risk):
    if base.count(NAME_OLD) != 1:
        raise SystemExit("the base's name line is not the one expected")
    if '[[command]]' in base or '[[order]]' in base:
        raise SystemExit('the base already has a command or an order')
    text = base.replace(NAME_OLD, (
        f'name = "Scenario 114, file {letter}: Midway, 3 to 7 June 1942; '
        f'{WORDS[letter]}"\n'))
    text = HEADER + text
    if not text.endswith('\n'):
        text += '\n'
    if given_allied:
        text += allied(risk)
    if given_japanese:
        text += japanese(risk)
    return text


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('repo', nargs='?',
                    default=os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    ap.add_argument('--check', action='store_true')
    a = ap.parse_args()
    with open(os.path.join(a.repo, BASE), encoding='utf-8') as f:
        base = f.read()
    differ = 0
    for name, letter, given_allied, given_japanese, risk in FILES:
        text = make(base, letter, given_allied, given_japanese, risk)
        path = os.path.join(a.repo, FOLDER, name)
        if a.check:
            try:
                with open(path, encoding='utf-8') as f:
                    kept = f.read()
            except FileNotFoundError:
                kept = None
            same = kept == text
            differ += not same
            print(f'{FOLDER}/{name}: ' + ('as made' if same else 'NOT as made: run the script without --check'))
        else:
            with open(path, 'w', encoding='utf-8') as f:
                f.write(text)
            print(f'wrote {FOLDER}/{name}')
    return 1 if differ else 0


if __name__ == '__main__':
    sys.exit(main())

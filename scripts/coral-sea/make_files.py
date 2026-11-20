"""Make the four files of scenario 105, the Coral Sea (decision 0188), from
its base, or check that the files kept are the ones it makes.

    python3 scripts/coral-sea/make_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It reads `data/scenarios/coral-sea/base_coral_sea.toml` as text and
writes `data/scenarios/coral-sea/01a_coral_sea.toml` to `01d_coral_sea.toml`.
With `--check` it writes nothing: it says whether each file kept is byte for
byte the one it would make, and exits 1 where one is not. Python 3.9 or
later, standard library only, deterministic. No build step runs it; the
scenario's test runs it with `--check`. Run it by hand whenever the base or
this script changes.

Each file is the base with exactly these changes, and nothing else:
  1. the header below, the same in all four, put before the base;
  2. its `name`;
  3. at the end, the `[[command]]` and `[[order]]` tables of its pairing
     (decision 0185): both sides given their orders (a), the Japanese side
     alone (b), the Allied side alone (c), neither (d). The orders are the
     hand calculation's ("What a side the engine plays is given"), on day 1.
So with their names and their command and order tables taken away the four
are the same, byte for byte.

Files of the same orders at another risk may be added later: give a row of
FILES a risk ("preserve" or "press"), and each intent of a side given orders
gets a `set_limit` of that risk after its other commands. The four files
here have none (balanced).
"""
import argparse
import os
import sys

BASE = 'data/scenarios/coral-sea/base_coral_sea.toml'
FOLDER = 'data/scenarios/coral-sea'

FILES = [
    # (file, letter, Allied given orders, Japanese given orders, risk)
    ('01a_coral_sea.toml', 'a', True, True, None),
    ('01b_coral_sea.toml', 'b', False, True, None),
    ('01c_coral_sea.toml', 'c', True, False, None),
    ('01d_coral_sea.toml', 'd', False, False, None),
]

NAME_OLD = 'name = "Scenario 105, the base: the Coral Sea, 6 to 10 May 1942"\n'

WORDS = {
    'a': 'both sides given their orders',
    'b': 'the Japanese side given its orders, the Allied side none',
    'c': 'the Allied side given its orders, the Japanese side none',
    'd': 'neither side given orders',
}

HEADER = '''# SCENARIO 105, THE CORAL SEA (decision 0188).
#
# Made by scripts/coral-sea/make_files.py from the base,
# data/scenarios/coral-sea/base_coral_sea.toml. Do not edit by hand: change
# the base or the script and make the files again
# (`python3 scripts/coral-sea/make_files.py --check` says whether each file is
# still the one it makes). There are four files, one for each pairing the
# table offers (decision 0185):
#   01a: both sides given their orders (staff against staff);
#   01b: the Japanese side given its orders, the Allied side none (a person
#        or an agent takes the Allied seat);
#   01c: the Allied side given its orders, the Japanese side none;
#   01d: neither side given orders.
# Each is the base with this header, a name of its own and, at the end, the
# `[[command]]` and `[[order]]` tables of the sides it gives orders, and
# nothing else: with their names and those tables taken away the four are
# the same, byte for byte. The orders are the hand calculation's
# (tests/scenario_105_the_coral_sea.rs, "What a side the engine plays is
# given"), on day 1, with no risk set (balanced):
#   Japanese: intent 1, take the place (-17,-4), its forces the invasion
#     force (task force 5) and its two land units (2, 3); intent 2, operate
#     task force 3 from (-4,1); intent 3, operate task force 4 from (-10,-2),
#     each with its own air groups (no forces limit); intent 4, deny the
#     waters within 2 hexes of (-12,-3), its forces the groups at Rabaul
#     (18, 19, 20); leave to prepare and leave to launch given. No estimate
#     of the garrison is held.
#   Allied: intent 1, hold Port Moresby (base 1), its forces the garrison
#     (land unit 1); intent 2, operate task force 1 from (-12,1), with its
#     own air groups (no forces limit); leave to prepare and leave to launch
#     given; and two orders of the commander's own, both to task force 2:
#     to move to (-12,-3), and to steam at 20 knots.
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
    text += command(s, 'set_limit', limit='"forces"', intent=1, land_units='[1]')
    text += command(s, 'set_intent', intent=2, intent_kind='"operate_carriers"',
                    task_force=1, hex='[-12, 1]')
    text += risk_limits(s, [1, 2], risk)
    text += leaves(s)
    text += order(s, 'task_force = 2', 'set_route', waypoints='[[-12, -3]]')
    text += order(s, 'task_force = 2', 'set_speed', knots=20)
    return text


def japanese(risk):
    s = 'japanese'
    text = command(s, 'set_intent', intent=1, intent_kind='"take_place"', hex='[-17, -4]')
    text += command(s, 'set_limit', limit='"forces"', intent=1, task_forces='[5]',
                    land_units='[2, 3]')
    text += command(s, 'set_intent', intent=2, intent_kind='"operate_carriers"',
                    task_force=3, hex='[-4, 1]')
    text += command(s, 'set_intent', intent=3, intent_kind='"operate_carriers"',
                    task_force=4, hex='[-10, -2]')
    text += command(s, 'set_intent', intent=4, intent_kind='"deny"', hex='[-12, -3]',
                    distance=2)
    text += command(s, 'set_limit', limit='"forces"', intent=4, air_groups='[18, 19, 20]')
    text += risk_limits(s, [1, 2, 3, 4], risk)
    text += leaves(s)
    return text


def make(base, letter, given_allied, given_japanese, risk):
    if base.count(NAME_OLD) != 1:
        raise SystemExit("the base's name line is not the one expected")
    if '[[command]]' in base or '[[order]]' in base:
        raise SystemExit('the base already has a command or an order')
    text = base.replace(NAME_OLD, (
        f'name = "Scenario 105, file {letter}: the Coral Sea, 6 to 10 May 1942; '
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

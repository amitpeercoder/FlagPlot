"""Make the three arrangement files of stage 3's proof (decision 0153) from
the staff's setup, or check that the files kept are the ones it makes.

    python3 make_arrangements.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It reads `data/scenarios/staff/07_full_slice.toml` as text and
writes `data/scenarios/stage-3-proof/07a_slice.toml`, `07b_slice.toml` and
`07c_slice.toml`. With `--check` it writes nothing: it says whether each file
kept is byte for byte the one it would make, and exits 1 where one is not.
Python 3.9 or later, standard library only, deterministic. No build step and
no test runs it; it is run by hand whenever the staff's setup changes, before
the proof is run.

Each file is the staff's setup with exactly these changes, and nothing else:
  1. a header, and its `name`;
  2. task force 6, "Placeholder night force A", restated by the architect's
     ruling in decision 0153 ("The loading check"): stopped at Shortland
     (base 7), its home port Shortland, on the bombardment mission, in place of
     its hex in the open Coral Sea on the surface combat mission; its day
     (`available_from = 18`) and its ships are as they were;
  3. the two islands of decision 0153, "The islands";
  4. the commands of decision 0153, "The intents as stated", for each side
     the arrangement gives intents, on day 1.
"""
import argparse
import os
import sys

STAFF = 'data/scenarios/staff/07_full_slice.toml'
FOLDER = 'data/scenarios/stage-3-proof'

ARRANGEMENTS = [
    # (file, letter, Allied from intents, Japanese from intents)
    ('07a_slice.toml', 'a', True, True),
    ('07b_slice.toml', 'b', False, True),
    ('07c_slice.toml', 'c', True, False),
]

NAME_OLD = ('name = "Scenario 7, the staff\'s setup: the full slice, 7 August to '
            '14 September 1942, no scripted order"\n')

FORCE_A_OLD = '''[[task_force]]
id = 6
name = "Placeholder night force A"
side = "japanese"
# Day 18 (24 August): made up (placeholder): night force A, first ordered to
# battle readiness on day 18 (scenario 7's comment at the night action).
available_from = 18
q = -8
r = 10
mission = "surface_combat"
'''

FORCE_A_NEW = '''[[task_force]]
id = 6
name = "Placeholder night force A"
side = "japanese"
# Day 18 (24 August): made up (placeholder): night force A, first ordered to
# battle readiness on day 18 (scenario 7's comment at the night action).
available_from = 18
# STAGE 3'S PROOF ONLY (decision 0153, "The loading check", the architect's
# ruling): restated as a placeholder bombardment force lying stopped at
# Shortland (base 7) with Shortland as its home port, in place of q = -8,
# r = 10 and the surface combat mission. Still a placeholder: no force of the
# record is named or rated by it.
at_base = 7
mission = "bombardment"
home_port = 7
'''

LETTER_WORDS = {
    'a': 'Allied intents and Japanese intents',
    'b': 'the Allied side given nothing, Japanese intents',
    'c': 'Allied intents, the Japanese side given nothing',
}

SERVES = {
    'a': 'conditions 1 to 4, 6 and 7, and both halves of condition 5',
    'b': 'the Allied half of condition 5',
    'c': 'the Japanese half of condition 5',
}


def header(letter, allied, japanese):
    def side_words(name, given):
        return (f'{name}: intents 1 and 2 on day 1, below' if given
                else f'{name}: given nothing (no intent, no order; its staff does nothing)')
    return f'''# STAGE 3'S PROOF, decision 0153, arrangement ({letter}): {LETTER_WORDS[letter]}.
# It serves {SERVES[letter]}.
#
# Made by scripts/stage-3-proof/make_arrangements.py from the staff's setup,
# data/scenarios/staff/07_full_slice.toml, by the smallest change. Do not edit
# by hand: change the script or the staff's setup and make it again
# (`python3 scripts/stage-3-proof/make_arrangements.py --check` says whether
# this file is still the one it makes). It differs from the staff's setup in
# these four things and nothing else:
#   1. This header and the `name`.
#   2. Task force 6, "Placeholder night force A" (Japanese, two placeholder
#      cruisers, ships 40 and 41, from day 18): stopped at Shortland (base 7),
#      home port Shortland, on the bombardment mission, in place of its hex
#      (-8,10) in the open Coral Sea on the surface combat mission. The
#      architect's ruling in decision 0153, "The loading check": the setup's
#      surface forces lie 14 to 18 hexes from the airfield and none is on the
#      bombardment mission, so no staff could bombard or reach the airfield's
#      waters in a night, which the record's Japanese ships did from Shortland.
#      Its day and its ships are unchanged. It stays a placeholder.
#   3. Two islands as data (decision 0153, "The islands"; decision 0151):
#      island 1, Guadalcanal, of Lunga (base 1); island 2, Tulagi, of Tulagi
#      (base 4). They are at the end of the file.
#   4. The intents as stated (decision 0153, "The intents as stated"), at the
#      end of the file, on day 1, with no forces limit and no risk set
#      (balanced), and both leaves given:
#        - {side_words('Allied', allied)};
#        - {side_words('Japanese', japanese)}.
#      Intent 1 is control island 1, Guadalcanal; intent 2 is control island
#      2, Tulagi. Nothing else is stated.
#
# Kept out of the loops that walk data/scenarios/*.toml by being in this
# folder (decision 0072): no loop of the tests, CI or scripts/ walks a
# subfolder of data/scenarios.
#
'''


ISLANDS = '''
# ---------------------------------------------------------------------------
# STAGE 3'S PROOF (decision 0153): the islands and the intents as stated.
# ---------------------------------------------------------------------------

# The two islands (decision 0153, "The islands"), each of a base the slice
# already has. Nothing else of the setup changes for them.
[[island]]
number = 1
name = "Guadalcanal"
places = [{ base = 1 }]

[[island]]
number = 2
name = "Tulagi"
places = [{ base = 4 }]
'''


def commands(side):
    words = 'Allied' if side == 'allied' else 'Japanese'
    return f'''
# The {words} intents of decision 0153, "The intents as stated", on day 1:
# no forces limit, no risk set (balanced), both leaves given.
[[command]]
day = 1
side = "{side}"
kind = "set_intent"
intent = 1
intent_kind = "control_island"
island = 1

[[command]]
day = 1
side = "{side}"
kind = "set_intent"
intent = 2
intent_kind = "control_island"
island = 2

[[command]]
day = 1
side = "{side}"
kind = "set_leave"
leave = "prepare"
given = true

[[command]]
day = 1
side = "{side}"
kind = "set_leave"
leave = "launch"
given = true
'''


def make(staff, letter, allied, japanese):
    if staff.count(NAME_OLD) != 1:
        raise SystemExit("the staff's setup's name line is not the one expected")
    if staff.count(FORCE_A_OLD) != 1:
        raise SystemExit("task force 6 in the staff's setup is not as expected: read the ruling again")
    if '[[island]]' in staff or '[[command]]' in staff or '[[order]]' in staff:
        raise SystemExit("the staff's setup already has an island, a command or an order")
    text = staff.replace(NAME_OLD, (
        f'name = "Stage 3\'s proof, arrangement ({letter}): {LETTER_WORDS[letter]}; '
        'the staff\'s setup of the full slice, 7 August to 14 September 1942"\n'))
    text = text.replace(FORCE_A_OLD, FORCE_A_NEW)
    text = header(letter, allied, japanese) + text
    if not text.endswith('\n'):
        text += '\n'
    text += ISLANDS
    if allied:
        text += commands('allied')
    if japanese:
        text += commands('japanese')
    return text


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('repo', nargs='?',
                    default=os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    ap.add_argument('--check', action='store_true')
    a = ap.parse_args()
    with open(os.path.join(a.repo, STAFF), encoding='utf-8') as f:
        staff = f.read()
    differ = 0
    for name, letter, allied, japanese in ARRANGEMENTS:
        text = make(staff, letter, allied, japanese)
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
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, 'w', encoding='utf-8') as f:
                f.write(text)
            print(f'wrote {FOLDER}/{name}')
    return 1 if differ else 0


if __name__ == '__main__':
    sys.exit(main())

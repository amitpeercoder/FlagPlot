"""Make the Midway games with submarines (decision 0225, as amended on
review), or check that the files kept are the ones it makes.

    python3 scripts/table-boats/make_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It writes four files to data/scenarios/table-boats/:

  01a_midway_submarines.toml  from data/scenarios/midway/01a_midway.toml,
                              its ground line changed to the table's ground
                              figures (ground_hunger.toml), as
                              scripts/table-copies/make_copies.py changes the
                              others: kept for the measure, not offered;
  01b_midway_submarines.toml  from data/scenarios/table-hunger/01b_midway.toml;
  01c_midway_submarines.toml  from data/scenarios/table-hunger/01c_midway.toml;
  01d_midway_submarines.toml  from data/scenarios/table-hunger/01d_midway.toml.

Each copy is the file it copies with exactly these changes, and nothing else:
  1. its header (the comment lines before its first key) written anew: the
     header below, which then gives the header of the file it copies as it
     stands;
  2. for file a alone, the ground line, as above;
  3. the three keys of the submarine files, put after the line that names
     the surface combat figures;
  4. at the end, the section of the five boats.
So with its header, its three keys and its section of boats taken away, a
copy is the file it copies line for line, but for file a's ground line
(statement 6 of the test, tests/boats_in_the_midway_games.rs).

With `--check` it writes nothing: it says whether each file kept is byte for
byte the one it would make, whether the folder holds anything else, and exits
1 where not. Python 3, standard library only, deterministic. No build step
runs it; the test runs it with `--check`. Run it by hand whenever a source
changes.
"""
import argparse
import os
import sys

FOLDER = 'data/scenarios/table-boats'

# (letter, source, the ground line is to be changed)
FILES = [
    ('a', 'data/scenarios/midway/01a_midway.toml', True),
    ('b', 'data/scenarios/table-hunger/01b_midway.toml', False),
    ('c', 'data/scenarios/table-hunger/01c_midway.toml', False),
    ('d', 'data/scenarios/table-hunger/01d_midway.toml', False),
]

GROUND_OLD = 'ground = "../../reference/ground.toml"\n'
GROUND_NEW = 'ground = "../../reference/ground_hunger.toml"\n'
AFTER = 'surface_combat = "../../reference/surface_combat.toml"\n'
KEYS = ('submarine_classes = "../../reference/submarine_classes_1942.csv"\n'
        'submarine_torpedoes = "../../reference/submarine_torpedoes_1942.csv"\n'
        'submarines = "../../reference/submarines.toml"\n')

WORDS = {
    'a': ('a copy of file a, both sides given their orders (staff against '
          'staff), on the table\'s ground figures. It is not offered at the '
          'table: it is kept for the measure of decision 0225, rule 8.'),
    'b': ('a copy of the table\'s copy of file b, the Japanese side given its '
          'orders and the Allied side none. The table\'s Midway plays it for '
          'that pairing.'),
    'c': ('a copy of the table\'s copy of file c, the Allied side given its '
          'orders and the Japanese side none. The table\'s Midway plays it for '
          'that pairing.'),
    'd': ('a copy of the table\'s copy of file d, neither side given orders. '
          'The table\'s Midway plays it for those pairings.'),
}


def wrap(text, first='# ', rest='# ', width=78):
    out, line = [], first
    for word in text.split():
        if len(line) + len(word) + (0 if line in (first, rest) else 1) > width and line.strip('# '):
            out.append(line.rstrip())
            line = rest
        line += ('' if line in (first, rest) else ' ') + word
    out.append(line.rstrip())
    return '\n'.join(out) + '\n'


def para(text):
    return wrap(text)


def item(text):
    return wrap(text, '#   - ', '#     ')


def header(letter, source):
    h = para('MIDWAY, WITH SUBMARINES (decision 0225, as amended on review on '
             'Nov 23, 2026).')
    h += '#\n'
    h += para(f'This file is {WORDS[letter]} It copies {source}.')
    h += '#\n'
    h += para('Made by scripts/table-boats/make_files.py. Do not edit by hand: '
              'change the file it copies or the script and make the files '
              'again (`python3 scripts/table-boats/make_files.py --check` says '
              'whether each file is still the one it makes). Each copy is the '
              'file it copies with this header, the three keys of the '
              'submarine files and the section SUBMARINES at the end, and '
              'nothing else (file a also names the table\'s ground figures). '
              'Its `name` is the name of the file it copies.')
    h += '#\n'
    h += para('IT HAS SUBMARINES: five boats, each on station where she starts, '
              'each in a section of her own at the end with her rows. '
              'American: USS Tambor (1,3), USS Trout (1,1), USS Nautilus '
              '(4,-2) and USS Gato (1,-1). Japanese: I-168 (5,1). Each name is '
              'the name of her row of ships.csv.')
    h += item('Each start is a PLACEHOLDER start, read from a row by a rule '
              '(decision 0225, amended rules 2 and 3). The American boats: an '
              'arc of Bates\'s disposition of 2 June (engagement_positions '
              '16010: 240 to 360 degrees, 150 miles from Midway, six boats; '
              '16009: 250 to 310 degrees, 200 miles, three boats) is read '
              'clockwise from its lower bearing, the first boat the row names '
              'at the lower bearing and the last at the higher, those between '
              'at even steps, every boat the row names counted; the bearing '
              'is laid off flat in the map\'s frame from the atoll\'s position '
              'at the arc\'s distance, and the boat starts in the hex that '
              'point falls in. I-168 starts in Midway\'s hex, where rows 16202 '
              'and 7673 have her on 1, 2 and 4 June; there she attacks nothing, '
              'since a force in the hex of its own base is not attacked.')
    h += item('THE HOMES ARE STAND-INS: the American boats\' home is Midway '
              '(base 1) and I-168\'s is Wake (base 2), each the base of her '
              'side on the map nearest her; the true homes are off the map. Two effects are accepted: a boat '
              'going home to a base that has changed side still leaves the map '
              'there; and I-168\'s way home to Wake is five days on passage, '
              'on which a search sights her at the whole chance.')
    h += item('NO ORDER is given to any boat: each keeps her station. The rows '
              'that moved the boats in the battle are not played: the order to '
              'close and attack the enemy to the north-west at 0710 on 4 June '
              '(engagement_events 16181) and the order to form on the circle of '
              '100 miles from Midway by dawn on 5 June (16182). '
              'A person orders his own boats; a side the staff plays leaves its '
              'boats on their stations, and no doctrine moves a boat without a '
              'patrol order of the commander\'s.')
    h += item('THE SIDES ARE UNEQUAL in every pairing: four American boats on '
              'the arcs west and north-west of Midway, across the way the '
              'Japanese forces come, against one Japanese boat at Midway '
              'itself. The Japanese cordons (Subron 3\'s KO line and Subron 5\'s '
              'OTSU line) and Subdivision 13 at French Frigate Shoals lie east '
              'of the map\'s edge and are not in the game.')
    h += item('NO SUSPECTED WATERS. The rows of what each side believed of the '
              'other\'s boats are left out: engagement_events 16842 (one boat '
              'west of Midway, 29 May), 16518 (one 200 miles south-west of '
              'Midway, 16 April) and 16202 (fourteen sighted by I-168 off '
              'Midway, 1 and 2 June). They give no distance, and waters would '
              'change where the staff stands its forces, so that this game '
              'would differ from the game without boats by more than boats.')
    h += item('LEFT OUT: the seven American boats the arcs name that have no '
              'ship row, Dolphin and Grenadier (16009), Grayling, Grouper and '
              'Gudgeon (16010), Flying Fish and Cachalot (engagement_positions '
              '16011, the inner arc); their places on the arcs are left empty. '
              'The other American boats of the battle (Cuttlefish, the support '
              'patrol and the boats north of Oahu) are off the map or have no '
              'ship row.')
    h += item('ESTIMATED FIGURES (decision 0207, rule 12). The classes are '
              'those derived as of 7 August 1942 (data/reference/'
              'submarine_classes_1942.csv); the five boats\' classes did not '
              'change between June and August. Three load figures estimated '
              'from like boats by decision 0207, not stated by any row: class '
              '6001, Tambor class (Tambor, Trout), its torpedo; class 6002, '
              'Gato class (Gato), its torpedo; class 6011, Narwhal class '
              '(Nautilus), its passage speed and its torpedo. An estimate is '
              'never evidence. Class 6005 (I-168) has none.')
    h += item('THE FIGURES OF THE SUBMARINES ARE PLAYED UNSET (decision 0225, '
              'amended rule 7): data/reference/submarines.toml, every one a '
              'placeholder, on the designer\'s word of Nov 23, 2026 that boats '
              'are not to be held back (decision 0225, the designer\'s '
              'answer: the table offers one game of Midway, and it has its '
              'boats). No objective counts a boat sunk.')
    h += '#\n'
    h += para('Kept out of the loops that walk data/scenarios/*.toml by being in '
              'this folder (decision 0072).')
    h += '#\n'
    h += para('THE GAME IT COPIES. What follows, to the first key, is the header '
              'of the file it copies, as it stands there. It holds of this '
              'file, but where it says the submarines are left out or not '
              'modelled (Nautilus and I-168 among those left out): the five '
              'boats above are in.')
    h += '#\n'
    return h


def boat(id_, name, side, class_, q, r, home, notes):
    text = '\n' + ''.join(para(n) if i == 0 else ('#\n' + para(n)) for i, n in enumerate(notes))
    text += (f'[[submarine]]\nid = {id_}\nname = "{name}"\nside = "{side}"\n'
             f'class = {class_}\nq = {q}\nr = {r}\nhome = {home}\n')
    return text


def section():
    text = ('\n# SUBMARINES (decision 0225, as amended on review). Made by '
            'scripts/table-boats/make_files.py.\n'
            '# Each boat starts on her station, her hex (no station key), with '
            'her class\'s\n'
            '# salvoes, sound, and is ordered nowhere. The rule that reads '
            'each American\n'
            '# start from her arc is in the header; the hexes are worked by '
            'hand in\n'
            '# decision 0225, "The hexes, worked by hand".\n')
    arc150 = ('engagement_positions 16010 (Bates, 2 June: Tambor, Trout, '
              'Grayling, Nautilus, Grouper and Gudgeon on the arc 240 to 360 '
              'degrees, 150 miles from Midway; steps of 24 degrees)')
    text += boat(1, 'USS Tambor', 'allied', 6001, 1, 3, 1, [
        'USS Tambor (ships.csv 6321; class 6001, Tambor class, by ship_facts '
        '16020). PLACEHOLDER start (1,3): ' + arc150 + ', first of six, '
        'bearing 240 degrees: the point 150 miles out lies 17.6 nm from the '
        'centre of (1,3). Home: Midway (base 1), a stand-in.'])
    text += boat(2, 'USS Trout', 'allied', 6001, 1, 1, 1, [
        'USS Trout (ships.csv 6322; class 6001, Tambor class, by ship_facts '
        '16021). PLACEHOLDER start (1,1): ' + arc150 + ', second of six, '
        'bearing 264 degrees: 9.1 nm from the centre of (1,1). Home: Midway '
        '(base 1), a stand-in.'])
    text += boat(3, 'USS Nautilus', 'allied', 6011, 4, -2, 1, [
        'USS Nautilus (ships.csv 6317; class 6011, Narwhal class, by ship_facts '
        '16019). PLACEHOLDER start (4,-2): ' + arc150 + ', fourth of six, '
        'bearing 312 degrees: 20.3 nm from the centre of (4,-2) and 20.6 nm '
        'from that of (4,-3). Her stated position at 0710 on 4 June '
        '(engagement_positions 16002: 30-00 N, 179-25 W, in the hex next to '
        'hers) agrees with this reading of the order and is not her start: it '
        'is of the second day. Home: Midway (base 1), a stand-in.'])
    text += boat(4, 'USS Gato', 'allied', 6002, 1, -1, 1, [
        'USS Gato (ships.csv 6326; class 6002, Gato class, by ship_facts '
        '16022). PLACEHOLDER start (1,-1): engagement_positions 16009 (Bates, '
        '2 June: Dolphin, Gato and Grenadier on the arc 250 to 310 degrees, '
        '200 miles from Midway; steps of 30 degrees), second of three, bearing '
        '280 degrees: 16.0 nm from the centre of (1,-1). Home: Midway (base '
        '1), a stand-in.'])
    text += boat(5, 'I-168', 'japanese', 6005, 5, 1, 2, [
        'I-168 (ships.csv 6424; class 6005, I-168 to I-175, by ship_facts '
        '16000). PLACEHOLDER start (5,1), Midway\'s hex: engagement_events '
        '16202 (she reconnoitred Midway from midnight of 1 June to noon of 2 '
        'June) and 7673 (surfaced 4,500 yards east of Eastern Island at 2154 '
        'on 4 June). Her one row in the positions file (engagement_positions '
        '5774, Yorktown sunk 159 miles east of Midway) is of a later day and '
        'takes an order to go there. Home: Wake (base 2), a stand-in.'])
    return text


def make(text, source, change_ground):
    lines = text.splitlines(keepends=True)
    first = next(i for i, line in enumerate(lines)
                 if line.strip() and not line.lstrip().startswith('#'))
    if not lines[first].startswith('name = '):
        raise SystemExit(f'{source}: its first key is not its name')
    head, body = ''.join(lines[:first]), ''.join(lines[first:])
    if change_ground:
        if body.count(GROUND_OLD) != 1:
            raise SystemExit(f'{source}: the line {GROUND_OLD.strip()!r} is not there exactly once')
        body = body.replace(GROUND_OLD, GROUND_NEW)
    elif body.count(GROUND_NEW) != 1:
        raise SystemExit(f'{source}: the line {GROUND_NEW.strip()!r} is not there exactly once')
    if body.count(AFTER) != 1:
        raise SystemExit(f'{source}: the line {AFTER.strip()!r} is not there exactly once')
    if '[[submarine]]' in body or 'submarine_classes' in body:
        raise SystemExit(f'{source}: it already has a submarine')
    body = body.replace(AFTER, AFTER + KEYS)
    if not body.endswith('\n'):
        raise SystemExit(f'{source}: it does not end with a new line')
    return head, body + section()


def main():
    parser = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    parser.add_argument('repo', nargs='?',
                        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    folder = os.path.join(args.repo, FOLDER)
    stale, names = [], set()
    for letter, source, change_ground in FILES:
        with open(os.path.join(args.repo, source), encoding='utf-8', newline='') as f:
            old_head, body = make(f.read(), source, change_ground)
        made = header(letter, source) + old_head + body
        name = f'01{letter}_midway_submarines.toml'
        names.add(name)
        target = os.path.join(folder, name)
        if args.check:
            try:
                with open(target, encoding='utf-8', newline='') as f:
                    kept = f.read()
            except OSError:
                kept = None
            if kept == made:
                print(f'{FOLDER}/{name}: as made')
            else:
                print(f'{FOLDER}/{name}: NOT as made: run the script without --check')
                stale.append(name)
        else:
            os.makedirs(folder, exist_ok=True)
            with open(target, 'w', encoding='utf-8', newline='') as f:
                f.write(made)
            print(f'wrote {FOLDER}/{name}')
    if os.path.isdir(folder):
        for other in sorted(os.listdir(folder)):
            if other not in names:
                print(f'{FOLDER}/{other}: not a file this script makes')
                stale.append(other)
    return 1 if stale else 0


if __name__ == '__main__':
    sys.exit(main())

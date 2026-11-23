"""Make the games with submarines (decision 0225, as amended on review, and
decision 0227, as amended on review), or check that the files kept are the
ones it makes.

    python3 scripts/table-boats/make_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It writes sixteen files to data/scenarios/table-boats/, four for
each of four games:

  Midway (15 boats)
    01a_midway_submarines.toml  from data/scenarios/midway/01a_midway.toml,
                                its ground line changed to the table's ground
                                figures (ground_hunger.toml), as
                                scripts/table-copies/make_copies.py changes the
                                others: kept for the measure, not offered;
    01b_, 01c_ and 01d_midway_submarines.toml  from
                                data/scenarios/table-hunger/01b_midway.toml,
                                01c_midway.toml and 01d_midway.toml.
  The Coral Sea (4 boats)
    03a_coral_sea_both_oilers_submarines.toml  from
                                data/scenarios/coral-sea/03a_coral_sea_both_oilers.toml
                                (ground line changed as above) and 03b_ to
                                03d_ from data/scenarios/table-hunger/.
  The Eastern Solomons over five days (5 boats)
    02a_eastern_solomons_oilers_submarines.toml  from
                                data/scenarios/five-days/02a_eastern_solomons_oilers.toml
                                (ground line changed) and 02b_ to 02d_ from
                                data/scenarios/table-hunger/.
  Guadalcanal, the 39 days (13 boats)
    07a_slice_submarines.toml   from data/scenarios/stage-3-proof/07a_slice.toml
                                (ground line changed); 07b_slice_submarines.toml
                                and 07c_slice_submarines.toml from
                                data/scenarios/table-hunger/07b_slice.toml and
                                07c_slice.toml; 07_full_slice_submarines.toml
                                from data/scenarios/table-hunger/07_full_slice.toml.

Each copy is the file it copies with exactly these changes, and nothing else:
  1. its header (the comment lines before its first key) written anew: the
     header below, which then gives the header of the file it copies as it
     stands;
  2. for file a alone, the ground line, as above;
  3. the three keys of the submarine files, put after the line that names
     the surface combat figures;
  4. at the end, the section of the boats, which opens with the two [side]
     tables of decision 0223's stage 0.
So with its header, its three keys and its section taken away, a copy is the
file it copies line for line, but for file a's ground line (statement 7 of
each game's test).

With `--check` it writes nothing: it says whether each file kept is byte for
byte the one it would make, whether the folder holds anything else, and exits
1 where not. Python 3, standard library only, deterministic. No build step
runs it; the tests run it with `--check`. Run it by hand whenever a source
changes.
"""
import argparse
import csv
import os
import sys

FOLDER = 'data/scenarios/table-boats'

GROUND_OLD = 'ground = "../../reference/ground.toml"\n'
GROUND_NEW = 'ground = "../../reference/ground_hunger.toml"\n'
AFTER = 'surface_combat = "../../reference/surface_combat.toml"\n'
KEYS = ('submarine_classes = "../../reference/submarine_classes_1942.csv"\n'
        'submarine_torpedoes = "../../reference/submarine_torpedoes_1942.csv"\n'
        'submarines = "../../reference/submarines.toml"\n')

SIDES = ('[side.allied]\n'
         'submarine_pattern = "off_bases"\n'
         'expects_boats_off_own_bases = true\n'
         '\n'
         '[side.japanese]\n'
         'submarine_pattern = "lines"\n'
         'expects_boats_off_own_bases = false\n')

CLASS_NAMES = {
    6001: 'Tambor class', 6002: 'Gato class', 6003: 'I-15 class',
    6005: 'I-168 group', 6007: 'RO-33 group', 6008: 'I-121 group',
    6009: 'S-1 group', 6010: 'S-42 group', 6011: 'Narwhal class',
    6012: 'Dolphin class', 6013: 'Cachalot class', 6014: 'I-9 group',
}

WORDS = {
    'a': ('a copy of file a, both sides given their orders (staff against '
          'staff), on the table\'s ground figures. It is not offered at the '
          'table: it is kept for the measure of decision 0227, stage 3.'),
    'b': ('a copy of the table\'s copy of file b, the Japanese side given its '
          'orders and the Allied side none. The table plays it for that '
          'pairing.'),
    'c': ('a copy of the table\'s copy of file c, the Allied side given its '
          'orders and the Japanese side none. The table plays it for that '
          'pairing.'),
    'd': ('a copy of the table\'s copy of file d, neither side given orders. '
          'The table plays it for those pairings.'),
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


# ------------------------------------------------------------------ the boats
#
# One boat: (id, name, side, class, q, r, home, enters, leave_day, note).
# `enters` and `leave_day` are None where the boat has none. The note is her
# comment: her ship row, her class's row, the rows of her start and what was
# read from them, her distance from the hex's centre where a point was laid
# off, and her stand-in home.

def B(id_, name, side, cls, q, r, home, enters, leave, note):
    return dict(id=id_, name=name, side=side, cls=cls, q=q, r=r, home=home,
                enters=enters, leave=leave, note=note)


def start_text(path):
    return path


# --- Midway

ARC150 = ('engagement_positions 16010 (Bates, 2 June: Tambor, Trout, '
          'Grayling, Nautilus, Grouper and Gudgeon on the arc 240 to 360 '
          'degrees, 150 miles from Midway; steps of 24 degrees)')
ARC200 = ('engagement_positions 16009 (Bates, 2 June: Dolphin, Gato and '
          'Grenadier on the arc 250 to 310 degrees, 200 miles from Midway; '
          'steps of 30 degrees)')
ARC50 = ('engagement_positions 16011 (Bates, 2 June: Flying Fish and Cachalot '
         'on the arc 310 to 350 degrees, 50 miles from Midway; steps of 40 '
         'degrees)')
LINE = ('engagement_positions 16013 (Bates, 2 June: the support patrol, a line '
        'of bearing 045 to 225 degrees through the point 075 degrees, 425 '
        'miles from Midway, the boats 30 miles apart; the boats in the row\'s '
        'order from the 045 end: Narwhal, Plunger, Trigger)')
MIDWAY_HOME = 'Home: Midway (base 1), a stand-in.'
ALL_ON_DAY_1 = ('She starts on day 1, on station ordered on 2 June, and has no '
                'leave day: no row gives one.')

MIDWAY_BOATS = [
    B(1, 'USS Tambor', 'allied', 6001, 1, 3, 1, None, None,
      'USS Tambor (ships.csv 6321; class 6001, Tambor class, by ship_facts '
      '16020). PLACEHOLDER start (1,3): ' + ARC150 + ', first of six, '
      'bearing 240 degrees: the point 150 miles out lies 17.6 nm from the '
      'centre of (1,3). ' + MIDWAY_HOME),
    B(2, 'USS Trout', 'allied', 6001, 1, 1, 1, None, None,
      'USS Trout (ships.csv 6322; class 6001, Tambor class, by ship_facts '
      '16021). PLACEHOLDER start (1,1): ' + ARC150 + ', second of six, '
      'bearing 264 degrees: 9.1 nm from the centre of (1,1). ' + MIDWAY_HOME),
    B(3, 'USS Nautilus', 'allied', 6011, 4, -2, 1, None, None,
      'USS Nautilus (ships.csv 6317; class 6011, Narwhal class, by ship_facts '
      '16019). PLACEHOLDER start (4,-2): ' + ARC150 + ', fourth of six, '
      'bearing 312 degrees: 20.3 nm from the centre of (4,-2) and 20.6 nm '
      'from that of (4,-3). Her stated position at 0710 on 4 June '
      '(engagement_positions 16002: 30-00 N, 179-25 W, in the hex next to '
      'hers) agrees with this reading of the order and is not her start: it '
      'is of the second day. ' + MIDWAY_HOME),
    B(4, 'USS Gato', 'allied', 6002, 1, -1, 1, None, None,
      'USS Gato (ships.csv 6326; class 6002, Gato class, by ship_facts '
      '16022). PLACEHOLDER start (1,-1): ' + ARC200 + ', second of three, '
      'bearing 280 degrees: 16.0 nm from the centre of (1,-1). '
      + MIDWAY_HOME),
    B(5, 'I-168', 'japanese', 6005, 5, 1, 2, None, None,
      'I-168 (ships.csv 6424; class 6005, I-168 to I-175, by ship_facts '
      '16000). PLACEHOLDER start (5,1), Midway\'s hex: engagement_events '
      '16202 (she reconnoitred Midway from midnight of 1 June to noon of 2 '
      'June) and 7673 (surfaced 4,500 yards east of Eastern Island at 2154 '
      'on 4 June). Her one row in the positions file (engagement_positions '
      '5774, Yorktown sunk 159 miles east of Midway) is of a later day and '
      'takes an order to go there. Home: Wake (base 2), a stand-in.'),
    B(6, 'USS Grayling', 'allied', 6001, 2, -1, 1, None, None,
      'USS Grayling (ships.csv 6323; class 6001, Tambor class, by ship_facts '
      '17226). PLACEHOLDER start (2,-1): ' + ARC150 + ', third of six, '
      'bearing 288 degrees: 15.7 nm from the centre of (2,-1) and 25.3 nm '
      'from that of (3,-1). ' + ALL_ON_DAY_1 + ' ' + MIDWAY_HOME),
    B(7, 'USS Grouper', 'allied', 6002, 6, -4, 1, None, None,
      'USS Grouper (ships.csv 6327; class 6002, Gato class, by ship_facts '
      '17234). PLACEHOLDER start (6,-4): ' + ARC150 + ', fifth of six, '
      'bearing 336 degrees: 17.9 nm from the centre of (6,-4) and 23.9 nm '
      'from that of (5,-3). Decision 0225 left her out for want of a ship '
      'row; her attack of the rows, on a destroyer alone, still cannot '
      'happen. ' + ALL_ON_DAY_1 + ' ' + MIDWAY_HOME),
    B(8, 'USS Gudgeon', 'allied', 6001, 7, -4, 1, None, None,
      'USS Gudgeon (ships.csv 6325; class 6001, Tambor class, by ship_facts '
      '17230). PLACEHOLDER start (7,-4): ' + ARC150 + ', sixth of six, '
      'bearing 360 degrees: 18.4 nm from the centre of (7,-4) and 22.7 nm '
      'from that of (8,-4). ' + ALL_ON_DAY_1 + ' ' + MIDWAY_HOME),
    B(9, 'USS Grenadier', 'allied', 6001, 3, -3, 1, None, None,
      'USS Grenadier (ships.csv 6324; class 6001, Tambor class, by ship_facts '
      '17228). PLACEHOLDER start (3,-3): ' + ARC200 + ', third of three, '
      'bearing 310 degrees: 9.7 nm from the centre of (3,-3) and 30.3 nm '
      'from that of (4,-4). ' + ALL_ON_DAY_1 + ' ' + MIDWAY_HOME),
    B(10, 'USS Flying Fish', 'allied', 6002, 5, -1, 1, None, None,
      'USS Flying Fish (ships.csv 6328; class 6002, Gato class, by ship_facts '
      '17240). PLACEHOLDER start (5,-1): ' + ARC50 + ', first of two, '
      'bearing 310 degrees: 18.6 nm from the centre of (5,-1) and 25.2 nm '
      'from that of (4,0). ' + ALL_ON_DAY_1 + ' ' + MIDWAY_HOME),
    B(11, 'USS Narwhal', 'allied', 6011, 18, -3, 1, None, None,
      'USS Narwhal (ships.csv 6329; class 6011, Narwhal class, by ship_facts '
      '17246). PLACEHOLDER start (18,-3): ' + LINE + '; the centre of the '
      'line is the point 075 degrees, 425 miles out; Narwhal is the first of '
      'three, 30 miles from the centre on 045 degrees: 15.4 nm from the '
      'centre of (18,-3) and 25.3 nm from that of (18,-4). ' + ALL_ON_DAY_1
      + ' ' + MIDWAY_HOME),
    B(12, 'USS Trigger', 'allied', 6002, 16, -2, 1, None, None,
      'USS Trigger (ships.csv 6331; class 6002, Gato class, by ship_facts '
      '17242). PLACEHOLDER start (16,-2): ' + LINE + '; Trigger is the third '
      'of three, 30 miles from the centre on 225 degrees: 7.9 nm from the '
      'centre of (16,-2) and 33.1 nm from that of (17,-2). Plunger, the '
      'second, is left out: her class is not derived. ' + ALL_ON_DAY_1 + ' '
      + MIDWAY_HOME),
    B(13, 'USS Dolphin', 'allied', 6012, 0, 2, 1, None, None,
      'USS Dolphin (ships.csv 6318; class 6012, Dolphin class, by ship_facts '
      '17212). PLACEHOLDER start (0,2): ' + ARC200 + ', first of three, '
      'bearing 250 degrees: 18.3 nm from the centre of (0,2) and 21.8 nm '
      'from that of (-1,3). ' + ALL_ON_DAY_1 + ' ' + MIDWAY_HOME),
    B(14, 'USS Cachalot', 'allied', 6013, 6, -1, 1, None, None,
      'USS Cachalot (ships.csv 6319; class 6013, Cachalot class, by '
      'ship_facts 17214). PLACEHOLDER start (6,-1): ' + ARC50 + ', second of '
      'two, bearing 350 degrees: 11.0 nm from the centre of (6,-1) and 29.1 '
      'nm from that of (5,-1). ' + ALL_ON_DAY_1 + ' ' + MIDWAY_HOME),
    B(15, 'USS Cuttlefish', 'allied', 6013, -13, 3, 1, None, None,
      'USS Cuttlefish (ships.csv 6320; class 6013, Cachalot class, by '
      'ship_facts 17216). PLACEHOLDER start (-13,3): engagement_positions '
      '17212 (Bates: she bore 263 degrees, 690 miles from Midway at 2400 on '
      '3 June), her first position in the game\'s days: the point lies 8.0 nm '
      'from the centre of (-13,3). Rows 16021 (27 N 170 E on 30 May) and 17213 '
      '(27 N 170-02 E at 0020 on 4 June) fall in the same hex. ' + ALL_ON_DAY_1
      + ' ' + MIDWAY_HOME),
]

# --- The Coral Sea

CS_HOME_ALLIED = ('Home: Port Moresby (base 1), a stand-in (the S-boats\' own '
                  'is Brisbane, off the map).')
CS_HOME_JAPANESE = ('Home: Rabaul (base 2), a stand-in (the rows have the '
                    'boats of the first weeks sail from Rabaul and return to '
                    'it).')
CS_BOATS = [
    B(1, 'USS S-42', 'allied', 6010, -3, -12, 1, None, None,
      'USS S-42 (ships.csv 17203; class 6010, S-42 group, by ship_facts '
      '17206). PLACEHOLDER start (-3,-12): engagement_events 16103 (off Cape '
      'St George on 5 May, the day before the game) and engagement_positions '
      '16005, the position of Okinoshima\'s sinking on 11 May, 5-06 S 153-48 '
      'E (the day after the game\'s last), read as hers since it is her '
      'target\'s: the point lies 4.2 nm from the centre of (-3,-12) and 36.2 '
      'nm from that of (-4,-12). No row has her leave those waters between, '
      'so she is read as in them through the days: the one use of a position '
      'outside a game\'s days, the architect\'s judgement (decision 0227). The '
      'hex is about 65 miles east-south-east of the hex where the rows put '
      '"eight miles south of Cape St George" (engagement_positions 16006 with '
      'engagement_events 16122); she is placed as ruled, at her one stated '
      'position, not in St George\'s Channel. She starts on day 1 and has no '
      'leave day: no row gives one. ' + CS_HOME_ALLIED),
    B(2, 'I-21', 'japanese', 6003, 0, 18, 2, None, None,
      'I-21 (ships.csv 17227; class 6003, I-15 class, by ship_facts 17257 '
      'and 17258). PLACEHOLDER start (0,18): engagement_positions 17202 (20 '
      'miles west of Noumea at about 0630 on 7 May; place row 16 for Noumea, '
      'bearing 270 degrees laid off flat in the map\'s frame): the point '
      'lies 8.7 nm from the centre of (0,18) and 31.4 nm from that of '
      '(0,17). Rows 17201 and 17204 have her off Noumea on 5 and 8 May. She '
      'starts on day 1 and has no leave day: no row gives one. '
      + CS_HOME_JAPANESE),
    B(3, 'RO-33', 'japanese', 6007, -17, -4, 2, None, 5,
      'RO-33 (ships.csv 17238; class 6007, RO-33 group, by ship_facts 17277). '
      'PLACEHOLDER start (-17,-4), the hex of Port Moresby (rule 2, third '
      'reading: at or off a place with no distance): engagement_positions '
      '17205, off Port Moresby from 5 May. Her leave day is 5, 10 May, on '
      'which the same row has her depart her patrol area. She is in the hex '
      'of a base the enemy holds. ' + CS_HOME_JAPANESE),
    B(4, 'RO-34', 'japanese', 6007, -17, -4, 2, None, None,
      'RO-34 (ships.csv 17239; class 6007, RO-33 group, by ship_facts 17279). '
      'PLACEHOLDER start (-17,-4), the hex of Port Moresby (rule 2, third '
      'reading): engagement_positions 17206, arrives off Port Moresby on 5 '
      'May. She has no leave day: no row gives one. She is in the hex of a '
      'base the enemy holds. ' + CS_HOME_JAPANESE),
]

# --- The Eastern Solomons over five days

FD_HOME_ALLIED = ('Home: Lunga (base 1), a stand-in (the S-boats\' own is '
                  'Brisbane, off the map).')
FD_HOME_JAPANESE = ('Home: Rabaul (base 2), a stand-in (the rows have the '
                    'boats of the first weeks sail from Rabaul and return to '
                    'it; the others\' is Truk, off the map).')
FD_BOATS = [
    B(1, 'USS S-42', 'allied', 6010, 2, -5, 1, 4, None,
      'USS S-42 (ships.csv 17203; class 6010, S-42 group, by ship_facts '
      '17206). PLACEHOLDER start (2,-5): engagement_events 17242, on station '
      'between Savo and Cape Esperance from 26 August (day 4), read over '
      'engagement_events 16152\'s 28 August (ruling 14 of decision 0227); '
      'place rows 10 (Savo Island, 9.1303 S 159.81619 E) and 32 (Cape '
      'Esperance): the points lie 9.3 nm and 8.9 nm from the centre of '
      '(2,-5) and 30.8 nm from that of (3,-5). She enters on day 4 and has '
      'no leave day: no row gives one. ' + FD_HOME_ALLIED),
    B(2, 'I-19', 'japanese', 6003, 7, -8, 2, None, None,
      'I-19 (ships.csv 17226; class 6003, I-15 class, by ship_facts 17254). '
      'PLACEHOLDER start (7,-8): engagement_positions 17239, 07-30 S 162-15 E '
      'at 0725 on 23 August, sighted on passage: 14.0 nm from the centre of '
      '(7,-8) and 28.6 nm from that of (7,-7). She stands there for the game '
      'unless ordered; she does not go on to her line. She starts on day 1 '
      'and has no leave day: the rows have her leave on 20 September, after '
      'the game. ' + FD_HOME_JAPANESE),
    B(3, 'I-17', 'japanese', 6003, 9, -9, 2, None, None,
      'I-17 (ships.csv 17225; class 6003, I-15 class, by ship_facts 17252). '
      'PLACEHOLDER start (9,-9): engagement_positions 17240, 06-48 S 163-20 E '
      'at 0745 on 23 August, on passage to join the patrol line: 15.6 nm from '
      'the centre of (9,-9) and 24.4 nm from that of (10,-9). She stands '
      'there for the game unless ordered. She starts on day 1 and has no '
      'leave day: the rows have her leave on 20 September, after the game. '
      + FD_HOME_JAPANESE),
    B(4, 'I-33', 'japanese', 6003, 8, -5, 2, 2, None,
      'I-33 (ships.csv 17233; class 6003, I-15 class, by ship_facts 17268). '
      'PLACEHOLDER start (8,-5): engagement_positions 17241, 09-21 S 163-35 E '
      'at 1105 on 24 August (day 2): 17.2 nm from the centre of (8,-5) and '
      '22.9 nm from that of (7,-4). She enters on day 2 and has no leave day: '
      'the rows have her leave on 20 September, after the game. '
      + FD_HOME_JAPANESE),
    B(5, 'RO-34', 'japanese', 6007, 2, -4, 2, None, None,
      'RO-34 (ships.csv 17239; class 6007, RO-33 group, by ship_facts 17279). '
      'PLACEHOLDER start (2,-4), the hex of Lunga (rule 2, third reading): '
      'engagement_positions 17261, reconnoitres Lunga Roads at 1827 on 23 '
      'August. She is in the hex of a base the enemy holds. She starts on day '
      '1 and has no leave day: no row gives one in these days. '
      + FD_HOME_JAPANESE),
]

# --- Guadalcanal, the 39 days

G_HOME_ALLIED = ('Home: Espiritu Santo (base 3), a stand-in (the S-boats\' own '
                 'is Brisbane, off the map).')
G_HOME_JAPANESE = ('Home: Rabaul (base 2), a stand-in (the rows have the '
                   'boats of the first weeks sail from Rabaul and return to '
                   'it; the others\' is Truk, off the map).')
G_NO_LEAVE = 'no row gives one'
G_BOATS = [
    B(1, 'USS S-38', 'allied', 6009, -5, -12, 3, None, None,
      'USS S-38 (ships.csv 16000; class 6009, S-1 group, by ship_facts '
      '16015). PLACEHOLDER start (-5,-12): engagement_events 16119 (off Cape '
      'St George on 7 August) and 16122 (eight miles south of it on 8 '
      'August); engagement_positions 16006, Meiyo Maru\'s position, 4-52 S '
      '152-43 E, read as hers since it is her target\'s: the point lies 17.6 '
      'nm from the centre of (-5,-12) and 23.5 nm from that of (-4,-13). She '
      'starts on day 1 and has no leave day: ' + G_NO_LEAVE + '. '
      + G_HOME_ALLIED),
    B(2, 'USS S-44', 'allied', 6010, -3, -17, 3, 4, None,
      'USS S-44 (ships.csv 16001; class 6010, S-42 group, by ship_facts '
      '16016). PLACEHOLDER start (-3,-17): engagement_positions 16008, '
      'Kako\'s position of 10 August (day 4), 02-28 S 152-11 E, read as hers '
      'since it is her target\'s: the point lies 16.9 nm from the centre of '
      '(-3,-17) and 25.8 nm from that of (-3,-16); row 16004 (2-15 S 152-15 '
      'E) is in the same hex. She enters on day 4 and has no leave day: '
      + G_NO_LEAVE + '. ' + G_HOME_ALLIED),
    B(3, 'USS S-37', 'allied', 6009, 2, -5, 3, 27, 38,
      'USS S-37 (ships.csv 17200; class 6009, S-1 group, by ship_facts '
      '17200). PLACEHOLDER start (2,-5): engagement_events 16150, north of '
      'Savo on 2 September (day 27); place row 10 (Savo Island): the point '
      'lies 9.3 nm from the centre of (2,-5). Her leave day is 38, 13 '
      'September, on which engagement_events 17239 ends her patrol; where '
      'the rows give only the end of a patrol she is kept on her station '
      'longer than she was. The hex is the hex of the Japanese stock west of '
      'the Lunga (base 6): she attacks no run that ends there. '
      + G_HOME_ALLIED),
    B(4, 'USS S-42', 'allied', 6010, 2, -5, 3, 20, None,
      'USS S-42 (ships.csv 17203; class 6010, S-42 group, by ship_facts '
      '17206). PLACEHOLDER start (2,-5): engagement_events 17242, on station '
      'between Savo and Cape Esperance from 26 August (day 20), read over '
      'engagement_events 16152\'s 28 August; place rows 10 and 32: the points '
      'lie 9.3 nm and 8.9 nm from the centre of (2,-5). She enters on day 20 '
      'and has no leave day: ' + G_NO_LEAVE + '. The hex is the hex of the '
      'Japanese stock west of the Lunga (base 6): she attacks no run that '
      'ends there. ' + G_HOME_ALLIED),
    B(5, 'USS S-45', 'allied', 6010, -2, -8, 3, 25, None,
      'USS S-45 (ships.csv 16002; class 6010, S-42 group, by ship_facts '
      '16017). PLACEHOLDER start (-2,-8), the hex of Shortland (rule 2, third '
      'reading: "in the Shortland Island area"): engagement_events 16165, on '
      'station there by the end of August, read as 31 August (day 25). She '
      'enters on day 25 and has no leave day: ' + G_NO_LEAVE + '. She is in '
      'the hex of a base the enemy holds, which has no airfield, and she '
      'attacks nothing lying at Shortland. ' + G_HOME_ALLIED),
    B(6, 'I-122', 'japanese', 6008, 3, -5, 2, 7, None,
      'I-122 (ships.csv 6437; class 6008, I-121 group, by ship_facts 16013). '
      'PLACEHOLDER start (3,-5), the hex of Tulagi: engagement_positions '
      '17226, 1.6 miles south-west of Tulagi at 1730 on 13 August (day 7), '
      'laid off from Tulagi\'s place row. She enters on day 7 and has no '
      'leave day: ' + G_NO_LEAVE + '. She lies at a base of her own side '
      'until it is taken. ' + G_HOME_JAPANESE),
    B(7, 'I-123', 'japanese', 6008, 2, -4, 2, 6, None,
      'I-123 (ships.csv 6438; class 6008, I-121 group, by ship_facts 16014). '
      'PLACEHOLDER start (2,-4), the hex of Lunga: engagement_positions '
      '17229, north of Lunga Point on 12 August (day 6). She enters on day 6 '
      'and has no leave day: the rows have her sunk on 29 August, day 23 '
      '(engagement_positions 16001, engagement_events 16054), an outcome '
      'and not scripted; she is kept as long as she lives. She lies at a '
      'base of her own side until it is taken. ' + G_HOME_JAPANESE),
    B(8, 'RO-34', 'japanese', 6007, 2, -4, 2, 17, 31,
      'RO-34 (ships.csv 17239; class 6007, RO-33 group, by ship_facts 17279). '
      'PLACEHOLDER start (2,-4), the hex of Lunga: her second patrol, '
      'engagement_positions 17261, reconnoitres Lunga Roads at 1827 on 23 '
      'August (day 17). Her leave day is 31, 6 September, on which '
      'engagement_events 17202 has her back at Rabaul. Her first patrol (off '
      'Tulagi on 10 August, back at Rabaul on 16 August) is left out: the '
      'second is the longer. She lies at a base of her own side until it is '
      'taken. ' + G_HOME_JAPANESE),
    B(9, 'I-19', 'japanese', 6003, 7, -8, 2, 17, None,
      'I-19 (ships.csv 17226; class 6003, I-15 class, by ship_facts 17254). '
      'PLACEHOLDER start (7,-8): engagement_positions 17239, 07-30 S 162-15 E '
      'at 0725 on 23 August (day 17): 14.0 nm from the centre of (7,-8). She '
      'stands there unless ordered. She enters on day 17 and has no leave '
      'day: the rows have her leave on 20 September, after the game. '
      + G_HOME_JAPANESE),
    B(10, 'I-17', 'japanese', 6003, 9, -9, 2, 17, None,
      'I-17 (ships.csv 17225; class 6003, I-15 class, by ship_facts 17252). '
      'PLACEHOLDER start (9,-9): engagement_positions 17240, 06-48 S 163-20 E '
      'at 0745 on 23 August (day 17): 15.6 nm from the centre of (9,-9). She '
      'stands there unless ordered. She enters on day 17 and has no leave '
      'day: the rows have her leave on 20 September, after the game. '
      + G_HOME_JAPANESE),
    B(11, 'I-33', 'japanese', 6003, 8, -5, 2, 18, None,
      'I-33 (ships.csv 17233; class 6003, I-15 class, by ship_facts 17268). '
      'PLACEHOLDER start (8,-5): engagement_positions 17241, 09-21 S 163-35 E '
      'at 1105 on 24 August (day 18): 17.2 nm from the centre of (8,-5). She '
      'enters on day 18 and has no leave day: the rows have her leave on 20 '
      'September, after the game. ' + G_HOME_JAPANESE),
    B(12, 'I-26', 'japanese', 6003, 8, -3, 2, 25, None,
      'I-26 (ships.csv 17228; class 6003, I-15 class, by ship_facts 17259). '
      'PLACEHOLDER start (8,-3): engagement_positions 16000, Saratoga\'s '
      'position when she fired, 10-34 S 164-18 E on 31 August (day 25), read '
      'as hers: the point lies 19.5 nm from the centre of (8,-3) and 20.8 nm '
      'from that of (7,-2), a difference of 1.3 miles; the rule gives (8,-3). '
      'Her area by engagement_positions 17247 (140 miles east of San '
      'Cristobal) cannot be laid off, San Cristobal having no place row. She '
      'enters on day 25 and has no leave day: the rows have her leave on 20 '
      'September, after the game. ' + G_HOME_JAPANESE),
    B(13, 'I-11', 'japanese', 6014, 4, -2, 2, 25, 36,
      'I-11 (ships.csv 17237; class 6014, I-9 group, by ship_facts 17275 and '
      '17276). PLACEHOLDER start (4,-2): engagement_positions 17248, 146 '
      'miles south-east of Tulagi on 31 August (day 25), laid off from '
      'Tulagi\'s place row 4 at bearing 135: the point lies 7.9 nm from the '
      'centre of (4,-2) and 32.3 nm from that of (3,-2). Her leave day is 36, '
      '11 September, on which engagement_events 17211 has her at Truk. '
      + G_HOME_JAPANESE),
]

# ------------------------------------------------------------------ the games

COMMON_NO_ORDER = (
    'NO ORDER is given to any boat: each keeps her station. A person orders '
    'his own boats; a side the staff plays leaves its boats where they start, '
    'since the staff moves a boat only on a patrol order of the commander\'s '
    '(decision 0223, as amended). The two [side] tables at the head of the '
    'section state each navy\'s pattern: the Allied side\'s is off bases and '
    'it expects boats off its own bases; the Japanese side\'s is lines and it '
    'does not (decision 0227). Neither moves a boat: the pattern is read only '
    'when a commander orders a patrol.')

COMMON_FIGURES = (
    'THE FIGURES OF THE SUBMARINES ARE PLAYED UNSET (decision 0225, amended '
    'rule 7): data/reference/submarines.toml, every one a placeholder, on '
    'the designer\'s word of Nov 23, 2026 that the table offers one game of '
    'each battle and where the battle has boats that game has them. No '
    'objective counts a boat sunk.')

COMMON_ROWS = (
    'The rows named are by file: engagement_positions.csv, '
    'engagement_events.csv and ship_facts.csv, by their ids; a place row is '
    'a row of place_positions.csv.')

COMMON_HOMES = (
    'THE HOMES ARE STAND-INS, named on each boat: the true homes are off the '
    'map, or are not read from the rows.')

COMMON_START = (
    'Each start is a PLACEHOLDER start (decision 0227, rule 2), one of '
    'three readings: a stated latitude and longitude, the hex the point falls '
    'in (where the position is her target\'s, it is read as hers); a bearing '
    'and a distance from a place with a preferred row in '
    'place_positions.csv, the point laid off flat in the map\'s frame; or at '
    'or off a place with no distance, the hex of the place. Where the rows '
    'give a boat two patrols in the days, the longer is read. She enters on '
    'the day of that position, on day 1 where a row has her in the same '
    'waters on or before the game\'s first day.')

GAMES = [
    dict(
        key='midway', title='MIDWAY', name='Midway',
        decision='decisions 0225 and 0227, each as amended on review',
        files=[
            ('a', 'data/scenarios/midway/01a_midway.toml', True),
            ('b', 'data/scenarios/table-hunger/01b_midway.toml', False),
            ('c', 'data/scenarios/table-hunger/01c_midway.toml', False),
            ('d', 'data/scenarios/table-hunger/01d_midway.toml', False),
        ],
        boats=MIDWAY_BOATS,
        count='fifteen',
        lists=('American: USS Tambor (1,3), USS Trout (1,1), USS Nautilus '
               '(4,-2), USS Gato (1,-1), USS Grayling (2,-1), USS Grouper '
               '(6,-4), USS Gudgeon (7,-4), USS Grenadier (3,-3), USS Flying '
               'Fish (5,-1), USS Narwhal (18,-3), USS Trigger (16,-2), USS '
               'Dolphin (0,2), USS Cachalot (6,-1) and USS Cuttlefish '
               '(-13,3). Japanese: I-168 (5,1).'),
        items=[
            'Each start is a PLACEHOLDER start, read from a row by a rule '
            '(decision 0225, amended rules 2 and 3, carried on by decision '
            '0227). The American boats on the arcs of Bates\'s disposition of '
            '2 June (engagement_positions 16010: 240 to 360 degrees, 150 '
            'miles from Midway, six boats; 16009: 250 to 310 degrees, 200 '
            'miles, three boats; 16011: 310 to 350 degrees, 50 miles, two '
            'boats): the arc is read clockwise from its lower bearing, the '
            'first boat the row names at the lower bearing and the last at '
            'the higher, those between at even steps, every boat the row '
            'names counted; the bearing is laid off flat in the map\'s frame '
            'from the atoll\'s position at the arc\'s distance, and the boat '
            'starts in the hex that point falls in. The support patrol '
            '(16013, a line of bearing 045 to 225 degrees through the point '
            '075 degrees, 425 miles out, the boats 30 miles apart): Narwhal '
            '30 miles from the centre on 045 and Trigger 30 miles on 225, '
            'Plunger between them left out. Cuttlefish: row 17212, 263 '
            'degrees, 690 miles at 2400 on 3 June. I-168 starts in '
            'Midway\'s hex, where rows 16202 and 7673 have her on 1, 2 and '
            '4 June; there she attacks nothing, since a force in the hex of '
            'its own base is not attacked. All ten new boats start on day 1, '
            'on stations ordered on 2 June.',
            'THE HOMES ARE STAND-INS: the American boats\' home is Midway '
            '(base 1) and I-168\'s is Wake (base 2), each the base of her '
            'side on the map nearest her; the true homes are off the map. '
            'Two effects are accepted: a boat going home to a base that has '
            'changed side still leaves the map there; and I-168\'s way home '
            'to Wake is five days on passage, on which a search sights her '
            'at the whole chance.',
            COMMON_NO_ORDER + ' The rows that moved the boats in the battle '
            'are not played: the order to close and attack the enemy to the '
            'north-west at 0710 on 4 June (engagement_events 16181) and the '
            'order to form on the circle of 100 miles from Midway by dawn on '
            '5 June (16182).',
            'THE SIDES ARE UNEQUAL in every pairing: fourteen American boats '
            'on the arcs west and north-west of Midway, across the way the '
            'Japanese forces come, and on a line east-north-east of the '
            'atoll, against one Japanese boat at Midway itself.',
            'ACCEPTED EFFECTS, each counted by the measure of decision 0227 '
            '(which reports who held each such base on each day): a force in '
            'the hex of a base its own side holds is not attacked, so '
            'I-168, in Midway\'s hex, attacks nothing that lies at Midway; a '
            'boat with one start stands there for the game unless ordered. '
            'A boat in the hex of a base the enemy holds is looked for at '
            'the full count of that base\'s search: no American boat is in '
            'such a hex at the start.',
            'NO SUSPECTED WATERS. The rows of what each side believed of the '
            'other\'s boats are left out: engagement_events 16842 (one boat '
            'west of Midway, 29 May), 16518 (one 200 miles south-west of '
            'Midway, 16 April), 16202 (fourteen sighted by I-168 off Midway, '
            '1 and 2 June) and 17246 to 17248 (of other waters). They give '
            'no distance, and waters would change where the staff stands its '
            'forces, so that this game would differ from the game without '
            'boats by more than boats.',
            'LEFT OUT: USS Plunger, whose class (Porpoise or Perch) is not '
            'derived (decision 0227, ruling 9); USS Pike, Tarpon, Growler and '
            'Finback, off the map; the Japanese cordons (Subron 3\'s KO line '
            'and Subron 5\'s OTSU line) and Subdivision 13 at French Frigate '
            'Shoals, off the map; and I-171, whose point M (19 N 174-26 W, '
            'engagement_positions 16015, hex (1,16) of this map) is of 31 '
            'May, three days before the game, for an operation that was '
            'cancelled, and whom the rows have on the KO line at 166-20 W '
            'from 1 June (engagement_events 16194), off the map: no row has '
            'her on the map in the game\'s days. Decision 0225\'s "I-168 '
            'alone lies on the map" is true of the game\'s days, not of the '
            'rows.',
        ]),
    dict(
        key='coral', title='THE CORAL SEA, 6 TO 10 MAY 1942',
        name='the Coral Sea',
        decision='decision 0227, as amended on review',
        files=[
            ('a', 'data/scenarios/coral-sea/03a_coral_sea_both_oilers.toml', True),
            ('b', 'data/scenarios/table-hunger/03b_coral_sea_both_oilers.toml', False),
            ('c', 'data/scenarios/table-hunger/03c_coral_sea_both_oilers.toml', False),
            ('d', 'data/scenarios/table-hunger/03d_coral_sea_both_oilers.toml', False),
        ],
        boats=CS_BOATS,
        count='four',
        lists=('American: USS S-42 (-3,-12). Japanese: I-21 (0,18), RO-33 '
               '(-17,-4) and RO-34 (-17,-4).'),
        items=[
            COMMON_START,
            COMMON_HOMES + ' Port Moresby is the Allied boat\'s; Rabaul is '
            'every Japanese boat\'s.',
            COMMON_NO_ORDER,
            'ACCEPTED EFFECTS, each counted by the measure of decision 0227 '
            '(which reports who held each such base on each day): a boat in '
            'the hex of a base the enemy holds is looked for at the full '
            'count of that base\'s search where the base flies one: at the '
            'start so of RO-33 and RO-34 at Port Moresby. A force in the hex '
            'of a base its own side holds is not attacked: so the RO boats '
            'may not attack what lies at Port Moresby from the start. A boat '
            'with one start stands there for the game unless ordered.',
            'S-42\'S POSITION is of the day after the game\'s last: the rows '
            'have her off Cape St George the day before the game and at that '
            'position the day after it, and nothing on file has her leave '
            'between. She is read as in those waters through the days, the '
            'one use of a position outside a game\'s days, the architect\'s '
            'judgement.',
            'NO SUSPECTED WATERS. The rows of what each side believed of the '
            'other\'s boats are left out: Japanese, engagement_events 16257 '
            '(Bismarck area: estimate 1 to 2, actual 4 boats); Allied, '
            '16513, 16534, 16750 and 16849. They give no distance, and '
            'waters would change where the staff stands its forces.',
            'LEFT OUT: I-28 and I-29 (class 6003) and I-22 and I-24 (class '
            'not derived) on the line south-west of Guadalcanal, which '
            'engagement_positions 17200 gives as a bearing with no distance; '
            'I-27, off the map; S-44 and S-47 (class 6010), off Cape St '
            'George and off New Hanover, places with no row (owed); S-38, '
            'S-39, S-41, S-43 and S-45, not in the map\'s waters in the '
            'days.',
        ]),
    dict(
        key='five', title='THE EASTERN SOLOMONS OVER FIVE DAYS, 23 TO 27 AUGUST 1942',
        name='the Eastern Solomons over five days',
        decision='decision 0227, as amended on review',
        files=[
            ('a', 'data/scenarios/five-days/02a_eastern_solomons_oilers.toml', True),
            ('b', 'data/scenarios/table-hunger/02b_eastern_solomons_oilers.toml', False),
            ('c', 'data/scenarios/table-hunger/02c_eastern_solomons_oilers.toml', False),
            ('d', 'data/scenarios/table-hunger/02d_eastern_solomons_oilers.toml', False),
        ],
        boats=FD_BOATS,
        count='five',
        lists=('American: USS S-42 (2,-5), entering on day 4. Japanese: I-19 '
               '(7,-8), I-17 (9,-9), I-33 (8,-5), entering on day 2, and '
               'RO-34 (2,-4).'),
        items=[
            COMMON_START,
            COMMON_HOMES + ' Lunga is the Allied boat\'s; Rabaul is every '
            'Japanese boat\'s.',
            COMMON_NO_ORDER,
            'ACCEPTED EFFECTS, each counted by the measure of decision 0227 '
            '(which reports who held each such base on each day): RO-34, in '
            'the hex of Lunga, a base the enemy holds, is looked for at the '
            'full count of the base\'s search where the base flies one, and '
            'may not attack what lies at Lunga, a force in the hex of a base '
            'its own side holds not being attacked. I-19 and I-17 start where '
            'they were sighted on passage and do not go on to their line; '
            'a boat with one start stands there for the game unless ordered. '
            'No boat has a leave day.',
            'NO SUSPECTED WATERS. The rows of what each side believed of the '
            'other\'s boats are left out: Allied, engagement_events 16298, '
            '16584, 16608, 16589 and 16590; Japanese, none on file, a gap. '
            'They give no distance, and waters would change where the staff '
            'stands its forces.',
            'LEFT OUT: I-121 (class 6008) and I-31 (class 6003), whose '
            'positions are bearings from San Cristobal, a place with no row '
            '(owed); I-26 and I-15 (class 6003) and I-174 and I-175 (class '
            '6005), whose areas in these days have no distance; I-9 and '
            'I-11, with no position in the days; I-123, with no dated '
            'position in the days; S-37, whose row gives her patrol\'s dates '
            'from Brisbane and not her days off Savo.',
        ]),
    dict(
        key='guadalcanal', title='GUADALCANAL, THE 39 DAYS, 7 AUGUST TO 14 SEPTEMBER 1942',
        name='the 39 days of Guadalcanal',
        decision='decision 0227, as amended on review',
        files=[
            ('a', 'data/scenarios/stage-3-proof/07a_slice.toml', True),
            ('b', 'data/scenarios/table-hunger/07b_slice.toml', False),
            ('c', 'data/scenarios/table-hunger/07c_slice.toml', False),
            ('d', 'data/scenarios/table-hunger/07_full_slice.toml', False),
        ],
        boats=G_BOATS,
        count='thirteen',
        lists=('American: USS S-38 (-5,-12), USS S-44 (-3,-17), USS S-37 '
               '(2,-5), USS S-42 (2,-5) and USS S-45 (-2,-8). Japanese: I-122 '
               '(3,-5), I-123 (2,-4), RO-34 (2,-4), I-19 (7,-8), I-17 (9,-9), '
               'I-33 (8,-5), I-26 (8,-3) and I-11 (4,-2). Most enter after '
               'day 1, on the day of their row.'),
        items=[
            COMMON_START,
            COMMON_HOMES + ' Espiritu Santo is every Allied boat\'s; Rabaul '
            'is every Japanese boat\'s.',
            COMMON_NO_ORDER,
            'ACCEPTED EFFECTS, each counted by the measure of decision 0227 '
            '(which reports who held each such base on each day): a boat in '
            'the hex of a base the enemy holds is looked for at the full '
            'count of that base\'s search where the base flies one. In these '
            'days Lunga and Tulagi start Japanese and with no airfield, and '
            'Shortland has none: I-123 and RO-34 at Lunga and I-122 at '
            'Tulagi lie at a base of their own side until it is taken, and '
            'S-45 at Shortland is looked for only by what search the base '
            'has. A force in the hex of a base its own side holds is not '
            'attacked: I-122, I-123 and RO-34 may attack an Allied force in '
            'their hex while the base is Japanese and not after it is '
            'Allied; S-45 attacks nothing lying at Shortland; S-37 and S-42, '
            'in the hex of the Japanese stock west of the Lunga, attack no '
            'run that ends there. I-19, I-17 and I-33 start where they were '
            'sighted on passage and do not go on to their line; a boat with '
            'one start stands there for the game unless ordered.',
            'LEAVE DAYS, said in full: I-19, I-17, I-33 and I-26 have none '
            'because the rows have them leave on 20 September, after the '
            'game. I-123 has none: the rows have her sunk on 29 August, day '
            '23, an outcome and not scripted; she is kept as long as she '
            'lives. I-122, S-38, S-44, S-42 and S-45 have none because no '
            'row gives one, a gap. S-37 (day 38), RO-34 (day 31) and I-11 '
            '(day 36) have the day the rows give.',
            'NO SUSPECTED WATERS. The rows of what each side believed of the '
            'other\'s boats are left out: Allied, engagement_events 16298, '
            '16584, 16608, 16589, 16590, 16303, 16610 and 16613; Japanese, '
            'none on file, a gap. They give no distance, and waters would '
            'change where the staff stands its forces.',
            'LEFT OUT: RO-34\'s first patrol (off Tulagi on 10 August, back '
            'at Rabaul on 16 August; the longer second patrol is read). '
            'I-121 and I-31, whose positions are bearings from San '
            'Cristobal, a place with no row (owed); I-175, whose first '
            'patrol ended the day the rows first place her (12 August) and '
            'whose second has one position, three days before the end; I-15, '
            'I-174, I-9, I-21 and I-24, with no position; S-41, S-46, S-47, '
            'S-43, Amberjack and the boats off Truk, with none on the map; '
            'S-39, aground on her way.',
        ]),
]


def classes_file(repo):
    out = {}
    with open(os.path.join(repo, 'data/reference/submarine_classes_1942.csv'),
              encoding='utf-8', newline='') as f:
        rows = csv.reader(l for l in f if not l.startswith('#'))
        head = next(rows)
        for row in rows:
            out[int(row[0])] = dict(zip(head, row))
    return out


def estimated_item(game, classes):
    by_class = {}
    for b in game['boats']:
        by_class.setdefault(b['cls'], []).append(b['name'])
    parts = []
    for cls in sorted(by_class):
        est = classes[cls]['estimated'].replace('torpedo_id', 'its torpedo') \
            .replace('passage_speed_kn', 'its passage speed')
        est = est.replace(' its', ' and its') if est.count('its') > 1 else est
        if est:
            parts.append(f'class {cls}, {CLASS_NAMES[cls]} ({", ".join(by_class[cls])}), {est}')
    none = [str(c) for c in sorted(by_class) if not classes[c]['estimated']]
    text = ('ESTIMATED FIGURES (decision 0207, rule 12; decision 0227). The '
            'classes are those derived as of 7 August 1942 '
            '(data/reference/submarine_classes_1942.csv), read for earlier '
            'dates; no boat here is of a class whose rows change between '
            'May and August. Figures that load estimated from like boats by '
            'decision 0207, not stated by any row: ' + '; '.join(parts) + '. '
            'An estimate is never evidence.')
    if none:
        text += (' Class ' + none[0] + ' has none.' if len(none) == 1
                 else ' Classes ' + ', '.join(none) + ' have none.')
    return text


def header(game, letter, source, classes):
    h = para(f'{game["title"]}, WITH SUBMARINES ({game["decision"]}).')
    h += '#\n'
    h += para(f'This file is {WORDS[letter]} It copies {source}.')
    h += '#\n'
    h += para('Made by scripts/table-boats/make_files.py. Do not edit by hand: '
              'change the file it copies or the script and make the files '
              'again (`python3 scripts/table-boats/make_files.py --check` says '
              'whether each file is still the one it makes). Each copy is the '
              'file it copies with this header, the three keys of the '
              'submarine files and the section SUBMARINES at the end, which '
              'opens with the two [side] tables, and nothing else (file a '
              'also names the table\'s ground figures). Its `name` is the '
              'name of the file it copies.')
    h += '#\n'
    h += para(f'IT HAS SUBMARINES: {game["count"]} boats, each in a section of '
              f'her own at the end with her rows. {game["lists"]} Each name '
              'is the name of her row of ships.csv. ' + COMMON_ROWS)
    for text in game['items']:
        h += item(text)
    h += item(estimated_item(game, classes))
    h += item(COMMON_FIGURES)
    h += '#\n'
    h += para('Kept out of the loops that walk data/scenarios/*.toml by being in '
              'this folder (decision 0072).')
    h += '#\n'
    h += para('THE GAME IT COPIES. What follows, to the first key, is the header '
              'of the file it copies, as it stands there. It holds of this '
              'file, but where it says the submarines are left out or not '
              'modelled: the boats above are in.')
    h += '#\n'
    return h


def boat(b):
    text = '\n' + para(b['note'])
    text += (f'[[submarine]]\nid = {b["id"]}\nname = "{b["name"]}"\n'
             f'side = "{b["side"]}"\nclass = {b["cls"]}\nq = {b["q"]}\n'
             f'r = {b["r"]}\nhome = {b["home"]}\n')
    if b['enters'] is not None:
        text += f'enters = {b["enters"]}\n'
    if b['leave'] is not None:
        text += f'leave_day = {b["leave"]}\n'
    return text


def section(game):
    text = ('\n# SUBMARINES (decisions 0225 and 0227). Made by '
            'scripts/table-boats/make_files.py.\n'
            '# Each boat starts on her station, her hex (no station key), with '
            'her class\'s\n'
            '# salvoes, sound, and is ordered nowhere. The rule that reads '
            'each start is in\n'
            '# the header; the hexes are worked by hand in decisions 0225 and '
            '0227,\n'
            '# "The hexes, worked by hand".\n'
            '\n'
            '# How each side\'s staff uses its boats (decision 0223, stage 0; '
            'decision 0227).\n')
    text += SIDES
    for b in game['boats']:
        text += boat(b)
    return text


def make(text, source, change_ground, game):
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
    if '[side' in body:
        raise SystemExit(f'{source}: it already has a [side] table: stop and ask')
    body = body.replace(AFTER, AFTER + KEYS)
    if not body.endswith('\n'):
        raise SystemExit(f'{source}: it does not end with a new line')
    return head, body + section(game)


def out_name(source):
    stem = os.path.basename(source)[:-len('.toml')]
    return f'{stem}_submarines.toml'


def main():
    parser = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    parser.add_argument('repo', nargs='?',
                        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    folder = os.path.join(args.repo, FOLDER)
    classes = classes_file(args.repo)
    stale, names = [], set()
    for game in GAMES:
        for letter, source, change_ground in game['files']:
            with open(os.path.join(args.repo, source), encoding='utf-8', newline='') as f:
                old_head, body = make(f.read(), source, change_ground, game)
            made = header(game, letter, source, classes) + old_head + body
            name = out_name(source)
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

"""Conditions 2, 3 and 4 of stage 3's proof (decision 0153, "How the
conditions are read"), counted in seeds from the umpire's exports of one
arrangement. It prints counts and never a verdict: the marks are read by the
architect.

    python3 conditions.py EXPORTS... [--batch BATCH.json] [--base 1]
                          [--list] [--jsonl FILE]

EXPORTS are export files (`sim-cli export ... --seed N --out FILE`) or
directories of them; any other JSON among them is skipped. `--batch` adds the
batch file's `refusals` of the same seeds beside condition 2, as a check.
`--base` is the base whose hex is "the hex of Lunga" (1, the record's; another
only to test the script on another scenario). `--list` prints the seeds on
which each count holds; `--jsonl` writes one line a seed with the events each
count rests on.

What each count reads (the README has the table):

2. Refused.
   - `order_rejected` whose `data.order` is an order with `issuer` "staff":
     an order a staff gave. One with `issuer` "player" (a scripted order or
     the commander's) and "local_commander" are counted apart.
   - `command_refused` with `data.issuer` "staff": a command a staff gave.
     With "player", the commander's command, counted apart; "local_commander"
     apart too.
3. The Allied side does something. L is the hex of base 1 in the file's
   `bases`.
   a. A `men_landed` in L whose unit is Allied (the unit's record; the task
      force's where no record names the unit).
   b. A `base_changed_side` of base 1 with `to` "allied".
   c. A `cargo_unloaded` of supply (no `cargo` key; fuel has
      `cargo: "fuel"`) with `unloaded_t` above 0, at a base whose hex is L and
      whose holder at that event is Allied, on a day after the day of the
      first Allied `men_landed` in L, by a `task_force` other than that first
      landing's.
   d. Any of: a `search_flown` with `data.base` 1 by an Allied group; a patrol
      sortie, an `air_group_fatigue` with `change` "gained" and `sortie`
      "patrol" in the morning or the afternoon, by an Allied group at base 1;
      a `strike_launched` or `base_strike_launched` by an Allied group at
      base 1. A group's base is its record in the state at the end of the step
      before the event (decision 0128's reading of a search, a patrol and a
      strike).
4. The Japanese side does something.
   a. A `men_landed` in L whose unit is Japanese, on a day after the day of
      the first Allied `men_landed` in L.
   b. A `ground_fight` in L with `japanese_attacks` true, where base 1's
      holder at that event is Allied. Printed beside it: the same where base 1
      has been Allied at any event before the fight (it may have changed back).
   c. Either: a `shore_bombardment` of base 1 whose `task_force` is Japanese;
      or, in the state at the end of a night step, a Japanese task force with
      `mission` "bombardment" or "surface_combat" whose `position` is L.
"""
import argparse
import json
import sys

from common import Game, exports, load_json, is_batch, hex_of

DAYLIGHT = ('morning', 'afternoon')
NIGHT_MISSIONS = ('bombardment', 'surface_combat')


def read_seed(game, base):
    """Every count of conditions 2, 3 and 4 for one game, with the events
    each rests on."""
    ev = game.events
    hexes = game.base_hexes()
    if base not in hexes:
        raise SystemExit(f'{game.path}: no base {base} in the file')
    lunga = hexes[base]
    r = {'seed': game.seed, 'file': game.path, 'lunga_hex': list(lunga)}

    # -- 2. refused ----------------------------------------------------------
    two = {k: [] for k in ('staff_orders_rejected', 'player_orders_rejected', 'local_commander_orders_rejected',
                           'unknown_orders_rejected', 'staff_commands_refused', 'commander_commands_refused',
                           'local_commander_commands_refused')}
    for e in ev:
        if e['kind'] == 'order_rejected':
            order = game.orders.get(e['data']['order'])
            issuer = order['issuer'] if order else None
            key = {'staff': 'staff_orders_rejected', 'player': 'player_orders_rejected',
                   'local_commander': 'local_commander_orders_rejected'}.get(issuer, 'unknown_orders_rejected')
            two[key].append({'event': e['id'], 'side': order['side'] if order else None})
        elif e['kind'] == 'command_refused':
            key = {'staff': 'staff_commands_refused', 'player': 'commander_commands_refused',
                   'local_commander': 'local_commander_commands_refused'}[e['data']['issuer']]
            two[key].append({'event': e['id'], 'side': e['data']['side'], 'command': e['data']['command']})
    r['two'] = two

    # -- the first Allied man landed in L ------------------------------------
    first = None
    for e in ev:
        if e['kind'] == 'men_landed' and hex_of(e['data']['hex']) == lunga and game.unit_side(e) == 'allied':
            first = e
            break
    r['first_allied_landing'] = (
        {'event': first['id'], 'day': first['day'], 'task_force': first['data']['task_force']} if first else None)

    # -- 3 -------------------------------------------------------------------
    three = {'a': [], 'b': [], 'c': [], 'd_search': [], 'd_patrol': [], 'd_strike': []}
    if first:
        three['a'].append(first['id'])
    for e, holders in game.base_sides_timeline():
        k, d = e['kind'], e['data']
        if k == 'base_changed_side' and d['base'] == base and d['to'] == 'allied':
            three['b'].append(e['id'])
        elif (k == 'cargo_unloaded' and 'cargo' not in d and d['unloaded_t'] > 0 and first
              and hexes.get(d['base']) == lunga and holders.get(d['base']) == 'allied'
              and e['day'] > first['day'] and d['task_force'] != first['data']['task_force']):
            three['c'].append(e['id'])
    for e in ev:
        k, d = e['kind'], e['data']
        if k == 'search_flown':
            if d.get('base') == base and game.side_of('air_groups', d['air_group']) == 'allied':
                three['d_search'].append(e['id'])
        elif k == 'air_group_fatigue':
            if (d.get('change') == 'gained' and d.get('sortie') == 'patrol' and e['pulse'] in DAYLIGHT
                    and game.side_of('air_groups', d['air_group']) == 'allied'
                    and game.group_base_before(e, d['air_group']) == base):
                three['d_patrol'].append(e['id'])
        elif k in ('strike_launched', 'base_strike_launched'):
            if (game.side_of('air_groups', d['air_group']) == 'allied'
                    and game.group_base_before(e, d['air_group']) == base):
                three['d_strike'].append(e['id'])
    r['three'] = three

    # -- 4 -------------------------------------------------------------------
    four = {'a': [], 'b': [], 'b_after_ever': [], 'c_bombardment': [], 'c_night_in_hex': []}
    ever_allied = False
    for e, holders in game.base_sides_timeline():
        k, d = e['kind'], e['data']
        if holders.get(base) == 'allied':
            ever_allied = True
        if k == 'men_landed' and first and hex_of(d['hex']) == lunga and game.unit_side(e) == 'japanese' \
                and e['day'] > first['day']:
            four['a'].append(e['id'])
        elif k == 'ground_fight' and hex_of(d['hex']) == lunga and d.get('japanese_attacks'):
            if holders.get(base) == 'allied':
                four['b'].append(e['id'])
            if ever_allied:
                four['b_after_ever'].append(e['id'])
        elif k == 'shore_bombardment' and d['base'] == base \
                and game.side_of('task_forces', d['task_force']) == 'japanese':
            four['c_bombardment'].append(e['id'])
    for day, phase, st in game.steps():
        if phase != 'night':
            continue
        for tf in st['task_forces'].values():
            if (tf.get('side') == 'japanese' and tf.get('mission') in NIGHT_MISSIONS
                    and hex_of(tf.get('position')) == lunga):
                four['c_night_in_hex'].append({'day': day, 'task_force': tf['id'], 'mission': tf['mission']})
    r['four'] = four
    return r


LINES = [
    # (label, the record's words, test on a seed's row)
    ('2', "an order a staff gave, rejected (order_rejected, issuer staff)",
     lambda r: bool(r['two']['staff_orders_rejected'])),
    ('2', "a command a staff gave, refused (command_refused, issuer staff)",
     lambda r: bool(r['two']['staff_commands_refused'])),
    ('2', "apart: a commander's command refused (command_refused, issuer player)",
     lambda r: bool(r['two']['commander_commands_refused'])),
    ('2', "apart: a player's or scripted order rejected (order_rejected, issuer player)",
     lambda r: bool(r['two']['player_orders_rejected'])),
    ('2', "apart: a local commander's order rejected or command refused",
     lambda r: bool(r['two']['local_commander_orders_rejected'] or r['two']['local_commander_commands_refused'])),
    ('2', "apart: an order rejected whose order is not in the file's orders",
     lambda r: bool(r['two']['unknown_orders_rejected'])),
    ('3a', 'lands men on Guadalcanal', lambda r: bool(r['three']['a'])),
    ('3b', 'takes the airfield', lambda r: bool(r['three']['b'])),
    ('3c', 'lands supply there after the landing', lambda r: bool(r['three']['c'])),
    ('3d', 'flies from the field (a search, a patrol or a strike)',
     lambda r: bool(r['three']['d_search'] or r['three']['d_patrol'] or r['three']['d_strike'])),
    ('3d', '  of which: a search', lambda r: bool(r['three']['d_search'])),
    ('3d', '  of which: a patrol', lambda r: bool(r['three']['d_patrol'])),
    ('3d', '  of which: a strike', lambda r: bool(r['three']['d_strike'])),
    ('4a', 'lands men on Guadalcanal after the Allied landing', lambda r: bool(r['four']['a'])),
    ('4b', 'attacks the airfield ashore (base 1 Allied at the fight)', lambda r: bool(r['four']['b'])),
    ('4b', '  beside it: base 1 Allied at any time before the fight', lambda r: bool(r['four']['b_after_ever'])),
    ('4c', 'bombards it or enters its waters by night',
     lambda r: bool(r['four']['c_bombardment'] or r['four']['c_night_in_hex'])),
    ('4c', '  of which: a shore_bombardment of base 1', lambda r: bool(r['four']['c_bombardment'])),
    ('4c', '  of which: a night pulse ended in the hex', lambda r: bool(r['four']['c_night_in_hex'])),
]


def ranges(seeds):
    seeds = sorted(s for s in seeds if s is not None)
    out, i = [], 0
    while i < len(seeds):
        j = i
        while j + 1 < len(seeds) and seeds[j + 1] == seeds[j] + 1:
            j += 1
        out.append(str(seeds[i]) if i == j else f'{seeds[i]}-{seeds[j]}')
        i = j + 1
    return ', '.join(out) or '(none)'


def batch_refusals(path, seeds):
    b = load_json(path)
    if not is_batch(b):
        raise SystemExit(f'{path} is not a batch file')
    rows = {}
    for run in b['per_run']:
        if run['seed'] in seeds:
            ref = run.get('refusals')
            if ref is None:
                raise SystemExit(f'{path}: no refusals (a batch of version 6 is needed)')
            rows[run['seed']] = ref
    return rows


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('exports', nargs='+')
    ap.add_argument('--batch')
    ap.add_argument('--base', type=int, default=1)
    ap.add_argument('--list', action='store_true')
    ap.add_argument('--jsonl')
    a = ap.parse_args()

    rows, scenario = [], None
    for path, doc in exports(a.exports):
        name = doc['scenario'].get('path') or doc['scenario'].get('name')
        if scenario is None:
            scenario = name
        elif name != scenario:
            raise SystemExit(f'{path}: scenario {name}, where the first file has {scenario}')
        rows.append(read_seed(Game(doc, path), a.base))
    if not rows:
        raise SystemExit('no export read')
    seeds = [r['seed'] for r in rows]
    if len(set(seeds)) != len(seeds):
        raise SystemExit('two exports of the same seed')
    rows.sort(key=lambda r: r['seed'])
    n = len(rows)
    print(f'scenario: {scenario}')
    print(f'exports read: {n} (seeds {ranges(seeds)}); the hex of base {a.base}: {tuple(rows[0]["lunga_hex"])}')
    print()
    for label, words, test in LINES:
        held = [r['seed'] for r in rows if test(r)]
        print(f'{label:3} {words}: {len(held)} of {n} seeds')
        if a.list:
            print(f'      on seeds: {ranges(held)}')
            if 0 < len(held) < n:
                print(f'      not on:   {ranges([s for s in seeds if s not in held])}')
    total = {k: sum(len(r['two'][k]) for r in rows) for k in rows[0]['two']}
    print()
    print('2 in events, over all seeds: ' + ', '.join(f'{k} {v}' for k, v in total.items()))
    if a.batch:
        br = batch_refusals(a.batch, set(seeds))
        missing = sorted(set(seeds) - set(br))
        b_staff = sum(br[s][side]['staff_orders_not_as_held'] + br[s][side]['staff_orders_other']
                      for s in br for side in ('allied', 'japanese'))
        b_cmds = sum(br[s][side]['commands'] for s in br for side in ('allied', 'japanese'))
        e_staff = sum(len(r['two']['staff_orders_rejected']) for r in rows if r['seed'] in br)
        e_cmds = sum(len(r['two'][k]) for r in rows if r['seed'] in br
                     for k in ('staff_commands_refused', 'commander_commands_refused', 'local_commander_commands_refused'))
        print(f'2 in the batch file, the same seeds ({len(br)}): staff orders refused {b_staff} '
              f'(events: {e_staff}); commands refused, every issuer, {b_cmds} (events: {e_cmds})')
        if missing:
            print(f'  seeds with no run in the batch file: {ranges(missing)}')
    if a.jsonl:
        with open(a.jsonl, 'w') as f:
            for r in rows:
                f.write(json.dumps(r) + '\n')


if __name__ == '__main__':
    sys.exit(main())

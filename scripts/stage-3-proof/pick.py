"""Condition 6 of stage 3's proof, legibility (decision 0153, "Legibility"):
the rule that picks the 24 reasons, written before the run.

    python3 pick.py EXPORTS... [--out FILE] [--seeds 10] [--rows]

EXPORTS are the exports of arrangement (a), seeds 1, 2, 3 ... (files or
directories; any other JSON is skipped). `--rows` prints the table of
matchers and which rows are tested, and reads nothing.

The rule. A reason is a `staff_reason` event. It rests on an order when its
`data.order` is set, and on a staff command when one of its causes is a
`command_taken` or `command_refused` with `data.issuer` "staff" (its text says
"reason for the command to ..."). A reason that rests on neither (a day with
no order, or a command of the commander's) is not taken. For each of the six
doctrines of the record's table, in its order, the first four reasons that
rest on an order or a staff command and that a row of that doctrine matches
are taken, in event order on seed 1, both sides together; where seed 1 gives
fewer than four, the rest are taken from seed 2, then 3, up to seed 10. A
doctrine that gives fewer than four on the first ten seeds is reported with
the reasons it gave and the words "fewer than four on the first ten seeds".

Each row of MATCHERS below reads only the reason's own fields and, for a
command, the command's: `data.statement` (the intent it serves),
`data.chain` (the intents above it), the order's `target` and `text` from
the file's `orders`, the command's `data.command` and `data.detail`, and
where a row says so the reason's own words. Rows are added here and nowhere
else. A reason that two doctrines' rows match is taken by each (it is
flagged in the output).

Each reader is given the reason's five parts and nothing else, with the two
questions of decision 0128: what the staff is trying to do, and what would
change its mind. The text given is `prompt_text` in the output.
"""
import argparse
import json
import re
import sys

from common import Game, exports

WANT = 4
DOCTRINES = [
    'Units made ready',
    'The force chosen',
    'Ashore',
    'The supply that follows',
    'By night',
    'Cover and reconnaissance',
]


def st(r):
    return r['statement']


def is_order(r, target_kind=None, text=None):
    if r['sort'] != 'order':
        return False
    if target_kind and not r['order_target'].startswith(target_kind + ':'):
        return False
    return text is None or re.search(text, r['order_text']) is not None


def is_command(r, name, detail=None):
    if r['sort'] != 'command' or r['command'] != name:
        return False
    return detail is None or re.search(detail, r['command_detail']) is not None


def limit_on_formed_ready(r, later):
    """A staff `set_limit` under a take on a "ready a landing" the staff
    formed; `later` true for a later morning than the forming, false for the
    morning of the forming."""
    if not (is_command(r, 'set_limit') and re.match(r'take the place at ', st(r))):
        return False
    m = re.match(r"limit intent:(\d+)'s forces", r['command_detail'])
    if not m:
        return False
    formed = r['ctx']['ready_formed'].get((r['side'], int(m.group(1))))
    if formed is None:
        return False
    return r['day'] > formed if later else r['day'] == formed


# (doctrine, row, what it matches, decisions, tested on (a scenario of main) or None)
MATCHERS = [
    ('Units made ready', '1a', 'an order serving "ready a landing"',
     '0141', '66, 67 to 69, 72, 73',
     lambda r: is_order(r) and re.match(r'ready a landing at ', st(r))),
    ('Units made ready', '1b', 'an order serving "build up a base"',
     '0141', '66',
     lambda r: is_order(r) and re.match(r'build up base:', st(r))),
    ('The force chosen', '2a', 'the staff command forming "ready a landing" under a take',
     '0142, 0149', '67 to 69, 72, 73',
     lambda r: is_command(r, 'form_intent', r'an intent to ready a landing at ') and re.match(r'take the place at ', st(r))),
    ('The force chosen', '2b', 'the staff\'s set_limit on that "ready a landing" on the morning it is formed (part of the forming)',
     '0142 item 12, 0149', '67 to 69, 72, 73',
     lambda r: limit_on_formed_ready(r, later=False)),
    ('The force chosen', '2c', 'the staff\'s set_limit on that "ready a landing" on a later morning: a later lift, '
     'or the choice changed by a look at the place',
     '0143 item 12, 0152 item 6; 0147', '68 (a second lift), 72 (a look at the place)',
     lambda r: limit_on_formed_ready(r, later=True)),
    ('Ashore', '3a', 'a posture order to a land unit under a take',
     '0143', '67, 68, 72, 73',
     lambda r: is_order(r, 'land_unit', r'^posture ') and re.match(r'take the place at ', st(r))),
    ('Ashore', '3b', 'a posture order to a land unit under a hold',
     '0144', '69',
     lambda r: is_order(r, 'land_unit', r'^posture ') and re.match(r'hold base:', st(r))),
    ('The supply that follows', '4a', 'an order or staff command of a "keep supplied" under a hold (a run)',
     '0144', '69',
     lambda r: re.match(r'keep base:\d+ .*days of supply', st(r)) and re.search(r'hold base:', r['chain'])),
    ('The supply that follows', '4b', 'an order to a task force serving a hold itself (a force with men aboard sent by a hold)',
     '0152 item 8', None,
     lambda r: is_order(r, 'task_force') and re.match(r'hold base:', st(r))),
    ('By night', '5a', 'a speed or route order to a task force whose reason says the run "goes by night"',
     '0145', '70',
     lambda r: is_order(r, 'task_force', r'^(speed |route via )') and 'goes by night' in r['words']),
    ('By night', '5b', 'a route order to a task force serving "bombard"',
     '0146', '71',
     lambda r: is_order(r, 'task_force', r'^route via ') and re.match(r'bombard base:', st(r))),
    ('Cover and reconnaissance', '6a', 'the staff command forming a carrier task under a take',
     '0147, 0148', '73',
     lambda r: is_command(r, 'form_intent', r'an intent to operate the carrier force') and re.match(r'take the place at ', st(r))),
    ('Cover and reconnaissance', '6b', 'the staff command forming a carrier task under a denial an island\'s control formed',
     '0151', None,
     lambda r: is_command(r, 'form_intent', r'an intent to operate the carrier force')
     and re.match(r'deny the waters', st(r)) and 'island' in r['chain']),
    ('Cover and reconnaissance', '6c', 'a base strike order (air mission base_strike)',
     '0148', '73',
     lambda r: is_order(r, 'air_group', r'^air mission base_strike')),
    ('Cover and reconnaissance', '6d', 'a reconnaissance order (air mission reconnaissance)',
     '0147', '72',
     lambda r: is_order(r, 'air_group', r'^air mission reconnaissance')),
]


def reasons(game):
    """Every staff reason of the game that rests on an order or a staff
    command, as a flat record, in event order."""
    ready_formed = {}
    for e in game.events:
        # The number of a "ready a landing" the staff formed is in the
        # intent_formed its form_intent command causes.
        if e['kind'] == 'intent_formed':
            cause = next((game.by_id.get(c['event']) for c in e['causes'] if 'event' in c), None)
            if cause and cause['kind'] == 'command_taken' and 'an intent to ready a landing at' in cause['data'].get('detail', ''):
                ready_formed[(e['data']['side'], e['data']['intent'])] = e['day']
    ctx = {'ready_formed': ready_formed}
    out = []
    for e in game.events:
        if e['kind'] != 'staff_reason':
            continue
        d = e['data']
        r = {'seed': game.seed, 'event': e['id'], 'day': e['day'], 'side': d.get('side'),
             'statement': d.get('statement', ''), 'chain': d.get('chain', ''),
             'sort': None, 'order': None, 'order_target': '', 'order_text': '',
             'command': '', 'command_detail': '',
             'evidence': d.get('evidence', ''), 'binding': d.get('binding', ''),
             'worked': d.get('worked', ''), 'would_change': d.get('would_change', ''),
             'text': e['text'], 'ctx': ctx}
        r['words'] = ' '.join([r['evidence'], r['binding'], r['worked'], r['would_change']])
        if 'order' in d:
            o = game.orders.get(d['order'])
            r['sort'], r['order'] = 'order', d['order']
            if o:
                r['order_target'], r['order_text'] = o['target'], o['text']
        else:
            for c in e['causes']:
                cause = game.by_id.get(c.get('event')) if 'event' in c else None
                if cause and cause['kind'] in ('command_taken', 'command_refused') and cause['data']['issuer'] == 'staff':
                    r['sort'] = 'command'
                    r['command'] = cause['data']['command']
                    r['command_detail'] = cause['data']['detail']
                    r['command_event'] = cause['id']
                    r['command_refused'] = cause['kind'] == 'command_refused'
                    break
        if r['sort']:
            out.append(r)
    return out


def rows_of(r):
    return [(doc, row) for doc, row, _, _, _, test in MATCHERS if test(r)]


def prompt(r):
    return '\n'.join([
        'Serves: ' + r['statement'] + (f' (under: {r["chain"]})' if r['chain'] else ''),
        'Evidence: ' + (r['evidence'] or '(none)'),
        'Binding: ' + (r['binding'] or '(none)'),
        'Worked: ' + (r['worked'] or '(none)'),
        'Would change: ' + (r['would_change'] or '(none)')])


def print_rows():
    for doc, row, what, decisions, tested, _ in MATCHERS:
        print(f'{row:3} {doc}: {what} (decision {decisions}); '
              + (f'tested on scenario {tested}' if tested else 'UNTESTED: not built on main'))


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('exports', nargs='*')
    ap.add_argument('--out')
    ap.add_argument('--seeds', type=int, default=10)
    ap.add_argument('--rows', action='store_true')
    ap.add_argument('--as-seed-1', action='store_true',
                    help='read the one export given as seed 1 whatever its seed (to try the rows on a scenario of main)')
    a = ap.parse_args()
    if a.rows:
        print_rows()
        return 0
    games = {}
    for path, doc in exports(a.exports):
        g = Game(doc, path)
        if a.as_seed_1:
            if games:
                raise SystemExit('--as-seed-1 takes one export')
            g.seed = 1
        if g.seed in games:
            raise SystemExit(f'two exports of seed {g.seed}')
        games[g.seed] = g
    seeds = [s for s in range(1, a.seeds + 1) if s in games]
    if 1 not in games:
        raise SystemExit('no export of seed 1')
    absent = [s for s in range(1, a.seeds + 1) if s not in games]

    taken = {doc: [] for doc in DOCTRINES}
    for seed in seeds:
        if all(len(v) >= WANT for v in taken.values()):
            break
        for r in reasons(games[seed]):
            matched = rows_of(r)
            for doc in DOCTRINES:
                if len(taken[doc]) >= WANT:
                    continue
                mine = [row for d, row in matched if d == doc]
                if mine:
                    r2 = dict(r, doctrine=doc, rows=mine, also=[f'{d} {row}' for d, row in matched if d != doc])
                    taken[doc].append(r2)

    out, n = [], 0
    for doc in DOCTRINES:
        got = taken[doc]
        used = sorted(set(r['seed'] for r in got))
        note = '' if len(got) >= WANT else '; fewer than four on the first ten seeds'
        print(f'{doc}: {len(got)} reasons, from seed(s) {", ".join(map(str, used)) or "none"}{note}')
        for r in got:
            n += 1
            what = (f'order:{r["order"]} {r["order_target"]} {r["order_text"]}' if r['sort'] == 'order'
                    else f'staff command {r["command"]}: {r["command_detail"]}')
            also = f'  [also matches {", ".join(r["also"])}]' if r['also'] else ''
            print(f'  {n:2}. seed {r["seed"]} day {r["day"]} #{r["event"]} {r["side"]} row {"/".join(r["rows"])}: {what}{also}')
            out.append({k: v for k, v in r.items() if k not in ('ctx', 'words', 'text')}
                       | {'number': n, 'prompt_text': prompt(r)})
    if absent:
        print(f'(no export of seed(s) {", ".join(map(str, absent))} was given)')
    untested = [row for _, row, _, _, tested, _ in MATCHERS if not tested]
    print(f'rows not tested on main: {", ".join(untested)} (python3 pick.py --rows)')
    if a.out:
        with open(a.out, 'w') as f:
            json.dump(out, f, indent=1)
    return 0


if __name__ == '__main__':
    sys.exit(main())

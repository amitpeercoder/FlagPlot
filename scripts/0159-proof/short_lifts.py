"""Condition 4 of decision 0159's proof, "No take goes short" (decision 0159,
"The proof: how each condition is read", 4), from the umpire's exports of
arrangement (a), one file a seed, and the scenario file the exports name. It
prints counts and lifts and never a verdict: the mark is read by the
architect.

    python3 short_lifts.py EXPORTS... [--repo DIR] [--list] [--jsonl FILE]

EXPORTS are export files or directories of them. The scenario file is the one
each export names (`scenario.path`), read from the top of the repository
(`--repo`, by default two folders above this script), which must hold that
file as the exports were made from it. Stage 3's `common.py` is reused for
reading the exports.

What it reads, for each side:

- **The takes.** Every intent of the kind `take_place` in the export's
  `final.commands` (its hex, its plan, its `limits`), with the intents above
  it by `parent`. A take or an intent above it with a `by_day` (a latest day)
  stops the script: the arrangement sets none, and waiting past a latest day
  is not read here. A `command_taken` that sets a risk ("risk of intent:")
  or a forces limit ("limit intent:N's forces") on a take or an intent above
  it is named as a warning, since `final.commands` gives the limits as they
  ended; every take is read at balanced, 2 men wanted for each man counted.
- **A lift.** A staff order (`orders[].issuer` "staff") to a task force whose
  text is a route ("route via ...") "acting at" the take's hex, and whose
  `staff_reason` (`data.order` the order) serves the take (`data.intent` the
  take, or a `ready_landing` whose `parent` is the take), given on a morning
  when the force lies at a base of its side (`position` the base's hex in
  the state at the end of the day before; bases of the start, and those
  `base_formed` made, with their holders as stage 3's `base_sides_timeline`
  gives them). That base is the lift's source and the day it sails. A route
  order to a force not at a base (a change of route at sea) is not a new
  lift; their number is printed as a check. The first lift of a take is its
  `ready_landing`'s `task_force`'s first; every other is a later lift.
- **The morning it was released.** For the first lift, the last morning on
  or before its sailing on which the take's plan went to `released`
  (`plan_state_changed`); for a later lift, the morning it sails. The two
  differ where a first lift waits after release; both are printed.
- **The men wanted.** The side's ground estimate at the take's hex in the
  state at the end of the day before that morning (the export's `steps`,
  `ground_estimates`, keyed by hex; an estimate changes only in an `end`
  step), its `men` times 2, the figure of balanced. With no estimate held,
  the lift wants no number and is not short.
- **The men carried.** The men on the `troops` of the force's ships (ships
  whose `location` is the force) in the state at the end of the last step on
  or after the end of the day before at which the force still lies at its
  source: the men aboard as it leaves. Every unit aboard is counted, working
  units with the rest, as "the men of the units it carries" reads.
- **The units that would make up the number,** read from the game's own
  record and not the staff's words: on that morning (the state at the end of
  the day before), each land unit of the side that the take's forces limit
  lets it use (the take's own forces limit, else the nearest above it; one
  naming land units names those, one naming none or no limit lets it use
  every land unit of the side), with men at the lift's source (ashore in the
  source's hex, by the unit's record, or aboard a ship of a task force lying
  in that hex), not aboard the lift and not aboard another force that sails
  as a lift that morning. Its day is the latest of the `available_from` of
  the unit, of the ship that carries it and of that ship's task force, in
  the scenario file (decision 0140; no key is day 1); it would be usable at
  the source within the horizon where that day is no later than the morning
  plus 7. Their men, at the source, are added to the men carried.
  The same is printed with fighting units only (no `working = true` in the
  scenario file), beside it, not judged.
- A lift is **short** where the men carried are fewer than the men wanted.
  It **counts against the mark** where the men carried and the men of those
  units reach the men wanted. A short lift with no such units is counted and
  reported apart.
"""
import argparse
import json
import os
import re
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'stage-3-proof'))
from common import exports, hex_of, SIDES, Game  # noqa: E402

BALANCED = 2
HORIZON = 7
PHASES = ['start', 'orders', 'night', 'morning', 'afternoon', 'end']
ROUTE = re.compile(r'^route via .*acting at \((-?\d+),(-?\d+)\)')


# -- the scenario file: a reader of the tables it needs ----------------------

def read_tables(path, wanted=('task_force', 'ship', 'land_unit')):
    """{table: [dict]} for each `[[table]]` named in `wanted`, with each
    `key = value` whose value is a whole number, a quoted string or true or
    false; any other value is kept as its text. Enough for the scenario
    files' unit tables; it stops on a line it cannot read inside them."""
    out = {t: [] for t in wanted}
    current = None
    depth = 0
    with open(path, encoding='utf-8') as f:
        for n, raw in enumerate(f, 1):
            line = raw.strip()
            if depth:
                depth += line.count('[') - line.count(']')
                continue
            if not line or line.startswith('#'):
                continue
            m = re.match(r'^\[\[([A-Za-z0-9_.]+)\]\]$', line)
            if m:
                current = {} if m.group(1) in wanted else None
                if current is not None:
                    out[m.group(1)].append(current)
                continue
            if re.match(r'^\[[A-Za-z0-9_.]+\]$', line):
                current = None
                continue
            m = re.match(r'^([A-Za-z0-9_]+)\s*=\s*(.*)$', line)
            if not m:
                if current is not None:
                    raise SystemExit(f'{path}:{n}: cannot read "{line}"')
                continue
            key, value = m.group(1), m.group(2)
            if value.startswith('"'):
                end = value.index('"', 1)
                parsed = value[1:end]
            else:
                value = value.split('#', 1)[0].strip()
                if value.startswith('[') or value.startswith('{'):
                    depth = value.count('[') - value.count(']') + value.count('{') - value.count('}')
                    parsed = value
                elif re.match(r'^-?[0-9_]+$', value):
                    parsed = int(value.replace('_', ''))
                elif value in ('true', 'false'):
                    parsed = value == 'true'
                else:
                    parsed = value
            if current is not None:
                current[key] = parsed
    return out


# -- the state, step by step -------------------------------------------------

def rebuild(doc):
    """[(day, phase, state)], state {kind: {id: record}} for task forces,
    ships and land units, and {side: {hex: estimate}} for the estimates."""
    state = {'task_forces': {}, 'ships': {}, 'land_units': {}, 'est': {s: {} for s in SIDES}}
    out = []
    for step in doc['steps']:
        ch, rm = step.get('changed', {}), step.get('removed', {})
        for kind in ('task_forces', 'ships', 'land_units'):
            for rec in ch.get(kind, []) or []:
                state[kind][rec['id']] = rec
            for rid in rm.get(kind, []) or []:
                state[kind].pop(rid, None)
        for side in SIDES:
            for rec in (ch.get('ground_estimates') or {}).get(side, []) or []:
                state['est'][side][hex_of(rec['hex'])] = rec
            for hx in (rm.get('ground_estimates') or {}).get(side, []) or []:
                state['est'][side].pop(hex_of(hx), None)
        snap = {k: dict(v) for k, v in state.items() if k != 'est'}
        snap['est'] = {s: dict(v) for s, v in state['est'].items()}
        out.append((step['day'], step['phase'], snap))
    return out


def state_before_day(steps, day):
    """The state at the end of the last step before day `day`'s orders."""
    best = None
    for d, phase, st in steps:
        if (d, PHASES.index(phase)) < (day, PHASES.index('orders')):
            best = st
        else:
            break
    return best


def force_hex(st, force):
    tf = st['task_forces'].get(force)
    return hex_of(tf['position']) if tf else None


def aboard(st, force):
    """{unit: men} aboard the force's ships."""
    out = {}
    for ship in st['ships'].values():
        loc = ship.get('location')
        if isinstance(loc, dict) and loc.get('task_force') == force and ship.get('troops'):
            t = ship['troops']
            out[t['unit']] = out.get(t['unit'], 0) + t['men']
    return out


# -- one seed ----------------------------------------------------------------

def read_seed(game, tables):
    doc = game.doc
    steps = rebuild(doc)
    orders = game.orders
    intents = {s: {i['id']: i for i in doc['final']['commands'][s]['intents']} for s in SIDES}
    reason_of = {}
    for e in game.events:
        if e['kind'] == 'staff_reason' and e['data'].get('order') is not None:
            reason_of.setdefault(e['data']['order'], e)
    released = {s: {} for s in SIDES}   # plan -> [days released]
    for e in game.events:
        if e['kind'] == 'plan_state_changed' and e['data'].get('to') == 'released':
            released[e['data']['side']].setdefault(e['data']['plan'], []).append(e['day'])

    # Base hexes and holders by day: the holder at the start of each day's orders.
    hexes = game.base_hexes()
    # The holders before the first event of each day: those of the morning's
    # orders.
    first_of_day = {}
    for e, holders in game.base_sides_timeline():
        first_of_day.setdefault(e['day'], holders)
    start_holders = {b['id']: b['side'] for b in doc.get('bases', [])}

    def bases_of(side, day):
        h = start_holders
        for d in sorted(first_of_day):
            if d <= day:
                h = first_of_day[d]
        return {hexes[b]: b for b, s in h.items() if s == side and b in hexes}

    tf_day = {t['id']: t.get('available_from', 1) for t in tables['task_force']}
    ship_day = {t['id']: t.get('available_from', 1) for t in tables['ship']}
    unit_rec = {u['id']: u for u in tables['land_unit']}

    def unit_day(st, unit):
        d = unit_rec.get(unit, {}).get('available_from', 1)
        for ship in st['ships'].values():
            t = ship.get('troops')
            loc = ship.get('location')
            if t and t['unit'] == unit and isinstance(loc, dict):
                d = max(d, ship_day.get(ship['id'], 1), tf_day.get(loc.get('task_force'), 1))
        return d

    warnings = []
    rows = []
    for side in SIDES:
        its = intents[side]

        def chain(i):
            out = [i]
            while out[-1].get('parent') in its:
                out.append(its[out[-1]['parent']])
            return out

        takes = [i for i in its.values() if i['kind'] == 'take_place']
        for take in takes:
            ch = chain(take)
            if any(x.get('by_day') is not None for x in ch):
                raise SystemExit(f'{game.path}: {side} intent {take["id"]} or one above it has a by_day: not read here')
            for e in game.events:
                if e['kind'] == 'command_taken' and e['data']['side'] == side:
                    t = e['data'].get('detail', '')
                    for x in ch:
                        if (t.startswith(f'risk of intent:{x["id"]} ')
                                or t.startswith(f"limit intent:{x['id']}'s forces")):
                            warnings.append(f'seed {game.seed} {side}: event {e["id"]}: "{t}"')
            limit = next((x['limits']['forces'] for x in ch if x.get('limits', {}).get('forces')), None)
            named = set(limit.get('land_units') or []) if limit else set()
            place = hex_of(take['hex'])
            landings = [i for i in its.values() if i['kind'] == 'ready_landing' and i.get('parent') == take['id']]
            first_force = min(landings, key=lambda i: i['id'])['task_force'] if landings else None
            serving = {take['id']} | {i['id'] for i in landings}
            limit_source = {}
            for i in landings:
                f = (i.get('limits') or {}).get('forces') or {}
                if f.get('source') is not None:
                    limit_source[i['task_force']] = f['source']
            # A force has come in where, after a day, it landed men at the
            # place or arrived somewhere.
            came_in = {}
            for e in game.events:
                d = e['data']
                if e['kind'] == 'men_landed' and hex_of(d['hex']) == place:
                    came_in.setdefault(d['task_force'], []).append(e['day'])
                elif e['kind'] == 'task_force_arrived':
                    came_in.setdefault(d['task_force'], []).append(e['day'])
            lifts, reroutes = [], 0
            for oid, o in sorted(orders.items()):
                if o['issuer'] != 'staff' or o['side'] != side or not o['target'].startswith('task_force:'):
                    continue
                m = ROUTE.match(o['text'])
                if not m or (int(m.group(1)), int(m.group(2))) != place:
                    continue
                r = reason_of.get(oid)
                if not r or r['data'].get('intent') not in serving:
                    continue
                force = int(o['target'].split(':')[1])
                day = o['day']
                st = state_before_day(steps, day)
                at = force_hex(st, force)
                before = [l for l in lifts if l['force'] == force]
                if before and not any(before[-1]['sails'] <= d < day for d in came_in.get(force, [])):
                    # The same passage, not come in since its last lift: a
                    # change of route, not a new lift.
                    reroutes += 1
                    continue
                src = bases_of(side, day).get(at)
                from_limit = False
                if src is None:
                    src = limit_source.get(force)
                    from_limit = True
                    if src is None or src not in hexes:
                        raise SystemExit(f'{game.path}: {side} lift of task_force:{force} on day {day}: '
                                         f'not at a base and no source in its landing\'s limit')
                lifts.append({'force': force, 'sails': day, 'source': src,
                              'source_hex': list(hexes[src]), 'at_hex': list(at) if at else None,
                              'source_from_limit': from_limit, 'order': oid})
            seen_first = False
            for lift in sorted(lifts, key=lambda l: (l['sails'], l['force'])):
                force, day, at = lift['force'], lift['sails'], tuple(lift['source_hex'])
                first = force == first_force and not seen_first
                seen_first = seen_first or first
                rel = [d for d in released[side].get(take['plan'], []) if d <= day]
                morning = (rel[-1] if rel else day) if first else day
                st = state_before_day(steps, morning)
                est = st['est'][side].get(place)
                wanted = est['men'] * BALANCED if est else None
                # The men aboard as it leaves its source.
                carried_state = state_before_day(steps, day)
                lies = tuple(lift['at_hex']) if lift['at_hex'] else None
                for d, phase, s in steps:
                    if (d, PHASES.index(phase)) < (day - 1, PHASES.index('end')):
                        continue
                    if force_hex(s, force) != lies:
                        break
                    carried_state = s
                carried_units = aboard(carried_state, force)
                carried = sum(carried_units.values())
                # Units at the source on that morning.
                st_m = state_before_day(steps, morning)
                lifts_that_morning = {l['force'] for l in lifts if l['sails'] == morning}
                exclude = set()
                for f in lifts_that_morning | {force}:
                    exclude |= set(aboard(st_m, f))
                exclude |= set(carried_units)
                men_at = {}
                for u in st_m['land_units'].values():
                    if u['side'] != side:
                        continue
                    if u.get('men_ashore', 0) > 0 and hex_of(u.get('hex')) == at:
                        men_at[u['id']] = men_at.get(u['id'], 0) + u['men_ashore']
                for ship in st_m['ships'].values():
                    t, loc = ship.get('troops'), ship.get('location')
                    if not t or not isinstance(loc, dict) or 'task_force' not in loc:
                        continue
                    if force_hex(st_m, loc['task_force']) == at and ship.get('side') == side:
                        men_at[t['unit']] = men_at.get(t['unit'], 0) + t['men']
                candidates = []
                for u, men in sorted(men_at.items()):
                    if u in exclude or (named and u not in named):
                        continue
                    d = unit_day(st_m, u)
                    if d <= morning + HORIZON:
                        candidates.append({'unit': u, 'men': men, 'day': d,
                                           'working': bool(unit_rec.get(u, {}).get('working'))})
                make_up = sum(c['men'] for c in candidates)
                make_up_fighting = sum(c['men'] for c in candidates if not c['working'])
                short = wanted is not None and carried < wanted
                rows.append({
                    'side': side, 'take': take['id'], 'place': list(place), 'force': force,
                    'lift': 'first' if first else 'later', 'sails': day, 'released': morning,
                    'source': lift['source'], 'source_from_limit': lift['source_from_limit'],
                    'order': lift['order'],
                    'estimate': est['men'] if est else None,
                    'estimate_day': est['day'] if est else None,
                    'estimate_grade': est['grade'] if est else None,
                    'wanted': wanted, 'carried': carried, 'carried_units': carried_units,
                    'short': short, 'candidates': candidates,
                    'counts': bool(short and carried + make_up >= wanted),
                    'counts_fighting_only': bool(short and carried + make_up_fighting >= wanted),
                })
            if reroutes:
                warnings.append(f'seed {game.seed} {side} take {take["id"]}: {reroutes} route order(s) to the place '
                                f'given to a force not at a base (not lifts)')
    return {'seed': game.seed, 'file': game.path, 'lifts': rows}, warnings


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('exports', nargs='+')
    ap.add_argument('--repo', default=os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    ap.add_argument('--list', action='store_true', help='every lift, one a line')
    ap.add_argument('--jsonl')
    a = ap.parse_args()

    tables, scenario, seeds, out, warnings = None, None, [], [], []
    for path, doc in exports(a.exports):
        name = doc['scenario'].get('path')
        if scenario is None:
            scenario = name
            tables = read_tables(os.path.join(a.repo, name))
            # The scenario file holds the units the export starts with.
            ids = {u['id'] for u in doc['initial']['land_units']}
            if ids != {u['id'] for u in tables['land_unit']}:
                raise SystemExit(f'{name}: its land units are not those of {path}')
        elif name != scenario:
            raise SystemExit(f'{path}: scenario {name}, where the first file has {scenario}')
        row, w = read_seed(Game(doc, path), tables)
        out.append(row)
        warnings += w
        seeds.append(row['seed'])
    if not out:
        raise SystemExit('no export read')
    if len(set(seeds)) != len(seeds):
        raise SystemExit('two exports of the same seed')
    out.sort(key=lambda r: r['seed'])
    print(f'scenario: {scenario}')
    print(f'exports read: {len(out)}')
    print()
    if a.list:
        print('seed side      take force lift  sails released source estimate(day,grade)          wanted carried short '
              'make-up(units)                   counts (fighting only)')
        for r in out:
            for l in r['lifts']:
                est = (f'{l["estimate"]} (day {l["estimate_day"]}, {l["estimate_grade"]})'
                       if l['estimate'] is not None else 'none held')
                mk = ', '.join(f'land_unit:{c["unit"]} {c["men"]} from day {c["day"]}'
                               + (' working' if c['working'] else '') for c in l['candidates']) or '-'
                print(f'{r["seed"]:4} {l["side"]:8} {l["take"]:5} {l["force"]:5} {l["lift"]:5} {l["sails"]:5} '
                      f'{l["released"]:8} {l["source"]:6} {est:32} {str(l["wanted"]):>6} {l["carried"]:7} '
                      f'{"yes" if l["short"] else "no":5} {mk:32} {"yes" if l["counts"] else "no"} '
                      f'({"yes" if l["counts_fighting_only"] else "no"})')
        print()
    for s in SIDES:
        ls = [(r['seed'], l) for r in out for l in r['lifts'] if l['side'] == s]
        n_short = sum(1 for _, l in ls if l['short'])
        n_count = sum(1 for _, l in ls if l['counts'])
        n_count_f = sum(1 for _, l in ls if l['counts_fighting_only'])
        n_none = sum(1 for _, l in ls if l['wanted'] is None)
        print(f'{s}: lifts {len(ls)} (first {sum(1 for _, l in ls if l["lift"] == "first")}, '
              f'later {sum(1 for _, l in ls if l["lift"] == "later")}); with no count held {n_none}; '
              f'short {n_short}; short with units in reach that make up the number {n_count} '
              f'(fighting units only: {n_count_f}); short with none in reach {n_short - n_count}')
        if n_count:
            print(f'  on seeds: {sorted({seed for seed, l in ls if l["counts"]})}')
    total = sum(1 for r in out for l in r['lifts'] if l['counts'])
    print(f'both sides: lifts short with units in reach that make up the number: {total}')
    if warnings:
        print()
        print(f'warnings ({len(warnings)}):')
        for w in warnings[:50]:
            print(f'  {w}')
        if len(warnings) > 50:
            print(f'  ... and {len(warnings) - 50} more (all in --jsonl)')
    if a.jsonl:
        with open(a.jsonl, 'w') as f:
            for r in out:
                f.write(json.dumps(r) + '\n')


if __name__ == '__main__':
    sys.exit(main())

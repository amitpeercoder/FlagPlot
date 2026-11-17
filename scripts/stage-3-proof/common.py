"""Shared reading of the umpire's files for the scripts of stage 3's proof
(decision 0153). Python 3.9 or later, standard library only.

These scripts are the umpire's: they read the truth (the export of
`sim-cli export` and the batch file of `sim-cli batch`). Nothing that plays
the game reads them or their output.

An export (replay version 6) is read for:
  - `events`: each event's `id`, `day`, `pulse` (null for a day-level event),
    `kind`, `text`, `causes` and `data`;
  - `orders`: each order's `id`, `side`, `day`, `issuer` and `target`;
  - `initial`, `steps` and `final`: the records of task forces, air groups,
    land units and bases, rebuilt step by step (a step carries the records
    that changed since the step before, and `removed` the ids gone);
  - `bases`: each base of the start with its hex (a base a landing formed is
    added by its `base_formed` event).
"""
import json
import os
import sys

PHASES = ['start', 'orders', 'night', 'morning', 'afternoon', 'end']
PHASE_INDEX = {p: i for i, p in enumerate(PHASES)}
KINDS = ('task_forces', 'air_groups', 'land_units', 'bases')
SIDES = ('allied', 'japanese')


def load_json(path):
    with open(path) as f:
        return json.load(f)


def is_export(doc):
    return isinstance(doc, dict) and doc.get('format') == 'flagplot-replay'


def is_batch(doc):
    return isinstance(doc, dict) and doc.get('format') == 'flag-plot-batch'


def export_paths(paths):
    """Every .json file named, or in a directory named, sorted by name. The
    caller drops those that are not exports."""
    out = []
    for p in paths:
        if os.path.isdir(p):
            out.extend(os.path.join(p, f) for f in sorted(os.listdir(p)) if f.endswith('.json'))
        else:
            out.append(p)
    return out


def exports(paths):
    """Yield (path, export) for each export among the paths, in seed order is
    left to the caller; a batch file or another JSON is skipped with a note on
    stderr."""
    for path in export_paths(paths):
        doc = load_json(path)
        if not is_export(doc):
            if not is_batch(doc):
                print(f'skipped (not an export): {path}', file=sys.stderr)
            continue
        yield path, doc


def hex_of(value):
    """A hex as a (q, r) tuple, from [q, r] or {"q": .., "r": ..}."""
    if value is None:
        return None
    if isinstance(value, dict):
        return (value['q'], value['r'])
    return (value[0], value[1])


def step_key(day, phase):
    return (day, PHASE_INDEX[phase])


def event_key(event):
    """Where an event falls among the steps: a pulse's event is in that
    pulse's step; a day-level event (pulse null) is placed in the day's
    `orders` step when it comes before the night, which the scripts here never
    need to tell apart, so it is given the `orders` step."""
    pulse = event.get('pulse')
    return (event['day'], PHASE_INDEX[pulse] if pulse else PHASE_INDEX['orders'])


class Game:
    """One export, read once."""

    def __init__(self, doc, path=''):
        self.doc = doc
        self.path = path
        self.seed = doc['scenario'].get('seed')
        self.events = doc['events']
        self.by_id = {e['id']: e for e in self.events}
        self.orders = {o['id']: o for o in doc.get('orders', [])}
        self._steps = None
        self._sides = None
        self._base_hexes = None

    # -- the records, step by step ------------------------------------------

    def steps(self):
        """A list of (day, phase, state) for each step, where state is
        {kind: {id: record}} for the four kinds, the state at the end of that
        step. Built once."""
        if self._steps is not None:
            return self._steps
        state = {k: {} for k in KINDS}
        out = []
        steps = self.doc.get('steps')
        if not steps:
            raise SystemExit(f'{self.path}: the export has no steps (replay version 4 or later is needed)')
        for step in steps:
            for kind in KINDS:
                for rec in step.get('changed', {}).get(kind, []) or []:
                    state[kind][rec['id']] = rec
                gone = step.get('removed', {}).get(kind)
                if isinstance(gone, list):
                    for rid in gone:
                        state[kind].pop(rid, None)
            snap = {k: dict(v) for k, v in state.items()}
            out.append((step['day'], step['phase'], snap))
        self._steps = out
        return out

    def state_before(self, event):
        """The state at the end of the last step before the event's step: for
        an event of the morning pulse, the state at the end of the night."""
        key = event_key(event)
        best = None
        for day, phase, st in self.steps():
            if step_key(day, phase) < key:
                best = st
            else:
                break
        return best

    def state_after(self, event):
        """The state at the end of the event's own step."""
        key = event_key(event)
        for day, phase, st in self.steps():
            if step_key(day, phase) >= key:
                return st
        return self.steps()[-1][2]

    # -- sides ---------------------------------------------------------------

    def sides(self):
        """{(kind, id): side} for every task force, air group, land unit and
        base any record of the file names (start, steps, end). A unit's side
        never changes; a base's does, and `base_side_at` reads that."""
        if self._sides is not None:
            return self._sides
        out = {}
        sources = [self.doc.get('initial', {}), self.doc.get('final', {})]
        sources += [s.get('changed', {}) for s in self.doc.get('steps', [])]
        for src in sources:
            for kind in ('task_forces', 'air_groups', 'land_units'):
                for rec in src.get(kind, []) or []:
                    if 'side' in rec:
                        out[(kind, rec['id'])] = rec['side']
        self._sides = out
        return out

    def side_of(self, kind, ident):
        return self.sides().get((kind, ident))

    def unit_side(self, men_landed):
        """The side of the unit of a `men_landed`, or, where no record names
        the unit, of the task force that landed it."""
        d = men_landed['data']
        return self.side_of('land_units', d['unit']) or self.side_of('task_forces', d['task_force'])

    # -- bases ---------------------------------------------------------------

    def base_hexes(self):
        """{base: (q, r)} from the file's `bases` and each `base_formed`."""
        if self._base_hexes is not None:
            return self._base_hexes
        out = {b['id']: (b['q'], b['r']) for b in self.doc.get('bases', [])}
        for e in self.events:
            if e['kind'] in ('base_formed', 'march_base_formed') and 'base' in e['data'] and 'hex' in e['data']:
                out[e['data']['base']] = hex_of(e['data']['hex'])
        self._base_hexes = out
        return out

    def base_sides_timeline(self):
        """Yield (event, {base: side}) for each event in order, the map being
        the holder of each base just before that event: the start's holders,
        a formed base's side from its `base_formed`, and each
        `base_changed_side`'s `to` applied after its own event."""
        holders = {b['id']: b['side'] for b in self.doc.get('bases', [])}
        for e in self.events:
            yield e, holders
            if e['kind'] in ('base_formed', 'march_base_formed') and 'base' in e['data'] and 'side' in e['data']:
                holders = dict(holders)
                holders[e['data']['base']] = e['data']['side']
            elif e['kind'] == 'base_changed_side':
                holders = dict(holders)
                holders[e['data']['base']] = e['data']['to']

    # -- air groups ----------------------------------------------------------

    def group_base_before(self, event, group):
        """The base the group was at in the state before the event, or None
        (aboard a ship, lost, or not yet in play)."""
        st = self.state_before(event)
        rec = st['air_groups'].get(group) if st else None
        if rec is None:
            st = self.state_after(event)
            rec = st['air_groups'].get(group) if st else None
        return rec.get('base') if rec else None


def by_seed(rows):
    return sorted(rows, key=lambda r: (r['seed'] is None, r['seed']))

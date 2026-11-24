"""The men unfed, as measured (decision 0161, "The measurement of the men unfed"),
with the count G in place of F as first written (decisions 0161 and 0229): a man
lost in a hex after a fall-back to it followed by the arrival of its own march.
Umpire's reading from exports. Prints counts, never a verdict.

    python3 -I unfed.py DIR [--all] [--rows OUT.jsonl]

`--all` reads every unit of the side, those ashore at the start included, and
prints the part of the men who begin ashore (decision 0229, "How it is judged", 3):
the men lost unfed over the men at the start, split into those lost in a hex where
a base of the side stood and the rest (on the march, with no base).

DIR holds seedN.json exports of one arrangement. The base reading (L landed,
U lost) is scripts/0159-proof/hunger.py's, imported and used as it is.
"""
import argparse, json, os, re, sys, glob
from collections import defaultdict
from fractions import Fraction

_HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(_HERE, '..', 'stage-3-proof'))
sys.path.insert(0, os.path.join(_HERE, '..', '0159-proof'))
from common import Game, load_json, hex_of, SIDES  # noqa
import hunger  # noqa

MOVES = ('unit_fell_back', 'spent_unit_fell_back', 'march_arrived')
HEXRE = re.compile(r'\((-?\d+),(-?\d+)\)')


ALL = False


def read_seed(game):
    base = hunger.read_seed(game)  # L, U per side, units, lost_events
    events = game.events
    order = {e['id']: i for i, e in enumerate(events)}
    # place of each counted unit: (event index, hex) of counted landings
    first_lost_by_id = {}
    counted_units = {}
    for s in SIDES:
        for u in base['units'][s]:
            counted_units[u] = s
    # rebuild each unit's counted landings as hunger.py counts them
    landed_ids = {i for s in SIDES for i in base['landed_events'][s]}
    place = defaultdict(list)
    for e in events:
        if e['id'] in landed_ids:
            place[e['data']['unit']].append((order[e['id']], hex_of(e['data']['hex'])))
    lost_ids = {i for s in SIDES for i in base['lost_events'][s]}
    if ALL:
        # extension: every men_lost_unfed of every unit; a unit with no counted landing has its
        # starting hex (the export's initial record) as its place
        init = {r['id']: hex_of(r['hex']) for r in game.doc['initial'].get('land_units', []) if r.get('hex')}
        lost_ids = {e['id'] for e in events if e['kind'] == 'men_lost_unfed'}
        for e in events:
            if e['kind'] == 'men_lost_unfed':
                u = e['data']['unit']
                if u not in counted_units:
                    counted_units[u] = game.side_of('land_units', u)
                    place[u] = [(-1, init.get(u, hex_of(e['data']['hex'])))]
    # moves of each unit, in event order
    moves = defaultdict(list)  # unit -> [(idx, kind, to)]
    for e in events:
        if e['kind'] in MOVES:
            moves[e['data']['unit']].append((order[e['id']], e['kind'], hex_of(e['data']['to'])))

    res = {s: {'F_unit_fell_back': 0, 'F_spent_unit_fell_back': 0, 'F_at_place': 0, 'F_elsewhere': 0,
               'A': 0, 'M': 0, 'X': 0, 'X_events': [], 'F_by_hex': defaultdict(int)} for s in SIDES}
    F_events = defaultdict(list)  # (side, hex) -> [event]
    for e in events:
        if e['id'] not in lost_ids:
            continue
        u = e['data']['unit']
        s = counted_units[u]
        n = e['data']['lost']
        H = hex_of(e['data']['hex'])
        idx = order[e['id']]
        here = [hx for i, hx in place[u] if i < idx]
        pl = here[-1] if here else place[u][0][1]
        prior = [m for m in moves[u] if m[0] < idx]
        last = prior[-1] if prior else None
        r = res[s]
        if last and last[1] in ('unit_fell_back', 'spent_unit_fell_back') and last[2] == H:
            r['F_' + last[1]] += n
            r['F_at_place' if H == pl else 'F_elsewhere'] += n
            r['F_by_hex'][H] += n
            F_events[(s, H)].append((e, last[1]))
        elif last is None and H == pl:
            r['A'] += n
        elif last and last[1] == 'march_arrived' and last[2] == H:
            r['M'] += n
            if len(prior) >= 2 and prior[-2][1] in ('unit_fell_back', 'spent_unit_fell_back') and prior[-2][2] == H:
                r['M_after_fell_back_to_same_hex'] = r.get('M_after_fell_back_to_same_hex', 0) + n
                F_events[(s, H)].append((e, 'march_after_' + prior[-2][1]))
        else:
            r['X'] += n
            r['X_events'].append({'event': e['id'], 'unit': u, 'hex': list(H), 'lost': n,
                                  'place': list(pl), 'last_move': (last[1], list(last[2])) if last else None})

    # per-hex table for any F
    holders_at = {}  # event id -> holders map just before
    for e, holders in game.base_sides_timeline():
        holders_at[e['id']] = holders
    base_hexes = game.base_hexes()
    formed = {e['data']['base'] for e in events if e['kind'] in ('base_formed', 'march_base_formed')}
    hexes = []
    for (s, H), lst in sorted(F_events.items()):
        lst = sorted(lst, key=lambda x: x[0]['id'])
        first_loss = lst[0][0]
        last_loss = lst[-1][0]
        # men who fell back there: the fall-back events of the side with `to` H, units whose loss was counted F
        units = sorted({e['data']['unit'] for e, _ in lst})
        fb = [e for e in events if e['kind'] in ('unit_fell_back', 'spent_unit_fell_back')
              and e['data']['side'] == s and hex_of(e['data']['to']) == H]
        fb_men = sum(e['data']['men'] for e in fb)
        fb_days = sorted({e['day'] for e in fb})
        first_fb_day = fb_days[0] if fb_days else None
        holders = holders_at[first_loss['id']]
        bases_here = [b for b, hx in base_hexes.items() if hx == H and holders.get(b) == s]
        base_info = [{'base': b, 'kind': 'formed' if b in formed else 'start'} for b in sorted(bases_here)]
        unloaded = 0.0
        for e in events:
            if e['kind'] == 'cargo_unloaded' and e['day'] >= (first_fb_day or 0):
                b = e['data']['base']
                if base_hexes.get(b) == H and holders_at[e['id']].get(b) == s:
                    unloaded += e['data']['unloaded_t']
        lastfed = None
        for e in events:
            if order[e['id']] >= order[first_loss['id']]:
                break
            if e['kind'] == 'men_fed' and e['data']['side'] == s and hex_of(e['data']['hex']) == H:
                lastfed = {'day': e['day'], 'men': e['data']['men'], 'fed': e['data']['fed'], 'unfed': e['data']['unfed'],
                           'base': e['data']['base']}
        # the staff's orders naming H from the first fall-back day
        named = defaultdict(int)
        for o in game.orders.values():
            if o['side'] != s or o.get('issuer') != 'staff' or o['day'] < (first_fb_day or 0):
                continue
            t = o['text']
            hs = [(int(a), int(b)) for a, b in HEXRE.findall(t)]
            if t.startswith('route via'):
                if 'acting at' in t:
                    pre, post = t.split('acting at', 1)
                    ph = [(int(a), int(b)) for a, b in HEXRE.findall(pre)]
                    if ph and ph[-1] == H:
                        named['route end'] += 1
                    elif H in ph:
                        named['route via (not end)'] += 1
                    if H in [(int(a), int(b)) for a, b in HEXRE.findall(post)]:
                        named['acting at'] += 1
                else:
                    if hs and hs[-1] == H:
                        named['route end'] += 1
                    elif H in hs:
                        named['route via (not end)'] += 1
            elif t.startswith('prepare a landing') and H in hs:
                named['prepare a landing'] += 1
            elif t.startswith('unload supply'):
                tf = o['target']
                if tf.startswith('task_force:'):
                    st = game.state_before({'day': o['day'], 'pulse': None})
                    rec = st['task_forces'].get(int(tf.split(':')[1])) if st else None
                    if rec and rec.get('position') and (rec['position']['q'], rec['position']['r']) == H:
                        named['unload (force lay there)'] += 1
            elif H in hs:
                named['other order naming it'] += 1
        for e in events:
            if e['kind'] == 'command_taken' and e['day'] >= (first_fb_day or 0) and e['data'].get('side', None) in (None, s):
                if f'({H[0]},{H[1]})' in e['text'] and s.capitalize() in e['text']:
                    named['command_taken naming it'] += 1
        hexes.append({
            'side': s, 'hex': list(H), 'units': units, 'men_fell_back': fb_men, 'fall_back_days': fb_days,
            'first_loss_day': first_loss['day'], 'last_loss_day': last_loss['day'],
            'men_lost_in_table': sum(e['data']['lost'] for e, _ in lst),
            'by_basis': {k: sum(e['data']['lost'] for e, kk in lst if kk == k) for k in sorted({kk for _, kk in lst})},
            'bases_of_the_side_at_first_loss': base_info,
            'tons_unloaded_from_first_fall_back': round(unloaded, 3),
            'last_men_fed_before_first_loss': lastfed,
            'orders_naming_it': dict(named),
        })

    out = {'seed': game.seed, 'L': base['landed'], 'U': base['lost']}
    # check beside the base reading: every men_lost_unfed of every unit of the side, and the part
    # of it that is of a unit with no counted landing (a unit ashore at the start), by hex
    allU = {s: 0 for s in SIDES}
    unc = {s: defaultdict(int) for s in SIDES}
    for e in events:
        if e['kind'] == 'men_lost_unfed':
            u = e['data']['unit']
            s = game.side_of('land_units', u)
            allU[s] += e['data']['lost']
            if e['id'] not in lost_ids:
                h = hex_of(e['data']['hex'])
                unc[s][f'{h[0]},{h[1]}'] += e['data']['lost']
    out['U_all_units'] = allU
    if ALL:
        out['U'] = allU
    out['U_uncounted_by_hex'] = {s: dict(unc[s]) for s in SIDES}
    # Decision 0229: the men who begin ashore (units of the export's initial record) and what
    # they lose unfed, in a hex where a base of their side stood at the loss and elsewhere.
    start_men = {s: 0 for s in SIDES}
    start_units = {}
    for r in game.doc['initial'].get('land_units', []):
        if r.get('hex') and r.get('men_ashore'):
            sd = r['side']
            start_men[sd] += r['men_ashore']
            start_units[r['id']] = sd
    start_lost = {s: 0 for s in SIDES}
    start_lost_base = {s: 0 for s in SIDES}
    for e, holders in game.base_sides_timeline():
        if e['kind'] == 'men_lost_unfed' and e['data']['unit'] in start_units:
            sd = start_units[e['data']['unit']]
            hx = hex_of(e['data']['hex'])
            start_lost[sd] += e['data']['lost']
            if any(bhx == hx and holders.get(b) == sd for b, bhx in game.base_hexes().items()):
                start_lost_base[sd] += e['data']['lost']
    out['start'] = {'men': start_men, 'lost': start_lost, 'lost_where_a_base_stood': start_lost_base}
    for s in SIDES:
        r = res[s]
        r['F_by_hex'] = {f'{h[0]},{h[1]}': v for h, v in r['F_by_hex'].items()}
        r['F'] = r['F_unit_fell_back'] + r['F_spent_unit_fell_back']
        out[s] = r
        out[s]['U_ext'] = allU[s] if ALL else None
    out['hexes'] = hexes
    return out


def med(vals):
    v = sorted(vals)
    n = len(v)
    if not n:
        return None
    return v[n // 2] if n % 2 else (v[n // 2 - 1] + v[n // 2]) / 2


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--rows')
    ap.add_argument('--all', action='store_true')
    a = ap.parse_args()
    global ALL
    ALL = a.all
    rows = []
    for p in sorted(glob.glob(os.path.join(a.dir, 'seed*.json')), key=lambda p: int(re.findall(r'\d+', os.path.basename(p))[0])):
        rows.append(read_seed(Game(load_json(p), p)))
    if a.rows:
        with open(a.rows, 'w') as f:
            for r in rows:
                f.write(json.dumps(r) + '\n')
    print(f'seeds read: {len(rows)}')
    for s in SIDES:
        on = [r for r in rows if r['L'][s] > 0]
        print(f'{s}: landed men on {len(on)} of {len(rows)} seeds')
        for name, f in (('U/L', lambda r: Fraction(r['U'][s], r['L'][s])),
                        ('F/L', lambda r: Fraction(r[s]['F'], r['L'][s])),
                        ('A/L', lambda r: Fraction(r[s]['A'], r['L'][s])),
                        ('M/L', lambda r: Fraction(r[s]['M'], r['L'][s])),
                        ('X/L', lambda r: Fraction(r[s]['X'], r['L'][s])),
                        ('(U-F)/L', lambda r: Fraction(r['U'][s] - r[s]['F'], r['L'][s]))):
            m = med([f(r) for r in on])
            print(f'  median {name}: {float(m):.4f}' if m is not None else f'  median {name}: -')
        mL = med([r['L'][s] for r in on]); mU = med([r['U'][s] for r in on]); mF = med([r[s]['F'] for r in on])
        print(f'  median L {mL}, U {mU}, F {mF}')
        withU = [r for r in on if r['U'][s] > 0]
        over = [r['seed'] for r in withU if 2 * r[s]['F'] > r['U'][s]]
        print(f'  seeds with U>0: {len(withU)}; seeds with F more than half of U: {len(over)}')
        print(f'  seeds with any F: {sum(1 for r in on if r[s]["F"] > 0)}; seeds with any X: {sum(1 for r in on if r[s]["X"] > 0)}')
        tot = defaultdict(int)
        for k in ('M_after_fell_back_to_same_hex', 'F_unit_fell_back', 'F_spent_unit_fell_back', 'F_at_place', 'F_elsewhere', 'A', 'M', 'X', 'F'):
            tot[k] = sum(r[s].get(k, 0) for r in rows)
        G = [Fraction(r[s]['F'] + r[s].get('M_after_fell_back_to_same_hex', 0), r['L'][s]) for r in on]
        mG = med(G)
        print(f'  median G/L (F plus the men lost after a fall-back to the hex followed by its own march): {float(mG):.4f}' if mG is not None else '  median G/L: -')
        st = [r for r in rows if r['start']['men'][s] > 0]
        if st:
            print('  men who begin ashore: median %s; lost unfed, median share %.4f; lost where a base of the side stood, median share %.4f (sums: men %s, lost %s, lost at a base %s)' % (
                med([r['start']['men'][s] for r in st]),
                float(med([Fraction(r['start']['lost'][s], r['start']['men'][s]) for r in st])),
                float(med([Fraction(r['start']['lost_where_a_base_stood'][s], r['start']['men'][s]) for r in st])),
                sum(r['start']['men'][s] for r in st), sum(r['start']['lost'][s] for r in st),
                sum(r['start']['lost_where_a_base_stood'][s] for r in st)))
        print('  sums over seeds:', dict(tot), 'U', sum(r['U'][s] for r in rows), 'L', sum(r['L'][s] for r in rows))
        unc_tot = defaultdict(int)
        for r in rows:
            for h, v in r['U_uncounted_by_hex'][s].items():
                unc_tot[h] += v
        print(f'  check: men_lost_unfed of every unit of the side: median {med([r["U_all_units"][s] for r in rows])}, '
              f'min {min(r["U_all_units"][s] for r in rows)}, max {max(r["U_all_units"][s] for r in rows)}; '
              f'of units with no counted landing, summed over seeds, by hex: {dict(sorted(unc_tot.items()))}')
        byhex = defaultdict(int); seeds_hex = defaultdict(set)
        for r in rows:
            for h, v in r[s]['F_by_hex'].items():
                byhex[h] += v; seeds_hex[h].add(r['seed'])
        print('  F by hex:', {h: (v, len(seeds_hex[h])) for h, v in sorted(byhex.items())}, '(men, seeds)')
    return 0


if __name__ == '__main__':
    sys.exit(main())

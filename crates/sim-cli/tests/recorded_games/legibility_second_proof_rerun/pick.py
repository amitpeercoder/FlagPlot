"""Pick the 24 reasons of decision 0128's legibility test (the architect's
ruling of Nov 16, 2026, restating the set before the test was run).

The rule, written before the pick. A reason is a staff_reason event. Its
doctrine is the kind of the intent it serves, read from its serves line:
  cover    "cover base:N from the air"
  raid     "raid base:N from base:M"
  carrier  "operate the carrier force ..."
  strike   "strike ships from base:N"
  stand    "stand at ... with task_force:N"
Sources: cover 6, raid 6 and carrier 4 from the week's arrangement (a)
(07a_week.toml), seed 1, both sides; strike 4 from scenario 53 and stand 4
from scenario 54, each its file as it stands (the main case), its own seed.
Within a doctrine the reasons are ordered by day, then event number. They
are taken alternately an order's reason and a day-with-no-order reason (an
order first): at each step the first not yet taken of the sort due; where a
doctrine has none of the sort due, the other sort. A reason whose worked
text is the same as one already taken is skipped. A reason of a staff
command (one caused by a command taken or refused, such as a plan made
ready or released under a leave) is neither an order nor a day with no
order and is not taken, as in stage 1's pick.
"""
import json, re, sys
S = sys.argv[1]
sources = {'cover': ('week', 6), 'raid': ('week', 6), 'carrier': ('week', 4), 'strike': ('53', 4), 'stand': ('54', 4)}
files = {'week': f'{S}/07a_seed1.json', '53': f'{S}/53.json', '54': f'{S}/54.json'}
pat = {'cover': r'^cover ', 'raid': r'^raid ', 'carrier': r'^operate the carrier force', 'strike': r'^strike ships', 'stand': r'^stand at '}
out = []
n = 0
for doc in ['cover', 'raid', 'carrier', 'strike', 'stand']:
    src, want = sources[doc]
    events = json.load(open(files[src]))['events']
    by_id = {e['id']: e for e in events}
    rows = []
    for e in events:
        if e['kind'] != 'staff_reason':
            continue
        d = e['data']
        m = re.search(r"\[serves (.*?)\]: ", e['text'])
        serves = m.group(1) if m else ''
        st = re.sub(r'^intent \d+: ', '', serves)
        if not re.search(pat[doc], st):
            continue
        cmd = any(by_id.get(c.get('event'), {}).get('kind') in ('command_taken', 'command_refused')
                  for c in e.get('causes', []) if isinstance(c, dict))
        if 'order' in d:
            sort = 'order'
        elif not cmd:
            sort = 'no_order'
        else:
            continue
        rows.append((e['day'], e['id'], sort, serves, d, e['text']))
    rows.sort(key=lambda r: (r[0], r[1]))
    taken, seen, due = [], set(), 'order'
    pools = {s: [r for r in rows if r[2] == s] for s in ('order', 'no_order')}
    while len(taken) < want:
        pick = None
        for sort in (due, 'no_order' if due == 'order' else 'order'):
            while pools[sort] and pools[sort][0][4]['worked'] in seen:
                pools[sort].pop(0)
            if pools[sort]:
                pick = pools[sort].pop(0)
                break
        if pick is None:
            break
        seen.add(pick[4]['worked'])
        taken.append(pick)
        due = 'no_order' if due == 'order' else 'order'
    for day, eid, sort, serves, d, text in taken:
        n += 1
        out.append({
            'number': n, 'doctrine': doc, 'source': src, 'sort': sort, 'day': day, 'event': eid,
            'side': d.get('side'), 'serves_line': serves,
            'evidence': d.get('evidence', ''), 'binding': d.get('binding', ''),
            'worked': d.get('worked', ''), 'would_change': d.get('would_change', ''),
            'prompt_text': '\n'.join([
                'Serves: ' + serves,
                'Evidence: ' + (d.get('evidence') or '(none)'),
                'Binding: ' + (d.get('binding') or '(none)'),
                'Worked: ' + (d.get('worked') or '(none)'),
                'Would change: ' + (d.get('would_change') or '(none)')]),
        })
print(json.dumps(out, indent=1))

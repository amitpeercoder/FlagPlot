"""Read the games of condition 7 of decision 0156's proof from the umpire's
side, once each game is over.

    python3 read_games.py ARM_FOLDER [--sim-cli PATH] [--out FILE]

ARM_FOLDER holds `games/gN.txt`, the game files `sim-cli play --game` kept,
one for each game N (seed N). For each it runs, from the top of the
repository (the folder two above this script):

    sim-cli play --game gN.txt --export-save gN.save.json --export-transcript gN.transcript.jsonl
    sim-cli export gN.save.json --out gN.export.json

and reads:
  - over: whether the game is over (`--export-save` is refused until it is);
  - strike_at_ships: whether an Allied air group launched a strike at ships:
    an event `strike_launched` (not `base_strike_launched`, which is a raid on
    a base) whose `data.air_group` is a group the export's `initial` gives
    side `allied`; with the count of such launches;
  - margin: the Allied margin, from the `result` of the last answer of the
    transcript that carries one (`result.allied.margin`), a decimal string;
  - orders_inside: the Allied orders the saved game holds with a `pulse`
    (given at 0600 or 1200 and handed in), and, beside it, the orders the
    transcript shows queued inside a day: each `issue_orders` call made with
    the clock at 0600 or 1200 (its answer's `for_pulse` set), counting its
    results with status `queued`, and those refused.

Prints one line a game and the figures of the arm: games over and not,
games with a strike at ships, the median margin with the least, the most and
the quartiles, and the orders inside a day (median, most). With --out it
writes the lines as JSON. Python 3.9 or later, standard library only. It is
the umpire's: nothing that plays the game reads it or what it prints.
"""
import argparse
import json
import os
import statistics
import subprocess
import sys
from decimal import Decimal

REPO = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))


def run(args):
    return subprocess.run(args, cwd=REPO, capture_output=True, text=True)


def quartiles(values):
    values = sorted(values)
    if len(values) < 2:
        return values[0], values[0]
    q = statistics.quantiles(values, n=4, method='inclusive')
    return q[0], q[2]


def read_game(sim_cli, folder, name):
    game = os.path.join(folder, 'games', name)
    stem = os.path.join(folder, 'games', name[:-4])
    save, transcript, export = stem + '.save.json', stem + '.transcript.jsonl', stem + '.export.json'
    row = {'game': name, 'over': False}
    r = run([sim_cli, 'play', '--game', game, '--export-save', save, '--export-transcript', transcript])
    if r.returncode != 0:
        row['why_not'] = (r.stderr or r.stdout).strip()
        r2 = run([sim_cli, 'play', '--game', game, '--export-transcript', transcript])
        row['transcript_written'] = r2.returncode == 0
        return row
    row['over'] = True
    r = run([sim_cli, 'export', save, '--out', export])
    if r.returncode != 0:
        row['export_error'] = r.stderr.strip()
        return row
    with open(export) as f:
        ex = json.load(f)
    allied_groups = {g['id'] for g in ex['initial']['air_groups'] if g['side'] == 'allied'}
    launches = [e for e in ex['events'] if e['kind'] == 'strike_launched'
                and e['data'].get('air_group') in allied_groups]
    row['strike_at_ships'] = bool(launches)
    row['strike_launches'] = len(launches)
    row['first_strike_pulse'] = launches[0]['pulse'] if launches else None
    with open(save) as f:
        sv = json.load(f)
    row['orders_inside_saved'] = sum(1 for o in sv['orders'] if o.get('side') == 'allied' and 'pulse' in o)
    row['orders_inside_saved_list'] = [
        {'pulse': o['pulse'], 'unit': o['unit'], 'order': o['order']}
        for o in sv['orders'] if o.get('side') == 'allied' and 'pulse' in o]
    margin = None
    queued_inside = refused_inside = 0
    tools = []
    with open(transcript) as f:
        for line in f:
            entry = json.loads(line)
            tools.append(entry['tool'])
            answer = entry.get('answer') or {}
            if isinstance(answer, dict) and answer.get('result') and isinstance(answer['result'], dict) \
                    and 'allied' in answer['result']:
                margin = answer['result']['allied']['margin']
            if entry['tool'] == 'issue_orders' and isinstance(answer, dict) and answer.get('for_pulse') in ('morning', 'afternoon'):
                for res in answer.get('results', []):
                    if res.get('status') == 'queued':
                        queued_inside += 1
                    elif res.get('status') == 'refused':
                        refused_inside += 1
    row['margin'] = margin
    row['orders_inside_queued'] = queued_inside
    row['orders_inside_refused'] = refused_inside
    row['calls'] = len(tools)
    row['tools_used'] = sorted(set(tools))
    return row


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('folder')
    parser.add_argument('--sim-cli', default=os.path.join(REPO, 'target', 'release', 'sim-cli'))
    parser.add_argument('--out')
    args = parser.parse_args()
    folder = os.path.abspath(args.folder)
    names = sorted((n for n in os.listdir(os.path.join(folder, 'games'))
                    if n.startswith('g') and n.endswith('.txt')),
                   key=lambda n: int(n[1:-4]))
    rows = [read_game(args.sim_cli, folder, n) for n in names]
    for row in rows:
        if row['over']:
            print(f"{row['game']}: over; strike at ships {row['strike_at_ships']} "
                  f"({row['strike_launches']} launches, first {row['first_strike_pulse']}); "
                  f"margin {row['margin']}; orders inside the day saved {row['orders_inside_saved']}, "
                  f"queued {row['orders_inside_queued']}, refused {row['orders_inside_refused']}; calls {row['calls']}")
        else:
            print(f"{row['game']}: NOT OVER: {row.get('why_not')}")
    over = [r for r in rows if r['over']]
    print(f"games: {len(rows)}; over {len(over)}; not over {len(rows) - len(over)}")
    if over:
        margins = [Decimal(r['margin']) for r in over if r['margin'] is not None]
        print(f"strike at ships: {sum(r['strike_at_ships'] for r in over)} of {len(over)}")
        if margins:
            q1, q3 = quartiles(margins)
            print(f"Allied margin: median {statistics.median(margins)}, least {min(margins)}, most {max(margins)}, "
                  f"quartiles {q1} and {q3} (of {len(margins)})")
        inside = [r['orders_inside_saved'] for r in over]
        print(f"orders inside the day (saved, with a pulse): median {statistics.median(inside)}, most {max(inside)}, "
              f"games with any {sum(1 for n in inside if n)}")
    if args.out:
        with open(args.out, 'w') as f:
            json.dump(rows, f, indent=1, default=str)
    return 0


if __name__ == '__main__':
    sys.exit(main())

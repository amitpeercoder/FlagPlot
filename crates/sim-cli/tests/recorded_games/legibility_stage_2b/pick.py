"""Condition 5 (decision 0138, item 12): pick the twelve cases and give each
reader only the tools' answers the item names.

python3.12 pick.py C4.jsonl SCENARIO OUTDIR

The rule, written before the pick: the first case, by the report's event
number, on each of the first twelve seeds in order that have one (seeds read
from c4.py's lines in seed order). Among cases of one seed on the same report,
the first by A's launch event, then by B's name. A's group is the group of
that launch; B's group is the lowest-numbered group meeting clause 4.

Each reader is given three answers of the tools, in plain text, as the tools
print them (no --json):
  1. for A: `events SCENARIO --seed N --strike air_group:A --day D --pulse PA
     --report R`  (why its group launched; PA the launch's pulse)
  2. for B: `events SCENARIO --seed N --strike air_group:B --day D --pulse P
     --report R`  (why its group did not launch in pulse P)
  3. for B: `events SCENARIO --seed N --report R --command B`  (when the report
     reached B)
The scenario line each prints at its head ("Scenario 5: ..., seed N: ...") is
kept: it names the game and nothing more.
"""
import json
import os
import subprocess
import sys

W = '/Users/amitpeer/Documents/FlagPilot/.claude/worktrees/stage2b-proof'
CLI = f'{W}/target/release/sim-cli'


def run(*a):
    p = subprocess.run([CLI, 'events', *a], cwd=W, capture_output=True, text=True, check=True)
    return p.stdout.rstrip('\n')


def main():
    c4, scen, outdir = sys.argv[1:4]
    rows = sorted((json.loads(l) for l in open(c4)), key=lambda r: r['seed'])
    picked = []
    for r in rows:
        if not r.get('cases'):
            continue
        c = sorted(r['cases'], key=lambda c: (c['report'], c['A_launch'], c['B']))[0]
        picked.append((r['seed'], c))
        if len(picked) == 12:
            break
    cases = []
    for i, (seed, c) in enumerate(picked, 1):
        bg = min(g['group'] for g in c['B_groups'])
        d = c['report_day']
        a1 = run(scen, '--seed', str(seed), '--strike', f"air_group:{c['A_group']}", '--day', str(d),
                 '--pulse', c['A_launch_pulse'], '--report', str(c['report']))
        a2 = run(scen, '--seed', str(seed), '--strike', f'air_group:{bg}', '--day', str(d),
                 '--pulse', c['report_pulse'], '--report', str(c['report']))
        a3 = run(scen, '--seed', str(seed), '--report', str(c['report']), '--command', c['B'])
        two = sorted([c['A'], c['B']])
        prompt = (
            f"Below are three answers from the umpire's tools of a wargame, about one sighting report, "
            f"report #{c['report']}, made in the {c['report_pulse']} of day {d}, and two commands of the "
            f"{c['side'].capitalize()} side: {two[0]} and {two[1]}. Read them and answer from them alone.\n\n"
            f"ANSWER 1\n$ events --strike air_group:{c['A_group']} --day {d} --pulse {c['A_launch_pulse']} "
            f"--report {c['report']}\n{a1}\n\n"
            f"ANSWER 2\n$ events --strike air_group:{bg} --day {d} --pulse {c['report_pulse']} "
            f"--report {c['report']}\n{a2}\n\n"
            f"ANSWER 3\n$ events --report {c['report']} --command {c['B']}\n{a3}\n\n"
            f"QUESTIONS\n"
            f"1. Which of the two commands, {two[0]} or {two[1]}, held the sighting (report #{c['report']}) "
            f"in the {c['report_pulse']} of day {d}?\n"
            f"2. When, and by which road, did the other command come to hold it, or did it never?\n"
            f"3. Why did the other command not strike at it in the {c['report_pulse']} of day {d}?\n"
            f"Answer each in one or two sentences, numbered 1 to 3.")
        cases.append({'case': i, 'seed': seed, 'side': c['side'], 'report': c['report'], 'day': d,
                      'pulse': c['report_pulse'], 'report_hex': c['report_hex'], 'A': c['A'],
                      'A_group': c['A_group'], 'A_launch': c['A_launch'],
                      'A_launch_pulse': c['A_launch_pulse'], 'B': c['B'], 'B_group': bg,
                      'B_first_held': c['B_first_held'],
                      'B_launched_elsewhere_in_P': c['B_launched_elsewhere_in_P'],
                      'answers': {'A_strike': a1, 'B_strike': a2, 'B_report': a3},
                      'prompt_text': prompt})
    os.makedirs(outdir, exist_ok=True)
    json.dump(cases, open(os.path.join(outdir, 'cases.json'), 'w'), indent=1)
    for c in cases:
        open(os.path.join(outdir, f"prompt_{c['case']:02d}.txt"), 'w').write(c['prompt_text'])
    print([(c['case'], c['seed'], c['report'], c['A'], c['B']) for c in cases])


if __name__ == '__main__':
    main()

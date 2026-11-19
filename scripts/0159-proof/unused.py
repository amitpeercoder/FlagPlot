"""Condition 2 of decision 0159's proof, "Nothing changed where it is unused"
(decision 0159, "The proof: how each condition is read", 2). It prints what
it finds and never a verdict: the mark is read by the architect.

    python3 unused.py [--before REV] [--after REV] [--sim SIM_CLI]
                      [--repo DIR] [--list]

For each scenario of `data/scenarios/*.toml` at AFTER "with no hold and no
take", it sets the scenario's two generated run files
(`crates/sim-cli/tests/run_baseline/NAME.txt` and `NAME.events.txt`) as they
were at BEFORE beside those at AFTER, with `git show`, and prints every line
that differs and is not a hash line.

BEFORE is the parent of the merge of pull request 446 (decision 0159's step 1),
found from the merge's message ("Merge pull request #446"), its first
parent: main before step 1. AFTER is HEAD, the run's commit. Each may be given
as any git revision.

A scenario is "with no hold and no take" where no intent of the kind hold,
take a place or control an island is in force on any day of its game,
whoever made it. An intent comes into force only by a command taken in: the
commander's `set_intent` or the staff's `form_intent` (no other issuer sets
an intent; `sim-core`'s intake records `CommandTaken` for each). So the
scenario's game at AFTER is read from its `run --events` file at AFTER: it
has such an intent where a line of the event kind `command_taken` (worded
"... command taken: ...") sets or forms an intent "to hold base:", "to take
the place at" or "to control island" (the words `sim-core` writes for the
kinds `Hold`, `TakePlace` and `ControlIsland`, `command.rs`,
`write_intent_kind`). A refused command puts nothing in force and is worded
"command refused", so it is not read. The two soaks (`08_soak`,
`45_soak_ashore`) keep no events file: their events are made with `--sim`
(`sim-cli run NAME --events` from the top of the repository, which must be
at AFTER). The same reading of BEFORE's events files is printed beside it as
a check; a scenario on which the two disagree is named.

A hash line is a line of the form `day N  hash HHHHHHHHHHHHHHHH  N events`
(regular expression `^day +[0-9]+ +hash [0-9a-f]{16} +[0-9]+ events$`): the
only lines of the run files that carry a hash. A differing line pair in which
both lines are hash lines is not printed among the failures; where such a pair
differs in more than its hash (its count of events), it is listed apart, for
the architect to read, since the line still carries a hash.

Lines are paired by `difflib` on the files with each hash cut to `-`, so a
hash line is set beside the hash line of the same day.
"""
import argparse
import difflib
import os
import re
import subprocess
import sys

BASELINES = 'crates/sim-cli/tests/run_baseline'
SCENARIOS = 'data/scenarios'
NO_EVENTS = ('08_soak', '45_soak_ashore')
HASH_LINE = re.compile(r'^day +[0-9]+ +hash [0-9a-f]{16} +[0-9]+ events$')
HASH = re.compile(r'(^day +[0-9]+ +hash )[0-9a-f]{16}')
TAKEN_INTENT = re.compile(r'command taken: (set intent:\d+|form under intent:\d+ an intent) '
                          r'to (hold base:|take the place at|control island )')


def git(repo, *args):
    return subprocess.run(['git', '-C', repo, *args], check=True, capture_output=True, text=True).stdout


def show(repo, rev, path):
    """The file at rev, or None where it is not there."""
    r = subprocess.run(['git', '-C', repo, 'show', f'{rev}:{path}'], capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else None


def merge_of_446_parent(repo):
    out = git(repo, 'log', '--merges', '--format=%H %P %s', 'HEAD')
    for line in out.splitlines():
        parts = line.split(' ', 3)
        if len(parts) == 4 and parts[3].startswith('Merge pull request #446 '):
            return parts[1], parts[0]
    raise SystemExit('no merge of pull request #446 in the history of HEAD')


def scenarios_at(repo, rev):
    out = git(repo, 'ls-tree', '--name-only', f'{rev}:{SCENARIOS}')
    return sorted(n[:-5] for n in out.splitlines() if n.endswith('.toml'))


def has_hold_or_take(events_text):
    found = [line for line in events_text.splitlines() if TAKEN_INTENT.search(line)]
    return found


def cut(text):
    return [HASH.sub(r'\1-', line) for line in text.splitlines()]


def compare(before, after):
    """(other lines differing, hash lines differing beyond the hash): each a
    list of ('-'|'+', line)."""
    b, a = before.splitlines(), after.splitlines()
    sm = difflib.SequenceMatcher(None, cut(before), cut(after), autojunk=False)
    other, hash_beyond = [], []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal':
            continue
        old, new = b[i1:i2], a[j1:j2]
        # A hash line is set beside the hash line of the same day in the
        # block; such a pair differs beyond its hash only in its count of
        # events (a change of the hash alone cuts equal).
        day_of = lambda line: line.split()[1]  # noqa: E731
        new_hash = {day_of(y): y for y in new if HASH_LINE.match(y)}
        paired = set()
        for x in old:
            if HASH_LINE.match(x) and day_of(x) in new_hash:
                paired.add(day_of(x))
                hash_beyond.append(('-', x))
                hash_beyond.append(('+', new_hash[day_of(x)]))
            else:
                other.append(('-', x))
        for y in new:
            if not (HASH_LINE.match(y) and day_of(y) in paired):
                other.append(('+', y))
    return other, hash_beyond


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--before')
    ap.add_argument('--after', default='HEAD')
    ap.add_argument('--sim', help='sim-cli built from AFTER, for the two soaks')
    ap.add_argument('--repo', default=os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    ap.add_argument('--list', action='store_true', help='print the lines that put a hold or a take in force')
    a = ap.parse_args()
    repo = a.repo
    if a.before:
        before = git(repo, 'rev-parse', a.before).strip()
        merge = None
    else:
        before, merge = merge_of_446_parent(repo)
    after = git(repo, 'rev-parse', a.after).strip()
    head = git(repo, 'rev-parse', 'HEAD').strip()
    print(f'before: {before}' + (f' (first parent of {merge}, the merge of pull request 446)' if merge else ''))
    print(f'after:  {after}')
    names = scenarios_at(repo, after)
    print(f'scenarios of {SCENARIOS}/*.toml at after: {len(names)}')
    print()

    unused, used, disagree = [], [], []
    for name in names:
        ev_path = f'{BASELINES}/{name}.events.txt'
        if name in NO_EVENTS:
            if not a.sim:
                raise SystemExit(f'{name} keeps no events file: give --sim (sim-cli built from after)')
            if after != head:
                raise SystemExit(f'{name}: --sim runs the scenario as checked out, which is not after')
            ev_after = subprocess.run([a.sim, 'run', f'{SCENARIOS}/{name}.toml', '--events'], cwd=repo,
                                      check=True, capture_output=True, text=True).stdout
            ev_before = None
        else:
            ev_after = show(repo, after, ev_path)
            if ev_after is None:
                raise SystemExit(f'{name}: no {ev_path} at after')
            ev_before = show(repo, before, ev_path)
        found = has_hold_or_take(ev_after)
        (used if found else unused).append(name)
        if ev_before is not None and bool(has_hold_or_take(ev_before)) != bool(found):
            disagree.append(name)
        if a.list and found:
            print(f'{name}: {len(found)} command(s) putting a hold, a take or an island in force, the first:')
            print(f'    {found[0].strip()}')
    print(f'with a hold, a take or an island in force ({len(used)}): {", ".join(used)}')
    print()
    print(f'with no hold and no take ({len(unused)}): {", ".join(unused)}')
    if disagree:
        print(f'NOTE: read from before\'s events files the reading differs for: {", ".join(disagree)}')
    else:
        print('read from before\'s events files the reading is the same (the soaks are read at after only)')
    print()

    failures = 0
    for name in unused:
        for suffix in ('.txt', '.events.txt'):
            path = f'{BASELINES}/{name}{suffix}'
            old, new = show(repo, before, path), show(repo, after, path)
            if old is None and new is None:
                continue
            if old is None or new is None:
                print(f'{path}: {"not at before" if old is None else "not at after"}')
                failures += 1
                continue
            other, hash_beyond = compare(old, new)
            if hash_beyond:
                print(f'{path}: hash lines differing beyond the hash ({len(hash_beyond) // 2}), not counted:')
                for sign, line in hash_beyond:
                    print(f'  {sign} {line}')
            if other:
                failures += len(other)
                print(f'{path}: lines differing that are not hash lines ({len(other)}):')
                for sign, line in other:
                    print(f'  {sign} {line}')
    print()
    print(f'run files read: {sum(1 for n in unused for s in (".txt", ".events.txt") if show(repo, after, f"{BASELINES}/{n}{s}") is not None)}; '
          f'lines differing that are not hash lines, or files missing: {failures}')
    return 0


if __name__ == '__main__':
    sys.exit(main())

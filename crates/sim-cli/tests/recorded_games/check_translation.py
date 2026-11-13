#!/usr/bin/env python3
"""Checks that a recorded game's transcript differs from its old one only in handles.

Decision 0086 numbers a side's own ships, task forces and air groups from 1.
After `translate_handles.py` and `regame.py` a transcript is made again; this
checks it against the transcript it replaces.

Usage (from the repository root):

    python3 crates/sim-cli/tests/recorded_games/check_translation.py \\
        OLD_TRANSCRIPT NEW_TRANSCRIPT HANDLES_0086_JSON

Each line of the two transcripts is read as JSON and the two are walked
together. A `ship:N`, `task_force:N` or `air_group:N` token (in text, or in the
list of a `detach` order) is taken out of each side and the two skeletons must
be equal: every other word, number, key and place of the answer is the same.
The pairs of tokens found (old, new) must then agree with the table in
`handles_0086.json`: an old number in the table is the new one the table gives,
and a unit made in play (not in the table) has a new number above all the
table's, in the order of the old numbers. Each old number pairs with one new
number and the reverse, over the whole transcript. Exit status 0 and the line
`OK` if nothing else differs; otherwise the first differences are printed.
"""

import json
import re
import sys

TOKEN = re.compile(r"(?<![A-Za-z0-9_])(ship|task_force|air_group):(\d+)(?!\d)")


def skeleton_pairs(old, new, pairs, path, problems):
    """Walks two values; records token pairs; notes any other difference."""
    if isinstance(old, str) and isinstance(new, str):
        a, b = TOKEN.sub(lambda m: "<%s>" % m.group(1), old), TOKEN.sub(
            lambda m: "<%s>" % m.group(1), new
        )
        if a != b:
            problems.append("%s: text differs: %r vs %r" % (path, old[:120], new[:120]))
            return
        ta, tb = TOKEN.findall(old), TOKEN.findall(new)
        for (ka, na), (kb, nb) in zip(ta, tb):
            pairs.add((ka, int(na), int(nb)))
        return
    if isinstance(old, dict) and isinstance(new, dict):
        if list(old.keys()) != list(new.keys()):
            problems.append("%s: keys differ" % path)
            return
        for key in old:
            if (
                key == "ships"
                and old.get("kind") == "detach"
                and isinstance(old[key], list)
                and isinstance(new[key], list)
                and len(old[key]) == len(new[key])
            ):
                for a, b in zip(old[key], new[key]):
                    pairs.add(("ship", a, b))
            else:
                skeleton_pairs(old[key], new[key], pairs, path + "." + key, problems)
        return
    if isinstance(old, list) and isinstance(new, list):
        if len(old) != len(new):
            problems.append("%s: lengths differ" % path)
            return
        for i, (a, b) in enumerate(zip(old, new)):
            skeleton_pairs(a, b, pairs, "%s[%d]" % (path, i), problems)
        return
    if old != new or type(old) != type(new):
        problems.append("%s: %r vs %r" % (path, old, new))


def main():
    old_file, new_file, table_file = sys.argv[1:4]
    old_lines = open(old_file).read().splitlines()
    new_lines = open(new_file).read().splitlines()
    table = json.load(open(table_file))["old_to_new"]
    problems = []
    pairs = set()
    if len(old_lines) != len(new_lines):
        problems.append("the transcripts have %d and %d lines" % (len(old_lines), len(new_lines)))
    for i, (a, b) in enumerate(zip(old_lines, new_lines)):
        skeleton_pairs(json.loads(a), json.loads(b), pairs, "line %d" % (i + 1), problems)
    for kind in ("ship", "task_force", "air_group"):
        own = {int(o): n for o, n in table[kind].items()}
        found = sorted((o, n) for k, o, n in pairs if k == kind)
        old_to_new, new_to_old = {}, {}
        for o, n in found:
            if old_to_new.setdefault(o, n) != n or new_to_old.setdefault(n, o) != o:
                problems.append("%s: %d and %d are not one to one" % (kind, o, n))
            if o in own:
                if own[o] != n:
                    problems.append("%s:%d became %d, the table says %d" % (kind, o, n, own[o]))
            elif n <= len(own):
                problems.append("%s:%d (made in play?) became %d, a number of the table" % (kind, o, n))
        made = [(o, n) for o, n in found if o not in own]
        if [n for _, n in made] != sorted(n for _, n in made):
            problems.append("%s: units made in play are not numbered in order: %r" % (kind, made))
        print(
            "%s: %d token pairs, %d own, %d made in play"
            % (kind, len(found), len(found) - len(made), len(made))
        )
    if problems:
        print("DIFFERENCES OTHER THAN HANDLES (first 20 of %d):" % len(problems))
        for p in problems[:20]:
            print("  " + p)
        sys.exit(1)
    print("OK: %d lines, differing in handles only" % len(old_lines))


if __name__ == "__main__":
    main()

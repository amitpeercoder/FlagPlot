#!/usr/bin/env python3
"""Translates a recorded game's requests to the numbering of decision 0086.

Before decision 0086 a side saw its own ships, task forces and air groups by
the scenario's numbers. Now each side sees its own units numbered 1, 2, 3...
among its own units of that kind, ranked by the scenario's id.

Usage (from the repository root):

    python3 crates/sim-cli/tests/recorded_games/translate_handles.py GAME_DIR

It reads the first line of GAME_DIR/game.jsonl (the scenario and the seat),
reads the scenario's [[ship]], [[task_force]] and [[air_group]] tables, works
out each of the seat's own units' new number, writes the table (old to new, by
kind) to GAME_DIR/handles_0086.json, and rewrites GAME_DIR/game.jsonl. Only
`ship:N`, `task_force:N` and `air_group:N` tokens of the seat's own units
change, and the bare ship numbers of a `detach` order. A token of the
other side's unit, or of a number that is nobody's, is moved above the seat's
own numbers so that it still names no unit of the seat; a token that names
a unit that the scenario file does not hold (one made in play) stops the run.
Run it once on a game: the table is of the old numbers and the file then holds
the new ones, so a second run stops at the table that already exists.
"""

import json
import os
import re
import sys

KINDS = ("ship", "task_force", "air_group")
TOKEN = re.compile(r"(?<![A-Za-z0-9_])(ship|task_force|air_group):(\d+)(?!\d)")


def scenario_units(path):
    """{kind: {id: side}} from the [[ship]], [[task_force]], [[air_group]] tables."""
    units = {k: {} for k in KINDS}
    current = None
    pending = []
    for raw in open(path).read().splitlines():
        line = raw.split("#", 1)[0].strip()
        if not line:
            continue
        if line.startswith("["):
            name = line.strip("[]").strip()
            current = {} if line.startswith("[[") and name in KINDS else None
            kind = name if current is not None else None
            if current is not None:
                current["_kind"] = kind
                pending.append(current)
            continue
        if current is not None and "=" in line:
            key, value = [s.strip() for s in line.split("=", 1)]
            if key in ("id", "side"):
                current[key] = value.strip('"')
    for table in pending:
        units[table["_kind"]][int(table["id"])] = table["side"]
    return units


def tables_for(units, side):
    """{kind: {old id: new number}} for the side's own units."""
    out = {}
    for kind in KINDS:
        ids = sorted(i for i, s in units[kind].items() if s == side)
        out[kind] = {old: new + 1 for new, old in enumerate(ids)}
    return out


def mapper(tables, units_all):
    def map_token(kind, old):
        if old in tables[kind]:
            return tables[kind][old]
        if old in units_all[kind]:
            # The other side's unit: stays no unit of the seat.
            return len(tables[kind]) + old
        raise SystemExit(
            "stop: %s:%d is in no scenario table (a unit made in play?)" % (kind, old)
        )

    return map_token


def translate_text(text, map_token):
    def swap(m):
        return "%s:%d" % (m.group(1), map_token(m.group(1), int(m.group(2))))

    return TOKEN.sub(swap, text)


def translate_value(value, map_token, own_ships):
    if isinstance(value, str):
        return translate_text(value, map_token)
    if isinstance(value, list):
        return [translate_value(v, map_token, own_ships) for v in value]
    if isinstance(value, dict):
        out = {}
        for key, v in value.items():
            if key == "ships" and value.get("kind") == "detach" and isinstance(v, list):
                out[key] = [
                    map_token("ship", n) if isinstance(n, int) else n for n in v
                ]
            else:
                out[key] = translate_value(v, map_token, own_ships)
        return out
    return value


def dump(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def main():
    game_dir = sys.argv[1].rstrip("/")
    game = os.path.join(game_dir, "game.jsonl")
    table_file = os.path.join(game_dir, "handles_0086.json")
    if os.path.exists(table_file):
        raise SystemExit("stop: %s exists; this game is translated" % table_file)
    lines = open(game).read().splitlines()
    head = json.loads(lines[0])
    units = scenario_units(head["scenario"])
    tables = tables_for(units, head["seat"])
    map_token = mapper(tables, units)
    out = [lines[0]]
    changed = 0
    for line in lines[1:]:
        if not line.strip():
            out.append(line)
            continue
        value = json.loads(line)
        assert dump(value) == line, "the line is not in the written form: " + line[:60]
        new = dump(translate_value(value, map_token, tables["ship"]))
        changed += new != line
        out.append(new)
    identity = all(old == new for t in tables.values() for old, new in t.items())
    with open(table_file, "w") as f:
        json.dump(
            {
                "decision": "0086",
                "scenario": head["scenario"],
                "seat": head["seat"],
                "old_to_new": {k: {str(o): n for o, n in t.items()} for k, t in tables.items()},
            },
            f,
            indent=1,
            sort_keys=True,
        )
        f.write("\n")
    if not identity or changed:
        open(game, "w").write("\n".join(out) + "\n")
    print(
        "%s: %s, %d request lines changed" % (game_dir, "identity" if identity else "renumbered", changed)
    )


if __name__ == "__main__":
    main()

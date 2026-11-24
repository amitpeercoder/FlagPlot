#!/usr/bin/env python3
"""Makes what the browser's site holds that is not the module or its data
(decision 0229, stage 4, rulings 14 and 16): the page, the list of games, the
chart of each map and the directives. Called by `scripts/web/build.sh` with the
folder of the site:

    python3 -I scripts/web/site.py target/web/site

It reads the table's files with the bridge's own readers (`table/serve.py`:
`load_scenarios`, `load_chart` and `read_directives`), so there is one reader of
each. Of the page it changes one line, the marked one, which becomes the script
tag of `door.js`; nothing else. Nothing it reads is the simulation's data: the
maps are known to both sides (decision 0154) and the simulation reads none of
the files made here.
"""

import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
MARK = "<!--@door-->"
TAG = '<script src="door.js"></script>'


def bridge():
    """The bridge's module, read as a library: it holds the table's readers."""
    spec = importlib.util.spec_from_file_location("table_serve", os.path.join(ROOT, "table", "serve.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def write(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as handle:
        json.dump(value, handle, separators=(",", ":"))


def main(out):
    serve = bridge()
    # The list: the games a person plays against the engine, which is all the build offers.
    listed = []
    for entry in serve.load_scenarios(os.path.join(ROOT, "table", "scenarios.json")):
        pairings = [
            p for p in entry["pairings"] if sorted((p["allied"], p["japanese"])) == ["engine", "person"]
        ]
        if pairings:
            listed.append(dict(entry, pairings=pairings))
    write(os.path.join(out, "scenarios.json"), {"scenarios": listed})
    # The chart of each map that a game offered lies on, in one file named for the map.
    for entry in listed:
        map_file = entry.get("map", serve.DEFAULT_MAP)
        name = os.path.splitext(os.path.basename(map_file))[0]
        chart = serve.load_chart(os.path.join(ROOT, map_file))
        write(
            os.path.join(out, "chart", name + ".json"),
            {"hexes": chart["hexes"], "frame": chart["frame"], "coast": chart["coast"], "names": chart["names"]},
        )
        for file in entry.get("directives", {}).values():
            write(
                os.path.join(out, "directives", os.path.basename(file)),
                serve.read_directives(os.path.join(ROOT, file)),
            )
    # The page: the table's own, with its one marked line made the script tag of the door.
    with open(os.path.join(ROOT, "table", "index.html")) as handle:
        lines = handle.read().split("\n")
    if lines.count(MARK) != 1:
        sys.exit(f"table/index.html must hold the line {MARK} exactly once")
    lines[lines.index(MARK)] = TAG
    with open(os.path.join(out, "index.html"), "w") as handle:
        handle.write("\n".join(lines))


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("usage: site.py SITE_FOLDER")
    main(sys.argv[1])

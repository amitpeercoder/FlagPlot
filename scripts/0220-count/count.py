#!/usr/bin/env python3
"""The count owed by decision 0220 ("The count, as stated"), before its build.

The umpire's: it reads `sim-cli export` files, which show the truth. Nothing
that plays the game reads it or what it prints. It tunes nothing and gives no
verdict. Python 3.9 or later, standard library only.

  count.py pairs EXPORT.json [--label L] [--seed N]   one JSON line a pair
  count.py sum PAIRS.jsonl...                         the counts, by scenario
                                                      and side, and summed

A sighting is a `report_received` event whose cause is a `search_attempt`,
taken with the task force that attempt names. For each (side, enemy task
force) the sightings are put in order; two in one pulse are one, the first.
Each sighting after the first is paired with the one before. For a pair:
  refreshed or new     the later report's `created` flag;
  old contact held     for a new contact only: the contact the earlier
                       sighting made or refreshed has no `contact_dropped`
                       or `contact_not_there` event on the later sighting's
                       day or before it (held at the dusk that ends that day);
  R4's distance        hours from the start of the earlier pulse to the end of
                       the later one, at 20 knots (40 nm a hex), whole hexes
                       cut, against the hex distance of the two reports'
                       hexes; equal is within;
  knots it would take  hex distance * 40 over the hours between the two
                       pulses' starts.
A pulse is night 0-12, morning 12-18, afternoon 18-24 of the day's hours.
"""
import json
import math
import sys
from collections import defaultdict

OFFSET = {"night": 0, "morning": 12, "afternoon": 18}
LENGTH = {"night": 12, "morning": 6, "afternoon": 6}
KNOTS = 20
HEX_NM = 40


def hex_distance(a, b):
    dq, dr = a[0] - b[0], a[1] - b[1]
    return (abs(dq) + abs(dr) + abs(dq + dr)) // 2


def start(day, pulse):
    return (day - 1) * 24 + OFFSET[pulse]


def pairs_of(export):
    events = export["events"]
    by_id = {e["id"]: e for e in events}
    gone = defaultdict(list)  # (side, contact) -> days it was dropped
    for e in events:
        if e["kind"] in ("contact_dropped", "contact_not_there"):
            gone[(e["data"]["side"], e["data"]["contact"])].append(e["day"])
    sightings = defaultdict(list)
    for e in events:
        if e["kind"] != "report_received":
            continue
        att = [by_id[c["event"]] for c in e["causes"] if c.get("event") in by_id]
        att = [a for a in att if a["kind"] == "search_attempt"]
        if not att:
            continue
        assert e["pulse"] in OFFSET, e
        d = e["data"]
        sightings[(d["side"], att[0]["data"]["task_force"])].append(
            dict(day=e["day"], pulse=e["pulse"], hex=d["hex"], created=d["created"],
                 contact=d["contact"], side=d["side"]))
    out = []
    for (side, tf), seq in sorted(sightings.items()):
        kept = []
        for s in seq:
            if kept and (kept[-1]["day"], kept[-1]["pulse"]) == (s["day"], s["pulse"]):
                continue
            kept.append(s)
        for old, new in zip(kept, kept[1:]):
            hours = start(new["day"], new["pulse"]) + LENGTH[new["pulse"]] - start(old["day"], old["pulse"])
            reach = hours * KNOTS // HEX_NM
            dist = hex_distance(old["hex"], new["hex"])
            gap = start(new["day"], new["pulse"]) - start(old["day"], old["pulse"])
            row = dict(side=side, task_force=tf, days_between=new["day"] - old["day"],
                       new_contact=new["created"], dist=dist, reach=reach,
                       within=dist <= reach, knots=dist * HEX_NM / gap)
            if new["created"]:
                row["old_held"] = not any(
                    day <= new["day"] for day in gone.get((side, old["contact"]), []))
            out.append(row)
    return out


def cmd_pairs(argv):
    path, label, seed = argv[0], "", 0
    if "--label" in argv:
        label = argv[argv.index("--label") + 1]
    if "--seed" in argv:
        seed = int(argv[argv.index("--seed") + 1])
    with open(path) as f:
        rows = pairs_of(json.load(f))
    for r in rows:
        r.update(label=label, seed=seed)
        print(json.dumps(r))


def bucket(days):
    return "same day" if days == 0 else ("next day" if days == 1 else "2+ days")


def tally(rows):
    n = len(rows)
    new = [r for r in rows if r["new_contact"]]
    kn = sorted(r["knots"] for r in rows)
    p90 = kn[math.ceil(0.9 * n) - 1] if n else None
    return dict(later=n, refreshed=n - len(new), new=len(new),
                new_held=sum(1 for r in new if r["old_held"]),
                within=sum(1 for r in rows if r["within"]),
                worst=(kn[-1] if n else None), p90=p90)


def show(name, rows):
    t = tally(rows)
    share = f"{t['within']}/{t['later']} ({100 * t['within'] / t['later']:.1f}%)" if t["later"] else "-"
    w = f"{t['worst']:.2f}" if t["worst"] is not None else "-"
    p = f"{t['p90']:.2f}" if t["p90"] is not None else "-"
    print(f"{name:<44} later {t['later']:>5}  refreshed {t['refreshed']:>5}  new {t['new']:>5}  "
          f"old held {t['new_held']:>5}/{t['new']:<5}  within {share:<16} worst kn {w:>8}  p90 kn {p:>8}")


def cmd_sum(argv):
    rows = []
    for p in argv:
        with open(p) as f:
            rows += [json.loads(line) for line in f if line.strip()]
    groups = defaultdict(list)
    for r in rows:
        groups[(r["label"], r["side"])].append(r)
    for key in sorted(groups):
        show(f"{key[0]} / {key[1]}", groups[key])
    for side in ("allied", "japanese"):
        show(f"ALL / {side}", [r for r in rows if r["side"] == side])
    show("ALL / both sides", rows)
    print()
    for b in ("same day", "next day", "2+ days"):
        sel = [r for r in rows if bucket(r["days_between"]) == b]
        show(f"by gap: {b}", sel)
    print()
    for label in sorted({r["label"] for r in rows}):
        for b in ("same day", "next day", "2+ days"):
            sel = [r for r in rows if r["label"] == label and bucket(r["days_between"]) == b]
            if sel:
                show(f"{label} gap {b}", sel)


if __name__ == "__main__":
    {"pairs": cmd_pairs, "sum": cmd_sum}[sys.argv[1]](sys.argv[2:])

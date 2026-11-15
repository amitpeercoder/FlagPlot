"""Pick the 24 reasons of ruling 39 from the exports of 36, 40a, 41a and 42b.

The rule, written before the pick: every StaffReason event of the Allied side
in the four exports is classed by the first of these that fits:
  displaced  its words name "displaced by"
  hold       its words name a hold or an override ("held", "the hold", "overrid...")
  leave      its words turn on a leave ("leave to prepare" or "leave to launch")
  order      it is the reason of an order
  no_order   it is a decision to give no order (no order and no command cited)
(a reason of a staff command that fits none of the first three is in no kind).
Within a kind the reasons are taken by day, lowest first, then by scenario in
the order 36, 40a, 41a, 42b, then by event number; a reason whose worked text
is the same as one already taken is skipped. Taken: 6 orders, 6 no_order, 4
displaced, 4 hold, 4 leave.
"""
import json, re, sys

files = sys.argv[1:]
order = ["36", "40a", "41a", "42b"]
rows = []
for label, path in zip(order, files):
    replay = json.load(open(path))
    events = replay["events"]
    by_id = {e["id"]: e for e in events}
    for e in events:
        if e["kind"] != "staff_reason":
            continue
        d = e["data"]
        if d.get("side") != "allied":
            continue
        words = " ".join(str(d.get(k, "")) for k in ("evidence", "binding", "worked", "would_change"))
        cmd = any(by_id.get(c.get("event"), {}).get("kind") in ("command_taken", "command_refused") for c in e.get("causes", []) if isinstance(c, dict))
        if "displaced by" in words:
            kind = "displaced"
        elif re.search(r"\bheld\b|\bthe hold\b|\boverrid", words):
            kind = "hold"
        elif "leave to prepare" in words or "leave to launch" in words:
            kind = "leave"
        elif "order" in d:
            kind = "order"
        elif not cmd:
            kind = "no_order"
        else:
            continue
        rows.append((e["day"], order.index(label), e["id"], label, kind, d))

want = {"order": 6, "no_order": 6, "displaced": 4, "hold": 4, "leave": 4}
taken = {k: [] for k in want}
seen = set()
for day, _, eid, label, kind, d in sorted(rows):
    if len(taken[kind]) >= want[kind] or d["worked"] in seen:
        continue
    seen.add(d["worked"])
    taken[kind].append((label, day, eid, d))

out = []
n = 0
for kind in ["order", "no_order", "displaced", "hold", "leave"]:
    for label, day, eid, d in taken[kind]:
        n += 1
        out.append({
            "number": n, "kind": kind, "scenario": label, "day": day, "event": eid,
            "serves": {k: d.get(k) for k in ("intent", "plan", "amendment") if d.get(k) is not None},
            "evidence": d.get("evidence", ""), "binding": d.get("binding", ""),
            "worked": d.get("worked", ""), "would_change": d.get("would_change", ""),
        })
print(json.dumps(out, indent=1))
print({k: len(v) for k, v in taken.items()}, file=sys.stderr)

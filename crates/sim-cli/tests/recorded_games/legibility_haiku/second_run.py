"""The four leave reasons of the second run (decision 0107, "As built (step 4)",
"The mend and the second run").

Run as: python3 pick.py <the four exports> > picked.json, then
python3 second_run.py picked.json <the four exports, in pick.py's order>.
pick.py's rule is run unchanged on the mended build's exports; this takes the
reasons it numbers 21 to 24 (the leave kind), adds to each its serves part as
the reason now gives it (the intent's statement, and the serves line exactly
as the log and the export word it, cut from the event's text), and writes the
text the model is given, in the first run's layout with that serves line.
"""
import json, re, sys

picked = json.load(open(sys.argv[1]))
order = ["36", "40a", "41a", "42b"]
events = {}
for label, path in zip(order, sys.argv[2:]):
    for e in json.load(open(path))["events"]:
        events[(label, e["id"])] = e

out = []
for row in picked:
    if row["kind"] != "leave":
        continue
    e = events[(row["scenario"], row["event"])]
    line = re.search(r"\[serves (.*?)\]: ", e["text"]).group(1)
    serves = dict(row["serves"])
    serves["statement"] = e["data"]["statement"]
    row = dict(row, serves=serves, serves_line=line)
    row["prompt_text"] = "\n".join([
        "Serves: " + line,
        "Evidence: " + (row["evidence"] or "(none)"),
        "Binding: " + (row["binding"] or "(none)"),
        "Worked: " + (row["worked"] or "(none)"),
        "Would change: " + (row["would_change"] or "(none)"),
    ])
    out.append(row)
assert [r["number"] for r in out] == [21, 22, 23, 24], [r["number"] for r in out]
print(json.dumps(out, indent=1))

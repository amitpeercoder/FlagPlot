#!/usr/bin/env python3
"""Times a dry run of a written plan on the table's 39 days, for the designer's
answer of Nov 27, 2026 to decision 0232 (the table's 39 days offer a dry run
once it is timed on his machine).

    cargo build --release -p sim-cli
    python3 scripts/time-table-dry-run.py [--sim-cli PATH] [--sizes small,standard,full] [--runs 3]

It reads data/scenarios/table-rear-bases/07b_slice_submarines.toml as it is (it
names data/reference/trial.toml and has island 3, "Guadalcanal and Tulagi", of
bases 1 and 4) and makes no copy. It takes the Allied seat through `sim-cli
mcp`, previews "Seize and hold Guadalcanal and Tulagi" with both landings
written (Tulagi with its transports, Lunga after it and not before day 4), and
for each size asks `trial` of that preview, which plays the draft and the
staff's plan beside it. Each size is asked in a process of its own, `--runs`
times, and the middle time is printed in seconds beside the mark: the time the
designer accepted for a trial beside another on the 39 days, with a tenth over
(docs/validation/stage-4-proof-setup.md, condition 10). It changes no file, and
nothing reads what it prints. It reads the clock, outside the simulation.
Standard library only. Run by hand on the designer's machine.
"""
import argparse
import json
import os
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SOURCE = "data/scenarios/table-rear-bases/07b_slice_submarines.toml"
MARKS = {"small": 2.2, "standard": 7.1, "full": 40.0}


class Seat:
    def __init__(self, cli, scenario):
        self.process = subprocess.Popen([cli, "mcp", scenario, "--seat", "allied"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, cwd=ROOT)
        self.number = 0
        self.request("initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "time-table-dry-run", "version": "0"}})
        self.process.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
        self.process.stdin.flush()

    def request(self, method, params):
        self.number += 1
        self.process.stdin.write(json.dumps({"jsonrpc": "2.0", "id": self.number, "method": method, "params": params}) + "\n")
        self.process.stdin.flush()
        return json.loads(self.process.stdout.readline())

    def call(self, tool, arguments):
        reply = self.request("tools/call", {"name": tool, "arguments": arguments})
        result = reply.get("result", {})
        text = "".join(part.get("text", "") for part in result.get("content", []))
        if "error" in reply or result.get("isError"):
            sys.exit(f"{tool} was refused: {reply.get('error', {}).get('message') or text}")
        return json.loads(text)

    def close(self):
        self.process.stdin.close()
        self.process.wait(60)


def one(cli, scenario, size):
    seat = Seat(cli, scenario)
    listed = seat.call("list_forces", {})
    force = [int(f["task_force"]["handle"].split(":")[1]) for f in listed["task_forces"] if f["task_force"]["name"] == "Placeholder transports, Tulagi"][0]
    bases = {b["base"]["name"]: int(b["base"]["handle"].split(":")[1]) for b in seat.call("list_bases", {})["enemy_bases"]}
    order = seat.call("preview", {"command": "set_intent", "intent_kind": "control_island", "island": 3})["intent"]
    shown = seat.call("preview", {"command": "set_intent", "intent": order, "intent_kind": "control_island", "island": 3, "under": [
        {"intent": order + 1, "intent_kind": "take_place", "base": bases["Tulagi"], "limits": [{"limit": "forces", "task_forces": [force], "ships": []}]},
        {"intent": order + 2, "intent_kind": "take_place", "base": bases["Lunga"], "limits": [{"limit": "not_before", "from_day": 4}, {"limit": "after", "other_intent": order + 1}]}]})
    began = time.monotonic()
    answer = seat.call("trial", {"preview": shown["preview"], "size": size})
    taken = time.monotonic() - began
    seat.close()
    if (answer.get("beside") or {}).get("of") != "left_to_the_staff":
        sys.exit("the trial did not play the staff's plan beside the draft")
    return taken, answer["asked"]["worlds"], answer["asked"]["days"]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--sim-cli", default="target/release/sim-cli")
    parser.add_argument("--sizes", default="small,standard,full")
    parser.add_argument("--runs", type=int, default=3)
    args = parser.parse_args()
    cli = os.path.abspath(args.sim_cli)
    if not os.path.exists(cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    text = open(os.path.join(ROOT, SOURCE)).read()
    if '\ntrial = "../../reference/trial.toml"\n' not in text or "\nnumber = 3\nname = \"Guadalcanal and Tulagi\"\nplaces = [{ base = 1 }, { base = 4 }]\n" not in text:
        sys.exit(f"{SOURCE} is not as this script reads it (a trial key, island 3 of bases 1 and 4)")
    for size in args.sizes.split(","):
        times, worlds, days = [], 0, 0
        for _ in range(args.runs):
            taken, worlds, days = one(cli, SOURCE, size)
            times.append(taken)
        middle = sorted(times)[len(times) // 2]
        print(f"{size}: {worlds} runs of {days} days, twice: {middle:.1f} s (the middle of {', '.join(f'{t:.1f}' for t in times)}); the mark is {MARKS[size]} s: {'within' if middle <= MARKS[size] else 'OVER'}", flush=True)


if __name__ == "__main__":
    main()

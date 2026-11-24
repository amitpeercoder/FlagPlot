#!/usr/bin/env python3
"""The table's bridge: one seat of one game, served to a browser on this machine.

It starts `sim-cli mcp SCENARIO --seat SIDE` and passes the page's tool calls
to it, one at a time. The page gets what the seat's tools answer and nothing
else: this file never reads a replay, a save, a batch or the event log, and it
does not read the scenario. It reads one data file, the map (for drawing the
chart and its coast), which both sides know.

Run from the top of the repository:

    python3 table/serve.py data/scenarios/stage-3-proof/07b_slice.toml \
        --seat allied --start 1942-08-07 \
        --island "1=Guadalcanal@1" --island "2=Tulagi@4"

then open http://127.0.0.1:8742/. Standard library only. It listens on this
machine alone. Decision 0154.

With `--watch` it holds a watched game instead (decision 0182, step 2): it
starts `sim-cli serve SCENARIO --watched`, each side named by `--seat` is an
agent's, and the other is the script's. Each agent has a door of its own and
the watcher has the page, each behind a key made at start:

    python3 table/serve.py data/scenarios/78_orders_inside_a_day.toml \
        --watch --seat allied --seat japanese

prints the watcher's address and one door for each agent's seat. A door
answers only its own seat's calls; nothing the watcher reads reaches it.

Started with no scenario it opens at the new game screen (decision 0185):

    python3 table/serve.py

prints the table's address, behind a key. On it a person chooses a scenario
from `table/scenarios.json`, a seat (a side, or to watch) and who plays each
other side (the engine or an agent), and begins. The bridge chooses how it
holds the game when it is begun: a person against the engine as without
`--watch`; a game watched as with it; and a person against an agent on
`sim-cli serve --watched` with both seats played, the bridge stepping the
clock whenever both are ready and no watcher at all. Each side has a door for
an agent, whose address holds while the bridge runs.
"""

import argparse
import csv
import json
import os
import re
import secrets
import shlex
import signal
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlsplit

HERE = os.path.dirname(os.path.abspath(__file__))
SIDES = ("allied", "japanese")
# A watched game cannot be begun again while an agent has called its door
# within this many seconds (decision 0182, amended 20).
RECENT = 300
# The tool of the door's own: it holds again for the answer of a call of time
# (decision 0182, amended 17).
WAIT_TOOL = {
    "name": "wait",
    "description": (
        "Wait for the answer of your call of end_day, run or advance. In this game a watcher "
        "holds the clock, and such a call is answered when the clock has reached the stop you "
        "asked for. If its answer was {\"waiting\": true}, call wait (no arguments) until the "
        "answer comes; it is the answer the call would have given."
    ),
    "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
}
DOOR_WORDS = (
    "\n\nThis game is watched. A watcher holds the clock and moves it a step at a time. Your "
    "calls of end_day, run and advance are answered when the clock has reached the stop you "
    "asked for, which may take a while; meanwhile you can make no other call. If such a call "
    "answers {\"waiting\": true}, call the tool wait, again as often as it answers so, until "
    "the answer comes."
)


class Game:
    """The seat's server, spoken to over its standard input and output."""

    def __init__(self, command):
        self.lock = threading.Lock()
        self.next_id = 0
        self.process = subprocess.Popen(
            command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True
        )
        self.request(
            "initialize",
            {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "flag-plot-table", "version": "0"},
            },
        )
        self.send({"jsonrpc": "2.0", "method": "notifications/initialized"})

    def send(self, message):
        self.process.stdin.write(json.dumps(message) + "\n")
        self.process.stdin.flush()

    def request(self, method, params):
        self.next_id += 1
        self.send({"jsonrpc": "2.0", "id": self.next_id, "method": method, "params": params})
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError("the game's server has stopped")
        return json.loads(line)

    def tools(self):
        """The names of the tools this seat is offered in this game."""
        with self.lock:
            reply = self.request("tools/list", {})
        return [tool["name"] for tool in reply.get("result", {}).get("tools", [])]

    def call(self, tool, arguments):
        """One tool call. Answers {"ok": the answer} or {"refused": the words}."""
        with self.lock:
            reply = self.request("tools/call", {"name": tool, "arguments": arguments})
        if "error" in reply:
            return {"refused": reply["error"].get("message", "refused")}
        result = reply.get("result", {})
        text = "".join(part.get("text", "") for part in result.get("content", []))
        if result.get("isError"):
            return {"refused": text}
        try:
            return {"ok": json.loads(text)}
        except ValueError:
            return {"ok": {"text": text}}

    def finish(self, timeout=60):
        """Ends the game: its server keeps what it was told to on leaving."""
        with self.lock:
            try:
                self.process.stdin.close()
            except OSError:
                pass
            try:
                self.process.wait(timeout)
            except subprocess.TimeoutExpired:
                self.process.terminate()


# ---------------------------------------------------------------------------
# A watched game (decision 0182, step 2).
# ---------------------------------------------------------------------------


class Slot:
    """One request to `serve`, and its answer line once it has come."""

    def __init__(self):
        self.done = threading.Event()
        self.line = None

    def put(self, line):
        self.line = line
        self.done.set()

    def answer(self):
        """The answer as the bridge gives it: {"ok": ..} or {"refused": ..}."""
        line = self.line
        if "error" in line:
            error = line["error"]
            out = {"refused": error.get("message", "refused")}
            if error.get("allowed"):
                out["allowed"] = error["allowed"]
            return out
        return {"ok": line.get("result")}


class Watched:
    """`sim-cli serve --watched`, spoken to in JSON lines. Its answers are read
    on a thread of their own and matched to their requests by `id`, since a
    held call's answer comes after later requests' (decision 0182, amended 18)."""

    def __init__(self, command, journal=None):
        # Every request that can change the game is written here as it is
        # sent, so a game killed outright can be played again from it.
        self.journal = open(journal, "w") if journal else None
        self.write_lock = threading.Lock()
        self.table_lock = threading.Lock()
        self.pending = {}
        self.next_id = 0
        self.ended = None
        self.process = subprocess.Popen(
            command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True
        )
        self.reader = threading.Thread(target=self.read, daemon=True)
        self.reader.start()

    def read(self):
        for raw in self.process.stdout:
            try:
                line = json.loads(raw)
            except ValueError:
                continue
            with self.table_lock:
                slot = self.pending.pop(line.get("id"), None)
            if slot is not None:
                slot.put(line)
        # The server has stopped: every call still waiting is told so.
        with self.table_lock:
            self.ended = self.ended or "the game's server has stopped"
            left, self.pending = self.pending, {}
        for slot in left.values():
            slot.put({"error": {"message": self.ended}})

    def abandon(self, words):
        """Answers every request still waiting with `words`, and every later
        one: the game is being ended (decision 0185, amended 13)."""
        with self.table_lock:
            self.ended = words
            left, self.pending = self.pending, {}
        for slot in left.values():
            slot.put({"error": {"message": words}})

    def send(self, request):
        """Sends one request and gives its slot, whose answer may come later."""
        slot = Slot()
        with self.table_lock:
            if self.ended:
                slot.put({"error": {"message": self.ended}})
                return slot
            self.next_id += 1
            request = dict(request, id=self.next_id)
            self.pending[self.next_id] = slot
        with self.write_lock:
            try:
                self.process.stdin.write(json.dumps(request) + "\n")
                self.process.stdin.flush()
                if self.journal and request.get("watcher") in (None, "step"):
                    self.journal.write(json.dumps(request) + "\n")
                    self.journal.flush()
            except (BrokenPipeError, ValueError, OSError):
                with self.table_lock:
                    self.pending.pop(request["id"], None)
                slot.put({"error": {"message": "the game's server has stopped"}})
        return slot

    def ask(self, request, timeout=120):
        """A request answered at once (any but a seat's call of time)."""
        slot = self.send(request)
        if not slot.done.wait(timeout):
            return {"refused": "the game's server did not answer"}
        return slot.answer()

    def finish(self, timeout=60):
        """Ends the game: `serve` keeps what it was told to on leaving."""
        with self.write_lock:
            try:
                self.process.stdin.close()
            except OSError:
                pass
        try:
            self.process.wait(timeout)
        except subprocess.TimeoutExpired:
            self.process.terminate()


class Door:
    """One agent's seat: its key, the call of time it holds, and when it last
    called (decision 0182, amended 15 and 17)."""

    def __init__(self, side, key):
        self.side = side
        self.key = key
        self.lock = threading.Lock()
        self.held = None
        self.held_tool = None
        self.last_call = 0.0
        # Words owed to a call of wait whose game has ended (decision 0185).
        self.note = None

    def reset(self):
        with self.lock:
            self.held = None
            self.held_tool = None


def waiting_words(side, tool):
    return {
        "waiting": True,
        "tool": tool,
        "words": (
            f"The clock has not yet reached the stop of your {tool}. Call the tool wait for "
            "its answer."
        ),
    }


def seat_call(game, door, tool, arguments, hold, sent=None):
    """One call at a seat's door. A call is passed to `serve` as the seat's;
    one not answered within `hold` seconds is held, and `wait` collects it.
    `sent`, where given, is told once the call has gone to `serve`."""
    door.last_call = time.monotonic()
    with door.lock:
        note, door.note = door.note, None
    if tool == "wait":
        if note:
            return {"refused": note}
        with door.lock:
            slot, held_tool = door.held, door.held_tool
        if slot is None:
            return {"refused": "nothing waits: you have no call of end_day, run or advance held"}
    else:
        with door.lock:
            if door.held is not None and door.held.done.is_set():
                return {
                    "refused": (
                        f"your call of {door.held_tool} has been answered: call wait to "
                        "collect its answer first"
                    )
                }
        slot = game.send(
            {"seat": door.side, "tool": tool, "arguments": arguments if arguments else {}}
        )
        if sent is not None:
            sent()
        held_tool = tool
    if not slot.done.wait(hold):
        with door.lock:
            if door.held is None:
                door.held, door.held_tool = slot, held_tool
        return waiting_words(door.side, held_tool)
    with door.lock:
        if door.held is slot:
            door.held, door.held_tool = None, None
    return slot.answer()


DEFAULT_MAP = "data/map/south_pacific.csv"


def read_map(path):
    hexes = []
    with open(path, newline="") as handle:
        rows = csv.DictReader(line for line in handle if not line.startswith("#"))
        for row in rows:
            hexes.append(
                [
                    int(row["q"]),
                    int(row["r"]),
                    row["kind"],
                    float(row["lat"]),
                    float(row["lon"]),
                    int(row["land_percent"] or 0),
                ]
            )
    return hexes


def read_frame(path):
    """A map's place on the earth, from the lines of its header (decision 0199):
    the centre of hex (0,0), the factor for longitude and the extent, every
    longitude in the run (above 180 where the map crosses the date line), as
    the export's map.frame has it. None for a map with no such lines."""
    both = read_frame_and_edges(path)
    return both[0] if both else None


def read_frame_and_edges(path):
    """The frame of read_frame and the extent's north and south edges in
    degrees (the clipping script wants them), or None."""
    head = []
    with open(path) as handle:
        for line in handle:
            if not line.startswith("#"):
                break
            head.append(line)
    text = "".join(head)
    centre = re.search(r"hex \(0,0\) is centred at ([\d.]+) ([NS]), ([\d.]+) ([EW])", text)
    factor = re.search(r"east_nm\s*=\s*\(lon - [\d.]+\) \* 60 \* ([\d.]+)", text)
    extent = re.search(
        r"Extent: every hex whose centre lies from ([\d.]+) ([NS]) to ([\d.]+) ([NS]) and from ([\d.]+) ([EW]) to ([\d.]+) ([EW])",
        text,
    )
    if not (centre and factor and extent):
        return None

    def signed(value, letter):
        return -float(value) if letter in "SW" else float(value)

    west = signed(extent.group(5), extent.group(6))
    east = signed(extent.group(7), extent.group(8))
    centre_lon = signed(centre.group(3), centre.group(4))
    if east < west:
        east += 360
        if centre_lon < west:
            centre_lon += 360
    frame = {
        "centre_lat": signed(centre.group(1), centre.group(2)),
        "centre_lon": centre_lon,
        "factor": float(factor.group(1)),
        "west": west,
        "east": east,
    }
    return frame, signed(extent.group(1), extent.group(2)), signed(extent.group(3), extent.group(4))


CHARTS = {}


def load_chart(path):
    """The chart of a map file: its hexes, its frame, and the coast and names
    beside this script in chart/ that are named for the map file (for the eye
    only: the game plays on the hexes and reads none of them). Loaded once for
    each map."""
    key = os.path.abspath(path)
    if key not in CHARTS:
        base = os.path.join(HERE, "chart", os.path.splitext(os.path.basename(path))[0])
        coast = names = None
        if os.path.exists(base + "_coast.json"):
            with open(base + "_coast.json") as handle:
                coast = json.load(handle)
        if os.path.exists(base + "_names.json"):
            with open(base + "_names.json") as handle:
                names = json.load(handle)["names"]
        CHARTS[key] = {"hexes": read_map(path), "frame": read_frame(path), "coast": coast, "names": names}
    return CHARTS[key]


def read_directives(path):
    with open(path) as handle:
        return json.load(handle)["directives"]


VENDOR = os.path.join(HERE, "vendor")
VENDOR_KINDS = {
    ".js": "text/javascript; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".woff2": "font/woff2",
}


def vendored(path):
    """What the page loads from nobody else (decision 0229, ruling 40): the
    drawing library and the fonts, kept in vendor/. A path that begins /vendor/
    gives (content type, bytes) of the file it names there; anything else, or a
    path that leaves the folder or names a file of another kind, gives None."""
    if not path.startswith("/vendor/"):
        return None
    name = os.path.normpath(os.path.join(VENDOR, path[len("/vendor/") :]))
    kind = VENDOR_KINDS.get(os.path.splitext(name)[1])
    if kind is None or os.path.commonpath([VENDOR, name]) != VENDOR or not os.path.isfile(name):
        return None
    with open(name, "rb") as handle:
        return kind, handle.read()


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument(
        "scenario",
        nargs="?",
        help="the game to hold; with none the table opens at the new game screen (decision 0185)",
    )
    parser.add_argument(
        "--seat",
        action="append",
        choices=list(SIDES),
        help="the side played: one, or with --watch each side an agent plays",
    )
    parser.add_argument("--map", default=DEFAULT_MAP)
    parser.add_argument("--start", default="", help="the date of day 1, as 1942-08-07")
    parser.add_argument(
        "--island",
        action="append",
        default=[],
        help='an island the scenario names, as "1=Guadalcanal@1": its number, a name, '
        "and the number of a base that stands on it. No tool lists islands yet.",
    )
    parser.add_argument(
        "--directives",
        action="append",
        default=[],
        help="a file of directives from the commander above, as table/directives/ has them; "
        'with --watch, "allied=FILE" or "japanese=FILE" for each side that has one',
    )
    parser.add_argument("--port", type=int, default=8742)
    parser.add_argument("--sim-cli", default="target/release/sim-cli")
    parser.add_argument(
        "--watch",
        action="store_true",
        help="a watched game: agents at the seats named by --seat, the page at the clock",
    )
    parser.add_argument(
        "--keep",
        default="",
        help="with --watch: a folder to keep the saved game and each seat's transcript in on leaving",
    )
    parser.add_argument(
        "--hold",
        type=float,
        default=50,
        help="with --watch: seconds a door holds a call of time before it answers that it waits",
    )
    parser.add_argument(
        "--scenarios",
        default=os.path.join(HERE, "scenarios.json"),
        help="with no scenario: the list of scenarios the new game screen offers",
    )
    args = parser.parse_args()
    if args.scenario is None:
        given = [
            name
            for name, value in (
                ("--seat", args.seat),
                ("--start", args.start),
                ("--island", args.island),
                ("--directives", args.directives),
                ("--watch", args.watch),
            )
            if value
        ]
        if given:
            sys.exit(
                f"{', '.join(given)} name a game: with no scenario the new game screen chooses it"
            )

    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    islands = []
    for item in args.island:
        number, rest = item.split("=", 1)
        name, base = rest.rsplit("@", 1)
        islands.append({"number": int(number), "name": name, "base": int(base)})
    chart = load_chart(args.map)
    if args.scenario is None:
        home(args, load_chart)
        return
    if args.watch:
        watch(args, islands, chart)
        return
    if args.seat and len(args.seat) > 1:
        sys.exit("one --seat only: two seats are a watched game (--watch)")
    if args.keep:
        sys.exit("--keep is for a watched game (--watch)")
    seat = args.seat[0] if args.seat else "allied"
    directives = []
    if args.directives:
        directives = read_directives(args.directives[-1])
    config = {
        "seat": seat,
        "start": args.start,
        "islands": islands,
        "directives": directives,
    }
    command = [args.sim_cli, "mcp", args.scenario, "--seat", seat]
    # The game in play. A restart ends it and begins the scenario again from
    # its first evening: nothing of the old game is kept.
    held = {"game": Game(command)}
    swap = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def answer(self, status, kind, body):
            self.send_response(status)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def json(self, value, status=200):
            self.answer(status, "application/json", json.dumps(value).encode())

        def do_GET(self):
            if self.path in ("/", "/index.html"):
                with open(os.path.join(HERE, "index.html"), "rb") as page:
                    self.answer(200, "text/html; charset=utf-8", page.read())
            elif vendored(urlsplit(self.path).path):
                self.answer(200, *vendored(urlsplit(self.path).path))
            elif self.path == "/config":
                self.json(config)
            elif self.path == "/tools":
                self.json(held["game"].tools())
            elif self.path == "/map":
                self.json(chart["hexes"])
            elif self.path == "/frame":
                self.json(chart["frame"])
            elif self.path == "/coast" and chart["coast"] is not None:
                self.json(chart["coast"])
            elif self.path == "/names" and chart["names"] is not None:
                self.json(chart["names"])
            else:
                self.answer(404, "text/plain", b"not found")

        def do_POST(self):
            if self.path not in ("/call", "/restart"):
                self.answer(404, "text/plain", b"not found")
                return
            # Only this machine's own page may call: a page of another site
            # cannot send JSON here without asking first, and is not answered.
            if self.headers.get("Content-Type", "").split(";")[0] != "application/json":
                self.json({"refused": "send JSON"}, 415)
                return
            if self.path == "/restart":
                with swap:
                    old = held["game"]
                    try:
                        held["game"] = Game(command)
                    except (RuntimeError, OSError) as error:
                        self.json({"refused": str(error)}, 500)
                        return
                    old.process.terminate()
                self.json({"ok": True})
                return
            try:
                length = int(self.headers.get("Content-Length", "0"))
                body = json.loads(self.rfile.read(length))
                tool, arguments = body["tool"], body.get("arguments", {})
            except (ValueError, KeyError):
                self.json({"refused": "a call is {tool, arguments}"}, 400)
                return
            try:
                self.json(held["game"].call(tool, arguments))
            except RuntimeError as error:
                self.json({"refused": str(error)}, 500)

        def log_message(self, *_):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    print(f"The table is at http://127.0.0.1:{args.port}/  (seat: {seat})", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        held["game"].process.terminate()


def watch(args, islands, chart):
    """A watched game: the page holds the clock and reads, each agent has a door."""
    agents = []
    for side in args.seat or []:
        if side in agents:
            sys.exit(f"--seat names the {side} side twice")
        agents.append(side)
    if not agents:
        sys.exit("--watch needs --seat for each side an agent plays: --seat allied, --seat japanese or both")
    agents.sort(key=SIDES.index)
    directives = {}
    for item in args.directives:
        side, _, path = item.partition("=")
        if side not in SIDES or not path:
            side, path = agents[0], item
        directives[side] = read_directives(path)
    keep = os.path.abspath(args.keep) if args.keep else ""
    if keep:
        os.makedirs(keep, exist_ok=True)

    # The keys: one for each agent's door and one for the watcher, new at
    # every start. They keep a program from calling a door not its own by
    # mistake; they do not stop a program on this machine that sets out to.
    watcher_key = secrets.token_urlsafe(18)
    doors = {side: Door(side, secrets.token_urlsafe(18)) for side in agents}
    by_key = {door.key: door for door in doors.values()}
    games = {"count": 0}

    def paths(number):
        suffix = "" if number == 1 else f"-{number}"
        return (
            os.path.join(keep, f"save{suffix}.json"),
            os.path.join(keep, f"transcript{suffix}.jsonl"),
            suffix,
        )

    def start():
        games["count"] += 1
        command = [args.sim_cli, "serve", args.scenario, "--watched"]
        for side in agents:
            command += ["--seat", f"{side}=player"]
        if keep:
            save, transcript, suffix = paths(games["count"])
            command += ["--save", save, "--transcript", transcript]
            return Watched(command, os.path.join(keep, f"journal{suffix}.jsonl"))
        return Watched(command)

    def finish(game, number):
        """Ends a game and splits its transcript into one file a seat."""
        game.finish()
        if not keep:
            return
        _, transcript, suffix = paths(number)
        if not os.path.exists(transcript):
            return
        lines = {side: [] for side in agents}
        with open(transcript) as handle:
            for line in handle:
                try:
                    seat = json.loads(line).get("seat")
                except ValueError:
                    continue
                if seat in lines:
                    lines[seat].append(line)
        for side, kept in lines.items():
            with open(os.path.join(keep, f"transcript_{side}{suffix}.jsonl"), "w") as handle:
                handle.writelines(kept)

    held = {"game": start(), "number": 1}
    swap = threading.Lock()

    # Whether the scenario has orders inside a day: the agents' seats then have
    # `advance`, and a step is a pulse.
    first = held["game"].ask({"seat": agents[0], "request": "tools"})
    tool_names = [t["name"] for t in (first.get("ok") or {}).get("tools", [])]
    if "refused" in first:
        held["game"].finish()
        sys.exit(f"the game could not be opened: {first['refused']}")
    config = {
        "seat": agents[0],
        "start": args.start,
        "islands": islands,
        "directives": directives.get(agents[0], []),
        "watch": {
            "agents": agents,
            "inside_day": "advance" in tool_names,
            "directives": directives,
        },
    }

    def watcher_tools(side):
        # The watcher's list, as `serve` names it when asked for no tool.
        answer = held["game"].ask({"watcher": "read", "side": side, "tool": ""})
        return answer.get("allowed", [])

    def same(given, key):
        return secrets.compare_digest(given.encode(), key.encode())

    def recently_called():
        now = time.monotonic()
        return [d.side for d in doors.values() if d.last_call and now - d.last_call < RECENT]

    class Handler(BaseHTTPRequestHandler):
        def answer(self, status, kind, body):
            self.send_response(status)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def json(self, value, status=200):
            self.answer(status, "application/json", json.dumps(value).encode())

        def where(self):
            """(kind, door or None, the rest of the path, the query), or None
            for a path that is no door's. A wrong key is no door."""
            parts = urlsplit(self.path)
            words = parts.path.split("/")
            query = parse_qs(parts.query)
            if len(words) >= 3 and words[1] == "watch" and same(words[2], watcher_key):
                return "watch", None, "/" + "/".join(words[3:]), query
            if len(words) >= 3 and words[1] == "seat":
                door = next((d for k, d in by_key.items() if same(words[2], k)), None)
                if door is not None:
                    return "seat", door, "/" + "/".join(words[3:]), query
            return None

        def refuse(self):
            self.json({"refused": "no door here: use the address the table printed when it started"}, 403)

        def body(self):
            if self.headers.get("Content-Type", "").split(";")[0] != "application/json":
                raise ValueError("send JSON")
            length = int(self.headers.get("Content-Length", "0"))
            value = json.loads(self.rfile.read(length) or b"{}")
            if not isinstance(value, dict):
                raise ValueError("send a JSON object")
            return value

        def side_of(self, query, body=None):
            side = ((body or {}).get("side")) or (query.get("side") or [agents[0]])[0]
            if side not in agents:
                raise ValueError(f"the watcher looks from a side an agent plays: {', '.join(agents)}")
            return side

        def do_GET(self):
            found = self.where()
            if found is None:
                if urlsplit(self.path).path == "/":
                    self.answer(
                        200,
                        "text/plain; charset=utf-8",
                        b"This game is watched. Open the watcher's address the table printed when it started.\n",
                    )
                else:
                    self.refuse()
                return
            kind, door, path, query = found
            game = held["game"]
            if kind == "seat":
                if path == "/tools":
                    answer = game.ask({"seat": door.side, "request": "tools"})
                    if "ok" in answer:
                        answer = {"ok": {"tools": answer["ok"]["tools"] + [WAIT_TOOL]}}
                    self.json(answer)
                elif path == "/instructions":
                    answer = game.ask({"seat": door.side, "request": "instructions"})
                    if "ok" in answer:
                        answer = {"ok": {"instructions": answer["ok"]["instructions"] + DOOR_WORDS}}
                    self.json(answer)
                else:
                    self.answer(404, "text/plain", b"not found")
                return
            try:
                if path in ("/", "/index.html"):
                    with open(os.path.join(HERE, "index.html"), "rb") as page:
                        self.answer(200, "text/html; charset=utf-8", page.read())
                elif vendored(path):
                    self.answer(200, *vendored(path))
                elif path == "/config":
                    self.json(config)
                elif path == "/tools":
                    self.json(watcher_tools(self.side_of(query)))
                elif path == "/map":
                    self.json(chart["hexes"])
                elif path == "/frame":
                    self.json(chart["frame"])
                elif path == "/coast" and chart["coast"] is not None:
                    self.json(chart["coast"])
                elif path == "/names" and chart["names"] is not None:
                    self.json(chart["names"])
                elif path == "/standing":
                    self.json(game.ask({"watcher": "standing"}))
                elif path == "/seat_calls":
                    self.json(game.ask({"watcher": "seat_calls", "side": self.side_of(query)}))
                else:
                    self.answer(404, "text/plain", b"not found")
            except ValueError as error:
                self.json({"refused": str(error)}, 400)

        def do_POST(self):
            found = self.where()
            if found is None:
                self.refuse()
                return
            kind, door, path, query = found
            try:
                body = self.body()
            except ValueError as error:
                self.json({"refused": str(error)}, 415)
                return
            game = held["game"]
            if kind == "seat":
                if path != "/call" or not isinstance(body.get("tool"), str):
                    self.json({"refused": "a call is POST /call with {tool, arguments}"}, 400)
                    return
                arguments = body.get("arguments") or {}
                self.json(seat_call(game, door, body["tool"], arguments, args.hold))
                return
            try:
                if path == "/call":
                    if not isinstance(body.get("tool"), str):
                        raise ValueError("a read is {tool, arguments, side}")
                    self.json(
                        game.ask(
                            {
                                "watcher": "read",
                                "side": self.side_of(query, body),
                                "tool": body["tool"],
                                "arguments": body.get("arguments") or {},
                            }
                        )
                    )
                elif path == "/step":
                    self.json(game.ask({"watcher": "step"}, timeout=600))
                elif path == "/restart":
                    with swap:
                        busy = recently_called()
                        if busy:
                            self.json(
                                {
                                    "refused": "an agent has called in the last few minutes ("
                                    + ", ".join(busy)
                                    + "): the game is not begun again under it"
                                }
                            )
                            return
                        old, number = held["game"], held["number"]
                        finish(old, number)
                        held["game"], held["number"] = start(), number + 1
                        for each in doors.values():
                            each.reset()
                    self.json({"ok": True})
                else:
                    self.answer(404, "text/plain", b"not found")
            except ValueError as error:
                self.json({"refused": str(error)}, 400)

        def log_message(self, *_):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    server.daemon_threads = True
    base = f"http://127.0.0.1:{args.port}"
    print(f"A watched game. The watcher's table is at {base}/watch/{watcher_key}/", flush=True)
    for side in agents:
        print(f"The {side} seat's door is {base}/seat/{doors[side].key}", flush=True)
    others = [side for side in SIDES if side not in agents]
    if others:
        print(f"The {others[0]} side is the script's.", flush=True)
    if keep:
        print(f"On leaving, the game and the transcripts are kept in {keep}", flush=True)

    def leave(*_):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, leave)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
        finish(held["game"], held["number"])


# ---------------------------------------------------------------------------
# The new game screen, and the games it begins (decision 0185).
# ---------------------------------------------------------------------------

ROOT = os.path.dirname(HERE)
TIME_TOOLS = ("end_day", "run", "advance")
# What a call still held is answered when its game ends (amended 13).
ENDED = (
    "The game has ended at the table and a new one has begun. Your next call is in the new "
    "game: read its opening words again (the instructions) before you go on."
)
# The person's own calls are held this long before the page is told to wait.
PERSON_HOLD = 20
HOME_WAIT_TOOL = {
    "name": "wait",
    "description": (
        "Wait for the answer of a call that answered {\"waiting\": true}. Any call may be held: "
        "a call of end_day, run or advance until the clock has reached the stop you asked for, "
        "and any other call while the game works another. Call wait (no arguments) as often as "
        "it answers so, until the answer comes; it is the answer the call would have given."
    ),
    "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
}
# The door's opening words for a game of a person against an agent (amended
# 6 and 7): no watcher; the clock moves when both commanders have finished.
VERSUS_WORDS = (
    "\n\nThe other side has a commander of its own, and there is no one else at the clock: it "
    "moves when both commanders have finished. Your calls of end_day, run and advance are your "
    "word that you have finished, and are answered when the clock has reached the stop you "
    "asked for, which may take a while; meanwhile you can make no other call. The game answers "
    "one call at a time, so any call, not only a call of time, may be held while the other "
    "commander's call is worked. A call that answers {\"waiting\": true} is still held: call the "
    "tool wait, again as often as it answers so, until the answer comes."
)
WATCHED_WORDS = DOOR_WORDS + (
    " The game answers one call at a time, so any call, not only a call of time, may be held "
    "while another is worked, and answer {\"waiting\": true} the same way."
)
SIDE_WORDS = {"allied": "Allied", "japanese": "Japanese"}
# What the person is told beside what he copies (amended 12).
AGENT_NOTES = [
    "A model at a keyboard stops of itself long before a campaign ends. Tell it to go on when it stops, or run it so that it goes on.",
    "A model with a shell could read the scenario's file and see the other side. Asking it not to is the only guard.",
    "A call may wait up to about fifty seconds before it answers that it waits; the model then calls wait, and goes on waiting.",
]


def load_scenarios(path):
    """The list of the screen, checked: every file it names is there."""
    with open(path) as handle:
        listed = json.load(handle)["scenarios"]
    for entry in listed:
        for pairing in entry["pairings"]:
            holders = [pairing[side] for side in SIDES]
            if "person" in holders and holders.count("person") != 1:
                sys.exit(f"{path}: {entry['id']}: one person at most")
            if "person" not in holders and "agent" not in holders:
                sys.exit(f"{path}: {entry['id']}: a pairing with no one playing")
            if not os.path.exists(os.path.join(ROOT, pairing["file"])):
                sys.exit(f"{path}: {entry['id']}: no file {pairing['file']}")
        if not os.path.exists(os.path.join(ROOT, entry.get("map", DEFAULT_MAP))):
            sys.exit(f"{path}: {entry['id']}: no map file {entry['map']}")
        for side, file in entry.get("directives", {}).items():
            if not os.path.exists(os.path.join(ROOT, file)):
                sys.exit(f"{path}: {entry['id']}: no directives file {file}")
    return listed


def first_directive(path):
    """The directive a side receives on the first evening: (from, its mission)."""
    for directive in read_directives(path):
        if directive.get("when") == "start":
            for part in directive.get("parts", []):
                if "mission" in part.get("heading", "").lower():
                    return directive.get("from", ""), " ".join(part.get("text", []))
    return None


class Hand:
    """The game in hand: what was chosen, how it is held, and what it knows of
    the agents' seats."""

    def __init__(self, number, entry, pairing):
        self.number = number
        self.entry = entry
        self.pairing = pairing
        holders = {side: pairing[side] for side in SIDES}
        self.person = next((s for s in SIDES if holders[s] == "person"), None)
        self.agents = [s for s in SIDES if holders[s] == "agent"]
        if self.person and self.agents:
            self.kind = "versus"
        elif self.person:
            self.kind = "engine"
        else:
            self.kind = "watch"
        self.game = None
        self.config = None
        self.person_door = None
        self.reads = []
        # When each agent's door was last called in this game: a door not
        # called is "not yet at its seat" (amended 16).
        self.seen = {}
        self.stop = threading.Event()
        self.poke = threading.Event()


def home(args, load_chart):
    """The bridge started with no scenario: the new game screen, and the game
    begun from it, held as its seats need (decision 0185)."""
    listed = load_scenarios(args.scenarios)
    by_id = {entry["id"]: entry for entry in listed}
    keep = os.path.abspath(args.keep) if args.keep else ""
    if keep:
        os.makedirs(keep, exist_ok=True)
    # The person's key, for the screen and his table; and a door for each
    # side, whose address holds while the bridge runs (amended 13). They keep
    # a program from calling what is not its own by mistake, and no more.
    table_key = secrets.token_urlsafe(18)
    doors = {side: Door(side, secrets.token_urlsafe(18)) for side in SIDES}
    base = f"http://127.0.0.1:{args.port}"
    state = {"hand": None, "count": 0}
    swap = threading.Lock()

    def same(given, key):
        return secrets.compare_digest(given.encode(), key.encode())

    def map_of(entry):
        """The map file of a scenario of the list: named by its entry, else the present map."""
        return os.path.join(ROOT, (entry or {}).get("map", DEFAULT_MAP))

    def chart_in_hand():
        """The chart of the game in hand, loaded when the game opened; with
        none in hand, that of --map."""
        return load_chart(map_of(state["hand"].entry) if state["hand"] else args.map)

    def directives_of(entry, side):
        file = entry.get("directives", {}).get(side)
        return read_directives(os.path.join(ROOT, file)) if file else []

    def folder(number):
        path = os.path.join(keep, f"game-{number}")
        os.makedirs(path, exist_ok=True)
        return path

    def open_game(hand):
        """Starts the game in hand as its seats need. Gives words of refusal,
        or None."""
        entry, file = hand.entry, os.path.join(ROOT, hand.pairing["file"])
        load_chart(map_of(entry))
        kept = folder(hand.number) if keep else ""
        common = {
            "start": entry.get("start", ""),
            "islands": entry.get("islands", []),
            "home": {"scenario": entry["name"], "kind": hand.kind, "agents": hand.agents, "number": hand.number},
        }
        if hand.kind == "engine":
            command = [args.sim_cli, "mcp", file, "--seat", hand.person]
            if kept:
                command += ["--save", os.path.join(kept, "save.json"), "--transcript", os.path.join(kept, "transcript.jsonl")]
            try:
                hand.game = Game(command)
            except (RuntimeError, OSError, ValueError) as error:
                return f"the game could not be opened: {error}"
            hand.config = dict(common, seat=hand.person, directives=directives_of(entry, hand.person))
            return None
        played = ([hand.person] if hand.person else []) + hand.agents
        command = [args.sim_cli, "serve", file, "--watched"]
        for side in sorted(played, key=SIDES.index):
            command += ["--seat", f"{side}=player"]
        if kept:
            command += ["--save", os.path.join(kept, "save.json"), "--transcript", os.path.join(kept, "transcript.jsonl")]
            hand.game = Watched(command, os.path.join(kept, "journal.jsonl"))
        else:
            hand.game = Watched(command)
        first = hand.game.ask({"seat": played[0], "request": "tools"})
        if "refused" in first:
            hand.game.finish()
            return f"the game could not be opened: {first['refused']}"
        inside = "advance" in [t["name"] for t in first["ok"]["tools"]]
        if hand.kind == "watch":
            hand.config = dict(
                common,
                seat=hand.agents[0],
                directives=directives_of(entry, hand.agents[0]),
                watch={
                    "agents": hand.agents,
                    "inside_day": inside,
                    "directives": {side: directives_of(entry, side) for side in hand.agents},
                },
            )
            return None
        # A person against an agent: his seat's calls go through a door of
        # his own, held as an agent's are; the bridge steps the clock.
        hand.person_door = Door(hand.person, "")
        read = hand.game.ask({"watcher": "read", "side": hand.person, "tool": ""})
        hand.reads = read.get("allowed", [])
        hand.config = dict(
            common,
            seat=hand.person,
            directives=directives_of(entry, hand.person),
            versus={"other": hand.agents[0], "inside_day": inside},
        )
        threading.Thread(target=stepper, args=(hand,), daemon=True).start()
        return None

    def stepper(hand):
        """Steps the clock whenever both seats are ready (amended 3 to 8)."""
        game = hand.game
        while not hand.stop.is_set():
            hand.poke.wait(0.5)
            hand.poke.clear()
            if hand.stop.is_set():
                return
            standing = game.ask({"watcher": "standing"}, timeout=120).get("ok")
            if not standing or standing["game_over"] or standing["stopped"] or standing["waiting_for"]:
                continue
            game.ask({"watcher": "step"}, timeout=3600)

    def split(hand):
        """Each seat's calls, from the game's transcript, to a file of its own."""
        transcript = os.path.join(keep, f"game-{hand.number}", "transcript.jsonl")
        if not os.path.exists(transcript):
            return
        lines = {side: [] for side in SIDES}
        with open(transcript) as handle:
            for line in handle:
                try:
                    seat = json.loads(line).get("seat")
                except ValueError:
                    continue
                if seat in lines:
                    lines[seat].append(line)
        for side, kept in lines.items():
            if kept:
                with open(os.path.join(keep, f"game-{hand.number}", f"transcript_{side}.jsonl"), "w") as handle:
                    handle.writelines(kept)

    def end(hand):
        """Ends the game in hand. Every call still held is answered that it
        has ended and a new one begun (amended 13 and 15)."""
        hand.stop.set()
        hand.poke.set()
        for side in SIDES:
            door = doors[side]
            with door.lock:
                if door.held is not None and side in hand.agents:
                    door.note = ENDED
                door.held, door.held_tool = None, None
        if hand.game is None:
            return
        if isinstance(hand.game, Watched):
            hand.game.abandon(ENDED)
            hand.game.finish()
            if keep:
                split(hand)
        else:
            hand.game.finish()

    def begin(entry, pairing):
        """Ends the game in hand, whoever is at a seat, and begins another."""
        old = state["hand"]
        state["hand"] = None
        if old is not None:
            end(old)
        state["count"] += 1
        hand = Hand(state["count"], entry, pairing)
        refused = open_game(hand)
        if refused:
            return refused
        state["hand"] = hand
        return None

    def offered(entry):
        """A scenario as the screen shows it: no file named."""
        return {
            "id": entry["id"],
            "name": entry["name"],
            "dates": entry.get("dates", ""),
            "days": entry.get("days"),
            "lines": entry.get("lines", []),
            "sides": entry.get("sides", {}),
            "pairings": [{k: v for k, v in p.items() if k != "file"} for p in entry["pairings"]],
        }

    def agent_seat(side, entry):
        """What is given to copy for an agent's seat, and what the person is
        told beside it (amended 11 and 12)."""
        door = f"{base}/seat/{doors[side].key}"
        program = os.path.join(HERE, "seat_mcp.py")
        name = f"flag-plot-{side}"
        settings = {"mcpServers": {name: {"command": "python3", "args": [program, "--url", door]}}}
        words = (
            f"You command the {SIDE_WORDS[side]} side in Flag Plot, a wargame of the Pacific War: "
            f"{entry['name']}, {entry.get('dates', '')}. Your seat is the {name} tools."
        )
        directive = None
        file = entry.get("directives", {}).get(side)
        if file:
            directive = first_directive(os.path.join(ROOT, file))
        if directive:
            words += f" Your directive, from the {directive[0]}: {directive[1]}"
        elif entry.get("sides", {}).get(side):
            words += f" Your mission: {entry['sides'][side]}"
        words += (
            " Play the game to its end: go on until a call answers that the game is over. Read "
            "nothing but what the tools tell you: not the game's files, not the scenario, nothing "
            "else on this machine."
        )
        return {
            "side": side,
            "door": door,
            "claude": f"claude mcp add {name} -- python3 {shlex.quote(program)} --url {door}",
            "settings": json.dumps(settings, indent=2),
            "brief": words,
            "notes": AGENT_NOTES,
        }

    def person_call(hand, tool, arguments):
        """The person's call at his own seat in a game against an agent. While
        his call is held his reads are the watcher's of his own side, and
        nothing else is taken (amended 3)."""
        door = hand.person_door
        hold = min(args.hold, PERSON_HOLD)
        if tool == "wait":
            # The held call stays with the door until an answer is given, so
            # a page loaded again picks the wait up where the last one left it.
            with door.lock:
                slot = door.held
            if slot is None:
                return {"refused": "nothing waits: you have no call held"}
        elif tool in TIME_TOOLS:
            slot = hand.game.send({"seat": hand.person, "tool": tool, "arguments": arguments or {}})
            with door.lock:
                door.held = slot
            hand.poke.set()
        else:
            # Whether his seat has finished is the game's own word, asked each
            # time: a call of time is held from the moment it is sent.
            standing = hand.game.ask({"watcher": "standing"}).get("ok") or {}
            mine = next((s for s in standing.get("seats", []) if s.get("seat") == hand.person), {})
            if mine.get("ready"):
                if tool not in hand.reads:
                    return {
                        "refused": "you have finished, and cannot do that until the clock has moved: "
                        "you may read meanwhile"
                    }
                return hand.game.ask({"watcher": "read", "side": hand.person, "tool": tool, "arguments": arguments})
            return seat_call(hand.game, door, tool, arguments, hold, sent=hand.poke.set)
        if not slot.done.wait(hold):
            return waiting_words(hand.person, tool if tool != "wait" else "call")
        with door.lock:
            if door.held is slot:
                door.held = None
        return slot.answer()

    def standing_for_person(hand):
        """Where the game stands, as the person may be told it: his own seat,
        and of the other only whether its commander is at his seat and whether
        he is waited for (amended 8)."""
        answer = hand.game.ask({"watcher": "standing"})
        if "ok" not in answer:
            return answer
        full = answer["ok"]
        seats = []
        for seat in full.get("seats", []):
            if seat.get("seat") == hand.person:
                with hand.person_door.lock:
                    seat = dict(seat, held=hand.person_door.held is not None)
                seats.append(seat)
            else:
                side = seat.get("seat")
                seats.append({"seat": side, "at_seat": side in hand.seen, "waited_for": bool(seat.get("waited_for"))})
        return {
            "ok": {
                "kind": "standing",
                "clock": full.get("clock"),
                "game_over": full.get("game_over"),
                "stopped": full.get("stopped"),
                "seats": seats,
                "waiting_for": full.get("waiting_for", []),
            }
        }

    def recently_called(hand):
        now = time.monotonic()
        return [side for side, at in hand.seen.items() if now - at < RECENT]

    class Handler(BaseHTTPRequestHandler):
        def answer(self, status, kind, body):
            self.send_response(status)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def json(self, value, status=200):
            self.answer(status, "application/json", json.dumps(value).encode())

        def where(self):
            parts = urlsplit(self.path)
            words = parts.path.split("/")
            query = parse_qs(parts.query)
            if len(words) >= 3 and words[1] == "table" and same(words[2], table_key):
                return "table", None, "/" + "/".join(words[3:]), query
            if len(words) >= 3 and words[1] == "seat":
                door = next((d for d in doors.values() if same(words[2], d.key)), None)
                if door is not None:
                    return "seat", door, "/" + "/".join(words[3:]), query
            return None

        def refuse(self):
            self.json({"refused": "no door here: use the address the table printed when it started"}, 403)

        def body(self):
            if self.headers.get("Content-Type", "").split(";")[0] != "application/json":
                raise ValueError("send JSON")
            length = int(self.headers.get("Content-Length", "0"))
            value = json.loads(self.rfile.read(length) or b"{}")
            if not isinstance(value, dict):
                raise ValueError("send a JSON object")
            return value

        def door_hand(self, door):
            """The game in hand where this door's side is an agent's, or the
            words of refusal."""
            hand = state["hand"]
            if hand is None:
                return None, "no game is in hand at the table: a new one is being chosen"
            if door.side not in hand.agents:
                return None, f"the {door.side} seat is not an agent's in the game in hand"
            hand.seen[door.side] = time.monotonic()
            return hand, None

        def do_GET(self):
            found = self.where()
            if found is None:
                if urlsplit(self.path).path == "/":
                    self.answer(
                        200,
                        "text/plain; charset=utf-8",
                        b"Open the table's address the bridge printed when it started.\n",
                    )
                else:
                    self.refuse()
                return
            kind, door, path, query = found
            if kind == "seat":
                hand, refused = self.door_hand(door)
                if refused:
                    self.json({"refused": refused})
                    return
                if path == "/tools":
                    answer = hand.game.ask({"seat": door.side, "request": "tools"})
                    if "ok" in answer:
                        answer = {"ok": {"tools": answer["ok"]["tools"] + [HOME_WAIT_TOOL]}}
                    self.json(answer)
                elif path == "/instructions":
                    answer = hand.game.ask({"seat": door.side, "request": "instructions"})
                    if "ok" in answer:
                        words = VERSUS_WORDS if hand.kind == "versus" else WATCHED_WORDS
                        answer = {"ok": {"instructions": answer["ok"]["instructions"] + words}}
                    self.json(answer)
                else:
                    self.answer(404, "text/plain", b"not found")
                return
            hand = state["hand"]
            try:
                if path in ("/", "/index.html"):
                    with open(os.path.join(HERE, "index.html"), "rb") as page:
                        self.answer(200, "text/html; charset=utf-8", page.read())
                elif vendored(path):
                    self.answer(200, *vendored(path))
                elif path == "/config":
                    self.json(hand.config if hand else {"screen": True})
                elif path == "/scenarios":
                    self.json({"scenarios": [offered(entry) for entry in listed]})
                elif path == "/agent":
                    side = (query.get("side") or [""])[0]
                    chosen = (query.get("scenario") or [""])[0]
                    entry = by_id.get(chosen) if chosen else (hand.entry if hand else None)
                    if side not in SIDES or entry is None:
                        raise ValueError("an agent's seat is asked with a side and a scenario")
                    self.json({"ok": agent_seat(side, entry)})
                elif path == "/map":
                    self.json(chart_in_hand()["hexes"])
                elif path == "/frame":
                    self.json(chart_in_hand()["frame"])
                elif path == "/coast" and chart_in_hand()["coast"] is not None:
                    self.json(chart_in_hand()["coast"])
                elif path == "/names" and chart_in_hand()["names"] is not None:
                    self.json(chart_in_hand()["names"])
                elif hand is None:
                    self.json({"refused": "no game is in hand: choose one on the new game screen"}, 409)
                elif path == "/tools":
                    if hand.kind == "engine":
                        self.json(hand.game.tools())
                    elif hand.kind == "versus":
                        answer = hand.game.ask({"seat": hand.person, "request": "tools"})
                        self.json([t["name"] for t in (answer.get("ok") or {}).get("tools", [])])
                    else:
                        side = (query.get("side") or [hand.agents[0]])[0]
                        if side not in hand.agents:
                            raise ValueError(f"the watcher looks from a side an agent plays: {', '.join(hand.agents)}")
                        self.json(hand.game.ask({"watcher": "read", "side": side, "tool": ""}).get("allowed", []))
                elif path == "/standing":
                    if hand.kind == "versus":
                        self.json(standing_for_person(hand))
                    elif hand.kind == "watch":
                        answer = hand.game.ask({"watcher": "standing"})
                        for seat in (answer.get("ok") or {}).get("seats", []):
                            if seat.get("seat") in hand.agents:
                                seat["at_seat"] = seat["seat"] in hand.seen
                        self.json(answer)
                    else:
                        self.json({"refused": "this game has no standing to ask: the other side is the engine's"})
                elif path == "/seat_calls":
                    if hand.kind != "watch":
                        self.json({"refused": "there is no watcher in this game"}, 403)
                    else:
                        side = (query.get("side") or [hand.agents[0]])[0]
                        if side not in hand.agents:
                            raise ValueError(f"the watcher looks from a side an agent plays: {', '.join(hand.agents)}")
                        self.json(hand.game.ask({"watcher": "seat_calls", "side": side}))
                else:
                    self.answer(404, "text/plain", b"not found")
            except ValueError as error:
                self.json({"refused": str(error)}, 400)

        def do_POST(self):
            found = self.where()
            if found is None:
                self.refuse()
                return
            kind, door, path, query = found
            try:
                body = self.body()
            except ValueError as error:
                self.json({"refused": str(error)}, 415)
                return
            if kind == "seat":
                if path != "/call" or not isinstance(body.get("tool"), str):
                    self.json({"refused": "a call is POST /call with {tool, arguments}"}, 400)
                    return
                hand, refused = self.door_hand(door)
                if refused:
                    with door.lock:
                        note, door.note = door.note, None
                    self.json({"refused": note or refused})
                    return
                arguments = body.get("arguments") or {}
                self.json(seat_call(hand.game, door, body["tool"], arguments, args.hold, sent=hand.poke.set))
                return
            try:
                if path == "/begin":
                    entry = by_id.get(body.get("scenario"))
                    if entry is None:
                        raise ValueError("no such scenario on the list")
                    asked = {side: body.get(side) for side in SIDES}
                    pairing = next(
                        (p for p in entry["pairings"] if all(p[side] == asked[side] for side in SIDES)),
                        None,
                    )
                    if pairing is None:
                        raise ValueError("that game is not offered for this scenario")
                    with swap:
                        refused = begin(entry, pairing)
                    if refused:
                        self.json({"refused": refused}, 500)
                    else:
                        self.json({"ok": True})
                    return
                if path == "/new":
                    with swap:
                        old = state["hand"]
                        state["hand"] = None
                        if old is not None:
                            end(old)
                    self.json({"ok": True})
                    return
                hand = state["hand"]
                if hand is None:
                    self.json({"refused": "no game is in hand: choose one on the new game screen"}, 409)
                    return
                if path == "/restart":
                    with swap:
                        if hand.kind != "engine":
                            busy = recently_called(hand)
                            if busy:
                                self.json(
                                    {
                                        "refused": "an agent has called in the last few minutes ("
                                        + ", ".join(busy)
                                        + "): the game is not begun again under it. New game ends it."
                                    }
                                )
                                return
                        refused = begin(hand.entry, hand.pairing)
                    self.json({"refused": refused} if refused else {"ok": True})
                    return
                if path == "/step":
                    if hand.kind != "watch":
                        self.json({"refused": "there is no watcher in this game"}, 403)
                    else:
                        self.json(hand.game.ask({"watcher": "step"}, timeout=600))
                    return
                if path != "/call":
                    self.answer(404, "text/plain", b"not found")
                    return
                if not isinstance(body.get("tool"), str):
                    raise ValueError("a call is {tool, arguments}")
                arguments = body.get("arguments") or {}
                if hand.kind == "engine":
                    if "side" in body:
                        raise ValueError("there is no watcher in this game")
                    try:
                        self.json(hand.game.call(body["tool"], arguments))
                    except RuntimeError as error:
                        self.json({"refused": str(error)}, 500)
                elif hand.kind == "versus":
                    if "side" in body:
                        self.json({"refused": "there is no watcher in this game"}, 403)
                    else:
                        self.json(person_call(hand, body["tool"], arguments))
                else:
                    side = body.get("side") or hand.agents[0]
                    if side not in hand.agents:
                        raise ValueError(f"the watcher looks from a side an agent plays: {', '.join(hand.agents)}")
                    self.json(
                        hand.game.ask(
                            {"watcher": "read", "side": side, "tool": body["tool"], "arguments": arguments}
                        )
                    )
            except ValueError as error:
                self.json({"refused": str(error)}, 400)

        def log_message(self, *_):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    server.daemon_threads = True
    print(f"The table is at {base}/table/{table_key}/  (the new game screen)", flush=True)
    if keep:
        print(f"Each game is kept in {keep}/game-N when it ends", flush=True)

    def leave(*_):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, leave)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
        if state["hand"] is not None:
            end(state["hand"])


if __name__ == "__main__":
    main()

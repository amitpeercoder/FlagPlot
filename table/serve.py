#!/usr/bin/env python3
"""The table's bridge: one seat of one game, served to a browser on this machine.

It starts `sim-cli mcp SCENARIO --seat SIDE` and passes the page's tool calls
to it, one at a time. The page gets what the seat's tools answer and nothing
else: this file never reads a replay, a save, a batch or the event log, and it
does not read the scenario. It reads one data file, the map (for drawing the
coast), which both sides know.

Run from the top of the repository:

    python3 table/serve.py data/scenarios/stage-3-proof/07b_slice.toml \
        --seat allied --start 1942-08-07 \
        --island "1=Guadalcanal@1" --island "2=Tulagi@4"

then open http://127.0.0.1:8742/. Standard library only. It listens on this
machine alone. Decision 0154.
"""

import argparse
import csv
import json
import os
import subprocess
import sys
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

HERE = os.path.dirname(os.path.abspath(__file__))


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


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("scenario")
    parser.add_argument("--seat", default="allied", choices=["allied", "japanese"])
    parser.add_argument("--map", default="data/map/south_pacific.csv")
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
        default="",
        help="a file of directives from the commander above, as table/directives/ has them",
    )
    parser.add_argument("--port", type=int, default=8742)
    parser.add_argument("--sim-cli", default="target/release/sim-cli")
    args = parser.parse_args()

    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    islands = []
    for item in args.island:
        number, rest = item.split("=", 1)
        name, base = rest.rsplit("@", 1)
        islands.append({"number": int(number), "name": name, "base": int(base)})
    directives = []
    if args.directives:
        with open(args.directives) as handle:
            directives = json.load(handle)["directives"]
    config = {
        "seat": args.seat,
        "start": args.start,
        "islands": islands,
        "directives": directives,
    }
    chart = read_map(args.map)
    # The coastline drawn on the chart, where the map has one beside this
    # script: for the eye only. The game plays on the hexes and reads none of it.
    coast = None
    coast_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "chart", os.path.splitext(os.path.basename(args.map))[0] + "_coast.json")
    if os.path.exists(coast_path):
        with open(coast_path) as handle:
            coast = json.load(handle)
    # The names lettered on the chart, where the map has a file of them: for
    # the eye only, as the coastline is.
    names = None
    names_path = coast_path.replace("_coast.json", "_names.json")
    if os.path.exists(names_path):
        with open(names_path) as handle:
            names = json.load(handle)["names"]
    game = Game([args.sim_cli, "mcp", args.scenario, "--seat", args.seat])

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
            elif self.path == "/config":
                self.json(config)
            elif self.path == "/tools":
                self.json(game.tools())
            elif self.path == "/map":
                self.json(chart)
            elif self.path == "/coast" and coast is not None:
                self.json(coast)
            elif self.path == "/names" and names is not None:
                self.json(names)
            else:
                self.answer(404, "text/plain", b"not found")

        def do_POST(self):
            if self.path != "/call":
                self.answer(404, "text/plain", b"not found")
                return
            # Only this machine's own page may call: a page of another site
            # cannot send JSON here without asking first, and is not answered.
            if self.headers.get("Content-Type", "").split(";")[0] != "application/json":
                self.json({"refused": "send JSON"}, 415)
                return
            try:
                length = int(self.headers.get("Content-Length", "0"))
                body = json.loads(self.rfile.read(length))
                tool, arguments = body["tool"], body.get("arguments", {})
            except (ValueError, KeyError):
                self.json({"refused": "a call is {tool, arguments}"}, 400)
                return
            try:
                self.json(game.call(tool, arguments))
            except RuntimeError as error:
                self.json({"refused": str(error)}, 500)

        def log_message(self, *_):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    print(f"The table is at http://127.0.0.1:{args.port}/  (seat: {args.seat})", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        game.process.terminate()


if __name__ == "__main__":
    main()

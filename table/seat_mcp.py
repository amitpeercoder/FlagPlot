#!/usr/bin/env python3
"""One seat's door of a watched game, offered as a Model Context Protocol server.

The table's bridge in watch mode (`table/serve.py --watch`) prints a door for
each agent's seat. This program speaks the protocol on its standard input and
output (JSON-RPC, one message a line: `initialize`, `tools/list`,
`tools/call`) and passes each tool call to that door, so that a model with
tools takes the seat as it takes `sim-cli mcp`. It adds no tool of its own:
the door's list already has `wait`. It starts no model and holds no key but
the door's, which is in the address. Decision 0182, step 2.

    python3 table/seat_mcp.py --url http://127.0.0.1:8760/seat/KEY

Standard library only.
"""

import argparse
import json
import sys
import urllib.error
import urllib.request

PROTOCOLS = ("2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05")


class Door:
    def __init__(self, url):
        self.url = url.rstrip("/")

    def get(self, path):
        with urllib.request.urlopen(self.url + path, timeout=120) as reply:
            return json.load(reply)

    def call(self, tool, arguments):
        request = urllib.request.Request(
            self.url + "/call",
            data=json.dumps({"tool": tool, "arguments": arguments}).encode(),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        # A call of time is held by the door for up to a minute or so.
        with urllib.request.urlopen(request, timeout=600) as reply:
            return json.load(reply)


def text_of(answer, tool, arguments, as_text):
    """A door's answer as a tool result: the answer as JSON text, or the
    briefing's text where it was asked as text, as `sim-cli mcp` gives it."""
    if "refused" in answer:
        words = answer["refused"]
        if answer.get("allowed"):
            words += " (one of: " + ", ".join(answer["allowed"]) + ")"
        return words, True
    if "waiting" in answer:
        return json.dumps(answer, indent=2), False
    value = answer.get("ok")
    if as_text and isinstance(value, dict) and isinstance(value.get("text"), str):
        return value["text"], False
    return json.dumps(value, indent=2), False


def handle(door, message):
    method = message.get("method")
    params = message.get("params") or {}
    if method == "initialize":
        asked = params.get("protocolVersion")
        instructions = door.get("/instructions")
        return {
            "protocolVersion": asked if asked in PROTOCOLS else PROTOCOLS[0],
            "capabilities": {"tools": {"listChanged": False}},
            "serverInfo": {"name": "flag-plot-seat", "title": "Flag Plot, a watched seat", "version": "0"},
            "instructions": (instructions.get("ok") or {}).get("instructions", ""),
        }
    if method == "ping":
        return {}
    if method == "tools/list":
        tools = door.get("/tools")
        if "refused" in tools:
            raise RuntimeError(tools["refused"])
        return {"tools": tools["ok"]["tools"]}
    if method == "tools/call":
        tool = params.get("name")
        if not isinstance(tool, str):
            raise ValueError('"name" is needed: the tool to call')
        arguments = params.get("arguments") or {}
        if not isinstance(arguments, dict):
            raise ValueError('"arguments" must be an object')
        # The briefing is text unless asked for data, as `sim-cli mcp` has it.
        as_text = False
        if tool == "get_briefing" and arguments.get("text") in (None, True):
            arguments = dict(arguments, text=True)
            as_text = True
        text, is_error = text_of(door.call(tool, arguments), tool, arguments, as_text)
        return {"content": [{"type": "text", "text": text}], "isError": is_error}
    raise LookupError(f"Method not found: {method}")


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--url", required=True, help="the seat's door, as the table printed it")
    args = parser.parse_args()
    door = Door(args.url)
    out = sys.stdout
    for raw in sys.stdin:
        raw = raw.strip()
        if not raw:
            continue
        try:
            message = json.loads(raw)
        except ValueError:
            out.write(json.dumps({"jsonrpc": "2.0", "id": None, "error": {"code": -32700, "message": "Parse error"}}) + "\n")
            out.flush()
            continue
        if not isinstance(message, dict) or "id" not in message:
            continue  # a notification: nothing is answered
        reply = {"jsonrpc": "2.0", "id": message["id"]}
        try:
            reply["result"] = handle(door, message)
        except LookupError as error:
            reply["error"] = {"code": -32601, "message": str(error)}
        except ValueError as error:
            reply["error"] = {"code": -32602, "message": str(error)}
        except (urllib.error.URLError, OSError, RuntimeError) as error:
            reply["error"] = {"code": -32603, "message": f"the door did not answer: {error}"}
        out.write(json.dumps(reply) + "\n")
        out.flush()


if __name__ == "__main__":
    main()

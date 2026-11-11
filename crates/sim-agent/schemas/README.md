# Schemas of the agent interface

One JSON Schema file for each kind of answer the agent interface gives
(decision 0048). They are written by hand and checked, by a validator in the
tests of `crates/sim-agent`, against every answer the scenario tests produce.

Every answer is a JSON object with a top-level `"format": "flagplot-agent"`,
`"version": 1` and a `"kind"` that names it.

| Kind | File | Given by |
| --- | --- | --- |
| `briefing` | `briefing.schema.json` | `briefing` |
| `list_forces` | `list_forces.schema.json` | `list_forces` |
| `list_bases` | `list_bases.schema.json` | `list_bases` |
| `list_contacts` | `list_contacts.schema.json` | `list_contacts` |
| `entity` | `entity.schema.json` | `get_entity` |
| `explain` | `explain.schema.json` | `explain` |
| `not_known` | `not_known.schema.json` | `get_entity` and `explain`, for anything the side does not know of |

`defs.schema.json` holds what the files share: the decimal string, the unit
handle, the place and the one-line summaries of a task force, an air group, a
base and a contact. Files refer to it as `defs.schema.json#/$defs/name`.
`entity.schema.json` and `explain.schema.json` accept either their own answer
or `not_known`.

## How numbers are written

A count of things is a JSON integer. A quantity of the game (tons, fatigue,
miles that can be a decimal, shots) is a decimal string such as `"12"` or
`"0.25"`, exactly as the game holds it. There are no JSON floats anywhere.

## The version rule

`version` is 1. A field removed from an answer, or a field whose meaning
changes, makes the version 2. A field added does not change it, so a reader
must ignore fields it does not know. (The schemas here set
`additionalProperties: false` because they describe version 1 exactly; a
reader of a newer minor change should not.)

## What is not in an answer

An answer holds nothing of the enemy that the side may not know (decisions
0012 and 0047): an enemy unit appears only as a contact or, by the log's
number, in an event the side may see. Asked about anything it does not know of,
`get_entity` and `explain` give the same `not_known` document, which carries
nothing of the question.

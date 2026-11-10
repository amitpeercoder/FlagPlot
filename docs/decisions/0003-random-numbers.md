# 0003: Random numbers

Status: settled.

## The gap in the spec

The spec asks for a counter-based generator keyed on seed, day, pulse, subsystem and entity ID. It does not name an algorithm.

## Decision

- Each draw is a pure function of its key. There is no shared generator that advances as the day runs.
- The key fields are folded together with the SplitMix64 finaliser. Each field has its own tag constant, so equal values in different fields cannot cancel.
- A key opens a stream. The n-th draw of a stream mixes the key with n.
- The entity part of the key is the entity kind and ID together, so ship 3 and task force 3 get different streams.
- End-of-day phases belong to no pulse and use pulse code 0.
- `below(n)` uses rejection, so there is no bias toward low values.

## Why

SplitMix64 is small, fast, well studied and needs no dependency. The simulation needs reproducible, well-spread numbers. It does not need cryptographic strength.

## Consequences

- Processing entities in any order gives the same results. A test proves it.
- The generator's output is part of the save format. A test pins five output values. If it fails, saved games no longer replay, and the change needs a new decision record.
- A rule that needs several random numbers for one entity in one phase takes them from one stream, in a fixed order. A rule must not share a stream between entities.

# 0004: State hash

Status: settled.

## The gap in the spec

The spec asks for a state hash per day. It does not say which hash or what it covers.

## Decision

- 64-bit FNV-1a, written by hand in `sim-core`.
- The hash walks the state in a fixed order, field by field. It does not hash memory or a serialised form.
- It covers the seed, the day, all truth state, both belief states, the count of orders and events, and the ID counters.
- It does not cover names, reference data, the map or the contents of the event log.
- `sim-core` has no dependencies at all.

## Why

The hash exists to detect divergence between two runs. It is not a security feature, so a cryptographic hash is not needed. Writing it by hand keeps `sim-core` free of dependencies, which keeps the determinism rules easy to audit.

## Consequences

- A new state field must be added to `hash::hash_world` in the same change, or two different states could share a hash. Nothing enforces this yet.
- A 64-bit hash can collide by chance. For comparing two runs day by day, that risk is negligible. If hashes are ever used to identify states across many games, move to a wider hash.
- Reference data and the map are not hashed. A replay against changed data files will diverge without the hash of day 0 saying why. Hashing the data files into the save is a later step.

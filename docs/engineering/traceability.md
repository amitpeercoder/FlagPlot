# Traceability rules

What lets every outcome say what caused it. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

Every outcome must be able to say what caused it. This cannot be added later.

- **Every state change records an event.** If a rule changes truth state and writes no event, that is a bug.
- **Every event names its causes:** the orders and earlier events that led to it. Follow the pattern in `resolve.rs`: movement cites the standing orders, and fuel burn and arrival cite the movement event.
- **Every computed quantity or chance records a breakdown** of its factors when `SimConfig::record_breakdowns` is on. Breakdown recording must never change what happens or the state hash.
- **Every order carries its origin:** who issued it and the intent it serves.

## Adding a kind of event

An event kind is declared once and read in a few places that each ask a different question. Decision 0235 made the export's reading mechanical; the other places are decisions or English and stay by hand. The places, in the order to meet them:

1. **The declaration and its word, `crates/sim-core/src/events.rs`.** A variant goes inside `event_kinds!` with its word (the snake case of its name; a test holds this and that every word differs) and its fields: `NewKind = "new_kind" { ship: ShipId, hex: Hex },`. The macro makes the enum, `EventKind::tag()` (the word), `EventKind::KINDS` and `EventKind::fields()`, which reads each field by its type and names it by its own name. `rustfmt` does not format the body of the call, so keep the layout of the variants beside it. A kind added at the end of the declaration is the least to review.
2. **`EventKind::entities()`** in the same file: the entities the event is about, chosen by hand (the compiler fails until a kind is placed).
3. **`Display`** in the same file: the event's English, by hand. No text for a side is ever built from `fields()`.
4. **Who may see it, `crates/sim-core/src/side_events.rs`:** `seen` (whole, cut or not at all) and the functions beside it that match every kind and fail to compile until the kind is placed: `first_known_round`, `fight_round`, `count_from_known_round`, `whole_may_have_hidden_causes` and `project`. Decide it; there is no default (`docs/engineering/truth-and-belief.md`).
5. **The export's `data`, `crates/sim-cli/src/export/events.rs`.** If every field's type has a reading (below) and the JSON is to be exactly the fields in the order declared, written under their own names, write nothing here: `describe` reaches `_` and reads the fields by their types. If not, mark the kind `[by_hand]` in the declaration (`NewKind = "new_kind" [by_hand] { .. }`), write its arm in `describe` and add its word to `HAND_ARMS`. A mark with no arm, or an arm with no mark, fails a test (`hand_arms_are_the_marked_kinds`); a marked kind whose arm is missing panics with its word the first time an export meets it, which CI's export of every scenario does. The cut forms of a side's events are written by hand in `export/sides.rs`.
6. **What a measure counts, `crates/sim-cli/src/batch_record.rs`:** `count` names the kind, to count it or to say that nothing is counted. The compiler fails until it is named.
7. **What a side is told in its briefing, `crates/sim-agent/src/briefing.rs`:** the same question, asked of a kind a side is given, and the compiler fails until it is answered.
8. **Whatever else is a rule's:** a new kind comes with a rule, so the rulebook (`docs/rules/`), the glossary, the rule card and the tests of `docs/engineering/rule-complexity.md` and `traceability.md` above apply. The agent's name of the kind (`kind_name` in `sim-agent`) follows from the variant's name.

**The types that have a reading.** A field of one of these types is read by `fields()` and never written by hand; each has one reading wherever it is exported. Anything else needs `[by_hand]`.

| Type | Reading |
| --- | --- |
| `Fixed` | a quantity |
| `u32`, `u8` | a whole number |
| `bool` | a flag, always written (a kind that leaves a false flag out stays by hand) |
| `Hex` | `[q, r]` |
| the 14 ids with a public number (`ShipId`, `ShipClassId`, `AircraftTypeId`, `TaskForceId`, `AirGroupId`, `PilotId`, `BaseId`, `LandUnitId`, `SubmarineId`, `SubmarineClassId`, `SubmarineTorpedoId`, `IntentId`, `FormationId`, `InterruptId`), by one impl over `EntityId`, so a new such id reads with no line | its number |
| `ContactId`, `OrderId`, `EventId` | its number, read with the umpire's key (decision 0086) |
| `PlanId` | its number |
| `Side` | `allied` or `japanese` |
| `Vec<Hex>` | a list of `[q, r]` |
| `Vec<T>` of those ids | a list of their numbers |
| `Option<T>` of any type above or below | the value, or nothing, so the key is left out |
| `EntityRef` | its `Display` (`ship:3`) |
| words of the type's own `name()`: `AirFirer`, `AirTarget`, `CommandKind`, `ExperienceSource`, `FieldworksChange`, `FuelOrderEnd`, `GroundGrade`, `Job`, `MarchStop`, `PlanState`, `ShipKind`, `SortieKind`, `SubmarineLeftReason` | the word |
| words written in `sim-core`'s `field_value.rs`: `Weapon`, `DamageTrack`, `EngagementEndReason`, `DetachReason`, `ShipSystem`, `SystemState`, `LoadingLoss`, `RehearsalLoss`, `RosterLeft`, `ForceFatigueChange`, `LandFatigueChange`, `EntryWait` | the word |

A new reading is added in `crates/sim-core/src/field_value.rs` only if the type is exported the same way in every kind that holds it. Where `sim-cli`'s `export/names.rs` has a word for the same type, it calls the one in `sim-core`. When a kind is changed to be read by type, its arm goes only if the export is byte for byte what it was and the arm has no branch beyond the type's own reading (decision 0235, ruling 2). `fields()` asks for the `UmpireKey`, so it is the umpire's: nothing that plays can call it.

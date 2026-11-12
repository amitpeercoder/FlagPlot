# 0062: What `batch` records for each run

Status: settled by the architect. It adds no rule and changes no state, hash, event or random draw, and nothing a player sees. The batch file is the umpire's.

## The gap

Step 2 judges a measure over runs: the record must lie inside the middle 90 percent of 1,000 runs and hold at least 1 run in 20 (`docs/step2-spec.md`, "What counts as agreement"). So every count a measure reads must be there for each run, not summed over them. Until now `batch` recorded ships sunk, the result and the final hash for each run, and the spec's "The harness" says it is to record more: "aircraft lost by side and cause, hits by ship, sorties flown, supply used, and the round a force broke off". `sim-cli validate` will read that file and the measures under `data/validation/`. This decision says what is recorded and from which events, so that every measure that a run can serve is served.

## Decisions

1. **The batch file is at version 2.** Every new key is always written, empty when there is nothing, so that a reader that needs the new keys can refuse a version 1 file cleanly, and a run with none is told from an old file. Every old key and its value is unchanged; new keys are added after them. The existing test's assert of `version == 1` is changed to 2. (Decision 0057, item 6, kept version 1 for additions nothing needed; here a reader needs them.)
2. **Header tables**, written once from the state before day 1: `task_forces` `[{id, name, side}]`; `ships[].task_force`, the force the ship starts in (null for a ship in port); `air_groups` `[{id, name, side, aircraft_type}]` with the type's name; `bases` `[{id, name, side}]`.
3. **Per run**, all from the event log and the end state:

| Field | Shape | Counted from |
| --- | --- | --- |
| `hits_on` | `[{ship, gun, torpedo, bomb}]`, ships with at least one hit | `Salvo.hits` by `target` and `weapon` (gun, torpedo); `AirAttack.hits` by `ship` and `weapon`. Hits as rolled |
| `fire` | `{japanese, allied}`, each `{gun:{shots, hits}, torpedo:{shots, hits}}` | `Salvo.shots` and `.hits` by the firer's side, surface salvos only. `shots` is the exact sum as a decimal string |
| `systems_out` | `[{ship, system}]`; `system` is one of `propulsion`, `fire_control`, `main_battery`, `torpedo_mounts`, `flight_deck` | End state: every system `Out` of a ship not sunk |
| `air_losses` | `[{air_group, shot_down, destroyed_on_ground, damaged_destroyed_on_ground, lost_with_ship, worn_out, ditched_fuel, ditched_no_deck, pilots_lost}]`, groups with any non-zero value | `AircraftLost.aircraft`; `AircraftDestroyedOnGround.aircraft`; `DamagedAircraftDestroyedOnGround.aircraft`; `AirGroupLost.aircraft`; `AircraftWorn.written_off`; `AircraftDitched.ditched`; `AircraftDitchedNoDeck.aircraft`; `PilotsLost.lost` |
| `ready` | `[{air_group, first_day, by_day}]`, a row for every group that exists at the end. `first_day` is always 1: `by_day[0]` is day 1 and `by_day[i]` is day `i + 1`. 0 before a formed group exists | The state after each day's last pulse |
| `claims` | `{japanese, allied}`, each `{sunk, hits, shot_down, on_ground}` | `ClaimReceived` by `side` |
| `attacks` | `[{day, pulse, air_group, target_kind, target, weapon?, aircraft}]`, one for each attack | `AirAttack` (target a ship, `weapon` gun, torpedo or bomb, `aircraft` from `shots`) and `BaseAttacked` (target a base, `aircraft` from `shots`) |
| `escorts` | `[{day, pulse, air_group, with_air_group, aircraft}]` | `EscortsSent` |
| `broke_off` | `[{day, pulse, task_force, side, round}]`, every one, in log order | `ForceBrokeOff`; the side is the force's |
| `fights` | `[{day, pulse, rounds}]`, one for each surface fight, in log order | `EngagementEnded`: `round` is the round the fight ended in, which is the number of rounds fought |
| `supply` | `[{base, sortie_aircraft, sorties_t, upkeep_t, repairs_t}]` | `BaseSupplySpent`: `aircraft` where the reason is sorties, `paid_t` by reason, as decimal strings |
| `formed_groups` | `[{id, name, side, aircraft_type, from}]` | Groups at the end that were not at the start; `from` is the source group of the `AircraftLandedElsewhere` with `formed` that made it |

A formed group's losses and ready counts are recorded under its own id; `batch` does not attribute them back to its source.

4. **"End of day N"** is the state after that day's last pulse. Repair, arrivals and replacements have been applied at the start of that day's night pulse, the first pulse of the day.
5. **`crippled_runs` is unchanged:** it still counts flotation damage at or over the scenario's level. The measures' "propulsion out" and "flight deck out" are `systems_out`.
6. **The run loop goes day by day** (`execute_observing`, a day's state handed to a closure), to read `ready`. It makes the same days in the same order, so the run, its events and every hash are those of `execute`. A test shows it against running to the end and against `run --seed`.
7. **Breakdowns stay off in batch.** Nothing under `crates/sim-core/src` changes. `batch` does not refuse a run without `--label`.

## Counting conventions, checked against the code

- **One `PilotsLost` for every pilot loss:** aircraft lost away from home, ditching, a lost deck, a ship sunk. The `pilots_lost` field of `AircraftDitched` and `AircraftDitchedNoDeck` is the same pilots, so `pilots_lost` here reads `PilotsLost` only.
- **Six events lose an aircraft for good**, as `objectives::tallies` counts them: `AircraftLost`, `AirGroupLost`, `AircraftDestroyedOnGround`, `DamagedAircraftDestroyedOnGround`, `AircraftWorn.written_off`, and `AircraftDitched` with `AircraftDitchedNoDeck`. A test checks the sum of the nine fields of `air_losses` by side against `tallies`.
- **The seven aircraft keys are disjoint,** so their sum for a group is every aircraft it lost for good, each once. Checked in `airstrike.rs`, `upkeep.rs`, `landing.rs` and `damage.rs`: each loss takes the aircraft out of `ready` or `damaged` as it is counted (`lose_aircraft`; ground hits, ready or damaged; `ditched`; wear's write-off; no deck), so none can be counted by a later event. `damaged_destroyed_on_ground` is apart from `destroyed_on_ground`: one takes from `damaged`, the other from `ready`, in two events. A ship's sinking takes `ready` less the aircraft in the air, plus all `damaged`, as `lost_with_ship`, and only that; the aircraft in the air stay in `ready` and end as a landing elsewhere (no loss) or a ditching with no deck. Aircraft worn but not written off go to `damaged` and are not counted until they are lost another way. A test sums the seven by side against the objectives tally on scenarios 05, 06 and 12c.
- **A fight's rounds.** Every fight, however it ends (a force sunk, a break-off, slipping away, passing unseen, the last round), records one `EngagementEnded` whose `round` is the round it ended in. In a fight that ends by slipping away the last round is counted, though no shot is fired in it.
- **A landing elsewhere is no loss.** `AircraftLandedElsewhere` is not counted; it shows in `ready`.
- **Shots can be fractional.** A `Salvo`'s `shots` is a `Fixed` (a damaged battery, the ammunition cap), so the sum is written exactly. `AirAttack.shots` and `BaseAttacked.shots` are whole numbers, one for each aircraft attacking.
- **A claim's side is the claimant.**
- **Hits are counted as rolled.** A salvo is rolled on the ships afloat at the start of the round, and an air attack on the state at the start of the step, so hits are rolled at a ship that another salvo of the same round sinks. Over 200 seeds of Savo Island, `ShipDamaged` events on an already-sunk ship carried 4,322 of 43,112 hits (10 percent); in the other scenarios tried (4a, 5, 5d, 7) 0.5 to 8 percent. No salvo or attack is aimed at a ship sunk in an earlier round or pulse. The count is the one the rules record; whether a measure should discount it is for the measures.
- **An escort entry names the raid's first strike group.** `EscortsSent.with_air_group` is the first group of the raid the fighters went with, not every group in it. To add a raid's escorts to its attackers, match an escort entry to the attacks of the same day and pulse by that group. If that first group has no bombers left when the raid attacks, it records no attack and the escorts match nothing; that is rare and is left as it is.

## Left out, and why

- **Crews recovered.** No measure counts them; `pilots_lost` carries what they came to.
- **Shots by ship,** and gun hits by calibre. The measures read per-side fire and per-ship hits.
- **Who sighted whom first, and whether a side "found" the other.** No measure in the files reads them; Cape Esperance's is still to be written.
- **Days a field could not fly.** The spec's table lists it; no measure file has it.
- **A carrier group's sortie count.** `supply` holds the aircraft sent on sorties from bases only.
- **Aircraft that landed elsewhere.**

## Passages are not from batch

`data/validation/passages.toml` and the spec say a passage is checked with one calculation, not a thousand runs: the sea route, the miles the rules charge and the minutes at the stated speed. No run is read.

## Which fields a measure sums is the measures file's

The batch gives the pieces. Which of them a measure adds, and what a measure's words mean, belong to the spec session. Six questions were sent to it on Nov 12, 2026: what "lost to enemy action" and "forced down" hold (a combat write-off has no rule; wear's `written_off` is recorded as `worn_out`); what counts as "in the raid" (aircraft launched or attacking, escorts, a raid of ten); naming air groups by id in the measures; claimed hits on one ship (a claim names no ship); the label; and fractional shots. Until they are answered `validate` is to take no meaning from the batch's field names that the measures file has not given.

## Rule card

None. This adds no rule: it reads the log and the end state and counts.

## What it costs

- A larger file. Measured in release, 1,000 runs: Savo Island 1.66 MB in 7.5 s, Rabaul against Guadalcanal 5.6 MB in 1.7 s. A batch of Savo Island was about 0.1 MB before.
- A batch from before this decision is version 1 and has no counts; `validate` refuses it.

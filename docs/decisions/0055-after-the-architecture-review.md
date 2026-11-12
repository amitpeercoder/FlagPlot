# 0055: After the architecture review

Status: decisions 1 to 4 are confirmed, made by the designer on Nov 12, 2026. Decisions 5 to 9 are settled by the architect. The pillar wording at the end is proposed and needs review; `docs/pillars.md` is not changed.

## The review

On Nov 12, 2026 the architecture was reviewed against a target wider than the slice: a grand strategic game, played by a person or an AI agent, against the engine or against another person or agent, with an economy, command and control, and storytelling. The designer set that target as a long-term direction, not the next milestone. So the question was what the code makes expensive later, not what is missing now.

Five reviewers read the code, each from one of those aims, without the architect's reasoning in front of them. The architect checked their main findings against the code. This record holds the decisions that came of it and the four changes made at once.

## The designer's decisions

1. **The day is the smallest command cycle.** A player gives orders between days and cannot interrupt or give orders inside one. That stands for the distance between a commander and his forces. What happens inside a day is decided by the standing orders a unit already has.
2. **A saved game is not promised to survive a change of rules.** Promising it would limit what the rules may become.
3. **A side knows its own units exactly.** There is no fog over a side's own forces. Decision 1 already keeps the player at a distance: he learns what his forces did when the day is over.
4. **Play between people is left for later.** What must work is an agent against an agent, an agent against a person, and either against the engine, all run by one host that holds the truth.

## What the architect settled

5. **A player is shown his own count of orders, not the log's number.** The log numbers both sides' orders in one run, Allied first each day, so the Japanese side's numbers moved with how many orders the Allies gave. `sim-agent` now numbers a side's own orders 1, 2, 3... in the order they were taken in, as it already does for events (decision 0048), and the log's numbers never leave the crate. `SideView::orders` gives a side its own orders. The log, the state and the hash are unchanged.
6. **An order a rule gives inside the day is issued by the local commander, not the staff.** `Issuer::LocalCommander` is added at the end. The straggler rule (decision 0016) uses it. `Staff` is kept for the planner of step 3, so that "who decided this" can tell the planner's choice from a standing order taking effect. A scenario file still scripts orders as the player's or the staff's only. The issuer is not hashed.
7. **A random number key names the rule that draws.** `RngKey` gains `draw`, a value of the new `Draw` enum, and `World::rng` and `World::rng_pair` take it. A rule added from now on adds its own value with its own code, at the end. The eleven draws that exist are `Draw::Legacy`, code 0, which adds nothing to the key: their streams, and every pinned number, are what they were. No new rule may use `Legacy`.
8. **Every field of the state is named beside the hash.** `hash::name_every_field` takes each state struct apart with no `..`. A field added to the state does not compile until it is named there, with a comment if it is not hashed. It changes no hash.
9. **The next change to the layout of the hash raises its version.** Nine tag bytes exist only so that old states hash as they did. That is not continued: the next field that cannot be added without a tag raises `flagplot-state-v1` to `v2`, drops the tags, and the golden files are remade in the same change. It is not done now because a stage is in flight on another branch and every golden file would conflict.

## Why

- **Decision 5.** The leak test compared a side's answers only when its order numbers matched in both worlds, and counted the rest as not applicable. That hid this case. The test now compares orders by the side's own count, and has a case where the enemy gives one more order on day 1.
- **Decision 6.** It costs a line now. Once saved games and explanations rely on "staff" meaning both, it costs a format change.
- **Decision 7.** Two rules in one subsystem that draw for the same pair of entities in the same pulse would share a stream, and nothing would say so. Tagging the existing draws would move every outcome and so every hand calculation, for no gain: they do not collide today. Tagging only new ones costs nothing.
- **Decision 8.** A forgotten field means two different states with one hash. Nothing checked for it.

The eleven legacy draws are told apart by the shape of their keys:

| Rule | Subsystem | Entity | Second entity |
| --- | --- | --- | --- |
| Search sighting | Search | air group | task force |
| Escorts fire at the patrol | Air combat | escort air group | target task force or base |
| Patrol fires | Air combat | patrol air group | raid's air group |
| Anti-aircraft fire | Air combat | ship or base | raid's air group |
| Air attack on a ship | Air combat | striking air group | task force |
| Air attack on a base | Air combat | striking air group | base |
| Aircraft wear | Upkeep | air group | none |
| Alarm roll | Surface combat | task force | none |
| Sighting in a fight | Surface combat | task force | task force |
| A ship's fire | Surface combat | ship | task force |
| Sighting report | Intelligence | task force | task force |

Escort fire and an air attack have the same shape. They do not meet because escorts come from a group on combat air patrol and an attack from a group on a strike mission, and a group has one mission. A rule that lets one group do both in a pulse would end that, and whoever adds it moves escort fire to its own `Draw`.

## What it costs

- **Decision 1** makes standing orders the player's only lever inside a day. The choices a rule makes there (which contact to strike, when to break off, what a straggler does) carry more weight, and several are written into rule code where no order reaches them. See "Left for later".
- **Decision 2** means a game in progress may stop replaying after a rule change. Today it fails with a hash that does not match, partway through.
- **Decision 5** changes the order numbers in what a player is shown. No recorded game in the repository changed.
- **Decision 7** leaves a rule that is enforced by review, not by the compiler: nothing stops a new rule from passing `Legacy`.

## Left for later

Found by the review and not acted on here. None blocks step 1.

- **Before step 3 (the staff planner).** Pull the choices rules make inside a day into functions of belief and standing orders, so the planner and an enemy AI can ask what a commander will do. Make strike target priority an order: today it is the order the contact kinds were declared in. Give a saved game a rules version, so an old one is refused at once with a reason (follows from decision 2). Decide how an enemy ship is named to the side that fights it: today by its true number (decision 0047).
- **With decision 4.** `sim-cli` gives only one side a player. Agent against agent on one host needs both seats open.
- **For the long-term target.** Ships, bases and air groups cannot be made during a game. Ports and airfields are on the map, which is not state, so they cannot be built or bombed down. Each resource is its own set of fields. A ship class has a number local to its scenario and no refit over time. The event log grows without limit and the state points into it.
- **Housekeeping.** Constructors for test fixtures, so a new state field does not touch every struct literal (48 for `TaskForce` alone). Left until the stage in flight has merged.

## Proposed wording for the pillars

Decision 1 fits the pillars as written, but two lines can be read as stopping the clock inside a day. Proposed, for the designer to accept or change:

- Pillar 3: after "The simulation resolves in days" add "and a day, once begun, runs to its end".
- Interrupt: "the player defines what stops the clock between days, such as...".

# 0232, as built: A plan the commander writes

## Built: step 2 (Nov 27, 2026, pull request 748)

Rules 20 to 26, in the tools alone: `preview` takes `under`; `approve` and `take_back` treat an order and its written part as one; `trial` reads the staff's own plan as the second trial for a written part and refuses `dates` for one. Agent interface 57. `sim-core` (but for one test), `sim-staff`, `sim-data` and `sim-trial` are unchanged: rules label 35, saved game 29, doctrine `supply-63` and state layout v60 stand, and no game moved. The recorded transcripts differ in the interface version alone. Scenario 137b's 25 tests are filled to the architect's statements.

## The architect's rulings on pull request 748 (Nov 27, 2026)

1. **A window through a preview of a change** was still offered for a queued landing he wrote and for a queued order with landings under it. Rule 26 refuses both; the tool now reads the parent from the side's command state where the draft gives none, and statement 18 gains the two cases.
2. **The invariant's case owed from pull request 739,** a commander's node beneath a staff's intent, is added to `sim-core`'s test.
3. **The seat's number of a task force.** The statements wrote the game's task force 3 where a seat gives its own number, 2. The build read it rightly, and the doc comment now says so.
4. Everything else is as the record and the statements have it.

## Amended (Nov 27, 2026): step 2 merged before stage 4's proof

The record held step 2 until the proof was run. On the designer's word of Nov 27, 2026 it was merged first. The record's Status, Standing and "Where it stands" say so, and `docs/validation/stage-4-proof-setup.md` is amended a second time, before any run, on its own comparison.

## Step 3: the statements, and what writing them ruled (Nov 27, 2026)

The statements of rules 27 to 31 are written by the architect before any code of step 3, in the form the table's checks take: the doc comment of `table/check_plan.py` (21 statements, with the words fixed there), which `table/check_screen.py` runs, and statements 2, 6 and 7 of `tests/table_rear_bases_files.rs` for the island. The page's model of a written plan is checked in node against values the architect set, and what the page will send was put to the engine through the bridge on a throwaway copy of the table's file with the island: it holds. Step 3 is class 2 (a change to what the table gives, the rules as they are), built on the designer's answer 2; the architect reads its pull request.

**Versions.** None moves: rules label 35, saved game 29, doctrine `supply-63`, agent interface 57 and state layout v60 stand. No crate changes. No game moves: the three files of `data/scenarios/table-rear-bases/` play as before, line for line with the hash lines kept (run by the architect on two of the three; the build shows all three). The files' own hashes change, so a game kept from before is not taken up on the new files.

**Checked before writing.** The table's 39 days offer "Seize and hold Guadalcanal" and "Seize and hold Tulagi" today, from each place's card (`ends()` in the page, from the islands of `table/scenarios.json`). `preview` with `under` and the trial's second trial reach a seat at the table whole: the bridge and `seat_mcp.py` pass a tool's arguments and its answer as they are (read in both, and run through the person's bridge). The page itself sends no `under` and reads no `left_to_the_staff` yet, which is step 3.

**Ruled, where rules 27 to 31 were silent or did not fit what is there.** No rule of the game moves.

1. **An island of more than one place in the table's list.** An island's entry in `table/scenarios.json` gains `places`, its bases in order, the first its `base`; `serve.py --island` takes `N=Name@1,4`. The page offered an island only from the card of its one `base`.
2. **The third file had no island.** `07_full_slice_submarines.toml`, the game of a person against an agent and of two agents, has no island, though the list offers its islands in every game of the 39 days: "Seize and hold Guadalcanal" is refused there today ("island 1 is not an island of the scenario"). The script gives that file islands 1 and 2 as the other two have them, and the third. It is the table's copy, made to say what the table's list says; no file of record changes. Told to the designer with the merge.
3. **A landing's lines** are rule 27's list and no more: task forces, land units, the base it sails from, not before, wait for, after, what must be known, the risk, the date. Rule 27 called them "the lines a take offers today", which is not so of two things. A take's paper also offers air groups, keep within, strike first and "none" of a kind, which nothing reads on a take he wrote under an island (rule 10) and which are not offered. And no paper offers a date today: the date is the landing's own `by_day`, shown as "Done by", offered on a landing alone, and not changed after signing in this build (a date is the landing's `set_intent` given again).
4. **The order the landings are sent in** is the order he wrote them, but that a landing another waits on goes before the one that waits: an "after" may name only an earlier command of the batch (rule 20). The page gives the order and each landing its number, from the number the staff's first reading of the draft gave, so that an "after" names what it means and a stale number is a refusal and never a wait on another order.
5. **A landing written under an order that stands** (rule 22) is offered on the plan's paper while the island's plan is a draft or preparing, on each take the staff would form. Rules 27 to 31 did not place it, and the designer's answer 3 promised it ("before you give the island's plan its leave"). On the table's 39 days leave to launch is withheld at the start, so this is the usual way in.
6. **GIVE IT BACK TO THE STAFF has no staff's reading.** Rule 30 says it is "read before signing as any command is"; no tool reads a withdrawal before it is given (`preview` takes a `set_intent` or a `set_limit`). The paper itself is the reading: what is given back, when it acts, a dry run beside the landing kept where the game has a trial, then the signature. A preview of a withdrawal is left out.
7. **SIGNED said a landing's limits as the order's.** The queue's `goes_with` of an order now holds its landings and their limits; the page's line took every limit among them for the order's own. Statement 13 mends it.
8. **Rule 31 reads the engine's words in one place:** "intent:K" where K is a landing he wrote is said "your landing at" its place, as a link, by the readings in statement 21. Everything else the page learns of whose a node is, it reads from `maker`, `parent` and `standing`.
9. **The staff's reading, the mark and the refusal** are in the statements: the reading ends by saying it is worked with his plan; every task is marked YOURS or THE STAFF'S; a refusal is said at the landing and the line the engine's place in the batch points to.

**Found, for the manager.**

- **The table's 39 days have no dry run.** None of the three files names a trial's figures, so `trial` is not offered there and the page shows no DRY RUN, on any order. Rule 29 can be built and checked in node, but not seen on the one game that has two landings (below).
- On a throwaway copy that names `data/reference/trial.toml`, a draft with both landings written, tried beside the staff's plan: the small size (8 runs of 7 days, twice) answered in about 5 s and the standard (16 of 14, twice) in 228 s, in a container of 4 cores; the times the designer accepted have 6.4 s for a standard trial beside another on the named machine (`docs/validation/stage-4-proof-setup.md`, condition 10), measured on another file of the 39 days, so the time is to be measured there on the table's file before it is offered. In both, the island's end was met in no run by either plan.
- `approve`'s notes still say of an order "the staff gives no order for it until you set one (set_limit forces)", which is not so. The page shows no notes; a lead for the tools.

## Asked of the designer (Nov 27, 2026)

1. **Shall the table's 39 days offer a dry run?** Today they offer none: the files name no trial's figures. Step 3's two columns, "Your plan" and "The staff's plan", need one, and the 39 days are the only game on the table with two landings to write. The architect recommends yes: the three files of the table's 39 days name `data/reference/trial.toml`, the placeholder figures the table's three other battles name, once the times are measured on your machine. Two things to know first. A dry run then shows on every order of the 39 days, not only on a written plan. And in the tries made, over 7 and over 14 days, the island was won in no run by either plan, so the first row of the two columns will often read "no run" twice and the difference will be in the costs.

   On a yes the script writes `trial = "../../reference/trial.toml"` after the files' `class_sides` line, as its seventh change, statement 2 of `tests/table_rear_bases_files.rs` gains that one line, and `the_dry_run_beside` in `table/check_plan.py`, which is written, runs. On a no, rule 29 is built and held by the node check alone.

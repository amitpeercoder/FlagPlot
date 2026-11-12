# Design note: intelligence

Status: a note for discussion, written by the architect on Nov 12, 2026 at the designer's request. Nothing here is built. The four questions it first left open were answered the same day; the answers are below. It says what intelligence could be in Flag Plot, what it rests on that exists, and in what order it could arrive. Historical statements below are general and unsourced; any figure the game uses must come from sourced facts, as everywhere.

## Why it belongs in the game

The Pacific War was fought on what each side believed. Commanders acted on sightings that were wrong, claims that were inflated, and now and then on a decoded signal that told them where the enemy would be. A simulation of the war without intelligence would give both sides either too little, a war fought blind, or too much, a war fought on the truth.

The pillars already ask for this. The player commands on belief and the staff explains what it knows and how it knows it.

## What exists to build on

- **Belief is written only from reports.** `BeliefState::apply_report` is the one door. A side's own units are known exactly; the enemy reaches it only as reports. This rule is what every tool for a player rests on (decisions 0007, 0047).
- **Two kinds of report so far:** a sighting from search, which can be wrong about kind and number (decision 0012), and a battle claim, which can overstate (decisions 0014, 0056).
- **Contacts age and are dropped.** A strike that finds nothing drops its contact (decision 0028).
- **Every report is an event with causes,** so a side can ask why it believes something and be shown the report.
- **Contact confidence was cut** after scenario 5 because nothing read it (decision 0032). It is the natural thing to bring back when several sources can report the same thing.

## The idea in one line

Intelligence is more kinds of report through the same door, each from a named source with its own delay, reliability and reach, and belief that grows firmer when independent sources agree.

## Three layers

### Layer 1: assessment after the fact

What a side believes it did gets checked against what it later sees.

- A ship claimed sunk and then sighted again is no longer believed sunk.
- A reconnaissance flight over a base counts the aircraft on the field after a raid, and the claim of aircraft destroyed is set against it.
- Slow, reliable confirmation: months later a loss is admitted or learned of, and an old claim is confirmed or struck off.

Needs: claims (being built). Adds: a claim that can be revised by a later report, and a record of what a side believed on each day, so that "what did I think on the 24th" stays answerable.

### Layer 2: standing sources

Sources that are not the side's own searching aircraft and ships.

| Source | What it gives | Shape in the game |
| --- | --- | --- |
| Coastwatchers | Ships and aircraft passing a fixed place, often hours before they arrive | A searcher that does not move, with a small radius and a delay before its report arrives |
| Reconnaissance of harbours and airfields | What lies at a base: ships in port, aircraft on the field | A mission for an air group, which reports a base's contents with error |
| Radio direction finding | That a force is at sea in an area, with a poor position | A sighting with a wide error in place and no count |
| Submarines on patrol | Sightings off enemy bases | Arrives with submarines |

Needs: nothing new in the rules of belief. Each source is data: where it is, what it can see, how often it is right, how late it reports. Adds: a `source` on every report, shown to the player, and the return of confidence.

### Layer 3: reading intentions

Traffic analysis and codebreaking do not say where a ship is. They say what the enemy means to do: a convoy is to reach a place around a date, the carriers have sailed, an operation is set for a day.

- This is a new kind of belief: an **estimate**, a statement about the enemy's plan with a date, a place, a subject and a firmness. It is not a contact.
- It can only exist once the enemy's plans exist as objects to be read. That is the staff planner of step 3, which serves both sides. A scripted enemy has no plan to intercept.
- An estimate is made from the enemy's plan through a filter: part of it, late, with error, and sometimes nothing. The filter is figures for each side and each period.
- An estimate is evidence for the staff. "Signals intelligence expects a convoy at Guadalcanal around the 24th" is the reason the staff proposes a search arc to the north on the 23rd.

## Belief that grows firmer

- Each report carries its source. Reports of the same thing from independent sources raise a contact's or an estimate's confidence; a report that contradicts lowers it.
- Confidence must be read by a rule to earn its place (the lesson of decision 0032). Candidates: the staff's strike decision asks for more confidence before it commits a carrier's strike; the briefing orders what it shows by it.
- A side never sees how right a source truly is. It sees the source's name and its record so far, as the side itself has experienced it.

## Each side's level

- The figures for a source differ by side and by period, and come from the record. The American advantage in signals intelligence was real and uneven; the Japanese advantage in night observation was real too. What each was worth in a given month is research for `data/oob`, not something to assert here.
- A scenario states which sources each side has.

## Rules it must keep

- **No peek at the truth.** Intelligence arrives as a report with a source and a date. A decoded signal that hands over an enemy force's true position, id or strength would break the rule the agent interface rests on. The leak tests of decision 0047 should cover every new source: two worlds that differ only in what no source of the side could see give the same belief.
- **Deterministic.** Whether a source reports, when, and how wrong, comes from keyed draws like every other chance.
- **Traceable.** An estimate names the reports behind it. The enemy plan it was drawn from is truth and is never shown.
- **No clerical work.** The player says what they want watched; the staff tasks reconnaissance and reads the traffic, and briefs the result.

## What to learn from War in the Pacific first

Before any of this is planned, read how War in the Pacific: Admiral's Edition modelled intelligence (its signals intelligence reports and detection levels) through `docs/research/witpae-manual-index.md`, and note in our own words what worked and where it buried the player in text. Mechanics and lessons only, no numbers.

## Answers from the product managers

Agreed by the designer on Nov 12, 2026, on the architect's suggestion. These are directions for the design, not built rules; each becomes a decision record when its layer is planned.

1. **A source does not lie outright, at first.** A report can be incomplete, late and mistaken, and that is enough to make intelligence uncertain. Planted information (false traffic, a decoy meant to be seen) is deception: a named later feature of its own, because it needs orders to deceive, a staff and an opponent able to use them, and a way to explain afterwards what must stay hidden during play. Every report carries its source from the start, so a planted report later is a report whose source was compromised.
2. **A side can make itself harder to read by one order: radio silence.** A force keeping silence gives direction finding and traffic analysis nothing. The price is that its own sightings reach its side late and it cannot be redirected until it breaks silence. It has the shape of the readiness order (decision 0037): one setting on a force, with a benefit and a cost. It arrives with the standing sources (layer 2). A side is never shown how well the enemy is reading it.
3. **Late confirmation is shown twice.** During the game it arrives as a report like any other, with a source and a date, and revises the earlier claim, so a long campaign is not fought on a belief that can never be corrected. After the game the history shows everything: what happened beside what each side believed on each day. How late and how complete the confirmation is are figures by side and period, and may be "never, within this scenario".
4. **One picture, with the sources one step down.** The staff presents a single intelligence picture. Any item opens to show the reports behind it: which source, when, what each said. Where sources disagree the picture says so and does not average. A source's record so far, as this side has learned it, is shown beside its name. There is no screen or tool for each source.

## A possible order

1. Layer 1, with claims, in or just after step 1: claims revised by later sightings.
2. Layer 2 in step 2: coastwatchers and reconnaissance change how the historical battles open, so they belong with the tuning against the record.
3. Layer 3 with step 3, designed together with the staff planner so that a plan is something an enemy can partly read.

# 0104: A beaten side falls back

Status: a plan by the architect, written on Nov 14, 2026. The designer answered its two questions on Nov 14, 2026: the men fall back by marching (decision 1), and men with nowhere to go surrender (decision 7, his choice against the architect's proposal to leave surrender out). Confirmed. Nothing is built. It adds one placeholder figure.

## The gap

A side beaten in a hex loses its bases there, and its men stay where they are, unfed (decision 0090, 17). They take a share of the fire aimed at the hex, eat nothing and waste by the day beside the men who beat them. The garrison of Lunga left the airfield on 7 August and went west. Decision 0101 lists the garrison staying as a thing the lengthened slice will show wrong.

## Decisions

1. **A beaten side's units fall back, if there is somewhere to go.** When a side is beaten in a hex, each of its units with men ashore there, fighting or working, is set to march out of it, by the marching rule that exists (decision 0094). The march takes the days any march takes.
2. **Where to.** Of the hexes linked to this one by land, in hex order: the first that holds a base of the unit's side; if none does, the first that holds no enemy man of a fighting unit; if none, the unit stays, as it does today.
3. **It is a march like any other,** but for three things. It has no order: its cause is the event that says the side was beaten. The unit takes no supply with it, since its side has just lost its bases in the hex. And it goes on the defence.
4. **The fatigue of defeat.** A unit that falls back gains `beaten_fatigue` at once, up to 100, beside the fatigue of each day's march.
5. **The side may countermand it.** From the next day the unit takes orders as any unit does: a new march order replaces the march, and it may be set to attack again where it stands.
6. **While it is still in the hex it can be fought.** The hex is contested, as always, when a unit there is on attack. A victor who goes on attacking fires on men who have not yet left.
7. **Men with nowhere to go surrender.** A unit of a beaten side that decision 2 gives no hex to, on an island of one hex or with the enemy in every hex beside it, surrenders in the same phase: every man ashore is lost and the unit is destroyed if it has none aboard. The men count as men lost on the objectives scale (decision 0090), as men lost in the fight do. *The designer's choice, Nov 14, 2026. The architect had proposed that they stay and starve, as today.* It follows that a side that means to take its men off by sea must do it before it is beaten.
8. **What a side knows.** The side sees its own units' marches whole, and that they fall back and why. The enemy is told nothing of a falling back: it learns the men have gone as it learns anything of the enemy ashore, by its own men's report of the hex. A surrender is told to both sides, with the men: the side that takes prisoners counts them.

## Placeholder figure

| File | Key | Value | What it is |
| --- | --- | --- | --- |
| `ground.toml` | `beaten_fatigue` | `30` | Fatigue a unit gains when its side is beaten and it falls back |

It rests on no source. In the frozen step 1 figures it is `30` too; no frozen scenario has a land link out of a hex where a side is beaten, so no earlier result changes. That is checked when it is built, and a scenario that does change goes back to the architect.

## What it changes in the August scenario

The naval guard troops and the two construction units at Lunga, beaten on the first or second day, march to (2,-5), which comes first in hex order and holds the Japanese base decision 0101 states there. They arrive three days later, on the placeholder march, and are fed there. Tulagi's hex has no land link: its garrison and its construction unit surrender on the day they are beaten. In the record nearly all of them died fighting and a few were taken; either way they are gone from the hex.

## Why

- **It uses the march that exists.** No man is moved by a rule that is not already in the rulebook.
- **A defeat becomes something a side lives through,** with men to gather and feed, and something the victor may press.
- **It removes men who do nothing from the hex,** where they soaked up fire.

## What it leaves out

- **Prisoners as anything but men lost.**
- **Taking beaten men off by sea.**
- **A retreat by sea.** Men are taken off only by order, as today.
- **A choice of where to fall back to,** beyond a new order the next day.
- **Disorder beyond fatigue.**

## What it costs

- **No new state.** A march without an order is the one change to the march's state, in the hash and named beside it; the hash version is raised and the generated files remade.
- **Two new events** with their rows in `side_events.rs`.
- **The rulebook:** chapter 7 ("A side beaten", "Marching overland") and the glossary, in the same pull request.

## Rule card

| | |
| --- | --- |
| Reads | The side beaten and the hex; its units' men ashore; the land links; the bases and the enemy's fighting men in the linked hexes; the figure |
| Writes | A unit's march, posture and fatigue; the events |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 1:** the men fall back by marching. Agreed.
2. **Decision 7:** the designer chose a rule of surrender now, for men with nowhere to go.

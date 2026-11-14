# 0031: The Eastern Solomons scenario: what is in it and what is left out

Status: confirmed. The designer approved the forces with the plan for stage 5e on Nov 11, 2026. The positions, air groups and orders are data and scenario inputs, settled by the architect.

## Decisions

1. **One day: 18:00 on 23 August to 18:00 on 24 August.** The 25th is left out with the convoy it was fought over.
2. **Four forces and one base.** Task Force 16 (Enterprise) and Task Force 11 (Saratoga), in one hex. Nagumo's two carriers with the ships of the vanguard. The Ryujo detachment. Henderson Field. Left out: Wasp's force, away fuelling; the Advance Force and the convoy, whose classes are mostly unresearched; the flying boats and B-17s; and the 25th.
3. **Enterprise's force has the lower number,** so a strike on the hex attacks it and not Saratoga's (decision 0026, 7). That is what happened.
4. **The American forces do not move.** Their positions at 08:00 and at 12:00 fall in the same hex, (7,-5).
5. **The Japanese forces start at the map's northern edge and steam on one course at one speed,** chosen so that each is where the sources put it at noon on the 24th, to the hex: the Ryujo detachment in (7,-10) at 20 knots, and the carriers in (8,-11) at 17 knots. The carriers' noon position is one hex nearer the Americans than the track chart's point, which was read by eye and is good to about a hex; the nearer hex is the one consistent with the strike that was flown, about 250 miles.
6. **Air groups are the squadrons and their aircraft operational on the day,** or on hand where no operational figure is sourced. Each cites its row.
7. **Each of Tone and Chikuma carries five E13A1 floatplanes.** This rests on an establishment of June 1942, not a count on the day. The battleships' and other cruisers' floatplanes are left out: their numbers are not sourced.
8. **Who searches.** Enterprise's scouting squadron searches the northern half, 290 to 090, to 200 miles, as it did. The cruisers' floatplanes search 110 to 200. No source gives the Japanese sectors; a quarter-circle toward the south-east is an assumption.

   *Nov 14, 2026: the Japanese search is set from the record by decision 0073, and the remaining orders by decision 0081.*

9. **Who strikes.** Every dive bomber and torpedo squadron is ordered to strike ships. The Japanese attack planes are within reach of nothing where it really is, and almost always stay on deck, which is what they did.
10. **Fighters.** American fighters all patrol: with a radius of 105 miles they could not escort (decision 0030). The Japanese carriers keep four fifths overhead and send the rest; the first wave had ten Zeros of about fifty. Ryujo keeps three tenths; the rest were for her raid on Henderson Field, which is not modelled, so they fly nothing.
11. **In step 1 the scenario is run and its spread reported, not judged,** as at Savo Island (decision 0024, 12).

## What the map does to the battle

Distances are between hex centres, 40 miles apart, and a strike or search reaches a hex or does not.

| From | To | At noon | Who reaches |
| --- | --- | --- | --- |
| American carriers | Ryujo | 200 miles | The SBD-3 (200). Not the TBF-1 (190), which did attack her on the day, nor the F4F-4 (105) |
| American carriers | Nagumo's carriers | 240 miles | Nobody |
| Nagumo's carriers | American carriers | 240 miles | The D3A1 (252), the A6M2 (261), the E13A1 (253). Not the B5N2 (175) |
| Ryujo | American carriers | 200 miles | The E13A1 and the A6M2. Not the B5N2 |
| Henderson Field | Ryujo | 240 miles | Nobody |

In the morning every force is 320 miles or more from every enemy, and nothing is found.

## What the first runs show

Over seeds 1 to 1000 (`sim-cli batch`), with the placeholder figures:

| | In the runs | On the day |
| --- | --- | --- |
| Ryujo | Sunk in 304, crippled in 78 more | Sunk |
| Enterprise | Hit in 518 runs, 1.3 bombs a run on average; never sunk | Three bombs; not sunk |
| Saratoga, Shokaku, Zuikaku | Never attacked | Never attacked |
| Japanese aircraft lost | 32 on average, up to 76 | 24 of the first wave's 37, and more of the second after dark |
| American aircraft lost | 2.3 on average, up to 7 | About 20, most of them to fuel and the dark |

Unlike Savo Island, this has the shape of the day. What made it so is in the facts, not the placeholders: which aircraft reach which hex.

Two things in it are not the day's. Ryujo survives in most runs because the scouts miss her in four runs of ten and a wrong report sends the strike to an empty hex in three of the rest. And the American losses are far too low, because nothing but fighters and guns can lose an aircraft: there are no operational losses yet (scenario 6).

## Not modelled, and it mattered on the day

- **Radar warning and fighter direction.** Enterprise's radar saw the raid at 88 miles.
- **Lost reports.** The American scouts' sighting of Nagumo's carriers never arrived; the Japanese strike leader missed his floatplanes' reports.
- **A second wave that fails to find its target.** Here both Japanese carriers' dive bombers fly at once, each as its own raid, and each meets the whole American patrol (decision 0027, 12).
- **A strike on an airfield.** Ryujo's raid on Henderson Field.

## Rule card

No rule. These are scenario inputs.

## Amended by decision 0058 (Nov 12, 2026)

The Japanese starts were worked back from noon fixes with 40 miles a hex. With straight-line miles (decision 0058), settled by the architect: the carrier striking force keeps its start, (9,-19) at 17 knots, which is 302 miles from (8,-11) and brings it there at 11:46; the Ryujo detachment starts in (7,-19) at 20 knots, 360 miles from (7,-10), and arrives at noon as before. From (10,-19) it would have arrived at 09:51, at odds with the 09:35 sighting. The full slice takes the same starts.

## Note, Nov 12, 2026

Ryujo's noon hex, her raid and her fighters' share were changed after decision 0063 (the lines above are left as they were written). She is in (7,-9) at noon and not (7,-10): the record has her launch from 200 to 202 miles of Guadalcanal (engagement_events 331, engagement_positions 310), and (7,-10) is 223 miles from Henderson Field by hex centres, which is the grid's error. Her attack unit raids Henderson Field (the line "A strike on an airfield" under what is left out is no longer true of it), as a base strike on extended reach (engagement_outcomes 1171), and her fighters keep 0.35 and send 15 of 23 with it, where item 10 above says three tenths. The reasons and the rows are in decision 0063, "As built (the scenario orders)".

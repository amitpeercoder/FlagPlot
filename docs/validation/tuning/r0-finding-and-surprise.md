# Round 0: finding and surprise. Nothing moved

Savo Island, surface figures, Nov 13, 2026. The first mechanism in the order of tuning (`docs/step2-spec.md`, "Tuning", 2). Ruled by the coordinating architect: no figure of finding and surprise is moved, and there is no finding round.

## What was done before it

The supported step was not a figure. The radar bonus to sighting ships at night was being given to ships whose sets were air-search sets or had no plan display, and that was mended in the derivation (decision 0077). `radar_sighting_bonus` stays 0.2.

## The measure

`night_fights_the_japanese_did_not_sight_first`, record 0.

| | Night fights the Japanese sight first / both in one round / the Allied side first | Runs holding the record | 25th, median, 75th |
| --- | --- | --- | --- |
| Before decision 0077 | 872 / 711 / 414 | 204 | 1, 1, 2 |
| The baseline tuning starts from | 1,140 / 542 / 316 | 335 | 0, 1, 1 |

It agrees by the rule, with the record in the lower quarter of the runs and the middle run still having one fight go the other way.

## What was looked at and not moved

`not_alert_sighting_factor`, 0.5, on scratch copies, 1,000 seeds each:

| Value | Japanese first / both / Allied first | Runs holding the record | 25th, median, 75th |
| --- | --- | --- | --- |
| 0.5 | 1,140 / 542 / 316 | 335 | 0, 1, 1 |
| 0.4 | 1,244 / 465 / 288 | 396 | 0, 1, 1 |
| 0.3 | 1,351 / 391 / 255 | 468 | 0, 1, 1 |
| 0.2 | 1,467 / 321 / 210 | 547 | 0, 0, 1 |

It is not moved because no row gives a size (the rows say the Allied ships were at their second condition of readiness with captains asleep, `engagement_events` 23 and 24, and nothing of how much worse such a force sees), and because it is the figure of every force not alert in every battle, the Japanese at Cape Esperance among them.

`alert_contact_distance_hexes` (3), `sighting_chance_night` (0.5) and `experience_sighting_strength` (0.25) have no source for a size and are not moved. The first is not inert at Savo Island: at 7 or more the contact the Allied side holds at the start would alert both groups.

## The gap, named and not chased

A round is five minutes and both forces roll in it, so "both in the same round" is about a quarter of night fights whatever the figures are. In the record the Japanese sighted each group seven and thirteen minutes before it sighted them. The rules also end a force's surprise in the round after it is first fired on (decision 0076), so the two measures of rounds surprised are reported and not judged.

## Not moved in any round

The break-off levels of scenario 4 (decision 0024, the note of Nov 13, 2026): the northern group at 1, the southern group and the Japanese force at 0.5.

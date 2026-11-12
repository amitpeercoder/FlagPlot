# Strike launch ranges in 1942, and what launching far cost

Status: research report for the architect, read on 2026-11-12. Nothing here is settled. The rows are in `data/oob`: `engagement_events.csv` ids 396 to 423 (one `strike_launch` row per strike per source, the stated distance in `range_m`), 25 rows in `engagement_outcomes.csv` for aircraft lost returning or landed elsewhere, 11 gaps, and four new engagements (`coral-sea`, `midway`, `santa-cruz`, `lae-salamaua-raid`). "Miles" in the naval sources are taken as nautical; each row says so. Six of the quotations below were checked a second time against the saved pages.

## The question asked first: Saratoga's torpedo bombers against Ryujo

**They flew and they attacked.** The 1943 Navy narrative: eight TBFs launched with thirty dive bombers at about 1435 on 24 August; one turned back; five attacked Ryujo ("Three torpedo planes approached from the starboard bow and two from the port bow") and two attacked a cruiser. The only range the source states is the search planes' sighting of Ryujo at 198 miles at 1440. The group found her at 1606 and attacked at 1620, and Ryujo had been closing. No aircraft of the strike was lost. So torpedo bombers went to a target reported just under 200 miles away and came back.

## Strikes with a stated range

| Battle | Carrier | Aircraft | Stated range at launch | Lost or diverted for fuel, distance or darkness |
| --- | --- | --- | --- | --- |
| Coral Sea, 7 May | Lexington, Yorktown | 50 and 42 | "within 160 miles" | None |
| Coral Sea, 8 May | Yorktown, Lexington | 39 and 43 | "about 170 miles" at the sighting | Lexington: one torpedo plane out of fuel; five more lost to fighters or fuel, not split |
| Midway, 4 June | Hornet, Enterprise | 60 and 57 | 155 miles | Hornet: all 10 fighters ditched for fuel; 13 dive bombers landed at Midway and 2 ditched there. Enterprise: 18 dive bombers lost, "thought" mostly to fuel |
| Midway, 4 June, evening | Hornet | 16 dive bombers | 162 miles | None; last landing 2059 |
| Midway, 5 June | Hornet | 26 dive bombers | Attacked at 278 miles from the launch position after a 315-mile search | One ditched out of fuel at night; one landed with dry tanks |
| Midway, 5 June | Enterprise | 32 dive bombers | 265 miles flown outbound | None; recovered in darkness at 2203 |
| Midway, 6 June | Hornet | 34, then 24 | 142 miles; 110 miles | None |
| Eastern Solomons, 24 August | Saratoga | 30 dive bombers, 8 torpedo bombers | 198 miles at the sighting | None |
| Eastern Solomons, 24 August, 1708 | Enterprise | 11 dive bombers, 7 torpedo bombers, 7 fighters | Not stated; about 150 miles by the positions | The dive bombers landed at Henderson Field as planned. Torpedo bombers landed aboard at night with "between 4 and 40 gallons"; one crashed in the barrier |
| Eastern Solomons, 23 August | Saratoga | 31 dive bombers, 6 torpedo bombers | Not stated | All landed at Henderson Field for the night, "following instructions" |
| Santa Cruz, 26 October | Hornet, Enterprise | 29, 25 and 19 | "about 210 miles" | Seven torpedo bombers ditched across the three groups, and one missing |

Japanese launch ranges: one found since, in the vanguard's signal log: the first wave was sent toward the sighting aircraft's estimated position, 153 degrees and about 260 nm from the carriers at the time of the sighting, some fifty minutes before it left (see `japanese-air-records.md`, second pass; one reading, medium confidence). The Japanese forms read today give launch times and losses but not the distance. Zuikaku's second wave, which found nothing and came back after dark, lost one aircraft for fuel by her own form.

## What the record shows

- **The usual launch was at 150 to 210 miles, for every type together.** Dive bombers, torpedo bombers and fighters went as one group to the same target. No source read launches the dive bombers and holds the torpedo bombers back for range.
- **At about 155 to 175 miles losses for fuel were rare, unless something went wrong** on the way: at Midway the enemy turned, the wind lay the wrong way and nobody broke radio silence. Then Hornet lost all ten of her fighters and Enterprise 18 of 33 dive bombers, most of them thought lost to fuel.
- **At 200 miles and beyond, a few aircraft were lost for fuel as a matter of course** (Santa Cruz), and the fighters were the first to run short: a fighter leader there refused another 90 miles.
- **The long reach was done on purpose, with a plan for recovery:** dive bombers only; the carrier closing while they were out; a night recovery accepted; or a field ashore named as the place to land.
- **Dusk was as hard a limit as fuel.** Enterprise's strike of 24 August was first dropped because the aircraft "would not be able to return before nightfall", then sent with Guadalcanal as the alternative.

## Doctrine and practice, in the sources' words

- Deferred departure: "Deferred departure was used, and the launching required about an hour" (Midway).
- Fighters held back for fuel: "To conserve fuel, the fighters were not launched till 0905" (Yorktown, Midway); at Santa Cruz the waves went separately "to save time and fuel".
- Fighters must match the bombers: "VF which accompany the attack group should have the same cruising range as the aircraft they accompany" (Hornet's captain, after Midway).
- Range as the deciding factor: "A factor of the highest importance in determining success or failure of a carrier task force is the striking range of its air group", and "There will always be a gap between theoretical airplane performance and what is actually practicable in war operations" (Enterprise's captain, after Eastern Solomons). He also held that delay to confirm a contact cost more than distance.
- Wind: with the wind "directly away from the enemy ... our attack planes had a longer run back to the ship" (Hornet, Midway).
- "Point Option: course 000, speed 10 knots" appears in Enterprise's report with no definition. It reads as the course and speed the carrier told her aircraft she would hold for their return; that is an inference.

**Not found:** a stated standard strike radius by aircraft type, or a written rule on launching beyond radius and recovering closer. The fleet's tactical publications of 1941 (USF-74 and USF-77) were not fetched and are the place to look.

## Second pass, 2026-11-12: doctrine and stated radius

The fleet's tactical orders of 1941 (USF-74 and USF-77) could not be found online. What was read instead: the 1945 edition of the fleet's tactical orders (USF 10B), two fleet bulletins of 1942 on battle experience, the Naval War College analyses of Coral Sea and Midway, and the official data sheets for the TBF-1 (1943) and SBD-5 (1944). Five of these quotations were checked a second time against the saved texts.

**Point Option, defined** (USF 10B, paragraph 3362(a); the Midway analysis quotes the same wording): "Point Option is a point moving on a specified course and speed such that, at any instant, it represents the predicted position of the carrier, with an allowable error of less than half the radius of visibility."

**Closing instead of range.**
- Fleet bulletin on the Solomons, of a carrier raid of 5 October 1942: "With the limited range of our present aircraft, it is necessary for the carrier to approach objective within 125 miles in order to insure hitting objective and recovering aircraft while retiring at high speed."
- Fleet bulletin on Midway, of one carrier's launch: "No point option given but in lieu thereof closing of the enemy was indicated."
- The Midway analysis: the task force commander delayed the launch to close, and "the controlling factor ... appears to have been the combat radius of the torpedo planes which was about 175 miles". Dive bombers then ditched "because of a shortage of gasoline due in part to the inability of TF 16 to close Point Option".
- On 5 June the same commander "closed the contact at 25 knots" behind his long-range strike.

**Stated radius** (rows in `aircraft_type_facts.csv` for the TBF-1; the SBD-5 and TBD-1 are not types in the data, so they have no rows):

| Type | Load | Stated radius | Source |
| --- | --- | --- | --- |
| TBF-1 | Torpedo, 335 gallons | 225 nm | Data sheet, 1 July 1943 |
| TBF-1 | Torpedo, two wing tanks | 340 nm | The same |
| TBF-1 | One 1,000 lb bomb | 235 nm | The same |
| SBD-5 | One 1,000 lb bomb | 240 nm | Data sheet, 1 June 1944 |
| SBD-5 | One 500 lb bomb | 260 nm | The same |
| TBD-1 | Torpedo | "about 175 miles" | The Midway analysis, 1948 |

The data sheet states how its radius is worked out: 20 minutes of warm-up, a minute for take-off, 10 for rendezvous, 15 of combat, and 60 minutes at best-range speed for rendezvous, landing and reserve, with fuel use raised by 5 per cent. It is a still-air figure from a fixed point. Wind and the carrier's own movement are not in it.

**What this adds to the reading above.** The published radius and the practice differ by exactly the two things the sheets leave out. A strike could be sent to the radius or past it if the carrier would be nearer on its return, and it lost aircraft inside the radius when the carrier could not keep to Point Option or the wind lay away from the enemy. The model's torpedo radius of 190 nm sits between the TBD's 175 and the TBF's 225.

**Still not found:** any written rule on launching beyond radius, on sending dive bombers without torpedo planes for range, or on landing ashore; and any Japanese radius figure for the A6M2, D3A1 or B5N2. The only Japanese statement found is from 1944: a plan to return "within 300 mile radius", changed to landing at shore bases when the distance became 400.

## For the rule

A hard cutoff at the type's radius is not what the record shows: a torpedo group at 198 miles flew and returned whole. What the record supports is a cost that rises with distance past roughly 175 miles and is paid in aircraft lost on the return, falling first on fighters; that is waived when the carrier closes or a friendly field lies beyond the target; and that is worse near dusk. The figures to fit it are the rows above. That reading is mine; the rule is yours.

## Gaps

Japanese launch ranges for Eastern Solomons, including Ryujo's strike on Henderson Field; Saratoga's own action report; the Japanese strikes at Midway and Santa Cruz; the Lae and Salamaua raid; the fleet's tactical publications.

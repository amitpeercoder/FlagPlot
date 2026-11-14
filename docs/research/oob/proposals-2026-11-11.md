# Proposals for the architect, 2026-11-11: cruiser floatplanes, aerial torpedoes, anti-aircraft fits

Status: research proposals. Nothing here is settled. Every figure is a row in `data/oob` with its source and locator; the row ids are given so a settlement can name them. "Checked" means the page image or page text was read a second time against the row, not only by the researcher that found it. *(Nov 14, 2026: the search recommendation of section 1c, 300 nm, was replaced by decision 0073, which uses the recorded 330 miles.)*

## 1. Cruiser floatplanes at Eastern Solomons (the priority request)

### 1a. Aichi E13A1 (`e13a1`)

| Item | Proposal | Row | Source | How sure |
| --- | --- | --- | --- | --- |
| Range, normal fuel | 760 nm (at 98 kn, 179 US gal) | 494 | US Technical Air Intelligence Center data sheet "JAKE 11", December 1944. Checked | Medium. A US estimate of late 1944, not a Japanese figure for 1942 |
| Range, maximum fuel | 1,194 nm (at 103 kn, 297 US gal) | 493 | Same sheet. Checked | Medium |
| Cruise speed | 130 kn (75% of maximum, 1,500 ft) | 495 | Same sheet. Checked | Medium |
| Maximum speed | 192 kn at 7,600 ft | 496 | Same sheet. Checked | Medium. Websites give 203 kn |
| Pilot armour, self-sealing tanks | no, no | 498, 499 | Same sheet: "Neither armor plate or self-sealing tanks are known to be used on this plane". Checked | Low. The sentence is hedged |
| Endurance | 14.9 hours | 516 | Japanese Wikipedia only, with a 3,326 km maximum range | Low. No better source found |

The existing 1,128 nm row (117, Wikipedia, "normal") matches no condition on the data sheet. It sits near the maximum-fuel figure, so it is probably a maximum range mislabelled. **Recommendation:** take the data sheet's pair (760 normal, 1,194 maximum) and do not mix it with the website figure. A search radius of 300 miles was flown by these ships' planes at Midway (see 1c), which the 1,194 nm figure allows and the 760 nm figure allows only with little time on station.

New attributes: `endurance_h` and `fuel_capacity_l` for aircraft types.

### 1b. What the ships carried

| Ship | Proposal for 24 August 1942 | Rows | Source | How sure |
| --- | --- | --- | --- | --- |
| Tone | 5 E13A1 | 284, 285 | Japanese Wikipedia infobox: establishment from 25 June 1942 "five Type 0 reconnaissance seaplanes", citing a Navy Ministry telegram of that date in the Yokosuka Naval District war diary. Until then three E13A1 and two Type 95 (E8N) | Low to medium. An establishment, not a count on the day, and the primary document was not read. It is consistent with plane numbers up to No. 5 on both ships and with Tone's "four floatplanes" at Santa Cruz (row 276) |
| Chikuma | 5 E13A1 | 289, 290, 293, 277 | The same telegram. The Chikuma page also says her seaplanes "were unified to three-seat seaplanes" after Midway. The Japanese ship record names her "No. 2 Jake" on the day | Low to medium, as Tone |
| Kumano, Suzuya | 3 E13A1 each, one of which could be an E7K2 | 295, 296, 300, 301 | Japanese Wikipedia infobox, a second telegram of 25 June 1942. Kumano flew an E7K2 in October (row 282) | Low |
| Hiei | F1M2, number unknown (capacity 3) | 278, 306, 305 | The Japanese ship record and Japanese Wikipedia both have a Hiei F1M2 flying on 24 August; she lost one to SBDs | Type medium, number low |
| Kirishima | F1M2 in October; capacity 3 | 280, 308 | Japanese ship record, Japanese Wikipedia | Low for August |

**Recommendation:** five E13A1 on each of Tone and Chikuma, as a named assumption resting on the establishment. The F1M2 matters only for the battleships; its figures are in rows 505 to 515 (695 nm normal, 890 nm maximum, 125 kn cruise, no armour or tank protection, stated outright).

New types `e8n2`, `e8n1` and `e7k2` exist only so the rows can name them. No performance figures were found for them.

### 1c. The search of 24 August

What is sourced (Japanese ship record of movement unless marked):

- 1100: Tone and Chikuma "launch seven floatplanes on a late morning search" (existing row 172). The record does not split the seven between the ships.
- 1400: Chikuma's No. 2 E13A1 finds the US force and is shot down (existing row 179). Its report, and later messages from Hiei's F1M2 and Chikuma's No. 5, were not picked up by the strike leader.
- 1430: a Chikuma floatplane reports Enterprise's position (existing row 180).
- Japanese Wikipedia, from Japanese books: a Chikuma plane found the US fleet "just after noon" and went missing; a Hiei plane found the carriers "just after 2 pm" and reported.
- The vanguard was only 5 to 10 nm ahead of the carriers (Japanese Wikipedia, citing the official history).
- Tone was with the Ryujo detachment from 0400 by the same record that has her launching with Chikuma at 1100. One Japanese page has a Tone seaplane on deck used to send deceptive radio messages. So the seven may have been mostly Chikuma's and the battleships'. Not resolved.
- Japanese losses included three reconnaissance seaplanes (Japanese Wikipedia, from the official history).

Not found, and in `gaps.csv`: who ordered the search, its sectors and ordered distance, the split of the seven aircraft, and any reported position. The only distance on record for these ships' planes is Midway, where they searched "out 300 miles".

**Recommendation for the scenario:** seven search aircraft launched at 1100 from the vanguard, radius 300 nm as an assumption taken from Midway, with the sighting three hours after launch as the check on it.

## 2. US Mark 13 aerial torpedo in August 1942

**Proposal: Mark 13 Mod 1, 400 lb of TNT (181.4 kg), 33.5 knots, 6,000 yards, about 1,927 lb ready for war shot.** Rows 434, 437, 440, 443, 453.

- The Navy's own pamphlet on the torpedo (O.P. 629(A), foreword dated 27 August 1942) tabulates three models. The Mark 13 and Mark 13-1 carry a 400 lb charge in war head Mark 13, "400 lbs. of cast T.N.T.". Only the Mark 13-2 carries 600 lb, at 40 knots and 4,000 yards. Checked against the page images.
- The Bureau of Ordnance history names "the Mark 13 Mod 1" in its account of early-war complaints, and says the heavier explosive was first used in the fall of 1942.
- The Navy torpedo history still sets the Mark 13's "400 pounds" against a new design in summer 1943.
- navweaps says Mod 1 entered service in 1940 and calls Mod 2 an experimental 40-knot torpedo whose development was cancelled.

How sure: fairly sure of 400 lb. No squadron issue record was found, so that the carriers held Mod 1 and not the older Mod 0 is an inference; the warhead is the same either way. Rows 382 and 448 (600 lb) describe later models and should not be read for 1942.

A structural point: all models share the one key `us-torpedo-mk13`, so their rows dispute each other by format. Separate keys per model would remove that. Not done here, because fits refer to the key.

## 3. Japanese Type 91 aerial torpedo in August 1942

**The models are now dated; which one the carriers held is not.**

- US Naval Technical Mission report O-01-2: Modification 2 (1941), charge "increased from 330 to 450 lbs" (204 kg); Modification 3 (1942), charge "increased to 528 lbs" (530 in its table; 240 kg). Rows 455, 456, 462, 463. Checked against the page text.
- This agrees with navweaps (rows 226, 227). Wikipedia's 235 kg (row 228) agrees with neither and is the outlier.
- The report gives only the year for Modification 3. It also says "The heaviest types were used in land-based torpedo planes and the lightest ones in carrier-based aircraft", without naming models.

**Recommendation: Modification 2, 204 kg, for carrier aircraft in August 1942**, as an assumption: Modification 3 has no month, new models reach units late, and the report's one sentence on carriers points to the lighter head. How sure: low. The difference is 36 kg, or 18 per cent.

## 4. Enterprise's 20 mm guns on 24 August 1942

**No new evidence settles it.** The settled value (38, row 509) is left as it was.

- The gunnery officer's report for the action (new source) names 20 mm batteries up to No. 8 and a "gun #5 of battery #6", and asks for more 20 mm guns. It gives no total. Row 624.
- Wikipedia's undated, unsourced table has 32 from mid-June to mid-September 1942 and 44 after (rows 620, 622). That supports 32, not 38 or 40, at low grade.
- A photo caption dates the first 20 mm guns to 19 March 1942 (row 623).
- The same report shows the fifth 1.1-inch mount "newly installed" on the forecastle and .50-calibre guns still firing (885 rounds), so the website line that swaps all the .50s for 20 mm in mid-1942 is not exact.

**Recommendation:** keep 38 as it is, or move to 32, the one figure two websites share. How sure: low either way. The yard records or Friedman's carrier book would settle it.

## 5. Lower priority

- **Grayson and Monssen, 5-inch guns: four each, as a recommendation.** NavSource lists four for both (rows 628, 629). A destroyer history site says the ships of their squadron "landed their No. 3 5-inch mount" (rows 630, 631, by inference from the squadron). Against: Wikipedia and the older Navy history header give Monssen five (rows 633, 634, undated). How sure: medium for Grayson, low to medium for Monssen.
- **Farragut class 20 mm: no ship-by-ship date found.** Macdonough had her No. 3 gun removed by 20 April 1942 at Mare Island (row 636), earlier than the "5-9.1942" of the website. Dewey and Worden went to the yard in late 1942 and Farragut shows no yard period before the battle; that suggests, without stating, that those three still had .50-calibre guns on 24 August.

## What was looked for and not found

In `gaps.csv`: the search orders and sectors, a count of floatplanes aboard on the day, E13A1 endurance from a contemporary source, any figures for the E8N2 and E7K2, the date the 600 lb head reached the Mark 13, squadron issue of either torpedo, Enterprise's gun total from a ship document, and dated fits for the four Farragut-class ships.

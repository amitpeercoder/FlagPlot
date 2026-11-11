# Rabaul against Guadalcanal in the air, 21 August to 17 September 1942: what the sources give scenario 6

Status: research note for scenario 6, read on 2026-11-11. Nothing is settled here. Where sources disagree, each has its own row and the architect settles it. The rows are in `data/oob/engagement_events.csv` (15 new), `engagement_outcomes.csv` (55), `engagement_forces.csv` (19), `engagement_positions.csv` (5), `air_unit_facts.csv` (33), `aircraft_type_facts.csv` (19), `place_positions.csv` (3), `air_units.csv` (3), `sources.csv` (5) and `gaps.csv` (18). The engagement is `rabaul-guadalcanal-air` (id 1002).

## The window

21 August to 17 September 1942, as asked. No better-documented four weeks was found. The window runs from the first Marine fighters (20 August) to the lull in raids after the Bloody Ridge fight (15 to 27 September, Wikipedia citing Frank). The sources read cover the larger raids of 24 August, 2 September and 10 to 14 September, and almost nothing day by day outside them. The two sources that would give the days, Lundstrom (2005, pp.184-219) and Frank (1990), were not in hand.

## What was read

| Source | Used for |
| --- | --- |
| Hough, Ludwig and Shaw, *Pearl Harbor to Guadalcanal* (Marine Corps), chapter VI | Arrivals, raids of 24 August, 2 and 11 to 14 September, strength of Cactus |
| Miller, *Guadalcanal: The First Offensive*, chapters IV to VI | Arrivals, serviceability on 25 and 29 to 30 August, raids of 10 to 13 September, 22 September status, fuel |
| Shaw, *First Offensive* (1992 pamphlet) | Arrivals, raids of 22, 24, 29 and 31 August (some of which look wrong, see below) |
| Craven and Cate, *AAF* vol. IV, chapters 1 and 2 | P-400s, B-17s, fuel reserve, King's 57 percent loss rate, distance |
| Shaw and Kane, *Isolation of Rabaul* (Marine Corps, 1963) | Distance, Zeke and Betty paragraphs (late 1943, Allied intelligence) |
| Miller, *CARTWHEEL* | Distance, Rabaul fields |
| Wikipedia: Cactus Air Force, Tainan Air Group, 6th Air Group, 25th and 26th Air Flotillas, Mitsubishi A6M Zero | Raid-by-raid detail citing Lundstrom, Hata and Frank, none of which was read. All rows low and unconfirmed |
| j-aircraft.com, airpages.ru, Pacific Wrecks, Hollway (HistoryNet) | Aircraft statements and a few raid facts. Low |

Not read: Lundstrom, Frank, Bergerud, Hata, Izawa and Shores, Francillon, Sherrod (no copy online), Japanese Monograph 98 (the file was too large to fetch), the USSBS Rabaul report (not found on HyperWar; the DTIC copy refused the request), Morison. The AWM article by Shindo does not discuss the range.

## 1. The Zero's reach from Rabaul

**The distance.** Four statements, in `engagement_positions.csv`: 565 nm from Rabaul to Guadalcanal (Miller, CARTWHEEL); 650 statute miles, Henderson to Vunakanau and Lakunai, which is 565 nm (Shaw and Kane p.447); 675 miles (Craven and Cate, July 1942, 586 nm if statute); and 556 miles each way for the 7 August escort (Wikipedia on Tainan, citing Frank and Bergerud; 483 nm if statute, 556 nm if the miles are nautical). The first two agree. Wikipedia may be giving nautical miles; an enthusiast page seen only in a search result also says "almost 560 nautical miles" but was not read.

**What the sources say about the aircraft.** Nothing found states a combat radius for the escort. What exists:

| Statement | Source, confidence | Row |
| --- | --- | --- |
| Radius "up to 300 miles from a carrier and well over 500 miles from a land base"; "nearly 600 miles" (Lundstrom as quoted) | j-aircraft, medium, no page | 361 to 363, existing |
| 7 August: 18 Tainan Zeros escorted bombers Rabaul to Guadalcanal, "556 miles each way", the longest fighter mission to that date | Wikipedia, low; cites Frank pp.67-69, Bergerud p.557 | a6m2-21 combat_radius_nm 483 |
| Total range 1,580 miles with maximum fuel load and economy speed (original Zeke, 1,373 nm) | Shaw and Kane p.449, medium | a6m2-21 range_max_nm |
| Drop tank 330 L (Green and Swanborough via Wikipedia); 320 L (Francillon 1979 p.399 via Wikipedia); about 150 gal rated, holds about half (existing row 365 note, War Department) | Wikipedia, low | drop_tank_l |
| Design requirement: two hours at normal power or six to eight hours at economical cruise | Wikipedia citing Francillon 1970 p.363, low | gap row only |
| Model 32 sacrificed nearly 1,000 km (620 miles) of range | Wikipedia citing Nohara 1993 p.76, low | a6m3-32 performance_note |
| Model 32 "about 20 percent less range" than the Model 21 (Lundstrom as quoted) | j-aircraft, medium | a6m3-32 performance_note |
| Rabaul-based Zeros "had to travel nearly to their maximum range to reach Guadalcanal and return"; the Model 32 shorter range was a significant limitation and it was unsuited to the campaign | Wikipedia, low; footnotes blank | a6m3-32 performance_note |
| Model 32 could operate from Buin once ready in October 1942 | Wikipedia (6th Air Group), low | a6m3-32 performance_note |
| Buka was operational in October, the only one of the central and northern Solomons fields finished in time to matter | Shaw and Kane p.447, medium | in the text of this note |
| Model 22 had the longest range of any A6M, needed to reach Guadalcanal; "560 miles" | airpages.ru, low | a6m3-22 performance_note |
| Internal fuel: Model 22 480 L, Model 32 570 L, each plus a 330 L tank | j-aircraft, low, no source | internal_fuel_l, drop_tank_l |

So the escort was flown (7 August is the only dated Tainan example; the 6th Air Group's 18 Zeros from Kisarazu flew the ferry via Kavieng and began Guadalcanal missions on 11 September, per Wikipedia citing Lundstrom p.191). No source gives a figure for a round trip with the tank. The existing 261 nm figure is Lundstrom's carrier radius; the same quote says well over 500 miles (434 nm) from a land base, and nearly 600 miles (521 nm) in another place. The distance itself is 565 nm each way, 1,130 nm round trip, which is under the 1,373 nm of Shaw and Kane and the 1,677 nm of the Pacific Wrecks row, and needs the flight's own reserve. I do not settle it.

**Not entered.** A forum-style FAQ thread on j-aircraft has Richard Dunn saying there is "significant evidence" that early Rabaul to Guadalcanal Zeros did not drop their tanks, and a poster citing Sakai saying they did. The policy forbids forums in fact rows, so it is a gap row only.

## 2. G4M1

Shaw and Kane give a cruising range of 2,110 miles with normal fuel and bomb loads and up to 2,200 lb of bombs, for a Betty that does not carry a model number in a late-1943 chapter. Entered low. The existing rows already give 1,000 kg and 1,540 nm. No source in hand states the load used on the Henderson raids.

## 3. Japanese units

Unit rows are few. Existing rows (4th, Tainan, 2nd, Misawa, Kisarazu, 6th, Chitose, Kanoya, Yokohama) already hold most dated strengths for 6 to 22 August. New: the 26th Air Flotilla (Misawa, Kisarazu, 6th and Chitose detachments) and the 25th Air Flotilla (4th, Tainan, 2nd, Toko and Yokohama detachments) as listed on 12 September (Wikipedia citing Lundstrom pp.189-191); Rabaul totals: 160 aircraft by 23 August (Allied estimate, Miller), 117 at Rabaul on 20 September (Wikipedia citing Frank), 85 delivered 15 to 27 September (same), and 140 added on 12 September (Hough, US estimate). Hollway (no references) says 60 fighters and 72 medium bombers reinforced Rabaul. No daily strengths by type were found.

## 4. Cactus Air Force

Arrivals: VMF-223 (19 F4F) and VMSB-232 (12 SBD) on 20 August; 5 P-400s on 22 August and 9 more on 27 August; Flight 300 (11 SBD, Enterprise) on 24 August; VMF-224 and VMSB-231 about 30 August; 24 VF-5 F4Fs on 11 September (Hough) or 4 September (Wikipedia); 18 F4Fs from Hornet and Wasp, 12 VS-3 SBDs and 6 VT-8 TBFs on 13 September (60 in all from 11 to 13 September). Counts: 64 aircraft and 86 pilots (Hough, month inferred); operational counts 20 (25 August), 8 F4F and 14 P-400 (29 August), 5 F4F (30 August), and on 22 September 30 F4F, 22 SBD, 7 TBF and 5 P-400. Hollway lists 3 P-400, 22 SBD and 11 F4F on 10 September.

## 5. Raids: why the engagement tables, and how they were used

A month of raids does fit `engagement_events.csv` (one row per raid, kind `strike_attack`, with counts in `value` and the source's words in `as_stated`) and `engagement_outcomes.csv` (one row per measure per side per raid: `bombers_attacking`, `fighters_escorting`, `aircraft_lost`, `aircraft_claimed_shot_down`, `aircraft_lost_operational`, `killed`). The subject names are plain text, such as `raid-1942-09-12` and `cactus-air-force-1942-09-12`, because the file allows a force name. New measure names were added by me where marked in a note. What does not fit well: the sources give a few raids only, so the tables hold fifteen events, not twenty-eight days; and a loss by cause for one raid is one row per cause. I used `aircraft_lost_operational` for accidents and kept claims apart from losses (US statements about Japanese losses are `aircraft_claimed_shot_down`, low).

| Date | Japanese sent | Japanese lost | Cactus lost |
| --- | --- | --- | --- |
| 24 Aug (Ryujo strike) | 15 bombers and 12 fighters (Hough) or 6 Kates and 15 Zeros (Wikipedia) | 16 claimed (Hough, Shaw), 21 claimed (Miller), 6 aircraft lost and 8 aircrew killed (Wikipedia citing Lundstrom) | 4 (Hough), 3 (Miller, Wikipedia); 2 pilots killed (Wikipedia), 3 (Hough, Miller) |
| 2 Sep | heavy raid at 1135 | 7 claimed | none in the air |
| 10 Sep | 27 bombers and 30 Zeros (Miller) | 4 claimed | 1 |
| 11 Sep | 26 bombers and 8 Zeros (Hough) | | 1 P-400 on the ground; 11 Marines killed |
| 12 Sep | 42 planes (Hough), 28 bombers (Miller), 25 Bettys and 15 Zeros (Wikipedia) | 15 claimed (Hough), 8 claimed (Miller), 7 (Wikipedia) | 1 pilot killed landing |
| 13 Sep | three waves | 11 claimed (Hough); 6 (Wikipedia) | 5 (Hough), 6 (Wikipedia) |
| 14 Sep | 19 to 24 floatplanes | 8 (Wikipedia) or 3 (Pacific Wrecks) | none in the air |

Cumulative: Cactus shot down 56 for 11 in the first ten days (Hough, a claim); 27 Marine fighters and 9 pilots lost 21 August to 11 September, against 31 Japanese aircraft destroyed in ten raids (Wikipedia citing Lundstrom p.184); King's 57 percent loss rate for Marine squadrons over 25 days (Craven and Cate); August to November, 43 destroyed on the field and 86 operational losses (Hollway, whole campaign).

## 6 and 7. Operational and pilot losses, replacements

Only the pieces in the outcomes rows. No sourced split of Japanese losses by cause was found, and no sourced share lost on the flight home. Pacific Wrecks and Wikipedia mention aircraft that ditched or were lost returning, with no totals. A loss appendix from a book, offered as a sample page on a bookseller's site, listed individual Japanese ditchings to 30 September but could not be read. Replacements: Cactus received 23 aircraft and Rabaul 85 from 15 to 27 September (Wikipedia citing Frank).

## 8. Supply at Henderson Field

400 drums of aviation gasoline on 10 August (Miller); fuel hand-pumped from 55-gallon drums (Hough, Wikipedia, Hollway); a reserve for about four days in mid-September (Craven and Cate); 3,823 drums landed on 18 September, one day outside the window, by McCawley, Bellatrix, MacFarland and Tracy (Miller); Shaw says that convoy brought aviation gas and the first ammunition resupply since D-Day. Hollway says there were plenty of bombs but no bomb hoists. Nothing found for Rabaul. B-17s were based 640 miles away and could not be sustained at Henderson (35,800 gallons by hand for twenty B-17s, Miller; over 3,500 gallons per bomber, Hough).

## 9. Positions

Henderson Field (Pacific Wrecks, -9.42806, 160.05472, the modern airport site), Vunakanau (Pacific Wrecks, labels printed reversed and no hemisphere, so read as 4 deg 19 min S 152 deg 8 min E, inferred) and the Rabaul Airfield Complex (Wikipedia, centre point, not Lakunai alone).

## Where sources disagree

- Distance Rabaul to Guadalcanal: 565 nm (Miller, Shaw and Kane), 586 nm if 675 statute miles (Craven and Cate), 483 or 556 nm for the 7 August escort (Wikipedia).
- 24 August: size of the Japanese force, losses on both sides, and whether it came from the carrier Ryujo (Hough, Wikipedia) or Rabaul at noon (Miller).
- Whether Rabaul raided Henderson on 1 to 8 September: "almost daily" at noon (Hough, Miller), none (Wikipedia citing Lundstrom p.189).
- 10 and 11 September: one raid or two (counts 27 and 30 against 26 and 8).
- 12 and 13 September: bombers and fighters sent, claimed and actual losses, and the attack times (Hough 0950, 1300, 1730; Miller 1020, 1320, 1750). Two reads of Miller chapter V attached one 20-fighter intercept to 12 and to 13 September.
- VF-5 arrival: 4 September (Wikipedia) or 11 September (Hough). VMF-224 and VMSB-231: 30 August (Shaw, Pacific Wrecks) or a date read as 21 August (Hough).
- Shaw's 22 August (13 of 16 bombers destroyed) and 29 August (Ryujo strike) entries match no other source. Ryujo was sunk on 24 August.
- 14 September floatplane raid: 3 lost (Pacific Wrecks) or 8 (Wikipedia).
- Model 32 range reduction: 20 percent (Lundstrom as quoted) or 620 miles (Nohara via Wikipedia).
- Drop tank: 330 L or 320 L.

## Looked for, not found

See the 18 new rows in `gaps.csv`: a stated radius for the escort; whether tanks were dropped; Japanese strength by type and date; raid composition for most days; Japanese losses by cause and the flight-home share; Japanese pilot losses in the window; US pilot replacements; fuel and bombs by date; Lakunai's own position.

## Added for stage 6e

Read on 2026-11-11. Three things were asked for. Nothing is settled here. Rows: `aircraft_type_facts.csv` 536 to 546, `weapons.csv` 1062, `weapon_facts.csv` 466 and 467, `engagement_events.csv` 290 to 296, `engagement_outcomes.csv` 497 to 499, `sources.csv` 1075 to 1078, six rows in `gaps.csv`.

### G4M1 Model 11: armour and fuel tanks (rows 536 to 542)

Every source read says the same thing: the type had no crew armour and unprotected wing tanks, left out to save weight for range. `pilot_armour` is `no` (Wikipedia, HistoryNet; both grade C, so the value stays `unconfirmed`). `self_sealing_tanks` is `no` (Pacific Wrecks, Wikipedia, HistoryNet, Pacific Eagles; Pacific Wrecks is graded B, so the rows read `preferred`). All are tertiary or enthusiast sources, so every row is `low`. No wartime Allied intelligence sheet on the Betty was found, though such sheets exist for the Zero (rows 455 to 465). The later fit is separate: Wikipedia's variants list gives March 1943 for rubber ply sheets under the wing outer surfaces and 5 mm plates for the tail gunner (row 540, a `performance_note`, because a `partial` value would have overlapped the `no` rows). HistoryNet and Pacific Wrecks say a later layer was added and give no date. Nothing read puts any protection on Model 11 before the window. Other sites give "the 663rd airframe" for the change; those were seen only as search summaries and are not entered. Wikipedia says self-sealing tanks came with the G4M2 and G4M3, but its own G4M2 entries do not describe them.

### Guns defending Henderson Field (events 290 to 296, outcomes 497 to 499)

`data/oob` has no table for the guns of a place or a land unit. `fits.csv` takes a ship class, ship or aircraft type as carrier, and `air_units` are flying units. The nearest fit used is `engagement_events.csv`: one row per statement, actor the battalion's antiaircraft group, target `henderson-field` (a `places.csv` key), `value` the count, the date in `as_stated` or `time` when the source gives one. This is a workable record of what was said, but it is not a base-defence table that a scenario can load by date. If the scenario needs the guns by date, a format decision is open: either a `land_units` identity file with its own facts, or `fits.csv` allowing `place` as a carrier type. Neither was done.

What the sources give:

| Fact | Value | Source | Date |
| --- | --- | --- | --- |
| First 90 mm battery | 4 guns, on the edge of the airfield | Zimmerman p.57 | None |
| 90 mm batteries | 2, with 3 automatic weapons batteries | Miller ch V | Before the end of August |
| Third 90 mm battery | 1 more, moved from Tulagi | Miller ch V; Hough VI-5 | Early September |
| 37 mm and .50 cal guns | Emplaced for antiboat fire, no counts | Zimmerman p.56 | After 9 August |
| Effect | Bombers forced to 25,000 ft; bombing still fairly accurate | Zimmerman pp.62-63 | August |

Not found: the number of guns beyond the first battery (eight and twelve are inferences from four a battery and are not entered), any count of 40 mm, 20 mm or .50 cal guns in an antiaircraft role (the 3d Defense Battalion's report of 7 March 1943 that Miller cites, and Condition Red's printed pages, were not in hand), and any date for the batteries becoming operational. Zimmerman's 12 August map does not date the battery. A search summary gave 12 August for the first battery; it was not read on a page and is not entered.

Antiaircraft kills: none is credited in the window. The histories give the window's kills to fighters. Two counts exist outside it: 2 by shore-based antiaircraft on 8 August (Hough, location of the guns not stated) and 5 of 27 on 25 October (Zimmerman). The 90 mm weapon has an entry (`us-90mm-m1-aa`) with calibre and a ceiling for the M1A1 (Wikipedia, `low`). The Marine histories read do not name the mark.

### Radius and bomb load (rows 543 to 546)

No source read gives a combat radius for the G4M1 with a bomb load, from Rabaul or anywhere. Wikipedia and HistoryNet give only the Navy's design requirement (3,700 km or 2,300 miles loaded, with an 800 kg torpedo or bombs), which is a target and not a result. The earlier rows stand: 1,540 nm with no condition, and 2,110 statute miles with normal fuel and bomb loads (late 1943, Allied summary). On the load: the maximum is 1,000 kg (earlier rows). For the Henderson raids of the window no load was found; the 8 August mission carried torpedoes. A forum post (row 545) says the standard Navy bomb was 60 kg, 6 x 60 kg was common on 1943 Solomons patrols, and mixed loads of one 250 kg and six 60 kg were used. Pacific Wrecks (row 546) says three Bettys dropped 60 kg and 250 kg bombs at Henderson on 23 March 1943, no count. All of these fall after the window.

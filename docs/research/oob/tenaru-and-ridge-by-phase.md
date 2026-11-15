# The Tenaru and Edson's Ridge, by phase

Status: a research report for the architect, read on 2026-11-15. Nothing here is settled and nothing here proposes a rule or a figure. The rows are in `engagement_outcomes.csv` ids 4056 to 4132 (engagement `rabaul-guadalcanal-air`, fights `battle-of-the-tenaru-1942-08-21` and `edsons-ridge-1942-09-12`) and `land_unit_facts.csv` ids 1186 to 1216. One new entity is in `land_units.csv` (1100, the 3d Battalion, 124th Infantry), three new sources in `sources.csv` (1276 to 1278) and fifteen entries in `gaps.csv` (the last fifteen lines).

## What was asked

The architect asked for the two ground fights to be taken apart by phase: which units and men were committed in each phase and which were held back, the counter-attack of 21 August (who ordered it, with what units, at what hours, what it cost each side as against the night), and the same for the three nights and the day between at the ridge. The rows already held (`engagement_outcomes.csv` 1742 to 1749, 1872 to 1880, 1912 to 2048, and the `land_unit_facts.csv` rows on the Ichiki and Kawaguchi detachments) give totals and one or two words of "committed". This pass looked for the phases inside them.

## How it was read

- **HyperWar transcriptions** (Hough, Ludwig and Shaw vol. I chapters 4 and 5; Miller chapters IV and V; Zimmerman chapters 4 and 5; Hoffman, *From Makin to Bougainville*): fetched as HTML and saved. Every quotation in a row from these was checked by program against the saved text, allowing only a change of whitespace, of curly to straight quotation marks and of the soft hyphens in the scans. All passed. The locator says the page marker the text follows (`--N--` in Hough, Miller and Hoffman, `page N` in Zimmerman). Hough's and Miller's markers do not say whether the number heads or closes the page, so the locators say "text after marker" and no printed page is claimed.
- **The 1st Marine Division's Phase IV report** (the division commander's own report, with its annexes), from the OCR text of the DTIC scan on archive.org. The OCR misreads letters and breaks words; each row says so, and quotations avoid the garbled stretches. The 1st Marines' history (Annex G), the 5th Marines' diary (Annex F) and the intelligence summary (Annex A) are part of it.
- **Shaw's commemorative pamphlet** (Gutenberg text), **Saito's two NIDS papers** (PDF text layers), **a wargamers' order-of-battle chart** (PDF text layer) and **Wikipedia** (both articles, saved as text; its notes read too).
- **Two pages were read only through the fetch tool,** which summarises the page and returns short quotations: Hoffman's *Silk Chutes and Hard Fighting* (the site refuses a direct download) and a history blog's article on the Tenaru. Rows from them are graded medium or low, say so, and were not checked by program.
- **Four pages did not open:** the Marine Corps University tank-doctrine article (HTTP 403), the Marine University PDF of Zimmerman (HTTP 403), history.navy.mil's Guadalcanal page (certificate error) and an Armor-school article on Benning (connection refused). Nothing is taken from them. Zimmerman was read in its HyperWar form instead.
- **Not read: Frank, *Guadalcanal* (1990),** nor Smith, *Bloody Ridge*, Griffith, Jersey, Christ or Alexander. Wikipedia and the blog rest on them for most of their phase detail, and say so in their notes.

The check shows the words are on the page. It does not show that the figure sits against the right unit; that filing is mine and the note on each row says what it rests on.

## The Tenaru, 20 to 21 August 1942

### Before the night: what was held back

| Statement | Source | Row |
| --- | --- | --- |
| Five infantry battalions in the Lunga perimeter: four on beach defence, one in division reserve (the 1st Battalion, 1st Marines). | Division report Phase IV; Hough | 4056, 4057 |
| Why the division kept it: sending it east meant accepting that the main Japanese force had landed there and could be dealt with by one battalion; if only part had been met, the reserve's absence "would become a serious manpower shortage in the perimeter". | Hough, Ch.4 | 4058, 4059 |
| Night of 20 to 21: the 1st Marines in line, the 2d Battalion (Pollock) at the river mouth with its right flank several hundred yards upstream, the 3d on its left to the Lunga, the 1st to the right rear in division reserve, not in prepared positions. The 2d Battalion's reserve was Company G. | Zimmerman Ch.4 | held row 1935; 4062 |
| The sand spit was held by one 37 mm gun of the Special Weapons Battalion, a few machine guns and a small group of riflemen; the position was not wired in. Wikipedia has 100 men of the Special Weapons Battalion with two 37 mm guns. | Division report; Wikipedia | 4060 |

### Phase 1, the first rush (0310; 0130 in two sources)

| Statement | Source | Row |
| --- | --- | --- |
| A column of some 200 rushed the sandspit at 0310. Division report: "some 200 enemy infantry attacking in mass formation". Miller and the 1st Marines annex also say about 200. | Hough; division report; Miller; Annex G | 4061, 4068; held 1923, 1925 |
| Most were stopped by small arms and the 37 mm gun; a few got into Pollock's lines; Company G counterattacked and wiped out the rest or drove them back across the river. | Hough; Zimmerman; division report | 4062; held 1914 |
| The main force of the Ichiki unit "remained on the point without giving any direct support" to the first rush. When it failed this force fired mortars and 70 mm guns. After the rush the Ichiki Force installed itself along the beach east of the river mouth. | Zimmerman; Hough; Miller | 4063, 4064, 4065 |
| The 1st Marines' history: "during the engagement the enemy exhibited no sign of reserve power". | Annex G | 4066 |
| Wikipedia: a first wave of about 100 of the 2nd Company after a green flare at 0130, covered by the rest. | Wikipedia | 1195 |

Pollock holds that every phase of the night took place somewhat earlier than the times in the Final Report (Zimmerman fn 52), and Annex G puts the first flare at 2030 on 20 August (Zimmerman fn 45). Shaw says 0130 (4132), and so does Wikipedia (1195). Rows 4075 and 4132.

### Phase 2, the second assault, and what stayed east of the creek

| Statement | Source | Row |
| --- | --- | --- |
| A second enemy company circled the river's mouth by wading beyond the breakers and charged the 2d Battalion's beach positions west of the mouth. Zimmerman: "about the size of a company". Broken up at the water's edge by machine guns, 37 mm canister and the 75 mm howitzers. | Hough; Zimmerman | 4067; held 1913 |
| Wikipedia: second assault at 0230 by the 1st and 3rd Companies, 150 to 200 men, "the last concentrated charge across the sandbar"; another wave at about 0500 by wading through the surf. | Wikipedia (citing Frank) | 1196, 1197 |
| Japanese attempts to break out: east along the beach (stopped by elements of Company C), south from the point (stopped by the Company D detachment), up the right bank of the creek (wiped out), and by sea in landing boats from Red Beach. | Zimmerman; division report; Annex G | 4070, 4071 |
| A platoon with machine guns stood near Block Four village, east of the Ilu and apart from the main body; Company C met it first. | Division report; Hough; Miller | 4069 |
| The blog's table of the first echelon: headquarters 164, battalion headquarters 23, four rifle companies of 105 (420), machine-gun company 110, battalion-gun platoon 50, engineer company 150, total 917 (citing Frank p. 147). It adds that the remaining rifle company and the engineer company would be in the rear. | The blog, via the fetch tool | 1188 to 1194 |

None of the sources read gives the number held back east of the creek. The Marine sources count the whole force at 900 or 1,000 landed (Hough, Miller, Zimmerman, Shaw, NIDS); the division report's estimate after mopping up is "about 1100 men" (4081); the report's intelligence annex says the attacking group was "approximately one battalion reinforced, virtually annihilated save for a small rear echelon left behind at Taivu" (4082). Saito says the first echelon was "approximately 900", with Detachment headquarters, an infantry battalion and two infantry guns (1186); and the detachment as a whole about 2,000 (1187).

### Phase 3, the counter-attack (daybreak to 1700)

| Statement | Source | Row |
| --- | --- | --- |
| By daybreak Pollock reported he could hold; Vandegrift ordered him to hold at the river mouth while the division reserve, the 1st Battalion, 1st Marines, enveloped Ichiki. | Hough | 4072 |
| The division report's stated basis: "The enemy force did not appear to exceed one reinforced battalion" and "it was decided to pass the division reserve battalion across the river". | Division report | 4073 |
| Zimmerman: Vandegrift decided on the envelopment because most of the enemy appeared concentrated in a small area, and Cates "ordered orally" the 1st Battalion to go upstream, cross, and attack northwest on the right bank. Zimmerman puts this after the artillery concentrations at 0400 and 0515. | Zimmerman | 4074 |
| Before 0700 Cresswell crossed upstream and posted Company D elements to the south; by 0900 his companies crossed their lines of departure (Hough); in position to begin the attack at 0950 (division report). | Hough; division report | 4076 |
| The battalion's own layout: Companies A and C each with a platoon of Company D in line, Company B with the mortar and a machine-gun platoon as reserve, the rest of Company D as riflemen. | Zimmerman | 4076 |
| The 3d Battalion, 11th Marines fired on the beach point at 0403, 0515, 0722, 0742 and 0851. Company C, 1st Engineers, strengthened Pollock's line in the morning. | Miller; Hough | 4128, 4077 |
| By 1230 Companies C and A had advanced over 2,000 yards, Company C at the mouth of the Block Four River; at 1400 the enemy was surrounded and the battalion assaulted. | Miller; division report | 4129 |
| A platoon of light tanks, supported by infantry, crossed the sand bar at 1500. Two tanks were disabled. Over by 1700. Shaw says five tanks; Wikipedia four or five. | Hough; Miller; division report; Shaw; Wikipedia | 4078 to 4080, 4130 |

### Losses

No source read splits the losses of the night from those of the afternoon. The Marine total is 34 killed and 75 wounded (division report, Hough, Zimmerman, Annex G); Miller says 35 killed; the blog 44 and 71 (4083). Annex G splits the American dead by battalion only: 25 in the 2d (the night's defenders), 7 in the 1st (the envelopment), 2 in the Special Weapons Battalion (held rows 1940 to 1949). The Japanese total is "nearly 800" (Hough), "about 900" (division report), 777 bodies counted (blog, 1200); survivors "little more than 100" (Japanese Monograph 45), 130 (Miller), "about 128" back at Taivu by 29 August (1199), about thirty (Wikipedia, 1198).

## Edson's Ridge, 12 to 14 September 1942

### Before the first night

| Statement | Source | Row |
| --- | --- | --- |
| The inland line, 4,000 yards of outposts, was held by the 1st Marines, artillerymen, engineers, most of the pioneers and the raider-parachute battalion, which Vandegrift took out of division reserve. The reserve was then 2d Battalion, 5th Marines. | Hough; Miller; Hoffman | 4084, 4089 |
| Edson's unit on 11 September: 840 men (Wikipedia, text) or about 600 Raiders and 214 Paramarines (its note 30). The division report says "some 300 Marines" were on the ridge on the second night, after two days' fighting. | Wikipedia; division report | 4092, 4125, 4091 |
| Japanese plan of 6 September as Kawaguchi gave it to the 17th Army: attack at 1600 on 12 September from the south, the Oka force attacking from the coast on the west. | Saito 2013 | 1202 |
| Hoffman: Kawaguchi detailed one of his four battalions to make a diversionary attack along the Tenaru, leaving about 2,500 for the main assault. The battalion is not named. | Hoffman | 4094 |
| Order of battle from Frank by a wargamers' chart: Japanese 1/124th (Kokusho), 3/124th (Watanabe), 2/4th (Tamura); American 1st Parachute and 1st Raider Battalions, 1st Pioneer Battalion, 2/5 and two engineer companies arriving later. | Chart | 1206 |
| Division report: the force against the ridge "undoubtedly exceeded 2000 men". | Division report | 4093 |

### Night of 12 to 13 September (first night)

| Statement | Source | Row |
| --- | --- | --- |
| American line, Zimmerman: two understrength parachute companies on the left tied to the engineers, Raider Company B on their right, one platoon of Company C at the lagoon, the rest of Company C to the Lunga. Hoffman: one parachute company forward and the rest echeloned to the rear, two raider companies on the right, the remaining raiders in reserve, Company D no larger than a platoon. About half the troops in the parachute position withdrew in the night (Griffith, in Zimmerman fn 57). | Zimmerman; Hoffman | 4085 to 4088, 4100 |
| Hoffman: only one battalion had reached its jump-off point; Kawaguchi could not pass the order to delay. Wikipedia: the three Center Body battalions arrived after 22:00 against a plan for 14:00; only the Kuma Battalion was on time. | Hoffman; Wikipedia | held 859; 1213 |
| Hough: the enemy probed lightly, one desultory attack made a slight penetration that was not exploited. Division report: "a continuous and confusing series of encounters". Zimmerman: two strong thrusts down the east bank of the lagoon and a third, cutting off two left platoons of Company C. Shaw: a first thrust at 2100 on the left, one on the right with a penetration, a third; Edson said at 0230 he could hold. Hoffman: piecemeal attacks round the lagoon; the Marines on the ridge "remained comparatively untouched". | Hough; division report; Zimmerman; Shaw; Hoffman | 4096 to 4099 |
| Attackers by Wikipedia: Kokusho's 1st Battalion overran a platoon of Company C; entangled with Watanabe's 3d Battalion. | Wikipedia | 1207, 1209 |
| American losses for the night: twelve killed (Wikipedia, citing Frank). No primary gives a figure for this night alone. | Wikipedia | 4101 |

### Day of 13 September: the counter-attack and the withdrawal

| Statement | Source | Row |
| --- | --- | --- |
| Hough: early on the 13th Edson launched a counteroffensive, found the enemy too strong, and in the afternoon withdrew north. | Hough | 4102 |
| Zimmerman: Companies A and D, not yet engaged, sent up the right bank of the Lunga to eliminate the salient; failed; Company D "had almost ceased to exist". | Zimmerman | 4103 |
| Hoffman (Makin): "a counterattack by his reserve companies". Hoffman (Silk Chutes, via the fetch tool): "by the raiders of Company B and the parachutists of Company A". Division report: "the Raiders again attempted to advance". | Hoffman; division report | 4104, 4105, 4106 |
| Edson to his company commanders, as Shaw has it: "They were just testing, just testing. They'll be back." | Shaw | 4107 |
| Late afternoon: the B Companies of both battalions pulled back to the ridge between Hills 80 and 120, an engineer company inserted on the right, Raider Company A to the Lunga, the other two parachute companies on the left, the remains of Companies C and D in reserve behind Hill 120. The 2d Battalion, 5th Marines moved to a position along the south of the airfield. | Hoffman; division report; Hough | 4104, 4089 |
| Hough: Raider Company C was Edson's reserve on the northern knoll. | Hough | 4090 |

### Night of 13 to 14 September (second night)

| Statement | Source | Row |
| --- | --- | --- |
| Onset: 1830 (Zimmerman), shortly after nightfall (Miller), 2000 (Saito), after Washing-Machine Charley's flare about 2100 (Hough). | several | 4108 |
| Japanese: Saito has the attack in three groups, right, centre and left, the right and left stopped by wire, and the centre two battalions abreast with the Aoba Battalion (2d Battalion, 4th Infantry) behind them leapfrogging to a point just short of the division headquarters. Wikipedia: Kokusho's battalion against Raider Company B, then toward the airfield, Kokusho and about 100 killed; Tamura's battalion (about 320 in two companies on Hill 80); one company of Watanabe's through a gap, most of Watanabe's battalion not in the attack. | Saito 2013; Wikipedia | 1203 to 1205, 1208, 1210, 1211 |
| Zimmerman: Kawaguchi gave only passing attention to the left flank and did not follow through on the right; Hough: two battalions against the centre and right, at least two battalions by 2200, "more than a dozen" assaults in the night. | Zimmerman; Hough | 4095; held 2001, 2014, 2015 |
| American reserve moves. About 2100 Edson ordered Raider Company C and Parachute Company A to form a reserve line round Hill 120. At 2230 the parachutists were driven off their spur; Edson ordered Torgerson to counter-attack with two reorganized parachute companies. At 0400 Edson asked for the reserve battalion; the companies of 2/5 came up one at a time, Company G first. | Hoffman; Hough; Zimmerman | 4109 to 4112, 4116, 4131 |
| Kawaguchi sent in two more attacks late which artillery broke up as the troops assembled. | Hoffman | 4113 |
| 105 mm: 1,992 rounds, from 2100 on the 13th to dawn (Hough, Miller), or "in defense of the ridge" without a night (Zimmerman); the division report says 2,000. 75 mm: 878 by the 2d and 3d Battalions, 11th Marines, at other positions. | Hough; Miller; Zimmerman | 4114, 4115 |
| Daybreak: three P-400s strafed the survivors; Hough says the disorganized Japanese were "bombed and strafed into retreat". | Hough; Zimmerman | 4122 |

### The flank fights

| Statement | Source | Row |
| --- | --- | --- |
| East, 3d Battalion, 1st Marines: at midnight an attack across the plain on the extreme right flank company; by morning at least one battalion with some artillery held the woods. About two companies attacked (Miller); one battalion reinforced with 70 mm guns and antitank guns (Annex G); 28 dead along K Company's wire; 200 estimated killed. Zimmerman names the Japanese 1st Battalion, 124th; Miller says the Ichiki rear echelon "may have attacked"; Wikipedia the Kuma Battalion (Mizuno). | division report; Miller; Annex G; Zimmerman; Wikipedia | 4117, 4118, 4120, 1212, 1216 |
| Six light tanks sent across the plain in daylight without infantry (Hough, Miller). Three or four lost to antitank guns, one into the Tenaru. The 1st Marines' annex says five tanks, which "returned unharmed" in a first assault and lost three in a second. | Hough; Miller; Annex G | 4119, 4120 |
| West, 3d Battalion, 5th Marines: 5th Marines' diary: at 0330 on the 14th attacked by a force "estimated to be about 1000", about 40 killed. Miller: "a third Japanese force, probably under Colonel Oka's command, struck on the afternoon of 14 September". Wikipedia: Oka's 650, two companies at about 0400, another company on a small ridge; the rest of Oka's unit did not find the Marine lines. | Annex F; Miller; Wikipedia | 4121, 1214 |

### Losses and aftermath

The 1st Raiders lost 135 and the 1st Parachute Battalion 128 over the two nights, 59 dead or missing (Hoffman, both works; 4123). Wikipedia's note: 86 of 240 deployed parachutists walked off (4124); Shaw: 89 (held 494). Division report 31 killed, 103 wounded, 9 missing; Shaw 59, 194 and 10 (held rows 2039 to 2047). No source splits them by night.

Japanese: "about 600" dead on the ridge (Miller, division report), 633 killed and 505 wounded in the Japanese report (held 2024, 2031). Wikipedia alone gives losses by battalion: Tamura's 350, Kokusho's 200, Oka's 120, Mizuno's 100, Watanabe's 60, about 830 in all (4126). Zimmerman: the 1st Battalion, 124th, and the 2d Battalion, 4th, badly hit, the Ichiki balance wiped out (4127).

## Where sources disagree

| Point | One account | Another |
| --- | --- | --- |
| Hour of the first rush at the Ilu | 0310 (Hough, Miller, Zimmerman, division report, Annex G) | 0130 (Shaw, 4132; Wikipedia, 1195). Pollock: all times in the report are too late (4075). |
| Size of the first rush | about 200 (all the Marine and Army accounts) | about 100 of the 2nd Company, then 150 to 200 of the 1st and 3rd (Wikipedia, blog, citing Frank) |
| Number of rushes | one rush and one company circling by sea (Hough) | three or four waves at 0130, 0230 and 0500 (Wikipedia) |
| Who ordered the envelopment | Vandegrift (Hough, Zimmerman), Cates orally (Zimmerman) | the commanders on the spot, "conferred" (Wikipedia) |
| Marine dead at the Tenaru | 34 (four Marine sources), 35 (Miller) | 44 (blog) |
| Tanks at the Tenaru | a platoon (Hough, Miller); five (Shaw) | four or five (Wikipedia) |
| Intensity of the night of 12 to 13 September | light probing (Hough, Hoffman, division report) | three heavy assaults (Zimmerman, Shaw) |
| Who counter-attacked on the morning of 13 September | Raider B and Parachute A (Hoffman, Silk Chutes) | "reserve companies" (Hoffman, Makin); A and D (Zimmerman) |
| Onset of the attack on 13 September | 1830 (Zimmerman) | "shortly after nightfall" (Hough, Miller), 2000 (Saito) |
| Marines on the ridge | about 300 (division report, second night) | 840 deployed, 830 the second night (Wikipedia) |
| What attacked the east flank | 1st Battalion, 124th (Zimmerman) | Ichiki's rear echelon, "may have" (Miller); the Kuma Battalion (Wikipedia) |
| When the west flank was hit | at 0330 on the 14th (5th Marines' diary) | in the afternoon of the 14th (Miller), about 0400 (Wikipedia) |
| Why Watanabe's battalion did not attack | lost contact with its commander (Wikipedia text) | for unknown reasons stayed in place (Wikipedia note 45, from Frank) |

Nothing is reconciled here.

## What is a gap

All in `gaps.csv`. The ones that matter most:

- The strength of each Japanese rush at the Ilu and of what stayed east of the creek, from a Japanese record. The only company-level roster is a blog's, citing Frank.
- The loss of each side split between the night and the afternoon counter-attack at the Tenaru. Every source gives a day total.
- The text and hour of the order for the envelopment.
- The strength in men of each Japanese battalion at the ridge, and which battalion was sent as the diversion on the Tenaru.
- American strength by company on each night, and American losses by night.
- A Japanese report naming the battalions committed each night and those held back. Japanese Monograph 34 is not online; Kawaguchi's own report was not found.
- Frank (1990) was not available. Most of the phase and battalion detail above that comes from Wikipedia or the blog rests on it.

## Rows that would not fit the vocabulary

`land_unit_facts.csv` and the measures of `engagement_outcomes.csv` have no attribute for "held back", for a commander's stated reason, or for losses by phase. Held-back statements are filed under a new measure `jp_held_back_note`; reasons and decisions are `action_note`; losses by battalion are `note` rows on the battalion entity (Wikipedia's figures are in the `jp_casualties_total_note` row 4126). `us_artillery_rounds_fired` is a new measure for the two rounds-fired rows. None of the new measures is in a closed list; `oob-check` does not check them.

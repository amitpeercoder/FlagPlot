# Land forces on Guadalcanal and Tulagi with their parents, 7 August to 14 September 1942

Status: a research report for the architect, read on 2026-11-15. Nothing here is settled and nothing here proposes a rule or a figure. Every row is `unconfirmed`. The rows are in `land_unit_facts.csv` ids 1400 to 1583 (184 rows), 39 new entities in `land_units.csv` (ids 1101 to 1146) and 17 entries at the end of `gaps.csv`.

## What was asked

A later rule will address a commander's intent to a formation, so each land unit of either side needs its parent (division or detachment, then regiment, then battalion, and attached units), and the dates it was under that parent where that changed. The earlier passes (`land-forces.md`, the two outside-research notes, `tenaru-and-ridge-by-phase.md`) have strengths, landings, commanders and moves, and 29 `parent_stated` rows with the parent as free text. This pass adds the chain of command.

## The attributes

A unit can have several parents at once, and the sources do not mean the same thing by them. The rows say which kind they are.

| Attribute | Value | Means |
| --- | --- | --- |
| `parent_stated` | entity key | The standing parent, where a source says the unit belongs to it (or its own name does). Earlier rows use free text for the value; these use the parent's key. |
| `attached_to` | entity key | A unit from outside its parent, attached or reinforcing, with dates. |
| `task_org_parent` | entity key | The superior in a task organization an order or history states, for a period or an operation. |
| `listed_under` | entity key | The unit is printed under that heading in a roster (Hough's Appendix C) and no source says more. Not read as organic. |
| `direct_support_of` | entity key | An artillery battalion in direct support of that unit. |

The `parent` column of `land_units.csv` is filled for 16 existing units whose parent was blank and is stated by a row of this pass (the 1st, 5th, 7th and 11th Marines, the division's separate battalions, the 124th Infantry, the 4th Infantry, the Kawaguchi Detachment and the 2d Marines). It is left blank for units that were attached or whose parent is disputed (the raiders, the 3d Defense Battalion, the Ichiki Detachment, the construction units). No existing row, id or key was changed.

## How it was read

Pages were fetched plainly with `curl` (the default agent), saved, and each quotation was checked by program against the saved text, allowing only a change of whitespace, of curly to straight quotation marks and of soft hyphens. All 184 passed. The locator gives the page marker the text follows (`--N--` in Hough, Miller, Shaw and the monographs, `page N` in Zimmerman); as in the earlier notes, the marker does not say whether the number heads or closes the page, so the locators say "text after marker" and no printed page is claimed. No site refused a request. Two pages did not open for a reason that is not a refusal: `niehorster.org` (the Japanese and American orders of battle of that site) failed its certificate check (curl exit 60, "no alternative certificate subject name"), and the archive.org text of the MacArthur Reports returned only about 70,000 characters of the volume, with no passage on the Kawaguchi Detachment. Nothing was taken from either, and the certificate check was not turned off.

Sources used, by rows: Hough, Ludwig and Shaw vol. I (90, including Appendix E, the division's Operation Order 7-42, and Appendix C, the task organization and command list), Miller (27), Zimmerman (6), Shaw's pamphlet (10), Japanese Monograph 45 (4), Wikipedia (31, graded low, resting on Frank and Smith), the Nafziger charts (9, graded low, resting on Frank), and the translated diary of a platoon leader of the 8th Independent Antitank Gun Company (7, graded low). Not read: Frank, *Guadalcanal*; Smith, *Bloody Ridge*; Christ, Alexander, Griffith, Jersey; the division's final report annexes for this pass; the Senshi Sosho; Japanese Monograph 34. Wikipedia and the Nafziger charts are the only sources read for most of the Japanese grouping at the ridge, and both rest on Frank.

## The American tree

### The 1st Marine Division as it landed (Operation Order 7-42, 20 July 1942)

The order (Hough, Appendix E) gives the task organization. Rows 1400 to 1417 put each group under the division and each unit in its group.

```
1st Marine Division (Vandegrift)
  Combat Group A: 5th Marines reinforced, less Combat Team 2 (Hunt)
      Combat Team 1 = 1/5, Combat Team 3 = 3/5, 5th Marines headquarters
  Combat Group B: 1st Marines reinforced (Cates)
      Combat Teams 4, 5, 6 = 1/1, 2/1, 3/1
  Tulagi Group (Edson): 1st Raider Battalion + Combat Team 2 (= 2/5, less Battery E, 11th Marines)
  Gavutu Group (Williams): 1st Parachute Battalion
  Support Group (del Valle): 11th Marines (headquarters and 5/11), 1st Engineer Bn (less A, B, C),
      1st Special Weapons Bn (less two platoons), 1st Pioneer Bn (less A, B); the 3d Defense Battalion
      to pass to its control on landing
  Division Reserve (Arthur): 2d Marines reinforced, less Combat Team A
  Florida Group (Hill): Combat Team A = 1/2
  3d Defense Battalion (Pepper): a group of its own in the order
```

Combat teams and groups were set up on 29 June (Miller): each combat group had one infantry regiment, one artillery battalion, one company each of the tank, engineer, pioneer, amphibian tractor and medical battalions, and scout, special weapons and transport platoons; each combat team had one infantry battalion, one field artillery battery and platoons of engineer, pioneer and amphibian tractor personnel.

Above the groups on the Tulagi side stood the Assistant Division Commander, Brigadier General Rupertus (Shaw: he "led the assault forces slated to take Tulagi, Gavutu, and Tanambogo"). I filed that as an entity of its own, "Rupertus' command, Tulagi area" (1118), with the raiders, the parachutists and 2/5 under it from 7 August to the day each crossed to Guadalcanal (21 August; 31 August), and the 2d Marines' battalions attached from 8 August. The sources call it "General Rupertus' command" and "Rupertus' forces"; none gives it a name or an end date.

### Attached from outside the division

- **The 2d Marines** came from the 2d Marine Division (Roster, App. C; King's proposal of 27 June). Miller: the order "organized the reinforcing units" (the 2d Marines, the 1st Raider Battalion, the 3d Defense Battalion) "which had not then joined the division". Its three battalions were committed on 8 and 9 August "by orders from Rupertus direct to the battalion commander in each case and not through the normal chain of command" (Zimmerman; Hough's note names the same letter); the regimental headquarters stayed afloat and went to Espiritu Santo on 9 August with about 1,390 men. Turner claimed the regiment as his own (Hough: his "unrestricted claim of 'possession'"); Miller says Vandegrift persuaded him in August that it should remain with the 1st Division. The part left at Espiritu Santo became a "2d Provisional Raider Battalion", disbanded 22 September.
- **The 2d Marines' battalions:** 1/2 (Florida Group, then Tulagi from 9 August), 2/2 (Tulagi, 9 August), 3/2 (Gavutu, then Tanambogo, from 8 August; "took over" Gavutu, Tanambogo and Makambo on 9 August; to Guadalcanal 14 to 16 September). Company B of 1/2 was ordered to report to the parachute battalion on Gavutu on 7 August.
- **The 2d Marines' reinforcing elements:** 3/10 (75 mm, with Batteries H and I at Tulagi; Battery G and the headquarters batteries went away on 9 August), Company C of the 2d Tank Battalion, Company A of the 2d Amphibian Tractor Battalion, Company A of the 2d Engineer and 2d Pioneer Battalions. The last four are inferred from Miller's list of the regiment's reinforcing troops and the TF 62 list of men at Tulagi.
- **The 1st Raider Battalion and the 3d Defense Battalion** "had been added to Vandegrift's command" (Shaw). The raiders were organic to no regiment of the division; Hough's Appendix C lists them with the division.
- **The 7th Marines** (third regiment of the division) had been detached to Samoa on 21 March (row 622). On 26 July Callaghan told the commanders at Koro that Ghormley had ordered it to be ready to embark on four days' notice as a reinforcement. It was not in the landing, not on the island until 18 September (4,180, 4,262 or 3,665 men by source, rows 612 to 614), and so was not under the division's command at the ridge. Turner organized Task Force 65 to carry it (sailed 14 September; Ghormley ordered it to reinforce the garrison on 13 September on Turner's report of 12 September); its 75 mm artillery battalion, 1/11, came with it. Hough's roster dates it from 18 September.

### The division's own troops, 9 August

TF 62's war diary lists what was left (Hough Ch.2): on Guadalcanal about 10,000, the 5th Marines less 2/5, the 1st Marines, the 11th Marines less Battery E and the 1st and 4th Battalions, the 1st Tank, Engineer, Pioneer, Amphibian Tractor, Service and Medical Battalions "less detachments", the headquarters, signal and military police companies, part of the 2d Platoon of the 1st Scout Company, units of the 3d Defense Battalion; on Tulagi 6,075, the raiders, parachutists, 2/5 (with a pioneer platoon attached), the three battalions of the 2d Marines, Batteries H and I of 3/10, a part of the 3d Defense Battalion and the 2d Marine Division's company elements named above. These are rows `listed_under` from Appendix C and `attached_to` from the table.

### The artillery

The 11th Marines' five battalions: 1/11 and 4/11 were not in the landing (4/11, the 155 mm battalion, stayed in New Zealand with the rear echelon; 1/11, a 75 mm battalion, came with the 7th Marines on 18 September); 2/11 and 3/11 (75 mm) "came ashore with the assault battalions of the 5th and 1st Marines" and reverted to the regiment's headquarters when it landed; 5/11 (105 mm) was general support. 5/11 was in direct support of Edson's force at the ridge (Hough). The first statement of which battalion supports which regiment is for the 26 September operation (1/11 with the 7th Marines, 2/11 with the 5th, 5/11 with the Whaling Group, 3/11 for the perimeter), outside the window.

### After the landing: the moves that changed the tree

| Date | Move | Row |
| --- | --- | --- |
| 21 Aug | 2/5 crosses from Tulagi, becomes the 5th Marines' reserve | 1514 |
| 31 Aug | Raiders and parachutists cross; consolidated on 2 Sept into a provisional battalion under Edson (Hough; Miller) | 1510 to 1512 |
| Early Sept | A 90 mm battery of the 3d Defense Battalion crosses from Tulagi to Guadalcanal; a detachment of the 5th Defense Battalion lands at Tulagi on 8 Sept | 1518 to 1520 |
| 11 to 12 Sept | The provisional battalion leaves "division reserve" for the ridge south of the airfield | 1513 |
| 13 to 14 Sept | At the ridge: Edson's two battalions; 5/11 in direct support; 2/5 in reserve, its companies moving in from 0400 on the 14th | 1463, 1514, 1515 |
| 14 to 16 Sept | 3/2 crosses from Tulagi | no new row (Zimmerman and Miller, in the earlier notes) |
| 18 Sept | 7th Marines and 1/11 land; the parachute battalion leaves | 1480, 1505 to 1509 |

The ridge on the American side, per two charts: the Nafziger chart gives the 1st Parachute Battalion (headquarters, A, B, C Companies), the 1st Raider Battalion (A to E), the 1st Pioneer Battalion and the 1st Amphibian Tractor Battalion (not engaged), and "arrived later" 2/5 and Companies C and D of the engineers. Wikipedia's chart adds the 1st Tank Battalion, 3/1 and 3/11. Hough adds the engineers' and pioneers' positions on the flanks and the 5th Battalion. These are notes on the division, not parent rows.

## The Japanese tree

### The garrison on 7 August

Parents for the naval units come from two weak sources (the Nafziger chart of 7 August, which names Frank as its only source, and Wikipedia's footnotes), and no American official history gives them.

```
11th Air Fleet (Base Air Force; Tsukahara)
  25th Air Flotilla (Rabaul)
    Yokohama Air Group (Tulagi, Gavutu): flying boats and floatplanes   [Wikipedia, citing Lundstrom]
Eighth Fleet (Mikawa)
  8th Base Force: 81st, 82d, 84th Guard Units                           [Nafziger chart]
  attached: 3d Kure SNLF, Sasebo 5th SNLF, 10th to 15th Airfield and Base Construction Units  [Nafziger]
  84th Guard Unit reported to the Eighth Fleet; the Kure 3d SNLF "formed" it   [Wikipedia]
```

On Guadalcanal: the 11th and 13th Construction Units and a guard unit (the 81st per Nafziger, the 84th per Wikipedia), under Captain Kanae Monzen (Wikipedia; no American history names him), with the 1st Camp Relief Unit by 12 August. At Tulagi and Gavutu: the Kure 3d SNLF in two groups (Lt. Maruyama, Lt. (j.g.) Yoshimoto; Zimmerman), the Yokohama Air Group and the 14th Construction Unit. The Sasebo 5th SNLF was not ashore: its ship was sunk on 8 August (Zimmerman). The Yokosuka 5th SNLF landed 16 to 18 August and no source read gives its parent (a gap).

### The Ichiki Detachment

The Army General Staff gave the 17th Army the Ichiki Detachment, the 35th Brigade and the Aoba Detachment on 23 August (Japanese Monograph 45, rows 1521 to 1523); the detachment had been put under the 17th Army on 10 August (rows 253, 254) and attached to the 35th Brigade at Truk on 12 August (row 255).

```
7th Division
  28th Infantry Regiment
    Ichiki Detachment (Colonel Ichiki)  = the reinforced 2d Battalion (Hough, Miller, Zimmerman), 2,000 men
      first echelon / "advance unit" (about 900; landed 18 Aug) + engineer company
      second echelon / "rear echelon" (the Kuma battalion, Major Mizuno)
         8th Independent Antitank Gun Company (from the 16th Division's 9th Regiment; attached to the Ichiki Detachment 19 May)
```

### The Kawaguchi Detachment (35th Brigade) at the ridge, 12 to 14 September

```
17th Army
  Kawaguchi Detachment (35th Infantry Brigade, Maj. Gen. Kawaguchi)
    Center Body (Kawaguchi; Wikipedia's name), about 3,000:
      1st Battalion, 124th Infantry (Kokusho)
      3d Battalion, 124th Infantry (Watanabe)
      2d Battalion, 4th (Aoba) Infantry (Tamura), attached from the 2d Division
    East body: Kuma battalion (the Ichiki second echelon, Major Mizuno)
    West body: Oka group (124th Infantry headquarters, Colonel Oka, and the 2d Battalion, 124th, which lost its commander Takamatsu to the air attacks on the barges)
    Artillery group (from 7 Sept; Captain Nakaoka): the 8th Independent Antitank Gun Company, a regimental gun platoon of the 124th, machine-gun platoons of the 4th and the 124th   [diary only]
    plus engineers, signal, antitank and artillery elements in unnamed parts
```

The 3d Battalion, 4th (Aoba) Infantry landed at Kamimbo on 11 September (row 942) and did not take part; Miller says the rest of the Aoba regiment came by the end of September. Hough has Kawaguchi land at Tasimboko "with the Ichiki rear echelon and the 1st and 3d Battalions, 124th" and Oka "with the remainder of the force--the 2d Battalion, 124th" at Kokumbona; Wikipedia's plan of 7 September is the three bodies above. Inui's diary puts the Kuma battalion under Kawaguchi from 1 September and the antitank company under his direct control on 4 September. Hough and Miller have Kawaguchi's brigade "built around" the 124th, with the Ichiki rear echelon and the 2d Battalion, 4th, added.

## Where the sources disagree

| Question | Source A | Source B | Rows |
| --- | --- | --- | --- |
| What the Ichiki Detachment was | The reinforced 2d Battalion of the 28th Infantry (Hough, Miller; "one reinforced battalion of the 28th Infantry", Zimmerman) | The 28th Infantry Regiment (Wikipedia text; Monograph 45 says "one infantry regiment" for the Midway force) | 1524, 1526 to 1528; row 252 |
| Which battalion of the 28th the first echelon was | 2d Battalion (Hough, Miller; Wikipedia's own table of the detachment) | 1st Battalion under Major Kuramoto (a Wikipedia footnote citing Frank and others) | 1529, 1530 |
| Who attacked 3/1 on the east flank, 13 to 14 Sept | The 1st Battalion, 124th (Zimmerman, row 1014) | The Ichiki rear echelon, "possibly" (Hough), "may have" (Miller); the Kuma battalion (Wikipedia) | 1563 to 1565 |
| Who was "Tamura's battalion", and who was with Oka | Wikipedia's text calls Tamura's battalion "Kawaguchi's 2nd Battalion"; its own footnote says it was the 2d of the 4th and the 2d of the 124th was with Oka | The Hough and Miller accounts put the 2d of the 124th with Oka and the 2d of the 4th in the main body | 1552 to 1555, 1566 |
| Who commanded the 1st Battalion, 124th | Ishitari (Zimmerman, row 14) | Kokusho (the Nafziger chart, Wikipedia, Inui; rows 1206, 1207) | existing |
| The guard unit on Guadalcanal on 7 August | The 81st Guard Unit (Nafziger) | The 84th Guard Unit (Wikipedia, for 12 August, and at Tulagi on 7 August) | 1573 to 1575, 1582 |
| Who commanded the Japanese on Guadalcanal | Captain Kanae Monzen for the naval units (Wikipedia, 7 to 8 August) | Kawaguchi from 31 August "in command of all the Japanese troops" (Wikipedia); Miller has his own force divided in two | 1581, 1583 |
| Whether Battery G, 10th Marines, was ashore at Tulagi | Aboard with the regimental headquarters on 9 Aug; returned 9 Oct (Hough; Zimmerman Ch.III) | Firing at a submarine from Tulagi, undated (Zimmerman footnote to Ch.V) | 1451 |
| Which battalion of the 11th Marines supported Edson | The 5th (Hough, Miller) | The 3d (Wikipedia's list of units involved) | 1463, 1516 |
| Where the 5th Defense Battalion's detachment came from | Landed at Tulagi 8 Sept (Hough App. C) | Part of the battalion left Samoa with the 7th Marines on 2 Sept and stayed at Espiritu Santo (Miller) | 1484, 1520, gap |

On the American side the order of 20 July, Hough's roster and the TF 62 list agree on every infantry regiment and battalion; the differences are in the artillery battalions and a few attachments, in the table. The larger differences are in the Japanese attacking forces.

## What is missing

In `gaps.csv`, with where each would be found. The ones that matter most for a rule that addresses a formation:

- **No Japanese record of the Japanese order of battle.** Everything above for the Ichiki Detachment, the Kawaguchi Detachment's make-up and the garrison's parents rests on American histories that drew on Japanese documents, on Wikipedia and on one chart that cite Frank, and on one diary. Japanese Monograph 34 (the 17th Army's operations) is the likeliest source and was not found online. The Senshi Sosho volume would settle which battalion of the 28th, the guard units' numbers, and the construction units' parent.
- **The Yokosuka 5th SNLF's parent.**
- **The command of Tulagi after about 22 August,** when Colonel Arthur returned: whether the 2d Marines' headquarters replaced Rupertus, and when the Tulagi Group was dissolved.
- **Whose command the Seabees, Cub-1 and the 5th Defense Battalion's detachment were under.**
- **The 1/11 and 4/11 arrival dates and whether 2/11 and 3/11 were paired with the combat groups.** The division's Phase II report annexes (the artillery annex) are the place.
- **Names of the Kawaguchi Detachment's engineer, signal, artillery and antitank units in September.** One diary gives the antitank company and the gun platoons under Nakaoka.
- **The batteries of the 3d Defense Battalion by group and date; the other "division troops" with the 7th Marines on 18 September; the division's headquarters, signal, police and scout companies as units.**

## For the architect

- **`listed_under` is not `parent_stated`.** Hough's Appendix C is the only roster read for most of the battalions-under-regiments rows and says no more than its nesting. The relations are not in doubt for the 1st, 5th, 7th and 11th Marines and their battalions; they are inferred from a list. A rule that addresses an intent to a formation could read `listed_under` for the American side where no row says otherwise, and should read `task_org_parent` for the dated groups of the order.
- **The American chain is complete and sourced to the division's own order and to Hough**; the dates are those of the order (20 July), of the landing (7 August) and of the moves above, with end dates left blank where no source gives one.
- **The Japanese chain below the 17th Army is thin and low-grade.** The task groups (Center Body, East body, Oka group, artillery group) are the best available reading and are marked as drawn from Wikipedia and a diary.
- **Entities added:** `us-2d-marine-division`, `us-10th-marines`, the five battalions of the 11th Marines, two batteries of 3/10, four company elements of the 2d Marine Division and two of the 1st, the 1st Medical and Service Battalions, `us-rupertus-command-tulagi`, the 3d Barrage Balloon Squadron, two defense battalion detachments, `us-task-force-65`; on the Japanese side the Eighth Fleet, the 25th Air Flotilla, the 11th Air Fleet, the Yokohama Air Group, the 81st and 84th Guard Units, the 8th Base Force, the 14th Construction Unit, the 7th Division, the 28th Infantry and its 1st and 2d Battalions, the 2d Battalion of the 124th, the Center Body, the artillery group, the 8th Independent Antitank Gun Company and the Ichiki engineer company.
- **Row ids** run from 1400 to 1583 (not contiguous with the previous last id, 1386, so a parallel branch that added rows is not in conflict).

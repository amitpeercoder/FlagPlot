# Shipboard anti-aircraft fire, South Pacific, Aug 1942 - Feb 1943

Status: research note. Design input for scenarios 3 to 5; it proposes nothing binding.

How to read it: drafted by a research subagent from pages read on 2026-11-10, then reviewed. Each statement carries its source. The Navy's own history site would not load that day, so some points rest on Wikipedia or a single website; those are marked "unverified", "W" or "WP lead" and should be checked against a primary source before a rule relies on them.

Wikipedia is flagged "(WP lead)" wherever used.

## US guns

- The 5in/38 entered service in 1934 and was paired with the Mark 37 director on most ships built 1939-46. In 1941 trials North Carolina downed drones at 12,000-13,000 ft, about twice the reach of the 5in/25. Rate was 12-22 rounds a minute depending on mount. [5"/38 Mark 12, NavWeaps, https://www.navweaps.com/Weapons/WNUS_5-38_mk12.php, "Description", "Rates of Fire"]
- The 1.1in/75 reached ships from 1937. It was unreliable, jammed, and its fuzes were so sensitive that bore prematures were common. It was manual-aimed, effective range seldom exceeded 1,000 yd, and its Mark 44 director had no lead computation. It was replaced by 20 mm and 40 mm guns during the war. [1.1"/75 Mark 1, NavWeaps, https://www.navweaps.com/Weapons/WNUS_1-1-75_mk1.php, "Reliability problems", "Replacement"]
- The first US Navy shipboard 40 mm quad went on Wyoming on 22 Jun 1942. The first twin went on Coghlan on 1 Jul 1942. Mark 51 director accuracy is credited for half the Japanese planes downed Oct 1944-Feb 1945. [40 mm Bofors, NavWeaps, https://www.navweaps.com/Weapons/WNUS_4cm-56_mk12.php, "First installations"] That page does not name North Carolina or South Dakota. Which ships carried 40 mm or 20 mm guns at Eastern Solomons or Santa Cruz is unverified.
- Enterprise at Eastern Solomons: 5in, 1.1in and 20 mm fire was heavy and effective. Fire-control radar could not track attackers, so fire opened only after dives began. A hit killed power to the after 5in guns, forcing hand operation at half rate. [ONI Eastern Solomons, https://ibiblio.org/hyperwar/USN/USN-CN-ESols/USN-CN-ESolomons-4.html, "Air Attack on the Enterprise Force"]
- South Dakota at Santa Cruz: (WP lead, unverified) AA credited with 26 of 99 claimed kills. [Santa Cruz, Wikipedia, https://en.wikipedia.org/wiki/Battle_of_the_Santa_Cruz_Islands]
- South Dakota's damage analysis for Nov 1942 lists 20 mm, 40 mm and 1.1in mounts with a Mk. 45 1.1in director, but gives no counts. [South Dakota Damage Analysis, Lundgren, NavWeaps, https://www.navweaps.com/index_lundgren/South_Dakota_Damage_Analysis.php]

## Claims versus reality

- At Eastern Solomons, Enterprise claimed 15 planes and North Carolina 7. Officers estimated about 70 enemy destroyed, with fighters credited for 43 of the attack group. [ONI Eastern Solomons, ESolomons-4.html]
- (WP lead) The US claimed 70 Japanese aircraft, though only 37 were present. Actual Japanese loss from all causes was 25. Seven D3As attacking North Carolina were all shot down. [Eastern Solomons, Wikipedia, https://en.wikipedia.org/wiki/Battle_of_the_Eastern_Solomons, "Aircraft losses"]
- The two sources give inconsistent figures, so any claimed-versus-real gap should be treated as probabilistic.

## VT fuze

- First combat kill with proximity-fuzed AA ammunition: USS Helena, 5 Jan 1943, a D3A south of Guadalcanal. [Proximity fuze history, NavWeaps, https://navweaps.com/index_tech/tech-129.php] A search summary cited a different date, 6 Jan. not fetched: the USNI page, so this is unverified.
- Claimed effect: three to four times better than time fuzes. In 1943 about 25 percent of AA rounds used VT but gave 51 percent of kills. Shells reached the Pacific in late fall 1942. [same, "Effectiveness", "Pacific use"]

## Share of kills by gun

- The COMINCH summary for Oct 1944-Jan 1945 gives: against suicide attacks, 40 mm about 50 percent, 20 mm 27, 5in 19, and 1.1in about 0. Against non-suicide attacks, 5in 34 percent, 1.1in 32, 40 mm 29, 20 mm 2. These are post-period figures. [Antiaircraft Action Summary ch. II, COMINCH, 1945, https://www.ibiblio.org/hyperwar/USN/rep/Kamikaze/AAA-Summary/AAA-Summary-2.html]
- The same series says Jan 1943-Jan 1944 attacks were direct and often night, and AA was largely successful. [COMINCH, ch. I, https://www.ibiblio.org/hyperwar/USN/rep/Kamikaze/AAA-Summary/AAA-Summary-1.html, "Changes in Tactics"]
- Not found: any official AA-versus-fighter share for 1942-43.

## Land-based AA

- 3rd Defense Battalion AA gunners were Henderson Field's only air defense while rain grounded planes. On 25 Oct 1942 they accounted for 5 of 27 enemy planes downed, and fighters took the rest. [The Guadalcanal Campaign, USMC monograph, https://www.ibiblio.org/hyperwar/USMC/USMC-M-Guadalcanal/USMC-M-Guadalcanal-7.html] This page does not mention 90 mm guns.

## Japanese AA

- Type 96 25 mm: 15-round magazines, cartridge ejection problems at high elevation, effective ceiling about 3,000 m. The US Naval Technical Mission ranked as worst: inability to train or elevate fast enough, poor sights, then vibration. Most mounts were manual and few had blast shields. [25 mm Type 96, NavWeaps, https://www.navweaps.com/Weapons/WNJAP_25mm-60_mg.php, "Mount/Turret Data"]
- Type 89 12.7 cm: Japan's first dedicated AA gun, with a fast rate of fire, but low muzzle velocity limited range and ceiling. Sustained rate was about 8 rounds a minute. [12.7 cm Type 89, NavWeaps, https://www.navweaps.com/Weapons/WNJAP_5-40_t89.php]
- 10 cm Type 98, Akizuki class: called the best Japanese AA gun, 15-19 rounds a minute, time-fuzed HE only. Early ships had two Type 94 directors. [10 cm Type 98, NavWeaps, https://navweaps.com/Weapons/WNJAP_39-65_t98.php] Akizuki herself was completed on 11 June 1942 and is in the Eastern Solomons order of battle (`data/oob/ship_facts.csv`, from her combinedfleet.com record), so the class is just inside the period.
- No source read covers Japanese radar fire control, though a USSBS interviewee says his destroyers had no radar and used visual fire control at Tassafaronga. [Interrogation Nav No. 60, USSBS, 1945, https://www.ibiblio.org/hyperwar/AAF/USSBS/IJO/IJO-60.html]

## What a rule would need to model

- Gun type: range, rate, shell and fuze effect (time, proximity, impact).
- Director and fire control: tracking quality, radar ability (low at this date), loss of power or director.
- Mount training and elevation speed, manual versus powered.
- Crew reliability: stoppages, jams, hand-working after damage.
- Warning time: radar plot warning, and fire opening only once a dive begins.
- Formation and screen geometry: how many guns bear, friendly-fire risk to low-flying returning planes.
- Fighter CAP interaction: who gets credit, and fire held back when fighters are on the attackers.
- Claim inflation (claimed kills exceed real losses).
- Shell supply: VT availability in late 1942 and early 1943, ammunition expended.
- Land AA versus ship AA, and weather grounding aircraft.

## Sources (accessed 2026-11-10)

- NavWeaps pages: 5"/38 Mark 12; 1.1"/75 Mark 1; 40 mm Bofors; 25 mm Type 96; 12.7 cm Type 89; 10 cm Type 98; Proximity fuze (tech-129); South Dakota Damage Analysis (URLs inline).
- ONI Combat Narrative, Eastern Solomons, 1943: https://ibiblio.org/hyperwar/USN/USN-CN-ESols/USN-CN-ESolomons-4.html
- COMINCH, Antiaircraft Action Summary, ch. I and II, 1945 (URLs inline).
- USMC monograph, The Guadalcanal Campaign: https://www.ibiblio.org/hyperwar/USMC/USMC-M-Guadalcanal/USMC-M-Guadalcanal-7.html
- USSBS Interrogation Nav No. 60, 1945: https://www.ibiblio.org/hyperwar/AAF/USSBS/IJO/IJO-60.html
- Wikipedia (leads only): Eastern Solomons and Santa Cruz articles.

Not loaded: history.navy.mil (errors), usni.org (403), navweaps 20 mm and 5in/25 pages (404).

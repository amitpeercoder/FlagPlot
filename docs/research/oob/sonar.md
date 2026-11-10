# Sonar and anti-submarine detection, 1942-43

Status: research note. Design input for scenarios 3 to 5; it proposes nothing binding.

How to read it: drafted by a research subagent from pages read on 2026-11-10, then reviewed. Each statement carries its source. The Navy's own history site would not load that day, so some points rest on Wikipedia or a single website; those are marked "unverified", "W" or "WP lead" and should be checked against a primary source before a rule relies on them.

Source note: the best sources reached were the HyperWar copy of War Damage Report 58 (Japanese ASW, from the Naval Technical Mission to Japan work) and navweaps. US QC-specific figures were not found. Wikipedia entries are flagged "W".

## US sonar and its limits

No source read gave a page giving QC-series range or speed figures; the claim that QC sets equipped all US destroyers in 1941 came only from a search snippet of a secondary history, so it is unverified. A postwar Navy manual on a later set (QHBa, not QC) gives the physics: the echo cannot be separated from the outgoing ping at about 150 yards or less, so contact is lost in the last part of an attack; contact on a deep submarine can fade near 700 yards; high speed raises the noise at the transducer; the first thermocline usually sits at 50-100 ft and bends the beam downward, shortening range. Throwing weapons avoid the blind approach. [Navy sonar manual chapter, reproduced at eugeneleeslover.com, n.d., https://eugeneleeslover.com/USNAVY/CHAPTER-28-A.html, Sec. 28A4-28A5] Wartime sets also needed the operator to turn the beam by hand in small steps. [same, Sec. 28A3] Treat these as later-period principles applied to 1942 by inference.

## US depth charges and ahead-thrown weapons

Mark 6 (300 lb TNT, sink rate 8 fps, 50-300 ft) and Mark 7 (600 lb, 9 fps) entered service in 1938; a Mark 7 modification in August 1942 allowed 600 ft; the faster-sinking Mark 9 came in 1943. The K-gun projector appeared in 1941 (range 60-150 yards); Hedgehog production began late 1942. [navweaps, US ASW weapons, https://navweaps.com/Weapons/WAMUS_ASW.php] Hedgehog fired ahead and kept sonar contact, cutting blind time by about two thirds, and failed attacks did not disturb the water. [same] More generally, forward-throwing weapons let escorts hold contact, and hunter-killer groups hunted rather than waiting at convoys, later built around escort carriers. [Anti-submarine warfare, W, https://en.wikipedia.org/wiki/Anti-submarine_warfare, "Battle of the Atlantic"] The Navy manual also says wartime experience showed air-surface hunter-killer teams to be very effective. [eugeneleeslover.com, Sec. 28A1]

## Japanese detection and ASW practice

Japanese listening gear was fairly good and could often hear US submarines at 2,000 m or more when the listening ship was stopped or slow; echo-ranging gear was mediocre, about like US QC of about 1937; small escorts often had only a hydrophone; no bathythermographs, so no use of temperature layers. [War Damage Report 58, Section III, HyperWar, https://ibiblio.org/hyperwar/USN/rep/WDR/WDR58/WDR58-3.html]

Weapons: depth charges Type 95 (standard from 1940, sink rate about 6 fps, settings 30 m and 60 m, 90 and 120 added on later mods) and Type 2 (1942-43, faster). Only broadside throwers and Y-guns existed, with no ahead thrower. Early-war destroyers carried almost no charges; escorts (kaibokan) 12-18. [navweaps, Japanese ASW, https://navweaps.com/Weapons/WAMJAP_ASW.php] Fuze error: detonation could differ from setting by up to 60 ft; attacks were broken off too soon; shallow settings. [WDR 58, same]

The May Incident: a congressman's June 1943 remark that Japanese charges ran shallow is credited by Lockwood with up to ten boats lost, but a Navy report on enemy ASW is said not to mention any change, so effect is disputed. [Search summary of Mental Floss and others; not fetched: the pages, unverified.] Also navweaps notes serious Japanese ASW investment began only late 1943-early 1944. [navweaps, same]

## Japanese submarine successes, Aug-Nov 1942

- **Saratoga, 31 Aug:** I-26 fired six, one hit (one broached after a steering fault). North Carolina's radar had picked up I-26 at 03:30 but the destroyer sent lost contact. MacDonough and Phelps gained sound contact afterward and wrongly claimed a kill. [Japanese submarine I-26, W, https://en.wikipedia.org/wiki/Japanese_submarine_I-26, "Third war patrol"]
- **Wasp, North Carolina, O'Brien, 15 Sep:** I-19 fired six from 985 yards at 11:45 (combinedfleet; Wikipedia gives 14:44, probably a different time zone, unverified). Two or three hit Wasp; one hit North Carolina (about 1,000 tons flooded, still 25 knots) and one O'Brien. Before this, I-19's operator heard heavy screws at 09:50. The first charge fell six minutes after the last hit, about 30 charges in all (Wikipedia says 80, a conflict). [I-19, combinedfleet, https://www.combinedfleet.com/I-19.htm; USS Wasp (CV-7), W, "Loss"]
- **Juneau, 13 Nov:** I-26 fired three; the crippled Juneau, torpedoed earlier, took one and blew up. No US ASW response is recorded, and no source read says whether the screen had sonar contact. [I-26, W, "Fourth war patrol"; Naval Battle of Guadalcanal, W, https://en.wikipedia.org/wiki/Naval_Battle_of_Guadalcanal, "Aftermath"]

Common features: submarines fired from inside or just ahead of the screen, after the screen lost contact or was distracted by rescue work (Wasp's destroyers kept clear while picking up survivors). [USS Wasp, W]

## What a rule would need to model

- Sonar detection depending on own-ship speed, with a noise penalty at high speed.
- Water conditions (thermal layers) and the escort's ability to know them.
- Blind approach: contact lost at close range during an attack.
- Depth knowledge: whether the escort can estimate target depth, and setting accuracy.
- Weapon pattern: stern rolled, side-thrown, ahead-thrown; sink rate versus submarine evasion time.
- Screen geometry and its being disrupted by rescue, damage or maneuver.
- Persistence: how long escorts keep hunting before accepting a kill.
- Own-force training and equipment differences between sides, changing over the war.
- Passive listening on the surface force as a detection source.

## Sources (accessed 2026-11-10)

- War Damage Report 58, Submarine Report, Section III, HyperWar, https://ibiblio.org/hyperwar/USN/rep/WDR/WDR58/WDR58-3.html
- navweaps, US and Japanese ASW pages, https://navweaps.com/Weapons/WAMUS_ASW.php and https://navweaps.com/Weapons/WAMJAP_ASW.php
- combinedfleet.com, I-19, https://www.combinedfleet.com/I-19.htm
- Navy sonar manual chapter 28 (mirror), https://eugeneleeslover.com/USNAVY/CHAPTER-28-A.html
- Wikipedia (leads): Anti-submarine warfare; I-26; USS Wasp (CV-7); Naval Battle of Guadalcanal.

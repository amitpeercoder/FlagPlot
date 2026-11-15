# Rearming in 1942

Status: a research report for the architect, read on 2026-11-15. Nothing here is settled and nothing here proposes a game rule or figure. The place rows are `place_facts.csv` ids 896 to 925, which `oob-check` does not read (it names no place file; `fuelling-in-port.md` says why). Their attributes (`rearming_event`, `rearming_note`, `ammunition_depot_note`, `ammunition_stock_note`, `bomb_handling_note`) are not in `attributes.csv`. Two class rows are in `ship_class_facts.csv` (2044, 2045). New sources are 1336 to 1338; nine gaps are the last lines of `gaps.csv`.

## What was asked

How, where and how fast ships took on main-battery and anti-aircraft ammunition and torpedoes after a fight in 1942; how air units were given bombs and torpedoes; what a ship carried; and forces that fought again without rearming. The pass looked for dated events (ship, place, supplier, quantity, hours), where each kind of ammunition could be had by date, forces that went short, what each ship type carried, and bombs on hand at Henderson Field.

## How it was read

Every page was read through the web fetch tool, which returns an extract answered from the page, not the page itself. It limits quotation to 125 characters, so some passages are given as short quotations with the rest summarised, and the quotations in the rows are as the extract printed them. They were not checked by program against saved page text, as the fuelling pass did. A reader who needs an exact span should open the page. Pages that would not open are listed at the end. An outside list of leads was the starting point; each of its statements was looked for on its page and none is cited.

## The leads, and what the pages say

| Lead | Page | Result |
| --- | --- | --- |
| Salt Lake City took 141 rounds 5-inch/25 common from Rainier, 18 Sep 1942 | ussslcca25.com deckl15a.htm | Verified word for word. The watch is 1800-2000, Berth A-2, Great Roads, Noumea. Row 896 |
| Salt Lake City also took 1,500 rounds .30 cal tracer | same | Verified. The entry also has 1,500 rounds .30 cal armor-piercing service. Row 896 |
| Fletcher rearmed from Hinton Helper at Espiritu Santo, 31 Oct | ussfletcher.org/history/1942.html | Verified word for word. The same entry has 847 rounds expended on 30 Oct. Row 904 |
| Aaron Ward finished obtaining ammunition replacements, 0522, 1 Nov | scottdavis61.com/dd483.htm | Verified word for word; also cleared Hinton Helper's port side, anchored 0536, sailed 1500. The page begins on 1 Nov. Row 905 |
| Fletcher received ammunition at Segond Channel, 16 Nov | ussfletcher.org/history/guadalcanal.html | Verified word for word. Row 907 |
| Fletcher received 10 torpedoes from Rigel, 24 Nov | same, and 1942.html | Verified word for word on both pages. Row 908 |
| San Francisco replenished from Salt Lake City | BBBO-5, printed p. 41 | Verified word for word at Espiritu Santo. Row 898 |
| Helena from Boise | same sentence | Verified word for word; the place is carried over from the first clause. Row 899 |
| Depots at Noumea, Espiritu Santo, Efate handled aviation ammunition | BBBO-6, p. 49 | Verified word for word. Rows 910 and 912 (and 911 for Espiritu Santo) |
| Rigel sent to Espiritu Santo | BBBO-5, pp. 39-40 | Verified with different wording: the extract reads that Halsey sent the repair ship Rigel, which could give limited fuel and provisions. It does not say she gave ammunition. No row |
| Transport doctrine: 21.5 tons a hatch an hour for small arms ammunition, 13.0 for 90-mm | transport-20.html | Verified word for word. The table is introduced by "Hourly rates for some type of cargo were". The lines before it say 15 tons a hatch an hour is a reasonable overall average "with sufficient boats". September 1944, a transport loading rate. No row: not a combatant's rearming and not 1942 |
| Fletcher fired ten torpedoes at 0222-0223, 13 Nov | ussfletcher.org/history/guadalcanal.html | Verified word for word: a half salvo of five at 0222 and another five at 0223, in the action report. Not a row (an expenditure, not a receipt); see section 3 |
| Ghormley ordered destroyer-transports to carry bombs and other aviation supplies | Miller ch. 4 | Verified word for word on printed p. 83, not 83-84. Row 921 |
| Henderson Field lacked bomb trucks, carts and hoists | Miller ch. 4 | Verified word for word on p. 85, not 84. Row 922 |
| 1,200 rounds a gun for merchant-ship 20-mm west of Hawaii | Armed Guard history ch. III | Verified: "only 1,200 rounds per gun" on p. 195, with area 1-A from 7 Dec 1942. It is also given earlier (p. 190, 15 May 1942) for area 1 ships. No row: merchant ships |
| 3,000 rounds for .50-cal on those routes | same | Verified: "was set at 3,000 rounds", pp. 195-196, and the 19 Aug 1942 directive for three other areas raised it from 1,000 to 3,000 (p. 194). Only the first 100,000 of 200,000 characters of the chapter were read. No row |

Not opened: the NHHC pages (Guadalcanal Campaign, H-Gram 010, Fletcher DANFS, Landing Operations Doctrine; the connection failed or the certificate could not be verified), the USNI article (HTTP 403 in the lead's account; not tried again). Also not opened by this pass: history.navy.mil Rainier DANFS (connection refused), H-Gram 013-1 (certificate could not be verified) and a navylive.dodlive.mil article (certificate did not match the host).

## 1. Dated rearming events

| Row | Date | Place | Statement | Source |
| --- | --- | --- | --- | --- |
| (SLC deck log, no place) | 4 Jul 1942 | Brisbane, Vacuum Oil Dock | Salt Lake City "Received from H.M.A.S. AUSTRALIA, twenty (20) high explosive 8" fused projectiles" in the 1200-1600 watch | slc-decklogs-1942, decklg13.htm (entry 4 July, 12-16 hours); no row (Brisbane is not in `places.csv`) |
| 896 | 18 Sep 1942 | Noumea, Great Roads | Salt Lake City from the ammunition ship Rainier: 141 5-inch/25 common, 1,500 .30 AP tracer, 1,500 .30 AP service; 1800-2000 watch | slc-decklogs-1942 |
| 903 | Sep 1942 | Tongatabu | South Dakota "cannibalized of 1,000 rounds of 5-inch" before sailing for Pearl Harbor | carter-bbbo |
| 898, 899 | Oct 1942, after the 11-12 Oct action | Espiritu Santo | San Francisco replenished from Salt Lake City; Helena from Boise. No quantity, caliber or day | carter-bbbo |
| 904 | 31 Oct 1942, arrived 1400 | Espiritu Santo | Fletcher "rearmed from SS HINTON HELPER" after expending 847 rounds 5-inch on 30 Oct | fletcher-assoc-history-1942 |
| 905 | 1 Nov 1942, ended 0522 | Segond Channel | Aaron Ward "finished obtaining ammunition replacements" alongside Hinton Helper | aaron-ward-diary-nov-1942 |
| 906 | 7 Nov 1942 | Espiritu Santo | Fletcher "loaded ammunition and supplies" | fletcher-assoc-history-1942 |
| 907 | 16 Nov 1942 | Segond Channel | Fletcher "Received ammunition" | fletcher-assoc-guadalcanal |
| 908 | 24 Nov 1942, anchored 0918 | Segond Channel, east entrance | Fletcher received 10 torpedoes from Rigel | fletcher-assoc-guadalcanal |
| 909 | 5 Dec 1942 | Segond Channel | Fletcher received 5 torpedoes from Drayton | fletcher-assoc-history-1942 |
| 925 | 1 Oct 1942 | Shortland | Chokai "Embarks Type 0 Common shells instead of Type 91 APs": a change of shell type, no quantity | combinedfleet |
| 897 | 5 Nov 1942 | Pearl Harbor | Salt Lake City "Transferring of service ammunition" in the 1200-1600 watch; direction and quantity not stated | slc-decklogs-1942 |

What these give and do not give. Five of the events have a place, a ship and a named supplier (a merchant ship twice, a repair ship, an ammunition ship, a destroyer). Only two give a quantity (141 rounds and 3,000 small-arms rounds; 10 and 5 torpedoes). The hour of ending is given for Aaron Ward (0522); no entry gives a start and an end together. The Salt Lake City entry is a two-hour watch for a small issue. Hinton Helper is a merchant ship ("SS"), which both Fletcher's chronology and Aaron Ward's diary name as the supplier within a day of each other; no source read says what she carried.

Fletcher's torpedoes in the extracts: ten fired on 13 Nov (two half salvos, 0222 and 0223); ten received from Rigel 24 Nov; ten expended and 60 rounds of 5-inch on 30 Nov; five received from Drayton on 5 Dec. No torpedo receipt is entered between 13 and 24 Nov in the pages read.

## 2. Where ammunition could be had

| Row | Date | Place | Statement |
| --- | --- | --- | --- |
| 901 | Oct 1942 | Espiritu Santo | Cub 13 had brought ammunition for light guns, which sufficed; turret-gun ammunition was being shipped |
| 902 | Oct 1942 | Efate, Espiritu Santo, Noumea | Storage areas allocated |
| 910 | late fall 1942 | Noumea | Ammunition depot; about 100 small magazines, storage for ordnance material, mines, torpedoes |
| 911 | Dec 1942 to Sep 1944 | Espiritu Santo | First munitions landed in December 1942; 38,000 tons in 175 magazines at the peak, September 1944; issues largely direct to ships until mid-1944 |
| 912 | late fall 1942 | Efate | Smaller depot |
| 913 | 1943 | Espiritu Santo | 75 steel magazines and 16 open bomb-storage areas; torpedo servicing facilities begun 17 Mar 1943 |
| 914-917 | May to Sep 1942 | Tongatabu, Fiji, Noumea | The ammunition ship Rainier at Tongatabu in late July and ordered with the tanker Kanawha to leave on 28 Jul; "to the Fijis" at the end of July; at Noumea 5 Aug to 24 Sep (DANFS as copied on historycentral); 100 1,000-lb, 400 500-lb and 100 100-lb bombs ordered loaded from shore stocks at Tongatabu on 29 Jul |
| 918 | 11 and 12 Aug 1942 | Noumea (inferred) | No 4-inch ammunition and 1,000 rounds of 3-inch for the destroyer-transports; Nimitz sends 6,000 3-inch and 4,000 4-inch in Vestal and Kitty Hawk, and 200,000 rounds 20-mm in Cabrillo |
| 919-924 | Aug to Sep 1942 | Henderson Field, Espiritu Santo | See section 4 |

Rainier's two movements read together: Carter has her at Tongatabu in late July and the DANFS mirror has her shifting "to the Fijis" then; the sources are consistent on Noumea from 5 Aug and that is where Salt Lake City's receipt of 18 Sep took place. The Building the Navy's Bases chapter on the South Pacific gives no 1942 magazine at Noumea, Efate or Espiritu Santo (its first is the 1943 depot, row 913); its 1942 item is eight magazines at Bora Bora, built by mid-March, which is outside this pass.

## 3. Forces that fought again, and shortages

- After the 11-12 Oct action Carter has the still unmet need as about 2,000 rounds "4-inch .38-caliber" and 5 torpedoes for destroyers, and says these needs had been foreseen and shipment made (row 900). He had written earlier on the same page that the vessels had so far needed little replacement because there had been no heavy-gun engagements. The caliber is printed as 4-inch; the 38-caliber gun in these ships was the 5-inch, and the printed caliber was not corrected.
- Aaron Ward's action report of 12-13 Nov (scottdavis61.com/dd483.htm) says "No torpedoes were fired" and gives "Total rounds fired from 5" guns was 215." It also gives AA ammunition expended on 11 and 12 Nov (20 rounds, 1 per cent of service allowance; 80 rounds, 5 per cent; 80 rounds, 5 per cent). The service allowance itself is not printed on the page.
- After Tassafaronga: Office of Naval Intelligence's narrative (HyperWar) gives Fletcher 10 torpedoes fired in two half salvos, Perkins 8, Drayton 2; and Pensacola's starshell "partly because of insufficient ready ammunition". NHHC's H-Gram 013-1 is said by a search excerpt to say six of Tanaka's eight destroyers carried no torpedo reloads for their supply drums; the page was not opened and no row was made (gap).
- Marines on Guadalcanal and Tulagi had "only 5 units of fire" on 16 Aug (row 919); a convoy on 18 Sep brought "the first resupply of ammunition since D-Day" (row 920).
- A shortage of ammunition that stopped a force fighting was not found for any of the night actions in the pages read.

## 4. Air units and bombs

| Row | Statement | Source |
| --- | --- | --- |
| 921 | 12 Aug 1942: Ghormley ordered destroyer-transports loaded with gasoline, lubricants, bombs, ammunition and ground crews; no number | Miller p. 83 |
| 922 | No bomb trucks, carts, hoists or gas trucks (p. 85); fuelling by drum and hand pump; handling-equipment shortage, not a bomb shortage | Miller |
| 919 | Marines' 5 units of fire, 16 Aug | Carter |
| 920 | First ammunition resupply 18 Sep (7th Marines convoy) | Shaw |
| 923 | Bombing up "without bomb carts"; ten men hoisted a 500-lb bomb on a truck bed; two aircraft with guns but "no bombs" | Wings at War no. 3 ch. 4 |
| 924 | B-17s from Espiritu Santo could take full bomb loads once the field worked by 1 Aug; Navy moved .50-cal ammunition and 300,000 gal gasoline to Espiritu for the 11th Group | Craven and Cate vol. IV ch. 3 |

No passage read gives a count of bombs or aerial torpedoes on hand at Henderson Field, or a day on which they ran short. The chapters give shortages of fuel, bomb-handling equipment and trucks instead.

## 5. What a ship carried

Rows 2044 and 2045 are new; the table adds what the NavWeaps pages print for classes already held.

| Item | Statement | Row or note |
| --- | --- | --- |
| North Carolina class 16-inch | "about 148 rounds" a gun; OP 755 stowage 404 + 525 + 404 = 1,333 projectiles over three turrets plus 9 drill projectiles a turret; OP 755 first issued 1941, updated October 1944 | 2044 |
| North Carolina class 5-inch/38 | 450 rounds a gun, from the 5-inch/38 page's list | 2045 |
| Fletcher class 5-inch/38 | 425 rounds a gun (575 in later ships) | the class is not in `ship_classes.csv`; in this note only |
| Saint Louis, Cleveland, Fargo 6-inch/47 and 5-inch/38 | the 5-inch/38 page lists 500 rounds | row 1092 holds Saint Louis's 500 |
| Yorktown and Essex 5-inch/38 | 450 rounds | not entered (class not held) |
| Pre-war destroyers, Farragut to Sims | 300 rounds | rows 665, 667 |
| Cruisers of 9-gun 8-inch type | "peace-time outfit of 100 - 125 rounds per gun with a wartime allowance of 1,400 rounds per ship"; Northampton and Portland 1,500 a ship reduced after July 1942 to just over 1,300; New Orleans reduced after August 1942 to 125 a gun | rows 656-663 |
| Merchant ships, area 1-A west of Hawaii, from 7 Dec 1942 | 20-mm 1,200 rounds a gun; .50-cal 3,000 rounds | Armed Guard history chapter III, pp. 194-196; not in the data |

South Dakota's issue of 1,000 rounds of 5-inch (row 903) is the only count of a stowage of one ship in these pages.

## Where sources disagree

- Rainier at the end of July: Carter's order of 28 Jul has her leaving Tongatabu (destination cut off in the extract); DANFS as copied on historycentral has her shifting to the Fijis then. Both are kept (rows 915 and 916); they are not in conflict.
- Salt Lake City's own deck log records no transfer of ammunition to San Francisco after the action; Carter says San Francisco replenished from her. The log is a selection by watch, so it neither confirms nor denies (row 898).
- Carter prints "4-inch .38-caliber" for a need after Cape Esperance, as above.

## What is still a gap

All in `gaps.csv`, last nine lines. In short: a rearming with a start, an end and a quantity; the dates and holdings of the depots and ammunition ships (Lassen, Nitro and Shasta were not read); Japanese base stocks of shells, torpedoes and ordnance; the Tassafaronga reload statements (not opened); Fletcher's supplier and quantities; bombs and torpedoes on hand at Henderson Field; the Fletcher class's stowage as an entity; a dated allowance for each battery by class.

## Pages that did not open

history.navy.mil Rainier DANFS (connection refused); H-Gram 010-1 and 013-1 (certificate could not be verified); usnhistory.navylive.dodlive.mil (certificate did not match the host); combinedfleet.com tabular pages for Kongo, Haruna, Hiei, Kinugasa and Naganami under the addresses tried (HTTP 404). The history.navy.mil Fletcher, Landing Operations Doctrine and Guadalcanal Campaign pages named in the leads were not tried again.

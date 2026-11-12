# Airmen lost and recovered over Guadalcanal, by name, 21 August to 17 September 1942

Status: research report for the architect, read on 2026-11-12. Nothing here is settled. The rows are in a new file, `data/oob/aircrew_fates.csv` (118 rows, one per man per source), with 11 period totals in `engagement_outcomes.csv` and 32 new gaps. One passage was checked a second time by me (the Army squadron on 30 August); the rest is as the researchers recorded it, including their readings of the Japanese forms.

## Disputes and gaps first

**Disputes**
- **Dates.** One Marine pilot's loss is 24 August in his squadron's history and 26 August in two lists. A squadron commander's ditching is about 7 September by one history and 11 September by two lists. The Navy's loss list dates the losses of 31 August to the 30th.
- **Causes.** Two Marine pilots lost on 31 August: oxygen failure by the air group's diary as quoted, shot down by fighters by a Marine history.
- **How one pilot got back** from 31 August: through the jungle or from the sea, after six days or nine.
- **The Army squadron on 30 August.** The Army's wartime account: two pilots "did not return" and two others walked in after bailing out (checked). The dispatch of the day: four aircraft lost, three pilots missing. The two who walked in are not named anywhere read.
- **When the Navy fighters came ashore.** 4 September (Wikipedia, citing a book not read) or 11 September (two websites, and the dispatches).
- **Japanese forms against Japanese Wikipedia.** Two Tainan pilots are "not returned" or "missing" on the forms and "killed" on Wikipedia. The forms do not say more than that a man did not come back.

**Gaps**
- **The US lists are not complete for survivors.** The list of the missing covers only men never recovered. Marine pilots who were wounded and returned are mostly unnamed.
- The Navy fighters ashore: four men found, none named. No loss list for them.
- The Navy dive bomber crews ashore: one name. The Army squadron: its own history was not found.
- The Marine squadrons' and the air group's war diaries are behind a paywall; no free catalog record was found. The second fighter squadron's history and the second dive bomber squadron's history were not found.
- Two databases of the missing (the battle monuments commission and the defence accounting agency) were not searched.

## The count that answers the question

The best single source is a transcription of the **Navy's overseas aircraft loss list** for August and September 1942, which lists each aircraft lost with its pilot and a one-letter fate. For the two Marine fighter squadrons in the period:

| | Aircraft lost with a named pilot | Pilot survived | Missing | Died |
| --- | --- | --- | --- | --- |
| VMF-223 (with attached pilots) | 18 | 12 | 5 | 1 |
| VMF-224 | 9 | 4 | 4 | 1 |
| **Both** | **27** | **16** | **9** | **2** |

**Sixteen of 27, or 59 per cent,** of Marine fighter pilots whose aircraft were lost in the period survived. Three cautions go with it:
- The list's fate letters are not explained on the page; reading them as survived, died and missing is an inference, though it fits every man whose fate another source gives.
- The list does not state the cause. It includes aircraft wrecked in accidents as well as those shot down, so it is the chance of surviving the loss of an aircraft from this field, not of surviving being shot down.
- It is one website's transcription of an official list.

It agrees with the figure the data already held from Wikipedia: 27 Marine fighters lost and 9 pilots killed to 11 September.

**The narrower question, shot down in air combat,** can only be answered from the men other sources name (the squadron history, the Marine aviation history, the list of the missing, a website that tracks wrecks). My count from those rows:

| | Lost | Recovered |
| --- | --- | --- |
| VMF-223 | 4 in combat (24 or 26 August, three men; 10 September) and 2 on 13 September whose cause is given as operational by one source | 4 who brought a damaged aircraft back and crash-landed at the field (two of them hurt); 3 who came down away from it: one who got ashore at Tulagi, one picked up by a destroyer, one returned by islanders after five days |
| VMF-224 | 5 (31 August, two men; 5 September, two men; 9 September) | 1 who walked or swam back after six to nine days; the squadron commander twice, once crash-landing at the field and once swimming ashore |

So among pilots whose aircraft came down **away from the field**, about 5 or 6 were recovered and 9 to 11 were not: roughly one in three. Counting those who nursed a damaged aircraft home and wrecked it on landing, it is about 10 of 20, one in two. These counts are small, they favour men whose stories were told, and they are mine.

**For the placeholder of 0.6:** the loss list's 59 per cent supports it if the rule's "shot down over its own base" includes aircraft lost to any cause in the fight over the field. For a fighter that comes down away from the field the named cases point lower, near one in three.

## The other units

| Unit | Found | Complete? |
| --- | --- | --- |
| Marine dive bombers (VMSB-232) | 8 men lost, none recovered: four pilots and four rear-seat men (anti-aircraft fire on 28 August, a storm on 6 September, a crash on 14 September). Her history's total of 7 pilots killed by 13 October fits | Pilots believed complete; rear-seat men only where missing |
| Marine dive bombers (VMSB-231) | One man, fate unresolved | No |
| Army 67th Fighter Squadron | 30 August: four shot down, two lost and two walked back. One pilot badly burned in a take-off crash on 8 September | No |
| Navy fighters ashore | Two killed (12 and 13 September), two recovered (11 and 14 September), all unnamed | No |
| Navy dive bombers ashore | One pilot who failed to return on 25 August; two men lost on 13 September | No |
| Japanese fighters, from the unit forms | 16 pilots did not return between 26 August and 14 September (Tainan 9, Shokaku's fighters at Buka 5, the 2nd air group 2). Three more force-landed at Buka or the Shortlands and came back. So 3 of 19, 16 per cent | Probably, for the forms read. No form shows a missing pilot who later returned |

Every Japanese row carries the caution that "missing" or "not returned" on a unit form can include men who lived.

## The file

`data/oob/aircrew_fates.csv`, one row per man per source statement. `oob-check` does not read it yet. Columns: `id`, `engagement`, `date`, `unit`, `name`, `rank`, `role` (pilot, gunner, radioman), `aircraft`, `event`, `fate`, `recovered_how`, `returned_date`, `days_until_return`, `as_stated`, then the usual source columns and `note`.

- `event`: `shot-down-by-fighters`, `shot-down-by-bombers`, `shot-down-by-aa`, `shot-down-cause-not-stated`, `operational-crash`, `missing-on-mission`, `damaged-crash-landed`, `forced-landing`.
- `fate`: `killed`, `missing`, `recovered`, `wounded-returned`, `wounded-evacuated`, `prisoner`, `not-stated`.
- `recovered_how`: blank, or `parachuted-over-field`, `crash-landed-at-base`, `picked-up-from-sea`, `swam-ashore`, `returned-overland`, `returned-by-natives-or-coastwatchers`, `returned-by-flying-boat`, `forced-landing-at-friendly-base`, `not-stated`.
- May be blank: `name` (some men are not named by the source, and the row says so), `rank`, `aircraft`, `recovered_how`, `returned_date`, `days_until_return`, `note`. A `date` may be a month alone where the source gives no day.
- A man appears once per source, so he must not be counted by counting rows. The loss-list rows label an unstated cause as `shot-down-cause-not-stated`; that is the researcher's label, and the list itself gives no cause.

## What would settle it

The air group's and the squadrons' war diaries, which name every man by day, are in the National Archives' war diary collection behind a paywall. The Navy loss list for the Navy squadrons ashore and the Army squadron's own history would complete the other units.

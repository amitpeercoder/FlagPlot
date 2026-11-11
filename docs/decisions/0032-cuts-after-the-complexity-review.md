# 0032: What the complexity review cut and merged

Status: confirmed by the designer, Nov 11, 2026 ("cut both"), with the review in `docs/complexity-review-after-scenario-5.md`.

## Decisions

1. **A sighted task force no longer rolls to notice the aircraft that sighted it.** Decision 0012, 17 and 18, is withdrawn: the roll, its figure, its event and the time a force stored. Nothing read the time. The designer suggested noticing so that a force could react; it comes back with the rule for the reaction, which is the staff's.
2. **A contact no longer has a confidence.** Decision 0012 gave a fresh report 0.8, multiplied it by 0.8 each pulse and dropped the contact below 0.3. That is an age of 5 pulses, and nothing else read the number. A contact is now dropped when it reaches an age of 5 pulses, one figure in place of three, and reports and contacts carry no confidence. The spec says a contact "loses confidence each pulse"; it now simply ages. Confidence comes back when a report can be more or less sure of what it saw.
3. **One figure says how old a contact may be and still count as fresh:** 2 pulses. It serves both tests that used 2: whether a new report refreshes it (decision 0012, 13) and whether it alerts a force (decision 0021). They had a figure each.
4. **Where a gun or bomb hit lands is one figure,** the chance it lands on flotation. The rest land on fire. There were two figures that had to add to 1.
5. **Clock times in the log are rounded to the nearest minute,** not cut. 10:40 no longer prints as 10:39.

## What does not change

No search, sighting, report, fight or strike comes out differently, and no contact is dropped or refreshed at a different time. The state hash changes for every scenario with a contact or a sighted force, because two fields left the state. Event logs lose the noticing events and the confidence on report lines.

## The contact ages that remain, and why there are three

| Test | Figure | Why it is its own |
| --- | --- | --- |
| Fresh: a new report refreshes it; it alerts a force nearby | 2 pulses | Is this still news |
| Can be struck | This pulse or the one before (decision 0026) | Is it worth flying at. Stricter than fresh, because a strike flies to one hex |
| Forgotten | 5 pulses | Is it worth keeping on the plot at all |

## Rule card

| | |
| --- | --- |
| Reads | A contact's age |
| Writes | The side's contacts: aged or dropped at the end of each pulse |
| Events | `ContactAged`, `ContactDropped`, without a confidence |
| Randomness | None. One roll is removed |

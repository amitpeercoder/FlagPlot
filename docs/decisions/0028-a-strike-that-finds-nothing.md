# 0028: A strike that finds nothing drops the contact

Status: confirmed by the designer, Nov 11, 2026.

Summary: A strike that finds nothing drops the contact it was sent at

Standing: Confirmed

## The gap

Decision 0026 deferred "telling the side that a contact was not there". Stage 5d's two-day runs showed what that costs: a strike that found empty sea flew to the same hex again in the next pulse, because finding nothing taught its side nothing. The legibility test's reader called it a gap unprompted.

## Decisions

1. **A strike that finds nothing reports it.** The report says: nothing at this hex.
2. **The side drops the contact the strike was sent at,** if it still holds that contact and the contact still names that hex. An event says so, citing the strike's finding. If the contact has gone, or a later report has moved it, the report changes nothing and records nothing.
3. **It goes through the one way belief changes,** a report applied to the side's belief state, like every sighting.
4. **It uses no random number.**
5. **A strike group with no bomber left finds nothing and reports nothing** (decision 0027, 16), so its contact is kept.
6. **A dropped contact's number is not used again.** If the side sights the force again it is a new contact.

## What this changes in earlier stages

A strike launched in the afternoon at the morning's contact now needs that contact to have survived the morning: either the morning strike attacked, and its own report refreshed the contact, or there was no morning strike. In scenarios 5b, 5c and 5d the chance of an afternoon launch falls from 0.759209 to 0.684234. Their hand calculations are corrected and marked.

## Why

- **Dropping, not doubting.** The strike went to the hex and looked. Lowering the contact's confidence would need a rule for what confidence does to a strike decision, which does not exist.
- **Only that contact, only if it still names that hex.** The side learns that one hex is empty, not that the enemy is nowhere near. The enemy is most often one hex away, and the next search finds it again.

## Deferred

- A strike searching the hexes round the one it was sent to (decision 0026).
- A search that covers a hex and sees nothing dropping a contact there.

## Rule card

| | |
| --- | --- |
| Reads | The contact a strike was launched at; the hex the strike flew to |
| Writes | The launching side's contacts, through a report |
| Events | `ContactNotThere`, caused by `StrikeFoundNothing` |
| Randomness | None |

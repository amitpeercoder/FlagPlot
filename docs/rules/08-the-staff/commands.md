# The staff: Commands

Part of chapter 8, [The staff](README.md). The commands that change command state, who may give each, and every reason one is refused.

## Commands

A command is a change to the side's command state, handed in beside orders. It is checked against the side's own units and its command state, and it is accepted or refused with a reason, as an order is. A refused command changes nothing but the log. [0107 items 1 and 2]

### Who may give each command

| Command | Given by | What it does |
| --- | --- | --- |
| `set_intent` | Commander | Sets an intent: a root, or, with a `parent`, a sub-intent under an intent of his own of a pair open to him (decision 0232); or changes the figures of one in force that the commander made, never its kind or its parent |
| `withdraw_intent` | Commander, or the staff on an intent it made (decision 0151, item 8b) | Withdraws an intent in force that the giver made, everything beneath it, and ends its plan. The commander's on an intent the staff made is refused as "Made by the staff"; the staff's on one the commander made is refused as "Not yours". The staff gives it only for the island's neutralising that has done its work (Controlling an island, The staff withdraws) |
| `set_limit` | Commander, or the staff for a forces limit | Sets one limit, on any intent; the staff sets a forces limit only, and only on an intent it formed (decision 0142, 12) |
| `set_leave` | Commander | Gives or withholds leave to prepare, or leave to launch |
| `set_pursuit` | Commander | Sets the leave to pursue (`ask`, `go` or `stay`) on an intent in force, or clears it so that it is read up the tree again (decision 0214, rule 4) |
| `answer_pursuit` | Commander | Answers, yes or no, a force's asking leave to pursue a contact (decision 0214, rule 8) |
| `approve` | Commander, or the staff with leave | Moves a draft to preparing, or a preparing plan to released |
| `cancel_plan` | Commander | Cancels a plan that has not ended |
| `hold` | Commander | Holds a ship, a task force, a whole air group, one setting of an air group (its search arc), a land unit or a submarine from the staff |
| `release` | Commander | Lifts a hold, whatever made it |
| `set_interrupt` | Commander | Sets a numbered interrupt, or sets it again, or switches a standing kind off or on (Interrupts, below) |
| `acknowledge` | Commander | Marks the last firing of an interrupt seen |
| `commit_ships` | Staff | Commits ships of the intent's pool to an approved plan from one day to another |
| `free_ships` | Staff | Ends the commitments of ships to a plan |
| `add_run` | Staff | Adds a run to an approved plan, with the plan's next run number |
| `time_run` | Staff | Gives a run new days |
| `mark_plan` | Staff | Marks an approved plan complete or failed |
| `end_run` | Staff | Ends a run of an approved plan for good (Holds and the override, below; The doctrine) |
| `form_intent` | Staff | Forms a sub-intent under an intent in force whose plan is released, or, for a "ready a landing" under "take a place", preparing or released (The intent tree, below; decision 0142, 12). Under "control an island" it forms a take, a hold, a denial, a neutralising or, since decision 0231, a supply of a base beside a place (Controlling an island, below) |

A command from the other issuer is refused as not yours. A staff may give `approve` only where the commander has given it the leave for the move: leave to prepare for a draft, leave to launch for a plan that is preparing. The commander's `approve` needs no leave. [0107 items 4 and 21]

The commander's commands are handed in with the orders for a day and taken in at its first step, the Allied side's first, each side's in the order handed in, so a hold or a release given for a day binds from that morning, before the staff is asked. The staff's are taken in after the staff is asked, in the order handed in, before any order. [0107, The order of a day; rulings 17 and 23]

### Every reason a command is refused

The reasons for one command are tried in the order written. The first that applies is the one recorded. A unit or a base that is not the side's, and one that does not exist, are refused alike, so a refusal never shows that an enemy unit exists. A ship is the side's own only if it is not sunk.

| Command | Reason | When |
| --- | --- | --- |
| Any | Not yours | The issuer may not give this kind of command. Tried first |
| `set_intent` | Intent number not usable | The number is 0, or it is the number of an intent that was withdrawn |
| `set_intent` | Made by the staff | The number is that of an intent in force that the staff formed |
| `set_intent` | Kind cannot change | The number is that of an intent in force of another kind; it is withdrawn and another stated (the architect's ruling after the build of 0117) |
| `set_intent` | Parent cannot change (decision 0232) | The number is that of an intent in force and a parent is given that is not the one it has. A parent left out is read as the one it has, and for a root as none. "intent:N stands under intent:M and cannot be moved: withdraw it and state another"; for a root, "intent:N is a root and cannot be moved under another". Tried after "Kind cannot change" |
| `set_intent` | No such intent (decision 0232) | The parent is not in force. A parent never set, one withdrawn and one of the other side's are refused alike, naming the parent |
| `set_intent` | Intent number not usable (decision 0232) | The number is new and is not above the parent's: the tree holds a parent's number below its child's |
| `set_intent` | Cannot stand under (decision 0232) | The parent was formed by the staff, naming the parent. In this build that is every case the pair's own check refuses too, so no reason of its own is added |
| `set_intent` | Wrong plan state (decision 0232) | The plan the parent serves has ended |
| `set_intent` | Cannot stand under (decision 0232) | The pair is not open to the commander, or the place is not the hex of a named place of the parent's island, naming the parent |
| `set_intent` | Already planned (decision 0232) | Another intent than this one is a "take a place" of the same hex, in force on the side, and either its plan has not ended (in any tree, whoever made it) or the commander wrote it under this island (whatever its plan). "intent:K already plans the place at H" |
| `set_intent` | Not your base | The base the kind names as the side's own is not one of the side's. Every kind that names a base of the side's names one |
| `set_intent` | Not an enemy base | The target of a neutralise or a raid intent is not a base on the side's map that is not the side's (decisions 0120, 5 and 9). A base the side may not know of and one that does not exist are refused alike, in the same words. Tried after "Not your base" |
| `set_intent` | Not your force | The task force a stand-at, operate-carriers or ready-a-landing intent names is not one of the side's: "H is not one of the side's forces". A force of the other side and one that does not exist are refused alike (decisions 0124, 7, and 0125, 1). Tried after "Not an enemy base" |
| `set_intent` | Not on the map | The hex of a deny intent, a patrol intent (decision 0218, 11), a stand-at intent or an operate-carriers intent is not on the map (decisions 0123, 1, 0124, 7, and 0125, 1). Tried after "Not your base", "Not an enemy base" and "Not your force" |
| `set_intent` | Not a coast hex | The hex of a ready-a-landing intent is not a coast hex on the map: "H is not a coast hex on the map" (decision 0141, 1). Tried after "Not on the map" |
| `set_intent` | Not land | The hex of a take-a-place intent is not a hex of land on the map, coast or inland: "H is not a hex of land on the map" (decision 0142, 1). A hex off the map is refused so too. Tried after "Not a coast hex" |
| `set_intent` | No such island | The island a control-an-island intent names is not an island of the scenario: "island N is not an island of the scenario" (decision 0151, 3). Tried with the other checks of an intent's kind, after "Not land" |
| `set_intent` | Not a root | The kind is search waters, which a commander cannot state: it stands under a deny intent only and is the staff's (decision 0123, 11). Tried after "Made by the staff" and "Kind cannot change", before the kind's other checks |
| `set_intent` | Contact not held | The contact a destroy-a-force intent names is not one the commander's picture holds: "contact N is not held: the commander's picture holds no contact of that number", or, for the number 0 or none, "no contact is named" (decision 0220, R12). A contact held and not fresh is not refused, and neither is the contact the order already follows. Tried after "Not a root", with the other checks of an intent's kind |
| `set_intent` | No days | The floor is 0, or an interval is given and is 0. A "keep supplied" formed by the staff under a hold or, since decision 0231, under "control an island" may have a floor of 0, which intake allows and no staff now sends (decision 0230: the floor the staff forms is never less than 1 day); an interval of 0 is refused there too |
| `withdraw_intent` | No such intent | No intent of that number is in force |
| `withdraw_intent` | Made by the staff | The commander gave it and the intent was formed by the staff |
| `withdraw_intent` | Not yours | The staff gave it and the intent was made by the commander (decision 0151, 8b) |
| `form_intent` | No such intent | The parent is not in force. Tried after the issuer |
| `form_intent` | Wrong plan state | The parent's plan is not released; a ready-a-landing under a take-a-place is taken in while the plan is preparing too (decision 0142, 12), and so is an operate-a-carrier-force under a take-a-place, a force moved to look (decision 0159, 14) |
| `form_intent` | Cannot stand under | The kind cannot stand under the parent's kind: the pairs built are cover from the air under hold a place for the same base, and a raid under neutralise a base for the same target, strike ships and search waters under deny an area, and strike ships under hold a place for the same base (decision 0123, 5 and 11); and, since decisions 0124 and 0125, stand at a hex under deny an area where the hex is the centre of the waters, stand at a hex under hold a place where the hex is the base's, and operate a carrier force under either (the hex of a stand task under a hold that is not the base's is refused in the same words); and, since decisions 0144 and 0159, keep supplied under hold a place for the held base or for a base of the side in the held base's hex (a base in another hex, or the other side's, is refused in the same words); and, since decision 0142, ready a landing under take a place where the landing's hex is the place; and, since decision 0220, strike ships, search waters and operate a carrier force under destroy a force (no stand at a hex), and, since decision 0151, take a place, hold a place, deny an area and neutralise a base under control an island, and, since decision 0231, keep supplied under control an island for a base of the side (a base of the enemy's or of no one is refused in the same words; whether the base is on the island's land is the staff's reading, not intake's), and operate a carrier force under the deny an area that stands under an island. A patrol the staff cannot form: it is refused so under every parent, and no kind stands under a patrol (decision 0218, 11) |
| `form_intent` | Not your base | The base is not one of the side's |
| `form_intent` | Not your force | The task force of a stand-at or operate-carriers intent is not one of the side's |
| `form_intent` | Not on the map | The hex of a deny, stand-at or operate-carriers intent is not on the map |
| `form_intent` | Not an enemy base | The target is not a base on the side's map that is not the side's |
| `set_limit` | No such intent | The limit is of an intent (all but the reserve) and no intent of that number is in force. Tried first |
| `set_limit` | Not yours | The staff gave it and the intent is not one the staff formed (decision 0142, 12). Tried after "No such intent"; a staff's limit of another kind than forces is refused as not yours first |
| `set_limit` not before, wait for, after | No wait limit on | The intent is not a "take a place" or a "control an island", or the staff formed it: "intent:N is not a take_place or a control_island of the commander's: only those may be told to wait" (decision 0159, 9). Tried after "Not yours", for a limit set and one cleared alike |
| `set_limit` knowledge | No knowledge limit on | The intent is not a "take a place" or a "control an island", or the staff formed it: "intent:N is not a take_place or a control_island of the commander's: only those may be told how much must be known" (decision 0159, 12). Tried after "Not yours", for a limit set and one cleared alike |
| `set_limit` knowledge | Recent count of no days | The limit asks for a recent count of 0 days (decision 0159, 12). Tried after "No knowledge limit on" |
| `set_limit` targets | Bad target kinds | The limit names no kind, or a kind twice (the tool also refuses a `waters_first` of true with no kinds): "the targets limit on intent:N must name one to six kinds of ship, each once" (decision 0190). Tried after "Not yours" |
| `set_limit` targets | No waters first on | `waters_first` is true and the intent is not a "deny an area" and does not stand beneath one: "intent:N is not a deny and does not stand beneath one: only there may the waters come first" (decision 0190, 27). Tried after "Bad target kinds" |
| `set_limit` forces | No source | The intent is a supply intent or a take-a-place intent and the limit names no source (decision 0142, 2) |
| `set_limit` forces | Named and none of | The limit names a unit of a kind and lists that kind in `none_of`: "the forces limit names air_groups and lists them in none_of: a kind is named or listed, not both" (decision 0222, ruling A). Tried before "Not your ship" |
| `set_limit` forces | In place with no task force | `in_place` is true and the limit names no task force: "the forces limit says its task forces work from where they lie and names no task force" (decision 0222, ruling I). Tried after "Named and none of" |
| `set_limit` forces | Not your ship | A listed ship is not the side's, in ship order |
| `set_limit` forces | Not your force | A listed task force is not the side's, in force order |
| `set_limit` forces | Not your group | A listed air group is not the side's, in group order |
| `set_limit` forces | Not your base | The source is not one of the side's |
| `set_limit` forces | Not your land unit | A listed land unit is not the side's, in unit order (decision 0141) |
| `set_limit` forces | Not your submarine | A listed submarine is not the side's, does not exist or is sunk, in boat order (decision 0218, 34). Tried after "Not your land unit" |
| `set_limit` area | Bad area | The area has no hex, or a hex off the map |
| `set_limit` wait for | Not your force, not your land unit | The task force or land unit named is not the side's: the same words as a forces limit naming it, so a force or unit of the other side and one that does not exist are refused alike (decision 0159, 9) |
| `set_limit` after | No such intent | The intent named is not in force: one withdrawn, and one never set, are refused alike |
| `set_limit` after | Not an end to wait for | The intent named is the limit's own intent, or is not a "take a place" or a "control an island", or the staff formed it, or the limit would close a ring of waits: with the limit in place, a walk of the "after" limits read up the tree, from the intent the limit is set on or from any intent beneath it, comes back to the intent it started from, directly or through others (No wait that waits for itself, in The intent tree, below): "intent:N is not another take_place or control_island of the commander's to wait for" (decision 0159, 9; the ring, the architect's ruling on step 3) |
| `set_limit` protected | Not your ship | A listed ship is not the side's |
| `set_limit` reserve | Not your base | The base is not one of the side's |
| `set_limit` reserve | Bad weight | The supply reserve, then the fuel reserve, is below 0 |
| `set_limit` forces, area, risk | Wider than its parent | The intent has a parent and the limit does not narrow (The intent tree, Limits read up the tree). Tried after every other reason of the limit |
| `approve` | No such plan | No plan of that number |
| `approve` | Wrong plan state | The plan is neither a draft nor preparing |
| `approve` | No leave | The staff gave it, and the leave the move needs is withheld |
| `cancel_plan` | No such plan | No plan of that number |
| `cancel_plan` | Wrong plan state | The plan has ended |
| `commit_ships` | No such plan | No plan of that number |
| `commit_ships` | Wrong plan state | The plan is not approved |
| `commit_ships` | No forces limit, no source | The plan's intent has no forces limit of its own; or it has one that names no source |
| `commit_ships` | Bad days | The first day is before the day of the command, or after the last |
| `commit_ships` | No such run | A run is named and the plan has no run of that number |
| `hold` | Not your ship | A ship is named and it is not the side's, or is sunk, or does not exist |
| `hold` | Not your force | A task force is named and it is not the side's |
| `hold` | Not your group | An air group is named, whole or by a setting, and it is not the side's |
| `hold` | Not your land unit | A land unit is named and it is not the side's (decision 0141, 14) |
| `hold` | Not your submarine | A submarine is named and it is not the side's, does not exist or is sunk: one reason for all (decision 0218, 34). Each of these five is tried for the one target the command names |
| `hold` | Already held | The target has a hold already, made by a command or by an order. A whole air group and its search arc are two targets, and a group may have both |
| `release` | Not held | The target has no hold. An enemy unit and one that does not exist are refused so too |
| `set_pursuit` | No such intent | No intent of that number is in force: "intent:N is not in force". An intent the commander made and one the staff formed are alike. Not an amendment: nothing of a plan is looked at (decision 0214, rule 4) |
| `answer_pursuit` | Not your force | The task force is not one of the side's: "F is not one of the side's forces". A force of the other side and one that does not exist are refused alike. Nothing else is checked: the answer need not match an asking that was made, and the contact's number is not looked at (decision 0214, rule 8) |
| `set_interrupt` | Interrupt number not usable | A numbered interrupt is set and its number is 0 |
| `set_interrupt` | Not your base | A numbered interrupt is set and its base is not one of the side's |
| `set_interrupt` | No days | A numbered interrupt is set and its number of days is 0. Setting a standing kind has no other reason: it is refused only as not yours |
| `acknowledge` | No such interrupt | A numbered interrupt is named and none of that number was set |
| `acknowledge` | No such plan | A standing interrupt for a plan is named and the side has no plan of that number |
| `acknowledge` | Not your ship | A standing interrupt for a ship is named and the ship is not the side's, whether it exists or not |
| `acknowledge` | Not your force | A standing interrupt for a task force is named and the force is not the side's, whether it exists or not (decision 0214) |
| `acknowledge` | Not fired | The interrupt has never fired: for a standing one, the side holds no record of it |
| `acknowledge` | Already acknowledged | The last firing is acknowledged already |
| `commit_ships` | Not your ship, not in the pool, held, already committed | For each ship in ship order, in that order: it is not the side's; it is not in the pool of the forces limit; it is held (itself, its force, or any ship of its force); it has a commitment that overlaps the days asked for |
| `free_ships` | No such plan | No plan of that number. A plan of any state is found |
| `free_ships` | Not committed | A ship has no commitment to the plan |
| `add_run` | No such plan, wrong plan state, no forces limit | As for `commit_ships` |
| `add_run` | No ships | The run names no ship |
| `add_run` | Not your ship, not in the pool, held | For each ship in ship order, in that order, as for `commit_ships` (a ship that is held is refused before the load and the days are looked at) |
| `add_run` | Bad weight | The load is not above 0 |
| `add_run` | Bad days | The days are not in order, or the load day is before the day of the command |
| `time_run` | No such plan, wrong plan state | As for `commit_ships` |
| `time_run` | No such run | The plan has no run of that number |
| `time_run` | Bad days | The days are not in order, or the free day is before the day of the command |
| `mark_plan` | No such plan, wrong plan state | As for `commit_ships` |
| `end_run` | No such plan, wrong plan state | As for `commit_ships`: the plan is not approved |
| `end_run` | No such run | The plan has no run of that number |
| `end_run` | Run ended | The run has ended already |

`set_leave` is refused only when the staff gives it, and so are `set_pursuit` and `answer_pursuit`, as not yours: the staff has neither. [0107 items 1, 4, 22 and 27, rulings 17, 20, 31 and 33; 0214]

**The reasons of a sub-intent the commander states** are tried in the order of the table, after "Intent number not usable" (a number of 0 or a withdrawn one), "Made by the staff" and "Kind cannot change", and before the kind's own reasons, which are those of a root ("Not land"). The parent's plan may be a draft, preparing or released: he writes his part before he gives leave, or after. Every check is made when the intent is set again as when it is first set, the intent itself left out of "Already planned": a set-again that keeps its place passes, and one that names another hex passes only where that hex is a named place of the island that nothing else plans. A refused command changes nothing but the log. [0232, rule 3]

The same intent number set twice. Two `set_intent` commands of one day that name the same new number are taken in in the order handed in: the first sets the intent and drafts its plan, and the second changes the intent that now exists. The plan is a draft, so the second is no amendment and drafts nothing. The intent is as the second left it. [0107]

### What a command does and records

A command that passes is applied, then `CommandTaken` is recorded with the whole command and its issuer. It has no cause: it is the root record of the change. A command that fails records `CommandRefused` with the command, its issuer and the reason, and has no cause. What an accepted command sets off is recorded after `CommandTaken` as an event that cites it:

| Event | Recorded when |
| --- | --- |
| `PlanDrafted` | An intent's number is first set, or an intent in force whose plan has ended is set again; and, for a take a place the commander states under an island, when it is stated, after the command's `CommandTaken`, citing it. Names the plan and the intent |
| `PlanStateChanged` | A plan changes state, by `approve`, `cancel_plan`, `mark_plan` or the withdrawal of its intent. Names the plan and both states |
| `PlanAmended` | A change is kept as an amendment. Names the plan and the amendment's number |
| `IntentFormed` | A `form_intent` is taken in. Names the side, the new intent's number and its parent |
| `IntentWithdrawn` | A `withdraw_intent` is taken in: one for each intent beneath the one withdrawn, in the order the tree gives (The intent tree, below). Names the side, the intent and the one it stood under |

A `set_leave`, a `set_pursuit`, a `hold`, a `release`, a `set_interrupt`, an `acknowledge`, a `commit_ships`, a `free_ships`, an `add_run`, a `time_run` and an `end_run` set off no other event: the command's own record, with the whole command, is the record of the change. An `answer_pursuit` sets off none either; a yes begins the pursuit (Operating a carrier force, item 34) and its `CommandTaken` is the event the pursuit and the staff's orders under it cite as their cause. There is no command number. Withdrawing an intent whose plan has already ended, and setting an intent that is a draft again, set off nothing. A sub-intent the commander states records no `IntentFormed`: that event is a `form_intent`'s, and the record of his intent is his command, as for a root; its `CommandTaken` holds the whole command with its parent. Stating it is no amendment of the island's plan, as the staff's forming of one is none. [0107; 0232, rule 4]

One more event is recorded for a hold, and it is not a command's:

| Event | Recorded when |
| --- | --- |
| `HeldByOrder` | The override (below) holds a unit by an order of the commander's. Names the side, what is held and the order. It cites the order and the event that accepted it, and is recorded after them, at the step of the override, with no pulse; for an order given inside a day, at the start of the pulse it is taken in, with that pulse (decision 0156) |

A side sees: its own `CommandTaken`, `CommandRefused`, `PlanDrafted`, `PlanStateChanged`, `PlanAmended`, `HeldByOrder`, `IntentFormed` and `IntentWithdrawn`, whole, and the enemy sees none of them. Each of these kinds is decided in `side_events.rs`. The two events of step 5, `StaffNotice` and `InterruptFired`, are the same (Interrupts and Notices, below). [0107, ruling 23; 0117]

//! Scenario 130: a limit binds what it names. A commander's forces limit
//! binds only the kinds it names, a forces limit and an area can be
//! cleared, an order is read and signed with its limits as one, a reason
//! says which limits it speaks of, and supply formed under a ranked order
//! is ranked with it.
//!
//! Proves the second step of decision 0222 as amended on review: rules 2
//! to 7 of the section "Amended on review". The data is placeholder data.
//! Nothing here is computed: every statement is of what the staff forms,
//! orders and says, so there is no arithmetic to work, and each game is
//! described by what it must hold. The builder makes each game as small
//! as holds it.
//!
//! # The statements
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a statement here to make a test pass. A mismatch
//! goes back to the architect, and so does a statement that cannot be so
//! given the code.
//!
//! ## Part A: a forces limit binds only the kinds it names (rule 2)
//!
//! The setting: a side with a commander and a staff; a base B with a
//! bomber group and a search group in reach of some waters W; a carrier
//! force C and a surface force S at sea; an order to deny W.
//!
//! 1. With no forces limit, the staff forms the tasks at B for W that it
//!    forms today. Call them the base tasks.
//! 2. With a forces limit on the order that names C and nothing else, the
//!    staff still forms the base tasks, with the same groups, and C
//!    serves the order. Before this rule the order had no base at all.
//! 3. With a forces limit that names the bomber group of B and nothing
//!    else, the search group of B is not used by the order (air groups
//!    are named, so air groups are bound), and a task force the staff
//!    would have used with no limit is still used (task forces are not
//!    named, so they are the staff's).
//! 4. With a forces limit that names C, the staff stations C as it
//!    stations a free carrier force for that order: at the hex its rule
//!    chooses, not at the hex C lies in that morning. Where the commander
//!    gives C the task himself (an order to operate C from a hex), C works
//!    from that hex.
//! 5. A limit that names a land unit binds land units and nothing else; a
//!    limit that names a boat binds boats and nothing else; a limit that
//!    names a source binds the source and nothing else. One game each, on
//!    an order that reads the kind, showing a kind not named still chosen
//!    by the staff.
//! 6. Two limits on the chain, one on the order and one above it: a unit
//!    of a kind both name is used only where both name it; a kind only
//!    one names is bound by that one alone.
//! 7. Under an order to take a place and an order to control an island
//!    the staff forms, orders and says what it did before this rule: the
//!    staffed games on file with such orders give the same orders and
//!    commands.
//! 8. The staffed games on file whose limits name forces on other kinds
//!    of order are listed by the test's own run, each with whether its
//!    orders moved. The list is written into the decision's "As built";
//!    no number of them is asserted here.
//!
//! ## Part B: clearing (rule 3)
//!
//! 9. A forces limit set and then cleared leaves the order as if none had
//!    been set: the staff forms what statement 1 has. The clearing is a
//!    command like the setting, is refused for an order not the side's,
//!    and is an amendment of a plan in force as the setting is.
//! 10. The same for an area.
//! 11. Clearing a limit that is not set changes nothing and is not
//!    refused.
//! 12. A forces limit that names nothing is still what it is today,
//!    "use nothing", and is not the same as cleared: with it the staff
//!    forms no task for the order and says the limit names nothing.
//!
//! ## Part C: an order read and signed with its limits (rule 4)
//!
//! 13. A preview of an order with a forces limit, a risk and an area
//!    given with it shows what the staff would do under all three: the
//!    same as the staff does on the first morning after the order and the
//!    three limits are signed.
//! 14. Signing that preview queues the order and its limits together;
//!    taking the order back takes its limits back with it.
//! 15. A preview with a limit the engine would refuse says the refusal
//!    and queues nothing.
//! 16. A preview at "press" differs from one at "preserve" where the
//!    doctrine reads the risk: one game in which a cover's share of the
//!    patrol differs, shown in both previews.
//!
//! ## Part D: a reason says which limits it speaks of (rule 5)
//!
//! 17. A reason whose words say a forces limit holds a thing back carries
//!    that limit in its list: its kind, forces, and the order it is set
//!    on, which may be an order above the one the reason is of.
//! 18. The same for an area, a source, a rank, a risk, the targets and
//!    each of a landing's four, each on one game in which the staff gives
//!    that reason today.
//! 19. A reason that speaks of no limit carries an empty list, and its
//!    words are what they were.
//! 20. Recording the list changes no order, no command and no hash.
//!
//! ## Part E: supply ranked with its order (rule 6)
//!
//! 21. Two orders to hold two places, each with supply the staff forms
//!    under it, and ships enough for one run: with the second order
//!    ranked first, its place is supplied first. Before this rule the
//!    lower number was, whatever the ranks.
//! 22. With neither ranked, the lower number is supplied first, as today.
//! 23. A rank on the supply itself, where the commander gave that order,
//!    is read before a rank above it.
//!
//! ## Part F: what does not move
//!
//! 24. A game with no forces limit, no area and no rank gives the orders,
//!    commands, reasons' words and hash it gave. The staffed scenarios on
//!    file are compared before and after; those that move are the ones
//!    statement 8 lists and the ones statement 21 changes, and no other.

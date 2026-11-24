//! Flag Plot staff: what a side's staff decides, from its view alone
//! (decision 0107, step 3, stage 1).
//!
//! `sim-core` defines what a staff is ([`sim_core::command::Staff`]); this
//! crate provides the one there is, [`SupplyStaff`], with the doctrine for
//! keeping a base supplied. Every function here takes a
//! [`sim_core::view::SideView`] (and plain numbers beside it) and nothing
//! else: no world, no umpire's view, no full event log. A test reads the
//! source to check. Floating point is forbidden, as in `sim-core`, because
//! the staff's decisions reach the hash.
//!
//! The staff holds no state of its own: asked twice on the same view it
//! gives the same answer. What it remembers is in the side's command state,
//! through the staff commands it hands in.
//!
//! Every loop that resolves days takes its staffs from [`staffs`], so that a
//! runner and `replay --check-staff` ask the same staff. A saved game
//! records [`DOCTRINE_VERSION`].

#![deny(clippy::float_arithmetic)]

mod air;
mod ammunition;
mod assemble;
mod available;
mod beside;
mod bombard;
pub mod carrier;
mod cover;
mod deny;
mod doctrine;
mod escort;
mod fleet;
mod follow;
pub mod fuelling;
mod handin;
mod holding;
mod island;
mod lifts;
mod look;
mod morning;
mod neutralise;
mod night;
mod order;
mod patrol;
mod picture;
mod planner;
mod projection;
mod proposal;
pub mod raid;
mod ready;
mod reasons;
mod route;
pub mod search;
mod source;
pub mod stand;
pub mod station;
pub mod strike;
mod take;
pub mod targets;
mod told;
mod turn_back;
mod units;
mod waters;
pub mod way;

pub use available::{counted_by_a_take, Counted};
pub use projection::project;

use std::collections::BTreeMap;

use sim_core::command::{Proposal, Staff, StaffDecisions, Staffs};
use sim_core::ids::IntentId;
use sim_core::state::Side;
use sim_core::view::SideView;

/// The version of the doctrine. A change to what a staff decides from a
/// view changes it. A saved game records it; a game is resumed for more
/// play only under the version it was begun under. `supply-2` adds the
/// priority order with what is handed in this morning counted as committed,
/// the protected limit, and displaced demand. `supply-3` adds holds, runs
/// set aside, cargo aboard carried, and the fallback of a cancelled plan.
/// `supply-4` adds the notices: cannot meet, waits for a leave, a run ended.
/// `supply-5` hands in each reason with its intent's statement in words, and
/// names a plan with what it is for where a reason's working names no base.
/// `supply-6` counts the fuelling hours at the source before it calls a ship
/// free (decision 0112): the morning after the day it has finished fuelling,
/// at the source's fuel berth rate or lighter rate; with no rate given, as
/// before, the morning after it is back.
/// `supply-7` frees the ships of a plan that has ended first in the
/// morning, before any intent is served, where it freed them last; an intent
/// served that morning may then commit them (found by the soak ashore,
/// scenario 45, decision 0093 item 10).
/// `supply-8` adds the air doctrine after the supply doctrine (decisions
/// 0115 and 0117): the leaves for a hold or cover intent, forming cover from
/// the air under a held base, and each cover intent's patrol, search and rest
/// orders for the air groups at its base. A side with no hold or cover
/// intent gets exactly what `supply-7` gave.
/// `supply-9` adds decision 0120: the neutralise doctrine at the area's
/// level, which forms raid tasks and gives no order, and the raid doctrine at
/// the base's level; a group a raid task may use as a bomber is the raid's
/// and not the cover's; every reason says which level decided. A side with
/// no neutralise or raid intent gets the orders `supply-8` gave.
/// `supply-10` adds decision 0123: the deny doctrine at the area's level,
/// which forms strike-ships and search-waters tasks, the strike doctrine at
/// the base's level with each strike's sizing decided at the group's, the
/// search doctrine, and a cover's arc that takes the waters as threats. A side
/// with no deny, strike-ships or search-waters intent gets the orders
/// `supply-9` gave.
/// `supply-11` adds decisions 0124 and 0125: a task force as an addressee,
/// the stand and carrier tasks formed under a deny or a hold intent whose
/// forces limit names task forces, and their doctrines at the force's level.
/// A side whose intents name no task force gets the orders `supply-10` gave.
/// `supply-12` adds decision 0126: under a keep-supplied intent whose forces
/// limit names task forces, the threat to a run, an escort from the named
/// forces on the escort mission by the risk level, the escort's charge given
/// and cleared, and a stand task at the destination for each named surface
/// force. An intent that names no task force gets the orders `supply-11`
/// gave.
/// `supply-13` adds decision 0127: a surface force short of ammunition by its
/// risk level does not sortie and is not a free escort, with the day from
/// which it may be used read with its fuel. A force short of nothing gets
/// the orders `supply-12` gave.
/// `supply-14` adds the architect's ruling of Nov 16, 2026, on decisions
/// 0124, 8, and 0125, 2: the staff forms no stand task and no carrier task
/// for a force that already has a task of that kind in force, whoever made
/// it and wherever it stands. A side with no such task stated beside an end
/// that names the force gets the orders `supply-13` gave.
/// `supply-15` adds decision 0130: a carrier force is routed at cruise where
/// that brings it there in the night and at full speed where it does not,
/// and closes where the first fresh contact of a kind it wants within a
/// night and its bombers' reach is beyond that reach, stays while it is in
/// reach and goes back to its hex when there is none; the passage home for
/// fuel is at cruise, and a ship the engine's measure has run dry short of
/// a passage is short of the fuel for it, in the carrier and stand doctrines.
/// `supply-16` adds decision 0131: a search is flown once a day at preserve
/// the force and at balanced (in both pulses at press) by the cover, the
/// search of waters and the carrier force; the least tired bomber group
/// that can search and fly takes the search each morning on a carrier force
/// with no group that can only search, and at a base whose cover or search
/// task has none where a strike or raid task there has two or more; and a
/// carrier force with a ship out of fuel is given no route.
/// `supply-17` amends decision 0131's rule 6 for a carrier force: its
/// search is flown once a day at preserve the force alone, and in both
/// daylight pulses at balanced and at press; a base's cover and its search
/// of waters are as `supply-16` had them.
/// `supply-18` adds decision 0136: every doctrine addressed to the area
/// reads the side's own units as the area command holds them, with the
/// moment a held state is of in a reason where it is not the pulse just ended,
/// and a carrier force and a surface force that stands by night are set to
/// radio silence at preserve the force where report figures are loaded. With
/// no report figures, or command levels off, the orders and reasons are those
/// `supply-17` gave.
/// `supply-19` adds decision 0137: a carrier force whose command has a
/// commander weighs a contact that is its own at one level of risk more
/// careful (cautious) or bolder (bold) than its task's in the closing rule,
/// and says so in the reason. With no commander, a steady one, command
/// levels off or no contact of its own, the orders and reasons are those
/// `supply-18` gave.
/// `supply-20` adds decision 0140: the staff leaves out a unit before its
/// day (a ship in no pool, a task force no task may use, an air group no
/// intent may use) and names the day in its reasons ("may be used from day
/// 5"); an intent addressed to a force before its day gets one reason and no
/// order. With no day, the orders and reasons are those `supply-19` gave.
/// `supply-21` adds decision 0141: the tasks "ready a landing" and "build up
/// a base", which give land units their activity and a force its landing
/// order, take leave to prepare and never ask leave to launch, and hand in
/// the notices ready, late and met once each. A side with neither task gets
/// the orders and reasons `supply-20` gave.
/// `supply-22` adds decision 0142: the end "take a place", which chooses
/// the units that land and forms "ready a landing" for them, asks leave to
/// launch when the landing is ready, and on leave loads supply, embarks the
/// men, sails and lands them on attack, and marks the plan complete when the
/// place is the side's; a "ready a landing" task whose force lies at no base
/// of its side is ready as it stands. A side with no "take a place" gets the
/// orders and reasons `supply-21` gave, but for a "ready a landing" task
/// whose force lies away from a base of its side.
/// `supply-23` adds decision 0143: ashore, each morning, "take a place"
/// sets its men to attack when those that can attack are enough against the
/// side's estimate for the risk, and to defend when they are not; it sends
/// the units its first lift left behind in a second lift, which goes as the
/// force stands; and the notice that the end is met says the intent's words.
/// `supply-24` adds decision 0144: a hold of a base not the side's waits for
/// it; once it is the side's, the men of the hold on attack are set to
/// defend and "keep supplied" is formed under the hold from the source its
/// forces limit names; "take a place" with such a hold beside it checks that
/// the place can be held before it asks leave to launch, and lands the
/// supply that lasts until the first run follows, with a margin for the
/// risk. A side with no "take a place" and no hold of a place with men, or
/// not its own, gets the orders and reasons `supply-22` gave.
/// `supply-25` adds decision 0145: at preserve the force and at balanced, a
/// supply run and a lift of "take a place" to a place with an enemy base on
/// the side's map within 15 hexes and not in its hex are timed to arrive in
/// the night, at cruise or else at full speed with the fuel reckoned at that
/// speed, carry no more in each ship than the night's hours can land, and
/// where no speed arrives in the dark go by day at balanced or do not sail
/// at preserve the force. A side with no such run gets the orders and
/// reasons `supply-24` gave.
/// `supply-26` adds decision 0146: the task "bombard a base", formed under
/// a released "neutralise a base" or "take a place" for each named force on
/// the bombardment mission and served at the force's level as a stand by
/// night, its route acting at the target's hex. A side with no such task and
/// no force on the bombardment mission named under those ends gets the
/// orders and reasons `supply-25` gave.
/// `supply-27` adds the architect's amendment of decision 0145, item 2a: a
/// run to a place where a base of the side's own has a fighter group of the
/// side's based at it goes as before, at every risk. A side with no run by
/// night to such a place gets the orders and reasons `supply-26` gave.
/// `supply-28` adds the architect's ruling 6 on decision 0145: item 2a
/// reaches a base of the side's own in a hex next to the place as well. A
/// side with no run by night to a place next to its own fighters gets the
/// orders and reasons `supply-27` gave.
/// `supply-29` adds decisions 0147 and 0148: under "take a place" the
/// staff sends an air group to look at the place, chooses its units again
/// on the first count seen from the air or by men in contact, and launches
/// only on the count the risk asks for; and it forms "operate a carrier
/// force" at a covering hex for each named carrier force, whose bombers
/// strike the enemy's base at the place on a morning with no ship to strike,
/// and sends the force home when the place is taken. A side with no "take a
/// place" gets the orders and reasons `supply-28` gave.
/// `supply-30` adds decision 0149: a "take a place" whose forces limit
/// does not name all three of the task forces, the land units and the
/// source has the rest chosen by the staff (the source by the lift each
/// base gives, the units, the transport force and the forces to bombard,
/// cover and escort), kept as the forces limit of the task it forms. A side
/// with no "take a place", or one whose forces limit names all three, gets
/// the orders and reasons `supply-29` gave.
/// `supply-32` adds decision 0152: a transport force with men aboard is a
/// lift as it stands, taken by "take a place" as its first lift before any
/// base and sent as a later lift once the first is in, sent by "hold a
/// place" where no take can use it, and ready as it stands wherever it lies;
/// the supply under an island's holds keeps its source and sets each hold's
/// forces limit each morning to a pool that only grows. A side with no force
/// with men aboard and no island gets the orders `supply-31` gave.
/// `supply-33` adds decision 0151, item 8c: under "control an island" a
/// take takes no force on the bombardment mission, and each such force
/// stopped at a base of the side is given a bombard task under the island's
/// first neutralising of an enemy base at a named place. A side with no
/// island, or none with a force on the bombardment mission, gets the orders
/// `supply-32` gave.
/// `supply-34` adds decision 0152, item 5b: a take higher in priority that
/// sends a later lift as it stands that morning is given it before a take
/// lower in priority chooses its first lift; and, by the architect's ruling
/// on reading 7, a first lift with no ship afloat is lost and so no longer
/// on its way. A side with at most one take in force, and none whose first
/// lift has lost every ship, gets the orders `supply-33` gave.
/// `supply-35` adds decision 0159, rule A: a "hold a place" whose base is not
/// the side's, where the side has a base of its own in that hex and men
/// ashore there, forms "keep supplied" for the side's own base under the
/// hold, as it does for a base that is the side's, and its reason says
/// through which base the men are fed. A side with no such hold gets the
/// orders `supply-34` gave.
/// `supply-36` adds decision 0159, rule B: a "take a place" whose first lift
/// would sail short of the men wanted, at preserve the force or balanced,
/// waits, as a draft where it is one, for the first day in reach that brings
/// them (7 days ahead, or what can still be ready by its latest day), and at
/// balanced, where no such day comes, hands in the notice that it cannot
/// bring enough once and forms nothing. A side with no take short of men
/// gets the orders `supply-35` gave.
/// `supply-37` adds decision 0159, rule C: a "take a place" whose landing is
/// ready is not released, and leave to launch is not asked, while a limit
/// "not before", "wait for" or "after" read up its tree is unmet; a limit
/// that can never be met as it stands is said once with the notice that the
/// intent cannot be met. A side with no such limit gets the orders
/// `supply-36` gave.
/// `supply-38` adds decision 0159, rule D, items 12 and 13: a "knowledge"
/// limit read up a take's tree sets how much must be known before the
/// landing goes (none, a count, or a count no more than so many days old),
/// in place of the risk, which still sets the men wanted; and the need to
/// know of each take whose plan is preparing is worked as data. A side with
/// no such limit gets the orders and reasons `supply-37` gave.
/// `supply-39` adds decision 0159, rule D, item 14, as the architect ruled
/// it on Nov 18, 2026: a take whose plan is preparing, whose need to know is
/// not met, with no group its forces limit names that reaches the place, and
/// a group it names that can search aboard a carrier force it names, forms
/// that force's carrier task then, at the covering hex where the scout
/// reaches the place, else the first hex on her way where one does (at
/// preserve the force only with leave to launch); before release the task
/// strikes nothing at the place and gives a group looking at it no order. A
/// side with no such take gets the orders `supply-38` gave.
/// `supply-40` adds decision 0162: a supply run by night names in its load
/// order the most any one ship is to hold, the least of the ships' limits,
/// and is cut only to what those levels allow, in place of the cut to the
/// first ship; the speed is chosen on the load as spread. A side with no
/// supply run by night gets the orders `supply-39` gave; one whose run was
/// cut now carries more.
/// `supply-41` adds decision 0163: under "control an island" the source of
/// supply is the side's base with the most supply above its reserve, named
/// on the first morning with no transport force lying there; and a lift of
/// a take, or a force a hold sends, that has landed goes on to the source
/// of the hold of the place where every ship has the fuel, else back where
/// it came from. A side with no "control an island" and no hold whose
/// source differs from where its lifts came from gets the orders
/// `supply-40` gave.
/// `supply-42` adds decision 0169: at balanced a lift of a take with men
/// aboard, a later lift as it stands or a second lift, short of the men
/// wanted with too few in reach by the take's last day, goes only where its
/// men can be fed: the place can be held by decision 0144's check and what
/// it has aboard lasts until supply follows. Where they cannot, the take
/// does not lift and hands in `cannot_bring_enough` once. A side whose
/// takes hold no estimate, or whose lifts are not short, gets the orders
/// `supply-41` gave.
/// `supply-43` adds decision 0191: the first lift of a released take, with
/// men aboard and not landing, is turned back to its source by the risk
/// while a fresh contact reported as a carrier is within 8 hexes of its way
/// to the place, kept back until two clear mornings, and held at its
/// passage on such a morning. A side that holds no contact reported as a
/// carrier near a lift's way gets the orders `supply-42` gave.
/// `supply-44` adds decision 0192: a stand task's force with no home port,
/// not stopped at a base of the side, is routed to its station, one way, on
/// the first morning it is not there and not under way to it, and lies
/// stopped there; its fuel is weighed to the nearest base by sea that can
/// fuel it, and it goes in to that base, its home port from then, when a
/// ship is short of the passage there. A force with a home port stopped at
/// sea on no route is routed home at cruise. A side with no such stand gets
/// the orders `supply-43` gave.
/// `supply-45` adds decision 0190: a targets limit on the chain of a task
/// that strikes ships sets its bombers' target list to the limit's kinds at
/// every risk; a deny whose chain carries one with `waters_first` gives the
/// bombers it binds its waters to strike first, adds them to a carrier
/// force's search arc and has the force close on a contact in them. A side
/// that sets no targets limit gets the orders `supply-44` gave.
/// `supply-46` adds decision 0193: on the morning of the judgement ashore,
/// where the men that can attack are fewer than the men wanted and nothing
/// more is coming to the place, the men wanted fall to the estimate at
/// balanced and to none at press, a unit on attack is not set back to
/// defend at balanced or press, and where it waits the staff hands in
/// `cannot_bring_enough` on each such morning. A side with no take whose
/// men ashore are short with nothing more coming gets the orders and
/// reasons `supply-45` gave.
/// `supply-47` adds decision 0194: with fewer than 3 days of the game left
/// the judgement ashore sets no unit on defend to attack, and sets to
/// defend a unit on attack whose side's attack at the place failed the day
/// before; and in the test of what is coming the latest day is the end's
/// date or the game's last day, whichever is the earlier. A side with no
/// take whose men are ashore in the game's last days, and none that waits
/// on a unit or lift due after the game's end, gets the orders `supply-46`
/// gave.
/// `supply-48` adds decision 0195: the staff reads what is held apart for the
/// side until its day as a unit that may be used from its day: a force
/// named by an intent and not yet on the map is given no order and a reason
/// "{force} enters on day {day}", and a force or land unit held apart counts
/// as coming where a take counts what it may use and where more is coming.
/// With nothing held apart its orders and reasons are those `supply-47` gave.
/// `supply-49` adds decision 0196: the staff counts the side's supply lines in
/// its projections of a base's stock and in the day a source falls to its
/// reserve, and names them in its reasons. With no line its orders and
/// reasons are those `supply-48` gave.
/// `supply-50` adds decision 0202: a commander's deny or neutralise whose
/// forces limit names no task force takes every free carrier force of the
/// side, in one pass over them in priority order, at a station chosen by the
/// risk, the circles of danger of the enemy's bases and fresh carrier
/// contacts, and the fuel; the force searches its way ahead while it goes
/// there, and under a neutralise its bombers raid the target. A side with no
/// such end, or no free carrier force, gets the orders `supply-49` gave.
/// `supply-51` adds decision 0201, stage A: each morning a carrier or stand
/// task's force with no home port and a ship below a quarter of her capacity,
/// and no oiler's force to send it to, is named in a reason of the task; in
/// the carrier and stand doctrines a force that would go home for fuel is
/// given the order to fuel from an oiler's force nearer than home, and a
/// force with no home port one with a ship below a quarter, and while the
/// order stands it is given no route; and a stand force with a home port
/// stopped at sea whose order ended the day before and that has the fuel goes
/// back to its station. A side with no such force gets the orders and reasons
/// `supply-50` gave.
/// `supply-52` adds decision 0203, stage 1: a carrier force going to a
/// station the staff chose, at balanced or preserve, is given the way of
/// least danger by day among those that arrive within its risk's lateness and
/// are in fuel, as the points of a route given once and kept while it is
/// under way on it; where the shortest way is taken it is routed as before,
/// its reason saying why where that way lies in the enemy's reach by day. A
/// side with no such force, or whose force's shortest way is the one taken,
/// gets the orders `supply-51` gave, and the same reasons where that way is
/// in fuel and enters no hex inside a circle of danger by day.
/// `supply-53` adds decision 0214: a carrier force whose closing rule gives
/// nothing, because every fresh contact of a kind it closes on is beyond a
/// night and its bombers' reach, looks for a find within `pursuit_nights`
/// nights, inside the order's area; with the order's leave to pursue at go it pursues the find and says
/// so, at ask it asks leave in a notice, and on the commander's yes it
/// pursues; and at the first dusk it has neither a find nor a contact to close
/// on the pursuit is ended in a notice. A side with no such find, or with the
/// leave at stay, gets the orders and reasons `supply-52` gave.
/// `supply-54` adds decision 0217: a deny that no base can serve but a
/// carrier task under it does, whoever made the deny (the staff's own under
/// "control an island" included), hands in no notice that it cannot be met,
/// and its reason for giving the bases no task names the carrier tasks
/// first; and the reason of a carrier station the staff chose carries its
/// cautions (the station stands in danger; the fuel was not measured). It
/// gives the orders `supply-53` gave. `supply-55` adds decision 0218, stage
/// 3: the patrol of submarines (a patrol intent gives each boat it may use
/// the station of her place in the waters and orders her there, and gives no
/// order, with its reason, to a boat that keeps her station or cannot serve)
/// and the staff's regard for the waters a side suspects enemy submarines
/// work in (a station inside them is passed over where a place outside
/// serves, and carries the caution `station_in_submarine_waters` where none
/// does). It gives the orders `supply-54` gave to a side with no patrol
/// intent and no suspected waters. `supply-56` adds decision 0222, rule 1:
/// the reason of a hold with no source on it or above it names the bases of
/// the side that could be one by the island's test of a source, each with
/// its supply above its reserve, or, where none could, says that no other
/// base of the side holds supply above its reserve and the place lives on
/// what it has, with its days of supply today, and offers nothing. It gives
/// the orders `supply-55` gave. `supply-57` adds decision 0222, rules 2, 5
/// and 6: a commander's forces limit binds only the kinds it names or lists
/// in `none_of` (outside a take and an island, which keep their own rules),
/// so a limit that names task forces leaves the bases' air groups to the
/// staff; a carrier force named in a commander's deny or neutralise is
/// stationed as a free one is, and one under a hold says it works from where
/// it lies; a reason records the limits it speaks of; and supply is served
/// in rank order read up the tree. It gives the orders `supply-56` gave to a
/// side whose forces limits name no carrier force of a deny or neutralise
/// without its own task and name every kind they mean to bind. `supply-58`
/// adds decision 0223, stage A: where a side's scenario gives it a submarine
/// pattern, the stations of a patrol are given by it, on lines across the
/// approach from the nearest enemy base more than 5 hexes from the centre
/// (`lines`) or off the enemy bases in or next to the waters (`off_bases`),
/// the boats left spread as before counting the stations given, each station
/// order's reason saying which rule gave it. It gives the orders `supply-57`
/// gave to a side with no pattern. `supply-59` adds decision 0228: where a side's
/// scenario states that its staff chooses its sources of supply, one rule
/// chooses the source under a commander's hold and under "control an island"
/// (a base that can spare the place's need, one with a transport force empty
/// there, the most to spare, the nearer, the lower number), the staff forms the
/// supply under a hold with no source named and sets its forces limit each
/// morning, a run from a source the staff chose is cut to what the source can
/// spare, an empty pool of ships the staff is to find does not mark a plan
/// failed, and a source the commander names is not refused for another
/// order's draw on it. It gives the orders `supply-58` gave to a side without
/// the key. `supply-60` adds decision 0220's order to destroy a force and
/// gives the orders `supply-59` gave to a side that gives none. `supply-61`
/// adds decision 0229: under "control an island" the staff forms a "keep
/// supplied" for each base of the side on the island's land and in no named
/// place's hex where the side has men ashore or marching, sets its forces
/// limit as a hold's, withdraws it when no man is there, none marches to it
/// and no run of it is loaded or at sea, and records one reason for men at a
/// hex with no base of the side. It gives the orders `supply-60` gave to a
/// side with no such base and men. `supply-62` adds decision 0230: the floor
/// of a "keep supplied" the staff forms is the margin of its risk and never
/// less than 1 day (10, 5, 1), at all three sites that form one, and the
/// reason says "the least floor, at press" where it is 1 at press. It gives
/// the orders `supply-61` gave to a side with no hold or island at press.
pub const DOCTRINE_VERSION: &str = "supply-62";

/// The staff: the doctrine for keeping a base supplied (stage 1) and, after
/// it, the air doctrine for holding a place and covering a base from the air
/// (step 2 of stage 2). A side with no intent in force and no commitment to
/// free gets nothing from it; one with no hold or cover intent gets nothing
/// from the air doctrine.
#[derive(Clone, Copy, Debug, Default)]
pub struct SupplyStaff;

impl Staff for SupplyStaff {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions {
        let mut decisions = doctrine::decide(view);
        reasons::state_intents(&view, &mut decisions);
        // The air doctrine states its own reasons on the state it planned.
        decisions.append(air::decide(&view));
        decisions
    }

    fn propose(&self, view: SideView<'_>, intent: IntentId) -> Option<Proposal> {
        if view
            .command_state()
            .intents
            .get(&intent)
            .is_some_and(|found| air::is_air(found.kind))
        {
            return air::propose(&view, intent);
        }
        let mut proposal = proposal::propose(view, intent)?;
        reasons::state_intents_of_proposal(&view, &mut proposal);
        Some(proposal)
    }

    fn needs_to_know(&self, view: SideView<'_>) -> Vec<sim_core::command::NeedToKnow> {
        air::needs_to_know(&view)
    }

    fn plan_orders(&self, view: SideView<'_>) -> Vec<sim_core::command::PlanOrder> {
        air::plan_orders(&view)
    }
}

/// The staff of each side: the supply staff for both. A side with no intent
/// has a staff that does nothing.
pub fn staffs() -> BTreeMap<Side, Box<dyn Staff + Send>> {
    Side::ALL
        .into_iter()
        .map(|side| (side, Box::new(SupplyStaff) as Box<dyn Staff + Send>))
        .collect()
}

/// The engine's [`Staffs`] over staffs held in a map, as [`staffs`] gives
/// them.
pub fn as_staffs(held: &BTreeMap<Side, Box<dyn Staff + Send>>) -> Staffs<'_> {
    let mut staffs = Staffs::none();
    for (side, staff) in held {
        staffs = staffs.with(*side, staff.as_ref());
    }
    staffs
}

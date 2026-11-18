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
mod bombard;
pub mod carrier;
mod cover;
mod deny;
mod doctrine;
mod escort;
mod fleet;
mod handin;
mod holding;
mod island;
mod lifts;
mod look;
mod morning;
mod neutralise;
mod night;
mod picture;
mod planner;
mod projection;
mod proposal;
pub mod raid;
mod ready;
mod reasons;
mod route;
pub mod search;
pub mod stand;
pub mod strike;
mod take;
mod units;

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
pub const DOCTRINE_VERSION: &str = "supply-34";

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

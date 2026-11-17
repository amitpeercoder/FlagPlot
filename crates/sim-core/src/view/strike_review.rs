//! A side's review of one naval strike group's decision in one pulse (decision
//! 0138, item 2): the answer the rules gave on the picture of the group's
//! command, and for each contact of that picture why it was or was not
//! struck.
//!
//! The review is the question of decision 0100 asked of a pulse that has
//! been decided, at the moment of its strike decision, and written out in
//! full. It writes no rule of its own. The tests of a contact are
//! [`contact_verdict`](crate::airstrike), the gates of the group are
//! [`strike_group_gate`](crate::airstrike), the sizing is
//! [`strike_walk`](crate::airstrike), the answer is [`SideView::naval_strike`].
//! It reads the side's own groups and the picture of the group's command,
//! and nothing of the enemy: a contact is read as the command's picture
//! holds it, not as it is.

use std::collections::BTreeMap;

use crate::airstrike::{
    contact_verdict, contacts_to_strike, strike_age_limit, strike_aircraft, strike_group_gate,
    strike_walk, NavalStrike, StrikeGate,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, ContactId, EventId};
use crate::orders::{AirMission, Reach};
use crate::pictures::CommandRef;
use crate::refdata::{AttackWeapon, ShipKind};
use crate::time::Pulse;
use crate::view::SideView;

/// What the strike rule makes of one contact for one group: the tests of
/// age, kind and reach, in the order they are applied (decision 6 of 0063).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContactVerdict {
    /// Young enough, a kind on the list and within reach: a contact the
    /// group could launch at.
    Qualifies,
    /// Older than the group's strike contact age allows.
    TooOld { age_pulses: u32, limit_pulses: u32 },
    /// Taken for a kind that is not on the group's target list.
    KindNotOnList,
    /// Farther from the group than it may strike, at the reach it is set to.
    OutOfReach { distance_nm: u32, reach_nm: Fixed },
}

/// A gate of the group itself that stops it launching, whatever contacts its
/// command holds, as the strike decision applies them in order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StrikeStop {
    /// On neither strike mission (the review of a group is made for the naval
    /// strike mission).
    NotOnStrikeMission,
    /// On the base strike mission, which decides at another step and reads no
    /// contact. Used by the review only.
    OnBaseStrike,
    /// No aircraft ready.
    NoReadyAircraft,
    /// No position: aboard no ship afloat and at no base.
    NoPosition,
    /// The pulse is dark where the group is.
    Dark,
    /// Its aircraft type carries no weapon.
    NoWeapon,
    /// No aircraft able to fly: a flight deck out, fatigue, a base grounded
    /// or without supply, no pilot, a move by order that day.
    NothingAbleToFly,
    /// Aircraft able to fly, but the share held back leaves none.
    AllHeldBack,
}

/// Why a contact of the picture was or was not struck by the group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContactFate {
    /// The group launched, or would launch, at it, with this many aircraft.
    Chosen { aircraft: u32 },
    /// Older than the group's strike contact age allows.
    TooOld { age_pulses: u32, limit_pulses: u32 },
    /// Beyond the reach the group is set to.
    OutOfReach { distance_nm: u32, reach_nm: Fixed },
    /// A kind not on the group's target list.
    KindNotOnList,
    /// It qualified, but another contact came first in the group's order
    /// (the kind first on its list, then the nearest, then the lowest
    /// number): this one, by its number in the picture.
    CameFirst { chosen: u32 },
    /// It qualified, but the groups before this one had sent at it all that
    /// it needs (decision 0118): the need (`None`: the kind has no figure)
    /// and what had been sent.
    Served { need: Option<u32>, sent_before: u32 },
    /// It qualified, but the group cannot launch at all.
    GroupStopped(StrikeStop),
}

/// One contact of the command's picture and what the rule made of it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ContactReview {
    /// The contact's number in the command's picture.
    pub number: u32,
    pub id: ContactId,
    pub hex: Hex,
    /// The kind the contact is taken for.
    pub kind: ShipKind,
    pub ships: u32,
    pub age_pulses: u32,
    /// The report the contact rests on: the event that recorded the report
    /// that last made or refreshed it.
    pub report_event: EventId,
    /// Whether the command made the report itself, rather than being told of
    /// it (decision 0137, 5); false for every contact with command levels
    /// off.
    pub own: bool,
    pub fate: ContactFate,
}

/// The review of one group's strike decision in one pulse.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StrikeReview {
    pub group: AirGroupId,
    pub pulse: Pulse,
    /// The command the group belongs to, whose picture the decision read.
    pub command: CommandRef,
    /// Whether command levels are on in this game.
    pub command_levels: bool,
    /// Whether reports travel between commands in this game.
    pub reports_travel: bool,
    pub mission: AirMission,
    /// Where the group is; `None` if it has no position.
    pub hex: Option<Hex>,
    pub reach: Reach,
    /// How far it may strike, in miles, at the reach it is set to.
    pub reach_nm: Fixed,
    /// The oldest contact, in pulses, it launches on.
    pub age_limit_pulses: u32,
    /// Its target list, most important first.
    pub target_list: Vec<ShipKind>,
    /// The aircraft it is able to fly (the question `AircraftAbleToFly`).
    pub able_to_fly: u32,
    /// The share of them it holds back.
    pub held_share: Fixed,
    /// The aircraft a strike of it sends: those able to fly less the share
    /// held back.
    pub can_send: u32,
    /// Whether the pulse is dark where the group is.
    pub dark: bool,
    /// Whether the group sizes its strikes (decision 0118).
    pub sizing: bool,
    /// The first gate of the group itself that stops it, if one does.
    pub stop: Option<StrikeStop>,
    /// The answer of the naval strike question at this moment, whatever the
    /// gates say: the contact and the aircraft it would send, from
    /// [`SideView::naval_strike`].
    pub answer: Option<NavalStrike>,
    /// Every contact of the command's picture, in the picture's order (the
    /// contact number).
    pub contacts: Vec<ContactReview>,
}

impl StrikeReview {
    /// What the strike decision launches: the answer, if no gate stops the
    /// group and it sends at least one aircraft. `None`: it launches nothing.
    pub fn launches(&self) -> Option<NavalStrike> {
        match self.stop {
            None => self.answer.filter(|strike| strike.aircraft > 0),
            Some(_) => None,
        }
    }

    /// The contact the review's own walk of the picture took, by its number
    /// in the picture: it must be the contact of [`StrikeReview::answer`].
    pub fn chosen_contact(&self) -> Option<&ContactReview> {
        self.contacts
            .iter()
            .find(|contact| matches!(contact.fate, ContactFate::Chosen { .. }))
    }
}

impl SideView<'_> {
    /// The review of what `group`, one of the side's own, decides in the
    /// strike decision of `pulse`, from the picture of its command: ask it at
    /// the moment of that decision (`Moment::Phase` with
    /// `PulsePhase::StrikeDecisions`), when the side's groups before it have
    /// launched nothing yet and the answer is the one the decision gives.
    /// `None` for a number that is not one of this side's groups, or a game
    /// without air combat figures.
    ///
    /// It is the question of decision 0100 written out: the group's
    /// facts, the answer, and each contact of the picture with the first test
    /// it failed or what came of it. It reads the side's own units and the
    /// command's picture, and nothing of the enemy.
    pub fn strike_review(&self, group: AirGroupId, pulse: Pulse) -> Option<StrikeReview> {
        let asked = self.air_groups().find(|own| own.id == group)?;
        let figures = self.ref_data.air_combat.as_ref()?;
        let kind = self.aircraft_type(asked.aircraft_type)?;
        let command = crate::pictures::command_of_group(self.truth, group)?;
        let origin = self.truth.air_group_hex(asked);
        let reach_nm = self.reach_nm(asked);
        let age_limit = strike_age_limit(asked, figures);
        let able = self.flyable_aircraft(asked);
        let can_send = strike_aircraft(able, asked.strike_held_share);

        // The gates of the group itself: the function the decision calls,
        // given the answer of the question it would ask.
        let gate = strike_group_gate(asked, origin, pulse, kind.attack_weapon, || able);
        let stop = match (asked.mission, gate) {
            (AirMission::BaseStrike(_), _) => Some(StrikeStop::OnBaseStrike),
            (_, Err(stop)) => Some(stop),
            (_, Ok(StrikeGate { .. })) => None,
        };
        let dark = origin.is_some_and(|hex| crate::time::is_dark(hex, pulse));

        // The picture the decision read, and the walk of it.
        let picture = self.picture(command);
        let view = self.command(command);
        let mut fates: BTreeMap<ContactId, ContactFate> = BTreeMap::new();
        if let Some(origin) = origin {
            let candidates = contacts_to_strike(picture, asked, origin, reach_nm, age_limit);
            let sent = self.sent_before(group);
            let walk = strike_walk(&candidates, asked, can_send, figures, &sent);
            let number_of = |index: usize| candidates[index].0.contact;
            for (index, sizing) in &walk.served {
                fates.insert(
                    number_of(*index),
                    ContactFate::Served {
                        need: sizing.need,
                        sent_before: sizing.sent_before,
                    },
                );
            }
            if let Some((index, strike)) = walk.taken {
                fates.insert(
                    number_of(index),
                    ContactFate::Chosen {
                        aircraft: strike.aircraft,
                    },
                );
                for (choice, _) in candidates.iter().skip(index + 1) {
                    fates.insert(
                        choice.contact,
                        ContactFate::CameFirst {
                            chosen: number_of(index).raw(),
                        },
                    );
                }
            }
        }
        let contacts = picture
            .contacts()
            .map(|contact| {
                let fate = match origin {
                    None => ContactFate::GroupStopped(stop.unwrap_or(StrikeStop::NoPosition)),
                    Some(origin) => {
                        match contact_verdict(contact, asked, origin, reach_nm, age_limit) {
                            ContactVerdict::Qualifies => match stop {
                                Some(stop) => ContactFate::GroupStopped(stop),
                                None => *fates
                                    .get(&contact.id)
                                    .expect("a qualifying contact is a candidate the walk read"),
                            },
                            ContactVerdict::TooOld {
                                age_pulses,
                                limit_pulses,
                            } => ContactFate::TooOld {
                                age_pulses,
                                limit_pulses,
                            },
                            ContactVerdict::KindNotOnList => ContactFate::KindNotOnList,
                            ContactVerdict::OutOfReach {
                                distance_nm,
                                reach_nm,
                            } => ContactFate::OutOfReach {
                                distance_nm,
                                reach_nm,
                            },
                        }
                    }
                };
                ContactReview {
                    number: contact.id.raw(),
                    id: contact.id,
                    hex: contact.estimated_hex,
                    kind: contact.estimated_kind,
                    ships: contact.estimated_ship_count,
                    age_pulses: contact.age_pulses,
                    report_event: contact.last_report_event,
                    own: view.is_some_and(|view| view.is_own(contact.id)),
                    fate,
                }
            })
            .collect();
        Some(StrikeReview {
            group,
            pulse,
            command,
            command_levels: self.command_levels(),
            reports_travel: self.reports_travel(),
            mission: asked.mission,
            hex: origin,
            reach: asked.reach,
            reach_nm,
            age_limit_pulses: age_limit,
            target_list: asked.target_priority.clone(),
            able_to_fly: able,
            held_share: asked.strike_held_share,
            can_send,
            dark,
            sizing: asked.strike_sizing,
            stop,
            answer: self.naval_strike(group),
            contacts,
        })
    }

    /// What the side's groups before `group` (in group number order) send at
    /// each contact in the pulse, as [`SideView::naval_strike`] works it: a
    /// group on the naval strike mission with ready aircraft and a weapon
    /// launches what its own walk of its command's picture gives.
    fn sent_before(&self, group: AirGroupId) -> BTreeMap<ContactId, u32> {
        let figures = self.ref_data.air_combat.as_ref();
        let mut sent = crate::airstrike::sent_at_contacts(self.pulse.strikes_in_flight, self.side);
        let (Some(figures), Some(_)) = (figures, self.air_groups().find(|own| own.id == group))
        else {
            return sent;
        };
        let mut before: Vec<_> = self
            .air_groups()
            .filter(|own| own.id < group && own.mission == AirMission::NavalStrike)
            .collect();
        before.sort_by_key(|own| own.id);
        for own in before {
            let armed = self
                .aircraft_type(own.aircraft_type)
                .is_some_and(|kind| kind.attack_weapon != AttackWeapon::None);
            if own.ready == 0 || !armed {
                continue;
            }
            let Some(launch) = self.naval_strike_of(own, figures, &sent) else {
                continue;
            };
            if launch.aircraft == 0 {
                continue;
            }
            *sent.entry(launch.choice.contact).or_insert(0) += launch.aircraft;
        }
        sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::airstrike::{
        contact_verdict, sized_strike, strike_walk, StrikeChoice, StrikeSizing,
    };
    use crate::belief::Contact;
    use crate::ids::{AircraftTypeId, ShipId};
    use crate::refdata::AirCombatFigures;
    use crate::state::{AirGroup, AirGroupLocation, Side};

    fn group() -> AirGroup {
        AirGroup {
            ready: 10,
            strength: 10,
            mission: AirMission::NavalStrike,
            ..AirGroup::new(
                AirGroupId::new(1),
                "Test group".to_string(),
                Side::Allied,
                AirGroupLocation::Ship(ShipId::new(1)),
                AircraftTypeId::new(1),
            )
        }
    }

    fn contact(number: u32, q: i32, kind: ShipKind, age: u32) -> Contact {
        Contact {
            id: ContactId::new(number),
            estimated_hex: Hex::new(q, 0),
            estimated_ship_count: 2,
            estimated_kind: kind,
            age_pulses: age,
            last_report_event: EventId::new(7),
        }
    }

    fn n(whole: i64) -> Fixed {
        Fixed::from_int(whole)
    }

    #[test]
    fn a_contact_is_judged_by_age_then_kind_then_reach() {
        let mut group = group();
        let origin = Hex::new(0, 0);
        let reach = n(250);
        let verdict = |c: &Contact, group: &AirGroup| contact_verdict(c, group, origin, reach, 2);

        // A hex is 40 nm: 3 hexes is 120 nm, 7 is 280.
        assert_eq!(
            verdict(&contact(1, 3, ShipKind::Carrier, 2), &group),
            ContactVerdict::Qualifies
        );
        assert_eq!(
            verdict(&contact(1, 3, ShipKind::Carrier, 3), &group),
            ContactVerdict::TooOld {
                age_pulses: 3,
                limit_pulses: 2
            }
        );
        assert_eq!(
            verdict(&contact(1, 7, ShipKind::Carrier, 0), &group),
            ContactVerdict::OutOfReach {
                distance_nm: 280,
                reach_nm: n(250)
            }
        );
        group.target_priority = vec![ShipKind::Cruiser];
        assert_eq!(
            verdict(&contact(1, 3, ShipKind::Carrier, 0), &group),
            ContactVerdict::KindNotOnList
        );
        // The first test that fails is the one named: too old, though also
        // of a kind not on the list and out of reach.
        assert_eq!(
            verdict(&contact(1, 7, ShipKind::Carrier, 5), &group),
            ContactVerdict::TooOld {
                age_pulses: 5,
                limit_pulses: 2
            }
        );
        // Then the kind, before the reach.
        assert_eq!(
            verdict(&contact(1, 7, ShipKind::Carrier, 0), &group),
            ContactVerdict::KindNotOnList
        );
    }

    #[test]
    fn the_gates_of_the_group_are_applied_in_order() {
        use crate::refdata::AttackWeapon;
        use crate::time::Pulse::{Morning, Night};
        let here = Some(Hex::new(0, 0));
        let mut g = group();
        let ok = |g: &AirGroup, origin, pulse, weapon, able| {
            strike_group_gate(g, origin, pulse, weapon, || able)
        };
        assert_eq!(
            ok(&g, here, Morning, AttackWeapon::Bomb, 10),
            Ok(StrikeGate {
                origin: Hex::new(0, 0),
                able: 10,
                aircraft: 10
            })
        );
        assert_eq!(
            ok(&g, None, Morning, AttackWeapon::Bomb, 10),
            Err(StrikeStop::NoPosition)
        );
        assert_eq!(
            ok(&g, here, Night, AttackWeapon::Bomb, 10),
            Err(StrikeStop::Dark)
        );
        assert_eq!(
            ok(&g, here, Morning, AttackWeapon::None, 10),
            Err(StrikeStop::NoWeapon)
        );
        assert_eq!(
            ok(&g, here, Morning, AttackWeapon::Bomb, 0),
            Err(StrikeStop::NothingAbleToFly)
        );
        // The held share is applied once, to the able aircraft, cut toward
        // zero: half of 3 is 1, all of 10 is none.
        g.strike_held_share = Fixed::from_ratio(1, 2);
        assert_eq!(
            ok(&g, here, Morning, AttackWeapon::Bomb, 3).map(|gate| gate.aircraft),
            Ok(1)
        );
        g.strike_held_share = Fixed::ONE;
        assert_eq!(
            ok(&g, here, Morning, AttackWeapon::Bomb, 10),
            Err(StrikeStop::AllHeldBack)
        );
        g.strike_held_share = Fixed::ZERO;
        g.ready = 0;
        assert_eq!(
            ok(&g, here, Morning, AttackWeapon::Bomb, 10),
            Err(StrikeStop::NoReadyAircraft)
        );
        g.ready = 4;
        g.mission = AirMission::Search;
        assert_eq!(
            ok(&g, here, Morning, AttackWeapon::Bomb, 10),
            Err(StrikeStop::NotOnStrikeMission)
        );
        // The question of the able aircraft is asked only past the gates
        // before it.
        let mut asked = false;
        let _ = strike_group_gate(&g, None, Morning, AttackWeapon::Bomb, || {
            asked = true;
            1
        });
        assert!(!asked);
    }

    #[test]
    fn the_walk_serves_a_contact_the_groups_before_have_sent_enough_at() {
        let mut figures = AirCombatFigures::from_air_combat_toml();
        figures
            .strike_aircraft_per_ship
            .insert(ShipKind::Carrier, 6);
        let mut group = group();
        group.strike_sizing = true;
        let choice = |number: u32, kind: ShipKind| StrikeChoice {
            kind,
            distance_nm: n(120),
            contact: ContactId::new(number),
            hex: Hex::new(3, 0),
            report_event: EventId::new(7),
        };
        // Two carriers reported: a need of 12. Seven were sent at contact 1.
        let candidates = [
            (choice(1, ShipKind::Carrier), 2),
            (choice(2, ShipKind::Carrier), 2),
        ];
        let mut sent = BTreeMap::new();
        sent.insert(ContactId::new(1), 12);
        sent.insert(ContactId::new(2), 7);
        let walk = strike_walk(&candidates, &group, 10, &figures, &sent);
        // Contact 1 is served, contact 2 still needs 5.
        assert_eq!(walk.served.len(), 1);
        assert_eq!(walk.served[0].0, 0);
        assert_eq!(walk.served[0].1.need, Some(12));
        assert_eq!(walk.served[0].1.sent_before, 12);
        let (index, strike) = walk.taken.expect("it launches at the second");
        assert_eq!(index, 1);
        assert_eq!(strike.aircraft, 5);
        // Every contact served: nothing is taken.
        sent.insert(ContactId::new(2), 12);
        let walk = strike_walk(&candidates, &group, 10, &figures, &sent);
        assert!(walk.taken.is_none());
        assert_eq!(walk.served.len(), 2);
        // The function the walk shares gives the same on one contact.
        assert_eq!(
            sized_strike(candidates[1].0, 2, 10, &figures, &sent),
            Err(StrikeSizing {
                need: Some(12),
                sent_before: 12
            })
        );
        // A group with sizing off takes the first and all it can send.
        group.strike_sizing = false;
        let walk = strike_walk(&candidates, &group, 10, &figures, &sent);
        assert_eq!(walk.taken.map(|(i, s)| (i, s.aircraft)), Some((0, 10)));
        assert!(walk.served.is_empty());
    }
}

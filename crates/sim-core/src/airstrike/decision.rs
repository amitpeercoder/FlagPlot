//! The strike decisions step: which group launches, at what, and with how
//! many (decisions 0026, 0034, 0114 and 0118), and the choices a side's view
//! shares with it (decision 0100).

use super::*;

impl World {
    /// The strike decision step of one daylight pulse (decisions 3 to 6 of
    /// 0026, and 3 of 0034 for a group on the base strike mission, which
    /// decides at [`World::decide_base_strike`]). Each group on the naval strike mission, in group order, that
    /// has aircraft, a position, light and a flight deck that lets it launch
    /// at least one aircraft, takes the contact its side holds that is no
    /// older than the group's strike contact age allows ([`strike_age_limit`]),
    /// within its reach and of a kind on its target list: the kind first on
    /// the list, then the nearest, then the lowest contact number. With no
    /// such contact it launches nothing. It launches the aircraft it can fly
    /// less the share it holds back ([`strike_aircraft`]; decision 0114). A
    /// group that sizes its strikes walks those contacts and launches at the
    /// first not yet served, no more than the contact needs less what the
    /// side's groups before it sent there in the pulse
    /// ([`naval_strike_launch`]; decision 0118).
    ///
    /// The decision reads the group's own side's belief and the group's own
    /// state. It never reads the enemy.
    pub(crate) fn decide_strikes(&mut self, day: u32, pulse: Pulse) {
        debug_assert!(
            self.strikes_in_flight.is_empty(),
            "the last pulse's strikes landed"
        );
        let Some(figures) = self.ref_data.air_combat.clone() else {
            return;
        };
        let groups: Vec<AirGroup> = self
            .truth
            .air_groups
            .values()
            .filter(|group| strike_mission_stop(group).is_none())
            .cloned()
            .collect();
        for group in groups {
            let aircraft_type = &self.ref_data.aircraft_types[&group.aircraft_type];
            // The gates of the group itself, in the order they are applied:
            // a position, light, a weapon, aircraft able to fly (the
            // question of decision 0100, asked only if the gates before it
            // let the group through) and the share the group holds back
            // (decision 0114, 8), applied once, to the whole number the
            // limits gave. A strike of none is not launched and nothing is
            // recorded. The one function: [`strike_group_gate`].
            let Ok(StrikeGate {
                origin,
                able,
                aircraft,
            }) = strike_group_gate(
                &group,
                self.truth.air_group_hex(&group),
                pulse,
                aircraft_type.attack_weapon,
                || self.aircraft_able_to_fly(&group),
            )
            else {
                continue;
            };
            // How far the group may strike: its type's radius on normal
            // reach, more on extended (decision 2 of 0059). The one function.
            let reach_nm = self.ref_data.reach_nm(group.aircraft_type, group.reach);
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "aircraft_able_to_fly",
                        value: Fixed::from_int(i64::from(able)),
                    },
                    Factor {
                        name: "share_held_back",
                        value: group.strike_held_share,
                    },
                ],
                result: Fixed::from_int(i64::from(aircraft)),
            });

            if let AirMission::BaseStrike(base) = group.mission {
                // The base's side and hex are public facts (decision 0034, 2).
                let target = self
                    .truth
                    .bases
                    .get(&base)
                    .map(|target| (target.side, target.hex));
                let launch = base_strike_launch(
                    &group,
                    base,
                    day,
                    pulse,
                    origin,
                    aircraft_type.attack_weapon,
                    aircraft,
                    reach_nm,
                    target,
                );
                self.trace_choice(group.side, || {
                    (
                        Question::Air(AirQuestion::BaseStrike {
                            group: group.id,
                            pulse,
                        }),
                        Answer::Air(AirAnswer::BaseStrike(launch)),
                    )
                });
                if let Some(launch) = launch {
                    self.launch_base_strike(day, pulse, &group, launch, breakdown);
                }
                continue;
            }

            // The one function (decision 6 of 0063, and 0118): the contact
            // this group would strike now and the aircraft it sends there,
            // from its side's belief, its own state and what its side's
            // groups before it have sent in this pulse.
            let sent = sent_at_contacts(&self.strikes_in_flight, group.side);
            let chosen = naval_strike_launch(
                self.picture(group.side, self.command_of_group(group.id)),
                &group,
                origin,
                reach_nm,
                strike_age_limit(&group, &figures),
                aircraft,
                &figures,
                &sent,
            );
            self.trace_choice(group.side, || {
                (
                    Question::Air(AirQuestion::Strike { group: group.id }),
                    Answer::Air(AirAnswer::Strike(chosen)),
                )
            });
            // A sized group whose contacts are all served does not launch
            // and records nothing (decision 0118, 5).
            let Some(NavalStrike {
                choice:
                    StrikeChoice {
                        kind: taken_for,
                        distance_nm,
                        contact,
                        hex,
                        report_event,
                    },
                aircraft,
                sizing,
            }) = chosen
            else {
                continue;
            };
            // A sized launch's breakdown adds the need of the contact (where
            // the kind has a figure), what had been sent at it and, as the
            // result, what the group sent (decision 0118, 10).
            let breakdown = breakdown.map(|mut breakdown| {
                if let Some(StrikeSizing { need, sent_before }) = sizing {
                    if let Some(need) = need {
                        breakdown.factors.push(Factor {
                            name: "need_of_contact",
                            value: Fixed::from_int(i64::from(need)),
                        });
                    }
                    breakdown.factors.push(Factor {
                        name: "sent_before",
                        value: Fixed::from_int(i64::from(sent_before)),
                    });
                    breakdown.result = Fixed::from_int(i64::from(aircraft));
                }
                breakdown
            });

            let causes: Vec<Cause> = group
                .mission_order
                .map(Cause::Order)
                .into_iter()
                .chain(reach_cause(&group))
                .chain(target_cause(&group))
                .chain([Cause::Event(report_event)])
                .chain(group.strike_age_order.map(Cause::Order))
                .chain(group.strike_share_order.map(Cause::Order))
                .chain(group.strike_sizing_order.map(Cause::Order))
                .collect();
            let launched = self.events.record(
                day,
                Some(pulse),
                EventKind::StrikeLaunched {
                    air_group: group.id,
                    contact,
                    hex,
                    aircraft,
                    distance_nm,
                },
                causes,
                breakdown,
            );
            self.strikes_in_flight.push(StrikeInFlight {
                air_group: group.id,
                side: group.side,
                origin: group.location,
                aim: Aim::Contact(contact),
                hex,
                aircraft,
                launched,
                taken_for: Some(taken_for),
            });
            // A force whose group launches a naval strike breaks radio
            // silence (decision 0136, 17).
            self.break_silence_of_group(group.id, day, pulse);
        }
    }

    /// Launches the base strike of one group that [`base_strike_launch`]
    /// said launches (decision 3 of 0034, and 7 of 0063): it records the
    /// launch and the strike in flight, and notes the day of the launch so
    /// the group launches once a game day. It reads no belief and needs no
    /// contact.
    fn launch_base_strike(
        &mut self,
        day: u32,
        pulse: Pulse,
        group: &AirGroup,
        launch: BaseStrikeLaunch,
        breakdown: Option<Breakdown>,
    ) {
        let BaseStrikeLaunch {
            base,
            hex,
            distance_nm,
            aircraft,
        } = launch;
        let causes: Vec<Cause> = group
            .mission_order
            .map(Cause::Order)
            .into_iter()
            .chain(reach_cause(group))
            .chain(group.strike_share_order.map(Cause::Order))
            .collect();
        self.truth
            .air_groups
            .get_mut(&group.id)
            .expect("the group is in the state")
            .last_base_strike_day = Some(day);
        let launched = self.events.record(
            day,
            Some(pulse),
            EventKind::BaseStrikeLaunched {
                air_group: group.id,
                base,
                hex,
                aircraft,
                distance_nm,
            },
            causes,
            breakdown,
        );
        self.strikes_in_flight.push(StrikeInFlight {
            air_group: group.id,
            side: group.side,
            origin: group.location,
            aim: Aim::Base(base),
            hex,
            aircraft,
            launched,
            taken_for: None,
        });
        // A force whose group launches a raid breaks radio silence
        // (decision 0136, 17).
        self.break_silence_of_group(group.id, day, pulse);
    }
}

/// The reach order as a cause of a launch or of escorts sent, when the group
/// is on extended reach (decision 0059): without it the group could not have
/// gone so far. A group on normal reach has no such cause, so nothing in
/// the log of a game with no reach order changes.
pub(super) fn reach_cause(group: &AirGroup) -> Option<Cause> {
    match (group.reach, group.reach_order) {
        (Reach::Extended, Some(order)) => Some(Cause::Order(order)),
        _ => None,
    }
}

/// The target order as a cause of a launch, when the group has one (decision
/// 3 of 0063): a group with no target order cites nothing new, so no log of
/// a game with no target order changes.
fn target_cause(group: &AirGroup) -> Option<Cause> {
    group.target_order.map(Cause::Order)
}

/// The oldest contact, in pulses, `group` launches a naval strike on
/// (decision 0114): its own setting, or the figure of the air combat
/// figures. The strike decision and a side's view both read it.
pub(crate) fn strike_age_limit(group: &AirGroup, figures: &AirCombatFigures) -> u32 {
    group
        .strike_contact_age
        .unwrap_or(figures.strike_contact_max_age_pulses)
}

/// The aircraft a strike launches when the group is able to fly `able` and
/// holds back `held_share` of them (decision 0114, 8): the able aircraft
/// times one less the share, cut to a whole number toward zero. The strike
/// decision and a side's view both call it (decision 0100, 1).
pub(crate) fn strike_aircraft(able: u32, held_share: Fixed) -> u32 {
    let sent = (Fixed::from_int(i64::from(able)) * (Fixed::ONE - held_share)).trunc();
    u32::try_from(sent).expect("a share of the aircraft is not negative")
}

/// The contact a naval strike group would launch at: what
/// [`naval_strike_launch`] chose.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StrikeChoice {
    /// The kind the contact is taken for.
    pub kind: ShipKind,
    /// The contact's air distance from the group, in whole miles.
    pub distance_nm: Fixed,
    pub contact: ContactId,
    /// The hex the contact names.
    pub hex: Hex,
    /// The event that recorded the report the contact rests on.
    pub report_event: EventId,
}

/// What the strike rule makes of one contact of a picture for one group
/// (decision 6 of 0063): the tests of age, kind and reach, in the order they
/// are applied. A contact is one the group could launch at only if it
/// [`ContactVerdict::Qualifies`]; [`contacts_to_strike`] keeps those. A side
/// asking why a group did not strike at a contact reads this verdict, not a
/// second copy of the tests (`view::strike_review`).
pub(crate) fn contact_verdict(
    contact: &Contact,
    group: &AirGroup,
    origin: Hex,
    reach_nm: Fixed,
    max_age_pulses: u32,
) -> ContactVerdict {
    if contact.age_pulses > max_age_pulses {
        return ContactVerdict::TooOld {
            age_pulses: contact.age_pulses,
            limit_pulses: max_age_pulses,
        };
    }
    if !group.target_priority.contains(&contact.estimated_kind) {
        return ContactVerdict::KindNotOnList;
    }
    if !origin.air_distance_within(contact.estimated_hex, reach_nm) {
        return ContactVerdict::OutOfReach {
            distance_nm: origin.air_distance_nm(contact.estimated_hex),
            reach_nm,
        };
    }
    ContactVerdict::Qualifies
}

/// Every contact `group`, at `origin`, could launch at now (decision 6 of
/// 0063), each with the ships it reports, in the order it chooses among
/// them. A contact qualifies if it is no older than `max_age_pulses`, lies
/// within `reach_nm` of `origin` and is taken for a kind on the group's
/// target list ([`contact_verdict`]). They are ordered by the kind that
/// comes first on the list, then the nearest, then the lowest contact
/// number; a group with sizing off takes the first.
///
/// It reads the side's belief and the group's own state and orders, and
/// nothing else: never the enemy.
pub(crate) fn contacts_to_strike(
    belief: &BeliefState,
    group: &AirGroup,
    origin: Hex,
    reach_nm: Fixed,
    max_age_pulses: u32,
) -> Vec<(StrikeChoice, u32)> {
    let mut found: Vec<_> = belief
        .contacts()
        .filter(|contact| {
            contact_verdict(contact, group, origin, reach_nm, max_age_pulses)
                == ContactVerdict::Qualifies
        })
        .collect();
    found.sort_by_key(|contact| {
        (
            group.target_rank(contact.estimated_kind),
            origin.air_distance_nm(contact.estimated_hex),
            contact.id,
        )
    });
    found
        .into_iter()
        .map(|contact| {
            (
                StrikeChoice {
                    kind: contact.estimated_kind,
                    distance_nm: Fixed::from_int(i64::from(
                        origin.air_distance_nm(contact.estimated_hex),
                    )),
                    contact: contact.id,
                    hex: contact.estimated_hex,
                    report_event: contact.last_report_event,
                },
                contact.estimated_ship_count,
            )
        })
        .collect()
}

/// Why a group on neither strike mission, or with nothing ready, is not a
/// strike group this pulse; `None` for a group the strike decision looks at.
/// The strike decision's list of groups and [`strike_group_gate`] both ask it.
pub(crate) fn strike_mission_stop(group: &AirGroup) -> Option<StrikeStop> {
    if !matches!(
        group.mission,
        AirMission::NavalStrike | AirMission::BaseStrike(_)
    ) {
        return Some(StrikeStop::NotOnStrikeMission);
    }
    if group.ready == 0 {
        return Some(StrikeStop::NoReadyAircraft);
    }
    None
}

/// What a group that passed the gates of [`strike_group_gate`] has: where it
/// is, the aircraft it is able to fly and the aircraft a strike of it sends
/// (those less the share held back).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct StrikeGate {
    pub(crate) origin: Hex,
    pub(crate) able: u32,
    pub(crate) aircraft: u32,
}

/// The gates of the group itself that the strike decision applies, in the
/// order it applies them (decisions 3 to 6 of 0026, 0114 and 0034): the
/// strike missions, ready aircraft, a position (`origin`), light in the
/// pulse, a weapon, aircraft able to fly (`able` is called only if the gates
/// before it let the group through: it is the question of decision 0100 and
/// is traced) and a share held back that leaves at least one. The strike
/// decision and a side's review of a pulse both call it; the answer names the
/// first gate that stops the group.
pub(crate) fn strike_group_gate(
    group: &AirGroup,
    origin: Option<Hex>,
    pulse: Pulse,
    weapon: AttackWeapon,
    able: impl FnOnce() -> u32,
) -> Result<StrikeGate, StrikeStop> {
    if let Some(stop) = strike_mission_stop(group) {
        return Err(stop);
    }
    let origin = origin.ok_or(StrikeStop::NoPosition)?;
    if time::is_dark(origin, pulse) {
        return Err(StrikeStop::Dark);
    }
    if weapon == AttackWeapon::None {
        return Err(StrikeStop::NoWeapon);
    }
    let able = able();
    if able == 0 {
        return Err(StrikeStop::NothingAbleToFly);
    }
    let aircraft = strike_aircraft(able, group.strike_held_share);
    if aircraft == 0 {
        return Err(StrikeStop::AllHeldBack);
    }
    Ok(StrikeGate {
        origin,
        able,
        aircraft,
    })
}

/// A naval strike a group would launch (decision 0118): the contact, the
/// aircraft it sends, and, for a group that sizes its strikes, how the size
/// was found.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NavalStrike {
    pub choice: StrikeChoice,
    /// The aircraft sent.
    pub aircraft: u32,
    /// `None` for a group with sizing off, which sends all it can.
    pub sizing: Option<StrikeSizing>,
}

/// How a sized strike's size was found (decision 0118, 2 to 4).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StrikeSizing {
    /// The need of the contact: the ships it reports times the figure for
    /// the kind it reports. `None`: the kind has no figure and needs
    /// everything.
    pub need: Option<u32>,
    /// The aircraft launched at the contact in this pulse by the side's
    /// groups before this one.
    pub sent_before: u32,
}

/// The need of a contact reporting `ships` ships taken for `kind` (decision
/// 0118, 2): the ships times the figure for the kind, `None` where the kind
/// has no figure, which needs everything.
pub(crate) fn strike_need(figures: &AirCombatFigures, kind: ShipKind, ships: u32) -> Option<u32> {
    figures
        .strike_aircraft_per_ship
        .get(&kind)
        .map(|per_ship| per_ship.saturating_mul(ships))
}

/// The aircraft each contact has had launched at it in this pulse by
/// `side`'s groups (decision 0118, 3): the strikes in flight of the side
/// aimed at a contact, sized or not. A base strike aims at no contact.
pub(crate) fn sent_at_contacts(strikes: &[StrikeInFlight], side: Side) -> BTreeMap<ContactId, u32> {
    let mut sent: BTreeMap<ContactId, u32> = BTreeMap::new();
    for strike in strikes.iter().filter(|strike| strike.side == side) {
        if let Aim::Contact(contact) = strike.aim {
            *sent.entry(contact).or_insert(0) += strike.aircraft;
        }
    }
    sent
}

/// The naval strike `group`, at `origin`, launches now when it can send
/// `can_send` aircraft (those it is able to fly less its held share) and its
/// side's groups before it have sent `sent` at each contact (decision 0118,
/// 4 to 7). With sizing off it takes the first contact it could launch on,
/// as [`contacts_to_strike`] orders them, with all it can send. With sizing on it
/// walks those contacts in the same order and takes the first whose need is
/// above what has been sent there, sending the smaller of what it can send
/// and what is still needed; a contact of a kind with no figure is never
/// served and gets all it can send. `None`: it launches nothing, having no
/// contact, or every one served.
///
/// It reads the side's belief, the group's own state and the figures, and
/// nothing of the enemy. The strike decision and the side's view both call
/// it (decision 0100, 1).
#[allow(clippy::too_many_arguments)]
pub(crate) fn naval_strike_launch(
    belief: &BeliefState,
    group: &AirGroup,
    origin: Hex,
    reach_nm: Fixed,
    max_age_pulses: u32,
    can_send: u32,
    figures: &AirCombatFigures,
    sent: &BTreeMap<ContactId, u32>,
) -> Option<NavalStrike> {
    let candidates = contacts_to_strike(belief, group, origin, reach_nm, max_age_pulses);
    strike_walk(&candidates, group, can_send, figures, sent)
        .taken
        .map(|(_, strike)| strike)
}

/// What the walk of [`strike_walk`] found among the candidates, in the order
/// [`contacts_to_strike`] gave them.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct StrikeWalk {
    /// The index of the candidate the group launches at and the strike. `None`:
    /// it launches nothing, having no candidate, or every one served.
    pub(crate) taken: Option<(usize, NavalStrike)>,
    /// The candidates before the one taken (all of them, if none was) that
    /// were served already, by index, with how it was found.
    pub(crate) served: Vec<(usize, StrikeSizing)>,
}

/// Walks `candidates` (as [`contacts_to_strike`] orders them) as the strike
/// decision does (decision 0118, 4 to 7): with sizing off it takes the first
/// with all the group can send; with sizing on it takes the first whose need
/// is above what has been sent there, sending the smaller of what it can send
/// and what is still needed ([`sized_strike`]), and notes each it passed
/// because it was served. The one place the walk is written: the strike
/// question and a side's review of a pulse both read it.
pub(crate) fn strike_walk(
    candidates: &[(StrikeChoice, u32)],
    group: &AirGroup,
    can_send: u32,
    figures: &AirCombatFigures,
    sent: &BTreeMap<ContactId, u32>,
) -> StrikeWalk {
    let mut served = Vec::new();
    for (index, (choice, ships)) in candidates.iter().enumerate() {
        if !group.strike_sizing {
            return StrikeWalk {
                taken: Some((
                    index,
                    NavalStrike {
                        choice: *choice,
                        aircraft: can_send,
                        sizing: None,
                    },
                )),
                served,
            };
        }
        match sized_strike(*choice, *ships, can_send, figures, sent) {
            Ok(strike) => {
                return StrikeWalk {
                    taken: Some((index, strike)),
                    served,
                }
            }
            Err(sizing) => served.push((index, sizing)),
        }
    }
    StrikeWalk {
        taken: None,
        served,
    }
}

/// The strike of a group that sizes its strikes at one contact reporting
/// `ships` ships (decision 0118, 4 to 7), with `sent` already launched at it
/// by the side's groups before: the smaller of `can_send` and what is still
/// needed, or everything for a kind with no figure. `Err`: the contact is
/// served, its need no more than what was sent there.
pub(crate) fn sized_strike(
    choice: StrikeChoice,
    ships: u32,
    can_send: u32,
    figures: &AirCombatFigures,
    sent: &BTreeMap<ContactId, u32>,
) -> Result<NavalStrike, StrikeSizing> {
    let sent_before = sent.get(&choice.contact).copied().unwrap_or(0);
    let need = strike_need(figures, choice.kind, ships);
    let aircraft = match need {
        None => can_send,
        Some(need) if need > sent_before => can_send.min(need - sent_before),
        Some(_) => return Err(StrikeSizing { need, sent_before }),
    };
    Ok(NavalStrike {
        choice,
        aircraft,
        sizing: Some(StrikeSizing { need, sent_before }),
    })
}

/// Whether `group`, on the base strike mission against `base`, launches in
/// `pulse` of game day `day` (decision 3 of 0034, and 7 of 0063), and what it
/// launches. It launches every aircraft it can fly (`aircraft`, the answer of
/// [`able_to_fly`]) in a daylight pulse, if its type carries a weapon, it has
/// not launched at a base that game day, the base exists, is held by the other
/// side and is within `reach_nm` of `origin`. So it flies in the first
/// daylight pulse in which the base is in reach, once a day.
///
/// `target` is the base's side and hex, `None` if it does not exist or the
/// side does not know of it: both are public facts, which every side has
/// (decision 0034, 2). The function reads no belief, needs no contact and
/// nothing else of the base. The strike decision and a side's view both call
/// it (decision 0100, 1).
#[allow(clippy::too_many_arguments)]
pub(crate) fn base_strike_launch(
    group: &AirGroup,
    base: BaseId,
    day: u32,
    pulse: Pulse,
    origin: Hex,
    weapon: AttackWeapon,
    aircraft: u32,
    reach_nm: Fixed,
    target: Option<(Side, Hex)>,
) -> Option<BaseStrikeLaunch> {
    if time::is_dark(origin, pulse) || weapon == AttackWeapon::None || aircraft == 0 {
        return None;
    }
    // Once in a game day, in the first daylight pulse the base is in reach
    // (decision 7 of 0063).
    if group.last_base_strike_day == Some(day) {
        return None;
    }
    let (side, hex) = target?;
    if side == group.side {
        return None;
    }
    let distance_nm = Fixed::from_int(i64::from(origin.air_distance_nm(hex)));
    if !origin.air_distance_within(hex, reach_nm) {
        return None;
    }
    Some(BaseStrikeLaunch {
        base,
        hex,
        distance_nm,
        aircraft,
    })
}

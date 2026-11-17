//! Reports between commands (decision 0135, step 2 of stage 2b).
//!
//! With command levels on and report figures loaded, a report is held first
//! by the command that saw the thing ([`World::apply_report`] with
//! [`Holders::Commands`]) and passed on from there: up to its superior,
//! sideways to every command of the side within its radio reach, and, once
//! the area command first holds it, down to the commands under the area
//! command (items 3 to 7). Each passing, to each listener, is drawn: it
//! arrives in the pulse it is sent, a pulse later, or never (item 8). A late
//! report is kept in [`ReportInFlight`] and applied at the start of the next
//! pulse, before anything of that pulse is seen (item 12). A command hears a
//! report once (item 13); each command remembers the reports it has heard
//! for as long as a contact could be kept ([`CommandPost::heard`]).
//!
//! A passing is an event, [`EventKind::ReportSent`] (item 21), caused by the
//! report's own event. With no report figures none of this happens and every
//! report reaches every command at once, as decision 0134 has it.

use std::collections::BTreeSet;

use crate::belief::{Report, SightingReport};
use crate::command::Addressee;
use crate::events::{Arrival, Breakdown, Cause, Direction, EventKind, Factor, ReportListener};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EntityRef, EventId, TaskForceId};
use crate::pictures::{self, by_hex, CommandRef, Holders};
use crate::refdata::{PassingChances, ReportFigures};
use crate::rng::{Draw, Subsystem};
use crate::state::Side;
use crate::time::Pulse;
use crate::world::World;

/// A report on its way to a listener, to arrive in a later pulse (decision
/// 0135, 12). Hashed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReportInFlight {
    pub(crate) side: Side,
    /// The report as the listener will get it: "nothing there" by hex.
    pub(crate) report: Report,
    /// The report's event: what a command's heard set is keyed on.
    pub(crate) key: EventId,
    /// The day and pulse the report was made.
    pub(crate) made: (u32, Pulse),
    pub(crate) sender: CommandRef,
    pub(crate) direction: Direction,
    pub(crate) listener: CommandRef,
    /// The "report sent" event that sent it.
    pub(crate) sent_event: EventId,
    /// The day and pulse it arrives.
    pub(crate) arrives: (u32, Pulse),
}

/// A report a silent force made and has not sent (decision 0136, 17): what
/// sending it later needs. Hashed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnsentReport {
    /// The report as it is passed: "nothing there" by hex.
    pub(crate) report: Report,
    /// The report's event.
    pub(crate) key: EventId,
    /// The day and pulse it was made; `None` for a report of an end-of-day
    /// phase, counted as made in the afternoon.
    pub(crate) day: u32,
    pub(crate) pulse: Option<Pulse>,
}

/// A moment as a number of pulses from day 1's night.
pub(crate) fn ordinal(day: u32, pulse: Pulse) -> u64 {
    u64::from(day) * 3 + pulse.code() - 1
}

/// The pulse after `(day, pulse)`.
pub(crate) fn next_pulse(day: u32, pulse: Pulse) -> (u32, Pulse) {
    match pulse {
        Pulse::Night => (day, Pulse::Morning),
        Pulse::Morning => (day, Pulse::Afternoon),
        Pulse::Afternoon => (day + 1, Pulse::Night),
    }
}

/// The hex a report is about, for the area limit of an intent (item 5).
fn report_hex(report: &Report) -> Hex {
    match report {
        Report::Sighting(sighting) => sighting.estimated_hex,
        Report::BattleClaim(claim) => claim.hex,
        Report::NothingThere { hex, .. } => *hex,
        Report::Ground(ground) => ground.hex,
        Report::NoneFound(none) => none.hex,
    }
}

/// The report as a picture gets it `pulses` pulses after it was made: a
/// sighting is as old as it is (item 14).
fn aged(report: &Report, pulses: u64) -> Report {
    match report {
        Report::Sighting(sighting) => Report::Sighting(SightingReport {
            age_pulses: sighting
                .age_pulses
                .saturating_add(u32::try_from(pulses).unwrap_or(u32::MAX)),
            ..sighting.clone()
        }),
        other => other.clone(),
    }
}

/// What a draw of one listener gave: the arrival and the chances it was
/// compared with.
pub(crate) fn arrival_of(chances: PassingChances, roll: i64) -> Arrival {
    if roll < chances.on_time.raw() {
        Arrival::OnTime
    } else if roll < (chances.on_time + chances.late).raw() {
        Arrival::Late
    } else {
        Arrival::Never
    }
}

impl World {
    /// Passes a report on from the commands that hold it first (decision
    /// 0135, 3 to 7), after the caller has recorded the report's event. A
    /// no-op unless command levels are on, report figures are loaded and
    /// `holders` names commands.
    pub(crate) fn pass_report(&mut self, side: Side, report: &Report, holders: &Holders) {
        let Holders::Commands {
            commands,
            key,
            day,
            pulse,
        } = holders
        else {
            return;
        };
        if !self.reports_travel() {
            return;
        }
        let (key, day, pulse) = (*key, *day, *pulse);
        let made = (day, pulse.unwrap_or(Pulse::Afternoon));
        let now = (day, pulse);
        let report = by_hex(report);
        let held = self.resolve_holders(side, commands);
        for sender in held {
            if sender == CommandRef::Area {
                // The holder is the area command itself: it sends down.
                self.area_sends_down(side, &report, key, made, now, None, None);
                continue;
            }
            // A silent force sends nothing of its own until it breaks
            // silence (decision 0136, 16): the report is kept.
            if let CommandRef::TaskForce(force) = sender {
                if self.radio_silent.contains(&force) && !self.broke_silence.contains(&force) {
                    self.unsent_reports
                        .entry(force)
                        .or_default()
                        .push(UnsentReport {
                            report: report.clone(),
                            key,
                            day,
                            pulse,
                        });
                    continue;
                }
            }
            self.send_own_report(side, sender, &report, key, made, now);
        }
    }

    /// A silent force breaks silence in this pulse (decision 0136, 17): the
    /// first call of the pulse marks it, so that its state report is made
    /// and the reports it makes from now are sent as made, and sends at once
    /// every report it kept that is not yet as old as a contact may be,
    /// oldest first (by when it was made, then its event), as of this pulse,
    /// each as old as it is. A no-op unless reports travel and the force is
    /// silent, and after the first call of a pulse.
    pub(crate) fn break_silence(&mut self, force: TaskForceId, day: u32, pulse: Pulse) {
        if !self.reports_travel() || !self.radio_silent.contains(&force) {
            return;
        }
        if !self.broke_silence.insert(force) {
            return;
        }
        let Some(kept) = self.unsent_reports.remove(&force) else {
            return;
        };
        let Some(side) = self.truth.task_forces.get(&force).map(|f| f.side) else {
            return;
        };
        let drop_age = self
            .ref_data
            .search
            .as_ref()
            .map_or(0, |figures| u64::from(figures.contact_drop_age_pulses));
        let now = ordinal(day, pulse);
        let mut kept: Vec<UnsentReport> = kept
            .into_iter()
            .filter(|unsent| {
                let made = ordinal(unsent.day, unsent.pulse.unwrap_or(Pulse::Afternoon));
                now.saturating_sub(made) < drop_age
            })
            .collect();
        kept.sort_by_key(|unsent| {
            (
                ordinal(unsent.day, unsent.pulse.unwrap_or(Pulse::Afternoon)),
                unsent.key,
            )
        });
        for unsent in kept {
            let made = (unsent.day, unsent.pulse.unwrap_or(Pulse::Afternoon));
            self.send_own_report(
                side,
                CommandRef::TaskForce(force),
                &unsent.report,
                unsent.key,
                made,
                (day, Some(pulse)),
            );
        }
    }

    /// [`World::break_silence`] for the force of an air group's command: a
    /// group aboard a silent force's ship launching a strike.
    pub(crate) fn break_silence_of_group(
        &mut self,
        group: crate::ids::AirGroupId,
        day: u32,
        pulse: Pulse,
    ) {
        if let Some(CommandRef::TaskForce(force)) = pictures::command_of_group(&self.truth, group) {
            self.break_silence(force, day, pulse);
        }
    }

    /// A command sends a report of its own: up to its superior, sideways to
    /// every command within its radio reach and, once the area command first
    /// holds it, down. The one place a command's own report is sent, now or,
    /// for a silent force's, later (decision 0136, 17). `made` is when the
    /// report was made and `now` the moment of sending.
    pub(crate) fn send_own_report(
        &mut self,
        side: Side,
        sender: CommandRef,
        report: &Report,
        key: EventId,
        made: (u32, Pulse),
        now: (u32, Option<Pulse>),
    ) {
        let Some(superior) = self.chain.as_ref().expect("on")[&side]
            .get(&sender)
            .and_then(|post| post.superior())
        else {
            return;
        };
        let mut listeners = vec![(superior, Direction::Up)];
        listeners.extend(
            self.within_reach(side, sender)
                .into_iter()
                .map(|command| (command, Direction::Sideways)),
        );
        let sent = self.send(side, report, key, made, now, sender, &listeners, None);
        if sent.area_first_held {
            self.area_sends_down(side, report, key, made, now, Some(sender), Some(sent.event));
        }
    }

    /// Every command of `side` but `sender` and the area command whose place
    /// is within the radio reach of `sender`, in command order (item 4).
    /// The area command has no place and so no reach.
    fn within_reach(&self, side: Side, sender: CommandRef) -> Vec<CommandRef> {
        let Some(figures) = self.ref_data.reports else {
            return Vec::new();
        };
        let Some((from, reach)) = self.place_and_reach(&figures, sender) else {
            return Vec::new();
        };
        self.chain.as_ref().expect("on")[&side]
            .keys()
            .copied()
            .filter(|command| *command != sender && *command != CommandRef::Area)
            .filter(|command| {
                self.place_of(*command)
                    .is_some_and(|place| from.air_distance_within(place, reach))
            })
            .collect()
    }

    /// Where a command is: a task force's hex, a base's hex; none for the
    /// area command.
    fn place_of(&self, command: CommandRef) -> Option<Hex> {
        match command {
            CommandRef::Area => None,
            CommandRef::TaskForce(force) => {
                self.truth.task_forces.get(&force).map(|f| f.position())
            }
            CommandRef::Base(base) => self.truth.bases.get(&base).map(|b| b.hex),
        }
    }

    /// Where a command is and how far its radio reaches (item 7): a task
    /// force's is its longest-ranged ship's, a base's its own.
    fn place_and_reach(
        &self,
        figures: &ReportFigures,
        command: CommandRef,
    ) -> Option<(Hex, Fixed)> {
        let place = self.place_of(command)?;
        let reach = match command {
            CommandRef::Area => return None,
            CommandRef::Base(_) => figures.reach_base_nm,
            CommandRef::TaskForce(force) => self.truth.task_forces[&force]
                .ships
                .iter()
                .filter_map(|ship| self.truth.ships.get(ship))
                .filter_map(|ship| self.ref_data.ship_classes.get(&ship.class))
                .map(|class| figures.reach_of_kind(class.kind))
                .max()
                .unwrap_or(Fixed::ZERO),
        };
        Some((place, reach))
    }

    /// The area command sends a report it first holds down to the commands
    /// under it (item 5), but the one that sent it to the area command
    /// (`excluded`), and those the report does not bear on. `brought_by` is
    /// the "report sent" event that brought the report to it, a cause of
    /// this one.
    #[allow(clippy::too_many_arguments)]
    fn area_sends_down(
        &mut self,
        side: Side,
        report: &Report,
        key: EventId,
        made: (u32, Pulse),
        now: (u32, Option<Pulse>),
        excluded: Option<CommandRef>,
        brought_by: Option<EventId>,
    ) {
        let hex = report_hex(report);
        let under: Vec<CommandRef> = self.chain.as_ref().expect("on")[&side]
            .iter()
            .filter(|(command, post)| {
                **command != CommandRef::Area
                    && post.superior() == Some(CommandRef::Area)
                    && Some(**command) != excluded
            })
            .map(|(command, _)| *command)
            .filter(|command| self.report_bears_on(side, *command, hex))
            .collect();
        let listeners: Vec<(CommandRef, Direction)> = under
            .into_iter()
            .map(|command| (command, Direction::Down))
            .collect();
        self.send(
            side,
            report,
            key,
            made,
            now,
            CommandRef::Area,
            &listeners,
            brought_by,
        );
    }

    /// Whether a report about `hex` bears on `command` (item 5): any report,
    /// if no intent in force is addressed to it or one of them has no area
    /// limit; else only one inside the union of those intents' areas, each
    /// read up the tree.
    fn report_bears_on(&self, side: Side, command: CommandRef, hex: Hex) -> bool {
        let state = self.command_state(side);
        let addressed = |addressee: Addressee| match (addressee, command) {
            (Addressee::TaskForce(force), CommandRef::TaskForce(this)) => force == this,
            (Addressee::Base(base), CommandRef::Base(this)) => base == this,
            (Addressee::AirGroup(group), command) => {
                crate::pictures::command_of_group(&self.truth, group) == Some(command)
            }
            _ => false,
        };
        let mine: Vec<_> = state
            .intents_in_force()
            .filter(|intent| addressed(intent.addressee))
            .map(|intent| intent.id)
            .collect();
        if mine.is_empty() {
            return true;
        }
        let mut union: BTreeSet<Hex> = BTreeSet::new();
        for intent in mine {
            match state.area_read_up(intent) {
                None => return true,
                Some(area) => union.extend(area),
            }
        }
        union.contains(&hex)
    }

    /// Sends `report` from `sender` to `listeners` in order, each drawn:
    /// records the "report sent" event (nothing with no listener) and
    /// applies it, now or a pulse later, to each that has not heard it.
    /// `made` is when the report was made and `now` the moment of sending.
    #[allow(clippy::too_many_arguments)]
    fn send(
        &mut self,
        side: Side,
        report: &Report,
        key: EventId,
        made: (u32, Pulse),
        now: (u32, Option<Pulse>),
        sender: CommandRef,
        listeners: &[(CommandRef, Direction)],
        brought_by: Option<EventId>,
    ) -> Sent {
        let nothing = Sent {
            event: key,
            area_first_held: false,
        };
        let Some(figures) = self.ref_data.reports else {
            return nothing;
        };
        if listeners.is_empty() {
            return nothing;
        }
        let (day, pulse) = now;
        // The pulse a late report is counted from.
        let counted = pulse.unwrap_or(Pulse::Afternoon);
        let mut sent: Vec<ReportListener> = Vec::new();
        let mut factors: Vec<Factor> = Vec::new();
        let mut arriving = Fixed::ZERO;
        for (command, direction) in listeners {
            let chances = match direction {
                Direction::Up => figures.up,
                Direction::Sideways => figures.sideways,
                Direction::Down => figures.down,
            };
            // A report sent down draws on a stream of its own, so that a
            // second road is a second chance (decision 0135, 11).
            let draw = match direction {
                Direction::Down => Draw::ReportPassedDown,
                Direction::Up | Direction::Sideways => Draw::ReportPassed,
            };
            let mut stream = match command {
                CommandRef::Area => self.rng(
                    day,
                    pulse,
                    Subsystem::Intelligence,
                    draw,
                    EntityRef::Event(key),
                ),
                CommandRef::TaskForce(force) => self.rng_pair(
                    day,
                    pulse,
                    Subsystem::Intelligence,
                    draw,
                    EntityRef::Event(key),
                    EntityRef::TaskForce(*force),
                ),
                CommandRef::Base(base) => self.rng_pair(
                    day,
                    pulse,
                    Subsystem::Intelligence,
                    draw,
                    EntityRef::Event(key),
                    EntityRef::Base(*base),
                ),
            };
            // One number is always taken, whatever the chances.
            let roll = i64::try_from(stream.below(Fixed::SCALE.unsigned_abs()))
                .expect("a roll fits in i64");
            let arrival = arrival_of(chances, roll);
            if arrival != Arrival::Never {
                arriving += Fixed::ONE;
            }
            factors.push(Factor {
                name: "on_time_chance",
                value: chances.on_time,
            });
            factors.push(Factor {
                name: "late_chance",
                value: chances.late,
            });
            // Named "draw", as a raw die is everywhere: a side is never given it.
            factors.push(Factor {
                name: "draw",
                value: Fixed::from_raw(roll),
            });
            sent.push(ReportListener {
                command: *command,
                direction: *direction,
                arrival,
            });
        }
        let breakdown = self.config.record_breakdowns.then_some(Breakdown {
            factors,
            result: arriving,
        });
        let causes: Vec<Cause> = std::iter::once(Cause::Event(key))
            .chain(brought_by.map(Cause::Event))
            .collect();
        let event = self.events.record(
            day,
            pulse,
            EventKind::ReportSent {
                side,
                sender,
                report: key,
                listeners: sent.clone(),
            },
            causes,
            breakdown,
        );
        let mut area_first_held = false;
        for listener in &sent {
            match listener.arrival {
                Arrival::Never => {}
                Arrival::OnTime => {
                    if self.hear(side, listener.command, report, key, made, (day, counted)) {
                        area_first_held |= listener.command == CommandRef::Area;
                    }
                }
                Arrival::Late => {
                    let arrives = next_pulse(day, counted);
                    self.reports_in_flight.push(ReportInFlight {
                        side,
                        report: report.clone(),
                        key,
                        made,
                        sender,
                        direction: listener.direction,
                        listener: listener.command,
                        sent_event: event,
                        arrives,
                    });
                }
            }
        }
        Sent {
            event,
            area_first_held,
        }
    }

    /// Applies `report`, made at `made`, to `command`'s picture at `now`, if
    /// the command is one and has not heard it (item 13). A sighting is as
    /// old as the pulses since it was made (item 14). Returns whether it was
    /// applied.
    fn hear(
        &mut self,
        side: Side,
        command: CommandRef,
        report: &Report,
        key: EventId,
        made: (u32, Pulse),
        now: (u32, Pulse),
    ) -> bool {
        let Some(figures) = self.ref_data.search.clone() else {
            return false;
        };
        let age = ordinal(now.0, now.1).saturating_sub(ordinal(made.0, made.1));
        let report = aged(report, age);
        let Some(post) = self
            .chain
            .as_mut()
            .and_then(|chain| chain.get_mut(&side))
            .and_then(|posts| posts.get_mut(&command))
        else {
            return false;
        };
        if post.heard.contains_key(&key) {
            return false;
        }
        // A report sent to a command is one it was told of (decision 0137,
        // 5).
        post.apply(&report, &figures, false);
        post.heard.insert(key, made);
        true
    }

    /// Applies the reports in flight that arrive in this pulse, at its start,
    /// before anything of it is seen (item 12), in the order of the reports'
    /// events and then of the listeners. A listener that is no longer a
    /// command of the side is dropped; one that has heard the report hears
    /// nothing new. The area command, hearing a report for the first time,
    /// sends it down now.
    pub(crate) fn deliver_late_reports(&mut self, day: u32, pulse: Pulse) {
        if self.reports_in_flight.is_empty() {
            return;
        }
        let (due, later): (Vec<ReportInFlight>, Vec<ReportInFlight>) =
            std::mem::take(&mut self.reports_in_flight)
                .into_iter()
                .partition(|flight| flight.arrives == (day, pulse));
        self.reports_in_flight = later;
        let mut due = due;
        due.sort_by_key(|flight| (flight.key, flight.listener));
        for flight in due {
            if !self.is_command(flight.side, flight.listener) {
                continue;
            }
            let applied = self.hear(
                flight.side,
                flight.listener,
                &flight.report,
                flight.key,
                flight.made,
                (day, pulse),
            );
            if applied && flight.listener == CommandRef::Area {
                self.area_sends_down(
                    flight.side,
                    &flight.report,
                    flight.key,
                    flight.made,
                    (day, Some(pulse)),
                    Some(flight.sender),
                    Some(flight.sent_event),
                );
            }
        }
    }

    /// Forgets the reports each command heard longer ago than a contact may
    /// be kept (item 5 of the build): at the end of a pulse, after the
    /// ageing.
    pub(crate) fn prune_heard(&mut self, day: u32, pulse: Pulse, drop_age_pulses: u32) {
        let Some(chain) = self.chain.as_mut() else {
            return;
        };
        let now = ordinal(day, pulse);
        for posts in chain.values_mut() {
            for post in posts.values_mut() {
                post.heard.retain(|_, (made_day, made_pulse)| {
                    now.saturating_sub(ordinal(*made_day, *made_pulse)) < u64::from(drop_age_pulses)
                });
            }
        }
        // A silent force's unsent reports older than a contact may be are
        // dropped with them (decision 0136, 17).
        for reports in self.unsent_reports.values_mut() {
            reports.retain(|unsent| {
                let made = ordinal(unsent.day, unsent.pulse.unwrap_or(Pulse::Afternoon));
                now.saturating_sub(made) < u64::from(drop_age_pulses)
            });
        }
        self.unsent_reports.retain(|_, reports| !reports.is_empty());
    }
}

/// What [`World::send`] did: the event it recorded and whether the area
/// command heard the report for the first time.
struct Sent {
    event: EventId,
    area_first_held: bool,
}

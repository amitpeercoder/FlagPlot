//! The briefing as text, made from the briefing's data and from nothing else
//! (decision 0048, 5).
//!
//! Fixed headings in capitals, one fact per line starting with `- `. A part
//! with nothing in it says `none`. Units read `task_force:5 "Name"`, places
//! `(q,r)` followed by the name of the side's own base there. Numbers are
//! whole or exact decimals. Nothing is advised and nothing is guessed. The
//! fixed words contain no digits, so every number in the text comes from the
//! data.

use crate::briefing::{BaseChangesPart, Briefing, EnemyAshorePart, LandUnitsPart, StrikeReport};
use crate::common::{Handle, Place};
use crate::objectives::{ObjectiveLine, ResultPart, ShipKindPoints};

fn plain(word: &str) -> String {
    word.replace('_', " ")
}

fn plural(n: u32, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

fn cap(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn ships(n: u32) -> String {
    if n == 1 {
        "1 ship".to_string()
    } else {
        format!("{n} ships")
    }
}

fn ev(id: u32) -> String {
    format!(" [event {id}]")
}

fn evs(ids: &[u32]) -> String {
    match ids {
        [] => String::new(),
        [one] => ev(*one),
        many => {
            let list: Vec<String> = many.iter().map(ToString::to_string).collect();
            format!(" [events {}]", list.join(", "))
        }
    }
}

fn names(handles: &[Handle]) -> String {
    let list: Vec<String> = handles.iter().map(Handle::text).collect();
    list.join(", ")
}

struct Out(String);

impl Out {
    fn heading(&mut self, heading: &str) {
        if !self.0.is_empty() {
            self.0.push('\n');
        }
        self.0.push_str(heading);
        self.0.push('\n');
    }

    /// A line under the one before it, indented two spaces.
    fn sub(&mut self, text: String) {
        self.0.push_str("  - ");
        self.0.push_str(&text);
        self.0.push('\n');
    }

    /// A line under a `sub` line.
    fn sub_sub(&mut self, text: String) {
        self.0.push_str("    - ");
        self.0.push_str(&text);
        self.0.push('\n');
    }

    fn line(&mut self, text: String) {
        self.0.push_str("- ");
        self.0.push_str(&text);
        self.0.push('\n');
    }

    /// One line per item, or `label: none`.
    fn each<T>(&mut self, label: &str, items: &[T], f: impl Fn(&T) -> String) {
        if items.is_empty() {
            self.line(format!("{label}: none"));
        }
        for item in items {
            self.line(f(item));
        }
    }
}

fn at(place: &Place) -> String {
    place.text()
}

impl Briefing {
    /// The briefing as plain text, made from this data and nothing else.
    pub fn to_text(&self) -> String {
        let mut o = Out(String::new());
        o.0.push_str("FLAG PLOT BRIEFING\n");
        let rules = self.heading.rules_in_play;

        o.heading("HEADING");
        let h = &self.heading;
        o.line(format!("Side: {}", h.side));
        match h.last_day {
            Some(last) if h.day > last => o.line(format!(
                "Day: {} (the game is over: day {} was the last)",
                h.day, last
            )),
            Some(last) => {
                o.line(format!("Day: {} of {}", h.day, last));
                if h.day == last {
                    o.line("This is the last day: the game ends when you end it.".to_string());
                }
            }
            None => o.line(format!("Day: {}", h.day)),
        }
        o.line(format!("Scenario: {}", h.scenario));
        match h.yesterday {
            Some(day) => o.line(format!(
                "Covers: day {day} and the state at the start of day {}",
                h.day
            )),
            None => o.line("Covers: the state only, no day has been resolved".to_string()),
        }

        o.heading("OBJECTIVES");
        let ob = &self.objectives;
        match &ob.mission {
            None => o.line("none: this scenario gives no mission and no objectives".to_string()),
            Some(mission) => {
                o.line(format!("Mission: {mission}"));
                for line in &ob.objectives {
                    objective_text(&mut o, line);
                }
                if let Some(l) = &ob.levels {
                    o.line(format!(
                        "Levels: the result is read from the margin, your points less the enemy's. A margin of {} or more is a minor victory, {} or more a major one, {} or more a decisive one; the same margins against you are defeats; less than {} either way is a stalemate",
                        l.minor.0, l.major.0, l.decisive.0, l.minor.0
                    ));
                }
            }
        }
        if let Some(result) = &ob.result {
            result_text(&mut o, result);
        }

        o.heading("ORDERS");
        o.line(format!("Accepted yesterday: {}", self.orders.accepted));
        o.each("Refused yesterday", &self.orders.refused, |r| {
            format!(
                "Refused: order:{} for {}: {}{}",
                r.order,
                r.target.text(),
                r.reason,
                ev(r.event)
            )
        });

        o.heading("CONTACTS");
        let c = &self.contacts;
        o.each("New contacts", &c.new, |x| {
            format!(
                "New contact: {}, {} estimated, taken for {}, at {}{}",
                x.contact.text(),
                ships(x.ship_count),
                x.taken_for,
                at(&x.place),
                ev(x.event)
            )
        });
        o.each("Refreshed contacts", &c.refreshed, |x| {
            let mut s = format!(
                "Refreshed contact: {}, {} estimated",
                x.contact.text(),
                ships(x.ship_count)
            );
            if let Some(n) = x.was_ship_count {
                s.push_str(&format!(" (was {n})"));
            }
            s.push_str(&format!(", taken for {}", x.taken_for));
            if let Some(k) = x.was_taken_for {
                s.push_str(&format!(" (was {k})"));
            }
            s.push_str(&format!(", at {}", at(&x.place)));
            if let Some(p) = &x.was_place {
                s.push_str(&format!(" (was {})", p.coords()));
            }
            s.push_str(&ev(x.event));
            s
        });
        o.each("Dropped contacts", &c.dropped, |x| {
            format!(
                "Dropped contact: {}, age {}{}",
                x.contact.text(),
                plural(x.age_pulses, "pulse", "pulses"),
                ev(x.event)
            )
        });
        o.each("Contacts found not there", &c.not_there, |x| {
            format!(
                "Not there: {} at {}{}",
                x.contact.text(),
                at(&x.place),
                ev(x.event)
            )
        });
        o.each("Contacts held", &c.held, |x| {
            format!(
                "Held contact: {} at {}, {} estimated, taken for {}, age {}",
                x.contact.text(),
                at(&x.place),
                ships(x.ship_count),
                x.taken_for,
                plural(x.age_pulses, "pulse", "pulses")
            )
        });

        if let Some(ashore) = &self.enemy_ashore {
            enemy_ashore_text(&mut o, ashore);
        }

        o.heading("ACTION YESTERDAY");
        let a = &self.action;
        o.each("Fights at sea", &a.fights, |x| {
            format!(
                "Fight at {}: {}, ended {} in round {}{}",
                at(&x.place),
                x.force.text(),
                plain(x.ended),
                x.rounds,
                ev(x.event)
            )
        });
        o.each("Salvoes fired at sea", &a.fired, |x| {
            format!(
                "Gunfire: {} {}: {}, {} shots",
                x.firer.text(),
                x.weapon,
                plural(x.salvos, "salvo", "salvos"),
                x.shots.0
            )
        });
        o.each("Own fighters' fire", &a.air_fire, |x| {
            let who = if x.firing == "patrol" {
                "Patrol fire"
            } else {
                "Escort fire"
            };
            format!(
                "{who}: {} at the enemy's {}, {} shots{}",
                x.firer.text(),
                x.at,
                x.shots.0,
                ev(x.event)
            )
        });
        o.each("Own anti-aircraft fire", &a.guns, |x| {
            format!(
                "Anti-aircraft fire: {} at a raid, {} shots{}",
                x.by.text(),
                x.shots.0,
                ev(x.event)
            )
        });
        if a.strikes.is_empty() {
            o.line("Strikes and attacks flown: none".to_string());
        }
        for x in &a.strikes {
            let mut s = match x.kind {
                "air_attack" => format!("Air attack: {}", x.air_group.text()),
                "base_attack" => format!("Base attack: {}", x.air_group.text()),
                other => format!("{}: {}", cap(&plain(other)), x.air_group.text()),
            };
            if let Some(t) = &x.target {
                let word = if x.kind == "air_attack" || x.kind == "base_attack" {
                    "on"
                } else {
                    "at"
                };
                s.push_str(&format!(" {word} {}", t.text()));
            }
            if let Some(p) = &x.place {
                s.push_str(&format!(", place {}", at(p)));
            }
            if let Some(n) = x.aircraft {
                s.push_str(&format!(", {} aircraft", n.0));
            }
            if let Some(w) = x.weapon {
                s.push_str(&format!(", {w}"));
            }
            if let Some(d) = x.distance_nm {
                s.push_str(&format!(", {} miles", d.0));
            }
            s.push_str(&ev(x.event));
            o.line(s);
            if x.kind == "air_attack" || x.kind == "strike_found_nothing" {
                if x.reports.is_empty() {
                    o.sub("Report: none".to_string());
                }
                for r in &x.reports {
                    o.sub(report_text(r));
                }
            }
        }
        o.each("Raids on bases", &a.raids_on_bases, |x| {
            // Men are named only when a hit landed on them (decision 0092).
            let men = if x.hits_on_men > 0 {
                format!("{} on men, ", x.hits_on_men)
            } else {
                String::new()
            };
            format!(
                "Raid on {}: {} aircraft attacked, {} {}, {} on aircraft, {} on supply, {men}{} on nothing{}",
                x.base.text(),
                x.aircraft.0,
                x.hits,
                if x.hits == 1 { "hit" } else { "hits" },
                x.hits_on_aircraft,
                x.hits_on_supply,
                x.hits - x.hits_on_aircraft - x.hits_on_supply - x.hits_on_men,
                ev(x.event)
            )
        });
        // Shore bombardment (decision 0092): only when there is one.
        for x in &a.bombardments {
            o.line(format!(
                "Shore bombardment by {} of {} at {}: {} shells, {} weighed shots; the hits are not reported to you, only the claim{}",
                x.force.text(),
                x.base.text(),
                at(&x.place),
                x.shells.0,
                x.weighed_shots.0,
                ev(x.event)
            ));
        }
        for x in &a.shell_fire {
            o.line(format!(
                "Shell fire: {} fired {} shells ({} weighed shots) at {}, {} rounds left{}",
                x.ship.text(),
                x.shells.0,
                x.weighed_shots.0,
                x.base.text(),
                x.rounds_left.0,
                ev(x.event)
            ));
        }
        for x in &a.bombardments_against {
            o.line(format!(
                "Bombarded from the sea: {} at {}: {} shells, {} weighed shots, {} {}, {} on aircraft, {} on supply, {} on men, {} on nothing; the ships are not known{}",
                x.base.text(),
                at(&x.place),
                x.shells.0,
                x.weighed_shots.0,
                x.hits,
                if x.hits == 1 { "hit" } else { "hits" },
                x.hits_on_aircraft,
                x.hits_on_supply,
                x.hits_on_men,
                x.hits - x.hits_on_aircraft - x.hits_on_supply - x.hits_on_men,
                ev(x.event)
            ));
        }
        o.each("Attacks on ships", &a.attacks_on_ships, |x| {
            match x.aircraft {
                Some(n) => {
                    let mut s = format!(
                        "Attack on {} from the air: {} aircraft",
                        x.against.text(),
                        n.0
                    );
                    if x.attacks > 1 {
                        s.push_str(&format!(", {}", plural(x.attacks, "attack", "attacks")));
                    }
                    s.push_str(&evs(&x.events));
                    s
                }
                None => format!(
                    "Fired on: enemy {} at {}: {}",
                    x.weapon.unwrap_or("guns"),
                    x.against.text(),
                    plural(x.attacks, "salvo", "salvos")
                ),
            }
        });

        o.heading("LOSSES YESTERDAY");
        let l = &self.losses;
        o.each("Ships sunk", &l.ships_sunk, |x| {
            format!(
                "Ship sunk: {} of {}{}",
                x.ship.text(),
                x.task_force.text(),
                ev(x.event)
            )
        });
        o.each("Ships damaged", &l.ships_damaged, |x| {
            let mut s = format!(
                "Ship damaged: {}: {} {}",
                x.ship.text(),
                x.hits,
                if x.hits == 1 { "hit" } else { "hits" }
            );
            if let Some(v) = x.flotation_after {
                s.push_str(&format!(", flotation damage {}", v.0));
            }
            if let Some(v) = x.fire_after {
                s.push_str(&format!(", fire damage {}", v.0));
            }
            for change in &x.systems {
                if change.before == change.after {
                    s.push_str(&format!(", {} hit, no change", plain(change.system)));
                } else {
                    s.push_str(&format!(
                        ", {} {} to {}",
                        plain(change.system),
                        change.before,
                        change.after
                    ));
                }
            }
            s.push_str(&evs(&x.events));
            s
        });
        o.each("Aircraft lost", &l.aircraft, |x| {
            let mut causes = Vec::new();
            if x.shot_down > 0 {
                causes.push(format!("{} shot down", x.shot_down));
            }
            if x.destroyed_on_ground > 0 {
                causes.push(format!("{} destroyed on the ground", x.destroyed_on_ground));
            }
            if x.written_off > 0 {
                causes.push(format!("{} written off", x.written_off));
            }
            if x.lost_with_ship > 0 {
                causes.push(format!("{} lost with their ship", x.lost_with_ship));
            }
            if x.ditched > 0 {
                causes.push(format!("{} ran out of fuel on the way home", x.ditched));
            }
            if x.no_deck > 0 {
                causes.push(format!(
                    "{} came down in the sea for want of a deck",
                    x.no_deck
                ));
            }
            if x.pilots_lost > 0 {
                causes.push(plural(x.pilots_lost, "pilot lost", "pilots lost"));
            }
            format!(
                "Aircraft lost: {}: {}{}",
                x.air_group.text(),
                causes.join(", "),
                evs(&x.events)
            )
        });
        for x in &l.aircraft_landed_elsewhere {
            o.line(format!(
                "Landed elsewhere: {} landed {} aircraft at {}, {} {}{}",
                x.from.text(),
                x.aircraft,
                x.at.text(),
                if x.formed { "forming" } else { "joining" },
                x.to.text(),
                evs(&[x.event])
            ));
        }
        for x in &l.air_groups_rebased {
            let mut line = format!(
                "Rebased: {} moved from {} to {}, {} miles (transfer range {} miles), with {} ready",
                x.air_group.text(),
                x.from.text(),
                x.to.text(),
                x.distance_nm.0,
                x.transfer_range_nm.0,
                x.ready
            );
            if let Some(pilots) = x.pilots {
                line.push_str(&format!(" and {pilots} pilots"));
            }
            line.push_str("; it flew nothing that day");
            line.push_str(&evs(&[x.event]));
            o.line(line);
        }
        o.each("Claims yesterday", &l.claims, |x| {
            if x.from_bombardment {
                // From the sea (decision 0092, 17): aircraft on the ground.
                format!(
                    "Claim from the sea at {}: {} aircraft destroyed on the ground{}",
                    at(&x.place),
                    x.claimed_on_ground,
                    ev(x.event)
                )
            } else if x.from_air {
                let (place, words) = sim_core::events::air_claim_words(
                    x.claimed_sunk,
                    x.claimed_hits,
                    x.taken_for,
                    x.claimed_shot_down,
                    x.claimed_on_ground,
                );
                format!("Claim {place} {}: {words}{}", at(&x.place), ev(x.event))
            } else {
                format!(
                    "Claim: {} enemy sunk at {}{}",
                    ships(x.claimed_sunk),
                    at(&x.place),
                    ev(x.event)
                )
            }
        });

        o.heading("TASK FORCES");
        if self.task_forces.is_empty() {
            o.line("Task forces: none".to_string());
        }
        for f in &self.task_forces {
            let mut s = format!(
                "{} at {}: {}, {}, speed {}",
                f.task_force.text(),
                at(&f.place),
                plain(f.status),
                plain(f.mission),
                f.speed
            );
            if let Some(d) = &f.destination {
                s.push_str(&format!(", heading for {}", at(d)));
            }
            s.push_str(&format!(
                ", readiness {}, {}",
                f.readiness,
                if f.alert { "alert" } else { "not alert" }
            ));
            if rules.fatigue {
                s.push_str(&format!(", fatigue {}", f.fatigue.0));
            }
            if f.status == "away_at_yard" {
                // A force away has nothing of route, fuel or movement to
                // say (decision 0069); its own part gives the day it is back.
                o.line(format!(
                    "{} at {}: away at the yard, {}",
                    f.task_force.text(),
                    at(&f.place),
                    ships(f.ships_afloat)
                ));
                continue;
            }
            s.push_str(&format!(", {} afloat", ships(f.ships_afloat)));
            match f.movement {
                "cannot_move" => s.push_str(", CANNOT MOVE (propulsion out)"),
                "slowed" => s.push_str(", slowed (propulsion damaged)"),
                _ => {}
            }
            if let (Some(supply), Some(fuel)) = (f.cargo_supply_t, f.cargo_fuel_t) {
                s.push_str(&format!(
                    ", cargo aboard: {} t supply, {} t fuel",
                    supply.0, fuel.0
                ));
            }
            if let Some(halt) = &f.halt {
                s.push_str(&format!(
                    ", HALTED at {} since {} to {}{}",
                    at(&halt.place),
                    halt.since,
                    waits_text(&halt.waits_for),
                    if halt.bombarded {
                        " (it has bombarded)"
                    } else {
                        ""
                    }
                ));
            }
            if !f.act_at.is_empty() {
                let marks: Vec<String> = f.act_at.iter().map(at).collect();
                s.push_str(&format!(", route marked to act at {}", marks.join(", ")));
            }
            if let Some(p) = &f.preparation {
                let roster: Vec<String> = p.roster.iter().map(|u| u.handle.clone()).collect();
                s.push_str(&format!(
                    ", PREPARING a landing at {} with {}: loading {}, lands at {} of the port's rate there",
                    at(&p.hex),
                    if roster.is_empty() {
                        "no unit on its roster".to_string()
                    } else {
                        format!("roster {}", roster.join(", "))
                    },
                    p.loading.0,
                    p.landing_rate.0
                ));
                if let Some(order) = p.order {
                    s.push_str(&format!(" (by your order {order})"));
                }
            }
            if let Some(place) = &f.landing_at {
                s.push_str(&format!(
                    ", landing at {} (works at the landing rate until the unloading there is complete)",
                    at(place)
                ));
            }
            o.line(s);
            if let Some(fuel) = &f.least_fuelled {
                let range = match fuel.range_nm {
                    Some(r) => format!("{r} miles"),
                    None => "no range figure".to_string(),
                };
                o.sub(format!(
                    "Least fuelled: {} with {} t, range {} at cruising speed",
                    fuel.ship.text(),
                    fuel.fuel_t.0,
                    range
                ));
            }
        }

        if !self.away_at_yard.is_empty() {
            o.heading("AWAY AT THE YARD");
            for a in &self.away_at_yard {
                let back = match (a.returns_before_end, a.last_day) {
                    (Some(false), Some(last)) => {
                        format!("does not return before the game ends (day {last} is the last)")
                    }
                    _ => format!("back on day {}", a.return_day),
                };
                o.line(format!(
                    "{} at {}: {}, {}",
                    a.task_force.text(),
                    a.port.text(),
                    names(&a.ships),
                    back
                ));
            }
        }

        if let Some(halts) = &self.halts {
            o.heading("HALTS");
            for h in &halts.halted {
                o.line(format!(
                    "Halted yesterday: {} at {} at {} to {}{}",
                    h.task_force.text(),
                    at(&h.place),
                    h.at,
                    waits_text(&h.waits_for),
                    ev(h.event)
                ));
            }
            for h in &halts.steamed_on {
                let why: Vec<&str> = h
                    .reasons
                    .iter()
                    .map(|r| match *r {
                        "bombarded" => "it had bombarded",
                        "cannot_bombard_any_longer" => "it could bombard no longer",
                        _ => "its cargo work was done",
                    })
                    .collect();
                o.line(format!(
                    "Steamed on yesterday: {} from {}: {}{}",
                    h.task_force.text(),
                    at(&h.place),
                    why.join(" and "),
                    ev(h.event)
                ));
            }
        }

        o.heading("AIR GROUPS");
        // Squadron name: the part before the first comma.
        let squadron = |g: &crate::lines::AirGroupLine| -> Option<String> {
            g.air_group
                .name
                .as_ref()
                .map(|n| n.split(',').next().unwrap_or(n).trim().to_string())
        };
        o.each("Air groups", &self.air_groups, |g| {
            let mut s = format!("{} ({})", g.air_group.text(), g.aircraft_type);
            if g.location.is_none() {
                s.push_str(
                    ": lost with its base, which changed side; it has no place and takes no order",
                );
                return s;
            }
            // Two of the side's own groups under one squadron's name are
            // two groups with their own aircraft, not one counted twice.
            if let Some(mine) = squadron(g) {
                let others: Vec<&str> = self
                    .air_groups
                    .iter()
                    .filter(|o| o.air_group.handle != g.air_group.handle)
                    .filter(|o| squadron(o).as_deref() == Some(mine.as_str()))
                    .map(|o| o.air_group.handle.as_str())
                    .collect();
                if !others.is_empty() {
                    s.push_str(&format!(", a separate group from {},", others.join(", ")));
                }
            }
            let Some(location) = &g.location else {
                s.push_str(
                    ": lost with its base, which changed side; it has no place and takes no order",
                );
                return s;
            };
            s.push_str(&format!(" at {}", location.text()));
            if let Some(p) = &g.place {
                // A base's name is already in the location.
                let base = location.handle.starts_with("base:");
                s.push_str(&format!(" {}", if base { p.coords() } else { at(p) }));
            }
            s.push_str(&format!(": {} ready", g.ready));
            if rules.wear {
                s.push_str(&format!(", {} damaged", g.damaged));
            }
            match g.pilots {
                Some(p) => s.push_str(&format!(", {p} pilots")),
                // Pilots are counted for some groups only: where they are
                // not, no pilot limit applies to what the group flies.
                None if rules.pilots => s.push_str(", pilots not counted (no pilot limit)"),
                None => {}
            }
            if rules.fatigue {
                s.push_str(&format!(", fatigue {}", g.fatigue.0));
            }
            s.push_str(&format!(
                ", radius {} miles, search range {} miles",
                g.strike_radius_nm.0, g.search_range_nm.0
            ));
            if let Some(reach) = g.reach_nm {
                s.push_str(&format!(", extended reach {} miles", reach.0));
            }
            if let Some(list) = &g.target_priority {
                s.push_str(&format!(", target priority {}", list.join(", ")));
            }
            s.push_str(&format!(", mission {}", plain(g.mission)));
            if let Some(t) = &g.target {
                s.push_str(&format!(" at {}", t.text()));
            }
            if g.moved_yesterday {
                s.push_str(", moved to this base yesterday and flew nothing yesterday");
            }
            if let Some(k) = &g.kept_on_ground_by_field {
                s.push_str(&format!(
                    ", {} of its aircraft kept on the ground by the damage ({}) to the airfield",
                    k.aircraft, k.damage.0
                ));
            }
            if let Some(w) = &g.would_strike {
                s.push_str(&format!(
                    ", would strike {} now ({}, {} miles)",
                    w.contact.text(),
                    w.taken_for,
                    w.distance_nm
                ));
            }
            if rules.fatigue {
                if let Some(t) = g.rest_above {
                    s.push_str(&format!(", stands down above {}", t.0));
                }
                if g.stood_down {
                    s.push_str(", stood down");
                }
            }
            s
        });

        o.heading("BASES");
        if self.bases.is_empty() {
            o.line("Bases: none".to_string());
        }
        for b in &self.bases {
            let stocks = |o: &mut Out| {
                let Some(stocks) = &b.stocks_yesterday else {
                    return;
                };
                if let (true, Some(s)) = (rules.supply, &stocks.supply) {
                    let causes = causes_text(&[
                        ("landed", s.landed_t.0),
                        ("loaded onto ships", s.loaded_onto_ships_t.0),
                        ("sorties and upkeep", s.sorties_and_upkeep_t.0),
                        ("repairs", s.repairs_t.0),
                        (
                            "feeding men ashore",
                            s.men_fed_t.map_or(sim_core::fixed::Fixed::ZERO, |d| d.0),
                        ),
                        ("destroyed by raids", s.destroyed_by_raids_t.0),
                        (
                            "taken by marching units",
                            s.taken_by_marches_t
                                .map_or(sim_core::fixed::Fixed::ZERO, |d| d.0),
                        ),
                        (
                            "brought by marching units",
                            s.arrived_with_marches_t
                                .map_or(sim_core::fixed::Fixed::ZERO, |d| d.0),
                        ),
                    ]);
                    o.sub(format!(
                        "Stocks yesterday: supply {} t to {} t ({causes})",
                        s.start_t.0, s.end_t.0
                    ));
                }
                if let Some(f) = &stocks.fuel {
                    let causes = causes_text(&[
                        ("landed", f.landed_t.0),
                        ("loaded onto ships", f.loaded_onto_ships_t.0),
                        ("refuelling ships", f.refuelling_ships_t.0),
                    ]);
                    o.sub(format!(
                        "Stocks yesterday: fuel {} t to {} t ({causes})",
                        f.start_t.0, f.end_t.0
                    ));
                }
            };
            if rules.supply {
                o.line(format!(
                    "{} at {}: supply {} t, fuel {} t (for ships; aircraft do not use it)",
                    b.base.text(),
                    b.place.coords(),
                    b.supply_t.0,
                    b.fuel_t.0
                ));
                let lasts = match b.days_of_supply {
                    Some(d) => format!("supply lasts {} days at yesterday's spending", d.0),
                    None => "no spending yesterday".to_string(),
                };
                o.sub(format!(
                    "Yesterday: {} t supply spent, {} t destroyed; {}",
                    b.spent_yesterday_t.0, b.destroyed_yesterday_t.0, lasts
                ));
            } else {
                o.line(format!(
                    "{} at {}: fuel {} t (for ships; aircraft do not use it)",
                    b.base.text(),
                    b.place.coords(),
                    b.fuel_t.0
                ));
            }
            stocks(&mut o);
            if let Some(w) = &b.works {
                let airfield = match (w.airfield, &w.airfield_work_needed) {
                    (false, Some(needed)) => {
                        format!(
                            "no airfield, {} of {} work done",
                            w.airfield_work.0, needed.0
                        )
                    }
                    (false, None) => format!("no airfield, {} work done", w.airfield_work.0),
                    (true, _) => "an airfield".to_string(),
                };
                o.sub(format!(
                    "Works: {airfield}; defences {}; yesterday's work on the port {}, on the aircraft {}",
                    w.defences.0, w.port_work_yesterday.0, w.aircraft_work_yesterday.0
                ));
            }
            if let Some(d) = &b.airfield_damage {
                let level = match (d.takes_aircraft, &d.refused_at) {
                    (true, Some(at)) => format!(
                        "it takes aircraft; at damage {} or more it takes none",
                        at.0
                    ),
                    (true, None) => "it takes aircraft".to_string(),
                    (false, Some(at)) => format!(
                        "it takes no aircraft (damage {} or more): rebases to it are refused, aircraft with no deck land elsewhere and arrivals wait until the damage is below {}",
                        at.0, at.0
                    ),
                    (false, None) => "it takes no aircraft".to_string(),
                };
                o.sub(format!(
                    "Airfield damage {}: {} of every 100 aircraft may fly; {}",
                    d.damage.0, d.share_that_flies.0, level
                ));
            }
            if let Some(y) = &b.airfield_yesterday {
                for a in &y.damaged {
                    o.sub(format!(
                        "Airfield hit yesterday: {} hit{} on the airfield, damage {}{}",
                        a.hits,
                        if a.hits == 1 { "" } else { "s" },
                        a.damage.0,
                        ev(a.event)
                    ));
                }
                for a in &y.mended {
                    o.sub(format!(
                        "Airfield mended yesterday: {} mended {} points of damage, damage {}{}",
                        a.land_unit.text(),
                        a.points.0,
                        a.damage.0,
                        ev(a.event)
                    ));
                }
                for a in &y.arrivals_waiting {
                    o.sub(format!(
                        "Arrival held back yesterday: {} aircraft due at {} wait, the airfield's damage {} takes no aircraft{}",
                        a.aircraft,
                        a.air_group.text(),
                        a.damage.0,
                        ev(a.event)
                    ));
                }
            }
            for r in &b.returned_yesterday {
                o.sub(format!(
                    "Returned yesterday from the yard: {} with {}{}",
                    r.task_force.text(),
                    names(&r.ships),
                    ev(r.event)
                ));
            }
            if !b.repaired_yesterday.is_empty() {
                o.sub("Ships repaired yesterday:".to_string());
                for r in &b.repaired_yesterday {
                    let left = |label: &str, list: &[&'static str]| -> String {
                        if list.is_empty() {
                            String::new()
                        } else {
                            let words: Vec<String> = list.iter().map(|w| plain(w)).collect();
                            format!(", {label} {}", words.join(", "))
                        }
                    };
                    o.0.push_str("    - ");
                    o.0.push_str(&format!(
                        "{}: {} points, fire damage {}, flotation damage {}{}{}{}{}\n",
                        r.ship.text(),
                        r.points.0,
                        r.fire_damage.0,
                        r.flotation_damage.0,
                        left("restored:", &r.restored),
                        left("still degraded:", &r.systems_degraded),
                        left("still out:", &r.systems_out),
                        ev(r.event)
                    ));
                }
            }
        }

        if let Some(land) = &self.land_units {
            land_units_text(&mut o, land);
        }

        if let Some(changes) = &self.base_changes {
            base_changes_text(&mut o, changes);
        }

        o.heading("ARRIVALS");
        o.each(
            "Arrivals due on the coming day and the next two days",
            &self.arrivals.due,
            |x| {
                format!(
                    "Due day {}: {}, {} aircraft, {} pilots, arriving in the night pulse that begins that day, before its morning",
                    x.day,
                    x.air_group.text(),
                    x.aircraft,
                    x.pilots
                )
            },
        );
        o.each("Replacements", &self.arrivals.replacements, |x| {
            format!(
                "Replacements: {} receives {} {} aircraft and {} pilots a day",
                x.base.text(),
                x.aircraft_per_day,
                x.aircraft_type,
                x.pilots_per_day
            )
        });

        o.heading("WARNINGS");
        let w = &self.warnings;
        o.each("Ships that cannot reach a base", &w.ship_cannot_reach_base, |x| {
            match (&x.nearest_base, x.distance_nm) {
                (Some(base), Some(d)) => format!(
                    "Cannot reach base: {} is {} miles from {}, range {} miles at cruising speed",
                    x.ship.text(),
                    d,
                    base.text(),
                    x.range_nm
                ),
                _ => format!(
                    "Cannot reach base: {} has no base it can reach by sea, range {} miles at cruising speed",
                    x.ship.text(),
                    x.range_nm
                ),
            }
        });
        if rules.supply {
            o.each("Bases short of supply", &w.base_supply_low, |x| {
                if x.reason == "no_supply" {
                    format!("Supply low: {} has no supply", x.base.text())
                } else {
                    let days = x.days_of_supply.map_or(String::new(), |d| d.0.to_string());
                    format!(
                        "Supply low: {} has {} t, {} days at yesterday's spending of {} t",
                        x.base.text(),
                        x.supply_t.0,
                        days,
                        x.spent_yesterday_t.0
                    )
                }
            });
        } else {
            o.line("Bases short of supply: base supply is not in play".to_string());
        }
        o.each(
            "Bases without fuel and a force stopped there",
            &w.base_without_fuel_with_force,
            |x| {
                format!(
                    "No fuel: {} has no fuel and {} stopped there",
                    x.base.text(),
                    names(&x.task_forces)
                )
            },
        );
        o.each(
            "Air groups short of pilots",
            &w.air_group_short_of_pilots,
            |x| {
                format!(
                    "Short of pilots: {} has {} pilots for {} ready aircraft",
                    x.air_group.text(),
                    x.pilots,
                    x.ready
                )
            },
        );
        if rules.fatigue {
            o.each("Air groups tired", &w.air_group_tired, |x| {
                let mut s = format!("Tired: {} fatigue {}", x.air_group.text(), x.fatigue.0);
                if let Some(t) = x.rest_above {
                    s.push_str(&format!(", stands down above {}", t.0));
                }
                if x.stood_down {
                    s.push_str(", stood down");
                }
                s
            });
        } else {
            o.line("Air groups tired: fatigue is not in play".to_string());
        }
        if !rules.wear {
            o.line("Aircraft wear: not in play".to_string());
        }
        o.each(
            "Task forces that cannot move or are slowed",
            &w.task_force_propulsion,
            |x| {
                let ships: Vec<String> = x
                    .ships
                    .iter()
                    .map(|s| format!("{} propulsion {}", s.ship.text(), s.propulsion))
                    .collect();
                if x.effect == "cannot_move" {
                    format!(
                        "Cannot move: {} (the force has no speed while a ship's propulsion is out): {}",
                        x.task_force.text(),
                        ships.join(", ")
                    )
                } else {
                    format!(
                        "Slowed: {} (it steams at the speed of its slowest ship): {}",
                        x.task_force.text(),
                        ships.join(", ")
                    )
                }
            },
        );

        o.each(
            "Strike groups with nothing in reach",
            &w.strike_group_out_of_reach,
            |x| {
                // On extended reach the test is against the reach in force.
                let (word, miles) = match x.reach_nm {
                    Some(reach) => ("extended reach", reach.0),
                    None => ("radius", x.radius_nm.0),
                };
                if x.mission == "naval_strike" {
                    format!(
                        "Out of reach: {} is on naval strike with a {word} of {miles} miles and holds contacts, but none is within it; the nearest is {}, {} miles away",
                        x.air_group.text(),
                        x.nearest.text(),
                        x.distance_nm
                    )
                } else {
                    format!(
                        "Out of reach: {} is on base strike with a {word} of {miles} miles, but its target {} is {} miles away",
                        x.air_group.text(),
                        x.nearest.text(),
                        x.distance_nm
                    )
                }
            },
        );

        // Left out when none, so the briefing of a game with no target
        // order reads as it did.
        if !w.strike_group_holding.is_empty() {
            o.each(
                "Strike groups holding their strike",
                &w.strike_group_holding,
                |x| {
                    format!(
                        "Holding: {} is on naval strike with a target list of {} and holds contacts within {} {} miles, but none is of a kind on its list; the nearest is {}, taken for a {}, {} miles away; it launches nothing at them",
                        x.air_group.text(),
                        x.target_priority.join(", "),
                        if x.reach_nm.is_some() { "its extended reach of" } else { "its radius of" },
                        x.reach_nm.unwrap_or(x.radius_nm).0,
                        x.nearest.text(),
                        x.nearest_taken_for,
                        x.distance_nm
                    )
                },
            );
        }

        o.0
    }
}

/// The side's estimates of the enemy's men ashore (decision 0092, 19).
fn enemy_ashore_text(o: &mut Out, a: &EnemyAshorePart) {
    o.heading("ENEMY MEN ASHORE (ESTIMATES)");
    o.line(
        "These are estimates from reports, not counts: each can be wrong by up to half either way, and an estimate is never corrected until a new report replaces it"
            .to_string(),
    );
    for e in &a.estimates {
        let source = if e.day == 0 {
            format!("opening estimate, day {}", e.day)
        } else {
            format!("reported day {}", e.day)
        };
        o.line(format!(
            "Estimate: about {} enemy men ashore at {}, {source}, {} old{}",
            e.men,
            at(&e.place),
            plural(e.age_days, "day", "days"),
            ev(e.event)
        ));
    }
    for b in &a.bases_found_yesterday {
        o.line(format!(
            "Enemy base found yesterday: {} at {}, now on your map{}",
            b.base.text(),
            at(&b.place),
            ev(b.event)
        ));
    }
}

/// What yesterday's change of side of bases did to the side (decision 0090,
/// 29 and 30).
fn base_changes_text(o: &mut Out, c: &BaseChangesPart) {
    o.heading("BASES THAT CHANGED SIDE");
    for t in &c.taken {
        o.line(format!(
            "Base taken yesterday: {} at {} is yours now, with {} t supply and {} t fuel kept ({} t supply and {} t fuel lost in the change){}",
            t.base.text(),
            at(&t.place),
            t.supply_t.0,
            t.fuel_t.0,
            t.supply_lost_t.0,
            t.fuel_lost_t.0,
            ev(t.event)
        ));
    }
    for l in &c.lost {
        o.line(format!(
            "Base lost yesterday: {} at {} is the enemy's now{}",
            l.base.text(),
            at(&l.place),
            ev(l.event)
        ));
    }
    for g in &c.air_groups_lost {
        o.line(format!(
            "Air group lost with its base: {} with {} aircraft, lost with {}{}",
            g.air_group.text(),
            g.aircraft,
            g.base.text(),
            ev(g.event)
        ));
    }
    for s in &c.ships_lost {
        o.line(format!(
            "Ship lost in port: {} was lost with {}{}",
            s.ship.text(),
            s.base.text(),
            ev(s.event)
        ));
    }
    for h in &c.home_ports_lost {
        o.line(format!(
            "Home port lost: {} has no home port, since {} changed side{}",
            h.task_force.text(),
            h.base.text(),
            ev(h.event)
        ));
    }
    for m in &c.strike_missions_cleared {
        o.line(format!(
            "Strike mission cleared: {} has no mission, since {} is now your own{}",
            m.air_group.text(),
            m.base.text(),
            ev(m.event)
        ));
    }
}

/// The land units part (decision 0066, 21): each unit's men and place, then
/// what yesterday's events say happened to them, then the days of supply.
fn land_units_text(o: &mut Out, land: &LandUnitsPart) {
    o.heading("LAND UNITS");
    for u in &land.units {
        if u.destroyed {
            o.line(format!("{}: destroyed, no man left", u.land_unit.text()));
            continue;
        }
        let mut s = format!("{}: ", u.land_unit.text());
        let mut parts: Vec<String> = Vec::new();
        if let Some(place) = &u.place {
            parts.push(format!("{} men ashore at {}", u.men_ashore, at(place)));
        }
        if u.men_aboard > 0 {
            let ships: Vec<String> = u
                .aboard
                .iter()
                .map(|a| format!("{} {}", a.ship.text(), a.men))
                .collect();
            parts.push(format!(
                "{} men aboard ({})",
                u.men_aboard,
                ships.join(", ")
            ));
        }
        s.push_str(&parts.join("; "));
        s.push_str(&format!(
            "; posture {}, fatigue {}, fed share {}",
            u.posture, u.fatigue.0, u.fed_share.0
        ));
        if let Some(order) = u.posture_order {
            s.push_str(&format!(" (posture set by your order {order})"));
        }
        if let Some(w) = u.when_spent {
            s.push_str(&format!(
                "; when its attack is spent: {}",
                w.replace('_', " ")
            ));
        }
        if let Some(m) = &u.march {
            if let Some(f) = &m.falling_back {
                let why = if f.reason == "attack_spent" {
                    format!("after its attack at {} was spent", at(&f.from))
                } else {
                    format!("after the defeat at {}", at(&f.from))
                };
                s.push_str(&format!(
                    "; falling back to {} {}, day {} of {}; carries no supply ({} counted){}",
                    at(&m.to),
                    why,
                    m.days_marched + 1,
                    m.of_days,
                    m.days_counted.0,
                    ev(f.event)
                ));
            } else {
                s.push_str(&format!(
                    "; marching to {}, {} of {} days marched ({} counted)",
                    at(&m.to),
                    m.days_marched,
                    m.of_days,
                    m.days_counted.0
                ));
                if let Some(order) = m.order {
                    s.push_str(&format!(" (by your order {order})"));
                }
            }
        }
        if !u.march_to.is_empty() {
            let to: Vec<String> = u.march_to.iter().map(at).collect();
            s.push_str(&format!("; can march to {}", to.join(", ")));
        }
        // The experience and the day's activity are said when they are not
        // the standard ones, so a unit that was never trained or fought reads
        // as it always did.
        if u.experience != 2 || u.training.days > 0 || u.fighting.days > 0 || u.activity != "ready"
        {
            s.push_str(&format!(
                "; experience {} ({} of {} days of training, up to experience {}; {} of {} days of fighting)",
                u.experience,
                u.training.days,
                u.training.of_days,
                u.training.up_to,
                u.fighting.days,
                u.fighting.of_days
            ));
        }
        s.push_str(&format!(
            "; skills assault {}, holding {}, weight {}, movement clear {}, jungle {}, mountain {}, atoll {}",
            u.skills.assault,
            u.skills.holding,
            u.skills.weight,
            u.skills.movement.clear,
            u.skills.movement.jungle,
            u.skills.movement.mountain,
            u.skills.movement.atoll
        ));
        if u.skills.works != 2 {
            s.push_str(&format!(", works {}", u.skills.works));
        }
        if u.working {
            s.push_str("; working (it only works: always on defend, never attacks, its men do not fire or hold a base)");
        }
        if let Some(job) = u.job {
            s.push_str(&format!(
                "; works at {}{}",
                job,
                if u.job_is_default {
                    " (the default job)"
                } else {
                    ""
                }
            ));
        }
        if u.activity != "ready" {
            s.push_str(&format!("; activity {}", u.activity));
            if let Some(order) = u.activity_order {
                s.push_str(&format!(" (by your order {order})"));
            }
        }
        if let Some(r) = &u.rehearsal {
            s.push_str(&format!(
                "; rehearsed {} for a landing at {}",
                r.rehearsal.0,
                at(&r.hex)
            ));
        }
        if let Some(r) = &u.roster {
            s.push_str(&format!(
                "; on the roster of {} for a landing at {}, where it would gain fatigue {} landing",
                r.task_force.text(),
                at(&r.hex),
                r.landing_fatigue.0
            ));
        }
        s.push_str(match u.fed_last_night {
            "fed" => "; fed last night",
            "partly_fed" => "; partly fed last night",
            "unfed" => "; unfed last night",
            "not_known" => "; not known whether fed last night",
            _ => "",
        });
        o.line(s);
    }
    let before = o.0.len();
    for m in &land.landed_yesterday {
        o.line(format!(
            "Landed yesterday: {} men of {} from {} at {}, {} ashore and {} left aboard{}",
            m.men,
            m.land_unit.text(),
            m.ship.text(),
            at(&m.place),
            m.ashore,
            m.aboard,
            ev(m.event)
        ));
    }
    for m in &land.embarked_yesterday {
        o.line(format!(
            "Embarked yesterday: {} men of {} aboard {} at {}, {} in the ship and {} left ashore{}",
            m.men,
            m.land_unit.text(),
            m.ship.text(),
            at(&m.place),
            m.aboard,
            m.ashore,
            ev(m.event)
        ));
    }
    for b in &land.bases_formed_yesterday {
        o.line(format!(
            "Base formed yesterday: {} at {} by the landing of {}{}",
            b.base.text(),
            at(&b.place),
            b.task_force.text(),
            ev(b.event)
        ));
    }
    for d in &land.divided_yesterday {
        o.line(format!(
            "Unit divided yesterday: the {} men of {} aboard {} became {}, since {} has men ashore elsewhere{}",
            d.men,
            d.from.text(),
            d.task_force.text(),
            d.land_unit.text(),
            d.from.text(),
            ev(d.event)
        ));
    }
    for l in &land.loading_complete_yesterday {
        let cargo = match l.cargo {
            Some(cargo) => format!(" and {cargo}"),
            None => String::new(),
        };
        let place = match &l.base {
            Some(base) => format!(" at {}", base.text()),
            None => " off the shore".to_string(),
        };
        o.line(format!(
            "Loading finished yesterday: {} loaded {}{cargo}{place}{}",
            l.task_force.text(),
            l.land_unit.text(),
            ev(l.event)
        ));
    }
    for f in &land.fed_last_night {
        let from = match &f.base {
            Some(base) => format!("{} fed from {}", f.fed, base.text()),
            None => "none fed, no base of the side in the hex".to_string(),
        };
        o.line(format!(
            "Fed last night: {} men ashore at {}: {}, {} unfed{}",
            f.men,
            at(&f.place),
            from,
            f.unfed,
            ev(f.event)
        ));
    }
    for l in &land.lost_yesterday {
        let line = match l.cause {
            "unfed" => format!(
                "{} lost {} men unfed{}, {} left ashore",
                l.land_unit.text(),
                l.men,
                l.place
                    .as_ref()
                    .map_or(String::new(), |p| format!(" at {}", at(p))),
                l.ashore.unwrap_or(0)
            ),
            "fight" => format!(
                "{} lost {} men in the fight{}, {} left ashore",
                l.land_unit.text(),
                l.men,
                l.place
                    .as_ref()
                    .map_or(String::new(), |p| format!(" at {}", at(p))),
                l.ashore.unwrap_or(0)
            ),
            "fire" => format!(
                "{} lost {} men to bombs or shells{}, {} left ashore",
                l.land_unit.text(),
                l.men,
                l.place
                    .as_ref()
                    .map_or(String::new(), |p| format!(" at {}", at(p))),
                l.ashore.unwrap_or(0)
            ),
            _ => format!(
                "{} lost {} men with {}",
                l.land_unit.text(),
                l.men,
                l.ship.as_ref().map_or(String::new(), Handle::text)
            ),
        };
        o.line(format!("Men lost yesterday: {line}{}", ev(l.event)));
    }
    for d in &land.destroyed_yesterday {
        o.line(format!(
            "Destroyed yesterday: {} has no man left{}",
            d.land_unit.text(),
            ev(d.event)
        ));
    }
    for f in &land.fights_yesterday {
        o.line(format!(
            "Fight ashore yesterday at {}{}",
            at(&f.place),
            ev(f.event)
        ));
    }
    for f in &land.fire_yesterday {
        o.line(format!(
            "Fire yesterday: {} fired at {} with {} men, {} shots{}",
            f.land_unit.text(),
            at(&f.place),
            f.firing_men,
            f.shots.0,
            ev(f.event)
        ));
    }
    for f in &land.fatigue_yesterday {
        o.line(format!(
            "Fatigue yesterday: {} {}, fatigue now {}{}",
            f.land_unit.text(),
            match f.change {
                "gained" => "attacked and tired",
                "marched" => "marched and tired",
                "marched_and_attacked" => "marched, attacked and tired",
                "landed" => "landed and tired",
                _ => "rested",
            },
            f.fatigue.0,
            ev(f.event)
        ));
    }
    for b in &land.back_to_defend_yesterday {
        o.line(format!(
            "Back to defend yesterday: {} was too tired to attack at fatigue {} and its attack order is spent{}",
            b.land_unit.text(),
            b.fatigue.0,
            ev(b.event)
        ));
    }
    for r in &land.fed_share_restored_yesterday {
        o.line(format!(
            "Fed share restored: {} has no men ashore and its fed share is 1{}",
            r.land_unit.text(),
            ev(r.event)
        ));
    }
    for m in &land.marches_yesterday {
        o.line(format!(
            "March yesterday: {} marched from {} toward {}, {} of {} days ({} counted){}",
            m.land_unit.text(),
            at(&m.from),
            at(&m.to),
            m.days,
            m.of_days,
            m.counted.0,
            ev(m.event)
        ));
    }
    for m in &land.march_arrivals_yesterday {
        let carried = match &m.base {
            Some(base) => format!("carrying {} t of supply into {}", m.supply_t.0, base.text()),
            None => "carrying no supply".to_string(),
        };
        o.line(format!(
            "Arrived yesterday: {} men of {} from {} at {}, {carried}{}",
            m.men,
            m.land_unit.text(),
            at(&m.from),
            at(&m.to),
            ev(m.event)
        ));
    }
    for b in &land.bases_formed_by_march_yesterday {
        o.line(format!(
            "Base formed yesterday: {} at {} by the march of {}; it is on your map only{}",
            b.base.text(),
            at(&b.place),
            b.land_unit.text(),
            ev(b.event)
        ));
    }
    for m in &land.marches_stopped_yesterday {
        let why = match m.reason {
            "ordered" => "you ordered it to stay",
            "no_men_ashore" => "it has no men ashore",
            _ => "no land link joins its hex to the hex it marched to",
        };
        o.line(format!(
            "March stopped yesterday: {} stopped its march to {} after {}: {why}{}",
            m.land_unit.text(),
            at(&m.to),
            plural(m.days, "day", "days"),
            ev(m.event)
        ));
    }
    for a in &land.attacks_failed_yesterday {
        o.line(format!(
            "Attack failed yesterday at {}: your attack did not carry the hex and its men are tired for it{}",
            at(&a.place),
            ev(a.event)
        ));
    }
    for f in &land.fell_back_yesterday {
        let why = if f.reason == "attack_spent" {
            "its attack spent"
        } else {
            "after the defeat"
        };
        o.line(format!(
            "Fell back yesterday: {} fell back from {} to {}, {}, {} men, fatigue {}{}",
            f.land_unit.text(),
            at(&f.from),
            at(&f.to),
            why,
            f.men,
            f.fatigue.0,
            ev(f.event)
        ));
    }
    for f in &land.surrendered_yesterday {
        o.line(format!(
            "Surrendered yesterday: {} surrendered at {}, {} men, with nowhere to fall back to{}",
            f.land_unit.text(),
            at(&f.place),
            f.men,
            ev(f.event)
        ));
    }
    for p in &land.prisoners_taken_yesterday {
        o.line(format!(
            "An enemy unit surrendered at {}: {} men taken prisoner{}",
            at(&p.place),
            p.men,
            ev(p.event)
        ));
    }
    for l in &land.loading_yesterday {
        let what = match (l.change, l.reason, l.loading_before) {
            ("lost", Some("spent"), Some(before)) => format!(
                "the landing at {} is done and its order with it (loading was {})",
                at(&l.hex),
                before.0
            ),
            ("lost", Some(why), Some(before)) => format!(
                "loading for {} fell from {} to {} ({})",
                at(&l.hex),
                before.0,
                l.loading.0,
                match why {
                    "new_landing_place" => "a new landing place was ordered",
                    _ => "a unit was added to the roster",
                }
            ),
            _ => format!("loading for {} is now {}", at(&l.hex), l.loading.0),
        };
        o.line(format!(
            "Loading yesterday: {}: {what}{}",
            l.task_force.text(),
            ev(l.event)
        ));
    }
    for r in &land.rehearsal_yesterday {
        let what = match (r.change, r.reason) {
            ("lost", Some(why)) => format!(
                "lost its rehearsal of {} for {} ({})",
                r.rehearsal.0,
                at(&r.hex),
                match why {
                    "new_landing_place" => "its force's landing place was changed",
                    "landed" => "its men landed",
                    _ => "it rehearsed for another landing place",
                }
            ),
            _ => format!(
                "rehearsed for the landing at {}, now {}{}",
                at(&r.hex),
                r.rehearsal.0,
                r.task_force
                    .as_ref()
                    .map_or(String::new(), |f| format!(" (roster of {})", f.text()))
            ),
        };
        o.line(format!(
            "Rehearsal yesterday: {} {what}{}",
            r.land_unit.text(),
            ev(r.event)
        ));
    }
    for r in &land.roster_left_yesterday {
        o.line(format!(
            "Roster yesterday: {} left the roster of {}: {}{}",
            r.land_unit.text(),
            r.task_force.text(),
            match r.reason {
                "taken_by_another_force" => "another force's order took it",
                "dropped_by_order" => "your order dropped it",
                _ => "its men landed",
            },
            ev(r.event)
        ));
    }
    for p in &land.preparation_shared_yesterday {
        let units: Vec<String> = p.units.iter().map(|u| u.handle.clone()).collect();
        o.line(format!(
            "Preparation shared yesterday: {} split off {}, which has the landing at {} with loading {} and {}{}",
            p.task_force.text(),
            p.new_force.text(),
            at(&p.hex),
            p.loading.0,
            if units.is_empty() {
                "no unit on its roster".to_string()
            } else {
                format!("roster {}", units.join(", "))
            },
            ev(p.event)
        ));
    }
    for d in &land.experience_days_yesterday {
        o.line(format!(
            "Experience yesterday: {} counted a day of {}, {} toward the next step from experience {}{}",
            d.land_unit.text(),
            d.source,
            d.days,
            d.experience,
            ev(d.event)
        ));
    }
    for r in &land.experience_raised_yesterday {
        o.line(format!(
            "Experience raised yesterday: {} reached experience {} by {}{}",
            r.land_unit.text(),
            r.experience,
            r.by,
            ev(r.event)
        ));
    }
    for w in &land.work_yesterday {
        let what = match w.job {
            "airfield" => format!("{} done on the airfield", w.total.0),
            "port" => format!("{} on the port yesterday", w.total.0),
            "aircraft" => format!("{} on the aircraft yesterday", w.total.0),
            _ => format!("defences {}", w.total.0),
        };
        o.line(format!(
            "Work yesterday: {} worked at the {} of {}, {} of work, {}{}",
            w.land_unit.text(),
            w.job,
            w.base.text(),
            w.work.0,
            what,
            ev(w.event)
        ));
    }
    for a in &land.airfields_built_yesterday {
        o.line(format!(
            "Airfield built yesterday: {} at {} has an airfield, {} of work done{}",
            a.base.text(),
            at(&a.place),
            a.work_done.0,
            ev(a.event)
        ));
    }
    if o.0.len() == before {
        o.line("Yesterday: no man was landed, embarked, fed, unfed or lost".to_string());
    }
    for h in &land.supply_by_hex {
        match (&h.base, h.supply_t) {
            (Some(base), Some(stock)) => {
                let lasts = match h.days_of_supply {
                    Some(days) => format!("lasts {} days at the present charge", days.0),
                    None => "has no charge".to_string(),
                };
                // The men are fed from every base of the side in the hex
                // (decision 0090, 24a): with more than one, all are named and
                // the stock is theirs together.
                let (named, holds) = if h.other_bases.is_empty() {
                    (base.text(), "holds")
                } else {
                    let mut names: Vec<String> = vec![base.text()];
                    names.extend(h.other_bases.iter().map(Handle::text));
                    let last = names.pop().expect("two names at least");
                    (format!("{} and {last}", names.join(", ")), "hold")
                };
                o.line(format!(
                    "Supply at {}: {named} {holds} {} t, the daily charge is {} t ({} aircraft and {} men), so it {lasts}",
                    at(&h.place),
                    stock.0,
                    h.daily_charge_t.0,
                    h.aircraft,
                    h.men_ashore
                ));
            }
            _ => o.line(format!(
                "Supply at {}: no base of the side here, so the {} men ashore get none",
                at(&h.place),
                h.men_ashore
            )),
        }
    }
}

/// The causes of a stock's change, as `landed 5, repairs 2`: the causes that
/// moved anything, or `nothing moved` if none did.
fn causes_text(causes: &[(&str, sim_core::fixed::Fixed)]) -> String {
    let moved: Vec<String> = causes
        .iter()
        .filter(|(_, t)| !t.is_zero())
        .map(|(name, t)| format!("{name} {t} t"))
        .collect();
    if moved.is_empty() {
        "nothing moved".to_string()
    } else {
        moved.join(", ")
    }
}

/// Points for each kind of ship, as `carrier 10, battleship 8`.
fn ship_points_text(points: &[ShipKindPoints]) -> String {
    let list: Vec<String> = points
        .iter()
        .map(|p| format!("{} {}", p.kind, p.points.0))
        .collect();
    list.join(", ")
}

fn objective_text(o: &mut Out, line: &ObjectiveLine) {
    let base = line.base.as_ref().map(Handle::text).unwrap_or_default();
    let points = line.points.map(|p| p.0.to_string()).unwrap_or_default();
    let what = match line.kind {
        "ships_sunk" => format!(
            "Objective: enemy ships sunk, points for each: {}",
            ship_points_text(&line.ship_points)
        ),
        "ships_crippled" => format!(
            "Objective: enemy ships left afloat with propulsion or flight deck out, points for each: {}",
            ship_points_text(&line.ship_points)
        ),
        "ships_out_of_action" => format!(
            "Objective: enemy ships out of action, counted by the share of the {}-day horizon each is away for repair (her days of repair at the end plus those she left for a yard with), points for a ship away for the whole horizon: {}",
            line.horizon_days.unwrap_or(0),
            ship_points_text(&line.ship_points)
        ),
        "aircraft_destroyed" => {
            format!("Objective: enemy aircraft destroyed, points for each: {points}")
        }
        "cargo_landed" => format!(
            "Objective: tons of supply and fuel you unload at {base} and do not load back, points for each ton: {points}"
        ),
        "base_held" => format!(
            "Objective: {base} is your own side's at the end of the game, points: {points}"
        ),
        "men_lost" => format!(
            "Objective: enemy men lost, by any cause, points for each hundred men: {points}"
        ),
        _ if line.closed == Some(true) => format!(
            "Objective: {base} shut at the end of each day (no supply, or no aircraft ready), points if shut every day: {points}, earned by the share of days"
        ),
        _ => format!(
            "Objective: {base} open at the end of each day (supply and an aircraft ready), points if open every day: {points}, earned by the share of days"
        ),
    };
    o.line(what);
    let known = &line.known;
    o.sub(match known.basis {
        "exact" if known.days.is_some() => format!(
            "What you know of your progress: {} of {} days{}",
            known.count.0,
            known.days.unwrap_or(0),
            known
                .points
                .map(|p| format!(", worth {} points", p.0))
                .unwrap_or_default()
        ),
        "exact" if line.kind == "base_held" => format!(
            "What you know of your progress: {}{}",
            if known.count.0.is_zero() {
                "the base is not yours now"
            } else {
                "the base is yours now"
            },
            known
                .points
                .map(|p| if known.count.0.is_zero() {
                    format!(", so no points yet (worth {} if it is yours at the end)", line.points.map_or(p.0, |x| x.0))
                } else {
                    format!(", worth {} points if it stays so", p.0)
                })
                .unwrap_or_default()
        ),
        "exact" => format!(
            "What you know of your progress: {}{}",
            known.count.0,
            known
                .points
                .map(|p| format!(", worth {} points", p.0))
                .unwrap_or_default()
        ),
        // What the side is told by its own claims, which can be wrong.
        "claims" if line.kind == "aircraft_destroyed" => format!(
            "What you know of your progress: {} enemy aircraft claimed shot down or destroyed on the ground, worth {} points (claims can be wrong)",
            known.count.0,
            known.points.map(|p| p.0.to_string()).unwrap_or_default()
        ),
        "claims" => format!(
            "What you know of your progress: {} enemy ships claimed sunk{} (claims can be wrong)",
            known.count.0,
            known
                .points
                .map(|p| format!(", worth {} points for the kinds you attacked", p.0))
                .unwrap_or_else(|| ", of kinds you were not told".to_string())
        ),
        _ => "What you know of your progress: nothing you can see shows it; it is counted at the end of the game".to_string(),
    });
}

fn result_text(o: &mut Out, result: &ResultPart) {
    o.line("The game is over. The result, counted from the truth:".to_string());
    for (name, side) in [("Allied", &result.allied), ("Japanese", &result.japanese)] {
        o.line(format!(
            "{name}: {} points, margin {}, {}",
            side.total.0, side.margin.0, side.outcome
        ));
        for score in &side.objectives {
            let base = score
                .base
                .as_ref()
                .map(|b| format!(" at {}", b.text()))
                .unwrap_or_default();
            let ships: Vec<String> = score
                .ships
                .iter()
                .map(|s| format!("{} {}", s.ships, s.kind))
                .collect();
            let by_kind = if ships.is_empty() {
                String::new()
            } else {
                format!(" ({})", ships.join(", "))
            };
            let of_days = match (score.days, score.horizon_days) {
                (Some(days), _) => format!(" of {days} days"),
                (None, Some(horizon)) => format!(" over {horizon} days"),
                (None, None) => String::new(),
            };
            o.sub(format!(
                "{}{base}: count {}{of_days}{by_kind}, points {}",
                plain(score.kind),
                score.count.0,
                score.points.0
            ));
            for s in &score.ships_out {
                o.sub_sub(format!(
                    "{} {}: {} days at the end + {} left with = {} days out, share {}, points {}",
                    s.ship, s.kind, s.at_end, s.left_with, s.days_out, s.share.0, s.points.0
                ));
            }
        }
    }
    if let Some(history) = &result.history {
        o.line(format!(
            "History (what happened in the real battle, not in this game): {history}"
        ));
    }
}

/// What a strike reported, as one line.
fn report_text(r: &StrikeReport) -> String {
    let contact = r.contact.as_ref().map(Handle::text).unwrap_or_default();
    let place = r.place.as_ref().map(Place::text).unwrap_or_default();
    let body = match r.result {
        "contact_new" | "contact_refreshed" => format!(
            "{} {}, {} estimated, taken for {}, at {}",
            if r.result == "contact_new" {
                "new contact"
            } else {
                "refreshed contact"
            },
            contact,
            ships(r.ship_count.unwrap_or(0)),
            r.taken_for.unwrap_or(""),
            place
        ),
        "contact_dropped" => format!("found nothing, {contact} dropped"),
        "contact_not_there" => format!("found nothing: {contact} not there at {place}"),
        _ => format!(
            "claim: {} enemy sunk at {}",
            ships(r.claimed_sunk.unwrap_or(0)),
            place
        ),
    };
    format!("Report: {body}{}", ev(r.event))
}

/// What a halted force waits to do, in words (decision 0096).
fn waits_text(waits_for: &[&str]) -> String {
    let words: Vec<&str> = waits_for
        .iter()
        .map(|w| match *w {
            "bombard" => "bombard (at night)",
            _ => "work cargo",
        })
        .collect();
    words.join(" and ")
}

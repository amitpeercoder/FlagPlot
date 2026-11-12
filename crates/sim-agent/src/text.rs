//! The briefing as text, made from the briefing's data and from nothing else
//! (decision 0048, 5).
//!
//! Fixed headings in capitals, one fact per line starting with `- `. A part
//! with nothing in it says `none`. Units read `task_force:5 "Name"`, places
//! `(q,r)` followed by the name of the side's own base there. Numbers are
//! whole or exact decimals. Nothing is advised and nothing is guessed. The
//! fixed words contain no digits, so every number in the text comes from the
//! data.

use crate::briefing::{Briefing, StrikeReport};
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

        o.heading("ACTION YESTERDAY");
        let a = &self.action;
        o.each("Fights at sea", &a.fights, |x| {
            format!(
                "Fight at {}: {}, ended {} in round {}{}",
                at(&x.place),
                names(&x.forces),
                plain(x.ended),
                x.rounds,
                ev(x.event)
            )
        });
        o.each("Salvoes fired at sea", &a.fired, |x| {
            format!(
                "Gunfire: {} {} at {}: {}, {} shots",
                x.firer.text(),
                x.weapon,
                x.target.text(),
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
                "{who}: {} at the {} of {}, {} shots{}",
                x.firer.text(),
                x.at,
                x.target.text(),
                x.shots.0,
                ev(x.event)
            )
        });
        o.each("Own anti-aircraft fire", &a.guns, |x| {
            format!(
                "Anti-aircraft fire: {} at the raid of enemy {}, {} shots{}",
                x.by.text(),
                x.raid.text(),
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
            format!(
                "Raid on {} by enemy {}: {} aircraft attacked, {} {}, {} on aircraft, {} on supply, {} on nothing{}",
                x.base.text(),
                x.air_group.text(),
                x.aircraft.0,
                x.hits,
                if x.hits == 1 { "hit" } else { "hits" },
                x.hits_on_aircraft,
                x.hits_on_supply,
                x.hits - x.hits_on_aircraft - x.hits_on_supply,
                ev(x.event)
            )
        });
        o.each("Attacks on ships", &a.attacks_on_ships, |x| {
            match x.aircraft {
                Some(n) => {
                    let mut s = format!(
                        "Attack on {} by enemy {}: {} aircraft",
                        x.against.text(),
                        x.by.text(),
                        n.0
                    );
                    if x.attacks > 1 {
                        s.push_str(&format!(", {}", plural(x.attacks, "attack", "attacks")));
                    }
                    s
                }
                None => format!(
                    "Fired on: enemy {} {} at {}: {}",
                    x.by.text(),
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
                s.push_str(&format!(
                    ", {} {} to {}",
                    plain(change.system),
                    change.before,
                    change.after
                ));
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
        o.each("Claims yesterday", &l.claims, |x| {
            if x.from_air {
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

        o.heading("AIR GROUPS");
        o.each("Air groups", &self.air_groups, |g| {
            let mut s = format!(
                "{} ({}) at {}",
                g.air_group.text(),
                g.aircraft_type,
                g.location.text()
            );
            if let Some(p) = &g.place {
                // A base's name is already in the location.
                let base = g.location.handle.starts_with("base:");
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
            s.push_str(&format!(", mission {}", plain(g.mission)));
            if let Some(t) = &g.target {
                s.push_str(&format!(" at {}", t.text()));
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
                        ("destroyed by raids", s.destroyed_by_raids_t.0),
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
                    "{} at {}: supply {} t, fuel {} t",
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
                    "{} at {}: fuel {} t",
                    b.base.text(),
                    b.place.coords(),
                    b.fuel_t.0
                ));
            }
            stocks(&mut o);
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

        o.0
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
        "aircraft_destroyed" => {
            format!("Objective: enemy aircraft destroyed, points for each: {points}")
        }
        "cargo_landed" => format!(
            "Objective: tons of supply and fuel you unload at {base}, points for each ton: {points}"
        ),
        _ if line.closed == Some(true) => format!(
            "Objective: {base} shut at the end (no supply, or no aircraft ready), points: {points}"
        ),
        _ => format!(
            "Objective: {base} open at the end (supply and an aircraft ready), points: {points}"
        ),
    };
    o.line(what);
    let known = &line.known;
    o.sub(match known.basis {
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
            o.sub(format!(
                "{}{base}: count {}{by_kind}, points {}",
                plain(score.kind),
                score.count.0,
                score.points.0
            ));
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

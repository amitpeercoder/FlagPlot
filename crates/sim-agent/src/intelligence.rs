//! The tool `intelligence` (decision 0226, "The paper, as amended"): what a
//! seat holds of the enemy, gathered on one answer.
//!
//! It reads the asking seat's picture (contacts, claims, estimates of men
//! ashore, reports of submarines, suspected waters), the public bases, and
//! the side's own events, day 0 among them, and nothing else. It makes no
//! rule, rolls nothing, keeps no state and is in no hash. A line of a part is
//! of the type the tool that already says that thing uses, so one wording of
//! a contact, a claim, a raid and an estimate holds across the tools. It
//! writes no sentence: its only words are those of suspected waters, as told.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use serde_json::{json, Map, Value};
use sim_core::belief::BattleClaim;
use sim_core::events::EventKind;
use sim_core::ids::{ContactId, EntityRef};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::time::Pulse;
use sim_core::view::SideView;

use crate::answer::{FORMAT, VERSION};
use crate::briefing::{read_day, Claim, ClaimLine, ClaimedType};
use crate::common::Ctx;
use crate::queries::{list_bases, list_contacts};

/// The parts of the paper, in its order.
pub const PARTS: [&str; 8] = [
    "seen",
    "lost_sight_of",
    "claimed",
    "enemy_has_done",
    "ashore",
    "enemy_bases",
    "enemy_submarines",
    "before_the_game",
];

/// The parts only a seat that plays the whole side is given (rule 5).
const WHOLE_SIDE_ONLY: [&str; 3] = ["lost_sight_of", "enemy_has_done", "before_the_game"];

fn to_json<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).expect("a line serialises")
}

fn pulse_name(pulse: Option<Pulse>) -> Value {
    pulse.map_or(Value::Null, |p| json!(p.name()))
}

fn pulse_rank(pulse: Option<Pulse>) -> u8 {
    match pulse {
        None => 0,
        Some(Pulse::Night) => 1,
        Some(Pulse::Morning) => 2,
        Some(Pulse::Afternoon) => 3,
    }
}

/// The number of a handle written `word:number`.
fn handle_number(line: &Value, key: &str) -> u32 {
    line[key]["handle"]
        .as_str()
        .and_then(|h| h.rsplit(':').next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// A claim of the picture as the briefing's `Claim` says it, with its
/// `pulse` added. `event` is left out where the seat is not shown the event.
fn claim_line(ctx: &Ctx<'_>, claim: &BattleClaim) -> Value {
    let seen = ctx.number_if_seen(claim.received_event);
    let line = Claim {
        ground_types: claim
            .ground_types
            .iter()
            .map(|(aircraft_type, aircraft)| ClaimedType {
                aircraft_type: ctx.aircraft_type_name(*aircraft_type),
                aircraft: *aircraft,
            })
            .collect(),
        air_types: claim
            .air_types
            .iter()
            .map(|(aircraft_type, aircraft)| ClaimedType {
                aircraft_type: ctx.aircraft_type_name(*aircraft_type),
                aircraft: *aircraft,
            })
            .collect(),
        claimed_by: claim
            .by_unit
            .iter()
            .map(|line| ClaimLine {
                unit: ctx.entity(line.unit),
                since_dissolved: !ctx.unit_in_game(line.unit),
                shot_down: line.shot_down,
                on_ground: line.on_ground,
                hits: line.hits,
            })
            .collect(),
        place: ctx.place(claim.hex),
        claimed_sunk: claim.claimed_sunk,
        claimed_hits: claim.claimed_hits,
        taken_for: claim.claimed_kind.map(sim_core::refdata::ShipKind::name),
        claimed_shot_down: claim.claimed_shot_down,
        claimed_on_ground: claim.claimed_on_ground,
        from_air: claim.from_air,
        from_bombardment: claim.from_bombardment,
        from_submarine: claim.from_submarine,
        event: seen.unwrap_or(0),
    };
    let mut value = to_json(&line);
    let map = value.as_object_mut().expect("a claim is an object");
    map.insert("pulse".into(), json!(claim.pulse.name()));
    if seen.is_none() {
        map.remove("event");
    }
    value
}

/// What the seat holds of the enemy, as the answer of the tool.
pub fn intelligence(view: &SideView<'_>) -> Value {
    let all = view.events();
    let mut ctx = Ctx::new(*view);
    ctx.set_events(&all);
    let picture = view.seat_picture();
    // A seat that plays the whole side: no command levels, or the area seat
    // under the easier setting, whose picture is the side's own belief
    // (decision 0226, ruling 10).
    let whole = !view.command_levels() || (view.seat().is_none() && view.has_one_picture());

    let held = list_contacts(view, None);

    // The side's events by day, from day 1.
    let mut by_day: BTreeMap<u32, Vec<SideEvent>> = BTreeMap::new();
    for event in &all {
        if event.day >= 1 {
            by_day.entry(event.day).or_default().push(event.clone());
        }
    }
    let days: Vec<(u32, crate::briefing::DayRead)> = by_day
        .iter()
        .map(|(day, events)| (*day, read_day(&ctx, events, &all)))
        .collect();

    let mut parts: BTreeMap<&'static str, Value> = BTreeMap::new();

    // SEEN: the contacts held, the least age first, and of one age the
    // higher number first.
    let mut seen: Vec<Value> = held.contacts.iter().map(to_json).collect();
    seen.sort_by(|a, b| {
        a["age_pulses"]
            .as_u64()
            .cmp(&b["age_pulses"].as_u64())
            .then(handle_number(b, "contact").cmp(&handle_number(a, "contact")))
    });
    if !seen.is_empty() {
        parts.insert("seen", json!(seen));
    }

    if whole {
        if let Some(lost) = lost_sight_of(&ctx, &all, &picture) {
            parts.insert("lost_sight_of", lost);
        }
    }

    // CLAIMED: every claim the picture holds, by day.
    let mut claim_days: BTreeMap<u32, Vec<Value>> = BTreeMap::new();
    let (mut sunk, mut hits, mut shot_down, mut on_ground) = (0u64, 0u64, 0u64, 0u64);
    for claim in picture.claims() {
        sunk += u64::from(claim.claimed_sunk);
        hits += u64::from(claim.claimed_hits);
        shot_down += u64::from(claim.claimed_shot_down);
        on_ground += u64::from(claim.claimed_on_ground);
        claim_days
            .entry(claim.day)
            .or_default()
            .push(claim_line(&ctx, claim));
    }
    if !claim_days.is_empty() {
        let listed: Vec<Value> = claim_days
            .into_iter()
            .rev()
            .map(|(day, claims)| json!({"day": day, "claims": claims}))
            .collect();
        parts.insert(
            "claimed",
            json!({
                "days": listed,
                "totals": {
                    "sunk": sunk,
                    "hits": hits,
                    "shot_down": shot_down,
                    "on_ground": on_ground,
                },
            }),
        );
    }

    // WHAT THE ENEMY HAS DONE: by day, the latest first.
    if whole {
        let mut done: Vec<Value> = Vec::new();
        for (day, read) in days.iter().rev() {
            let mut entry = Map::new();
            let mut any = false;
            let mut put = |key: &str, lines: Vec<Value>| {
                if !lines.is_empty() {
                    any = true;
                    entry.insert(key.to_string(), Value::Array(lines));
                }
            };
            put(
                "raids",
                read.action.raids_on_bases.iter().map(to_json).collect(),
            );
            put(
                "bombardments",
                read.action
                    .bombardments_against
                    .iter()
                    .map(to_json)
                    .collect(),
            );
            put(
                "attacks_on_ships",
                read.action.attacks_on_ships.iter().map(to_json).collect(),
            );
            put("fights", read.action.fights.iter().map(to_json).collect());
            if any {
                let mut line = Map::new();
                line.insert("day".into(), json!(day));
                line.extend(entry);
                done.push(Value::Object(line));
            }
        }
        if !done.is_empty() {
            parts.insert("enemy_has_done", json!(done));
        }
    }

    // ASHORE: the estimates held, and every hex found empty.
    let mut ashore = Map::new();
    if !held.ground_estimates.is_empty() {
        ashore.insert(
            "estimates".into(),
            json!(held
                .ground_estimates
                .iter()
                .map(to_json)
                .collect::<Vec<_>>()),
        );
    }
    let mut found_empty: Vec<Value> = Vec::new();
    for (day, read) in days.iter().rev() {
        for line in &read.found_empty {
            let mut value = to_json(line);
            value
                .as_object_mut()
                .expect("a line is an object")
                .insert("day".into(), json!(day));
            found_empty.push(value);
        }
    }
    if !found_empty.is_empty() {
        ashore.insert("found_empty".into(), json!(found_empty));
    }
    if !ashore.is_empty() {
        parts.insert("ashore", Value::Object(ashore));
    }

    // ENEMY BASES: only those of which something is held.
    let own = crate::record::seat_name(view.side());
    let mut bases: Vec<Value> = Vec::new();
    for base in list_bases(view, None).enemy_bases {
        if base.held_by == own {
            continue;
        }
        let estimate = held
            .ground_estimates
            .iter()
            .find(|e| e.place.q == base.place.q && e.place.r == base.place.r);
        let ground: Vec<&BattleClaim> = picture
            .claims()
            .iter()
            .filter(|c| {
                c.from_air
                    && c.claimed_on_ground > 0
                    && c.hex.q == base.place.q
                    && c.hex.r == base.place.r
            })
            .collect();
        if estimate.is_none() && ground.is_empty() {
            continue;
        }
        let mut line = Map::new();
        line.insert("base".into(), to_json(&base.base));
        line.insert("place".into(), to_json(&base.place));
        line.insert("held_by".into(), json!(base.held_by));
        if let Some(estimate) = estimate {
            line.insert("estimate".into(), to_json(estimate));
        }
        if !ground.is_empty() {
            let total: u64 = ground.iter().map(|c| u64::from(c.claimed_on_ground)).sum();
            line.insert("claimed_on_ground_from_air".into(), json!(total));
            line.insert(
                "claims".into(),
                json!(ground
                    .iter()
                    .map(|c| json!({"day": c.day, "number": c.claimed_on_ground}))
                    .collect::<Vec<_>>()),
            );
        }
        bases.push(Value::Object(line));
    }
    if !bases.is_empty() {
        parts.insert("enemy_bases", json!(bases));
    }

    // ENEMY SUBMARINES: the reports held and the waters as told.
    let mut subs = Map::new();
    if !held.submarine_reports.is_empty() {
        subs.insert(
            "reports".into(),
            json!(held
                .submarine_reports
                .iter()
                .map(to_json)
                .collect::<Vec<_>>()),
        );
    }
    if !held.suspected_waters.is_empty() {
        subs.insert(
            "suspected_waters".into(),
            json!(held
                .suspected_waters
                .iter()
                .map(to_json)
                .collect::<Vec<_>>()),
        );
    }
    if !subs.is_empty() {
        parts.insert("enemy_submarines", Value::Object(subs));
    }

    if whole {
        if let Some(before) = before_the_game(&ctx, &all) {
            parts.insert("before_the_game", before);
        }
    }

    let named: Vec<&str> = PARTS
        .iter()
        .copied()
        .filter(|name| parts.contains_key(name))
        .collect();
    let mut answer = Map::new();
    answer.insert("format".into(), json!(FORMAT));
    answer.insert("version".into(), json!(VERSION));
    answer.insert("kind".into(), json!("intelligence"));
    answer.insert("parts".into(), json!(named));
    for name in &named {
        answer.insert((*name).to_string(), parts[name].clone());
    }
    if !whole {
        answer.insert("not_kept".into(), json!(WHOLE_SIDE_ONLY));
    }
    Value::Object(answer)
}

/// LOST SIGHT OF: each contact dropped, the latest first.
fn lost_sight_of(
    ctx: &Ctx<'_>,
    all: &[SideEvent],
    picture: &sim_core::view::CommandView<'_>,
) -> Option<Value> {
    let held: BTreeSet<ContactId> = picture.contacts().map(|c| c.id).collect();
    // The last report that made or refreshed each contact.
    let mut last: BTreeMap<ContactId, Value> = BTreeMap::new();
    // (day, pulse rank, contact number, line)
    let mut lost: BTreeMap<ContactId, (u32, u8, Value)> = BTreeMap::new();
    for event in all {
        let SideEventKind::Whole(kind) = &event.kind else {
            continue;
        };
        match kind {
            EventKind::ReportReceived {
                contact,
                hex,
                ship_count,
                kind,
                ..
            } => {
                last.insert(
                    *contact,
                    json!({
                        "day": event.day,
                        "pulse": pulse_name(event.pulse),
                        "place": to_json(&ctx.place(*hex)),
                        "ship_count": ship_count,
                        "taken_for": kind.name(),
                        "event": ctx.number(event.id),
                    }),
                );
            }
            EventKind::ContactDropped {
                contact,
                age_pulses,
                ..
            } if !held.contains(contact) => {
                lost.entry(*contact).or_insert_with(|| {
                    (
                        event.day,
                        pulse_rank(event.pulse),
                        json!({
                            "day": event.day,
                            "pulse": pulse_name(event.pulse),
                            "event": ctx.number(event.id),
                            "why": "no_report_since",
                            "age_pulses": age_pulses,
                        }),
                    )
                });
            }
            EventKind::ContactNotThere { contact, hex, .. } if !held.contains(contact) => {
                lost.entry(*contact).or_insert_with(|| {
                    (
                        event.day,
                        pulse_rank(event.pulse),
                        json!({
                            "day": event.day,
                            "pulse": pulse_name(event.pulse),
                            "event": ctx.number(event.id),
                            "why": "strike_found_nothing",
                            "place": to_json(&ctx.place(*hex)),
                        }),
                    )
                });
            }
            _ => {}
        }
    }
    let mut lines: Vec<(u32, u8, u32, Value)> = lost
        .into_iter()
        .filter_map(|(contact, (day, rank, dropped))| {
            let report = last.get(&contact)?.clone();
            let handle = ctx.entity(EntityRef::Contact(contact));
            let number = handle
                .handle
                .rsplit(':')
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or(0);
            Some((
                day,
                rank,
                number,
                json!({
                    "contact": to_json(&handle),
                    "last_report": report,
                    "dropped": dropped,
                }),
            ))
        })
        .collect();
    lines.sort_by_key(|l| std::cmp::Reverse((l.0, l.1, l.2)));
    (!lines.is_empty()).then(|| json!(lines.into_iter().map(|l| l.3).collect::<Vec<_>>()))
}

/// BEFORE THE GAME: the contacts and the estimates of men the side was given
/// at the start, as given.
fn before_the_game(ctx: &Ctx<'_>, all: &[SideEvent]) -> Option<Value> {
    let mut ages: BTreeMap<ContactId, u32> = BTreeMap::new();
    for event in all.iter().filter(|e| e.day == 0) {
        if let SideEventKind::Whole(EventKind::ContactAged {
            contact,
            age_pulses,
            ..
        }) = &event.kind
        {
            let age = ages.entry(*contact).or_insert(0);
            *age = (*age).max(*age_pulses);
        }
    }
    let mut contacts: Vec<Value> = Vec::new();
    let mut estimates: Vec<Value> = Vec::new();
    for event in all.iter().filter(|e| e.day == 0) {
        let SideEventKind::Whole(kind) = &event.kind else {
            continue;
        };
        match kind {
            EventKind::ReportReceived {
                contact,
                hex,
                ship_count,
                kind,
                created: true,
                ..
            } => contacts.push(json!({
                "contact": to_json(&ctx.entity(EntityRef::Contact(*contact))),
                "place": to_json(&ctx.place(*hex)),
                "ship_count": ship_count,
                "taken_for": kind.name(),
                "age_pulses": ages.get(contact).copied().unwrap_or(0),
                "event": ctx.number(event.id),
            })),
            EventKind::GroundReportReceived {
                hex, men, grade, ..
            } => estimates.push(json!({
                "place": to_json(&ctx.place(*hex)),
                "men": men,
                "came_from": grade.name(),
                "event": ctx.number(event.id),
            })),
            _ => {}
        }
    }
    let mut part = Map::new();
    if !contacts.is_empty() {
        part.insert("contacts".into(), json!(contacts));
    }
    if !estimates.is_empty() {
        part.insert("estimates".into(), json!(estimates));
    }
    (!part.is_empty()).then_some(Value::Object(part))
}

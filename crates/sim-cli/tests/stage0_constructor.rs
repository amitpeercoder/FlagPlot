//! EXPERIMENT (decision 0100, 14), branch `s0-experiment`, never merged.
//!
//! Builds a whole `World` from one side's `SideView` plus public data and
//! stated assumptions, then resolves days in it. The constructor
//! (`build_from_view`) takes only a `SideView`, the public map and reference
//! data, and the declared assumptions: never the live world or the truth.
//! The test around it reads the live world only to get the view and the
//! side's own scripted orders for the coming days.
//!
//! Run: `cargo test -p sim-cli --test stage0_constructor -- --nocapture`

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use sim_core::belief::Contact;
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::{Hex, HEX_NM};
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, ContactId, LandUnitId, ShipClassId, ShipId, TaskForceId,
};
use sim_core::map::Map;
use sim_core::orders::{AirMission, Mission, Reach};
use sim_core::refdata::{AircraftRole, RefData, ShipKind};
use sim_core::side_events::SideEventKind;
use sim_core::state::{
    AirGroup, AirGroupLocation, Base, LandUnit, Ship, ShipLocation, Side, TaskForce, TruthState,
};
use sim_core::view::SideView;
use sim_core::{SimError, StartingContact, StartingEstimate, World};
use sim_data::Scenario;

/// The public data a constructor may use besides the view.
struct Public {
    map: Map,
    ref_data: RefData,
}

/// What the experiment records as it builds.
#[derive(Default)]
struct Ledger {
    /// Taken from the view, by method.
    from_view: Vec<String>,
    /// Made up. Every one is an assumption.
    assumed: Vec<String>,
    /// Could not be built, or was left out.
    unbuilt: Vec<String>,
    /// Hard cases met in this build.
    hard: Vec<String>,
}

impl Ledger {
    fn view(&mut self, s: impl Into<String>) {
        self.from_view.push(s.into());
    }
    fn assume(&mut self, s: impl Into<String>) {
        self.assumed.push(s.into());
    }
    fn unbuilt(&mut self, s: impl Into<String>) {
        self.unbuilt.push(s.into());
    }
    fn hard(&mut self, s: impl Into<String>) {
        self.hard.push(s.into());
    }
}

/// The declared assumptions that are switches.
#[derive(Clone, Copy)]
struct Switches {
    /// The invented enemy holds a contact on each of the view side's forces
    /// (by hex, most important kind), age 0.
    enemy_knows_us: bool,
    /// Forces the scenario DECLARES the side believes exist and has not seen
    /// (hex, ships, most important kind). The view has no such thing: this is
    /// what the experiment says the scenario must supply.
    declared_unseen: &'static [(i32, i32, u32, ShipKind)],
}

struct Built {
    world: World,
    ledger: Ledger,
}

const INVENTED_FLOOR: u32 = 900_000;

fn other(side: Side) -> Side {
    match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    }
}

fn kind_mission(kind: ShipKind) -> Mission {
    match kind {
        ShipKind::Carrier => Mission::Carrier,
        ShipKind::Battleship | ShipKind::Cruiser | ShipKind::Destroyer => Mission::SurfaceCombat,
        ShipKind::Transport | ShipKind::Auxiliary => Mission::Transport,
    }
}

/// The nearest navigable hex to `hex` (ties to the lowest hex), for a contact
/// whose reported hex is land or off the map.
fn navigable_near(map: &Map, hex: Hex) -> Option<Hex> {
    if map.is_navigable(hex) {
        return Some(hex);
    }
    map.iter()
        .map(|(h, _)| *h)
        .filter(|h| map.is_navigable(*h))
        .min_by_key(|h| (h.distance(hex), *h))
}

/// THE CONSTRUCTOR. Takes the view, the public data, the switches and the
/// seed. It reads no live world.
fn build_from_view(
    view: &SideView<'_>,
    public: &Public,
    switches: Switches,
    seed: u64,
) -> Result<Built, SimError> {
    let mut ledger = Ledger::default();
    let own = view.side();
    let enemy = other(own);
    let rd = &public.ref_data;
    let mut truth = TruthState::default();

    // ---- own side: exact, from the view -------------------------------------
    for ship in view.ships() {
        truth.ships.insert(ship.id, ship.clone());
    }
    for tf in view.task_forces() {
        truth.task_forces.insert(tf.id, tf.clone());
    }
    for base in view.bases() {
        truth.bases.insert(base.id, base.clone());
    }
    for group in view.air_groups() {
        truth.air_groups.insert(group.id, group.clone());
    }
    for unit in view.land_units() {
        truth.land_units.insert(unit.id, unit.clone());
    }
    truth.replacements = view.replacements().cloned().collect();
    truth.arrivals = view.arrivals().cloned().collect();
    ledger.view(
        "ships(), task_forces(), bases(), air_groups(), land_units(), replacements(), arrivals(): the side's own state, whole",
    );
    // Dangling ids: own state cites events and orders of the live log.
    let own_tfs_with_event_refs = truth
        .task_forces
        .values()
        .filter(|tf| tf.alerted_by.is_some() || tf.arrival_event.is_some() || tf.halt.is_some())
        .count();
    ledger.assume(format!(
        "own state cites events/orders of the live log (alerted_by, arrival_event, halt.event, *_order); {own_tfs_with_event_refs} forces cite an event; copied as they are, they point at nothing in the new log"
    ));

    // ---- the enemy's bases: public facts only --------------------------------
    let mut next_ship = INVENTED_FLOOR;
    let mut next_tf = INVENTED_FLOOR;
    let mut next_group = INVENTED_FLOOR;
    let mut next_unit = INVENTED_FLOOR;
    let mut enemy_base_ids: BTreeSet<BaseId> = BTreeSet::new();
    for pb in view.enemy_bases() {
        if truth.bases.contains_key(&pb.id) {
            ledger.unbuilt(format!(
                "ID clash: enemy base {} has the id of an own base",
                pb.id
            ));
            continue;
        }
        enemy_base_ids.insert(pb.id);
        truth.bases.insert(
            pb.id,
            Base {
                id: pb.id,
                name: pb.name.clone(),
                side: pb.side,
                hex: pb.hex,
                supply_t: Fixed::from_int(1000),
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: Fixed::from_int(500),
                ship_repair_points_per_day: Fixed::ZERO,
                yard_passage_days: None,
                on_map: true,
                airfield: true,
                airfield_work: Fixed::ZERO,
                defences: Fixed::ZERO,
                day_work: None,
            },
        );
    }
    ledger.view("enemy_bases(): id, name, hex, side of each enemy base on the map (PublicBase)");
    if !enemy_base_ids.is_empty() {
        ledger.assume(format!(
            "each of the {} enemy bases has supply 1000 t, fuel 500 t, no AA, no repairs, no defences, an airfield (the view gives none of these)",
            enemy_base_ids.len()
        ));
    }
    ledger.unbuilt(
        "enemy air groups at enemy bases: nothing in the view says what is there (none built)",
    );
    ledger.unbuilt("enemy replacement streams and arrivals: not in the view (none built)");
    ledger.unbuilt("enemy bases formed by a landing and not on the map: not in the view at all");

    // ---- which classes and types are the enemy's -----------------------------
    // The reference tables have no side column. "Not a class the side itself
    // uses" is the proxy.
    let own_classes: BTreeSet<ShipClassId> = view.ships().map(|s| s.class).collect();
    let own_types: BTreeSet<AircraftTypeId> = view.air_groups().map(|g| g.aircraft_type).collect();
    let class_for = |kind: ShipKind, ledger: &mut Ledger| -> ShipClassId {
        let of_kind = rd.ship_classes.values().filter(|c| c.kind == kind);
        if let Some(c) = of_kind.clone().find(|c| !own_classes.contains(&c.id)) {
            return c.id;
        }
        let fallback = of_kind
            .map(|c| c.id)
            .next()
            .or_else(|| rd.ship_classes.keys().next().copied())
            .expect("a class exists");
        ledger.hard(format!(
            "no class of kind {} outside the side's own: used class {} (the side's own class) as the enemy's",
            kind.name(),
            fallback
        ));
        fallback
    };
    let type_for = |role: AircraftRole| -> Option<AircraftTypeId> {
        rd.aircraft_types
            .values()
            .filter(|t| t.role == role && !own_types.contains(&t.id))
            .map(|t| t.id)
            .next()
    };

    // ---- the enemy's forces from what the side holds -------------------------
    let current: Vec<&Contact> = view.contacts().collect();
    let mut placed_forces: Vec<(Hex, u32, ShipKind, &'static str)> = Vec::new();
    for c in &current {
        placed_forces.push((
            c.estimated_hex,
            c.estimated_ship_count,
            c.estimated_kind,
            "contact",
        ));
    }
    ledger.view("contacts(): hex, count, kind, age of each contact");
    // A claim of a raid names the kind of ship its pilots believed they hit.
    // If a contact within 2 hexes is of a less important kind, raise it.
    for pf in placed_forces.iter_mut() {
        for cl in view.claims() {
            if let Some(k) = cl.claimed_kind {
                if cl.hex.distance(pf.0) <= 2 && k < pf.2 {
                    ledger.hard(format!(
                        "a contact of {}x{} at {} is raised to kind {} by a day-{} claim of a raid there (a contact of the wrong kind)",
                        pf.1, pf.2.name(), pf.0, k.name(), cl.day
                    ));
                    pf.2 = k;
                }
            }
        }
    }
    for (q, r, n, k) in switches.declared_unseen {
        placed_forces.push((Hex::new(*q, *r), *n, *k, "declared-unseen"));
        ledger.assume(format!("DECLARED by the experiment (the scenario would have to say it): {n} ships, top kind {}, at ({q},{r}), never seen", k.name()));
    }

    // Lost forces: a contact reported and later dropped, from the side's events.
    let mut last_report: BTreeMap<ContactId, (Hex, u32, ShipKind, u32)> = BTreeMap::new();
    let mut dropped: Vec<ContactId> = Vec::new();
    let mut ever_reported: BTreeSet<ContactId> = BTreeSet::new();
    let mut report_events = 0u32;
    for ev in view.events() {
        if let SideEventKind::Whole(kind) = &ev.kind {
            match kind {
                EventKind::ReportReceived {
                    contact,
                    hex,
                    ship_count,
                    kind,
                    ..
                } => {
                    last_report.insert(*contact, (*hex, *ship_count, *kind, ev.day));
                    ever_reported.insert(*contact);
                    report_events += 1;
                }
                EventKind::ContactDropped { contact, .. } => dropped.push(*contact),
                _ => {}
            }
        }
    }
    ledger.view(format!(
        "events(): {report_events} ReportReceived and {} ContactDropped events give the history of contacts (the log is cut to what the side may see)",
        dropped.len()
    ));
    let mut lost_placed = 0;
    let mut lost_deduped = 0;
    for id in &dropped {
        let Some((hex, count, kind, day_last)) = last_report.get(id).copied() else {
            ledger.hard(format!(
                "a dropped contact with no report in the visible log"
            ));
            continue;
        };
        // How far could it have gone since its last report? Cruise 15 kn.
        let days = view.day().saturating_sub(day_last) + 1;
        let reach_hexes = (15 * 24 * days as i64 / HEX_NM) as u32;
        let seen_again = current
            .iter()
            .any(|c| c.estimated_kind == kind && c.estimated_hex.distance(hex) <= reach_hexes);
        if seen_again {
            lost_deduped += 1;
            ledger.hard(format!(
                "two reports of one force (probably): a dropped contact of {} {} last at {} is within reach ({} hexes) of a current contact of the kind; not placed twice",
                count, kind.name(), hex, reach_hexes
            ));
            continue;
        }
        lost_placed += 1;
        placed_forces.push((hex, count, kind, "lost"));
        ledger.assume(format!(
            "lost force: {count} {} last reported at {hex} on day {day_last}, placed AT its last hex, not spread over the {reach_hexes}-hex disc it could have reached (one world holds one place)",
            kind.name()
        ));
    }
    ledger.assume(format!("{lost_placed} lost forces placed, {lost_deduped} dropped as probably the same as a held contact"));

    // Build forces. Ships of the class of the kind, full fuel and ammunition,
    // undamaged, torpedoes ready, no cargo, no troops, default experience.
    for (hex_raw, count, kind, why) in &placed_forces {
        let Some(hex) = navigable_near(&public.map, *hex_raw) else {
            ledger.unbuilt(format!("{why} at {hex_raw}: no navigable hex on the map"));
            continue;
        };
        if hex != *hex_raw {
            ledger.hard(format!(
                "{why} of {count} {} reported at {hex_raw}, which is not navigable (land or off the map): moved to {hex}",
                kind.name()
            ));
        }
        // A contact's kind is the MOST IMPORTANT kind in the group and its
        // count is ALL the ships. So: one ship of the kind, and the rest
        // destroyers, except a transport or auxiliary contact, which is all
        // of the kind (a convoy).
        let top_class = class_for(*kind, &mut ledger);
        let escort_class = if matches!(kind, ShipKind::Transport | ShipKind::Auxiliary) {
            top_class
        } else {
            class_for(ShipKind::Destroyer, &mut ledger)
        };
        if *count > 1
            && !matches!(
                kind,
                ShipKind::Destroyer | ShipKind::Transport | ShipKind::Auxiliary
            )
        {
            ledger.assume(format!(
                "contact of {count} ships of kind {}: 1 {} and {} destroyers (the contact says only the most important kind and the whole count)",
                kind.name(), kind.name(), count - 1
            ));
        }
        let tf_id = TaskForceId::new(next_tf);
        next_tf += 1;
        let mut tf = TaskForce::at(
            tf_id,
            format!("Assumed force ({why}) {tf_id}"),
            enemy,
            hex,
            kind_mission(*kind),
        );
        let mut carrier_capacity = 0u32;
        for n_ship in 0..*count {
            let class_id = if n_ship == 0 { top_class } else { escort_class };
            let class = &rd.ship_classes[&class_id];
            let sid = ShipId::new(next_ship);
            next_ship += 1;
            let mut ship = Ship::new(
                sid,
                format!("Assumed {} {sid}", kind.name()),
                class_id,
                enemy,
                ShipLocation::TaskForce(tf_id),
            );
            ship.fuel_t = class.fuel_capacity_t;
            ship.gun_rounds =
                Fixed::from_int(i64::from(class.main_guns) * i64::from(class.gun_rounds_per_gun));
            ship.torpedoes_ready = class.torpedo_tubes > 0;
            tf.ships.insert(sid);
            carrier_capacity += class.aircraft_capacity;
            truth.ships.insert(sid, ship);
            // Aircraft aboard a carrier: a fighter group and a bomber group.
            if class.aircraft_capacity > 0 {
                let (fighters, bombers) = (
                    class.aircraft_capacity / 3,
                    class.aircraft_capacity - class.aircraft_capacity / 3,
                );
                for (role, n) in [
                    (AircraftRole::Fighter, fighters),
                    (AircraftRole::Bomber, bombers),
                ] {
                    let Some(ty) = type_for(role) else {
                        ledger.unbuilt(format!(
                            "no enemy aircraft type of role {role:?}: no group aboard {sid}"
                        ));
                        continue;
                    };
                    let gid = AirGroupId::new(next_group);
                    next_group += 1;
                    truth.air_groups.insert(
                        gid,
                        AirGroup {
                            id: gid,
                            name: format!("Assumed {role:?} group {gid}"),
                            side: enemy,
                            location: AirGroupLocation::Ship(sid),
                            aircraft_type: ty,
                            ready: n,
                            damaged: 0,
                            mission: AirMission::Rest,
                            search: None,
                            mission_order: None,
                            arc_order: None,
                            patrol_share: Fixed::ONE,
                            share_order: None,
                            fatigue: Fixed::ZERO,
                            rest_above: None,
                            rest_order: None,
                            stood_down: false,
                            pilots: None,
                            strength: n,
                            reach: Reach::Normal,
                            reach_order: None,
                            target_priority: ShipKind::ALL.to_vec(),
                            target_order: None,
                            last_base_strike_day: None,
                            rebased_day: None,
                        },
                    );
                }
            }
        }
        let _ = carrier_capacity;
        truth.task_forces.insert(tf_id, tf);
    }
    ledger.assume("class of a kind: the lowest-id class of that kind that the side does not itself use (the class tables have no side column); falls back to the side's own class when there is none");
    ledger.assume("each invented ship: full fuel, full gun rounds, torpedoes ready, undamaged, no cargo, no troops, experience 2");
    ledger.assume("each invented carrier carries one fighter group (1/3 of capacity) and one bomber group (2/3), ready, no damage, pilots not counted, mission Rest, fatigue 0, enemy type = lowest-id type of the role the side does not itself use");
    ledger.assume("invented forces: mission by kind (carrier -> Carrier, gunships -> SurfaceCombat, transport/auxiliary -> Transport), readiness Normal, no route, no orders, no home port");
    ledger.assume(format!(
        "invented ids start at {INVENTED_FLOOR} (ships, forces, groups, units): above every scenario number and below the made-in-play floor"
    ));

    // The enemy's men ashore, from the side's own estimates.
    let mut estimates: Vec<StartingEstimate> = Vec::new();
    for e in view.ground_estimates() {
        estimates.push(StartingEstimate {
            side: own,
            hex: e.hex,
            men: e.men,
        });
        let uid = LandUnitId::new(next_unit);
        next_unit += 1;
        truth.land_units.insert(
            uid,
            LandUnit::new(
                uid,
                format!("Assumed garrison {uid}"),
                enemy,
                e.men,
                Some(e.hex),
            ),
        );
        ledger.assume(format!(
            "enemy men ashore at {}: one unit of {} men (the estimate of day {}), posture defend, default skills",
            e.hex, e.men, e.day
        ));
    }
    if rd.ground.is_none() && !estimates.is_empty() {
        ledger.unbuilt("estimates of men ashore but no ground figures");
    }
    ledger.view("ground_estimates(): hex, men, day of the side's estimates of the enemy ashore");

    // ---- the world ------------------------------------------------------------
    let mut world = World::new(seed, rd.clone(), public.map.clone(), truth)?;

    // The side's own belief: contacts re-entered as starting contacts (this
    // needs day 0), estimates likewise.
    let mut starting: Vec<StartingContact> = current
        .iter()
        .map(|c| StartingContact {
            side: own,
            hex: c.estimated_hex,
            ship_count: c.estimated_ship_count,
            kind: c.estimated_kind,
            age_pulses: c.age_pulses,
        })
        .collect();
    // What the invented enemy believes of us.
    if switches.enemy_knows_us {
        let mut by_hex: BTreeMap<Hex, (u32, ShipKind)> = BTreeMap::new();
        for tf in view.task_forces() {
            let kinds: Vec<ShipKind> = tf
                .ships
                .iter()
                .filter_map(|s| view.ships().find(|x| x.id == *s))
                .filter_map(|s| view.ship_class(s.class))
                .map(|c| c.kind)
                .collect();
            let Some(top) = kinds.iter().min().copied() else {
                continue;
            };
            let entry = by_hex.entry(tf.position()).or_insert((0, top));
            entry.0 += kinds.len() as u32;
            entry.1 = entry.1.min(top);
        }
        // Merge forces closer than the search figures' merge distance into one
        // contact, as the world itself would.
        let merge = rd.search.as_ref().map_or(0, |f| f.merge_distance_hexes);
        let mut clusters: Vec<(Hex, u32, ShipKind)> = Vec::new();
        for (hex, (n, kind)) in by_hex {
            if let Some(c) = clusters.iter_mut().find(|c| c.0.distance(hex) <= merge * 2) {
                c.1 += n;
                c.2 = c.2.min(kind);
            } else {
                clusters.push((hex, n, kind));
            }
        }
        let by_hex: Vec<(Hex, (u32, ShipKind))> =
            clusters.into_iter().map(|(h, n, k)| (h, (n, k))).collect();
        for (hex, (n, kind)) in by_hex {
            starting.push(StartingContact {
                side: enemy,
                hex,
                ship_count: n,
                kind,
                age_pulses: 0,
            });
        }
        ledger.assume("the invented enemy holds an age-0 contact on each of the side's forces (by hex, most important kind, true count): the worst case for the side");
    } else {
        ledger.assume("the invented enemy holds no contacts: it knows nothing of the side");
    }
    // A contact that merges with another (as the search figures merge) is an error here.
    let before = starting.len();
    match world.hold_starting_contacts(starting.clone()) {
        Ok(()) => {}
        Err(e) => {
            ledger.hard(format!(
                "hold_starting_contacts refused the {before} contacts: {e}; retried one by one, dropping those refused"
            ));
            // Retry: add one at a time, dropping the ones that are refused.
            let mut w2 = World::new(seed, rd.clone(), public.map.clone(), world.umpire().clone())?;
            let mut kept = Vec::new();
            for sc in starting {
                let mut trial = kept.clone();
                trial.push(sc);
                let mut probe = w2.clone();
                if probe.hold_starting_contacts(trial.clone()).is_ok() {
                    kept = trial;
                } else {
                    ledger.hard(format!(
                        "dropped contact at {} (count {}, {} pulses old)",
                        sc.hex, sc.ship_count, sc.age_pulses
                    ));
                }
            }
            w2.hold_starting_contacts(kept)?;
            world = w2;
        }
    }
    world.hold_starting_estimates(estimates)?;
    ledger.assume("own contacts: re-entered through hold_starting_contacts at day 0 (ages kept, creation order = oldest first, so contact numbers are NOT the live numbers); their events are logged on day 0");
    ledger
        .assume("own ground estimates: re-entered as opening estimates: their day is lost (day 0)");
    world.experiment_set_day(view.day());
    ledger.assume("the world's day is set to the view's day by a helper added on this branch (World has no public way: a game that has begun cannot take starting contacts)");

    // Claims.
    let n_claims = view.claims().len();
    for claim in view.claims() {
        world.experiment_hold_claim(own, claim);
    }
    ledger.view(format!(
        "claims(): {n_claims} battle claims, hex, day, pulse, counts, kind"
    ));
    ledger.assume("claims re-entered by a branch-only helper (no public way to write a claim into a world's belief)");

    // Objectives: the side's own only.
    ledger.view(format!(
        "mission(), objectives(), levels(), horizon_days(): own mission {:?}, {} own objectives, levels {:?}, horizon {:?}",
        view.mission().is_some(),
        view.objectives().len(),
        view.levels().is_some(),
        view.horizon_days()
    ));
    ledger.unbuilt("Objectives for World::set_objectives: the enemy's SideObjectives are not in the view; the world is left with no objectives (so result() is unavailable)");
    ledger.assume(format!("seed {seed}: the view has no seed; the caller supplies one (a trial must key it on permitted inputs)"));
    ledger.assume("the event log of the new world holds only day-0 belief events (the view's events cannot be replayed into a log); the live log's ids in own state dangle");
    ledger.assume("orders: the live order book is not rebuilt (OrderId is opaque; no way to make one); own standing orders live in the state (missions, routes, *_order ids) and continue; `pending` orders are the caller's");

    Ok(Built { world, ledger })
}

// ---------------------------------------------------------------------------
// the test around it
// ---------------------------------------------------------------------------

fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn summary(world: &World, side: Side) -> String {
    let v = world.side_view(side);
    let ships = v
        .ships()
        .filter(|s| s.location != ShipLocation::Sunk)
        .count();
    let ready: u32 = v.air_groups().map(|g| g.ready).sum();
    let enemy_ships = world
        .umpire()
        .ships
        .values()
        .filter(|s| s.side != side && s.location != ShipLocation::Sunk)
        .count();
    format!(
        "day {}: own afloat {ships}, own ready aircraft {ready}, contacts {}, claims {}, (umpire) enemy afloat {enemy_ships}",
        v.day(),
        v.contacts().count(),
        v.claims().len()
    )
}

#[test]
fn constructor_experiment() {
    let path = root().join("data/scenarios/07_full_slice.toml");
    // Public data: the map and the reference data of a freshly loaded scenario.
    let fresh = Scenario::load(&path).expect("loads");
    let public = Public {
        map: fresh.world.map().clone(),
        ref_data: fresh.world.ref_data().clone(),
    };
    let mut live = Scenario::load(&path).expect("loads");

    let checkpoints = [2u32, 3, 4, 12];
    let mut report = String::new();
    for &cp in &checkpoints {
        while live.world.day() < cp {
            live.next_day().expect("day");
        }
        for side in Side::ALL {
            for enemy_knows_us in [false, true] {
                if enemy_knows_us && cp != 3 {
                    continue;
                }
                let (built, build_err) = {
                    let view = live.world.side_view(side);
                    match build_from_view(
                        &view,
                        &public,
                        Switches {
                            enemy_knows_us,
                            declared_unseen: &[],
                        },
                        4242,
                    ) {
                        Ok(b) => (Some(b), None),
                        Err(e) => (None, Some(e.to_string())),
                    }
                };
                report.push_str(&format!(
                    "\n==== BUILD: view of {} after day {cp}, enemy_knows_us={enemy_knows_us}\n",
                    side.name()
                ));
                let Some(mut built) = built else {
                    report.push_str(&format!("BUILD FAILED: {}\n", build_err.unwrap()));
                    continue;
                };
                let l = &built.ledger;
                report.push_str(&format!("from_view: {}\n", l.from_view.len()));
                for s in &l.assumed {
                    report.push_str(&format!("  assumed: {s}\n"));
                }
                for s in &l.unbuilt {
                    report.push_str(&format!("  unbuilt: {s}\n"));
                }
                for s in &l.hard {
                    report.push_str(&format!("  hard:    {s}\n"));
                }
                report.push_str(&format!(
                    "  built world: {} ships, {} forces, {} bases, {} air groups, {} land units; {}; log {} events\n",
                    built.world.umpire().ships.len(),
                    built.world.umpire().task_forces.len(),
                    built.world.umpire().bases.len(),
                    built.world.umpire().air_groups.len(),
                    built.world.umpire().land_units.len(),
                    summary(&built.world, side),
                    built.world.experiment_event_count()
                ));
                // Resolve two days in the built world: the side's own scripted
                // orders for those days (its own standing instructions), none
                // for the invented enemy.
                for d in [cp + 1, cp + 2] {
                    if d > live.days {
                        break;
                    }
                    let own_orders: Vec<_> = live
                        .scripted_orders()
                        .get(&d)
                        .map(|v| {
                            v.iter()
                                .filter(|(s, _)| *s == side)
                                .map(|(_, r)| r.clone())
                                .collect()
                        })
                        .unwrap_or_default();
                    let n = own_orders.len();
                    built.world.submit_orders(side, own_orders);
                    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        built.world.resolve_day()
                    }));
                    match res {
                        Ok(Ok(s)) => report.push_str(&format!(
                            "  resolved day {d} with {n} own scripted orders: ok, {} events, {}\n",
                            s.event_count,
                            summary(&built.world, side)
                        )),
                        Ok(Err(e)) => {
                            report.push_str(&format!("  resolved day {d}: ERROR {e}\n"));
                            break;
                        }
                        Err(p) => {
                            let msg = p
                                .downcast_ref::<String>()
                                .cloned()
                                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                                .unwrap_or_default();
                            report.push_str(&format!("  resolved day {d}: PANIC {msg}\n"));
                            break;
                        }
                    }
                }
                // The live world's next two days, for comparison (own view).
                if !enemy_knows_us {
                    let mut l2 = live.clone();
                    for _ in 0..2 {
                        if l2.next_day().ok().flatten().is_none() {
                            break;
                        }
                    }
                    report.push_str(&format!(
                        "  LIVE, same two days: {}\n",
                        summary(&l2.world, side)
                    ));
                }
            }
        }
        // View summary of what each side holds at this day.
        for side in Side::ALL {
            let v = live.world.side_view(side);
            report.push_str(&format!(
                "\n[view of {} after day {cp}] contacts {:?}; claims {}; ground estimates {}; enemy bases {}; visible events {}\n",
                side.name(),
                v.contacts()
                    .map(|c| format!("{}x{} at {} age {}", c.estimated_ship_count, c.estimated_kind.name(), c.estimated_hex, c.age_pulses))
                    .collect::<Vec<_>>(),
                v.claims().len(),
                v.ground_estimates().count(),
                v.enemy_bases().count(),
                v.events().len()
            ));
        }
    }
    println!("{report}");
    std::fs::write("/tmp/stage0_constructor_report.txt", &report).ok();
}

/// Referee (the umpire's truth; the constructor never sees it): how far the
/// invented enemy is from the real one, and the hard cases in the slice.
#[test]
fn referee_what_the_constructor_missed() {
    let path = root().join("data/scenarios/07_full_slice.toml");
    let mut live = Scenario::load(&path).expect("loads");
    let mut report = String::new();
    for cp in [2u32, 3, 4, 12] {
        while live.world.day() < cp {
            live.next_day().expect("day");
        }
        let truth = live.world.umpire();
        for side in Side::ALL {
            let enemy = other(side);
            let rd = live.world.ref_data();
            let mut by_kind: BTreeMap<&str, u32> = BTreeMap::new();
            for sh in truth
                .ships
                .values()
                .filter(|s| s.side == enemy && s.location != ShipLocation::Sunk)
            {
                *by_kind
                    .entry(rd.ship_classes[&sh.class].kind.name())
                    .or_default() += 1;
            }
            let aircraft: u32 = truth
                .air_groups
                .values()
                .filter(|g| g.side == enemy)
                .map(|g| g.ready + g.damaged)
                .sum();
            let aboard: u32 = truth
                .air_groups
                .values()
                .filter(|g| g.side == enemy && matches!(g.location, AirGroupLocation::Ship(_)))
                .map(|g| g.ready + g.damaged)
                .sum();
            let sunk_by_claims: u32 = live
                .world
                .side_view(side)
                .claims()
                .iter()
                .map(|c| c.claimed_sunk)
                .sum();
            let truly_sunk = truth
                .ships
                .values()
                .filter(|s| s.side == enemy && s.location == ShipLocation::Sunk)
                .count();
            report.push_str(&format!(
                "[truth after day {cp}, enemy of {}] afloat by kind {by_kind:?}; aircraft {aircraft} ({aboard} aboard ships); enemy ships truly sunk {truly_sunk}, claimed sunk {sunk_by_claims}\n",
                side.name()
            ));
            for c in live.world.side_view(side).contacts() {
                let mut near: BTreeMap<&str, u32> = BTreeMap::new();
                for sh in truth
                    .ships
                    .values()
                    .filter(|s| s.side == enemy && s.location != ShipLocation::Sunk)
                {
                    if let ShipLocation::TaskForce(tf) = sh.location {
                        if truth.task_forces[&tf].position().distance(c.estimated_hex) <= 2 {
                            *near
                                .entry(rd.ship_classes[&sh.class].kind.name())
                                .or_default() += 1;
                        }
                    }
                }
                report.push_str(&format!(
                    "    contact {}x{} at {} age {}: true ships within 2 hexes {near:?}\n",
                    c.estimated_ship_count,
                    c.estimated_kind.name(),
                    c.estimated_hex,
                    c.age_pulses
                ));
            }
            for cl in live.world.side_view(side).claims() {
                report.push_str(&format!(
                    "    claim day {} at {}: sunk {}, hits {}, kind {:?}, air {}\n",
                    cl.day,
                    cl.hex,
                    cl.claimed_sunk,
                    cl.claimed_hits,
                    cl.claimed_kind.map(|k| k.name()),
                    cl.from_air
                ));
            }
        }
    }
    println!("{report}");
    std::fs::write("/tmp/stage0_referee_report.txt", &report).ok();
}

/// Leak probes: does any method of the view hand over enemy truth? Truth is
/// used here only to know which ids are the enemy's.
#[test]
fn leak_probes() {
    use sim_core::choices::air::AirQuestion;
    use sim_core::choices::ground::GroundQuestion;
    use sim_core::choices::movement::MovementQuestion;
    use sim_core::choices::surface::SurfaceQuestion;
    use sim_core::choices::Question;
    use sim_core::orders::SpeedSetting;
    let path = root().join("data/scenarios/07_full_slice.toml");
    let mut live = Scenario::load(&path).expect("loads");
    let mut report = String::new();
    for cp in [4u32, 12] {
        while live.world.day() < cp {
            live.next_day().expect("day");
        }
        for side in Side::ALL {
            let enemy = other(side);
            let truth = live.world.umpire();
            let view = live.world.side_view(side);
            // 1. Enemy ids in what the view shows.
            let mut needles: Vec<String> = Vec::new();
            for sh in truth.ships.values().filter(|s| s.side == enemy) {
                needles.push(format!("ShipId({})", sh.id.raw()));
                needles.push(format!("ship:{}", sh.id.raw()));
            }
            for tf in truth.task_forces.values().filter(|s| s.side == enemy) {
                needles.push(format!("TaskForceId({})", tf.id.raw()));
                needles.push(format!("tf:{}", tf.id.raw()));
            }
            for g in truth.air_groups.values().filter(|s| s.side == enemy) {
                needles.push(format!("AirGroupId({})", g.id.raw()));
                needles.push(format!("group:{}", g.id.raw()));
            }
            let mut hits = 0;
            for ev in view.events() {
                let text = format!("{ev:?}");
                for n in &needles {
                    if text.contains(n.as_str()) {
                        hits += 1;
                        report.push_str(&format!(
                            "  LEAK? event {} contains {n}: {}\n",
                            ev.number,
                            &text[..text.len().min(160)]
                        ));
                        break;
                    }
                }
            }
            report.push_str(&format!(
                "[{} day {cp}] enemy-id needles in {} visible events: {hits}\n",
                side.name(),
                view.events().len()
            ));
            // Enemy NAMES in events? (names of enemy ships/forces)
            let mut name_hits = 0;
            for ev in view.events() {
                let text = format!("{ev:?}");
                for sh in truth.ships.values().filter(|s| s.side == enemy) {
                    if text.contains(&sh.name) {
                        name_hits += 1;
                        break;
                    }
                }
            }
            report.push_str(&format!(
                "   enemy ship NAMES in visible events: {name_hits}\n"
            ));
            // 2. Ask probes with enemy ids.
            let mut answered = 0;
            let mut seen_labels: BTreeSet<String> = BTreeSet::new();
            let mut ask = |q: Question, label: &str, report: &mut String| {
                if let Some(a) = view.ask(&q) {
                    answered += 1;
                    let key = format!("{q:?}")
                        .split(|c| c == '{' || c == '(')
                        .take(3)
                        .collect::<Vec<_>>()
                        .join("/");
                    if seen_labels.insert(key.clone()) {
                        report.push_str(&format!("  LEAK? ask({label}) {key} answered: {a:?}\n"));
                    }
                }
            };
            let own_ship = view.ships().next().map(|s| s.id);
            let own_group = view.air_groups().next().map(|g| g.id);
            for g in truth.air_groups.values().filter(|s| s.side == enemy) {
                for q in [
                    AirQuestion::Strike { group: g.id },
                    AirQuestion::AircraftAbleToFly { group: g.id },
                    AirQuestion::BaseStrike {
                        group: g.id,
                        pulse: sim_core::time::Pulse::Morning,
                    },
                    AirQuestion::FighterSplit { group: g.id },
                    AirQuestion::Escort { group: g.id },
                    AirQuestion::Rest { group: g.id },
                    AirQuestion::Landing {
                        groups: vec![(g.id, 5)],
                    },
                ] {
                    ask(Question::Air(q), "air, enemy group", &mut report);
                }
                if let Some(og) = own_group {
                    ask(
                        Question::Air(AirQuestion::Landing {
                            groups: vec![(og, 5), (g.id, 5)],
                        }),
                        "air landing, own+enemy group",
                        &mut report,
                    );
                }
                let _ = view
                    .strike_choice(g.id)
                    .map(|_| report.push_str("  LEAK? strike_choice enemy group\n"));
            }
            for tf in truth.task_forces.values().filter(|s| s.side == enemy) {
                for q in [
                    SurfaceQuestion::Alert { task_force: tf.id },
                    SurfaceQuestion::Ready {
                        task_force: tf.id,
                        day: 5,
                        pulse: sim_core::time::Pulse::Night,
                    },
                    SurfaceQuestion::Stragglers { task_force: tf.id },
                    SurfaceQuestion::HeadHome { task_force: tf.id },
                    SurfaceQuestion::BreakOff {
                        task_force: tf.id,
                        began_with: tf.ships.iter().copied().collect(),
                        fired_on: false,
                        already_avoiding: false,
                    },
                ] {
                    ask(Question::Surface(q), "surface, enemy force", &mut report);
                }
                for q in [
                    MovementQuestion::Reroute {
                        force: tf.id,
                        first_waypoint: None,
                    },
                    MovementQuestion::Halt { force: tf.id },
                    MovementQuestion::SteamOn { force: tf.id },
                ] {
                    ask(Question::Movement(q), "movement, enemy force", &mut report);
                }
                if view
                    .forecast_passage(tf.id, tf.position(), Some(SpeedSetting::Cruise))
                    .is_some()
                {
                    report.push_str("  LEAK? forecast_passage enemy force\n");
                }
                if view.yard_plan(tf.id).is_some() {
                    report.push_str("  LEAK? yard_plan enemy force\n");
                }
                if view.landing_rate_factor(tf.id, tf.position()).is_some() {
                    report.push_str("  LEAK? landing_rate_factor enemy force\n");
                }
                if let Some(os) = own_ship {
                    // an own force asked with one enemy ship among those it "began with"
                    if let (Some(own_tf), false) = (view.task_forces().next(), tf.ships.is_empty())
                    {
                        let mut began: Vec<_> = own_tf.ships.iter().copied().collect();
                        began.push(tf.ships.iter().copied().next().unwrap_or(os));
                        ask(
                            Question::Surface(SurfaceQuestion::BreakOff {
                                task_force: own_tf.id,
                                began_with: began,
                                fired_on: false,
                                already_avoiding: false,
                            }),
                            "surface break-off, own force + one enemy ship",
                            &mut report,
                        );
                    }
                }
            }
            for u in truth.land_units.values().filter(|s| s.side == enemy) {
                ask(
                    Question::Ground(GroundQuestion::BackToDefend {
                        unit: u.id,
                        fatigue: Fixed::ZERO,
                    }),
                    "ground back-to-defend enemy unit",
                    &mut report,
                );
                ask(
                    Question::Ground(GroundQuestion::UnitDay {
                        unit: u.id,
                        marched: false,
                        contested: false,
                        rostered_with: None,
                    }),
                    "ground unit day, enemy unit",
                    &mut report,
                );
                if view.men_aboard(u.id).is_some()
                    || view.march_destinations(u.id).is_some()
                    || view.daily_charge_t(u.id).is_some()
                    || view.roster_of(u.id).is_some()
                {
                    report.push_str("  LEAK? ground method answered for an enemy unit\n");
                }
            }
            // A ground UnitDay for an own unit rostered with an enemy force.
            if let (Some(u), Some(tf)) = (
                view.land_units().next(),
                truth.task_forces.values().find(|t| t.side == enemy),
            ) {
                ask(
                    Question::Ground(GroundQuestion::UnitDay {
                        unit: u.id,
                        marched: false,
                        contested: false,
                        rostered_with: Some(tf.id),
                    }),
                    "ground unit day, own unit rostered with an ENEMY force",
                    &mut report,
                );
            }
            report.push_str(&format!(
                "   ask probes answered for enemy ids: {answered}\n"
            ));
            // 3. Things the view hands over that could carry truth about the enemy.
            let nb = view.enemy_bases().count();
            let on_map_truth = truth
                .bases
                .values()
                .filter(|b| b.side == enemy && b.on_map)
                .count();
            let off_map_truth = truth
                .bases
                .values()
                .filter(|b| b.side == enemy && !b.on_map)
                .count();
            report.push_str(&format!(
                "   enemy_bases() {nb} (truth: {on_map_truth} on map, {off_map_truth} off map)\n"
            ));
        }
    }
    println!("{report}");
    std::fs::write("/tmp/stage0_leak_report.txt", &report).ok();
}

/// The enemy carrier never seen: the slice's Allied view after day 3 holds no
/// contact, and the Japanese carriers strike on day 4. Declare an unseen
/// force, as a scenario would have to, and see what the built world does.
#[test]
fn declared_unseen_carrier() {
    let path = root().join("data/scenarios/07_full_slice.toml");
    let fresh = Scenario::load(&path).expect("loads");
    let public = Public {
        map: fresh.world.map().clone(),
        ref_data: fresh.world.ref_data().clone(),
    };
    let mut live = Scenario::load(&path).expect("loads");
    while live.world.day() < 3 {
        live.next_day().expect("day");
    }
    let rabaul = live
        .world
        .side_view(Side::Allied)
        .enemy_bases()
        .next()
        .expect("a base")
        .hex;
    println!("public: enemy base hex {rabaul}");
    let mut report = String::new();
    // Declared places: at the enemy base; and 3 hexes from the real battle hex.
    let cases: [(&str, &'static [(i32, i32, u32, ShipKind)]); 3] = [
        ("none", &[]),
        ("at the base (-5,?)", &[(-5, 0, 6, ShipKind::Carrier)]),
        ("near the real fleet", &[(7, -12, 17, ShipKind::Carrier)]),
    ];
    for (label, declared) in cases {
        let declared: &'static [(i32, i32, u32, ShipKind)] = if label.starts_with("at the base") {
            // Use the public base hex.
            Box::leak(Box::new([(rabaul.q, rabaul.r, 6u32, ShipKind::Carrier)]))
        } else {
            declared
        };
        let built = {
            let view = live.world.side_view(Side::Allied);
            build_from_view(
                &view,
                &public,
                Switches {
                    enemy_knows_us: false,
                    declared_unseen: declared,
                },
                4242,
            )
        };
        let mut built = built.expect("builds");
        for d in [4u32, 5] {
            let own: Vec<_> = live
                .scripted_orders()
                .get(&d)
                .map(|v| {
                    v.iter()
                        .filter(|(s, _)| *s == Side::Allied)
                        .map(|(_, r)| r.clone())
                        .collect()
                })
                .unwrap_or_default();
            built.world.submit_orders(Side::Allied, own);
            let r = built.world.resolve_day();
            let v = built.world.side_view(Side::Allied);
            report.push_str(&format!(
                "[declared: {label}] day {d}: {:?}; {}; contacts {:?}; claims {}\n",
                r.map(|s| s.event_count).map_err(|e| e.to_string()),
                summary(&built.world, Side::Allied),
                v.contacts()
                    .map(|c| format!(
                        "{}x{} at {}",
                        c.estimated_ship_count,
                        c.estimated_kind.name(),
                        c.estimated_hex
                    ))
                    .collect::<Vec<_>>(),
                v.claims().len()
            ));
        }
    }
    println!("{report}");
    std::fs::write("/tmp/stage0_declared_report.txt", &report).ok();
}

/// One look at the truth near (0,10) on day 9, where the Japanese view holds a
/// "carrier" contact.
#[test]
fn referee_wrong_kind_near_0_10() {
    let path = root().join("data/scenarios/07_full_slice.toml");
    let mut live = Scenario::load(&path).expect("loads");
    let mut out = String::new();
    while live.world.day() < 9 {
        live.next_day().expect("day");
    }
    let truth = live.world.umpire();
    let rd = live.world.ref_data();
    for sh in truth
        .ships
        .values()
        .filter(|s| s.location != ShipLocation::Sunk)
    {
        if let ShipLocation::TaskForce(tf) = sh.location {
            let pos = truth.task_forces[&tf].position();
            if pos.distance(Hex::new(0, 10)) <= 2 {
                out.push_str(&format!(
                    "{} {} at {}\n",
                    sh.side.name(),
                    rd.ship_classes[&sh.class].kind.name(),
                    pos
                ));
            }
        }
    }
    let v = live.world.side_view(Side::Japanese);
    for c in v.contacts() {
        out.push_str(&format!(
            "Japanese contact {}x{} at {} age {}\n",
            c.estimated_ship_count,
            c.estimated_kind.name(),
            c.estimated_hex,
            c.age_pulses
        ));
    }
    println!("{out}");
    std::fs::write("/tmp/stage0_wrongkind.txt", &out).ok();
}

/// Made-in-play ids (>= 1,000,000) in a side's own state: a fresh world's
/// counters start at the floor again.
#[test]
fn made_ids_in_own_state() {
    let path = root().join("data/scenarios/07_full_slice.toml");
    let mut live = Scenario::load(&path).expect("loads");
    let mut out = String::new();
    for cp in [2u32, 4, 8, 12, 13] {
        while live.world.day() < cp {
            live.next_day().expect("day");
        }
        for side in Side::ALL {
            let v = live.world.side_view(side);
            let tfs: Vec<u32> = v
                .task_forces()
                .map(|t| t.id.raw())
                .filter(|r| *r >= 1_000_000)
                .collect();
            let gs: Vec<u32> = v
                .air_groups()
                .map(|g| g.id.raw())
                .filter(|r| *r >= 1_000_000)
                .collect();
            out.push_str(&format!(
                "day {cp} {}: made task forces {tfs:?}, made air groups {gs:?}\n",
                side.name()
            ));
        }
    }
    println!("{out}");
}

//! The state hash.
//!
//! One 64-bit number that summarises the whole game state at the end of a
//! day. Two runs with the same starting state, orders and seed must produce
//! the same hash on any machine. A replay that diverges shows up as the first
//! day whose hash differs.
//!
//! The hash is FNV-1a over a hand-written, fixed-order walk of the state.
//! It detects divergence. It is not a security feature. See
//! `docs/decisions/0004-state-hash.md`.

use crate::belief::{BattleClaim, Contact, GroundEstimate};
use crate::command::{
    Addressee, Amendment, AmendmentChange, Baseline, CommandState, Commitment, ForcesLimit, Hold,
    HoldMaker, HoldTarget, Intent, IntentKind, IntentLimits, InterruptCondition, InterruptStatus,
    Leaves, Limit, Maker, Plan, Run, RunDays, SetInterrupt, StandingSubject,
};
use crate::events::Cause;
use crate::ids::{BaseId, IntentId};
use crate::orders::{Activity, AirMission, Job, Mission, SpeedSetting, WhenSpent};
use crate::refdata::ShipKind;
use crate::state::Side;
use crate::state::{
    AirGroup, AirGroupLocation, Arrival, Base, CargoOrder, DayWork, Halt, HaltWork, HitsOnMen,
    LandUnit, March, Preparation, Replacement, Route, RulesOfEngagement, SearchArc, Ship,
    ShipLocation, ShipSystem, ShipSystems, Skills, TaskForce, Troops, TruthState, YardTrip,
};
use crate::world::World;

const FNV_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;

/// Feeds values into an FNV-1a hash in a fixed byte order.
#[derive(Clone, Debug)]
pub struct StateHasher(u64);

impl StateHasher {
    pub const fn new() -> StateHasher {
        StateHasher(FNV_OFFSET)
    }

    pub fn bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(FNV_PRIME);
        }
    }

    pub fn u8(&mut self, value: u8) {
        self.bytes(&[value]);
    }

    pub fn u32(&mut self, value: u32) {
        self.bytes(&value.to_le_bytes());
    }

    pub fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes());
    }

    pub fn i32(&mut self, value: i32) {
        self.bytes(&value.to_le_bytes());
    }

    pub fn i64(&mut self, value: i64) {
        self.bytes(&value.to_le_bytes());
    }

    /// A length or count. Always written as 64 bits so the hash does not
    /// depend on the machine's pointer width.
    pub fn count(&mut self, value: usize) {
        self.u64(u64::try_from(value).expect("count fits in 64 bits"));
    }

    /// An optional raw ID: a presence byte, then the value.
    pub fn optional_u32(&mut self, value: Option<u32>) {
        match value {
            Some(raw) => {
                self.u8(1);
                self.u32(raw);
            }
            None => self.u8(0),
        }
    }

    pub const fn finish(&self) -> u64 {
        self.0
    }
}

impl Default for StateHasher {
    fn default() -> Self {
        Self::new()
    }
}

const fn mission_code(mission: Mission) -> u8 {
    match mission {
        Mission::Transport => 1,
        Mission::SurfaceCombat => 2,
        Mission::Carrier => 3,
        Mission::Bombardment => 4,
        Mission::Escort => 5,
    }
}

const fn speed_code(speed: SpeedSetting) -> u8 {
    match speed {
        SpeedSetting::Cruise => 1,
        SpeedSetting::Full => 2,
        SpeedSetting::Knots(_) => 3,
    }
}

/// The first bytes hashed: the version of the hash's layout. It is raised
/// whenever what is hashed, or its order, changes, and the golden files are
/// remade in the same change (decision 9 of 0055). Version 1 kept old hashes
/// alive by writing a new field only where it was in use, behind a tag byte;
/// version 2 writes every field of every unit, always, in one order;
/// version 3 adds a task force's `leg_nm` after its `progress_nm` (decision 0058);
/// version 4 adds an air group's `reach` and `reach_order` after its
/// `arc_order` (decision 0059);
/// version 5 adds an air group's `target_priority`, `target_order` and
/// `last_base_strike_day` after its `reach_order` (decision 0063);
/// version 6 adds a ship's `experience` after its location (decision 0064);
/// version 7 adds a ship's `torpedoes_one_side_fired` after its
/// `torpedoes_ready` (decision 0083);
/// version 8 replaces the one task force counter with a counter for each side
/// and each kind of unit made in play, hashed Allied then Japanese, task
/// forces then air groups (decision 0086);
/// version 9 adds a base's `ship_repair_points_per_day` and `yard_passage_days`
/// after its `fuel_t`, a ship's `system_repair_points` after its `experience`,
/// the ship location `Yard` (code 4), and a task force's `yard_trip` after its
/// `fatigue` (decision 0069);
/// version 10 adds a ship's `troops` after its `system_repair_points`, a
/// base's `on_map` after its `yard_passage_days`, a cargo order's `unit` after
/// its order and the cargo action `LoadNoCargo` (code 4), and the land units
/// after the arrivals (decision 0066);
/// version 11 adds an air group's `rebased_day` after its
/// `last_base_strike_day` (decision 0091);
/// version 12 adds a land unit's `posture`, `posture_order`, `fatigue` and
/// `fed_share` after its hex, and the air group location `Lost` (code 3)
/// (decision 0090);
/// version 13 adds a claim's `from_bombardment` after its `from_air`, and a
/// side's ground estimates after its claims (decision 0092);
/// version 14 adds a land unit's `march` (its hex, days marched and order)
/// after its `fed_share` (decision 0094);
/// version 15 adds a route's `act_at` after its `next_index`, the task force
/// status `Halted` (code 5), and a task force's `halt` after its `yard_trip`
/// (decision 0096);
/// version 16 adds a task force's `preparation` and `landing_at` after its
/// `halt`, and a land unit's `rehearsal`, `rehearsal_hex`, `activity`,
/// `activity_order`, `experience`, `training_days` and `fight_days` after
/// its `march` (decisions 0095 and 0097);
/// version 17 adds a march's `counted` after its `days`, and a land unit's
/// `skills` after its `fight_days` (decision 0098);
/// version 18 adds a base's `airfield`, `airfield_work`, `defences` and
/// `day_work` after its `on_map`, the activity `Work` (code 2) with its job
/// after the activity's code, a skill's `works` after its `atoll`, and a land
/// unit's `working` after its skills (decision 0099);
/// version 19 adds a base's `airfield_damage` after its `day_work` and an
/// arrival's `waiting` after its pilots (decision 0103);
/// version 20 makes a march's order its cause, an order or the event of a
/// falling back (decision 0104);
/// version 21 adds a land unit's `when_spent` after its `working`
/// (decision 0105);
/// version 22 adds a land unit's `fieldworks` after its `when_spent`, and
/// the day's hits on men by hex and side after the land units (decision
/// 0106);
/// version 23 adds a cargo order's `weight_t` and `heeds_reserve` after its
/// `unit`, and a base's `supply_reserve_t` and `fuel_reserve_t` after its
/// `airfield_damage` (decision 0111);
/// version 24 adds each side's command state after the beliefs, Allied then
/// Japanese: its intents, leaves, plans, commitments and next plan number
/// (decision 0107);
/// version 25 adds each side's holds after its commitments and a run's ended
/// day after its `days.free` (decision 0107, step 3);
/// version 26 adds each side's interrupts: those set, the standing ones that
/// have fired, and the standing kinds switched off, after its holds
/// (decision 0107, step 4);
/// version 27 adds an air group's `strike_contact_age`, `strike_age_order`,
/// `strike_held_share` and `strike_share_order` after its `rebased_day`
/// (decision 0114);
/// version 28 adds an intent's `parent`, `maker` and `addressee` after its
/// kind, the kinds cover and hold (codes 2 and 3), a forces limit's
/// `air_groups` after its task forces and its `source` as optional, the risk
/// limit as optional (unset and balanced are told apart, since a limit not
/// set is read up the tree), and the hold of a whole air group (code 4)
/// (decisions 0115 and 0117);
/// version 29 adds the kinds neutralise a base (code 4, its target) and raid
/// a base (code 5, its base and then its target) (decision 0120);
/// version 30 adds an air group's `strike_sizing` and `strike_sizing_order`
/// after its `strike_share_order` (decision 0118), and the kinds deny an area
/// (code 6, its hex and then its distance), strike ships (code 7, its base)
/// and search waters (code 8, its base) (decision 0123).
pub const STATE_VERSION: &[u8] = b"flagplot-state-v30";

/// Names every field of the state, with no `..` in any pattern. It does
/// nothing when it runs. It is here so that a field added to the state does
/// not compile until it is named below, which is the moment to add it to
/// [`hash_world`] or to say here why it is not hashed (decision 0055).
/// [`BeliefState`](crate::belief::BeliefState) keeps its fields private and
/// is hashed through its accessors, so it is not named here.
fn name_every_field(world: &World) {
    let World {
        seed: _,
        // Not state: how to record, never what happens.
        config: _,
        // Not hashed: the rules and the map are the same for a whole game.
        // A saved game records the hash of each data file instead.
        ref_data: _,
        map: _,
        truth,
        beliefs,
        // Hashed by count and next number only.
        orders: _,
        order_ids: _,
        // Hashed: every counter, Allied then Japanese, task forces then air groups.
        made_ids: _,
        // Not hashed: orders waiting for the next day are not yet state.
        pending: _,
        // Hashed after the beliefs, Allied then Japanese (decision 0107).
        command,
        // Not hashed: commands and a staff's hand-in waiting for the next
        // day are not yet state, as orders waiting are not.
        pending_commands: _,
        pending_staff: _,
        // Not hashed: what the staff handed in on the last day, kept for
        // the runner that saves the game. What it changed is state.
        staff_handed_in: _,
        // Hashed by count only.
        events: _,
        // Not hashed: the hashes themselves.
        day_hashes: _,
        // Not hashed: scratch state of one pulse, empty when the day ends.
        strikes_in_flight: _,
        sorties: _,
        grounded_bases: _,
        // Not hashed: scratch state of one pulse, the damage of each base
        // with a damaged airfield at its start (decision 0103).
        field_damage: _,
        deck_losses: _,
        // Not hashed: fixed for the game and read by no rule (decision 5
        // of 0054).
        objectives: _,
        // Not state: the trace of choices, how to record, never what
        // happens (decision 0100, 7).
        choice_trace: _,
    } = world;
    let TruthState {
        day: _,
        ships,
        task_forces,
        bases,
        air_groups,
        replacements,
        arrivals,
        land_units,
        hits_on_men,
    } = truth;
    if let Some(HitsOnMen { hits: _, causes: _ }) = hits_on_men.values().next() {}
    if let Some(ship) = ships.values().next() {
        let Ship {
            id: _,
            // Not hashed: a name changes nothing that happens.
            name: _,
            class: _,
            side: _,
            fuel_t: _,
            supply_t: _,
            cargo_fuel_t: _,
            location: _,
            flotation_damage: _,
            fire_damage: _,
            gun_rounds: _,
            torpedoes_ready: _,
            torpedoes_one_side_fired: _,
            systems,
            experience: _,
            system_repair_points: _,
            troops,
        } = ship;
        if let Some(Troops { unit: _, men: _ }) = troops {}
        let ShipSystems {
            propulsion: _,
            fire_control: _,
            main_battery: _,
            torpedo_mounts: _,
            flight_deck: _,
        } = systems;
    }
    if let Some(task_force) = task_forces.values().next() {
        let TaskForce {
            id: _,
            name: _,
            side: _,
            ships: _,
            mission: _,
            speed: _,
            route,
            from_hex: _,
            toward_hex: _,
            progress_nm: _,
            leg_nm: _,
            status: _,
            route_order: _,
            speed_order: _,
            cargo_order,
            arrived_at_hours: _,
            arrival_event: _,
            readiness: _,
            readiness_order: _,
            alerted: _,
            alerted_by: _,
            rules_of_engagement,
            rules_order: _,
            home_port: _,
            fatigue: _,
            yard_trip,
            halt,
            preparation,
            landing_at: _,
        } = task_force;
        let Route {
            waypoints: _,
            next_index: _,
            act_at: _,
        } = route;
        if let Some(Halt {
            work: HaltWork {
                bombard: _,
                cargo: _,
            },
            at_hours: _,
            event: _,
            bombarded: _,
            cargo_completed: _,
        }) = halt
        {}
        if let Some(Preparation {
            hex: _,
            roster: _,
            loading: _,
            order: _,
        }) = preparation
        {}
        let RulesOfEngagement {
            stance: _,
            break_off: _,
            stragglers: _,
            keep_share: _,
        } = rules_of_engagement;
        if let Some(CargoOrder {
            action: _,
            order: _,
            unit: _,
            weight_t: _,
            heeds_reserve: _,
        }) = cargo_order
        {}
        if let Some(YardTrip {
            base: _,
            return_day: _,
            departure: _,
        }) = yard_trip
        {}
    }
    if let Some(base) = bases.values().next() {
        let Base {
            id: _,
            name: _,
            side: _,
            hex: _,
            supply_t: _,
            anti_aircraft: _,
            aircraft_repairs_per_day: _,
            fuel_t: _,
            ship_repair_points_per_day: _,
            yard_passage_days: _,
            on_map: _,
            airfield: _,
            airfield_work: _,
            defences: _,
            day_work,
            airfield_damage: _,
            supply_reserve_t: _,
            fuel_reserve_t: _,
        } = base;
        if let Some(DayWork {
            day: _,
            port: _,
            aircraft: _,
        }) = day_work
        {}
    }
    if let Some(group) = air_groups.values().next() {
        let AirGroup {
            id: _,
            name: _,
            side: _,
            location: _,
            aircraft_type: _,
            ready: _,
            damaged: _,
            mission: _,
            search,
            mission_order: _,
            arc_order: _,
            patrol_share: _,
            share_order: _,
            fatigue: _,
            rest_above: _,
            rest_order: _,
            stood_down: _,
            pilots: _,
            strength: _,
            reach: _,
            reach_order: _,
            target_priority: _,
            target_order: _,
            last_base_strike_day: _,
            rebased_day: _,
            strike_contact_age: _,
            strike_age_order: _,
            strike_held_share: _,
            strike_share_order: _,
            strike_sizing: _,
            strike_sizing_order: _,
        } = group;
        if let Some(SearchArc {
            from_deg: _,
            to_deg: _,
            range_nm: _,
        }) = search
        {}
    }
    if let Some(Replacement {
        side: _,
        base: _,
        aircraft_type: _,
        aircraft_per_day: _,
        pilots_per_day: _,
    }) = replacements.first()
    {}
    if let Some(Arrival {
        day: _,
        air_group: _,
        aircraft: _,
        pilots: _,
        waiting: _,
    }) = arrivals.first()
    {}
    if let Some(LandUnit {
        id: _,
        // Not hashed: a name changes nothing that happens.
        name: _,
        side: _,
        men_ashore: _,
        hex: _,
        posture: _,
        posture_order: _,
        fatigue: _,
        fed_share: _,
        march: _,
        rehearsal: _,
        rehearsal_hex: _,
        activity: _,
        activity_order: _,
        experience: _,
        training_days: _,
        fight_days: _,
        skills:
            Skills {
                assault: _,
                holding: _,
                weight: _,
                clear: _,
                jungle: _,
                mountain: _,
                atoll: _,
                works: _,
            },
        working: _,
        when_spent: _,
        fieldworks: _,
    }) = land_units.values().next()
    {}
    if let Some(Some(March {
        to: _,
        days: _,
        counted: _,
        cause: _,
    })) = land_units.values().next().map(|unit| unit.march)
    {}
    for state in command.values() {
        let CommandState {
            intents,
            leaves,
            plans,
            commitments,
            next_plan: _,
            holds,
            interrupts,
            standing,
            standing_off,
        } = state;
        if let Some(Intent {
            id: _,
            kind,
            parent: _,
            maker,
            addressee,
            limits,
            set_day: _,
            plan: _,
            withdrawn_day: _,
        }) = intents.values().next()
        {
            name_intent_kind(*kind);
            match maker {
                Maker::Commander | Maker::Staff => {}
            }
            match addressee {
                Addressee::Area | Addressee::Base(_) | Addressee::AirGroup(_) => {}
            }
            name_limits(limits);
        }
        let Leaves {
            prepare: _,
            launch: _,
        } = leaves;
        if let Some(Plan {
            id: _,
            intent: _,
            state: _,
            baseline,
            amendments,
            runs,
        }) = plans.values().next()
        {
            if let Some(Baseline {
                day: _,
                kind: _,
                limits,
            }) = baseline
            {
                name_limits(limits);
            }
            if let Some(Amendment {
                number: _,
                day: _,
                change,
            }) = amendments.first()
            {
                match change {
                    AmendmentChange::Intent(_) | AmendmentChange::Limit(_) => {}
                }
            }
            if let Some(Run {
                number: _,
                ships: _,
                load_t: _,
                source: _,
                days:
                    RunDays {
                        load: _,
                        sail: _,
                        arrive: _,
                        free: _,
                    },
                ended: _,
            }) = runs.values().next()
            {}
        }
        if let Some(Commitment {
            plan: _,
            run: _,
            from_day: _,
            to_day: _,
        }) = commitments.values().flatten().next()
        {}
        if let Some((target, Hold { day: _, by })) = holds.iter().next() {
            match target {
                HoldTarget::Ship(_) | HoldTarget::TaskForce(_) | HoldTarget::AirGroup(_) => {}
                HoldTarget::Setting {
                    group: _,
                    setting: _,
                } => {}
            }
            match by {
                HoldMaker::Command | HoldMaker::Order(_) => {}
            }
        }
        if let Some((
            _,
            SetInterrupt {
                id: _,
                condition,
                set_day: _,
                status,
            },
        )) = interrupts.iter().next()
        {
            let InterruptCondition::SupplyBelow { base: _, days: _ } = condition;
            let InterruptStatus {
                armed: _,
                last_fired: _,
                acknowledged: _,
            } = status;
        }
        if let Some(((_, subject), status)) = standing.iter().next() {
            match subject {
                StandingSubject::Plan(_) | StandingSubject::Ship(_) => {}
            }
            let InterruptStatus {
                armed: _,
                last_fired: _,
                acknowledged: _,
            } = status;
        }
        let _ = standing_off.iter().next();
    }
    for belief in beliefs.values() {
        if let Some(Contact {
            id: _,
            estimated_hex: _,
            estimated_ship_count: _,
            estimated_kind: _,
            age_pulses: _,
            last_report_event: _,
        }) = belief.contacts().next()
        {}
        if let Some(BattleClaim {
            hex: _,
            day: _,
            pulse: _,
            claimed_sunk: _,
            // The air figures of a claim are hashed after the claims, only
            // when a claim has any (decision 0056).
            claimed_hits: _,
            claimed_kind: _,
            claimed_shot_down: _,
            claimed_on_ground: _,
            from_air: _,
            from_bombardment: _,
            received_event: _,
        }) = belief.claims().first()
        {}
        if let Some(GroundEstimate {
            hex: _,
            men: _,
            day: _,
            report_event: _,
        }) = belief.ground_estimates().next()
        {}
    }
}

/// Names every field of every kind of intent, with no `..`.
fn name_intent_kind(kind: IntentKind) {
    match kind {
        IntentKind::KeepSupplied {
            base: _,
            floor_days: _,
            interval_days: _,
        }
        | IntentKind::Cover { base: _ }
        | IntentKind::Hold { base: _ }
        | IntentKind::Neutralise { target: _ }
        | IntentKind::Raid { base: _, target: _ }
        | IntentKind::Deny {
            hex: _,
            distance: _,
        }
        | IntentKind::StrikeShips { base: _ }
        | IntentKind::SearchWaters { base: _ } => {}
    }
}

/// Names every field of the limits of an intent, with no `..`.
fn name_limits(limits: &IntentLimits) {
    let IntentLimits {
        forces,
        area: _,
        priority: _,
        protected: _,
        risk: _,
    } = limits;
    if let Some(ForcesLimit {
        ships: _,
        task_forces: _,
        air_groups: _,
        source: _,
    }) = forces
    {}
}

/// Hashes the kind of an intent: its code and its fields.
fn hash_intent_kind(h: &mut StateHasher, kind: IntentKind) {
    h.u8(kind.code());
    match kind {
        IntentKind::KeepSupplied {
            base,
            floor_days,
            interval_days,
        } => {
            h.u32(base.raw());
            h.u32(floor_days);
            h.optional_u32(interval_days);
        }
        IntentKind::Cover { base } | IntentKind::Hold { base } => h.u32(base.raw()),
        IntentKind::Neutralise { target } => h.u32(target.raw()),
        IntentKind::Raid { base, target } => {
            h.u32(base.raw());
            h.u32(target.raw());
        }
        // Decision 0123.
        IntentKind::Deny { hex, distance } => {
            h.i32(hex.q);
            h.i32(hex.r);
            h.u32(distance);
        }
        IntentKind::StrikeShips { base } | IntentKind::SearchWaters { base } => h.u32(base.raw()),
    }
}

/// Hashes the limits of an intent: forces, area, priority, protected, risk.
fn hash_limits(h: &mut StateHasher, limits: &IntentLimits) {
    match &limits.forces {
        Some(forces) => {
            h.u8(1);
            hash_forces(h, forces);
        }
        None => h.u8(0),
    }
    match &limits.area {
        Some(area) => {
            h.u8(1);
            h.count(area.len());
            for hex in area {
                h.i32(hex.q);
                h.i32(hex.r);
            }
        }
        None => h.u8(0),
    }
    h.optional_u32(limits.priority);
    h.count(limits.protected.len());
    for ship in &limits.protected {
        h.u32(ship.raw());
    }
    match limits.risk {
        Some(risk) => h.u8(risk.code()),
        None => h.u8(0),
    }
}

fn hash_forces(h: &mut StateHasher, forces: &ForcesLimit) {
    h.count(forces.ships.len());
    for ship in &forces.ships {
        h.u32(ship.raw());
    }
    h.count(forces.task_forces.len());
    for force in &forces.task_forces {
        h.u32(force.raw());
    }
    h.count(forces.air_groups.len());
    for group in &forces.air_groups {
        h.u32(group.raw());
    }
    h.optional_u32(forces.source.map(BaseId::raw));
}

/// Hashes one limit as a command set it, for an amendment: a code for the
/// kind, then its fields. Codes are never renumbered.
fn hash_limit(h: &mut StateHasher, limit: &Limit) {
    match limit {
        Limit::Forces { intent, limit } => {
            h.u8(1);
            h.u32(intent.raw());
            hash_forces(h, limit);
        }
        Limit::Area { intent, hexes } => {
            h.u8(2);
            h.u32(intent.raw());
            h.count(hexes.len());
            for hex in hexes {
                h.i32(hex.q);
                h.i32(hex.r);
            }
        }
        Limit::Priority { intent, rank } => {
            h.u8(3);
            h.u32(intent.raw());
            h.optional_u32(*rank);
        }
        Limit::Protected { intent, ships } => {
            h.u8(4);
            h.u32(intent.raw());
            h.count(ships.len());
            for ship in ships {
                h.u32(ship.raw());
            }
        }
        Limit::Reserve {
            base,
            supply_t,
            fuel_t,
        } => {
            h.u8(5);
            h.u32(base.raw());
            h.i64(supply_t.raw());
            h.i64(fuel_t.raw());
        }
        Limit::Risk { intent, risk } => {
            h.u8(6);
            h.u32(intent.raw());
            h.u8(risk.code());
        }
    }
}

/// Hashes one side's command state in a fixed walk of every field (decision
/// 0107).
fn hash_command_state(h: &mut StateHasher, state: &CommandState) {
    h.count(state.intents.len());
    for intent in state.intents.values() {
        h.u32(intent.id.raw());
        hash_intent_kind(h, intent.kind);
        h.optional_u32(intent.parent.map(IntentId::raw));
        h.u8(intent.maker.code());
        h.u8(intent.addressee.code());
        match intent.addressee {
            Addressee::Area => {}
            Addressee::Base(base) => h.u32(base.raw()),
            Addressee::AirGroup(group) => h.u32(group.raw()),
        }
        hash_limits(h, &intent.limits);
        h.u32(intent.set_day);
        h.u32(intent.plan.raw());
        h.optional_u32(intent.withdrawn_day);
    }
    h.u8(u8::from(state.leaves.prepare));
    h.u8(u8::from(state.leaves.launch));
    h.count(state.plans.len());
    for plan in state.plans.values() {
        h.u32(plan.id.raw());
        h.u32(plan.intent.raw());
        h.u8(plan.state.code());
        match &plan.baseline {
            Some(baseline) => {
                h.u8(1);
                h.u32(baseline.day);
                hash_intent_kind(h, baseline.kind);
                hash_limits(h, &baseline.limits);
            }
            None => h.u8(0),
        }
        h.count(plan.amendments.len());
        for amendment in &plan.amendments {
            h.u32(amendment.number);
            h.u32(amendment.day);
            match &amendment.change {
                AmendmentChange::Intent(kind) => {
                    h.u8(1);
                    hash_intent_kind(h, *kind);
                }
                AmendmentChange::Limit(limit) => {
                    h.u8(2);
                    hash_limit(h, limit);
                }
            }
        }
        h.count(plan.runs.len());
        for run in plan.runs.values() {
            h.u32(run.number);
            h.count(run.ships.len());
            for ship in &run.ships {
                h.u32(ship.raw());
            }
            h.i64(run.load_t.raw());
            h.u32(run.source.raw());
            h.u32(run.days.load);
            h.u32(run.days.sail);
            h.u32(run.days.arrive);
            h.u32(run.days.free);
            h.optional_u32(run.ended);
        }
    }
    h.count(state.commitments.len());
    for (ship, list) in &state.commitments {
        h.u32(ship.raw());
        h.count(list.len());
        for commitment in list {
            h.u32(commitment.plan.raw());
            h.optional_u32(commitment.run);
            h.u32(commitment.from_day);
            h.u32(commitment.to_day);
        }
    }
    h.count(state.holds.len());
    for (target, hold) in &state.holds {
        h.u8(target.code());
        match target {
            HoldTarget::Ship(ship) => h.u32(ship.raw()),
            HoldTarget::TaskForce(force) => h.u32(force.raw()),
            HoldTarget::Setting { group, setting } => {
                h.u32(group.raw());
                h.u8(setting.code());
            }
            HoldTarget::AirGroup(group) => h.u32(group.raw()),
        }
        h.u32(hold.day);
        match hold.by {
            HoldMaker::Command => h.u8(1),
            HoldMaker::Order(order) => {
                h.u8(2);
                h.u32(order.raw());
            }
        }
    }
    h.count(state.interrupts.len());
    for interrupt in state.interrupts.values() {
        h.u32(interrupt.id.raw());
        match interrupt.condition {
            InterruptCondition::SupplyBelow { base, days } => {
                h.u8(interrupt.condition.code());
                h.u32(base.raw());
                h.u32(days);
            }
        }
        h.u32(interrupt.set_day);
        hash_interrupt_status(h, &interrupt.status);
    }
    h.count(state.standing.len());
    for ((kind, subject), status) in &state.standing {
        h.u8(kind.code());
        h.u8(subject.code());
        h.u32(subject.raw());
        hash_interrupt_status(h, status);
    }
    h.count(state.standing_off.len());
    for kind in &state.standing_off {
        h.u8(kind.code());
    }
    h.u32(state.next_plan);
}

/// Hashes where an interrupt stands: armed, the day it last fired,
/// acknowledged.
fn hash_interrupt_status(h: &mut StateHasher, status: &InterruptStatus) {
    h.u8(u8::from(status.armed));
    h.optional_u32(status.last_fired);
    h.u8(u8::from(status.acknowledged));
}

/// Hashes everything that defines the state of a game. When a field is added
/// to the state, it must be added here too, or two different states could
/// share a hash.
pub(crate) fn hash_world(world: &World) -> u64 {
    name_every_field(world);
    // A strike launched in a pulse lands in the same pulse.
    debug_assert!(
        world.strikes_in_flight.is_empty(),
        "strikes were launched and have not landed"
    );
    let mut h = StateHasher::new();
    h.bytes(STATE_VERSION);
    h.u64(world.seed);

    let truth = &world.truth;
    h.u32(truth.day);

    h.count(truth.ships.len());
    for ship in truth.ships.values() {
        h.u32(ship.id.raw());
        h.u32(ship.class.raw());
        h.u8(ship.side.code());
        h.i64(ship.fuel_t.raw());
        h.i64(ship.supply_t.raw());
        h.i64(ship.flotation_damage.raw());
        h.i64(ship.fire_damage.raw());
        h.i64(ship.gun_rounds.raw());
        h.u8(u8::from(ship.torpedoes_ready));
        h.u8(u8::from(ship.torpedoes_one_side_fired));
        for system in ShipSystem::ALL {
            h.u8(ship.systems.get(system).code());
        }
        h.i64(ship.cargo_fuel_t.raw());
        match ship.location {
            ShipLocation::TaskForce(id) => {
                h.u8(1);
                h.u32(id.raw());
            }
            ShipLocation::Port(id) => {
                h.u8(2);
                h.u32(id.raw());
            }
            ShipLocation::Sunk => h.u8(3),
            ShipLocation::Yard(id) => {
                h.u8(4);
                h.u32(id.raw());
            }
        }
        // After the location (decision 0064).
        h.u8(ship.experience);
        // After the repair points (decision 0069).
        h.i64(ship.system_repair_points.raw());
        // Last in the ship (decision 0066).
        match ship.troops {
            Some(troops) => {
                h.u8(1);
                h.u32(troops.unit.raw());
                h.u32(troops.men);
            }
            None => h.u8(0),
        }
    }

    h.count(truth.task_forces.len());
    for task_force in truth.task_forces.values() {
        h.u32(task_force.id.raw());
        h.u8(task_force.side.code());
        h.count(task_force.ships.len());
        for ship in &task_force.ships {
            h.u32(ship.raw());
        }
        h.u8(mission_code(task_force.mission));
        h.u8(speed_code(task_force.speed));
        if let SpeedSetting::Knots(knots) = task_force.speed {
            h.i64(knots.raw());
        }
        h.count(task_force.route.waypoints.len());
        for waypoint in &task_force.route.waypoints {
            h.i32(waypoint.q);
            h.i32(waypoint.r);
        }
        h.count(task_force.route.next_index);
        // After the route index (decision 0096).
        h.count(task_force.route.act_at.len());
        for hex in &task_force.route.act_at {
            h.i32(hex.q);
            h.i32(hex.r);
        }
        h.i32(task_force.from_hex.q);
        h.i32(task_force.from_hex.r);
        match task_force.toward_hex {
            Some(toward) => {
                h.u8(1);
                h.i32(toward.q);
                h.i32(toward.r);
            }
            None => h.u8(0),
        }
        h.i64(task_force.progress_nm.raw());
        h.i64(task_force.leg_nm.raw());
        h.u8(task_force.status.code());
        h.optional_u32(task_force.route_order.map(|id| id.raw()));
        h.optional_u32(task_force.speed_order.map(|id| id.raw()));
        match task_force.cargo_order {
            Some(cargo) => {
                h.u8(1);
                h.u8(cargo.action.code());
                h.u32(cargo.order.raw());
                // After the order (decision 0066).
                h.optional_u32(cargo.unit.map(|unit| unit.raw()));
                // After the unit (decision 0111).
                match cargo.weight_t {
                    Some(weight) => {
                        h.u8(1);
                        h.i64(weight.raw());
                    }
                    None => h.u8(0),
                }
                h.u8(u8::from(cargo.heeds_reserve));
            }
            None => h.u8(0),
        }
        match task_force.arrived_at_hours {
            Some(hours) => {
                h.u8(1);
                h.i64(hours.raw());
            }
            None => h.u8(0),
        }
        h.optional_u32(task_force.arrival_event.map(|id| id.raw()));
        h.u8(task_force.readiness.code());
        h.optional_u32(task_force.readiness_order.map(|id| id.raw()));
        h.u8(u8::from(task_force.alerted));
        h.optional_u32(task_force.alerted_by.map(|id| id.raw()));
        h.u8(task_force.rules_of_engagement.stance.code());
        h.i64(task_force.rules_of_engagement.break_off.raw());
        h.u8(task_force.rules_of_engagement.stragglers.code());
        h.i64(task_force.rules_of_engagement.keep_share.raw());
        h.optional_u32(task_force.rules_order.map(|id| id.raw()));
        h.optional_u32(task_force.home_port.map(|id| id.raw()));
        h.i64(task_force.fatigue.raw());
        // Last in the task force (decision 0069).
        match task_force.yard_trip {
            Some(trip) => {
                h.u8(1);
                h.u32(trip.base.raw());
                h.u32(trip.return_day);
                h.u32(trip.departure.raw());
            }
            None => h.u8(0),
        }
        // Last in the task force (decision 0096).
        match task_force.halt {
            Some(halt) => {
                h.u8(1);
                h.u8(u8::from(halt.work.bombard));
                h.u8(u8::from(halt.work.cargo));
                h.i64(halt.at_hours.raw());
                h.u32(halt.event.raw());
                h.optional_u32(halt.bombarded.map(|id| id.raw()));
                h.optional_u32(halt.cargo_completed.map(|id| id.raw()));
            }
            None => h.u8(0),
        }
        // After the halt (decision 0095): the preparation, whose roster is
        // in unit order, and the hex where landing has begun.
        match &task_force.preparation {
            Some(preparation) => {
                h.u8(1);
                h.i32(preparation.hex.q);
                h.i32(preparation.hex.r);
                h.count(preparation.roster.len());
                for unit in &preparation.roster {
                    h.u32(unit.raw());
                }
                h.i64(preparation.loading.raw());
                h.u32(preparation.order.raw());
            }
            None => h.u8(0),
        }
        match task_force.landing_at {
            Some(hex) => {
                h.u8(1);
                h.i32(hex.q);
                h.i32(hex.r);
            }
            None => h.u8(0),
        }
    }

    h.count(truth.bases.len());
    for base in truth.bases.values() {
        h.u32(base.id.raw());
        h.u8(base.side.code());
        h.i32(base.hex.q);
        h.i32(base.hex.r);
        h.i64(base.supply_t.raw());
        h.u32(base.anti_aircraft);
        h.u32(base.aircraft_repairs_per_day);
        h.i64(base.fuel_t.raw());
        // After the fuel (decision 0069).
        h.i64(base.ship_repair_points_per_day.raw());
        match base.yard_passage_days {
            Some(days) => {
                h.u8(1);
                h.u32(days);
            }
            None => h.u8(0),
        }
        // After the base's place on the map (decision 0066).
        h.u8(u8::from(base.on_map));
        // Last in the base (decision 0099).
        h.u8(u8::from(base.airfield));
        h.i64(base.airfield_work.raw());
        h.i64(base.defences.raw());
        match base.day_work {
            Some(work) => {
                h.u8(1);
                h.u32(work.day);
                h.i64(work.port.raw());
                h.i64(work.aircraft.raw());
            }
            None => h.u8(0),
        }
        // After the base's day of work (decision 0103).
        h.i64(base.airfield_damage.raw());
        // After the airfield's damage (decision 0111).
        h.i64(base.supply_reserve_t.raw());
        h.i64(base.fuel_reserve_t.raw());
    }

    h.count(truth.air_groups.len());
    for group in truth.air_groups.values() {
        h.u32(group.id.raw());
        h.u8(group.side.code());
        match group.location {
            AirGroupLocation::Base(base) => {
                h.u8(1);
                h.u32(base.raw());
            }
            AirGroupLocation::Ship(ship) => {
                h.u8(2);
                h.u32(ship.raw());
            }
            // Lost with its base (decision 0090).
            AirGroupLocation::Lost => h.u8(3),
        }
        h.u32(group.aircraft_type.raw());
        h.u32(group.ready);
        h.u32(group.damaged);
        h.i64(group.fatigue.raw());
        match group.rest_above {
            Some(level) => {
                h.u8(1);
                h.i64(level.raw());
            }
            None => h.u8(0),
        }
        h.optional_u32(group.rest_order.map(|id| id.raw()));
        h.u8(u8::from(group.stood_down));
        h.optional_u32(group.pilots);
        h.u32(group.strength);
        h.u8(group.mission.code());
        // The base strike mission (code 5) is followed by its target base.
        if let AirMission::BaseStrike(base) = group.mission {
            h.u32(base.raw());
        }
        h.i64(group.patrol_share.raw());
        h.optional_u32(group.share_order.map(|id| id.raw()));
        match group.search {
            Some(arc) => {
                h.u8(1);
                h.u32(arc.from_deg);
                h.u32(arc.to_deg);
                h.i64(arc.range_nm.raw());
            }
            None => h.u8(0),
        }
        h.optional_u32(group.mission_order.map(|id| id.raw()));
        h.optional_u32(group.arc_order.map(|id| id.raw()));
        h.u8(group.reach.code());
        h.optional_u32(group.reach_order.map(|id| id.raw()));
        h.count(group.target_priority.len());
        for kind in &group.target_priority {
            h.u8(kind.code());
        }
        h.optional_u32(group.target_order.map(|id| id.raw()));
        h.optional_u32(group.last_base_strike_day);
        // Last in the group (decision 0091).
        h.optional_u32(group.rebased_day);
        // Last in the group (decision 0114).
        h.optional_u32(group.strike_contact_age);
        h.optional_u32(group.strike_age_order.map(|id| id.raw()));
        h.i64(group.strike_held_share.raw());
        h.optional_u32(group.strike_share_order.map(|id| id.raw()));
        // Last in the group (decision 0118).
        h.u8(u8::from(group.strike_sizing));
        h.optional_u32(group.strike_sizing_order.map(|id| id.raw()));
    }

    h.count(truth.replacements.len());
    for stream in &truth.replacements {
        h.u8(stream.side.code());
        h.u32(stream.base.raw());
        h.u32(stream.aircraft_type.raw());
        h.u32(stream.aircraft_per_day);
        h.u32(stream.pilots_per_day);
    }

    h.count(truth.arrivals.len());
    for arrival in &truth.arrivals {
        h.u32(arrival.day);
        h.u32(arrival.air_group.raw());
        h.u32(arrival.aircraft);
        h.u32(arrival.pilots);
        // After the pilots (decision 0103).
        h.u8(u8::from(arrival.waiting));
    }

    // After the arrivals (decision 0066).
    h.count(truth.land_units.len());
    for unit in truth.land_units.values() {
        h.u32(unit.id.raw());
        h.u8(unit.side.code());
        h.u32(unit.men_ashore);
        match unit.hex {
            Some(hex) => {
                h.u8(1);
                h.i32(hex.q);
                h.i32(hex.r);
            }
            None => h.u8(0),
        }
        // After the hex (decision 0090).
        h.u8(unit.posture.code());
        h.optional_u32(unit.posture_order.map(|id| id.raw()));
        h.i64(unit.fatigue.raw());
        h.i64(unit.fed_share.raw());
        // After the fed share (decision 0094).
        match unit.march {
            Some(march) => {
                h.u8(1);
                h.i32(march.to.q);
                h.i32(march.to.r);
                h.u32(march.days);
                h.i64(march.counted.raw());
                // The cause, after the counted days (decision 0104): an order
                // or the event of a falling back.
                match march.cause {
                    Cause::Order(id) => {
                        h.u8(1);
                        h.u32(id.raw());
                    }
                    Cause::Event(id) => {
                        h.u8(2);
                        h.u32(id.raw());
                    }
                }
            }
            None => h.u8(0),
        }
        // After the march (decisions 0095 and 0097).
        h.i64(unit.rehearsal.raw());
        match unit.rehearsal_hex {
            Some(hex) => {
                h.u8(1);
                h.i32(hex.q);
                h.i32(hex.r);
            }
            None => h.u8(0),
        }
        h.u8(unit.activity.code());
        // The job of a work activity, 0 for none (decision 0099).
        if let Activity::Work(job) = unit.activity {
            h.u8(job.map_or(0, Job::code));
        }
        h.optional_u32(unit.activity_order.map(|id| id.raw()));
        h.u8(unit.experience);
        h.u32(unit.training_days);
        h.u32(unit.fight_days);
        // After the counts (decision 0098), in a fixed order.
        let skills = unit.skills;
        for skill in [
            skills.assault,
            skills.holding,
            skills.weight,
            skills.clear,
            skills.jungle,
            skills.mountain,
            skills.atoll,
            skills.works,
        ] {
            h.u8(skill);
        }
        // After the skills (decision 0099).
        h.u8(u8::from(unit.working));
        // After working (decision 0105): 0 for no instruction.
        h.u8(unit.when_spent.map_or(0, WhenSpent::code));
        // After when_spent (decision 0106).
        h.i64(unit.fieldworks.raw());
    }

    // After the land units (decision 0106): empty at the end of every day.
    h.count(truth.hits_on_men.len());
    for ((hex, side), hits) in &truth.hits_on_men {
        h.i32(hex.q);
        h.i32(hex.r);
        h.u8(side.code());
        h.u32(hits.hits);
        h.count(hits.causes.len());
        for cause in &hits.causes {
            h.u32(cause.raw());
        }
    }

    h.count(world.beliefs.len());
    for (side, belief) in &world.beliefs {
        // The refreshed-this-pulse set is transient and not hashed: it must
        // be empty here, after the last pulse's ageing.
        debug_assert!(
            !belief.has_pending_refreshes(),
            "contacts were refreshed and not aged before the state was hashed"
        );
        h.u8(side.code());
        h.u32(belief.next_contact_raw());
        h.count(belief.contacts().count());
        for contact in belief.contacts() {
            h.u32(contact.id.raw());
            h.i32(contact.estimated_hex.q);
            h.i32(contact.estimated_hex.r);
            h.u32(contact.estimated_ship_count);
            h.u8(contact.estimated_kind.code());
            h.u32(contact.age_pulses);
            h.u32(contact.last_report_event.raw());
        }
        h.count(belief.claims().len());
        for claim in belief.claims() {
            h.i32(claim.hex.q);
            h.i32(claim.hex.r);
            h.u32(claim.day);
            h.u64(claim.pulse.code());
            h.u32(claim.claimed_sunk);
            h.u32(claim.received_event.raw());
            h.u32(claim.claimed_hits);
            h.u8(claim.claimed_kind.map_or(0, ShipKind::code));
            h.u32(claim.claimed_shot_down);
            h.u32(claim.claimed_on_ground);
            h.u8(u8::from(claim.from_air));
            // After `from_air` (decision 0092).
            h.u8(u8::from(claim.from_bombardment));
        }
        // After the claims (decision 0092), in hex order.
        h.count(belief.ground_estimates().count());
        for estimate in belief.ground_estimates() {
            h.i32(estimate.hex.q);
            h.i32(estimate.hex.r);
            h.u32(estimate.men);
            h.u32(estimate.day);
            h.u32(estimate.report_event.raw());
        }
    }

    // After the beliefs (decision 0107): each side's command state, Allied
    // then Japanese.
    for side in Side::ALL {
        h.u8(side.code());
        hash_command_state(&mut h, &world.command[&side]);
    }

    h.count(world.orders.len());
    h.u32(world.order_ids.next_raw());
    for side in Side::ALL {
        h.u32(world.made_ids.next_task_force(side));
    }
    for side in Side::ALL {
        h.u32(world.made_ids.next_air_group(side));
    }
    h.count(world.events.len());

    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_published_fnv1a_test_vectors() {
        let hash = |text: &str| {
            let mut h = StateHasher::new();
            h.bytes(text.as_bytes());
            h.finish()
        };
        assert_eq!(hash(""), 0xCBF2_9CE4_8422_2325);
        assert_eq!(hash("a"), 0xAF63_DC4C_8601_EC8C);
        assert_eq!(hash("foobar"), 0x8594_4171_F739_67E8);
    }

    #[test]
    fn absent_and_present_ids_hash_differently() {
        let mut absent = StateHasher::new();
        absent.optional_u32(None);
        let mut present = StateHasher::new();
        present.optional_u32(Some(0));
        assert_ne!(absent.finish(), present.finish());
    }
}

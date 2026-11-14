//! Static reference data, loaded once and never changed by resolution.
//!
//! Only the fields the scenarios so far need exist: ship classes (speed,
//! fuel, cargo capacity, guns, torpedoes, toughness and radar), the port
//! size table, aircraft types (search range and sweep width), the search
//! figures of decision 0012 and the surface combat figures of decision 0014.
//! The spec lists more for a ship class (armour, aircraft capacity and so
//! on) and one more entity (weapons). They arrive with the scenarios that use
//! them.

use std::collections::BTreeMap;

use crate::fixed::Fixed;
use crate::ids::{AircraftTypeId, ShipClassId};
use crate::orders::Reach;
use crate::state::{Ship, ShipSystem, SystemState};

/// Broad type of a ship class.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ShipKind {
    Carrier,
    Battleship,
    Cruiser,
    Destroyer,
    Transport,
    Auxiliary,
}

impl ShipKind {
    /// Every kind, in importance order (decision 9 of
    /// `docs/decisions/0012-search-and-contacts.md`): the carrier is the most
    /// important. The derived `Ord` matches this order, so the most important
    /// kind of a set is its minimum.
    pub const ALL: [ShipKind; 6] = [
        ShipKind::Carrier,
        ShipKind::Battleship,
        ShipKind::Cruiser,
        ShipKind::Destroyer,
        ShipKind::Transport,
        ShipKind::Auxiliary,
    ];

    /// A stable number for the kind. Part of the state hash. Never renumber
    /// it; add new kinds at the end.
    pub const fn code(self) -> u8 {
        match self {
            ShipKind::Carrier => 1,
            ShipKind::Battleship => 2,
            ShipKind::Cruiser => 3,
            ShipKind::Destroyer => 4,
            ShipKind::Transport => 5,
            ShipKind::Auxiliary => 6,
        }
    }

    /// The kind's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            ShipKind::Carrier => "carrier",
            ShipKind::Battleship => "battleship",
            ShipKind::Cruiser => "cruiser",
            ShipKind::Destroyer => "destroyer",
            ShipKind::Transport => "transport",
            ShipKind::Auxiliary => "auxiliary",
        }
    }
}

/// One class of ship.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShipClass {
    pub id: ShipClassId,
    pub name: String,
    pub kind: ShipKind,
    /// Economical speed, in knots. Endurance is quoted at this speed.
    pub cruise_speed_kn: Fixed,
    /// Top speed, in knots.
    pub max_speed_kn: Fixed,
    /// Full fuel load, in tons.
    pub fuel_capacity_t: Fixed,
    /// Distance a full fuel load lasts at cruise speed, in nautical miles.
    pub endurance_nm: Fixed,
    /// Supply the ship can carry, in tons. May be zero.
    pub cargo_capacity_t: Fixed,
    /// Guns in the main battery. May be zero.
    pub main_guns: u32,
    /// Rounds one gun fires in a minute. May be a decimal (real guns fire
    /// 3.5 a minute); zero if the ship has no main battery.
    pub gun_rounds_per_min: Fixed,
    /// Rounds carried for each gun.
    pub gun_rounds_per_gun: u32,
    /// Damage one gun hit does, before the target's toughness. May be a
    /// decimal.
    pub gun_damage: Fixed,
    /// Torpedo tubes. May be zero.
    pub torpedo_tubes: u32,
    /// Torpedo tubes that bear on one side (decisions 0074 and 0083): from 0
    /// to `torpedo_tubes`, and above 0 when `torpedo_tubes` is. Equal to
    /// `torpedo_tubes` for a centreline fit and for every placeholder and
    /// frozen class.
    pub torpedo_tubes_one_side: u32,
    /// Damage one torpedo hit does, before the target's toughness. May be a
    /// decimal.
    pub torpedo_damage: Fixed,
    /// Damage a ship of the class can take. A hit does
    /// 100 x damage / toughness points. Above zero. May be a decimal.
    pub toughness: Fixed,
    /// True if the class carries radar.
    pub radar: bool,
    /// Aircraft the ship can carry, in all the air groups aboard (decision 2
    /// of `docs/decisions/0025-carrier-air-groups.md`). 0 for a ship that
    /// carries none.
    pub aircraft_capacity: u32,
    /// Anti-aircraft shots the ship fires at a raid on its task force
    /// (decision 7 of `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`).
    /// 0 for a ship that has none.
    pub anti_aircraft: u32,
    /// Men the ship can carry, apart from its cargo (decision 0066, 4): men
    /// take no cargo room and cargo takes no berths. 0 for a ship that
    /// carries none, and wherever a class file does not give the column.
    pub troop_capacity_men: u32,
}

impl ShipClass {
    /// True if ships of the class have the system (decision 2 of
    /// `docs/decisions/0016-systems-and-stragglers.md`): propulsion always;
    /// main battery and fire control if the class has main guns; torpedo
    /// mounts if it has tubes; the flight deck if it carries aircraft.
    pub fn has_system(&self, system: ShipSystem) -> bool {
        match system {
            ShipSystem::Propulsion => true,
            ShipSystem::FireControl | ShipSystem::MainBattery => self.main_guns > 0,
            ShipSystem::TorpedoMounts => self.torpedo_tubes > 0,
            ShipSystem::FlightDeck => self.aircraft_capacity > 0,
        }
    }

    /// Describes what is wrong with the class's numbers, if anything.
    pub fn problem(&self) -> Option<String> {
        let positive = [
            ("cruise speed", self.cruise_speed_kn),
            ("max speed", self.max_speed_kn),
            ("fuel capacity", self.fuel_capacity_t),
            ("endurance", self.endurance_nm),
        ];
        for (label, value) in positive {
            if value <= Fixed::ZERO {
                return Some(format!("{} has no positive {label}", self.id));
            }
        }
        if self.max_speed_kn < self.cruise_speed_kn {
            return Some(format!(
                "{} has a max speed below its cruise speed",
                self.id
            ));
        }
        if self.cargo_capacity_t.is_negative() {
            return Some(format!("{} has a negative cargo capacity", self.id));
        }
        if self.gun_rounds_per_min.is_negative() {
            return Some(format!("{} has a negative gun rate of fire", self.id));
        }
        if self.gun_damage.is_negative() || self.torpedo_damage.is_negative() {
            return Some(format!("{} has a negative damage figure", self.id));
        }
        if self.toughness <= Fixed::ZERO {
            return Some(format!("{} has no toughness", self.id));
        }
        if self.torpedo_tubes_one_side > self.torpedo_tubes {
            return Some(format!(
                "{} has more torpedo tubes on one side than tubes",
                self.id
            ));
        }
        if self.torpedo_tubes > 0 && self.torpedo_tubes_one_side == 0 {
            return Some(format!(
                "{} has torpedo tubes but none that bear on one side",
                self.id
            ));
        }
        None
    }
}

/// What an aircraft type is for.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AircraftRole {
    /// Long-range search aircraft.
    Patrol,
    /// Aircraft that strike ships (decision 0026).
    Bomber,
    /// Fighters: combat air patrol and escort, attacking nothing on the
    /// surface (decision 0027). Added at the end.
    Fighter,
}

impl AircraftRole {
    /// A stable number for the role. Never renumber it; add new roles at the
    /// end.
    pub const fn code(self) -> u8 {
        match self {
            AircraftRole::Patrol => 1,
            AircraftRole::Bomber => 2,
            AircraftRole::Fighter => 3,
        }
    }
}

/// What an aircraft type attacks ships with (decision 2 of
/// `docs/decisions/0026-air-strikes.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AttackWeapon {
    /// Nothing: the type cannot be given the naval strike mission.
    None,
    Bomb,
    Torpedo,
}

impl AttackWeapon {
    /// A stable number for the weapon. Never renumber it; add new weapons at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            AttackWeapon::None => 1,
            AttackWeapon::Bomb => 2,
            AttackWeapon::Torpedo => 3,
        }
    }
}

/// One type of aircraft.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AircraftType {
    pub id: AircraftTypeId,
    pub name: String,
    pub role: AircraftRole,
    /// The furthest a search by this type can reach, in nautical miles. Zero
    /// for a type that cannot search.
    pub search_range_nm: Fixed,
    /// The width of sea one aircraft covers as it flies, in nautical miles.
    /// Zero for a type that cannot search.
    pub sweep_width_nm: Fixed,
    /// The furthest a strike by this type can reach, in nautical miles. Zero
    /// for a type that cannot strike.
    pub strike_range_nm: Fixed,
    /// What the type attacks ships with.
    pub attack_weapon: AttackWeapon,
    /// The damage one hit of the attack weapon does, before the target's
    /// toughness. Zero for a type that attacks with nothing.
    pub attack_damage: Fixed,
    /// Whether the type's pilots are armoured (decision 11 of
    /// `docs/decisions/0029-figures-for-carriers-and-aircraft.md`).
    pub pilot_armour: bool,
    /// Whether the type's fuel tanks are self-sealing (decision 11 of 0029).
    pub self_sealing_tanks: bool,
}

impl AircraftType {
    /// Describes what is wrong with the type's numbers, if anything.
    pub fn problem(&self) -> Option<String> {
        // Only a type that cannot search may have no search range.
        if self.role == AircraftRole::Patrol {
            let positive = [
                ("search range", self.search_range_nm),
                ("sweep width", self.sweep_width_nm),
            ];
            for (label, value) in positive {
                if value <= Fixed::ZERO {
                    return Some(format!("{} has no positive {label}", self.id));
                }
            }
        }
        let not_negative = [
            ("search range", self.search_range_nm),
            ("sweep width", self.sweep_width_nm),
            ("strike range", self.strike_range_nm),
            ("attack damage", self.attack_damage),
        ];
        for (label, value) in not_negative {
            if value.is_negative() {
                return Some(format!("{} has a negative {label}", self.id));
            }
        }
        if self.role == AircraftRole::Fighter && self.attack_weapon != AttackWeapon::None {
            return Some(format!(
                "{} is a fighter and must attack with nothing",
                self.id
            ));
        }
        if self.attack_weapon != AttackWeapon::None
            && (self.strike_range_nm <= Fixed::ZERO || self.attack_damage <= Fixed::ZERO)
        {
            return Some(format!(
                "{} attacks with a weapon but has no positive strike range and attack damage",
                self.id
            ));
        }
        None
    }
}

/// The figures of the search, report and contact rules. See
/// `docs/decisions/0012-search-and-contacts.md`. Every chance-like value is
/// between 0 and 1.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SearchFigures {
    /// A contact that reaches this age in pulses, unrefreshed, is dropped
    /// (decision 0032).
    pub contact_drop_age_pulses: u32,
    /// A report refreshes a contact at most this many hexes away...
    pub merge_distance_hexes: u32,
    /// ...and no older than this many pulses. The same figure says how old a
    /// contact may be and still alert a force (decision 0032).
    pub fresh_contact_max_age_pulses: u32,
    /// Chance a report names a neighbouring hex.
    pub position_error_chance: Fixed,
    /// Chance a report counts one ship too many.
    pub count_over_chance: Fixed,
    /// Chance a report counts one ship too few.
    pub count_under_chance: Fixed,
    /// True kind, then reported kind, then the chance of that report. Each
    /// true kind's row adds to exactly 1.
    pub kind_confusion: BTreeMap<ShipKind, BTreeMap<ShipKind, Fixed>>,
}

impl SearchFigures {
    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        let in_unit = |value: Fixed| value >= Fixed::ZERO && value <= Fixed::ONE;
        let chances = [
            ("position error chance", self.position_error_chance),
            ("count over chance", self.count_over_chance),
            ("count under chance", self.count_under_chance),
        ];
        for (label, value) in chances {
            if !in_unit(value) {
                return Some(format!("search {label} {value} is not between 0 and 1"));
            }
        }
        if self.contact_drop_age_pulses == 0 {
            return Some("search contact drop age is 0 pulses".to_string());
        }
        if self.count_over_chance + self.count_under_chance > Fixed::ONE {
            return Some("search count over and under chances add to more than 1".to_string());
        }
        for kind in ShipKind::ALL {
            let Some(row) = self.kind_confusion.get(&kind) else {
                return Some(format!("search kind confusion has no row for {kind:?}"));
            };
            let mut total = Fixed::ZERO;
            for (reported, chance) in row {
                if !in_unit(*chance) {
                    return Some(format!(
                        "search kind confusion {kind:?} as {reported:?} is {chance}, not between 0 and 1"
                    ));
                }
                total += *chance;
            }
            if total != Fixed::ONE {
                return Some(format!(
                    "search kind confusion row for {kind:?} adds to {total}, not 1"
                ));
            }
        }
        None
    }
}

/// The figures of the surface combat rule. See
/// `docs/decisions/0014-surface-combat.md`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SurfaceCombatFigures {
    /// Minutes in one round of a fight. Above zero.
    pub round_minutes: u32,
    /// Rounds in a fight at most. Above zero.
    pub max_rounds: u32,
    /// Chance a force sights the enemy in a round, at night.
    pub sighting_chance_night: Fixed,
    /// The same by day.
    pub sighting_chance_day: Fixed,
    /// Added to the sighting chance if any ship afloat has radar.
    pub radar_sighting_bonus: Fixed,
    /// Multiplies the sighting chance of a force that is not alert.
    pub not_alert_sighting_factor: Fixed,
    /// A force at normal readiness is alerted by a contact its side holds
    /// within this many hexes of it (decision 3 of
    /// `docs/decisions/0021-readiness-and-alarm.md`) and no older than
    /// `SearchFigures::fresh_contact_max_age_pulses` (decision 0032).
    pub alert_contact_distance_hexes: u32,
    /// Chance the alarm of a finished fight alerts another force in its hex
    /// (decision 5 of 0021). From 0 to 1.
    pub alarm_chance: Fixed,
    /// Chance one gun shot hits, at night.
    pub gun_hit_chance_night: Fixed,
    /// The same by day.
    pub gun_hit_chance_day: Fixed,
    /// Chance one torpedo hits, at night.
    pub torpedo_hit_chance_night: Fixed,
    /// The same by day.
    pub torpedo_hit_chance_day: Fixed,
    /// Multiplies the chance to hit a surprised target.
    pub surprised_target_factor: Fixed,
    /// Multiplies the chance to hit a target lit by fire, at night.
    pub lit_target_factor: Fixed,
    /// Share of gun and bomb hits that land on flotation. The rest land on
    /// fire (decision 0032).
    pub gun_hit_flotation: Fixed,
    /// Chance, for each damage point a hit does, that the hit also knocks a
    /// system down one level; 1 at most (decision 0033). Guns, torpedoes and
    /// bombs alike. Not negative; may exceed 1.
    pub system_hit_chance_per_point: Fixed,
    /// Multiplies what a degraded system gives.
    pub degraded_factor: Fixed,
    /// Multiplies the gun hit chance of a firer whose fire control is out.
    pub fire_control_out_factor: Fixed,
    /// Which system a gun system hit lands on: each system's share. A system
    /// absent from the table has share 0. The shares add to 1.
    pub gun_system_hit: BTreeMap<ShipSystem, Fixed>,
    /// The same for a torpedo system hit.
    pub torpedo_system_hit: BTreeMap<ShipSystem, Fixed>,
    /// Escorts given to a straggler sent home, by its kind. Has an entry for
    /// every kind.
    pub straggler_escorts: BTreeMap<ShipKind, u32>,
    /// Flotation damage at which a ship counts as crippled.
    pub crippled_flotation: Fixed,
    /// Fire damage at which the enemy believes a ship is sinking.
    pub claimed_sunk_fire: Fixed,
    /// How much a gun shot's chance to hit, in a dark fight, rises with the
    /// firing ship's experience above 2 and falls with it below (decision
    /// 0064). From 0 to 0.5. At 0 experience does nothing to it.
    pub experience_gun_strength: Fixed,
    /// The same for a torpedo's chance to hit.
    pub experience_torpedo_strength: Fixed,
    /// The same for a force's chance to sight the enemy, which reads the
    /// mean experience of the ships it has afloat.
    pub experience_sighting_strength: Fixed,
    /// The share of its gun shots a force fires, for the whole of a fight,
    /// when it was not ready as the fight began (decision 0068). From 0.1 to
    /// 1; at 1 the rule does nothing.
    pub unready_gun_share: Fixed,
    /// The share of its torpedo shots a force fires, in the one round in
    /// which its torpedoes are fired, when it was not ready as the fight
    /// began (decision 0071). From 0.1 to 1; at 1 the rule does nothing.
    pub unready_torpedo_share: Fixed,
    /// Forward repair mends flotation damage only below this (decision 0069).
    /// From 0 to 100.
    pub forward_repair_max_flotation: Fixed,
    /// The points a forward base must put into one degraded system to make it
    /// work (decision 0069). Above 0.
    pub forward_repair_system_points: Fixed,
    /// Days of yard repair for each point of flotation damage (decision 0069).
    /// Not negative.
    pub yard_days_per_flotation_point: Fixed,
    /// Days of yard repair for each system degraded. Not negative.
    pub yard_days_per_system_degraded: Fixed,
    /// Days of yard repair for each system out. Not negative and not below
    /// `yard_days_per_system_degraded`.
    pub yard_days_per_system_out: Fixed,
    /// The figures of a night's shore bombardment (decision 0092). `None`
    /// when the file names none of them: then no force bombards.
    pub bombardment: Option<BombardmentFigures>,
}

/// The figures of shore bombardment (decision 0092, 2, 3, 5 and 17).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BombardmentFigures {
    /// Minutes a ship fires in a night's bombardment. Above 0.
    pub minutes: u32,
    /// The chance a weighed shot hits something the base table names. From
    /// 0 to 1.
    pub hit_chance: Fixed,
    /// The gun damage of a shell that counts as one shot. Above 0.
    pub reference_gun_damage: Fixed,
    /// The chance a weighed shot that destroyed no aircraft is claimed as
    /// one. From 0 to 1.
    pub false_claim_chance: Fixed,
}

impl BombardmentFigures {
    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        if self.minutes == 0 {
            return Some("surface combat bombardment_minutes is not above 0".to_string());
        }
        if self.reference_gun_damage <= Fixed::ZERO {
            return Some(format!(
                "surface combat bombardment_reference_gun_damage {} is not above 0",
                self.reference_gun_damage
            ));
        }
        for (label, value) in [
            ("bombardment_hit_chance", self.hit_chance),
            ("bombardment_false_claim_chance", self.false_claim_chance),
        ] {
            if value.is_negative() || value > Fixed::ONE {
                return Some(format!(
                    "surface combat {label} {value} is not between 0 and 1"
                ));
            }
        }
        None
    }
}

/// The least `unready_gun_share` and `unready_torpedo_share` may be (decisions
/// 0068, 7 and 0071).
pub const MIN_UNREADY_GUN_SHARE: Fixed = Fixed::from_raw(Fixed::SCALE / 10);

/// The most a strength of experience may be (decision 0064, 6).
pub const MAX_EXPERIENCE_STRENGTH: Fixed = Fixed::from_raw(Fixed::SCALE / 2);

/// The factor an experience gives a chance in a dark fight:
/// `1 + strength x (experience - 2)` (decision 0064, 6). `experience` is a
/// ship's value, or a force's mean in the fixed arithmetic. At a strength of
/// 0 it is exactly 1.
pub fn experience_factor(strength: Fixed, experience: Fixed) -> Fixed {
    Fixed::ONE + strength * (experience - Fixed::from_int(i64::from(Ship::DEFAULT_EXPERIENCE)))
}

impl SurfaceCombatFigures {
    /// What a system in `state` multiplies: working 1, degraded the degraded
    /// factor, out 0 (for fire control, the fire control out factor).
    pub fn system_factor(&self, system: ShipSystem, state: SystemState) -> Fixed {
        match state {
            SystemState::Working => Fixed::ONE,
            SystemState::Degraded => self.degraded_factor,
            SystemState::Out if system == ShipSystem::FireControl => self.fire_control_out_factor,
            SystemState::Out => Fixed::ZERO,
        }
    }

    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        if self.round_minutes == 0 {
            return Some("surface combat round minutes is not above 0".to_string());
        }
        if self.max_rounds == 0 {
            return Some("surface combat max rounds is not above 0".to_string());
        }
        let in_unit = |value: Fixed| value >= Fixed::ZERO && value <= Fixed::ONE;
        let chances = [
            ("night sighting chance", self.sighting_chance_night),
            ("day sighting chance", self.sighting_chance_day),
            ("radar sighting bonus", self.radar_sighting_bonus),
            ("alarm chance", self.alarm_chance),
            ("night gun hit chance", self.gun_hit_chance_night),
            ("day gun hit chance", self.gun_hit_chance_day),
            ("night torpedo hit chance", self.torpedo_hit_chance_night),
            ("day torpedo hit chance", self.torpedo_hit_chance_day),
            ("gun hit flotation share", self.gun_hit_flotation),
            ("degraded factor", self.degraded_factor),
            ("fire control out factor", self.fire_control_out_factor),
        ];
        for (label, value) in chances {
            if !in_unit(value) {
                return Some(format!(
                    "surface combat {label} {value} is not between 0 and 1"
                ));
            }
        }
        let factors = [
            ("not alert sighting factor", self.not_alert_sighting_factor),
            ("surprised target factor", self.surprised_target_factor),
            ("lit target factor", self.lit_target_factor),
            (
                "system hit chance per point",
                self.system_hit_chance_per_point,
            ),
        ];
        for (label, value) in factors {
            if value.is_negative() {
                return Some(format!("surface combat {label} {value} is negative"));
            }
        }
        let tables = [
            ("gun system hit", &self.gun_system_hit),
            ("torpedo system hit", &self.torpedo_system_hit),
        ];
        for (label, table) in tables {
            let mut total = Fixed::ZERO;
            for (system, share) in table {
                if !in_unit(*share) {
                    return Some(format!(
                        "surface combat {label} share for {} {share} is not between 0 and 1",
                        system.name()
                    ));
                }
                total += *share;
            }
            if total != Fixed::ONE {
                return Some(format!(
                    "surface combat {label} shares add to {total}, not 1"
                ));
            }
        }
        for kind in ShipKind::ALL {
            if !self.straggler_escorts.contains_key(&kind) {
                return Some(format!(
                    "surface combat straggler escorts has no entry for {}",
                    kind.name()
                ));
            }
        }
        let hundred = Fixed::from_int(100);
        let strengths = [
            ("experience gun strength", self.experience_gun_strength),
            (
                "experience torpedo strength",
                self.experience_torpedo_strength,
            ),
            (
                "experience sighting strength",
                self.experience_sighting_strength,
            ),
        ];
        for (label, value) in strengths {
            if value < Fixed::ZERO || value > MAX_EXPERIENCE_STRENGTH {
                return Some(format!(
                    "surface combat {label} {value} is not between 0 and 0.5"
                ));
            }
        }
        if self.unready_gun_share < MIN_UNREADY_GUN_SHARE || self.unready_gun_share > Fixed::ONE {
            return Some(format!(
                "surface combat unready gun share {} is not between 0.1 and 1",
                self.unready_gun_share
            ));
        }
        if self.unready_torpedo_share < MIN_UNREADY_GUN_SHARE
            || self.unready_torpedo_share > Fixed::ONE
        {
            return Some(format!(
                "surface combat unready torpedo share {} is not between 0.1 and 1",
                self.unready_torpedo_share
            ));
        }
        if self.forward_repair_max_flotation < Fixed::ZERO
            || self.forward_repair_max_flotation > hundred
        {
            return Some(format!(
                "surface combat forward repair max flotation {} is not between 0 and 100",
                self.forward_repair_max_flotation
            ));
        }
        if self.forward_repair_system_points <= Fixed::ZERO {
            return Some(format!(
                "surface combat forward repair system points {} is not above 0",
                self.forward_repair_system_points
            ));
        }
        let yard_days = [
            (
                "yard days per flotation point",
                self.yard_days_per_flotation_point,
            ),
            (
                "yard days per system degraded",
                self.yard_days_per_system_degraded,
            ),
            ("yard days per system out", self.yard_days_per_system_out),
        ];
        for (label, value) in yard_days {
            if value.is_negative() {
                return Some(format!("surface combat {label} {value} is negative"));
            }
        }
        if self.yard_days_per_system_out < self.yard_days_per_system_degraded {
            return Some(format!(
                "surface combat yard days per system out {} is below yard days per system degraded {}",
                self.yard_days_per_system_out, self.yard_days_per_system_degraded
            ));
        }
        let levels = [
            ("crippled flotation", self.crippled_flotation),
            ("claimed sunk fire", self.claimed_sunk_fire),
        ];
        for (label, value) in levels {
            if value < Fixed::ZERO || value > hundred {
                return Some(format!(
                    "surface combat {label} {value} is not between 0 and 100"
                ));
            }
        }
        if let Some(problem) = self
            .bombardment
            .as_ref()
            .and_then(BombardmentFigures::problem)
        {
            return Some(problem);
        }
        None
    }
}

/// The figures of the air strike rule. See
/// `docs/decisions/0026-air-strikes.md`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AirCombatFigures {
    /// A group strikes a contact no more than this many pulses old.
    pub strike_contact_max_age_pulses: u32,
    /// Chance one aircraft's bomb hits.
    pub bomb_hit_chance: Fixed,
    /// Chance one aircraft's torpedo hits.
    pub torpedo_hit_chance: Fixed,
    /// Chance one fighter's shot brings down an aircraft (decision 6 of
    /// `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`).
    pub fighter_hit_chance: Fixed,
    /// Chance one anti-aircraft shot brings down an aircraft (decision 7 of
    /// 0027).
    pub anti_aircraft_hit_chance: Fixed,
    /// What a shot's chance is multiplied by when the target type has pilot
    /// armour (decision 11 of 0029).
    pub pilot_armour_factor: Fixed,
    /// What a shot's chance is multiplied by when the target type has
    /// self-sealing fuel tanks (decision 11 of 0029).
    pub self_sealing_tanks_factor: Fixed,
    /// Which system a bomb's system hit lands on: each system's share. A
    /// system absent from the table has share 0. The shares add to 1.
    pub bomb_system_hit: BTreeMap<ShipSystem, Fixed>,
    /// The weight of supply one hit on a base's supply destroys, in tons
    /// (decision 9 of `docs/decisions/0034-raids-on-a-base.md`).
    pub base_supply_destroyed_per_hit_t: Fixed,
    /// The chance a bomb hit on a base lands on aircraft on the ground
    /// (decision 7 of 0034).
    pub base_hit_aircraft: Fixed,
    /// The chance a bomb hit on a base lands on its supply (decision 7 of
    /// 0034). With `base_hit_aircraft` it adds to 1 at most; the rest of the
    /// hits land on nothing.
    pub base_hit_supply: Fixed,
    /// The chance a hit on a base lands on men ashore there (decision 8 of
    /// 0092). With the other two it adds to 1 at most. A file that names no
    /// share for men gives 0, and every such hit lands on nothing.
    pub base_hit_men: Fixed,
    /// The chance a hit on a base lands on its airfield (decision 2 of
    /// 0103), counted only where the base has one. With the other three it
    /// adds to 1 at most. A file that names no share for the airfield gives
    /// 0, and every such hit lands as before.
    pub base_hit_airfield: Fixed,
    /// The chance each whole shot of fighters that missed is claimed as an
    /// aircraft shot down (decision 3 of
    /// `docs/decisions/0056-claims-from-the-air.md`).
    pub fighter_false_claim_chance: Fixed,
    /// The same for the guns of ships and bases.
    pub anti_aircraft_false_claim_chance: Fixed,
    /// The chance each whole bomb or torpedo that missed is claimed as a hit
    /// on a ship, or as an aircraft destroyed on the ground (decisions 4 and
    /// 7 of 0056).
    pub bomb_false_claim_chance: Fixed,
    /// The share of its strike radius an air group on extended reach may go
    /// beyond it (decision 2 of
    /// `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md`).
    pub extended_reach_share: Fixed,
    /// The chance an aircraft that came back from a sortie beyond its radius
    /// runs out of fuel, for each whole radius beyond (decision 3 of 0059).
    /// It may be more than 1: the chance itself is cut at 1.
    pub ditch_chance_per_radius_beyond: Fixed,
    /// The chance the crew of an aircraft out of fuel on the way home is
    /// recovered (decision 6 of 0059).
    pub crew_recovered_short_of_home: Fixed,
    /// The chance the pilot of a patrol aircraft shot down over its own base
    /// is recovered (decision 6a of 0059).
    pub crew_recovered_over_own_base: Fixed,
    /// The same for a patrol over its own ships at sea (decision 6a of 0059).
    pub crew_recovered_beside_own_force: Fixed,
    /// The chance the crew of an aircraft that came down in the sea for want
    /// of a deck, with no ship of its side in the hex, is recovered
    /// (decision 9 of 0059).
    pub crew_recovered_open_sea: Fixed,
    /// The share of the guns' shots of a raid's target that are worked before
    /// the attack, the rest after it (decision 0079). 1 is step 1's rule, all
    /// before; 0 all after.
    pub anti_aircraft_before_attack_share: Fixed,
    /// The share of a raid's patrol that the escorts draw off one for one,
    /// the rest split between the escorts not drawn off and the bombers in
    /// proportion (decision 0080). 1 is step 1's rule; 0 is the split in
    /// proportion alone.
    pub patrol_drawn_off_share: Fixed,
    /// The share of the patrol's shots at a raid's bombers that are worked
    /// before the attack, the rest after it (decision 0087). 1 is step 1's
    /// rule, all before; 0 all after.
    pub patrol_before_attack_share: Fixed,
}

impl AirCombatFigures {
    /// The air group's reach in miles for a type of strike radius
    /// `radius_nm` (decision 2 of 0059): the radius on normal reach, and on
    /// extended reach the radius times (1 + `extended_reach_share`), cut to
    /// a whole mile. Every test of how far a strike or an escort may go is
    /// made against this ([`RefData::reach_nm`]).
    pub fn reach_nm(&self, radius_nm: Fixed, reach: Reach) -> Fixed {
        match reach {
            Reach::Normal => radius_nm,
            Reach::Extended => {
                Fixed::from_int((radius_nm * (Fixed::ONE + self.extended_reach_share)).trunc())
            }
        }
    }

    /// The share of its radius a sortie of one-way distance `distance_nm`
    /// went beyond it: (distance less radius) over radius, cut to millionths.
    /// 0 for a sortie within the radius or for a type with no radius.
    pub fn share_beyond_radius(distance_nm: Fixed, radius_nm: Fixed) -> Fixed {
        if radius_nm <= Fixed::ZERO || distance_nm <= radius_nm {
            return Fixed::ZERO;
        }
        (distance_nm - radius_nm).mul_div(Fixed::ONE, radius_nm)
    }

    /// The chance an aircraft that came back runs out of fuel (decision 3 of
    /// 0059): `ditch_chance_per_radius_beyond` times the share beyond the
    /// radius, the share cut first, then the product, and 1 at most.
    pub fn ditch_chance(&self, distance_nm: Fixed, radius_nm: Fixed) -> Fixed {
        (self.ditch_chance_per_radius_beyond * Self::share_beyond_radius(distance_nm, radius_nm))
            .min(Fixed::ONE)
    }

    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        let in_unit = |value: Fixed| value >= Fixed::ZERO && value <= Fixed::ONE;
        // Decisions 2, 3 and 6 of 0059. A reach beyond double the radius, or
        // a chance per radius beyond of more than 100, is a mistake.
        for (label, value) in [
            ("extended reach share", self.extended_reach_share),
            (
                "crew recovered short of home",
                self.crew_recovered_short_of_home,
            ),
            (
                "crew recovered over own base",
                self.crew_recovered_over_own_base,
            ),
            (
                "crew recovered beside own force",
                self.crew_recovered_beside_own_force,
            ),
            ("crew recovered in open sea", self.crew_recovered_open_sea),
            (
                "anti-aircraft before attack share",
                self.anti_aircraft_before_attack_share,
            ),
            ("patrol drawn off share", self.patrol_drawn_off_share),
            (
                "patrol before attack share",
                self.patrol_before_attack_share,
            ),
        ] {
            if !in_unit(value) {
                return Some(format!("air combat {label} {value} is not between 0 and 1"));
            }
        }
        if self.ditch_chance_per_radius_beyond < Fixed::ZERO
            || self.ditch_chance_per_radius_beyond > Fixed::from_int(100)
        {
            return Some(format!(
                "air combat ditch chance per radius beyond {} is not between 0 and 100",
                self.ditch_chance_per_radius_beyond
            ));
        }
        let chances = [
            ("bomb hit chance", self.bomb_hit_chance),
            ("torpedo hit chance", self.torpedo_hit_chance),
            ("fighter hit chance", self.fighter_hit_chance),
            ("anti-aircraft hit chance", self.anti_aircraft_hit_chance),
            ("pilot armour factor", self.pilot_armour_factor),
            ("self-sealing tanks factor", self.self_sealing_tanks_factor),
            (
                "fighter false claim chance",
                self.fighter_false_claim_chance,
            ),
            (
                "anti-aircraft false claim chance",
                self.anti_aircraft_false_claim_chance,
            ),
            ("bomb false claim chance", self.bomb_false_claim_chance),
        ];
        for (label, value) in chances {
            if !in_unit(value) {
                return Some(format!("air combat {label} {value} is not between 0 and 1"));
            }
        }
        let mut total = Fixed::ZERO;
        for (system, share) in &self.bomb_system_hit {
            if !in_unit(*share) {
                return Some(format!(
                    "air combat bomb system hit share for {} {share} is not between 0 and 1",
                    system.name()
                ));
            }
            total += *share;
        }
        if total != Fixed::ONE {
            return Some(format!(
                "air combat bomb system hit shares add to {total}, not 1"
            ));
        }
        for (label, value) in [
            ("aircraft", self.base_hit_aircraft),
            ("supply", self.base_hit_supply),
            ("men", self.base_hit_men),
            ("airfield", self.base_hit_airfield),
        ] {
            if !in_unit(value) {
                return Some(format!(
                    "air combat base hit chance on {label} {value} is not between 0 and 1"
                ));
            }
        }
        let landed = self.base_hit_aircraft
            + self.base_hit_supply
            + self.base_hit_men
            + self.base_hit_airfield;
        if landed > Fixed::ONE {
            return Some(format!(
                "air combat base hit chances on aircraft, supply, men and airfield add to {landed}, more than 1"
            ));
        }
        if self.base_supply_destroyed_per_hit_t.is_negative() {
            return Some(format!(
                "air combat base supply destroyed per hit {} is negative",
                self.base_supply_destroyed_per_hit_t
            ));
        }
        None
    }
}

/// What a port of one size can handle. See
/// `docs/decisions/0011-cargo-handling.md`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PortSize {
    /// Ships that can work cargo alongside at once.
    pub berths: u32,
    /// Tons per hour moved by one ship on a berth.
    pub berth_rate_t_per_h: Fixed,
    /// Tons per hour moved by one ship without a berth, working by lighter.
    pub lighter_rate_t_per_h: Fixed,
}

impl PortSize {
    /// Describes what is wrong with the figures for port size `size`, if
    /// anything.
    pub fn problem(&self, size: u8) -> Option<String> {
        if self.berth_rate_t_per_h.is_negative() {
            return Some(format!("port size {size} has a negative berth rate"));
        }
        if self.lighter_rate_t_per_h.is_negative() {
            return Some(format!("port size {size} has a negative lighter rate"));
        }
        if self.berths > 0 && self.berth_rate_t_per_h.is_zero() {
            return Some(format!(
                "port size {size} has berths but no positive berth rate"
            ));
        }
        None
    }
}

/// All reference data for a game.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RefData {
    pub ship_classes: BTreeMap<ShipClassId, ShipClass>,
    /// Cargo handling figures by port size, 0 to 10.
    pub port_sizes: BTreeMap<u8, PortSize>,
    pub aircraft_types: BTreeMap<AircraftTypeId, AircraftType>,
    /// The search figures. `None` only in unit tests that have no air
    /// groups; the loader always supplies it.
    pub search: Option<SearchFigures>,
    /// The surface combat figures. `None` only in unit tests that have no
    /// fights; the loader always supplies it.
    pub surface_combat: Option<SurfaceCombatFigures>,
    /// The air combat figures of decision 0026. `None` if the scenario names
    /// no file: no air group can then be given the naval strike mission.
    pub air_combat: Option<AirCombatFigures>,
    /// The wear and repair figures of decision 0035. `None` if the scenario
    /// names no file: no air group is worn, none is repaired and nothing is
    /// recorded.
    pub air_upkeep: Option<AirUpkeepFigures>,
    /// The fatigue figures of decision 0036. `None` if the scenario names no
    /// file: no air group tires or rests, none is stood down and nothing is
    /// recorded.
    pub fatigue: Option<FatigueFigures>,
    /// The base supply figures of decision 0039. `None` if the scenario
    /// names no file: nothing is spent, no base is out of supply and nothing
    /// is recorded.
    pub air_supply: Option<AirSupplyFigures>,
    /// The ground figures of decision 0066. `None` if the scenario names no
    /// file: no man eats and the men's supply phase does nothing. A scenario
    /// with a land unit must name one.
    pub ground: Option<GroundFigures>,
}

impl RefData {
    /// How far a strike or an escort of an air group of type `aircraft_type`
    /// may go with the reach `reach` in force, in miles: the type's strike
    /// radius on normal reach, and the extended reach of
    /// [`AirCombatFigures::reach_nm`] on extended. This is the one function
    /// every test of a strike's or an escort's distance is made against
    /// (decision 2 of 0059): the strike decision, the base strike, the
    /// escorts, `measure` and the briefing's warning. Without the air combat
    /// figures no group can be on extended reach, and the answer is the
    /// radius. A type that does not exist has none: 0.
    pub fn reach_nm(&self, aircraft_type: AircraftTypeId, reach: Reach) -> Fixed {
        let radius = self
            .aircraft_types
            .get(&aircraft_type)
            .map_or(Fixed::ZERO, |aircraft| aircraft.strike_range_nm);
        match &self.air_combat {
            Some(figures) => figures.reach_nm(radius, reach),
            None => radius,
        }
    }

    /// How far an air group of type `aircraft_type` may fly to another base
    /// in one move (decision 3 of 0091): twice the longer of the type's search
    /// range and its strike radius, the distance it flies out and back, flown
    /// one way. The group's reach does not change it. This is the one
    /// function the rebase order, `measure` and the briefing read. A type
    /// that does not exist has none: 0.
    pub fn transfer_range_nm(&self, aircraft_type: AircraftTypeId) -> Fixed {
        self.aircraft_types
            .get(&aircraft_type)
            .map_or(Fixed::ZERO, |aircraft| {
                aircraft.search_range_nm.max(aircraft.strike_range_nm) * 2
            })
    }
}

/// The figures of the base supply rule. See
/// `docs/decisions/0039-base-supply-consumption.md`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AirSupplyFigures {
    /// Supply one aircraft's sortie costs its base, in tons.
    pub sortie_supply_t: Fixed,
    /// The supply added for each 100 nm of the sortie.
    pub sortie_supply_t_per_100_nm: Fixed,
    /// Supply each aircraft at a base, ready or damaged, costs it a day.
    pub daily_supply_t_per_aircraft: Fixed,
    /// Supply one repair at a base costs.
    pub repair_supply_t: Fixed,
}

impl AirSupplyFigures {
    /// What one aircraft's sortie of `distance_nm` costs its base (decision
    /// 1 of 0039): the figure for the sortie plus the figure per 100 nm
    /// times the distance over 100.
    pub fn sortie_cost(&self, distance_nm: Fixed) -> Fixed {
        self.sortie_supply_t
            + self
                .sortie_supply_t_per_100_nm
                .mul_div(distance_nm, Fixed::from_int(100))
    }

    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        for (label, value) in [
            ("sortie supply", self.sortie_supply_t),
            ("sortie supply per 100 nm", self.sortie_supply_t_per_100_nm),
            (
                "daily supply per aircraft",
                self.daily_supply_t_per_aircraft,
            ),
            ("repair supply", self.repair_supply_t),
        ] {
            if value.is_negative() {
                return Some(format!("air {label} {value} is negative"));
            }
        }
        None
    }
}

/// The figures of land units carried by sea and fed from a base
/// (`docs/decisions/0066-land-units-carried-and-fed.md`), and of the fight
/// ashore and a base that changes side
/// (`docs/decisions/0090-the-fight-ashore-and-a-base-that-changes-side.md`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GroundFigures {
    /// The tons of a ship's work in a pulse that moving one man takes
    /// (decision 7 of 0066).
    pub handling_t_per_man: Fixed,
    /// Supply a man ashore costs his base a day, in tons (decision 16 of
    /// 0066).
    pub daily_supply_t_per_man: Fixed,
    /// The share of unfed men lost in a day (decision 19 of 0066).
    pub unfed_loss_share_per_day: Fixed,
    /// Firing men to one shot (decision 7 of 0090).
    pub men_per_shot: Fixed,
    /// The chance a shot hits, for a side that attacks (decision 7 of 0090).
    pub hit_chance: Fixed,
    /// The chance a shot hits, for a side that defends (decision 7 of 0090).
    pub defender_hit_chance: Fixed,
    /// Men a hit removes, a whole number (decision 9 of 0090).
    pub men_per_hit: Fixed,
    /// Fatigue a unit on attack gains on a day it fired (decision 12 of
    /// 0090).
    pub fatigue_per_attack_day: Fixed,
    /// Fatigue every other unit ashore loses in a day (decision 12 of 0090).
    pub rest_per_day: Fixed,
    /// Above this fatigue a unit on attack goes back to defend (decision 14
    /// of 0090).
    pub attack_fatigue_limit: Fixed,
    /// A side attacked is beaten below this share of the attacker's men
    /// (decision 15 of 0090).
    pub beaten_share: Fixed,
    /// The share of a base's supply and fuel its taker keeps (decision 19 of
    /// 0090).
    pub captured_stock_share: Fixed,
    /// Men a bomb or shell hit on men removes, a whole number (decision 9 of
    /// 0092). A file that names none gives 0: a hit on men removes nobody.
    pub men_lost_per_base_hit: Fixed,
    /// The bounds of a ground report's error (decision 14 of 0092). `None`
    /// when the file names neither: then no ground report is made.
    pub report_error: Option<ReportError>,
    /// Days to march from a hex to a neighbour, at least 1 (decision 4 of
    /// 0094).
    pub march_days_per_hex: u32,
    /// Fatigue a unit gains on a day it marches (decision 8 of 0094).
    pub fatigue_per_march_day: Fixed,
    /// Days of its daily charge a marching unit takes with it (decision 9 of
    /// 0094).
    pub march_supply_days: Fixed,
    /// Preparation (loading and rehearsal) a force and each unit of its
    /// roster gain for a day in port with the order, from 0 to 100
    /// (decision 4 of 0095).
    pub preparation_per_day: Fixed,
    /// The share of a ship's unloading rate a force with no loading works
    /// at while it lands men, from 0 to 1 (decision 5 of 0095). 1 switches
    /// the rule off.
    pub unprepared_landing_share: Fixed,
    /// The share of its loading a force keeps when a unit is added to its
    /// roster, from 0 to 1 (decisions 9 and 10 of 0095).
    pub preparation_kept_on_change: Fixed,
    /// Fatigue a unit gains on landing with no rehearsal, from 0 to 100
    /// (decision 6 of 0095). 0 switches the rule off.
    pub unprepared_landing_fatigue: Fixed,
    /// How much a step of experience moves the chance a shot hits, from 0 to
    /// 0.5 so that the factor is never below 0 (decision 2 of 0097). 0
    /// switches the rule off.
    pub experience_fire_strength: Fixed,
    /// Days of training for one step of experience, at least 1 (decision 3
    /// of 0097).
    pub training_days_per_level: u32,
    /// The experience training cannot pass, from 0 to 4 (decision 3 of
    /// 0097).
    pub training_ceiling: u32,
    /// Days of firing in a fight for one step of experience, at least 1
    /// (decision 5 of 0097).
    pub fight_days_per_level: u32,
    /// How much a step of the assault skill moves the rehearsal gained in a
    /// day (decision 3 of 0098). 0 switches the skill off.
    pub assault_skill_strength: Fixed,
    /// How much a step of a movement skill moves a day's march (decision 3
    /// of 0098). 0 switches the skill off.
    pub movement_skill_strength: Fixed,
    /// How much a step of the holding skill moves a defender's chance to hit
    /// (decision 3 of 0098). 0 switches the skill off.
    pub holding_skill_strength: Fixed,
    /// How much a step of weight moves a unit's shots and the supply each of
    /// its men costs (decision 3 of 0098). 0 switches the skill off.
    pub weight_strength: Fixed,
    /// Work a man of a standard working unit makes in a day (decision 3 of
    /// 0099), 0 or more.
    pub work_per_man_day: Fixed,
    /// Work to build an airfield (decision 5 of 0099), above 0.
    pub airfield_work: Fixed,
    /// Work in a day that doubles the next day's unloading at a base
    /// (decision 4 of 0099), above 0.
    pub work_to_double_unloading: Fixed,
    /// Work in a day for one more aircraft repaired the next night (decision
    /// 4 of 0099), above 0.
    pub work_per_aircraft_repair: Fixed,
    /// Work for one point of a base's defences (decision 4 of 0099), above 0.
    pub work_per_defence_point: Fixed,
    /// How much full defences raise a defender's chance to hit (decision 4
    /// of 0099), from 0 to 1. 0 switches defences off in the fight.
    pub defences_strength: Fixed,
    /// The share of a working unit's work that a fighting unit set to work
    /// makes (decision 3a of 0099), from 0 to 1.
    pub fighting_unit_work_share: Fixed,
    /// Damage one hit on an airfield adds (decision 3 of 0103), 0 or more.
    pub airfield_damage_per_hit: Fixed,
    /// Work to mend one point of an airfield's damage (decision 7 of 0103),
    /// above 0.
    pub work_per_airfield_damage_point: Fixed,
    /// The damage at or above which a field takes no aircraft (decision 5 of
    /// 0103), from 0 to 100.
    pub airfield_landing_refused_at: Fixed,
    /// Fatigue a unit gains when its side is beaten and it falls back
    /// (decision 1 of 0104), from 0 to 100.
    pub beaten_fatigue: Fixed,
    /// The most firing men a side counts in a day's fight, as a multiple of
    /// the other side's fighting men ashore in the hex at the start of the
    /// fight (decision 1 of 0105), 1 or more.
    pub engagement_ratio: Fixed,
    /// Fatigue a unit on attack gains on a day its side's attack failed,
    /// beside the fatigue of a day's attack (decision 5 of 0105), from 0 to
    /// 100.
    pub failed_attack_fatigue: Fixed,
    /// The share of the men lost to bombs and shells that full cover saves
    /// (decision 2 of 0106), from 0 to 1. 0 switches the rule off.
    pub cover_from_fire: Fixed,
    /// The share of a defending unit's losses in the fight that full cover
    /// saves (decision 3 of 0106), from 0 to 1.
    pub cover_in_fight: Fixed,
    /// Fieldworks a fighting unit on the defence digs in a day (decision 5
    /// of 0106), from 0 to 100.
    pub fieldworks_per_day: Fixed,
    /// The most fieldworks a unit digs for itself (decisions 5 and 8 of
    /// 0106), the top of the improvised grade: from 0 to `prepared_from`.
    pub fieldworks_limit: Fixed,
    /// The cover at which the prepared grade begins (decision 7 of 0106).
    pub prepared_from: Fixed,
    /// The cover at which the fortified grade begins (decision 7 of 0106),
    /// from `prepared_from` to 100.
    pub fortified_from: Fixed,
    /// How many times dearer a point of defences is in the prepared grade
    /// (decision 8 of 0106), 1 or more.
    pub prepared_work_factor: Fixed,
    /// How many times dearer than in the improvised grade a point is in the
    /// fortified grade (decision 8 of 0106), 1 or more.
    pub fortified_work_factor: Fixed,
    /// Fatigue a unit gains for each hit on its side's men in its hex in a
    /// day (decision 10 of 0106), from 0 to 100.
    pub strain_per_hit: Fixed,
    /// The most fatigue strain adds in a day (decision 10 of 0106), from 0
    /// to 100.
    pub strain_per_day: Fixed,
    /// The fatigue above which strain adds nothing (decision 10 of 0106),
    /// from 0 to 100.
    pub strain_limit: Fixed,
    /// The share of strain that full cover saves (decision 10 of 0106), from
    /// 0 to 1.
    pub cover_from_strain: Fixed,
    /// The cover a hit on men wears away (decision 12 of 0106), from 0 to
    /// 100.
    pub cover_wear_per_hit: Fixed,
}

/// The least and the most a ground report can be, as a share of the truth
/// (decision 14 of 0092).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReportError {
    pub low: Fixed,
    pub high: Fixed,
}

impl GroundFigures {
    /// The first values of the figures (decision 0066, "Placeholder
    /// figures", and decision 0090, "Placeholder figures"), for tests that
    /// build a world by hand.
    #[cfg(test)]
    pub(crate) const FIRST: GroundFigures = GroundFigures {
        handling_t_per_man: Fixed::from_raw(250_000),
        daily_supply_t_per_man: Fixed::from_raw(5_000),
        unfed_loss_share_per_day: Fixed::from_raw(50_000),
        men_per_shot: Fixed::from_int(100),
        hit_chance: Fixed::from_raw(300_000),
        defender_hit_chance: Fixed::from_raw(600_000),
        men_per_hit: Fixed::from_int(20),
        fatigue_per_attack_day: Fixed::from_int(20),
        rest_per_day: Fixed::from_int(5),
        attack_fatigue_limit: Fixed::from_int(60),
        beaten_share: Fixed::from_raw(200_000),
        captured_stock_share: Fixed::from_raw(500_000),
        men_lost_per_base_hit: Fixed::from_int(10),
        report_error: Some(ReportError {
            low: Fixed::from_raw(500_000),
            high: Fixed::from_raw(1_500_000),
        }),
        march_days_per_hex: 3,
        fatigue_per_march_day: Fixed::from_int(10),
        march_supply_days: Fixed::from_int(5),
        preparation_per_day: Fixed::from_int(5),
        unprepared_landing_share: Fixed::from_raw(500_000),
        preparation_kept_on_change: Fixed::from_raw(500_000),
        unprepared_landing_fatigue: Fixed::from_int(40),
        experience_fire_strength: Fixed::from_raw(250_000),
        training_days_per_level: 30,
        training_ceiling: 2,
        fight_days_per_level: 10,
        assault_skill_strength: Fixed::from_raw(250_000),
        movement_skill_strength: Fixed::from_raw(250_000),
        holding_skill_strength: Fixed::from_raw(150_000),
        weight_strength: Fixed::from_raw(250_000),
        work_per_man_day: Fixed::ONE,
        airfield_work: Fixed::from_int(20_000),
        work_to_double_unloading: Fixed::from_int(1_000),
        work_per_aircraft_repair: Fixed::from_int(200),
        work_per_defence_point: Fixed::from_int(200),
        defences_strength: Fixed::from_raw(500_000),
        fighting_unit_work_share: Fixed::from_raw(500_000),
        airfield_damage_per_hit: Fixed::from_int(2),
        work_per_airfield_damage_point: Fixed::from_int(50),
        airfield_landing_refused_at: Fixed::from_int(50),
        beaten_fatigue: Fixed::from_int(30),
        engagement_ratio: Fixed::from_int(1_000_000),
        failed_attack_fatigue: Fixed::ZERO,
        cover_from_fire: Fixed::ZERO,
        cover_in_fight: Fixed::ZERO,
        fieldworks_per_day: Fixed::ZERO,
        fieldworks_limit: Fixed::ZERO,
        prepared_from: Fixed::from_int(30),
        fortified_from: Fixed::from_int(60),
        prepared_work_factor: Fixed::ONE,
        fortified_work_factor: Fixed::ONE,
        strain_per_hit: Fixed::ZERO,
        strain_per_day: Fixed::ZERO,
        strain_limit: Fixed::from_int(60),
        cover_from_strain: Fixed::ZERO,
        cover_wear_per_hit: Fixed::ZERO,
    };

    /// Describes what is wrong with the figures, if anything. The weight of
    /// a man, his daily charge and the men to a shot divide, so they must be
    /// above 0; a hit removes a whole number of men, at least 1; chances and
    /// shares are from 0 to 1, and the beaten share below 1 so that both
    /// sides cannot be beaten at once (decision 15 of 0090); fatigue figures
    /// are from 0 to 100; a march takes at least a day and carries no less
    /// than nothing (decision 0094).
    pub fn problem(&self) -> Option<String> {
        for (label, value) in [
            ("handling_t_per_man", self.handling_t_per_man),
            ("daily_supply_t_per_man", self.daily_supply_t_per_man),
            ("men_per_shot", self.men_per_shot),
            ("men_per_hit", self.men_per_hit),
        ] {
            if value <= Fixed::ZERO {
                return Some(format!("ground {label} {value} is not above 0"));
            }
        }
        if self.men_per_hit != Fixed::from_int(self.men_per_hit.trunc()) {
            return Some(format!(
                "ground men_per_hit {} is not a whole number",
                self.men_per_hit
            ));
        }
        for (label, share) in [
            ("unfed_loss_share_per_day", self.unfed_loss_share_per_day),
            ("hit_chance", self.hit_chance),
            ("defender_hit_chance", self.defender_hit_chance),
            ("captured_stock_share", self.captured_stock_share),
        ] {
            if share.is_negative() || share > Fixed::ONE {
                return Some(format!("ground {label} {share} is not between 0 and 1"));
            }
        }
        if self.beaten_share.is_negative() || self.beaten_share >= Fixed::ONE {
            return Some(format!(
                "ground beaten_share {} is not from 0 to below 1",
                self.beaten_share
            ));
        }
        for (label, value) in [
            ("fatigue_per_attack_day", self.fatigue_per_attack_day),
            ("rest_per_day", self.rest_per_day),
            ("attack_fatigue_limit", self.attack_fatigue_limit),
            ("fatigue_per_march_day", self.fatigue_per_march_day),
        ] {
            if value.is_negative() || value > Fixed::from_int(100) {
                return Some(format!("ground {label} {value} is not between 0 and 100"));
            }
        }
        if self.men_lost_per_base_hit.is_negative()
            || self.men_lost_per_base_hit != Fixed::from_int(self.men_lost_per_base_hit.trunc())
        {
            return Some(format!(
                "ground men_lost_per_base_hit {} is not a whole number of 0 or more",
                self.men_lost_per_base_hit
            ));
        }
        if self.march_days_per_hex == 0 {
            return Some("ground march_days_per_hex 0 is not 1 or more".to_string());
        }
        if self.march_supply_days.is_negative() {
            return Some(format!(
                "ground march_supply_days {} is negative",
                self.march_supply_days
            ));
        }
        // Preparation and experience (decisions 0095 and 0097).
        for (label, value) in [
            ("preparation_per_day", self.preparation_per_day),
            (
                "unprepared_landing_fatigue",
                self.unprepared_landing_fatigue,
            ),
        ] {
            if value.is_negative() || value > Fixed::from_int(100) {
                return Some(format!("ground {label} {value} is not between 0 and 100"));
            }
        }
        for (label, share) in [
            ("unprepared_landing_share", self.unprepared_landing_share),
            (
                "preparation_kept_on_change",
                self.preparation_kept_on_change,
            ),
        ] {
            if share.is_negative() || share > Fixed::ONE {
                return Some(format!("ground {label} {share} is not between 0 and 1"));
            }
        }
        if self.experience_fire_strength.is_negative()
            || self.experience_fire_strength > Fixed::from_ratio(1, 2)
        {
            return Some(format!(
                "ground experience_fire_strength {} is not between 0 and 0.5",
                self.experience_fire_strength
            ));
        }
        for (label, days) in [
            ("training_days_per_level", self.training_days_per_level),
            ("fight_days_per_level", self.fight_days_per_level),
        ] {
            if days == 0 {
                return Some(format!("ground {label} 0 is not 1 or more"));
            }
        }
        // The strengths of the skills (decision 0098): below 0.5, so that a
        // skill of 0 never makes a factor of 0 (a march that never ends, a
        // unit that eats nothing).
        for (label, strength) in [
            ("assault_skill_strength", self.assault_skill_strength),
            ("movement_skill_strength", self.movement_skill_strength),
            ("holding_skill_strength", self.holding_skill_strength),
            ("weight_strength", self.weight_strength),
        ] {
            if strength.is_negative() || strength >= Fixed::from_ratio(1, 2) {
                return Some(format!(
                    "ground {label} {strength} is not at least 0 and below 0.5"
                ));
            }
        }
        // Work and what it builds (decision 0099): the four figures that
        // divide are above 0, the work a man makes is not below 0, and the
        // strength of defences and the share a fighting unit makes are from
        // 0 to 1.
        if self.work_per_man_day.is_negative() {
            return Some(format!(
                "ground work_per_man_day {} is negative",
                self.work_per_man_day
            ));
        }
        for (label, value) in [
            ("airfield_work", self.airfield_work),
            ("work_to_double_unloading", self.work_to_double_unloading),
            ("work_per_aircraft_repair", self.work_per_aircraft_repair),
            ("work_per_defence_point", self.work_per_defence_point),
            (
                "work_per_airfield_damage_point",
                self.work_per_airfield_damage_point,
            ),
        ] {
            if value <= Fixed::ZERO {
                return Some(format!("ground {label} {value} is not above 0"));
            }
        }
        for (label, share) in [
            ("defences_strength", self.defences_strength),
            ("fighting_unit_work_share", self.fighting_unit_work_share),
        ] {
            if share.is_negative() || share > Fixed::ONE {
                return Some(format!("ground {label} {share} is not between 0 and 1"));
            }
        }
        // The fatigue of defeat (decision 0104).
        if self.beaten_fatigue.is_negative() || self.beaten_fatigue > Fixed::from_int(100) {
            return Some(format!(
                "ground beaten_fatigue {} is not between 0 and 100",
                self.beaten_fatigue
            ));
        }
        // Who is engaged, and an attack that fails (decision 0105). A ratio
        // of 1 or more never cuts the smaller side (decision 1).
        if self.engagement_ratio < Fixed::ONE {
            return Some(format!(
                "ground engagement_ratio {} is below 1",
                self.engagement_ratio
            ));
        }
        if self.failed_attack_fatigue.is_negative()
            || self.failed_attack_fatigue > Fixed::from_int(100)
        {
            return Some(format!(
                "ground failed_attack_fatigue {} is not between 0 and 100",
                self.failed_attack_fatigue
            ));
        }
        // Cover, fieldworks and strain (decision 0106).
        for (label, share) in [
            ("cover_from_fire", self.cover_from_fire),
            ("cover_in_fight", self.cover_in_fight),
            ("cover_from_strain", self.cover_from_strain),
        ] {
            if share.is_negative() || share > Fixed::ONE {
                return Some(format!("ground {label} {share} is not between 0 and 1"));
            }
        }
        for (label, value) in [
            ("fieldworks_per_day", self.fieldworks_per_day),
            ("strain_per_hit", self.strain_per_hit),
            ("strain_per_day", self.strain_per_day),
            ("strain_limit", self.strain_limit),
            ("cover_wear_per_hit", self.cover_wear_per_hit),
        ] {
            if value.is_negative() || value > Fixed::from_int(100) {
                return Some(format!("ground {label} {value} is not between 0 and 100"));
            }
        }
        if self.prepared_from.is_negative()
            || self.fortified_from < self.prepared_from
            || self.fortified_from > Fixed::from_int(100)
        {
            return Some(format!(
                "ground prepared_from {} and fortified_from {} are not 0 or more with the prepared not above the fortified and the fortified not above 100",
                self.prepared_from, self.fortified_from
            ));
        }
        if self.fieldworks_limit.is_negative() || self.fieldworks_limit > self.prepared_from {
            return Some(format!(
                "ground fieldworks_limit {} is not between 0 and prepared_from {}",
                self.fieldworks_limit, self.prepared_from
            ));
        }
        for (label, factor) in [
            ("prepared_work_factor", self.prepared_work_factor),
            ("fortified_work_factor", self.fortified_work_factor),
        ] {
            if factor < Fixed::ONE {
                return Some(format!("ground {label} {factor} is below 1"));
            }
        }
        // The airfield's damage (decision 0103).
        if self.airfield_damage_per_hit.is_negative() {
            return Some(format!(
                "ground airfield_damage_per_hit {} is negative",
                self.airfield_damage_per_hit
            ));
        }
        if self.airfield_landing_refused_at.is_negative()
            || self.airfield_landing_refused_at > Fixed::from_int(100)
        {
            return Some(format!(
                "ground airfield_landing_refused_at {} is not between 0 and 100",
                self.airfield_landing_refused_at
            ));
        }
        if self.training_ceiling > 4 {
            return Some(format!(
                "ground training_ceiling {} is not between 0 and 4",
                self.training_ceiling
            ));
        }
        if let Some(error) = self.report_error {
            if error.low.is_negative() || error.high < error.low {
                return Some(format!(
                    "ground report_error_low {} and report_error_high {} are not 0 or more with the low not above the high",
                    error.low, error.high
                ));
            }
        }
        None
    }
}

/// The figures of the fatigue rule. See
/// `docs/decisions/0036-fatigue-and-rest.md`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FatigueFigures {
    /// Fatigue an air group gains in a daylight pulse in which it flew, for
    /// the sortie.
    pub air_fatigue_per_sortie: Fixed,
    /// The fatigue added for each 100 nm of the sortie.
    pub air_fatigue_per_100_nm: Fixed,
    /// Fatigue an air group loses in a daylight pulse in which it did not
    /// fly.
    pub air_rest_per_pulse: Fixed,
    /// Fatigue a task force gains at the start of a pulse in which it is
    /// alert (decision 0037).
    pub force_fatigue_per_pulse_alert: Fixed,
    /// Fatigue a task force loses at the start of a pulse in which it is not
    /// alert (decision 0037).
    pub force_rest_per_pulse: Fixed,
}

impl FatigueFigures {
    /// The fatigue a group gains for a sortie of `distance_nm` (decision 4
    /// of 0036): the figure for the sortie plus the figure per 100 nm times
    /// the distance over 100.
    pub fn gain(&self, distance_nm: Fixed) -> Fixed {
        self.air_fatigue_per_sortie
            + self
                .air_fatigue_per_100_nm
                .mul_div(distance_nm, Fixed::from_int(100))
    }

    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        for (label, value) in [
            ("air fatigue per sortie", self.air_fatigue_per_sortie),
            ("air fatigue per 100 nm", self.air_fatigue_per_100_nm),
            ("air rest per pulse", self.air_rest_per_pulse),
            (
                "force fatigue per alert pulse",
                self.force_fatigue_per_pulse_alert,
            ),
            ("force rest per pulse", self.force_rest_per_pulse),
        ] {
            if value < Fixed::ZERO || value > Fixed::from_int(100) {
                return Some(format!("{label} {value} is not between 0 and 100"));
            }
        }
        None
    }
}

/// The figures of the wear and repair rule. See
/// `docs/decisions/0035-wear-repair-and-operational-losses.md`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AirUpkeepFigures {
    /// Chance an aircraft that flew comes back worn, before the distance.
    pub wear_base_chance: Fixed,
    /// The chance added for each 100 nm of the sortie.
    pub wear_chance_per_100_nm: Fixed,
    /// The share of the worn aircraft that are written off.
    pub write_off_share: Fixed,
    /// Damaged aircraft a carrier repairs each day, for the groups aboard it.
    pub carrier_repairs_per_day: u32,
}

impl AirUpkeepFigures {
    /// The chance an aircraft is worn on a sortie of `distance_nm`: the base
    /// chance plus the chance per 100 nm times the distance over 100, 1 at
    /// most (decision 4 of 0035).
    pub fn wear_chance(&self, distance_nm: Fixed) -> Fixed {
        let for_distance = self
            .wear_chance_per_100_nm
            .mul_div(distance_nm, Fixed::from_int(100));
        (self.wear_base_chance + for_distance).min(Fixed::ONE)
    }

    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        for (label, value) in [
            ("wear base chance", self.wear_base_chance),
            ("wear chance per 100 nm", self.wear_chance_per_100_nm),
            ("write-off share", self.write_off_share),
        ] {
            if value < Fixed::ZERO || value > Fixed::ONE {
                return Some(format!("air upkeep {label} {value} is not between 0 and 1"));
            }
        }
        None
    }
}

#[cfg(test)]
impl AirCombatFigures {
    /// The figures of `data/reference/air_combat.toml`, written out.
    pub(crate) fn from_air_combat_toml() -> AirCombatFigures {
        let tenth = |n: i64| Fixed::from_ratio(n, 10);
        AirCombatFigures {
            strike_contact_max_age_pulses: 1,
            bomb_hit_chance: tenth(2),
            torpedo_hit_chance: Fixed::from_ratio(15, 100),
            fighter_hit_chance: tenth(3),
            anti_aircraft_hit_chance: Fixed::from_ratio(5, 100),
            pilot_armour_factor: Fixed::from_ratio(75, 100),
            self_sealing_tanks_factor: Fixed::from_ratio(75, 100),
            bomb_system_hit: [
                (ShipSystem::FlightDeck, tenth(5)),
                (ShipSystem::Propulsion, tenth(2)),
                (ShipSystem::FireControl, tenth(1)),
                (ShipSystem::MainBattery, tenth(2)),
            ]
            .into_iter()
            .collect(),
            base_supply_destroyed_per_hit_t: Fixed::from_int(50),
            base_hit_aircraft: tenth(3),
            base_hit_supply: tenth(3),
            // Decision 0092's first value.
            base_hit_men: tenth(2),
            base_hit_airfield: Fixed::ZERO,
            fighter_false_claim_chance: tenth(1),
            anti_aircraft_false_claim_chance: Fixed::from_ratio(5, 100),
            bomb_false_claim_chance: tenth(1),
            extended_reach_share: Fixed::from_ratio(25, 100),
            ditch_chance_per_radius_beyond: Fixed::from_int(2),
            crew_recovered_short_of_home: tenth(7),
            crew_recovered_over_own_base: tenth(6),
            crew_recovered_beside_own_force: Fixed::from_ratio(95, 100),
            crew_recovered_open_sea: tenth(5),
            // Step 1's rule (decision 0079); the unit tests are worked on it.
            anti_aircraft_before_attack_share: Fixed::ONE,
            // Step 1's rule (decision 0080); the unit tests are worked on it.
            patrol_drawn_off_share: Fixed::ONE,
            // Step 1's rule (decision 0087); the unit tests are worked on it.
            patrol_before_attack_share: Fixed::ONE,
        }
    }
}

#[cfg(test)]
impl SurfaceCombatFigures {
    /// The figures of `data/reference/step1/surface_combat.toml`, written
    /// out: the strengths of experience are 0, so that the tests of the other
    /// rules are not touched by it.
    pub(crate) fn from_surface_combat_toml() -> SurfaceCombatFigures {
        let hundredth = |n: i64| Fixed::from_ratio(n, 100);
        SurfaceCombatFigures {
            round_minutes: 5,
            max_rounds: 6,
            sighting_chance_night: hundredth(50),
            sighting_chance_day: Fixed::ONE,
            radar_sighting_bonus: hundredth(20),
            not_alert_sighting_factor: hundredth(50),
            alert_contact_distance_hexes: 3,
            alarm_chance: hundredth(50),
            gun_hit_chance_night: hundredth(3),
            gun_hit_chance_day: hundredth(5),
            torpedo_hit_chance_night: hundredth(4),
            torpedo_hit_chance_day: hundredth(2),
            surprised_target_factor: hundredth(150),
            lit_target_factor: Fixed::from_int(2),
            gun_hit_flotation: hundredth(75),
            system_hit_chance_per_point: hundredth(6),
            degraded_factor: hundredth(50),
            fire_control_out_factor: hundredth(25),
            gun_system_hit: [
                (ShipSystem::Propulsion, hundredth(20)),
                (ShipSystem::FireControl, hundredth(30)),
                (ShipSystem::MainBattery, hundredth(35)),
                (ShipSystem::TorpedoMounts, hundredth(15)),
            ]
            .into_iter()
            .collect(),
            torpedo_system_hit: [
                (ShipSystem::Propulsion, hundredth(80)),
                (ShipSystem::FireControl, hundredth(10)),
                (ShipSystem::MainBattery, hundredth(10)),
            ]
            .into_iter()
            .collect(),
            straggler_escorts: [
                (ShipKind::Carrier, 2),
                (ShipKind::Battleship, 2),
                (ShipKind::Cruiser, 1),
                (ShipKind::Destroyer, 0),
                (ShipKind::Transport, 1),
                (ShipKind::Auxiliary, 0),
            ]
            .into_iter()
            .collect(),
            crippled_flotation: Fixed::from_int(50),
            claimed_sunk_fire: Fixed::from_int(30),
            experience_gun_strength: Fixed::ZERO,
            experience_torpedo_strength: Fixed::ZERO,
            experience_sighting_strength: Fixed::ZERO,
            unready_gun_share: Fixed::ONE,
            unready_torpedo_share: Fixed::ONE,
            forward_repair_max_flotation: Fixed::from_int(10),
            forward_repair_system_points: Fixed::from_int(10),
            yard_days_per_flotation_point: Fixed::from_int(2),
            yard_days_per_system_degraded: Fixed::from_int(5),
            yard_days_per_system_out: Fixed::from_int(30),
            bombardment: Some(BombardmentFigures {
                minutes: 30,
                hit_chance: hundredth(5),
                reference_gun_damage: Fixed::from_int(20),
                false_claim_chance: hundredth(1),
            }),
        }
    }
}

#[cfg(test)]
impl SearchFigures {
    /// The figures of `data/reference/search.toml`, written out.
    pub(crate) fn from_search_toml() -> SearchFigures {
        let tenth = |n: i64| Fixed::from_ratio(n, 10);
        let row = |entries: &[(ShipKind, i64)]| -> BTreeMap<ShipKind, Fixed> {
            entries.iter().map(|(k, n)| (*k, tenth(*n))).collect()
        };
        let mut kind_confusion = BTreeMap::new();
        kind_confusion.insert(
            ShipKind::Carrier,
            row(&[
                (ShipKind::Carrier, 7),
                (ShipKind::Battleship, 1),
                (ShipKind::Cruiser, 1),
                (ShipKind::Transport, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Battleship,
            row(&[
                (ShipKind::Battleship, 7),
                (ShipKind::Carrier, 1),
                (ShipKind::Cruiser, 2),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Cruiser,
            row(&[
                (ShipKind::Cruiser, 7),
                (ShipKind::Battleship, 1),
                (ShipKind::Destroyer, 1),
                (ShipKind::Carrier, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Destroyer,
            row(&[
                (ShipKind::Destroyer, 7),
                (ShipKind::Cruiser, 2),
                (ShipKind::Transport, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Transport,
            row(&[
                (ShipKind::Transport, 7),
                (ShipKind::Auxiliary, 2),
                (ShipKind::Carrier, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Auxiliary,
            row(&[(ShipKind::Auxiliary, 7), (ShipKind::Transport, 3)]),
        );
        SearchFigures {
            contact_drop_age_pulses: 5,
            merge_distance_hexes: 2,
            fresh_contact_max_age_pulses: 2,
            position_error_chance: tenth(3),
            count_over_chance: tenth(2),
            count_under_chance: tenth(2),
            kind_confusion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ship_kinds_are_in_importance_order_with_codes_one_to_six() {
        let mut sorted = ShipKind::ALL;
        sorted.sort();
        assert_eq!(sorted, ShipKind::ALL);
        let codes: Vec<u8> = ShipKind::ALL.iter().map(|kind| kind.code()).collect();
        assert_eq!(codes, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn the_figures_of_the_search_file_are_sound() {
        assert_eq!(SearchFigures::from_search_toml().problem(), None);
    }

    #[test]
    fn the_figures_of_the_surface_combat_file_are_sound() {
        assert_eq!(
            SurfaceCombatFigures::from_surface_combat_toml().problem(),
            None
        );
    }

    fn combat_problem_of(change: impl FnOnce(&mut SurfaceCombatFigures)) -> String {
        let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
        change(&mut figures);
        figures.problem().expect("the change makes a problem")
    }

    #[test]
    fn each_strength_of_experience_must_be_between_zero_and_a_half() {
        let over = Fixed::from_ratio(51, 100);
        let under = Fixed::from_ratio(-1, 100);
        for (label, set) in [
            (
                "experience gun strength",
                (|f: &mut SurfaceCombatFigures, v| f.experience_gun_strength = v)
                    as fn(&mut SurfaceCombatFigures, Fixed),
            ),
            ("experience torpedo strength", |f, v| {
                f.experience_torpedo_strength = v
            }),
            ("experience sighting strength", |f, v| {
                f.experience_sighting_strength = v
            }),
        ] {
            assert!(combat_problem_of(|f| set(f, over)).contains(label));
            assert!(combat_problem_of(|f| set(f, under)).contains(label));
            let mut edge = SurfaceCombatFigures::from_surface_combat_toml();
            set(&mut edge, Fixed::from_ratio(1, 2));
            assert_eq!(edge.problem(), None);
            set(&mut edge, Fixed::ZERO);
            assert_eq!(edge.problem(), None);
        }
    }

    #[test]
    fn the_repair_figures_are_checked() {
        let f = |n: i64| Fixed::from_int(n);
        type Setter = fn(&mut SurfaceCombatFigures, Fixed);
        let cases: [(&str, Setter, Fixed, bool); 12] = [
            (
                "max flotation",
                |x, v| x.forward_repair_max_flotation = v,
                f(0),
                true,
            ),
            (
                "max flotation",
                |x, v| x.forward_repair_max_flotation = v,
                f(100),
                true,
            ),
            (
                "max flotation",
                |x, v| x.forward_repair_max_flotation = v,
                f(-1),
                false,
            ),
            (
                "max flotation",
                |x, v| x.forward_repair_max_flotation = v,
                f(101),
                false,
            ),
            (
                "system points",
                |x, v| x.forward_repair_system_points = v,
                f(1),
                true,
            ),
            (
                "system points",
                |x, v| x.forward_repair_system_points = v,
                f(0),
                false,
            ),
            (
                "per flotation point",
                |x, v| x.yard_days_per_flotation_point = v,
                f(0),
                true,
            ),
            (
                "per flotation point",
                |x, v| x.yard_days_per_flotation_point = v,
                f(-1),
                false,
            ),
            (
                "per system degraded",
                |x, v| x.yard_days_per_system_degraded = v,
                f(-1),
                false,
            ),
            (
                "per system degraded",
                |x, v| x.yard_days_per_system_degraded = v,
                f(30),
                true,
            ),
            (
                "per system degraded",
                |x, v| x.yard_days_per_system_degraded = v,
                f(31),
                false,
            ),
            (
                "per system out",
                |x, v| x.yard_days_per_system_out = v,
                f(4),
                false,
            ),
        ];
        for (label, set, value, ok) in cases {
            let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
            set(&mut figures, value);
            let problem = figures.problem();
            assert_eq!(problem.is_none(), ok, "{label} {value}: {problem:?}");
            if let Some(problem) = problem {
                assert!(
                    problem.contains(label) || problem.contains("yard days per system"),
                    "{problem}"
                );
            }
        }
    }

    #[test]
    fn the_unready_gun_share_must_be_between_a_tenth_and_one() {
        for (value, ok) in [
            (Fixed::from_ratio(1, 10), true),
            (Fixed::from_ratio(1, 2), true),
            (Fixed::ONE, true),
            (Fixed::from_ratio(99, 1000), false),
            (Fixed::ZERO, false),
            (Fixed::from_ratio(101, 100), false),
            (Fixed::from_ratio(-1, 2), false),
        ] {
            let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
            figures.unready_gun_share = value;
            let problem = figures.problem();
            assert_eq!(problem.is_none(), ok, "{value}");
            if let Some(problem) = problem {
                assert!(problem.contains("unready gun share"), "{problem}");
            }
        }
    }

    #[test]
    fn the_unready_torpedo_share_must_be_between_a_tenth_and_one() {
        for (value, ok) in [
            (Fixed::from_ratio(1, 10), true),
            (Fixed::from_ratio(1, 2), true),
            (Fixed::ONE, true),
            (Fixed::from_ratio(99, 1000), false),
            (Fixed::ZERO, false),
            (Fixed::from_ratio(101, 100), false),
            (Fixed::from_ratio(-1, 2), false),
        ] {
            let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
            figures.unready_torpedo_share = value;
            let problem = figures.problem();
            assert_eq!(problem.is_none(), ok, "{value}");
            if let Some(problem) = problem {
                assert!(problem.contains("unready torpedo share"), "{problem}");
            }
        }
    }

    #[test]
    fn each_surface_combat_chance_must_be_between_zero_and_one() {
        let over = Fixed::from_ratio(11, 10);
        let under = Fixed::from_ratio(-1, 10);
        assert!(combat_problem_of(|f| f.sighting_chance_night = over).contains("night sighting"));
        assert!(combat_problem_of(|f| f.sighting_chance_day = under).contains("day sighting"));
        assert!(combat_problem_of(|f| f.radar_sighting_bonus = over).contains("radar"));
        assert!(combat_problem_of(|f| f.alarm_chance = over).contains("alarm chance"));
        assert!(combat_problem_of(|f| f.alarm_chance = under).contains("alarm chance"));
        assert!(combat_problem_of(|f| f.gun_hit_chance_night = under).contains("night gun"));
        assert!(combat_problem_of(|f| f.gun_hit_chance_day = over).contains("day gun"));
        assert!(combat_problem_of(|f| f.torpedo_hit_chance_night = over).contains("night torpedo"));
        assert!(combat_problem_of(|f| f.torpedo_hit_chance_day = under).contains("day torpedo"));
        // 0 and 1 themselves are allowed.
        let mut edge = SurfaceCombatFigures::from_surface_combat_toml();
        edge.gun_hit_chance_night = Fixed::ZERO;
        edge.sighting_chance_night = Fixed::ONE;
        edge.alarm_chance = Fixed::ONE;
        assert_eq!(edge.problem(), None);
        edge.alarm_chance = Fixed::ZERO;
        assert_eq!(edge.problem(), None);
    }

    #[test]
    fn a_surface_combat_factor_may_not_be_negative() {
        let negative = Fixed::from_ratio(-1, 10);
        assert!(
            combat_problem_of(|f| f.not_alert_sighting_factor = negative)
                .contains("not alert sighting factor")
        );
        assert!(combat_problem_of(|f| f.surprised_target_factor = negative)
            .contains("surprised target factor"));
        assert!(combat_problem_of(|f| f.lit_target_factor = negative).contains("lit target"));
        // A factor above 1 is allowed, and so is zero.
        let mut edge = SurfaceCombatFigures::from_surface_combat_toml();
        edge.lit_target_factor = Fixed::ZERO;
        edge.surprised_target_factor = Fixed::from_int(5);
        assert_eq!(edge.problem(), None);
    }

    #[test]
    fn the_gun_hit_flotation_share_must_be_between_zero_and_one() {
        let message = combat_problem_of(|f| f.gun_hit_flotation = Fixed::from_ratio(-1, 10));
        assert!(message.contains("flotation share"), "{message}");
        let message = combat_problem_of(|f| f.gun_hit_flotation = Fixed::from_ratio(11, 10));
        assert!(message.contains("flotation share"), "{message}");
    }

    #[test]
    fn the_system_hit_factors_must_be_between_zero_and_one() {
        let over = Fixed::from_ratio(11, 10);
        let under = Fixed::from_ratio(-1, 10);
        assert!(combat_problem_of(|f| f.degraded_factor = over).contains("degraded factor"));
        assert!(combat_problem_of(|f| f.fire_control_out_factor = under)
            .contains("fire control out factor"));
    }

    #[test]
    fn the_system_hit_chance_per_point_must_not_be_negative_but_may_exceed_one() {
        let message = combat_problem_of(|f| {
            f.system_hit_chance_per_point = Fixed::from_ratio(-1, 100);
        });
        assert!(message.contains("system hit chance per point"), "{message}");
        let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
        figures.system_hit_chance_per_point = Fixed::from_int(5);
        assert_eq!(figures.problem(), None);
    }

    #[test]
    fn each_system_hit_table_must_add_to_one_with_shares_in_range() {
        let message = combat_problem_of(|f| {
            f.gun_system_hit
                .insert(ShipSystem::Propulsion, Fixed::from_ratio(3, 10));
        });
        assert!(
            message.contains("gun system hit shares add to"),
            "{message}"
        );
        let message = combat_problem_of(|f| {
            f.torpedo_system_hit.remove(&ShipSystem::Propulsion);
        });
        assert!(
            message.contains("torpedo system hit shares add to"),
            "{message}"
        );
        // One share outside 0 to 1, the table still adding to 1.
        let message = combat_problem_of(|f| {
            f.torpedo_system_hit
                .insert(ShipSystem::Propulsion, Fixed::from_ratio(11, 10));
            f.torpedo_system_hit
                .insert(ShipSystem::FireControl, Fixed::from_ratio(-1, 10));
        });
        assert!(message.contains("not between 0 and 1"), "{message}");
        // A table may name a single system.
        let mut one = SurfaceCombatFigures::from_surface_combat_toml();
        one.gun_system_hit = [(ShipSystem::MainBattery, Fixed::ONE)]
            .into_iter()
            .collect();
        assert_eq!(one.problem(), None);
    }

    #[test]
    fn every_kind_needs_a_straggler_escort_entry() {
        let message = combat_problem_of(|f| {
            f.straggler_escorts.remove(&ShipKind::Transport);
        });
        assert!(message.contains("transport"), "{message}");
    }

    #[test]
    fn a_systems_factor_follows_its_state() {
        let figures = SurfaceCombatFigures::from_surface_combat_toml();
        for system in ShipSystem::ALL {
            assert_eq!(
                figures.system_factor(system, SystemState::Working),
                Fixed::ONE
            );
            assert_eq!(
                figures.system_factor(system, SystemState::Degraded),
                Fixed::from_ratio(1, 2)
            );
        }
        assert_eq!(
            figures.system_factor(ShipSystem::FireControl, SystemState::Out),
            Fixed::from_ratio(1, 4)
        );
        for system in [
            ShipSystem::Propulsion,
            ShipSystem::MainBattery,
            ShipSystem::TorpedoMounts,
        ] {
            assert_eq!(figures.system_factor(system, SystemState::Out), Fixed::ZERO);
        }
    }

    #[test]
    fn the_air_combat_figures_of_the_file_are_valid_and_a_bad_table_is_not() {
        let good = AirCombatFigures::from_air_combat_toml();
        assert_eq!(good.problem(), None);
        let mut bad = good.clone();
        bad.bomb_system_hit
            .insert(ShipSystem::TorpedoMounts, Fixed::from_ratio(1, 10));
        assert!(bad.problem().expect("problem").contains("add to"));
        let mut bad = good.clone();
        bad.bomb_hit_chance = Fixed::from_ratio(11, 10);
        assert!(bad.problem().expect("problem").contains("bomb hit chance"));
        let mut bad = good;
        bad.torpedo_hit_chance = Fixed::from_int(2);
        assert!(bad
            .problem()
            .expect("problem")
            .contains("torpedo hit chance"));
    }

    #[test]
    fn a_class_has_the_systems_its_weapons_give_it() {
        let mut armed = test_class();
        armed.aircraft_capacity = 10;
        for system in ShipSystem::ALL {
            assert!(armed.has_system(system));
        }
        // The flight deck is the system of a class that carries aircraft.
        armed.aircraft_capacity = 0;
        assert!(!armed.has_system(ShipSystem::FlightDeck));
        let mut guns_only = test_class();
        guns_only.torpedo_tubes = 0;
        guns_only.torpedo_tubes_one_side = 0;
        assert!(guns_only.has_system(ShipSystem::MainBattery));
        assert!(guns_only.has_system(ShipSystem::FireControl));
        assert!(!guns_only.has_system(ShipSystem::TorpedoMounts));
        let mut tubes_only = test_class();
        tubes_only.main_guns = 0;
        assert!(!tubes_only.has_system(ShipSystem::MainBattery));
        assert!(!tubes_only.has_system(ShipSystem::FireControl));
        assert!(tubes_only.has_system(ShipSystem::TorpedoMounts));
        tubes_only.torpedo_tubes = 0;
        tubes_only.torpedo_tubes_one_side = 0;
        assert!(tubes_only.has_system(ShipSystem::Propulsion));
        assert!(!tubes_only.has_system(ShipSystem::TorpedoMounts));
    }

    #[test]
    fn a_fight_needs_rounds_and_minutes() {
        assert!(combat_problem_of(|f| f.round_minutes = 0).contains("round minutes"));
        assert!(combat_problem_of(|f| f.max_rounds = 0).contains("max rounds"));
    }

    #[test]
    fn the_damage_levels_must_be_between_zero_and_one_hundred() {
        let over = Fixed::from_int(101);
        let under = Fixed::from_int(-1);
        assert!(combat_problem_of(|f| f.crippled_flotation = over).contains("crippled"));
        assert!(combat_problem_of(|f| f.claimed_sunk_fire = under).contains("claimed sunk"));
        let mut edge = SurfaceCombatFigures::from_surface_combat_toml();
        edge.crippled_flotation = Fixed::from_int(100);
        edge.claimed_sunk_fire = Fixed::ZERO;
        assert_eq!(edge.problem(), None);
    }

    fn test_class() -> ShipClass {
        ShipClass {
            id: ShipClassId::new(1),
            name: "Test".to_string(),
            kind: ShipKind::Cruiser,
            cruise_speed_kn: Fixed::from_int(16),
            max_speed_kn: Fixed::from_int(32),
            fuel_capacity_t: Fixed::from_int(2000),
            endurance_nm: Fixed::from_int(8000),
            cargo_capacity_t: Fixed::ZERO,
            main_guns: 8,
            gun_rounds_per_min: Fixed::from_int(3),
            gun_rounds_per_gun: 100,
            gun_damage: Fixed::from_int(20),
            torpedo_tubes: 8,
            torpedo_tubes_one_side: 8,
            torpedo_damage: Fixed::from_int(200),
            toughness: Fixed::from_int(400),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
            troop_capacity_men: 0,
        }
    }

    #[test]
    fn a_ship_class_needs_toughness_but_may_have_no_weapons() {
        assert_eq!(test_class().problem(), None);
        let mut bad = test_class();
        bad.toughness = Fixed::ZERO;
        assert!(bad.problem().expect("problem").contains("toughness"));
        let mut unarmed = test_class();
        unarmed.main_guns = 0;
        unarmed.gun_rounds_per_min = Fixed::ZERO;
        unarmed.gun_rounds_per_gun = 0;
        unarmed.gun_damage = Fixed::ZERO;
        unarmed.torpedo_tubes = 0;
        unarmed.torpedo_tubes_one_side = 0;
        unarmed.torpedo_damage = Fixed::ZERO;
        assert_eq!(unarmed.problem(), None);
    }

    #[test]
    fn the_tubes_on_one_side_are_some_of_the_tubes_and_at_least_one() {
        let mut wing = test_class();
        wing.torpedo_tubes_one_side = 4;
        assert_eq!(wing.problem(), None);
        let mut too_many = test_class();
        too_many.torpedo_tubes_one_side = 9;
        assert!(too_many
            .problem()
            .expect("problem")
            .contains("on one side than tubes"));
        let mut none_bear = test_class();
        none_bear.torpedo_tubes_one_side = 0;
        assert!(none_bear
            .problem()
            .expect("problem")
            .contains("none that bear on one side"));
    }

    #[test]
    fn an_aircraft_type_needs_a_positive_range_and_sweep_width() {
        let good = AircraftType {
            id: AircraftTypeId::new(1),
            name: "Test".to_string(),
            role: AircraftRole::Patrol,
            search_range_nm: Fixed::from_int(600),
            sweep_width_nm: Fixed::from_int(40),
            strike_range_nm: Fixed::ZERO,
            attack_weapon: AttackWeapon::None,
            attack_damage: Fixed::ZERO,
            pilot_armour: false,
            self_sealing_tanks: false,
        };
        assert_eq!(good.problem(), None);
        let mut bad = good.clone();
        bad.search_range_nm = Fixed::ZERO;
        assert!(bad.problem().expect("problem").contains("search range"));
        let mut bad = good.clone();
        bad.sweep_width_nm = Fixed::from_int(-1);
        assert!(bad.problem().expect("problem").contains("sweep width"));
    }

    fn problem_of(change: impl FnOnce(&mut SearchFigures)) -> String {
        let mut figures = SearchFigures::from_search_toml();
        change(&mut figures);
        figures.problem().expect("the change makes a problem")
    }

    #[test]
    fn each_chance_must_be_between_zero_and_one() {
        let over = Fixed::from_ratio(11, 10);
        let under = Fixed::from_ratio(-1, 10);
        assert!(problem_of(|f| f.position_error_chance = over).contains("position error"));
        assert!(problem_of(|f| f.count_over_chance = under).contains("count over"));
        assert!(problem_of(|f| f.count_under_chance = over).contains("count under"));
        // 0 and 1 themselves are allowed.
        let mut edge = SearchFigures::from_search_toml();
        edge.position_error_chance = Fixed::ZERO;
        edge.count_over_chance = Fixed::ONE;
        edge.count_under_chance = Fixed::ZERO;
        assert_eq!(edge.problem(), None);
    }

    #[test]
    fn count_chances_may_not_add_to_more_than_one() {
        let message = problem_of(|f| {
            f.count_over_chance = Fixed::from_ratio(6, 10);
            f.count_under_chance = Fixed::from_ratio(5, 10);
        });
        assert!(message.contains("more than 1"));
        // Exactly 1 is allowed.
        let mut figures = SearchFigures::from_search_toml();
        figures.count_over_chance = Fixed::from_ratio(6, 10);
        figures.count_under_chance = Fixed::from_ratio(4, 10);
        assert_eq!(figures.problem(), None);
    }

    #[test]
    fn every_kind_needs_a_confusion_row_that_adds_to_one() {
        assert!(problem_of(|f| {
            f.kind_confusion.remove(&ShipKind::Destroyer);
        })
        .contains("no row"));
        assert!(problem_of(|f| {
            f.kind_confusion
                .get_mut(&ShipKind::Carrier)
                .expect("row")
                .insert(ShipKind::Carrier, Fixed::from_ratio(6, 10));
        })
        .contains("not 1"));
        assert!(problem_of(|f| {
            f.kind_confusion
                .get_mut(&ShipKind::Carrier)
                .expect("row")
                .insert(ShipKind::Carrier, Fixed::from_ratio(8, 10));
        })
        .contains("not 1"));
    }

    #[test]
    fn a_confusion_entry_outside_zero_to_one_is_caught() {
        // Carrier row 0.7 + 0.1 + 0.1 + 0.1 = 1. Make one entry -0.1 and
        // another 0.9 higher so the row still adds to 1: only the range check
        // can catch it.
        let message = problem_of(|f| {
            let row = f.kind_confusion.get_mut(&ShipKind::Carrier).expect("row");
            row.insert(ShipKind::Battleship, Fixed::from_ratio(-1, 10));
            row.insert(ShipKind::Carrier, Fixed::from_ratio(9, 10));
        });
        assert!(message.contains("not between 0 and 1"));
    }
}

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
use crate::state::{ShipSystem, SystemState};

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
}

impl AirCombatFigures {
    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        let in_unit = |value: Fixed| value >= Fixed::ZERO && value <= Fixed::ONE;
        let chances = [
            ("bomb hit chance", self.bomb_hit_chance),
            ("torpedo hit chance", self.torpedo_hit_chance),
            ("fighter hit chance", self.fighter_hit_chance),
            ("anti-aircraft hit chance", self.anti_aircraft_hit_chance),
            ("pilot armour factor", self.pilot_armour_factor),
            ("self-sealing tanks factor", self.self_sealing_tanks_factor),
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
        ] {
            if !in_unit(value) {
                return Some(format!(
                    "air combat base hit chance on {label} {value} is not between 0 and 1"
                ));
            }
        }
        let landed = self.base_hit_aircraft + self.base_hit_supply;
        if landed > Fixed::ONE {
            return Some(format!(
                "air combat base hit chances on aircraft and supply add to {landed}, more than 1"
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
            ("fatigue per sortie", self.air_fatigue_per_sortie),
            ("fatigue per 100 nm", self.air_fatigue_per_100_nm),
            ("rest per pulse", self.air_rest_per_pulse),
        ] {
            if value < Fixed::ZERO || value > Fixed::from_int(100) {
                return Some(format!("air {label} {value} is not between 0 and 100"));
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
        }
    }
}

#[cfg(test)]
impl SurfaceCombatFigures {
    /// The figures of `data/reference/surface_combat.toml`, written out.
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
        assert!(guns_only.has_system(ShipSystem::MainBattery));
        assert!(guns_only.has_system(ShipSystem::FireControl));
        assert!(!guns_only.has_system(ShipSystem::TorpedoMounts));
        let mut tubes_only = test_class();
        tubes_only.main_guns = 0;
        assert!(!tubes_only.has_system(ShipSystem::MainBattery));
        assert!(!tubes_only.has_system(ShipSystem::FireControl));
        assert!(tubes_only.has_system(ShipSystem::TorpedoMounts));
        tubes_only.torpedo_tubes = 0;
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
            torpedo_damage: Fixed::from_int(200),
            toughness: Fixed::from_int(400),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
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
        unarmed.torpedo_damage = Fixed::ZERO;
        assert_eq!(unarmed.problem(), None);
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

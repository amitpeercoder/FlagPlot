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

use crate::fixed::Fixed;
use crate::orders::{AirMission, Mission, SpeedSetting};
use crate::state::{AirGroupLocation, ShipLocation, ShipSystem, SystemState};
use crate::world::World;

/// Marks a flight deck that is degraded or out in the hash of a ship.
const FLIGHT_DECK_TAG: u8 = 0xF1;
/// Tag of an air group's patrol share (decision 0027). See `hash_world`.
const PATROL_SHARE_TAG: u8 = 0xF2;
/// Tag of a base's anti-aircraft shots (decision 0034). See `hash_world`.
const BASE_GUNS_TAG: u8 = 0xF3;
/// Tag of a base's repairs a day (decision 0035). See `hash_world`.
const BASE_REPAIRS_TAG: u8 = 0xF4;
/// Tag of an air group's damaged aircraft (decision 0035). See `hash_world`.
const DAMAGED_TAG: u8 = 0xF5;
/// Tag of an air group's fatigue, rest threshold and stand-down (decision
/// 0036). See `hash_world`.
const FATIGUE_TAG: u8 = 0xF6;
/// Tag of an air group's pilots and strength, and of the replacement streams
/// (decision 0038). See `hash_world`.
const PILOTS_TAG: u8 = 0xF7;
/// Tag of a task force's fatigue (decision 0037).
const FORCE_FATIGUE_TAG: u8 = 0xF8;
/// Tag of the scenario's arrivals (decision 0041).
const ARRIVALS_TAG: u8 = 0xF9;

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

/// Hashes everything that defines the state of a game. When a field is added
/// to the state, it must be added here too, or two different states could
/// share a hash.
pub(crate) fn hash_world(world: &World) -> u64 {
    // A strike launched in a pulse lands in the same pulse.
    debug_assert!(
        world.strikes_in_flight.is_empty(),
        "strikes were launched and have not landed"
    );
    let mut h = StateHasher::new();
    h.bytes(b"flagplot-state-v1");
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
        for system in ShipSystem::ALL {
            // The flight deck (code 5, decision 0026) is hashed below, so
            // that a ship whose flight deck works hashes as it did before
            // the system existed.
            if system != ShipSystem::FlightDeck {
                h.u8(ship.systems.get(system).code());
            }
        }
        // A flight deck that is not working: a tag byte, then its state.
        // The tag 0xF1 cannot be taken for what follows, which is the
        // location tag 1, 2 or 3, so every state has one reading. A working
        // flight deck writes nothing.
        if ship.systems.flight_deck != SystemState::Working {
            h.u8(FLIGHT_DECK_TAG);
            h.u8(ship.systems.flight_deck.code());
        }
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
        h.u8(task_force.status.code());
        h.optional_u32(task_force.route_order.map(|id| id.raw()));
        h.optional_u32(task_force.speed_order.map(|id| id.raw()));
        match task_force.cargo_order {
            Some(cargo) => {
                h.u8(1);
                h.u8(cargo.action.code());
                h.u32(cargo.order.raw());
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
    }

    h.count(truth.bases.len());
    for base in truth.bases.values() {
        h.u32(base.id.raw());
        h.u8(base.side.code());
        h.i32(base.hex.q);
        h.i32(base.hex.r);
        h.i64(base.supply_t.raw());
        // A base with no guns writes nothing, so it hashes as it did before
        // bases had guns (decision 0034).
        if base.anti_aircraft != 0 {
            h.u8(BASE_GUNS_TAG);
            h.u32(base.anti_aircraft);
        }
        // Likewise a base that repairs no aircraft (decision 0035).
        if base.aircraft_repairs_per_day != 0 {
            h.u8(BASE_REPAIRS_TAG);
            h.u32(base.aircraft_repairs_per_day);
        }
    }

    h.count(truth.air_groups.len());
    for group in truth.air_groups.values() {
        h.u32(group.id.raw());
        h.u8(group.side.code());
        // A group at a base hashes exactly as it did when it could only be
        // at a base: the base's id, four bytes. A group aboard a ship writes
        // the marker u32::MAX, then the ship's id. No base has that id.
        match group.location {
            AirGroupLocation::Base(base) => h.u32(base.raw()),
            AirGroupLocation::Ship(ship) => {
                h.u32(u32::MAX);
                h.u32(ship.raw());
            }
        }
        h.u32(group.aircraft_type.raw());
        h.u32(group.ready);
        // A group with no damaged aircraft writes nothing, so it hashes as
        // it did before damaged aircraft (decision 0035). The tag 0xF5
        // follows the ready count and the mission code that follows it is 1
        // to 5, so every state has one reading.
        if group.damaged != 0 {
            h.u8(DAMAGED_TAG);
            h.u32(group.damaged);
        }
        // A group that is not tired, has no rest threshold and is not stood
        // down writes nothing, so it hashes as it did before fatigue
        // (decision 0036). The tag 0xF6 follows the damaged count (or the
        // ready count) and the mission code that follows it is 1 to 5, so
        // every state has one reading.
        if group.fatigue != Fixed::ZERO
            || group.rest_above.is_some()
            || group.rest_order.is_some()
            || group.stood_down
        {
            h.u8(FATIGUE_TAG);
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
        }
        // A scenario with no replacements and a group whose pilots are not
        // tracked write nothing, so they hash as they did before pilots
        // (decision 0038). Strength is read by replacements alone, so it is
        // hashed only where there are some. The tag 0xF7 is followed by
        // the mission code, 1 to 5.
        if group.pilots.is_some() || !truth.replacements.is_empty() || !truth.arrivals.is_empty() {
            h.u8(PILOTS_TAG);
            h.optional_u32(group.pilots);
            h.u32(group.strength);
        }
        h.u8(group.mission.code());
        // The base strike mission (code 5) is followed by its target base.
        if let AirMission::BaseStrike(base) = group.mission {
            h.u32(base.raw());
        }
        // A patrol share that is not the default 1, or that an order set:
        // a tag byte, the share, then the order. The tag 0xF2 follows the
        // mission code (1 to 4) and so can only be taken for the search
        // tag, which is 0 or 1: every state has one reading. A group on the
        // default share with no order writes nothing, so it hashes as it did
        // before the share existed.
        if group.patrol_share != Fixed::ONE || group.share_order.is_some() {
            h.u8(PATROL_SHARE_TAG);
            h.i64(group.patrol_share.raw());
            h.optional_u32(group.share_order.map(|id| id.raw()));
        }
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
    }

    // The replacement streams: nothing at all without any (decision 0038).
    if !truth.replacements.is_empty() {
        h.u8(PILOTS_TAG);
        h.count(truth.replacements.len());
        for stream in &truth.replacements {
            h.u8(stream.side.code());
            h.u32(stream.base.raw());
            h.u32(stream.aircraft_type.raw());
            h.u32(stream.aircraft_per_day);
            h.u32(stream.pilots_per_day);
        }
    }

    // The arrivals: nothing at all without any (decision 0041).
    if !truth.arrivals.is_empty() {
        h.u8(ARRIVALS_TAG);
        h.count(truth.arrivals.len());
        for arrival in &truth.arrivals {
            h.u32(arrival.day);
            h.u32(arrival.air_group.raw());
            h.u32(arrival.aircraft);
            h.u32(arrival.pilots);
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
        }
    }

    h.count(world.orders.len());
    h.u32(world.order_ids.next_raw());
    h.u32(world.task_force_ids.next_raw());
    h.count(world.events.len());

    // Task force fatigue (decision 0037) comes last, so that nothing after
    // it can be mistaken for it: for each tired force, in ID order, the tag
    // 0xF7, its ID and its fatigue. A world with no tired force writes
    // nothing and hashes as it did before.
    for task_force in truth.task_forces.values() {
        if task_force.fatigue != Fixed::ZERO {
            h.u8(FORCE_FATIGUE_TAG);
            h.u32(task_force.id.raw());
            h.i64(task_force.fatigue.raw());
        }
    }

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

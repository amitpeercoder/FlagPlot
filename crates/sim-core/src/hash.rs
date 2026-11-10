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

use crate::orders::{Mission, SpeedSetting};
use crate::state::ShipLocation;
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
    }
}

/// Hashes everything that defines the state of a game. When a field is added
/// to the state, it must be added here too, or two different states could
/// share a hash.
pub(crate) fn hash_world(world: &World) -> u64 {
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
        match ship.location {
            ShipLocation::TaskForce(id) => {
                h.u8(1);
                h.u32(id.raw());
            }
            ShipLocation::Port(id) => {
                h.u8(2);
                h.u32(id.raw());
            }
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
        match task_force.noticed_sighted_at_hours {
            Some(hours) => {
                h.u8(1);
                h.i64(hours.raw());
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
    }

    h.count(truth.air_groups.len());
    for group in truth.air_groups.values() {
        h.u32(group.id.raw());
        h.u8(group.side.code());
        h.u32(group.base.raw());
        h.u32(group.aircraft_type.raw());
        h.u32(group.ready);
        h.u8(group.mission.code());
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
            h.i64(contact.confidence.raw());
            h.u32(contact.last_report_event.raw());
        }
    }

    h.count(world.orders.len());
    h.u32(world.order_ids.next_raw());
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

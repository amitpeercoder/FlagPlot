//! Hex coordinates.
//!
//! The map is a hex grid in axial coordinates at 40 nautical miles per hex.
//! Everything here is integer arithmetic.

use core::fmt;

/// Distance between the centres of two neighbouring hexes, in nautical miles.
pub const HEX_NM: i64 = 40;

/// A hex in axial coordinates.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    /// The six neighbour offsets, in the fixed order used to break ties.
    /// Changing this order changes the routes ships take.
    pub const DIRECTIONS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

    pub const fn new(q: i32, r: i32) -> Hex {
        Hex { q, r }
    }

    /// Number of hex steps between two hexes.
    pub fn distance(self, other: Hex) -> u32 {
        let dq = self.q - other.q;
        let dr = self.r - other.r;
        (dq.unsigned_abs() + dr.unsigned_abs() + (dq + dr).unsigned_abs()) / 2
    }

    /// The six neighbours, in [`Hex::DIRECTIONS`] order.
    pub fn neighbors(self) -> [Hex; 6] {
        Self::DIRECTIONS.map(|(dq, dr)| Hex::new(self.q + dq, self.r + dr))
    }

    /// The neighbour that is one step closer to `target`. When two neighbours
    /// are equally close, the one that comes first in [`Hex::DIRECTIONS`]
    /// wins. Returns `self` when already there.
    pub fn step_toward(self, target: Hex) -> Hex {
        if self == target {
            return self;
        }
        let mut best = self;
        let mut best_distance = u32::MAX;
        for neighbor in self.neighbors() {
            let distance = neighbor.distance(target);
            if distance < best_distance {
                best = neighbor;
                best_distance = distance;
            }
        }
        best
    }

    /// Every hex entered when stepping from `self` to `target`, ending with
    /// `target`. Empty when already there.
    pub fn steps_to(self, target: Hex) -> Vec<Hex> {
        let mut steps = Vec::new();
        let mut current = self;
        while current != target {
            current = current.step_toward(target);
            steps.push(current);
        }
        steps
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.q, self.r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_counts_hex_steps() {
        let origin = Hex::new(0, 0);
        assert_eq!(origin.distance(origin), 0);
        assert_eq!(origin.distance(Hex::new(8, 3)), 11);
        assert_eq!(origin.distance(Hex::new(3, -3)), 3);
        assert_eq!(Hex::new(8, 3).distance(origin), 11);
        for neighbor in origin.neighbors() {
            assert_eq!(origin.distance(neighbor), 1);
        }
    }

    #[test]
    fn stepping_takes_a_shortest_path() {
        let start = Hex::new(0, 0);
        let target = Hex::new(8, 3);
        let steps = start.steps_to(target);
        assert_eq!(steps.len(), 11);
        assert_eq!(steps.last(), Some(&target));
        let mut previous = start;
        for step in &steps {
            assert_eq!(previous.distance(*step), 1);
            previous = *step;
        }
    }

    #[test]
    fn ties_break_in_direction_order() {
        // From the origin, (1,0) and (0,1) are both 10 steps from (8,3).
        // (1,0) comes first in DIRECTIONS.
        assert_eq!(Hex::new(0, 0).step_toward(Hex::new(8, 3)), Hex::new(1, 0));
        // The ninth hex entered on that route.
        assert_eq!(Hex::new(0, 0).steps_to(Hex::new(8, 3))[8], Hex::new(8, 1));
    }

    #[test]
    fn stepping_toward_the_current_hex_stays_put() {
        let hex = Hex::new(4, -2);
        assert_eq!(hex.step_toward(hex), hex);
        assert!(hex.steps_to(hex).is_empty());
    }
}

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

    /// How far an aircraft flies between the centres of two hexes, in whole
    /// nautical miles (decision 0040): the straight line, 40 times the
    /// square root of dq^2 + dr^2 + dq*dr, to the nearest mile. Integer
    /// arithmetic only. Along an axis it is 40 times [`Hex::distance`].
    /// Ships do not use it: they move hex by hex.
    pub fn air_distance_nm(self, other: Hex) -> u32 {
        let dq = i64::from(self.q) - i64::from(other.q);
        let dr = i64::from(self.r) - i64::from(other.r);
        let n = dq * dq + dr * dr + dq * dr;
        let x = (HEX_NM * HEX_NM) as u64 * n as u64;
        // The nearest integer to sqrt(x) is (isqrt(4x) + 1) / 2.
        let nearest = (4 * x).isqrt().div_ceil(2);
        u32::try_from(nearest).expect("an air distance on a map fits in u32")
    }

    /// The six neighbours, in [`Hex::DIRECTIONS`] order.
    pub fn neighbors(self) -> [Hex; 6] {
        Self::DIRECTIONS.map(|(dq, dr)| Hex::new(self.q + dq, self.r + dr))
    }

    /// The neighbour that is one step closer to `target` in a straight line,
    /// ignoring land. It defines the open-water path: on open water the
    /// map's shortest-path step ([`crate::map::Map::next_step`], decision
    /// 0017) is the same hex. Movement uses the map's step, not this one.
    /// When two neighbours
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
    fn air_distance_along_an_axis_is_forty_per_hex() {
        let origin = Hex::new(0, 0);
        for steps in 0..=30i32 {
            for (dq, dr) in [(1, 0), (0, 1), (1, -1), (-1, 0), (0, -1), (-1, 1)] {
                let hex = Hex::new(steps * dq, steps * dr);
                assert_eq!(origin.air_distance_nm(hex), 40 * steps.unsigned_abs());
                assert_eq!(hex.air_distance_nm(origin), 40 * steps.unsigned_abs());
            }
        }
    }

    #[test]
    fn air_distance_known_values() {
        let o = Hex::new(0, 0);
        assert_eq!(Hex::new(1, -6).air_distance_nm(o), 223);
        assert_eq!(Hex::new(5, -6).air_distance_nm(o), 223);
        assert_eq!(Hex::new(1, -9).air_distance_nm(o), 342);
        assert_eq!(Hex::new(5, -9).air_distance_nm(o), 312);
        assert_eq!(Hex::new(-5, -13).air_distance_nm(Hex::new(2, -4)), 556);
    }

    #[test]
    fn air_distance_is_between_the_line_and_the_staircase() {
        for dq in -40..=40i32 {
            for dr in -40..=40i32 {
                let a = Hex::new(3, -2);
                let b = Hex::new(3 + dq, -2 + dr);
                let air = u64::from(a.air_distance_nm(b));
                assert_eq!(air, u64::from(b.air_distance_nm(a)));
                let staircase = 40 * u64::from(a.distance(b));
                assert!(air <= staircase);
                // 0.866 = 866 / 1000, and sqrt(3)/2 is 0.8660254, so allow
                // the rounding of half a mile.
                assert!(air * 1000 + 500 >= staircase * 866, "{dq},{dr}");
            }
        }
    }

    #[test]
    fn air_distance_is_the_nearest_whole_mile() {
        for dq in -40..=40i32 {
            for dr in -40..=40i32 {
                let n = i64::from(dq * dq + dr * dr + dq * dr);
                let x4 = 4 * 1600 * n;
                // The nearest whole k to sqrt(x) is the one with
                // (2k - 1)^2 < 4x < (2k + 1)^2 (4x is even, so never equal).
                let nearest = (0..=4000i64)
                    .find(|k| x4 < (2 * k + 1) * (2 * k + 1))
                    .expect("in range");
                let got = Hex::new(0, 0).air_distance_nm(Hex::new(dq, dr));
                assert_eq!(i64::from(got), nearest, "{dq},{dr}");
            }
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

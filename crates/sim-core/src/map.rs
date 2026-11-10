//! The hex map: what each hex is and what sites it holds, and the shortest
//! sea paths over it (decisions 10 to 15 of
//! `docs/decisions/0017-map-and-pathfinding.md`).

use std::collections::{BTreeMap, VecDeque};

use crate::fixed::Fixed;
use crate::hex::Hex;

/// What a hex is made of.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum HexKind {
    DeepSea,
    ShallowSea,
    Coast,
    Land,
}

impl HexKind {
    /// A stable number for the kind. Part of the state hash.
    pub const fn code(self) -> u8 {
        match self {
            HexKind::DeepSea => 1,
            HexKind::ShallowSea => 2,
            HexKind::Coast => 3,
            HexKind::Land => 4,
        }
    }
}

/// Ground cover on a hex that has land.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Terrain {
    Clear,
    Jungle,
    Mountain,
    Atoll,
}

/// Reference data for one hex.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HexData {
    pub kind: HexKind,
    pub terrain: Option<Terrain>,
    /// 0 means no port. Sizes run 1 to 10.
    pub port_size: u8,
    /// 0 means no airfield. Sizes run 1 to 10.
    pub airfield_size: u8,
    pub anchorage: bool,
    /// Latitude of the hex centre in degrees, south negative. Reference data
    /// from the generated map; not hashed, like the rest of the map.
    pub lat: Option<Fixed>,
    /// Longitude of the hex centre in degrees, west negative. Reference data
    /// from the generated map; not hashed.
    pub lon: Option<Fixed>,
    /// Share of the hex that is land, in whole percent, rounded down (0 to
    /// 100). Reference data from the generated map; not hashed.
    pub land_percent: u8,
    /// Sides of the hex that ships cannot cross, indexed like
    /// [`Hex::DIRECTIONS`] (decision 0017, decision 7). Reference data; not
    /// hashed. A side is closed if either hex marks it, so a one-sided file
    /// error cannot open it ([`Map::can_step`]).
    pub closed_sides: [bool; 6],
}

impl HexData {
    pub const fn open_sea() -> HexData {
        HexData {
            kind: HexKind::DeepSea,
            terrain: None,
            port_size: 0,
            airfield_size: 0,
            anchorage: false,
            lat: None,
            lon: None,
            land_percent: 0,
            closed_sides: [false; 6],
        }
    }

    /// Whether ships can enter the hex.
    pub const fn is_navigable(&self) -> bool {
        matches!(
            self.kind,
            HexKind::DeepSea | HexKind::ShallowSea | HexKind::Coast
        )
    }
}

/// The whole map. Read-only once a game starts.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Map {
    hexes: BTreeMap<Hex, HexData>,
}

impl Map {
    pub fn new() -> Map {
        Map::default()
    }

    /// Adds a hex, replacing any earlier data for it.
    pub fn insert(&mut self, hex: Hex, data: HexData) {
        self.hexes.insert(hex, data);
    }

    pub fn get(&self, hex: Hex) -> Option<&HexData> {
        self.hexes.get(&hex)
    }

    pub fn contains(&self, hex: Hex) -> bool {
        self.hexes.contains_key(&hex)
    }

    /// True when the hex is on the map and ships can enter it.
    pub fn is_navigable(&self, hex: Hex) -> bool {
        self.get(hex).is_some_and(HexData::is_navigable)
    }

    /// The one rule for a ship's step (decision 0017, decisions 7 and 10):
    /// true when `from` and its neighbour in `direction` (an index into
    /// [`Hex::DIRECTIONS`]) are both on the map and navigable, and neither
    /// hex marks the side between them closed. Checking both marks means a
    /// one-sided error in a file cannot open a side. The relation is
    /// symmetric: `can_step(a, d)` equals `can_step(neighbour, (d + 3) % 6)`,
    /// because the opposite of direction `d` is `(d + 3) % 6`. A `direction`
    /// above 5 is false.
    pub fn can_step(&self, from: Hex, direction: usize) -> bool {
        let Some((dq, dr)) = Hex::DIRECTIONS.get(direction) else {
            return false;
        };
        let neighbor = Hex::new(from.q + dq, from.r + dr);
        let (Some(here), Some(there)) = (self.get(from), self.get(neighbor)) else {
            return false;
        };
        here.is_navigable()
            && there.is_navigable()
            && !here.closed_sides[direction]
            && !there.closed_sides[(direction + 3) % 6]
    }

    /// The fewest steps from every reachable hex to `target`, over hexes that
    /// are on the map and that ships can enter (decision 0017, decision 10).
    ///
    /// A breadth-first search from the target, taking neighbours in
    /// [`Hex::DIRECTIONS`] order, never crossing a closed side
    /// ([`Map::can_step`]). A hex missing from the result cannot reach
    /// the target. If the target is off the map or ships cannot enter it, the
    /// result is empty: a land or off-map target is unreachable from
    /// everywhere. The target itself is at distance 0.
    ///
    /// The map never changes during a game, so callers compute this once per
    /// target and read it many times. Nothing is stored (decision 12).
    pub fn sea_distances_to(&self, target: Hex) -> BTreeMap<Hex, u32> {
        let mut distances = BTreeMap::new();
        if !self.is_navigable(target) {
            return distances;
        }
        distances.insert(target, 0);
        let mut queue = VecDeque::new();
        queue.push_back(target);
        while let Some(hex) = queue.pop_front() {
            let next = distances[&hex] + 1;
            for (direction, neighbor) in hex.neighbors().into_iter().enumerate() {
                // The ship would move from `neighbor` to `hex`. `can_step`
                // is symmetric, so testing the step out of `hex` is the same
                // test as the step into it.
                if !distances.contains_key(&neighbor) && self.can_step(hex, direction) {
                    distances.insert(neighbor, next);
                    queue.push_back(neighbor);
                }
            }
        }
        distances
    }

    /// The fewest steps by sea from `from` to `to`, or `None` when ships
    /// cannot get from one to the other.
    pub fn sea_distance(&self, from: Hex, to: Hex) -> Option<u32> {
        self.sea_distances_to(to).get(&from).copied()
    }

    /// The next hex on the path from `from` to `target`: the first neighbour
    /// of `from`, in [`Hex::DIRECTIONS`] order, whose sea distance to the
    /// target is one less than `from`'s (decision 0017, decision 10).
    ///
    /// `None` when `from` is the target, or the target cannot be reached
    /// from `from`. On open water this is [`Hex::step_toward`] (decision 11).
    pub fn next_step(&self, from: Hex, target: Hex) -> Option<Hex> {
        self.next_step_in(&self.sea_distances_to(target), from)
    }

    /// The step rule on a distance field already computed by
    /// [`Map::sea_distances_to`]. The one implementation of the rule:
    /// movement uses it with a field it keeps for the whole waypoint. The
    /// step must also be open ([`Map::can_step`]): a neighbour across a
    /// closed side is never chosen, even if its distance happens to be one
    /// less by another route.
    pub fn next_step_in(&self, distances: &BTreeMap<Hex, u32>, from: Hex) -> Option<Hex> {
        let here = *distances.get(&from)?;
        let wanted = here.checked_sub(1)?;
        from.neighbors()
            .into_iter()
            .enumerate()
            .find(|(direction, neighbor)| {
                distances.get(neighbor) == Some(&wanted) && self.can_step(from, *direction)
            })
            .map(|(_, neighbor)| neighbor)
    }

    pub fn len(&self) -> usize {
        self.hexes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hexes.is_empty()
    }

    /// All hexes in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&Hex, &HexData)> {
        self.hexes.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hx(q: i32, r: i32) -> Hex {
        Hex::new(q, r)
    }

    /// Open sea over the box `q_range` x `r_range`.
    fn sea(
        q_range: core::ops::RangeInclusive<i32>,
        r_range: core::ops::RangeInclusive<i32>,
    ) -> Map {
        let mut map = Map::new();
        for q in q_range {
            for r in r_range.clone() {
                map.insert(hx(q, r), HexData::open_sea());
            }
        }
        map
    }

    fn make_land(map: &mut Map, hexes: &[(i32, i32)]) {
        for (q, r) in hexes {
            map.insert(
                hx(*q, *r),
                HexData {
                    kind: HexKind::Land,
                    ..HexData::open_sea()
                },
            );
        }
    }

    /// Marks the side of `hex` in `direction` closed in `hex`'s own data
    /// only. The neighbour's data is left alone.
    fn close_one_side(map: &mut Map, hex: Hex, direction: usize) {
        let mut data = map.get(hex).expect("hex is on the map").clone();
        data.closed_sides[direction] = true;
        map.insert(hex, data);
    }

    /// Marks the side between `hex` and its neighbour closed in both.
    fn close_both_sides(map: &mut Map, hex: Hex, direction: usize) {
        close_one_side(map, hex, direction);
        let (dq, dr) = Hex::DIRECTIONS[direction];
        close_one_side(map, hx(hex.q + dq, hex.r + dr), (direction + 3) % 6);
    }

    /// The path `next_step` gives from `from` to `target`, from `from` (not
    /// included) to `target`.
    fn path(map: &Map, from: Hex, target: Hex) -> Vec<Hex> {
        let distances = map.sea_distances_to(target);
        let mut hexes = Vec::new();
        let mut here = from;
        while let Some(next) = map.next_step_in(&distances, here) {
            hexes.push(next);
            here = next;
        }
        assert_eq!(here, target, "the path ends at the target");
        hexes
    }

    #[test]
    fn distances_go_round_a_wall() {
        // q across, r down, north at the top. # is land, T the target.
        //
        //   r=0:   .  .  #  T  .      (0,0) (1,0) (2,0) (3,0) (4,0)
        //   r=1:    .  .  #  .  .     (0,1) (1,1) (2,1) (3,1) (4,1)
        //   r=2:     .  .  .  .  .    (0,2) (1,2) (2,2) (3,2) (4,2)
        //
        // From the target (3,0): (3,1) and (4,0) are 1; (3,2) and (4,1) are
        // 2 ((3,1) -> (3,2), (4,0) -> (4,1)); (2,2) is 2 ((2,2) -> (3,1));
        // (4,2) is 3; (1,2) is 3; (1,1) and (0,2) are 4; (1,0) and (0,1)
        // are 5; (0,0) is 6. The wall hexes are not in the field.
        // The straight line from (1,0) to (3,0) is 2.
        let mut map = sea(0..=4, 0..=2);
        make_land(&mut map, &[(2, 0), (2, 1)]);
        let distances = map.sea_distances_to(hx(3, 0));
        let expected = [
            ((3, 0), 0),
            ((3, 1), 1),
            ((4, 0), 1),
            ((3, 2), 2),
            ((4, 1), 2),
            ((2, 2), 2),
            ((4, 2), 3),
            ((1, 2), 3),
            ((1, 1), 4),
            ((0, 2), 4),
            ((1, 0), 5),
            ((0, 1), 5),
            ((0, 0), 6),
        ];
        assert_eq!(distances.len(), expected.len());
        for ((q, r), steps) in expected {
            assert_eq!(distances.get(&hx(q, r)), Some(&steps), "({q},{r})");
        }
        assert_eq!(map.sea_distance(hx(1, 0), hx(3, 0)), Some(5));
        assert_eq!(hx(1, 0).distance(hx(3, 0)), 2);
        // From (1,0) the directions in order lead to (2,0) land, (2,-1) off
        // the map, (1,-1) off the map, (0,0) at 6, (0,1) at 5, and (1,1) at
        // 4: that is one less than 5, so (1,1).
        assert_eq!(map.next_step(hx(1, 0), hx(3, 0)), Some(hx(1, 1)));
        assert_eq!(map.next_step(hx(3, 0), hx(3, 0)), None);
    }

    /// The map of Part 1 of `tests/scenario_04b_pathfinding.rs`.
    ///
    /// ```text
    ///   q:    -1 0 1 2 3 4 5 6 7 8 9
    ///   r=-3:  . . . . . . . . . . .
    ///   r=-2:  . . . . . . . . . . .
    ///   r=-1:  . . . . . # . . . . .
    ///   r= 0:  . S . . . # . . . T .
    ///   r= 1:  . . . . . # . . . . .
    ///   r= 2:  . . . . . . . . . . .
    ///   r= 3:  . . . . . . . . . . .
    /// ```
    fn island_map() -> Map {
        let mut map = sea(-1..=9, -3..=3);
        make_land(&mut map, &[(4, -1), (4, 0), (4, 1)]);
        map
    }

    #[test]
    fn the_tie_break_table_of_the_architects_hand_calculation() {
        // Target (8,0). Every row is checked against the table in Part 1 of
        // the scenario test: steps left, then the first direction in the
        // order (+1,0) (+1,-1) (0,-1) (-1,0) (-1,+1) (0,+1) that leads to a
        // hex one step nearer.
        let map = island_map();
        let target = hx(8, 0);
        let rows = [
            // (from, steps left, next)
            ((0, 0), 10, (1, 0)),
            ((1, 0), 9, (2, 0)),
            ((2, 0), 8, (3, 0)),
            // (4,0) and (4,-1) are land; (3,-1) is 7 from the target, (2,0)
            // is 8, (2,1) is 7, and (3,1) is 6: the first with 6.
            ((3, 0), 7, (3, 1)),
            // (4,1) and (4,0) are land; (3,0) is 7 and (2,1) is 7; (2,2) is
            // 6, no nearer than (3,1) itself; (0,+1) leads to (3,2), 5.
            ((3, 1), 6, (3, 2)),
            ((3, 2), 5, (4, 2)),
            ((4, 2), 4, (5, 2)),
            ((5, 2), 3, (6, 2)),
            // (7,2) is 2 from the target, not 1: (+1,0) fails and (+1,-1)
            // leads to (7,1), which is 1.
            ((6, 2), 2, (7, 1)),
            // (8,1) is 1 from the target; (+1,-1) leads to (8,0) itself.
            ((7, 1), 1, (8, 0)),
        ];
        for ((fq, fr), steps, (nq, nr)) in rows {
            assert_eq!(
                map.sea_distance(hx(fq, fr), target),
                Some(steps),
                "distance from ({fq},{fr})"
            );
            assert_eq!(
                map.next_step(hx(fq, fr), target),
                Some(hx(nq, nr)),
                "next step from ({fq},{fr})"
            );
        }
        // Ten steps, 400 nm; the straight line is 8 through land.
        assert_eq!(hx(0, 0).distance(target), 8);
        assert_eq!(
            path(&map, hx(0, 0), target),
            vec![
                hx(1, 0),
                hx(2, 0),
                hx(3, 0),
                hx(3, 1),
                hx(3, 2),
                hx(4, 2),
                hx(5, 2),
                hx(6, 2),
                hx(7, 1),
                hx(8, 0),
            ]
        );
    }

    #[test]
    fn a_target_ringed_by_land_is_unreachable() {
        // (2,2) is sea; its six neighbours (3,2) (3,1) (2,1) (1,2) (1,3)
        // (2,3) are land.
        //
        //   r=1:     .  .  #  #  .       (2,1) (3,1) land
        //   r=2:    .  #  T  #  .        (1,2) (3,2) land
        //   r=3:   .  #  #  .  .         (1,3) (2,3) land
        let mut map = sea(0..=4, 0..=4);
        make_land(&mut map, &[(3, 2), (3, 1), (2, 1), (1, 2), (1, 3), (2, 3)]);
        let target = hx(2, 2);
        // Only the target is in the field.
        assert_eq!(map.sea_distances_to(target).len(), 1);
        assert_eq!(map.sea_distance(hx(0, 0), target), None);
        assert_eq!(map.next_step(hx(0, 0), target), None);
        // And the sea inside the ring cannot get out.
        assert_eq!(map.sea_distance(target, hx(0, 0)), None);
        assert_eq!(map.next_step(target, hx(0, 0)), None);
    }

    #[test]
    fn a_land_or_off_map_target_is_unreachable_from_everywhere() {
        let mut map = sea(0..=4, 0..=2);
        make_land(&mut map, &[(2, 1)]);
        assert!(map.sea_distances_to(hx(2, 1)).is_empty());
        assert_eq!(map.sea_distance(hx(0, 0), hx(2, 1)), None);
        assert_eq!(map.next_step(hx(0, 0), hx(2, 1)), None);
        // Off the map.
        assert!(map.sea_distances_to(hx(9, 9)).is_empty());
        assert_eq!(map.sea_distance(hx(0, 0), hx(9, 9)), None);
        assert_eq!(map.next_step(hx(0, 0), hx(9, 9)), None);
        // A start on land or off the map reaches nothing.
        assert_eq!(map.sea_distance(hx(2, 1), hx(0, 0)), None);
        assert_eq!(map.sea_distance(hx(9, 9), hx(0, 0)), None);
    }

    #[test]
    fn on_open_water_the_step_is_the_straight_line_step() {
        // All sea, q 0 to 8 and r 0 to 6: 63 hexes, 63 x 62 ordered pairs.
        // Decision 0017, decision 11: the shortest-path step and the greedy
        // step are the same hex, and the distance is the hex distance.
        let map = sea(0..=8, 0..=6);
        let hexes: Vec<Hex> = map.iter().map(|(hex, _)| *hex).collect();
        assert_eq!(hexes.len(), 63);
        for target in &hexes {
            let distances = map.sea_distances_to(*target);
            for from in &hexes {
                assert_eq!(distances.get(from), Some(&from.distance(*target)));
                if from == target {
                    assert_eq!(map.next_step_in(&distances, *from), None);
                } else {
                    assert_eq!(
                        map.next_step(*from, *target),
                        Some(from.step_toward(*target)),
                        "{from} to {target}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_one_hex_strait_forces_the_path_through_it() {
        // Land across q = 2 except the strait at (2,1).
        //
        //   r=0:   S  .  #  .  T        (0,0) (1,0) (2,0) (3,0) (4,0)
        //   r=1:    .  .  =  .  .      (0,1) (1,1) (2,1) (3,1) (4,1)
        //   r=2:     .  .  #  .  .     (0,2) (1,2) (2,2) (3,2) (4,2)
        //
        // (= is the strait.) Steps change q by one at most, so every path
        // visits q = 2, and only (2,1) is open. (0,0) to (2,1) is 3 steps
        // (dq 2, dr 1: (2 + 1 + 3) / 2), and (2,1) to (4,0) is 2 (dq 2, dr
        // -1: (2 + 1 + 1) / 2): 5 in all, against 4 in a straight line.
        // Steps: (1,0) [first direction, and 2 from (2,1)], (1,1) [(2,0) is
        // land, (2,-1), (1,-1) are off the map, (0,0) and (0,1) are no
        // nearer], (2,1), then (3,1) [(+1,0) is 1 from the target], then
        // (4,0) [(+1,0) leads to (4,1), still 1 away; (+1,-1) reaches it].
        let mut map = sea(0..=4, 0..=2);
        make_land(&mut map, &[(2, 0), (2, 2)]);
        let target = hx(4, 0);
        assert_eq!(map.sea_distance(hx(0, 0), target), Some(5));
        assert_eq!(hx(0, 0).distance(target), 4);
        let route = path(&map, hx(0, 0), target);
        assert_eq!(
            route,
            vec![hx(1, 0), hx(1, 1), hx(2, 1), hx(3, 1), hx(4, 0)]
        );
        assert!(route.contains(&hx(2, 1)));
        // Close the strait and the far side is cut off.
        make_land(&mut map, &[(2, 1)]);
        assert_eq!(map.sea_distance(hx(0, 0), target), None);
    }

    #[test]
    fn opposite_directions_are_three_apart() {
        for (d, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
            assert_eq!(Hex::DIRECTIONS[(d + 3) % 6], (-dq, -dr), "direction {d}");
        }
    }

    #[test]
    fn a_closed_side_blocks_a_step_both_ways() {
        // Two hexes side by side; the side between them is direction 0 of
        // (0,0) and direction 3 of (1,0).
        //
        //   r=0:   A  B        A = (0,0), B = (1,0)
        //
        // Open: one step each way. Then three ways to close it: the mark on
        // A alone, on B alone, and on both. Each closes the side both ways.
        let (a, b) = (hx(0, 0), hx(1, 0));
        let open = sea(0..=1, 0..=0);
        assert!(open.can_step(a, 0) && open.can_step(b, 3));
        assert_eq!(open.sea_distance(a, b), Some(1));
        assert_eq!(open.next_step(a, b), Some(b));
        assert_eq!(open.next_step(b, a), Some(a));

        let mut only_a = sea(0..=1, 0..=0);
        close_one_side(&mut only_a, a, 0);
        let mut only_b = sea(0..=1, 0..=0);
        close_one_side(&mut only_b, b, 3);
        let mut both = sea(0..=1, 0..=0);
        close_both_sides(&mut both, a, 0);
        for (name, map) in [("only A", only_a), ("only B", only_b), ("both", both)] {
            assert!(!map.can_step(a, 0), "{name}: A to B");
            assert!(!map.can_step(b, 3), "{name}: B to A");
            assert_eq!(map.sea_distance(a, b), None, "{name}");
            assert_eq!(map.sea_distance(b, a), None, "{name}");
            assert_eq!(map.next_step(a, b), None, "{name}");
            assert_eq!(map.next_step(b, a), None, "{name}");
            // Each hex still reaches itself, and only itself.
            assert_eq!(map.sea_distances_to(a).len(), 1, "{name}");
            assert_eq!(map.sea_distances_to(b).len(), 1, "{name}");
        }
    }

    #[test]
    fn can_step_needs_both_hexes_on_the_map_and_navigable() {
        let mut map = sea(0..=1, 0..=0);
        make_land(&mut map, &[(1, 0)]);
        assert!(!map.can_step(hx(0, 0), 0), "into land");
        assert!(!map.can_step(hx(1, 0), 3), "out of land");
        assert!(!map.can_step(hx(0, 0), 3), "off the map");
        assert!(!map.can_step(hx(5, 5), 0), "from off the map");
        assert!(!map.can_step(hx(0, 0), 6), "no such direction");
    }

    #[test]
    fn can_step_is_symmetric_whatever_is_closed() {
        // A 4 x 3 box with some sides closed in one hex only, some in both,
        // and a land hex.
        let mut map = sea(0..=3, 0..=2);
        close_one_side(&mut map, hx(1, 1), 0);
        close_one_side(&mut map, hx(1, 1), 5);
        close_one_side(&mut map, hx(2, 0), 4);
        close_both_sides(&mut map, hx(0, 1), 1);
        close_one_side(&mut map, hx(3, 2), 3);
        make_land(&mut map, &[(2, 2)]);
        let hexes: Vec<Hex> = map.iter().map(|(hex, _)| *hex).collect();
        let mut closed = 0;
        for hex in hexes {
            for (direction, neighbor) in hex.neighbors().into_iter().enumerate() {
                let step = map.can_step(hex, direction);
                assert_eq!(
                    step,
                    map.can_step(neighbor, (direction + 3) % 6),
                    "{hex} direction {direction}"
                );
                closed += usize::from(!step);
            }
        }
        assert!(closed > 0);
    }

    #[test]
    fn a_ring_of_closed_sides_cuts_off_a_sea_hex() {
        // Every hex is sea, but the six sides of (2,2) are closed: three
        // marked on (2,2) itself and three marked only on the neighbours.
        //
        //   r=1:     .  .  .  .  .       (2,1) (3,1) are its upper neighbours
        //   r=2:    .  .  O  .  .        (1,2) (3,2) its east and west
        //   r=3:   .  .  .  .  .         (1,3) (2,3) its lower neighbours
        let mut map = sea(0..=4, 0..=4);
        let centre = hx(2, 2);
        for direction in 0..3 {
            close_one_side(&mut map, centre, direction);
        }
        for direction in 3..6 {
            let (dq, dr) = Hex::DIRECTIONS[direction];
            let neighbor = hx(centre.q + dq, centre.r + dr);
            close_one_side(&mut map, neighbor, (direction + 3) % 6);
        }
        // Every other hex is sea, so the rest of the map is one piece of 24.
        let outside = map.sea_distances_to(hx(0, 0));
        assert_eq!(outside.len(), 24);
        assert!(!outside.contains_key(&centre));
        assert_eq!(map.sea_distance(centre, hx(0, 0)), None);
        assert_eq!(map.next_step(centre, hx(0, 0)), None);
        // And nothing gets in or out: the field from the ringed hex is
        // itself alone, and each neighbour cannot reach it.
        assert_eq!(map.sea_distances_to(centre).len(), 1);
        for neighbor in centre.neighbors() {
            assert_eq!(map.sea_distance(neighbor, centre), None);
            assert_eq!(map.sea_distance(centre, neighbor), None);
        }
    }

    #[test]
    fn a_strait_is_the_one_open_side_in_a_closed_line() {
        // Two rows, every hex sea. The line between row 0 and row 1 is
        // closed: for each (q,0), direction 5 (to (q,1)) and direction 4
        // (to (q-1,1)). The one exception is direction 5 of (2,0), the
        // strait between (2,0) and (2,1). The marks are on row 0 only.
        //
        //   r=0:   S  .  .  .  .      (0,0) (1,0) (2,0) (3,0) (4,0)
        //   r=1:    T  .  .  .  .     (0,1) (1,1) (2,1) (3,1) (4,1)
        //          wall everywhere between the rows except (2,0)-(2,1)
        //
        // S = (0,0) and T = (0,1) are neighbours, one step apart in a
        // straight line. Every path from row 0 to row 1 crosses at (2,0) to
        // (2,1). (0,0) to (2,0) is 2 steps along the row, the strait is 1,
        // and (2,1) to (0,1) is 2 steps along row 1: 5 in all. The path,
        // taking the first direction that is one nearer at each hex:
        // (1,0), (2,0), (2,1), (1,1), (0,1).
        let mut map = sea(0..=4, 0..=1);
        for q in 0..=4 {
            if q != 2 {
                close_one_side(&mut map, hx(q, 0), 5);
            }
            close_one_side(&mut map, hx(q, 0), 4);
        }
        let (start, target) = (hx(0, 0), hx(0, 1));
        assert_eq!(start.distance(target), 1);
        assert_eq!(map.sea_distance(start, target), Some(5));
        assert_eq!(map.sea_distance(target, start), Some(5));
        assert_eq!(
            path(&map, start, target),
            vec![hx(1, 0), hx(2, 0), hx(2, 1), hx(1, 1), hx(0, 1)]
        );
        // The strait is the only open crossing.
        let crossings = (0..=4)
            .flat_map(|q| [(q, 5usize), (q, 4usize)])
            .filter(|(q, direction)| map.can_step(hx(*q, 0), *direction))
            .count();
        assert_eq!(crossings, 1);
        assert!(map.can_step(hx(2, 0), 5) && map.can_step(hx(2, 1), 2));
        // Without the wall the two are a step apart.
        assert_eq!(sea(0..=4, 0..=1).sea_distance(start, target), Some(1));
    }

    #[test]
    fn next_step_never_crosses_a_closed_side() {
        // A = (0,0), B = (1,0), C = (0,1), T = (1,1). B and C are both
        // neighbours of A and of T.
        //
        //   r=0:   A  B
        //   r=1:    C  T
        //
        // A's neighbours in direction order: (1,0) = B first, then (0,1) = C
        // (direction 5). Both are 1 from T, so the step from A goes to B,
        // the first in order.
        let (a, b, c, t) = (hx(0, 0), hx(1, 0), hx(0, 1), hx(1, 1));
        let open = sea(0..=1, 0..=1);
        let open_field = open.sea_distances_to(t);
        assert_eq!(open_field.get(&a), Some(&2));
        assert_eq!(open_field.get(&b), Some(&1));
        assert_eq!(open_field.get(&c), Some(&1));
        assert_eq!(open.next_step_in(&open_field, a), Some(b));

        // Close the side A-B. B is still 1 from T, and A is still 2 by way
        // of C, so B has distance one less than A and is across a closed
        // side: the step must be C.
        let mut map = sea(0..=1, 0..=1);
        close_one_side(&mut map, a, 0);
        let field = map.sea_distances_to(t);
        assert_eq!(field.get(&a), Some(&2));
        assert_eq!(field.get(&b), Some(&1));
        assert_eq!(map.next_step_in(&field, a), Some(c));
        assert_eq!(map.next_step(a, t), Some(c));

        // The same with the field of the open map, which says B is one
        // nearer: the closed side still stops the step.
        assert_eq!(map.next_step_in(&open_field, a), Some(c));
        // Close A-C as well: both nearer neighbours are across closed
        // sides, so there is no step even on the stale field.
        close_one_side(&mut map, a, 5);
        assert_eq!(map.next_step_in(&open_field, a), None);
        // And the fresh field says A cannot reach T at all.
        assert_eq!(map.sea_distance(a, t), None);
    }
}

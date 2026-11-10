//! Drawing the map as text.

use sim_core::hex::Hex;
use sim_core::map::{HexKind, Map};
use std::collections::BTreeMap;

/// Whether a coast hex has a closed side toward a neighbour that is on the
/// map and is not land. (A side toward land is closed anyway: a ship cannot
/// enter land. This is the closing that the coastline data makes.)
fn closed_toward_water(map: &Map, hex: Hex) -> bool {
    hex.neighbors()
        .into_iter()
        .enumerate()
        .any(|(d, neighbor)| {
            map.get(neighbor)
                .is_some_and(|data| data.kind != HexKind::Land)
                && !map.can_step(hex, d)
        })
}

/// Draws the map, one character per hex: `#` land, `%` coast with a closed
/// side toward a neighbour that is on the map and is not land, `+` any other
/// coast, `.` sea. A mark's character replaces the hex's. North (smaller r)
/// is at the top.
///
/// Each hex is its character and a space, two columns wide. Row r starts
/// with the label `r <r> q <q>:`, where q is the q of the first hex of the
/// row, and is shifted right by r - r_min columns, half a hex per row, so a
/// hex's neighbours in the rows above and below sit diagonally next to it.
/// Column of hex (q, r) after the label = 2 x (q - q_min) + (r - r_min),
/// where q_min is the least q on the map.
pub fn render(map: &Map, marks: &[((i32, i32), char)]) -> String {
    if map.is_empty() {
        return String::new();
    }
    let marks: BTreeMap<(i32, i32), char> = marks.iter().copied().collect();
    let hexes: Vec<Hex> = map.iter().map(|(hex, _)| *hex).collect();
    let q_min = hexes.iter().map(|h| h.q).min().unwrap_or(0);
    let r_min = hexes.iter().map(|h| h.r).min().unwrap_or(0);
    let r_max = hexes.iter().map(|h| h.r).max().unwrap_or(0);

    let mut out = String::new();
    for r in r_min..=r_max {
        let in_row: Vec<i32> = hexes.iter().filter(|h| h.r == r).map(|h| h.q).collect();
        let (Some(first), Some(last)) = (in_row.iter().min(), in_row.iter().max()) else {
            out.push_str(&format!("r {r:>3}:\n"));
            continue;
        };
        out.push_str(&format!("r {r:>3} q {first:>3}: "));
        out.push_str(&" ".repeat((2 * (first - q_min) + (r - r_min)) as usize));
        let mut line = String::new();
        for q in *first..=*last {
            let hex = Hex::new(q, r);
            let c = match (marks.get(&(q, r)), map.get(hex)) {
                (Some(c), _) => *c,
                (None, Some(data)) => match data.kind {
                    HexKind::Land => '#',
                    HexKind::Coast if closed_toward_water(map, hex) => '%',
                    HexKind::Coast => '+',
                    HexKind::DeepSea | HexKind::ShallowSea => '.',
                },
                (None, None) => ' ',
            };
            line.push(c);
            line.push(' ');
        }
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use sim_core::map::HexData;

    fn put(map: &mut Map, q: i32, r: i32, kind: HexKind, closed: &[usize]) {
        let mut data = HexData {
            kind,
            ..HexData::open_sea()
        };
        for d in closed {
            data.closed_sides[*d] = true;
        }
        map.insert(Hex::new(q, r), data);
    }

    /// Row 0 holds (0,0) land, then three coast hexes: (1,0) with a closed
    /// side to the coast hex (2,0), (2,0) closed back, and (3,0) whose only
    /// closed side (direction 4) leads to the land hex (2,1). Row 1 holds
    /// (-1,1) and (0,1) sea, then land (2,1); (1,1) is not on the map.
    ///
    /// - (0,0) is `#`; (1,0) and (2,0) are `%`; (3,0) is `+`, because its
    ///   closed side is toward land.
    /// - The least q is -1. Row 0 starts at q 0, 2 x (0 - -1) + 0 = 2
    ///   columns in; row 1 starts at q -1, 2 x 0 + 1 = 1 column in.
    #[test]
    fn rows_are_labelled_with_their_first_q_and_coast_with_closed_sides_is_percent() {
        let mut map = Map::new();
        put(&mut map, 0, 0, HexKind::Land, &[]);
        put(&mut map, 1, 0, HexKind::Coast, &[0]);
        put(&mut map, 2, 0, HexKind::Coast, &[3]);
        put(&mut map, 3, 0, HexKind::Coast, &[4]);
        put(&mut map, 2, 1, HexKind::Land, &[1]);
        put(&mut map, -1, 1, HexKind::DeepSea, &[]);
        put(&mut map, 0, 1, HexKind::DeepSea, &[]);
        let text = render(&map, &[((0, 1), 'X')]);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "r   0 q   0:   # % % +");
        assert_eq!(lines[1], "r   1 q  -1:  . X   #");
    }
}

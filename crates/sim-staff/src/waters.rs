//! The staff's regard for submarine waters (decision 0218, rule 55).
//!
//! Submarine waters, as the deciding command's picture holds them: a hex
//! inside any suspected waters held (within its distance of its hex, by hex
//! distance), or within one hex of any report of a submarine held. A place
//! the staff chooses for a standing force is taken outside them where
//! another place serves. Only the choice of the hex is regarded; no route is
//! changed (decision 0203's way is as it was). With no belief of submarines
//! no hex is in submarine waters and nothing here changes a word.

use sim_core::hex::Hex;
use sim_core::view::SideView;

use sim_core::fixed::Fixed;

/// Why `hex` lies in submarine waters the picture holds, or `None` where it
/// does not. The suspected waters are tried first, in the order told, then
/// the reports, in hex order.
pub(crate) fn in_submarine_waters(view: &SideView<'_>, hex: Hex) -> Option<String> {
    for waters in view.suspected_waters() {
        if hex.distance(waters.hex) <= waters.distance_hexes {
            return Some(format!(
                "inside the suspected submarine waters within {} hexes of {}",
                waters.distance_hexes, waters.hex
            ));
        }
    }
    for report in view.submarine_reports() {
        if hex.distance(report.hex) <= 1 {
            return Some(format!(
                "within one hex of a report of a submarine at {}, day {}",
                report.hex, report.day
            ));
        }
    }
    None
}

/// The hex chosen along a route, with the hexes passed over for submarine
/// waters before it.
pub(crate) struct Regard {
    pub(crate) hex: Hex,
    /// The hexes passed over, each with why, in route order.
    pub(crate) passed: Vec<(Hex, String)>,
    /// Every hex within the reach lay in submarine waters, so the first
    /// serves.
    pub(crate) none_outside: bool,
}

impl Regard {
    /// The words the forming reason adds: what was passed over and why, or
    /// that no place outside submarine waters serves. Empty where the hex
    /// is in no submarine waters and nothing was passed over.
    pub(crate) fn words(&self) -> String {
        if self.none_outside {
            return format!(
                "; no place outside submarine waters serves, so it is {}",
                self.hex
            );
        }
        self.passed
            .iter()
            .map(|(hex, why)| format!("; {hex} passed over: {why}"))
            .collect()
    }
}

/// [`crate::carrier::close_to`] with the regard: of the hexes of `path`
/// within `reach` of `contact`, the first not in submarine waters; where all
/// are in them, the first.
pub(crate) fn close_to_regarding(
    view: &SideView<'_>,
    path: &[Hex],
    contact: Hex,
    reach: Fixed,
) -> Option<Regard> {
    let mut passed: Vec<(Hex, String)> = Vec::new();
    let mut first: Option<Hex> = None;
    for hex in path
        .iter()
        .copied()
        .filter(|h| Fixed::from_int(i64::from(h.air_distance_nm(contact))) <= reach)
    {
        match in_submarine_waters(view, hex) {
            None => {
                return Some(Regard {
                    hex,
                    passed,
                    none_outside: false,
                })
            }
            Some(why) => {
                first.get_or_insert(hex);
                passed.push((hex, why));
            }
        }
    }
    first.map(|hex| Regard {
        hex,
        passed: Vec::new(),
        none_outside: true,
    })
}

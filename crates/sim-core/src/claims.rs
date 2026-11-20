//! Who made a raid's claim, and the types of the aircraft it claims
//! destroyed on the ground (decisions 0179 and 0180).
//!
//! The functions here are the rules' own arithmetic, with no draw and no
//! state: the world calls them as a raid's claim is made, and the tests
//! call them on the hand calculation's tables.

use crate::ids::{AirGroupId, AircraftTypeId, EntityRef};

/// One unit's line in a raid's claim (decision 0180, 1): what the unit
/// claims of the aircraft shot down, of the aircraft destroyed on the
/// ground and of the hits on ships. The unit is an air group, or, for
/// guns, the task force or the base the firing is recorded under.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitClaim {
    pub unit: EntityRef,
    pub shot_down: u32,
    pub on_ground: u32,
    pub hits: u32,
}

impl UnitClaim {
    /// A line of nothing, which a claim leaves out.
    pub const fn is_nothing(&self) -> bool {
        self.shot_down == 0 && self.on_ground == 0 && self.hits == 0
    }
}

/// One group's part of a firing of several groups (decision 0180, 3): its
/// whole shots, the aircraft its shots really shot down, and its shots
/// that missed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FiringPart {
    pub group: AirGroupId,
    pub shots: u32,
    pub real: u32,
    pub missed: u32,
}

/// Shares `whole` among parts in proportion to `weights` (decisions 0179, 3
/// and 0180, 3): each is given the whole part of `whole` times its share,
/// and what is left goes one at a time to the parts with the largest part
/// cut off, a tie to the one given first (the lower number: the caller
/// gives them in order of number). Every weight above 0; the result adds
/// to `whole`.
fn share_out(whole: u32, weights: &[u32]) -> Vec<u32> {
    let total: u64 = weights.iter().map(|weight| u64::from(*weight)).sum();
    if total == 0 {
        return vec![0; weights.len()];
    }
    let mut given: Vec<u32> = Vec::with_capacity(weights.len());
    let mut cut: Vec<u64> = Vec::with_capacity(weights.len());
    for weight in weights {
        let product = u64::from(whole) * u64::from(*weight);
        given.push(u32::try_from(product / total).expect("a share is no more than the whole"));
        cut.push(product % total);
    }
    let left = whole - given.iter().sum::<u32>();
    let mut order: Vec<usize> = (0..weights.len()).collect();
    // The largest part cut off first; on a tie, the one given first.
    order.sort_by_key(|index| (std::cmp::Reverse(cut[*index]), *index));
    for index in order.into_iter().take(left as usize) {
        given[index] += 1;
    }
    given
}

/// Parts a firing of several groups among them, shot by shot (decision
/// 0180, 3). `in_firing` is each group and the aircraft it has in the
/// firing; `shots` the firing's whole shots; `hit_shots` the shots that hit,
/// counted from 1 in the order drawn; `real` the aircraft the firing really
/// shot down, no more than its hits. The shots are shared in proportion to
/// the aircraft; the group with the lowest number has the first of them,
/// and each group has the hits of its own shots. Where the firing really
/// shot down fewer than it hit, the cut comes off the last shots first: the
/// first `real` hits in order are the real ones.
///
/// Gives the groups in order of number, a group with no shots among them.
pub fn part_firing(
    in_firing: &[(AirGroupId, u32)],
    shots: u32,
    hit_shots: &[u32],
    real: u32,
) -> Vec<FiringPart> {
    let mut groups: Vec<(AirGroupId, u32)> = in_firing
        .iter()
        .copied()
        .filter(|(_, aircraft)| *aircraft > 0)
        .collect();
    groups.sort_by_key(|(group, _)| *group);
    let weights: Vec<u32> = groups.iter().map(|(_, aircraft)| *aircraft).collect();
    let shares = share_out(shots, &weights);
    let real_shots: Vec<u32> = hit_shots.iter().copied().take(real as usize).collect();
    let mut first = 1;
    groups
        .iter()
        .zip(shares)
        .map(|((group, _), share)| {
            let range = first..first + share;
            first += share;
            let hits = u32::try_from(hit_shots.iter().filter(|s| range.contains(s)).count())
                .expect("few shots");
            let real = u32::try_from(real_shots.iter().filter(|s| range.contains(s)).count())
                .expect("few shots");
            FiringPart {
                group: *group,
                shots: share,
                real,
                missed: share - hits,
            }
        })
        .collect()
}

/// The lines of the groups that dropped bombs or torpedoes in a raid
/// (decision 0180, 5): `groups` is each group, in the order the groups
/// attacked, with what it dropped and what it really did (aircraft
/// destroyed on a base, hits on ships). Its line is what it really did and
/// the false claims `false_of` draws for what of its own did not count,
/// group by group in that order. On a base the line is of aircraft on the
/// ground, else of hits. A group that claims nothing has a line of nothing,
/// which the claim leaves out.
pub fn strike_lines(
    groups: &[(AirGroupId, u32, u32)],
    on_base: bool,
    mut false_of: impl FnMut(u32) -> u32,
) -> Vec<UnitClaim> {
    groups
        .iter()
        .map(|(group, dropped, real)| {
            let claimed = real + false_of(dropped.saturating_sub(*real));
            UnitClaim {
                unit: EntityRef::AirGroup(*group),
                shot_down: 0,
                on_ground: if on_base { claimed } else { 0 },
                hits: if on_base { 0 } else { claimed },
            }
        })
        .collect()
}

/// The types of the aircraft a raid claims destroyed on the ground, when
/// the raiders make them out (decision 0179, 3): `real` the aircraft its
/// bombs really destroyed, by the type of the group on the ground they
/// belonged to; `false_claimed` the aircraft it claims falsely; `field` the
/// aircraft on the ground at the base as first counted, by type. The false
/// ones are shared over the field's types in proportion to their numbers
/// there, the whole part first and what is left one at a time to the types
/// with the largest part cut off, a tie to the lower number. No draw.
///
/// Gives the list in order of the type's number, with no pair of 0: empty
/// when the field is, else adding to the aircraft claimed.
pub fn ground_claim_types(
    real: &[(AircraftTypeId, u32)],
    false_claimed: u32,
    field: &[(AircraftTypeId, u32)],
) -> Vec<(AircraftTypeId, u32)> {
    let mut field: Vec<(AircraftTypeId, u32)> = field
        .iter()
        .copied()
        .filter(|(_, aircraft)| *aircraft > 0)
        .collect();
    if field.is_empty() {
        return Vec::new();
    }
    field.sort_by_key(|(kind, _)| *kind);
    let weights: Vec<u32> = field.iter().map(|(_, aircraft)| *aircraft).collect();
    let shares = share_out(false_claimed, &weights);
    let mut list: std::collections::BTreeMap<AircraftTypeId, u32> =
        std::collections::BTreeMap::new();
    for (kind, aircraft) in real {
        *list.entry(*kind).or_insert(0) += aircraft;
    }
    for ((kind, _), share) in field.iter().zip(shares) {
        *list.entry(*kind).or_insert(0) += share;
    }
    list.into_iter().filter(|(_, n)| *n > 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_share_goes_whole_part_first_then_by_the_largest_part_cut_off() {
        assert_eq!(share_out(3, &[6, 4]), vec![2, 1]);
        assert_eq!(share_out(5, &[1, 1, 1]), vec![2, 2, 1]);
        assert_eq!(share_out(0, &[1, 1]), vec![0, 0]);
        assert_eq!(share_out(1, &[1, 1]), vec![1, 0]);
    }
}

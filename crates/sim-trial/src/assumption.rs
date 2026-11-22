//! What was assumed in making a picture's enemy (decision 0174, 21): one
//! entry for each thing made or left out, with the rule and the figures it
//! was made by, in the order the rules were worked. It is what the estimate
//! will name (decision 0161, 7 and 13). Data beside the picture.

use core::fmt;

use sim_core::believed::Standing;
use sim_core::command::IntentKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, LandUnitId, TaskForceId};
use sim_core::refdata::{AircraftRole, ShipKind};

/// What a force was made for: a contact, two contacts taken for one force,
/// or a force the side believed in. A contact and a believed force are
/// named by their place in the side's own list, from 1.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Source {
    Contact(u32),
    /// Two contacts taken for one force (rule 12): the first and the second
    /// by their places.
    Pair(u32, u32),
    /// A believed force, by its place in the side's list and its name.
    Believed(u32, String),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::Contact(place) => write!(f, "contact {place}"),
            Source::Pair(first, second) => write!(f, "contacts {first} and {second}"),
            Source::Believed(place, name) => write!(f, "believed force {place} ({name})"),
        }
    }
}

/// One thing assumed in making a picture's enemy.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Assumption {
    /// Rule 5: a base of the enemy's on the side's map, given the trial's
    /// figures of supply and fuel, no guns, and an airfield with nothing at
    /// it.
    EnemyBase {
        base: BaseId,
        hex: Hex,
        supply_t: Fixed,
        fuel_t: Fixed,
    },
    /// Rule 6: a fighting unit of an estimate's men, in its hex, on defend.
    MenAshore {
        unit: LandUnitId,
        hex: Hex,
        men: u32,
    },
    /// Rule 11: a claim of the side's own, from the air, against a more
    /// important kind, within the merging distance of a contact's place,
    /// raised the contact's kind. The claim is named by its place in the
    /// side's list of claims.
    KindRaised {
        contact: u32,
        claim: u32,
        from: ShipKind,
        to: ShipKind,
    },
    /// Rule 12: two contacts that may be one force, and whether they are
    /// one in this world.
    Pair { first: u32, second: u32, one: bool },
    /// Rule 8: ships of a kind not built, for the enemy has no class of it.
    ShipsLeftOut {
        source: Source,
        kind: ShipKind,
        count: u32,
    },
    /// Rule 9: a carrier's air group not made, for the enemy has no type of
    /// its role, or its share of the carrier's aircraft is none.
    GroupLeftOut {
        source: Source,
        role: AircraftRole,
        aircraft: u32,
    },
    /// Rules 7 to 10 and 16: a force made, of a kind, with its ships, and
    /// its air groups.
    ForceMade {
        force: TaskForceId,
        source: Source,
        kind: ShipKind,
        ships: u32,
        groups: Vec<AirGroupId>,
    },
    /// No force made: no ship of it could be built, or it could be placed
    /// nowhere ships can be.
    ForceNotMade { source: Source, why: NotMade },
    /// Rules 13 and 17: where a force is put, from the hex it was reported
    /// or last known at: the hours since, the speed of its slowest class,
    /// the miles, and the steps those give, at 40 miles a step, rounded
    /// down. With 0 hours it is at that hex, and nothing is drawn.
    Placed {
        force: TaskForceId,
        from: Hex,
        hours: u32,
        speed_kn: Fixed,
        miles: Fixed,
        steps: u32,
        at: Hex,
    },
    /// Rule 17: a believed force of no known place, put at a base of the
    /// enemy's on the side's map.
    AtEnemyBase {
        force: TaskForceId,
        base: BaseId,
        hex: Hex,
    },
    /// Rule 14: a force believed absent, built in no world: the side ruled
    /// it out.
    RuledOut { place: u32, name: String },
    /// Rule 14: a force of unknown standing, not built in this world.
    UnknownNotInWorld { place: u32, name: String },
    /// Rule 15: a force believed in is not built, for a contact the side
    /// holds is taken for it.
    TakenForContact {
        place: u32,
        name: String,
        contact: u32,
    },
    /// Rule 18: the enemy is given no belief: no contact, no claim and no
    /// estimate.
    EnemyGivenNoBelief,
    /// Rule 19: an intent the enemy is taken to hold, handed in for him on
    /// the first morning with his leaves given.
    EnemyIntent { kind: IntentKind },
    /// Rule 19: the side believes nothing of what the enemy wants. He is
    /// given no intent and holds what he holds.
    EnemyGivenNoIntent,
    /// Decision 0176, rule 2: believed men made, a fighting unit of their men
    /// in their hex on defend, made as the unit of an estimate is. Named by
    /// their place in the side's list of believed men, from 1.
    BelievedMenMade {
        unit: LandUnitId,
        place: u32,
        name: String,
        hex: Hex,
        men: u32,
        standing: Standing,
    },
    /// Decision 0176, rule 2: believed men believed absent, made in no world.
    BelievedMenRuledOut {
        place: u32,
        name: String,
        hex: Hex,
        men: u32,
    },
    /// Decision 0176, rule 2: believed men of unknown standing, not made in
    /// this world.
    BelievedMenNotInWorld {
        place: u32,
        name: String,
        hex: Hex,
        men: u32,
    },
    /// Decision 0176, amended 6: believed men set aside, made in no world, for
    /// the side knows better.
    BelievedMenSetAside {
        place: u32,
        name: String,
        hex: Hex,
        men: u32,
        standing: Standing,
        why: SetAside,
    },
    /// Decision 0209, rule 12: the enemy's men ashore are taken as supplied
    /// in full in their fights and charged nothing, for the side does not
    /// know his stocks. Made only where the figures of 0209 are given.
    EnemySupplied,
}

/// Why believed men were set aside (decision 0176, amended 6).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetAside {
    /// The side holds an estimate of the hex of a day after the scenario's
    /// start: a count it has had since, of that day.
    CountSince { day: u32 },
    /// The place is the side's: it holds a base in the hex and the enemy
    /// none there on its map.
    PlaceIsTheSides,
}

/// Why no force was made.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NotMade {
    /// No ship of it could be built (rule 8).
    NoShip,
    /// No hex ships can be in is within its reach of where it was (rule 13).
    NowhereToBe,
    /// Its place is not known and the side's map has no base of the enemy's
    /// (rule 17).
    NoEnemyBase,
}

impl Assumption {
    /// The decision whose rule it was made by: 174, or 176 for believed men.
    pub const fn decision(&self) -> u32 {
        match self {
            Assumption::BelievedMenMade { .. }
            | Assumption::BelievedMenRuledOut { .. }
            | Assumption::BelievedMenNotInWorld { .. }
            | Assumption::BelievedMenSetAside { .. } => 176,
            Assumption::EnemySupplied => 209,
            _ => 174,
        }
    }

    /// The number of the rule it was made by, of decision 0174, or of
    /// decision 0176 for believed men ([`Assumption::decision`]).
    pub const fn rule(&self) -> u32 {
        match self {
            Assumption::BelievedMenMade { .. }
            | Assumption::BelievedMenRuledOut { .. }
            | Assumption::BelievedMenNotInWorld { .. }
            | Assumption::BelievedMenSetAside { .. } => 2,
            Assumption::EnemySupplied => 12,
            Assumption::EnemyBase { .. } => 5,
            Assumption::MenAshore { .. } => 6,
            Assumption::KindRaised { .. } => 11,
            Assumption::Pair { .. } => 12,
            Assumption::ShipsLeftOut { .. } => 8,
            Assumption::GroupLeftOut { .. } => 9,
            Assumption::ForceMade { source, .. } => match source {
                Source::Believed(..) => 16,
                Source::Contact(_) | Source::Pair(..) => 10,
            },
            Assumption::ForceNotMade { why, .. } => match why {
                NotMade::NoShip => 8,
                NotMade::NowhereToBe => 13,
                NotMade::NoEnemyBase => 17,
            },
            Assumption::Placed { .. } => 13,
            Assumption::AtEnemyBase { .. } => 17,
            Assumption::RuledOut { .. } | Assumption::UnknownNotInWorld { .. } => 14,
            Assumption::TakenForContact { .. } => 15,
            Assumption::EnemyGivenNoBelief => 18,
            Assumption::EnemyIntent { .. } | Assumption::EnemyGivenNoIntent => 19,
        }
    }
}

fn kind_name(kind: ShipKind) -> &'static str {
    kind.name()
}

fn role_name(role: AircraftRole) -> &'static str {
    match role {
        AircraftRole::Patrol => "patrol aircraft",
        AircraftRole::Bomber => "bombers",
        AircraftRole::Fighter => "fighters",
    }
}

impl fmt::Display for Assumption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.decision() == 174 {
            write!(f, "rule {}: ", self.rule())?;
        } else {
            write!(f, "rule {} of decision 0{}: ", self.rule(), self.decision())?;
        }
        match self {
            Assumption::EnemyBase {
                base,
                hex,
                supply_t,
                fuel_t,
            } => write!(
                f,
                "{base} at {hex} is given {supply_t} t of supply and {fuel_t} t of fuel, no \
                 guns, and an airfield with nothing at it (enemy_base_supply_t, \
                 enemy_base_fuel_t)"
            ),
            Assumption::MenAshore { unit, hex, men } => write!(
                f,
                "{unit}, a fighting unit of {men} men, the side's estimate, is ashore at {hex} \
                 on defend"
            ),
            Assumption::KindRaised {
                contact,
                claim,
                from,
                to,
            } => write!(
                f,
                "contact {contact}, taken for a {}, is built as a {}, the kind of claim \
                 {claim}",
                kind_name(*from),
                kind_name(*to)
            ),
            Assumption::Pair { first, second, one } => {
                if *one {
                    write!(
                        f,
                        "contacts {first} and {second} are one force in this world"
                    )
                } else {
                    write!(
                        f,
                        "contacts {first} and {second} are two forces in this world"
                    )
                }
            }
            Assumption::ShipsLeftOut {
                source,
                kind,
                count,
            } => write!(
                f,
                "{count} {} of {source} left out: the enemy has no class of the kind",
                kind_name(*kind)
            ),
            Assumption::GroupLeftOut {
                source,
                role,
                aircraft,
            } => write!(
                f,
                "the {} of {source}, {aircraft} aircraft, left out: none to give, or the \
                 enemy has no type of the role",
                role_name(*role)
            ),
            Assumption::ForceMade {
                force,
                source,
                kind,
                ships,
                groups,
            } => write!(
                f,
                "{force} made for {source}: {ships} ships, a {} force, with {} air groups",
                kind_name(*kind),
                groups.len()
            ),
            Assumption::ForceNotMade { source, why } => match why {
                NotMade::NoShip => write!(f, "no force made for {source}: no ship of it built"),
                NotMade::NowhereToBe => write!(
                    f,
                    "no force made for {source}: nowhere ships can be within its reach"
                ),
                NotMade::NoEnemyBase => write!(
                    f,
                    "no force made for {source}: its place is not known and the map has no \
                     base of the enemy's"
                ),
            },
            Assumption::Placed {
                force,
                from,
                hours,
                speed_kn,
                miles,
                steps,
                at,
            } => write!(
                f,
                "{force} is at {at}: {hours} hours from {from} at {speed_kn} kn is {miles} nm, \
                 {steps} steps"
            ),
            Assumption::AtEnemyBase { force, base, hex } => {
                write!(f, "{force}, its place not known, is at {base}, at {hex}")
            }
            Assumption::RuledOut { place, name } => write!(
                f,
                "believed force {place} ({name}) is believed absent: the side ruled it out"
            ),
            Assumption::UnknownNotInWorld { place, name } => write!(
                f,
                "believed force {place} ({name}), its standing unknown, is not in this world"
            ),
            Assumption::TakenForContact {
                place,
                name,
                contact,
            } => write!(
                f,
                "contact {contact} is taken for believed force {place} ({name}), which is not \
                 built"
            ),
            Assumption::EnemyGivenNoBelief => write!(
                f,
                "the enemy is given no belief: no contact, no claim and no estimate"
            ),
            Assumption::EnemyIntent { kind } => {
                write!(f, "the enemy is taken to intend {kind:?}, his leaves given")
            }
            Assumption::EnemyGivenNoIntent => write!(
                f,
                "the side believes nothing of what the enemy wants: he is given no intent"
            ),
            Assumption::BelievedMenMade {
                unit,
                place,
                name,
                hex,
                men,
                standing,
            } => write!(
                f,
                "{unit}, a fighting unit of {men} men, believed men {place} ({name}), standing \
                 {}, is ashore at {hex} on defend",
                standing.name()
            ),
            Assumption::BelievedMenRuledOut {
                place,
                name,
                hex,
                men,
            } => write!(
                f,
                "believed men {place} ({name}), {men} at {hex}, are believed absent: the side \
                 ruled them out"
            ),
            Assumption::BelievedMenNotInWorld {
                place,
                name,
                hex,
                men,
            } => write!(
                f,
                "believed men {place} ({name}), {men} at {hex}, their standing unknown, are not \
                 in this world"
            ),
            Assumption::BelievedMenSetAside {
                place,
                name,
                hex,
                men,
                why,
                ..
            } => match why {
                SetAside::CountSince { day } => write!(
                    f,
                    "believed men {place} ({name}), {men} at {hex}, are set aside: the side has \
                     a count of the place since, of day {day}"
                ),
                SetAside::PlaceIsTheSides => write!(
                    f,
                    "believed men {place} ({name}), {men} at {hex}, are set aside: the place is \
                     the side's"
                ),
            },
            Assumption::EnemySupplied => write!(
                f,
                "the enemy's men ashore are taken as supplied in full in their fights, and \
                 charged nothing for them"
            ),
        }
    }
}

//! Flag Plot trials (decisions 0174 and 0175, steps 4 and 5 of step 3's
//! stage 4).
//!
//! A trial tries a side's course in many worlds that could be the one it is
//! in. This crate makes those worlds' pictures, builds the worlds and plays
//! them (decision 0161, D).
//!
//! - [`pictures`] makes the pictures of a trial: from a side's view, what
//!   that side believed (the forces it believed in and the intents it
//!   believed the enemy held, decision 0173), the trial's figures and the
//!   trial's settings, one picture for each world, numbered from 1. The
//!   side's own half is its view's, unchanged (decision 0171). The enemy's
//!   half is made by the written rules of decision 0174, and each picture
//!   comes with the list of what was assumed in making it ([`Assumption`]).
//! - [`pictures_from_half`] does the same from a side's own half handed in
//!   beside its view, so that a test can alter what the half believes. The
//!   half must be of the view's side and day.
//! - [`build_world`] builds the world of a picture, from the reference data
//!   and the map it is handed, on the stream of its trial and its world
//!   (decision 0170, 12), and hands in for the enemy, for the first morning,
//!   the intents the side believed he held (rule 19).
//! - [`estimate`] reads an estimate of one intent of the side tried from a
//!   trial as played (decision 0176): in how many worlds its end was met and
//!   kept, met and lost again, or not met; what it cost each side; what was
//!   assumed; and where the worlds part. In counts of worlds, never in
//!   chances or points. [`met`] is the test of an end, usable on a live
//!   game's view as on a world's record.
//! - [`begin`] and [`play`] play a trial (decision 0175): one course, the
//!   commander's commands of the side tried each on a day of the world, in
//!   each world from 1, for a horizon of days, with both sides' staffs on
//!   doctrine, on the trial's figures' seed. [`begin_from_pictures`] plays
//!   pictures handed in. [`Playing`] is worked a world's day at a time and
//!   can be dropped at any step; [`Played`] is the worlds as played, each
//!   with its number, its assumptions, its days, the commands of the course
//!   it refused, why it could not be played if it could not, and the world
//!   as its play left it.
//!
//! What the rules read is fixed (decision 0161, 6): a side's view, what the
//! scenario says that side believed, and the trial's figures and settings.
//! Never the world, the umpire's view or the full log. A test reads this
//! crate's source to check (`tests/source.rs`). Floating point is
//! forbidden, as in `sim-core`.
//!
//! Every draw is the trial's own (rule 3), in the trial's domain: keyed on
//! the seed, the trial, the world, the picture's day, and the thing drawn
//! for by its place in the side's own lists. The same view, beliefs, seed,
//! trial and world give the same picture, byte for byte.

#![deny(clippy::float_arithmetic)]

mod assumption;
mod draw;
mod enemy;
mod estimate;
mod play;
mod window;

pub use assumption::{Assumption, NotMade, SetAside, Source};
pub use estimate::{
    end_of, estimate, met, Assumed, AssumedNote, AssumedOf, Costs, Displaced, End, Estimate, Kept,
    Losses, OtherPlan, PartSide, PartedBy, Parting, Spread, Went, WorldFigures,
};
pub use play::{
    begin, begin_from_pictures, play, Asked, BasesAt, CourseCommand, Failed, Landing, Played,
    PlayedDay, PlayedWorld, Playing, RefusedCommand, Stepped, TakenIn,
};
pub use sim_core::refdata::TrialSize;
pub use window::{
    begin_window, dates, given_dates, play_window, window, world_for_world, Chosen, CoverSpread,
    Date, DatePlayed, Dates, Due, DueKind, EnemyAt, EnemyOnTheEve, PlayedWindow, Window,
    WindowAsked, WindowDate, WindowPlaying, WorldForWorld, MOST_DATES,
};

use core::fmt;

use sim_core::believed::{BelievedForce, BelievedIntent, BelievedMen};
use sim_core::command::{CommandKind, IntentKind, Leave, PursuitBy};
use sim_core::ids::IntentId;
use sim_core::map::Map;
use sim_core::refdata::{RefData, TrialFigures};
use sim_core::rng::{Domain, Stream};
use sim_core::state::Side;
use sim_core::view::SideView;
use sim_core::world::SimError;
use sim_core::world_picture::{PictureHalf, WorldPicture};
use sim_core::World;

/// A trial's settings (decision 0174, 2): its seed, its number and how many
/// worlds it has.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Trial {
    pub seed: u64,
    /// The trial's number: the first part of its draws' domain.
    pub trial: u32,
    /// How many worlds: the pictures are of worlds 1 to this.
    pub worlds: u32,
}

/// What a side believed when the game began (decision 0173): the forces of
/// the enemy's it believed in and had not seen, and the intents it believed
/// he held; and the men of the enemy's it believed may be ashore at a place
/// beyond its estimate (decision 0176, 1); each in the order the scenario
/// gives them. Every entry must be the tried side's own belief.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Believed {
    pub forces: Vec<BelievedForce>,
    pub intents: Vec<BelievedIntent>,
    pub men: Vec<BelievedMen>,
}

/// The picture of one world of a trial, with what was assumed in making it
/// (decision 0174, 2 and 21).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrialPicture {
    /// The world's number in its trial, from 1.
    pub world: u32,
    /// What the built world draws from: the trial's seed, and the domain of
    /// the trial and the world.
    pub stream: Stream,
    /// The side tried: the one whose view the picture was made from.
    pub side: Side,
    /// The picture: the side's own half and the enemy's, joined.
    pub picture: WorldPicture,
    /// What the enemy is taken to want (rule 19): the intents the side
    /// believed he held, in the order given. [`build_world`] hands them in.
    pub enemy_intents: Vec<IntentKind>,
    /// What was assumed, in the order the rules were worked (rule 21). Data
    /// beside the picture and no part of it.
    pub assumptions: Vec<Assumption>,
}

/// Why a trial's pictures were not made: words that name the fault.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Refused(pub String);

impl fmt::Display for Refused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a trial is refused: {}", self.0)
    }
}

impl std::error::Error for Refused {}

/// The pictures of a trial (decision 0174, 2), from a side's view: one for
/// each world, 1 to `trial.worlds`, the side's own half as the view gives
/// it ([`SideView::own_half`]) and the enemy's made by rule. Refused where
/// the view gives no half (a day open, a view cut to a command), where a
/// belief handed in is not the view's side's, where the view has no search
/// figures, and where the side's own units reach the numbers the made
/// units take (rule 20).
pub fn pictures(
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    trial: Trial,
) -> Result<Vec<TrialPicture>, Refused> {
    let half = view.own_half().map_err(|e| Refused(e.to_string()))?;
    pictures_from_half(&half, view, believed, figures, trial)
}

/// [`pictures`] from a side's own half handed in beside its view: the
/// half's units and belief are read, and the view gives the rest (the map,
/// the reference data and the bases of the enemy's on the side's map). The
/// half must be of the view's side and day.
pub fn pictures_from_half(
    half: &PictureHalf,
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    trial: Trial,
) -> Result<Vec<TrialPicture>, Refused> {
    if half.side != view.side() || half.day != view.day() {
        return Err(Refused(format!(
            "the half is the {} side's of day {}, and the view the {} side's of day {}",
            half.side.name(),
            half.day,
            view.side().name(),
            view.day()
        )));
    }
    enemy::check(half, view, believed)?;
    // The commander's answers to a force's asking leave to pursue name a
    // contact by its number and, for a yes, the event of the answer
    // (decision 0214, "Who answers where no one is there"). A made-up world's
    // contacts are numbered anew and its log is its own, so the answers are
    // not carried into it. The side's settings (`set_pursuit`) are, as part
    // of its orders.
    let mut own = half.clone();
    own.command
        .pursuits
        .retain(|_, pursuit| !matches!(pursuit.by, PursuitBy::Answer(_)));
    own.command.pursuit_refusals.clear();
    (1..=trial.worlds)
        .map(|world| {
            let (enemy_half, assumptions) =
                enemy::make(half, view, believed, figures, trial, world)?;
            let picture = WorldPicture::from_halves(own.clone(), enemy_half)
                .map_err(|e| Refused(e.to_string()))?;
            Ok(TrialPicture {
                world,
                stream: Stream {
                    seed: trial.seed,
                    domain: Domain {
                        trial: trial.trial,
                        world,
                    },
                },
                side: half.side,
                picture,
                enemy_intents: believed.intents.iter().map(|intent| intent.kind).collect(),
                assumptions,
            })
        })
        .collect()
}

/// The world of a picture (decision 0174, 2), built by
/// [`World::from_picture`] from the reference data and the map handed in,
/// on the picture's stream. What the enemy is taken to want is handed in
/// for him for the first morning (rule 19): his leave to prepare and to
/// launch, then each intent as a commander's `set_intent`, numbered from 1
/// in the order given, at the risk a commander gets by default. With no
/// intent nothing is handed in. Refused as the constructor refuses.
pub fn build_world(picture: &TrialPicture, ref_data: RefData, map: Map) -> Result<World, SimError> {
    // The world is played on the figures as the side knows them: a figure
    // given by side holds the side's own value for both, so the enemy it
    // makes up does as its own navy does (decision 0213, as reviewed, 3).
    let ref_data = ref_data.as_seen_by(picture.side);
    let mut built = World::from_picture(picture.picture.clone(), ref_data, map, picture.stream)?;
    // The side the picture makes up is taken as supplied in full in its
    // fights ashore, and charged nothing (decision 0209, 12): the side does
    // not know his stocks. It acts only where the figures of 0209 are given.
    built.set_supplied_in_full(enemy::other(picture.side));
    if !picture.enemy_intents.is_empty() {
        let mut commands = vec![
            CommandKind::SetLeave {
                leave: Leave::Prepare,
                given: true,
            },
            CommandKind::SetLeave {
                leave: Leave::Launch,
                given: true,
            },
        ];
        commands.extend((1..).zip(&picture.enemy_intents).map(|(number, kind)| {
            CommandKind::SetIntent {
                intent: IntentId::new(number),
                kind: *kind,
            }
        }));
        built.submit_commands(enemy::other(picture.side), commands);
    }
    Ok(built)
}

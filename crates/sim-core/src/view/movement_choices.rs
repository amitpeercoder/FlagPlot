//! A side's questions of the passage of task forces: turning back or going on when a route changes, and halting at a marked waypoint and steaming on (decision 0100, 2).
//!
//! Each answer is the engine's own function ([`TaskForce::reroute_centre`],
//! `movement::halt_index` and `movement::steam_on`) fed from the side's own
//! task forces, ships, bases and land units, the figures and the map. The
//! enemy bases come only as the map shows them: where a base is and which
//! side holds it, for a base that is on the side's map
//! ([`SideView::enemy_bases`]).

use crate::choices::movement::{MovementAnswer, MovementQuestion, Rerouted};
use crate::hex::Hex;
use crate::ids::{BaseId, TaskForceId};
use crate::movement::{halt_index, steam_on, HaltGround};
use crate::state::TaskForce;
use crate::view::SideView;

impl SideView<'_> {
    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise.
    pub(crate) fn ask_movement(&self, question: &MovementQuestion) -> Option<MovementAnswer> {
        match *question {
            MovementQuestion::Reroute {
                force,
                first_waypoint,
            } => {
                let force = self.own_force(force)?;
                Some(MovementAnswer::Reroute(
                    force.reroute_centre(self.map, first_waypoint).map(
                        |(turns_back, steers_from)| Rerouted {
                            turns_back,
                            steers_from,
                        },
                    ),
                ))
            }
            MovementQuestion::Halt { force } => {
                let force = self.own_force(force)?;
                let ground = self.halt_ground();
                Some(MovementAnswer::Halt(halt_index(&ground, force, &|at| {
                    self.enemy_base_on_map_at(at)
                })))
            }
            MovementQuestion::SteamOn { force } => {
                let force = self.own_force(force)?;
                if force.halt.is_none() || force.ships.is_empty() {
                    return None;
                }
                let ground = self.halt_ground();
                Some(MovementAnswer::SteamOn(steam_on(&ground, force, &|at| {
                    self.enemy_base_on_map_at(at)
                })))
            }
        }
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its units the choice applies to: for each force, whether a new route
    /// from the waypoint it is steaming toward (or none) would turn it back,
    /// and where it would halt; for each halted force, whether it steams on.
    pub(crate) fn movement_questions(&self) -> Vec<MovementQuestion> {
        let mut questions = Vec::new();
        for force in self.task_forces() {
            questions.push(MovementQuestion::Reroute {
                force: force.id,
                first_waypoint: force.route.waypoints.get(force.route.next_index).copied(),
            });
            questions.push(MovementQuestion::Halt { force: force.id });
            if force.halt.is_some() && !force.ships.is_empty() {
                questions.push(MovementQuestion::SteamOn { force: force.id });
            }
        }
        questions
    }

    /// One of the side's own task forces.
    fn own_force(&self, id: TaskForceId) -> Option<&TaskForce> {
        self.truth
            .task_forces
            .get(&id)
            .filter(|force| force.side == self.side)
    }

    /// What the movement rules read of the world: the figures, the map, and
    /// the ships, bases and land units, indexed only by the side's own
    /// units (see [`HaltGround`]).
    fn halt_ground(&self) -> HaltGround<'_> {
        HaltGround {
            ref_data: self.ref_data,
            map: self.map,
            ships: &self.truth.ships,
            bases: &self.truth.bases,
            land_units: &self.truth.land_units,
        }
    }

    /// The lowest-numbered base on the map that the side does not own in
    /// `hex`: the public fact the bombardment rule reads.
    fn enemy_base_on_map_at(&self, hex: Hex) -> Option<BaseId> {
        self.enemy_bases()
            .find(|base| base.hex == hex)
            .map(|base| base.id)
    }
}

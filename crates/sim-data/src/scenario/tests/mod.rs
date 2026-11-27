use super::objectives::*;
use super::*;
use crate::error::DataError;
use crate::quantity::Quantity;
use serde::Deserialize;
use sim_core::believed::BelievedIntent;
use sim_core::command::CommandKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, EntityRef, IntentId, LandUnitId, ShipClassId, ShipId,
    SubmarineClassId, SubmarineId, TaskForceId,
};
use sim_core::objectives::Objective;
use sim_core::orders::{
    Activity, AirMission, BaseOrder, Cargo, Job, LandUnitOrder, OrderKind, Readiness, SpeedSetting,
    Stance, StragglerInstruction, TaskForceOrder,
};
use sim_core::pictures::CommandRef;
use sim_core::refdata::ShipKind;
use sim_core::state::RulesOfEngagement;
use sim_core::state::{
    AirGroupLocation, ShipSystems, Side, Skills, SubmarineCondition, SystemState,
};
use sim_core::time::Pulse;
use sim_core::world::MADE_IDS_FLOOR;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

mod air;
mod bases;
mod beliefs;
mod commanders;
mod file;
mod islands;
mod land;
mod objectives;
mod orders;
mod ships;
mod submarines;
mod supply;

const HEAD: &str = r#"
name = "Test"
placeholder = true
seed = 5
days = 2
map = "../map/placeholder_search.toml"
ship_classes = "../reference/ship_classes.csv"
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"

[[base]]
id = 1
name = "Allied base"
side = "allied"
q = 0
r = 0

[[base]]
id = 2
name = "Japanese base"
side = "japanese"
q = 5
r = 0
"#;

/// Parses `HEAD` plus `extra`, as if it sat in the scenarios folder.
fn parse_with(extra: &str) -> Result<Scenario, DataError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    Scenario::parse(&format!("{HEAD}{extra}"), &path)
}

fn error_of(extra: &str) -> String {
    parse_with(extra)
        .expect_err("the scenario is rejected")
        .message
}

fn task_force(placement: &str) -> String {
    format!(
            "[[task_force]]\nid = 1\nname = \"TF\"\nside = \"allied\"\n{placement}\nmission = \"escort\"\n"
        )
}

fn air_group(base: u32, aircraft_type: u32) -> String {
    format!(
            "[[air_group]]\nid = 1\nname = \"AG\"\nside = \"allied\"\nbase = {base}\naircraft_type = {aircraft_type}\nready = 3\n"
        )
}

fn arrival(day: u32, group: u32, aircraft: u32, pilots: &str) -> String {
    format!("[[arrival]]\nday = {day}\nair_group = {group}\naircraft = {aircraft}\n{pilots}")
}

fn error_of_group<T>(result: &Result<T, DataError>) -> String {
    match result {
        Ok(_) => panic!("expected an error"),
        Err(error) => error.to_string(),
    }
}

/// `HEAD` with the air combat key added after the surface combat key.
fn with_air_combat(extra: &str) -> Result<Scenario, DataError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let head = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            "surface_combat = \"../reference/surface_combat.toml\"\nair_combat = \"../reference/air_combat.toml\"\n",
        );
    Scenario::parse(&format!("{head}{extra}"), &path)
}

fn ship(extra: &str) -> String {
    format!(
            "[[ship]]\nid = 1\nname = \"S\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = 1\n{extra}\n"
        )
}

fn with_force(extra: &str) -> String {
    format!("{}{}", task_force("at_base = 1"), extra)
}

// Land units (decision 0066).

/// A small scenario on the landing map: base 1 at (0,0), a force of two
/// troop transports (600 men each) at it, a force of one fast transport
/// (200 men) at sea at (10,0), a Japanese force, and `extra` after them.
/// `ground` says whether the ground figures are named.
fn landing(ground: bool, extra: &str) -> Result<Scenario, DataError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let ground_key = if ground {
        "ground = \"../reference/ground.toml\"\n"
    } else {
        ""
    };
    let text = format!(
        r#"
name = "Test"
placeholder = true
seed = 5
days = 4
map = "../map/placeholder_landing.toml"
ship_classes = ["../reference/ship_classes.csv", "../reference/ship_classes_placeholder_transports.csv"]
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"
{ground_key}
[[base]]
id = 1
name = "Port"
side = "allied"
q = 0
r = 0

[[task_force]]
id = 1
name = "Convoy"
side = "allied"
at_base = 1
mission = "transport"

[[task_force]]
id = 2
name = "Fast group"
side = "allied"
q = 10
r = 0
mission = "transport"

[[task_force]]
id = 3
name = "Japanese group"
side = "japanese"
q = 5
r = 0
mission = "transport"

[[ship]]
id = 1
name = "Troop ship 1"
class = 10
side = "allied"
fuel_t = 1000
task_force = 1

[[ship]]
id = 2
name = "Troop ship 2"
class = 10
side = "allied"
fuel_t = 1000
task_force = 1

[[ship]]
id = 3
name = "Fast ship"
class = 11
side = "allied"
fuel_t = 500
task_force = 2

[[ship]]
id = 4
name = "Japanese cargo ship"
class = 3
side = "japanese"
fuel_t = 1000
task_force = 3
{extra}"#
    );
    Scenario::parse(&text, &path)
}

fn landing_error(ground: bool, extra: &str) -> String {
    match landing(ground, extra) {
        Ok(_) => panic!("the scenario loads"),
        Err(error) => error.to_string(),
    }
}

fn unit_aboard(id: u32, men: u32, task_force: u32) -> String {
    format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = {men}\ntask_force = {task_force}\n"
        )
}

fn unit_ashore(id: u32, men: u32, q: i32, r: i32) -> String {
    format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = {men}\nq = {q}\nr = {r}\n"
        )
}

// -----------------------------------------------------------------
// What a side believed, and whose a class is (decision 0173).
// -----------------------------------------------------------------

/// Writes a file of sides with `rows` under the system's temporary
/// folder and returns its absolute path, which a scenario may name.
fn sides_file(rows: &str) -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!(
        "flagplot_class_sides_{}_{}.csv",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::write(&path, format!("table,id,side,note\n{rows}")).expect("write the file of sides");
    path
}

/// The rows of a whole file of sides for the placeholder classes and
/// types of `HEAD` (the record's statement 1): class 1 the Allied side's,
/// every other class and every type both sides'.
const PLACEHOLDER_SIDES: &str = "ship_class,1,allied,test\n\
        ship_class,2,both,test\n\
        ship_class,3,both,test\n\
        ship_class,4,both,test\n\
        ship_class,5,both,test\n\
        ship_class,6,both,test\n\
        ship_class,8,both,test\n\
        aircraft_type,1,both,test\n\
        aircraft_type,2,both,test\n\
        aircraft_type,3,both,test\n\
        aircraft_type,4,both,test\n";

/// `HEAD` naming the file of sides `rows`, plus `extra`.
fn parse_with_sides(rows: &str, extra: &str) -> Result<Scenario, DataError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let sides = sides_file(rows);
    let head = HEAD.replacen(
        "days = 2\n",
        &format!("days = 2\nclass_sides = \"{}\"\n", sides.display()),
        1,
    );
    let result = Scenario::parse(&format!("{head}{extra}"), &path);
    fs::remove_file(&sides).expect("remove the file of sides");
    result
}

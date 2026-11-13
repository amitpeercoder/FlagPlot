//! Scenario 16: every rule of step 2 acting together, in one night and one
//! day (`docs/step2-spec.md`, "Tests while figures move", 8).
//!
//! It is the complexity review of step 2. Each rule below has its own
//! scenario and its own hand calculation; this one checks that they give the
//! same numbers when they act in one day, in the order the day resolves them,
//! and that where two of them read or write the same thing the later one reads
//! what the earlier one wrote. The decisions are 0058 (ships charged
//! straight-line miles), 0059 (reach, the fuel roll, a lost deck, landing
//! elsewhere, ditching for want of a deck, the patrol pilot), 0063 (target
//! lists, the afternoon base strike), 0064 (experience), 0067 (the base count
//! at the day's end) and 0068 (a force not ready fires part of its guns).
//!
//! # The hand calculation (the architect's; nobody else changes a number)
//!
//! ## The figures
//!
//! The frozen step 1 figures (`data/reference/step1`) with these changed.
//! Each change either switches a rule on that the frozen value switches off,
//! or takes a chance to 0 or 1 so that the day is determined.
//!
//! | File | Figure | Frozen | Here | Why |
//! | --- | --- | --- | --- | --- |
//! | surface_combat | experience_gun_strength, _torpedo_, _sighting_ | 0 | 0.4 | 0064 on, as scenario 14a |
//! | surface_combat | unready_gun_share | 1 | 0.5 | 0068 on, as scenario 15a |
//! | surface_combat | system_hit_chance_per_point | 0.06 | 0.1 | a bomb of 10 points knocks a system with chance 1 (12c, case D) |
//! | air_combat | bomb_hit_chance | 0.2 | 1 | every bomb hits |
//! | air_combat | bomb_system_hit: flight_deck | 0.5 | 1 | every knock is on the flight deck |
//! | air_combat | bomb_system_hit: propulsion, fire_control, main_battery | 0.2, 0.1, 0.2 | 0 | the same |
//! | air_combat | crew_recovered_beside_own_force | 1 | 0.95 | a pilot can be lost |
//! | air_combat | crew_recovered_open_sea | 1 | 0.5 | not reached; set so that reaching it would show |
//! | air_combat | crew_recovered_over_own_base | 1 | 0.6 | the same |
//! | air_upkeep | wear_base_chance, wear_chance_per_100_nm | 0.05, 0.02 | 0 | the afternoon has every aircraft the morning had |
//! | search | position_error_chance, count_over_chance, count_under_chance | 0.3, 0.2, 0.2 | 0 | a report is the truth |
//! | search | every row of kind confusion | spread | the kind itself at 1 | a cruiser is reported as a cruiser |
//!
//! Kept at the frozen value and read below: extended_reach_share 0.25,
//! ditch_chance_per_radius_beyond 2, crew_recovered_short_of_home 0.7,
//! fighter_hit_chance 0.3, strike_contact_max_age_pulses 0, the base hit
//! shares 0.3 (aircraft) and 0.3 (supply), base_supply_destroyed_per_hit_t 50,
//! the night sighting chance 0.5, the factor 0.5 for a force not alert, the
//! night chances to hit 0.03 (guns) and 0.04 (torpedoes), the factor 1.5
//! against a surprised force and 2 against a lit one.
//!
//! ## Where everything is
//!
//! Air distance is 40 x sqrt(dq^2 + dr^2 + dq x dr), rounded to whole miles.
//!
//! | From | To | dq, dr | Sum | Miles |
//! | --- | --- | --- | --- | --- |
//! | Carrier A (2,0) | Carrier J at 06:00 (11,0) | 9, 0 | 81 | 360 |
//! | Carrier A (2,0) | Carrier J at noon (9,0) | 7, 0 | 49 | 280 |
//! | Carrier A (2,0) | The field (9,-6) | 7, -6 | 43 | 262 |
//! | Carrier A (2,0) | The night's fight (2,-4) | 0, -4 | 16 | 160 |
//! | Carrier J at 06:00 (11,0) | The field (9,-6) | -2, -6 | 52 | 288 |
//! | Carrier J at noon (9,0) | The field (9,-6) | 0, -6 | 36 | 240 |
//! | Carrier J at noon (9,0) | The night's fight (2,-4) | -7, -4 | 93 | 386 |
//!
//! Radius of the dive bombers and fighters: 250. Extended reach:
//! 250 x 1.25 = 312.5, cut to 312. The scouts have no strike radius and
//! search to 300.
//!
//! ## 1. Steaming (0058)
//!
//! Carrier J, (14,3) to (9,0) at 16 knots: 280 miles in a straight line, eight
//! steps off the grid's axes. The first three steps are charged the miles
//! each brings her nearer, 30, 27 and 23; the other five are along an axis,
//! 40 each: 30 + 27 + 23 + 5 x 40 = 280.
//!
//! - Night, 12 hours: 192 miles. Five steps done (160) and 32 into the sixth.
//!   The breakdown's steps are 30, 27, 23, 40, 40, 40. She is in (11,0) at
//!   06:00 and burns 64 t.
//! - Morning: the 88 miles left, in 5.5 hours. She arrives at (9,0) at 11:30.
//!
//! The Japanese cruisers, (7,-4) west along the row at 16 knots, are charged
//! 40 a step. They enter (2,-4) after 180 miles, at 05:15, and the fight
//! begins there.
//!
//! ## 2. The night fight (0064 and 0068 together)
//!
//! Force 3 (Allied): three cruisers of experience 1, stopped, normal
//! readiness, its side holding no contact. So it is not alert, and not ready.
//! Force 4 (Japanese): three cruisers of experience 3, at battle readiness.
//! So it is alert, and ready.
//!
//! Experience factor, 1 + 0.4 x (experience - 2): 0.6 for the Allied ships,
//! 1.4 for the Japanese.
//!
//! **Sighting, round 1.** Base x alert x experience, capped at 1:
//!
//! - Force 3 looks: 0.5 x 0.5 x 0.6 = 0.15.
//! - Force 4 looks: 0.5 x 1 x 1.4 = 0.7.
//!
//! **Gun shots.** A full salvo is 8 guns x 3 a minute x 5 minutes = 120.
//!
//! - A Japanese ship, ready: 120 x her main battery factor. No
//!   `unready_gun_share` in her breakdown.
//! - An Allied ship, not ready: 120 x her main battery factor x 0.5, so 60
//!   with her battery working and 30 with it degraded, in every round of the
//!   fight. Force 3 becomes alert in round 1 and still fires the share to the
//!   end: readiness is fixed when the fight begins. Her breakdown names
//!   `unready_gun_share`, after the main battery factor.
//!
//! **Chance to hit.** Base x surprised x lit x fire control x experience, the
//! experience factor last. With fire control working:
//!
//! | Firer | Target as it is | Guns | Torpedoes |
//! | --- | --- | --- | --- |
//! | Japanese | neither | 0.03 x 1.4 = 0.042 | 0.04 x 1.4 = 0.056 |
//! | Japanese | surprised | 0.03 x 1.5 x 1.4 = 0.063 | 0.04 x 1.5 x 1.4 = 0.084 |
//! | Japanese | lit | 0.03 x 2 x 1.4 = 0.084 | |
//! | Allied | neither | 0.03 x 0.6 = 0.018 | 0.04 x 0.6 = 0.024 |
//! | Allied | surprised | 0.03 x 1.5 x 0.6 = 0.027 | 0.04 x 1.5 x 0.6 = 0.036 |
//! | Allied | lit | 0.03 x 2 x 0.6 = 0.036 | |
//!
//! A firer whose fire control is degraded has half the chance of her row
//! (seed 1942, round 2: an Allied ship at 60 shots x 0.009), and one whose
//! fire control is out has a quarter of it.
//!
//! The two rules never touch the same number. The share changes the shots and
//! experience changes the chance; they meet only in the expected hits. An
//! Allied ship with everything working, against a force neither surprised nor
//! lit: 60 x 0.018 = 1.08 expected, where a Japanese ship has
//! 120 x 0.042 = 5.04. Torpedoes read experience and never the share: 8 shots.
//!
//! **After the fight** each side reports the other: cruisers, at (2,-4), the
//! true count of ships afloat. That is contact 1 for each side.
//!
//! ## 3. The morning: the hold (0063 on a contact from 0068's fight)
//!
//! - Searches. Carrier J is 360 miles from carrier A, beyond the 300 of both
//!   sides' scouts. Nothing is sighted.
//! - The Allied dive bomber (group 2, one aircraft). The one contact its side
//!   holds is the cruisers at (2,-4): age 0, 160 miles, within reach. Its
//!   target list is "carrier" alone. It does not launch. With the target list
//!   order taken out of the file it launches one aircraft at contact 1, at
//!   160 miles: the list is what holds it.
//! - The Japanese dive bombers (group 5). Their side's contact 1 is 461 miles
//!   from (11,0): beyond 312. They do not launch.
//! - The Japanese attack unit (group 6), ordered to strike the field on
//!   normal reach. The field is 288 miles from (11,0): beyond 250. It does not
//!   launch.
//!
//! No strike is launched by either side in the morning.
//!
//! ## 4. The afternoon
//!
//! Positions are read as the pulse begins: carrier J is at (9,0).
//!
//! **Search.** Five scouts on an arc 20 degrees wide, the target at 280:
//! 57.295779 x 5 x 20 / (280 x 20) = 5729.5779 / 5600 = 1.0231, capped at 1.
//! Each side sights the other's carrier: a carrier, one ship, at its true
//! hex. Contact 2 for each side.
//!
//! **Launches** (0059's reach read by 0063's choice, in one function):
//!
//! | Group | Launches | Why |
//! | --- | --- | --- |
//! | 2, Allied dive bomber | 1 at contact 2, 280 miles | A carrier, on its list; 250 < 280 <= 312, and it is on extended reach. On normal reach it does not launch |
//! | 5, Japanese dive bombers | 6 at contact 2, 280 miles | The same reach |
//! | 6, Japanese attack unit | 4 at the field, 240 miles | 240 <= 250. Its one base strike of the day, in the afternoon (0063, decision 7) |
//! | 7, Japanese fighters | 12 escorts with group 5 | Patrol share 0; group 5's raid has more bombers than group 6's (6 against 4) and is within the fighters' reach of 312 |
//!
//! Carrier A's deck is working when the launches are decided, so her strike
//! goes though her deck is lost later in the pulse.
//!
//! **Over carrier A.** Her patrol is 12 fighters and the escorts are 12, so
//! every patrol shot is at an escort and none at a bomber: 12 shots x 0.3 =
//! 3.6 expected each way. There are no anti-aircraft guns. A patrol fighter
//! shot down over its own force loses its pilot with chance 1 - 0.95 = 0.05
//! (0059, the patrol pilot). An escort shot down loses its pilot.
//!
//! **Bombs**, at chance 1:
//!
//! - Group 5 on carrier A: 6 shots, 6 hits, 10 points each, flotation 60 (she
//!   sinks at 100, so she cannot sink). Each hit knocks a system with chance
//!   10 x 0.1 = 1, and the system is the flight deck: working to degraded,
//!   degraded to out, then four hits with no change. **The deck is lost.**
//! - Group 2 on carrier J: 1 shot, 1 hit, flotation 10, flight deck working to
//!   degraded. Her deck is not lost, and her aircraft come home to it.
//! - Group 6 on the field: 4 shots, 4 hits. Each is on aircraft with share
//!   0.3, on supply with 0.3 and on nothing with 0.4. No aircraft are on the
//!   ground: the landing comes later in the pulse, so the raid never meets the
//!   aircraft that land. A hit on supply destroys 50 t. With S hits on supply
//!   the stock is 500 - 50 x S.
//!
//! **The fuel roll** (0059), for a sortie beyond the radius: 280 is 30 beyond
//! 250, a share of 30 / 250 = 0.12, and the chance is 0.12 x 2 = 0.24 an
//! aircraft. A crew is recovered with chance 0.7.
//!
//! - Group 2: 1 aircraft x 0.24 = 0.24 expected.
//! - Group 5: 6 x 0.24 = 1.44 expected.
//! - Group 7: the escorts not shot down, each at 0.24.
//! - Group 6: no roll. 240 is not beyond 250.
//!
//! **Coming home to no deck** (0059), after the fuel roll and before wear.
//! Carrier A has no other carrier in her hex.
//!
//! - Group 1, the 5 scouts, which searched this pulse: the field is 262 miles
//!   off, within their 300. All 5 land there and form group 8, with 5 pilots.
//!   Group 1 is left with 0 aircraft and 0 pilots.
//! - Group 2, its aircraft if the fuel roll spared it: the field at 262 is
//!   beyond 250. It comes down in the sea at (2,0), beside ships of its own
//!   side: the crew is recovered with chance 0.95.
//! - Group 3, the patrol fighters not shot down: on patrol they are in the
//!   air (0059, decision 7). The field is beyond 250. They come down in the
//!   sea beside their own ships, each crew recovered with chance 0.95.
//!
//! The Japanese groups land aboard carrier J, with no event.
//!
//! **Supply.** The sortie of the scouts went with them to group 8, so the
//! field pays for it: 5 aircraft x 2 t = 10 t. A carrier would have paid
//! nothing.
//!
//! ## 5. The day's end (0067 reading what 0059 and 0063 left)
//!
//! The field: 500 - 10 - 50 x S tonnes, which is above zero for any S up to
//! 4 (290 at the least), and 5 aircraft ready, the landed scouts. So it is
//! **open**, in every run. Without the landing it would hold no aircraft and
//! be closed.
//!
//! The result. The Allied side has the field open for 1 of 1 days: 5 points.
//! The Japanese side has carrier A crippled, her flight deck out: 10 points,
//! and 0 for the field closed. Aircraft destroyed are 0.5 each: the aircraft
//! shot down and those ditched for fuel or for want of a deck. The 5 scouts
//! that landed are not destroyed.
//!
//! ## 6. The order within the afternoon
//!
//! By event number: the searches and reports; the launches and the escorts;
//! the fighters' shots and the losses, with the patrol pilot's draw; the
//! bombs; the damage, with the deck going out; the fuel roll; the homecoming
//! to no deck; wear; the supply paid; and, last of the day, the field's
//! count. Each reads what the one before wrote: the fuel roll takes its
//! ditched aircraft off before the homecoming counts who is left, and the
//! homecoming moves the sortie before supply is paid.
//!
//! ## 7. Seed 1942, pinned
//!
//! Not worked by hand: what the draws gave on the scenario's own seed, pinned
//! so that a change shows. It is remade only when a rule changes.
//!
//! - Night: both forces sight in round 1; force 3 breaks off in round 2; no
//!   ship is sunk.
//! - Over carrier A: 5 patrol fighters shot down, no pilot lost; 2 escorts
//!   shot down.
//! - Fuel: group 2, none ditched. Group 5, 3 ditched, 2 crews recovered, 1
//!   pilot lost, 3 ready left. Group 7, 10 rolled, 6 ditched, 5 crews
//!   recovered, 1 pilot lost, 4 ready left.
//! - No deck: group 1 lands 5 at the field. Group 2, 1 into the sea, the crew
//!   recovered. Group 3, 7 into the sea, 6 crews recovered, 1 pilot lost.
//! - The raid on the field: 1 hit on aircraft, 0 on supply, 3 on nothing.
//! - The field at the day's end: 490 t, 5 aircraft ready, open.
//! - Aircraft destroyed: 11 Japanese (2 + 3 + 6) and 13 Allied (5 + 1 + 7).
//! - Result: Allied 5.5 + 5 = 10.5, Japanese 6.5 + 10 = 16.5, margin 6, a
//!   minor Japanese victory. 166 events.
//!
//! ## 8. Properties, over seeds 1 to 1,000
//!
//! In every run, on the figures above:
//!
//! - The night fight takes place. Sighting chances in round 1 are 0.15 and
//!   0.7. Every gun salvo of a force 3 ship in the night's fight names
//!   `unready_gun_share` 0.5 and has 60 or 30 shots; no salvo of a force 4
//!   ship names it; every torpedo salvo has no share. Every salvo's chance is
//!   a row of the table in 2, or half or a quarter of one.
//! - No strike is launched in the morning.
//! - In the afternoon the four launches are as in 4, carrier A's flight deck
//!   goes out, carrier J's is degraded and not out, and no carrier sinks.
//! - No patrol shot is at a bomber.
//! - The 5 scouts land at the field; the field pays 10 t; at the day's end
//!   it holds 500 - 10 - 50 x S tonnes and 5 ready aircraft, and is open.
//! - Each group's aircraft are all accounted for: those sent are shot down,
//!   ditched for fuel, landed, in the sea, or home aboard. No group has fewer
//!   than 0 pilots or loses more pilots than aircraft.
//! - No event comes before its cause, and a replay gives the same hash.
//! - The enemy's view of a ditching or a landing elsewhere is cut as
//!   `side_events.rs` has it.
//!
//! Across the runs, each within four standard deviations of its mean:
//!
//! - Patrol fighters shot down, and escorts shot down: 12 at 0.3 each, mean
//!   3.6 a run.
//! - Aircraft ditched for fuel: 0.24 of those rolled. Crews recovered: 0.7 of
//!   those ditched.
//! - Crews recovered beside their own force: 0.95 of the aircraft in the sea.
//! - Hits on the field's supply: 0.3 of 4, mean 1.2 a run.
//!
//! A surface fight by day happens only in a run where a force lies stopped in
//! the fight's hex with a ship's propulsion out (the stragglers rule). The
//! test counts those runs and asserts nothing more of them.
//!
//! ## 9. Two days: the raid meets the aircraft that landed
//!
//! In one day the raid on the field comes before the landing. The same file
//! with `days = 2` brings them together, on seeds 1 to 200:
//!
//! - Group 6 launches at the field once on day 2, and in the morning: carrier
//!   J is still at (9,0), 240 miles off, from the day's first pulse.
//! - Group 8's 5 scouts are on the ground when the raid arrives, and the
//!   raid's hits on aircraft fall on them.
//! - The field's count at the end of day 2 is the ready aircraft group 8 has
//!   left in the state, and the tonnes the state holds; it is open only if
//!   both are above zero.
//! - No aircraft takes off from carrier A on day 2: her deck is out.
//!
//! ## 10. On the frozen figures, nothing altered
//!
//! No breakdown names `experience_factor` or `unready_gun_share`, and every
//! Allied gun salvo with a working battery has 120 shots.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use sim_core::events::{
    AirFirer, AirTarget, Cause, Event, EventKind, SortieKind, SupplyUse, Weapon,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, AircraftTypeId, BaseId, ShipId, TaskForceId};
use sim_core::objectives::tallies;
use sim_core::orders::Reach;
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroupLocation, ShipLocation, ShipSystem, Side, SystemState};
use sim_core::time::Pulse;
use sim_core::{SimConfig, World};
use sim_data::{LoadOptions, Scenario};

mod common;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SCENARIO: &str = "16_all_together.toml";

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn group(id: u32) -> AirGroupId {
    AirGroupId::new(id)
}

fn ship(id: u32) -> ShipId {
    ShipId::new(id)
}

const FIELD: BaseId = BaseId::new(1);
const CARRIER_A: u32 = 1;
const CARRIER_J: u32 = 2;
const AT_A: Hex = Hex::new(2, 0);
const FIGHT: Hex = Hex::new(2, -4);
const FIELD_HEX: Hex = Hex::new(9, -6);

// ---------------------------------------------------------------------
// The figures: the frozen ones altered as the table of the hand calculation
// says. Everything below goes through `Setup`.
// ---------------------------------------------------------------------

/// The whole of a variant: the altered reference and the scenario's text.
struct Setup {
    base: Scenario,
}

fn working_options(tag: &str) -> LoadOptions {
    common::altered_reference(tag, |dir| {
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[
                (
                    "experience_gun_strength = \"0\"",
                    "experience_gun_strength = \"0.4\"",
                ),
                (
                    "experience_torpedo_strength = \"0\"",
                    "experience_torpedo_strength = \"0.4\"",
                ),
                (
                    "experience_sighting_strength = \"0\"",
                    "experience_sighting_strength = \"0.4\"",
                ),
                ("unready_gun_share = \"1\"", "unready_gun_share = \"0.5\""),
                (
                    "system_hit_chance_per_point = \"0.06\"",
                    "system_hit_chance_per_point = \"0.1\"",
                ),
            ],
        );
        common::edit_file_by(
            dir,
            "air_combat.toml",
            &[
                ("bomb_hit_chance = \"0.2\"", "bomb_hit_chance = \"1\""),
                ("flight_deck = \"0.5\"", "flight_deck = \"1\""),
                ("propulsion = \"0.2\"", "propulsion = \"0\""),
                ("fire_control = \"0.1\"", "fire_control = \"0\""),
                ("main_battery = \"0.2\"", "main_battery = \"0\""),
                (
                    "crew_recovered_beside_own_force = \"1\"",
                    "crew_recovered_beside_own_force = \"0.95\"",
                ),
                (
                    "crew_recovered_open_sea = \"1\"",
                    "crew_recovered_open_sea = \"0.5\"",
                ),
                (
                    "crew_recovered_over_own_base = \"1\"",
                    "crew_recovered_over_own_base = \"0.6\"",
                ),
            ],
        );
        common::edit_file_by(
            dir,
            "air_upkeep.toml",
            &[
                ("wear_base_chance = \"0.05\"", "wear_base_chance = \"0\""),
                (
                    "wear_chance_per_100_nm = \"0.02\"",
                    "wear_chance_per_100_nm = \"0\"",
                ),
            ],
        );
        common::edit_file(dir, "search.toml", |text| {
            let at = text
                .find("[kind_confusion.carrier]")
                .expect("the confusion tables");
            let mut head = text[..at].to_string();
            for (from, to) in [
                (
                    "position_error_chance = \"0.3\"",
                    "position_error_chance = \"0\"",
                ),
                ("count_over_chance = \"0.2\"", "count_over_chance = \"0\""),
                ("count_under_chance = \"0.2\"", "count_under_chance = \"0\""),
            ] {
                head = common::replace_once(&head, from, to);
            }
            for kind in [
                "carrier",
                "battleship",
                "cruiser",
                "destroyer",
                "transport",
                "auxiliary",
            ] {
                head.push_str(&format!("[kind_confusion.{kind}]\n{kind} = \"1\"\n\n"));
            }
            head
        });
    })
}

fn scenario_path() -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(SCENARIO)
}

fn scenario_text() -> String {
    fs::read_to_string(scenario_path()).expect("the scenario reads")
}

/// The order block of the file that begins with `start`, cut out.
fn without_block(text: &str, start: &str) -> String {
    let at = text
        .find(start)
        .unwrap_or_else(|| panic!("{start:?} is in the text"));
    let rest = &text[at + start.len()..];
    let end = rest.find("\n[[").map_or(rest.len(), |i| i + 1);
    format!("{}{}", &text[..at], &rest[end..])
}

const TARGET_ORDER: &str = "[[order]]\nday = 1\nside = \"allied\"\nair_group = 2\nkind = \"set_target_priority\"\nkinds = [\"carrier\"]\n";
const REACH_ORDER_G2: &str = "[[order]]\nday = 1\nside = \"allied\"\nair_group = 2\nkind = \"set_reach\"\nreach = \"extended\"\n";

impl Setup {
    fn working(tag: &str) -> Setup {
        Setup::with_text(tag, scenario_text())
    }

    fn with_text(tag: &str, text: String) -> Setup {
        let options = working_options(tag);
        let base = common::from_text_with(&text, &scenario_path(), &options)
            .expect("the scenario loads on the working figures");
        Setup { base }
    }

    fn run(&self, seed: u64) -> World {
        let mut scenario = self.base.with_seed(seed).expect("a seed can be set");
        scenario.run_to_end().expect("the run resolves");
        scenario.world
    }
}

/// Runs `check` on seeds `first..=last`, on as many threads as the machine
/// has, and returns the results in seed order.
fn over_seeds<T: Send>(
    setup: &Setup,
    first: u64,
    last: u64,
    check: impl Fn(u64, &World) -> T + Sync,
) -> Vec<T> {
    let seeds: Vec<u64> = (first..=last).collect();
    let threads = std::thread::available_parallelism().map_or(4, |p| p.get().min(8));
    let chunk = seeds.len().div_ceil(threads);
    let check = &check;
    std::thread::scope(|scope| {
        let handles: Vec<_> = seeds
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .map(|&seed| check(seed, &setup.run(seed)))
                        .collect::<Vec<T>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("a run's checks hold"))
            .collect()
    })
}

// ---------------------------------------------------------------------
// Reading the log.
// ---------------------------------------------------------------------

fn events(world: &World) -> Vec<&Event> {
    world.events().iter().collect()
}

fn in_pulse(world: &World, pulse: Pulse) -> Vec<&Event> {
    world
        .events()
        .iter()
        .filter(|e| e.pulse == Some(pulse))
        .collect()
}

fn factor(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .unwrap_or_else(|| {
            panic!(
                "event {} has a breakdown",
                event.id.umpire_raw(common::umpire_key())
            )
        })
        .factors
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| {
            panic!(
                "event {} names {name}",
                event.id.umpire_raw(common::umpire_key())
            )
        })
        .value
}

fn names_factor(event: &Event, name: &str) -> bool {
    event
        .breakdown
        .as_ref()
        .is_some_and(|b| b.factors.iter().any(|f| f.name == name))
}

fn steps(event: &Event) -> Vec<i64> {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .filter(|f| f.name == "step_charge_nm")
        .map(|f| f.value.trunc())
        .collect()
}

fn the_one<'a>(list: &[&'a Event]) -> &'a Event {
    assert_eq!(list.len(), 1, "exactly one event");
    list[0]
}

/// Whether `a` is among the causes of `b`.
fn cites(b: &Event, a: &Event) -> bool {
    b.causes.contains(&Cause::Event(a.id))
}

fn truth_group(world: &World, id: u32) -> (u32, Option<u32>) {
    let g = &world.umpire().air_groups[&group(id)];
    (g.ready, g.pilots)
}

// ---------------------------------------------------------------------
// What a run says, gathered once, with the determined parts asserted as they
// are read.
// ---------------------------------------------------------------------

#[derive(Default, Debug)]
struct Run {
    /// Patrol fighters shot down over carrier A (H1) and escorts shot down (H2).
    patrol_shot_down: u32,
    escorts_shot_down: u32,
    /// Fuel: aircraft rolled, ditched, crews recovered, summed over groups.
    rolled: u32,
    ditched: u32,
    recovered: u32,
    /// Aircraft down in the sea for want of a deck, and their crews recovered.
    sea: u32,
    sea_recovered: u32,
    /// Patrol aircraft lost with the pilot draw, and its crews recovered.
    patrol_drawn: u32,
    patrol_recovered: u32,
    /// Hits on the field's supply.
    supply_hits: u32,
    /// A fight in the morning or afternoon.
    day_fight: bool,
    /// The night's fight passed unseen: no report.
    unseen: bool,
}

fn ships_sunk_at_night(world: &World, tf: u32) -> u32 {
    in_pulse(world, Pulse::Night)
        .iter()
        .filter(|e| match e.kind {
            EventKind::ShipSunk { task_force, .. } => task_force.raw() == tf,
            _ => false,
        })
        .count() as u32
}

/// The night: the fight, its chances and its reports (section 2).
fn check_night(world: &World, run: &mut Run) {
    let night = in_pulse(world, Pulse::Night);
    let begun: Vec<&Event> = night
        .iter()
        .copied()
        .filter(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
        .collect();
    let begun = the_one(&begun);
    match begun.kind {
        EventKind::EngagementBegun {
            a,
            b,
            hex,
            elapsed_hours,
        } => {
            assert_eq!((a.raw(), b.raw()), (3, 4));
            assert_eq!(hex, FIGHT);
            // 180 miles at 16 knots: 11.25 hours, 05:15, inside the night.
            assert_eq!(elapsed_hours, fx("11.25"));
        }
        _ => unreachable!(),
    }

    let mut any_sighted = false;
    for event in &night {
        match event.kind {
            EventKind::SightingRoll {
                round,
                task_force,
                chance,
                sighted,
                ..
            } => {
                any_sighted |= sighted;
                let allied = task_force.raw() == 3;
                // The experience factor of the force's ships, in every roll.
                let experience = if allied { fx("0.6") } else { fx("1.4") };
                assert_eq!(factor(event, "experience_factor"), experience);
                assert_eq!(factor(event, "base"), fx("0.5"));
                assert_eq!(factor(event, "radar_bonus"), Fixed::ZERO);
                if round == 1 {
                    // Force 3 is not alert (0.5), force 4 is (1).
                    let alert = if allied { fx("0.5") } else { Fixed::ONE };
                    assert_eq!(factor(event, "alert_factor"), alert);
                    let expected = if allied { fx("0.15") } else { fx("0.7") };
                    assert_eq!(chance, expected);
                    assert_eq!(
                        event.breakdown.as_ref().expect("a breakdown").result,
                        expected
                    );
                }
            }
            EventKind::Salvo {
                firer,
                weapon,
                shots,
                chance,
                expected,
                ..
            } => {
                any_sighted = true;
                let allied = (3..=5).contains(&firer.raw());
                let experience = if allied { fx("0.6") } else { fx("1.4") };
                assert_eq!(factor(event, "experience_factor"), experience);
                let surprised = factor(event, "surprised_factor");
                let lit = factor(event, "lit_factor");
                assert!(surprised == Fixed::ONE || surprised == fx("1.5"));
                assert!(lit == Fixed::ONE || lit == n(2));
                let base = factor(event, "base_chance");
                match weapon {
                    Weapon::Gun => {
                        assert_eq!(base, fx("0.03"));
                        assert_eq!(factor(event, "guns"), n(8));
                        assert_eq!(factor(event, "rate"), n(3));
                        assert_eq!(factor(event, "minutes"), n(5));
                        let battery = factor(event, "main_battery_factor");
                        assert!(battery == Fixed::ONE || battery == fx("0.5"));
                        // 8 guns x 3 a minute x 5 minutes = 120, times the
                        // battery factor, times the share if not ready.
                        let full = n(120) * battery;
                        if allied {
                            assert_eq!(factor(event, "unready_gun_share"), fx("0.5"));
                            assert_eq!(shots, full * fx("0.5"));
                            assert!(shots == n(60) || shots == n(30));
                        } else {
                            assert!(!names_factor(event, "unready_gun_share"));
                            assert_eq!(shots, full);
                            assert!(shots == n(120) || shots == n(60));
                        }
                        let fire_control = factor(event, "fire_control_factor");
                        assert!(
                            [Fixed::ONE, fx("0.5"), fx("0.25")].contains(&fire_control),
                            "fire control factor {fire_control:?}"
                        );
                        // Base x surprised x lit x fire control x experience,
                        // in that order.
                        assert_eq!(chance, base * surprised * lit * fire_control * experience);
                    }
                    Weapon::Torpedo => {
                        assert_eq!(base, fx("0.04"));
                        assert!(!names_factor(event, "unready_gun_share"));
                        let mounts = factor(event, "torpedo_mount_factor");
                        assert_eq!(factor(event, "tubes"), n(8));
                        assert_eq!(shots, n(8) * mounts);
                        assert_eq!(chance, base * surprised * lit * experience);
                    }
                    Weapon::Bomb => panic!("no bomb in a surface fight"),
                }
                assert_eq!(expected, shots * chance);
                assert_eq!(
                    event.breakdown.as_ref().expect("a breakdown").result,
                    expected
                );
            }
            _ => {}
        }
    }

    // After the fight: a report each way, cruisers at (2,-4), the true count
    // afloat, contact 1.
    let reports: Vec<&Event> = night
        .iter()
        .copied()
        .filter(|e| matches!(e.kind, EventKind::ReportReceived { .. }))
        .collect();
    if reports.is_empty() {
        assert!(!any_sighted, "a fight that sighted reports");
        run.unseen = true;
    } else {
        assert_eq!(reports.len(), 2);
        for report in reports {
            match report.kind {
                EventKind::ReportReceived {
                    side,
                    contact,
                    hex,
                    ship_count,
                    kind,
                    created,
                } => {
                    assert_eq!(contact.umpire_raw(common::umpire_key()), 1);
                    assert_eq!(hex, FIGHT);
                    assert!(created);
                    assert_eq!(kind, sim_core::refdata::ShipKind::Cruiser);
                    // Each side reports the other force's ships afloat.
                    let enemy = if side == Side::Allied { 4 } else { 3 };
                    let afloat = 3 - ships_sunk_at_night(world, enemy);
                    assert_eq!(ship_count, if afloat == 0 { 3 } else { afloat });
                }
                _ => unreachable!(),
            }
        }
    }
}

/// The morning: nothing is launched and nothing sighted (section 3).
fn check_morning(world: &World, run: &mut Run) {
    for event in in_pulse(world, Pulse::Morning) {
        assert!(
            !matches!(
                event.kind,
                EventKind::StrikeLaunched { .. }
                    | EventKind::BaseStrikeLaunched { .. }
                    | EventKind::EscortsSent { .. }
                    | EventKind::AirAttack { .. }
                    | EventKind::BaseAttacked { .. }
                    | EventKind::AirFight { .. }
                    | EventKind::SearchAttempt { .. }
            ),
            "the morning has no strike and no sighting: event {}",
            event.id.umpire_raw(common::umpire_key())
        );
        if let EventKind::ReportReceived {
            contact, created, ..
        } = event.kind
        {
            // Only a fight can report in the morning, and it refreshes the
            // night's contact.
            assert!(!created && contact.umpire_raw(common::umpire_key()) == 1);
        }
        if matches!(event.kind, EventKind::EngagementBegun { .. }) {
            run.day_fight = true;
        }
    }
}

fn group_events<'a>(list: &[&'a Event], keep: impl Fn(&EventKind) -> bool) -> Vec<&'a Event> {
    list.iter().copied().filter(|e| keep(&e.kind)).collect()
}

fn of_group(kind: &EventKind, id: u32) -> bool {
    match kind {
        EventKind::AircraftDitched { air_group, .. }
        | EventKind::AircraftDitchedNoDeck { air_group, .. }
        | EventKind::AircraftLost { air_group, .. }
        | EventKind::StrikeLaunched { air_group, .. }
        | EventKind::BaseStrikeLaunched { air_group, .. }
        | EventKind::AirAttack { air_group, .. }
        | EventKind::BaseAttacked { air_group, .. } => air_group.raw() == id,
        EventKind::AircraftLandedElsewhere { from, .. } => from.raw() == id,
        _ => false,
    }
}

/// The afternoon (sections 4 and 5). Returns nothing; fills `run`.
fn check_afternoon(world: &World, run: &mut Run) {
    let all = events(world);
    let afternoon = in_pulse(world, Pulse::Afternoon);
    let truth = world.umpire();

    // Search: five scouts, an arc of 20 degrees, 280 miles: chance 1.
    let attempts = group_events(&afternoon, |k| matches!(k, EventKind::SearchAttempt { .. }));
    assert_eq!(attempts.len(), 2);
    for (event, (air_group, tf)) in attempts.iter().zip([(1, 2), (4, 1)]) {
        match event.kind {
            EventKind::SearchAttempt {
                air_group: g,
                task_force,
                distance_nm,
                chance,
                sighted,
            } => {
                assert_eq!((g.raw(), task_force.raw()), (air_group, tf));
                assert_eq!(distance_nm, n(280));
                assert_eq!(chance, Fixed::ONE);
                assert!(sighted);
                assert_eq!(factor(event, "aircraft"), n(5));
                assert_eq!(factor(event, "sweep_width_nm"), n(20));
                assert_eq!(factor(event, "arc_deg"), n(20));
                assert_eq!(factor(event, "distance_nm"), n(280));
                assert_eq!(factor(event, "degrees_per_radian"), fx("57.295779"));
                // 57.295779 x 5 x 20 / (280 x 20) = 1.0231 before the cap.
                assert_eq!(
                    event.breakdown.as_ref().expect("a breakdown").result,
                    Fixed::ONE
                );
            }
            _ => unreachable!(),
        }
    }
    // The reports: a carrier, one ship, at its true hex; contact 2.
    let reports = group_events(&afternoon, |k| {
        matches!(k, EventKind::ReportReceived { .. })
    });
    let created: Vec<&Event> = reports
        .iter()
        .copied()
        .filter(|e| matches!(e.kind, EventKind::ReportReceived { created: true, .. }))
        .collect();
    assert_eq!(created.len(), 2);
    for event in created {
        match event.kind {
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                ..
            } => {
                assert_eq!(contact.umpire_raw(common::umpire_key()), 2);
                assert_eq!(ship_count, 1);
                assert_eq!(kind, sim_core::refdata::ShipKind::Carrier);
                assert_eq!(
                    hex,
                    if side == Side::Allied {
                        Hex::new(9, 0)
                    } else {
                        AT_A
                    }
                );
            }
            _ => unreachable!(),
        }
    }

    // The launches.
    let launched = |id: u32| -> &Event {
        let found = group_events(
            &afternoon,
            |k| matches!(k, EventKind::StrikeLaunched { air_group, .. } | EventKind::BaseStrikeLaunched { air_group, .. } if air_group.raw() == id),
        );
        the_one(&found)
    };
    let l2 = launched(2);
    match l2.kind {
        EventKind::StrikeLaunched {
            contact,
            hex,
            aircraft,
            distance_nm,
            ..
        } => {
            assert_eq!(contact.umpire_raw(common::umpire_key()), 2);
            assert_eq!(hex, Hex::new(9, 0));
            assert_eq!(aircraft, 1);
            assert_eq!(distance_nm, n(280));
        }
        _ => panic!("group 2 launches at a contact"),
    }
    let l5 = launched(5);
    match l5.kind {
        EventKind::StrikeLaunched {
            contact,
            hex,
            aircraft,
            distance_nm,
            ..
        } => {
            assert_eq!(contact.umpire_raw(common::umpire_key()), 2);
            assert_eq!(hex, AT_A);
            assert_eq!(aircraft, 6);
            assert_eq!(distance_nm, n(280));
        }
        _ => panic!("group 5 launches at a contact"),
    }
    let l6 = launched(6);
    match l6.kind {
        EventKind::BaseStrikeLaunched {
            base,
            hex,
            aircraft,
            distance_nm,
            ..
        } => {
            assert_eq!(base, FIELD);
            assert_eq!(hex, FIELD_HEX);
            assert_eq!(aircraft, 4);
            assert_eq!(distance_nm, n(240));
        }
        _ => panic!("group 6 launches at the base"),
    }
    // Causes: group 2 cites its three orders (mission, reach, target list) and
    // the report; group 5 two orders and the report; group 6 its one order.
    let orders = |e: &Event| {
        e.causes
            .iter()
            .filter(|c| matches!(c, Cause::Order(_)))
            .count()
    };
    let reports_cited = |e: &Event| {
        e.causes
            .iter()
            .filter(|c| matches!(c, Cause::Event(_)))
            .count()
    };
    assert_eq!((orders(l2), reports_cited(l2)), (3, 1));
    assert_eq!((orders(l5), reports_cited(l5)), (2, 1));
    assert_eq!((orders(l6), reports_cited(l6)), (1, 0));
    let sent = the_one(&group_events(&afternoon, |k| {
        matches!(k, EventKind::EscortsSent { .. })
    }));
    match sent.kind {
        EventKind::EscortsSent {
            air_group,
            with_air_group,
            aircraft,
        } => {
            assert_eq!(
                (air_group.raw(), with_air_group.raw(), aircraft),
                (7, 5, 12)
            );
        }
        _ => unreachable!(),
    }

    // Over carrier A: 12 shots each way at 0.3, none at a bomber.
    let fights = group_events(&afternoon, |k| matches!(k, EventKind::AirFight { .. }));
    assert_eq!(
        fights.len(),
        2,
        "escorts at the patrol, patrol at the escorts"
    );
    let mut patrol_hits = 0;
    let mut escort_hits = 0;
    for event in &fights {
        match event.kind {
            EventKind::AirFight {
                firer,
                target,
                firing,
                at,
                shots,
                chance,
                expected,
                hits,
                ..
            } => {
                assert_ne!(at, AirTarget::Bombers, "no patrol shot at a bomber");
                assert_eq!(shots, n(12));
                assert_eq!(chance, fx("0.3"));
                assert_eq!(expected, fx("3.6"));
                assert!(hits <= 12);
                match firing {
                    AirFirer::Escorts => {
                        assert_eq!((firer.raw(), target.raw(), at), (7, 3, AirTarget::Patrol));
                        patrol_hits = hits;
                    }
                    AirFirer::Patrol => {
                        assert_eq!((firer.raw(), target.raw(), at), (3, 7, AirTarget::Escorts));
                        escort_hits = hits;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    run.patrol_shot_down = patrol_hits;
    run.escorts_shot_down = escort_hits;

    // Losses and the patrol pilot's draw.
    let lost = |id: u32| -> Vec<&Event> {
        group_events(
            &afternoon,
            |k| matches!(k, EventKind::AircraftLost { air_group, .. } if air_group.raw() == id),
        )
    };
    let patrol_lost = lost(3);
    let mut patrol_recovered_here = 0u32;
    if patrol_hits == 0 {
        assert!(patrol_lost.is_empty());
    } else {
        let event = the_one(&patrol_lost);
        match event.kind {
            EventKind::AircraftLost {
                aircraft,
                remaining,
                ..
            } => {
                assert_eq!(aircraft, patrol_hits);
                assert_eq!(remaining, 12 - patrol_hits);
                // The draw: over a ship, at beside-own-force 0.95, and the
                // breakdown says how many crews were recovered.
                assert_eq!(factor(event, "aircraft_lost"), n(i64::from(aircraft)));
                assert_eq!(factor(event, "crew_recovered_chance"), fx("0.95"));
                let recovered = event.breakdown.as_ref().expect("a breakdown").result;
                assert!(recovered >= Fixed::ZERO && recovered <= n(i64::from(aircraft)));
                run.patrol_drawn += aircraft;
                run.patrol_recovered += recovered.trunc() as u32;
                patrol_recovered_here = recovered.trunc() as u32;
            }
            _ => unreachable!(),
        }
    }
    assert!(lost(1).is_empty() && lost(2).is_empty() && lost(4).is_empty());
    assert!(lost(5).is_empty() && lost(6).is_empty());
    let escorts_lost = lost(7);
    if escort_hits == 0 {
        assert!(escorts_lost.is_empty());
    } else {
        let event = the_one(&escorts_lost);
        match event.kind {
            EventKind::AircraftLost { aircraft, .. } => assert_eq!(aircraft, escort_hits),
            _ => unreachable!(),
        }
        // An escort shot down loses its pilot: no draw, no breakdown.
        assert!(event.breakdown.is_none());
    }

    // Bombs, at chance 1, and the decks.
    let attack = |id: u32| -> &Event {
        the_one(&group_events(
            &afternoon,
            |k| matches!(k, EventKind::AirAttack { air_group, .. } if air_group.raw() == id),
        ))
    };
    for (id, ship_id, shots) in [(5, 1, 6), (2, 2, 1)] {
        let event = attack(id);
        match event.kind {
            EventKind::AirAttack {
                ship: target,
                weapon,
                shots: s,
                chance,
                hits,
                ..
            } => {
                assert_eq!(target, ship(ship_id));
                assert_eq!(weapon, Weapon::Bomb);
                assert_eq!(s, n(shots));
                assert_eq!(chance, Fixed::ONE);
                assert_eq!(hits, shots as u32);
            }
            _ => unreachable!(),
        }
    }
    let base_attack = the_one(&group_events(&afternoon, |k| {
        matches!(k, EventKind::BaseAttacked { .. })
    }));
    let (hits_on_aircraft, hits_on_supply) = match base_attack.kind {
        EventKind::BaseAttacked {
            air_group,
            base,
            shots,
            chance,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            ..
        } => {
            assert_eq!((air_group.raw(), base), (6, FIELD));
            assert_eq!((shots, chance, hits), (n(4), Fixed::ONE, 4));
            assert!(hits_on_aircraft + hits_on_supply <= 4);
            (hits_on_aircraft, hits_on_supply)
        }
        _ => unreachable!(),
    };
    run.supply_hits = hits_on_supply;
    // No aircraft are on the ground when the raid comes: none destroyed there.
    assert!(group_events(&afternoon, |k| matches!(
        k,
        EventKind::AircraftDestroyedOnGround { .. }
            | EventKind::DamagedAircraftDestroyedOnGround { .. }
    ))
    .is_empty());
    let _ = hits_on_aircraft;

    let deck_hits = |id: u32| -> Vec<&Event> {
        group_events(
            &afternoon,
            |k| matches!(k, EventKind::SystemHit { ship: s, .. } if s.raw() == id),
        )
    };
    let a_hits = deck_hits(CARRIER_A);
    let a_states: Vec<(ShipSystem, SystemState, SystemState)> = a_hits
        .iter()
        .map(|e| match e.kind {
            EventKind::SystemHit {
                system,
                before,
                after,
                ..
            } => (system, before, after),
            _ => unreachable!(),
        })
        .collect();
    let mut expected = vec![
        (
            ShipSystem::FlightDeck,
            SystemState::Working,
            SystemState::Degraded,
        ),
        (
            ShipSystem::FlightDeck,
            SystemState::Degraded,
            SystemState::Out,
        ),
    ];
    expected.extend([(ShipSystem::FlightDeck, SystemState::Out, SystemState::Out); 4]);
    assert_eq!(a_states, expected);
    let deck_out = a_hits[1];
    let j_hits = deck_hits(CARRIER_J);
    assert_eq!(j_hits.len(), 1);
    assert!(matches!(
        j_hits[0].kind,
        EventKind::SystemHit {
            system: ShipSystem::FlightDeck,
            before: SystemState::Working,
            after: SystemState::Degraded,
            ..
        }
    ));
    // No carrier sinks (a fight by day in a few runs can sink a cruiser).
    assert!(afternoon.iter().all(|e| !matches!(
        e.kind,
        EventKind::ShipSunk { ship: s, .. } if s.raw() == CARRIER_A || s.raw() == CARRIER_J
    )));
    for id in [CARRIER_A, CARRIER_J] {
        assert_ne!(truth.ships[&ship(id)].location, ShipLocation::Sunk);
    }
    // 6 hits of 10 points; 1 hit of 10: neither can sink.
    let damage = |id: u32| {
        let s = &truth.ships[&ship(id)];
        s.flotation_damage + s.fire_damage
    };
    assert_eq!(damage(CARRIER_A), n(60));
    assert_eq!(damage(CARRIER_J), n(10));
    assert!(truth.ships[&ship(CARRIER_A)].flotation_damage <= n(60));

    // The fuel roll: sorties of 280 beyond a radius of 250 (0.12 of it), at
    // 0.12 x 2 = 0.24; group 6's 240 rolls nothing.
    let fuel = |id: u32| -> Vec<&Event> {
        group_events(
            &afternoon,
            |k| matches!(k, EventKind::AircraftDitched { air_group, .. } if air_group.raw() == id),
        )
    };
    assert!(fuel(6).is_empty() && fuel(1).is_empty() && fuel(3).is_empty() && fuel(4).is_empty());
    let mut fuel_ditched: BTreeMap<u32, u32> = BTreeMap::new();
    for (id, sent, kind, launch) in [
        (2, 1, SortieKind::Strike, l2),
        (5, 6, SortieKind::Strike, l5),
        (7, 12 - escort_hits, SortieKind::Escort, sent),
    ] {
        let event = the_one(&fuel(id));
        match event.kind {
            EventKind::AircraftDitched {
                sortie,
                distance_nm,
                radius_nm,
                aircraft,
                chance,
                expected,
                ditched,
                crews_recovered,
                pilots_lost,
                ..
            } => {
                assert_eq!(sortie, kind);
                assert_eq!(distance_nm, n(280));
                assert_eq!(radius_nm, n(250));
                assert_eq!(aircraft, sent);
                assert_eq!(chance, fx("0.24"));
                assert_eq!(expected, n(i64::from(sent)) * fx("0.24"));
                assert_eq!(factor(event, "share_beyond_radius"), fx("0.12"));
                assert_eq!(factor(event, "ditch_chance_per_radius_beyond"), n(2));
                assert_eq!(factor(event, "chance"), fx("0.24"));
                assert_eq!(factor(event, "crew_recovered_short_of_home"), fx("0.7"));
                assert!(ditched <= aircraft);
                let recovered = crews_recovered.expect("pilots are counted");
                assert!(recovered <= ditched);
                assert_eq!(pilots_lost, ditched - recovered);
                assert!(cites(event, launch));
                fuel_ditched.insert(id, ditched);
                run.rolled += aircraft;
                run.ditched += ditched;
                run.recovered += recovered;
            }
            _ => unreachable!(),
        }
    }

    // Coming home to no deck: the scouts to the field, the others to the sea
    // beside their ships, at 0.95. Group 1's 5 form group 8.
    let landed = group_events(&afternoon, |k| {
        matches!(k, EventKind::AircraftLandedElsewhere { .. })
    });
    let landed = the_one(&landed);
    match landed.kind {
        EventKind::AircraftLandedElsewhere {
            from,
            to,
            at,
            aircraft,
            pilots,
            formed,
        } => {
            assert_eq!((from.raw(), to.raw()), (1, 8));
            assert_eq!(at, AirGroupLocation::Base(FIELD));
            assert_eq!((aircraft, pilots, formed), (5, 5, true));
            assert!(cites(landed, deck_out));
        }
        _ => unreachable!(),
    }
    let no_deck: Vec<&Event> = group_events(&afternoon, |k| {
        matches!(k, EventKind::AircraftDitchedNoDeck { .. })
    });
    let sea_of = |id: u32| no_deck.iter().copied().find(|e| of_group(&e.kind, id));
    let sea_expected = [(2, 1 - fuel_ditched[&2]), (3, 12 - patrol_hits)];
    let mut sea_aircraft_allied = 0u32;
    for (id, aircraft_expected) in sea_expected {
        match sea_of(id) {
            None => assert_eq!(aircraft_expected, 0),
            Some(event) => match event.kind {
                EventKind::AircraftDitchedNoDeck {
                    hex,
                    aircraft,
                    beside_own_force,
                    chance,
                    crews_recovered,
                    pilots_lost,
                    ..
                } => {
                    assert_eq!(hex, AT_A);
                    assert_eq!(aircraft, aircraft_expected);
                    assert!(beside_own_force);
                    assert_eq!(chance, fx("0.95"));
                    assert_eq!(factor(event, "aircraft"), n(i64::from(aircraft)));
                    assert_eq!(factor(event, "crew_recovered_chance"), fx("0.95"));
                    assert_eq!(factor(event, "ships_of_its_side_in_the_hex"), Fixed::ONE);
                    let recovered = crews_recovered.expect("pilots are counted");
                    assert_eq!(pilots_lost, aircraft - recovered);
                    assert!(cites(event, deck_out));
                    sea_aircraft_allied += aircraft;
                    run.sea += aircraft;
                    run.sea_recovered += recovered;
                }
                _ => unreachable!(),
            },
        }
    }
    assert_eq!(
        no_deck.len(),
        sea_expected.iter().filter(|(_, a)| *a > 0).count()
    );
    // The Japanese groups land aboard carrier J, with no event of the kind;
    // and nothing of it in the morning.
    for event in &all {
        if matches!(
            event.kind,
            EventKind::AircraftLandedElsewhere { .. } | EventKind::AircraftDitchedNoDeck { .. }
        ) {
            assert_eq!(event.pulse, Some(Pulse::Afternoon));
        }
    }
    assert!(!no_deck.iter().any(|e| (4..=7).contains(&group_of(e))));

    // Wear, then supply: the field pays 10 t for 5 aircraft sent.
    let spent = the_one(&group_events(&afternoon, |k| {
        matches!(
            k,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::Sorties,
                ..
            }
        )
    }));
    match spent.kind {
        EventKind::BaseSupplySpent {
            base,
            aircraft,
            asked_t,
            paid_t,
            ..
        } => {
            assert_eq!(base, FIELD);
            assert_eq!(aircraft, 5);
            assert_eq!((asked_t, paid_t), (n(10), n(10)));
        }
        _ => unreachable!(),
    }
    // Wear is rolled for group 8's sortie (it went with the aircraft), and
    // none is worn on these figures.
    let worn: Vec<&Event> =
        group_events(&afternoon, |k| matches!(k, EventKind::AircraftWorn { .. }));
    assert!(worn.iter().any(
        |e| matches!(e.kind, EventKind::AircraftWorn { air_group, .. } if air_group.raw() == 8)
    ));
    assert!(!worn.iter().any(
        |e| matches!(e.kind, EventKind::AircraftWorn { air_group, .. } if air_group.raw() == 1)
    ));

    // The field at the day's end (section 5).
    let end: Vec<&Event> = all
        .iter()
        .copied()
        .filter(|e| matches!(e.kind, EventKind::BaseAtDayEnd { .. }))
        .collect();
    let end = the_one(&end);
    match end.kind {
        EventKind::BaseAtDayEnd {
            base,
            supply_t,
            aircraft_ready,
        } => {
            assert_eq!(base, FIELD);
            assert_eq!(supply_t, n(500 - 10 - 50 * i64::from(hits_on_supply)));
            assert!(supply_t >= n(290));
            assert_eq!(aircraft_ready, 5);
            assert_eq!(truth.bases[&FIELD].supply_t, supply_t);
        }
        _ => unreachable!(),
    }
    assert_eq!(end.pulse, None);

    // The order of the afternoon, by event number.
    let first = |keep: &dyn Fn(&EventKind) -> bool| -> Option<u32> {
        afternoon
            .iter()
            .find(|e| keep(&e.kind))
            .map(|e| e.id.umpire_raw(common::umpire_key()))
    };
    let classes: Vec<(&str, Option<u32>)> = vec![
        (
            "search",
            first(&|k| matches!(k, EventKind::SearchAttempt { .. })),
        ),
        (
            "launch",
            first(&|k| matches!(k, EventKind::StrikeLaunched { .. })),
        ),
        (
            "escorts",
            first(&|k| matches!(k, EventKind::EscortsSent { .. })),
        ),
        (
            "fighters",
            first(&|k| matches!(k, EventKind::AirFight { .. })),
        ),
        (
            "losses",
            first(&|k| matches!(k, EventKind::AircraftLost { .. })),
        ),
        (
            "bombs",
            first(&|k| matches!(k, EventKind::AirAttack { .. })),
        ),
        ("deck", first(&|k| matches!(k, EventKind::SystemHit { .. }))),
        (
            "fuel",
            first(&|k| matches!(k, EventKind::AircraftDitched { .. })),
        ),
        (
            "home",
            first(&|k| matches!(k, EventKind::AircraftLandedElsewhere { .. })),
        ),
        (
            "wear",
            first(&|k| matches!(k, EventKind::AircraftWorn { .. })),
        ),
        (
            "supply",
            first(&|k| matches!(k, EventKind::BaseSupplySpent { .. })),
        ),
    ];
    let present: Vec<(&str, u32)> = classes
        .iter()
        .filter_map(|(n_, id)| id.map(|i| (*n_, i)))
        .collect();
    for pair in present.windows(2) {
        assert!(pair[0].1 < pair[1].1, "{} before {}", pair[0].0, pair[1].0);
    }
    assert!(present.last().expect("some").1 < end.id.umpire_raw(common::umpire_key()));
    // The deck goes out before the aircraft come home, and the fuel roll
    // takes its aircraft off before the homecoming counts who is left.
    assert!(deck_out.id < landed.id);

    // Each group's aircraft are all accounted for, at the day's end.
    let (g1, g1p) = truth_group(world, 1);
    assert_eq!((g1, g1p), (0, Some(0)));
    assert_eq!(truth_group(world, 8), (5, Some(5)));
    let (g2, _) = truth_group(world, 2);
    assert_eq!(g2, 0);
    let (g3, g3p) = truth_group(world, 3);
    assert_eq!(g3, 0);
    // Group 3: 12 sent as a patrol, shot down or in the sea; its pilots are
    // 12 less those lost with the shot-down aircraft and in the sea.
    let patrol_pilots_lost =
        (patrol_hits - patrol_recovered_here) + sea_of(3).map_or(0, pilots_lost_of);
    assert_eq!(g3p, Some(12 - patrol_pilots_lost));
    assert!(g3p.expect("counted") <= 12);
    assert_eq!(truth_group(world, 4), (5, Some(5)));
    assert_eq!(truth_group(world, 6), (4, Some(4)));
    assert_eq!(truth_group(world, 5).0, 6 - fuel_ditched[&5]);
    assert_eq!(truth_group(world, 7).0, 12 - escort_hits - fuel_ditched[&7]);
    // Pilots lost never exceed the aircraft lost or ditched.
    let (_, p5) = truth_group(world, 5);
    assert!(6 - p5.expect("counted") <= fuel_ditched[&5]);
    let (_, p7) = truth_group(world, 7);
    assert!(12 - p7.expect("counted") <= escort_hits + fuel_ditched[&7]);
    let (_, p2) = truth_group(world, 2);
    assert!(1 - p2.expect("counted") <= 1);

    // The result (section 5).
    let result = world.result().expect("the scenario has objectives");
    let find = |side: Side, kind: &str| {
        result
            .side(side)
            .objectives
            .iter()
            .find(|o| o.kind == kind)
            .unwrap_or_else(|| panic!("{kind}"))
            .clone()
    };
    let field_open = find(Side::Allied, "base_open");
    assert_eq!((field_open.count, field_open.days), (n(1), Some(1)));
    assert_eq!(field_open.points, n(5));
    let field_shut = find(Side::Japanese, "base_open");
    assert_eq!(field_shut.count, Fixed::ZERO);
    assert_eq!(field_shut.points, Fixed::ZERO);
    let crippled = find(Side::Japanese, "ships_crippled");
    assert_eq!(
        crippled.by_kind.get(&sim_core::refdata::ShipKind::Carrier),
        Some(&1)
    );
    assert_eq!(crippled.points, n(10));
    assert_eq!(find(Side::Allied, "ships_crippled").points, Fixed::ZERO);
    let tally = tallies(world.umpire(), world.events(), world.ref_data());
    // Allied aircraft lost: the patrol shot down, group 2 and 3 ditched.
    let allied_lost = patrol_hits + fuel_ditched[&2] + sea_aircraft_allied;
    assert_eq!(tally[&Side::Japanese].aircraft, allied_lost);
    let japanese_lost = escort_hits + fuel_ditched[&5] + fuel_ditched[&7];
    assert_eq!(tally[&Side::Allied].aircraft, japanese_lost);
    assert_eq!(
        find(Side::Japanese, "aircraft_destroyed").points,
        n(i64::from(allied_lost)) * fx("0.5")
    );
    assert_eq!(
        find(Side::Allied, "aircraft_destroyed").points,
        n(i64::from(japanese_lost)) * fx("0.5")
    );
}

fn group_of(event: &Event) -> u32 {
    match event.kind {
        EventKind::AircraftDitchedNoDeck { air_group, .. } => air_group.raw(),
        _ => 0,
    }
}

fn pilots_lost_of(event: &Event) -> u32 {
    match event.kind {
        EventKind::AircraftDitchedNoDeck { pilots_lost, .. } => pilots_lost,
        _ => 0,
    }
}

/// Every cause of an event comes before it.
fn causes_precede(world: &World) {
    for event in world.events().iter() {
        for cause in &event.causes {
            if let Cause::Event(id) = cause {
                assert!(
                    id.umpire_raw(common::umpire_key()) < event.id.umpire_raw(common::umpire_key()),
                    "event {} cites a later event {}",
                    event.id.umpire_raw(common::umpire_key()),
                    id.umpire_raw(common::umpire_key())
                );
            }
        }
    }
}

/// Everything of sections 2 to 6 and the properties of section 8 that hold
/// in each run on its own.
fn check_run(world: &World) -> Run {
    let mut run = Run::default();
    causes_precede(world);
    check_night(world, &mut run);
    check_morning(world, &mut run);
    check_afternoon(world, &mut run);
    let afternoon_fight = in_pulse(world, Pulse::Afternoon)
        .iter()
        .any(|e| matches!(e.kind, EventKind::EngagementBegun { .. }));
    run.day_fight |= afternoon_fight;
    if run.day_fight {
        // A fight by day needs a force stopped in the fight's hex: the
        // Japanese force, with a ship whose propulsion is out (force 3 lies
        // stopped all along).
        let truth = world.umpire();
        assert!(
            (6..=8).any(|id| {
                truth.ships[&ship(id)].systems.get(ShipSystem::Propulsion) == SystemState::Out
            }),
            "a day fight without a Japanese ship dead in the water"
        );
    }
    run
}

/// Whether `count` successes in `trials` at chance `p` is within four
/// standard deviations of the mean.
fn within_four_sigma(count: u32, trials: u32, p: f64) -> bool {
    let trials = f64::from(trials);
    let mean = trials * p;
    let sigma = (trials * p * (1.0 - p)).sqrt();
    (f64::from(count) - mean).abs() <= 4.0 * sigma + 1e-9
}

// ---------------------------------------------------------------------
// The setting (the table of figures, where everything is, section 1).
// ---------------------------------------------------------------------

#[test]
fn the_figures_and_the_places_are_what_the_calculation_uses() {
    let setup = Setup::working("setting");
    let world = &setup.base.world;
    let data = world.ref_data();
    let surface = data.surface_combat.as_ref().expect("surface figures");
    assert_eq!(surface.experience_gun_strength, fx("0.4"));
    assert_eq!(surface.experience_torpedo_strength, fx("0.4"));
    assert_eq!(surface.experience_sighting_strength, fx("0.4"));
    assert_eq!(surface.unready_gun_share, fx("0.5"));
    assert_eq!(surface.system_hit_chance_per_point, fx("0.1"));
    assert_eq!(surface.sighting_chance_night, fx("0.5"));
    assert_eq!(surface.not_alert_sighting_factor, fx("0.5"));
    assert_eq!(surface.gun_hit_chance_night, fx("0.03"));
    assert_eq!(surface.torpedo_hit_chance_night, fx("0.04"));
    assert_eq!(surface.surprised_target_factor, fx("1.5"));
    assert_eq!(surface.lit_target_factor, n(2));
    let air = data.air_combat.as_ref().expect("air combat figures");
    assert_eq!(air.bomb_hit_chance, Fixed::ONE);
    assert_eq!(air.fighter_hit_chance, fx("0.3"));
    assert_eq!(air.strike_contact_max_age_pulses, 0);
    assert_eq!(air.bomb_system_hit[&ShipSystem::FlightDeck], Fixed::ONE);
    assert_eq!(air.base_hit_aircraft, fx("0.3"));
    assert_eq!(air.base_hit_supply, fx("0.3"));
    assert_eq!(air.base_supply_destroyed_per_hit_t, n(50));
    assert_eq!(air.extended_reach_share, fx("0.25"));
    assert_eq!(air.ditch_chance_per_radius_beyond, n(2));
    assert_eq!(air.crew_recovered_short_of_home, fx("0.7"));
    assert_eq!(air.crew_recovered_beside_own_force, fx("0.95"));
    assert_eq!(air.crew_recovered_open_sea, fx("0.5"));
    assert_eq!(air.crew_recovered_over_own_base, fx("0.6"));
    let upkeep = data.air_upkeep.as_ref().expect("air upkeep figures");
    assert_eq!(upkeep.wear_base_chance, Fixed::ZERO);
    assert_eq!(upkeep.wear_chance_per_100_nm, Fixed::ZERO);

    // Radius 250, extended reach 312, the scouts to 300; a bomb is 10 points.
    let dive = AircraftTypeId::new(3);
    assert_eq!(data.reach_nm(dive, Reach::Normal), n(250));
    assert_eq!(data.reach_nm(dive, Reach::Extended), n(312));
    let class = &data.ship_classes[&sim_core::ids::ShipClassId::new(4)];
    assert_eq!(class.toughness, n(600));
    assert_eq!(class.aircraft_capacity, 36);
    assert_eq!(
        n(100).mul_div(data.aircraft_types[&dive].attack_damage, class.toughness),
        n(10)
    );
    let scout = &data.aircraft_types[&AircraftTypeId::new(2)];
    assert_eq!(
        (scout.strike_range_nm, scout.search_range_nm),
        (Fixed::ZERO, n(300))
    );

    // The table of air distances.
    let at = |q, r| Hex::new(q, r);
    for (from, to, miles) in [
        (at(2, 0), at(11, 0), 360),
        (at(2, 0), at(9, 0), 280),
        (at(2, 0), at(9, -6), 262),
        (at(2, 0), at(2, -4), 160),
        (at(11, 0), at(9, -6), 288),
        (at(9, 0), at(9, -6), 240),
        (at(9, 0), at(2, -4), 386),
        (at(11, 0), at(2, -4), 461),
    ] {
        assert_eq!(from.air_distance_nm(to), miles, "{from:?} to {to:?}");
    }
    // The start: where the forces and the field are, and what each carries.
    let truth = world.umpire();
    let position = |tf: u32| truth.task_forces[&TaskForceId::new(tf)].position();
    assert_eq!(position(1), at(2, 0));
    assert_eq!(position(2), at(14, 3));
    assert_eq!(position(3), at(2, -4));
    assert_eq!(position(4), at(7, -4));
    assert_eq!(truth.bases[&FIELD].hex, FIELD_HEX);
    assert_eq!(truth.bases[&FIELD].supply_t, n(500));
    let experience = |id: u32| truth.ships[&ship(id)].experience;
    assert_eq!([3, 4, 5].map(experience), [1, 1, 1]);
    assert_eq!([6, 7, 8].map(experience), [3, 3, 3]);
    let aboard = |ship_id: u32| -> u32 {
        truth
            .air_groups
            .values()
            .filter(|g| g.location == AirGroupLocation::Ship(ship(ship_id)))
            .map(|g| g.ready + g.damaged)
            .sum()
    };
    assert_eq!((aboard(1), aboard(2)), (18, 27));
}

#[test]
fn carrier_j_is_charged_straight_line_miles_and_the_cruisers_forty_a_step() {
    let setup = Setup::working("steaming");
    let world = setup.run(1942);
    let steamed = |tf: u32, pulse: Pulse| -> Vec<&Event> {
        in_pulse(&world, pulse)
            .into_iter()
            .filter(|e| matches!(e.kind, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == tf))
            .collect()
    };
    // Carrier J: 192 miles in the night, in (11,0) at 06:00, 64 t burned.
    let night = steamed(2, Pulse::Night);
    let night = the_one(&night);
    match night.kind {
        EventKind::TaskForceSteamed {
            from,
            to,
            distance_nm,
            speed_kn,
            ..
        } => {
            assert_eq!((from, to), (Hex::new(14, 3), Hex::new(11, 0)));
            assert_eq!((distance_nm, speed_kn), (n(192), n(16)));
        }
        _ => unreachable!(),
    }
    // 30 + 27 + 23 + 5 x 40 = 280 along the way; the night's segment begins
    // six steps: 30, 27, 23, 40, 40 done (160) and the sixth 32 in.
    assert_eq!(steps(night), [30, 27, 23, 40, 40, 40]);
    assert_eq!(factor(night, "time_limit_nm"), n(192));
    assert_eq!(factor(night, "pulse_hours"), n(12));
    assert_eq!(
        night.breakdown.as_ref().expect("a breakdown").result,
        n(192)
    );
    assert_eq!(
        Hex::new(14, 3).air_distance_nm(Hex::new(9, 0)),
        280,
        "30 + 27 + 23 + 5 x 40"
    );
    let burned: Vec<&Event> = in_pulse(&world, Pulse::Night)
        .into_iter()
        .filter(|e| matches!(e.kind, EventKind::FuelBurned { ship: s, task_force, .. } if s == ship(2) && task_force.raw() == 2))
        .collect();
    match the_one(&burned).kind {
        EventKind::FuelBurned { burned_t, .. } => assert_eq!(burned_t, n(64)),
        _ => unreachable!(),
    }
    // The morning: the 88 miles left, arriving at (9,0) after 5.5 hours.
    let morning = steamed(2, Pulse::Morning);
    let morning = the_one(&morning);
    match morning.kind {
        EventKind::TaskForceSteamed {
            from,
            to,
            distance_nm,
            ..
        } => {
            assert_eq!((from, to), (Hex::new(11, 0), Hex::new(9, 0)));
            assert_eq!(distance_nm, n(88));
        }
        _ => unreachable!(),
    }
    let arrived: Vec<&Event> = in_pulse(&world, Pulse::Morning)
        .into_iter()
        .filter(|e| matches!(e.kind, EventKind::TaskForceArrived { task_force, .. } if task_force.raw() == 2))
        .collect();
    match the_one(&arrived).kind {
        EventKind::TaskForceArrived {
            hex, elapsed_hours, ..
        } => {
            assert_eq!(hex, Hex::new(9, 0));
            // 12 hours of night and 5.5 of morning.
            assert_eq!(elapsed_hours, fx("17.5"));
        }
        _ => unreachable!(),
    }
    assert!(steamed(2, Pulse::Afternoon).is_empty());
    // The Japanese cruisers: 40 a step, 180 miles to (2,-4).
    let cruisers: Vec<&Event> = steamed(4, Pulse::Night);
    match cruisers[0].kind {
        EventKind::TaskForceSteamed {
            from,
            to,
            distance_nm,
            ..
        } => {
            assert_eq!((from, to), (Hex::new(7, -4), FIGHT));
            assert_eq!(distance_nm, n(180));
        }
        _ => unreachable!(),
    }
    assert_eq!(steps(cruisers[0]), [40, 40, 40, 40, 40]);
}

// ---------------------------------------------------------------------
// Section 8: every seed from 1 to 1,000 (which carries sections 2 to 6).
// ---------------------------------------------------------------------

#[test]
fn over_seeds_1_to_1000_every_run_is_as_calculated_and_the_counts_are_near_their_means() {
    let setup = Setup::working("seeds");
    let runs = over_seeds(&setup, 1, 1000, |_, world| check_run(world));
    let total = |f: fn(&Run) -> u32| runs.iter().map(f).sum::<u32>();
    let unseen = total(|r| u32::from(r.unseen));
    let day_fights = total(|r| u32::from(r.day_fight));
    eprintln!(
        "runs {}, night passed unseen {unseen}, day fights {day_fights}",
        runs.len()
    );

    // Patrol fighters and escorts shot down: 12 shots at 0.3 each.
    let shots = 12 * runs.len() as u32;
    assert!(within_four_sigma(total(|r| r.patrol_shot_down), shots, 0.3));
    assert!(within_four_sigma(
        total(|r| r.escorts_shot_down),
        shots,
        0.3
    ));
    // Ditched for fuel: 0.24 of the aircraft rolled; crews: 0.7 of those.
    let (rolled, ditched, recovered) = (
        total(|r| r.rolled),
        total(|r| r.ditched),
        total(|r| r.recovered),
    );
    eprintln!("fuel: rolled {rolled}, ditched {ditched}, crews {recovered}");
    assert!(within_four_sigma(ditched, rolled, 0.24));
    assert!(within_four_sigma(recovered, ditched, 0.7));
    // In the sea for want of a deck: 0.95 of the crews; and of the patrol's
    // pilot draws the same.
    let (sea, sea_recovered) = (total(|r| r.sea), total(|r| r.sea_recovered));
    eprintln!("sea: {sea}, crews {sea_recovered}");
    assert!(within_four_sigma(sea_recovered, sea, 0.95));
    let (drawn, patrol_recovered) = (total(|r| r.patrol_drawn), total(|r| r.patrol_recovered));
    assert!(within_four_sigma(patrol_recovered, drawn, 0.95));
    // Hits on the field's supply: 0.3 of 4 a run.
    let supply = total(|r| r.supply_hits);
    eprintln!("supply hits {supply}");
    assert!(within_four_sigma(supply, 4 * runs.len() as u32, 0.3));
    // Fewer than a few in a hundred runs have a fight by day.
    assert!(day_fights < 50);
}

// ---------------------------------------------------------------------
// Section 3 and 4: what the orders do. The hold is the list; the strike
// beyond the radius is the reach.
// ---------------------------------------------------------------------

#[test]
fn with_the_target_list_taken_out_group_2_launches_at_the_cruisers_in_the_morning() {
    let text = without_block(&scenario_text(), TARGET_ORDER);
    let setup = Setup::with_text("no_list", text);
    let mut launched = 0;
    for seed in 1..=50 {
        let world = setup.run(seed);
        let strikes: Vec<&Event> = in_pulse(&world, Pulse::Morning)
            .into_iter()
            .filter(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
            .collect();
        let reported = in_pulse(&world, Pulse::Night)
            .iter()
            .any(|e| matches!(e.kind, EventKind::ReportReceived { .. }));
        if !reported {
            assert!(strikes.is_empty());
            continue;
        }
        let strike = the_one(&strikes);
        match strike.kind {
            EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                aircraft,
                distance_nm,
            } => {
                assert_eq!(
                    (
                        air_group.raw(),
                        contact.umpire_raw(common::umpire_key()),
                        hex
                    ),
                    (2, 1, FIGHT)
                );
                assert_eq!((aircraft, distance_nm), (1, n(160)));
            }
            _ => unreachable!(),
        }
        // Its causes: the mission order and the reach order, and no list.
        let orders = strike
            .causes
            .iter()
            .filter(|c| matches!(c, Cause::Order(_)))
            .count();
        assert_eq!(orders, 2);
        launched += 1;
    }
    assert!(launched > 40);
}

#[test]
fn on_normal_reach_group_2_never_launches_and_nothing_of_its_rolls() {
    let text = without_block(&scenario_text(), REACH_ORDER_G2);
    let setup = Setup::with_text("normal_reach", text);
    for seed in 1..=50 {
        let world = setup.run(seed);
        for event in events(&world) {
            assert!(
                !of_group(&event.kind, 2) || matches!(event.kind, EventKind::AircraftLost { .. }),
                "group 2 does nothing on normal reach: event {}",
                event.id.umpire_raw(common::umpire_key())
            );
        }
        // Group 5 is unchanged: its strike is launched and rolled.
        assert_eq!(
            events(&world)
                .iter()
                .filter(|e| matches!(e.kind, EventKind::AircraftDitched { air_group, .. } if air_group.raw() == 5))
                .count(),
            1
        );
    }
}

// ---------------------------------------------------------------------
// Section 7: seed 1942, pinned.
// ---------------------------------------------------------------------

#[test]
fn seed_1942_gives_what_it_gave_pinned() {
    let setup = Setup::working("pinned");
    let mut scenario = setup.base.with_seed(1942).expect("a seed");
    let summaries = scenario.run_to_end().expect("the run resolves");
    assert_eq!(summaries[0].event_count, 166);
    let world = scenario.world;
    let run = check_run(&world);
    assert!(!run.unseen && !run.day_fight);

    // Night: both sight in round 1; force 3 breaks off in round 2; no ship
    // is sunk.
    let night = in_pulse(&world, Pulse::Night);
    let rolls: Vec<(u32, u32, bool)> = night
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::SightingRoll {
                round,
                task_force,
                sighted,
                ..
            } => Some((round, task_force.raw(), sighted)),
            _ => None,
        })
        .collect();
    assert_eq!(rolls, [(1, 3, true), (1, 4, true)]);
    let broke: Vec<(u32, u32)> = night
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::ForceBrokeOff {
                round, task_force, ..
            } => Some((round, task_force.raw())),
            _ => None,
        })
        .collect();
    assert_eq!(broke, [(2, 3)]);
    assert!(night
        .iter()
        .all(|e| !matches!(e.kind, EventKind::ShipSunk { .. })));

    // Over carrier A: 5 patrol fighters shot down, no pilot lost; 2 escorts.
    assert_eq!((run.patrol_shot_down, run.escorts_shot_down), (5, 2));
    assert_eq!((run.patrol_drawn, run.patrol_recovered), (5, 5));

    // Fuel and no deck.
    let afternoon = in_pulse(&world, Pulse::Afternoon);
    let fuel = |id: u32| -> (u32, u32, Option<u32>, u32, u32) {
        let found = group_events(
            &afternoon,
            |k| matches!(k, EventKind::AircraftDitched { air_group, .. } if air_group.raw() == id),
        );
        match the_one(&found).kind {
            EventKind::AircraftDitched {
                aircraft,
                ditched,
                crews_recovered,
                pilots_lost,
                ready,
                ..
            } => (aircraft, ditched, crews_recovered, pilots_lost, ready),
            _ => unreachable!(),
        }
    };
    assert_eq!(fuel(2), (1, 0, Some(0), 0, 1));
    assert_eq!(fuel(5), (6, 3, Some(2), 1, 3));
    assert_eq!(fuel(7), (10, 6, Some(5), 1, 4));
    let sea = |id: u32| -> (u32, Option<u32>, u32) {
        let found = group_events(
            &afternoon,
            |k| matches!(k, EventKind::AircraftDitchedNoDeck { air_group, .. } if air_group.raw() == id),
        );
        match the_one(&found).kind {
            EventKind::AircraftDitchedNoDeck {
                aircraft,
                crews_recovered,
                pilots_lost,
                ..
            } => (aircraft, crews_recovered, pilots_lost),
            _ => unreachable!(),
        }
    };
    assert_eq!(sea(2), (1, Some(1), 0));
    assert_eq!(sea(3), (7, Some(6), 1));

    // The raid: 1 hit on aircraft, 0 on supply, 3 on nothing.
    let raid = the_one(&group_events(&afternoon, |k| {
        matches!(k, EventKind::BaseAttacked { .. })
    }));
    assert!(matches!(
        raid.kind,
        EventKind::BaseAttacked {
            hits: 4,
            hits_on_aircraft: 1,
            hits_on_supply: 0,
            ..
        }
    ));
    // The field: 490 t, 5 aircraft ready, open.
    let end = the_one(
        &events(&world)
            .into_iter()
            .filter(|e| matches!(e.kind, EventKind::BaseAtDayEnd { .. }))
            .collect::<Vec<_>>(),
    );
    assert!(matches!(
        end.kind,
        EventKind::BaseAtDayEnd {
            supply_t,
            aircraft_ready: 5,
            ..
        } if supply_t == n(490)
    ));
    // Aircraft destroyed: 11 Japanese (2 + 3 + 6) and 13 Allied (5 + 1 + 7).
    let tally = tallies(world.umpire(), world.events(), world.ref_data());
    assert_eq!(tally[&Side::Allied].aircraft, 11);
    assert_eq!(tally[&Side::Japanese].aircraft, 13);
    // Result: Allied 5.5 + 5 = 10.5, Japanese 6.5 + 10 = 16.5, margin 6, a
    // minor Japanese victory.
    let result = world.result().expect("objectives");
    assert_eq!(result.allied.total, fx("10.5"));
    assert_eq!(result.japanese.total, fx("16.5"));
    assert_eq!(result.japanese.margin, n(6));
    assert_eq!(result.japanese.outcome.name(), "minor victory");
}

// ---------------------------------------------------------------------
// Section 8, the rest: a replay, breakdowns, and what the enemy sees.
// ---------------------------------------------------------------------

#[test]
fn a_replay_gives_the_same_hash_and_recording_breakdowns_changes_nothing() {
    let setup = Setup::working("replay");
    for seed in [1, 2, 3, 77, 1942] {
        let first = setup.run(seed);
        let second = setup.run(seed);
        assert_eq!(first.day_hashes(), second.day_hashes());
        assert_eq!(first.events().len(), second.events().len());
        let mut scenario = setup.base.with_seed(seed).expect("a seed");
        scenario.world.set_config(SimConfig {
            record_breakdowns: false,
        });
        scenario.run_to_end().expect("the run resolves");
        assert_eq!(first.day_hashes(), scenario.world.day_hashes());
        assert_eq!(first.events().len(), scenario.world.events().len());
    }
    assert_ne!(setup.run(1).day_hashes(), setup.run(2).day_hashes());
}

#[test]
fn a_side_sees_its_own_ditchings_and_landings_whole_and_the_enemy_none() {
    let setup = Setup::working("views");
    let owner = |id: AirGroupId| {
        if id.raw() <= 3 {
            Side::Allied
        } else {
            Side::Japanese
        }
    };
    let mut seen_total = 0;
    for seed in 1..=100 {
        let world = setup.run(seed);
        let mut own = BTreeMap::new();
        for event in events(&world) {
            let id = match event.kind {
                EventKind::AircraftDitched { air_group, .. }
                | EventKind::AircraftDitchedNoDeck { air_group, .. } => air_group,
                EventKind::AircraftLandedElsewhere { from, .. } => from,
                _ => continue,
            };
            *own.entry(owner(id)).or_insert(0usize) += 1;
        }
        for side in Side::ALL {
            let seen = world
                .side_view(side)
                .events()
                .iter()
                .filter(|e| {
                    matches!(
                        &e.kind,
                        SideEventKind::Whole(
                            EventKind::AircraftDitched { .. }
                                | EventKind::AircraftDitchedNoDeck { .. }
                                | EventKind::AircraftLandedElsewhere { .. }
                        )
                    )
                })
                .count();
            assert_eq!(seen, own.get(&side).copied().unwrap_or(0), "seed {seed}");
            seen_total += seen;
        }
    }
    assert!(seen_total > 100);
}

// ---------------------------------------------------------------------
// Section 9: two days. The raid meets the aircraft that landed.
// ---------------------------------------------------------------------

#[test]
fn over_two_days_the_raid_on_day_2_falls_on_the_scouts_that_landed() {
    let text = common::replace_once(&scenario_text(), "days = 1\n", "days = 2\n");
    let setup = Setup::with_text("two_days", text);
    let runs = over_seeds(&setup, 1, 200, |seed, world| {
        let day =
            |d: u32| -> Vec<&Event> { world.events().iter().filter(|e| e.day == d).collect() };
        let day2 = day(2);
        // Group 6 launches at the field once on day 2, in the morning.
        let launches: Vec<&&Event> = day2
            .iter()
            .filter(|e| matches!(e.kind, EventKind::BaseStrikeLaunched { air_group, .. } if air_group.raw() == 6))
            .collect();
        assert_eq!(launches.len(), 1, "seed {seed}");
        assert_eq!(launches[0].pulse, Some(Pulse::Morning));
        match launches[0].kind {
            EventKind::BaseStrikeLaunched {
                base,
                aircraft,
                distance_nm,
                ..
            } => {
                assert_eq!(base, FIELD);
                assert_eq!(distance_nm, n(240));
                assert!(aircraft >= 1);
            }
            _ => unreachable!(),
        }
        // Group 8's 5 scouts are on the ground; the raid's hits on aircraft
        // fall on them and on nothing else.
        let attack: Vec<&&Event> = day2
            .iter()
            .filter(|e| matches!(e.kind, EventKind::BaseAttacked { .. }))
            .collect();
        let hits_on_aircraft: u32 = attack
            .iter()
            .map(|e| match e.kind {
                EventKind::BaseAttacked {
                    hits_on_aircraft, ..
                } => hits_on_aircraft,
                _ => 0,
            })
            .sum();
        let destroyed: Vec<(u32, u32)> = day2
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } => Some((air_group.raw(), aircraft)),
                _ => None,
            })
            .collect();
        let on_group_8: u32 = destroyed.iter().map(|(_, a)| *a).sum();
        assert!(destroyed.iter().all(|(g, _)| *g == 8), "seed {seed}");
        assert_eq!(on_group_8, hits_on_aircraft.min(5), "seed {seed}");
        // No aircraft takes off from carrier A: her deck is out.
        for event in &day2 {
            assert!(
                !matches!(
                    event.kind,
                    EventKind::SearchFlown { air_group, .. }
                        | EventKind::StrikeLaunched { air_group, .. }
                        | EventKind::EscortsSent { air_group, .. }
                        if air_group.raw() <= 3
                ),
                "seed {seed}: event {} flies from carrier A",
                event.id.umpire_raw(common::umpire_key())
            );
        }
        // The field's count at the end of day 2 is what the state holds.
        let ends: Vec<&Event> = world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::BaseAtDayEnd { .. }))
            .collect();
        assert_eq!(ends.len(), 2);
        let (supply, ready) = match ends[1].kind {
            EventKind::BaseAtDayEnd {
                supply_t,
                aircraft_ready,
                ..
            } => (supply_t, aircraft_ready),
            _ => unreachable!(),
        };
        let truth = world.umpire();
        assert_eq!(supply, truth.bases[&FIELD].supply_t);
        let at_field: u32 = truth
            .air_groups
            .values()
            .filter(|g| g.location == AirGroupLocation::Base(FIELD))
            .map(|g| g.ready)
            .sum();
        assert_eq!(ready, at_field);
        assert_eq!(ready, truth.air_groups[&group(8)].ready);
        // Open only if both are above zero: the result's days agree.
        let result = world.result().expect("objectives");
        let open = result
            .allied
            .objectives
            .iter()
            .find(|o| o.kind == "base_open")
            .expect("the field objective");
        let day_one_open = 1;
        let day_two_open = u32::from(supply > Fixed::ZERO && ready > 0);
        assert_eq!(open.count, n(i64::from(day_one_open + day_two_open)));
        assert_eq!(open.days, Some(2));
        (on_group_8, hits_on_aircraft)
    });
    // Over 200 runs the raid does fall on the scouts: 0.3 of 2 or 3 hits.
    assert!(runs.iter().any(|(destroyed, _)| *destroyed > 0));
}

// ---------------------------------------------------------------------
// Section 10: the frozen figures, nothing altered.
// ---------------------------------------------------------------------

#[test]
fn on_the_frozen_figures_no_breakdown_names_experience_or_the_share() {
    let base = common::load(&scenario_path()).expect("the scenario loads on the frozen figures");
    let mut gun_salvos = 0;
    for seed in 1..=100 {
        let mut scenario = base.with_seed(seed).expect("a seed");
        scenario.run_to_end().expect("the run resolves");
        let world = scenario.world;
        for event in events(&world) {
            assert!(!names_factor(event, "experience_factor"), "seed {seed}");
            assert!(!names_factor(event, "unready_gun_share"), "seed {seed}");
            if let EventKind::Salvo {
                firer,
                weapon: Weapon::Gun,
                shots,
                ..
            } = event.kind
            {
                if event.pulse == Some(Pulse::Night)
                    && (3..=5).contains(&firer.raw())
                    && factor(event, "main_battery_factor") == Fixed::ONE
                {
                    assert_eq!(shots, n(120));
                    gun_salvos += 1;
                }
            }
        }
    }
    assert!(gun_salvos > 100);
}

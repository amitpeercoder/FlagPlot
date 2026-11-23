//! What the staff does with decision 0136, step 3 of stage 2b: the order that
//! sets a carrier force to radio silence at preserve the force (item 18), and
//! the doctrines addressed to the area reading the forces as the area command
//! holds them (items 9 and 11). The games are scenario 62 (two carrier
//! forces, command levels on) on the frozen step 1 figures, with report
//! figures of this file's own in which every chance is 0 or 1. The
//! architect's hand calculation of the whole is
//! `tests/scenario_63_a_superiors_knowledge.rs`, which these do not repeat.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use sim_core::command::{CommandKind, StaffDecisions};
use sim_core::ids::TaskForceId;
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::pictures::CommandRef;
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario, FROZEN_REFERENCE_DIR};
use sim_staff::{as_staffs, staffs};

const REPORTS_LINE: &str = "reports = \"../reference/reports_placeholder_scenario_62.toml\"\n";
const LEVELS_LINE: &str = "command_levels = true\n";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn scenario_path() -> PathBuf {
    root().join("data/scenarios/62_reports_between_commands.toml")
}

fn once(text: &str, original: &str, changed: &str) -> String {
    assert_eq!(
        text.matches(original).count(),
        1,
        "{original:?} appears once"
    );
    text.replace(original, changed)
}

/// The frozen figures with the table of aircraft for each ship that scenario
/// 62 adds, copied to a folder of this test's own under the build directory.
fn options() -> LoadOptions {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "reference_held_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).expect("make the folder");
    for entry in std::fs::read_dir(root().join(FROZEN_REFERENCE_DIR)).expect("frozen reads") {
        let entry = entry.expect("an entry");
        if entry.file_name().to_string_lossy().ends_with(".md") {
            continue;
        }
        std::fs::copy(entry.path(), dir.join(entry.file_name())).expect("copy");
    }
    let file = dir.join("air_combat.toml");
    let text = std::fs::read_to_string(&file).expect("air combat reads");
    std::fs::write(
        &file,
        format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n"),
    )
    .expect("write");
    LoadOptions::with_reference_root(dir)
}

/// Report figures of this test's own: `up` is `(on_time, late)`, every other
/// chance 0.
fn reports_line(up: (u32, u32)) -> String {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let file = dir.join(format!(
        "reports_held_{}_{}.toml",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let body = format!(
        "placeholder = true\n\n[up]\non_time = \"{}\"\nlate = \"{}\"\n\n[sideways]\non_time = \"0\"\nlate = \"0\"\n\n[down]\non_time = \"0\"\nlate = \"0\"\n\n[radio_reach_nm]\ncarrier = 600\nbattleship = 600\ncruiser = 600\nother_ship = 300\nbase = 1000\n",
        up.0, up.1
    );
    std::fs::write(&file, body).expect("write the report figures");
    format!("reports = \"{}\"\n", file.display())
}

/// Scenario 62 with report figures `up`, task force 1's intent at `risk`.
fn text(up: (u32, u32), risk: &str) -> String {
    let text = std::fs::read_to_string(scenario_path()).expect("scenario 62 reads");
    let text = once(&text, REPORTS_LINE, &reports_line(up));
    once(&text, "risk = \"press\"", &format!("risk = \"{risk}\""))
}

/// What the Allied staff handed in each day of the game.
fn play(text: &str) -> (Scenario, Vec<StaffDecisions>) {
    let mut scenario =
        Scenario::from_text_with(text, &scenario_path(), &options()).expect("the scenario loads");
    let held = staffs();
    let mut handed = Vec::new();
    while scenario
        .next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
        .is_some()
    {
        handed.push(
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
    }
    (scenario, handed)
}

/// The forces the staff set to radio silence in a day's hand-in.
fn silenced(decisions: &StaffDecisions) -> Vec<TaskForceId> {
    decisions
        .orders
        .iter()
        .filter_map(|o| match &o.kind {
            OrderKind::TaskForce {
                target,
                order: TaskForceOrder::SetRadioSilence { on: true },
            } => Some(*target),
            _ => None,
        })
        .collect()
}

fn any_silence_order(handed: &[StaffDecisions]) -> bool {
    handed.iter().any(|d| {
        d.orders.iter().any(|o| {
            matches!(
                o.kind,
                OrderKind::TaskForce {
                    order: TaskForceOrder::SetRadioSilence { .. },
                    ..
                }
            )
        })
    })
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

const ON_TIME: (u32, u32) = (1, 0);
const NEVER: (u32, u32) = (0, 0);

#[test]
fn silence_is_given_at_preserve_with_report_figures_and_command_levels_on() {
    let (scenario, handed) = play(&text(ON_TIME, "preserve"));
    // Task force 1 is at preserve the force; task force 2's intent has no
    // risk set, so it is at balanced and is left as it is.
    assert_eq!(silenced(&handed[0]), vec![force(1)]);
    let order = handed[0]
        .orders
        .iter()
        .find(|o| {
            matches!(
                o.kind,
                OrderKind::TaskForce {
                    order: TaskForceOrder::SetRadioSilence { .. },
                    ..
                }
            )
        })
        .expect("the order");
    assert!(order.reason.worked.contains("radio silence"));
    assert!(order.reason.worked.contains("decision 0136, 19"));
    // The staff never gives it twice: once on, it is not given again.
    assert!(handed[1..].iter().all(|d| silenced(d).is_empty()));
    assert_eq!(
        scenario
            .world
            .side_view(Side::Allied)
            .radio_silence(force(1)),
        Some(true)
    );
    assert_eq!(
        scenario
            .world
            .side_view(Side::Allied)
            .radio_silence(force(2)),
        Some(false)
    );
}

#[test]
fn silence_is_not_given_where_it_is_already_on() {
    let base = text(ON_TIME, "preserve");
    let text = once(
        &base,
        "name = \"Placeholder carrier force 1\"\n",
        "name = \"Placeholder carrier force 1\"\nradio_silence = true\n",
    );
    let (_, handed) = play(&text);
    assert!(!any_silence_order(&handed));
}

#[test]
fn silence_is_not_given_at_balanced_or_press() {
    for risk in ["balanced", "press"] {
        let (_, handed) = play(&text(ON_TIME, risk));
        assert!(!any_silence_order(&handed), "at {risk}");
    }
}

#[test]
fn silence_is_not_given_with_no_report_figures_or_with_command_levels_off() {
    let base = text(ON_TIME, "preserve");
    let reports_at = base.find("reports = \"").expect("the reports line");
    let end = reports_at + base[reports_at..].find('\n').expect("a line") + 1;
    let without_reports = format!("{}{}", &base[..reports_at], &base[end..]);
    let (scenario, handed) = play(&without_reports);
    assert!(!scenario.world.side_view(Side::Allied).reports_travel());
    assert!(!any_silence_order(&handed));
    let (_, handed) = play(&once(&without_reports, LEVELS_LINE, ""));
    assert!(!any_silence_order(&handed));
}

/// Scenario 62 with its first intent a deny intent over (0,5) naming task
/// force 1: an intent addressed to the area, which forms the carrier task.
/// The deny's forces limit says `in_place` since decision 0222, ruling I.
fn deny_text(up: (u32, u32)) -> String {
    let text = text(up, "press");
    let text = once(
        &text,
        "intent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [0, 0]",
        "intent_kind = \"deny\"\nhex = [0, 5]\ndistance = 0",
    );
    format!(
        "{text}\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[0, 2]]\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1]\nin_place = true\n"
    )
}

#[test]
fn an_area_doctrine_reads_a_force_where_the_area_command_holds_it() {
    // Up never arrives: the area command holds task force 1 as of the start
    // and reads her there, in every later reason.
    let (scenario, handed) = play(&deny_text(NEVER));
    let view = scenario.world.side_view(Side::Allied);
    assert!(view.reports_travel());
    let known = view.known_to(CommandRef::Area);
    let at = |v: &sim_core::view::SideView<'_>| {
        v.task_forces()
            .find(|f| f.id == force(1))
            .map(|f| f.position())
            .expect("the force")
    };
    // The force has been moved by her own task; the area command holds her
    // where she began.
    assert_ne!(at(&view), at(&known.view()));
    assert_eq!(
        known.view().held_as_of(CommandRef::TaskForce(force(1))),
        Some(sim_core::held::AsOf::Start)
    );
    // The reasons of the area's forming rule say where the state is of.
    let said: Vec<String> = handed
        .iter()
        .flat_map(|d| d.commands.iter().map(|c| c.reason.worked.clone()))
        .chain(
            handed
                .iter()
                .flat_map(|d| d.no_orders.iter().map(|r| r.worked.clone())),
        )
        .collect();
    assert!(
        said.iter().any(|w| w.contains(
            "task_force:1 (as of the start) is on the carrier mission with 1 carrier(s) afloat and 3 air group(s) aboard, and lies at (0,0)"
        )),
        "{said:#?}"
    );
}

#[test]
fn with_every_report_on_time_the_area_doctrine_gives_what_command_levels_off_gives() {
    let on = play(&deny_text(ON_TIME)).1;
    let text = deny_text(ON_TIME);
    let off_text = {
        let t = once(&text, LEVELS_LINE, "");
        let at = t.find("reports = \"").expect("a reports line");
        let end = at + t[at..].find('\n').expect("a line") + 1;
        format!("{}{}", &t[..at], &t[end..])
    };
    let off = play(&off_text).1;
    assert_eq!(on.len(), off.len());
    let mut formed = 0;
    for (a, b) in on.iter().zip(&off) {
        // What the area forms, with its reasons, and its notices: the words
        // "as of" never appear.
        assert_eq!(a.commands, b.commands);
        assert_eq!(a.notices, b.notices);
        formed += a
            .commands
            .iter()
            .filter(|c| matches!(c.kind, CommandKind::FormIntent { .. }))
            .count();
        for c in &a.commands {
            assert!(!c.reason.worked.contains("as of"));
        }
    }
    assert!(formed > 0, "the area formed a task");
}

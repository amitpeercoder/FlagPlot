//! What the four tests of the games with submarines share (decisions 0225 and
//! 0227): the statements, each written once, run on the table of a game.
//!
//! A game's test file holds its table of boats (name, side, number, class,
//! hex, home, the day she enters and her leave day), the statements as the
//! architect wrote them in the decision, and one test for each statement that
//! calls the function here. The hexes, the days and the bases are statements
//! of the decision, not numbers that rest on a figure: no statement holds a
//! number that rests on a figure, and a class's torpedo and salvoes are
//! compared with the class file the copy loads, not with a number.

use std::path::PathBuf;

use sim_core::command::{SubmarineDoctrine, SubmarinePattern};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, SubmarineClassId, SubmarineId};
use sim_core::state::{Side, SubmarineCondition, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use super::{data_dir, load_live};

/// A boat as the statements give her.
pub struct Boat {
    pub id: u32,
    pub name: &'static str,
    pub side: Side,
    pub class: u32,
    /// The hex she starts in, which is her station.
    pub hex: (i32, i32),
    /// The base she goes home to, and its name in the game.
    pub home: u32,
    pub home_name: &'static str,
    /// The day she enters, or none (on the map from day 1).
    pub enters: Option<u32>,
    /// The day she leaves for home, or none.
    pub leave: Option<u32>,
}

/// A game with submarines: its four copies and its table of boats.
pub struct Game {
    /// For each of the four letters: the letter, the copy's file name in
    /// `data/scenarios/table-boats/` and the file it copies, as a path under
    /// `data/scenarios/`.
    pub copies: [(char, &'static str, &'static str); 4],
    pub boats: &'static [Boat],
    /// The number of files the script says are as made, for the whole
    /// folder: sixteen.
    pub script_files: usize,
}

/// The keys of the submarine files.
const KEYS: [&str; 3] = [
    "submarine_classes = \"../../reference/submarine_classes_1942.csv\"",
    "submarine_torpedoes = \"../../reference/submarine_torpedoes_1942.csv\"",
    "submarines = \"../../reference/submarines.toml\"",
];

/// Where the section of boats begins: the blank line before it is the last
/// line of the file it copies.
const SECTION: &str = "\n\n# SUBMARINES (decisions 0225 and 0227)";

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

impl Game {
    fn copy(&self, letter: char) -> PathBuf {
        let name = self
            .copies
            .iter()
            .find(|c| c.0 == letter)
            .unwrap_or_else(|| panic!("no copy {letter}"))
            .1;
        data_dir().join("scenarios/table-boats").join(name)
    }

    fn source(&self, letter: char) -> PathBuf {
        let path = self
            .copies
            .iter()
            .find(|c| c.0 == letter)
            .unwrap_or_else(|| panic!("no copy {letter}"))
            .2;
        data_dir().join("scenarios").join(path)
    }

    pub fn letters(&self) -> Vec<char> {
        self.copies.iter().map(|c| c.0).collect()
    }

    /// Loaded on the live figures, as the table plays it.
    pub fn load(&self, letter: char) -> Scenario {
        load_live(&self.copy(letter)).unwrap_or_else(|e| panic!("copy {letter} loads: {e}"))
    }

    // Statement 1.
    pub fn statement_1(&self) {
        for letter in self.letters() {
            let scenario = self.load(letter);
            let truth = scenario.world.umpire();
            let have: Vec<(u32, String, Side)> = truth
                .submarines
                .values()
                .map(|b| (b.id.raw(), b.name.clone(), b.side))
                .collect();
            let want: Vec<(u32, String, Side)> = self
                .boats
                .iter()
                .map(|b| (b.id, b.name.to_string(), b.side))
                .collect();
            assert_eq!(have, want, "copy {letter}");
        }
    }

    // Statement 2.
    pub fn statement_2(&self) {
        for letter in self.letters() {
            let scenario = self.load(letter);
            let truth = scenario.world.umpire();
            for b in self.boats {
                let boat = &truth.submarines[&SubmarineId::new(b.id)];
                let start = Hex::new(b.hex.0, b.hex.1);
                let name = b.name;
                assert_eq!(boat.hex, start, "copy {letter}: {name}");
                assert_eq!(boat.station, start, "copy {letter}: {name}");
                assert!(boat.passage.is_none(), "copy {letter}: {name}");
                assert!(
                    boat.on_station(b.enters.unwrap_or(1)),
                    "copy {letter}: {name} on station on her day"
                );
                assert_eq!(boat.enters_day, b.enters, "copy {letter}: {name} enters");
                assert_eq!(boat.leave_day, b.leave, "copy {letter}: {name} leave day");
                assert!(
                    scenario.world.map().is_navigable(start),
                    "copy {letter}: {name} at {start:?}, a hex ships can enter"
                );
            }
        }
    }

    // Statement 3.
    pub fn statement_3(&self) {
        for letter in self.letters() {
            let scenario = self.load(letter);
            let data = scenario
                .world
                .ref_data()
                .submarines
                .as_ref()
                .expect("the submarine files");
            let truth = scenario.world.umpire();
            for b in self.boats {
                let boat = &truth.submarines[&SubmarineId::new(b.id)];
                let name = b.name;
                assert_eq!(
                    boat.class,
                    SubmarineClassId::new(b.class),
                    "copy {letter}: {name}"
                );
                let of_class = &data.classes[&boat.class];
                assert!(
                    data.torpedoes.contains_key(&of_class.torpedo),
                    "copy {letter}: {name}'s class names a torpedo the files give"
                );
                assert_eq!(
                    boat.salvoes, of_class.salvoes_carried,
                    "copy {letter}: {name} has her class's salvoes"
                );
                assert!(boat.salvoes > 0, "copy {letter}: {name}");
            }
        }
    }

    // Statement 4.
    pub fn statement_4(&self) {
        for letter in self.letters() {
            let scenario = self.load(letter);
            let truth = scenario.world.umpire();
            for b in self.boats {
                let boat = &truth.submarines[&SubmarineId::new(b.id)];
                assert_eq!(boat.home, BaseId::new(b.home), "copy {letter}: {}", b.name);
                assert_eq!(
                    truth.bases[&BaseId::new(b.home)].name,
                    b.home_name,
                    "copy {letter}: {}'s home",
                    b.name
                );
            }
        }
    }

    // Statement 5.
    pub fn statement_5(&self) {
        for letter in self.letters() {
            let scenario = self.load(letter);
            assert!(
                scenario.world.umpire().waters_waiting.is_empty(),
                "copy {letter}"
            );
            for side in [Side::Allied, Side::Japanese] {
                assert!(
                    scenario.world.side_view(side).suspected_waters().is_empty(),
                    "copy {letter}: {side:?}"
                );
            }
            let text = read(&self.copy(letter));
            assert!(
                !text.lines().any(|l| l.trim() == "[[suspected_waters]]"),
                "copy {letter}"
            );
        }
    }

    // Statement 6.
    pub fn statement_6(&self) {
        for letter in self.letters() {
            let scenario = self.load(letter);
            assert_eq!(
                scenario.world.side_view(Side::Allied).submarine_doctrine(),
                SubmarineDoctrine {
                    pattern: Some(SubmarinePattern::OffBases),
                    expects_boats_off_own_bases: true,
                },
                "copy {letter}: the Allied side"
            );
            assert_eq!(
                scenario
                    .world
                    .side_view(Side::Japanese)
                    .submarine_doctrine(),
                SubmarineDoctrine {
                    pattern: Some(SubmarinePattern::Lines),
                    expects_boats_off_own_bases: false,
                },
                "copy {letter}: the Japanese side"
            );
        }
    }

    // Statement 7.
    pub fn statement_7(&self) {
        for letter in self.letters() {
            let text = read(&self.copy(letter));
            let at = text.find(SECTION).expect("the section of boats");
            let without = &text[..at + 1];
            let section = &text[at + 1..];
            assert_eq!(
                section.matches("\n[[submarine]]\n").count(),
                self.boats.len(),
                "copy {letter}: the section holds the boats"
            );
            assert_eq!(
                section.matches("\n[side.allied]\n").count(),
                1,
                "copy {letter}: the section holds the Allied [side] table"
            );
            assert_eq!(
                section.matches("\n[side.japanese]\n").count(),
                1,
                "copy {letter}: the section holds the Japanese [side] table"
            );
            assert!(
                section.find("[side.japanese]").unwrap() < section.find("[[submarine]]").unwrap(),
                "copy {letter}: the [side] tables open the section"
            );
            assert_eq!(
                without.matches("submarine]]").count(),
                0,
                "copy {letter}: no boat before the section"
            );
            assert!(
                !body(without).iter().any(|l| l.starts_with("[side")),
                "copy {letter}: no [side] table before the section"
            );
            let mut lines = body(without);
            for key in KEYS {
                let found: Vec<usize> = lines
                    .iter()
                    .enumerate()
                    .filter(|(_, l)| **l == key)
                    .map(|(i, _)| i)
                    .collect();
                assert_eq!(found.len(), 1, "copy {letter}: {key}");
                lines.remove(found[0]);
            }
            let original = read(&self.source(letter));
            let mut want = body(&original);
            if letter == 'a' {
                // File a's copy is on the table's ground figures, as the
                // copies of b, c and d come from files that are.
                let i = want
                    .iter()
                    .position(|l| *l == "ground = \"../../reference/ground.toml\"")
                    .expect("file a names the live ground figures");
                want[i] = "ground = \"../../reference/ground_hunger.toml\"";
            } else {
                assert!(
                    want.contains(&"ground = \"../../reference/ground_hunger.toml\""),
                    "the table's copy {letter} names the table's ground figures"
                );
            }
            assert_eq!(lines, want, "copy {letter}");
        }
    }

    fn played_on_seed_1(&self, letter: char) -> TruthState {
        let mut scenario = self.load(letter).with_seed(1).expect("seed 1");
        let held = staffs();
        while scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .is_some()
        {}
        scenario.world.umpire().clone()
    }

    /// Statement 8 on the copies of `letters`: played on seed 1 with no
    /// order to any boat, every boat on the map at the end that is not going
    /// home is in the hex she started in.
    pub fn statement_8(&self, letters: &[char]) {
        for &letter in letters {
            let end = self.played_on_seed_1(letter);
            for b in self.boats {
                let boat = &end.submarines[&SubmarineId::new(b.id)];
                let going_home = boat.passage.is_some_and(|p| p.home);
                if boat.condition == SubmarineCondition::Sunk
                    || !boat.on_map(u32::MAX)
                    || going_home
                {
                    continue;
                }
                assert_eq!(
                    boat.hex,
                    Hex::new(b.hex.0, b.hex.1),
                    "copy {letter}: {}, {:?}",
                    b.name,
                    boat.condition
                );
            }
        }
    }

    /// The script, run with `--check`: every file kept is the one it makes
    /// and the folder holds nothing else.
    pub fn the_script_says_each_copy_is_the_one_it_makes(&self) {
        let out = std::process::Command::new("python3")
            .arg(repo().join("scripts/table-boats/make_files.py"))
            .arg(repo())
            .arg("--check")
            .output()
            .expect("python3 runs the script");
        let said = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "{said}{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(
            said.matches(": as made").count(),
            self.script_files,
            "{said}"
        );
        assert_eq!(
            said.matches("not a file this script makes").count(),
            0,
            "{said}"
        );
    }
}

fn read(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// A file without its header: the comment lines (and blank lines) before
/// its first key.
fn body(text: &str) -> Vec<&str> {
    let lines: Vec<&str> = text.lines().collect();
    let first = lines
        .iter()
        .position(|l| {
            let l = l.trim_start();
            !l.is_empty() && !l.starts_with('#')
        })
        .expect("a key");
    lines[first..].to_vec()
}

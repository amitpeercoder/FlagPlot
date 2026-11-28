//! The games of Midway with submarines: decision 0225, as amended on review on Nov 23, 2026, and decision 0227, as amended on review the same day.
//!
//! Four copies of the game in `data/scenarios/table-boats/`, each built on
//! (decision 0234, step D) its source, which was copied by the script
//! `scripts/table-boats/make_files.py` until then: of the table's copies of
//! files b, c and d (`data/scenarios/table-hunger/`), the three copies being
//! what the table plays, and of file a on the table's ground figures, kept for the
//! measure of decision 0227, stage 3, and not offered. Each has its boats on
//! station, each from a row by the rules of the decision ("The rules of
//! placing"; for Midway, decision 0225's amended rules 2 and 3 too). The
//! hexes are worked by hand in the decision ("The hexes, worked by hand").
//!
//! The copies play on the live figures, as the table plays them: no
//! statement here holds a number that rests on a figure (a hex, a class id,
//! a base, a count of boats, a day, the text of a file), and the class's
//! torpedo and salvoes are compared with the class file the copy loads, not
//! with a number.
//!
//! # The statements the test holds
//!
//! The statements are the architect's, written before the code (decision
//! 0227, "The statements each game's test holds"), and are kept here as
//! written. A mismatch goes back to the architect, and nobody else changes
//! a statement here.
//!
//! Each copy, loaded:
//!
//! 1. It has the submarines of its table above, by name, side and number,
//!    and no other.
//! 2. Each starts in her hex, on station there, in a hex ships can enter;
//!    each enters on her day and has her leave day, or none.
//! 3. Each is of her class, with her class's torpedo and salvoes as the
//!    class file loaded gives them.
//! 4. Each one's home is the base rule 6 names.
//! 5. It has no suspected waters.
//! 6. The Allied side's pattern is off bases and it expects boats off its
//!    own bases; the Japanese side's is lines and it does not.
//! 7. Taken with its submarines, their three keys and its two `[side]`
//!    tables removed, it is the game it copies, line for line but for its
//!    header, and for file a its ground figures.
//! 8. Played on seed 1 with no order to any boat, every boat on the map at
//!    the end that is not going home is in the hex she started in.
//!
//! Since step D of decision 0234 each file is built on the file it copies and
//! the script that made them is retired: statement 7 is read as tables, the
//! copy resolved against its source resolved, and the script's check is
//! withdrawn (reading (d)). No statement holds a number that rests on a
//! figure.

use crate::common;

use common::boats_in_a_game::{Boat, Game};
use sim_core::state::Side;

/// The boats, in the order of the table in the decision.
const BOATS: [Boat; 15] = [
    Boat {
        id: 1,
        name: "USS Tambor",
        side: Side::Allied,
        class: 6001,
        hex: (1, 3),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 2,
        name: "USS Trout",
        side: Side::Allied,
        class: 6001,
        hex: (1, 1),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 3,
        name: "USS Nautilus",
        side: Side::Allied,
        class: 6011,
        hex: (4, -2),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 4,
        name: "USS Gato",
        side: Side::Allied,
        class: 6002,
        hex: (1, -1),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 5,
        name: "I-168",
        side: Side::Japanese,
        class: 6005,
        hex: (5, 1),
        home: 2,
        home_name: "Wake",
        enters: None,
        leave: None,
    },
    Boat {
        id: 6,
        name: "USS Grayling",
        side: Side::Allied,
        class: 6001,
        hex: (2, -1),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 7,
        name: "USS Grouper",
        side: Side::Allied,
        class: 6002,
        hex: (6, -4),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 8,
        name: "USS Gudgeon",
        side: Side::Allied,
        class: 6001,
        hex: (7, -4),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 9,
        name: "USS Grenadier",
        side: Side::Allied,
        class: 6001,
        hex: (3, -3),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 10,
        name: "USS Flying Fish",
        side: Side::Allied,
        class: 6002,
        hex: (5, -1),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 11,
        name: "USS Narwhal",
        side: Side::Allied,
        class: 6011,
        hex: (18, -3),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 12,
        name: "USS Trigger",
        side: Side::Allied,
        class: 6002,
        hex: (16, -2),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 13,
        name: "USS Dolphin",
        side: Side::Allied,
        class: 6012,
        hex: (0, 2),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 14,
        name: "USS Cachalot",
        side: Side::Allied,
        class: 6013,
        hex: (6, -1),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
    Boat {
        id: 15,
        name: "USS Cuttlefish",
        side: Side::Allied,
        class: 6013,
        hex: (-13, 3),
        home: 1,
        home_name: "Midway",
        enters: None,
        leave: None,
    },
];

const GAME: Game = Game {
    copies: [
        ('a', "01a_midway_submarines.toml", "midway/01a_midway.toml"),
        (
            'b',
            "01b_midway_submarines.toml",
            "table-hunger/01b_midway.toml",
        ),
        (
            'c',
            "01c_midway_submarines.toml",
            "table-hunger/01c_midway.toml",
        ),
        (
            'd',
            "01d_midway_submarines.toml",
            "table-hunger/01d_midway.toml",
        ),
    ],
    boats: &BOATS,
};

fn game() -> &'static Game {
    &GAME
}

#[test]
fn statement_1_the_submarines_of_its_table_and_no_other() {
    game().statement_1();
}

#[test]
fn statement_2_they_start_on_station_enter_on_their_day_and_leave_on_theirs() {
    game().statement_2();
}

#[test]
fn statement_3_each_of_her_class_with_its_torpedo_and_salvoes() {
    game().statement_3();
}

#[test]
fn statement_4_each_ones_home_is_the_base_the_rule_names() {
    game().statement_4();
}

#[test]
fn statement_5_no_suspected_waters() {
    game().statement_5();
}

#[test]
fn statement_6_the_two_sides_patterns() {
    game().statement_6();
}

#[test]
fn statement_7_without_its_boats_it_is_the_game_it_copies() {
    game().statement_7();
}

#[test]
fn statement_8_played_on_seed_1_no_boat_on_the_map_has_moved() {
    game().statement_8(&game().letters());
}

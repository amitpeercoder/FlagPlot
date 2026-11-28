//! Every scenario test, as one test program (see `Cargo.toml`). Each file is a
//! module; the files are unchanged but for how they reach `common`.

// Some scenario files load the agent's schema validator from its file, each as
// a module of its own, as each was its own test program before.
#![allow(clippy::duplicate_mod)]

mod common;

#[path = "boats_in_the_coral_sea_games.rs"]
mod boats_in_the_coral_sea_games;
#[path = "boats_in_the_five_days_games.rs"]
mod boats_in_the_five_days_games;
#[path = "boats_in_the_midway_games.rs"]
mod boats_in_the_midway_games;
#[path = "boats_in_the_thirty_nine_days_games.rs"]
mod boats_in_the_thirty_nine_days_games;
#[path = "choices_asked_and_run.rs"]
mod choices_asked_and_run;
#[path = "command_levels_ashore.rs"]
mod command_levels_ashore;
#[path = "day_in_parts.rs"]
mod day_in_parts;
#[path = "estimates_never_evidence.rs"]
mod estimates_never_evidence;
#[path = "five_days_first_run.rs"]
mod five_days_first_run;
#[path = "frozen_figures_mechanism.rs"]
mod frozen_figures_mechanism;
#[path = "proof_0159_orders.rs"]
mod proof_0159_orders;
#[path = "proof_stage_4_case_r_preparation.rs"]
mod proof_stage_4_case_r_preparation;
#[path = "proof_stage_4_case_w1_the_garrison_believed_half.rs"]
mod proof_stage_4_case_w1_the_garrison_believed_half;
#[path = "proof_stage_4_case_w2_a_carrier_believed_absent.rs"]
mod proof_stage_4_case_w2_a_carrier_believed_absent;
#[path = "proof_stage_4_case_w3_an_intent_not_as_stated.rs"]
mod proof_stage_4_case_w3_an_intent_not_as_stated;
#[path = "proof_stage_4_cases_s1_and_s2.rs"]
mod proof_stage_4_cases_s1_and_s2;
#[path = "proof_stage_4_harness.rs"]
mod proof_stage_4_harness;
#[path = "scenario_01_transit.rs"]
mod scenario_01_transit;
#[path = "scenario_01b_refuel.rs"]
mod scenario_01b_refuel;
#[path = "scenario_02_supply_run.rs"]
mod scenario_02_supply_run;
#[path = "scenario_02b_fuel_run.rs"]
mod scenario_02b_fuel_run;
#[path = "scenario_03_search.rs"]
mod scenario_03_search;
#[path = "scenario_04_experience_values.rs"]
mod scenario_04_experience_values;
#[path = "scenario_04_savo_island.rs"]
mod scenario_04_savo_island;
#[path = "scenario_04a2_stragglers.rs"]
mod scenario_04a2_stragglers;
#[path = "scenario_04a_night_action.rs"]
mod scenario_04a_night_action;
#[path = "scenario_04b_pathfinding.rs"]
mod scenario_04b_pathfinding;
#[path = "scenario_04c_meeting.rs"]
mod scenario_04c_meeting;
#[path = "scenario_04c_readiness.rs"]
mod scenario_04c_readiness;
#[path = "scenario_05_eastern_solomons.rs"]
mod scenario_05_eastern_solomons;
#[path = "scenario_05a_carrier_search.rs"]
mod scenario_05a_carrier_search;
#[path = "scenario_05b_strike.rs"]
mod scenario_05b_strike;
#[path = "scenario_05c_air_combat.rs"]
mod scenario_05c_air_combat;
#[path = "scenario_05d_carrier_duel.rs"]
mod scenario_05d_carrier_duel;
#[path = "scenario_06_rabaul_guadalcanal.rs"]
mod scenario_06_rabaul_guadalcanal;
#[path = "scenario_06a_base_raid.rs"]
mod scenario_06a_base_raid;
#[path = "scenario_06b_wear_and_repair.rs"]
mod scenario_06b_wear_and_repair;
#[path = "scenario_06c2_tired_lookouts.rs"]
mod scenario_06c2_tired_lookouts;
#[path = "scenario_06c_fatigue_and_rest.rs"]
mod scenario_06c_fatigue_and_rest;
#[path = "scenario_06d1_pilots_and_replacements.rs"]
mod scenario_06d1_pilots_and_replacements;
#[path = "scenario_06d2_base_supply.rs"]
mod scenario_06d2_base_supply;
#[path = "scenario_07_determinism.rs"]
mod scenario_07_determinism;
#[path = "scenario_08_soak.rs"]
mod scenario_08_soak;
#[path = "scenario_09a_side_events.rs"]
mod scenario_09a_side_events;
#[path = "scenario_100_what_ifs_from_a_seat.rs"]
mod scenario_100_what_ifs_from_a_seat;
#[path = "scenario_101_a_game_watched.rs"]
mod scenario_101_a_game_watched;
#[path = "scenario_102_the_enemys_objectives_shown.rs"]
mod scenario_102_the_enemys_objectives_shown;
#[path = "scenario_103_the_eastern_solomons_over_five_days.rs"]
mod scenario_103_the_eastern_solomons_over_five_days;
#[path = "scenario_104_men_put_ashore.rs"]
mod scenario_104_men_put_ashore;
#[path = "scenario_105_the_coral_sea.rs"]
mod scenario_105_the_coral_sea;
#[path = "scenario_106_what_to_strike_first.rs"]
mod scenario_106_what_to_strike_first;
#[path = "scenario_107_a_lift_turned_back.rs"]
mod scenario_107_a_lift_turned_back;
#[path = "scenario_108_a_station_kept_from_the_sea.rs"]
mod scenario_108_a_station_kept_from_the_sea;
#[path = "scenario_109_men_ashore_with_no_more_coming.rs"]
mod scenario_109_men_ashore_with_no_more_coming;
#[path = "scenario_10_objectives.rs"]
mod scenario_10_objectives;
#[path = "scenario_10b_objectives.rs"]
mod scenario_10b_objectives;
#[path = "scenario_110_the_last_day_ashore.rs"]
mod scenario_110_the_last_day_ashore;
#[path = "scenario_111_forces_that_arrive.rs"]
mod scenario_111_forces_that_arrive;
#[path = "scenario_112_stock_from_off_the_map.rs"]
mod scenario_112_stock_from_off_the_map;
#[path = "scenario_113_men_who_replace_the_lost.rs"]
mod scenario_113_men_who_replace_the_lost;
#[path = "scenario_114_midway.rs"]
mod scenario_114_midway;
#[path = "scenario_115_refuelling_at_sea.rs"]
mod scenario_115_refuelling_at_sea;
#[path = "scenario_116_carriers_serve_an_end.rs"]
mod scenario_116_carriers_serve_an_end;
#[path = "scenario_117_the_way_a_force_takes.rs"]
mod scenario_117_the_way_a_force_takes;
#[path = "scenario_118_a_dry_run_inside_a_day.rs"]
mod scenario_118_a_dry_run_inside_a_day;
#[path = "scenario_119_hunger_takes_time.rs"]
mod scenario_119_hunger_takes_time;
#[path = "scenario_11_claims.rs"]
mod scenario_11_claims;
#[path = "scenario_120_real_oilers.rs"]
mod scenario_120_real_oilers;
#[path = "scenario_121_a_japanese_oiler_by_estimate.rs"]
mod scenario_121_a_japanese_oiler_by_estimate;
#[path = "scenario_122_hungry_men_are_slow_to_mend.rs"]
mod scenario_122_hungry_men_are_slow_to_mend;
#[path = "scenario_123_a_fight_costs_supply.rs"]
mod scenario_123_a_fight_costs_supply;
#[path = "scenario_124_the_type_claimed_shot_down.rs"]
mod scenario_124_the_type_claimed_shot_down;
#[path = "scenario_125_a_force_asks_leave_to_pursue.rs"]
mod scenario_125_a_force_asks_leave_to_pursue;
#[path = "scenario_126_fire_among_aircraft.rs"]
mod scenario_126_fire_among_aircraft;
#[path = "scenario_127_a_strike_is_spread.rs"]
mod scenario_127_a_strike_is_spread;
#[path = "scenario_128_submarines_against_warships.rs"]
mod scenario_128_submarines_against_warships;
#[path = "scenario_129_submarines_the_staff_and_the_patrol.rs"]
mod scenario_129_submarines_the_staff_and_the_patrol;
#[path = "scenario_12a_long_reach.rs"]
mod scenario_12a_long_reach;
#[path = "scenario_12b_patrol_pilots.rs"]
mod scenario_12b_patrol_pilots;
#[path = "scenario_12c_no_deck.rs"]
mod scenario_12c_no_deck;
#[path = "scenario_12d_flight_300.rs"]
mod scenario_12d_flight_300;
#[path = "scenario_130_a_limit_binds_what_it_names.rs"]
mod scenario_130_a_limit_binds_what_it_names;
#[path = "scenario_131_the_pattern_of_a_patrol.rs"]
mod scenario_131_the_pattern_of_a_patrol;
#[path = "scenario_132_a_source_the_staff_chooses.rs"]
mod scenario_132_a_source_the_staff_chooses;
#[path = "scenario_133_a_force_followed.rs"]
mod scenario_133_a_force_followed;
#[path = "scenario_134_the_estimate_of_boats.rs"]
mod scenario_134_the_estimate_of_boats;
#[path = "scenario_135_men_beside_a_place.rs"]
mod scenario_135_men_beside_a_place;
#[path = "scenario_136_supply_at_press.rs"]
mod scenario_136_supply_at_press;
#[path = "scenario_137_a_plan_the_commander_writes.rs"]
mod scenario_137_a_plan_the_commander_writes;
#[path = "scenario_137b_the_draft_as_one.rs"]
mod scenario_137b_the_draft_as_one;
#[path = "scenario_138_one_ship.rs"]
mod scenario_138_one_ship;
#[path = "scenario_139_two_mouths.rs"]
mod scenario_139_two_mouths;
#[path = "scenario_13a_target_priority.rs"]
mod scenario_13a_target_priority;
#[path = "scenario_13b_afternoon_base_strike.rs"]
mod scenario_13b_afternoon_base_strike;
#[path = "scenario_140_men_then_food.rs"]
mod scenario_140_men_then_food;
#[path = "scenario_14a_experience.rs"]
mod scenario_14a_experience;
#[path = "scenario_15a_unready_fire.rs"]
mod scenario_15a_unready_fire;
#[path = "scenario_15b_unready_torpedoes.rs"]
mod scenario_15b_unready_torpedoes;
#[path = "scenario_16_all_together.rs"]
mod scenario_16_all_together;
#[path = "scenario_17_cape_esperance.rs"]
mod scenario_17_cape_esperance;
#[path = "scenario_17a_target_choice.rs"]
mod scenario_17a_target_choice;
#[path = "scenario_18_torpedo_sides.rs"]
mod scenario_18_torpedo_sides;
#[path = "scenario_19a_guns_and_the_attack.rs"]
mod scenario_19a_guns_and_the_attack;
#[path = "scenario_19b_whom_the_patrol_fires_at.rs"]
mod scenario_19b_whom_the_patrol_fires_at;
#[path = "scenario_19c_patrol_after_the_attack.rs"]
mod scenario_19c_patrol_after_the_attack;
#[path = "scenario_20a_ship_repair.rs"]
mod scenario_20a_ship_repair;
#[path = "scenario_20b_out_of_action.rs"]
mod scenario_20b_out_of_action;
#[path = "scenario_21_landing.rs"]
mod scenario_21_landing;
#[path = "scenario_22_rebase.rs"]
mod scenario_22_rebase;
#[path = "scenario_23_fight_ashore.rs"]
mod scenario_23_fight_ashore;
#[path = "scenario_24_shells_and_estimates.rs"]
mod scenario_24_shells_and_estimates;
#[path = "scenario_25_march.rs"]
mod scenario_25_march;
#[path = "scenario_26_act_and_steam_on.rs"]
mod scenario_26_act_and_steam_on;
#[path = "scenario_27_preparing_a_landing.rs"]
mod scenario_27_preparing_a_landing;
#[path = "scenario_28_experience_and_training.rs"]
mod scenario_28_experience_and_training;
#[path = "scenario_29_skills.rs"]
mod scenario_29_skills;
#[path = "scenario_30_working_units.rs"]
mod scenario_30_working_units;
#[path = "scenario_31_airfield_damage.rs"]
mod scenario_31_airfield_damage;
#[path = "scenario_32_falling_back.rs"]
mod scenario_32_falling_back;
#[path = "scenario_33_engaged_and_failed.rs"]
mod scenario_33_engaged_and_failed;
#[path = "scenario_34_cover.rs"]
mod scenario_34_cover;
#[path = "scenario_35_days_of_supply.rs"]
mod scenario_35_days_of_supply;
#[path = "scenario_36_one_convoy.rs"]
mod scenario_36_one_convoy;
#[path = "scenario_37_men_falling_back.rs"]
mod scenario_37_men_falling_back;
#[path = "scenario_38_an_empty_hex.rs"]
mod scenario_38_an_empty_hex;
#[path = "scenario_39_load_reserve_and_forces.rs"]
mod scenario_39_load_reserve_and_forces;
#[path = "scenario_40_one_pool_two_hexes.rs"]
mod scenario_40_one_pool_two_hexes;
#[path = "scenario_41_hold_change_release.rs"]
mod scenario_41_hold_change_release;
#[path = "scenario_42_interrupts_and_runs.rs"]
mod scenario_42_interrupts_and_runs;
#[path = "scenario_44_fuelling_takes_time.rs"]
mod scenario_44_fuelling_takes_time;
#[path = "scenario_45_soak_ashore.rs"]
mod scenario_45_soak_ashore;
#[path = "scenario_46_two_strike_settings.rs"]
mod scenario_46_two_strike_settings;
#[path = "scenario_47_cover_from_the_air.rs"]
mod scenario_47_cover_from_the_air;
#[path = "scenario_48_hold_a_place.rs"]
mod scenario_48_hold_a_place;
#[path = "scenario_49_a_strike_sized_to_its_target.rs"]
mod scenario_49_a_strike_sized_to_its_target;
#[path = "scenario_50_neutralise_a_base.rs"]
mod scenario_50_neutralise_a_base;
#[path = "scenario_51_an_escort_force.rs"]
mod scenario_51_an_escort_force;
#[path = "scenario_52_rearming.rs"]
mod scenario_52_rearming;
#[path = "scenario_53_deny_an_area.rs"]
mod scenario_53_deny_an_area;
#[path = "scenario_54_a_force_that_stands_by_night.rs"]
mod scenario_54_a_force_that_stands_by_night;
#[path = "scenario_55_a_carrier_force_from_an_end.rs"]
mod scenario_55_a_carrier_force_from_an_end;
#[path = "scenario_56_an_escorted_run.rs"]
mod scenario_56_an_escorted_run;
#[path = "scenario_57_going_back_to_rearm.rs"]
mod scenario_57_going_back_to_rearm;
#[path = "scenario_58_a_carrier_force_closes.rs"]
mod scenario_58_a_carrier_force_closes;
#[path = "scenario_59_a_search_kept_up.rs"]
mod scenario_59_a_search_kept_up;
#[path = "scenario_60_two_sightings_in_one_pulse.rs"]
mod scenario_60_two_sightings_in_one_pulse;
#[path = "scenario_61_commands_and_their_pictures.rs"]
mod scenario_61_commands_and_their_pictures;
#[path = "scenario_62_reports_between_commands.rs"]
mod scenario_62_reports_between_commands;
#[path = "scenario_63_a_superiors_knowledge.rs"]
mod scenario_63_a_superiors_knowledge;
#[path = "scenario_64_commanders_and_initiative.rs"]
mod scenario_64_commanders_and_initiative;
#[path = "scenario_65_a_day_from_which_a_force_may_be_used.rs"]
mod scenario_65_a_day_from_which_a_force_may_be_used;
#[path = "scenario_66_units_made_ready.rs"]
mod scenario_66_units_made_ready;
#[path = "scenario_67_taking_a_place.rs"]
mod scenario_67_taking_a_place;
#[path = "scenario_68_ashore.rs"]
mod scenario_68_ashore;
#[path = "scenario_69_taken_and_held.rs"]
mod scenario_69_taken_and_held;
#[path = "scenario_70_a_run_by_night.rs"]
mod scenario_70_a_run_by_night;
#[path = "scenario_71_bombardment_from_the_sea.rs"]
mod scenario_71_bombardment_from_the_sea;
#[path = "scenario_72_reconnaissance.rs"]
mod scenario_72_reconnaissance;
#[path = "scenario_73_carrier_cover.rs"]
mod scenario_73_carrier_cover;
#[path = "scenario_74_the_force_assembled.rs"]
mod scenario_74_the_force_assembled;
#[path = "scenario_76_controlling_an_island.rs"]
mod scenario_76_controlling_an_island;
#[path = "scenario_77_forces_with_men_aboard.rs"]
mod scenario_77_forces_with_men_aboard;
#[path = "scenario_78_orders_inside_a_day.rs"]
mod scenario_78_orders_inside_a_day;
#[path = "scenario_79_fed_before_it_is_taken.rs"]
mod scenario_79_fed_before_it_is_taken;
#[path = "scenario_80_a_take_that_waits.rs"]
mod scenario_80_a_take_that_waits;
#[path = "scenario_81_told_to_wait.rs"]
mod scenario_81_told_to_wait;
#[path = "scenario_82_how_much_must_be_known.rs"]
mod scenario_82_how_much_must_be_known;
#[path = "scenario_83_a_force_moved_to_look.rs"]
mod scenario_83_a_force_moved_to_look;
#[path = "scenario_84_the_plan_as_an_order.rs"]
mod scenario_84_the_plan_as_an_order;
#[path = "scenario_85_a_nights_load_spread.rs"]
mod scenario_85_a_nights_load_spread;
#[path = "scenario_86_fed_with_the_lift.rs"]
mod scenario_86_fed_with_the_lift;
#[path = "scenario_87_the_order_of_a_staffs_take.rs"]
mod scenario_87_the_order_of_a_staffs_take;
#[path = "scenario_88_what_the_attacked_saw.rs"]
mod scenario_88_what_the_attacked_saw;
#[path = "scenario_89_a_raid_recognised_by_experience.rs"]
mod scenario_89_a_raid_recognised_by_experience;
#[path = "scenario_90_a_take_that_does_not_go_to_starve.rs"]
mod scenario_90_a_take_that_does_not_go_to_starve;
#[path = "scenario_91_a_world_from_a_picture.rs"]
mod scenario_91_a_world_from_a_picture;
#[path = "scenario_92_a_sides_own_half.rs"]
mod scenario_92_a_sides_own_half;
#[path = "scenario_93_the_enemys_half.rs"]
mod scenario_93_the_enemys_half;
#[path = "scenario_94_a_trial.rs"]
mod scenario_94_a_trial;
#[path = "scenario_95_the_estimate.rs"]
mod scenario_95_the_estimate;
#[path = "scenario_96_the_window.rs"]
mod scenario_96_the_window;
#[path = "scenario_97_a_seats_trial.rs"]
mod scenario_97_a_seats_trial;
#[path = "scenario_98_the_type_claimed_on_the_ground.rs"]
mod scenario_98_the_type_claimed_on_the_ground;
#[path = "scenario_99_a_groups_tally_of_claims.rs"]
mod scenario_99_a_groups_tally_of_claims;
#[path = "scenario_the_wide_sea.rs"]
mod scenario_the_wide_sea;
#[path = "scenarios_built_on_another.rs"]
mod scenarios_built_on_another;
#[path = "table_rear_bases_files.rs"]
mod table_rear_bases_files;

/// `autotests = false` means a file with no `mod` line here would never run, and
/// nothing would say so. This names every top-level `.rs` file of the `tests` package that
/// this file does not declare as a module.
/// Scenario 75 is exempt by name: the file holds a doc comment and no test.
#[test]
fn every_test_file_is_declared_as_a_module() {
    use std::collections::BTreeSet;
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = std::fs::read_to_string(dir.join("all.rs")).expect("the root file is read");
    let mut declared: BTreeSet<String> = BTreeSet::new();
    for line in root.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("#[path = \"") {
            if let Some(path) = rest.split('"').next() {
                declared.insert(path.rsplit('/').next().unwrap_or(path).to_string());
            }
        }
        let rest = line
            .strip_prefix("pub mod ")
            .or_else(|| line.strip_prefix("mod "));
        if let Some(name) = rest.and_then(|r| r.strip_suffix(';')) {
            declared.insert(format!("{name}.rs"));
        }
    }
    let mut missing: BTreeSet<String> = BTreeSet::new();
    for entry in std::fs::read_dir(dir).expect("the directory is read") {
        let name = entry
            .expect("an entry is read")
            .file_name()
            .to_string_lossy()
            .into_owned();
        if name.ends_with(".rs")
            && name != "all.rs"
            && !declared.contains(&name)
            && name != "scenario_75_leaving_a_place.rs"
        {
            missing.insert(name);
        }
    }
    assert!(
        missing.is_empty(),
        "not declared as a module in all.rs, so never run: {missing:?}"
    );
}

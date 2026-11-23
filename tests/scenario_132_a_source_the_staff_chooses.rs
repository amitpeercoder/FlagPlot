mod common;
use sim_core::state::Side;
use sim_data::LoadOptions;
use sim_staff::{as_staffs, staffs};

fn play(text: &str, days: u32) {
    let file = common::data_dir().join("scenarios/a-source-the-staff-chooses/132_a_source_the_staff_chooses.toml");
    let mut scenario = common::from_text_with(text, &file, &common::frozen()).expect("loads");
    let held = staffs();
    loop {
        let day = scenario.world.day() + 1;
        if day > days { break; }
        let s = scenario.next_day_watched(as_staffs(&held), &mut |_, _| {}).expect("day");
        if s.is_none() { break; }
        let d = scenario.world.staff_handed_in().get(&Side::Allied).cloned().unwrap_or_default();
        println!("=== DAY {day}");
        for c in &d.commands { println!("CMD {:?}", c.kind); }
        for o in &d.orders { println!("ORD {:?}", o.kind); }
        for o in &d.no_orders { println!("NOORD {}", &o.worked.chars().take(160).collect::<String>()); }
        for e in scenario.world.events().iter() { if let sim_core::events::EventKind::CommandRefused { reason, command, .. } = &e.kind { println!("REFUSED {:?} {:?}", reason, command); } }
    }
}

#[test]
fn probe() {
    let file = common::data_dir().join("scenarios/a-source-the-staff-chooses/132_a_source_the_staff_chooses.toml");
    let text = std::fs::read_to_string(file).unwrap();
    play(&text, 10);
}

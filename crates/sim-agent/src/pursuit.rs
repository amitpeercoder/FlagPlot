//! The asking of a force to go after what it has found, as a seat is shown it
//! at the stop in the evening (decision 0214, rules 6 to 9), and the leave to
//! pursue, the pursuits and the refusals as `command_state` shows them.
//!
//! Everything here is worked from a side's view and the staff's own answer:
//! nothing is stored, nothing of the enemy's is read, and the game is not
//! changed. The staff is asked on the view of the coming day, with the
//! commands the seat has queued taken in, as `staff_orders` asks it.

use serde_json::{json, Value};
use sim_core::command::{
    CommandKind, CommandState, Maker, NoticeKind, PursuitBy, Staff, StandingKind, StandingSubject,
};
use sim_core::hex::Hex;
use sim_core::ids::IntentId;
use sim_core::view::SideView;

use crate::staffing::{dec, Names};

fn hex_json(hex: Hex) -> Value {
    json!({"q": hex.q, "r": hex.r})
}

fn hex_words(hex: Hex) -> String {
    format!("({},{})", hex.q, hex.r)
}

/// The askings the staff would hand in for the coming day that would stop
/// the clock before it (decision 0214, rule 7): each notice that a force
/// asks leave to pursue, where the side has not switched the standing
/// interrupt off and the side's live record of it for the force is absent or
/// armed. `live` is the side's view of the game as it stands; `preview` is
/// its view of the coming day with its queued commands taken in, on which the
/// staff is asked. Empty where nothing would stop it.
pub(crate) fn askings(
    live: &SideView<'_>,
    preview: &SideView<'_>,
    staff: &dyn Staff,
) -> Vec<Value> {
    let coming = preview.command_state();
    if coming
        .standing_off
        .contains(&StandingKind::AsksLeaveToPursue)
    {
        return Vec::new();
    }
    // An asking names the intent the force would leave: with none in force
    // the staff has nothing to ask about.
    if coming.intents_in_force().next().is_none() {
        return Vec::new();
    }
    let record = live.command_state();
    let n = Names::new(preview);
    let decisions = staff.decide(*preview);
    let mut found = Vec::new();
    for notice in &decisions.notices {
        if notice.kind != NoticeKind::AsksLeaveToPursue {
            continue;
        }
        let Some(pursuit) = notice.pursuit else {
            continue;
        };
        let Some(find) = pursuit.find else {
            continue;
        };
        let armed = record
            .standing
            .get(&(
                StandingKind::AsksLeaveToPursue,
                StandingSubject::TaskForce(pursuit.force),
            ))
            .is_none_or(|status| status.armed);
        if !armed {
            continue;
        }
        let force = n.force(pursuit.force);
        let hours = find.hours.map(dec);
        let mut words = format!(
            "task force {force} asks leave to pursue contact {}, a {} reported at {}, to {}",
            find.contact,
            find.kind.name(),
            hex_words(find.reported_at),
            hex_words(find.to),
        );
        if let Some(hours) = find.hours {
            words.push_str(&format!(", {hours} hours"));
        }
        words.push_str(&format!("; intent {} would wait", pursuit.intent.raw()));
        found.push(json!({
            "task_force": force,
            "intent": pursuit.intent.raw(),
            "contact": find.contact,
            "kind": find.kind.name(),
            "age_pulses": find.age,
            "reported_at": hex_json(find.reported_at),
            "to": hex_json(find.to),
            "hours": hours,
            "interrupt": {"kind": "asks_leave_to_pursue", "task_force": force},
            "words": words,
        }));
    }
    found
}

/// The askings in one sentence's worth of words.
pub(crate) fn asking_words(askings: &[Value]) -> String {
    askings
        .iter()
        .filter_map(|asking| asking["words"].as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

/// The intent's leave to pursue as it is read up the tree: the setting, the
/// intent it is set on (`"default"` where none is set on the chain) and
/// whether one is set.
pub(crate) fn pursue_read_up_json(state: &CommandState, intent: IntentId) -> Value {
    let (leave, from) = state.pursue_read_up(intent);
    json!({
        "leave": leave.name(),
        "from": from.map_or_else(|| json!("default"), |id| json!(id.raw())),
        "set": from.is_some(),
    })
}

/// A force's pursuits, by answer or taken, and the commander's refusals
/// (decision 0214, rules 8 and 10), in the side's numbers.
pub(crate) fn pursuits_json(n: &Names<'_>, state: &CommandState) -> Value {
    json!({
        "pursuits": state
            .pursuits
            .iter()
            .map(|(force, pursuit)| {
                let by = match pursuit.by {
                    PursuitBy::Answer(_) => "answer",
                    PursuitBy::Taken => "taken",
                };
                json!({
                    "task_force": n.force(*force),
                    "by": by,
                    "contact": pursuit.contact,
                    "since_day": pursuit.since_day,
                })
            })
            .collect::<Vec<_>>(),
        "refusals": state
            .pursuit_refusals
            .iter()
            .map(|(force, contact)| json!({
                "task_force": n.force(*force),
                "contact": contact,
            }))
            .collect::<Vec<_>>(),
    })
}

/// The meaning of the three settings, for `command_state`.
pub(crate) const PURSUIT_MEANING: &str = "ask: the staff asks your leave for each find (a carrier force it would send after what it has found beyond a night's steaming), and the clock stops in the evening before the day is played; go: the staff takes the leave as given and tells you; stay: the staff does not look for such a find. Default: ask, or go where the leave to launch is given.";

/// The leave to pursue of each order of the side's, as a dry run carries it
/// (decision 0214, "Who answers where no one is there"): where a course gives
/// leave to launch that the side lacks, the leave to pursue of an order with
/// none set on its chain would read as go, where it reads as ask in the game.
/// So each commander's intent in force with none set on its chain gets a
/// `set_pursuit` of the value it has now, to be handed in with the course.
/// `also` is an intent the course itself brings (a previewed `set_intent`),
/// which is not in force yet; `except` is one the course leaves out.
pub fn trial_pursuit_settings(
    state: &CommandState,
    also: Option<IntentId>,
    except: Option<IntentId>,
) -> Vec<CommandKind> {
    let mut intents: Vec<IntentId> = state
        .intents_in_force()
        .filter(|intent| intent.maker == Maker::Commander)
        .map(|intent| intent.id)
        .collect();
    if let Some(also) = also {
        if !intents.contains(&also) {
            intents.push(also);
        }
    }
    intents
        .into_iter()
        .filter(|intent| Some(*intent) != except)
        .filter_map(|intent| {
            let (leave, from) = state.pursue_read_up(intent);
            from.is_none().then_some(CommandKind::SetPursuit {
                intent,
                leave: Some(leave),
            })
        })
        .collect()
}

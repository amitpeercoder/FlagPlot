//! The picture a doctrine reads (decision 0134, 13): that of the command its
//! intent is addressed to, the area, a base or a task force. With command
//! levels off every command's picture is the side's own belief, so a doctrine
//! reads what it read before (item 16).

use sim_core::command::Addressee;
use sim_core::pictures::CommandRef;
use sim_core::view::{CommandView, SideView};

/// The view of the command `addressee` is: an air group's is the command it
/// belongs to. Where that command is no longer one of the side's (a base
/// lost, a force that has ceased to be), the area command's.
pub(crate) fn picture_of<'v>(view: &SideView<'v>, addressee: Addressee) -> CommandView<'v> {
    let command = match addressee {
        Addressee::Area => Some(CommandRef::Area),
        Addressee::Base(base) => Some(CommandRef::Base(base)),
        Addressee::TaskForce(force) => view.command_of_task_force(force),
        Addressee::AirGroup(group) => view.command_of_group(group),
    };
    command
        .and_then(|command| view.command(command))
        .or_else(|| view.command(CommandRef::Area))
        .expect("a side always has its area command")
}

/// A contact named by its number in a reason (decision 0135, 20 and 22).
/// Where reports travel between commands the picture is one command's among
/// several, so the words name it: "contact 1 held by task_force:2". Where
/// they do not, the words are "contact 1", as they have always been.
pub(crate) fn contact_words(view: &CommandView<'_>, number: u32) -> String {
    if view.reports_travel() {
        format!("contact {number} held by {}", view.command())
    } else {
        format!("contact {number}")
    }
}

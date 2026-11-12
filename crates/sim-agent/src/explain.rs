//! `explain`: an event as the side may see it and its causes (decision 10 of
//! 0048).
//!
//! The causes are the ones the side's view gives (decision 0047, 9): its own
//! orders and the events it may see. An event says whether it has causes the
//! side may not see. The walk is a flat list of nodes, each naming the nodes
//! of its causes by index; a node reached twice is listed once. It stops at
//! [`DEPTH_LIMIT`] and says so.

use std::collections::{BTreeMap, VecDeque};

use serde::Serialize;
use sim_core::events::{Cause, EventKind};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::view::SideView;

use crate::answer::{NotKnown, FORMAT, VERSION};
use crate::common::{kind_name, Ctx, Dec, Handle};

/// How many steps from the asked event the walk goes.
pub const DEPTH_LIMIT: u32 = 12;

/// The answer to `explain`: the tree of causes, or the one answer given for
/// every event the side may not see.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(untagged)]
pub enum ExplainAnswer {
    Found(Box<Explain>),
    NotKnown(NotKnown),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Explain {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    /// The event asked about, which is node 0.
    pub event: u32,
    pub depth_limit: u32,
    /// True if some node at the depth limit has causes that were not followed.
    pub truncated: bool,
    pub nodes: Vec<ExplainNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ExplainNode {
    pub index: u32,
    /// Steps from the asked event by the shortest way found.
    pub depth: u32,
    /// `event` or `order`.
    #[serde(rename = "type")]
    pub node_type: &'static str,
    /// The event's number in the full log, or the order's number.
    pub id: u32,
    pub day: Option<u32>,
    pub pulse: Option<&'static str>,
    /// The event's kind in snake_case; `None` for an order.
    pub event_kind: Option<String>,
    /// The side's version of the event, as the log writes it; for an order,
    /// the log's line of its acceptance or refusal, if there is one.
    pub text: Option<String>,
    pub entities: Vec<Handle>,
    pub breakdown: Option<NodeBreakdown>,
    /// The nodes of the causes the side may see, by index.
    pub causes: Vec<u32>,
    /// True if the event has causes the side may not see.
    pub hidden_causes: bool,
    /// True if the causes were not followed because of the depth limit.
    pub causes_truncated: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct NodeBreakdown {
    pub factors: Vec<NodeFactor>,
    pub result: Option<Dec>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct NodeFactor {
    pub name: String,
    pub value: Dec,
}

/// The side's version of its event number `event` and its causes. It builds the
/// side's events from the whole log each time; to ask about many events, build
/// an [`Explainer`] once.
pub fn explain(view: &SideView<'_>, event: u32) -> ExplainAnswer {
    Explainer::new(view).explain(event)
}

/// The side's events and the names of its units, built once, for asking
/// `explain` about many events. Events are numbered by the side's own
/// numbering: 1, 2, 3... in the order its view gives them.
pub struct Explainer<'a> {
    ctx: Ctx<'a>,
    /// The events by the side's own number.
    events: BTreeMap<u32, SideEvent>,
}

/// A node of the walk: an event by the side's number, or an order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Event(u32),
    Order(u32),
}

impl<'a> Explainer<'a> {
    pub fn new(view: &SideView<'a>) -> Explainer<'a> {
        let list = view.events();
        let mut ctx = Ctx::new(*view);
        ctx.set_events(&list);
        let events = list
            .into_iter()
            .enumerate()
            .map(|(i, e)| (u32::try_from(i + 1).expect("few events"), e))
            .collect();
        Explainer { ctx, events }
    }

    /// The numbers of the events the side may see: 1 to how many there are.
    pub fn event_numbers(&self) -> Vec<u32> {
        self.events.keys().copied().collect()
    }

    /// The side's version of its event number `event` and its causes.
    pub fn explain(&self, event: u32) -> ExplainAnswer {
        if !self.events.contains_key(&event) {
            return ExplainAnswer::NotKnown(NotKnown::of("explain"));
        }
        // Breadth first, so each node has its shortest depth.
        let mut nodes: Vec<ExplainNode> = Vec::new();
        let mut seen: BTreeMap<Key, u32> = BTreeMap::new();
        let mut truncated = false;
        let mut queue: VecDeque<(Key, u32)> = VecDeque::new();
        seen.insert(Key::Event(event), 0);
        nodes.push(self.blank(Key::Event(event), 0, 0));
        queue.push_back((Key::Event(event), 0));
        while let Some((key, depth)) = queue.pop_front() {
            let Key::Event(number) = key else { continue };
            let e = &self.events[&number];
            let index = seen[&key];
            if depth >= DEPTH_LIMIT {
                if !e.causes.is_empty() {
                    nodes[index as usize].causes_truncated = true;
                    truncated = true;
                }
                continue;
            }
            let mut links = Vec::new();
            for cause in &e.causes {
                let next = match cause {
                    Cause::Event(c) => Key::Event(self.ctx.number(*c)),
                    Cause::Order(o) => Key::Order(o.raw()),
                };
                let at = match seen.get(&next) {
                    Some(at) => *at,
                    None => {
                        let at = u32::try_from(nodes.len()).expect("a walk has few nodes");
                        seen.insert(next, at);
                        nodes.push(self.blank(next, at, depth + 1));
                        queue.push_back((next, depth + 1));
                        at
                    }
                };
                links.push(at);
            }
            nodes[index as usize].causes = links;
        }
        ExplainAnswer::Found(Box::new(Explain {
            format: FORMAT,
            version: VERSION,
            kind: "explain",
            event,
            depth_limit: DEPTH_LIMIT,
            truncated,
            nodes,
        }))
    }

    /// A node with its facts and no links yet.
    fn blank(&self, key: Key, index: u32, depth: u32) -> ExplainNode {
        let ctx = &self.ctx;
        match key {
            Key::Event(number) => {
                let e = &self.events[&number];
                ExplainNode {
                    index,
                    depth,
                    node_type: "event",
                    id: number,
                    day: Some(e.day),
                    pulse: e.pulse.map(|p| p.name()),
                    event_kind: Some(kind_name(&e.kind)),
                    text: Some(e.kind.to_string()),
                    entities: e
                        .kind
                        .entities()
                        .into_iter()
                        .map(|x| ctx.entity(x))
                        .collect(),
                    breakdown: e.breakdown.as_ref().map(|b| NodeBreakdown {
                        factors: b
                            .factors
                            .iter()
                            .map(|f| NodeFactor {
                                name: f.name.to_string(),
                                value: Dec(f.value),
                            })
                            .collect(),
                        result: b.result.map(Dec),
                    }),
                    causes: Vec::new(),
                    hidden_causes: e.hidden_causes,
                    causes_truncated: false,
                }
            }
            Key::Order(order) => {
                // The log's line for the order's acceptance or refusal, if
                // the side has one.
                let text = self.events.values().find_map(|e| match &e.kind {
                    SideEventKind::Whole(
                        EventKind::OrderAccepted { order: o, .. }
                        | EventKind::OrderRejected { order: o, .. },
                    ) if o.raw() == order => Some(e.kind.to_string()),
                    _ => None,
                });
                ExplainNode {
                    index,
                    depth,
                    node_type: "order",
                    id: order,
                    day: None,
                    pulse: None,
                    event_kind: None,
                    text,
                    entities: Vec::new(),
                    breakdown: None,
                    causes: Vec::new(),
                    hidden_causes: false,
                    causes_truncated: false,
                }
            }
        }
    }
}

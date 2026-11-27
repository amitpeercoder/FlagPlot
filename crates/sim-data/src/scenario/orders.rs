use crate::quantity::Quantity;
use serde::Deserialize;
use sim_core::orders::{
    Activity, Issuer, Mission, Posture, Reach, Readiness, SpeedSetting, Stance,
    StragglerInstruction, WhenSpent,
};
use sim_core::state::Side;

#[derive(Deserialize)]
pub(super) struct OrderSpec {
    /// The day the order takes effect. Day 1 is the first day.
    pub(super) day: u32,
    pub(super) side: SideSpec,
    /// Exactly one of `task_force`, `air_group`, `land_unit`, `base` and
    /// `submarine` must be given.
    pub(super) task_force: Option<u32>,
    pub(super) air_group: Option<u32>,
    /// A base, for a `form_task_force` order (decision 0111).
    pub(super) base: Option<u32>,
    /// A land unit, for a `set_posture`, `march` or `set_activity` order.
    pub(super) land_unit: Option<u32>,
    /// A submarine, for a `station` order (decision 0218).
    pub(super) submarine: Option<u32>,
    #[serde(default)]
    pub(super) issuer: IssuerSpec,
    pub(super) intent: Option<u32>,
    /// The pulse an order given inside a day is handed in for: `"morning"`
    /// (given at 0600) or `"afternoon"` (given at 1200). Left out for an
    /// order of dusk. Only in a scenario with `orders_inside_a_day = true`
    /// (decision 0156).
    pub(super) pulse: Option<String>,
    #[serde(flatten)]
    pub(super) action: ActionSpec,
}

// The variant names are the `kind` strings in scenario files. `OrderSpec`
// flattens this enum, which turns off its own unknown field check, so the
// check lives here instead. It only works on struct variants, which is why
// `Load` and `Unload` have empty braces.
/// What a load order takes aboard.
#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum CargoSpec {
    Supply,
    Fuel,
    /// Only with a unit (decision 0066): the men alone.
    #[serde(rename = "none")]
    NoCargo,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum ActionSpec {
    /// `act_at` marks waypoints to act at, by hex (decision 0096, 1); each
    /// must be one of the waypoints, which the order's intake checks.
    SetRoute {
        waypoints: Vec<[i32; 2]>,
        #[serde(default)]
        act_at: Vec<[i32; 2]>,
        /// `stand_at` marks waypoints to stand at for the night, by hex
        /// (decision 0124, 1); each must be one of the waypoints.
        #[serde(default)]
        stand_at: Vec<[i32; 2]>,
    },
    /// Exactly one of `speed` and `knots` (decision 7 of 0021).
    SetSpeed {
        speed: Option<SpeedSpec>,
        knots: Option<Quantity>,
    },
    SetMission {
        mission: MissionSpec,
    },
    /// `cargo` is "supply" or "fuel" and defaults to supply (decision 0046).
    /// With a `unit` (decision 0066) it may also be "none".
    /// With `weight_t` and no `unit` (decision 0111): load until the force
    /// has that many tons aboard. With `weight_t` it may give `each_t`
    /// (decision 0162): the most any one ship is to hold, above 0.
    Load {
        cargo: Option<CargoSpec>,
        unit: Option<u32>,
        weight_t: Option<Quantity>,
        each_t: Option<Quantity>,
    },
    Unload {},
    /// Join the force to another of its side stopped at the same base
    /// (decision 0111).
    Join {
        into: u32,
    },
    /// Stand the force down into port (decision 0111). No other key.
    StandDown {},
    /// Give the force a charge to escort, another task force of its side by
    /// number, or with no `charge` clear it (decision 0121).
    Escort {
        charge: Option<u32>,
    },
    /// Fuel at sea from another task force of the side by number, the giver
    /// (decision 0201, 4).
    FuelFrom {
        giver: u32,
    },
    /// An order to a base: form the ships, lying in port there in no force,
    /// into a task force (decision 0111).
    FormTaskForce {
        ships: Vec<u32>,
    },
    /// `target_base` is required for `base_strike` and given for no other
    /// mission (decision 0034).
    /// `hex` is required for `reconnaissance` and given for no other mission
    /// (decision 0147).
    SetAirMission {
        mission: AirMissionSpec,
        target_base: Option<u32>,
        hex: Option<[i32; 2]>,
    },
    SetSearchArc {
        from_deg: i32,
        to_deg: i32,
        range_nm: Quantity,
    },
    /// The share of its aircraft a group on combat air patrol keeps over
    /// its hex (decision 0027).
    SetPatrolShare {
        share: Quantity,
    },
    /// The fatigue above which the staff stands the group down (decision
    /// 0036).
    SetRestThreshold {
        fatigue: Quantity,
    },
    /// `reach` is "normal" or "extended" (decision 0059).
    SetReach {
        reach: ReachSpec,
    },
    /// `kinds` are ship kinds by name, in the order the group goes for them
    /// (decision 0063). An empty list or a kind named twice is refused when
    /// the order reaches intake, as an out-of-range value is.
    SetTargetPriority {
        kinds: Vec<String>,
    },
    /// The oldest contact, in pulses, the group launches a naval strike on
    /// (decision 0114). The range is checked at intake, as for the patrol
    /// share.
    SetStrikeContactAge {
        pulses: i64,
    },
    /// The share of its aircraft the group holds back from a strike
    /// (decision 0114).
    SetStrikeHeldShare {
        share: Quantity,
    },
    /// Switch on or off the sizing of the group's strikes to their target
    /// (decision 0118).
    SetStrikeSizing {
        on: bool,
    },
    /// Switch on or off the group's flying its search once a day, in the
    /// morning (decision 0131).
    SetSearchOnce {
        on: bool,
    },
    /// The waters the group strikes first, as `[q, r]` pairs (decision
    /// 0190, 13). An empty list clears them; a hex off the map or named
    /// twice is refused when the order reaches intake.
    SetStrikeFirstWaters {
        hexes: Vec<[i32; 2]>,
    },
    /// Move the air group to the named base of its side (decision 0091). A
    /// base that is not the side's, or out of reach, is refused at intake.
    /// The base is the order's `base` key, which `OrderSpec` reads first
    /// (decision 0111: it is also the target key of a base order).
    Rebase {},
    SetRulesOfEngagement {
        stance: StanceSpec,
        break_off: Quantity,
        #[serde(default)]
        stragglers: StragglersSpec,
        keep_share: Option<Quantity>,
    },
    /// The base is the order's `base` key, as for `Rebase`.
    SetHomePort {},
    SetReadiness {
        readiness: ReadinessSpec,
    },
    Detach {
        ships: Vec<u32>,
    },
    /// Send the whole force to the yard from the base it is stopped at
    /// (decision 0069). No other key.
    SendToYard {},
    /// A land unit's posture, `defend` or `attack` (decision 0090). An
    /// attack order may carry `when_spent`, `hold` or `fall_back`: what the
    /// unit does when its attack is spent (decision 6 of 0105).
    SetPosture {
        posture: PostureSpec,
        #[serde(default)]
        when_spent: Option<WhenSpentSpec>,
    },
    /// March a land unit to a neighbouring hex (decision 0094). The hex is
    /// `[q, r]`.
    March {
        hex: [i32; 2],
    },
    /// Prepare a landing at a coast hex with the listed land units as the
    /// roster (decision 0095). The hex is `[q, r]`; the roster may be empty.
    PrepareLanding {
        hex: [i32; 2],
        roster: Vec<u32>,
    },
    /// A land unit's activity, `ready` or `train` (decisions 0095 and 0097).
    /// With `work`, an optional `job`: `airfield`, `port`, `aircraft` or
    /// `defences` (decision 0099).
    SetActivity {
        activity: ActivitySpec,
        job: Option<String>,
    },
    /// Do no cargo work: clear the force's cargo order (decision 0191). No
    /// other key.
    NoCargoWork {},
    /// Send a submarine to a station and keep it (decision 0218). The hex is
    /// `[q, r]`.
    Station {
        hex: [i32; 2],
    },
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum ActivitySpec {
    Ready,
    Train,
    Work,
}

impl From<ActivitySpec> for Activity {
    fn from(spec: ActivitySpec) -> Activity {
        match spec {
            ActivitySpec::Ready => Activity::Ready,
            ActivitySpec::Train => Activity::Train,
            ActivitySpec::Work => Activity::Work(None),
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum PostureSpec {
    Defend,
    Attack,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum WhenSpentSpec {
    Hold,
    FallBack,
}

impl From<WhenSpentSpec> for WhenSpent {
    fn from(spec: WhenSpentSpec) -> WhenSpent {
        match spec {
            WhenSpentSpec::Hold => WhenSpent::Hold,
            WhenSpentSpec::FallBack => WhenSpent::FallBack,
        }
    }
}

impl From<PostureSpec> for Posture {
    fn from(spec: PostureSpec) -> Posture {
        match spec {
            PostureSpec::Defend => Posture::Defend,
            PostureSpec::Attack => Posture::Attack,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReachSpec {
    Normal,
    Extended,
}

impl From<ReachSpec> for Reach {
    fn from(spec: ReachSpec) -> Reach {
        match spec {
            ReachSpec::Normal => Reach::Normal,
            ReachSpec::Extended => Reach::Extended,
        }
    }
}

#[derive(Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub(super) enum StragglersSpec {
    #[default]
    Keep,
    Detach,
    Leave,
    Retire,
}

impl From<StragglersSpec> for StragglerInstruction {
    fn from(spec: StragglersSpec) -> StragglerInstruction {
        match spec {
            StragglersSpec::Keep => StragglerInstruction::Keep,
            StragglersSpec::Detach => StragglerInstruction::Detach,
            StragglersSpec::Leave => StragglerInstruction::Leave,
            StragglersSpec::Retire => StragglerInstruction::Retire,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReadinessSpec {
    Normal,
    Battle,
}

impl From<ReadinessSpec> for Readiness {
    fn from(spec: ReadinessSpec) -> Readiness {
        match spec {
            ReadinessSpec::Normal => Readiness::Normal,
            ReadinessSpec::Battle => Readiness::Battle,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum StanceSpec {
    Engage,
    Avoid,
}

impl From<StanceSpec> for Stance {
    fn from(spec: StanceSpec) -> Stance {
        match spec {
            StanceSpec::Engage => Stance::Engage,
            StanceSpec::Avoid => Stance::Avoid,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum AirMissionSpec {
    Rest,
    Search,
    NavalStrike,
    CombatAirPatrol,
    BaseStrike,
    Reconnaissance,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SideSpec {
    Allied,
    Japanese,
}

impl From<SideSpec> for Side {
    fn from(spec: SideSpec) -> Side {
        match spec {
            SideSpec::Allied => Side::Allied,
            SideSpec::Japanese => Side::Japanese,
        }
    }
}

#[derive(Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub(super) enum IssuerSpec {
    #[default]
    Player,
    Staff,
}

impl From<IssuerSpec> for Issuer {
    fn from(spec: IssuerSpec) -> Issuer {
        match spec {
            IssuerSpec::Player => Issuer::Player,
            IssuerSpec::Staff => Issuer::Staff,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum MissionSpec {
    Transport,
    SurfaceCombat,
    Carrier,
    Bombardment,
    Escort,
}

impl From<MissionSpec> for Mission {
    fn from(spec: MissionSpec) -> Mission {
        match spec {
            MissionSpec::Transport => Mission::Transport,
            MissionSpec::SurfaceCombat => Mission::SurfaceCombat,
            MissionSpec::Carrier => Mission::Carrier,
            MissionSpec::Bombardment => Mission::Bombardment,
            MissionSpec::Escort => Mission::Escort,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum SpeedSpec {
    Cruise,
    Full,
}

impl From<SpeedSpec> for SpeedSetting {
    fn from(spec: SpeedSpec) -> SpeedSetting {
        match spec {
            SpeedSpec::Cruise => SpeedSetting::Cruise,
            SpeedSpec::Full => SpeedSetting::Full,
        }
    }
}

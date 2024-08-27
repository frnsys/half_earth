#![feature(generic_arg_infer)]

mod diff;
mod events;
pub mod flavor;
mod industries;
mod kinds;
mod npcs;
mod production;
mod projects;
mod regions;
mod state;
mod util;
mod world;

pub use diff::{Change, Diff};
pub use events::{
    mean_demand_outlook_change,
    mean_income_outlook_change,
    Condition,
    ConditionKind,
    Effect,
    EffectKind,
    Event,
    Flag,
    IconEvent,
    Likelihood,
    LocalVariable,
    Phase as EventPhase,
    PlayerVariable,
    Probability,
    Request as NPCRequest,
    WorldVariable,
    ICON_EVENTS,
};
pub use industries::Industry;
pub use kinds::*;
pub use npcs::{NPCRelation, NPC};
pub use production::{Process, ProcessFeature};
pub use projects::{
    Cost,
    Factor,
    FactorKind,
    Group,
    Outcome,
    Project,
    Status,
    Type as ProjectType,
    Upgrade,
};
pub use regions::{Income, Latitude, Region};
pub use state::{Emissions, ResolvedEvent, State, Update};
pub use util::*;
pub use world::World;

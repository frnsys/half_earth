#![feature(generic_arg_infer)]
#[macro_use]

mod kinds;
mod events;
pub mod flavor;
mod industries;
mod npcs;
mod production;
mod projects;
mod regions;
mod state;
pub mod surface;
mod util;
mod world;

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

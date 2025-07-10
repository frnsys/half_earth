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

#[cfg(test)]
mod test {
    use super::*;

    /// Test for a bug where fuel production would drop to zero
    /// when using full-on green hydrogen.
    #[test]
    fn test_green_hydrogen_scenario() {
        let mut state = State::default();

        let changes: Vec<_> = state
            .world
            .processes
            .iter()
            .map(|proc| (proc.id, -(proc.mix_share as isize)))
            .collect();
        for (id, change) in changes {
            state.change_process_mix_share(&id, change);
        }

        let mix = [
            ("Green Hydrogen", 20),
            ("Floating Wind Turbines", 10),
            ("Solar PV", 6),
            ("Terrestrial Wind Power", 4),
            ("Organic Crop Ag", 10),
            ("Smallholder Farms", 8),
            ("Vertical Farming", 2),
            ("Cellular Meat", 18),
            ("Organic Livestock Ag", 2),
        ];
        let mut changes = vec![];
        for (name, share) in mix {
            let id = state
                .world
                .processes
                .iter()
                .find(|proc| proc.name == name)
                .unwrap()
                .id;
            changes.push((id, share));
        }
        for (id, change) in changes {
            state.change_process_mix_share(&id, change);
        }

        for (name, share) in mix {
            let mix_share = state
                .world
                .processes
                .iter()
                .find(|proc| proc.name == name)
                .unwrap()
                .mix_share;
            assert_eq!(share as usize, mix_share);
        }

        let tgav = 1.2678074;
        for _ in 0..5 {
            state.step_year(tgav);
            let produced = state.produced.total();
            for (_output, amount) in produced.items() {
                assert!(amount > 0.);
            }
        }
    }
}

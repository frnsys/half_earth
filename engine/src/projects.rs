use crate::{
    Collection, HasId, Id,
    events::{Effect, Probability},
    flavor::ProjectFlavor,
    kinds::{Output, OutputMap},
    npcs::{NPC, NPCRelation, RELATIONSHIP_CHANGE_AMOUNT},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{Display, EnumDiscriminants, EnumIter, EnumString, IntoStaticStr};

/// The project's status.
#[derive(
    Display,
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Default,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum Status {
    #[default]
    Inactive,
    Building,
    Active,
    Halted,
    Stalled,
    Finished,
}

/// The project's category.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Default,
    IntoStaticStr,
    EnumIter,
    EnumString,
    Display,
)]
pub enum Group {
    #[default]
    Other,
    Space,
    Nuclear,
    Restoration,
    Agriculture,
    Food,
    Geoengineering,
    Population,
    Control,
    Protection,
    Electrification,
    Behavior,
    Limits,
    Energy,
    Materials,
    Buildings,
    Cities,
}

/// The type of project.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Default,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Display,
)]
pub enum Type {
    #[default]
    Policy,
    Research,
    Initiative,
}

/// The type of project cost.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Cost {
    Fixed(usize),
    Dynamic(f32, Factor),
}
impl Default for Cost {
    fn default() -> Self {
        Cost::Fixed(0)
    }
}

/// A cost factor used to compute dynamic costs.
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter, EnumString, IntoStaticStr, Display))]
#[strum_discriminants(name(FactorKind))]
pub enum Factor {
    Time,
    Income,
    Output(Output),
}

impl From<FactorKind> for Factor {
    fn from(kind: FactorKind) -> Self {
        match kind {
            FactorKind::Time => Factor::Time,
            FactorKind::Income => Factor::Income,
            FactorKind::Output => Factor::Output(Output::default()),
        }
    }
}

/// An outcome resulting from the completion of a project.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Outcome {
    pub effects: Vec<Effect>,
    pub probability: Probability,
}

/// An upgrade for a project.
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Upgrade {
    pub cost: usize,
    pub effects: Vec<Effect>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Project {
    pub id: Id,
    pub name: String,
    pub kind: Type,
    pub group: Group,
    pub ongoing: bool,
    pub gradual: bool,
    pub locked: bool,

    // For policies, the cost is the political capital cost;
    // for research and initiatives, it's the base years to completion
    pub cost: usize,
    pub base_cost: Cost,
    pub cost_modifier: f32,
    pub progress: f32,
    pub points: usize,
    pub estimate: usize,
    pub status: Status,
    pub level: usize,
    pub completed_at: usize,
    pub required_majority: f32,
    pub effects: Vec<Effect>,
    pub outcomes: Vec<Outcome>,
    pub upgrades: Vec<Upgrade>,
    pub active_outcome: Option<usize>,

    pub supporters: Vec<Id>,
    pub opposers: Vec<Id>,

    pub flavor: ProjectFlavor,
    pub notes: String,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl HasId for Project {
    fn id(&self) -> &Id {
        &self.id
    }
}

/// How many years a project takes to complete
/// for the given amount of points.
/// Has to be at least 1
pub fn years_for_points(points: usize, cost: usize) -> f32 {
    (cost as f32 / (points as f32).powf(1. / 2.75))
        .round()
        .max(1.)
}

impl Project {
    pub fn new() -> Project {
        Project {
            id: Id::new_v4(),
            name: "New Project".into(),
            cost_modifier: 1.,
            ..Default::default()
        }
    }

    pub fn is_policy(&self) -> bool {
        self.kind == Type::Policy
    }

    /// A project which is active can be made inactive.
    pub fn is_active(&self) -> bool {
        self.status == Status::Active
    }

    /// A project which is finished can never be "un"-finished.
    pub fn is_finished(&self) -> bool {
        self.status == Status::Finished
    }

    pub fn is_online(&self) -> bool {
        self.is_active() || self.is_finished()
    }

    pub fn is_building(&self) -> bool {
        self.status == Status::Building
    }

    pub fn is_haltable(&self) -> bool {
        self.is_online() && (self.kind == Type::Policy || self.ongoing)
    }

    pub fn can_upgrade(&self) -> bool {
        self.next_upgrade().is_some()
    }

    pub fn can_downgrade(&self) -> bool {
        self.kind == Type::Policy && self.level > 0
    }

    pub fn years_remaining(&self) -> usize {
        let remaining = 1. - self.progress;
        let progress_per_year = 1. / years_for_points(self.points, self.cost);
        (remaining / progress_per_year).round() as usize
    }

    /// Advance this project's implementation
    pub fn build(&mut self) -> bool {
        match &mut self.status {
            Status::Building => {
                self.progress += 1. / years_for_points(self.points, self.cost);
                if self.progress >= 1. {
                    self.status = if self.ongoing {
                        Status::Active
                    } else {
                        Status::Finished
                    };
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn start(&mut self) -> bool {
        self.status = Status::Building;
        self.kind == Type::Policy
    }

    pub fn stop(&mut self) -> (ProjectChanges, bool) {
        let mut changes = ProjectChanges::default();

        if self.status == Status::Active || self.status == Status::Finished {
            changes.remove_effects.extend(self.active_effects().clone());

            if let Some(outcome_id) = self.active_outcome {
                let effects = &self.outcomes[outcome_id].effects;
                changes.remove_effects.extend(effects.clone());
            }

            for npc_id in &self.supporters {
                changes
                    .relationships
                    .push((*npc_id, -RELATIONSHIP_CHANGE_AMOUNT));
            }
            for npc_id in &self.opposers {
                changes
                    .relationships
                    .push((*npc_id, RELATIONSHIP_CHANGE_AMOUNT));
            }
        }

        if self.progress > 0. {
            self.status = Status::Halted;
        } else {
            self.status = Status::Inactive;
        }

        let is_policy = self.kind == Type::Policy;

        (changes, is_policy)
    }

    pub fn set_points(&mut self, points: usize) {
        self.points = points;
        self.estimate = years_for_points(self.points, self.cost) as usize;
    }

    pub fn update_cost(
        &mut self,
        year: usize,
        income_level: f32,
        demand: &OutputMap,
        modifier: f32,
    ) {
        let cost = match self.base_cost {
            Cost::Fixed(c) => c,
            Cost::Dynamic(m, factor) => {
                let c = match factor {
                    // Kind of arbitrarily choose 1980 as the starting point
                    Factor::Time => m * (year - 1980) as f32,
                    Factor::Income => m * (1. + income_level),
                    Factor::Output(output) => m * demand[output],
                };
                c.round().max(0.) as usize
            }
        };
        self.cost = (cost as f32 * self.cost_modifier * modifier)
            .round()
            .max(0.) as usize;
    }

    pub fn upgrade(&mut self) -> ProjectChanges {
        let mut changes = ProjectChanges::default();

        // Upgrade effects replace the previous effects.
        // EXCEPT for locks/unlocks.
        let to_remove = self
            .active_effects()
            .iter()
            .filter(|effect| {
                !matches!(
                    effect,
                    Effect::LocksProject(..)
                        | Effect::UnlocksProject(..)
                        | Effect::UnlocksProcess(..)
                        | Effect::UnlocksNPC(..)
                        | Effect::AddEvent(..)
                )
            })
            .cloned();
        changes.remove_effects.extend(to_remove);
        let upgraded = if self.level < self.upgrades.len() {
            self.level += 1;
            true
        } else {
            false
        };
        if upgraded {
            changes.add_effects.extend(self.active_effects().clone());
        } else {
            changes.remove_effects.clear();
        }

        changes
    }

    pub fn downgrade(&mut self) -> ProjectChanges {
        let mut changes = ProjectChanges::default();
        changes.remove_effects.extend(self.active_effects().clone());

        let downgraded = if self.level > 0 {
            self.level -= 1;
            true
        } else {
            false
        };

        if downgraded {
            changes.add_effects.extend(self.active_effects().clone());
        } else {
            changes.remove_effects.clear();
        }

        changes
    }

    pub fn next_upgrade(&self) -> Option<&Upgrade> {
        self.upgrades.get(self.level)
    }

    pub fn prev_upgrade(&self) -> Option<&Upgrade> {
        if self.level > 0 {
            self.upgrades.get(self.level - 1)
        } else {
            None
        }
    }

    pub fn advance(&mut self, year: usize) -> ProjectChanges {
        let mut changes = ProjectChanges::default();

        // For gradual projects, we apply
        // interpolated effects.
        let prev_progress = self.progress;
        if prev_progress > 0. && self.gradual {
            for effect in &self.effects {
                changes.remove_effects.push(effect.clone() * prev_progress);
            }
        }

        let completed = self.build();
        if completed {
            self.completed_at = year;
            changes.add_effects.extend(self.effects.iter().cloned());

            for npc_id in &self.supporters {
                changes
                    .relationships
                    .push((*npc_id, RELATIONSHIP_CHANGE_AMOUNT));
            }
            for npc_id in &self.opposers {
                changes
                    .relationships
                    .push((*npc_id, -RELATIONSHIP_CHANGE_AMOUNT));
            }

            changes.completed = true;
        } else if self.gradual {
            for effect in &self.effects {
                changes.add_effects.push(effect.clone() * self.progress);
            }
        }
        changes
    }

    pub fn active_effects(&self) -> &Vec<Effect> {
        if self.level == 0 {
            &self.effects
        } else {
            &self.upgrades[self.level - 1].effects
        }
    }

    pub fn active_effects_with_outcomes(&self) -> Vec<&Effect> {
        let mut effects = vec![];
        if self.is_online() {
            effects.extend(self.active_effects().iter());
            if let Some(id) = self.active_outcome {
                effects.extend(self.outcomes[id].effects.iter());
            }
        }
        effects
    }

    pub fn update_required_majority(&mut self, npcs: &Collection<NPC>) {
        let opposers = self
            .opposers
            .iter()
            .filter(|id| !npcs[*id].locked && npcs[*id].relation() != NPCRelation::Ally)
            .count();
        let supporters = self
            .supporters
            .iter()
            .filter(|id| !npcs[*id].locked)
            .count();
        self.required_majority = if opposers > supporters { 0.5 } else { 0. };
    }
}

#[derive(Default, Debug)]
pub struct ProjectChanges {
    pub completed: bool,
    pub remove_effects: Vec<Effect>,
    pub add_effects: Vec<Effect>,
    pub relationships: Vec<(Id, f32)>,
}

impl Collection<Project> {
    fn in_progress(&mut self) -> impl Iterator<Item = &mut Project> {
        self.iter_mut()
            .filter(|p| matches!(p.status, Status::Building))
    }

    pub fn changeable(&self) -> impl Iterator<Item = &Project> {
        self.unlocked().filter(|p| p.is_online() || p.is_building())
    }

    pub fn part_of_plan(&self) -> impl Iterator<Item = &Project> {
        self.iter().filter(|p| p.is_online() || p.is_building())
    }

    pub fn online(&self) -> impl Iterator<Item = &Project> {
        self.unlocked().filter(|p| p.is_online())
    }

    pub fn unlocked(&self) -> impl Iterator<Item = &Project> {
        self.iter().filter(|p| !p.locked)
    }

    pub fn recent(&self, year: usize) -> impl Iterator<Item = &Project> {
        self.iter().filter(move |p| {
            if p.status == Status::Finished {
                // Completed within the past ten years
                p.completed_at >= year - 10
            } else {
                p.status == Status::Active || (p.status == Status::Building && p.gradual)
            }
        })
    }

    /// Advance all projects in progress.
    pub fn step(&mut self, year: usize) -> Vec<(Id, ProjectChanges)> {
        self.in_progress()
            .map(|project| {
                let updates = project.advance(year);
                (project.id, updates)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::events::Likelihood;

    #[test]
    fn test_build_project() {
        let mut p = Project {
            id: Id::new_v4(),
            name: "Test Project".into(),
            points: 1,
            cost: 1,
            base_cost: Cost::Fixed(1),
            cost_modifier: 1.,
            kind: Type::Policy,
            status: Status::Building,
            outcomes: vec![Outcome {
                effects: vec![],
                probability: Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![],
                },
            }],
            ..Default::default()
        };

        for _ in 0..12 {
            p.build();
        }
        assert_eq!(p.status, Status::Finished);

        p.ongoing = true;
        p.status = Status::Building;
        p.progress = 0.;
        for _ in 0..12 {
            p.build();
        }
        assert_eq!(p.status, Status::Active);
    }

    #[test]
    fn test_project_estimate() {
        let mut p = Project {
            id: Id::new_v4(),
            name: "Test Project".into(),
            cost: 10,
            base_cost: Cost::Fixed(10),
            cost_modifier: 1.,
            kind: Type::Policy,
            status: Status::Building,
            outcomes: vec![Outcome {
                effects: vec![],
                probability: Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![],
                },
            }],
            ..Default::default()
        };

        p.set_points(1);
        assert_eq!(p.estimate, 10);
        let prev_estimate = p.estimate;

        p.set_points(10);
        assert!(prev_estimate > p.estimate);
    }

    #[test]
    fn test_project_outcomes() {
        // let p = Project {
        //     id: Id::new_v4(),
        //     name: "Test Project".into(),
        //     cost: 1,
        //     base_cost: Cost::Fixed(1),
        //     cost_modifier: 1.,
        //     kind: Type::Policy,
        //     status: Status::Building,
        //     outcomes: vec![
        //         Outcome {
        //             effects: vec![],
        //             probability: Probability {
        //                 likelihood: Likelihood::Guaranteed,
        //                 conditions: vec![
        //                     Condition::WorldVariable(
        //                         WorldVariable::Year,
        //                         Comparator::Equal,
        //                         10.,
        //                     ),
        //                 ],
        //             },
        //         },
        //         Outcome {
        //             effects: vec![],
        //             probability: Probability {
        //                 likelihood: Likelihood::Guaranteed,
        //                 conditions: vec![],
        //             },
        //         },
        //     ],
        //     ..Default::default()
        // };

        // let mut state = State::default();
        // TODO move this test to state

        // Should be the second outcome
        // since the first condition isn't met
        // let outcome = p.roll_outcome(&state, &mut rng);
        // let (_outcome, i) = outcome.unwrap();
        // assert_eq!(i, 1);
        //
        // // Now should be the first,
        // state.world.year = 10;
        // let outcome = p.roll_outcome(&state, &mut rng);
        // let (_outcome, i) = outcome.unwrap();
        // assert_eq!(i, 0);
    }
}

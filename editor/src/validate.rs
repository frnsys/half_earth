use std::collections::HashSet;

use hes_engine::{Condition, Effect, Id, World, flavor::DialogueNext};

// Errors:
// - Effect refers to entity that doesn't exist.
// - Condition refers to entity that doesn't exist.

struct IdTracker {
    projects: Vec<Id>,
    processes: Vec<Id>,
    industries: Vec<Id>,
    events: Vec<Id>,
}
impl IdTracker {
    fn new(world: &World) -> Self {
        Self {
            projects: world.projects.iter().map(|item| item.id).collect(),
            processes: world.processes.iter().map(|item| item.id).collect(),
            industries: world.industries.iter().map(|item| item.id).collect(),
            events: world.events.iter().map(|item| item.id).collect(),
        }
    }

    fn check_effect(&self, effect: &Effect) -> bool {
        if let Some(id) = effect.project_id() {
            self.projects.contains(&id)
        } else if let Some(id) = effect.process_id() {
            self.processes.contains(&id)
        } else if let Some(id) = effect.industry_id() {
            self.industries.contains(&id)
        } else if let Some(id) = effect.event_id() {
            self.events.contains(&id)
        } else {
            true
        }
    }

    fn check_condition(&self, condition: &Condition) -> bool {
        if let Some(id) = condition.project_id() {
            self.projects.contains(&id)
        } else if let Some(id) = condition.process_id() {
            self.processes.contains(&id)
        } else {
            true
        }
    }
}

/// Find all references to this id, returning the names
/// of the entities that reference it.
pub fn find_references(id: Id, world: &World) -> Vec<String> {
    let mut referenced_by: HashSet<String> = HashSet::default();

    let check_effect = move |effect: &Effect| {
        [
            effect.project_id(),
            effect.process_id(),
            effect.industry_id(),
            effect.event_id(),
        ]
        .into_iter()
        .any(|id_| id_ == Some(id))
    };

    let check_condition =
        move |cond: &Condition| cond.project_id() == Some(id) || cond.process_id() == Some(id);

    for item in world.projects.iter() {
        for effect in &item.effects {
            if check_effect(effect) {
                referenced_by.insert(item.name.clone());
            }
        }
        for outcome in &item.outcomes {
            for effect in &outcome.effects {
                if check_effect(effect) {
                    referenced_by.insert(item.name.clone());
                }
            }
            for cond in &outcome.probability.conditions {
                if check_condition(cond) {
                    referenced_by.insert(item.name.clone());
                }
            }
        }
        for upgrade in &item.upgrades {
            for effect in &upgrade.effects {
                if check_effect(effect) {
                    referenced_by.insert(item.name.clone());
                }
            }
        }

        for dialogue in &item.flavor.outcomes {
            for line in &dialogue.lines {
                if let Some(DialogueNext::Responses(resps)) = &line.next {
                    for resp in resps {
                        for effect in &resp.effects {
                            if check_effect(effect) {
                                referenced_by.insert(item.name.clone());
                            }
                        }
                        for cond in &resp.conditions {
                            if check_condition(cond) {
                                referenced_by.insert(item.name.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    for item in world.events.iter() {
        for effect in &item.effects {
            if check_effect(effect) {
                referenced_by.insert(item.name.clone());
            }
        }
        for prob in &item.probabilities {
            for cond in &prob.conditions {
                if check_condition(cond) {
                    referenced_by.insert(item.name.clone());
                }
            }
        }

        for line in &item.flavor.dialogue.lines {
            if let Some(DialogueNext::Responses(resps)) = &line.next {
                for resp in resps {
                    for effect in &resp.effects {
                        if check_effect(effect) {
                            referenced_by.insert(item.name.clone());
                        }
                    }
                    for cond in &resp.conditions {
                        if check_condition(cond) {
                            referenced_by.insert(item.name.clone());
                        }
                    }
                }
            }
        }
    }

    referenced_by.into_iter().collect()
}

pub fn validate(world: &World) -> Vec<String> {
    let mut errors = vec![];
    let tracker = IdTracker::new(world);

    for item in world.projects.iter() {
        for effect in &item.effects {
            if !tracker.check_effect(effect) {
                errors.push(format!(
                    "Project effects of {:?} refers to a non-existent entity.",
                    item.name
                ));
            }
        }
        for outcome in &item.outcomes {
            for effect in &outcome.effects {
                if !tracker.check_effect(effect) {
                    errors.push(format!(
                        "Project outcomes of {:?} refers to a non-existent entity.",
                        item.name
                    ));
                }
            }
            for cond in &outcome.probability.conditions {
                if !tracker.check_condition(cond) {
                    errors.push(format!(
                        "Project outcome conditions for {:?} refers to a non-existent entity.",
                        item.name
                    ));
                }
            }
        }
        for upgrade in &item.upgrades {
            for effect in &upgrade.effects {
                if !tracker.check_effect(effect) {
                    errors.push(format!(
                        "Project upgrades of {:?} refers to a non-existent entity.",
                        item.name
                    ));
                }
            }
        }
    }

    for item in world.events.iter() {
        for effect in &item.effects {
            if !tracker.check_effect(effect) {
                errors.push(format!(
                    "Event {:?} refers to a non-existent entity.",
                    item.name
                ));
            }
        }
        for prob in &item.probabilities {
            for cond in &prob.conditions {
                if !tracker.check_condition(cond) {
                    errors.push(format!(
                        "Event conditions for {:?} refers to a non-existent entity.",
                        item.name
                    ));
                }
            }
        }
    }
    errors
}

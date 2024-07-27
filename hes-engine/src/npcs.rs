use std::fmt::Display;

use crate::{
    flavor::NPCFlavor,
    projects::Project,
    Collection,
    HasId,
    Id,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

pub fn update_seats(
    outlook_change: f32,
    projects: &[&Project],
    npcs: &mut Collection<NPC>,
) {
    let mut supporters: Vec<Id> = vec![];
    let mut opposers: Vec<Id> = vec![];
    for project in projects {
        for id in &project.supporters {
            if !npcs[id].locked {
                supporters.push(*id);
            }
        }
        for id in &project.opposers {
            if !npcs[id].locked {
                opposers.push(*id);
            }
        }
    }

    let total = supporters.len() + opposers.len();
    let change = outlook_change / total as f32;
    for id in &supporters {
        npcs[id].support += change;
    }
    for id in &opposers {
        npcs[id].support -= change;
    }

    let mut total_support = 0.;
    for npc in npcs.iter_mut() {
        if !npc.locked {
            npc.support = f32::max(0., npc.support);
            total_support += npc.support;
        }
    }

    for npc in npcs.iter_mut() {
        if !npc.locked {
            npc.seats = npc.support / total_support;
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct NPC {
    pub id: Id,
    pub relationship: f32,
    pub locked: bool,
    pub support: f32,
    pub seats: f32,
    pub flavor: NPCFlavor,
    pub name: String,
    pub extra_seats: usize,
}

impl HasId for NPC {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl Display for NPC {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl NPC {
    // NPCs are hardcoded cause it's a bit more complicated
    // to make them editable.
    pub fn load() -> Collection<Self> {
        serde_json::from_str(include_str!(
            "../assets/npcs.json"
        ))
        .unwrap()
    }

    pub fn is_ally(&self) -> bool {
        self.relation() == NPCRelation::Ally
    }

    pub fn relation(&self) -> NPCRelation {
        if self.relationship >= 5. {
            NPCRelation::Ally
        } else if self.relationship <= 1. {
            NPCRelation::Nemesis
        } else {
            NPCRelation::Neutral
        }
    }

    pub fn relationship_name(&self) -> &'static str {
        if self.relationship >= 5. {
            "Ally"
        } else if self.relationship >= 4. {
            "Friendly"
        } else if self.relationship <= 1. {
            "Nemesis"
        } else {
            "Neutral"
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Display,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum NPCRelation {
    Neutral,
    Nemesis,
    Ally,
}
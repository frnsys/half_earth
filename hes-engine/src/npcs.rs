use crate::{flavor::NPCFlavor, projects::Project};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

pub fn update_seats(
    outlook_change: f32,
    projects: &[&Project],
    npcs: &mut Vec<NPC>,
) {
    let mut supporters: Vec<usize> = vec![];
    let mut opposers: Vec<usize> = vec![];
    for project in projects {
        for id in &project.supporters {
            if !npcs[*id].locked {
                supporters.push(*id);
            }
        }
        for id in &project.opposers {
            if !npcs[*id].locked {
                opposers.push(*id);
            }
        }
    }

    let total = supporters.len() + opposers.len();
    let change = outlook_change / total as f32;
    for id in supporters {
        npcs[id].support += change;
    }
    for id in opposers {
        npcs[id].support -= change;
    }

    let mut total_support = 0.;
    for npc in &mut *npcs {
        if !npc.locked {
            npc.support = f32::max(0., npc.support);
            total_support += npc.support;
        }
    }

    for npc in &mut *npcs {
        if !npc.locked {
            npc.seats = npc.support / total_support;
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct NPC {
    pub id: usize,
    pub relationship: f32,
    pub locked: bool,
    pub support: f32,
    pub seats: f32,
    pub flavor: NPCFlavor,
    pub name: String,
    pub extra_seats: usize,
}

impl NPC {
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

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NPCRelation {
    Neutral,
    Nemesis,
    Ally,
}
impl std::fmt::Display for NPCRelation {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let term = match self {
            Self::Neutral => "neutral",
            Self::Nemesis => "nemesis",
            Self::Ally => "ally",
        };
        write!(f, "{}", term)
    }
}

// impl Serialize for NPC {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_struct("NPC", 7)?;
//         seq.serialize_field("id", &self.id)?;
//         seq.serialize_field("name", &self.name)?;
//         seq.serialize_field("relationship", &self.relationship)?;
//         seq.serialize_field("locked", &self.locked)?;
//         seq.serialize_field("support", &self.support)?;
//         seq.serialize_field("seats", &self.seats)?;
//         seq.serialize_field("is_ally", &())?; // TODO derived
//                                                                                   // fields
//         seq.end()
//     }
// }

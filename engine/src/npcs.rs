use crate::projects::Project;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::Serialize as Ser;
use crate::save::{Saveable, coerce};
use serde_json::{json, Value};

pub fn update_seats(outlook_change: f32, projects: &Vec<&Project>, npcs: &mut Vec<NPC>) {
    let mut supporters: Vec<usize> = vec![];
    let mut opposers: Vec<usize> = vec![];
    for project in projects {
        for id in &project.supporters {
            supporters.push(*id);
        }
        for id in &project.opposers {
            opposers.push(*id);
        }
    }

    let total = supporters.len() + opposers.len();
    let change = outlook_change/total as f32;
    for id in supporters {
        npcs[id].support += change;
    }
    for id in opposers {
        npcs[id].support -= change;
    }

    let mut total_support = 0.;
    for npc in &mut *npcs {
        npc.support = f32::max(0., npc.support);
        total_support += npc.support;
    }

    for npc in &mut *npcs {
        npc.seats = npc.support/total_support;
    }
}

#[derive(Clone)]
pub struct NPC {
    pub id: usize,
    pub name: &'static str,
    pub relationship: isize,
    pub locked: bool,
    pub support: f32,
    pub seats: f32,
}

impl NPC {
    pub fn relation(&self) -> NPCRelation {
        if self.relationship >= 5 {
            NPCRelation::Ally
        } else if self.relationship <= 1 {
            NPCRelation::Nemesis
        } else {
            NPCRelation::Neutral
        }
    }
}

#[derive(Ser, Debug, Clone, PartialEq)]
pub enum NPCRelation {
    Neutral,
    Nemesis,
    Ally
}

impl Serialize for NPC {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_struct("NPC", 7)?;
        seq.serialize_field("id", &self.id)?;
        seq.serialize_field("name", &self.name)?;
        seq.serialize_field("relationship", &self.relationship)?;
        seq.serialize_field("locked", &self.locked)?;
        seq.serialize_field("support", &self.support)?;
        seq.serialize_field("seats", &self.seats)?;
        seq.serialize_field("is_ally", &(self.relation() == NPCRelation::Ally))?;
        seq.end()
    }
}


impl Saveable for NPC {
    fn save(&self) -> Value {
        json!({
            "relationship": self.relationship,
            "locked": self.locked,
            "support": self.support,
            "seats": self.seats,
        })
    }

    fn load(&mut self, state: Value) {
        self.relationship = coerce(&state["relationship"]);
        self.locked = coerce(&state["locked"]);
        self.support = coerce(&state["support"]);
        self.seats = coerce(&state["seats"]);
    }
}

use serde::Serialize;
use crate::projects::Project;

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

#[derive(Serialize, Clone)]
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

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum NPCRelation {
    Neutral,
    Nemesis,
    Ally
}

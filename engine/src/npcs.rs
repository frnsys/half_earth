use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NPC {
    pub id: usize,
    pub name: &'static str,
    pub relationship: isize,
    pub locked: bool,
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

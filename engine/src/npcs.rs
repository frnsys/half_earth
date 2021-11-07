use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NPC {
    pub id: usize,
    pub name: &'static str,
    pub relationship: f32,
}

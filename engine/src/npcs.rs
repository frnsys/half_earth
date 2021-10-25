use serde::Serialize;

#[derive(Serialize)]
pub struct NPC {
    pub id: usize,
    pub name: &'static str,
    pub relationship: f32,
}

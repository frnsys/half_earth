use serde::{Deserialize, Serialize};
use serde_bytes;
use std::collections::BTreeMap;

pub type ProjectLockers = BTreeMap<usize, usize>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageData {
    File(String),
    Data {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
        mime: String,
    },
}
impl Default for ImageData {
    fn default() -> Self {
        ImageData::File("DEFAULT.jpg".into())
    }
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default,
)]
pub struct Image {
    pub data: ImageData,
    pub attribution: String,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default,
)]
pub struct ProjectFlavor {
    pub image: Image,
    pub description: String,
    pub outcomes: Vec<Dialogue>,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default,
)]
pub struct EventFlavor {
    pub arc: String,
    pub dialogue: Dialogue,
    pub image: Option<Image>,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default,
)]
pub struct RegionFlavor {
    pub image: Image,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default,
)]
pub struct IndustryFlavor {
    pub image: Image,
    pub description: String,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default,
)]
pub struct ProcessFlavor {
    pub image: Image,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCFlavor {
    pub description: String,
    pub effects: String,
    pub likes: String,
    pub dislikes: String,
    pub color: String,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Default,
)]
pub struct Dialogue {
    pub root: usize,
    pub lines: Vec<DialogueLine>,
    pub event_id: Option<usize>,
    pub region_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogueLine {
    pub id: usize,
    pub next: Option<DialogueNext>,
    pub speaker: String,
    pub text: String,
}
impl DialogueLine {
    pub fn has_decision(&self) -> bool {
        self.next.as_ref().is_some_and(|next| {
            matches!(next, DialogueNext::Branches(..))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub id: usize,
    pub line_id: Option<usize>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DialogueNext {
    Line { id: usize },
    Branches(Vec<Branch>),
}

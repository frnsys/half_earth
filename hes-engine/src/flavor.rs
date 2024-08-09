use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_bytes;
use std::collections::BTreeMap;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::events::{Condition, Effect};

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogueLine {
    pub id: usize,
    pub next: Option<DialogueNext>,
    pub speaker: Speaker,
    pub text: String,
}
impl DialogueLine {
    pub fn has_decision(&self) -> bool {
        self.next.as_ref().is_some_and(|next| {
            matches!(next, DialogueNext::Responses(..))
        })
    }
}
impl Default for DialogueLine {
    fn default() -> Self {
        DialogueLine {
            id: 0,
            next: None,
            speaker: Speaker::Game,
            text: "".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub id: usize,
    pub next_line: Option<usize>,
    pub text: String,

    #[serde(default)]
    pub conditions: Vec<Condition>,

    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DialogueNext {
    Line { id: usize },
    Responses(Vec<Response>),
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum Speaker {
    Game,
    Gossy,
    TheEconomist,
    TheEcologist,
    TheClimatologist,
    TheGeoengineer,
    TheFarmer,
    TheAlien,
    TheCitizen,
    TheDoomCultist,
    TheLeatherUnderground,
    TheEarthLiberationFront,
    TheWretched,
    TheSpacer,
    TheDoctor,
    TheEngineer,
    TheSoldier,
    TheAuthoritarian,
    TheAnimalLiberationist,
    ThePosadist,
    TheUtopian,
    TheAccelerationist,
    TheMalthusian,
    TheEcofeminist,
    TheConsumerist,
    TheFanonist,
    TheEnvironmentalist,
    TheHero,
}

impl std::fmt::Display for Speaker {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Game => "[GAME]",
                Self::Gossy => "Gossy",
                Self::TheEconomist => "The Economist",
                Self::TheEcologist => "The Ecologist",
                Self::TheClimatologist => "The Climatologist",
                Self::TheGeoengineer => "The Geoengineer",
                Self::TheFarmer => "The Farmer",
                Self::TheAlien => "The Alien",
                Self::TheCitizen => "The Citizen",
                Self::TheDoomCultist => "The Doom Cultist",
                Self::TheLeatherUnderground =>
                    "The Leather Underground",
                Self::TheEarthLiberationFront =>
                    "The Earth Liberation Front",
                Self::TheWretched => "The Wretched",
                Self::TheSpacer => "The Spacer",
                Self::TheDoctor => "The Doctor",
                Self::TheEngineer => "The Engineer",
                Self::TheSoldier => "The Soldier",
                Self::TheAuthoritarian => "The Authoritarian",
                Self::TheAnimalLiberationist =>
                    "The Animal Liberationist",
                Self::ThePosadist => "The Posadist",
                Self::TheUtopian => "The Utopian",
                Self::TheAccelerationist =>
                    "The Accelerationist",
                Self::TheMalthusian => "The Malthusian",
                Self::TheEcofeminist => "The Ecofeminist",
                Self::TheConsumerist => "The Consumerist",
                Self::TheFanonist => "The Fanonist",
                Self::TheEnvironmentalist =>
                    "The Environmentalist",
                Self::TheHero => "The Hero",
            }
        )
    }
}

impl Serialize for Speaker {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Speaker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "[GAME]" => Ok(Self::Game),
            "Gossy" => Ok(Self::Gossy),
            "The Economist" => Ok(Self::TheEconomist),
            "The Ecologist" => Ok(Self::TheEcologist),
            "The Climatologist" => Ok(Self::TheClimatologist),
            "The Geoengineer" => Ok(Self::TheGeoengineer),
            "The Farmer" => Ok(Self::TheFarmer),
            "The Alien" => Ok(Self::TheAlien),
            "The Citizen" => Ok(Self::TheCitizen),
            "The Doom Cultist" => Ok(Self::TheDoomCultist),
            "The Leather Underground" => {
                Ok(Self::TheLeatherUnderground)
            }
            "The Earth Liberation Front" => {
                Ok(Self::TheEarthLiberationFront)
            }
            "The Wretched" => Ok(Self::TheWretched),
            "The Spacer" => Ok(Self::TheSpacer),
            "The Doctor" => Ok(Self::TheDoctor),
            "The Engineer" => Ok(Self::TheEngineer),
            "The Soldier" => Ok(Self::TheSoldier),
            "The Authoritarian" => Ok(Self::TheAuthoritarian),
            "The Animal Liberationist" => {
                Ok(Self::TheAnimalLiberationist)
            }
            "The Posadist" => Ok(Self::ThePosadist),
            "The Utopian" => Ok(Self::TheUtopian),
            "The Accelerationist" => {
                Ok(Self::TheAccelerationist)
            }
            "The Malthusian" => Ok(Self::TheMalthusian),
            "The Ecofeminist" => Ok(Self::TheEcofeminist),
            "The Consumerist" => Ok(Self::TheConsumerist),
            "The Fanonist" => Ok(Self::TheFanonist),
            "The Environmentalist" => {
                Ok(Self::TheEnvironmentalist)
            }
            "The Hero" => Ok(Self::TheHero),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &[".."],
            )),
        }
    }
}

use egui::ImageSource;
use hes_engine::flavor::Speaker;

use crate::image;

pub fn speaker_icon(speaker: &Speaker) -> ImageSource<'_> {
    match speaker {
        Speaker::Game => image!("characters/placeholder.png"),
        Speaker::Gossy => image!("characters/Gossy.webp"),
        Speaker::TheEconomist => {
            image!("characters/The Economist.webp")
        }
        Speaker::TheEcologist => {
            image!("characters/The Ecologist.webp")
        }
        Speaker::TheClimatologist => {
            image!("characters/The Climatologist.webp")
        }
        Speaker::TheGeoengineer => {
            image!("characters/The Geoengineer.webp")
        }
        Speaker::TheFarmer => {
            image!("characters/The Farmer.webp")
        }
        Speaker::TheAlien => {
            image!("characters/The Alien.webp")
        }
        Speaker::TheCitizen => {
            image!("characters/The Citizen.webp")
        }
        Speaker::TheDoomCultist => {
            image!("characters/placeholder.png")
        }
        Speaker::TheLeatherUnderground => {
            image!("characters/The Leather Underground.webp")
        }
        Speaker::TheEarthLiberationFront => {
            image!("characters/The Earth Liberation Front.webp")
        }
        Speaker::TheWretched => {
            image!("characters/The Wretched.webp")
        }
        Speaker::TheSpacer => {
            image!("characters/The Spacer.webp")
        }
        Speaker::TheDoctor => {
            image!("characters/The Doctor.webp")
        }
        Speaker::TheEngineer => {
            image!("characters/The Engineer.webp")
        }
        Speaker::TheSoldier => {
            image!("characters/The Soldier.webp")
        }
        Speaker::TheAuthoritarian => {
            image!("characters/The Authoritarian.webp")
        }
        Speaker::TheAnimalLiberationist => {
            image!("characters/The Animal Liberationist.webp")
        }
        Speaker::ThePosadist => {
            image!("characters/The Posadist.png")
        }
        Speaker::TheUtopian => {
            image!("characters/The Utopian.webp")
        }
        Speaker::TheAccelerationist => {
            image!("characters/The Accelerationist.webp")
        }
        Speaker::TheMalthusian => {
            image!("characters/The Malthusian.webp")
        }
        Speaker::TheEcofeminist => {
            image!("characters/The Ecofeminist.webp")
        }
        Speaker::TheConsumerist => {
            image!("characters/The Consumerist.webp")
        }
        Speaker::TheFanonist => {
            image!("characters/The Fanonist.webp")
        }
        Speaker::TheEnvironmentalist => {
            image!("characters/The Environmentalist.webp")
        }
        Speaker::TheHero => image!("characters/The Hero.webp"),
    }
}

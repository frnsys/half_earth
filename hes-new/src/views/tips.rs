use egui::Align2;
use hes_engine::{Industry, NPC, Process, Project, Region};

use crate::{display::DisplayEvent, views::FactorsCard};

#[derive(Clone)]
pub struct TipState {
    tip: Option<Tip>,
    should_show: bool,
}
impl Default for TipState {
    fn default() -> Self {
        Self {
            tip: None,
            should_show: false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Tip {
    pub text: String,
    pub icon: &'static str,
    pub card: Option<TipCard>,
    pub subicon: Option<&'static str>,
    pub supicon: Option<&'static str>,
}
impl Tip {
    pub fn card(mut self, card: impl Into<TipCard>) -> Self {
        self.card = Some(card.into());
        self
    }

    pub fn subicon(mut self, icon: &'static str) -> Self {
        self.subicon = Some(icon);
        self
    }
}
impl egui::Widget for Tip {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Area::new("tip".into())
            .anchor(Align2::CENTER_TOP, egui::vec2(0., 20.))
            .show(ui.ctx(), |ui| {
                super::parts::raised_frame(ui, |ui| {
                    ui.set_max_width(700.);
                    ui.label(&self.text);
                });
            })
            .response
    }
}

/// Define a tooltip.
pub fn tip(icon: &'static str, text: impl Into<String>) -> Tip {
    Tip {
        icon,
        text: text.into(),
        card: None,
        subicon: None,
        supicon: None,
    }
}

pub fn add_tip(tip: Tip, resp: egui::Response) {
    resp.on_hover_ui(|ui| {
        ui.add(tip);
    });
}

#[derive(Clone, PartialEq)]
pub enum TipCard {
    Project(Project),
    Process(Process),
    Region(Region),
    Processes(Vec<Process>),
    Industry(Industry),
    Factors(FactorsCard),
    Event(DisplayEvent),
    NPC(NPC),
}
impl From<Project> for TipCard {
    fn from(value: Project) -> Self {
        TipCard::Project(value)
    }
}
impl From<Process> for TipCard {
    fn from(value: Process) -> Self {
        TipCard::Process(value)
    }
}
impl From<NPC> for TipCard {
    fn from(value: NPC) -> Self {
        TipCard::NPC(value)
    }
}
impl From<Region> for TipCard {
    fn from(value: Region) -> Self {
        TipCard::Region(value)
    }
}
impl From<Industry> for TipCard {
    fn from(value: Industry) -> Self {
        TipCard::Industry(value)
    }
}
impl From<Vec<Process>> for TipCard {
    fn from(value: Vec<Process>) -> Self {
        TipCard::Processes(value)
    }
}
impl From<FactorsCard> for TipCard {
    fn from(value: FactorsCard) -> Self {
        TipCard::Factors(value)
    }
}
impl From<DisplayEvent> for TipCard {
    fn from(value: DisplayEvent) -> Self {
        TipCard::Event(value)
    }
}

use egui::{Align2, Color32, Order};
use hes_engine::{Industry, NPC, Process, Project, Region};

use crate::{
    display::{DisplayEvent, Icon},
    text::bbcode,
    views::FactorsCard,
};

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
    pub icon: Icon,
    pub card: Option<TipCard>,
    pub subicon: Option<Icon>,
    pub supicon: Option<Icon>,
}
impl Tip {
    pub fn card(mut self, card: impl Into<TipCard>) -> Self {
        self.card = Some(card.into());
        self
    }

    pub fn subicon(mut self, icon: Icon) -> Self {
        self.subicon = Some(icon);
        self
    }

    pub fn render(&self, ctx: &egui::Context) {
        egui::Area::new("tip".into())
            .order(Order::Tooltip)
            .anchor(Align2::CENTER_TOP, egui::vec2(0., 20.))
            .show(ctx, |ui| {
                super::parts::raised_frame(ui, |ui| {
                    ui.set_max_width(480.);
                    ui.style_mut()
                        .visuals
                        .override_text_color =
                        Some(Color32::WHITE);
                    bbcode(ui, &self.text);
                });
            });
    }
}

/// Define a tooltip.
pub fn tip(icon: Icon, text: impl Into<String>) -> Tip {
    Tip {
        icon,
        text: text.into(),
        card: None,
        subicon: None,
        supicon: None,
    }
}

pub fn add_tip(
    tip: Tip,
    resp: egui::Response,
) -> egui::Response {
    let is_dragging = resp.ctx.dragged_id().is_some();
    if !is_dragging && resp.contains_pointer() {
        // if resp.hovered() { // TODO hovered is unreliable/inconsistent?
        tip.render(&resp.ctx);
    }
    resp
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

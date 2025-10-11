use egui::{Color32, LayerId, Order, PopupAnchor};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{Industry, NPC, Process, Project, Region};

use crate::{
    display::{DisplayEvent, Icon},
    parts::{glow_fill, h_center, raised_frame},
    text::bbcode,
    views::FactorsCard,
};

#[derive(Clone, PartialEq)]
pub struct Tip {
    text: String,
    icon: Icon,
    card: Option<TipCard>,
    subicon: Option<Icon>,
    supicon: Option<Icon>,
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

    pub fn supicon(mut self, icon: Icon) -> Self {
        self.supicon = Some(icon);
        self
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        // if let Some(card) = &self.card {
        //     overlay(ctx, |ui| {
        //         // TODO
        //         ui.label("testing")
        //     });
        // }

        h_center(ui, "tip", |tui| {
            tui.ui(|ui| {
                raised_frame().shadow().show(ui, |ui| {
                    ui.set_max_width(480.);
                    ui.style_mut()
                        .visuals
                        .override_text_color =
                        Some(Color32::WHITE);
                    ui.horizontal_top(|ui| {
                        ui.add(self.icon.size(24.));
                        ui.add(bbcode(&self.text));
                    });
                });
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
    let popup_id = egui::Id::new("tip");
    let rect = resp.rect;

    if resp.contains_pointer() {
        let painter = resp.ctx.layer_painter(resp.layer_id);
        glow_fill(&painter, rect, Color32::WHITE);
    }

    if resp.interact(egui::Sense::click()).clicked() {
        egui::Popup::open_id(&resp.ctx, popup_id);
        resp.ctx.memory_mut(|mem| {
            mem.data.insert_temp(popup_id, tip);
        });
    }
    resp
}

pub fn render_tip(ctx: &egui::Context) {
    let popup_id = egui::Id::new("tip");

    if let Some(tip) =
        ctx.memory(|mem| mem.data.get_temp::<Tip>(popup_id))
    {
        let height = ctx.screen_rect().height();
        egui::Popup::new(
            popup_id,
            ctx.clone(),
            PopupAnchor::Position(egui::Pos2::ZERO),
            LayerId::new(Order::Tooltip, popup_id),
        )
        .open_memory(None)
        .width(ctx.screen_rect().width())
        .frame(
            egui::Frame::NONE
                .fill(Color32::from_black_alpha(220))
                .inner_margin(egui::Margin::symmetric(18, 18)),
        )
        .show(|ui| {
            ui.set_width(ui.available_width());
            ui.set_height(height);
            tip.render(ui);
        });
    }
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

use egui::{
    Color32,
    LayerId,
    Order,
    PopupAnchor,
    PopupCloseBehavior,
};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{Industry, NPC, Process, Project};

use crate::{
    display::{DisplayEvent, Icon, icons},
    parts::{glow_fill, h_center, raised_frame},
    state::GameState,
    text::bbcode,
    views::{Card, FactorsCard, render_event_card},
};

#[derive(Clone)]
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

    // Doesn't support cards
    pub fn render_as_hover(&self, ui: &mut egui::Ui) {
        raised_frame().shadow().show(ui, |ui| {
            ui.set_max_width(480.);
            ui.style_mut().visuals.override_text_color =
                Some(Color32::WHITE);
            ui.horizontal_top(|ui| {
                ui.add(self.icon.size(24.));
                ui.add(bbcode(&self.text));
            });
        });
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &GameState,
    ) -> bool {
        let mut clicked_outside = false;
        if !self.text.is_empty() {
            h_center(ui, "tip", |tui| {
                tui.ui(|ui| {
                    let resp = raised_frame().shadow().show(
                        ui,
                        |ui| {
                            ui.set_max_width(480.);
                            ui.style_mut()
                                .visuals
                                .override_text_color =
                                Some(Color32::WHITE);
                            ui.horizontal_top(|ui| {
                                ui.add(self.icon.size(24.));
                                ui.add(bbcode(&self.text));
                            });
                        },
                    );
                    clicked_outside = resp.clicked_elsewhere();
                });
            });
        }

        if let Some(card) = &mut self.card {
            ui.add_space(32.);
            ui.style_mut().wrap_mode =
                Some(egui::TextWrapMode::Extend);
            h_center(ui, "tip-card", |tui| {
                tui.ui(|ui| {
                    let resp = match card {
                        TipCard::Project(card) => {
                            card.render(ui, state, false)
                        }
                        TipCard::Process(card) => {
                            card.render(ui, state, false)
                        }
                        TipCard::Processes(cards) => {
                            // TODO horizontal scrolling, but seems to interfere with taffy
                            ui.horizontal(|ui| {
                                ui.style_mut()
                                    .spacing
                                    .item_spacing
                                    .x = 32.;
                                ui.set_width(
                                    ui.available_width(),
                                );
                                for card in cards {
                                    card.render(
                                        ui, state, false,
                                    );
                                }
                            })
                            .response
                        }
                        TipCard::Industry(card) => {
                            card.render(ui, state, false)
                        }
                        TipCard::Factors(factors_card) => {
                            ui.set_width(420.);
                            factors_card.render(ui)
                        }
                        TipCard::Event(event) => {
                            ui.set_width(420.);
                            render_event_card(ui, state, event)
                        }
                        TipCard::NPC(card) => {
                            card.render(ui, state, false)
                        }
                    };
                    if !clicked_outside {
                        clicked_outside =
                            resp.clicked_elsewhere();
                    }
                });
            });
        }

        clicked_outside
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
    let rect = resp.rect;
    if resp.contains_pointer() {
        let painter = resp.ctx.layer_painter(resp.layer_id);
        glow_fill(&painter, rect, Color32::WHITE);
    }

    if resp.interact(egui::Sense::click()).clicked() {
        open_tip(&resp, tip);
    }
    resp
}

pub fn add_card<C: Into<TipCard>>(
    card: C,
    resp: egui::Response,
) -> egui::Response {
    if resp.interact(egui::Sense::click()).clicked() {
        let mut tip = tip(icons::OTHER, "");
        tip.card = Some(card.into());
        open_tip(&resp, tip);
    }
    resp
}

fn open_tip(resp: &egui::Response, tip: Tip) {
    let popup_id = egui::Id::new("tip");
    let opened_id = popup_id.with("just-opened");
    let card_id = popup_id.with("card");

    egui::Popup::open_id(&resp.ctx, popup_id);
    resp.ctx.memory_mut(|mem| {
        // Need to track if the tip was opened this frame
        // so that the click doesn't immediately close it
        // the same frame.
        mem.data.insert_temp(opened_id, true);

        // Track the current tip card separately, as
        // we want to continue to display it if clicking
        // tips within the current tip.
        // E.g. if I have a tip open for `Solar PV` and
        // I click on an NPC in the card, I want the `Solar PV`
        // card to remain visible.
        if let Some(card) = &tip.card {
            mem.data.insert_temp(card_id, card.clone());
        }

        mem.data.insert_temp(popup_id, tip);
    });
}

pub fn add_hover_tip(
    tip: Tip,
    resp: egui::Response,
) -> egui::Response {
    if resp.contains_pointer() {
        egui::Popup::from_response(&resp)
            .frame(egui::Frame::NONE)
            .gap(8.)
            .show(|ui| {
                tip.render_as_hover(ui);
            });
    }
    resp
}

pub fn render_tip(ctx: &egui::Context, state: &GameState) {
    let popup_id = egui::Id::new("tip");
    let opened_id = popup_id.with("just-opened");
    let card_id = popup_id.with("card");

    if let Some(mut tip) =
        ctx.memory(|mem| mem.data.get_temp::<Tip>(popup_id))
    {
        let screen_size = ctx.screen_rect().size();
        egui::Popup::new(
            popup_id,
            ctx.clone(),
            PopupAnchor::Position(egui::Pos2::ZERO),
            LayerId::new(Order::Tooltip, popup_id),
        )
        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        .open_memory(None)
        .width(screen_size.x)
        .frame(
            egui::Frame::NONE
                .fill(Color32::from_black_alpha(220))
                .inner_margin(egui::Margin::symmetric(18, 18)),
        )
        .show(|ui| {
            ui.set_max_size(screen_size); // Needed for resizing
            ui.set_width(ui.available_width());
            ui.set_height(screen_size.y);

            if let Some(card) = ctx.memory(|mem| {
                mem.data.get_temp::<TipCard>(card_id)
            }) {
                tip.card = Some(card);
            }

            // Close only if the tip wasn't just opened this frame.
            let should_close = tip.render(ui, state);
            let just_opened = ctx
                .memory(|mem| {
                    mem.data.get_temp::<bool>(opened_id)
                })
                .unwrap_or_default();
            if should_close && !just_opened {
                // Close and clear the current tip card.
                egui::Popup::close_id(ctx, popup_id);
                ctx.memory_mut(|mem| {
                    mem.data.remove::<TipCard>(card_id);
                });
            }

            // No longer just-opened this frame.
            ctx.memory_mut(|mem| {
                mem.data.insert_temp(opened_id, false);
            });
        });
    }
}

#[derive(Clone)]
pub enum TipCard {
    Project(Card<Project>),
    Process(Card<Process>),
    Processes(Vec<Card<Process>>),
    Industry(Card<Industry>),
    Factors(FactorsCard),
    Event(DisplayEvent),
    NPC(Card<NPC>),
}
impl From<Project> for TipCard {
    fn from(value: Project) -> Self {
        TipCard::Project(Card::new(value))
    }
}
impl From<Process> for TipCard {
    fn from(value: Process) -> Self {
        TipCard::Process(Card::new(value))
    }
}
impl From<NPC> for TipCard {
    fn from(value: NPC) -> Self {
        TipCard::NPC(Card::new(value))
    }
}
impl From<Industry> for TipCard {
    fn from(value: Industry) -> Self {
        TipCard::Industry(Card::new(value))
    }
}
impl From<Vec<Process>> for TipCard {
    fn from(value: Vec<Process>) -> Self {
        TipCard::Processes(
            value.into_iter().map(Card::new).collect(),
        )
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

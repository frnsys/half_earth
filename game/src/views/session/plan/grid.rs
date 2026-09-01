use egui_taffy::TuiBuilderLogic;
use hes_engine::ProjectType;

use crate::{
    display::{HasIcon, icons},
    parts::h_center,
    state::{GameState, StateExt},
    views::{Card, cards::CARD_WIDTH, scanner::Scannable},
};

pub enum CardGridAction {
    Back,
    Changed,
}

pub struct CardGrid<T: Scannable> {
    cards: Vec<Card<T>>,
}

impl<T: Scannable> CardGrid<T> {
    pub fn new(cards: Vec<Card<T>>) -> Self {
        Self { cards }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> Option<CardGridAction> {
        let mut action = None;

        let width = ui.ctx().content_rect().width();
        let slots = (width / CARD_WIDTH).floor() as usize;

        ui.add_space(16.);
        h_center(ui, "cards-grid", |tui| {
            tui.ui(|ui| {
                for chunk in self.cards.chunks_mut(slots) {
                    ui.horizontal(|ui| {
                        for project in chunk {
                            ui.add_space(5.);
                            ui.vertical(|ui| {
                                project.render(ui, state, false);
                                ui.horizontal_top(|ui| {
                                    let spacing = ui.spacing().item_spacing.x;
                                    let button_width = (CARD_WIDTH - spacing) / 2.0;
                                    let button_size =
                                        egui::vec2(button_width, ui.spacing().interact_size.y);

                                    ui.add_enabled_ui(project.is_rem_allowed(state), |ui| {
                                        if ui
                                            .add_sized(
                                                button_size,
                                                egui::Button::new(
                                                    egui::RichText::new("-").size(14.),
                                                ),
                                            )
                                            .clicked()
                                        {
                                            project.rem_scan_done(state);
                                            action = Some(CardGridAction::Changed);
                                        }
                                    });

                                    ui.add_enabled_ui(project.is_add_allowed(state), |ui| {
                                        if ui
                                            .add_sized(
                                                button_size,
                                                egui::Button::new(
                                                    egui::RichText::new("+").size(14.),
                                                ),
                                            )
                                            .clicked()
                                        {
                                            project.add_scan_done(state);
                                            action = Some(CardGridAction::Changed);
                                        }
                                    });
                                });
                            });
                            ui.add_space(5.);
                        }
                    });
                    ui.add_space(16.);
                }
            });
        });

        ui.add_space(32.);

        egui::Area::new("project-points".into())
            .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0., -12.))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                egui::Frame::NONE
                    .fill(egui::Color32::from_black_alpha(200))
                    .corner_radius(4)
                    .inner_margin(4)
                    .show(ui, |ui| {
                        let points = &state.ui.points;
                        let pc_points = state.political_capital;

                        let mut i_points = points.initiative;
                        let mut r_points = points.research;

                        let next_point_cost = state.next_point_cost(&ProjectType::Initiative);
                        i_points += pc_points / next_point_cost as isize;

                        let next_point_cost = state.next_point_cost(&ProjectType::Research);
                        r_points += pc_points / next_point_cost as isize;

                        ui.horizontal_centered(|ui| {
                            const ICON_SIZE: f32 = 16.;
                            ui.label(pc_points.to_string());
                            ui.add(icons::POLITICAL_CAPITAL.size(ICON_SIZE));

                            ui.label(" / ");

                            ui.label(i_points.to_string());
                            ui.add(ProjectType::Initiative.icon().size(ICON_SIZE));

                            ui.label(" / ");

                            ui.label(r_points.to_string());
                            ui.add(ProjectType::Research.icon().size(ICON_SIZE));

                            ui.add_space(8.);

                            if ui.button("Back").clicked() {
                                action = Some(CardGridAction::Back);
                            }
                        });
                    });
            });

        action
    }
}

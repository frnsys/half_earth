use egui::{Color32, InnerResponse, Response, Ui};
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use hes_engine::{HasId, Id};

pub const ROW_HEIGHT: f32 = 18.;

pub enum Request {
    Delete(Id),
}

pub fn help(text: &str) -> impl FnOnce(&mut Ui) -> Response {
    move |ui| {
        ui.label(
            egui::RichText::new(text)
                .color(Color32::from_gray(180))
                .size(12.),
        )
    }
}

pub type ListResponse = egui::InnerResponse<Option<Request>>;

pub const SECTION_WIDTH: f32 = 720.;

pub fn frame() -> egui::Frame {
    egui::Frame::NONE
        .stroke(egui::Stroke::new(1., egui::Color32::WHITE))
        .fill(Color32::from_black_alpha(128))
        .inner_margin(12)
        .outer_margin(16)
        .corner_radius(6)
}

pub fn editable_list<T: Default + HasId>(
    ui: &mut egui::Ui,
    items: &mut Vec<T>,
    list_item: impl Fn(&mut egui::Ui, &mut T) -> egui::Response,
) -> ListResponse {
    let mut request = None;
    let mut changed = false;

    let mut resp = ui
        .vertical(|ui| {
            ui.set_width(SECTION_WIDTH);

            h_center(ui, "add-item", |ui| {
                ui.add_space(6.);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                let mut frame = frame()
                    .inner_margin(egui::Margin::symmetric(16, 4))
                    .begin(ui);
                frame.content_ui.label("+ New");

                let resp = frame.allocate_space(ui).interact(egui::Sense::click());
                if resp.hovered() {
                    frame.frame.fill = Color32::from_rgb(0x23, 0x86, 0x36);
                }
                frame.paint(ui);

                if resp.clicked() {
                    let new = T::default();
                    items.insert(0, new);
                }
            });

            for item in items.iter_mut() {
                let resp = frame()
                    .show(ui, |ui| {
                        let id = *item.id();
                        ui.push_id(id, |ui| {
                            let resp = list_item(ui, item);
                            if resp.changed() {
                                changed = true;
                            }
                        });
                    })
                    .response;

                let pos = resp.rect.right_top();
                let rect =
                    egui::Rect::from_min_max(pos + egui::vec2(-64., 6.), pos + egui::vec2(0., 24.));
                ui.place(rect, |ui: &mut egui::Ui| {
                    let resp = ui.button("Delete").on_hover_text("Double-click to delete.");

                    if resp.double_clicked() {
                        let id = item.id();
                        request = Some(Request::Delete(*id));
                    }

                    resp
                });
            }
        })
        .response;

    if changed {
        resp.mark_changed();
    }

    InnerResponse::new(request, resp)
}

pub fn h_center<T>(ui: &mut egui::Ui, id: &str, inner: impl FnOnce(&mut egui::Ui) -> T) -> T {
    tui(ui, ui.id().with(id))
        .reserve_available_space()
        .style(taffy::Style {
            flex_grow: 1.,
            flex_direction: taffy::FlexDirection::Row,
            min_size: taffy::Size {
                width: taffy::prelude::percent(1.),
                height: taffy::prelude::auto(),
            },
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(taffy::JustifyContent::SpaceAround),
            ..Default::default()
        })
        .show(|tui| tui.ui(inner))
}

pub fn flex_justified(
    ui: &mut egui::Ui,
    id: &str,
    left: impl FnOnce(&mut egui::Ui),
    right: impl FnOnce(&mut egui::Ui),
) {
    tui(ui, ui.id().with(id))
        .reserve_available_width()
        .style(taffy::Style {
            flex_grow: 1.,
            flex_direction: taffy::FlexDirection::Row,
            min_size: taffy::Size {
                width: taffy::prelude::percent(1.),
                height: taffy::prelude::auto(),
            },
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(taffy::JustifyContent::SpaceBetween),
            ..Default::default()
        })
        .show(|tui| {
            tui.ui(|ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                left(ui);
            });
            tui.ui(|ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                right(ui);
            });
        });
}

pub fn two_columns(
    ui: &mut egui::Ui,
    left: impl FnOnce(&mut egui::Ui),
    right: impl FnOnce(&mut egui::Ui),
) {
    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            left(ui);
        });
        cols[1].vertical(|ui| {
            right(ui);
        });
    });
}

pub fn three_columns(
    ui: &mut egui::Ui,
    left: impl FnOnce(&mut egui::Ui),
    middle: impl FnOnce(&mut egui::Ui),
    right: impl FnOnce(&mut egui::Ui),
) {
    ui.columns(3, |cols| {
        cols[0].vertical(|ui| {
            left(ui);
        });
        cols[1].vertical(|ui| {
            middle(ui);
        });
        cols[2].vertical(|ui| {
            right(ui);
        });
    });
}

const SPACE: f32 = 12.;

pub fn space(ui: &mut egui::Ui) {
    ui.add_space(SPACE);
}

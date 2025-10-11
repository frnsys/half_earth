use egui::{Color32, InnerResponse, Response, Ui};
use hes_engine::{HasId, Id};

pub enum Request {
    Delete(Id),
}

pub fn help(text: &str) -> impl FnOnce(&mut Ui) -> Response {
    move |ui| {
        ui.label(
            egui::RichText::new(text)
                .color(Color32::from_gray(220)),
        )
    }
}

pub type ListResponse = egui::InnerResponse<Option<Request>>;

pub fn editable_list<T: Default + HasId>(
    ui: &mut egui::Ui,
    items: &mut Vec<T>,
    list_item: impl Fn(&mut egui::Ui, &mut T) -> egui::Response,
) -> ListResponse {
    let mut request = None;
    let mut changed = false;

    let mut resp = egui::ScrollArea::vertical()
        .show(ui, |ui| {
            ui.vertical(|ui| {
                if ui.button("Add").clicked() {
                    let new = T::default();
                    items.insert(0, new);
                }

                for item in items.iter_mut() {
                    if ui.button("Delete").clicked() {
                        let id = item.id();
                        request = Some(Request::Delete(*id));
                    }

                    let resp = list_item(ui, item);
                    if resp.changed() {
                        changed = true;
                    }
                }
            })
            .response
        })
        .inner;

    if changed {
        resp.mark_changed();
    }

    InnerResponse::new(request, resp)
}

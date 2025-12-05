use egui::{Color32, Margin, PopupAnchor, RichText, Stroke, StrokeKind, emath::OrderedFloat};
use rust_i18n::t;
use treemap::{MapItem, Mappable, TreemapLayout};

pub struct TreeItem<'a> {
    pub label: &'a str,
    pub value: f32,
    pub color: Color32,
    pub display: &'a str,
}

pub fn treemap<'a>(
    ui: &mut egui::Ui,
    label: &str,
    (width, height): (f32, f32),
    mut items: Vec<TreeItem<'a>>,
) {
    // Must be sorted, to be consistent with what the layout system returns.
    items.sort_by_key(|item| OrderedFloat::from(-item.value));

    let layout = TreemapLayout::new();
    let bounds = treemap::Rect::from_points(0.0, 0.0, width as f64, height as f64);
    let total_area = width * height;
    let mut map_items: Vec<MapItem> = items
        .iter()
        .filter(|bar| bar.value > 0.)
        .map(|bar| MapItem::with_size((bar.value * total_area) as f64))
        .collect();

    layout.layout_items(&mut map_items, bounds);

    let (_, rect) = ui.allocate_space(egui::vec2(width, height));
    let painter = ui.painter();

    let mut tip = None;
    for (map_item, item) in map_items
        .into_iter()
        .zip(items.iter().filter(|bar| bar.value > 0.))
    {
        let item_bounds = map_item.bounds();
        let x = rect.left() + item_bounds.x as f32;
        let y = rect.top() + item_bounds.y as f32;
        let node = egui::Rect::from_two_pos(
            egui::pos2(x, y),
            egui::pos2(x + item_bounds.w as f32, y + item_bounds.h as f32),
        );
        painter.rect(
            node,
            0.,
            item.color,
            Stroke::new(1., Color32::from_gray(16)),
            egui::StrokeKind::Middle,
        );

        let font_id = egui::FontId::new(12.0, egui::FontFamily::Proportional);
        let galley = ui.fonts_mut(|f| {
            f.layout_delayed_color(t!(item.label).to_string(), font_id, f32::INFINITY)
        });

        let rect = egui::Rect::from_center_size(
            node.center(),
            galley.size() + Margin::symmetric(8, 4).sum(),
        );

        if rect.width() < node.width() {
            ui.painter().rect(
                rect,
                3.,
                Color32::from_gray(0x16),
                Stroke::NONE,
                StrokeKind::Outside,
            );
            let pos = node.center() - galley.size() / 2.;
            ui.painter().galley(pos, galley, Color32::WHITE);
        }

        if ui.rect_contains_pointer(node) {
            tip = Some(format!("{}: {}", t!(item.label), item.display));
        }
    }

    if let Some(tip) = tip {
        egui::Tooltip::always_open(
            ui.ctx().clone(),
            ui.layer_id(),
            egui::Id::from(format!("{label}-treemap-tooltip")),
            PopupAnchor::Pointer,
        )
        .show(|ui| {
            ui.set_min_width(120.);
            ui.label(
                RichText::new(tip)
                    .size(12.)
                    .color(egui::Color32::WHITE)
                    .family(egui::FontFamily::Proportional),
            );
        });
    }
}

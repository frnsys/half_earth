use egui::{
    Color32,
    FontDefinitions,
    FontFamily::{self, Proportional},
    FontId,
    TextStyle,
    style::Interaction,
};

const FONT_SIZE: f32 = 15.;
const FIELD_BG: Color32 = Color32::from_gray(24);
const PANEL_BG: Color32 = Color32::from_rgb(16, 16, 16);

trait FontLoader {
    fn load_font(&mut self, name: &str, font: &'static [u8]);
}
impl FontLoader for egui::FontDefinitions {
    fn load_font(&mut self, name: &str, font: &'static [u8]) {
        let font_data = egui::FontData::from_static(font);
        self.font_data
            .insert(name.to_string(), font_data.into());

        let mut default_fonts = FontDefinitions::default()
            .families
            .get(&FontFamily::Proportional)
            .expect("Default fonts are defined")
            .clone();
        default_fonts.insert(0, name.to_string());
        self.families.insert(family(name), default_fonts);
    }
}

fn family(name: &str) -> egui::FontFamily {
    egui::FontFamily::Name(name.into())
}

fn load_fonts() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    fonts.load_font(
        "W95FA",
        include_bytes!("../assets/fonts/W95FA/W95FA.otf"),
    );
    fonts.load_font(
        "Inter",
        include_bytes!("../assets/fonts/Inter/Inter-Light.ttf"),
    );
    fonts.load_font(
        "TimesTen",
        include_bytes!("../assets/fonts/TimesTen/TimesTen.ttf"),
    );
    fonts.load_font(
        "TimesTen-Italic",
        include_bytes!(
            "../assets/fonts/TimesTen/TimesTen-Italic.ttf"
        ),
    );
    fonts.load_font(
        "NotoSansThai",
        include_bytes!(
            "../assets/fonts/NotoSansThai-Regular.ttf"
        ),
    );
    fonts.load_font(
        "NotoSansJP",
        include_bytes!(
            "../assets/fonts/NotoSansJP-Regular.ttf"
        ),
    );

    let prop = fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap();
    prop.insert(0, "Inter".into());
    prop.push("NotoSansThai".into());
    prop.push("NotoSansJP".into());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "W95FA".into());

    let serif = fonts
        .families
        .entry(egui::FontFamily::Name("TimesTen".into()))
        .or_default();
    serif.push("NotoSansThai".into());
    serif.push("NotoSansJP".into());

    // fonts.load_font(
    //     "Bold",
    //     include_bytes!("../../../assets/fonts/Inter/Inter-Bold.ttf"),
    // );

    fonts
}

/// Setup app fonts and other styling
pub fn configure_style(ctx: &egui::Context) {
    ctx.set_fonts(load_fonts());

    let style = egui::Style {
        text_styles: [
            (
                TextStyle::Heading,
                FontId::new(24.0, family("TimesTen")),
            ),
            (
                TextStyle::Body,
                FontId::new(FONT_SIZE, Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(FONT_SIZE, family("W95FA")),
            ),
            (TextStyle::Button, FontId::new(9.0, Proportional)),
            (TextStyle::Small, FontId::new(12.0, Proportional)),
            (
                TextStyle::Name("TimesTen".into()),
                FontId::new(FONT_SIZE, family("TimesTen")),
            ),
            // (
            //     TextStyle::Name("Bold".into()),
            //     FontId::new(FONT_SIZE, family("Bold")),
            // ),
        ]
        .into(),
        spacing: egui::style::Spacing {
            scroll: egui::style::ScrollStyle {
                bar_inner_margin: 0.,
                ..Default::default()
            },

            // Center button text
            button_padding: egui::vec2(2., 2.5),
            ..Default::default()
        },
        interaction: Interaction {
            tooltip_delay: 0.,
            selectable_labels: false,
            ..Default::default()
        },
        ..Default::default()
    };

    ctx.set_style(style);

    let mut visuals = egui::Visuals {
        panel_fill: PANEL_BG,
        extreme_bg_color: FIELD_BG,
        image_loading_spinners: false,
        override_text_color: Some(Color32::WHITE),
        ..Default::default()
    };
    visuals.selection.bg_fill =
        Color32::from_rgb(0xfc, 0xba, 0x03);
    visuals.selection.stroke =
        egui::Stroke::new(1., egui::Color32::BLACK);
    //visuals.widgets.active.bg_fill = HIGHLIGHT_COLOR;
    //visuals.widgets.hovered.bg_fill = HIGHLIGHT_COLOR;
    visuals.widgets.hovered.weak_bg_fill =
        egui::Color32::from_gray(20);
    visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(0.5, egui::Color32::from_gray(64));
    visuals.widgets.open.weak_bg_fill =
        egui::Color32::from_gray(24);
    visuals.widgets.open.bg_fill = egui::Color32::from_gray(24);
    visuals.widgets.inactive.weak_bg_fill =
        egui::Color32::from_gray(24);

    // Affects tooltips
    visuals.popup_shadow = egui::Shadow::NONE;
    visuals.menu_corner_radius = 1.0.into();
    visuals.window_stroke = egui::Stroke::NONE;
    ctx.set_visuals(visuals);
}

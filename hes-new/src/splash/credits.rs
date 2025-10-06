use egui::{Color32, FontFamily, RichText, Sense, Stroke};

const CREDITS: &[(&str, &[&str])] = &[
    (
        "Concept",
        &[
            "Arthur Röing Baer",
            "Chiara Di Leone",
            "Drew Pendergrass",
            "Son La Pham",
            "Francis Tseng",
            "Gregory Vettese",
            "Troy Vettese",
        ],
    ),
    ("Design", &["Son La Pham", "Francis Tseng"]),
    ("Development", &["Francis Tseng", "Son La Pham"]),
    (
        "Research and Writing",
        &[
            "Lucy Chinen",
            "Drew Pendergrass",
            "Son La Pham",
            "Spencer Roberts",
            "Justin Saint-Loubert-Bié",
            "Francis Tseng",
            "Troy Vettese",
        ],
    ),
    ("Music", &["PRINCE SHIMA"]),
    (
        "Playtesting",
        &[
            "Spencer Roberts",
            "Dargan Frierson",
            "Sean Raspet",
            "Sarah Friend",
            "Filip Mesko",
            "Wassim Alsindi",
            "Bradley K",
            "Julia",
            "Grace Van de Pas",
            "Michael Vettese",
            "Xinyue",
            "Lukas Eigler-Harding",
            "Adrian Dinh",
            "Aural Ephem",
            "Nick Houde",
            "Simon Zhang",
            "Paul Turberg",
            "Jan Philipp Dapprich",
            "Matt Goerzen",
            "Neilson Koerner-Safrata",
            "Kira Simon-Kennedy",
            "Nicholas Carter",
        ],
    ),
    (
        "Translation",
        &[
            "Leo \"Fujoneko\" Belo",
            "Eduardo Eloy",
            "Marco Mangan",
            "Francisco Jota-Pérez",
            "Víctor Anadón Vega",
            "Christoph Rupprecht",
            "Thomas Helmis",
            "Merlin B.",
            "Chayangoon Thamma-Un",
            "เนติวิทย์ โชติภัทร์ไพศาล / Netiwit Chotiphatphaisal, Sam Yan Press",
            "Fatih Erdoğan",
        ],
    ),
];

pub struct Credits;
impl Credits {
    pub fn render(ui: &mut egui::Ui) -> bool {
        let mut close = false;
        ui.vertical_centered(|ui| {
            ui.style_mut().visuals.override_text_color =
                Some(Color32::WHITE);
            ui.add_space(32.);
            for (label, names) in CREDITS {
                ui.heading(*label);
                ui.add_space(18.);
                for name in names.iter() {
                    ui.label(RichText::new(*name).family(
                        FontFamily::Name("Serif".into()),
                    ));
                }
                ui.add_space(32.);
            }

            ui.style_mut()
                .visuals
                .widgets
                .noninteractive
                .bg_stroke = Stroke::new(1., Color32::WHITE);
            ui.separator();

            ui.add_space(32.);
            let resp =
                ui.heading("Back").interact(Sense::click());
            if resp.clicked() {
                close = true;
            }
            ui.add_space(32.);
        });
        close
    }
}

use std::fmt::Display;

use egui::{Color32, Sense};
use hes_engine::{flavor::Image, *};
use hes_images::flavor_image;
use strum::IntoEnumIterator;

use crate::parts;

pub fn edit<V: Editable>(value: V) -> Input<V> {
    Input::new(value)
}

pub fn percent<'a>(value: &'a mut f32) -> Input<Percent<'a>> {
    Input::new(Percent(value))
}

/// Like `percent` but limited to 0-100%.
pub fn share<'a>(value: &'a mut f32) -> Input<Share<'a>> {
    Input::new(Share(value))
}

pub fn textarea<'a>(
    value: &'a mut String,
) -> Input<LongText<'a>> {
    Input::new(LongText(value))
}

pub fn lock<'a>(value: &'a mut bool) -> Input<Lock<'a>> {
    Input::new(Lock(value))
}

pub fn toggle<'a>(
    value: &'a mut bool,
    on: &'static str,
    off: &'static str,
) -> Input<Toggle<'a>> {
    Input::new(Toggle(value, on, off))
}

pub fn toggle_enum<
    'a,
    E: Display + PartialEq + IntoEnumIterator,
>(
    value: &'a mut E,
) -> Input<ToggleEnum<'a, E>> {
    Input::new(ToggleEnum(value))
}

pub fn nonneg_float<'a>(
    value: &'a mut f32,
) -> Input<NonNeg<'a>> {
    Input::new(NonNeg(value))
}

pub struct Toggle<'a>(&'a mut bool, &'static str, &'static str);
pub struct ToggleEnum<
    'a,
    E: Display + PartialEq + IntoEnumIterator,
>(&'a mut E);
pub struct Lock<'a>(&'a mut bool);
pub struct Percent<'a>(&'a mut f32);
pub struct Share<'a>(&'a mut f32);
pub struct NonNeg<'a>(&'a mut f32);
pub struct LongText<'a>(&'a mut String);

pub trait Describe {
    fn describe(&self) -> &'static str;
}

pub trait Editable {
    fn edit(self, ui: &mut egui::Ui);
}

impl Editable for &mut String {
    fn edit(self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(self);
    }
}

impl<'a> Editable for LongText<'a> {
    fn edit(self, ui: &mut egui::Ui) {
        ui.text_edit_multiline(self.0);
    }
}

impl<'a> Editable for Lock<'a> {
    fn edit(self, ui: &mut egui::Ui) {
        let resp = if *self.0 {
            ui.label("🔒Locked")
        } else {
            ui.label("🔓Unlocked")
        };
        if resp.interact(Sense::click()).clicked() {
            *self.0 = !*self.0;
        }
    }
}

impl<'a> Editable for Toggle<'a> {
    fn edit(self, ui: &mut egui::Ui) {
        let on = *self.0;
        let resp = ui.colored_label(
            if on {
                Color32::WHITE
            } else {
                Color32::from_gray(128)
            },
            self.1,
        );
        if resp.interact(Sense::click()).clicked() {
            *self.0 = true;
        }

        let resp = ui.colored_label(
            if !on {
                Color32::WHITE
            } else {
                Color32::from_gray(128)
            },
            self.2,
        );
        if resp.interact(Sense::click()).clicked() {
            *self.0 = false;
        }
    }
}

impl<'a, E: Display + PartialEq + IntoEnumIterator> Editable
    for ToggleEnum<'a, E>
{
    fn edit(self, ui: &mut egui::Ui) {
        for v in E::iter() {
            let color = if v == *self.0 {
                Color32::WHITE
            } else {
                Color32::from_gray(128)
            };
            let resp = ui.colored_label(color, v.to_string());
            if resp.interact(Sense::click()).clicked() {
                *self.0 = v;
            }
        }
    }
}

impl Editable for &mut f32 {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(egui::DragValue::new(self));
    }
}

impl Editable for &mut usize {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(egui::DragValue::new(self));
    }
}

impl Editable for &mut bool {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(egui::Checkbox::new(self, ""));
    }
}

impl Editable for &mut Option<f32> {
    fn edit(self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut enable = self.is_some();
            ui.add(edit(&mut enable));

            if let Some(value) = self {
                ui.add(edit(value));
            }

            if enable != self.is_some() {
                *self = match self {
                    Some(_) => None,
                    None => Some(0.0),
                };
            }
        });
    }
}

impl Editable for &mut Option<Image> {
    fn edit(self, ui: &mut egui::Ui) {
        let mut enable = self.is_some();
        ui.add(edit(&mut enable));

        if let Some(image) = self {
            ui.add(edit(image));
        }

        if enable != self.is_some() {
            *self = match self {
                Some(_) => None,
                None => Some(Image::default()),
            };
        }
    }
}

impl<'a> Editable for Percent<'a> {
    fn edit(self, ui: &mut egui::Ui) {
        let mut value = *self.0 * 100.;
        ui.add(
            egui::DragValue::new(&mut value)
                .speed(1.0)
                .range(-500.0..=500.)
                .suffix("%"),
        );
        *self.0 = value / 100.;
    }
}

impl<'a> Editable for Share<'a> {
    fn edit(self, ui: &mut egui::Ui) {
        let mut value = *self.0 * 100.;
        ui.add(
            egui::DragValue::new(&mut value)
                .speed(1.0)
                .range(0.0..=100.)
                .suffix("%"),
        );
        *self.0 = value / 100.;
    }
}

impl<'a> Editable for NonNeg<'a> {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(
            egui::DragValue::new(self.0).range(0.0..=10000.),
        );
    }
}

impl Editable for &mut Image {
    fn edit(self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add(flavor_image(self));
            ui.add(edit(&mut self.attribution).label("Attribution").inline());
            ui.add(parts::help("Images will be bundled with your exported world, so it's recommended that you make sure they aren't too big. Recommended size is 360x240."));
        });
    }
}

impl Editable for &mut ByproductMap {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(
            edit(&mut self.co2)
                .label("CO2")
                .help("CO2 in grams."),
        );
        ui.add(
            edit(&mut self.ch4)
                .label("CH4")
                .help("CH4 (methane) in grams."),
        );
        ui.add(
            edit(&mut self.n2o)
                .label("N2O")
                .help("N2O (nitrous oxide) in grams."),
        );
        ui.add(
            edit(&mut self.biodiversity)
                .label("Biodiversity")
                .help(r#"Effects on biodiversity, in "pressure"; e.g. -1 pressure means +1 to the extinction rate."#),
        );
    }
}

impl Editable for &mut ResourceMap {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(
            edit(&mut self.land)
                .label("Land")
                .help("Land in square meters (m2)."),
        );
        ui.add(
            edit(&mut self.water)
                .label("Water")
                .help("Water in liters (L)."),
        );
        ui.add(
            edit(&mut self.electricity)
                .label("Electricity")
                .help("Electricity in kilowatt-hours (kWh)."),
        );
        ui.add(
            edit(&mut self.fuel)
                .label("Fuel")
                .help("Fuel in kilowatt-hours (kWh)."),
        );
    }
}

impl Editable for &mut OutputMap {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(
            edit(&mut self.fuel)
                .label("Fuel")
                .help("Fuel in kilowatt-hours (kWh)."),
        );
        ui.add(
            edit(&mut self.electricity)
                .label("Electricity")
                .help("Electricity in kilowatt-hours (kWh)."),
        );
        ui.add(
            edit(&mut self.plant_calories)
                .label("Plant Calories")
                .help("Plant calories in kilocalories (kcal)."),
        );
        ui.add(
            edit(&mut self.animal_calories)
                .label("Animal Calories")
                .help(
                    "Animal calories in kilocalories (kcal).",
                ),
        );
    }
}

impl Editable for &mut FeedstockMap {
    fn edit(self, ui: &mut egui::Ui) {
        ui.add(
            edit(&mut self.coal)
                .label("Coal")
                .help("Coal in grams (g)."),
        );
        ui.add(
            edit(&mut self.oil)
                .label("Oil")
                .help("Oil in liters (L)."),
        );
        ui.add(
            edit(&mut self.natural_gas)
                .label("Natural Gas")
                .help("Natural Gas in liters (L)"),
        );
        ui.add(
            edit(&mut self.thorium)
                .label("Thorium")
                .help("Thorium in grams (g)."),
        );
        ui.add(
            edit(&mut self.uranium)
                .label("Uranium")
                .help("Uranium in grams (g)."),
        );
        ui.add(
            edit(&mut self.lithium)
                .label("Lithium")
                .help("Lithium in grams (g)."),
        );
    }
}

impl Editable for &mut Factor {
    fn edit(self, ui: &mut egui::Ui) {
        let mut kind: FactorKind = (*self).into();
        ui.add(toggle_enum(&mut kind));
        if let Factor::Output(output) = self {
            ui.add(edit(output).label("Output Type").help(
                "The output to use for the demand factor.",
            ));
        }

        if kind != (*self).into() {
            *self = kind.into();
        }
    }
}

pub struct Input<V: Editable> {
    value: V,
    help: Option<String>,
    label: Option<String>,
    inline: bool,
    tooltip: Option<String>,
}
impl<V: Editable> Input<V> {
    fn new(value: V) -> Self {
        Self {
            value,
            inline: false,
            help: None,
            label: None,
            tooltip: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn tooltip(
        mut self,
        tooltip: impl Into<String>,
    ) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }
}

impl<V: Editable> egui::Widget for Input<V> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            if self.inline {
                ui.horizontal(|ui| {
                    if let Some(label) = self.label {
                        ui.label(label);
                    }
                    self.value.edit(ui);
                });
            } else {
                if let Some(label) = self.label {
                    ui.label(label);
                }
                self.value.edit(ui);
            }

            if let Some(help) = self.help {
                ui.label(help);
            }
        })
        .response
    }
}

impl Editable for &mut Vec<ProcessFeature> {
    fn edit(self, ui: &mut egui::Ui) {
        for v in ProcessFeature::iter() {
            let selected = self.contains(&v);
            let color = if selected {
                Color32::WHITE
            } else {
                Color32::from_gray(128)
            };
            let resp = ui
                .colored_label(color, v.to_string())
                .on_hover_text(v.describe())
                .interact(Sense::click());
            if resp.clicked() {
                if selected {
                    self.retain(|item| *item != v);
                } else {
                    self.push(v);
                }
            }
        }
    }
}

macro_rules! enum_dropdown {
    ($e:ident) => {
        impl Editable for &mut $e {
            fn edit(self, ui: &mut egui::Ui) {
                enum_dropdown(ui, stringify!($e), self);
            }
        }
    };
}

enum_dropdown!(Output);
enum_dropdown!(Resource);
enum_dropdown!(Feedstock);
enum_dropdown!(Byproduct);
enum_dropdown!(ProcessFeature);
enum_dropdown!(Income);
enum_dropdown!(Likelihood);
enum_dropdown!(WorldVariable);
enum_dropdown!(LocalVariable);
enum_dropdown!(PlayerVariable);
enum_dropdown!(Flag);
enum_dropdown!(RegionFlag);
enum_dropdown!(NPCRelation);
enum_dropdown!(Comparator);
enum_dropdown!(Status);
enum_dropdown!(ConditionKind);
enum_dropdown!(EffectKind);
enum_dropdown!(Latitude);
enum_dropdown!(EventPhase);
enum_dropdown!(Group);

#[derive(
    strum::Display, strum::EnumIter, PartialEq, Clone, Copy,
)]
pub enum CostKind {
    Fixed,
    Dynamic,
}

impl Editable for (&mut Vec<Id>, &Collection<NPC>) {
    fn edit(self, ui: &mut egui::Ui) {
        let (ids, opts) = self;
        for v in opts.iter() {
            let selected = ids.contains(&v.id);
            let color = if selected {
                Color32::WHITE
            } else {
                Color32::from_gray(128)
            };
            let resp = ui
                .colored_label(color, v.to_string())
                .interact(Sense::click());
            if resp.clicked() {
                if selected {
                    ids.retain(|item| *item != v.id);
                } else {
                    ids.push(v.id);
                }
            }
        }
    }
}

impl Editable for (&mut Id, &Collection<Process>) {
    fn edit(self, ui: &mut egui::Ui) {
        let (id, opts) = self;
        filter_list(ui, id, "process-picker", opts, |item| {
            &item.name
        });
    }
}

impl Editable for (&mut Id, &Collection<Project>) {
    fn edit(self, ui: &mut egui::Ui) {
        let (id, opts) = self;
        filter_list(ui, id, "project-picker", opts, |item| {
            &item.name
        });
    }
}

impl Editable for (&mut Id, &Collection<NPC>) {
    fn edit(self, ui: &mut egui::Ui) {
        let (id, opts) = self;
        filter_list(ui, id, "npc-picker", opts, |item| {
            &item.name
        });
    }
}

impl Editable for (&mut Id, &Collection<Industry>) {
    fn edit(self, ui: &mut egui::Ui) {
        let (id, opts) = self;
        filter_list(ui, id, "industry-picker", opts, |item| {
            &item.name
        });
    }
}

impl Editable for (&mut Id, &Collection<Event>) {
    fn edit(self, ui: &mut egui::Ui) {
        let (id, opts) = self;
        filter_list(ui, id, "event-picker", opts, |item| {
            &item.name
        });
    }
}

fn filter_list<'a, T: HasId>(
    ui: &mut egui::Ui,
    id: &mut Id,
    key: &'static str,
    opts: &Collection<T>,
    get_label: impl Fn(&T) -> &str,
) {
    let popup_id = ui.make_persistent_id(key);

    let resp = if let Some(current) = opts.try_get(id) {
        ui.label(get_label(&current))
    } else {
        ui.label("Nothing selected")
    };

    let resp = resp.interact(Sense::click());
    if resp.clicked() {
        egui::Popup::open_id(ui.ctx(), popup_id);
    }

    egui::Popup::from_response(&resp)
        .id(popup_id)
        .open_memory(None)
        .align(egui::RectAlign::BOTTOM_START)
        .close_behavior(
            egui::PopupCloseBehavior::CloseOnClickOutside,
        )
        .width(resp.rect.width())
        .frame(
            egui::Frame::NONE
                .corner_radius(2.)
                .fill(Color32::from_gray(8))
                .inner_margin(egui::Margin::symmetric(8, 8)),
        )
        .show(|ui| {
            ui.set_min_width(120.);

            let key: egui::Id = popup_id.with("query");
            let mut query: String = ui.memory(|mem| {
                mem.data.get_temp(key).unwrap_or_default()
            });
            let resp = ui.text_edit_singleline(&mut query);

            let q = query.to_lowercase();
            for item in opts.iter() {
                let label = get_label(item);
                if label.to_lowercase().contains(&q) {
                    let resp = ui.label(label);
                    if resp.interact(Sense::click()).clicked() {
                        *id = *item.id();
                    }
                }
            }

            if resp.changed() {
                ui.memory_mut(|mem| {
                    mem.data.insert_temp(key, query)
                });
            }
        });
}

/// Select a single value from the variants.
fn enum_dropdown<E: Display + PartialEq + IntoEnumIterator>(
    ui: &mut egui::Ui,
    id: &str,
    value: &mut E,
) -> egui::Response {
    egui::ComboBox::from_label(id)
        .selected_text(value.to_string())
        .show_ui(ui, |ui| {
            for opt in E::iter() {
                let label = opt.to_string();
                ui.selectable_value(value, opt, label);
            }
        })
        .response
}

fn edit_list<T>(
    list: &mut Vec<T>,
    new: impl FnOnce(&mut egui::Ui) -> Option<T>,
    edit: impl Fn(&mut egui::Ui, &mut T),
) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    move |ui| {
        ui.vertical(|ui| {
            if let Some(item) = new(ui) {
                list.push(item);
            }

            enum ListAction {
                Remove(usize),
                MoveUp(usize),
                MoveDown(usize),
            }

            let n = list.len();
            let mut action = None;
            for (i, item) in list.iter_mut().enumerate() {
                if ui
                    .button("Remove")
                    .on_hover_text("Double-click to delete")
                    .double_clicked()
                {
                    action = Some(ListAction::Remove(i));
                }

                if i > 0 {
                    if ui.button("Move Up").clicked() {
                        action = Some(ListAction::MoveUp(i));
                    }
                }
                if i < n - 1 {
                    if ui.button("Move Down").clicked() {
                        action = Some(ListAction::MoveDown(i));
                    }
                }

                edit(ui, item);
            }

            if let Some(action) = action {
                match action {
                    ListAction::Remove(i) => {
                        list.remove(i);
                    }
                    ListAction::MoveUp(i) => {
                        let j = i.saturating_sub(1);
                        list.swap(i, j);
                    }
                    ListAction::MoveDown(i) => {
                        let j = i.saturating_add(1).min(n - 1);
                        list.swap(i, j);
                    }
                }
            }
        })
        .response
    }
}

// impl<T: Default> Editable for &mut Vec<T>
// where
//     for<'a> &'a mut T: Editable,
// {
//     fn edit(self, ui: &mut egui::Ui) {
//     }
// }

impl Editable
    for (
        &mut Probability,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (prob, processes, projects, npcs) = self;
        ui.add(edit(&mut prob.likelihood));
        ui.add(edit((
            &mut prob.conditions,
            processes,
            projects,
            npcs,
        )));
    }
}

impl Editable
    for (
        &mut Vec<Probability>,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (list, processes, projects, npcs) = self;
        ui.add(parts::help("Probabilities are checked in their defined order, and the first probability with all conditions satisfied is the one that is rolled."));
        ui.add(edit_list(
            list,
            |ui| {
                if ui.button("Add").clicked() {
                    Some(Probability::default())
                } else {
                    None
                }
            },
            |ui, item| {
                ui.add(edit((item, processes, projects, npcs)));
            },
        ));
    }
}

impl Editable
    for (
        &mut Vec<Condition>,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (list, processes, projects, npcs) = self;
        ui.add(edit_list(
            list,
            |ui| {
                let mut kind: ConditionKind =
                    ui.memory(|mem| {
                        mem.data
                            .get_temp("new-condition".into())
                            .unwrap_or(
                                ConditionKind::WorldVariable,
                            )
                    });

                let orig = kind.clone();
                ui.add(edit(&mut kind));
                if orig != kind {
                    ui.memory_mut(|mem| {
                        mem.data.insert_temp(
                            "new-condition".into(),
                            kind,
                        )
                    });
                }

                if ui.button("Add").clicked() {
                    let default_process = processes.first().id;
                    let default_project = projects.first().id;
                    let default_npc = npcs.first().id;
                    let cond = Condition::from_kind(
                        kind,
                        default_process,
                        default_project,
                        default_npc,
                    );
                    Some(cond)
                } else {
                    None
                }
            },
            |ui, item| {
                ui.add(edit((item, processes, projects, npcs)));
            },
        ));
    }
}

impl Editable
    for (
        &mut Condition,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (cond, processes, projects, npcs) = self;
        let kind: ConditionKind = cond.clone().into();
        ui.label(kind.to_string());
        match cond {
            #[rustfmt::skip]
            Condition::WorldVariable(var, comp, value) => {
                let help = match var {
                    WorldVariable::Emissions => "The amount of annual emissions, in Gt CO2eq.",
                    WorldVariable::Temperature => "The global temperature anomaly, in C.",
                    WorldVariable::SeaLevelRise => "The amount of sea level rise, in meters.",
                    WorldVariable::SeaLevelRiseRate => "The annual change in sea level rise, in meters/year.",
                    WorldVariable::Precipitation => "The amount of precipitation, in cm/year.",
                    WorldVariable::PopulationGrowth => "The annual rate of population growth.",
                    _ => "The value to compare against.",
                };

                ui.add(parts::help("Compare against a global variable."));
                ui.add(
                    edit(var)
                        .label("Variable")
                        .help("The reference variable."),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Value")
                        .help(help)
                        .inline(),
                );
            }
            Condition::LocalVariable(var, comp, value) => {
                ui.add(parts::help("Compare against a local (regional) variable."));
                ui.add(
                    edit(var)
                        .label("Variable")
                        .help("The reference variable."),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Value")
                        .help("The value to compare against.")
                        .inline(),
                );
            }
            Condition::PlayerVariable(var, comp, value) => {
                ui.add(parts::help(
                    "Compare against a player variable.",
                ));
                ui.add(
                    edit(var)
                        .label("Variable")
                        .help("The reference variable."),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Value")
                        .help("The value to compare against.")
                        .inline(),
                );
            }
            Condition::ProcessOutput(id, comp, value) => {
                ui.add(parts::help(
                    "Compare against the output of a process.",
                ));
                ui.add(
                    edit((id, processes))
                        .label("Process")
                        .help(
                            "Which process to compare against.",
                        ),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(edit(value).label("Value").help("The output value to compare against.").inline());
            }
            Condition::ProcessMixShare(id, comp, value) => {
                ui.add(parts::help("Compare against the mix share (percentage) of a process."));
                ui.add(
                    edit((id, processes))
                        .label("Process")
                        .help(
                            "Which process to compare against.",
                        ),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    share(value)
                        .label("Mix Share")
                        .help(
                            "The mix share to compare against.",
                        )
                        .inline(),
                );
            }
            Condition::ProcessMixShareFeature(
                feat,
                comp,
                value,
            ) => {
                ui.add(parts::help("Compare against the total mix share (percentage) of processes with a particular feature."));
                ui.add(edit(feat).label("Process").help(
                    "Which process feature to compare against.",
                ));
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    share(value)
                        .label("Mix Share")
                        .help(
                            "The mix share to compare against.",
                        )
                        .inline(),
                );
            }
            Condition::ResourcePressure(
                resource,
                comp,
                value,
            ) => {
                ui.add(parts::help("Compare against the pressure on a particular resource. Pressure is represented as a percentage, where 0% means there is no pressure on the resource (demand for it is 0) and 100% means the demand for the resource equals its total supply."));
                ui.add(edit(resource).label("Resource").help(
                    "Which resource to compare against.",
                ));
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    share(value)
                        .label("Pressure")
                        .help("The value to compare against.")
                        .inline(),
                );
            }
            Condition::ResourceDemandGap(
                resource,
                comp,
                value,
            ) => {
                ui.add(parts::help("Compare against the gap between the demand and the supply of a particular resource, in the resource's units."));
                ui.add(edit(resource).label("Resource").help(
                    "Which resource to compare against.",
                ));
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Gap Size")
                        .help("The value to compare against.")
                        .inline(),
                );
            }
            Condition::OutputDemandGap(output, comp, value) => {
                ui.add(parts::help("Compare against the gap between the demand and the supply of a particular output, in the output's units."));
                ui.add(
                    edit(output).label("Output").help(
                        "Which output to compare against.",
                    ),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Gap Size")
                        .help("The value to compare against.")
                        .inline(),
                );
            }
            Condition::Demand(output, comp, value) => {
                ui.add(parts::help("Compare against the demand for a particular output, in the output's units."));
                ui.add(
                    edit(output).label("Output").help(
                        "Which output to compare against.",
                    ),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Demand Amount")
                        .help("The value to compare against.")
                        .inline(),
                );
            }
            Condition::ProjectStatus(id, status) => {
                ui.add(parts::help("Check if the status of a particular project matches the specified value."));
                ui.add(
                    edit((id, projects)).label("Project").help(
                        "Which project to compare against.",
                    ),
                );
                ui.add(
                    edit(status)
                        .label("Status")
                        .help("The expected status."),
                );
            }
            Condition::ActiveProjectUpgrades(
                id,
                comp,
                value,
            ) => {
                ui.add(parts::help("Compare against the number of active upgrades of a particular project."));
                ui.add(
                    edit((id, projects)).label("Project").help(
                        "Which project to compare against.",
                    ),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Number of Upgrades")
                        .help("The value to compare against."),
                );
            }
            Condition::RunsPlayed(comp, value) => {
                ui.add(parts::help("Compare against the number of times the player has played the game."));
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Number of Runs")
                        .help("The value to compare against."),
                );
            }
            Condition::RegionFlag(flag) => {
                ui.add(parts::help("Check if a matching region flag exists on a region."));
                ui.add(
                    edit(flag)
                        .label("Flag")
                        .help("Which flag to compare against."),
                );
            }
            Condition::NPCRelationship(id, relation) => {
                ui.add(parts::help("Check if the relationship status with a particular NPC matches the specified value."));
                ui.add(
                    edit((id, npcs))
                        .label("NPC")
                        .help("Which NPC to compare against."),
                );
                ui.add(
                    edit(relation).label("Relationship").help(
                        "The relationship to compare against.",
                    ),
                );
            }
            Condition::FeedstockYears(
                feedstock,
                comp,
                value,
            ) => {
                ui.add(parts::help("Compare against the estimated number of years before a particular feedstock is depleted."));
                ui.add(
                    edit(feedstock).label("Feedstock").help(
                        "Which feedstock to compare against.",
                    ),
                );
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Years")
                        .help("The value to compare against.")
                        .inline(),
                );
            }
            Condition::HasFlag(flag) => {
                ui.add(parts::help(
                    "Check if a matching flag exists.",
                ));
                ui.add(
                    edit(flag)
                        .label("Flag")
                        .help("Which flag to compare against."),
                );
            }
            Condition::WithoutFlag(flag) => {
                ui.add(parts::help(
                    "Check if a matching flag doesn't exist.",
                ));
                ui.add(
                    edit(flag)
                        .label("Flag")
                        .help("Which flag to compare against."),
                );
            }
            Condition::HeavyProjects(comp, value) => {
                ui.add(parts::help(r#"Compare against the number of active "Heavy" projects. This includes projects in the following groups: "Space", "Nuclear", "Geoengineering", "Electrification"."#));
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    edit(value)
                        .label("Number of Projects")
                        .help("The value to compare against."),
                );
            }
            Condition::ProtectLand(comp, value) => {
                ui.add(parts::help("Compare against the percentage of land under protection."));
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    share(value)
                        .label("Land Under Protection")
                        .help("The value to compare against."),
                );
            }
            Condition::WaterStress(comp, value) => {
                ui.add(parts::help("Compare against the percentage of water demanded over water available."));
                ui.add(
                    edit(comp)
                        .label("Comparator")
                        .help("The comparison operation."),
                );
                ui.add(
                    share(value)
                        .label("Percent of available water in use.")
                        .help("The value to compare against."),
                );
            }
        }
    }
}

impl Editable
    for (
        &mut Vec<Effect>,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<Industry>,
        &Collection<Event>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (
            list,
            processes,
            projects,
            industries,
            events,
            npcs,
        ) = self;
        ui.add(edit_list(
            list,
            |ui| {
                let mut kind: EffectKind = ui.memory(|mem| {
                    mem.data
                        .get_temp("new-effect".into())
                        .unwrap_or(EffectKind::WorldVariable)
                });

                let orig = kind.clone();
                ui.add(edit(&mut kind));
                if orig != kind {
                    ui.memory_mut(|mem| {
                        mem.data.insert_temp(
                            "new-effect".into(),
                            kind,
                        )
                    });
                }

                if ui.button("Add").clicked() {
                    let default_process = processes.first().id;
                    let default_project = projects.first().id;
                    let default_industry =
                        industries.first().id;
                    let default_event = events.first().id;
                    let default_npc = npcs.first().id;
                    let effect = Effect::from_kind(
                        kind,
                        default_process,
                        default_project,
                        default_industry,
                        default_event,
                        default_npc,
                    );
                    Some(effect)
                } else {
                    None
                }
            },
            |ui, item| {
                ui.add(edit((
                    item, processes, projects, industries,
                    events, npcs,
                )));
            },
        ));
    }
}

impl Editable
    for (
        &mut Effect,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<Industry>,
        &Collection<Event>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (
            effect,
            processes,
            projects,
            industries,
            events,
            npcs,
        ) = self;
        let kind: EffectKind = effect.clone().into();
        ui.label(kind.to_string());
        match effect {
            #[rustfmt::skip]
                    Effect::WorldVariable(var, value) => {
                        let help = match var {
                            WorldVariable::Emissions => "The amount to change annual emissions by, in Gt CO2eq.",
                            WorldVariable::Temperature => "The change in global temperatures, in C.",
                            WorldVariable::SeaLevelRise => "The amount to change the sea level rise by, in meters.",
                            WorldVariable::SeaLevelRiseRate => "The annual change in sea level rise, in meters/year.",
                            WorldVariable::Precipitation => "The amount to change the precipitation by, in cm/year.",
                            WorldVariable::PopulationGrowth => "The change in population growth.",
                            _ => "The amount to change the variable by.",
                        };

                        ui.add(
                            edit(var)
                                .label("Variable")
                                .help("What variable is changed."),
                        );
                        ui.add(
                            edit(value)
                                .label("Value")
                                .help(help)
                                .inline(),
                        );
                    }
            Effect::PlayerVariable(var, value) => {
                ui.add(
                    edit(var)
                        .label("Variable")
                        .help("What variable is changed."),
                );
                ui.add(
                            edit(value)
                                .label("Value")
                                .help("The amount to change the variable by.")
                                .inline(),
                        );
            }
            Effect::RegionHabitability(lat, value) => {
                ui.add(parts::help("Modify the habitability of all regions at the given latitude."));
                ui.add(
                    edit(lat)
                        .label("Latitude")
                        .help("What latitude is affected."),
                );
                ui.add(
                            edit(value)
                                .label("Value")
                                .help("The amount to change the habitability by.")
                                .inline(),
                        );
            }
            Effect::Resource(resource, value) => {
                ui.add(parts::help("Modify the availability of the specified resource by an absolute amount. Note that this won't do anything for fuel and electricity as those are dynamically calculated."));
                ui.add(
                    edit(resource)
                        .label("Resource")
                        .help("What resource is affected."),
                );
                ui.add(
                            edit(value)
                                .label("Value")
                                .help("The amount to change the resource reserves by.")
                                .inline(),
                        );
            }
            Effect::Demand(output, value) => {
                ui.add(parts::help("Modify all demand for the specified output by a percentage."));
                ui.add(
                    edit(output)
                        .label("Output")
                        .help("What output is affected."),
                );
                ui.add(
                            percent(value)
                                .label("Percent Change")
                                .help("The percent to modify this output's demand by.")
                                .inline(),
                        );
            }
            Effect::Output(output, value) => {
                ui.add(parts::help("Modify all production for the specified output by a percentage"));
                ui.add(
                    edit(output)
                        .label("Output")
                        .help("What output is affected."),
                );
                ui.add(
                            percent(value)
                                .label("Percent Change")
                                .help("The percent to modify this output's amount by.")
                                .inline(),
                        );
            }
            Effect::DemandAmount(output, value) => {
                ui.add(parts::help("Modify all demand for the specified output by an absolute amount."));
                ui.add(
                    edit(output)
                        .label("Output")
                        .help("What output is affected."),
                );
                ui.add(
                            edit(value)
                                .label("Amount")
                                .help("The amount to modify this output's demand by.")
                                .inline(),
                        );
            }
            Effect::OutputForFeature(feat, value) => {
                ui.add(parts::help("Modify the production efficiency of processes with the specified feature by a percentage. For example, a value of 10% means 10% more output is produced for the same resources/byproduct as the baseline."));
                ui.add(
                    edit(feat).label("Feature").help(
                        "What process feature is affected.",
                    ),
                );
                ui.add(
                            percent(value)
                                .label("Percent Change")
                                .help("The percent to modify the output by.")
                                .inline(),
                        );
            }
            Effect::OutputForProcess(id, value) => {
                ui.add(parts::help(
"Modify the production efficiency of a single process by a percentage. For example, a value of 10% means 10% more output is produced for the same resources/byproduct as the baseline."
                ));
                ui.add(
                    edit((id, processes))
                        .label("Process")
                        .help("Which process is affected."),
                );
                ui.add(
                            percent(value)
                                .label("Percent Change")
                                .help("The percent to modify this process's output by.")
                                .inline(),
                        );
            }
            Effect::CO2ForFeature(feat, value) => {
                ui.add(parts::help("Modify CO2 emitted for processes with the specified feature by a percentage."));
                ui.add(
                    edit(feat).label("Feature").help(
                        "What process feature is affected.",
                    ),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to modify this process's CO2 emissions by.")
                    .inline(),
                );
            }
            Effect::BiodiversityPressureForFeature(
                feat,
                value,
            ) => {
                ui.add(parts::help("Modify biodiversity pressure for processes with the specified feature by a percentage."));
                ui.add(
                    edit(feat).label("Feature").help(
                        "What process feature is affected.",
                    ),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to modify this process's biodiversity pressure by.")
                    .inline(),
                );
            }
            Effect::ProcessLimit(id, value) => {
                ui.add(parts::help(
"Modify the limit of the specified process by an absolute amount. If no process limit is defined for the process this will do nothing."
                ));
                ui.add(
                    edit((id, processes))
                        .label("Process")
                        .help("Which process is affected."),
                );
                ui.add(
                            edit(value)
                                .label("Amount")
                                .help("The amount to modify this process's limit by.")
                                .inline(),
                        );
            }
            Effect::Feedstock(feedstock, value) => {
                ui.add(parts::help("Modify the specified feedstock's reserves by a percentage"));
                ui.add(
                    edit(feedstock)
                        .label("Feedstock")
                        .help("What feedstock is affected."),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to modify this feedstock's amount by.")
                    .inline(),
                );
            }
            Effect::AddEvent(id) => {
                ui.add(parts::help(
"Add an event to the event pool (i.e. unlock it). Note: This effect is always hidden (not displayed to the user)."
                ));
                ui.add(
                    edit((id, events))
                        .label("Event")
                        .help("Which event is unlocked."),
                );
            }
            Effect::TriggerEvent(id, years) => {
                ui.add(parts::help(
"Trigger an event after a specified number of years. Note: This effect is always hidden (not displayed to the user)."
                ));
                ui.add(
                    edit((id, events))
                        .label("Event")
                        .help("Which event will be triggered."),
                );
                ui.add(
                            edit(years)
                                .label("Years")
                                .help("Years after which the event will be triggered.")
                                .inline(),
                        );
            }
            Effect::LocksProject(id) => {
                ui.add(parts::help(
"Locks a project (it will no longer be available)."
                ));
                ui.add(
                    edit((id, projects))
                        .label("Project")
                        .help("Which project is locked."),
                );
            }
            Effect::UnlocksProject(id) => {
                ui.add(parts::help("Unlocks a project."));
                ui.add(
                    edit((id, projects))
                        .label("Project")
                        .help("Which project is unlocked."),
                );
            }
            Effect::UnlocksProcess(id) => {
                ui.add(parts::help("Unlocks a process."));
                ui.add(
                    edit((id, processes))
                        .label("Process")
                        .help("Which process is unlocked."),
                );
            }
            Effect::UnlocksNPC(id) => {
                ui.add(parts::help("Unlocks a NPC."));
                ui.add(
                    edit((id, npcs))
                        .label("NPC")
                        .help("Which NPC is unlocked."),
                );
            }
            Effect::ProjectRequest(id, active, bounty) => {
                ui.add(parts::help(
                    "Starts a request for a project.",
                ));
                ui.add(
                    edit((id, projects))
                        .label("Project")
                        .help("Which project is requested."),
                );
                ui.add(toggle(active, "Active", "Inactive").label("Active").help("If the request is for this project to be implemented (active) or stopped (inactive)."));
                ui.add(
                    edit(bounty)
                    .label("Reward")
                    .help("How much political capital is awarded for fulfilling the request.")
                    .inline(),
                );
            }
            Effect::ProcessRequest(id, active, bounty) => {
                ui.add(parts::help(
                    "Starts a request for a process.",
                ));
                ui.add(
                    edit((id, processes))
                        .label("Process")
                        .help("Which process is requested."),
                );
                ui.add(toggle(active, "Active", "Inactive").label("Active").help("If the request is for this process to be active (mix share > 0) or stopped (mix share == 0)."));
                ui.add(
                    edit(bounty)
                    .label("Reward")
                    .help("How much political capital is awarded for fulfilling the request.")
                    .inline(),
                );
            }
            Effect::Migration => {
                ui.add(parts::help(
"Triggers a wave of migration across regions."
                ));
            }
            Effect::RegionLeave => {
                ui.add(parts::help("A region secedes."));
            }
            Effect::TerminationShock => {
                ui.add(parts::help(
r#"This effect only triggers when it is *unapplied*, in which case it undoes the temperature effect of the "Solar Radiation Management" project."#
                ));
            }
            Effect::AddRegionFlag(flag) => {
                ui.add(parts::help("Add a flag to a region."));
                ui.add(
                    edit(flag)
                        .label("Flag")
                        .help("Which flag to add."),
                );
            }
            Effect::AddFlag(flag) => {
                ui.add(parts::help("Set a flag."));
                ui.add(
                    edit(flag)
                        .label("Flag")
                        .help("Which flag to add."),
                );
            }
            Effect::NPCRelationship(id, change) => {
                ui.add(parts::help(
                    "Change the relationship with an NPC.",
                ));
                ui.add(edit((id, npcs)).label("NPC").help(
                    "Which NPC's relationship is affected.",
                ));
                ui.add(
                    edit(change)
                    .label("Value")
                    .help("The amount to change the relationship by.")
                    .inline(),
                );
            }
            Effect::ModifyProcessByproducts(
                id,
                byproduct,
                value,
            ) => {
                ui.add(parts::help(
"Modify the amount of a single byproduct for a single process by a percentage."
                ));
                ui.add(
                    edit((id, processes))
                        .label("Process")
                        .help("Which process is affected."),
                );
                ui.add(
                    edit(byproduct)
                        .label("Byproduct")
                        .help("Which byproduct is affected."),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to modify the byproduct by.")
                    .inline(),
                );
            }
            Effect::ModifyIndustryByproducts(
                id,
                byproduct,
                value,
            ) => {
                ui.add(parts::help(
"Modify the amount of a single byproduct for a single industry by a percentage. Note that the byproducts for many industries aren't inherent to the in
dustry but are rather because of emissions from its energy use. This modifier does *not* affect energy-use emissions, only direct emissions from the industry."
                ));
                ui.add(
                    edit((id, industries))
                        .label("Industry")
                        .help("Which industry is affected."),
                );
                ui.add(
                    edit(byproduct)
                        .label("Byproduct")
                        .help("Which byproduct is affected."),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to modify the byproduct by.")
                    .inline(),
                );
            }
            Effect::ModifyIndustryResources(
                id,
                resource,
                value,
            ) => {
                ui.add(parts::help(
"Modify the amount of a single resource used by a single industry by a percentage."
                ));
                ui.add(
                    edit((id, industries))
                        .label("Industry")
                        .help("Which industry is affected."),
                );
                ui.add(
                    edit(resource)
                        .label("Resource")
                        .help("Which resource is affected."),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to modify the resource by.")
                    .inline(),
                );
            }
            Effect::ModifyIndustryResourcesAmount(
                id,
                resource,
                value,
            ) => {
                ui.add(parts::help(
"Modify the amount of a single resource used by a single industry by an absolute amount."
                ));
                ui.add(
                    edit((id, industries))
                        .label("Industry")
                        .help("Which industry is affected."),
                );
                ui.add(
                    edit(resource)
                        .label("Resource")
                        .help("Which resource is affected."),
                );
                ui.add(
                    edit(value)
                    .label("Value")
                    .help("The amount to change the resource use by.")
                    .inline(),
                );
            }
            Effect::ModifyEventProbability(id, value) => {
                ui.add(parts::help(
"Modify the probability of an event occurring."
                ));
                ui.add(
                    edit((id, events))
                        .label("Event")
                        .help("Which event will be affected."),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to add to the event's probability.")
                    .inline(),
                );
            }
            Effect::ModifyIndustryDemand(id, value) => {
                ui.add(parts::help(
"Modify the demand for a single industry by a percentage."
                ));
                ui.add(
                    edit((id, industries))
                        .label("Industry")
                        .help("Which industry is affected."),
                );
                ui.add(
                    percent(value)
                    .label("Percent Change")
                    .help("The percent to modify the demand by.")
                    .inline(),
                );
            }
            Effect::DemandOutlookChange(output, mult) => {
                ui.add(parts::help("Apply a change in contentedness to every region based on its level of demand for the specified output, multiplied by the specified factor. Demand lev
el ranges from [1, 5], where 1 is the lowest demand level and 5 is the highest. For example, with `Output::Fuel` and a factor of 0.5 and a region with demand level 2, that means `2 * 0.5 = 1
` will be added to that region's contentedness. Note that this value is rounded, so if it were `3 * 0.5 = 1.5` this would be rounded to `2.0`."));
                ui.add(
                    edit(output)
                        .label("Output")
                        .help("What output is affected."),
                );
                ui.add(
                    edit(mult)
                    .label("Factor")
                    .help("Factor to scale the demand level by.")
                    .inline(),
                );
            }
            Effect::IncomeOutlookChange(mult) => {
                ui.add(parts::help("Apply a change in contentedness to every region based on its income level, multiplied by the specified factor. Income level ranges from [0, 3], where
 0 is the lowest income level and 3 is the highest. For example, with a factor of 0.5 and a region with income level 2, that means `2 * 0.5 = 1` will be added to that region's contentedness.
 Note that this value is rounded, so if it were `3 * 0.5 = 1.5` this would be rounded to `2.0`."));
                ui.add(
                    edit(mult)
                    .label("Factor")
                    .help("Factor to scale the contentedness by.")
                    .inline(),
                );
            }
            Effect::ProjectCostModifier(id, change) => {
                ui.add(parts::help("Modifies the cost a project by a percentage."));
                ui.add(
                    edit((id, projects))
                        .label("Project")
                        .help("Which project is affected."),
                );
                ui.add(
                    percent(change)
                    .label("Percent Change")
                    .help("The percent to modify the project's cost by.")
                    .inline(),
                );
            }
            Effect::ProtectLand(amount) => {
                ui.add(parts::help("Change the amount of land under protection by a percentage."));
                ui.add(
                    percent(amount)
                    .label("Percent Change")
                    .help("The percent to of land to add to/remove from protection.")
                    .inline(),
                );
            }
            Effect::BailOut(amount) => {
                ui.add(parts::help("Bail the player out by providing some political capital."));
                ui.add(
                    edit(amount)
                    .label("Amount")
                    .help("How much political capital to provide.")
                    .inline(),
                );
            }
            Effect::GameOver => {
                ui.add(parts::help(
                    "Trigger an immediate game over.",
                ));
            }
        }
    }
}

impl Editable
    for (
        &mut Outcome,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<Industry>,
        &Collection<Event>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (
            outcome,
            processes,
            projects,
            industries,
            events,
            npcs,
        ) = self;
        ui.add(edit(&mut outcome.probability.likelihood).label("Likelihood").help("The likelihood when all conditions are met."));
        ui.add(edit((
            &mut outcome.probability.conditions,
            processes,
            projects,
            npcs,
        )));
        ui.add(edit((
            &mut outcome.effects,
            processes,
            projects,
            industries,
            events,
            npcs,
        )));
    }
}

impl Editable
    for (
        &mut Vec<Outcome>,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<Industry>,
        &Collection<Event>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (
            list,
            processes,
            projects,
            industries,
            events,
            npcs,
        ) = self;
        ui.add(parts::help("Outcomes are checked in their defined order, so you should order them from least likely to most likely. For example, if you have a guaranteed outcome first, then that's the one that will always trigger. You should move it to the end so other outcomes can be checked before it."));
        ui.add(edit_list(
            list,
            |ui| {
                if ui.button("Add").clicked() {
                    Some(Outcome::default())
                } else {
                    None
                }
            },
            |ui, item| {
                ui.add(edit((
                    item, processes, projects, industries,
                    events, npcs,
                )));
            },
        ));
    }
}

impl Editable
    for (
        &mut Upgrade,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<Industry>,
        &Collection<Event>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (
            upgrade,
            processes,
            projects,
            industries,
            events,
            npcs,
        ) = self;
        ui.add(
            edit(&mut upgrade.cost)
                .label("Cost")
                .help("The upgrade cost."),
        );
        ui.add(edit((
            &mut upgrade.effects,
            processes,
            projects,
            industries,
            events,
            npcs,
        )));
    }
}

impl Editable
    for (
        &mut Vec<Upgrade>,
        &Collection<Process>,
        &Collection<Project>,
        &Collection<Industry>,
        &Collection<Event>,
        &Collection<NPC>,
    )
{
    fn edit(self, ui: &mut egui::Ui) {
        let (
            list,
            processes,
            projects,
            industries,
            events,
            npcs,
        ) = self;
        ui.add(edit_list(
            list,
            |ui| {
                if ui.button("Add").clicked() {
                    Some(Upgrade::default())
                } else {
                    None
                }
            },
            |ui, item| {
                ui.add(edit((
                    item, processes, projects, industries,
                    events, npcs,
                )));
            },
        ));
    }
}

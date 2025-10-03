use std::collections::BTreeMap;

use egui::Color32;
use enum_map::EnumMap;
use hes_engine::{Id, Output, State};

use crate::state::{PlanChange, Points, Tutorial};

mod npc;
mod process;
mod project;

pub struct CardState<'a> {
    pub state: &'a State,
    pub viewed: &'a Vec<Id>,
    pub plan_changes: &'a BTreeMap<Id, PlanChange>,
    pub queued_upgrades: &'a BTreeMap<Id, bool>,
    pub process_mix_changes:
        &'a EnumMap<Output, BTreeMap<Id, isize>>,
    pub process_points: &'a isize,
}

pub struct CardStateMut<'a> {
    pub state: &'a mut State,
    pub viewed: &'a mut Vec<Id>,
    pub plan_changes: &'a mut BTreeMap<Id, PlanChange>,
    pub queued_upgrades: &'a mut BTreeMap<Id, bool>,
    pub process_mix_changes:
        &'a mut EnumMap<Output, BTreeMap<Id, isize>>,
    pub points: &'a mut Points,
    pub tutorial: &'a mut Tutorial,
    pub process_points: &'a mut isize,
}

pub trait AsCard {
    fn bg_color(&self) -> Color32;
    fn header(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn figure(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn name(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn body(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn top_back(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn bottom_back(&self, ui: &mut egui::Ui, ctx: &CardState);
}

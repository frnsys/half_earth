use crate::{state::GameState, views::cards::AsCard};

mod cards;
mod process;
mod project;

pub use cards::Cards;

pub trait Scannable:
    AsCard + std::fmt::Debug + Clone + PartialEq + 'static
{
    fn add_label(&self, state: &GameState) -> Option<String>;
    fn add_scan_time(&self) -> f32;
    fn add_scan_done(
        &self,
        state: &mut GameState,
    ) -> ScanResult;
    fn is_add_visible(&self, state: &GameState) -> bool;
    fn is_add_allowed(&self, state: &GameState) -> bool;

    fn rem_label(&self, state: &GameState) -> Option<String>;
    fn rem_scan_time(&self) -> f32;
    fn rem_scan_done(
        &self,
        state: &mut GameState,
    ) -> ScanResult;
    fn is_rem_visible(&self, state: &GameState) -> bool;
    fn is_rem_allowed(&self, state: &GameState) -> bool;
}

pub enum ScanResult {
    SuccessContinue,
    SuccessStop,
    Rejected,
}

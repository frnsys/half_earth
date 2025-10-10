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

#[derive(Clone)]
pub struct ScannerControls {
    // reject_scan: Rc<dyn Fn() + 'static>,
    // pub progress_elem: HtmlElement<html::Div>,
}
impl ScannerControls {
    pub fn reject_scan(&self) {
        // TODO
        // (self.reject_scan)();
        // TODO
        // effects::shake_progress(to_ws_el(
        //     self.progress_elem.clone(),
        // ));
    }

    pub fn pulse_card(&self) {
        // TODO
        // effects::pulse_card();
    }

    pub fn pulse_level(&self) {
        // TODO
        // effects::pulse_level();
    }

    pub fn shrink_pulse_card(&self) {
        // TODO
        // effects::shrink_pulse_card();
    }

    pub fn shake_screen(&self) {
        // TODO
        // effects::shake_screen();
    }
}

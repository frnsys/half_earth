use crate::t;
use hes_engine::{industries::Industry, npcs::NPC, production::Process, projects::Project};
use leptos::*;

use super::FactorsCard;

#[derive(Clone)]
pub struct Tip {
    pub text: String,
    pub icon: &'static str,
    pub card: Option<TipCard>,
    pub subicon: Option<&'static str>,
    pub supicon: Option<&'static str>,
}
impl Tip {
    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = icon;
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn card(mut self, card: impl Into<TipCard>) -> Self {
        self.card = Some(card.into());
        self
    }

    pub fn subicon(mut self, icon: &'static str) -> Self {
        self.subicon = Some(icon);
        self
    }

    pub fn supicon(mut self, icon: &'static str) -> Self {
        self.supicon = Some(icon);
        self
    }
}
pub fn tip(icon: &'static str, text: String) -> Tip {
    Tip {
        icon,
        text,
        card: None,
        subicon: None,
        supicon: None,
    }
}

#[derive(Clone)]
pub enum TipCard {
    Project(Project),
    Process(Process),
    Processes(Vec<Process>),
    Industry(Industry),
    Factors(FactorsCard),
    NPC(NPC),
}
impl From<Project> for TipCard {
    fn from(value: Project) -> Self {
        TipCard::Project(value)
    }
}
impl From<Process> for TipCard {
    fn from(value: Process) -> Self {
        TipCard::Process(value)
    }
}
impl From<NPC> for TipCard {
    fn from(value: NPC) -> Self {
        TipCard::NPC(value)
    }
}
impl From<Industry> for TipCard {
    fn from(value: Industry) -> Self {
        TipCard::Industry(value)
    }
}
impl From<Vec<Process>> for TipCard {
    fn from(value: Vec<Process>) -> Self {
        TipCard::Processes(value)
    }
}
impl From<FactorsCard> for TipCard {
    fn from(value: FactorsCard) -> Self {
        TipCard::Factors(value)
    }
}

#[component]
pub fn ToolTip() -> impl IntoView {
    let tip_rw = expect_context::<RwSignal<Option<Tip>>>();
    let view = move || {
        if let Some(tip) = tip_rw.get() {
            Some(view! { {tip.text} })
        } else {
            None
        }
    };

    // TODO
    view! { {view} }
}

#[component(transparent)]
pub fn HasTip(children: Children, #[prop(into)] tip: MaybeSignal<Tip>) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| {
            child.on(ev::click, {
                let value = tip.clone();
                move |_| {
                    let tip_rw = expect_context::<RwSignal<Option<Tip>>>();
                    tip_rw.set(Some(value.get()))
                }
            })
        })
        .collect_view();

    view! { children }
}

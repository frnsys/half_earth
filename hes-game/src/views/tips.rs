use crate::t;
use hes_engine::projects::Project;
use leptos::*;

#[derive(Clone)]
pub struct Tip {
    pub text: String,
    pub icon: &'static str,
    pub card: Option<TipCard>,
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
}
pub fn tip(icon: &'static str, text: String) -> Tip {
    Tip {
        icon,
        text,
        card: None,
    }
}

#[derive(Clone)]
pub enum TipCard {
    Project(Project),
}
impl From<Project> for TipCard {
    fn from(value: Project) -> Self {
        TipCard::Project(value)
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

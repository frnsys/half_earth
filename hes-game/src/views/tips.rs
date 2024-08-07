use crate::views::cards::*;
use hes_engine::{Industry, Process, Project, Region, NPC};
use leptos::*;
use leptos_use::{use_document, use_event_listener};
use std::time::Duration;
use wasm_bindgen::JsCast;

use super::{DisplayEvent, FactorsCard};

#[derive(Clone)]
pub struct TipState {
    tip: Option<Tip>,
    should_show: bool,
}
impl Default for TipState {
    fn default() -> Self {
        Self {
            tip: None,
            should_show: false,
        }
    }
}

#[derive(Clone, PartialEq)]
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

/// Define a tooltip.
pub fn tip(icon: &'static str, text: String) -> Tip {
    Tip {
        icon,
        text,
        card: None,
        subicon: None,
        supicon: None,
    }
}

#[derive(Clone, PartialEq)]
pub enum TipCard {
    Project(Project),
    Process(Process),
    Region(Region),
    Processes(Vec<Process>),
    Industry(Industry),
    Factors(FactorsCard),
    Event(DisplayEvent),
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
impl From<Region> for TipCard {
    fn from(value: Region) -> Self {
        TipCard::Region(value)
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
impl From<DisplayEvent> for TipCard {
    fn from(value: DisplayEvent) -> Self {
        TipCard::Event(value)
    }
}

/// Check if this element has any ancestor with the specified class.
fn has_ancestor_with_class(
    element: web_sys::HtmlElement,
    class_name: &str,
) -> bool {
    let mut current: Option<web_sys::Element> =
        Some(element.unchecked_into());
    while let Some(el) = current {
        if el.class_list().contains(class_name) {
            return true;
        }
        current = el.parent_element();
    }
    false
}

#[component]
pub fn ToolTip() -> impl IntoView {
    let tip_rw = expect_context::<RwSignal<TipState>>();

    // The tip should actually be taken from here,
    // which preserves the current tip card if appropriate.
    let tip = create_memo(move |prev: Option<&Option<Tip>>| {
        if tip_rw.with(|state| !state.should_show) {
            return None;
        }

        tip_rw.with(|state| state.tip.clone()).map(|mut tip| {
            if let Some(Some(prev_tip)) = prev {
                if tip.card.is_some() {
                    tip
                } else {
                    tip.card = prev_tip.card.clone();
                    tip
                }
            } else {
                tip
            }
        })
    });

    let has_tip = move || tip.with(|tip| tip.is_some());

    let has_card = move || {
        with!(|tip| {
            tip.as_ref()
                .and_then(|tip| tip.card.as_ref())
                .is_some()
        })
    };

    let tip_ref = create_node_ref::<html::Div>();
    let overlay_ref = create_node_ref::<html::Div>();
    let should_show = Signal::derive(move || {
        tip_rw.with(|state| state.should_show)
    });

    // Dismiss the tooltip on click.
    let _ = use_event_listener(
        use_document(),
        ev::click,
        move |ev| {
            if !has_tip() || !should_show.get() {
                return;
            }

            let target: web_sys::HtmlElement =
                event_target(&ev);

            // If there's a card, make sure clicking
            // within the card does *not* dismiss the tooltip.
            // Otherwise clicking anywhere should remove the tooltip.
            let should_remove = !has_card()
                || (has_card()
                    && !has_ancestor_with_class(
                        target.clone(),
                        "tip--card",
                    ));

            // We don't actually remove the tooltip (i.e.
            // do `tip_rw.set(None)`) because this causes the tooltip
            // to immediately empty, whereas we want it to transition out.
            if should_remove {
                // If the clicked item also has a tooltip,
                // we let the click handler passthrough.
                // Otherwise we stop the click.
                let has_tip = has_ancestor_with_class(
                    target.clone(),
                    "has-tip",
                );
                if !has_tip {
                    ev.stop_immediate_propagation();
                }
                tip_rw
                    .update(|state| state.should_show = false);
            }
        },
    );

    let tip_view = move || {
        tip.get().map(|tip| {
            let sub_icon = move || {
                tip.subicon.map(|icon| {
                    view! { <img src=icon class="tip--subicon"/> }
                })
            };
            let sup_icon = move || {
                tip.supicon.map(|icon| {
                    view! { <img src=icon class="tip--supicon"/> }
                })
            };

            view! {
                <div class="tip--icon">
                    <img src=tip.icon/>
                    {sub_icon}
                    {sup_icon}
                </div>
                <div class="tip--body" inner_html=tip.text></div>
            }
        })
    };

    let card_view = move || {
        tip
            .get()
            .and_then(|tip| tip.card)
            .map(|card| match card {
                TipCard::Project(project) => {
                    let project = create_rw_signal(project);
                    view! { <ProjectCard project/> }
                }
                TipCard::Process(process) => {
                    let process = create_rw_signal(process);
                    view! { <ProcessCard process/> }
                }
                TipCard::Processes(processes) => {
                    view! {
                        <Cards
                            enabled=move || true
                            on_focus=move |_| {}
                            on_scroll_start=move |_| {}
                            on_scroll_end=move |_| {}
                        >
                            <For
                                each=move || processes.clone().into_iter()
                                key=|item| item.id
                                children=move |process| {
                                    let process = create_rw_signal(process);
                                    view! { <ProcessCard process/> }
                                }
                            />

                        </Cards>
                    }
                }
                TipCard::Industry(industry) => {
                    let industry = create_rw_signal(industry);
                    view! { <IndustryCard industry/> }
                }
                TipCard::Region(region) => {
                    let region = create_rw_signal(region);
                    view! { <RegionCard region/> }
                }
                TipCard::Factors(factors) => {
                    let factors = create_rw_signal(factors);
                    view! { <FactorsCard factors/> }
                }
                TipCard::Event(event) => {
                    let event = create_rw_signal(event);
                    view! { <EventCard event/> }
                }
                TipCard::NPC(npc) => {
                    let npc = create_rw_signal(npc);
                    view! { <NPCCard npc/> }
                }
            })
    };

    view! {
        <div class="tip-layer">
            <AnimatedShow
                when=should_show
                show_class="fade-in-180"
                hide_class="fade-out-180"
                hide_delay=Duration::from_millis(180)
            >
                <div
                    class="tip-wrapper"
                    class:overlay=move || should_show.get() && has_card()
                    ref=overlay_ref
                >
                    <div class="tip" ref=tip_ref>
                        {tip_view}
                    </div>
                    <AnimatedShow
                        when=MaybeSignal::derive(has_card)
                        show_class="bounce-in-400"
                        hide_class="bounce-out-400"
                        hide_delay=Duration::from_millis(400)
                    >
                        <div class="tip--card">{card_view}</div>
                    </AnimatedShow>
                </div>
            </AnimatedShow>
        </div>
    }
}

/// Use this component to wrap elements that should show a tooltip on hover.
#[component(transparent)]
pub fn HasTip(
    children: Children,
    #[prop(into)] tip: MaybeSignal<Tip>,
) -> impl IntoView {
    let tip_rw = expect_context::<RwSignal<TipState>>();
    let children = children()
        .nodes
        .into_iter()
        .map(|child| {
            child
                .on(ev::click, {
                    let value = tip.clone();
                    move |ev| {
                        ev.stop_immediate_propagation();
                        let tip_value = value.get();
                        tip_rw.set(TipState {
                            tip: Some(tip_value),
                            should_show: true,
                        });
                    }
                })
                // Hacky way to get tip class onto the element.
                .on(ev::pointerenter, |ev| {
                    if let Some(target) = ev.current_target() {
                        let target: web_sys::HtmlElement =
                            target
                                .dyn_into()
                                .expect("Is an HTML element");
                        target
                            .class_list()
                            .add_1("has-tip")
                            .unwrap();
                    }
                })
        })
        .collect_view();

    children
}

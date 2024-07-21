mod describe;

use std::collections::{BTreeMap, HashSet};

use crate::{
    icons::{self, fill_icons},
    with_state,
};
pub use describe::DisplayEffect;
use hes_engine::{
    events::*,
    projects::{Project, Status, Type},
};
use leptos::{
    component,
    expect_context,
    view,
    For,
    IntoView,
    RwSignal,
    Signal,
    SignalWith,
};

use super::{tip, HasTip, Tip};

// TODO
// I think what this needs to do is
// figure out the effects that have the same type and subtype,
// and if there are more than one consider the effect result to be unknown (i.e. "?")
// In fact what we should use is the effects.rs Param type for Unknown vs Known
fn outcome_effects(project: &Project) -> Vec<DisplayEffect> {
    struct Count {
        effect: DisplayEffect,
        count: usize,
        hashes: HashSet<Vec<u8>>,
    }
    let mut all_effects: BTreeMap<String, Count> =
        BTreeMap::default();
    let n_outcomes = project.outcomes.len();
    for outcome in &project.outcomes {
        for effect in &outcome.effects {
            // TODO effect.is_hidden
            let key = effect.fingerprint();
            let mut efx = DisplayEffect::from(effect);
            let hash = bincode::serialize(&effect).unwrap();
            efx.likelihood =
                Some(outcome.probability.likelihood);
            let count =
                all_effects.entry(key).or_insert_with(|| {
                    Count {
                        effect: efx,
                        count: 0,
                        hashes: HashSet::default(),
                    }
                });
            count.count += 1;
            count.hashes.insert(hash);
        }
    }

    all_effects
        .into_values()
        .map(
            |Count {
                 mut effect,
                 count,
                 hashes,
             }| {
                effect.is_unknown =
                    count != n_outcomes || hashes.len() > 1;
                effect
            },
        )
        .collect()
}
impl From<&Effect> for DisplayEffect {
    fn from(value: &Effect) -> Self {
        DisplayEffect {
            effect: value.clone(),
            likelihood: None,
            is_hidden: false,
            is_unknown: false,
        }
    }
}

pub fn active_effects(project: &Project) -> Vec<DisplayEffect> {
    let mut effects = vec![];

    if project.kind == Type::Policy && !project.is_active() {
        // Project outcome effects are secret and delayed
        effects.extend(
            project.effects.iter().map(DisplayEffect::from),
        );
    } else if project.status == Status::Inactive
        || project.status == Status::Building
    {
        effects.extend(
            project.effects.iter().map(DisplayEffect::from),
        );
        effects.extend(outcome_effects(project).into_iter());
    } else {
        effects.extend(
            project
                .active_effects()
                .iter()
                .map(DisplayEffect::from),
        );
        if let Some(id) = project.active_outcome {
            effects.extend(
                project.outcomes[id]
                    .effects
                    .iter()
                    .map(DisplayEffect::from),
            );
        }
    }

    effects
}

#[component]
pub fn Effects(
    #[prop(into)] effects: Signal<Vec<DisplayEffect>>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let items = with_state!(|state, _ui, effects| {
        let mut effects = effects
            .iter()
            .filter(|effect| !effect.is_hidden)
            .filter_map(|effect| {
                // TODO Here's where'd we also fill icons in `details.text`
                effect.tip(state).ok().map(|mut details| {
                    if effect.is_unknown {
                        details.tip.supicon =
                            Some(icons::CHANCE);
                    }
                    details.text = fill_icons(&details.text);
                    details
                })
            })
            // TODO dedupe?
            .collect::<Vec<_>>();
        effects.sort_by_key(|effect| effect.text.clone());
        effects
    });
    let class = format!("effects {}", class);

    // TODO sanitize use custom deck fields (project names, process names, etc)
    view! {
        <div class=class>
            <For
                each=move || items()
                key=|item| item.text.clone()
                children=move |item| {
                    view! {
                        <HasTip tip=item.tip>
                            <div class="effect">
                                <div class="effect--text" inner_html=item.text></div>
                            </div>
                        </HasTip>
                    }
                }
            />

        </div>
    }
}

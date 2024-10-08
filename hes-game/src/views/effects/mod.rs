mod describe;

use std::collections::{BTreeMap, HashSet};

use crate::icons::{self, fill_icons};
pub use describe::DisplayEffect;
use hes_engine::{Effect, Project, ProjectType, State, Status};
use leptos::{
    component,
    expect_context,
    view,
    with,
    For,
    IntoView,
    RwSignal,
    Signal,
};

use super::{tip, HasTip, Tip};

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

    if project.kind == ProjectType::Policy
        && !project.is_active()
    {
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
    let game = expect_context::<RwSignal<State>>();
    let items = move || {
        with!(|game, effects| {
            let mut effects = effects
                .iter()
                .filter(|effect| !effect.is_hidden)
                .filter_map(|effect| {
                    effect.tip(game).ok().map(|mut details| {
                        if effect.is_unknown {
                            details.tip.supicon =
                                Some(icons::CHANCE);
                        }
                        details.text =
                            fill_icons(&details.text);
                        details
                    })
                })
                .collect::<Vec<_>>();
            effects.sort_by_key(|effect| effect.text.clone());
            effects
        })
    };
    let class = format!("effects {}", class);

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

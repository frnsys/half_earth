use super::super::card::*;
use crate::{
    consts,
    display::AsText,
    icons::{self, HasIcon},
    memo,
    state::UIState,
    t,
    util::{scale_text, to_ws_el, ImageExt},
    views::{
        effects::{active_effects, DisplayEffect},
        tip,
        Effects,
        HasTip,
    },
};
use hes_engine::{
    Effect as EngineEffect,
    Flag,
    Group,
    Project,
    ProjectType,
    State,
};
use html::ToHtmlElement;
use leptos::*;

#[component]
pub fn ProjectCard(
    #[prop(into)] project: Signal<Project>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    let viewed = memo!(ui.viewed);
    let is_new = move || {
        with!(|project, viewed| !viewed.contains(&project.id))
    };

    let card_bg =
        move || with!(|project| card_color(&project.group).0);
    let card_fg =
        move || with!(|project| card_color(&project.group).1);

    let class = move || {
        with!(|project| {
            format!(
                "{} {}",
                if project.is_building() {
                    "in-progress"
                } else {
                    ""
                },
                if is_new() { "is-new" } else { "" }
            )
        })
    };

    let name = move || with!(|project| t!(&project.name));
    let group =
        move || with!(|project| t!(project.group.into()));
    let description = move || {
        with!(|project| t!(&project.flavor.description))
    };
    let implemented =
        move || with!(|project| project.is_online());
    let has_levels =
        move || with!(|project| !project.upgrades.is_empty());
    let level = move || with!(|project| project.level + 1);

    let name_memo = memo!(project.name);
    let name_ref = create_node_ref::<html::Div>();
    create_effect(move |_| {
        name_memo.track();
        if let Some(name_ref) = name_ref.get() {
            scale_text(
                to_ws_el(
                    name_ref
                        .parent_element()
                        .unwrap()
                        .to_leptos_element(),
                ),
                14,
            );
        }
    });

    // This is super hacky but I'm struggling to figure out
    // how to approach these problems in leptos.
    // As I understand it when a signal is updated in leptos
    // it immediately starts triggering any dependents,
    // which can lead to nested calls where an outer function
    // borrows a signal that an inner, deeper function tries
    // to borrow and can't, causing a borrow error.
    //
    // That in itself isn't necessarily a difficult problem to resolve,
    // but when that error is thrown in leptos you're pointed to
    // a line within the leptos library, not where the failed borrow
    // was attempted, not what signal the borrow failed on,
    // nor anything else that gives an idea of where to investigate.
    //
    // So you have to do a ton of trial-and-error, commenting out things
    // to try and narrow down where the error is actually happening.
    // And once you find that, then you have to figure out where in the call
    // stack the conflicting borrow is happening--again requiring an very
    // trial-and-error approach.
    //
    // A quick-and-dirty "solution" is what I'm doing here. You don't listen
    // on directly to "real" signal but instead via a proxy signal. This proxy
    // signal is updated as a side-effect when the real signal is updated,
    // but crucially it's updated *after* the current call stack is resolved
    // and any borrows are freed. Here I'm using `queue_microtask` but elsewhere
    // I'm using `use_timeout` which accomplishes the same thing, though
    // probably not as nicely.
    let plan_changes_proxy =
        create_rw_signal(with!(|ui| ui.plan_changes.clone()));
    let plan_changes_source = memo!(ui.plan_changes);
    create_effect(move |_| {
        plan_changes_source.track();
        queue_microtask(move || {
            let changes =
                ui.with_untracked(|ui| ui.plan_changes.clone());
            plan_changes_proxy.set(changes);
        });
    });

    let queued_upgrades_proxy =
        create_rw_signal(with!(|ui| ui
            .queued_upgrades
            .clone()));
    let queued_upgades_source = memo!(ui.queued_upgrades);
    create_effect(move |_| {
        queued_upgades_source.track();
        queue_microtask(move || {
            let upgrades = ui.with_untracked(|ui| {
                ui.queued_upgrades.clone()
            });
            queued_upgrades_proxy.set(upgrades);
        });
    });

    let upgrade_queued = move || {
        with!(|project, queued_upgrades_proxy| {
            queued_upgrades_proxy.get(&project.id)
                == Some(&true)
        })
    };

    let remaining_cost = move || {
        with!(|project, plan_changes_proxy| {
            if implemented() {
                0.to_string()
            } else if project.is_building() {
                match project.kind {
                    ProjectType::Policy => {
                        t!("1 planning cycle left")
                    }
                    _ => {
                        let years = project.years_remaining();
                        t!("{years} yrs left", years: years)
                    }
                }
            } else {
                let cost = if project.points > 0 {
                    project.estimate
                } else {
                    project.cost
                };
                match project.kind {
                    ProjectType::Policy => {
                        if let Some(changes) =
                            plan_changes_proxy.get(&project.id)
                        {
                            if changes.withdrawn {
                                0.to_string()
                            } else {
                                cost.to_string()
                            }
                        } else {
                            cost.to_string()
                        }
                    }
                    _ => {
                        t!("{cost} yrs", cost: cost)
                    }
                }
            }
        })
    };
    let cost_tip = move || {
        with!(|project| {
            match project.kind {
                ProjectType::Policy => tip(
                    icons::POLITICAL_CAPITAL,
                    t!("This policy costs {remainingCost} political capital to implement.", remainingCost: remaining_cost()),
                ),
                ProjectType::Initiative => tip(
                    icons::INITIATIVE,
                    t!("This will take about {remainingCost} to finish. Allocate more {kind} points to accelerate its progress.", remainingCost: remaining_cost(), kind: t!(project.kind.lower())),
                ),
                ProjectType::Research => tip(
                    icons::RESEARCH,
                    t!("This will take about {remainingCost} to finish. Allocate more {kind} points to accelerate its progress.", remainingCost: remaining_cost(), kind: t!(project.kind.lower())),
                ),
            }
        })
    };
    let is_countdown = move || {
        with!(|project| {
            project.kind != ProjectType::Policy
                || project.is_building()
        })
    };
    let show_pc_icon = move || {
        with!(|project| { project.kind == ProjectType::Policy })
    };
    let is_building =
        move || with!(|project| project.is_building());

    let parliament_suspended =
        memo!(game.flags.contains(&Flag::ParliamentSuspended));
    let player_seats = memo!(game.npcs.coalition_seats());
    let majority_satisfied = move || {
        with!(|parliament_suspended, player_seats| {
            if *parliament_suspended {
                true
            } else {
                with!(|project| {
                    let player_seats = *player_seats as f32;
                    player_seats > project.required_majority
                })
            }
        })
    };
    let warn_majority = move || {
        with!(|project| {
            project.required_majority > 0.
                && !majority_satisfied()
        })
    };
    let image =
        move || with!(|project| project.flavor.image.src());
    let has_points = move || {
        with!(|project| {
            project.kind != ProjectType::Policy
                && project.is_building()
        })
    };
    let points_display = move || {
        with!(|project| {
            (0..consts::MAX_POINTS).map(|i| {
                let tip = tip(project.kind.icon(), t!("{points} {kind} points are allocated to this project", points: project.points, kind: project.kind.lower()));
                let empty = i >= project.points;
                let icon = project.kind.icon();
                view! {
                    <HasTip tip>
                        <img class="pip" class:empty-point=empty src=icon/>
                    </HasTip>
                }
            }).collect::<Vec<_>>()
        })
    };

    let npcs = memo!(game.npcs);
    let opposers = move || {
        with!(|npcs, project| {
            project
                .opposers
                .iter()
                .map(|id| npcs[id].clone())
                .filter(|npc| !npc.locked)
                .collect::<Vec<_>>()
        })
    };
    let supporters = move || {
        with!(|npcs, project| {
            project
                .supporters
                .iter()
                .map(|id| npcs[id].clone())
                .filter(|npc| !npc.locked)
                .collect::<Vec<_>>()
        })
    };
    let has_opposers = move || !opposers().is_empty();
    let has_supporters = move || !supporters().is_empty();
    let opposers_views = move || {
        opposers()
            .into_iter()
            .map(|npc| {
                let tip = tip(npc.icon(), t!("{name} is opposed to this. If you implement it, your relationship will worsen by -<img src='{icon}' />.",
                        name: t!(&npc.name),
                        icon: icons::RELATIONSHIP,
                        ));
                view! {
                    <HasTip tip>
                        <img src=npc.icon()/>
                    </HasTip>
                }
        }).collect::<Vec<_>>()
    };
    let supporters_views = move || {
        supporters()
            .into_iter()
            .map(|npc| {
                let tip = tip(npc.icon(), t!("{name} supports this. If you implement it, your relationship will improve by +<img src='{icon}' />.",
                        name: t!(&npc.name),
                        icon: icons::RELATIONSHIP,
                        ));
                view! {
                    <HasTip tip>
                        <img src=npc.icon()/>
                    </HasTip>
                }
        }).collect::<Vec<_>>()
    };

    let passed = move || {
        with!(|project| {
            project.kind == ProjectType::Policy
                && (project.is_building()
                    || project.is_online())
        })
    };

    let visible_effect = |d: &DisplayEffect| -> bool {
        !matches!(
            d.effect,
            EngineEffect::ProjectRequest(..)
                | EngineEffect::ProcessRequest(..)
        )
    };

    let effects = move || {
        with!(|project| active_effects(project)
            .into_iter()
            .filter(visible_effect)
            .collect::<Vec<_>>())
    };

    let next_upgrade =
        move || -> Option<(usize, Vec<DisplayEffect>)> {
            with!(|project, plan_changes_proxy| {
                if project.upgrades.is_empty() {
                    None
                } else {
                    let idx = project.level;
                    if idx >= project.upgrades.len() {
                        None
                    } else {
                        let upgrade = &project.upgrades[idx];
                        let mut cost = upgrade.cost;
                        if let Some(changes) =
                            plan_changes_proxy.get(&project.id)
                        {
                            if changes.downgrades > 0 {
                                cost = 0;
                            }
                        }
                        let effects: Vec<DisplayEffect> =
                            upgrade
                                .effects
                                .iter()
                                .map(|e| e.into())
                                .collect();
                        Some((cost, effects))
                    }
                }
            })
        };
    let has_upgrade = move || {
        with!(|project| {
            project.is_active() && next_upgrade().is_some()
        })
    };
    let can_downgrade = move || {
        with!(|project| {
            project.kind == ProjectType::Policy
                && project.level > 0
        })
    };
    let has_downgrade = move || {
        with!(|project| {
            project.is_active() && can_downgrade()
        })
    };
    let prev_upgrade = move || {
        if can_downgrade() {
            with!(|project| {
                let idx = project.level as isize - 2;
                if idx < 0 {
                    let effects: Vec<DisplayEffect> = project
                        .effects
                        .iter()
                        .map(DisplayEffect::from)
                        .filter(visible_effect)
                        .collect();
                    Some((0, effects))
                } else {
                    if let Some(upgrade) =
                        project.upgrades.get(idx as usize)
                    {
                        let effects: Vec<DisplayEffect> =
                            upgrade
                                .effects
                                .iter()
                                .map(DisplayEffect::from)
                                .filter(visible_effect)
                                .collect();
                        Some((upgrade.cost, effects))
                    } else {
                        None
                    }
                }
            })
        } else {
            None
        }
    };
    let building_term = move || {
        with!(|project| match project.kind {
            ProjectType::Research => t!("Researching"),
            ProjectType::Initiative => t!("Building"),
            ProjectType::Policy => t!("Passing"),
        })
    };
    let image_attrib = move || {
        with!(|project| {
            project.flavor.image.attribution.clone()
        })
    };

    view! {
        <Card
            color=card_fg.into_signal()
            background=card_bg.into_signal()
            class=class.into_signal()
        >
            <Header slot>
                <div>{group}</div>
                <Show when=is_new>
                    <img class="new-card-icon" src="/assets/new.svg"/>
                </Show>
                <div class="project-cost">
                    <Show
                        when=implemented
                        fallback=move || {
                            view! {
                                <HasTip tip=cost_tip.into_signal()>
                                    <Show when=is_countdown>
                                        <img src=icons::TIME/>
                                    </Show>
                                    {remaining_cost}
                                    <Show when=show_pc_icon>
                                        <img src=icons::POLITICAL_CAPITAL/>
                                    </Show>
                                </HasTip>
                            }
                        }
                    >

                        <Show
                            when=has_levels
                            fallback=move || {
                                view! {
                                    <img src=icons::CHECK_BLK/>
                                    {t!("Completed")}
                                }
                            }
                        >
                            {t!("Level")}" "{level}
                        </Show>
                    </Show>
                </div>
                <img class="barcode" src="/assets/barcode.png"/>
            </Header>

            <Figure slot>
                <Show when=warn_majority>
                    <div class="project-required-majority">
                        <div>
                            <img src=icons::WARNING/>
                            {t!(
                                "Because of opposition, this requires a majority in parliament."
                            )}

                        </div>
                    </div>
                </Show>
                <img class="card-image" src=image/>
                <Show when=has_points>
                    <div class="card-tack-ul project-points">
                        {points_display}
                    </div>
                </Show>
                <Show when=has_opposers>
                    <div class="opposers">{opposers_views}</div>
                </Show>
                <Show when=has_supporters>
                    <div class="supporters">{supporters_views}</div>
                </Show>
            </Figure>

            <Body slot>
                <Show when=passed>
                    <div class="passed-stamp">
                        <img src="/assets/stamp.svg"/>
                    </div>
                </Show>
                <Effects
                    class="solo-effects"
                    effects=effects.into_signal()
                />

                {move || {
                     let is_active = with!(|project| project.is_active());
                     if is_active && let Some((cost, effects)) = next_upgrade() {
                         let is_upgrading = upgrade_queued();
                         let effects = create_rw_signal(effects);
                         Some(view! {
                            <div class="project-upgrade" class:upgrading={is_upgrading}>
                                <div class="project-upgrade--title">
                                    {move || {
                                         if is_upgrading {
                                             view! {
                                                <div>{t!("Upgrading in one planning cycle.")}</div>
                                             }.into_view()
                                         } else {
                                             view! {
                                                <div>{t!("Next Level")}</div>
                                                <div>
                                                    {cost} <img class="pip" src=icons::POLITICAL_CAPITAL/>
                                                </div>
                                             }.into_view()
                                         }
                                    }}
                                </div>
                                <Effects effects />
                            </div>
                         })
                     } else {
                         None
                     }
                }}
                <Show when=has_downgrade>
                    <div class="project-upgrade">
                        <div class="project-upgrade--title">
                            <div>{t!("Prev Level")}</div>
                        </div>
                        <Effects effects=move || prev_upgrade().unwrap().1/>
                    </div>
                </Show>
                <Show when=is_building>
                    <div class="project-status">{building_term}</div>
                </Show>
            </Body>

            <Name slot><div ref=name_ref>{name}</div></Name>
            <TopBack slot>
                <p class="card-desc">{description}</p>
            </TopBack>
            <BottomBack slot>
                <Show
                    when=move || has_opposers() || has_supporters()
                    fallback=move || {
                        view! { <div class="card-spacer"></div> }
                    }
                >

                    <div class="political-effects">
                        <div class="political-effects-title">
                            {t!("Political Effects")}
                        </div>
                        <div class="political-effects-cols">
                            <Show when=has_opposers>
                                <div class="political-effects-opposers">
                                    <div class="political-effects-label">{t!("Nay")}</div>
                                    <div class="political-effects-portraits">
                                        {opposers_views}
                                    </div>
                                </div>
                            </Show>
                            <Show when=has_supporters>
                                <div class="political-effects-supporters">
                                    <div class="political-effects-label">{t!("Yea")}</div>
                                    <div class="political-effects-portraits">
                                        {supporters_views}
                                    </div>
                                </div>
                            </Show>
                        </div>

                    </div>
                </Show>
                <div class="card-image-attribution">
                    {t!("Image:")}" "{image_attrib}
                </div>
            </BottomBack>
        </Card>
    }
}

pub fn card_color(
    group: &Group,
) -> (&'static str, &'static str) {
    match group {
        Group::Restoration => ("#247f24", "#000000"),
        Group::Protection => ("#53a553", "#000000"),
        Group::Nuclear => ("orange", "#000000"),
        Group::Agriculture => ("wheat", "#000000"),
        Group::Control => ("#d83535", "#000000"),
        Group::Population => ("#6b6bec", "#000000"),
        Group::Food => ("#f3ff56", "#000000"),
        Group::Space => ("#250441", "#d0c0e4"),
        Group::Geoengineering => ("#61688b", "#000000"),
        Group::Electrification => ("#fcba03", "#000000"),
        Group::Behavior => ("#b8ad91", "#000000"),
        Group::Limits => ("#4B5A85", "#ffffff"),
        Group::Energy => ("#fee94a", "#000000"),
        Group::Materials => ("#5f2929", "#ffffff"),
        Group::Buildings => ("#8f7ea9", "#000000"),
        Group::Cities => ("#566b6a", "#ffffff"),
        Group::Other => ("#e0e0e0", "#000000"),
    }
}

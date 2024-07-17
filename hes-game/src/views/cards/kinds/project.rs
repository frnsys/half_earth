use super::super::card::*;
use crate::{
    consts,
    display::AsText,
    icons::{self, HasIcon},
    state,
    state::GameExt,
    t,
    ui,
    views::{
        effects::{active_effects, DisplayEffect},
        tip,
        Effects,
        HasTip,
    },
};
use hes_engine::{
    events::Flag,
    projects::{Group, Project},
    years_remaining,
    ProjectType,
};
use leptos::*;

#[component]
pub fn ProjectCard(
    #[prop(into)] project: Signal<Project>,
) -> impl IntoView {
    let is_new = move || {
        // TODO
        // return !state.viewed.includes(this.ref_id);
        false
    };

    let card_bg = move || {
        project.with(|project| card_color(&project.group).0)
    };
    let card_fg = move || {
        project.with(|project| card_color(&project.group).1)
    };

    let class = move || {
        project.with(|project| {
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

    let name =
        move || project.with(|project| t!(&project.name));
    let group = move || {
        project.with(|project| t!(project.group.into()))
    };
    let description = move || {
        project.with(|project| t!(&project.flavor.description))
    };
    let implemented =
        move || project.with(|project| project.is_online());
    let has_levels = move || {
        project.with(|project| !project.upgrades.is_empty())
    };
    let level =
        move || project.with(|project| project.level + 1);

    let plan_changes = ui!(plan_changes.clone());
    let remaining_cost = move || {
        project.with(|project| {
            if implemented() {
                0.to_string()
            } else if project.is_building() {
                match project.kind {
                    ProjectType::Policy => {
                        t!("1 planning cycle left")
                    }
                    _ => {
                        let years = years_remaining(
                            project.progress,
                            project.points,
                            project.cost,
                        );
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
                            plan_changes.get().get(&project.id)
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
        project.with(|project| {
            match project.kind {
                ProjectType::Policy => tip(icons::POLITICAL_CAPITAL,
                    t!("This policy costs {remainingCost} political capital to implement.", remainingCost: remaining_cost())
                    ),
                ProjectType::Initiative => tip(icons::INITIATIVE, t!("This will take about {remainingCost} to finish. Allocate more {kind} points to accelerate its progress.", remainingCost: remaining_cost(), kind: t!(project.kind.lower()))),
                ProjectType::Research => tip(icons::RESEARCH, t!("This will take about {remainingCost} to finish. Allocate more {kind} points to accelerate its progress.", remainingCost: remaining_cost(), kind: t!(project.kind.lower())))
            }
        })
    };
    let is_countdown = move || {
        project.with(|project| {
            project.kind != ProjectType::Policy
                || project.is_building()
        })
    };
    let show_pc_icon = move || {
        project.with(|project| {
            project.kind == ProjectType::Policy
                || !project.is_building()
        })
    };
    let is_building =
        move || project.with(|project| project.is_building());

    let parliament_suspended =
        state!(flags.contains(&Flag::ParliamentSuspended));
    let player_seats = state!(player_seats());
    let majority_satisfied = move || {
        if parliament_suspended.get() {
            true
        } else {
            project.with(|project| {
                let player_seats = player_seats.get() as f32;
                player_seats > project.required_majority
            })
        }
    };
    let warn_majority = move || {
        project.with(|project| {
            project.required_majority > 0.
                && !majority_satisfied()
        })
    };
    let image = move || {
        project.with(|project| {
            format!(
                "/assets/content/images/{}",
                project.flavor.image.fname
            )
        })
    };
    let has_points = move || {
        project.with(|project| {
            project.kind != ProjectType::Policy
                && project.is_building()
        })
    };
    let proj_points_test =
        move || project.with(|project| project.points);
    let points_display = move || {
        project.with(move |project| {
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

    let npcs = state!(npcs.clone());
    let opposers = move || {
        let npcs = npcs.get();
        project.with(|project| {
            project
                .opposers
                .iter()
                .map(|id| npcs[*id].clone())
                .filter(|npc| !npc.locked)
                .collect::<Vec<_>>()
        })
    };
    let supporters = move || {
        let npcs = npcs.get();
        project.with(|project| {
            project
                .supporters
                .iter()
                .map(|id| npcs[*id].clone())
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
        project.with(|project| {
            project.kind == ProjectType::Policy
                && (project.is_building()
                    || project.is_online())
        })
    };
    let effects =
        move || project.with(|project| active_effects(project));

    let queued_upgrades = ui!(queued_upgrades.clone());
    let upgrade_queued = move || {
        project.with(|project| {
            queued_upgrades.get().get(&project.id)
                == Some(&true)
        })
    };
    let next_upgrade = move || {
        project.with(|project| {
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
                        plan_changes.get().get(&project.id)
                    {
                        if changes.downgrades > 0 {
                            cost = 0;
                        }
                    }
                    let effects: Vec<DisplayEffect> = upgrade
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
        project.with(|project| {
            project.is_active() && next_upgrade().is_some()
        })
    };
    let can_downgrade = move || {
        project.with(|project| {
            project.kind == ProjectType::Policy
                && project.level > 0
        })
    };
    let has_downgrade = move || {
        project.with(|project| {
            project.is_active() && can_downgrade()
        })
    };
    let prev_upgrade = move || {
        if can_downgrade() {
            project.with(|project| {
                let idx = project.level as isize - 2;
                if idx < 0 {
                    let effects: Vec<DisplayEffect> = project
                        .effects
                        .iter()
                        .map(DisplayEffect::from)
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
        project.with(|project| match project.kind {
            ProjectType::Research => t!("Researching"),
            ProjectType::Initiative => t!("Building"),
            ProjectType::Policy => t!("Passing"),
        })
    };
    let image_attrib = move || {
        project.with(|project| {
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

                            {t!("Level")}
                            {level}
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

                <Show when=has_upgrade>
                    <div class="project-upgrade" class:upgrading=upgrade_queued>
                        <div class="project-upgrade--title">
                            <Show
                                when=upgrade_queued
                                fallback=move || {
                                    next_upgrade()
                                        .map(|(cost, effects)| {
                                            view! {
                                                <div>{t!("Next Level")}</div>
                                                <div>
                                                    {cost} <img class="pip" src=icons::POLITICAL_CAPITAL/>
                                                </div>
                                            }
                                        })
                                }
                            >

                                <div>{t!("Upgrading in one planning cycle.")}</div>
                            </Show>
                        </div>
                        <Effects effects=move || next_upgrade().unwrap().1/>
                    </div>
                </Show>
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

            <Name slot>{name}</Name>
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
                    {t!("Image:")} {image_attrib}
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

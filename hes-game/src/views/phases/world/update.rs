use leptos::*;

use crate::{
    display::*,
    icons::{self, HasIcon},
    memo,
    t,
    util::{scale_text, to_ws_el, ImageExt},
    views::{
        effects::active_effects,
        events::Dialogue,
        intensity::{self, IntensityIcon},
        tip,
        Effects,
        HasTip,
    },
};
use hes_engine::{KindMap, State, Update as EngineUpdate};

#[component]
pub fn Updates(
    updates: RwSignal<Vec<EngineUpdate>>,
    #[prop(into)] on_done: Callback<()>,
) -> impl IntoView {
    let idx = create_rw_signal(0);
    let has_update =
        move || with!(|idx, updates| *idx < updates.len());
    let n_updates = move || with!(|updates| updates.len());
    let next_update = move || {
        if idx.get() + 1 < n_updates() {
            update!(|idx| *idx += 1);
        } else {
            on_done.call(());
        }
    };
    let update =
        move || with!(|idx, updates| updates[*idx].clone());

    view! {
        <Show when=has_update>
            <Update update on_done=move |_| next_update()/>
            </Show>
    }
}

#[component]
fn Update(
    #[prop(into)] update: Signal<EngineUpdate>,
    #[prop(into)] on_done: Callback<()>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();

    let init_can_close = with!(|update| update.is_region());
    let (can_close, set_can_close) =
        create_signal(init_can_close);

    let is_region = move || with!(|update| update.is_region());

    let title = move || {
        with!(|update| match update {
            EngineUpdate::Project { .. } => {
                t!("Project Completed")
            }
            EngineUpdate::Policy { .. } => t!("Policy Outcome"),
            EngineUpdate::Region { up: true, .. } => {
                t!("Region Developed")
            }
            EngineUpdate::Region { up: false, .. } => {
                t!("Region Contracted")
            }
        })
    };

    let regions = memo!(game.world.regions);
    let projects = memo!(game.world.projects);
    let image = move || {
        let image = with!(|projects, regions, update| {
            match update {
                EngineUpdate::Project { id }
                | EngineUpdate::Policy { id } => {
                    let proj = &projects[id];
                    proj.flavor.image.src()
                }
                EngineUpdate::Region { id, .. } => {
                    let region = &regions[id];
                    region.flavor.image.src()
                }
            }
        });
        format!("url('{image}')",)
    };

    let image_attrib = move || {
        with!(|projects, regions, update| {
            match update {
                EngineUpdate::Project { id }
                | EngineUpdate::Policy { id } => {
                    let proj = &projects[id];
                    proj.flavor.image.attribution.clone()
                }
                EngineUpdate::Region { id, .. } => {
                    let region = &regions[id];
                    region.flavor.image.attribution.clone()
                }
            }
        })
    };

    let name = move || {
        with!(|projects, regions, update| {
            match update {
                EngineUpdate::Project { id }
                | EngineUpdate::Policy { id } => {
                    let proj = &projects[id];
                    t!(&proj.name)
                }
                EngineUpdate::Region { id, .. } => {
                    let region = &regions[id];
                    t!(&region.name)
                }
            }
        })
    };

    let per_capita_demand = memo!(game.world.per_capita_demand);
    let outcomes = move || {
        with!(|projects, regions, update, per_capita_demand| {
            match update {
                EngineUpdate::Project { id }
                | EngineUpdate::Policy { id } => {
                    let proj = &projects[id];

                    set_can_close.set_untracked(
                        !proj.active_outcome.is_some(),
                    );

                    let effects = active_effects(proj);
                    let outcome_dialogue = proj.active_outcome.and_then(|id| {
                        proj.flavor.outcomes.get(id).cloned()
                    }).map(|outcome| {
                        let (dialogue, _) = create_signal(outcome);
                        view! {
                            <Dialogue dialogue on_start=move |_| {
                                set_can_close.set(false);
                            } on_done=move |_| {
                                set_can_close.set(true);
                                on_done.call(());
                            } />
                        }
                    });

                    // TODO this is a hack
                    let (sig, _) = create_signal(effects);

                    let effects_ref =
                        create_node_ref::<html::Div>();
                    create_effect(move |_| {
                        if let Some(effects) = effects_ref.get()
                        {
                            scale_text(to_ws_el(effects), 7);
                        }
                    });

                    view! {
                        <div class="event--effects" ref={effects_ref}>
                            <Effects effects=sig />
                        </div>
                        {outcome_dialogue}
                    }
                    .into_view()
                }
                EngineUpdate::Region { id, up } => {
                    let region = &regions[id];

                    let (prev_income, next_income) = if *up {
                        let next = region.income.level();
                        let prev = next - 1;
                        (prev, next)
                    } else {
                        let next = region.income.level();
                        let prev = next + 1;
                        (prev, next)
                    };

                    // This is somewhat redundant so we are only using
                    // literals in the `t!(...)` macro, which helps with extracting
                    // translation strings.
                    let html = if *up {
                        t!("This region's income level has increased to <strong>{income}</strong>. Demand for <img src='{iconElec}'>electricity, <img src='{iconFuel}'>fuel, <img src='{iconPCals}'>plant and <img src='{iconACals}'>animal-based food has been updated.",
                            income: region.income.lower(),
                            iconFuel: icons::FUEL,
                            iconElec: icons::ELECTRICITY,
                            iconPCals: icons::PLANT_CALORIES,
                            iconACals: icons::ANIMAL_CALORIES)
                    } else {
                        t!("This region's income level has contracted to <strong>{income}</strong>. Demand for <img src='{iconElec}'>electricity, <img src='{iconFuel}'>fuel, <img src='{iconPCals}'>plant and <img src='{iconACals}'>animal-based food has been updated.",
                            income: region.income.lower(),
                            iconFuel: icons::FUEL,
                            iconElec: icons::ELECTRICITY,
                            iconPCals: icons::PLANT_CALORIES,
                            iconACals: icons::ANIMAL_CALORIES)
                    };
                    let prev_tip = tip(
                        icons::WEALTH,
                        t!("This region's previous income level."),
                    );
                    let next_tip = tip(
                        icons::WEALTH,
                        t!("This region's new income level."),
                    );

                    let mut prev_region = region.clone();
                    prev_region.set_income_level(prev_income);

                    let demand =
                        region.demand(per_capita_demand);
                    let prev_demand =
                        prev_region.demand(per_capita_demand);
                    let pop = region.population;
                    let demand_changes = demand.items().map(|(output, demand)| {
                        let region_per_capita_demand = demand / pop;
                        let intensity = intensity::output_intensity(region_per_capita_demand, output);
                        let prev_region_per_capita_demand = prev_demand[output] / pop;
                        let prev_intensity = intensity::output_intensity(prev_region_per_capita_demand, output);

                        let prev_tip = tip(output.icon(), t!("This region's previous demand for {output}.", output: output.lower()));
                        let next_tip = tip(output.icon(), t!("This region's new demand for {output}.", output: output.lower()));

                        view! {
                            <div class="event--icon-change">
                                <HasTip tip=prev_tip>
                                    <IntensityIcon
                                    icon=output.icon() intensity=move || prev_intensity />
                                </HasTip>
                                <img src=icons::ARROW_RIGHT_LIGHT />
                                <HasTip tip=next_tip>
                                    <IntensityIcon
                                icon=output.icon() intensity=move || intensity />
                                </HasTip>
                                </div>

                        }
                    }).to_vec();

                    view! {
                        <div class="event--outcome" inner_html=html />
                            <div class="event--icon-changes">
                                <div class="event--icon-change">
                                    <HasTip tip=prev_tip>
                                        <IntensityIcon
                                            icon=icons::WEALTH
                                            intensity=move || prev_income + 1
                                            invert=true
                                        />
                                    </HasTip>
                                    <img src=icons::ARROW_RIGHT_LIGHT/>
                                    <HasTip tip=next_tip>
                                        <IntensityIcon
                                        icon=icons::WEALTH
                                        intensity=move || next_income + 1
                                        invert=true
                                        />
                                    </HasTip>
                                </div>
                            </div>
                            <div class="event--icon-changes event--icon-changes-group">
                                {demand_changes}
                        </div>

                    }
                    .into_view()
                }
            }
        })
    };

    let try_done = move |_| {
        if can_close.get() {
            on_done.call(());
        }
    };

    let attribution = move || {
        let attrib = image_attrib();
        if attrib.trim().is_empty() {
            "".into()
        } else {
            format!("{} {attrib}", t!("Image:"))
        }
    };

    view! {
        <div
            class="event project-completed"
            style:background-image=image
            on:click=try_done
            class:regionup=is_region>
              <div class="event--body">
                  <div class="arc">{title}</div>
                  <div class="image-attribution">
                      {attribution}
                  </div>
                    <div class="event--name">{name}</div>
                  {outcomes}
              </div>
            </div>
    }
}

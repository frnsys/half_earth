use leptos::*;

use crate::{
    display::*,
    icons::{self, HasIcon},
    t,
    util::ImageExt,
    views::{
        effects::active_effects,
        events::Dialogue,
        intensity::{self, IntensityIcon},
        tip,
        Effects,
        HasTip,
    },
    with_state,
};
use hes_engine::game::Update as EngineUpdate;

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
    let init_can_close =
        update.with(|update| update.is_region());
    let (can_close, set_can_close) =
        create_signal(init_can_close);

    let is_region =
        move || update.with(|update| update.is_region());

    let title = move || {
        update.with(|update| match update {
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

    let image = with_state!(|state, ui, update| {
        match update {
            EngineUpdate::Project { id }
            | EngineUpdate::Policy { id } => {
                let proj = &state.world.projects[id];
                proj.flavor.image.src()
            }
            EngineUpdate::Region { id, .. } => {
                let region = &state.world.regions[id];
                region.flavor.image.src()
            }
        }
    });

    let image_attrib = with_state!(|state, ui, update| {
        match update {
            EngineUpdate::Project { id }
            | EngineUpdate::Policy { id } => {
                let proj = &state.world.projects[id];
                proj.flavor.image.attribution.clone()
            }
            EngineUpdate::Region { id, .. } => {
                let region = &state.world.regions[id];
                region.flavor.image.attribution.clone()
            }
        }
    });

    let name = with_state!(|state, ui, update| {
        match update {
            EngineUpdate::Project { id }
            | EngineUpdate::Policy { id } => {
                let proj = &state.world.projects[id];
                t!(&proj.name)
            }
            EngineUpdate::Region { id, .. } => {
                let region = &state.world.regions[id];
                t!(&region.name)
            }
        }
    });

    let outcomes = with_state!(|state, ui, update| {
        match update {
            EngineUpdate::Project { id }
            | EngineUpdate::Policy { id } => {
                let proj = &state.world.projects[id];

                let effects = active_effects(proj);
                let outcome_dialogue = proj.active_outcome.map(|id| {
                    let (dialogue, _) = create_signal(proj.flavor.outcomes[id].clone());
                    view! {
                        <Dialogue dialogue on_start=move |_| {
                            set_can_close.set(false)
                        } on_done=move |_| {
                            set_can_close.set(true)
                        } />
                    }
                });

                // TODO this is a hack
                let (sig, _) = create_signal(effects);

                view! {
                    <div class="event--effects">
                        <Effects effects=sig />
                    </div>
                }
                .into_view()
            }
            EngineUpdate::Region { id, up } => {
                let region = &state.world.regions[id];

                let (prev_income, next_income) = if *up {
                    let next = region.income_level();
                    let prev = next - 1;
                    (prev, next)
                } else {
                    let next = region.income_level();
                    let prev = next + 1;
                    (prev, next)
                };

                // Ugh
                let change = if *up {
                    "increased"
                } else {
                    "contracted"
                };
                let html = t!(&format!("This region's income level has {change} to <strong>{{income}}</strong>. Demand for <img src='{{iconElec}}'>electricity, <img src='{{iconFuel}}'>fuel, <img src='{{iconPCals}}'>plant and <img src='{{iconACals}}'>animal-based food has been updated."),
                    income: region.income.lower(),
                    iconFuel: icons::FUEL,
                    iconElec: icons::ELECTRICITY,
                    iconPCals: icons::PLANT_CALORIES,
                    iconACals: icons::ANIMAL_CALORIES);
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

                let per_capita_demand =
                    state.world.output_demand;
                let demand = region.demand(&per_capita_demand);
                let prev_demand =
                    prev_region.demand(&per_capita_demand);
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
    });

    let try_done = move |_| {
        if can_close.get() {
            on_done.call(());
        }
    };

    view! {
        <div
            class="event project-completed"
            style:background-image=image
            on:click=try_done
            class:regionup=is_region
        >
            <div class="event--body">
                <div class="arc">{title}</div>
                <div class="image-attribution">
                    {t!("Image:")}" "{image_attrib}
                </div>
                <div class="event--name">{name}</div>
                {outcomes}
            </div>
        </div>
    }
}

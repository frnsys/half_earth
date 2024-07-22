use crate::{infinite_list, inputs::*};
use hes_engine::{
    kinds::Output,
    npcs::NPC,
    projects::{
        Cost,
        Factor,
        FactorKind,
        Group,
        Project,
        Type,
    },
    world::World,
    Collection,
};
use leptos::*;

#[component]
fn Project(
    signal: (Signal<Project>, SignalSetter<Project>),
) -> impl IntoView {
    let (read, write) = signal;
    let project = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(project.get());
    });

    let npcs = expect_context::<Signal<Collection<Ref<NPC>>>>();

    view! {
        <div class="project">
            <div class="name">
                <TextInput signal=slice!(project.name) />
                <div class="item-lock">
                    <ToggleInput
                        label="Locked"
                        tooltip=true
                        icons=("🔒Locked", "🔓Unlocked")
                        help="If this project is locked at the start."
                        signal=slice!(project.locked) />
                </div>
            </div>
            <div class="item-form">
                <div class="input-groups left-main-col">
                    <ImageInput signal=slice!(project.flavor.image) />
                </div>
                <div class="input-groups">
                    <EnumInput
                        label="Type"
                        help="The type of project."
                        signal=slice!(project.kind) />
                    <EnumInput
                        label="Category"
                        help="The project's category."
                        signal=slice!(project.group) />
                    <ToggleInput
                        label="Ongoing"
                        help="Is this a one-and-done project, or does it need continued maintenance?"
                        signal=slice!(project.ongoing) />
                    <Show when=move || with!(|project| project.kind == Type::Initiative)>
                        <ToggleInput
                            label="Gradual"
                            help="Does this project have to be 100% finished before the effects occur, or do they develop as the project is developed?"
                            signal=slice!(project.gradual) />
                    </Show>
                    <Cost project />
                </div>
            </div>

            <div class="item-form">
                <MultiEntitySelect
                    label="Supporters"
                    help="NPCs that support this project."
                    signal=slice!(project.supporters)
                    opts=npcs
                    />
                <MultiEntitySelect
                    label="Opposers"
                    help="NPCs that oppose this project."
                    signal=slice!(project.opposers)
                    opts=npcs
                    />
            </div>

            <div class="item-form effects-form">
                <Effects
                    effects=slice!(project.effects) />
            </div>
        </div>
    }
}

#[component]
fn Cost(project: RwSignal<Project>) -> impl IntoView {
    // Initialize cached values.
    let base_cost = project
        .with_untracked(|project| project.base_cost.clone());
    let multiplier_ = create_rw_signal(match base_cost {
        Cost::Dynamic(mult, _) => mult,
        _ => 0.01,
    });
    let factor_ = create_rw_signal(match base_cost {
        Cost::Dynamic(_, factor) => factor,
        _ => Factor::Income,
    });
    let fixed_cost = create_rw_signal(match base_cost {
        Cost::Fixed(cost) => cost,
        _ => 10,
    });

    let cost_view = move || {
        let base_cost =
            with!(|project| project.base_cost.clone());
        match base_cost {
            Cost::Fixed(cost) => {
                let label =
                    with!(|project| match project.kind {
                        Type::Policy => "Political Capital",
                        _ => "Build Years",
                    });
                view! {
                    <NumericInput
                        label=label
                        help="A fixed project cost."
                        signal=create_slice(project,
                            move |project| match project.base_cost {
                                Cost::Fixed(cost) => cost,
                                _ => fixed_cost.get()
                            },
                            move |project, val| {
                                project.base_cost = Cost::Fixed(val);
                                fixed_cost.set(val);
                            }) />
                }.into_view()
            }
            Cost::Dynamic(multiplier, factor) => {
                view! {
                    <EnumInput
                        label="Factor"
                        help="The factor to use for computing the cost."
                        signal=create_slice(project,
                            move |project| {
                                let factor = match project.base_cost {
                                    Cost::Dynamic(_, factor) => factor,
                                    _ => factor_.get()
                                };
                                FactorKind::from(factor)
                            },
                            move |project, factor_kind: FactorKind| {
                                let multiplier = match project.base_cost {
                                    Cost::Dynamic(multiplier, _) => multiplier,
                                    _ => multiplier_.get()
                                };
                                let factor = factor_kind.into();
                                project.base_cost = Cost::Dynamic(multiplier, factor);
                                factor_.set(factor);
                            }) />
                    <NumericInput
                        label="Factor Multiplier"
                        help="The project's cost equals this value multiplie by the factor's value."
                        signal=create_slice(project,
                            move |project| match project.base_cost {
                                Cost::Dynamic(multiplier, _) => multiplier,
                                _ => multiplier_.get()
                            },
                            move |project, val| {
                                let factor = match project.base_cost {
                                    Cost::Dynamic(_, factor) => factor,
                                    _ => factor_.get()
                                };
                                project.base_cost = Cost::Dynamic(multiplier, factor);
                                multiplier_.set(multiplier);
                            }) />

                    // NOTE: There is some problem here where if I use `<Show>`
                    // or a closure (`move || { .. }`) when the inner component
                    // is to render I get an "already borrowed" error.
                    // Just rendering the component and hiding it works though.
                    <div style:display=move || {
                        if with!(|factor_| matches!(factor_, Factor::Output(..))) {
                            "block"
                        } else {
                            "none"
                        }
                    }>
                        <EnumInput
                            label="Output Type"
                            help="The output to use for the demand factor."
                            signal=create_slice(project,
                                move |project| {
                                    match project.base_cost {
                                        Cost::Dynamic(_, Factor::Output(output)) => output,
                                        _ => Output::default()
                                    }
                                },
                                move |project, output: Output| {
                                    let multiplier = match project.base_cost {
                                        Cost::Dynamic(multiplier, _) => multiplier,
                                        _ => multiplier_.get()
                                    };
                                    let factor = Factor::Output(output);
                                    project.base_cost = Cost::Dynamic(multiplier, factor);
                                    factor_.set(factor);
                                }) />
                    </div>
                }.into_view()
            }
        }
    };

    let is_dynamic = move || {
        with!(|project| matches!(
            project.base_cost,
            Cost::Dynamic(..)
        ))
    };
    let is_static = move || {
        with!(|project| !matches!(
            project.base_cost,
            Cost::Dynamic(..)
        ))
    };

    view! {
        <div class="project-cost">
            <div class="project-cost-type-toggle">
                <span class:selected=is_static on:click=move |_| {
                    if !is_static() {
                        update!(|project| {
                            project.base_cost = Cost::Fixed(fixed_cost.get());
                        });
                    }
                }>Static</span>
                <span class:selected=is_dynamic on:click=move |_| {
                    if !is_dynamic() {
                        update!(|project| {
                            project.base_cost = Cost::Dynamic(multiplier_.get(), factor_.get());
                        });
                    }
                }>Dynamic</span>Cost
            </div>
            <div class="input-help">If this project uses a dynamically-calculated cost.</div>
            {cost_view}
        </div>
    }
}

infinite_list!(Projects, Project, projects);

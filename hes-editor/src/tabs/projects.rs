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
            </div>
            <ImageInput signal=slice!(project.flavor.image) />
            <EnumInput
                label="Type"
                help="The type of project."
                signal=slice!(project.kind) />
            <EnumInput
                label="Category"
                help="The project's category."
                signal=slice!(project.group) />
            <ToggleInput
                label="Locked"
                help="If this project is locked at the start."
                signal=slice!(project.locked) />
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
            <Effects
                effects=slice!(project.effects) />

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
                        Type::Policy =>
                            "Political Capital Cost",
                        _ => "Years to Completion",
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

    view! {
        <div class="project-cost">
            <ToggleInput
                label="Dynamic Cost"
                help="If this project uses a dynamically-calculated cost."
                signal=create_slice(project,
                    move |project| matches!(project.base_cost, Cost::Dynamic(..)),
                    move |project, is_dynamic| {
                        if is_dynamic {
                            project.base_cost = Cost::Dynamic(multiplier_.get(), factor_.get());
                        } else {
                            project.base_cost = Cost::Fixed(fixed_cost.get());
                        }
                    }) />
            {cost_view}
        </div>
    }
}

infinite_list!(Projects, Project, projects);

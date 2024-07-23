use crate::{infinite_list, inputs::*, subsignal};
use hes_engine::{
    events::Probability,
    kinds::Output,
    npcs::NPC,
    projects::{
        Cost,
        Factor,
        FactorKind,
        Group,
        Outcome,
        Project,
        Type,
        Upgrade,
    },
    world::World,
    Collection,
};
use leptos::*;

#[component]
fn Project(
    signal: (Signal<Project>, SignalSetter<Project>),
) -> impl IntoView {
    let project = signal;

    let npcs = expect_context::<Signal<Collection<Ref<NPC>>>>();

    view! {
        <div class="project" id={move || signal.0.with(|signal| signal.id.to_string())}>
            <div class="name">
                <TextInput signal=subsignal!(project.name) />
                <div class="item-lock">
                    <ToggleInput
                        label="Locked"
                        tooltip=true
                        icons=("🔒Locked", "🔓Unlocked")
                        help="If this project is locked at the start."
                        signal=subsignal!(project.locked) />
                </div>
            </div>
            <div class="item-form">
                <div class="input-groups left-main-col">
                    <ImageInput signal=subsignal!(project.flavor.image) />
                </div>
                <div class="input-groups">
                    <EnumInput
                        label="Type"
                        help="The type of project."
                        signal=subsignal!(project.kind) />
                    <EnumInput
                        label="Category"
                        help="The project's category."
                        signal=subsignal!(project.group) />
                    <ToggleInput
                        label="Ongoing"
                        help="Is this a one-and-done project, or does it need continued maintenance?"
                        signal=subsignal!(project.ongoing) />
                    <Show when=move || project.0.with(|project| project.kind == Type::Initiative)>
                        <ToggleInput
                            label="Gradual"
                            help="Does this project have to be 100% finished before the effects occur, or do they develop as the project is developed?"
                            signal=subsignal!(project.gradual) />
                    </Show>
                    <Cost project />
                </div>
            </div>

            <div class="item-form">
                <MultiEntitySelect
                    label="Supporters"
                    help="NPCs that support this project."
                    signal=subsignal!(project.supporters)
                    opts=npcs
                    />
                <MultiEntitySelect
                    label="Opposers"
                    help="NPCs that oppose this project."
                    signal=subsignal!(project.opposers)
                    opts=npcs
                    />
            </div>

            <div class="item-form effects-form">
                <Effects
                    double_col=true
                    effects=subsignal!(project.effects) />
            </div>

            <div class="item-form upgrades-form">
                <Upgrades upgrades=subsignal!(project.upgrades) />
            </div>

            <div class="item-form outcomes-form">
                <Outcomes outcomes=subsignal!(project.outcomes) />
            </div>

            <div class="item-form notes-form">
                <TextArea label="Notes" help="Optional notes" signal=subsignal!(project.notes) />
            </div>
        </div>
    }
}

#[component]
fn Cost(
    project: (Signal<Project>, SignalSetter<Project>),
) -> impl IntoView {
    let (read, write) = project;

    // Initialize cached values.
    let base_cost = read
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
        let base_cost = with!(|read| read.base_cost.clone());
        match base_cost {
            Cost::Fixed(cost) => {
                let label = with!(|read| match read.kind {
                    Type::Policy => "Political Capital",
                    _ => "Build Years",
                });
                view! {
                    <NumericInput
                        label=label
                        help="A fixed project cost."
                        signal=(
                            Signal::derive(
                                move || with!(|read| match read.base_cost {
                                    Cost::Fixed(cost) => cost,
                                    _ => fixed_cost.get()
                                })),
                            SignalSetter::map(move |val| {
                                let mut project = read.get();
                                project.base_cost = Cost::Fixed(val);
                                fixed_cost.set(val);
                                write.set(project);
                            })
                        ) />
                }.into_view()
            }
            Cost::Dynamic(multiplier, factor) => {
                view! {
                    <EnumInput
                        label="Factor"
                        help="The factor to use for computing the cost."
                        signal=(
                            Signal::derive(
                                move || with!(|read| {
                                    let factor = match read.base_cost {
                                        Cost::Dynamic(_, factor) => factor,
                                        _ => factor_.get()
                                    };
                                    FactorKind::from(factor)
                                })),
                            SignalSetter::map(move |factor_kind: FactorKind| {
                                let mut project = read.get();
                                let multiplier = match project.base_cost {
                                    Cost::Dynamic(multiplier, _) => multiplier,
                                    _ => multiplier_.get()
                                };
                                let factor = factor_kind.into();
                                project.base_cost = Cost::Dynamic(multiplier, factor);
                                factor_.set(factor);
                                write.set(project);
                            })
                        ) />
                    <NumericInput
                        label="Factor Multiplier"
                        help="The project's cost equals this value multiplie by the factor's value."
                        signal=(
                            Signal::derive(
                                move || with!(|read| match read.base_cost {
                                    Cost::Dynamic(multiplier, _) => multiplier,
                                    _ => multiplier_.get()
                                })),
                            SignalSetter::map(move |val| {
                                let mut project = read.get();
                                let factor = match project.base_cost {
                                    Cost::Dynamic(_, factor) => factor,
                                    _ => factor_.get()
                                };
                                project.base_cost = Cost::Dynamic(multiplier, factor);
                                multiplier_.set(multiplier);
                                write.set(project);
                            })
                        ) />

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
                            signal=(
                                Signal::derive(
                                    move || with!(|read| match read.base_cost {
                                        Cost::Dynamic(_, Factor::Output(output)) => output,
                                        _ => Output::default()
                                    })),
                                SignalSetter::map(move |output: Output| {
                                    let mut project = read.get();
                                    let multiplier = match project.base_cost {
                                        Cost::Dynamic(multiplier, _) => multiplier,
                                        _ => multiplier_.get()
                                    };
                                    let factor = Factor::Output(output);
                                    project.base_cost = Cost::Dynamic(multiplier, factor);
                                    factor_.set(factor);
                                    write.set(project);
                                })
                            ) />
                    </div>
                }.into_view()
            }
        }
    };

    let is_dynamic = move || {
        with!(|read| matches!(
            read.base_cost,
            Cost::Dynamic(..)
        ))
    };
    let is_static = move || {
        with!(|read| !matches!(
            read.base_cost,
            Cost::Dynamic(..)
        ))
    };

    view! {
        <div class="project-cost">
            <div class="project-cost-type-toggle">
                <span class:selected=is_static on:click=move |_| {
                    if !is_static() {
                        let mut project = read.get();
                        project.base_cost = Cost::Fixed(fixed_cost.get());
                        write.set(project);
                    }
                }>Static</span>
                <span class:selected=is_dynamic on:click=move |_| {
                    if !is_dynamic() {
                        let mut project = read.get();
                        project.base_cost = Cost::Dynamic(multiplier_.get(), factor_.get());
                        write.set(project);
                    }
                }>Dynamic</span>Cost
            </div>
            <div class="input-help">If this project uses a dynamically-calculated cost.</div>
            {cost_view}
        </div>
    }
}

#[component]
fn Upgrade<F>(
    upgrade: (Signal<Upgrade>, SignalSetter<Upgrade>),
    on_remove: F,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <div class="mutable-list-item-header">
            <NumericInput
                label="Cost"
                help="The upgrade cost."
                signal=subsignal!(upgrade.cost) />
            <div class="mutable-list-item-remove" title="Ctrl-click to remove without confirmation." on:click=on_remove>"✗"</div>
        </div>
        <Effects
            effects=subsignal!(upgrade.effects) />
    }
}

#[component]
fn Upgrades(
    upgrades: (
        Signal<Vec<Upgrade>>,
        SignalSetter<Vec<Upgrade>>,
    ),
) -> impl IntoView {
    let (read, write) = upgrades;

    view! {
        <div class="upgrades mutable-list mutable-list-double-col mutable-list-sortable">
            <div class="mutable-list-header">
                <h2>Upgrades</h2>
                <div class="mutable-list-add">
                    <div class="mutable-list-add-button" on:click=move |_| {
                        let upgrade = Upgrade::default();
                        let mut upgrades = read.get();
                        upgrades.insert(0, upgrade);
                        write.set(upgrades);
                    }>+Add</div>
                </div>
            </div>
            {move || {
                 let empty = with!(|read| read.is_empty());
                 if empty {
                     Some(view! {
                         <div class="empty">No upgrades defined.</div>
                     })
                 } else {
                     None
                 }
            }}
            <div class="mutable-list-items">
                {move || {
                     let n_upgrades =
                         with!(|read| read.len());
                     (0..n_upgrades).map(|i| {
                         view! {
                             <div class="project-upgrade mutable-list-item">
                                 <div class="mutable-list-item-index">
                                    <div class="move-up" class:hidden={i == 0} on:click=move |_| {
                                         let mut upgrades = read.get();
                                         upgrades.swap(i, i-1);
                                         write.set(upgrades);
                                    }>"⯅"</div>
                                    {i}
                                    <div class="move-down" class:hidden={i == n_upgrades - 1} on:click=move |_| {
                                         let mut upgrades = read.get();
                                         upgrades.swap(i, i+1);
                                         write.set(upgrades);
                                    }>"⯆"</div>
                                 </div>
                                 <Upgrade
                                     on_remove=move |ev: ev::MouseEvent| {
                                         let msg = "Are you sure you want to remove this upgrade?";
                                         if ev.ctrl_key() || window().confirm_with_message(msg).unwrap() {
                                             let mut upgrades = read.get();
                                             upgrades.remove(i);
                                             write.set(upgrades);
                                         }
                                     }
                                     upgrade=subsignal!(upgrades[i]) />
                            </div>
                         }
                     }).collect::<Vec<_>>()
                 }}
            </div>
        </div>
    }
}

#[component]
fn Outcome<F>(
    outcome: (Signal<Outcome>, SignalSetter<Outcome>),
    on_remove: F,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <div class="mutable-list-item-header">
            <EnumInput
                label="Likelihood"
                help="The likelihood when all conditions are met."
                signal=subsignal!(outcome.probability.likelihood) />
            <div class="mutable-list-item-remove" title="Ctrl-click to remove without confirmation." on:click=on_remove>"✗"</div>
        </div>
        <Conditions
            conditions=subsignal!(outcome.probability.conditions) />
        <Effects
            effects=subsignal!(outcome.effects) />
    }
}

#[component]
fn Outcomes(
    outcomes: (
        Signal<Vec<Outcome>>,
        SignalSetter<Vec<Outcome>>,
    ),
) -> impl IntoView {
    let (read, write) = outcomes;

    view! {
        <div class="outcomes mutable-list mutable-list-double-col mutable-list-sortable">
            <div class="mutable-list-header">
                <h2>Outcomes</h2>
                <div class="mutable-list-add">
                    <div class="mutable-list-add-button" on:click=move |_| {
                        let outcome = Outcome::default();
                        let mut outcomes = read.get();
                        outcomes.insert(0, outcome);
                        write.set(outcomes);
                    }>+Add</div>
                </div>
            </div>
            <div class="input-help">"Outcomes are checked in their defined order, so you should order them from least likely to most likely. For example, if you have a guaranteed outcome first, then that's the one that will always trigger. You should move it to the end so other outcomes can be checked before it."</div>
            {move || {
                 let empty = with!(|read| read.is_empty());
                 if empty {
                     Some(view! {
                         <div class="empty">No outcomes defined.</div>
                     })
                 } else {
                     None
                 }
            }}
            <div class="mutable-list-items">
                {move || {
                     let n_outcomes =
                         with!(|read| read.len());
                     (0..n_outcomes).map(|i| {
                         view! {
                             <div class="project-outcome mutable-list-item">
                                 <div class="mutable-list-item-index">
                                    <div class="move-up" class:hidden={i == 0} on:click=move |_| {
                                         let mut outcomes = read.get();
                                         outcomes.swap(i, i-1);
                                         write.set(outcomes);
                                    }>"⯅"</div>
                                    {i}
                                    <div class="move-down" class:hidden={i == n_outcomes - 1} on:click=move |_| {
                                         let mut outcomes = read.get();
                                         outcomes.swap(i, i+1);
                                         write.set(outcomes);
                                    }>"⯆"</div>
                                 </div>
                                 <Outcome
                                     on_remove=move |ev: ev::MouseEvent| {
                                         let msg = "Are you sure you want to remove this outcome?";
                                         if ev.ctrl_key() || window().confirm_with_message(msg).unwrap() {
                                             let mut outcomes = read.get();
                                             outcomes.remove(i);
                                             write.set(outcomes);
                                         }
                                     }
                                     outcome=subsignal!(outcomes[i]) />
                            </div>
                         }
                     }).collect::<Vec<_>>()
                 }}
            </div>
        </div>
    }
}

infinite_list!(Projects, Project, projects);

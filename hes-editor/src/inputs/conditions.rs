use crate::{enum_slice, inputs::*, subsignal};
use hes_engine::{
    events::{
        Condition,
        ConditionKind,
        LocalVariable,
        PlayerVariable,
        WorldVariable,
    },
    npcs::NPC,
    production::Process,
    projects::Project,
};
use leptos::*;

#[component]
fn Condition<F>(
    condition: (Signal<Condition>, SignalSetter<Condition>),
    on_remove: F,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    let (read, write) = condition;

    let processes =
        expect_context::<Signal<Collection<Ref<Process>>>>();
    let projects =
        expect_context::<Signal<Collection<Ref<Project>>>>();
    let npcs = expect_context::<Signal<Collection<Ref<NPC>>>>();

    let input = move || {
        match read.get() {
            Condition::WorldVariable(var, comp, value) => view! {
                <div class="input-help">"Compare against a global variable."</div>
                <EnumInput
                    label="Variable"
                    help="The reference variable."
                    signal=enum_slice!(|write| Condition::WorldVariable([var], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::WorldVariable(var, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::WorldVariable(var, comp, [value])) />
            }.into_view(),

            Condition::LocalVariable(var, comp, value) => view! {
                <div class="input-help">"Compare against a local (regional) variable."</div>
                <EnumInput
                    label="Variable"
                    help="The reference variable."
                    signal=enum_slice!(|write| Condition::LocalVariable([var], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::LocalVariable(var, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::LocalVariable(var, comp, [value])) />
            }.into_view(),

            Condition::PlayerVariable(var, comp, value) => view! {
                <div class="input-help">"Compare against a player variable."</div>
                <EnumInput
                    label="Variable"
                    help="The reference variable."
                    signal=enum_slice!(|write| Condition::PlayerVariable([var], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::PlayerVariable(var, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::PlayerVariable(var, comp, [value])) />
            }.into_view(),

            Condition::ProcessOutput(id, comp, value) => view! {
                <div class="input-help">"Compare against the output of a process."</div>
                <EntityPicker
                    label="Process"
                    opts=processes
                    help="Which process to compare against."
                    signal=enum_slice!(|write| Condition::ProcessOutput([id], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::ProcessOutput(id, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The output value to compare against."
                    signal=enum_slice!(|write| Condition::ProcessOutput(id, comp, [value])) />
            }.into_view(),

            Condition::ProcessMixShare(id, comp, value) => view! {
                <div class="input-help">"Compare against the mix share (percentage) of a process."</div>
                <EntityPicker
                    label="Process"
                    opts=processes
                    help="Which process to compare against."
                    signal=enum_slice!(|write| Condition::ProcessMixShare([id], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::ProcessMixShare(id, [comp], value)) />
                <PercentInput
                    inline=true
                    label="Mix Share"
                    help="The mix share to compare against."
                    signal=enum_slice!(|write| Condition::ProcessMixShare(id, comp, [value])) />
            }.into_view(),

            Condition::ProcessMixShareFeature(feat, comp, value) => view! {
                <div class="input-help">"Compare against the total mix share (percentage) of processes with a particular feature."</div>
                <EnumInput
                    label="Process"
                    help="Which process feature to compare against."
                    signal=enum_slice!(|write| Condition::ProcessMixShareFeature([feat], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::ProcessMixShareFeature(feat, [comp], value)) />
                <PercentInput
                    inline=true
                    label="Mix Share"
                    help="The mix share to compare against."
                    signal=enum_slice!(|write| Condition::ProcessMixShareFeature(feat, comp, [value])) />
            }.into_view(),

            Condition::ResourcePressure(resource, comp, value) => view! {
                <div class="input-help">"Compare against the pressure on a particular resource. Pressure is represented as a percentage, where 0% means there is no pressure on the resource (demand for it is 0) and 100% means the demand for the resource equals its total supply."</div>
                <EnumInput
                    label="Resource"
                    help="Which resource to compare against."
                    signal=enum_slice!(|write| Condition::ResourcePressure([resource], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::ResourcePressure(resource, [comp], value)) />
                <PercentInput
                    inline=true
                    label="Pressure"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::ResourcePressure(resource, comp, [value])) />
            }.into_view(),

            Condition::ResourceDemandGap(resource, comp, value) => view! {
                <div class="input-help">"Compare against the gap between the demand and the supply of a particular resource, in the resource's units."</div>
                <EnumInput
                    label="Resource"
                    help="Which resource to compare against."
                    signal=enum_slice!(|write| Condition::ResourceDemandGap([resource], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::ResourceDemandGap(resource, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Gap Size"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::ResourceDemandGap(resource, comp, [value])) />
            }.into_view(),

            Condition::OutputDemandGap(output, comp, value) => view! {
                <div class="input-help">"Compare against the gap between the demand and the supply of a particular output, in the output's units."</div>
                <EnumInput
                    label="Output"
                    help="Which output to compare against."
                    signal=enum_slice!(|write| Condition::OutputDemandGap([output], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::OutputDemandGap(output, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Gap Size"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::OutputDemandGap(output, comp, [value])) />
            }.into_view(),

            Condition::Demand(output, comp, value) => view! {
                <div class="input-help">"Compare against the demand for a particular output, in the output's units."</div>
                <EnumInput
                    label="Output"
                    help="Which output to compare against."
                    signal=enum_slice!(|write| Condition::Demand([output], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::Demand(output, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Demand Amount"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::Demand(output, comp, [value])) />
            }.into_view(),

            Condition::ProjectStatus(id, status) => view! {
                <div class="input-help">"Check if the status of a particular project matches the specified value."</div>
                <EntityPicker
                    label="Project"
                    opts=projects
                    help="Which project to compare against."
                    signal=enum_slice!(|write| Condition::ProjectStatus([id], status)) />
                <EnumInput
                    label="Status"
                    help="The expected status."
                    signal=enum_slice!(|write| Condition::ProjectStatus(id, [status])) />
            }.into_view(),

            Condition::ActiveProjectUpgrades(id, comp, count) => view! {
                <div class="input-help">"Compare against the number of active upgrades of a particular project."</div>
                <EntityPicker
                    label="Project"
                    opts=projects
                    help="Which project to compare against."
                    signal=enum_slice!(|write| Condition::ActiveProjectUpgrades([id], comp, count)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::ActiveProjectUpgrades(id, [comp], count)) />
                <NumericInput
                    inline=true
                    label="Number of Upgrades"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::ActiveProjectUpgrades(id, comp, [count])) />
            }.into_view(),

            Condition::RunsPlayed(comp, count) => view! {
                <div class="input-help">"Compare against the number of times the player has played the game. Note that the number of runs played is tracked locally within a browser, so it will not carry across browsers and if the browsing data (local storage) is cleared, the value will reset."</div>
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::RunsPlayed([comp], count)) />
                <NumericInput
                    inline=true
                    label="Number of Runs"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::RunsPlayed(comp, [count])) />
            }.into_view(),

            Condition::NPCRelationship(id, relation) => view! {
                <div class="input-help">"Check if the relationship status with a particular NPC matches the specified value."</div>
                <EntityPicker
                    label="NPC"
                    opts=npcs
                    help="Which NPC to compare against."
                    signal=enum_slice!(|write| Condition::NPCRelationship([id], relation)) />
                <EnumInput
                    label="Relationship"
                    help="The relationship to compare against."
                    signal=enum_slice!(|write| Condition::NPCRelationship(id, [relation])) />
            }.into_view(),

            Condition::FeedstockYears(feedstock, comp, value) => view! {
                <div class="input-help">"Compare against the estimated number of years before a particular feedstock is depleted."</div>
                <EnumInput
                    label="Feedstock"
                    help="Which feedstock to compare against."
                    signal=enum_slice!(|write| Condition::FeedstockYears([feedstock], comp, value)) />
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::FeedstockYears(feedstock, [comp], value)) />
                <NumericInput
                    inline=true
                    label="Years"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::FeedstockYears(feedstock, comp, [value])) />
            }.into_view(),

            Condition::RegionFlag(flag) => view! {
                <div class="input-help">"Check if a matching region flag exists on a region."</div>
                <EnumInput
                    label="Flag"
                    help="Which flag to compare against."
                    signal=enum_slice!(|write| Condition::RegionFlag([flag])) />
            }.into_view(),

            Condition::HasFlag(flag) => view! {
                <div class="input-help">"Check if a matching flag exists."</div>
                <EnumInput
                    label="Flag"
                    help="Which flag to compare against."
                    signal=enum_slice!(|write| Condition::HasFlag([flag])) />
            }.into_view(),

            Condition::WithoutFlag(flag) => view! {
                <div class="input-help">"Check if a matching flag doesn't exist."</div>
                <EnumInput
                    label="Flag"
                    help="Which flag to compare against."
                    signal=enum_slice!(|write| Condition::WithoutFlag([flag])) />
            }.into_view(),

            Condition::HeavyProjects(comp, count) => view! {
                <div class="input-help">{r#"Compare against the number of active "Heavy" projects. This includes projects in the following groups: "Space", "Nuclear", "Geoengineering", "Electrification"."#}</div>
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::HeavyProjects([comp], count)) />
                <NumericInput
                    inline=true
                    label="Number of Projects"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::HeavyProjects(comp, [count])) />
            }.into_view(),

            Condition::ProtectLand(comp, value) => view! {
                <div class="input-help">"Compare against the percentage of land under protection."</div>
                <EnumInput
                    label="Comparator"
                    help="The comparison operation."
                    signal=enum_slice!(|write| Condition::ProtectLand([comp], value)) />
                <PercentInput
                    inline=true
                    label="Land Under Protection"
                    help="The value to compare against."
                    signal=enum_slice!(|write| Condition::ProtectLand(comp, [value])) />
            }.into_view(),
        }
    };
    let label = move || {
        let kind: ConditionKind = with!(|read| read.into());
        kind.to_string()
    };

    view! {
        <div class="condition mutable-list-item">
            <div class="mutable-list-item-header">
                <label>{label}</label>
                <div class="mutable-list-item-remove" title="Ctrl-click to remove without confirmation." on:click=on_remove>"✗"</div>
            </div>
            {input}
        </div>
    }
}

#[component]
pub fn Conditions(
    conditions: (
        Signal<Vec<Condition>>,
        SignalSetter<Vec<Condition>>,
    ),
) -> impl IntoView {
    let (read, write) = conditions;

    let (new_kind, set_new_kind) =
        create_signal(ConditionKind::WorldVariable);

    let processes =
        expect_context::<Signal<Collection<Ref<Process>>>>();
    let projects =
        expect_context::<Signal<Collection<Ref<Project>>>>();
    let npcs = expect_context::<Signal<Collection<Ref<NPC>>>>();

    let default_process =
        move || with!(|processes| processes.first().id);
    let default_project =
        move || with!(|projects| projects.first().id);
    let default_npc = move || with!(|npcs| npcs.first().id);

    view! {
        <div class="conditions mutable-list">
            <div class="mutable-list-header">
                <h2>Conditions</h2>
                <div class="mutable-list-add">
                    <EnumInput
                        label="Condition Kind"
                        help="What kind of condition to create."
                        signal=(new_kind.into(), set_new_kind.into()) />
                    <div class="mutable-list-add-button" on:click=move |_| {
                        let condition = Condition::from_kind(
                            new_kind.get(),
                            default_process(),
                            default_project(),
                            default_npc(),
                            );
                        let mut conditions = read.get();
                        conditions.insert(0, condition);
                        write.set(conditions);
                    }>+Add</div>
                </div>
            </div>
            {move || {
                 let empty = with!(|read| read.is_empty());
                 if empty {
                     Some(view! {
                         <div class="empty">No conditions defined.</div>
                     })
                 } else {
                     None
                 }
            }}
            <div class="mutable-list-items">
                {move || {
                     let n_conditions = with!(|read| read.len());
                     (0..n_conditions).map(|i| {
                         view! {
                             <Condition
                                 on_remove=move |ev: ev::MouseEvent| {
                                     let msg = "Are you sure you want to remove this condition?";
                                     if ev.ctrl_key() || window().confirm_with_message(msg).unwrap() {
                                         let mut conditions = read.get();
                                         conditions.remove(i);
                                         write.set(conditions);
                                     }
                                 }
                                 condition=subsignal!(conditions[i]) />
                         }
                     }).collect::<Vec<_>>()
                 }}
            </div>
        </div>
    }
}

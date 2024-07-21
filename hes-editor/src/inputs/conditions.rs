use crate::{enum_slice, inputs::*};
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
fn Condition(
    condition: (Signal<Condition>, SignalSetter<Condition>),
) -> impl IntoView {
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
        <div class="effect">
            <label>{label}</label>
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
    let conditions = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(conditions.get());
    });

    let n_conditions = with!(|conditions| conditions.len());
    view! {
        <div class="conditions">
        {move || {
             (0..n_conditions).map(|i| {
                 view! {
                     <Condition
                         condition=create_slice(conditions,
                             move |conditions| conditions[i].clone(),
                             move |conditions, val| conditions[i] = val
                         ) />
                 }
             }).collect::<Vec<_>>()
         }}
        </div>
    }
}

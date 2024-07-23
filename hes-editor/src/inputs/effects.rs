use crate::{enum_slice, inputs::*, subsignal};
use hes_engine::{
    events::{Effect, EffectKind, Event, WorldVariable},
    industries::Industry,
    npcs::NPC,
    production::Process,
    projects::Project,
};
use leptos::*;

#[component]
fn Effect<F>(
    effect: (Signal<Effect>, SignalSetter<Effect>),
    on_remove: F,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    let (read, write) = effect;

    let processes =
        expect_context::<Signal<Collection<Ref<Process>>>>();
    let projects =
        expect_context::<Signal<Collection<Ref<Project>>>>();
    let events =
        expect_context::<Signal<Collection<Ref<Event>>>>();
    let industries =
        expect_context::<Signal<Collection<Ref<Industry>>>>();
    let npcs = expect_context::<Signal<Collection<Ref<NPC>>>>();

    let input = move || {
        match read.get() {
            Effect::WorldVariable(var, value) => view! {
                <EnumInput
                    label="Variable"
                    help="What variable is changed."
                    signal=enum_slice!(|write| Effect::WorldVariable([var], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The amount to change the variable by."
                    signal=enum_slice!(|write| Effect::WorldVariable(var, [value])) />
            }.into_view(),

            Effect::PlayerVariable(var, value) => view! {
                <EnumInput
                    label="Variable"
                    help="What variable is changed."
                    signal=enum_slice!(|write| Effect::PlayerVariable([var], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The amount to change the variable by."
                    signal=enum_slice!(|write| Effect::PlayerVariable(var, [value])) />
            }.into_view(),

            Effect::RegionHabitability(lat, value) => view! {
                <div class="input-help">"Modify the habitability of all regions at the given latitude."</div>
                <EnumInput
                    label="Latitude"
                    help="What latitude is affected."
                    signal=enum_slice!(|write| Effect::RegionHabitability([lat], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The amount to change the habitability by."
                    signal=enum_slice!(|write| Effect::RegionHabitability(lat, [value])) />
            }.into_view(),

            Effect::Resource(resource, value) => view! {
                <div class="input-help">"Modify the availability of the specified resource by an absolute amount."</div>
                <EnumInput
                    label="Resource"
                    help="What resource is affected."
                    signal=enum_slice!(|write| Effect::Resource([resource], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The amount to change the resource reserves by."
                    signal=enum_slice!(|write| Effect::Resource(resource, [value])) />
            }.into_view(),

            Effect::Demand(output, value) => view! {
                <div class="input-help">"Modify all demand for the specified output by a percentage."</div>
                <EnumInput
                    label="Output"
                    help="What output is affected."
                    signal=enum_slice!(|write| Effect::Demand([output], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify this output's demand by."
                    signal=enum_slice!(|write| Effect::Demand(output, [value])) />
            }.into_view(),

            Effect::DemandAmount(output, value) => view! {
                <div class="input-help">"Modify all demand for the specified output by an absolute amount."</div>
                <EnumInput
                    label="Output"
                    help="What output is affected."
                    signal=enum_slice!(|write| Effect::DemandAmount([output], value)) />
                <NumericInput
                    inline=true
                    label="Amount"
                    help="The amount to modify this output's demand by."
                    signal=enum_slice!(|write| Effect::DemandAmount(output, [value])) />
            }.into_view(),

            Effect::Output(output, value) => view! {
                <div class="input-help">"Modify all production for the specified output by a percentage."</div>
                <EnumInput
                    label="Output"
                    help="What output is affected."
                    signal=enum_slice!(|write| Effect::Output([output], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify this output's amount by."
                    signal=enum_slice!(|write| Effect::Output(output, [value])) />
            }.into_view(),

            Effect::OutputForFeature(feat, value) => view! {
                <div class="input-help">"Modify the production efficiency of processes with the specified feature by a percentage. For example, a value of 10% means 10% more output is produced for the same resources/byproduct as the baseline."</div>
                <EnumInput
                    label="Feature"
                    help="What process feature is affected."
                    signal=enum_slice!(|write| Effect::OutputForFeature([feat], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify the output by."
                    signal=enum_slice!(|write| Effect::OutputForFeature(feat, [value])) />
            }.into_view(),

            Effect::OutputForProcess(id, value) => view! {
                <div class="input-help">"Modify the production efficiency of a single process by a percentage. For example, a value of 10% means 10% more output is produced for the same resources/byproduct as the baseline."</div>
                <EntityPicker
                    label="Process"
                    opts=processes
                    help="Which process is affected."
                    signal=enum_slice!(|write| Effect::OutputForProcess([id], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify this process's output by."
                    signal=enum_slice!(|write| Effect::OutputForProcess(id, [value])) />
            }.into_view(),

            Effect::CO2ForFeature(feat, value) => view! {
                <div class="input-help">"Modify CO2 emitted for processes with the specified feature by a percentage."</div>
                <EnumInput
                    label="Feature"
                    help="What process feature is affected."
                    signal=enum_slice!(|write| Effect::CO2ForFeature([feat], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify this process's CO2 emissions by."
                    signal=enum_slice!(|write| Effect::CO2ForFeature(feat, [value])) />
            }.into_view(),

            Effect::BiodiversityPressureForFeature(feat, value) => view! {
                <div class="input-help">"Modify biodiversity pressure for processes with the specified feature by a percentage."</div>
                <EnumInput
                    label="Feature"
                    help="What process feature is affected."
                    signal=enum_slice!(|write| Effect::BiodiversityPressureForFeature([feat], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify this process's biodiversity pressure by."
                    signal=enum_slice!(|write| Effect::BiodiversityPressureForFeature(feat, [value])) />
            }.into_view(),

            Effect::ProcessLimit(id, value) => view! {
                <div class="input-help">"Modify the limit of the specified process by an absolute amount. If no process limit is defined for the process this will do nothing."</div>
                <EntityPicker
                    label="Process"
                    opts=processes
                    help="Which process is affected."
                    signal=enum_slice!(|write| Effect::ProcessLimit([id], value)) />
                <NumericInput
                    inline=true
                    label="Amount"
                    help="The amount to modify this process's limit by."
                    signal=enum_slice!(|write| Effect::ProcessLimit(id, [value])) />
            }.into_view(),

            Effect::Feedstock(feedstock, value) => view! {
                <div class="input-help">"Modify the specified feedstock's reserves by a percentage."</div>
                <EnumInput
                    label="Feedstock"
                    help="What feedstock is affected."
                    signal=enum_slice!(|write| Effect::Feedstock([feedstock], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify this feedstock's amount by."
                    signal=enum_slice!(|write| Effect::Feedstock(feedstock, [value])) />
            }.into_view(),

            Effect::AddEvent(id) => view! {
                <div class="input-help">"Add an event to the event pool (i.e. unlock it). Note: This effect is always hidden (not displayed to the user)."</div>
                <EntityPicker
                    label="Event"
                    opts=events
                    help="Which event is unlocked."
                    signal=enum_slice!(|write| Effect::AddEvent([id])) />
            }.into_view(),

            Effect::TriggerEvent(id, years) => view! {
                <div class="input-help">"Trigger an event after a specified number of years. Note: This effect is always hidden (not displayed to the user)."</div>
                <EntityPicker
                    label="Event"
                    opts=events
                    help="Which event will be triggered."
                    signal=enum_slice!(|write| Effect::TriggerEvent([id], years)) />
                <NumericInput
                    inline=true
                    label="Years"
                    help="Years after which the event will be triggered."
                    signal=enum_slice!(|write| Effect::TriggerEvent(id, [years])) />
            }.into_view(),

            Effect::LocksProject(id) => view! {
                <div class="input-help">"Locks a project (it will no longer be available)."</div>
                <EntityPicker
                    label="Project"
                    opts=projects
                    help="Which project is locked."
                    signal=enum_slice!(|write| Effect::LocksProject([id])) />
            }.into_view(),

            Effect::UnlocksProject(id) => view! {
                <div class="input-help">"Unlocks a project."</div>
                <EntityPicker
                    label="Project"
                    opts=projects
                    help="Which project is unlocked."
                    signal=enum_slice!(|write| Effect::UnlocksProject([id])) />
            }.into_view(),

            Effect::UnlocksProcess(id) => view! {
                <div class="input-help">"Unlocks a process."</div>
                <EntityPicker
                    label="Process"
                    opts=processes
                    help="Which process is unlocked."
                    signal=enum_slice!(|write| Effect::UnlocksProcess([id])) />
            }.into_view(),

            Effect::UnlocksNPC(id) => view! {
                <div class="input-help">"Unlocks an NPC."</div>
                <EntityPicker
                    label="NPC"
                    opts=npcs
                    help="Which NPC is unlocked."
                    signal=enum_slice!(|write| Effect::UnlocksNPC([id])) />
            }.into_view(),

            Effect::ProjectRequest(id, active, bounty) => view! {
                <div class="input-help">"Starts a request for a project."</div>
                <EntityPicker
                    label="Project"
                    opts=projects
                    help="Which project is requested."
                    signal=enum_slice!(|write| Effect::ProjectRequest([id], active, bounty)) />
                <ToggleInput
                    label="Active"
                    help="If the request is for this project to be implemented (active) or stopped (inactive)."
                    signal=enum_slice!(|write| Effect::ProjectRequest(id, [active], bounty)) />
                <NumericInput
                    inline=true
                    label="Reward"
                    help="How much political capital is awarded for fulfilling the request."
                    signal=enum_slice!(|write| Effect::ProjectRequest(id, active, [bounty])) />
            }.into_view(),

            Effect::ProcessRequest(id, active, bounty) => view! {
                <div class="input-help">"Starts a request for a process."</div>
                <EntityPicker
                    label="Process"
                    opts=processes
                    help="Which process is requested."
                    signal=enum_slice!(|write| Effect::ProcessRequest([id], active, bounty)) />
                <ToggleInput
                    label="Active"
                    help="If the request is for this process to be active (mix share > 0) or stopped (mix share == 0)."
                    signal=enum_slice!(|write| Effect::ProcessRequest(id, [active], bounty)) />
                <NumericInput
                    inline=true
                    label="Reward"
                    help="How much political capital is awarded for fulfilling the request."
                    signal=enum_slice!(|write| Effect::ProcessRequest(id, active, [bounty])) />
            }.into_view(),

            Effect::Migration => view! {
                <div class="input-help">"Triggers a wave of migration across regions."</div>
            }.into_view(),

            Effect::RegionLeave => view! {
                <div class="input-help">"Triggers a wave of migration across regions."</div>
            }.into_view(),

            Effect::AddRegionFlag(flag) => view! {
                <div class="input-help">"Add a flag to a region."</div>
                <EnumInput
                    label="Flag"
                    help="Which flag to add."
                    signal=enum_slice!(|write| Effect::AddRegionFlag([flag])) />
            }.into_view(),

            Effect::AddFlag(flag) => view! {
                <div class="input-help">"Set a flag."</div>
                <EnumInput
                    label="Flag"
                    help="Which flag to add."
                    signal=enum_slice!(|write| Effect::AddFlag([flag])) />
            }.into_view(),

            Effect::NPCRelationship(id, change) => view! {
                <div class="input-help">"Change the relationship with an NPC."</div>
                <EntityPicker
                    label="NPC"
                    opts=npcs
                    help="Which NPC's relationship is affected."
                    signal=enum_slice!(|write| Effect::NPCRelationship([id], change)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The amount to change the relationship by."
                    signal=enum_slice!(|write| Effect::NPCRelationship(id, [change])) />
            }.into_view(),

            Effect::ModifyProcessByproducts(id, byproduct, value) => view! {
                <div class="input-help">"Modify the amount of a single byproduct for a single process by a percentage."</div>
                <EntityPicker
                    label="Process"
                    opts=processes
                    help="Which process is affected."
                    signal=enum_slice!(|write| Effect::ModifyProcessByproducts([id], byproduct, value)) />
                <EnumInput
                    label="Byproduct"
                    help="What byproduct is affected."
                    signal=enum_slice!(|write| Effect::ModifyProcessByproducts(id, [byproduct], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify the byproduct by."
                    signal=enum_slice!(|write| Effect::ModifyProcessByproducts(id, byproduct, [value])) />
            }.into_view(),

            Effect::ModifyIndustryByproducts(id, byproduct, value) => view! {
                <div class="input-help">"Modify the amount of a single byproduct for a single industry by a percentage."</div>
                <EntityPicker
                    label="Industry"
                    opts=industries
                    help="Which industry is affected."
                    signal=enum_slice!(|write| Effect::ModifyIndustryByproducts([id], byproduct, value)) />
                <EnumInput
                    label="Byproduct"
                    help="What byproduct is affected."
                    signal=enum_slice!(|write| Effect::ModifyIndustryByproducts(id, [byproduct], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify the byproduct by."
                    signal=enum_slice!(|write| Effect::ModifyIndustryByproducts(id, byproduct, [value])) />
            }.into_view(),

            Effect::ModifyIndustryResources(id, resource, value) => view! {
                <div class="input-help">"Modify the amount of a single resource used by a single industry by a percentage."</div>
                <EntityPicker
                    label="Industry"
                    opts=industries
                    help="Which industry is affected."
                    signal=enum_slice!(|write| Effect::ModifyIndustryResources([id], resource, value)) />
                <EnumInput
                    label="Resource"
                    help="What resource is affected."
                    signal=enum_slice!(|write| Effect::ModifyIndustryResources(id, [resource], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify the resource by."
                    signal=enum_slice!(|write| Effect::ModifyIndustryResources(id, resource, [value])) />
            }.into_view(),

            Effect::ModifyIndustryResourcesAmount(id, resource, value) => view! {
                <div class="input-help">"Modify the amount of a single resource used by a single industry by an absolute amount."</div>
                <EntityPicker
                    label="Industry"
                    opts=industries
                    help="Which industry is affected."
                    signal=enum_slice!(|write| Effect::ModifyIndustryResourcesAmount([id], resource, value)) />
                <EnumInput
                    label="Resource"
                    help="What resource is affected."
                    signal=enum_slice!(|write| Effect::ModifyIndustryResourcesAmount(id, [resource], value)) />
                <NumericInput
                    inline=true
                    label="Value"
                    help="The amount to change the resource use by."
                    signal=enum_slice!(|write| Effect::ModifyIndustryResources(id, resource, [value])) />
            }.into_view(),

            Effect::ModifyIndustryDemand(id, value) => view! {
                <div class="input-help">"Modify the demand for a single industry by a percentage."</div>
                <EntityPicker
                    label="Industry"
                    opts=industries
                    help="Which industry is affected."
                    signal=enum_slice!(|write| Effect::ModifyIndustryDemand([id], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify the demand by."
                    signal=enum_slice!(|write| Effect::ModifyIndustryDemand(id, [value])) />
            }.into_view(),

            Effect::ModifyEventProbability(id, value) => view! {
                <div class="input-help">"Modify the probability of an event occurring."</div>
                <EntityPicker
                    label="Event"
                    opts=events
                    help="Which event will be affected."
                    signal=enum_slice!(|write| Effect::ModifyEventProbability([id], value)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to add to the event's probability."
                    signal=enum_slice!(|write| Effect::ModifyEventProbability(id, [value])) />
            }.into_view(),

            Effect::DemandOutlookChange(output, mult) => view! {
                <div class="input-help">"Apply a change in contentedness to every region based on its level of demand for the specified output, multiplied by the specified factor. Demand level ranges from [1, 5], where 1 is the lowest demand level and 5 is the highest. For example, with `Output::Fuel` and a factor of 0.5 and a region with demand level 2, that means `2 * 0.5 = 1` will be added to that region's contentedness. Note that this value is rounded, so if it were `3 * 0.5 = 1.5` this would be rounded to `2.0`."</div>
                <EnumInput
                    label="Output"
                    help="What output is affected."
                    signal=enum_slice!(|write| Effect::DemandOutlookChange([output], mult)) />
                <NumericInput
                    inline=true
                    label="Factor"
                    help="Factor to scale the demand level by."
                    signal=enum_slice!(|write| Effect::DemandOutlookChange(output, [mult])) />
            }.into_view(),

            Effect::IncomeOutlookChange(mult) => view! {
                <div class="input-help">"Apply a change in contentedness to every region based on its income level, multiplied by the specified factor. Income level ranges from [0, 3], where 0 is the lowest income level and 3 is the highest. For example, with a factor of 0.5 and a region with income level 2, that means `2 * 0.5 = 1` will be added to that region's contentedness. Note that this value is rounded, so if it were `3 * 0.5 = 1.5` this would be rounded to `2.0`."</div>
                <NumericInput
                    inline=true
                    label="Factor"
                    help="Factor to scale the demand level by."
                    signal=enum_slice!(|write| Effect::IncomeOutlookChange([mult])) />
            }.into_view(),

            Effect::ProjectCostModifier(id, change) => view! {
                <div class="input-help">"Modifies the cost a project by a percentage."</div>
                <EntityPicker
                    label="Project"
                    opts=projects
                    help="Which project is affected."
                    signal=enum_slice!(|write| Effect::ProjectCostModifier([id], change)) />
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to modify the project's cost by."
                    signal=enum_slice!(|write| Effect::ProjectCostModifier(id, [change])) />
            }.into_view(),

            Effect::ProtectLand(amount) => view! {
                <div class="input-help">"Change the amount of land under protection by a percentage."</div>
                <PercentInput
                    inline=true
                    label="Percent Change"
                    help="The percent to of land to add to/remove from protection."
                    signal=enum_slice!(|write| Effect::ProtectLand([amount])) />
            }.into_view(),

            Effect::BailOut(amount) => view! {
                <div class="input-help">"Bail the player out by providing some political capital."</div>
                <NumericInput
                    inline=true
                    label="Amount"
                    help="How much political capital to provide."
                    signal=enum_slice!(|write| Effect::BailOut([amount])) />
            }.into_view(),

            Effect::TerminationShock => view! {
                <div class="input-help">{r#"This effect only triggers when it is *unapplied*, in which case it undoes the temperature effect of the "Solar Radiation Management" project."#}</div>
            }.into_view(),

            Effect::GameOver => view! {
                <div class="input-help">"Trigger an immediate game over."</div>
            }.into_view(),
        }
    };
    let label = move || {
        let kind: EffectKind = with!(|read| read.into());
        kind.to_string()
    };

    view! {
        <div class="effect mutable-list-item">
            <div class="mutable-list-item-header">
                <label>{label}</label>
                <div class="mutable-list-item-remove" title="Ctrl-click to remove without confirmation." on:click=on_remove>"✗"</div>
            </div>
            {input}
        </div>
    }
}

#[component]
pub fn Effects(
    effects: (Signal<Vec<Effect>>, SignalSetter<Vec<Effect>>),
    #[prop(optional)] double_col: bool,
) -> impl IntoView {
    let (read, write) = effects;
    let (new_kind, set_new_kind) =
        create_signal(EffectKind::WorldVariable);

    let processes =
        expect_context::<Signal<Collection<Ref<Process>>>>();
    let projects =
        expect_context::<Signal<Collection<Ref<Project>>>>();
    let events =
        expect_context::<Signal<Collection<Ref<Event>>>>();
    let industries =
        expect_context::<Signal<Collection<Ref<Industry>>>>();
    let npcs = expect_context::<Signal<Collection<Ref<NPC>>>>();

    let default_process =
        move || with!(|processes| processes.first().id);
    let default_project =
        move || with!(|projects| projects.first().id);
    let default_industry =
        move || with!(|industries| industries.first().id);
    let default_event =
        move || with!(|events| events.first().id);
    let default_npc = move || with!(|npcs| npcs.first().id);

    view! {
        <div class="effects mutable-list" class:mutable-list-double-col={double_col}>
            <div class="mutable-list-header">
                <h2>Effects</h2>
                <div class="mutable-list-add">
                    <EnumInput
                        label="Effect Kind"
                        help="What kind of effect to create."
                        signal=(new_kind.into(), set_new_kind.into()) />
                    <div class="mutable-list-add-button" on:click=move |_| {
                        let effect = Effect::from_kind(
                            new_kind.get(),
                            default_process(),
                            default_project(),
                            default_industry(),
                            default_event(),
                            default_npc(),
                            );
                        let mut effects = read.get();
                        effects.insert(0, effect);
                        write.set(effects);
                    }>+Add</div>
                </div>
            </div>
            {move || {
                 let no_effects = with!(|read| read.is_empty());
                 if no_effects {
                     Some(view! {
                         <div class="empty">No effects defined.</div>
                     })
                 } else {
                     None
                 }
            }}
            <div class="mutable-list-items">
                {move || {
                     let n_effects = with!(|read| read.len());
                     (0..n_effects).map(|i| {
                         view! {
                             <Effect
                                 on_remove=move |ev: ev::MouseEvent| {
                                     let msg = "Are you sure you want to remove this effect?";
                                     if ev.ctrl_key() || window().confirm_with_message(msg).unwrap() {
                                         let mut effects = read.get();
                                         effects.remove(i);
                                         write.set(effects);
                                     }
                                 }
                                 effect=subsignal!(effects[i]) />
                         }
                     }).collect::<Vec<_>>()
                 }}
            </div>
        </div>
    }
}

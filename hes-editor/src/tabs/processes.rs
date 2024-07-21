use crate::inputs::*;
use hes_engine::{
    kinds::Feedstock,
    npcs::NPC,
    production::{Process, ProcessFeature},
    world::World,
    Collection,
};
use leptos::*;

impl Describe for ProcessFeature {
    fn describe(&self) -> &'static str {
        match self {
            Self::UsesPesticides => "For agriculture; does the process use a significant amount of pesticides",
            Self::UsesSynFertilizer => "For agriculture; does the process use a significant amount of synthetic fertilizer",
            Self::UsesLivestock => "For agriculture; does the process use a significant amount of livestock",
            Self::IsIntermittent => "For electricity sources; if the supply is intermittent",
            Self::MakesNuclearWaste => "For electricity sources, if the supply produces nuclear waste",
            Self::CanMeltdown => "For electricity sources, if the supply has a meltdown risk",
            Self::IsSolar => "If the process depends on sunlight",
            Self::IsCCS => "Whether this process produces CO2 that is then stored/transported/used",
            Self::IsCombustion => "If this process depends on combustion",
            Self::IsFossil => "If this process uses fossil fuels",
            Self::UsesOil => "If this process uses oil",
            Self::IsLaborIntensive => "If this process is especially labor intensive",
        }
    }
}

#[component]
fn Process(
    process: (Signal<Process>, SignalSetter<Process>),
) -> impl IntoView {
    let (read, write) = process;
    let process = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(process.get());
    });

    let npcs = expect_context::<Signal<Collection<NPC>>>();

    view! {
        <div class="process">
            <div class="name">
                <TextInput signal=slice!(process.name) />
            </div>
            <ImageInput signal=slice!(process.flavor.image) />
            <EnumInput
                label="Output Type"
                help="What this process produces."
                signal=slice!(process.output) />
            <EnumInput
                label="Feedstock Type"
                help=r#"What this feedstock this process requires. If no particular feedstock, just set to "Other"."#
                signal=create_slice(process,
                    move |process| process.feedstock.0,
                    move |process, val| process.feedstock.0 = val)
                />
            <Show when=move || with!(|process| process.feedstock.0 != Feedstock::Other)>
                <NumericInput
                    label="Feedstock Amount"
                    help="Feedstock required per unit output."
                    signal=create_slice(process,
                        move |process| process.feedstock.1,
                        move |process, val| process.feedstock.1 = val)
                />
            </Show>
            <ByproductMapInput
                label="Byproducts"
                help="Byproducts produced, per unit output."
                signal=slice!(process.byproducts) />
            <ResourceMapInput
                label="Resources"
                help="Resources used, per unit output."
                signal=slice!(process.resources) />
            <ToggleInput
                label="Locked"
                help="If this process is locked at the start."
                signal=slice!(process.locked) />
            <NumericInput
                label="Mix Share"
                help="What percent of total output production this process represents at the start. Note that 1 mix share = 5% of total output."
                signal=slice!(process.mix_share) />
            <MultiEnumInput
                label="Features"
                help="Special properties associated with this process."
                signal=slice!(process.features)
                />
            <OptionalNumericInput
                label="Output Limit"
                help="(Optional) This process can never produce more than this much output, effectively setting a limit on its mix share. This may be because, for example, of a finite availability, e.g. with geothermal."
                signal=slice!(process.limit)
                />
            <MultiEntitySelect
                label="Supporters"
                help="NPCs that support this process."
                signal=slice!(process.supporters)
                opts=npcs
                />
            <MultiEntitySelect
                label="Opposers"
                help="NPCs that oppose this process."
                signal=slice!(process.opposers)
                opts=npcs
                />
        </div>
    }
}

#[component]
pub fn Processes(world: RwSignal<World>) -> impl IntoView {
    let n_processes = with!(|world| world.processes.len());
    view! {
        <div class="processes">
        {move || {
             (0..n_processes).map(|i| {
                 view! {
                     <Process
                         process=create_slice(world,
                             move |world| world.processes.by_idx(i).clone(),
                             move |world, val| *world.processes.by_idx_mut(i) = val
                         ) />
                 }
             }).collect::<Vec<_>>()
         }}
        </div>
    }
}

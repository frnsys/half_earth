use crate::{infinite_list, inputs::*};
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
    signal: (Signal<Process>, SignalSetter<Process>),
) -> impl IntoView {
    let (read, write) = signal;
    let process = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(process.get());
    });

    let npcs = expect_context::<Signal<Collection<Ref<NPC>>>>();

    let feedstock_units = move || {
        with!(|process| match process.feedstock.0 {
            Feedstock::Oil | Feedstock::NaturalGas =>
                "liters (L)",
            Feedstock::Thorium
            | Feedstock::Uranium
            | Feedstock::Lithium
            | Feedstock::Coal => "grams (g)",
            Feedstock::Soil | Feedstock::Other => "(n/a)",
        })
    };
    let feedstock_name = move || {
        with!(|process| process.feedstock.0.to_string())
    };

    view! {
        <div class="process">
            <div class="name">
                <TextInput signal=slice!(process.name) />
                <div class="item-lock">
                    <ToggleInput
                        label="Locked"
                        tooltip=true
                        icons=("🔒Locked", "🔓Unlocked")
                        help="If this process is locked at the start."
                        signal=slice!(process.locked) />
                </div>
            </div>
            <div class="item-form">
                <div class="input-groups left-main-col">
                    <ImageInput signal=slice!(process.flavor.image) />
                    <NumericInput
                        inline=true
                        label="Mix Share"
                        help="What percent of total output production this process represents at the start. Note that 1 mix share = 5% of total output."
                        signal=slice!(process.mix_share) />
                    <OptionalNumericInput
                        label="Output Limit"
                        help="(Optional) This process can never produce more than this much output, effectively setting a limit on its mix share. This may be because, for example, of a finite availability, e.g. with geothermal."
                        signal=slice!(process.limit)
                        />
                </div>
                <div class="input-groups">
                    <EnumInput
                        label="Output Type"
                        help="What this process produces."
                        signal=slice!(process.output) />
                    <EnumInput
                        label="Feedstock Type"
                        help=r#"What this feedstock this process requires. If no particular feedstock, just set to "Other". Note that "Soil" is ignored."#
                        signal=create_slice(process,
                            move |process| process.feedstock.0,
                            move |process, val| process.feedstock.0 = val)
                        />
                    <Show when=move || with!(|process| process.feedstock.0 != Feedstock::Other)>
                        <div class="feedstock-amount">
                            <NumericInput
                                inline=true
                                label="Feedstock"
                                help=format!("Feedstock required per unit output, in {} of {}.", feedstock_units(), feedstock_name())
                                signal=create_slice(process,
                                    move |process| process.feedstock.1,
                                    move |process, val| process.feedstock.1 = val)
                            />
                        </div>
                    </Show>
                    <ByproductMapInput
                        label="Byproducts"
                        help="Byproducts produced, per unit output."
                        signal=slice!(process.byproducts) />
                    <ResourceMapInput
                        label="Resources"
                        help="Resources used, per unit output."
                        signal=slice!(process.resources) />
                </div>
            </div>
            <div class="item-form">
                <MultiEnumInput
                    label="Features"
                    help="Special properties associated with this process."
                    signal=slice!(process.features)
                    />
            </div>
            <div class="item-form">
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
        </div>
    }
}

infinite_list!(Processes, Process, processes);

use crate::game::State;
use crate::kinds::Output;
use crate::events::Event;

#[derive(Debug, Clone)]
pub enum WorldVariable {
    Emissions,
    Biodiversity,
    Temperature,
    Precipitation,
    SeaLevelRise,
    OzoneDamage
}

#[derive(Debug, Clone)]
pub enum LocalVariable {
    Population,
    Health,
    Safety,
    Outlook,
    Satiety
}

#[derive(Debug, Clone)]
pub enum Effect {
    /// Influence demand for output
    Demand(Output, f32),

    /// Influence output produced
    Output(Output, f32),

    /// Influence world/local variables
    World(WorldVariable, f32),
    Local(LocalVariable, f32),

    /// Trigger an event after n steps (guaranteed)
    TriggerEvent(Event, u8),

    /// Adds an event to the event pool;
    /// i.e. it's possible but not guaranteed to occur
    AddEvent(Event),
}

impl Effect {
    pub fn apply(&self, state: &mut State) {
        // match self {
        // }
        // TODO apply the effect
    }
}


// use crate::game::State;

// pub type Effect = Box<dyn EffectFn>;

// pub trait EffectFn: Fn(&mut State) {}
// impl<F> EffectFn for F where F: Fn(&mut State) {}
// impl std::fmt::Debug for dyn EffectFn {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Effect")
//     }
// }

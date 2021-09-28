use crate::kinds::{ResourceMap, ByproductMap, Output};

// TODO better way of doing this instead of lumping all together?
#[derive(Default, Debug)]
pub struct ProcessDetails {
    pub soil_impact: bool, // degrades or builds soil
    pub pesticides: Amount,
    pub fertilizer: Amount,
    pub livestock: bool,
    pub intermittent: bool,
}


pub struct Process {
    pub output: Output,
    pub details: ProcessDetails,
    pub reqs: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,


    // If the player has unlocked and/or banned
    // this process.
    pub unlocked: bool,
    pub banned: bool,
}


#[derive(Debug)]
pub enum Amount {
    None,
    VeryLow,
    Low,
    Mid,
    High,
    VeryHigh
}

impl Default for Amount {
    fn default() -> Self {
        Amount::None
    }
}


macro_rules! details {
    () => {
        ProcessDetails::default()
    };
    ($($field:ident: $value:expr),*) => {
        {
            let mut map = ProcessDetails::default();
            $(
                map.$field = $value;
            )*
            map
        }
    };
}

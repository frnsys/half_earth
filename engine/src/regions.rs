pub struct Region<'a> {
    pub name: &'a str,
    pub population: f32,

    pub income: Income,

    /// Public health
    pub health: f32,

    /// How hopeful are people in the region about the future?
    pub outlook: f32,

    /// Base habitability encapsulates
    /// other factors that influence habitability.
    /// E.g. negative events such as hurricanes should subtract
    /// from this value
    pub base_habitability: f32,
}

impl Region<'_> {
    // Simple mean
    pub fn contentedness(&self) -> f32 {
        (self.health + self.outlook)/2.
    }

    pub fn habitability(&self) -> f32 {
        // TODO Factors:
        // - regional temp, precip, sea_level_rise, health, safety,
        // - number of negative events
        todo!()
    }
}

pub enum Income {
    Low,
    LowerMiddle,
    UpperMiddle,
    High
}


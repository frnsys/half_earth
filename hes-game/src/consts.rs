/// PC cost for research/infrastructure points
pub const POINT_COST: u8 = 3;

/// Have to all be below these values to win
pub const WIN_EMISSIONS: f32 = 0.0;
pub const WIN_EXTINCTION: f32 = 20.0;
pub const WIN_TEMPERATURE: f32 = 1.0;

/// Set an upper cap (in GtCO2eq) to the amount of emissions we pass to hector,
/// because very large numbers end up breaking it.
pub const MAX_EMISSIONS: f32 = 200.;

/// How much PC is earned when completing a project
pub const PC_PER_COMPLETED_PROJECT: usize = 5;

pub const PROCESS_POINTS_PER_CYCLE: usize = 5;

pub const MAX_RELATIONSHIP: u8 = 6;

/// Bonus PC for the first n years
pub const HONEYMOON_PC: usize = 15;
pub const HONEYMOON_YEARS: usize = 5;

/// PC change per -0.1C temp change
pub const TEMPERATURE_PC: usize = 2;

/// PC change per -0.5Gt emissions change
pub const EMISSIONS_PC: usize = 5;

/// Max points for a project
pub const MAX_POINTS: usize = 12;

pub const MAX_BIODIVERSITY: f32 = 120.;
pub const MAX_CONTENTEDNESS: f32 = 40.;

/// PC earned per intensity level of contentedness.
pub const CONTENTEDNESS_PC: [isize; 6] = [0, 0, 5, 10, 20, 30];

/// PC earned per intensity level of extinction.
pub const EXTINCTION_PC: [isize; 6] = [20, 10, 0, -5, -5, -10];

/// In seconds
pub const PROJECT_CARD_SCAN_TIME: f32 = 0.7;
pub const PROJECT_CARD_WITHDRAW_TIME: f32 = 1.4;
pub const PROCESS_CARD_SCAN_TIME: f32 = 0.5;
pub const PROCESS_CARD_WITHDRAW_TIME: f32 = 0.4;

pub const MS_PER_YEAR: usize = 4000;

pub const PARLIAMENT_SEATS: &[usize] = &[2, 3, 4, 5, 5, 5, 4, 3, 2];

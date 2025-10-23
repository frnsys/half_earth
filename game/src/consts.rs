/// PC cost for research/infrastructure points
pub const POINT_COST: u8 = 3;

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
pub const TEMPERATURE_PC: isize = 2;

/// PC change per -0.5Gt emissions change
pub const EMISSIONS_PC: isize = 5;

/// PC change per -1 extinction rate change
pub const BIODIVERSITY_PC: isize = 2;

/// Max points for a project
pub const MAX_POINTS: usize = 12;

pub const MAX_BIODIVERSITY: f32 = 120.;
pub const MAX_CONTENTEDNESS: f32 = 40.;

/// Factor to compute contentedness change resulting from an
/// icon/world event, by its intensity.
pub const EVENT_INTENSITY_TO_CONTENTEDNESS: f32 = 0.1;

/// PC earned per intensity level of contentedness.
pub const CONTENTEDNESS_PC: [isize; 6] = [0, 0, 5, 10, 20, 30];

/// PC earned per intensity level of extinction.
pub const EXTINCTION_PC: [isize; 6] = [20, 10, 0, -5, -5, -10];

/// In milliseconds
pub const PROJECT_CARD_SCAN_TIME: f32 = 0.5 * 1000.;
pub const PROJECT_CARD_WITHDRAW_TIME: f32 = 1.2 * 1000.;
pub const PROCESS_CARD_SCAN_TIME: f32 = 0.4 * 1000.;
pub const PROCESS_CARD_WITHDRAW_TIME: f32 = 0.4 * 1000.;

/// How many ms for each year to pass in the world events view.
pub const MS_PER_YEAR: f32 = 2500.;

pub const PARLIAMENT_SEATS: &[usize] = &[9, 9, 7, 5, 3];

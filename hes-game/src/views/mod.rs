mod cards;
mod effects;
mod events;
mod factors;
mod globe;
mod help;
mod hud;
mod intensity;
mod menu;
mod phases;
mod scanner;
mod splash;
mod tips;

pub use cards::FactorsCard;
pub use effects::Effects;
pub use events::*;
pub use factors::{rank as rank_factors, Factor};
pub use globe::CalcSurface;
pub use help::*;
pub use phases::*;
pub use splash::*;
pub use tips::*;

/// Convenience macro for creating memoized signals.
#[macro_export]
macro_rules! memo {
    ($base:ident.$($path:ident).+) => {
        create_memo(move |_| {
            tracing::debug!("Memo called for {}.{}", stringify!($base), stringify!($($path).+));
            $base.with(move |value| value.$($path).+.clone())
        })
    };
    ($base:ident.$($path:ident).+ ($($arg:tt)*)) => {
        create_memo(move |_| {
            tracing::debug!("Memo called for {}.{}", stringify!($base), stringify!($($path).+($($arg)*)));
            $base.with(move |value| value.$($path).+($($arg)*))
        })
    };
}

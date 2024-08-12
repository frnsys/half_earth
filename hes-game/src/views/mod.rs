mod cards;
mod debug;
mod effects;
mod events;
mod factors;
mod game;
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
pub use debug::*;
pub use effects::Effects;
pub use events::*;
pub use factors::{rank as rank_factors, Factor};
pub use game::Game;
pub use help::*;
pub use phases::*;
pub use splash::*;
pub use tips::*;

/// Convenience macro for creating memoized signals.
#[macro_export]
macro_rules! memo {
    ($base:ident.$($path:ident).+) => {
        create_memo(move |_| {
            tracing::trace!("Memo called for {}.{}", stringify!($base), stringify!($($path).+));
            $base.with(move |value| value.$($path).+.clone())
        })
    };
    ($base:ident.$($($path:ident).+ ($($arg:tt)*)).+) => {
        create_memo(move |_| {
            tracing::trace!("Memo called for {}.{}", stringify!($base), stringify!($($($path).+($($arg)*)).+));
            $base.with(move |value| value.$($($path).+($($arg)*)).+)
        })
    };
}

/// This is super hacky but I'm struggling to figure out
/// how to approach these problems in leptos.
/// As I understand it when a signal is updated in leptos
/// it immediately starts triggering any dependents,
/// which can lead to nested calls where an outer function
/// borrows a signal that an inner, deeper function tries
/// to borrow and can't, causing a borrow error.
///
/// That in itself isn't necessarily a difficult problem to resolve,
/// but when that error is thrown in leptos you're pointed to
/// a line within the leptos library, not where the failed borrow
/// was attempted, not what signal the borrow failed on,
/// nor anything else that gives an idea of where to investigate.
///
/// So you have to do a ton of trial-and-error, commenting out things
/// to try and narrow down where the error is actually happening.
/// And once you find that, then you have to figure out where in the call
/// stack the conflicting borrow is happening--again requiring an very
/// trial-and-error approach.
///
/// A quick-and-dirty "solution" is what I'm doing here. You don't listen
/// on directly to "real" signal but instead via a proxy signal. This proxy
/// signal is updated as a side-effect when the real signal is updated,
/// but crucially it's updated *after* the current call stack is resolved
/// and any borrows are freed. Here I'm using `queue_microtask` but elsewhere
/// I'm using `use_timeout` which accomplishes the same thing, though
/// probably not as nicely.
#[macro_export]
macro_rules! proxy {
    ($base:ident.$($path:ident).+) => {{
        let init = $base.with_untracked(move |value| value.$($path).+.clone());
        let proxy = create_rw_signal(init);
        let source = memo!($base.$($path).+);
        create_effect(move |_| {
            source.track();
            queue_microtask(move || {
                let changes =
                    $base.with_untracked(|value| value.$($path).+.clone());
                proxy.set(changes);
            });
        });
        proxy
    }}
}

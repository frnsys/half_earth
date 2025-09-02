use crate::math::shared::use_simple_math;
use leptos::*;
use num::Float;
use paste::paste;

use_simple_math!(
    /// Reactive `abs()`.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_abs)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::math::use_abs;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (value, set_value) = create_signal(-32.25);
    /// let result: Signal<f64> = use_abs(value); // 32.25
    /// #
    /// # assert_eq!(result.get(), 32.25);
    /// # view! { }
    /// # }
    /// ```
    // #[doc(cfg(feature = "math"))]
    abs
);

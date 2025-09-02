use leptos::*;

crate::use_derive_signal!(
    /// Reactive `Result::is_ok()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::is_ok;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (example, set_example) = create_signal(
    ///     if js_sys::Math::random() < 0.5 { Ok("Example") } else { Err(()) }
    /// );
    ///
    /// let is_ok = is_ok(example);
    /// #
    /// # view! { }
    /// # }
    /// ```
    is_ok<Result<T, ()>, T: 'static> -> bool
    |value| value.is_ok()
);

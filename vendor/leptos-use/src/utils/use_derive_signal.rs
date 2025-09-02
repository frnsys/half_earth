/// Macro to easily create helper functions that derive a signal using a piece of code.
///
/// See [`fn@crate::is_ok`] or [`fn@crate::use_to_string`] as examples.
#[macro_export]
macro_rules! use_derive_signal {
    (
        $(#[$outer:meta])*
        $name:ident <$inner_signal_type:tt $(< $( $inner_type_param:tt ),+ >)? $(, $( $type_param:tt $( : $first_bound:tt $(+ $rest_bound:tt)* )? ),+ )? > -> $return_type:tt
        $($body:tt)+
    ) => {
        $(#[$outer])*
        pub fn $name<V $(, $( $type_param ),* )? >(value: V) -> Signal<$return_type>
        where
            V: Into<MaybeSignal<$inner_signal_type $(< $( $inner_type_param ),+ >)?>> $(, $( $type_param $( : $first_bound $(+ $rest_bound)* )? ),+ )?
        {
            let value = value.into();
            Signal::derive(move || value.with($($body)+))
        }
    };
}

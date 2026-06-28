// mod.rs : src/utils

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),* $(,)?) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
    (crate: $mod_name:ident, $($type_name:ident),* $(,)?) => {
        mod $mod_name;

        pub(crate) use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(crate:
    compare,
    compare_approximate_equality_by_margin,
    compare_approximate_equality_by_multiplier,
    compare_approximate_equality_by_zero_margin_or_multiplier,
);


// ///////////////////////////// end of file //////////////////////////// //

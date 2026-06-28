// mod.rs : src/internal

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

declare_and_publish!(crate: margin_evaluator, MarginEvaluator);
declare_and_publish!(crate: multiplier_evaluator, MultiplierEvaluator);
declare_and_publish!(crate: zero_margin_or_multiplier_evaluator, ZeroMarginOrMultiplierEvaluator);


// ///////////////////////////// end of file //////////////////////////// //

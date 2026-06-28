// mod.rs : src/traits

//! Traits.

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),* $(,)?) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(approximate_equality_evaluator, ApproximateEqualityEvaluator);
declare_and_publish!(testable_as_f64, TestableAsF64);


// ///////////////////////////// end of file //////////////////////////// //

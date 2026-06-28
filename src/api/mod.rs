// mod.rs : src/api

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),* $(,)?) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(evaluate_scalar, evaluate_scalar_eq_approx);
declare_and_publish!(evaluate_vector, evaluate_vector_eq_approx);
declare_and_publish!(margin, margin);
declare_and_publish!(multiplier, multiplier);
declare_and_publish!(zero_margin_or_multiplier, zero_margin_or_multiplier);


// ///////////////////////////// end of file //////////////////////////// //

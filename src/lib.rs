// lib.rs : test_help-rs

#![allow(non_camel_case_types)]
#![cfg_attr(all(test, feature = "nightly-constants"), feature(more_float_constants))]

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),* $(,)?) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(comparison_result, ComparisonResult);
declare_and_publish!(vector_comparison_result, VectorComparisonResult);

pub mod constants;
pub mod traits;

mod api;
mod internal;
#[macro_use]
mod macros;
mod utils;

pub use api::{
    evaluate_scalar_eq_approx,
    evaluate_vector_eq_approx,
    margin,
    multiplier,
    zero_margin_or_multiplier,
};


#[cfg(test)]
#[rustfmt::skip]
mod tests;


// ///////////////////////////// end of file //////////////////////////// //

//! Test helpers for Rust.
//!
//! **test_help-rs** provides assertion macros and evaluators for
//! approximate equality of floating-point scalars and vectors in unit
//! tests — filling gaps around `f32` and `f64` comparisons in Rust's
//! stock testing support.
//!
//! # Installation
//!
//! Reference in **Cargo.toml** in the usual way:
//!
//! ```toml
//! test_help-rs = { version = "0.1" }
//! ```
//!
//! # Components
//!
//! ## Macros
//!
//! * [`assert_scalar_eq_approx!`] — scalar approximate equality;
//! * [`assert_scalar_ne_approx!`] — scalar approximate inequality;
//! * [`assert_vector_eq_approx!`] — vector approximate equality;
//! * [`assert_vector_ne_approx!`] — vector approximate inequality;
//!
//! ## Functions
//!
//! * [`margin`] — margin-based [`ApproximateEqualityEvaluator`];
//! * [`multiplier`] — multiplier-based [`ApproximateEqualityEvaluator`];
//! * [`zero_margin_or_multiplier`] — combined stock evaluator used by
//!   the two-argument assertion macros;
//! * [`evaluate_scalar_eq_approx`] — scalar comparison without
//!   asserting;
//! * [`evaluate_vector_eq_approx`] — vector comparison without
//!   asserting;
//!
//! ## Types
//!
//! * [`ComparisonResult`] — outcome of a scalar comparison;
//! * [`VectorComparisonResult`] — outcome of a vector comparison;
//! * [`traits::ApproximateEqualityEvaluator`] — custom comparison
//!   strategy;
//! * [`traits::TestableAsF64`] — types usable with the assertion
//!   macros (via [`base_traits::ToF64`]);
//!
//! ## Constants
//!
//! * [`constants::DEFAULT_MARGIN`] and [`constants::DEFAULT_MULTIPLIER`]
//!   — stock tolerance values for the two-argument macros;
//!
//! # Examples
//!
//! ```
//! use test_helpers::{assert_scalar_eq_approx, margin};
//!
//! assert_scalar_eq_approx!(3.0, 3.0001, margin(0.0001));
//! ```
//!
//! See the project [README](https://github.com/synesissoftware/test_help-rs)
//! for further information.

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

mod internal;
#[macro_use]
mod macros;
mod utils;

declare_and_publish!(
    api,
    evaluate_scalar_eq_approx,
    evaluate_vector_eq_approx,
    margin,
    multiplier,
    zero_margin_or_multiplier,
);


#[cfg(test)]
#[rustfmt::skip]
mod tests;

// ///////////////////////////// end of file //////////////////////////// //

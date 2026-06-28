// evaluate_scalar.rs : src/api

use crate::{
    traits::{
        ApproximateEqualityEvaluator,
        TestableAsF64,
    },
    ComparisonResult,
};

use std::fmt as std_fmt;


// API functions

/// Compares two scalar values using `evaluator` without asserting.
///
/// Converts `expected` and `actual` to `f64` via [`TestableAsF64`], then
/// delegates to [`ApproximateEqualityEvaluator::evaluate`].
///
/// Used by [`assert_scalar_eq_approx!`] and
/// [`assert_scalar_ne_approx!`]; may also be used when building custom
/// assertion macros for application-defined types.
///
/// # Returns
///
/// A tuple of:
///
/// * [`ComparisonResult`];
/// * `margin_factor` reported by the evaluator, if any;
/// * `multiplier_factor` reported by the evaluator, if any;
///
/// # Examples
///
/// ```
/// use test_helpers::{
///     evaluate_scalar_eq_approx,
///     margin,
///     ComparisonResult,
/// };
///
/// let (result, _, _) = evaluate_scalar_eq_approx(&3.0, &3.0001, &margin(0.0001));
/// assert_eq!(ComparisonResult::ApproximatelyEqual, result);
/// ```
pub fn evaluate_scalar_eq_approx<T_expected, T_actual>(
    expected : &T_expected,
    actual : &T_actual,
    evaluator : &dyn ApproximateEqualityEvaluator,
) -> (
    ComparisonResult, // comparison_result
    Option<f64>,      // margin_factor
    Option<f64>,      // multiplier_factor
)
where
    T_expected : TestableAsF64 + std_fmt::Debug,
    T_actual : TestableAsF64 + std_fmt::Debug,
{
    let (expected, actual) = {
        let expected : &dyn TestableAsF64 = expected;
        let actual : &dyn TestableAsF64 = actual;

        let expected = expected.testable_as_f64();
        let actual = actual.testable_as_f64();

        (expected, actual)
    };

    evaluator.evaluate(expected, actual)
}


// ///////////////////////////// end of file //////////////////////////// //

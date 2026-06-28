// evaluate_vector.rs : src/api

use crate::{
    traits::{
        ApproximateEqualityEvaluator,
        TestableAsF64,
    },
    ComparisonResult,
    VectorComparisonResult,
};

use super::evaluate_scalar_eq_approx;

use std::{
    convert as std_convert,
    fmt as std_fmt,
};


// API functions

/// Compares two vectors element-wise using `evaluator` without asserting.
///
/// `expected` and `actual` may be slices, arrays, or [`Vec`]; each element
/// pair is compared via [`evaluate_scalar_eq_approx`].
///
/// Used by [`assert_vector_eq_approx!`] and
/// [`assert_vector_ne_approx!`]; may also be used when building custom
/// vector assertion macros.
///
/// # Returns
///
/// A tuple of:
///
/// * [`VectorComparisonResult`];
/// * `margin_factor` from the first inexact element match, if any;
/// * `multiplier_factor` from the first inexact element match, if any;
///
/// When lengths differ, returns
/// [`VectorComparisonResult::DifferentLengths`]
/// with `(None, None)` factors.
///
/// # Examples
///
/// ```
/// use test_helpers::{
///     evaluate_vector_eq_approx,
///     multiplier,
///     VectorComparisonResult,
/// };
///
/// let expected = &[3.0, -40404.0, 1.23456];
/// let actual = vec![3.0, -40410.0, 1.234567];
/// let (result, _, _) =
///     evaluate_vector_eq_approx(&expected, &actual, &multiplier(0.00015));
/// assert!(matches!(result, VectorComparisonResult::ApproximatelyEqual));
/// ```
pub fn evaluate_vector_eq_approx<T_expected, T_actual, T_expectedElement, T_actualElement>(
    expected : &T_expected,
    actual : &T_actual,
    evaluator : &dyn ApproximateEqualityEvaluator,
) -> (
    VectorComparisonResult, // comparison_result
    Option<f64>,            // margin_factor
    Option<f64>,            // multiplier_factor
)
where
    T_expected : std_convert::AsRef<[T_expectedElement]>,
    T_actual : std_convert::AsRef<[T_actualElement]>,
    T_expectedElement : TestableAsF64 + std_fmt::Debug,
    T_actualElement : TestableAsF64 + std_fmt::Debug,
{
    let expected = expected.as_ref();
    let actual = actual.as_ref();

    let expected_length = expected.len();
    let actual_length = actual.len();

    if expected_length != actual_length {
        (
            VectorComparisonResult::DifferentLengths {
                expected_length,
                actual_length,
            },
            None,
            None,
        )
    } else {
        let mut any_inexact = false;
        let mut margin_factor = None;
        let mut multiplier_factor = None;

        for ix in 0..expected_length {
            let expected_element = &expected[ix];
            let actual_element = &actual[ix];

            let (scalar_comparison_result, scalar_margin_factor, scalar_multiplier_factor) =
                evaluate_scalar_eq_approx(expected_element, actual_element, evaluator);

            match scalar_comparison_result {
                ComparisonResult::ExactlyEqual => (),
                ComparisonResult::ApproximatelyEqual => {
                    if !any_inexact {
                        any_inexact = true;
                        margin_factor = scalar_margin_factor;
                        multiplier_factor = scalar_multiplier_factor;
                    }
                },
                ComparisonResult::Unequal => {
                    let (expected_value_of_first_unequal_element, actual_value_of_first_unequal_element) = {
                        let expected : &dyn TestableAsF64 = &expected[ix];
                        let actual : &dyn TestableAsF64 = &actual[ix];

                        let expected = expected.testable_as_f64();
                        let actual = actual.testable_as_f64();

                        (expected, actual)
                    };

                    return (
                        VectorComparisonResult::UnequalElements {
                            index_of_first_unequal_element : ix,
                            expected_value_of_first_unequal_element,
                            actual_value_of_first_unequal_element,
                        },
                        scalar_margin_factor,
                        scalar_multiplier_factor,
                    );
                },
            };
        }

        (
            if any_inexact {
                VectorComparisonResult::ApproximatelyEqual
            } else {
                VectorComparisonResult::ExactlyEqual
            },
            margin_factor,
            multiplier_factor,
        )
    }
}


// ///////////////////////////// end of file //////////////////////////// //

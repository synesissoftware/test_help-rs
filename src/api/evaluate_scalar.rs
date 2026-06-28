// evaluate_scalar.rs : src/api

use crate::{
    traits::{
        ApproximateEqualityEvaluator,
        TestableAsF64,
    },
    ComparisonResult,
};

use std::fmt as std_fmt;


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

// margin_evaluator.rs : src/internal

use crate::{
    traits::ApproximateEqualityEvaluator,
    utils::compare_approximate_equality_by_margin,
    ComparisonResult,
};


/// [`ApproximateEqualityEvaluator`] implementation backing
/// [`crate::margin`].
#[derive(Debug)]
pub(crate) struct MarginEvaluator {
    pub(crate) factor : f64,
}


// API functions

impl MarginEvaluator {
}


// Mutating methods

impl MarginEvaluator {
}


// Non-mutating methods

impl MarginEvaluator {
}


// Implementation

impl MarginEvaluator {
}


// Trait implementations

impl ApproximateEqualityEvaluator for MarginEvaluator {
    fn evaluate(
        &self,
        expected : f64,
        actual : f64,
    ) -> (
        ComparisonResult, // comparison_result
        Option<f64>,      // margin_factor
        Option<f64>,      // multiplier_factor
    ) {
        let comparison_result = compare_approximate_equality_by_margin(expected, actual, self.factor);

        (comparison_result, Some(self.factor), None)
    }
}


// ///////////////////////////// end of file //////////////////////////// //

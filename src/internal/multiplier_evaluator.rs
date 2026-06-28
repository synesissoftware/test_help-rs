// multiplier_evaluator.rs : src/internal

use crate::{
    traits::ApproximateEqualityEvaluator,
    utils::compare_approximate_equality_by_multiplier,
    ComparisonResult,
};


/// [`ApproximateEqualityEvaluator`] implementation backing
/// [`crate::multiplier`].
#[derive(Debug)]
pub(crate) struct MultiplierEvaluator {
    pub(crate) factor : f64,
}


// API functions

impl MultiplierEvaluator {
}


// Mutating methods

impl MultiplierEvaluator {
}


// Non-mutating methods

impl MultiplierEvaluator {
}


// Implementation

impl MultiplierEvaluator {
}


// Trait implementations

impl ApproximateEqualityEvaluator for MultiplierEvaluator {
    fn evaluate(
        &self,
        expected : f64,
        actual : f64,
    ) -> (
        ComparisonResult, // comparison_result
        Option<f64>,      // margin_factor
        Option<f64>,      // multiplier_factor
    ) {
        let comparison_result = compare_approximate_equality_by_multiplier(expected, actual, self.factor);

        (comparison_result, None, Some(self.factor))
    }
}


// ///////////////////////////// end of file //////////////////////////// //

// zero_margin_or_multiplier_evaluator.rs : src/internal

use crate::{
    traits::ApproximateEqualityEvaluator,
    utils::compare_approximate_equality_by_zero_margin_or_multiplier,
    ComparisonResult,
};


/// [`ApproximateEqualityEvaluator`] implementation backing
/// [`crate::zero_margin_or_multiplier`].
#[derive(Debug)]
pub(crate) struct ZeroMarginOrMultiplierEvaluator {
    pub(crate) multiplier_factor :  f64,
    pub(crate) zero_margin_factor : f64,
}


// API functions

impl ZeroMarginOrMultiplierEvaluator {
}


// Mutating methods

impl ZeroMarginOrMultiplierEvaluator {
}


// Non-mutating methods

impl ZeroMarginOrMultiplierEvaluator {
}


// Implementation

impl ZeroMarginOrMultiplierEvaluator {
}


// Trait implementations

impl ApproximateEqualityEvaluator for ZeroMarginOrMultiplierEvaluator {
    fn evaluate(
        &self,
        expected : f64,
        actual : f64,
    ) -> (
        ComparisonResult, // comparison_result
        Option<f64>,      // margin_factor
        Option<f64>,      // multiplier_factor
    ) {
        let comparison_result = compare_approximate_equality_by_zero_margin_or_multiplier(
            expected,
            actual,
            self.multiplier_factor,
            self.zero_margin_factor,
        );

        (
            comparison_result,
            Some(self.zero_margin_factor),
            Some(self.multiplier_factor),
        )
    }
}


// ///////////////////////////// end of file //////////////////////////// //

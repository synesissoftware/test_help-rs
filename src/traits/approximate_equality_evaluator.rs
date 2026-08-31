// approximate_equality_evaluator.rs : src/traits

use crate::ComparisonResult;


/// Trait that defines a mechanism for performing approximate equality
/// evaluation.
pub trait ApproximateEqualityEvaluator {
    /// Compares two `f64` values and reports the applied tolerance factors.
    fn evaluate(
        &self,
        expected : f64,
        actual : f64,
    ) -> (
        ComparisonResult, // comparison_result
        Option<f64>,      // margin_factor
        Option<f64>,      // multiplier_factor
    );
}


// ///////////////////////////// end of file //////////////////////////// //

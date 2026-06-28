// multiplier.rs : src/api

use crate::{
    internal::MultiplierEvaluator,
    traits::ApproximateEqualityEvaluator,
};


// API functions

/// Creates an [`ApproximateEqualityEvaluator`] that operates by applying
/// the given `factor` as a multiplier to determine approximate equality.
pub fn multiplier(factor : f64) -> impl ApproximateEqualityEvaluator {
    MultiplierEvaluator {
        factor,
    }
}


// ///////////////////////////// end of file //////////////////////////// //

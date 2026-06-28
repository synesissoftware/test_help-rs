// margin.rs : src/api

use crate::{
    internal::MarginEvaluator,
    traits::ApproximateEqualityEvaluator,
};


// API functions

/// Creates an [`ApproximateEqualityEvaluator`] that operates by applying
/// the given `factor` as a margin to determine approximate equality.
pub fn margin(factor : f64) -> impl ApproximateEqualityEvaluator {
    MarginEvaluator {
        factor,
    }
}


// ///////////////////////////// end of file //////////////////////////// //

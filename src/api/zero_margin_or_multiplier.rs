// zero_margin_or_multiplier.rs : src/api

use crate::{
    internal::ZeroMarginOrMultiplierEvaluator,
    traits::ApproximateEqualityEvaluator,
};


// API functions

/// Creates an [`ApproximateEqualityEvaluator`] that operates by applying
/// the given `multiplier_factor` as a multiplier to determine approximate
/// equality in all cases except when or both comparands is zero, in which
/// case it applies the `zero_margin_factor` as a margin to determine
/// approximate equality.
pub fn zero_margin_or_multiplier(
    multiplier_factor : f64,
    zero_margin_factor : f64,
) -> impl ApproximateEqualityEvaluator {
    ZeroMarginOrMultiplierEvaluator {
        multiplier_factor,
        zero_margin_factor,
    }
}


// ///////////////////////////// end of file //////////////////////////// //

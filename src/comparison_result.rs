// comparison_result.rs : test_help-rs

/// Comparison result type.
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum ComparisonResult {
    /// The comparands are exactly equal.
    ExactlyEqual,
    /// The comparands are equal within the tolerance of the given margin or
    /// multiplier.
    ApproximatelyEqual,
    /// The comparands are not equal within the tolerance of the given
    /// margin or multiplier.
    Unequal,
}


// ///////////////////////////// end of file //////////////////////////// //

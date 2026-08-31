// vector_comparison_result.rs : test_help-rs

/// Vector comparison result type.
#[derive(Debug)]
pub enum VectorComparisonResult {
    /// All elements are exactly equal.
    ExactlyEqual,
    /// All elements are within the evaluator tolerance.
    ApproximatelyEqual,
    /// The vectors have different lengths.
    DifferentLengths {
        /// Length of the expected vector.
        expected_length : usize,
        /// Length of the actual vector.
        actual_length :   usize,
    },
    /// The first unequal element and its values.
    UnequalElements {
        /// Index of the first unequal element.
        index_of_first_unequal_element :          usize,
        /// Value of the first unequal expected element.
        expected_value_of_first_unequal_element : f64,
        /// Value of the first unequal actual element.
        actual_value_of_first_unequal_element :   f64,
    },
}


// ///////////////////////////// end of file //////////////////////////// //
